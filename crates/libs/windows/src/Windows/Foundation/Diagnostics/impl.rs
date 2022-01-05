#[cfg(feature = "implement_exclusive")]
pub trait IAsyncCausalityTracerStaticsImpl: Sized {
    fn TraceOperationCreation(&self, tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: &::windows::core::GUID, operationid: u64, operationname: &::windows::core::HSTRING, relatedcontext: u64) -> ::windows::core::Result<()>;
    fn TraceOperationCompletion(&self, tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: &::windows::core::GUID, operationid: u64, status: super::AsyncStatus) -> ::windows::core::Result<()>;
    fn TraceOperationRelation(&self, tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: &::windows::core::GUID, operationid: u64, relation: CausalityRelation) -> ::windows::core::Result<()>;
    fn TraceSynchronousWorkStart(&self, tracelevel: CausalityTraceLevel, source: CausalitySource, platformid: &::windows::core::GUID, operationid: u64, work: CausalitySynchronousWork) -> ::windows::core::Result<()>;
    fn TraceSynchronousWorkCompletion(&self, tracelevel: CausalityTraceLevel, source: CausalitySource, work: CausalitySynchronousWork) -> ::windows::core::Result<()>;
    fn TracingStatusChanged(&self, handler: &::core::option::Option<super::EventHandler<TracingStatusChangedEventArgs>>) -> ::windows::core::Result<super::EventRegistrationToken>;
    fn RemoveTracingStatusChanged(&self, cookie: &super::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IErrorDetailsImpl: Sized {
    fn Description(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn LongDescription(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn HelpUri(&self) -> ::windows::core::Result<super::Uri>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IErrorDetailsStaticsImpl: Sized {
    fn CreateFromHResultAsync(&self, errorcode: i32) -> ::windows::core::Result<super::IAsyncOperation<ErrorDetails>>;
}
pub trait IErrorReportingSettingsImpl: Sized {
    fn SetErrorOptions(&self, value: ErrorOptions) -> ::windows::core::Result<()>;
    fn GetErrorOptions(&self) -> ::windows::core::Result<ErrorOptions>;
}
pub trait IFileLoggingSessionImpl: Sized + IClosableImpl {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn AddLoggingChannel(&self, loggingchannel: &::core::option::Option<ILoggingChannel>) -> ::windows::core::Result<()>;
    fn AddLoggingChannelWithLevel(&self, loggingchannel: &::core::option::Option<ILoggingChannel>, maxlevel: LoggingLevel) -> ::windows::core::Result<()>;
    fn RemoveLoggingChannel(&self, loggingchannel: &::core::option::Option<ILoggingChannel>) -> ::windows::core::Result<()>;
    fn CloseAndSaveToFileAsync(&self) -> ::windows::core::Result<super::IAsyncOperation<super::super::Storage::StorageFile>>;
    fn LogFileGenerated(&self, handler: &::core::option::Option<super::TypedEventHandler<IFileLoggingSession, LogFileGeneratedEventArgs>>) -> ::windows::core::Result<super::EventRegistrationToken>;
    fn RemoveLogFileGenerated(&self, token: &super::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IFileLoggingSessionFactoryImpl: Sized {
    fn Create(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<FileLoggingSession>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILogFileGeneratedEventArgsImpl: Sized {
    fn File(&self) -> ::windows::core::Result<super::super::Storage::StorageFile>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoggingActivityImpl: Sized + IClosableImpl {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoggingActivity2Impl: Sized + IClosableImpl + ILoggingActivityImpl + ILoggingTargetImpl {
    fn Channel(&self) -> ::windows::core::Result<LoggingChannel>;
    fn StopActivity(&self, stopeventname: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn StopActivityWithFields(&self, stopeventname: &::windows::core::HSTRING, fields: &::core::option::Option<LoggingFields>) -> ::windows::core::Result<()>;
    fn StopActivityWithFieldsAndOptions(&self, stopeventname: &::windows::core::HSTRING, fields: &::core::option::Option<LoggingFields>, options: &::core::option::Option<LoggingOptions>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoggingActivityFactoryImpl: Sized {
    fn CreateLoggingActivity(&self, activityname: &::windows::core::HSTRING, loggingchannel: &::core::option::Option<ILoggingChannel>) -> ::windows::core::Result<LoggingActivity>;
    fn CreateLoggingActivityWithLevel(&self, activityname: &::windows::core::HSTRING, loggingchannel: &::core::option::Option<ILoggingChannel>, level: LoggingLevel) -> ::windows::core::Result<LoggingActivity>;
}
pub trait ILoggingChannelImpl: Sized + IClosableImpl {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Enabled(&self) -> ::windows::core::Result<bool>;
    fn Level(&self) -> ::windows::core::Result<LoggingLevel>;
    fn LogMessage(&self, eventstring: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn LogMessageWithLevel(&self, eventstring: &::windows::core::HSTRING, level: LoggingLevel) -> ::windows::core::Result<()>;
    fn LogValuePair(&self, value1: &::windows::core::HSTRING, value2: i32) -> ::windows::core::Result<()>;
    fn LogValuePairWithLevel(&self, value1: &::windows::core::HSTRING, value2: i32, level: LoggingLevel) -> ::windows::core::Result<()>;
    fn LoggingEnabled(&self, handler: &::core::option::Option<super::TypedEventHandler<ILoggingChannel, ::windows::core::IInspectable>>) -> ::windows::core::Result<super::EventRegistrationToken>;
    fn RemoveLoggingEnabled(&self, token: &super::EventRegistrationToken) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoggingChannel2Impl: Sized + IClosableImpl + ILoggingChannelImpl + ILoggingTargetImpl {
    fn Id(&self) -> ::windows::core::Result<::windows::core::GUID>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoggingChannelFactoryImpl: Sized {
    fn Create(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<LoggingChannel>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoggingChannelFactory2Impl: Sized {
    fn CreateWithOptions(&self, name: &::windows::core::HSTRING, options: &::core::option::Option<LoggingChannelOptions>) -> ::windows::core::Result<LoggingChannel>;
    fn CreateWithOptionsAndId(&self, name: &::windows::core::HSTRING, options: &::core::option::Option<LoggingChannelOptions>, id: &::windows::core::GUID) -> ::windows::core::Result<LoggingChannel>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoggingChannelOptionsImpl: Sized {
    fn Group(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetGroup(&self, value: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoggingChannelOptionsFactoryImpl: Sized {
    fn Create(&self, group: &::windows::core::GUID) -> ::windows::core::Result<LoggingChannelOptions>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoggingFieldsImpl: Sized {
    fn Clear(&self) -> ::windows::core::Result<()>;
    fn BeginStruct(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn BeginStructWithTags(&self, name: &::windows::core::HSTRING, tags: i32) -> ::windows::core::Result<()>;
    fn EndStruct(&self) -> ::windows::core::Result<()>;
    fn AddEmpty(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AddEmptyWithFormat(&self, name: &::windows::core::HSTRING, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddEmptyWithFormatAndTags(&self, name: &::windows::core::HSTRING, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddUInt8(&self, name: &::windows::core::HSTRING, value: u8) -> ::windows::core::Result<()>;
    fn AddUInt8WithFormat(&self, name: &::windows::core::HSTRING, value: u8, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddUInt8WithFormatAndTags(&self, name: &::windows::core::HSTRING, value: u8, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddUInt8Array(&self, name: &::windows::core::HSTRING, value: &[<u8 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddUInt8ArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[<u8 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddUInt8ArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[<u8 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddInt16(&self, name: &::windows::core::HSTRING, value: i16) -> ::windows::core::Result<()>;
    fn AddInt16WithFormat(&self, name: &::windows::core::HSTRING, value: i16, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddInt16WithFormatAndTags(&self, name: &::windows::core::HSTRING, value: i16, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddInt16Array(&self, name: &::windows::core::HSTRING, value: &[<i16 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddInt16ArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[<i16 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddInt16ArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[<i16 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddUInt16(&self, name: &::windows::core::HSTRING, value: u16) -> ::windows::core::Result<()>;
    fn AddUInt16WithFormat(&self, name: &::windows::core::HSTRING, value: u16, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddUInt16WithFormatAndTags(&self, name: &::windows::core::HSTRING, value: u16, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddUInt16Array(&self, name: &::windows::core::HSTRING, value: &[<u16 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddUInt16ArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[<u16 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddUInt16ArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[<u16 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddInt32(&self, name: &::windows::core::HSTRING, value: i32) -> ::windows::core::Result<()>;
    fn AddInt32WithFormat(&self, name: &::windows::core::HSTRING, value: i32, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddInt32WithFormatAndTags(&self, name: &::windows::core::HSTRING, value: i32, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddInt32Array(&self, name: &::windows::core::HSTRING, value: &[<i32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddInt32ArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[<i32 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddInt32ArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[<i32 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddUInt32(&self, name: &::windows::core::HSTRING, value: u32) -> ::windows::core::Result<()>;
    fn AddUInt32WithFormat(&self, name: &::windows::core::HSTRING, value: u32, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddUInt32WithFormatAndTags(&self, name: &::windows::core::HSTRING, value: u32, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddUInt32Array(&self, name: &::windows::core::HSTRING, value: &[<u32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddUInt32ArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[<u32 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddUInt32ArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[<u32 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddInt64(&self, name: &::windows::core::HSTRING, value: i64) -> ::windows::core::Result<()>;
    fn AddInt64WithFormat(&self, name: &::windows::core::HSTRING, value: i64, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddInt64WithFormatAndTags(&self, name: &::windows::core::HSTRING, value: i64, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddInt64Array(&self, name: &::windows::core::HSTRING, value: &[<i64 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddInt64ArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[<i64 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddInt64ArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[<i64 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddUInt64(&self, name: &::windows::core::HSTRING, value: u64) -> ::windows::core::Result<()>;
    fn AddUInt64WithFormat(&self, name: &::windows::core::HSTRING, value: u64, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddUInt64WithFormatAndTags(&self, name: &::windows::core::HSTRING, value: u64, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddUInt64Array(&self, name: &::windows::core::HSTRING, value: &[<u64 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddUInt64ArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[<u64 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddUInt64ArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[<u64 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddSingle(&self, name: &::windows::core::HSTRING, value: f32) -> ::windows::core::Result<()>;
    fn AddSingleWithFormat(&self, name: &::windows::core::HSTRING, value: f32, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddSingleWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: f32, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddSingleArray(&self, name: &::windows::core::HSTRING, value: &[<f32 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddSingleArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[<f32 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddSingleArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[<f32 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddDouble(&self, name: &::windows::core::HSTRING, value: f64) -> ::windows::core::Result<()>;
    fn AddDoubleWithFormat(&self, name: &::windows::core::HSTRING, value: f64, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddDoubleWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: f64, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddDoubleArray(&self, name: &::windows::core::HSTRING, value: &[<f64 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddDoubleArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[<f64 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddDoubleArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[<f64 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddChar16(&self, name: &::windows::core::HSTRING, value: u16) -> ::windows::core::Result<()>;
    fn AddChar16WithFormat(&self, name: &::windows::core::HSTRING, value: u16, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddChar16WithFormatAndTags(&self, name: &::windows::core::HSTRING, value: u16, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddChar16Array(&self, name: &::windows::core::HSTRING, value: &[<u16 as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddChar16ArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[<u16 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddChar16ArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[<u16 as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddBoolean(&self, name: &::windows::core::HSTRING, value: bool) -> ::windows::core::Result<()>;
    fn AddBooleanWithFormat(&self, name: &::windows::core::HSTRING, value: bool, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddBooleanWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: bool, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddBooleanArray(&self, name: &::windows::core::HSTRING, value: &[<bool as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddBooleanArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[<bool as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddBooleanArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[<bool as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddString(&self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn AddStringWithFormat(&self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddStringWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddStringArray(&self, name: &::windows::core::HSTRING, value: &[<::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddStringArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[<::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddStringArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[<::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddGuid(&self, name: &::windows::core::HSTRING, value: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn AddGuidWithFormat(&self, name: &::windows::core::HSTRING, value: &::windows::core::GUID, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddGuidWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &::windows::core::GUID, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddGuidArray(&self, name: &::windows::core::HSTRING, value: &[<::windows::core::GUID as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddGuidArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[<::windows::core::GUID as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddGuidArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[<::windows::core::GUID as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddDateTime(&self, name: &::windows::core::HSTRING, value: &super::DateTime) -> ::windows::core::Result<()>;
    fn AddDateTimeWithFormat(&self, name: &::windows::core::HSTRING, value: &super::DateTime, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddDateTimeWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &super::DateTime, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddDateTimeArray(&self, name: &::windows::core::HSTRING, value: &[<super::DateTime as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddDateTimeArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[<super::DateTime as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddDateTimeArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[<super::DateTime as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddTimeSpan(&self, name: &::windows::core::HSTRING, value: &super::TimeSpan) -> ::windows::core::Result<()>;
    fn AddTimeSpanWithFormat(&self, name: &::windows::core::HSTRING, value: &super::TimeSpan, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddTimeSpanWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &super::TimeSpan, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddTimeSpanArray(&self, name: &::windows::core::HSTRING, value: &[<super::TimeSpan as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddTimeSpanArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[<super::TimeSpan as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddTimeSpanArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[<super::TimeSpan as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddPoint(&self, name: &::windows::core::HSTRING, value: &super::Point) -> ::windows::core::Result<()>;
    fn AddPointWithFormat(&self, name: &::windows::core::HSTRING, value: &super::Point, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddPointWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &super::Point, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddPointArray(&self, name: &::windows::core::HSTRING, value: &[<super::Point as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddPointArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[<super::Point as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddPointArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[<super::Point as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddSize(&self, name: &::windows::core::HSTRING, value: &super::Size) -> ::windows::core::Result<()>;
    fn AddSizeWithFormat(&self, name: &::windows::core::HSTRING, value: &super::Size, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddSizeWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &super::Size, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddSizeArray(&self, name: &::windows::core::HSTRING, value: &[<super::Size as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddSizeArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[<super::Size as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddSizeArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[<super::Size as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddRect(&self, name: &::windows::core::HSTRING, value: &super::Rect) -> ::windows::core::Result<()>;
    fn AddRectWithFormat(&self, name: &::windows::core::HSTRING, value: &super::Rect, format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddRectWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &super::Rect, format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
    fn AddRectArray(&self, name: &::windows::core::HSTRING, value: &[<super::Rect as ::windows::core::DefaultType>::DefaultType]) -> ::windows::core::Result<()>;
    fn AddRectArrayWithFormat(&self, name: &::windows::core::HSTRING, value: &[<super::Rect as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat) -> ::windows::core::Result<()>;
    fn AddRectArrayWithFormatAndTags(&self, name: &::windows::core::HSTRING, value: &[<super::Rect as ::windows::core::DefaultType>::DefaultType], format: LoggingFieldFormat, tags: i32) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoggingOptionsImpl: Sized {
    fn Keywords(&self) -> ::windows::core::Result<i64>;
    fn SetKeywords(&self, value: i64) -> ::windows::core::Result<()>;
    fn Tags(&self) -> ::windows::core::Result<i32>;
    fn SetTags(&self, value: i32) -> ::windows::core::Result<()>;
    fn Task(&self) -> ::windows::core::Result<i16>;
    fn SetTask(&self, value: i16) -> ::windows::core::Result<()>;
    fn Opcode(&self) -> ::windows::core::Result<LoggingOpcode>;
    fn SetOpcode(&self, value: LoggingOpcode) -> ::windows::core::Result<()>;
    fn ActivityId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetActivityId(&self, value: &::windows::core::GUID) -> ::windows::core::Result<()>;
    fn RelatedActivityId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetRelatedActivityId(&self, value: &::windows::core::GUID) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoggingOptionsFactoryImpl: Sized {
    fn CreateWithKeywords(&self, keywords: i64) -> ::windows::core::Result<LoggingOptions>;
}
pub trait ILoggingSessionImpl: Sized + IClosableImpl {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SaveToFileAsync(&self, folder: &::core::option::Option<super::super::Storage::IStorageFolder>, filename: &::windows::core::HSTRING) -> ::windows::core::Result<super::IAsyncOperation<super::super::Storage::StorageFile>>;
    fn AddLoggingChannel(&self, loggingchannel: &::core::option::Option<ILoggingChannel>) -> ::windows::core::Result<()>;
    fn AddLoggingChannelWithLevel(&self, loggingchannel: &::core::option::Option<ILoggingChannel>, maxlevel: LoggingLevel) -> ::windows::core::Result<()>;
    fn RemoveLoggingChannel(&self, loggingchannel: &::core::option::Option<ILoggingChannel>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ILoggingSessionFactoryImpl: Sized {
    fn Create(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<LoggingSession>;
}
pub trait ILoggingTargetImpl: Sized {
    fn IsEnabled(&self) -> ::windows::core::Result<bool>;
    fn IsEnabledWithLevel(&self, level: LoggingLevel) -> ::windows::core::Result<bool>;
    fn IsEnabledWithLevelAndKeywords(&self, level: LoggingLevel, keywords: i64) -> ::windows::core::Result<bool>;
    fn LogEvent(&self, eventname: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn LogEventWithFields(&self, eventname: &::windows::core::HSTRING, fields: &::core::option::Option<LoggingFields>) -> ::windows::core::Result<()>;
    fn LogEventWithFieldsAndLevel(&self, eventname: &::windows::core::HSTRING, fields: &::core::option::Option<LoggingFields>, level: LoggingLevel) -> ::windows::core::Result<()>;
    fn LogEventWithFieldsAndOptions(&self, eventname: &::windows::core::HSTRING, fields: &::core::option::Option<LoggingFields>, level: LoggingLevel, options: &::core::option::Option<LoggingOptions>) -> ::windows::core::Result<()>;
    fn StartActivity(&self, starteventname: &::windows::core::HSTRING) -> ::windows::core::Result<LoggingActivity>;
    fn StartActivityWithFields(&self, starteventname: &::windows::core::HSTRING, fields: &::core::option::Option<LoggingFields>) -> ::windows::core::Result<LoggingActivity>;
    fn StartActivityWithFieldsAndLevel(&self, starteventname: &::windows::core::HSTRING, fields: &::core::option::Option<LoggingFields>, level: LoggingLevel) -> ::windows::core::Result<LoggingActivity>;
    fn StartActivityWithFieldsAndOptions(&self, starteventname: &::windows::core::HSTRING, fields: &::core::option::Option<LoggingFields>, level: LoggingLevel, options: &::core::option::Option<LoggingOptions>) -> ::windows::core::Result<LoggingActivity>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ITracingStatusChangedEventArgsImpl: Sized {
    fn Enabled(&self) -> ::windows::core::Result<bool>;
    fn TraceLevel(&self) -> ::windows::core::Result<CausalityTraceLevel>;
}
