#[cfg(feature = "propsys")]
windows_link::link!("mf.dll" "system" fn CreateNamedPropertyStore(ppstore : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "propsys")]
windows_link::link!("mfplat.dll" "system" fn CreatePropertyStore(ppstore : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "mfobjects")]
windows_link::link!("mf.dll" "system" fn MFCreate3GPMediaSink(pibytestream : *mut core::ffi::c_void, pvideomediatype : *mut core::ffi::c_void, paudiomediatype : *mut core::ffi::c_void, ppimediasink : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "mfobjects")]
windows_link::link!("mf.dll" "system" fn MFCreateAC3MediaSink(ptargetbytestream : *mut core::ffi::c_void, paudiomediatype : *mut core::ffi::c_void, ppmediasink : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "mfobjects")]
windows_link::link!("mf.dll" "system" fn MFCreateADTSMediaSink(ptargetbytestream : *mut core::ffi::c_void, paudiomediatype : *mut core::ffi::c_void, ppmediasink : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "mfobjects")]
windows_link::link!("mfsrcsnk.dll" "system" fn MFCreateAVIMediaSink(pibytestream : *mut core::ffi::c_void, pvideomediatype : *mut core::ffi::c_void, paudiomediatype : *mut core::ffi::c_void, ppimediasink : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "mfobjects")]
windows_link::link!("mf.dll" "system" fn MFCreateAggregateSource(psourcecollection : *mut core::ffi::c_void, ppaggsource : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "mfobjects")]
windows_link::link!("mf.dll" "system" fn MFCreateAudioRenderer(paudioattributes : *mut core::ffi::c_void, ppsink : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "mfobjects")]
windows_link::link!("mf.dll" "system" fn MFCreateAudioRendererActivate(ppactivate : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("mfsensorgroup.dll" "system" fn MFCreateCameraControlMonitor(symboliclink : windows_sys::core::PCWSTR, callback : *mut core::ffi::c_void, ppcameracontrolmonitor : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("mfsensorgroup.dll" "system" fn MFCreateCameraOcclusionStateMonitor(symboliclink : windows_sys::core::PCWSTR, callback : *mut core::ffi::c_void, occlusionstatemonitor : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "mfobjects")]
windows_link::link!("mfplat.dll" "system" fn MFCreateContentDecryptorContext(guidmediaprotectionsystemid : *const windows_sys::core::GUID, pd3dmanager : *mut core::ffi::c_void, pcontentprotectiondevice : *mut core::ffi::c_void, ppcontentdecryptorcontext : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("mfplat.dll" "system" fn MFCreateContentProtectionDevice(protectionsystemid : *const windows_sys::core::GUID, contentprotectiondevice : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("mf.dll" "system" fn MFCreateCredentialCache(ppcache : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "mfobjects")]
windows_link::link!("mf.dll" "system" fn MFCreateDeviceSource(pattributes : *mut core::ffi::c_void, ppsource : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "mfobjects")]
windows_link::link!("mf.dll" "system" fn MFCreateDeviceSourceActivate(pattributes : *mut core::ffi::c_void, ppactivate : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("mfcore.dll" "system" fn MFCreateExtendedCameraIntrinsicModel(distortionmodeltype : MFCameraIntrinsic_DistortionModelType, ppextendedcameraintrinsicmodel : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("mfcore.dll" "system" fn MFCreateExtendedCameraIntrinsics(ppextendedcameraintrinsics : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "mfobjects")]
windows_link::link!("mf.dll" "system" fn MFCreateFMPEG4MediaSink(pibytestream : *mut core::ffi::c_void, pvideomediatype : *mut core::ffi::c_void, paudiomediatype : *mut core::ffi::c_void, ppimediasink : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "mfobjects", feature = "objidlbase"))]
windows_link::link!("mfplat.dll" "system" fn MFCreateMFByteStreamOnStream(pstream : *mut core::ffi::c_void, ppbytestream : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "mfobjects")]
windows_link::link!("mfplat.dll" "system" fn MFCreateMFByteStreamOnStreamEx(punkstream : *mut core::ffi::c_void, ppbytestream : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "mfobjects")]
windows_link::link!("mf.dll" "system" fn MFCreateMP3MediaSink(ptargetbytestream : *mut core::ffi::c_void, ppmediasink : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "mfobjects")]
windows_link::link!("mf.dll" "system" fn MFCreateMPEG4MediaSink(pibytestream : *mut core::ffi::c_void, pvideomediatype : *mut core::ffi::c_void, paudiomediatype : *mut core::ffi::c_void, ppimediasink : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "mfobjects")]
windows_link::link!("mf.dll" "system" fn MFCreateMediaSession(pconfiguration : *mut core::ffi::c_void, ppmediasession : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "mfobjects")]
windows_link::link!("mfplat.dll" "system" fn MFCreateMediaTypeFromProperties(punkstream : *mut core::ffi::c_void, ppmediatype : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "mfobjects")]
windows_link::link!("mf.dll" "system" fn MFCreateMuxSink(guidoutputsubtype : windows_sys::core::GUID, poutputattributes : *mut core::ffi::c_void, poutputbytestream : *mut core::ffi::c_void, ppmuxsink : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("mf.dll" "system" fn MFCreateNetSchemePlugin(riid : *const windows_sys::core::GUID, ppvhandler : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "mfobjects")]
windows_link::link!("mf.dll" "system" fn MFCreatePMPMediaSession(dwcreationflags : u32, pconfiguration : *mut core::ffi::c_void, ppmediasession : *mut *mut core::ffi::c_void, ppenableractivate : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("mf.dll" "system" fn MFCreatePMPServer(dwcreationflags : u32, pppmpserver : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("mf.dll" "system" fn MFCreatePresentationClock(pppresentationclock : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "mfobjects")]
windows_link::link!("mfplat.dll" "system" fn MFCreatePresentationDescriptor(cstreamdescriptors : u32, apstreamdescriptors : *const *mut core::ffi::c_void, pppresentationdescriptor : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "mfobjects")]
windows_link::link!("mfplat.dll" "system" fn MFCreatePropertiesFromMediaType(pmediatype : *mut core::ffi::c_void, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("mf.dll" "system" fn MFCreateProtectedEnvironmentAccess(ppaccess : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "propsys")]
windows_link::link!("mf.dll" "system" fn MFCreateProxyLocator(pszprotocol : windows_sys::core::PCWSTR, pproxyconfig : *mut core::ffi::c_void, ppproxylocator : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("mfsensorgroup.dll" "system" fn MFCreateRelativePanelWatcher(videodeviceid : windows_sys::core::PCWSTR, displaymonitordeviceid : windows_sys::core::PCWSTR, pprelativepanelwatcher : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("mf.dll" "system" fn MFCreateRemoteDesktopPlugin(ppplugin : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "mftransform")]
windows_link::link!("mf.dll" "system" fn MFCreateSampleCopierMFT(ppcopiermft : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "mfobjects")]
windows_link::link!("mf.dll" "system" fn MFCreateSampleGrabberSinkActivate(pimfmediatype : *mut core::ffi::c_void, pimfsamplegrabbersinkcallback : *mut core::ffi::c_void, ppiactivate : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("mfsensorgroup.dll" "system" fn MFCreateSensorActivityMonitor(pcallback : *mut core::ffi::c_void, ppactivitymonitor : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("mfsensorgroup.dll" "system" fn MFCreateSensorGroup(sensorgroupsymboliclink : windows_sys::core::PCWSTR, ppsensorgroup : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("mfsensorgroup.dll" "system" fn MFCreateSensorProfile(profiletype : *const windows_sys::core::GUID, profileindex : u32, constraints : windows_sys::core::PCWSTR, ppprofile : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("mfsensorgroup.dll" "system" fn MFCreateSensorProfileCollection(ppsensorprofile : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "mfobjects")]
windows_link::link!("mfsensorgroup.dll" "system" fn MFCreateSensorStream(streamid : u32, pattributes : *mut core::ffi::c_void, pmediatypecollection : *mut core::ffi::c_void, ppstream : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("mf.dll" "system" fn MFCreateSequencerSegmentOffset(dwid : MFSequencerElementId, hnsoffset : MFTIME, pvarsegmentoffset : *mut super::PROPVARIANT) -> windows_sys::core::HRESULT);
windows_link::link!("mf.dll" "system" fn MFCreateSequencerSource(preserved : *mut core::ffi::c_void, ppsequencersource : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("mf.dll" "system" fn MFCreateSimpleTypeHandler(pphandler : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("mfplat.dll" "system" fn MFCreateSourceResolver(ppisourceresolver : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("mf.dll" "system" fn MFCreateStandardQualityManager(ppqualitymanager : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "mfobjects")]
windows_link::link!("mfplat.dll" "system" fn MFCreateStreamDescriptor(dwstreamidentifier : u32, cmediatypes : u32, apmediatypes : *const *mut core::ffi::c_void, ppdescriptor : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "mfobjects", feature = "objidlbase"))]
windows_link::link!("mfplat.dll" "system" fn MFCreateStreamOnMFByteStream(pbytestream : *mut core::ffi::c_void, ppstream : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "mfobjects")]
windows_link::link!("mfplat.dll" "system" fn MFCreateStreamOnMFByteStreamEx(pbytestream : *mut core::ffi::c_void, riid : *const windows_sys::core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("mfplat.dll" "system" fn MFCreateSystemTimeSource(ppsystemtimesource : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("mf.dll" "system" fn MFCreateTopoLoader(ppobj : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "mfobjects")]
windows_link::link!("mf.dll" "system" fn MFCreateTopology(pptopo : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "mfobjects")]
windows_link::link!("mf.dll" "system" fn MFCreateTopologyNode(nodetype : MF_TOPOLOGY_TYPE, ppnode : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("mfplat.dll" "system" fn MFCreateTrackedSample(ppmfsample : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("mf.dll" "system" fn MFCreateTranscodeProfile(pptranscodeprofile : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "mfobjects")]
windows_link::link!("mf.dll" "system" fn MFCreateTranscodeSinkActivate(ppactivate : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "mfobjects")]
windows_link::link!("mf.dll" "system" fn MFCreateTranscodeTopology(psrc : *mut core::ffi::c_void, pwszoutputfilepath : windows_sys::core::PCWSTR, pprofile : *mut core::ffi::c_void, pptranscodetopo : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "mfobjects")]
windows_link::link!("mf.dll" "system" fn MFCreateTranscodeTopologyFromByteStream(psrc : *mut core::ffi::c_void, poutputstream : *mut core::ffi::c_void, pprofile : *mut core::ffi::c_void, pptranscodetopo : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "mfobjects", feature = "windef"))]
windows_link::link!("mf.dll" "system" fn MFCreateVideoRendererActivate(hwndvideo : super::HWND, ppactivate : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "mfobjects")]
windows_link::link!("mfsrcsnk.dll" "system" fn MFCreateWAVEMediaSink(ptargetbytestream : *mut core::ffi::c_void, paudiomediatype : *mut core::ffi::c_void, ppmediasink : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "mfobjects")]
windows_link::link!("mfplat.dll" "system" fn MFDeserializePresentationDescriptor(cbdata : u32, pbdata : *const u8, pppd : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "mfobjects")]
windows_link::link!("mf.dll" "system" fn MFEnumDeviceSources(pattributes : *mut core::ffi::c_void, pppsourceactivate : *mut *mut *mut core::ffi::c_void, pcsourceactivate : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("mf.dll" "system" fn MFGetLocalId(verifier : *const u8, size : u32, id : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("mf.dll" "system" fn MFGetService(punkobject : *mut core::ffi::c_void, guidservice : *const windows_sys::core::GUID, riid : *const windows_sys::core::GUID, ppvobject : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("mfplat.dll" "system" fn MFGetSupportedMimeTypes(ppropvarmimetypearray : *mut super::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("mfplat.dll" "system" fn MFGetSupportedSchemes(ppropvarschemearray : *mut super::PROPVARIANT) -> windows_sys::core::HRESULT);
windows_link::link!("mf.dll" "system" fn MFGetSystemId(ppid : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("mfplat.dll" "system" fn MFGetSystemTime() -> MFTIME);
#[cfg(feature = "mfobjects")]
windows_link::link!("mf.dll" "system" fn MFGetTopoNodeCurrentType(pnode : *mut core::ffi::c_void, dwstreamindex : u32, foutput : windows_sys::core::BOOL, pptype : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("mfplat.dll" "system" fn MFIsContentProtectionDeviceSupported(protectionsystemid : *const windows_sys::core::GUID, issupported : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
windows_link::link!("mf.dll" "system" fn MFLoadSignedLibrary(pszname : windows_sys::core::PCWSTR, pplib : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "mfobjects")]
windows_link::link!("mf.dll" "system" fn MFRequireProtectedEnvironment(ppresentationdescriptor : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "mfobjects")]
windows_link::link!("mfplat.dll" "system" fn MFSerializePresentationDescriptor(ppd : *mut core::ffi::c_void, pcbdata : *mut u32, ppbdata : *mut *mut u8) -> windows_sys::core::HRESULT);
windows_link::link!("mf.dll" "system" fn MFShutdownObject(punk : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "mfobjects")]
windows_link::link!("mf.dll" "system" fn MFTranscodeGetAudioOutputAvailableTypes(guidsubtype : *const windows_sys::core::GUID, dwmftflags : u32, pcodecconfig : *mut core::ffi::c_void, ppavailabletypes : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct ASF_FLAT_PICTURE {
    pub bPictureType: u8,
    pub dwDataLen: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct ASF_FLAT_SYNCHRONISED_LYRICS {
    pub bTimeStampFormat: u8,
    pub bContentType: u8,
    pub dwLyricsLen: u32,
}
pub const CLSID_MPEG2ByteStreamPlugin: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x40871c59_ab40_471f_8dc3_1f259d862479);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DetectedFaceBound {
    pub sizeInBytes: u32,
    pub normalizedXPosition: f32,
    pub normalizedYPosition: f32,
    pub normalizedWidth: f32,
    pub normalizedHeight: f32,
    pub confidenceValue: i32,
    pub flags: u64,
}
pub const MEDIASINK_CANNOT_MATCH_CLOCK: u32 = 2;
pub const MEDIASINK_CAN_PREROLL: u32 = 16;
pub const MEDIASINK_CLOCK_REQUIRED: u32 = 8;
pub const MEDIASINK_FIXED_STREAMS: u32 = 1;
pub const MEDIASINK_RATELESS: u32 = 4;
pub const MEDIASINK_REQUIRE_REFERENCE_MEDIATYPE: u32 = 32;
pub type MFAudioConstriction = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MFBYTESTREAM_BUFFERING_PARAMS {
    pub cbTotalFileSize: u64,
    pub cbPlayableDataSize: u64,
    pub prgBuckets: *mut MF_LEAKY_BUCKET_PAIR,
    pub cBuckets: u32,
    pub qwNetBufferingTime: u64,
    pub qwExtraBufferingTimeDuringSeek: u64,
    pub qwPlayDuration: u64,
    pub dRate: f32,
}
impl Default for MFBYTESTREAM_BUFFERING_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type MFCLOCK_CHARACTERISTICS_FLAGS = i32;
pub const MFCLOCK_CHARACTERISTICS_FLAG_ALWAYS_RUNNING: MFCLOCK_CHARACTERISTICS_FLAGS = 4;
pub const MFCLOCK_CHARACTERISTICS_FLAG_FREQUENCY_10MHZ: MFCLOCK_CHARACTERISTICS_FLAGS = 2;
pub const MFCLOCK_CHARACTERISTICS_FLAG_IS_SYSTEM_CLOCK: MFCLOCK_CHARACTERISTICS_FLAGS = 8;
pub const MFCLOCK_FREQUENCY_HNS: u32 = 10000000;
pub const MFCLOCK_JITTER_DPC: u32 = 4000;
pub const MFCLOCK_JITTER_ISR: u32 = 1000;
pub const MFCLOCK_JITTER_PASSIVE: u32 = 10000;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MFCLOCK_PROPERTIES {
    pub qwCorrelationRate: u64,
    pub guidClockId: windows_sys::core::GUID,
    pub dwClockFlags: u32,
    pub qwClockFrequency: u64,
    pub dwClockTolerance: u32,
    pub dwClockJitter: u32,
}
pub type MFCLOCK_RELATIONAL_FLAGS = i32;
pub const MFCLOCK_RELATIONAL_FLAG_JITTER_NEVER_AHEAD: MFCLOCK_RELATIONAL_FLAGS = 1;
pub type MFCLOCK_STATE = i32;
pub const MFCLOCK_STATE_INVALID: MFCLOCK_STATE = 0;
pub const MFCLOCK_STATE_PAUSED: MFCLOCK_STATE = 3;
pub const MFCLOCK_STATE_RUNNING: MFCLOCK_STATE = 1;
pub const MFCLOCK_STATE_STOPPED: MFCLOCK_STATE = 2;
pub const MFCLOCK_TOLERANCE_UNKNOWN: u32 = 50000;
pub const MFCONTENTPROTECTIONDEVICE_FUNCTIONID_START: u32 = 67108864;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MFCONTENTPROTECTIONDEVICE_INPUT_DATA {
    pub HWProtectionFunctionID: u32,
    pub PrivateDataByteCount: u32,
    pub HWProtectionDataByteCount: u32,
    pub Reserved: u32,
    pub InputData: [u8; 4],
}
impl Default for MFCONTENTPROTECTIONDEVICE_INPUT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MFCONTENTPROTECTIONDEVICE_OUTPUT_DATA {
    pub PrivateDataByteCount: u32,
    pub MaxHWProtectionDataByteCount: u32,
    pub HWProtectionDataByteCount: u32,
    pub Status: windows_sys::core::HRESULT,
    pub TransportTimeInHundredsOfNanoseconds: i64,
    pub ExecutionTimeInHundredsOfNanoseconds: i64,
    pub OutputData: [u8; 4],
}
impl Default for MFCONTENTPROTECTIONDEVICE_OUTPUT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MFCONTENTPROTECTIONDEVICE_REALTIMECLIENT_DATA {
    pub TaskIndex: u32,
    pub ClassName: [u16; 260],
    pub BasePriority: i32,
}
impl Default for MFCONTENTPROTECTIONDEVICE_REALTIMECLIENT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MFCONTENTPROTECTIONDEVICE_REALTIMECLIENT_DATA_FUNCTIONID: u32 = 67108864;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MFCameraIntrinsic_CameraModel {
    pub FocalLength_x: f32,
    pub FocalLength_y: f32,
    pub PrincipalPoint_x: f32,
    pub PrincipalPoint_y: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MFCameraIntrinsic_DistortionModel6KT {
    pub Radial_k1: f32,
    pub Radial_k2: f32,
    pub Radial_k3: f32,
    pub Radial_k4: f32,
    pub Radial_k5: f32,
    pub Radial_k6: f32,
    pub Tangential_p1: f32,
    pub Tangential_p2: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MFCameraIntrinsic_DistortionModelArcTan {
    pub Radial_k0: f32,
    pub DistortionCenter_x: f32,
    pub DistortionCenter_y: f32,
    pub Tangential_x: f32,
    pub Tangential_y: f32,
}
pub type MFCameraIntrinsic_DistortionModelType = i32;
pub const MFCameraIntrinsic_DistortionModelType_6KT: MFCameraIntrinsic_DistortionModelType = 0;
pub const MFCameraIntrinsic_DistortionModelType_ArcTan: MFCameraIntrinsic_DistortionModelType = 1;
pub type MFCameraOcclusionState = u32;
pub const MFCameraOcclusionState_OccludedByCameraHardware: MFCameraOcclusionState = 2;
pub const MFCameraOcclusionState_OccludedByLid: MFCameraOcclusionState = 1;
pub const MFCameraOcclusionState_Open: MFCameraOcclusionState = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MFExtendedCameraIntrinsic_IntrinsicModel {
    pub Width: u32,
    pub Height: u32,
    pub SplitFrameId: u32,
    pub CameraModel: MFCameraIntrinsic_CameraModel,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MFINPUTTRUSTAUTHORITY_ACCESS_ACTION {
    pub Action: MFPOLICYMANAGER_ACTION,
    pub pbTicket: *mut u8,
    pub cbTicket: u32,
}
impl Default for MFINPUTTRUSTAUTHORITY_ACCESS_ACTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MFINPUTTRUSTAUTHORITY_ACCESS_PARAMS {
    pub dwSize: u32,
    pub dwVer: u32,
    pub cbSignatureOffset: u32,
    pub cbSignatureSize: u32,
    pub cbExtensionOffset: u32,
    pub cbExtensionSize: u32,
    pub cActions: u32,
    pub rgOutputActions: [MFINPUTTRUSTAUTHORITY_ACCESS_ACTION; 1],
}
impl Default for MFINPUTTRUSTAUTHORITY_ACCESS_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MFMEDIASOURCE_CAN_PAUSE: MFMEDIASOURCE_CHARACTERISTICS = 4;
pub const MFMEDIASOURCE_CAN_SEEK: MFMEDIASOURCE_CHARACTERISTICS = 2;
pub const MFMEDIASOURCE_CAN_SKIPBACKWARD: MFMEDIASOURCE_CHARACTERISTICS = 64;
pub const MFMEDIASOURCE_CAN_SKIPFORWARD: MFMEDIASOURCE_CHARACTERISTICS = 32;
pub type MFMEDIASOURCE_CHARACTERISTICS = i32;
pub const MFMEDIASOURCE_DOES_NOT_USE_NETWORK: MFMEDIASOURCE_CHARACTERISTICS = 128;
pub const MFMEDIASOURCE_HAS_MULTIPLE_PRESENTATIONS: MFMEDIASOURCE_CHARACTERISTICS = 16;
pub const MFMEDIASOURCE_HAS_SLOW_SEEK: MFMEDIASOURCE_CHARACTERISTICS = 8;
pub const MFMEDIASOURCE_IS_LIVE: MFMEDIASOURCE_CHARACTERISTICS = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MFMediaKeyStatus {
    pub pbKeyId: *mut u8,
    pub cbKeyId: u32,
    pub eMediaKeyStatus: MF_MEDIAKEY_STATUS,
}
impl Default for MFMediaKeyStatus {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MFNETSOURCE_AVGBANDWIDTHBPS_ID: MFNETSOURCE_STATISTICS_IDS = 8;
pub const MFNETSOURCE_BUFFERINGCOUNT_ID: MFNETSOURCE_STATISTICS_IDS = 21;
pub const MFNETSOURCE_BUFFERPROGRESS_ID: MFNETSOURCE_STATISTICS_IDS = 17;
pub const MFNETSOURCE_BUFFERSIZE_ID: MFNETSOURCE_STATISTICS_IDS = 16;
pub const MFNETSOURCE_BYTESRECEIVED_ID: MFNETSOURCE_STATISTICS_IDS = 9;
pub const MFNETSOURCE_CACHE_ACTIVE_COMPLETE: MFNETSOURCE_CACHE_STATE = 2;
pub const MFNETSOURCE_CACHE_ACTIVE_WRITING: MFNETSOURCE_CACHE_STATE = 1;
pub type MFNETSOURCE_CACHE_STATE = i32;
pub const MFNETSOURCE_CACHE_STATE_ID: MFNETSOURCE_STATISTICS_IDS = 12;
pub const MFNETSOURCE_CACHE_UNAVAILABLE: MFNETSOURCE_CACHE_STATE = 0;
pub const MFNETSOURCE_CONTENTBITRATE_ID: MFNETSOURCE_STATISTICS_IDS = 14;
pub const MFNETSOURCE_DOWNLOADPROGRESS_ID: MFNETSOURCE_STATISTICS_IDS = 28;
pub const MFNETSOURCE_FILE: MFNETSOURCE_PROTOCOL_TYPE = 3;
pub const MFNETSOURCE_HTTP: MFNETSOURCE_PROTOCOL_TYPE = 1;
pub const MFNETSOURCE_INCORRECTLYSIGNEDPACKETS_ID: MFNETSOURCE_STATISTICS_IDS = 22;
pub const MFNETSOURCE_LASTBWSWITCHTS_ID: MFNETSOURCE_STATISTICS_IDS = 18;
pub const MFNETSOURCE_LINKBANDWIDTH_ID: MFNETSOURCE_STATISTICS_IDS = 13;
pub const MFNETSOURCE_LOSTPACKETS_ID: MFNETSOURCE_STATISTICS_IDS = 1;
pub const MFNETSOURCE_MAXBITRATE_ID: MFNETSOURCE_STATISTICS_IDS = 24;
pub const MFNETSOURCE_MULTICAST: MFNETSOURCE_PROTOCOL_TYPE = 4;
pub const MFNETSOURCE_OUTPACKETS_ID: MFNETSOURCE_STATISTICS_IDS = 6;
pub const MFNETSOURCE_PROTOCOL_ID: MFNETSOURCE_STATISTICS_IDS = 10;
pub type MFNETSOURCE_PROTOCOL_TYPE = i32;
pub const MFNETSOURCE_RECEPTION_QUALITY_ID: MFNETSOURCE_STATISTICS_IDS = 25;
pub const MFNETSOURCE_RECOVEREDBYECCPACKETS_ID: MFNETSOURCE_STATISTICS_IDS = 4;
pub const MFNETSOURCE_RECOVEREDBYRTXPACKETS_ID: MFNETSOURCE_STATISTICS_IDS = 5;
pub const MFNETSOURCE_RECOVEREDPACKETS_ID: MFNETSOURCE_STATISTICS_IDS = 26;
pub const MFNETSOURCE_RECVPACKETS_ID: MFNETSOURCE_STATISTICS_IDS = 0;
pub const MFNETSOURCE_RECVRATE_ID: MFNETSOURCE_STATISTICS_IDS = 7;
pub const MFNETSOURCE_RESENDSRECEIVED_ID: MFNETSOURCE_STATISTICS_IDS = 3;
pub const MFNETSOURCE_RESENDSREQUESTED_ID: MFNETSOURCE_STATISTICS_IDS = 2;
pub const MFNETSOURCE_RTSP: MFNETSOURCE_PROTOCOL_TYPE = 2;
pub const MFNETSOURCE_SEEKRANGEEND_ID: MFNETSOURCE_STATISTICS_IDS = 20;
pub const MFNETSOURCE_SEEKRANGESTART_ID: MFNETSOURCE_STATISTICS_IDS = 19;
pub const MFNETSOURCE_SIGNEDSESSION_ID: MFNETSOURCE_STATISTICS_IDS = 23;
pub const MFNETSOURCE_SPEEDFACTOR_ID: MFNETSOURCE_STATISTICS_IDS = 15;
pub type MFNETSOURCE_STATISTICS_IDS = i32;
pub const MFNETSOURCE_TCP: MFNETSOURCE_TRANSPORT_TYPE = 1;
pub const MFNETSOURCE_TRANSPORT_ID: MFNETSOURCE_STATISTICS_IDS = 11;
pub type MFNETSOURCE_TRANSPORT_TYPE = i32;
pub const MFNETSOURCE_UDP: MFNETSOURCE_TRANSPORT_TYPE = 0;
pub const MFNETSOURCE_UNDEFINED: MFNETSOURCE_PROTOCOL_TYPE = 0;
pub const MFNETSOURCE_UNPREDEFINEDPROTOCOLNAME_ID: MFNETSOURCE_STATISTICS_IDS = 29;
pub const MFNETSOURCE_VBR_ID: MFNETSOURCE_STATISTICS_IDS = 27;
pub const MFNET_AUTHENTICATION_CLEAR_TEXT: MFNetAuthenticationFlags = 2;
pub const MFNET_AUTHENTICATION_LOGGED_ON_USER: MFNetAuthenticationFlags = 4;
pub const MFNET_AUTHENTICATION_PROXY: MFNetAuthenticationFlags = 1;
pub const MFNET_CREDENTIAL_ALLOW_CLEAR_TEXT: MFNetCredentialOptions = 4;
pub const MFNET_CREDENTIAL_DONT_CACHE: MFNetCredentialOptions = 2;
pub const MFNET_CREDENTIAL_SAVE: MFNetCredentialOptions = 1;
pub type MFNET_PROXYSETTINGS = i32;
pub const MFNET_PROXYSETTING_AUTO: MFNET_PROXYSETTINGS = 2;
pub const MFNET_PROXYSETTING_BROWSER: MFNET_PROXYSETTINGS = 3;
pub const MFNET_PROXYSETTING_MANUAL: MFNET_PROXYSETTINGS = 1;
pub const MFNET_PROXYSETTING_NONE: MFNET_PROXYSETTINGS = 0;
pub type MFNetAuthenticationFlags = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MFNetCredentialManagerGetParam {
    pub hrOp: windows_sys::core::HRESULT,
    pub fAllowLoggedOnUser: windows_sys::core::BOOL,
    pub fClearTextPackage: windows_sys::core::BOOL,
    pub pszUrl: windows_sys::core::PCWSTR,
    pub pszSite: windows_sys::core::PCWSTR,
    pub pszRealm: windows_sys::core::PCWSTR,
    pub pszPackage: windows_sys::core::PCWSTR,
    pub nRetries: i32,
}
impl Default for MFNetCredentialManagerGetParam {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type MFNetCredentialOptions = i32;
pub type MFNetCredentialRequirements = i32;
pub const MFOUTPUTATTRIBUTE_BUS: u32 = 32;
pub const MFOUTPUTATTRIBUTE_BUSIMPLEMENTATION: u32 = 65280;
pub const MFOUTPUTATTRIBUTE_COMPRESSED: u32 = 8;
pub const MFOUTPUTATTRIBUTE_DIGITAL: u32 = 1;
pub const MFOUTPUTATTRIBUTE_NONSTANDARDIMPLEMENTATION: u32 = 2;
pub const MFOUTPUTATTRIBUTE_SOFTWARE: u32 = 16;
pub const MFOUTPUTATTRIBUTE_VIDEO: u32 = 4;
pub type MFPMPSESSION_CREATION_FLAGS = i32;
pub const MFPMPSESSION_IN_PROCESS: MFPMPSESSION_CREATION_FLAGS = 2;
pub const MFPMPSESSION_UNPROTECTED_PROCESS: MFPMPSESSION_CREATION_FLAGS = 1;
pub type MFPOLICYMANAGER_ACTION = i32;
pub type MFRATE_DIRECTION = i32;
pub const MFRATE_FORWARD: MFRATE_DIRECTION = 0;
pub const MFRATE_REVERSE: MFRATE_DIRECTION = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MFRR_COMPONENTS {
    pub dwRRInfoVersion: u32,
    pub dwRRComponents: u32,
    pub pRRComponents: PMFRR_COMPONENT_HASH_INFO,
}
impl Default for MFRR_COMPONENTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MFRR_COMPONENT_HASH_INFO {
    pub ulReason: u32,
    pub rgHeaderHash: [u16; 43],
    pub rgPublicKeyHash: [u16; 43],
    pub wszName: [u16; 260],
}
impl Default for MFRR_COMPONENT_HASH_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MFRR_INFO_VERSION: u32 = 0;
pub const MFSEQUENCER_INVALID_ELEMENT_ID: u32 = 4294967295;
pub const MFSESSION_GETFULLTOPOLOGY_CURRENT: MFSESSION_GETFULLTOPOLOGY_FLAGS = 1;
pub type MFSESSION_GETFULLTOPOLOGY_FLAGS = i32;
pub const MFSESSION_SETTOPOLOGY_CLEAR_CURRENT: MFSESSION_SETTOPOLOGY_FLAGS = 4;
pub type MFSESSION_SETTOPOLOGY_FLAGS = i32;
pub const MFSESSION_SETTOPOLOGY_IMMEDIATE: MFSESSION_SETTOPOLOGY_FLAGS = 1;
pub const MFSESSION_SETTOPOLOGY_NORESOLUTION: MFSESSION_SETTOPOLOGY_FLAGS = 2;
pub const MFSHUTDOWN_COMPLETED: MFSHUTDOWN_STATUS = 1;
pub const MFSHUTDOWN_INITIATED: MFSHUTDOWN_STATUS = 0;
pub type MFSHUTDOWN_STATUS = i32;
pub const MFSTREAMSINK_MARKER_DEFAULT: MFSTREAMSINK_MARKER_TYPE = 0;
pub const MFSTREAMSINK_MARKER_ENDOFSEGMENT: MFSTREAMSINK_MARKER_TYPE = 1;
pub const MFSTREAMSINK_MARKER_EVENT: MFSTREAMSINK_MARKER_TYPE = 3;
pub const MFSTREAMSINK_MARKER_TICK: MFSTREAMSINK_MARKER_TYPE = 2;
pub type MFSTREAMSINK_MARKER_TYPE = i32;
pub type MFSampleAllocatorUsage = i32;
pub const MFSampleAllocatorUsage_DoesNotAllocate: MFSampleAllocatorUsage = 2;
pub const MFSampleAllocatorUsage_UsesCustomAllocator: MFSampleAllocatorUsage = 1;
pub const MFSampleAllocatorUsage_UsesProvidedAllocator: MFSampleAllocatorUsage = 0;
pub const MFSampleExtension_ExtendedCameraIntrinsics: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x560bc4a5_4de0_4113_9cdc_832db9740f3d);
pub type MFSensorDeviceMode = i32;
pub const MFSensorDeviceMode_Controller: MFSensorDeviceMode = 0;
pub const MFSensorDeviceMode_Shared: MFSensorDeviceMode = 1;
pub type MFSensorDeviceType = i32;
pub const MFSensorDeviceType_Device: MFSensorDeviceType = 1;
pub const MFSensorDeviceType_FrameProvider: MFSensorDeviceType = 3;
pub const MFSensorDeviceType_MediaSource: MFSensorDeviceType = 2;
pub const MFSensorDeviceType_SensorTransform: MFSensorDeviceType = 4;
pub const MFSensorDeviceType_Unknown: MFSensorDeviceType = 0;
pub type MFSensorStreamType = i32;
pub const MFSensorStreamType_Input: MFSensorStreamType = 1;
pub const MFSensorStreamType_Output: MFSensorStreamType = 2;
pub const MFSensorStreamType_Unknown: MFSensorStreamType = 0;
pub type MFSequencerElementId = u32;
pub type MFSequencerTopologyFlags = i32;
pub const MFStreamExtension_ExtendedCameraIntrinsics: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xaa74b3df_9a2c_48d6_8393_5bd1c1a81e6e);
pub type MFTIME = i64;
pub type MFTIMER_FLAGS = i32;
pub const MFTIMER_RELATIVE: MFTIMER_FLAGS = 1;
pub const MFTOPOLOGY_DXVA_DEFAULT: MFTOPOLOGY_DXVA_MODE = 0;
pub const MFTOPOLOGY_DXVA_FULL: MFTOPOLOGY_DXVA_MODE = 2;
pub type MFTOPOLOGY_DXVA_MODE = i32;
pub const MFTOPOLOGY_DXVA_NONE: MFTOPOLOGY_DXVA_MODE = 1;
pub type MFTOPOLOGY_HARDWARE_MODE = i32;
pub const MFTOPOLOGY_HWMODE_SOFTWARE_ONLY: MFTOPOLOGY_HARDWARE_MODE = 0;
pub const MFTOPOLOGY_HWMODE_USE_HARDWARE: MFTOPOLOGY_HARDWARE_MODE = 1;
pub const MFTOPOLOGY_HWMODE_USE_ONLY_HARDWARE: MFTOPOLOGY_HARDWARE_MODE = 2;
#[repr(C)]
#[cfg(feature = "mfobjects")]
#[derive(Clone, Copy)]
pub struct MFTOPONODE_ATTRIBUTE_UPDATE {
    pub NodeId: TOPOID,
    pub guidAttributeKey: windows_sys::core::GUID,
    pub attrType: super::MF_ATTRIBUTE_TYPE,
    pub Anonymous: MFTOPONODE_ATTRIBUTE_UPDATE_0,
}
#[cfg(feature = "mfobjects")]
impl Default for MFTOPONODE_ATTRIBUTE_UPDATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "mfobjects")]
#[derive(Clone, Copy)]
pub union MFTOPONODE_ATTRIBUTE_UPDATE_0 {
    pub u32: u32,
    pub u64: u64,
    pub d: f64,
}
#[cfg(feature = "mfobjects")]
impl Default for MFTOPONODE_ATTRIBUTE_UPDATE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "mfobjects")]
#[derive(Clone, Copy)]
pub struct MFT_REGISTRATION_INFO {
    pub clsid: windows_sys::core::GUID,
    pub guidCategory: windows_sys::core::GUID,
    pub uiFlags: u32,
    pub pszName: windows_sys::core::PCWSTR,
    pub cInTypes: u32,
    pub pInTypes: *mut super::MFT_REGISTER_TYPE_INFO,
    pub cOutTypes: u32,
    pub pOutTypes: *mut super::MFT_REGISTER_TYPE_INFO,
}
#[cfg(feature = "mfobjects")]
impl Default for MFT_REGISTRATION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MFVideoNormalizedRect {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}
pub type MFVideoSphericalFormat = i32;
pub const MFVideoSphericalFormat_3DMesh: MFVideoSphericalFormat = 3;
pub const MFVideoSphericalFormat_CubeMap: MFVideoSphericalFormat = 2;
pub const MFVideoSphericalFormat_Equirectangular: MFVideoSphericalFormat = 1;
pub const MFVideoSphericalFormat_Unsupported: MFVideoSphericalFormat = 0;
pub type MFVideoSphericalProjectionMode = i32;
pub const MFVideoSphericalProjectionMode_Flat: MFVideoSphericalProjectionMode = 1;
pub const MFVideoSphericalProjectionMode_Spherical: MFVideoSphericalProjectionMode = 0;
pub const MF_ACTIVATE_CUSTOM_MIXER_ALLOWFAIL: i32 = 1;
pub const MF_ACTIVATE_CUSTOM_PRESENTER_ALLOWFAIL: i32 = 1;
pub const MF_AUDIO_RENDERER_ATTRIBUTE_FLAGS_CROSSPROCESS: u32 = 1;
pub const MF_AUDIO_RENDERER_ATTRIBUTE_FLAGS_DONT_ALLOW_FORMAT_CHANGES: u32 = 4;
pub const MF_AUDIO_RENDERER_ATTRIBUTE_FLAGS_NOPERSIST: u32 = 2;
pub const MF_BOOT_DRIVER_VERIFICATION_FAILED: u32 = 1048576;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MF_BYTE_STREAM_CACHE_RANGE {
    pub qwStartOffset: u64,
    pub qwEndOffset: u64,
}
pub type MF_CAMERA_CONTROL_CONFIGURATION_TYPE = i32;
pub const MF_CAMERA_CONTROL_CONFIGURATION_TYPE_POSTSTART: MF_CAMERA_CONTROL_CONFIGURATION_TYPE = 1;
pub const MF_CAMERA_CONTROL_CONFIGURATION_TYPE_PRESTART: MF_CAMERA_CONTROL_CONFIGURATION_TYPE = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MF_CAMERA_CONTROL_RANGE_INFO {
    pub minValue: i32,
    pub maxValue: i32,
    pub stepValue: i32,
    pub defaultValue: i32,
}
pub const MF_COMPONENT_CERT_REVOKED: u32 = 32768;
pub const MF_COMPONENT_HS_CERT_REVOKED: u32 = 131072;
pub const MF_COMPONENT_INVALID_EKU: u32 = 16384;
pub const MF_COMPONENT_INVALID_ROOT: u32 = 65536;
pub const MF_COMPONENT_LS_CERT_REVOKED: u32 = 262144;
pub const MF_COMPONENT_REVOKED: u32 = 8192;
pub const MF_CONNECT_ALLOW_CONVERTER: MF_CONNECT_METHOD = 1;
pub const MF_CONNECT_ALLOW_DECODER: MF_CONNECT_METHOD = 3;
pub const MF_CONNECT_AS_OPTIONAL: MF_CONNECT_METHOD = 65536;
pub const MF_CONNECT_AS_OPTIONAL_BRANCH: MF_CONNECT_METHOD = 131072;
pub const MF_CONNECT_DIRECT: MF_CONNECT_METHOD = 0;
pub type MF_CONNECT_METHOD = i32;
pub const MF_CONNECT_RESOLVE_INDEPENDENT_OUTPUTTYPES: MF_CONNECT_METHOD = 4;
pub type MF_CROSS_ORIGIN_POLICY = i32;
pub const MF_CROSS_ORIGIN_POLICY_ANONYMOUS: MF_CROSS_ORIGIN_POLICY = 1;
pub const MF_CROSS_ORIGIN_POLICY_NONE: MF_CROSS_ORIGIN_POLICY = 0;
pub const MF_CROSS_ORIGIN_POLICY_USE_CREDENTIALS: MF_CROSS_ORIGIN_POLICY = 2;
pub const MF_DROP_MODE_1: MF_QUALITY_DROP_MODE = 1;
pub const MF_DROP_MODE_2: MF_QUALITY_DROP_MODE = 2;
pub const MF_DROP_MODE_3: MF_QUALITY_DROP_MODE = 3;
pub const MF_DROP_MODE_4: MF_QUALITY_DROP_MODE = 4;
pub const MF_DROP_MODE_5: MF_QUALITY_DROP_MODE = 5;
pub const MF_DROP_MODE_NONE: MF_QUALITY_DROP_MODE = 0;
pub const MF_GRL_ABSENT: u32 = 4096;
pub const MF_GRL_LOAD_FAILED: u32 = 16;
pub const MF_INVALID_GRL_SIGNATURE: u32 = 32;
pub const MF_KERNEL_MODE_COMPONENT_LOAD: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MF_LEAKY_BUCKET_PAIR {
    pub dwBitrate: u32,
    pub msBufferWindow: u32,
}
pub const MF_LICENSE_URL_TAMPERED: MF_URL_TRUST_STATUS = 2;
pub const MF_LICENSE_URL_TRUSTED: MF_URL_TRUST_STATUS = 1;
pub const MF_LICENSE_URL_UNTRUSTED: MF_URL_TRUST_STATUS = 0;
pub type MF_MEDIAKEYSESSION_MESSAGETYPE = i32;
pub const MF_MEDIAKEYSESSION_MESSAGETYPE_INDIVIDUALIZATION_REQUEST: MF_MEDIAKEYSESSION_MESSAGETYPE = 3;
pub const MF_MEDIAKEYSESSION_MESSAGETYPE_LICENSE_RELEASE: MF_MEDIAKEYSESSION_MESSAGETYPE = 2;
pub const MF_MEDIAKEYSESSION_MESSAGETYPE_LICENSE_RENEWAL: MF_MEDIAKEYSESSION_MESSAGETYPE = 1;
pub const MF_MEDIAKEYSESSION_MESSAGETYPE_LICENSE_REQUEST: MF_MEDIAKEYSESSION_MESSAGETYPE = 0;
pub type MF_MEDIAKEYSESSION_TYPE = i32;
pub const MF_MEDIAKEYSESSION_TYPE_PERSISTENT_LICENSE: MF_MEDIAKEYSESSION_TYPE = 1;
pub const MF_MEDIAKEYSESSION_TYPE_PERSISTENT_RELEASE_MESSAGE: MF_MEDIAKEYSESSION_TYPE = 2;
pub const MF_MEDIAKEYSESSION_TYPE_PERSISTENT_USAGE_RECORD: MF_MEDIAKEYSESSION_TYPE = 3;
pub const MF_MEDIAKEYSESSION_TYPE_TEMPORARY: MF_MEDIAKEYSESSION_TYPE = 0;
pub type MF_MEDIAKEY_STATUS = i32;
pub const MF_MEDIAKEY_STATUS_EXPIRED: MF_MEDIAKEY_STATUS = 1;
pub const MF_MEDIAKEY_STATUS_INTERNAL_ERROR: MF_MEDIAKEY_STATUS = 5;
pub const MF_MEDIAKEY_STATUS_OUTPUT_DOWNSCALED: MF_MEDIAKEY_STATUS = 2;
pub const MF_MEDIAKEY_STATUS_OUTPUT_NOT_ALLOWED: MF_MEDIAKEY_STATUS = 3;
pub const MF_MEDIAKEY_STATUS_OUTPUT_RESTRICTED: MF_MEDIAKEY_STATUS = 7;
pub const MF_MEDIAKEY_STATUS_RELEASED: MF_MEDIAKEY_STATUS = 6;
pub const MF_MEDIAKEY_STATUS_STATUS_PENDING: MF_MEDIAKEY_STATUS = 4;
pub const MF_MEDIAKEY_STATUS_USABLE: MF_MEDIAKEY_STATUS = 0;
pub type MF_MEDIASOURCE_STATUS_INFO = i32;
pub const MF_MEDIASOURCE_STATUS_INFO_FULLYSUPPORTED: MF_MEDIASOURCE_STATUS_INFO = 0;
pub const MF_MEDIASOURCE_STATUS_INFO_UNKNOWN: MF_MEDIASOURCE_STATUS_INFO = 1;
pub const MF_MINCRYPT_FAILURE: u32 = 268435456;
pub const MF_NUM_DROP_MODES: MF_QUALITY_DROP_MODE = 6;
pub const MF_NUM_QUALITY_LEVELS: MF_QUALITY_LEVEL = 6;
pub const MF_OBJECT_BYTESTREAM: MF_OBJECT_TYPE = 1;
pub const MF_OBJECT_INVALID: MF_OBJECT_TYPE = 2;
pub const MF_OBJECT_MEDIASOURCE: MF_OBJECT_TYPE = 0;
pub type MF_OBJECT_TYPE = i32;
pub const MF_OPM_ACP_FORCE_ULONG: MF_OPM_ACP_PROTECTION_LEVEL = 2147483647;
pub const MF_OPM_ACP_LEVEL_ONE: MF_OPM_ACP_PROTECTION_LEVEL = 1;
pub const MF_OPM_ACP_LEVEL_THREE: MF_OPM_ACP_PROTECTION_LEVEL = 3;
pub const MF_OPM_ACP_LEVEL_TWO: MF_OPM_ACP_PROTECTION_LEVEL = 2;
pub const MF_OPM_ACP_OFF: MF_OPM_ACP_PROTECTION_LEVEL = 0;
pub type MF_OPM_ACP_PROTECTION_LEVEL = i32;
pub const MF_OPM_CGMSA_COPY_FREELY: MF_OPM_CGMSA_PROTECTION_LEVEL = 1;
pub const MF_OPM_CGMSA_COPY_NEVER: MF_OPM_CGMSA_PROTECTION_LEVEL = 4;
pub const MF_OPM_CGMSA_COPY_NO_MORE: MF_OPM_CGMSA_PROTECTION_LEVEL = 2;
pub const MF_OPM_CGMSA_COPY_ONE_GENERATION: MF_OPM_CGMSA_PROTECTION_LEVEL = 3;
pub const MF_OPM_CGMSA_OFF: MF_OPM_CGMSA_PROTECTION_LEVEL = 0;
pub type MF_OPM_CGMSA_PROTECTION_LEVEL = i32;
pub const MF_OPM_CGMSA_REDISTRIBUTION_CONTROL_REQUIRED: MF_OPM_CGMSA_PROTECTION_LEVEL = 8;
pub const MF_OPTIONAL_NODE_REJECTED_MEDIA_TYPE: MF_TOPOLOGY_RESOLUTION_STATUS_FLAGS = 1;
pub const MF_OPTIONAL_NODE_REJECTED_PROTECTED_PROCESS: MF_TOPOLOGY_RESOLUTION_STATUS_FLAGS = 2;
pub const MF_PD_ADAPTIVE_STREAMING: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xea0d5d97_29f9_488b_ae6b_7d6b4136112b);
pub type MF_QUALITY_ADVISE_FLAGS = i32;
pub const MF_QUALITY_CANNOT_KEEP_UP: MF_QUALITY_ADVISE_FLAGS = 1;
pub type MF_QUALITY_DROP_MODE = i32;
pub type MF_QUALITY_LEVEL = i32;
pub const MF_QUALITY_NORMAL: MF_QUALITY_LEVEL = 0;
pub const MF_QUALITY_NORMAL_MINUS_1: MF_QUALITY_LEVEL = 1;
pub const MF_QUALITY_NORMAL_MINUS_2: MF_QUALITY_LEVEL = 2;
pub const MF_QUALITY_NORMAL_MINUS_3: MF_QUALITY_LEVEL = 3;
pub const MF_QUALITY_NORMAL_MINUS_4: MF_QUALITY_LEVEL = 4;
pub const MF_QUALITY_NORMAL_MINUS_5: MF_QUALITY_LEVEL = 5;
pub const MF_RESOLUTION_BYTESTREAM: i32 = 2;
pub const MF_RESOLUTION_CONTENT_DOES_NOT_HAVE_TO_MATCH_EXTENSION_OR_MIME_TYPE: i32 = 16;
pub const MF_RESOLUTION_DISABLE_LOCAL_PLUGINS: i32 = 64;
pub const MF_RESOLUTION_ENABLE_STORE_PLUGINS: i32 = 1024;
pub const MF_RESOLUTION_KEEP_BYTE_STREAM_ALIVE_ON_FAIL: i32 = 32;
pub const MF_RESOLUTION_MEDIASOURCE: i32 = 1;
pub const MF_RESOLUTION_PLUGIN_CONTROL_POLICY_APPROVED_ONLY: i32 = 128;
pub const MF_RESOLUTION_PLUGIN_CONTROL_POLICY_WEB_ONLY: i32 = 256;
pub const MF_RESOLUTION_PLUGIN_CONTROL_POLICY_WEB_ONLY_EDGEMODE: i32 = 512;
pub const MF_RESOLUTION_READ: i32 = 65536;
pub const MF_RESOLUTION_WRITE: i32 = 131072;
pub const MF_SD_SUPPORTS_PROTECTED_CODEC_SWITCH: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8fb6b117_862e_4b31_8dab_5e0a434caef0);
pub const MF_TEST_SIGNED_COMPONENT_LOADING: u32 = 16777216;
pub const MF_TOPOLOGY_MAX: MF_TOPOLOGY_TYPE = -1;
pub const MF_TOPOLOGY_OUTPUT_NODE: MF_TOPOLOGY_TYPE = 0;
pub type MF_TOPOLOGY_RESOLUTION_STATUS_FLAGS = i32;
pub const MF_TOPOLOGY_RESOLUTION_SUCCEEDED: MF_TOPOLOGY_RESOLUTION_STATUS_FLAGS = 0;
pub const MF_TOPOLOGY_SOURCESTREAM_NODE: MF_TOPOLOGY_TYPE = 1;
pub const MF_TOPOLOGY_TEE_NODE: MF_TOPOLOGY_TYPE = 3;
pub const MF_TOPOLOGY_TRANSFORM_NODE: MF_TOPOLOGY_TYPE = 2;
pub type MF_TOPOLOGY_TYPE = i32;
pub const MF_TOPONODE_DRAIN_ALWAYS: MF_TOPONODE_DRAIN_MODE = 1;
pub const MF_TOPONODE_DRAIN_DEFAULT: MF_TOPONODE_DRAIN_MODE = 0;
pub type MF_TOPONODE_DRAIN_MODE = i32;
pub const MF_TOPONODE_DRAIN_NEVER: MF_TOPONODE_DRAIN_MODE = 2;
pub const MF_TOPONODE_FLUSH_ALWAYS: MF_TOPONODE_FLUSH_MODE = 0;
pub type MF_TOPONODE_FLUSH_MODE = i32;
pub const MF_TOPONODE_FLUSH_NEVER: MF_TOPONODE_FLUSH_MODE = 2;
pub const MF_TOPONODE_FLUSH_SEEK: MF_TOPONODE_FLUSH_MODE = 1;
pub const MF_TRANSCODE_ADJUST_PROFILE_DEFAULT: MF_TRANSCODE_ADJUST_PROFILE_FLAGS = 0;
pub type MF_TRANSCODE_ADJUST_PROFILE_FLAGS = i32;
pub const MF_TRANSCODE_ADJUST_PROFILE_USE_SOURCE_ATTRIBUTES: MF_TRANSCODE_ADJUST_PROFILE_FLAGS = 1;
#[repr(C)]
#[cfg(feature = "mfobjects")]
#[derive(Clone, Copy)]
pub struct MF_TRANSCODE_SINK_INFO {
    pub dwVideoStreamID: u32,
    pub pVideoMediaType: *mut core::ffi::c_void,
    pub dwAudioStreamID: u32,
    pub pAudioMediaType: *mut core::ffi::c_void,
}
#[cfg(feature = "mfobjects")]
impl Default for MF_TRANSCODE_SINK_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type MF_TRANSCODE_TOPOLOGYMODE_FLAGS = i32;
pub const MF_TRANSCODE_TOPOLOGYMODE_HARDWARE_ALLOWED: MF_TRANSCODE_TOPOLOGYMODE_FLAGS = 1;
pub const MF_TRANSCODE_TOPOLOGYMODE_SOFTWARE_ONLY: MF_TRANSCODE_TOPOLOGYMODE_FLAGS = 0;
pub const MF_TRANSFER_VIDEO_FRAME_DEFAULT: MF_TRANSFER_VIDEO_FRAME_FLAGS = 0;
pub type MF_TRANSFER_VIDEO_FRAME_FLAGS = i32;
pub const MF_TRANSFER_VIDEO_FRAME_IGNORE_PAR: MF_TRANSFER_VIDEO_FRAME_FLAGS = 2;
pub const MF_TRANSFER_VIDEO_FRAME_STRETCH: MF_TRANSFER_VIDEO_FRAME_FLAGS = 1;
pub const MF_UNKNOWN_DURATION: u32 = 0;
pub type MF_URL_TRUST_STATUS = i32;
pub const MF_USER_MODE_COMPONENT_LOAD: u32 = 1;
pub const MF_VIDEO_PROCESSOR_ALGORITHM_DEFAULT: MF_VIDEO_PROCESSOR_ALGORITHM_TYPE = 0;
pub const MF_VIDEO_PROCESSOR_ALGORITHM_MRF_CRF_444: MF_VIDEO_PROCESSOR_ALGORITHM_TYPE = 1;
pub type MF_VIDEO_PROCESSOR_ALGORITHM_TYPE = i32;
pub type MF_VIDEO_PROCESSOR_MIRROR = i32;
pub type MF_VIDEO_PROCESSOR_ROTATION = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MF_VIDEO_SPHERICAL_VIEWDIRECTION {
    pub iHeading: i32,
    pub iPitch: i32,
    pub iRoll: i32,
}
pub const MF_WRAPPED_BUFFER_SERVICE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xab544072_c269_4ebc_a552_1c3b32bed5ca);
pub const MFaudioConstriction14_14: MFAudioConstriction = 3;
pub const MFaudioConstriction44_16: MFAudioConstriction = 2;
pub const MFaudioConstriction48_16: MFAudioConstriction = 1;
pub const MFaudioConstrictionMute: MFAudioConstriction = 4;
pub const MFaudioConstrictionOff: MFAudioConstriction = 0;
pub const MIRROR_HORIZONTAL: MF_VIDEO_PROCESSOR_MIRROR = 1;
pub const MIRROR_NONE: MF_VIDEO_PROCESSOR_MIRROR = 0;
pub const MIRROR_VERTICAL: MF_VIDEO_PROCESSOR_MIRROR = 2;
pub const PEACTION_COPY: MFPOLICYMANAGER_ACTION = 2;
pub const PEACTION_EXPORT: MFPOLICYMANAGER_ACTION = 3;
pub const PEACTION_EXTRACT: MFPOLICYMANAGER_ACTION = 4;
pub const PEACTION_LAST: MFPOLICYMANAGER_ACTION = 7;
pub const PEACTION_NO: MFPOLICYMANAGER_ACTION = 0;
pub const PEACTION_PLAY: MFPOLICYMANAGER_ACTION = 1;
pub const PEACTION_RESERVED1: MFPOLICYMANAGER_ACTION = 5;
pub const PEACTION_RESERVED2: MFPOLICYMANAGER_ACTION = 6;
pub const PEACTION_RESERVED3: MFPOLICYMANAGER_ACTION = 7;
pub type PMFRR_COMPONENTS = *mut MFRR_COMPONENTS;
pub type PMFRR_COMPONENT_HASH_INFO = *mut MFRR_COMPONENT_HASH_INFO;
pub const PRESENTATION_CURRENT_POSITION: u64 = 9223372036854775807;
pub const REQUIRE_PROMPT: MFNetCredentialRequirements = 1;
pub const REQUIRE_SAVE_SELECTED: MFNetCredentialRequirements = 2;
pub const ROTATION_NONE: MF_VIDEO_PROCESSOR_ROTATION = 0;
pub const ROTATION_NORMAL: MF_VIDEO_PROCESSOR_ROTATION = 1;
pub type SAMPLE_PROTECTION_VERSION = i32;
pub const SAMPLE_PROTECTION_VERSION_AES128CTR: SAMPLE_PROTECTION_VERSION = 4;
pub const SAMPLE_PROTECTION_VERSION_BASIC_LOKI: SAMPLE_PROTECTION_VERSION = 1;
pub const SAMPLE_PROTECTION_VERSION_NO: SAMPLE_PROTECTION_VERSION = 0;
pub const SAMPLE_PROTECTION_VERSION_RC4: SAMPLE_PROTECTION_VERSION = 3;
pub const SAMPLE_PROTECTION_VERSION_SCATTER: SAMPLE_PROTECTION_VERSION = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SENSORPROFILEID {
    pub Type: windows_sys::core::GUID,
    pub Index: u32,
    pub Unused: u32,
}
pub const SHA_HASH_LEN: u32 = 20;
pub const STR_HASH_LEN: u32 = 43;
pub const SequencerTopologyFlags_Last: MFSequencerTopologyFlags = 1;
pub type TOPOID = u64;
