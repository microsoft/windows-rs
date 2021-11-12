#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct AsyncCausalityTracer(pub *mut ::core::ffi::c_void);
pub struct CausalityRelation(i32);
pub struct CausalitySource(i32);
pub struct CausalitySynchronousWork(i32);
pub struct CausalityTraceLevel(i32);
#[repr(transparent)]
pub struct ErrorDetails(pub *mut ::core::ffi::c_void);
pub struct ErrorOptions(i32);
#[repr(transparent)]
pub struct FileLoggingSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IAsyncCausalityTracerStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IErrorDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IErrorDetailsStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IErrorReportingSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileLoggingSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IFileLoggingSessionFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILogFileGeneratedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILoggingActivity(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILoggingActivity2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILoggingActivityFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILoggingChannel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILoggingChannel2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILoggingChannelFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILoggingChannelFactory2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILoggingChannelOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILoggingChannelOptionsFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILoggingFields(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILoggingOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILoggingOptionsFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILoggingSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILoggingSessionFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ILoggingTarget(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ITracingStatusChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LogFileGeneratedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LoggingActivity(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LoggingChannel(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LoggingChannelOptions(pub *mut ::core::ffi::c_void);
pub struct LoggingFieldFormat(i32);
#[repr(transparent)]
pub struct LoggingFields(pub *mut ::core::ffi::c_void);
pub struct LoggingLevel(i32);
pub struct LoggingOpcode(i32);
#[repr(transparent)]
pub struct LoggingOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LoggingSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RuntimeBrokerErrorSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TracingStatusChangedEventArgs(pub *mut ::core::ffi::c_void);
