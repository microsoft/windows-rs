#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct CausalityRelation(pub i32);
impl CausalityRelation {
    pub const AssignDelegate: CausalityRelation = CausalityRelation(0i32);
    pub const Join: CausalityRelation = CausalityRelation(1i32);
    pub const Choice: CausalityRelation = CausalityRelation(2i32);
    pub const Cancel: CausalityRelation = CausalityRelation(3i32);
    pub const Error: CausalityRelation = CausalityRelation(4i32);
}
#[repr(transparent)]
pub struct CausalitySource(pub i32);
impl CausalitySource {
    pub const Application: CausalitySource = CausalitySource(0i32);
    pub const Library: CausalitySource = CausalitySource(1i32);
    pub const System: CausalitySource = CausalitySource(2i32);
}
#[repr(transparent)]
pub struct CausalitySynchronousWork(pub i32);
impl CausalitySynchronousWork {
    pub const CompletionNotification: CausalitySynchronousWork = CausalitySynchronousWork(0i32);
    pub const ProgressNotification: CausalitySynchronousWork = CausalitySynchronousWork(1i32);
    pub const Execution: CausalitySynchronousWork = CausalitySynchronousWork(2i32);
}
#[repr(transparent)]
pub struct CausalityTraceLevel(pub i32);
impl CausalityTraceLevel {
    pub const Required: CausalityTraceLevel = CausalityTraceLevel(0i32);
    pub const Important: CausalityTraceLevel = CausalityTraceLevel(1i32);
    pub const Verbose: CausalityTraceLevel = CausalityTraceLevel(2i32);
}
#[repr(transparent)]
pub struct ErrorDetails(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ErrorOptions(pub u32);
impl ErrorOptions {
    pub const None: ErrorOptions = ErrorOptions(0u32);
    pub const SuppressExceptions: ErrorOptions = ErrorOptions(1u32);
    pub const ForceExceptions: ErrorOptions = ErrorOptions(2u32);
    pub const UseSetErrorInfo: ErrorOptions = ErrorOptions(4u32);
    pub const SuppressSetErrorInfo: ErrorOptions = ErrorOptions(8u32);
}
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
#[repr(transparent)]
pub struct LoggingFields(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LoggingLevel(pub i32);
impl LoggingLevel {
    pub const Verbose: LoggingLevel = LoggingLevel(0i32);
    pub const Information: LoggingLevel = LoggingLevel(1i32);
    pub const Warning: LoggingLevel = LoggingLevel(2i32);
    pub const Error: LoggingLevel = LoggingLevel(3i32);
    pub const Critical: LoggingLevel = LoggingLevel(4i32);
}
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
#[repr(transparent)]
pub struct LoggingOptions(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct LoggingSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct RuntimeBrokerErrorSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct TracingStatusChangedEventArgs(pub *mut ::core::ffi::c_void);
