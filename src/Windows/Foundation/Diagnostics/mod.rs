#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Foundation_Diagnostics`*"]
pub struct AsyncCausalityTracer {}
impl AsyncCausalityTracer {
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn TraceOperationCreation<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>, Param4: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: Param2, operationid: u64, operationname: Param4, relatedcontext: u64) -> ::windows::runtime::Result<()> {
        Self::IAsyncCausalityTracerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), tracelevel, source, platformid.into_param().abi(), operationid, operationname.into_param().abi(), relatedcontext).ok() })
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn TraceOperationCompletion<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: Param2, operationid: u64, status: super::AsyncStatus) -> ::windows::runtime::Result<()> {
        Self::IAsyncCausalityTracerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), tracelevel, source, platformid.into_param().abi(), operationid, status).ok() })
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn TraceOperationRelation<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: Param2, operationid: u64, relation: CausalityRelation) -> ::windows::runtime::Result<()> {
        Self::IAsyncCausalityTracerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), tracelevel, source, platformid.into_param().abi(), operationid, relation).ok() })
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn TraceSynchronousWorkStart<'a, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: Param2, operationid: u64, work: CausalitySynchronousWork) -> ::windows::runtime::Result<()> {
        Self::IAsyncCausalityTracerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), tracelevel, source, platformid.into_param().abi(), operationid, work).ok() })
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn TraceSynchronousWorkCompletion(tracelevel: CausalityTraceLevel, source: CausalitySource, work: CausalitySynchronousWork) -> ::windows::runtime::Result<()> {
        Self::IAsyncCausalityTracerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), tracelevel, source, work).ok() })
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn TracingStatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::EventHandler<TracingStatusChangedEventArgs>>>(handler: Param0) -> ::windows::runtime::Result<super::EventRegistrationToken> {
        Self::IAsyncCausalityTracerStatics(|this| unsafe {
            let mut result__: super::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::EventRegistrationToken>(result__)
        })
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn RemoveTracingStatusChanged<'a, Param0: ::windows::runtime::IntoParam<'a, super::EventRegistrationToken>>(cookie: Param0) -> ::windows::runtime::Result<()> {
        Self::IAsyncCausalityTracerStatics(|this| unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), cookie.into_param().abi()).ok() })
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CausalityRelation(pub i32);
impl CausalityRelation {
    pub const AssignDelegate: CausalityRelation = CausalityRelation(0i32);
    pub const Join: CausalityRelation = CausalityRelation(1i32);
    pub const Choice: CausalityRelation = CausalityRelation(2i32);
    pub const Cancel: CausalityRelation = CausalityRelation(3i32);
    pub const Error: CausalityRelation = CausalityRelation(4i32);
}
impl ::std::convert::From<i32> for CausalityRelation {
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CausalitySource(pub i32);
impl CausalitySource {
    pub const Application: CausalitySource = CausalitySource(0i32);
    pub const Library: CausalitySource = CausalitySource(1i32);
    pub const System: CausalitySource = CausalitySource(2i32);
}
impl ::std::convert::From<i32> for CausalitySource {
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CausalitySynchronousWork(pub i32);
impl CausalitySynchronousWork {
    pub const CompletionNotification: CausalitySynchronousWork = CausalitySynchronousWork(0i32);
    pub const ProgressNotification: CausalitySynchronousWork = CausalitySynchronousWork(1i32);
    pub const Execution: CausalitySynchronousWork = CausalitySynchronousWork(2i32);
}
impl ::std::convert::From<i32> for CausalitySynchronousWork {
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CausalityTraceLevel(pub i32);
impl CausalityTraceLevel {
    pub const Required: CausalityTraceLevel = CausalityTraceLevel(0i32);
    pub const Important: CausalityTraceLevel = CausalityTraceLevel(1i32);
    pub const Verbose: CausalityTraceLevel = CausalityTraceLevel(2i32);
}
impl ::std::convert::From<i32> for CausalityTraceLevel {
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct ErrorDetails(::windows::runtime::IInspectable);
impl ErrorDetails {
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Description(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LongDescription(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn HelpUri(&self) -> ::windows::runtime::Result<super::Uri> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::Uri>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn CreateFromHResultAsync(errorcode: i32) -> ::windows::runtime::Result<super::IAsyncOperation<ErrorDetails>> {
        Self::IErrorDetailsStatics(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), errorcode, &mut result__).from_abi::<super::IAsyncOperation<ErrorDetails>>(result__)
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(931969793, 11465, 17039, [140, 85, 44, 153, 13, 70, 62, 143]);
}
impl ::windows::runtime::RuntimeName for ErrorDetails {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.ErrorDetails";
}
unsafe impl ::std::marker::Send for ErrorDetails {}
unsafe impl ::std::marker::Sync for ErrorDetails {}
#[doc = "*Required features: `Foundation_Diagnostics`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ErrorOptions(pub u32);
impl ErrorOptions {
    pub const None: ErrorOptions = ErrorOptions(0u32);
    pub const SuppressExceptions: ErrorOptions = ErrorOptions(1u32);
    pub const ForceExceptions: ErrorOptions = ErrorOptions(2u32);
    pub const UseSetErrorInfo: ErrorOptions = ErrorOptions(4u32);
    pub const SuppressSetErrorInfo: ErrorOptions = ErrorOptions(8u32);
}
impl ::std::convert::From<u32> for ErrorOptions {
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
impl ::std::ops::BitOr for ErrorOptions {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for ErrorOptions {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for ErrorOptions {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for ErrorOptions {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for ErrorOptions {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Foundation_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct FileLoggingSession(::windows::runtime::IInspectable);
impl FileLoggingSession {
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddLoggingChannel<'a, Param0: ::windows::runtime::IntoParam<'a, ILoggingChannel>>(&self, loggingchannel: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), loggingchannel.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddLoggingChannelWithLevel<'a, Param0: ::windows::runtime::IntoParam<'a, ILoggingChannel>>(&self, loggingchannel: Param0, maxlevel: LoggingLevel) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), loggingchannel.into_param().abi(), maxlevel).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn RemoveLoggingChannel<'a, Param0: ::windows::runtime::IntoParam<'a, ILoggingChannel>>(&self, loggingchannel: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), loggingchannel.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage")]
    #[doc = "*Required features: `Foundation_Diagnostics`, `Storage`*"]
    pub fn CloseAndSaveToFileAsync(&self) -> ::windows::runtime::Result<super::IAsyncOperation<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::IAsyncOperation<super::super::Storage::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogFileGenerated<'a, Param0: ::windows::runtime::IntoParam<'a, super::TypedEventHandler<IFileLoggingSession, LogFileGeneratedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn RemoveLogFileGenerated<'a, Param0: ::windows::runtime::IntoParam<'a, super::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(name: Param0) -> ::windows::runtime::Result<FileLoggingSession> {
        Self::IFileLoggingSessionFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<FileLoggingSession>(result__)
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(617038358, 65234, 16460, [137, 95, 31, 150, 153, 203, 2, 247]);
}
impl ::windows::runtime::RuntimeName for FileLoggingSession {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.FileLoggingSession";
}
impl ::std::convert::From<FileLoggingSession> for IFileLoggingSession {
    fn from(value: FileLoggingSession) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&FileLoggingSession> for IFileLoggingSession {
    fn from(value: &FileLoggingSession) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFileLoggingSession> for FileLoggingSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFileLoggingSession> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFileLoggingSession>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IFileLoggingSession> for &FileLoggingSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, IFileLoggingSession> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IFileLoggingSession>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::TryFrom<FileLoggingSession> for super::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: FileLoggingSession) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&FileLoggingSession> for super::IClosable {
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
        ::std::convert::TryInto::<super::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for FileLoggingSession {}
unsafe impl ::std::marker::Sync for FileLoggingSession {}
#[repr(transparent)]
#[doc(hidden)]
pub struct IAsyncCausalityTracerStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IAsyncCausalityTracerStatics {
    type Vtable = IAsyncCausalityTracerStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1350896422, 9854, 17691, [168, 144, 171, 106, 55, 2, 69, 238]);
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: ::windows::runtime::GUID, operationid: u64, operationname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, relatedcontext: u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: ::windows::runtime::GUID, operationid: u64, status: super::AsyncStatus) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: ::windows::runtime::GUID, operationid: u64, relation: CausalityRelation) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: ::windows::runtime::GUID, operationid: u64, work: CausalitySynchronousWork) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, tracelevel: CausalityTraceLevel, source: CausalitySource, work: CausalitySynchronousWork) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cookie: super::EventRegistrationToken) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IErrorDetails(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IErrorDetails {
    type Vtable = IErrorDetails_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(931969793, 11465, 17039, [140, 85, 44, 153, 13, 70, 62, 143]);
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct IErrorDetailsStatics(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IErrorDetailsStatics {
    type Vtable = IErrorDetailsStatics_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3077584720, 2845, 18120, [170, 14, 75, 129, 120, 228, 252, 233]);
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Foundation_Diagnostics`*"]
pub struct IErrorReportingSettings(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IErrorReportingSettings {
    type Vtable = IErrorReportingSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(372676498, 45118, 19361, [139, 184, 210, 143, 74, 180, 210, 192]);
}
impl IErrorReportingSettings {
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn SetErrorOptions(&self, value: ErrorOptions) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn GetErrorOptions(&self) -> ::windows::runtime::Result<ErrorOptions> {
        let this = self;
        unsafe {
            let mut result__: ErrorOptions = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ErrorOptions>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IErrorReportingSettings {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{16369792-b03e-4ba1-8bb8-d28f4ab4d2c0}");
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Foundation_Diagnostics`*"]
pub struct IFileLoggingSession(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFileLoggingSession {
    type Vtable = IFileLoggingSession_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(617038358, 65234, 16460, [137, 95, 31, 150, 153, 203, 2, 247]);
}
impl IFileLoggingSession {
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddLoggingChannel<'a, Param0: ::windows::runtime::IntoParam<'a, ILoggingChannel>>(&self, loggingchannel: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), loggingchannel.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddLoggingChannelWithLevel<'a, Param0: ::windows::runtime::IntoParam<'a, ILoggingChannel>>(&self, loggingchannel: Param0, maxlevel: LoggingLevel) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), loggingchannel.into_param().abi(), maxlevel).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn RemoveLoggingChannel<'a, Param0: ::windows::runtime::IntoParam<'a, ILoggingChannel>>(&self, loggingchannel: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), loggingchannel.into_param().abi()).ok() }
    }
    #[cfg(feature = "Storage")]
    #[doc = "*Required features: `Foundation_Diagnostics`, `Storage`*"]
    pub fn CloseAndSaveToFileAsync(&self) -> ::windows::runtime::Result<super::IAsyncOperation<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::IAsyncOperation<super::super::Storage::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogFileGenerated<'a, Param0: ::windows::runtime::IntoParam<'a, super::TypedEventHandler<IFileLoggingSession, LogFileGeneratedEventArgs>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn RemoveLogFileGenerated<'a, Param0: ::windows::runtime::IntoParam<'a, super::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for IFileLoggingSession {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{24c74216-fed2-404c-895f-1f9699cb02f7}");
}
impl ::std::convert::TryFrom<IFileLoggingSession> for super::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: IFileLoggingSession) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&IFileLoggingSession> for super::IClosable {
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
        ::std::convert::TryInto::<super::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
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
pub struct IFileLoggingSessionFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for IFileLoggingSessionFactory {
    type Vtable = IFileLoggingSessionFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4003499470, 33863, 19882, [145, 51, 18, 235, 70, 246, 151, 212]);
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILogFileGeneratedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILogFileGeneratedEventArgs {
    type Vtable = ILogFileGeneratedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(647927663, 3384, 19482, [181, 63, 179, 149, 216, 129, 223, 132]);
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
pub struct ILoggingActivity(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILoggingActivity {
    type Vtable = ILoggingActivity_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3154323777, 46950, 19637, [152, 72, 151, 172, 107, 166, 214, 12]);
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILoggingActivity2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILoggingActivity2 {
    type Vtable = ILoggingActivity2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(650287112, 25378, 17770, [175, 130, 128, 200, 100, 47, 23, 139]);
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, stopeventname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, stopeventname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, fields: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, stopeventname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, fields: ::windows::runtime::RawPtr, options: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILoggingActivityFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILoggingActivityFactory {
    type Vtable = ILoggingActivityFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1798550659, 57610, 19544, [151, 213, 16, 251, 69, 16, 116, 251]);
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, activityname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, loggingchannel: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, activityname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, loggingchannel: ::windows::runtime::RawPtr, level: LoggingLevel, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Foundation_Diagnostics`*"]
pub struct ILoggingChannel(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILoggingChannel {
    type Vtable = ILoggingChannel_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3919905603, 4567, 20225, [181, 202, 207, 73, 82, 120, 192, 168]);
}
impl ILoggingChannel {
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Enabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Level(&self) -> ::windows::runtime::Result<LoggingLevel> {
        let this = self;
        unsafe {
            let mut result__: LoggingLevel = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LoggingLevel>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogMessage<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, eventstring: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), eventstring.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogMessageWithLevel<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, eventstring: Param0, level: LoggingLevel) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), eventstring.into_param().abi(), level).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogValuePair<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value1: Param0, value2: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value1.into_param().abi(), value2).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogValuePairWithLevel<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value1: Param0, value2: i32, level: LoggingLevel) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value1.into_param().abi(), value2, level).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LoggingEnabled<'a, Param0: ::windows::runtime::IntoParam<'a, super::TypedEventHandler<ILoggingChannel, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn RemoveLoggingEnabled<'a, Param0: ::windows::runtime::IntoParam<'a, super::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ILoggingChannel {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{e9a50343-11d7-4f01-b5ca-cf495278c0a8}");
}
impl ::std::convert::TryFrom<ILoggingChannel> for super::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ILoggingChannel) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&ILoggingChannel> for super::IClosable {
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
        ::std::convert::TryInto::<super::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut LoggingLevel) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventstring: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventstring: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, level: LoggingLevel) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value1: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value2: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, value1: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value2: i32, level: LoggingLevel) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, handler: ::windows::runtime::RawPtr, result__: *mut super::EventRegistrationToken) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, token: super::EventRegistrationToken) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILoggingChannel2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILoggingChannel2 {
    type Vtable = ILoggingChannel2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2672573683, 2988, 17829, [158, 51, 186, 243, 243, 162, 70, 165]);
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
pub struct ILoggingChannelFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILoggingChannelFactory {
    type Vtable = ILoggingChannelFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1323064220, 44928, 19099, [176, 220, 57, 143, 154, 229, 32, 123]);
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILoggingChannelFactory2(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILoggingChannelFactory2 {
    type Vtable = ILoggingChannelFactory2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1282340317, 15143, 19913, [153, 240, 41, 156, 110, 70, 3, 161]);
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, options: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, options: ::windows::runtime::RawPtr, id: ::windows::runtime::GUID, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILoggingChannelOptions(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILoggingChannelOptions {
    type Vtable = ILoggingChannelOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3286779903, 3771, 19027, [140, 84, 222, 194, 73, 38, 203, 44]);
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
pub struct ILoggingChannelOptionsFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILoggingChannelOptionsFactory {
    type Vtable = ILoggingChannelOptionsFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2838581722, 32687, 16785, [135, 85, 94, 134, 220, 101, 216, 150]);
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
pub struct ILoggingFields(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILoggingFields {
    type Vtable = ILoggingFields_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3623270319, 30253, 17785, [131, 189, 82, 194, 59, 195, 51, 188]);
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: u8, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: u8, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const u8, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const u8, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: i16, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: i16, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const i16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const i16, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const i16, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: u16, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: u16, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const u16, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const u16, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: i32, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: i32, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const i32, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const i32, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: u32, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: u32, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const u32, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const u32, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: i64, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: i64, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const i64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const i64, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const i64, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: u64, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: u64, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const u64, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const u64, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: f32, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: f32, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const f32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const f32, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const f32, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: f64, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: f64, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const f64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const f64, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const f64, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: u16, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: u16, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const u16) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const u16, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const u16, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: bool, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: bool, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const bool) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const bool, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const bool, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: ::windows::runtime::GUID, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: ::windows::runtime::GUID, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const ::windows::runtime::GUID, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const ::windows::runtime::GUID, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: super::DateTime) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: super::DateTime, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: super::DateTime, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const super::DateTime) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const super::DateTime, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const super::DateTime, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: super::TimeSpan) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: super::TimeSpan, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: super::TimeSpan, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const super::TimeSpan) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const super::TimeSpan, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const super::TimeSpan, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: super::Point) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: super::Point, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: super::Point, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const super::Point) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const super::Point, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const super::Point, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: super::Size) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: super::Size, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: super::Size, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const super::Size) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const super::Size, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const super::Size, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: super::Rect) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: super::Rect, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value: super::Rect, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const super::Rect) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const super::Rect, format: LoggingFieldFormat) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, value_array_size: u32, value: *const super::Rect, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILoggingOptions(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILoggingOptions {
    type Vtable = ILoggingOptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2428270672, 402, 20317, [172, 38, 0, 106, 218, 202, 18, 216]);
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
pub struct ILoggingOptionsFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILoggingOptionsFactory {
    type Vtable = ILoggingOptionsFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3608397515, 39083, 17995, [159, 34, 163, 38, 132, 120, 54, 138]);
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Foundation_Diagnostics`*"]
pub struct ILoggingSession(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILoggingSession {
    type Vtable = ILoggingSession_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1646392070, 37760, 19159, [186, 245, 65, 234, 147, 16, 215, 104]);
}
impl ILoggingSession {
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Storage")]
    #[doc = "*Required features: `Foundation_Diagnostics`, `Storage`*"]
    pub fn SaveToFileAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFolder>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, folder: Param0, filename: Param1) -> ::windows::runtime::Result<super::IAsyncOperation<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), folder.into_param().abi(), filename.into_param().abi(), &mut result__).from_abi::<super::IAsyncOperation<super::super::Storage::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddLoggingChannel<'a, Param0: ::windows::runtime::IntoParam<'a, ILoggingChannel>>(&self, loggingchannel: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), loggingchannel.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddLoggingChannelWithLevel<'a, Param0: ::windows::runtime::IntoParam<'a, ILoggingChannel>>(&self, loggingchannel: Param0, maxlevel: LoggingLevel) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), loggingchannel.into_param().abi(), maxlevel).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn RemoveLoggingChannel<'a, Param0: ::windows::runtime::IntoParam<'a, ILoggingChannel>>(&self, loggingchannel: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), loggingchannel.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ILoggingSession {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{6221f306-9380-4ad7-baf5-41ea9310d768}");
}
impl ::std::convert::TryFrom<ILoggingSession> for super::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: ILoggingSession) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&ILoggingSession> for super::IClosable {
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
        ::std::convert::TryInto::<super::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, result__: *mut ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Storage")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, folder: ::windows::runtime::RawPtr, filename: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Storage"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, loggingchannel: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, loggingchannel: ::windows::runtime::RawPtr, maxlevel: LoggingLevel) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, loggingchannel: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ILoggingSessionFactory(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILoggingSessionFactory {
    type Vtable = ILoggingSessionFactory_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1318289125, 22781, 17888, [140, 47, 161, 50, 239, 249, 92, 30]);
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, name: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
#[doc = "*Required features: `Foundation_Diagnostics`*"]
pub struct ILoggingTarget(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ILoggingTarget {
    type Vtable = ILoggingTarget_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1710320693, 58248, 20006, [177, 122, 245, 28, 211, 168, 57, 22]);
}
impl ILoggingTarget {
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn IsEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn IsEnabledWithLevel(&self, level: LoggingLevel) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), level, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn IsEnabledWithLevelAndKeywords(&self, level: LoggingLevel, keywords: i64) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), level, keywords, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogEvent<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, eventname: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), eventname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogEventWithFields<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, LoggingFields>>(&self, eventname: Param0, fields: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), eventname.into_param().abi(), fields.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogEventWithFieldsAndLevel<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, LoggingFields>>(&self, eventname: Param0, fields: Param1, level: LoggingLevel) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), eventname.into_param().abi(), fields.into_param().abi(), level).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogEventWithFieldsAndOptions<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, LoggingFields>, Param3: ::windows::runtime::IntoParam<'a, LoggingOptions>>(&self, eventname: Param0, fields: Param1, level: LoggingLevel, options: Param3) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), eventname.into_param().abi(), fields.into_param().abi(), level, options.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn StartActivity<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, starteventname: Param0) -> ::windows::runtime::Result<LoggingActivity> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), starteventname.into_param().abi(), &mut result__).from_abi::<LoggingActivity>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn StartActivityWithFields<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, LoggingFields>>(&self, starteventname: Param0, fields: Param1) -> ::windows::runtime::Result<LoggingActivity> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), starteventname.into_param().abi(), fields.into_param().abi(), &mut result__).from_abi::<LoggingActivity>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn StartActivityWithFieldsAndLevel<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, LoggingFields>>(&self, starteventname: Param0, fields: Param1, level: LoggingLevel) -> ::windows::runtime::Result<LoggingActivity> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), starteventname.into_param().abi(), fields.into_param().abi(), level, &mut result__).from_abi::<LoggingActivity>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn StartActivityWithFieldsAndOptions<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, LoggingFields>, Param3: ::windows::runtime::IntoParam<'a, LoggingOptions>>(&self, starteventname: Param0, fields: Param1, level: LoggingLevel, options: Param3) -> ::windows::runtime::Result<LoggingActivity> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), starteventname.into_param().abi(), fields.into_param().abi(), level, options.into_param().abi(), &mut result__).from_abi::<LoggingActivity>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for ILoggingTarget {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"{65f16c35-e388-4e26-b17a-f51cd3a83916}");
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
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, fields: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, fields: ::windows::runtime::RawPtr, level: LoggingLevel) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, eventname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, fields: ::windows::runtime::RawPtr, level: LoggingLevel, options: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, starteventname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, starteventname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, fields: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, starteventname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, fields: ::windows::runtime::RawPtr, level: LoggingLevel, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, starteventname: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING>, fields: ::windows::runtime::RawPtr, level: LoggingLevel, options: ::windows::runtime::RawPtr, result__: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[repr(transparent)]
#[doc(hidden)]
pub struct ITracingStatusChangedEventArgs(::windows::runtime::IInspectable);
unsafe impl ::windows::runtime::Interface for ITracingStatusChangedEventArgs {
    type Vtable = ITracingStatusChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1091270417, 65339, 18303, [156, 154, 210, 239, 218, 48, 45, 195]);
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct LogFileGeneratedEventArgs(::windows::runtime::IInspectable);
impl LogFileGeneratedEventArgs {
    #[cfg(feature = "Storage")]
    #[doc = "*Required features: `Foundation_Diagnostics`, `Storage`*"]
    pub fn File(&self) -> ::windows::runtime::Result<super::super::Storage::StorageFile> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<super::super::Storage::StorageFile>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LogFileGeneratedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.LogFileGeneratedEventArgs;{269e976f-0d38-4c1a-b53f-b395d881df84})");
}
unsafe impl ::windows::runtime::Interface for LogFileGeneratedEventArgs {
    type Vtable = ILogFileGeneratedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(647927663, 3384, 19482, [181, 63, 179, 149, 216, 129, 223, 132]);
}
impl ::windows::runtime::RuntimeName for LogFileGeneratedEventArgs {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.LogFileGeneratedEventArgs";
}
unsafe impl ::std::marker::Send for LogFileGeneratedEventArgs {}
unsafe impl ::std::marker::Sync for LogFileGeneratedEventArgs {}
#[doc = "*Required features: `Foundation_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct LoggingActivity(::windows::runtime::IInspectable);
impl LoggingActivity {
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Channel(&self) -> ::windows::runtime::Result<LoggingChannel> {
        let this = &::windows::runtime::Interface::cast::<ILoggingActivity2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LoggingChannel>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn StopActivity<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, stopeventname: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ILoggingActivity2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), stopeventname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn StopActivityWithFields<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, LoggingFields>>(&self, stopeventname: Param0, fields: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ILoggingActivity2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), stopeventname.into_param().abi(), fields.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn StopActivityWithFieldsAndOptions<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, LoggingFields>, Param2: ::windows::runtime::IntoParam<'a, LoggingOptions>>(&self, stopeventname: Param0, fields: Param1, options: Param2) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ILoggingActivity2>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), stopeventname.into_param().abi(), fields.into_param().abi(), options.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn IsEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn IsEnabledWithLevel(&self, level: LoggingLevel) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), level, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn IsEnabledWithLevelAndKeywords(&self, level: LoggingLevel, keywords: i64) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), level, keywords, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogEvent<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, eventname: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ILoggingTarget>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), eventname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogEventWithFields<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, LoggingFields>>(&self, eventname: Param0, fields: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ILoggingTarget>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), eventname.into_param().abi(), fields.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogEventWithFieldsAndLevel<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, LoggingFields>>(&self, eventname: Param0, fields: Param1, level: LoggingLevel) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ILoggingTarget>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), eventname.into_param().abi(), fields.into_param().abi(), level).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogEventWithFieldsAndOptions<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, LoggingFields>, Param3: ::windows::runtime::IntoParam<'a, LoggingOptions>>(&self, eventname: Param0, fields: Param1, level: LoggingLevel, options: Param3) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ILoggingTarget>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), eventname.into_param().abi(), fields.into_param().abi(), level, options.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn StartActivity<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, starteventname: Param0) -> ::windows::runtime::Result<LoggingActivity> {
        let this = &::windows::runtime::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), starteventname.into_param().abi(), &mut result__).from_abi::<LoggingActivity>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn StartActivityWithFields<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, LoggingFields>>(&self, starteventname: Param0, fields: Param1) -> ::windows::runtime::Result<LoggingActivity> {
        let this = &::windows::runtime::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), starteventname.into_param().abi(), fields.into_param().abi(), &mut result__).from_abi::<LoggingActivity>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn StartActivityWithFieldsAndLevel<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, LoggingFields>>(&self, starteventname: Param0, fields: Param1, level: LoggingLevel) -> ::windows::runtime::Result<LoggingActivity> {
        let this = &::windows::runtime::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), starteventname.into_param().abi(), fields.into_param().abi(), level, &mut result__).from_abi::<LoggingActivity>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn StartActivityWithFieldsAndOptions<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, LoggingFields>, Param3: ::windows::runtime::IntoParam<'a, LoggingOptions>>(&self, starteventname: Param0, fields: Param1, level: LoggingLevel, options: Param3) -> ::windows::runtime::Result<LoggingActivity> {
        let this = &::windows::runtime::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), starteventname.into_param().abi(), fields.into_param().abi(), level, options.into_param().abi(), &mut result__).from_abi::<LoggingActivity>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn CreateLoggingActivity<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ILoggingChannel>>(activityname: Param0, loggingchannel: Param1) -> ::windows::runtime::Result<LoggingActivity> {
        Self::ILoggingActivityFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), activityname.into_param().abi(), loggingchannel.into_param().abi(), &mut result__).from_abi::<LoggingActivity>(result__)
        })
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn CreateLoggingActivityWithLevel<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ILoggingChannel>>(activityname: Param0, loggingchannel: Param1, level: LoggingLevel) -> ::windows::runtime::Result<LoggingActivity> {
        Self::ILoggingActivityFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), activityname.into_param().abi(), loggingchannel.into_param().abi(), level, &mut result__).from_abi::<LoggingActivity>(result__)
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3154323777, 46950, 19637, [152, 72, 151, 172, 107, 166, 214, 12]);
}
impl ::windows::runtime::RuntimeName for LoggingActivity {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.LoggingActivity";
}
impl ::std::convert::TryFrom<LoggingActivity> for super::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: LoggingActivity) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&LoggingActivity> for super::IClosable {
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
        ::std::convert::TryInto::<super::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<LoggingActivity> for ILoggingTarget {
    type Error = ::windows::runtime::Error;
    fn try_from(value: LoggingActivity) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&LoggingActivity> for ILoggingTarget {
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
        ::std::convert::TryInto::<ILoggingTarget>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for LoggingActivity {}
unsafe impl ::std::marker::Sync for LoggingActivity {}
#[doc = "*Required features: `Foundation_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct LoggingChannel(::windows::runtime::IInspectable);
impl LoggingChannel {
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Enabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Level(&self) -> ::windows::runtime::Result<LoggingLevel> {
        let this = self;
        unsafe {
            let mut result__: LoggingLevel = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LoggingLevel>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogMessage<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, eventstring: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), eventstring.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogMessageWithLevel<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, eventstring: Param0, level: LoggingLevel) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), eventstring.into_param().abi(), level).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogValuePair<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value1: Param0, value2: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value1.into_param().abi(), value2).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogValuePairWithLevel<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, value1: Param0, value2: i32, level: LoggingLevel) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), value1.into_param().abi(), value2, level).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LoggingEnabled<'a, Param0: ::windows::runtime::IntoParam<'a, super::TypedEventHandler<ILoggingChannel, ::windows::runtime::IInspectable>>>(&self, handler: Param0) -> ::windows::runtime::Result<super::EventRegistrationToken> {
        let this = self;
        unsafe {
            let mut result__: super::EventRegistrationToken = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), handler.into_param().abi(), &mut result__).from_abi::<super::EventRegistrationToken>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn RemoveLoggingEnabled<'a, Param0: ::windows::runtime::IntoParam<'a, super::EventRegistrationToken>>(&self, token: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), token.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Id(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = &::windows::runtime::Interface::cast::<ILoggingChannel2>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn IsEnabled(&self) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn IsEnabledWithLevel(&self, level: LoggingLevel) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), level, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn IsEnabledWithLevelAndKeywords(&self, level: LoggingLevel, keywords: i64) -> ::windows::runtime::Result<bool> {
        let this = &::windows::runtime::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), level, keywords, &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogEvent<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, eventname: Param0) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ILoggingTarget>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), eventname.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogEventWithFields<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, LoggingFields>>(&self, eventname: Param0, fields: Param1) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ILoggingTarget>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), eventname.into_param().abi(), fields.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogEventWithFieldsAndLevel<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, LoggingFields>>(&self, eventname: Param0, fields: Param1, level: LoggingLevel) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ILoggingTarget>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), eventname.into_param().abi(), fields.into_param().abi(), level).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn LogEventWithFieldsAndOptions<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, LoggingFields>, Param3: ::windows::runtime::IntoParam<'a, LoggingOptions>>(&self, eventname: Param0, fields: Param1, level: LoggingLevel, options: Param3) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<ILoggingTarget>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), eventname.into_param().abi(), fields.into_param().abi(), level, options.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn StartActivity<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, starteventname: Param0) -> ::windows::runtime::Result<LoggingActivity> {
        let this = &::windows::runtime::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), starteventname.into_param().abi(), &mut result__).from_abi::<LoggingActivity>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn StartActivityWithFields<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, LoggingFields>>(&self, starteventname: Param0, fields: Param1) -> ::windows::runtime::Result<LoggingActivity> {
        let this = &::windows::runtime::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), starteventname.into_param().abi(), fields.into_param().abi(), &mut result__).from_abi::<LoggingActivity>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn StartActivityWithFieldsAndLevel<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, LoggingFields>>(&self, starteventname: Param0, fields: Param1, level: LoggingLevel) -> ::windows::runtime::Result<LoggingActivity> {
        let this = &::windows::runtime::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), starteventname.into_param().abi(), fields.into_param().abi(), level, &mut result__).from_abi::<LoggingActivity>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn StartActivityWithFieldsAndOptions<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, LoggingFields>, Param3: ::windows::runtime::IntoParam<'a, LoggingOptions>>(&self, starteventname: Param0, fields: Param1, level: LoggingLevel, options: Param3) -> ::windows::runtime::Result<LoggingActivity> {
        let this = &::windows::runtime::Interface::cast::<ILoggingTarget>(self)?;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), starteventname.into_param().abi(), fields.into_param().abi(), level, options.into_param().abi(), &mut result__).from_abi::<LoggingActivity>(result__)
        }
    }
    #[cfg(feature = "deprecated")]
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(name: Param0) -> ::windows::runtime::Result<LoggingChannel> {
        Self::ILoggingChannelFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<LoggingChannel>(result__)
        })
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn CreateWithOptions<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, LoggingChannelOptions>>(name: Param0, options: Param1) -> ::windows::runtime::Result<LoggingChannel> {
        Self::ILoggingChannelFactory2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), name.into_param().abi(), options.into_param().abi(), &mut result__).from_abi::<LoggingChannel>(result__)
        })
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn CreateWithOptionsAndId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, LoggingChannelOptions>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(name: Param0, options: Param1, id: Param2) -> ::windows::runtime::Result<LoggingChannel> {
        Self::ILoggingChannelFactory2(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), name.into_param().abi(), options.into_param().abi(), id.into_param().abi(), &mut result__).from_abi::<LoggingChannel>(result__)
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3919905603, 4567, 20225, [181, 202, 207, 73, 82, 120, 192, 168]);
}
impl ::windows::runtime::RuntimeName for LoggingChannel {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.LoggingChannel";
}
impl ::std::convert::From<LoggingChannel> for ILoggingChannel {
    fn from(value: LoggingChannel) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&LoggingChannel> for ILoggingChannel {
    fn from(value: &LoggingChannel) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILoggingChannel> for LoggingChannel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILoggingChannel> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ILoggingChannel>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILoggingChannel> for &LoggingChannel {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILoggingChannel> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ILoggingChannel>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::TryFrom<LoggingChannel> for super::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: LoggingChannel) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&LoggingChannel> for super::IClosable {
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
        ::std::convert::TryInto::<super::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
impl ::std::convert::TryFrom<LoggingChannel> for ILoggingTarget {
    type Error = ::windows::runtime::Error;
    fn try_from(value: LoggingChannel) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&LoggingChannel> for ILoggingTarget {
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
        ::std::convert::TryInto::<ILoggingTarget>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for LoggingChannel {}
unsafe impl ::std::marker::Sync for LoggingChannel {}
#[doc = "*Required features: `Foundation_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct LoggingChannelOptions(::windows::runtime::IInspectable);
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
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn SetGroup<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(group: Param0) -> ::windows::runtime::Result<LoggingChannelOptions> {
        Self::ILoggingChannelOptionsFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), group.into_param().abi(), &mut result__).from_abi::<LoggingChannelOptions>(result__)
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3286779903, 3771, 19027, [140, 84, 222, 194, 73, 38, 203, 44]);
}
impl ::windows::runtime::RuntimeName for LoggingChannelOptions {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.LoggingChannelOptions";
}
unsafe impl ::std::marker::Send for LoggingChannelOptions {}
unsafe impl ::std::marker::Sync for LoggingChannelOptions {}
#[doc = "*Required features: `Foundation_Diagnostics`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for LoggingFieldFormat {
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct LoggingFields(::windows::runtime::IInspectable);
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
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn BeginStruct<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), name.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn BeginStructWithTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), name.into_param().abi(), tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn EndStruct(&self) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddEmpty<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), name.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddEmptyWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), name.into_param().abi(), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddEmptyWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), name.into_param().abi(), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt8<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: u8) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), name.into_param().abi(), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt8WithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: u8, format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), name.into_param().abi(), value, format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt8WithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: u8, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), name.into_param().abi(), value, format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt8Array<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<u8 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt8ArrayWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<u8 as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt8ArrayWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<u8 as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).18)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt16<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: i16) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).19)(::std::mem::transmute_copy(this), name.into_param().abi(), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt16WithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: i16, format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).20)(::std::mem::transmute_copy(this), name.into_param().abi(), value, format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt16WithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: i16, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).21)(::std::mem::transmute_copy(this), name.into_param().abi(), value, format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt16Array<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<i16 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).22)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt16ArrayWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<i16 as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).23)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt16ArrayWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<i16 as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).24)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt16<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: u16) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).25)(::std::mem::transmute_copy(this), name.into_param().abi(), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt16WithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: u16, format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).26)(::std::mem::transmute_copy(this), name.into_param().abi(), value, format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt16WithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: u16, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).27)(::std::mem::transmute_copy(this), name.into_param().abi(), value, format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt16Array<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<u16 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).28)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt16ArrayWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<u16 as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).29)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt16ArrayWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<u16 as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).30)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt32<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).31)(::std::mem::transmute_copy(this), name.into_param().abi(), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt32WithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: i32, format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).32)(::std::mem::transmute_copy(this), name.into_param().abi(), value, format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt32WithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: i32, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).33)(::std::mem::transmute_copy(this), name.into_param().abi(), value, format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt32Array<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<i32 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).34)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt32ArrayWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<i32 as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).35)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt32ArrayWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<i32 as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).36)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt32<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: u32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).37)(::std::mem::transmute_copy(this), name.into_param().abi(), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt32WithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: u32, format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).38)(::std::mem::transmute_copy(this), name.into_param().abi(), value, format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt32WithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: u32, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).39)(::std::mem::transmute_copy(this), name.into_param().abi(), value, format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt32Array<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<u32 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).40)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt32ArrayWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<u32 as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).41)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt32ArrayWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<u32 as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).42)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt64<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: i64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).43)(::std::mem::transmute_copy(this), name.into_param().abi(), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt64WithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: i64, format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).44)(::std::mem::transmute_copy(this), name.into_param().abi(), value, format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt64WithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: i64, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).45)(::std::mem::transmute_copy(this), name.into_param().abi(), value, format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt64Array<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<i64 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).46)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt64ArrayWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<i64 as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).47)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddInt64ArrayWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<i64 as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).48)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt64<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: u64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).49)(::std::mem::transmute_copy(this), name.into_param().abi(), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt64WithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: u64, format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).50)(::std::mem::transmute_copy(this), name.into_param().abi(), value, format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt64WithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: u64, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).51)(::std::mem::transmute_copy(this), name.into_param().abi(), value, format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt64Array<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<u64 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).52)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt64ArrayWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<u64 as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).53)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddUInt64ArrayWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<u64 as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).54)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddSingle<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: f32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).55)(::std::mem::transmute_copy(this), name.into_param().abi(), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddSingleWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: f32, format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).56)(::std::mem::transmute_copy(this), name.into_param().abi(), value, format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddSingleWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: f32, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).57)(::std::mem::transmute_copy(this), name.into_param().abi(), value, format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddSingleArray<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<f32 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).58)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddSingleArrayWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<f32 as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).59)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddSingleArrayWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<f32 as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).60)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddDouble<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: f64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).61)(::std::mem::transmute_copy(this), name.into_param().abi(), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddDoubleWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: f64, format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).62)(::std::mem::transmute_copy(this), name.into_param().abi(), value, format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddDoubleWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: f64, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).63)(::std::mem::transmute_copy(this), name.into_param().abi(), value, format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddDoubleArray<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<f64 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).64)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddDoubleArrayWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<f64 as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).65)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddDoubleArrayWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<f64 as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).66)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddChar16<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: u16) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).67)(::std::mem::transmute_copy(this), name.into_param().abi(), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddChar16WithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: u16, format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).68)(::std::mem::transmute_copy(this), name.into_param().abi(), value, format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddChar16WithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: u16, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).69)(::std::mem::transmute_copy(this), name.into_param().abi(), value, format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddChar16Array<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<u16 as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).70)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddChar16ArrayWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<u16 as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).71)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddChar16ArrayWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<u16 as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).72)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddBoolean<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: bool) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).73)(::std::mem::transmute_copy(this), name.into_param().abi(), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddBooleanWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: bool, format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).74)(::std::mem::transmute_copy(this), name.into_param().abi(), value, format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddBooleanWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: bool, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).75)(::std::mem::transmute_copy(this), name.into_param().abi(), value, format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddBooleanArray<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<bool as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).76)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddBooleanArrayWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<bool as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).77)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddBooleanArrayWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<bool as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).78)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddString<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).79)(::std::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddStringWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).80)(::std::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddStringWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).81)(::std::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddStringArray<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<::windows::runtime::HSTRING as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).82)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddStringArrayWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<::windows::runtime::HSTRING as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).83)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddStringArrayWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<::windows::runtime::HSTRING as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).84)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddGuid<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, name: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).85)(::std::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddGuidWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).86)(::std::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddGuidWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).87)(::std::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddGuidArray<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<::windows::runtime::GUID as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).88)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddGuidArrayWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<::windows::runtime::GUID as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).89)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddGuidArrayWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<::windows::runtime::GUID as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).90)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddDateTime<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::DateTime>>(&self, name: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).91)(::std::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddDateTimeWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::DateTime>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).92)(::std::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddDateTimeWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::DateTime>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).93)(::std::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddDateTimeArray<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<super::DateTime as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).94)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddDateTimeArrayWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<super::DateTime as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).95)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddDateTimeArrayWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<super::DateTime as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).96)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddTimeSpan<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::TimeSpan>>(&self, name: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).97)(::std::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddTimeSpanWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::TimeSpan>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).98)(::std::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddTimeSpanWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::TimeSpan>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).99)(::std::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddTimeSpanArray<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<super::TimeSpan as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).100)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddTimeSpanArrayWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<super::TimeSpan as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).101)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddTimeSpanArrayWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<super::TimeSpan as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).102)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddPoint<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::Point>>(&self, name: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).103)(::std::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddPointWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::Point>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).104)(::std::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddPointWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::Point>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).105)(::std::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddPointArray<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<super::Point as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).106)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddPointArrayWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<super::Point as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).107)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddPointArrayWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<super::Point as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).108)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddSize<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::Size>>(&self, name: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).109)(::std::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddSizeWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::Size>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).110)(::std::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddSizeWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::Size>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).111)(::std::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddSizeArray<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<super::Size as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).112)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddSizeArrayWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<super::Size as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).113)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddSizeArrayWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<super::Size as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).114)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddRect<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::Rect>>(&self, name: Param0, value: Param1) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).115)(::std::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddRectWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::Rect>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).116)(::std::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddRectWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>, Param1: ::windows::runtime::IntoParam<'a, super::Rect>>(&self, name: Param0, value: Param1, format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).117)(::std::mem::transmute_copy(this), name.into_param().abi(), value.into_param().abi(), format, tags).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddRectArray<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<super::Rect as ::windows::runtime::DefaultType>::DefaultType]) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).118)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr())).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddRectArrayWithFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<super::Rect as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).119)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr()), format).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddRectArrayWithFormatAndTags<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, name: Param0, value: &[<super::Rect as ::windows::runtime::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).120)(::std::mem::transmute_copy(this), name.into_param().abi(), value.len() as u32, ::std::mem::transmute(value.as_ptr()), format, tags).ok() }
    }
}
unsafe impl ::windows::runtime::RuntimeType for LoggingFields {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.LoggingFields;{d7f6b7af-762d-4579-83bd-52c23bc333bc})");
}
unsafe impl ::windows::runtime::Interface for LoggingFields {
    type Vtable = ILoggingFields_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3623270319, 30253, 17785, [131, 189, 82, 194, 59, 195, 51, 188]);
}
impl ::windows::runtime::RuntimeName for LoggingFields {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.LoggingFields";
}
unsafe impl ::std::marker::Send for LoggingFields {}
unsafe impl ::std::marker::Sync for LoggingFields {}
#[doc = "*Required features: `Foundation_Diagnostics`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct LoggingLevel(pub i32);
impl LoggingLevel {
    pub const Verbose: LoggingLevel = LoggingLevel(0i32);
    pub const Information: LoggingLevel = LoggingLevel(1i32);
    pub const Warning: LoggingLevel = LoggingLevel(2i32);
    pub const Error: LoggingLevel = LoggingLevel(3i32);
    pub const Critical: LoggingLevel = LoggingLevel(4i32);
}
impl ::std::convert::From<i32> for LoggingLevel {
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
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
impl ::std::convert::From<i32> for LoggingOpcode {
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
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct LoggingOptions(::windows::runtime::IInspectable);
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
            let mut result__: i64 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i64>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn SetKeywords(&self, value: i64) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Tags(&self) -> ::windows::runtime::Result<i32> {
        let this = self;
        unsafe {
            let mut result__: i32 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i32>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn SetTags(&self, value: i32) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Task(&self) -> ::windows::runtime::Result<i16> {
        let this = self;
        unsafe {
            let mut result__: i16 = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), &mut result__).from_abi::<i16>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn SetTask(&self, value: i16) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).11)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Opcode(&self) -> ::windows::runtime::Result<LoggingOpcode> {
        let this = self;
        unsafe {
            let mut result__: LoggingOpcode = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).12)(::std::mem::transmute_copy(this), &mut result__).from_abi::<LoggingOpcode>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn SetOpcode(&self, value: LoggingOpcode) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).13)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn ActivityId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).14)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn SetActivityId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).15)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn RelatedActivityId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::GUID = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).16)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn SetRelatedActivityId<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(&self, value: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).17)(::std::mem::transmute_copy(this), value.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn CreateWithKeywords(keywords: i64) -> ::windows::runtime::Result<LoggingOptions> {
        Self::ILoggingOptionsFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), keywords, &mut result__).from_abi::<LoggingOptions>(result__)
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2428270672, 402, 20317, [172, 38, 0, 106, 218, 202, 18, 216]);
}
impl ::windows::runtime::RuntimeName for LoggingOptions {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.LoggingOptions";
}
unsafe impl ::std::marker::Send for LoggingOptions {}
unsafe impl ::std::marker::Sync for LoggingOptions {}
#[doc = "*Required features: `Foundation_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct LoggingSession(::windows::runtime::IInspectable);
impl LoggingSession {
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Name(&self) -> ::windows::runtime::Result<::windows::runtime::HSTRING> {
        let this = self;
        unsafe {
            let mut result__: ::std::mem::ManuallyDrop<::windows::runtime::HSTRING> = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<::windows::runtime::HSTRING>(result__)
        }
    }
    #[cfg(feature = "Storage")]
    #[doc = "*Required features: `Foundation_Diagnostics`, `Storage`*"]
    pub fn SaveToFileAsync<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Storage::IStorageFolder>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(&self, folder: Param0, filename: Param1) -> ::windows::runtime::Result<super::IAsyncOperation<super::super::Storage::StorageFile>> {
        let this = self;
        unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), folder.into_param().abi(), filename.into_param().abi(), &mut result__).from_abi::<super::IAsyncOperation<super::super::Storage::StorageFile>>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddLoggingChannel<'a, Param0: ::windows::runtime::IntoParam<'a, ILoggingChannel>>(&self, loggingchannel: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).8)(::std::mem::transmute_copy(this), loggingchannel.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn AddLoggingChannelWithLevel<'a, Param0: ::windows::runtime::IntoParam<'a, ILoggingChannel>>(&self, loggingchannel: Param0, maxlevel: LoggingLevel) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).9)(::std::mem::transmute_copy(this), loggingchannel.into_param().abi(), maxlevel).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn RemoveLoggingChannel<'a, Param0: ::windows::runtime::IntoParam<'a, ILoggingChannel>>(&self, loggingchannel: Param0) -> ::windows::runtime::Result<()> {
        let this = self;
        unsafe { (::windows::runtime::Interface::vtable(this).10)(::std::mem::transmute_copy(this), loggingchannel.into_param().abi()).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Close(&self) -> ::windows::runtime::Result<()> {
        let this = &::windows::runtime::Interface::cast::<super::IClosable>(self)?;
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this)).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Create<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::HSTRING>>(name: Param0) -> ::windows::runtime::Result<LoggingSession> {
        Self::ILoggingSessionFactory(|this| unsafe {
            let mut result__: ::windows::runtime::RawPtr = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), name.into_param().abi(), &mut result__).from_abi::<LoggingSession>(result__)
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
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1646392070, 37760, 19159, [186, 245, 65, 234, 147, 16, 215, 104]);
}
impl ::windows::runtime::RuntimeName for LoggingSession {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.LoggingSession";
}
impl ::std::convert::From<LoggingSession> for ILoggingSession {
    fn from(value: LoggingSession) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&LoggingSession> for ILoggingSession {
    fn from(value: &LoggingSession) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILoggingSession> for LoggingSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILoggingSession> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ILoggingSession>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ILoggingSession> for &LoggingSession {
    fn into_param(self) -> ::windows::runtime::Param<'a, ILoggingSession> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ILoggingSession>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::TryFrom<LoggingSession> for super::IClosable {
    type Error = ::windows::runtime::Error;
    fn try_from(value: LoggingSession) -> ::windows::runtime::Result<Self> {
        ::std::convert::TryFrom::try_from(&value)
    }
}
impl ::std::convert::TryFrom<&LoggingSession> for super::IClosable {
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
        ::std::convert::TryInto::<super::IClosable>::try_into(self).map(::windows::runtime::Param::Owned).unwrap_or(::windows::runtime::Param::None)
    }
}
unsafe impl ::std::marker::Send for LoggingSession {}
unsafe impl ::std::marker::Sync for LoggingSession {}
#[doc = "*Required features: `Foundation_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct RuntimeBrokerErrorSettings(::windows::runtime::IInspectable);
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
        unsafe { (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), value).ok() }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn GetErrorOptions(&self) -> ::windows::runtime::Result<ErrorOptions> {
        let this = self;
        unsafe {
            let mut result__: ErrorOptions = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<ErrorOptions>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for RuntimeBrokerErrorSettings {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.RuntimeBrokerErrorSettings;{16369792-b03e-4ba1-8bb8-d28f4ab4d2c0})");
}
unsafe impl ::windows::runtime::Interface for RuntimeBrokerErrorSettings {
    type Vtable = IErrorReportingSettings_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(372676498, 45118, 19361, [139, 184, 210, 143, 74, 180, 210, 192]);
}
impl ::windows::runtime::RuntimeName for RuntimeBrokerErrorSettings {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.RuntimeBrokerErrorSettings";
}
impl ::std::convert::From<RuntimeBrokerErrorSettings> for IErrorReportingSettings {
    fn from(value: RuntimeBrokerErrorSettings) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&RuntimeBrokerErrorSettings> for IErrorReportingSettings {
    fn from(value: &RuntimeBrokerErrorSettings) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IErrorReportingSettings> for RuntimeBrokerErrorSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, IErrorReportingSettings> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IErrorReportingSettings>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IErrorReportingSettings> for &RuntimeBrokerErrorSettings {
    fn into_param(self) -> ::windows::runtime::Param<'a, IErrorReportingSettings> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IErrorReportingSettings>::into(::std::clone::Clone::clone(self)))
    }
}
unsafe impl ::std::marker::Send for RuntimeBrokerErrorSettings {}
unsafe impl ::std::marker::Sync for RuntimeBrokerErrorSettings {}
#[doc = "*Required features: `Foundation_Diagnostics`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug, :: windows :: runtime :: DeriveInterface)]
pub struct TracingStatusChangedEventArgs(::windows::runtime::IInspectable);
impl TracingStatusChangedEventArgs {
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn Enabled(&self) -> ::windows::runtime::Result<bool> {
        let this = self;
        unsafe {
            let mut result__: bool = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).6)(::std::mem::transmute_copy(this), &mut result__).from_abi::<bool>(result__)
        }
    }
    #[doc = "*Required features: `Foundation_Diagnostics`*"]
    pub fn TraceLevel(&self) -> ::windows::runtime::Result<CausalityTraceLevel> {
        let this = self;
        unsafe {
            let mut result__: CausalityTraceLevel = ::std::mem::zeroed();
            (::windows::runtime::Interface::vtable(this).7)(::std::mem::transmute_copy(this), &mut result__).from_abi::<CausalityTraceLevel>(result__)
        }
    }
}
unsafe impl ::windows::runtime::RuntimeType for TracingStatusChangedEventArgs {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(b"rc(Windows.Foundation.Diagnostics.TracingStatusChangedEventArgs;{410b7711-ff3b-477f-9c9a-d2efda302dc3})");
}
unsafe impl ::windows::runtime::Interface for TracingStatusChangedEventArgs {
    type Vtable = ITracingStatusChangedEventArgs_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1091270417, 65339, 18303, [156, 154, 210, 239, 218, 48, 45, 195]);
}
impl ::windows::runtime::RuntimeName for TracingStatusChangedEventArgs {
    const NAME: &'static str = "Windows.Foundation.Diagnostics.TracingStatusChangedEventArgs";
}
unsafe impl ::std::marker::Send for TracingStatusChangedEventArgs {}
unsafe impl ::std::marker::Sync for TracingStatusChangedEventArgs {}
