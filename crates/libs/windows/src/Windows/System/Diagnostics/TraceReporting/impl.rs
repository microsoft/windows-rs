#[cfg(feature = "implement_exclusive")]
pub trait IPlatformDiagnosticActionsStaticsImpl: Sized {
    fn IsScenarioEnabled(&self, scenarioid: &::windows::core::GUID) -> ::windows::core::Result<bool>;
    fn TryEscalateScenario(&self, scenarioid: &::windows::core::GUID, escalationtype: PlatformDiagnosticEscalationType, outputdirectory: &::windows::core::HSTRING, timestampoutputdirectory: bool, forceescalationupload: bool, triggers: &::core::option::Option<super::super::super::Foundation::Collections::IMapView<::windows::core::HSTRING, ::windows::core::HSTRING>>) -> ::windows::core::Result<bool>;
    fn DownloadLatestSettingsForNamespace(&self, partner: &::windows::core::HSTRING, feature: &::windows::core::HSTRING, isscenarionamespace: bool, downloadovercostednetwork: bool, downloadoverbattery: bool) -> ::windows::core::Result<PlatformDiagnosticActionState>;
    fn GetActiveScenarioList(&self) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<::windows::core::GUID>>;
    fn ForceUpload(&self, latency: PlatformDiagnosticEventBufferLatencies, uploadovercostednetwork: bool, uploadoverbattery: bool) -> ::windows::core::Result<PlatformDiagnosticActionState>;
    fn IsTraceRunning(&self, slottype: PlatformDiagnosticTraceSlotType, scenarioid: &::windows::core::GUID, traceprofilehash: u64) -> ::windows::core::Result<PlatformDiagnosticTraceSlotState>;
    fn GetActiveTraceRuntime(&self, slottype: PlatformDiagnosticTraceSlotType) -> ::windows::core::Result<PlatformDiagnosticTraceRuntimeInfo>;
    fn GetKnownTraceList(&self, slottype: PlatformDiagnosticTraceSlotType) -> ::windows::core::Result<super::super::super::Foundation::Collections::IVectorView<PlatformDiagnosticTraceInfo>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlatformDiagnosticTraceInfoImpl: Sized {
    fn ScenarioId(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn ProfileHash(&self) -> ::windows::core::Result<u64>;
    fn IsExclusive(&self) -> ::windows::core::Result<bool>;
    fn IsAutoLogger(&self) -> ::windows::core::Result<bool>;
    fn MaxTraceDurationFileTime(&self) -> ::windows::core::Result<i64>;
    fn Priority(&self) -> ::windows::core::Result<PlatformDiagnosticTracePriority>;
}
#[cfg(feature = "implement_exclusive")]
pub trait IPlatformDiagnosticTraceRuntimeInfoImpl: Sized {
    fn RuntimeFileTime(&self) -> ::windows::core::Result<i64>;
    fn EtwRuntimeFileTime(&self) -> ::windows::core::Result<i64>;
}
