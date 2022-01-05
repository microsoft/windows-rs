pub trait ITraceEventImpl: Sized {
    fn Clone();
    fn GetUserContext();
    fn GetEventRecord();
    fn SetPayload();
    fn SetEventDescriptor();
    fn SetProcessId();
    fn SetProcessorIndex();
    fn SetThreadId();
    fn SetThreadTimes();
    fn SetActivityId();
    fn SetTimeStamp();
    fn SetProviderId();
}
pub trait ITraceEventCallbackImpl: Sized {
    fn OnBeginProcessTrace();
    fn OnFinalizeProcessTrace();
    fn OnEvent();
}
pub trait ITraceReloggerImpl: Sized {
    fn AddLogfileTraceStream();
    fn AddRealtimeTraceStream();
    fn RegisterCallback();
    fn Inject();
    fn CreateEventInstance();
    fn ProcessTrace();
    fn SetOutputFilename();
    fn SetCompressionMode();
    fn Cancel();
}
