#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn IPlatformDiagnosticActionsStatics();
    fn IPlatformDiagnosticTraceInfo();
    fn IPlatformDiagnosticTraceRuntimeInfo();
    fn PlatformDiagnosticActionState();
    fn PlatformDiagnosticActions();
    fn PlatformDiagnosticEscalationType();
    fn PlatformDiagnosticEventBufferLatencies();
    fn PlatformDiagnosticTraceInfo();
    fn PlatformDiagnosticTracePriority();
    fn PlatformDiagnosticTraceRuntimeInfo();
    fn PlatformDiagnosticTraceSlotState();
    fn PlatformDiagnosticTraceSlotType();
}
