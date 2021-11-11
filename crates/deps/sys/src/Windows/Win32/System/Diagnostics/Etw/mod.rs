#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn CloseTrace();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ControlTraceA();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ControlTraceW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateTraceInstanceId();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CveEventWrite();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn EnableTrace();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn EnableTraceEx();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn EnableTraceEx2();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumerateTraceGuids();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn EnumerateTraceGuidsEx();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EventAccessControl();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn EventAccessQuery();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn EventAccessRemove();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn EventActivityIdControl();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EventEnabled();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EventProviderEnabled();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn EventRegister();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn EventSetInformation();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn EventUnregister();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn EventWrite();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn EventWriteEx();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EventWriteString();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn EventWriteTransfer();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FlushTraceA();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FlushTraceW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn GetTraceEnableFlags();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn GetTraceEnableLevel();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn GetTraceLoggerHandle();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`, `Win32_System_Time`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
    pub fn OpenTraceA();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`, `Win32_System_Time`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Time"))]
    pub fn OpenTraceW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ProcessTrace();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryAllTracesA();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryAllTracesW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryTraceA();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn QueryTraceProcessingHandle();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn QueryTraceW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterTraceGuidsA();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterTraceGuidsW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn RemoveTraceCallback();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn SetTraceCallback();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StartTraceA();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StartTraceW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StopTraceA();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StopTraceW();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TdhAggregatePayloadFilters();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn TdhCleanupPayloadEventFilterDescriptor();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn TdhCloseDecodingHandle();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TdhCreatePayloadFilter();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn TdhDeletePayloadFilter();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn TdhEnumerateManifestProviderEvents();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn TdhEnumerateProviderFieldInformation();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn TdhEnumerateProviderFilters();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn TdhEnumerateProviders();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn TdhEnumerateProvidersForDecodingSource();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TdhFormatProperty();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn TdhGetDecodingParameter();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn TdhGetEventInformation();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TdhGetEventMapInformation();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn TdhGetManifestEventInformation();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn TdhGetProperty();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn TdhGetPropertySize();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn TdhGetWppMessage();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TdhGetWppProperty();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TdhLoadManifest();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TdhLoadManifestFromBinary();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn TdhLoadManifestFromMemory();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn TdhOpenDecodingHandle();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn TdhQueryProviderFieldInformation();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn TdhSetDecodingParameter();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TdhUnloadManifest();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn TdhUnloadManifestFromMemory();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn TraceEvent();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TraceEventInstance();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn TraceMessage();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn TraceMessageVa();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn TraceQueryInformation();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn TraceSetInformation();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`*"]
    pub fn UnregisterTraceGuids();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdateTraceA();
    #[doc = "*Required features: `Win32_System_Diagnostics_Etw`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdateTraceW();
}
