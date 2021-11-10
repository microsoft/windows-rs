#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Foundation_Diagnostics`*"]
pub struct AsyncCausalityTracer {}
impl AsyncCausalityTracer {
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn TraceOperationCreation<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param4: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: Param2, operationid: u64, operationname: Param4, relatedcontext: u64) -> ::windows::runtime::Result<()> {
        Self::IAsyncCausalityTracerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), tracelevel, source, platformid.into_param().abi(), operationid, operationname.into_param().abi(), relatedcontext).ok() })
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn TraceOperationCompletion<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: Param2, operationid: u64, status: super::AsyncStatus) -> ::windows::runtime::Result<()> {
        Self::IAsyncCausalityTracerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), tracelevel, source, platformid.into_param().abi(), operationid, status).ok() })
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn TraceOperationRelation<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: Param2, operationid: u64, relation: CausalityRelation) -> ::windows::runtime::Result<()> {
        Self::IAsyncCausalityTracerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), tracelevel, source, platformid.into_param().abi(), operationid, relation).ok() })
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn TraceSynchronousWorkStart<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: Param2, operationid: u64, work: CausalitySynchronousWork) -> ::windows::runtime::Result<()> {
        Self::IAsyncCausalityTracerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), tracelevel, source, platformid.into_param().abi(), operationid, work).ok() })
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn TraceSynchronousWorkCompletion(tracelevel: CausalityTraceLevel, source: CausalitySource, work: CausalitySynchronousWork) -> ::windows::runtime::Result<()> {
        Self::IAsyncCausalityTracerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), tracelevel, source, work).ok() })
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn TracingStatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::EventHandler<TracingStatusChangedEventArgs>>>(handler: Param0) -> ::windows::runtime::Result<super::EventRegistrationToken> {
        Self::IAsyncCausalityTracerStatics(|this| unsafe {
            let mut result__: super::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn RemoveTracingStatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::EventRegistrationToken>>(cookie: Param0) -> ::windows::runtime::Result<()> {
        Self::IAsyncCausalityTracerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), cookie.into_param().abi()).ok() })
    }
    pub fn IAsyncCausalityTracerStatics<R, F: FnOnce(&IAsyncCausalityTracerStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<AsyncCausalityTracer, IAsyncCausalityTracerStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
impl ::windows::runtime::RuntimeName for AsyncCausalityTracer {
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
unsafe impl ::windows::runtime::Abi for CausalityRelation {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CausalityRelation {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Foundation.Diagnostics.CausalityRelation;i4)");
}
impl ::windows::runtime::DefaultType for CausalityRelation {
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
unsafe impl ::windows::runtime::Abi for CausalitySource {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CausalitySource {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Foundation.Diagnostics.CausalitySource;i4)");
}
impl ::windows::runtime::DefaultType for CausalitySource {
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
unsafe impl ::windows::runtime::Abi for CausalitySynchronousWork {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CausalitySynchronousWork {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Foundation.Diagnostics.CausalitySynchronousWork;i4)");
}
impl ::windows::runtime::DefaultType for CausalitySynchronousWork {
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
unsafe impl ::windows::runtime::Abi for CausalityTraceLevel {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for CausalityTraceLevel {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Foundation.Diagnostics.CausalityTraceLevel;i4)");
}
impl ::windows::runtime::DefaultType for CausalityTraceLevel {
    type DefaultType = Self;
}
#[doc = "*Required features: `Foundation_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct ErrorDetails(pub ::windows::runtime::IInspectable);
impl ErrorDetails {
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Description(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LongDescription(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn HelpUri(&self) -> ::windows::runtime::Result<super::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::Uri>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn CreateFromHResultAsync(errorcode: i32) -> ::windows::runtime::Result<super::IAsyncOperation<ErrorDetails>> {
        Self::IErrorDetailsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), errorcode, &mut result__).from_abi::<super::IAsyncOperation<ErrorDetails>>(result__)
        })
    }
    pub fn IErrorDetailsStatics<R, F: FnOnce(&IErrorDetailsStatics) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<ErrorDetails, IErrorDetailsStatics> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ErrorDetails {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.ErrorDetails;{378cbb01-2cc9-428f-8c55-2c990d463e8f})");
}
unsafe impl ::windows::runtime::Interface for ErrorDetails {
    type Vtable = IErrorDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x378cbb01_2cc9_428f_8c55_2c990d463e8f);
}
impl ::windows::runtime::RuntimeName for ErrorDetails {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ErrorDetails";
}
impl ::core::convert::From<ErrorDetails> for ::windows::runtime::IUnknown {
    fn from(value: ErrorDetails) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ErrorDetails> for ::windows::runtime::IUnknown {
    fn from(value: &ErrorDetails) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ErrorDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ErrorDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ErrorDetails> for ::windows::runtime::IInspectable {
    fn from(value: ErrorDetails) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ErrorDetails> for ::windows::runtime::IInspectable {
    fn from(value: &ErrorDetails) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ErrorDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ErrorDetails {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
unsafe impl ::windows::runtime::Abi for ErrorOptions {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for ErrorOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Foundation.Diagnostics.ErrorOptions;u4)");
}
impl ::windows::runtime::DefaultType for ErrorOptions {
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
pub struct FileLoggingSession(pub ::windows::runtime::IInspectable);
impl FileLoggingSession {
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddLoggingChannel<'a, Param0: ::windows::runtime::IntoParam<'a, ILoggingChannel>>(&self, loggingchannel: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), loggingchannel.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddLoggingChannelWithLevel<'a, Param0: ::windows::runtime::IntoParam<'a, ILoggingChannel>>(&self, loggingchannel: Param0, maxlevel: LoggingLevel) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), loggingchannel.into_param().abi(), maxlevel).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn RemoveLoggingChannel<'a, Param0: ::windows::runtime::IntoParam<'a, ILoggingChannel>>(&self, loggingchannel: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), loggingchannel.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage")]
    #[doc = "*Required features: `Foundation_Diagnostics`, `Storage`*"]
    pub fn CloseAndSaveToFileAsync(&self) -> ::windows::runtime::Result<super::IAsyncOperation<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::IAsyncOperation<super::super::Storage::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogFileGenerated<'a, Param0: ::windows::runtime::IntoParam<'a, super::TypedEventHandler<IFileLoggingSession, LogFileGeneratedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn RemoveLogFileGenerated<'a, Param0: ::windows::runtime::IntoParam<'a, super::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(name: Param0) -> ::windows::runtime::Result<FileLoggingSession> {
        Self::IFileLoggingSessionFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<FileLoggingSession>(result__)
        })
    }
    pub fn IFileLoggingSessionFactory<R, F: FnOnce(&IFileLoggingSessionFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<FileLoggingSession, IFileLoggingSessionFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for FileLoggingSession {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.FileLoggingSession;{24c74216-fed2-404c-895f-1f9699cb02f7})");
}
unsafe impl ::windows::runtime::Interface for FileLoggingSession {
    type Vtable = IFileLoggingSession_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x24c74216_fed2_404c_895f_1f9699cb02f7);
}
impl ::windows::runtime::RuntimeName for FileLoggingSession {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.FileLoggingSession";
}
impl ::core::convert::From<FileLoggingSession> for ::windows::runtime::IUnknown {
    fn from(value: FileLoggingSession) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&FileLoggingSession> for ::windows::runtime::IUnknown {
    fn from(value: &FileLoggingSession) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for FileLoggingSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a FileLoggingSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<FileLoggingSession> for ::windows::runtime::IInspectable {
    fn from(value: FileLoggingSession) -> Self {
        value.0
    }
}
impl ::core::convert::From<&FileLoggingSession> for ::windows::runtime::IInspectable {
    fn from(value: &FileLoggingSession) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for FileLoggingSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a FileLoggingSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IFileLoggingSession> for FileLoggingSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFileLoggingSession> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFileLoggingSession> for &FileLoggingSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFileLoggingSession> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<FileLoggingSession> for super::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: FileLoggingSession) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&FileLoggingSession> for super::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &FileLoggingSession) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IClosable> for FileLoggingSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IClosable> for &FileLoggingSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IClosable> {
        ::core::convert::TryInto::<super::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for FileLoggingSession {}
unsafe impl ::core::marker::Sync for FileLoggingSession {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IAsyncCausalityTracerStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAsyncCausalityTracerStatics {
    type Vtable = IAsyncCausalityTracerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x50850b26_267e_451b_a890_ab6a370245ee);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsyncCausalityTracerStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: ::windows::runtime::GUID, operationid: u64, operationname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, relatedcontext: u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: ::windows::runtime::GUID, operationid: u64, status: super::AsyncStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: ::windows::runtime::GUID, operationid: u64, relation: CausalityRelation) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: ::windows::runtime::GUID, operationid: u64, work: CausalitySynchronousWork) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tracelevel: CausalityTraceLevel, source: CausalitySource, work: CausalitySynchronousWork) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::EventRegistrationToken) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IErrorDetails(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IErrorDetails {
    type Vtable = IErrorDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x378cbb01_2cc9_428f_8c55_2c990d463e8f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IErrorDetails_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IErrorDetailsStatics(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IErrorDetailsStatics {
    type Vtable = IErrorDetailsStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb7703750_0b1d_46c8_aa0e_4b8178e4fce9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IErrorDetailsStatics_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, errorcode: i32, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Foundation_Diagnostics`*"]
pub struct IErrorReportingSettings(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IErrorReportingSettings {
    type Vtable = IErrorReportingSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x16369792_b03e_4ba1_8bb8_d28f4ab4d2c0);
}
impl IErrorReportingSettings {
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn SetErrorOptions(&self, value: ErrorOptions) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn GetErrorOptions(&self) -> ::windows::runtime::Result<ErrorOptions> {
        let this = self;
        unsafe {
            let mut result__: ErrorOptions = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ErrorOptions>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IErrorReportingSettings {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{16369792-b03e-4ba1-8bb8-d28f4ab4d2c0}");
}
impl ::core::convert::From<IErrorReportingSettings> for ::windows::runtime::IUnknown {
    fn from(value: IErrorReportingSettings) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IErrorReportingSettings> for ::windows::runtime::IUnknown {
    fn from(value: &IErrorReportingSettings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IErrorReportingSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IErrorReportingSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IErrorReportingSettings> for ::windows::runtime::IInspectable {
    fn from(value: IErrorReportingSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IErrorReportingSettings> for ::windows::runtime::IInspectable {
    fn from(value: &IErrorReportingSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IErrorReportingSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IErrorReportingSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IErrorReportingSettings_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ErrorOptions) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ErrorOptions) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Foundation_Diagnostics`*"]
pub struct IFileLoggingSession(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFileLoggingSession {
    type Vtable = IFileLoggingSession_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x24c74216_fed2_404c_895f_1f9699cb02f7);
}
impl IFileLoggingSession {
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddLoggingChannel<'a, Param0: ::windows::runtime::IntoParam<'a, ILoggingChannel>>(&self, loggingchannel: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), loggingchannel.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddLoggingChannelWithLevel<'a, Param0: ::windows::runtime::IntoParam<'a, ILoggingChannel>>(&self, loggingchannel: Param0, maxlevel: LoggingLevel) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), loggingchannel.into_param().abi(), maxlevel).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn RemoveLoggingChannel<'a, Param0: ::windows::runtime::IntoParam<'a, ILoggingChannel>>(&self, loggingchannel: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), loggingchannel.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage")]
    #[doc = "*Required features: `Foundation_Diagnostics`, `Storage`*"]
    pub fn CloseAndSaveToFileAsync(&self) -> ::windows::runtime::Result<super::IAsyncOperation<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::IAsyncOperation<super::super::Storage::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogFileGenerated<'a, Param0: ::windows::runtime::IntoParam<'a, super::TypedEventHandler<IFileLoggingSession, LogFileGeneratedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn RemoveLogFileGenerated<'a, Param0: ::windows::runtime::IntoParam<'a, super::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IFileLoggingSession {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{24c74216-fed2-404c-895f-1f9699cb02f7}");
}
impl ::core::convert::From<IFileLoggingSession> for ::windows::runtime::IUnknown {
    fn from(value: IFileLoggingSession) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&IFileLoggingSession> for ::windows::runtime::IUnknown {
    fn from(value: &IFileLoggingSession) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFileLoggingSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IFileLoggingSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<IFileLoggingSession> for ::windows::runtime::IInspectable {
    fn from(value: IFileLoggingSession) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IFileLoggingSession> for ::windows::runtime::IInspectable {
    fn from(value: &IFileLoggingSession) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for IFileLoggingSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a IFileLoggingSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<IFileLoggingSession> for super::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IFileLoggingSession) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&IFileLoggingSession> for super::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &IFileLoggingSession) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IClosable> for IFileLoggingSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IClosable> for &IFileLoggingSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IClosable> {
        ::core::convert::TryInto::<super::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileLoggingSession_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, loggingchannel: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, loggingchannel: ::windows::runtime::RawPtr, maxlevel: LoggingLevel) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, loggingchannel: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::EventRegistrationToken) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IFileLoggingSessionFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFileLoggingSessionFactory {
    type Vtable = IFileLoggingSessionFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xeea08dce_8447_4daa_9133_12eb46f697d4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFileLoggingSessionFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILogFileGeneratedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILogFileGeneratedEventArgs {
    type Vtable = ILogFileGeneratedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x269e976f_0d38_4c1a_b53f_b395d881df84);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILogFileGeneratedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILoggingActivity(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILoggingActivity {
    type Vtable = ILoggingActivity_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xbc032941_b766_4cb5_9848_97ac6ba6d60c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingActivity_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILoggingActivity2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILoggingActivity2 {
    type Vtable = ILoggingActivity2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x26c29808_6322_456a_af82_80c8642f178b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingActivity2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, stopeventname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, stopeventname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, fields: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, stopeventname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, fields: ::windows::runtime::RawPtr, options: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILoggingActivityFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILoggingActivityFactory {
    type Vtable = ILoggingActivityFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6b33b483_e10a_4c58_97d5_10fb451074fb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingActivityFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, activityname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, loggingchannel: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, activityname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, loggingchannel: ::windows::runtime::RawPtr, level: LoggingLevel, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Foundation_Diagnostics`*"]
pub struct ILoggingChannel(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILoggingChannel {
    type Vtable = ILoggingChannel_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe9a50343_11d7_4f01_b5ca_cf495278c0a8);
}
impl ILoggingChannel {
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Enabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Level(&self) -> ::windows::runtime::Result<LoggingLevel> {
        let this = self;
        unsafe {
            let mut result__: LoggingLevel = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LoggingLevel>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogMessage<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, eventstring: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), eventstring.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogMessageWithLevel<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, eventstring: Param0, level: LoggingLevel) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), eventstring.into_param().abi(), level).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogValuePair<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value1: Param0, value2: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value1.into_param().abi(), value2).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogValuePairWithLevel<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value1: Param0, value2: i32, level: LoggingLevel) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value1.into_param().abi(), value2, level).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LoggingEnabled<'a, Param0: ::windows::runtime::IntoParam<'a, super::TypedEventHandler<ILoggingChannel, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn RemoveLoggingEnabled<'a, Param0: ::windows::runtime::IntoParam<'a, super::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ILoggingChannel {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{e9a50343-11d7-4f01-b5ca-cf495278c0a8}");
}
impl ::core::convert::From<ILoggingChannel> for ::windows::runtime::IUnknown {
    fn from(value: ILoggingChannel) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ILoggingChannel> for ::windows::runtime::IUnknown {
    fn from(value: &ILoggingChannel) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ILoggingChannel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ILoggingChannel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ILoggingChannel> for ::windows::runtime::IInspectable {
    fn from(value: ILoggingChannel) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ILoggingChannel> for ::windows::runtime::IInspectable {
    fn from(value: &ILoggingChannel) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ILoggingChannel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ILoggingChannel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<ILoggingChannel> for super::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ILoggingChannel) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ILoggingChannel> for super::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ILoggingChannel) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IClosable> for ILoggingChannel {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IClosable> for &ILoggingChannel {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IClosable> {
        ::core::convert::TryInto::<super::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingChannel_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut LoggingLevel) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventstring: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventstring: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, level: LoggingLevel) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value1: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value2: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value1: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value2: i32, level: LoggingLevel) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::EventRegistrationToken) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILoggingChannel2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILoggingChannel2 {
    type Vtable = ILoggingChannel2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x9f4c3cf3_0bac_45a5_9e33_baf3f3a246a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingChannel2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILoggingChannelFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILoggingChannelFactory {
    type Vtable = ILoggingChannelFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4edc5b9c_af80_4a9b_b0dc_398f9ae5207b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingChannelFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILoggingChannelFactory2(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILoggingChannelFactory2 {
    type Vtable = ILoggingChannelFactory2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4c6ef5dd_3b27_4dc9_99f0_299c6e4603a1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingChannelFactory2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, options: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, options: ::windows::runtime::RawPtr, id: ::windows::runtime::GUID, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILoggingChannelOptions(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILoggingChannelOptions {
    type Vtable = ILoggingChannelOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc3e847ff_0ebb_4a53_8c54_dec24926cb2c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingChannelOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILoggingChannelOptionsFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILoggingChannelOptionsFactory {
    type Vtable = ILoggingChannelOptionsFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xa93151da_7faf_4191_8755_5e86dc65d896);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingChannelOptionsFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, group: ::windows::runtime::GUID, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILoggingFields(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILoggingFields {
    type Vtable = ILoggingFields_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd7f6b7af_762d_4579_83bd_52c23bc333bc);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingFields_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: u8, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: u8, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const u8, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const u8, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: i16, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: i16, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const i16, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const i16, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: u16, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: u16, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const u16, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const u16, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: i32, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: i32, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const i32, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const i32, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: u32, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: u32, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const u32, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const u32, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: i64, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: i64, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const i64, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const i64, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: u64, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: u64, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const u64, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const u64, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: f32, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: f32, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const f32, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const f32, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: f64, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: f64, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const f64, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const f64, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: u16, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: u16, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const u16, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const u16, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: bool, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: bool, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const bool, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const bool, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: ::windows::runtime::GUID, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: ::windows::runtime::GUID, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const ::windows::runtime::GUID, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const ::windows::runtime::GUID, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: super::DateTime) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: super::DateTime, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: super::DateTime, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const super::DateTime) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const super::DateTime, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const super::DateTime, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: super::TimeSpan) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: super::TimeSpan, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: super::TimeSpan, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const super::TimeSpan) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const super::TimeSpan, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const super::TimeSpan, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: super::Point) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: super::Point, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: super::Point, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const super::Point) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const super::Point, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const super::Point, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: super::Size) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: super::Size, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: super::Size, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const super::Size) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const super::Size, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const super::Size, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: super::Rect) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: super::Rect, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: super::Rect, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const super::Rect) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const super::Rect, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const super::Rect, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILoggingOptions(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILoggingOptions {
    type Vtable = ILoggingOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x90bc7850_0192_4f5d_ac26_006adaca12d8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingOptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut LoggingOpcode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: LoggingOpcode) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILoggingOptionsFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILoggingOptionsFactory {
    type Vtable = ILoggingOptionsFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd713c6cb_98ab_464b_9f22_a3268478368a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingOptionsFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, keywords: i64, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Foundation_Diagnostics`*"]
pub struct ILoggingSession(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILoggingSession {
    type Vtable = ILoggingSession_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6221f306_9380_4ad7_baf5_41ea9310d768);
}
impl ILoggingSession {
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Storage")]
    #[doc = "*Required features: `Foundation_Diagnostics`, `Storage`*"]
    pub fn SaveToFileAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFolder>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, folder: Param0, filename: Param1) -> ::windows::runtime::Result<super::IAsyncOperation<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), folder.into_param().abi(), filename.into_param().abi(), &mut result__).from_abi::<super::IAsyncOperation<super::super::Storage::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddLoggingChannel<'a, Param0: ::windows::runtime::IntoParam<'a, ILoggingChannel>>(&self, loggingchannel: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), loggingchannel.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddLoggingChannelWithLevel<'a, Param0: ::windows::runtime::IntoParam<'a, ILoggingChannel>>(&self, loggingchannel: Param0, maxlevel: LoggingLevel) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), loggingchannel.into_param().abi(), maxlevel).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn RemoveLoggingChannel<'a, Param0: ::windows::runtime::IntoParam<'a, ILoggingChannel>>(&self, loggingchannel: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), loggingchannel.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ILoggingSession {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{6221f306-9380-4ad7-baf5-41ea9310d768}");
}
impl ::core::convert::From<ILoggingSession> for ::windows::runtime::IUnknown {
    fn from(value: ILoggingSession) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ILoggingSession> for ::windows::runtime::IUnknown {
    fn from(value: &ILoggingSession) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ILoggingSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ILoggingSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ILoggingSession> for ::windows::runtime::IInspectable {
    fn from(value: ILoggingSession) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ILoggingSession> for ::windows::runtime::IInspectable {
    fn from(value: &ILoggingSession) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ILoggingSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ILoggingSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<ILoggingSession> for super::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ILoggingSession) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&ILoggingSession> for super::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &ILoggingSession) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IClosable> for ILoggingSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IClosable> for &ILoggingSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IClosable> {
        ::core::convert::TryInto::<super::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingSession_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, folder: ::windows::runtime::RawPtr, filename: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, loggingchannel: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, loggingchannel: ::windows::runtime::RawPtr, maxlevel: LoggingLevel) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, loggingchannel: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILoggingSessionFactory(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILoggingSessionFactory {
    type Vtable = ILoggingSessionFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4e937ee5_58fd_45e0_8c2f_a132eff95c1e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingSessionFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
#[doc = "*Required features: `Foundation_Diagnostics`*"]
pub struct ILoggingTarget(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILoggingTarget {
    type Vtable = ILoggingTarget_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x65f16c35_e388_4e26_b17a_f51cd3a83916);
}
impl ILoggingTarget {
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn IsEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn IsEnabledWithLevel(&self, level: LoggingLevel) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), level, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn IsEnabledWithLevelAndKeywords(&self, level: LoggingLevel, keywords: i64) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), level, keywords, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogEvent<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, eventname: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), eventname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogEventWithFields<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, LoggingFields>>(&self, eventname: Param0, fields: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), eventname.into_param().abi(), fields.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogEventWithFieldsAndLevel<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, LoggingFields>>(&self, eventname: Param0, fields: Param1, level: LoggingLevel) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), eventname.into_param().abi(), fields.into_param().abi(), level).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogEventWithFieldsAndOptions<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, LoggingFields>, Param3: ::windows::runtime::IntoParam<'a, LoggingOptions>>(&self, eventname: Param0, fields: Param1, level: LoggingLevel, options: Param3) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), eventname.into_param().abi(), fields.into_param().abi(), level, options.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn StartActivity<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, starteventname: Param0) -> ::windows::runtime::Result<LoggingActivity> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), starteventname.into_param().abi(), &mut result__).from_abi::<LoggingActivity>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn StartActivityWithFields<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, LoggingFields>>(&self, starteventname: Param0, fields: Param1) -> ::windows::runtime::Result<LoggingActivity> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), starteventname.into_param().abi(), fields.into_param().abi(), &mut result__).from_abi::<LoggingActivity>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn StartActivityWithFieldsAndLevel<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, LoggingFields>>(&self, starteventname: Param0, fields: Param1, level: LoggingLevel) -> ::windows::runtime::Result<LoggingActivity> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), starteventname.into_param().abi(), fields.into_param().abi(), level, &mut result__).from_abi::<LoggingActivity>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn StartActivityWithFieldsAndOptions<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, LoggingFields>, Param3: ::windows::runtime::IntoParam<'a, LoggingOptions>>(&self, starteventname: Param0, fields: Param1, level: LoggingLevel, options: Param3) -> ::windows::runtime::Result<LoggingActivity> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), starteventname.into_param().abi(), fields.into_param().abi(), level, options.into_param().abi(), &mut result__).from_abi::<LoggingActivity>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ILoggingTarget {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{65f16c35-e388-4e26-b17a-f51cd3a83916}");
}
impl ::core::convert::From<ILoggingTarget> for ::windows::runtime::IUnknown {
    fn from(value: ILoggingTarget) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&ILoggingTarget> for ::windows::runtime::IUnknown {
    fn from(value: &ILoggingTarget) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ILoggingTarget {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ILoggingTarget {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<ILoggingTarget> for ::windows::runtime::IInspectable {
    fn from(value: ILoggingTarget) -> Self {
        value.0
    }
}
impl ::core::convert::From<&ILoggingTarget> for ::windows::runtime::IInspectable {
    fn from(value: &ILoggingTarget) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for ILoggingTarget {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a ILoggingTarget {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoggingTarget_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, level: LoggingLevel, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, level: LoggingLevel, keywords: i64, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, fields: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, fields: ::windows::runtime::RawPtr, level: LoggingLevel) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, fields: ::windows::runtime::RawPtr, level: LoggingLevel, options: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, starteventname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, starteventname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, fields: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, starteventname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, fields: ::windows::runtime::RawPtr, level: LoggingLevel, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, starteventname: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING>, fields: ::windows::runtime::RawPtr, level: LoggingLevel, options: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITracingStatusChangedEventArgs(pub ::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITracingStatusChangedEventArgs {
    type Vtable = ITracingStatusChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x410b7711_ff3b_477f_9c9a_d2efda302dc3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ITracingStatusChangedEventArgs_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, count: *mut u32, values: *mut *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value: *mut i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut CausalityTraceLevel) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Foundation_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LogFileGeneratedEventArgs(pub ::windows::runtime::IInspectable);
impl LogFileGeneratedEventArgs {
    #[cfg(feature = "Storage")]
    #[doc = "*Required features: `Foundation_Diagnostics`, `Storage`*"]
    pub fn File(&self) -> ::windows::runtime::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::StorageFile>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LogFileGeneratedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.LogFileGeneratedEventArgs;{269e976f-0d38-4c1a-b53f-b395d881df84})");
}
unsafe impl ::windows::runtime::Interface for LogFileGeneratedEventArgs {
    type Vtable = ILogFileGeneratedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x269e976f_0d38_4c1a_b53f_b395d881df84);
}
impl ::windows::runtime::RuntimeName for LogFileGeneratedEventArgs {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.LogFileGeneratedEventArgs";
}
impl ::core::convert::From<LogFileGeneratedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: LogFileGeneratedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LogFileGeneratedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &LogFileGeneratedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LogFileGeneratedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a LogFileGeneratedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LogFileGeneratedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: LogFileGeneratedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LogFileGeneratedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &LogFileGeneratedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LogFileGeneratedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LogFileGeneratedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for LogFileGeneratedEventArgs {}
unsafe impl ::core::marker::Sync for LogFileGeneratedEventArgs {}
#[doc = "*Required features: `Foundation_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LoggingActivity(pub ::windows::runtime::IInspectable);
impl LoggingActivity {
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Channel(&self) -> ::windows::runtime::Result<LoggingChannel> {
        let this = &::windows::runtime::Interface::cast::<ILoggingActivity2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LoggingChannel>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn StopActivity<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, stopeventname: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ILoggingActivity2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), stopeventname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn StopActivityWithFields<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, LoggingFields>>(&self, stopeventname: Param0, fields: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ILoggingActivity2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), stopeventname.into_param().abi(), fields.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn StopActivityWithFieldsAndOptions<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, LoggingFields>, Param2: ::windows::runtime::IntoParam<'a, LoggingOptions>>(&self, stopeventname: Param0, fields: Param1, options: Param2) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ILoggingActivity2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), stopeventname.into_param().abi(), fields.into_param().abi(), options.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn IsEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn IsEnabledWithLevel(&self, level: LoggingLevel) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), level, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn IsEnabledWithLevelAndKeywords(&self, level: LoggingLevel, keywords: i64) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), level, keywords, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogEvent<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, eventname: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ILoggingTarget>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), eventname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogEventWithFields<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, LoggingFields>>(&self, eventname: Param0, fields: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ILoggingTarget>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), eventname.into_param().abi(), fields.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogEventWithFieldsAndLevel<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, LoggingFields>>(&self, eventname: Param0, fields: Param1, level: LoggingLevel) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ILoggingTarget>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), eventname.into_param().abi(), fields.into_param().abi(), level).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogEventWithFieldsAndOptions<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, LoggingFields>, Param3: ::windows::runtime::IntoParam<'a, LoggingOptions>>(&self, eventname: Param0, fields: Param1, level: LoggingLevel, options: Param3) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ILoggingTarget>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), eventname.into_param().abi(), fields.into_param().abi(), level, options.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn StartActivity<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, starteventname: Param0) -> ::windows::runtime::Result<LoggingActivity> {
        let this = &::windows::runtime::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), starteventname.into_param().abi(), &mut result__).from_abi::<LoggingActivity>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn StartActivityWithFields<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, LoggingFields>>(&self, starteventname: Param0, fields: Param1) -> ::windows::runtime::Result<LoggingActivity> {
        let this = &::windows::runtime::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), starteventname.into_param().abi(), fields.into_param().abi(), &mut result__).from_abi::<LoggingActivity>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn StartActivityWithFieldsAndLevel<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, LoggingFields>>(&self, starteventname: Param0, fields: Param1, level: LoggingLevel) -> ::windows::runtime::Result<LoggingActivity> {
        let this = &::windows::runtime::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), starteventname.into_param().abi(), fields.into_param().abi(), level, &mut result__).from_abi::<LoggingActivity>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn StartActivityWithFieldsAndOptions<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, LoggingFields>, Param3: ::windows::runtime::IntoParam<'a, LoggingOptions>>(&self, starteventname: Param0, fields: Param1, level: LoggingLevel, options: Param3) -> ::windows::runtime::Result<LoggingActivity> {
        let this = &::windows::runtime::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), starteventname.into_param().abi(), fields.into_param().abi(), level, options.into_param().abi(), &mut result__).from_abi::<LoggingActivity>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn CreateLoggingActivity<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ILoggingChannel>>(activityname: Param0, loggingchannel: Param1) -> ::windows::runtime::Result<LoggingActivity> {
        Self::ILoggingActivityFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), activityname.into_param().abi(), loggingchannel.into_param().abi(), &mut result__).from_abi::<LoggingActivity>(result__)
        })
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn CreateLoggingActivityWithLevel<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ILoggingChannel>>(activityname: Param0, loggingchannel: Param1, level: LoggingLevel) -> ::windows::runtime::Result<LoggingActivity> {
        Self::ILoggingActivityFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), activityname.into_param().abi(), loggingchannel.into_param().abi(), level, &mut result__).from_abi::<LoggingActivity>(result__)
        })
    }
    pub fn ILoggingActivityFactory<R, F: FnOnce(&ILoggingActivityFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<LoggingActivity, ILoggingActivityFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LoggingActivity {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.LoggingActivity;{bc032941-b766-4cb5-9848-97ac6ba6d60c})");
}
unsafe impl ::windows::runtime::Interface for LoggingActivity {
    type Vtable = ILoggingActivity_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xbc032941_b766_4cb5_9848_97ac6ba6d60c);
}
impl ::windows::runtime::RuntimeName for LoggingActivity {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.LoggingActivity";
}
impl ::core::convert::From<LoggingActivity> for ::windows::runtime::IUnknown {
    fn from(value: LoggingActivity) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LoggingActivity> for ::windows::runtime::IUnknown {
    fn from(value: &LoggingActivity) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LoggingActivity {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a LoggingActivity {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LoggingActivity> for ::windows::runtime::IInspectable {
    fn from(value: LoggingActivity) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LoggingActivity> for ::windows::runtime::IInspectable {
    fn from(value: &LoggingActivity) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LoggingActivity {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LoggingActivity {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::core::convert::TryFrom<LoggingActivity> for super::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: LoggingActivity) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LoggingActivity> for super::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &LoggingActivity) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IClosable> for LoggingActivity {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IClosable> for &LoggingActivity {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IClosable> {
        ::core::convert::TryInto::<super::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<LoggingActivity> for ILoggingTarget {
    type Error = ::windows::runtime::Error;
    fn try_from(value: LoggingActivity) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LoggingActivity> for ILoggingTarget {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &LoggingActivity) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILoggingTarget> for LoggingActivity {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILoggingTarget> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILoggingTarget> for &LoggingActivity {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILoggingTarget> {
        ::core::convert::TryInto::<ILoggingTarget>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for LoggingActivity {}
unsafe impl ::core::marker::Sync for LoggingActivity {}
#[doc = "*Required features: `Foundation_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LoggingChannel(pub ::windows::runtime::IInspectable);
impl LoggingChannel {
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Enabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Level(&self) -> ::windows::runtime::Result<LoggingLevel> {
        let this = self;
        unsafe {
            let mut result__: LoggingLevel = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LoggingLevel>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogMessage<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, eventstring: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), eventstring.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogMessageWithLevel<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, eventstring: Param0, level: LoggingLevel) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), eventstring.into_param().abi(), level).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogValuePair<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value1: Param0, value2: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value1.into_param().abi(), value2).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogValuePairWithLevel<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value1: Param0, value2: i32, level: LoggingLevel) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), value1.into_param().abi(), value2, level).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LoggingEnabled<'a, Param0: ::windows::runtime::IntoParam<'a, super::TypedEventHandler<ILoggingChannel, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::EventRegistrationToken = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn RemoveLoggingEnabled<'a, Param0: ::windows::runtime::IntoParam<'a, super::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = &::windows::runtime::Interface::cast::<ILoggingChannel2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn IsEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn IsEnabledWithLevel(&self, level: LoggingLevel) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), level, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn IsEnabledWithLevelAndKeywords(&self, level: LoggingLevel, keywords: i64) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), level, keywords, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogEvent<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, eventname: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ILoggingTarget>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), eventname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogEventWithFields<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, LoggingFields>>(&self, eventname: Param0, fields: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ILoggingTarget>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), eventname.into_param().abi(), fields.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogEventWithFieldsAndLevel<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, LoggingFields>>(&self, eventname: Param0, fields: Param1, level: LoggingLevel) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ILoggingTarget>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), eventname.into_param().abi(), fields.into_param().abi(), level).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogEventWithFieldsAndOptions<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, LoggingFields>, Param3: ::windows::runtime::IntoParam<'a, LoggingOptions>>(&self, eventname: Param0, fields: Param1, level: LoggingLevel, options: Param3) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ILoggingTarget>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), eventname.into_param().abi(), fields.into_param().abi(), level, options.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn StartActivity<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, starteventname: Param0) -> ::windows::runtime::Result<LoggingActivity> {
        let this = &::windows::runtime::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), starteventname.into_param().abi(), &mut result__).from_abi::<LoggingActivity>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn StartActivityWithFields<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, LoggingFields>>(&self, starteventname: Param0, fields: Param1) -> ::windows::runtime::Result<LoggingActivity> {
        let this = &::windows::runtime::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), starteventname.into_param().abi(), fields.into_param().abi(), &mut result__).from_abi::<LoggingActivity>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn StartActivityWithFieldsAndLevel<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, LoggingFields>>(&self, starteventname: Param0, fields: Param1, level: LoggingLevel) -> ::windows::runtime::Result<LoggingActivity> {
        let this = &::windows::runtime::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), starteventname.into_param().abi(), fields.into_param().abi(), level, &mut result__).from_abi::<LoggingActivity>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn StartActivityWithFieldsAndOptions<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, LoggingFields>, Param3: ::windows::runtime::IntoParam<'a, LoggingOptions>>(&self, starteventname: Param0, fields: Param1, level: LoggingLevel, options: Param3) -> ::windows::runtime::Result<LoggingActivity> {
        let this = &::windows::runtime::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), starteventname.into_param().abi(), fields.into_param().abi(), level, options.into_param().abi(), &mut result__).from_abi::<LoggingActivity>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(name: Param0) -> ::windows::runtime::Result<LoggingChannel> {
        Self::ILoggingChannelFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<LoggingChannel>(result__)
        })
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn CreateWithOptions<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, LoggingChannelOptions>>(name: Param0, options: Param1) -> ::windows::runtime::Result<LoggingChannel> {
        Self::ILoggingChannelFactory2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), name.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<LoggingChannel>(result__)
        })
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn CreateWithOptionsAndId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, LoggingChannelOptions>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(name: Param0, options: Param1, id: Param2) -> ::windows::runtime::Result<LoggingChannel> {
        Self::ILoggingChannelFactory2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), name.into_param().abi(), options.into_param().abi(), id.into_param().abi(), &mut result__).from_abi::<LoggingChannel>(result__)
        })
    }
    pub fn ILoggingChannelFactory<R, F: FnOnce(&ILoggingChannelFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<LoggingChannel, ILoggingChannelFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    pub fn ILoggingChannelFactory2<R, F: FnOnce(&ILoggingChannelFactory2) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<LoggingChannel, ILoggingChannelFactory2> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LoggingChannel {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.LoggingChannel;{e9a50343-11d7-4f01-b5ca-cf495278c0a8})");
}
unsafe impl ::windows::runtime::Interface for LoggingChannel {
    type Vtable = ILoggingChannel_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xe9a50343_11d7_4f01_b5ca_cf495278c0a8);
}
impl ::windows::runtime::RuntimeName for LoggingChannel {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.LoggingChannel";
}
impl ::core::convert::From<LoggingChannel> for ::windows::runtime::IUnknown {
    fn from(value: LoggingChannel) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LoggingChannel> for ::windows::runtime::IUnknown {
    fn from(value: &LoggingChannel) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LoggingChannel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a LoggingChannel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LoggingChannel> for ::windows::runtime::IInspectable {
    fn from(value: LoggingChannel) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LoggingChannel> for ::windows::runtime::IInspectable {
    fn from(value: &LoggingChannel) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LoggingChannel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LoggingChannel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, ILoggingChannel> for LoggingChannel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILoggingChannel> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILoggingChannel> for &LoggingChannel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILoggingChannel> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<LoggingChannel> for super::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: LoggingChannel) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LoggingChannel> for super::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &LoggingChannel) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IClosable> for LoggingChannel {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IClosable> for &LoggingChannel {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IClosable> {
        ::core::convert::TryInto::<super::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::core::convert::TryFrom<LoggingChannel> for ILoggingTarget {
    type Error = ::windows::runtime::Error;
    fn try_from(value: LoggingChannel) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LoggingChannel> for ILoggingTarget {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &LoggingChannel) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILoggingTarget> for LoggingChannel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILoggingTarget> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILoggingTarget> for &LoggingChannel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILoggingTarget> {
        ::core::convert::TryInto::<ILoggingTarget>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for LoggingChannel {}
unsafe impl ::core::marker::Sync for LoggingChannel {}
#[doc = "*Required features: `Foundation_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LoggingChannelOptions(pub ::windows::runtime::IInspectable);
impl LoggingChannelOptions {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<LoggingChannelOptions, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Group(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn SetGroup<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(group: Param0) -> ::windows::runtime::Result<LoggingChannelOptions> {
        Self::ILoggingChannelOptionsFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), group.into_param().abi(), &mut result__).from_abi::<LoggingChannelOptions>(result__)
        })
    }
    pub fn ILoggingChannelOptionsFactory<R, F: FnOnce(&ILoggingChannelOptionsFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<LoggingChannelOptions, ILoggingChannelOptionsFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LoggingChannelOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.LoggingChannelOptions;{c3e847ff-0ebb-4a53-8c54-dec24926cb2c})");
}
unsafe impl ::windows::runtime::Interface for LoggingChannelOptions {
    type Vtable = ILoggingChannelOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc3e847ff_0ebb_4a53_8c54_dec24926cb2c);
}
impl ::windows::runtime::RuntimeName for LoggingChannelOptions {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.LoggingChannelOptions";
}
impl ::core::convert::From<LoggingChannelOptions> for ::windows::runtime::IUnknown {
    fn from(value: LoggingChannelOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LoggingChannelOptions> for ::windows::runtime::IUnknown {
    fn from(value: &LoggingChannelOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LoggingChannelOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a LoggingChannelOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LoggingChannelOptions> for ::windows::runtime::IInspectable {
    fn from(value: LoggingChannelOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LoggingChannelOptions> for ::windows::runtime::IInspectable {
    fn from(value: &LoggingChannelOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LoggingChannelOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LoggingChannelOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
unsafe impl ::windows::runtime::Abi for LoggingFieldFormat {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for LoggingFieldFormat {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Foundation.Diagnostics.LoggingFieldFormat;i4)");
}
impl ::windows::runtime::DefaultType for LoggingFieldFormat {
    type DefaultType = Self;
}
#[doc = "*Required features: `Foundation_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LoggingFields(pub ::windows::runtime::IInspectable);
impl LoggingFields {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<LoggingFields, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Clear(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn BeginStruct<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), name.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn BeginStructWithTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), name.into_param().abi(), tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn EndStruct(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddEmpty<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), name.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddEmptyWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), name.into_param().abi(), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddEmptyWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), name.into_param().abi(), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt8<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: u8) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), name.into_param().abi(), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt8WithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: u8, format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt8WithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: u8, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt8Array<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<u8 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt8ArrayWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<u8 as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt8ArrayWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<u8 as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt16<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: i16) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::core::mem::transmute_copy(this), name.into_param().abi(), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt16WithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: i16, format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).20)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt16WithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: i16, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt16Array<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<i16 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).22)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt16ArrayWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<i16 as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).23)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt16ArrayWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<i16 as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).24)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt16<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: u16) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).25)(::core::mem::transmute_copy(this), name.into_param().abi(), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt16WithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: u16, format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).26)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt16WithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: u16, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt16Array<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<u16 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).28)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt16ArrayWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<u16 as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).29)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt16ArrayWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<u16 as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).30)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt32<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).31)(::core::mem::transmute_copy(this), name.into_param().abi(), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt32WithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: i32, format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).32)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt32WithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: i32, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).33)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt32Array<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<i32 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).34)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt32ArrayWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<i32 as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).35)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt32ArrayWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<i32 as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).36)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt32<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).37)(::core::mem::transmute_copy(this), name.into_param().abi(), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt32WithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: u32, format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).38)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt32WithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: u32, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).39)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt32Array<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<u32 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).40)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt32ArrayWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<u32 as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).41)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt32ArrayWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<u32 as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).42)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt64<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: i64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).43)(::core::mem::transmute_copy(this), name.into_param().abi(), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt64WithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: i64, format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).44)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt64WithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: i64, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).45)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt64Array<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<i64 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).46)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt64ArrayWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<i64 as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).47)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt64ArrayWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<i64 as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).48)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt64<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: u64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).49)(::core::mem::transmute_copy(this), name.into_param().abi(), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt64WithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: u64, format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).50)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt64WithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: u64, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).51)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt64Array<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<u64 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).52)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt64ArrayWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<u64 as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).53)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt64ArrayWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<u64 as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).54)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddSingle<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).55)(::core::mem::transmute_copy(this), name.into_param().abi(), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddSingleWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: f32, format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).56)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddSingleWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: f32, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).57)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddSingleArray<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<f32 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).58)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddSingleArrayWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<f32 as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).59)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddSingleArrayWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<f32 as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).60)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddDouble<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).61)(::core::mem::transmute_copy(this), name.into_param().abi(), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddDoubleWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: f64, format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).62)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddDoubleWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: f64, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).63)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddDoubleArray<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<f64 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).64)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddDoubleArrayWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<f64 as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).65)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddDoubleArrayWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<f64 as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).66)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddChar16<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: u16) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).67)(::core::mem::transmute_copy(this), name.into_param().abi(), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddChar16WithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: u16, format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).68)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddChar16WithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: u16, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).69)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddChar16Array<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<u16 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).70)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddChar16ArrayWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<u16 as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).71)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddChar16ArrayWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<u16 as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).72)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddBoolean<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).73)(::core::mem::transmute_copy(this), name.into_param().abi(), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddBooleanWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: bool, format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).74)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddBooleanWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: bool, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).75)(::core::mem::transmute_copy(this), name.into_param().abi(), value, format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddBooleanArray<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<bool as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).76)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddBooleanArrayWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<bool as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).77)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddBooleanArrayWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<bool as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).78)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddString<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).79)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddStringWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).80)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddStringWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).81)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddStringArray<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<::windows::runtime::HSTRING as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).82)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddStringArrayWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<::windows::runtime::HSTRING as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).83)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddStringArrayWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<::windows::runtime::HSTRING as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).84)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddGuid<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, name: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).85)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddGuidWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).86)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddGuidWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).87)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddGuidArray<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<::windows::runtime::GUID as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).88)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddGuidArrayWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<::windows::runtime::GUID as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).89)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddGuidArrayWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<::windows::runtime::GUID as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).90)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddDateTime<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::DateTime>>(&self, name: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).91)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddDateTimeWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::DateTime>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).92)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddDateTimeWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::DateTime>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).93)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddDateTimeArray<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<super::DateTime as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).94)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddDateTimeArrayWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<super::DateTime as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).95)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddDateTimeArrayWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<super::DateTime as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).96)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddTimeSpan<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::TimeSpan>>(&self, name: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).97)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddTimeSpanWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::TimeSpan>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).98)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddTimeSpanWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::TimeSpan>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).99)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddTimeSpanArray<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<super::TimeSpan as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).100)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddTimeSpanArrayWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<super::TimeSpan as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).101)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddTimeSpanArrayWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<super::TimeSpan as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).102)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddPoint<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::Point>>(&self, name: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).103)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddPointWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::Point>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).104)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddPointWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::Point>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).105)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddPointArray<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<super::Point as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).106)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddPointArrayWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<super::Point as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).107)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddPointArrayWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<super::Point as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).108)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddSize<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::Size>>(&self, name: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).109)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddSizeWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::Size>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).110)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddSizeWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::Size>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).111)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddSizeArray<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<super::Size as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).112)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddSizeArrayWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<super::Size as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).113)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddSizeArrayWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<super::Size as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).114)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddRect<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::Rect>>(&self, name: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).115)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddRectWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::Rect>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).116)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddRectWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::Rect>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).117)(::core::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddRectArray<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<super::Rect as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).118)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddRectArrayWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<super::Rect as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).119)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddRectArrayWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<super::Rect as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).120)(::core::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::core::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LoggingFields {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.LoggingFields;{d7f6b7af-762d-4579-83bd-52c23bc333bc})");
}
unsafe impl ::windows::runtime::Interface for LoggingFields {
    type Vtable = ILoggingFields_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xd7f6b7af_762d_4579_83bd_52c23bc333bc);
}
impl ::windows::runtime::RuntimeName for LoggingFields {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.LoggingFields";
}
impl ::core::convert::From<LoggingFields> for ::windows::runtime::IUnknown {
    fn from(value: LoggingFields) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LoggingFields> for ::windows::runtime::IUnknown {
    fn from(value: &LoggingFields) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LoggingFields {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a LoggingFields {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LoggingFields> for ::windows::runtime::IInspectable {
    fn from(value: LoggingFields) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LoggingFields> for ::windows::runtime::IInspectable {
    fn from(value: &LoggingFields) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LoggingFields {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LoggingFields {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
unsafe impl ::windows::runtime::Abi for LoggingLevel {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for LoggingLevel {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Foundation.Diagnostics.LoggingLevel;i4)");
}
impl ::windows::runtime::DefaultType for LoggingLevel {
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
unsafe impl ::windows::runtime::Abi for LoggingOpcode {
    type Abi = Self;
}
unsafe impl ::windows::runtime::RuntimeType for LoggingOpcode {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"enum(Windows.Foundation.Diagnostics.LoggingOpcode;i4)");
}
impl ::windows::runtime::DefaultType for LoggingOpcode {
    type DefaultType = Self;
}
#[doc = "*Required features: `Foundation_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LoggingOptions(pub ::windows::runtime::IInspectable);
impl LoggingOptions {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<LoggingOptions, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Keywords(&self) -> ::windows::runtime::Result<i64> {
        let this = self;
        unsafe {
            let mut result__: i64 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn SetKeywords(&self, value: i64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Tags(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn SetTags(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Task(&self) -> ::windows::runtime::Result<i16> {
        let this = self;
        unsafe {
            let mut result__: i16 = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), &mut result__).from_abi::<i16>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn SetTask(&self, value: i16) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Opcode(&self) -> ::windows::runtime::Result<LoggingOpcode> {
        let this = self;
        unsafe {
            let mut result__: LoggingOpcode = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::core::mem::transmute_copy(this), &mut result__).from_abi::<LoggingOpcode>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn SetOpcode(&self, value: LoggingOpcode) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn ActivityId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn SetActivityId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn RelatedActivityId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn SetRelatedActivityId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::core::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn CreateWithKeywords(keywords: i64) -> ::windows::runtime::Result<LoggingOptions> {
        Self::ILoggingOptionsFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), keywords, &mut result__).from_abi::<LoggingOptions>(result__)
        })
    }
    pub fn ILoggingOptionsFactory<R, F: FnOnce(&ILoggingOptionsFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<LoggingOptions, ILoggingOptionsFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LoggingOptions {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.LoggingOptions;{90bc7850-0192-4f5d-ac26-006adaca12d8})");
}
unsafe impl ::windows::runtime::Interface for LoggingOptions {
    type Vtable = ILoggingOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x90bc7850_0192_4f5d_ac26_006adaca12d8);
}
impl ::windows::runtime::RuntimeName for LoggingOptions {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.LoggingOptions";
}
impl ::core::convert::From<LoggingOptions> for ::windows::runtime::IUnknown {
    fn from(value: LoggingOptions) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LoggingOptions> for ::windows::runtime::IUnknown {
    fn from(value: &LoggingOptions) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LoggingOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a LoggingOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LoggingOptions> for ::windows::runtime::IInspectable {
    fn from(value: LoggingOptions) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LoggingOptions> for ::windows::runtime::IInspectable {
    fn from(value: &LoggingOptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LoggingOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LoggingOptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for LoggingOptions {}
unsafe impl ::core::marker::Sync for LoggingOptions {}
#[doc = "*Required features: `Foundation_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct LoggingSession(pub ::windows::runtime::IInspectable);
impl LoggingSession {
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::core::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Storage")]
    #[doc = "*Required features: `Foundation_Diagnostics`, `Storage`*"]
    pub fn SaveToFileAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFolder>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, folder: Param0, filename: Param1) -> ::windows::runtime::Result<super::IAsyncOperation<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), folder.into_param().abi(), filename.into_param().abi(), &mut result__).from_abi::<super::IAsyncOperation<super::super::Storage::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddLoggingChannel<'a, Param0: ::windows::runtime::IntoParam<'a, ILoggingChannel>>(&self, loggingchannel: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::core::mem::transmute_copy(this), loggingchannel.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddLoggingChannelWithLevel<'a, Param0: ::windows::runtime::IntoParam<'a, ILoggingChannel>>(&self, loggingchannel: Param0, maxlevel: LoggingLevel) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::core::mem::transmute_copy(this), loggingchannel.into_param().abi(), maxlevel).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn RemoveLoggingChannel<'a, Param0: ::windows::runtime::IntoParam<'a, ILoggingChannel>>(&self, loggingchannel: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::core::mem::transmute_copy(this), loggingchannel.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(name: Param0) -> ::windows::runtime::Result<LoggingSession> {
        Self::ILoggingSessionFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<LoggingSession>(result__)
        })
    }
    pub fn ILoggingSessionFactory<R, F: FnOnce(&ILoggingSessionFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<LoggingSession, ILoggingSessionFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LoggingSession {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.LoggingSession;{6221f306-9380-4ad7-baf5-41ea9310d768})");
}
unsafe impl ::windows::runtime::Interface for LoggingSession {
    type Vtable = ILoggingSession_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x6221f306_9380_4ad7_baf5_41ea9310d768);
}
impl ::windows::runtime::RuntimeName for LoggingSession {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.LoggingSession";
}
impl ::core::convert::From<LoggingSession> for ::windows::runtime::IUnknown {
    fn from(value: LoggingSession) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&LoggingSession> for ::windows::runtime::IUnknown {
    fn from(value: &LoggingSession) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for LoggingSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a LoggingSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<LoggingSession> for ::windows::runtime::IInspectable {
    fn from(value: LoggingSession) -> Self {
        value.0
    }
}
impl ::core::convert::From<&LoggingSession> for ::windows::runtime::IInspectable {
    fn from(value: &LoggingSession) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for LoggingSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a LoggingSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, ILoggingSession> for LoggingSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILoggingSession> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILoggingSession> for &LoggingSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILoggingSession> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::TryFrom<LoggingSession> for super::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: LoggingSession) -> ::windows::runtime::Result<Self> {
        ::core::convert::TryFrom::try_from(&value)
    }
}
impl ::core::convert::TryFrom<&LoggingSession> for super::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: &LoggingSession) -> ::windows::runtime::Result<Self> {
        ::windows::runtime::Interface::cast(value)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IClosable> for LoggingSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IClosable> {
        ::windows::runtime::IntoParam::into_param(&self)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, super::IClosable> for &LoggingSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::IClosable> {
        ::core::convert::TryInto::<super::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::core::marker::Send for LoggingSession {}
unsafe impl ::core::marker::Sync for LoggingSession {}
#[doc = "*Required features: `Foundation_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct RuntimeBrokerErrorSettings(pub ::windows::runtime::IInspectable);
impl RuntimeBrokerErrorSettings {
    pub fn new() -> ::windows::runtime::Result<Self> {
        Self::IActivationFactory(|f| f.activate_instance::<Self>())
    }
    fn IActivationFactory<R, F: FnOnce(&::windows::runtime::IActivationFactory) -> ::windows::runtime::Result<R>>(callback: F) -> ::windows::runtime::Result<R> {
        static mut SHARED: ::windows::runtime::FactoryCache<RuntimeBrokerErrorSettings, ::windows::runtime::IActivationFactory> = ::windows::runtime::FactoryCache::new();
        unsafe { SHARED.call(callback) }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn SetErrorOptions(&self, value: ErrorOptions) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn GetErrorOptions(&self) -> ::windows::runtime::Result<ErrorOptions> {
        let this = self;
        unsafe {
            let mut result__: ErrorOptions = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<ErrorOptions>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RuntimeBrokerErrorSettings {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.RuntimeBrokerErrorSettings;{16369792-b03e-4ba1-8bb8-d28f4ab4d2c0})");
}
unsafe impl ::windows::runtime::Interface for RuntimeBrokerErrorSettings {
    type Vtable = IErrorReportingSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x16369792_b03e_4ba1_8bb8_d28f4ab4d2c0);
}
impl ::windows::runtime::RuntimeName for RuntimeBrokerErrorSettings {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.RuntimeBrokerErrorSettings";
}
impl ::core::convert::From<RuntimeBrokerErrorSettings> for ::windows::runtime::IUnknown {
    fn from(value: RuntimeBrokerErrorSettings) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&RuntimeBrokerErrorSettings> for ::windows::runtime::IUnknown {
    fn from(value: &RuntimeBrokerErrorSettings) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for RuntimeBrokerErrorSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a RuntimeBrokerErrorSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<RuntimeBrokerErrorSettings> for ::windows::runtime::IInspectable {
    fn from(value: RuntimeBrokerErrorSettings) -> Self {
        value.0
    }
}
impl ::core::convert::From<&RuntimeBrokerErrorSettings> for ::windows::runtime::IInspectable {
    fn from(value: &RuntimeBrokerErrorSettings) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for RuntimeBrokerErrorSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a RuntimeBrokerErrorSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
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
impl<'a> ::windows::runtime::IntoParam<'a, IErrorReportingSettings> for RuntimeBrokerErrorSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, IErrorReportingSettings> {
        ::windows::runtime::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IErrorReportingSettings> for &RuntimeBrokerErrorSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, IErrorReportingSettings> {
        ::windows::runtime::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
unsafe impl ::core::marker::Send for RuntimeBrokerErrorSettings {}
unsafe impl ::core::marker::Sync for RuntimeBrokerErrorSettings {}
#[doc = "*Required features: `Foundation_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct TracingStatusChangedEventArgs(pub ::windows::runtime::IInspectable);
impl TracingStatusChangedEventArgs {
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Enabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::core::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn TraceLevel(&self) -> ::windows::runtime::Result<CausalityTraceLevel> {
        let this = self;
        unsafe {
            let mut result__: CausalityTraceLevel = ::core::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::core::mem::transmute_copy(this), &mut result__).from_abi::<CausalityTraceLevel>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TracingStatusChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.TracingStatusChangedEventArgs;{410b7711-ff3b-477f-9c9a-d2efda302dc3})");
}
unsafe impl ::windows::runtime::Interface for TracingStatusChangedEventArgs {
    type Vtable = ITracingStatusChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x410b7711_ff3b_477f_9c9a_d2efda302dc3);
}
impl ::windows::runtime::RuntimeName for TracingStatusChangedEventArgs {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.TracingStatusChangedEventArgs";
}
impl ::core::convert::From<TracingStatusChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: TracingStatusChangedEventArgs) -> Self {
        value.0 .0
    }
}
impl ::core::convert::From<&TracingStatusChangedEventArgs> for ::windows::runtime::IUnknown {
    fn from(value: &TracingStatusChangedEventArgs) -> Self {
        value.0 .0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for TracingStatusChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0 .0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a TracingStatusChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0 .0)
    }
}
impl ::core::convert::From<TracingStatusChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: TracingStatusChangedEventArgs) -> Self {
        value.0
    }
}
impl ::core::convert::From<&TracingStatusChangedEventArgs> for ::windows::runtime::IInspectable {
    fn from(value: &TracingStatusChangedEventArgs) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for TracingStatusChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IInspectable> for &'a TracingStatusChangedEventArgs {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IInspectable> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
unsafe impl ::core::marker::Send for TracingStatusChangedEventArgs {}
unsafe impl ::core::marker::Sync for TracingStatusChangedEventArgs {}
