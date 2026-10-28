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
#[cfg(all(feature = "guiddef", feature = "minwindef"))]
windows_link::link!("mf.dll" "system" fn MFCreateNetSchemePlugin(riid : *const super::IID, ppvhandler : *mut super::LPVOID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "mfobjects", feature = "shared"))]
windows_link::link!("mf.dll" "system" fn MFCreatePMPMediaSession(dwcreationflags : super::DWORD, pconfiguration : *mut core::ffi::c_void, ppmediasession : *mut *mut core::ffi::c_void, ppenableractivate : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("mf.dll" "system" fn MFCreatePMPServer(dwcreationflags : super::DWORD, pppmpserver : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("mf.dll" "system" fn MFCreatePresentationClock(pppresentationclock : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "mfobjects", feature = "shared"))]
windows_link::link!("mfplat.dll" "system" fn MFCreatePresentationDescriptor(cstreamdescriptors : super::DWORD, apstreamdescriptors : *const *mut core::ffi::c_void, pppresentationdescriptor : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "mfobjects"))]
windows_link::link!("mfplat.dll" "system" fn MFCreatePropertiesFromMediaType(pmediatype : *mut core::ffi::c_void, riid : *const super::IID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
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
#[cfg(feature = "shared")]
windows_link::link!("mfsensorgroup.dll" "system" fn MFCreateSensorProfile(profiletype : *const windows_sys::core::GUID, profileindex : super::UINT32, constraints : windows_sys::core::PCWSTR, ppprofile : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("mfsensorgroup.dll" "system" fn MFCreateSensorProfileCollection(ppsensorprofile : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "mfobjects", feature = "shared"))]
windows_link::link!("mfsensorgroup.dll" "system" fn MFCreateSensorStream(streamid : super::DWORD, pattributes : *mut core::ffi::c_void, pmediatypecollection : *mut core::ffi::c_void, ppstream : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("mf.dll" "system" fn MFCreateSequencerSegmentOffset(dwid : MFSequencerElementId, hnsoffset : MFTIME, pvarsegmentoffset : *mut super::PROPVARIANT) -> windows_sys::core::HRESULT);
windows_link::link!("mf.dll" "system" fn MFCreateSequencerSource(preserved : *mut core::ffi::c_void, ppsequencersource : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("mf.dll" "system" fn MFCreateSimpleTypeHandler(pphandler : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("mfplat.dll" "system" fn MFCreateSourceResolver(ppisourceresolver : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("mf.dll" "system" fn MFCreateStandardQualityManager(ppqualitymanager : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "mfobjects", feature = "shared"))]
windows_link::link!("mfplat.dll" "system" fn MFCreateStreamDescriptor(dwstreamidentifier : super::DWORD, cmediatypes : super::DWORD, apmediatypes : *const *mut core::ffi::c_void, ppdescriptor : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "mfobjects", feature = "objidlbase"))]
windows_link::link!("mfplat.dll" "system" fn MFCreateStreamOnMFByteStream(pbytestream : *mut core::ffi::c_void, ppstream : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "mfobjects"))]
windows_link::link!("mfplat.dll" "system" fn MFCreateStreamOnMFByteStreamEx(pbytestream : *mut core::ffi::c_void, riid : *const super::IID, ppv : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
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
#[cfg(all(feature = "mfobjects", feature = "shared"))]
windows_link::link!("mfplat.dll" "system" fn MFDeserializePresentationDescriptor(cbdata : super::DWORD, pbdata : *const super::BYTE, pppd : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "mfobjects", feature = "shared"))]
windows_link::link!("mf.dll" "system" fn MFEnumDeviceSources(pattributes : *mut core::ffi::c_void, pppsourceactivate : *mut *mut *mut core::ffi::c_void, pcsourceactivate : *mut super::UINT32) -> windows_sys::core::HRESULT);
#[cfg(feature = "shared")]
windows_link::link!("mf.dll" "system" fn MFGetLocalId(verifier : *const super::BYTE, size : super::UINT32, id : *mut windows_sys::core::PWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef"))]
windows_link::link!("mf.dll" "system" fn MFGetService(punkobject : *mut core::ffi::c_void, guidservice : *const windows_sys::core::GUID, riid : *const super::IID, ppvobject : *mut super::LPVOID) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("mfplat.dll" "system" fn MFGetSupportedMimeTypes(ppropvarmimetypearray : *mut super::PROPVARIANT) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
windows_link::link!("mfplat.dll" "system" fn MFGetSupportedSchemes(ppropvarschemearray : *mut super::PROPVARIANT) -> windows_sys::core::HRESULT);
windows_link::link!("mf.dll" "system" fn MFGetSystemId(ppid : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("mfplat.dll" "system" fn MFGetSystemTime() -> MFTIME);
#[cfg(all(feature = "mfobjects", feature = "shared"))]
windows_link::link!("mf.dll" "system" fn MFGetTopoNodeCurrentType(pnode : *mut core::ffi::c_void, dwstreamindex : super::DWORD, foutput : windows_sys::core::BOOL, pptype : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("mfplat.dll" "system" fn MFIsContentProtectionDeviceSupported(protectionsystemid : *const windows_sys::core::GUID, issupported : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
windows_link::link!("mf.dll" "system" fn MFLoadSignedLibrary(pszname : windows_sys::core::PCWSTR, pplib : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "mfobjects")]
windows_link::link!("mf.dll" "system" fn MFRequireProtectedEnvironment(ppresentationdescriptor : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "mfobjects", feature = "shared"))]
windows_link::link!("mfplat.dll" "system" fn MFSerializePresentationDescriptor(ppd : *mut core::ffi::c_void, pcbdata : *mut super::DWORD, ppbdata : *mut *mut super::BYTE) -> windows_sys::core::HRESULT);
windows_link::link!("mf.dll" "system" fn MFShutdownObject(punk : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "mfobjects", feature = "shared"))]
windows_link::link!("mf.dll" "system" fn MFTranscodeGetAudioOutputAvailableTypes(guidsubtype : *const windows_sys::core::GUID, dwmftflags : super::DWORD, pcodecconfig : *mut core::ffi::c_void, ppavailabletypes : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct ASF_FLAT_PICTURE {
    pub bPictureType: super::BYTE,
    pub dwDataLen: super::DWORD,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct ASF_FLAT_SYNCHRONISED_LYRICS {
    pub bTimeStampFormat: super::BYTE,
    pub bContentType: super::BYTE,
    pub dwLyricsLen: super::DWORD,
}
pub const CLSID_AudioResamplerMediaObject: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf447b69e_1884_4a7e_8055_346f74d6edb3);
pub const CLSID_CameraConfigurationManager: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6c92b540_5854_4a17_92b6_ac89c96e9683);
pub const CLSID_CreateMediaExtensionObject: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xef65a54d_0788_45b8_8b14_bc0f6a6b5137);
pub const CLSID_FaceDetectionMFT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc1e565e2_f2de_4537_9612_2f30a160eb5c);
pub const CLSID_FrameServerClassFactory: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9a93092c_9cdc_49b8_8349_cbcf3145fe0a);
pub const CLSID_FrameServerNetworkCameraSource: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7a213aa7_866f_414a_8c1a_275c7283a395);
pub const CLSID_HttpSchemePlugin: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x44cb442b_9da9_49df_b3fd_023777b16e50);
pub const CLSID_MP3DecMediaObject: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xbbeea841_0a63_4f52_a7ab_a9b3a84ed38a);
pub const CLSID_MPEG2ByteStreamPlugin: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x40871c59_ab40_471f_8dc3_1f259d862479);
pub const CLSID_MSAACDecMFT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x32d186a7_218f_4c75_8876_dd77273a8999);
pub const CLSID_MSDDPlusDecMFT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x177c0afe_900b_48d4_9e4c_57add250b3d4);
pub const CLSID_MSH264DecoderMFT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x62ce7e72_4c71_4d20_b15d_452831a87d9d);
pub const CLSID_MSH264EncoderMFT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6ca50344_051a_4ded_9779_a43305165e35);
pub const CLSID_MSH265DecoderMFT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x420a51a3_d605_430c_b4fc_45274fa6c562);
pub const CLSID_MSMPEGAudDecMFT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x70707b39_b2ca_4015_abea_f8447d22d88b);
pub const CLSID_MSMPEGDecoderMFT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2d709e52_123f_49b5_9cbc_9af5cde28fb9);
pub const CLSID_MSOpusDecoder: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x63e17c10_2d43_4c42_8fe3_8d8b63e46a6a);
pub const CLSID_MSVPxDecoder: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe3aaf548_c9a4_4c6e_234d_5ada374b0000);
pub const CLSID_NetSchemePlugin: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe9f4ebab_d97b_463e_a2b1_c54ee3f9414d);
pub const CLSID_UrlmonSchemePlugin: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9ec4b4f9_3029_45ad_947b_344de2a249e2);
pub const CLSID_VideoProcessorMFT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x88753b26_5b24_49bd_b2e7_0c445c78c982);
pub const CLSID_WMADecMediaObject: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2eeb4adf_4578_4d10_bca7_bb955f56320a);
pub const CLSID_WMVDecoderMFT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x82d353df_90bd_4382_8bc2_3f6192b76e34);
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DetectedFaceBound {
    pub sizeInBytes: super::DWORD,
    pub normalizedXPosition: f32,
    pub normalizedYPosition: f32,
    pub normalizedWidth: f32,
    pub normalizedHeight: f32,
    pub confidenceValue: super::LONG,
    pub flags: super::ULONGLONG,
}
pub const KSPROPERTYSETID_ANYCAMERACONTROL: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x94dd0c30_28c7_4efb_9d6b_812300fb0c7f);
pub const MEDIASINK_CANNOT_MATCH_CLOCK: i32 = 2;
pub const MEDIASINK_CAN_PREROLL: i32 = 16;
pub const MEDIASINK_CLOCK_REQUIRED: i32 = 8;
pub const MEDIASINK_FIXED_STREAMS: i32 = 1;
pub const MEDIASINK_RATELESS: i32 = 4;
pub const MEDIASINK_REQUIRE_REFERENCE_MEDIATYPE: i32 = 32;
pub type MFAudioConstriction = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MFBYTESTREAM_BUFFERING_PARAMS {
    pub cbTotalFileSize: super::QWORD,
    pub cbPlayableDataSize: super::QWORD,
    pub prgBuckets: *mut MF_LEAKY_BUCKET_PAIR,
    pub cBuckets: super::DWORD,
    pub qwNetBufferingTime: super::QWORD,
    pub qwExtraBufferingTimeDuringSeek: super::QWORD,
    pub qwPlayDuration: super::QWORD,
    pub dRate: f32,
}
pub type MFCLOCK_CHARACTERISTICS_FLAGS = i32;
pub const MFCLOCK_CHARACTERISTICS_FLAG_ALWAYS_RUNNING: MFCLOCK_CHARACTERISTICS_FLAGS = 4;
pub const MFCLOCK_CHARACTERISTICS_FLAG_FREQUENCY_10MHZ: MFCLOCK_CHARACTERISTICS_FLAGS = 2;
pub const MFCLOCK_CHARACTERISTICS_FLAG_IS_SYSTEM_CLOCK: MFCLOCK_CHARACTERISTICS_FLAGS = 8;
pub const MFCLOCK_FREQUENCY_HNS: i32 = 10000000;
pub const MFCLOCK_JITTER_DPC: i32 = 4000;
pub const MFCLOCK_JITTER_ISR: i32 = 1000;
pub const MFCLOCK_JITTER_PASSIVE: i32 = 10000;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MFCLOCK_PROPERTIES {
    pub qwCorrelationRate: u64,
    pub guidClockId: windows_sys::core::GUID,
    pub dwClockFlags: super::DWORD,
    pub qwClockFrequency: u64,
    pub dwClockTolerance: super::DWORD,
    pub dwClockJitter: super::DWORD,
}
pub type MFCLOCK_RELATIONAL_FLAGS = i32;
pub const MFCLOCK_RELATIONAL_FLAG_JITTER_NEVER_AHEAD: MFCLOCK_RELATIONAL_FLAGS = 1;
pub type MFCLOCK_STATE = i32;
pub const MFCLOCK_STATE_INVALID: MFCLOCK_STATE = 0;
pub const MFCLOCK_STATE_PAUSED: MFCLOCK_STATE = 3;
pub const MFCLOCK_STATE_RUNNING: MFCLOCK_STATE = 1;
pub const MFCLOCK_STATE_STOPPED: MFCLOCK_STATE = 2;
pub const MFCLOCK_TOLERANCE_UNKNOWN: i32 = 50000;
pub const MFCONNECTOR_AGP: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xac3aef60_ce43_11d9_92db_000bdb28ff98);
pub const MFCONNECTOR_COMPONENT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x57cd596b_ce47_11d9_92db_000bdb28ff98);
pub const MFCONNECTOR_COMPOSITE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x57cd596a_ce47_11d9_92db_000bdb28ff98);
pub const MFCONNECTOR_DISPLAYPORT_EMBEDDED: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x57cd5973_ce47_11d9_92db_000bdb28ff98);
pub const MFCONNECTOR_DISPLAYPORT_EXTERNAL: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x57cd5972_ce47_11d9_92db_000bdb28ff98);
pub const MFCONNECTOR_DVI: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x57cd596c_ce47_11d9_92db_000bdb28ff98);
pub const MFCONNECTOR_D_JPN: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x57cd5970_ce47_11d9_92db_000bdb28ff98);
pub const MFCONNECTOR_HDMI: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x57cd596d_ce47_11d9_92db_000bdb28ff98);
pub const MFCONNECTOR_LVDS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x57cd596e_ce47_11d9_92db_000bdb28ff98);
pub const MFCONNECTOR_MIRACAST: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x57cd5977_ce47_11d9_92db_000bdb28ff98);
pub const MFCONNECTOR_PCI: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xac3aef5d_ce43_11d9_92db_000bdb28ff98);
pub const MFCONNECTOR_PCIX: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xac3aef5e_ce43_11d9_92db_000bdb28ff98);
pub const MFCONNECTOR_PCI_Express: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xac3aef5f_ce43_11d9_92db_000bdb28ff98);
pub const MFCONNECTOR_SDI: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x57cd5971_ce47_11d9_92db_000bdb28ff98);
pub const MFCONNECTOR_SPDIF: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0b94a712_ad3e_4cee_83ce_ce32e3db6522);
pub const MFCONNECTOR_SVIDEO: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x57cd5969_ce47_11d9_92db_000bdb28ff98);
pub const MFCONNECTOR_TRANSPORT_AGNOSTIC_DIGITAL_MODE_A: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x57cd5978_ce47_11d9_92db_000bdb28ff98);
pub const MFCONNECTOR_TRANSPORT_AGNOSTIC_DIGITAL_MODE_B: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x57cd5979_ce47_11d9_92db_000bdb28ff98);
pub const MFCONNECTOR_UDI_EMBEDDED: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x57cd5975_ce47_11d9_92db_000bdb28ff98);
pub const MFCONNECTOR_UDI_EXTERNAL: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x57cd5974_ce47_11d9_92db_000bdb28ff98);
pub const MFCONNECTOR_UNKNOWN: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xac3aef5c_ce43_11d9_92db_000bdb28ff98);
pub const MFCONNECTOR_VGA: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x57cd5968_ce47_11d9_92db_000bdb28ff98);
pub const MFCONTENTPROTECTIONDEVICE_FUNCTIONID_START: i32 = 67108864;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct MFCONTENTPROTECTIONDEVICE_INPUT_DATA {
    pub HWProtectionFunctionID: super::DWORD,
    pub PrivateDataByteCount: super::DWORD,
    pub HWProtectionDataByteCount: super::DWORD,
    pub Reserved: super::DWORD,
    pub InputData: [super::BYTE; 4],
}
#[cfg(feature = "shared")]
impl Default for MFCONTENTPROTECTIONDEVICE_INPUT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct MFCONTENTPROTECTIONDEVICE_OUTPUT_DATA {
    pub PrivateDataByteCount: super::DWORD,
    pub MaxHWProtectionDataByteCount: super::DWORD,
    pub HWProtectionDataByteCount: super::DWORD,
    pub Status: windows_sys::core::HRESULT,
    pub TransportTimeInHundredsOfNanoseconds: super::LONGLONG,
    pub ExecutionTimeInHundredsOfNanoseconds: super::LONGLONG,
    pub OutputData: [super::BYTE; 4],
}
#[cfg(feature = "shared")]
impl Default for MFCONTENTPROTECTIONDEVICE_OUTPUT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MFCONTENTPROTECTIONDEVICE_REALTIMECLIENT_DATA {
    pub TaskIndex: super::DWORD,
    pub ClassName: [super::WCHAR; 260],
    pub BasePriority: super::LONG,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for MFCONTENTPROTECTIONDEVICE_REALTIMECLIENT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MFCONTENTPROTECTIONDEVICE_REALTIMECLIENT_DATA_FUNCTIONID: i32 = 67108864;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct MFCameraIntrinsic_CameraModel {
    pub FocalLength_x: super::FLOAT,
    pub FocalLength_y: super::FLOAT,
    pub PrincipalPoint_x: super::FLOAT,
    pub PrincipalPoint_y: super::FLOAT,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct MFCameraIntrinsic_DistortionModel6KT {
    pub Radial_k1: super::FLOAT,
    pub Radial_k2: super::FLOAT,
    pub Radial_k3: super::FLOAT,
    pub Radial_k4: super::FLOAT,
    pub Radial_k5: super::FLOAT,
    pub Radial_k6: super::FLOAT,
    pub Tangential_p1: super::FLOAT,
    pub Tangential_p2: super::FLOAT,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Default)]
pub struct MFCameraIntrinsic_DistortionModelArcTan {
    pub Radial_k0: super::FLOAT,
    pub DistortionCenter_x: super::FLOAT,
    pub DistortionCenter_y: super::FLOAT,
    pub Tangential_x: super::FLOAT,
    pub Tangential_y: super::FLOAT,
}
pub type MFCameraIntrinsic_DistortionModelType = i32;
pub const MFCameraIntrinsic_DistortionModelType_6KT: MFCameraIntrinsic_DistortionModelType = 0;
pub const MFCameraIntrinsic_DistortionModelType_ArcTan: MFCameraIntrinsic_DistortionModelType = 1;
pub type MFCameraOcclusionState = i32;
pub const MFCameraOcclusionState_OccludedByCameraHardware: MFCameraOcclusionState = 2;
pub const MFCameraOcclusionState_OccludedByLid: MFCameraOcclusionState = 1;
pub const MFCameraOcclusionState_Open: MFCameraOcclusionState = 0;
pub const MFENABLETYPE_MF_RebootRequired: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6d4d3d4b_0ece_4652_8b3a_f2d24260d887);
pub const MFENABLETYPE_MF_UpdateRevocationInformation: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe558b0b5_b3c4_44a0_924c_50d178932385);
pub const MFENABLETYPE_MF_UpdateUntrustedComponent: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9879f3d6_cee2_48e6_b573_9767ab172f16);
pub const MFENABLETYPE_WMDRMV1_LicenseAcquisition: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4ff6eeaf_0b43_4797_9b85_abf31815e7b0);
pub const MFENABLETYPE_WMDRMV7_Individualization: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xacd2c84a_b303_4f65_bc2c_2c848d01a989);
pub const MFENABLETYPE_WMDRMV7_LicenseAcquisition: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x003306df_4a06_4884_a097_ef6d22ec84a3);
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct MFExtendedCameraIntrinsic_IntrinsicModel {
    pub Width: super::UINT32,
    pub Height: super::UINT32,
    pub SplitFrameId: super::UINT32,
    pub CameraModel: MFCameraIntrinsic_CameraModel,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MFINPUTTRUSTAUTHORITY_ACCESS_ACTION {
    pub Action: MFPOLICYMANAGER_ACTION,
    pub pbTicket: *mut super::BYTE,
    pub cbTicket: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct MFINPUTTRUSTAUTHORITY_ACCESS_PARAMS {
    pub dwSize: super::DWORD,
    pub dwVer: super::DWORD,
    pub cbSignatureOffset: super::DWORD,
    pub cbSignatureSize: super::DWORD,
    pub cbExtensionOffset: super::DWORD,
    pub cbExtensionSize: super::DWORD,
    pub cActions: super::DWORD,
    pub rgOutputActions: [MFINPUTTRUSTAUTHORITY_ACCESS_ACTION; 1],
}
#[cfg(feature = "shared")]
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MFMediaKeyStatus {
    pub pbKeyId: *mut super::BYTE,
    pub cbKeyId: super::UINT,
    pub eMediaKeyStatus: MF_MEDIAKEY_STATUS,
}
pub const MFNETSOURCE_ACCELERATEDSTREAMINGDURATION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3cb1f277_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_AUTORECONNECTLIMIT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3cb1f27a_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_AUTORECONNECTPROGRESS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3cb1f282_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_AVGBANDWIDTHBPS_ID: MFNETSOURCE_STATISTICS_IDS = 8;
pub const MFNETSOURCE_BROWSERUSERAGENT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3cb1f28b_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_BROWSERWEBPAGE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3cb1f28c_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_BUFFERINGCOUNT_ID: MFNETSOURCE_STATISTICS_IDS = 21;
pub const MFNETSOURCE_BUFFERINGTIME: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3cb1f276_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_BUFFERPROGRESS_ID: MFNETSOURCE_STATISTICS_IDS = 17;
pub const MFNETSOURCE_BUFFERSIZE_ID: MFNETSOURCE_STATISTICS_IDS = 16;
pub const MFNETSOURCE_BYTESRECEIVED_ID: MFNETSOURCE_STATISTICS_IDS = 9;
pub const MFNETSOURCE_CACHEENABLED: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3cb1f279_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_CACHE_ACTIVE_COMPLETE: MFNETSOURCE_CACHE_STATE = 2;
pub const MFNETSOURCE_CACHE_ACTIVE_WRITING: MFNETSOURCE_CACHE_STATE = 1;
pub type MFNETSOURCE_CACHE_STATE = i32;
pub const MFNETSOURCE_CACHE_STATE_ID: MFNETSOURCE_STATISTICS_IDS = 12;
pub const MFNETSOURCE_CACHE_UNAVAILABLE: MFNETSOURCE_CACHE_STATE = 0;
pub const MFNETSOURCE_CLIENTGUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x60a2c4a6_f197_4c14_a5bf_88830d2458af);
pub const MFNETSOURCE_CONNECTIONBANDWIDTH: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3cb1f278_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_CONTENTBITRATE_ID: MFNETSOURCE_STATISTICS_IDS = 14;
pub const MFNETSOURCE_CREDENTIAL_MANAGER: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3cb1f280_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_CROSS_ORIGIN_SUPPORT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9842207c_b02c_4271_a2fc_72e49308e5c2);
pub const MFNETSOURCE_DOWNLOADPROGRESS_ID: MFNETSOURCE_STATISTICS_IDS = 28;
pub const MFNETSOURCE_DRMNET_LICENSE_REPRESENTATION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x47eae1bd_bdfe_42e2_82f3_54a48c17962d);
pub const MFNETSOURCE_ENABLE_DOWNLOAD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3cb1f29d_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_ENABLE_HTTP: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3cb1f299_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_ENABLE_MSB: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3cb1f296_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_ENABLE_PRIVATEMODE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x824779d8_f18b_4405_8cf1_464fb5aa8f71);
pub const MFNETSOURCE_ENABLE_RTSP: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3cb1f298_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_ENABLE_STREAMING: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3cb1f29c_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_ENABLE_TCP: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3cb1f295_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_ENABLE_UDP: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3cb1f294_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_FILE: MFNETSOURCE_PROTOCOL_TYPE = 3;
pub const MFNETSOURCE_FRIENDLYNAME: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5b2a7757_bc6b_447e_aa06_0dda1c646e2f);
pub const MFNETSOURCE_HOSTEXE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3cb1f28f_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_HOSTVERSION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3cb1f291_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_HTTP: MFNETSOURCE_PROTOCOL_TYPE = 1;
pub const MFNETSOURCE_HTTP_DOWNLOAD_SESSION_PROVIDER: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7d55081e_307d_4d6d_a663_a93be97c4b5c);
pub const MFNETSOURCE_INCORRECTLYSIGNEDPACKETS_ID: MFNETSOURCE_STATISTICS_IDS = 22;
pub const MFNETSOURCE_LASTBWSWITCHTS_ID: MFNETSOURCE_STATISTICS_IDS = 18;
pub const MFNETSOURCE_LINKBANDWIDTH_ID: MFNETSOURCE_STATISTICS_IDS = 13;
pub const MFNETSOURCE_LOGPARAMS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x64936ae8_9418_453a_8cda_3e0a668b353b);
pub const MFNETSOURCE_LOGURL: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3cb1f293_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_LOSTPACKETS_ID: MFNETSOURCE_STATISTICS_IDS = 1;
pub const MFNETSOURCE_MAXBITRATE_ID: MFNETSOURCE_STATISTICS_IDS = 24;
pub const MFNETSOURCE_MAXBUFFERTIMEMS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x408b24e6_4038_4401_b5b2_fe701a9ebf10);
pub const MFNETSOURCE_MAXUDPACCELERATEDSTREAMINGDURATION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4aab2879_bbe1_4994_9ff0_5495bd250129);
pub const MFNETSOURCE_MULTICAST: MFNETSOURCE_PROTOCOL_TYPE = 4;
pub const MFNETSOURCE_OUTPACKETS_ID: MFNETSOURCE_STATISTICS_IDS = 6;
pub const MFNETSOURCE_PEERMANAGER: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x48b29adb_febf_45ee_a9bf_efb81c492efc);
pub const MFNETSOURCE_PLAYERID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3cb1f28e_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_PLAYERUSERAGENT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3cb1f292_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_PLAYERVERSION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3cb1f28d_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_PPBANDWIDTH: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3cb1f281_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_PREVIEWMODEENABLED: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3cb1f27f_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_PROTOCOL: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3cb1f27d_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_PROTOCOL_ID: MFNETSOURCE_STATISTICS_IDS = 10;
pub type MFNETSOURCE_PROTOCOL_TYPE = i32;
pub const MFNETSOURCE_PROXYBYPASSFORLOCAL: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3cb1f286_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_PROXYEXCEPTIONLIST: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3cb1f285_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_PROXYHOSTNAME: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3cb1f284_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_PROXYINFO: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3cb1f29b_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_PROXYLOCATORFACTORY: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3cb1f283_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_PROXYPORT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3cb1f288_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_PROXYRERUNAUTODETECTION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3cb1f289_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_PROXYSETTINGS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3cb1f287_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_RECEPTION_QUALITY_ID: MFNETSOURCE_STATISTICS_IDS = 25;
pub const MFNETSOURCE_RECOVEREDBYECCPACKETS_ID: MFNETSOURCE_STATISTICS_IDS = 4;
pub const MFNETSOURCE_RECOVEREDBYRTXPACKETS_ID: MFNETSOURCE_STATISTICS_IDS = 5;
pub const MFNETSOURCE_RECOVEREDPACKETS_ID: MFNETSOURCE_STATISTICS_IDS = 26;
pub const MFNETSOURCE_RECVPACKETS_ID: MFNETSOURCE_STATISTICS_IDS = 0;
pub const MFNETSOURCE_RECVRATE_ID: MFNETSOURCE_STATISTICS_IDS = 7;
pub const MFNETSOURCE_RESENDSENABLED: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3cb1f27b_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_RESENDSRECEIVED_ID: MFNETSOURCE_STATISTICS_IDS = 3;
pub const MFNETSOURCE_RESENDSREQUESTED_ID: MFNETSOURCE_STATISTICS_IDS = 2;
pub const MFNETSOURCE_RESOURCE_FILTER: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x815d0ff6_265a_4477_9e46_7b80ad80b5fb);
pub const MFNETSOURCE_RTSP: MFNETSOURCE_PROTOCOL_TYPE = 2;
pub const MFNETSOURCE_SEEKRANGEEND_ID: MFNETSOURCE_STATISTICS_IDS = 20;
pub const MFNETSOURCE_SEEKRANGESTART_ID: MFNETSOURCE_STATISTICS_IDS = 19;
pub const MFNETSOURCE_SIGNEDSESSION_ID: MFNETSOURCE_STATISTICS_IDS = 23;
pub const MFNETSOURCE_SPEEDFACTOR_ID: MFNETSOURCE_STATISTICS_IDS = 15;
pub const MFNETSOURCE_SSLCERTIFICATE_MANAGER: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x55e6cb27_e69b_4267_940c_2d7ec5bb8a0f);
pub const MFNETSOURCE_STATISTICS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3cb1f274_0505_4c5d_ae71_0a556344efa1);
pub type MFNETSOURCE_STATISTICS_IDS = i32;
pub const MFNETSOURCE_STATISTICS_SERVICE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3cb1f275_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_STREAM_LANGUAGE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9ab44318_f7cd_4f2d_8d6d_fa35b492cecb);
pub const MFNETSOURCE_TCP: MFNETSOURCE_TRANSPORT_TYPE = 1;
pub const MFNETSOURCE_THINNINGENABLED: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3cb1f27c_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_TRANSPORT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3cb1f27e_0505_4c5d_ae71_0a556344efa1);
pub const MFNETSOURCE_TRANSPORT_ID: MFNETSOURCE_STATISTICS_IDS = 11;
pub type MFNETSOURCE_TRANSPORT_TYPE = i32;
pub const MFNETSOURCE_UDP: MFNETSOURCE_TRANSPORT_TYPE = 0;
pub const MFNETSOURCE_UDP_PORT_RANGE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3cb1f29a_0505_4c5d_ae71_0a556344efa1);
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
pub const MFNET_SAVEJOB_SERVICE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb85a587f_3d02_4e52_9565_55d3ec1e7ff7);
pub type MFNetAuthenticationFlags = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MFNetCredentialManagerGetParam {
    pub hrOp: windows_sys::core::HRESULT,
    pub fAllowLoggedOnUser: windows_sys::core::BOOL,
    pub fClearTextPackage: windows_sys::core::BOOL,
    pub pszUrl: windows_sys::core::PCWSTR,
    pub pszSite: windows_sys::core::PCWSTR,
    pub pszRealm: windows_sys::core::PCWSTR,
    pub pszPackage: windows_sys::core::PCWSTR,
    pub nRetries: super::LONG,
}
pub type MFNetCredentialOptions = i32;
pub type MFNetCredentialRequirements = i32;
#[cfg(feature = "shared")]
pub const MFOUTPUTATTRIBUTE_BUS: super::DWORD = 32;
#[cfg(feature = "shared")]
pub const MFOUTPUTATTRIBUTE_BUSIMPLEMENTATION: super::DWORD = 65280;
#[cfg(feature = "shared")]
pub const MFOUTPUTATTRIBUTE_COMPRESSED: super::DWORD = 8;
#[cfg(feature = "shared")]
pub const MFOUTPUTATTRIBUTE_DIGITAL: super::DWORD = 1;
#[cfg(feature = "shared")]
pub const MFOUTPUTATTRIBUTE_NONSTANDARDIMPLEMENTATION: super::DWORD = 2;
#[cfg(feature = "shared")]
pub const MFOUTPUTATTRIBUTE_SOFTWARE: super::DWORD = 16;
#[cfg(feature = "shared")]
pub const MFOUTPUTATTRIBUTE_VIDEO: super::DWORD = 4;
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_ASFMediaSource_ApproxSeek: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0xb4cd270f_244d_4969_bb92_3f0fb8316f10), pid: 1 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_ASFMediaSource_IterativeSeekIfNoIndex: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0x170b65dc_4a4e_407a_ac22_577f50e4a37c), pid: 1 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_ASFMediaSource_IterativeSeek_Max_Count: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0x170b65dc_4a4e_407a_ac22_577f50e4a37c), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_ASFMediaSource_IterativeSeek_Tolerance_In_MilliSecond: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0x170b65dc_4a4e_407a_ac22_577f50e4a37c), pid: 3 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_Content_DLNA_Profile_ID: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0xcfa31b45_525d_4998_bb44_3f7d81542fa4), pid: 1 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_HTTP_ByteStream_Cache_Limit: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0x86a2403e_c78b_44d7_8bc8_ff7258117508), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_HTTP_ByteStream_Caching_Mode: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0x86a2403e_c78b_44d7_8bc8_ff7258117508), pid: 1 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_HTTP_ByteStream_Download_Mode: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0x817f11b7_a982_46ec_a449_ef58aed53ca8), pid: 1 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_HTTP_ByteStream_Enable_Urlmon: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0xeda8afdf_c171_417f_8d17_2e0918303292), pid: 1 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_HTTP_ByteStream_Urlmon_Bind_Flags: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0xeda8afdf_c171_417f_8d17_2e0918303292), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_HTTP_ByteStream_Urlmon_Callback_QueryService: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0xeda8afdf_c171_417f_8d17_2e0918303292), pid: 5 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_HTTP_ByteStream_Urlmon_Security_Id: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0xeda8afdf_c171_417f_8d17_2e0918303292), pid: 3 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_HTTP_ByteStream_Urlmon_Window: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0xeda8afdf_c171_417f_8d17_2e0918303292), pid: 4 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_MediaProtectionContainerGuid: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0x42af3d7c_00cf_4a0f_81f0_adf524a5a5b5), pid: 1 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_MediaProtectionSystemContext: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0x636b271d_ddc7_49e9_a6c6_47385962e5bd), pid: 2 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_MediaProtectionSystemContextsPerTrack: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0x4454b092_d3da_49b0_8452_6850c7db764d), pid: 3 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_MediaProtectionSystemId: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0x636b271d_ddc7_49e9_a6c6_47385962e5bd), pid: 1 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_MediaProtectionSystemIdMapping: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0x636b271d_ddc7_49e9_a6c6_47385962e5bd), pid: 3 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_MediaSource_DisableReadAhead: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0x26366c14_c5bf_4c76_887b_9f1754db5f09), pid: 1 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_PMP_Creation_Callback: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0x28bb4de2_26a2_4870_b720_d26bbeb14942), pid: 1 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_SBESourceMode: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0x3fae10bb_f859_4192_b562_1868d3da3a02), pid: 1 };
#[cfg(all(feature = "shared", feature = "wtypes"))]
pub const MFPKEY_SourceOpenMonitor: super::PROPERTYKEY = super::PROPERTYKEY { fmtid: windows_sys::core::GUID::from_u128(0x074d4637_b5ae_465d_af17_1a538d2859dd), pid: 2 };
pub type MFPMPSESSION_CREATION_FLAGS = i32;
pub const MFPMPSESSION_IN_PROCESS: MFPMPSESSION_CREATION_FLAGS = 2;
pub const MFPMPSESSION_UNPROTECTED_PROCESS: MFPMPSESSION_CREATION_FLAGS = 1;
pub type MFPOLICYMANAGER_ACTION = i32;
pub const MFPROTECTIONATTRIBUTE_BEST_EFFORT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc8e06331_75f0_4ec1_8e77_17578f773b46);
pub const MFPROTECTIONATTRIBUTE_CONSTRICTVIDEO_IMAGESIZE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x008476fc_4b58_4d80_a790_e7297673161d);
pub const MFPROTECTIONATTRIBUTE_FAIL_OVER: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8536abc5_38f1_4151_9cce_f55d941229ac);
pub const MFPROTECTIONATTRIBUTE_HDCP_SRM: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6f302107_3477_4468_8a08_eef9db10e20f);
pub const MFPROTECTION_ACP: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc3fd11c6_f8b7_4d20_b008_1db17d61f2da);
pub const MFPROTECTION_CGMSA: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe57e69e9_226b_4d31_b4e3_d3db008736dd);
pub const MFPROTECTION_CONSTRICTAUDIO: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xffc99b44_df48_4e16_8e66_096892c1578a);
pub const MFPROTECTION_CONSTRICTVIDEO: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x193370ce_c5e4_4c3a_8a66_6959b4da4442);
pub const MFPROTECTION_CONSTRICTVIDEO_NOOPM: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa580e8cd_c247_4957_b983_3c2eebd1ff59);
pub const MFPROTECTION_DISABLE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8cc6d81b_fec6_4d8f_964b_cfba0b0dad0d);
pub const MFPROTECTION_DISABLE_SCREEN_SCRAPE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa21179a4_b7cd_40d8_9614_8ef2371ba78d);
pub const MFPROTECTION_FFT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x462a56b2_2866_4bb6_980d_6d8d9edb1a8c);
pub const MFPROTECTION_GRAPHICS_TRANSFER_AES_ENCRYPTION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc873de64_d8a5_49e6_88bb_fb963fd3d4ce);
pub const MFPROTECTION_HARDWARE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4ee7f0c1_9ed7_424f_b6be_996b33528856);
pub const MFPROTECTION_HDCP: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xae7cc03d_c828_4021_acb7_d578d27aaf13);
pub const MFPROTECTION_HDCP_WITH_TYPE_ENFORCEMENT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa4a585e8_ed60_442d_814d_db4d4220a06d);
pub const MFPROTECTION_PROTECTED_SURFACE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4f5d9566_e742_4a25_8d1f_d287b5fa0ade);
pub const MFPROTECTION_TRUSTEDAUDIODRIVERS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x65bdf3d2_0168_4816_a533_55d47b027101);
pub const MFPROTECTION_VIDEO_FRAMES: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x36a59cbc_7401_4a8c_bc20_46a7c9e597f0);
pub const MFPROTECTION_WMDRMOTA: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa267a6a1_362e_47d0_8805_4628598a23e4);
pub type MFRATE_DIRECTION = i32;
pub const MFRATE_FORWARD: MFRATE_DIRECTION = 0;
pub const MFRATE_REVERSE: MFRATE_DIRECTION = 1;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct MFRR_COMPONENTS {
    pub dwRRInfoVersion: super::DWORD,
    pub dwRRComponents: super::DWORD,
    pub pRRComponents: PMFRR_COMPONENT_HASH_INFO,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MFRR_COMPONENT_HASH_INFO {
    pub ulReason: super::DWORD,
    pub rgHeaderHash: [super::WCHAR; 43],
    pub rgPublicKeyHash: [super::WCHAR; 43],
    pub wszName: [super::WCHAR; 260],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for MFRR_COMPONENT_HASH_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MFRR_INFO_VERSION: i32 = 0;
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
pub const MFSampleExtension_DeviceTimestamp: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8f3e35e7_2dcd_4887_8622_2a58baa652b0);
pub const MFSampleExtension_ExtendedCameraIntrinsics: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x560bc4a5_4de0_4113_9cdc_832db9740f3d);
pub const MFSampleExtension_Spatial_CameraCoordinateSystem: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9d13c82f_2199_4e67_91cd_d1a4181f2534);
pub const MFSampleExtension_Spatial_CameraProjectionTransform: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x47f9fcb5_2a02_4f26_a477_792fdf95886a);
pub const MFSampleExtension_Spatial_CameraViewTransform: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4e251fa4_830f_4770_859a_4b8d99aa809b);
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
#[cfg(all(feature = "mfobjects", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct MFTOPONODE_ATTRIBUTE_UPDATE {
    pub NodeId: TOPOID,
    pub guidAttributeKey: windows_sys::core::GUID,
    pub attrType: super::MF_ATTRIBUTE_TYPE,
    pub Anonymous: MFTOPONODE_ATTRIBUTE_UPDATE_0,
}
#[cfg(all(feature = "mfobjects", feature = "shared"))]
impl Default for MFTOPONODE_ATTRIBUTE_UPDATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "mfobjects", feature = "shared"))]
#[derive(Clone, Copy)]
pub union MFTOPONODE_ATTRIBUTE_UPDATE_0 {
    pub u32: super::UINT32,
    pub u64: super::UINT64,
    pub d: f64,
}
#[cfg(all(feature = "mfobjects", feature = "shared"))]
impl Default for MFTOPONODE_ATTRIBUTE_UPDATE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "guiddef", feature = "mfobjects", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct MFT_REGISTRATION_INFO {
    pub clsid: super::CLSID,
    pub guidCategory: windows_sys::core::GUID,
    pub uiFlags: super::UINT32,
    pub pszName: windows_sys::core::PCWSTR,
    pub cInTypes: super::DWORD,
    pub pInTypes: *mut super::MFT_REGISTER_TYPE_INFO,
    pub cOutTypes: super::DWORD,
    pub pOutTypes: *mut super::MFT_REGISTER_TYPE_INFO,
}
pub const MFTranscodeContainerType_3GP: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x34c50167_4472_4f34_9ea0_c49fbacf037d);
pub const MFTranscodeContainerType_AC3: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6d8d91c3_8c91_4ed1_8742_8c347d5b44d0);
pub const MFTranscodeContainerType_ADTS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x132fd27d_0f02_43de_a301_38fbbbb3834e);
pub const MFTranscodeContainerType_AMR: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x025d5ad3_621a_475b_964d_66b1c824f079);
pub const MFTranscodeContainerType_ASF: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x430f6f6e_b6bf_4fc1_a0bd_9ee46eee2afb);
pub const MFTranscodeContainerType_AVI: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7edfe8af_402f_4d76_a33c_619fd157d0f1);
pub const MFTranscodeContainerType_FLAC: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x31344aa3_05a9_42b5_901b_8e9d4257f75e);
pub const MFTranscodeContainerType_FMPEG4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9ba876f1_419f_4b77_a1e0_35959d9d4004);
pub const MFTranscodeContainerType_MP3: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe438b912_83f1_4de6_9e3a_9ffbc6dd24d1);
pub const MFTranscodeContainerType_MPEG2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xbfc2dbf9_7bb4_4f8f_afde_e112c44ba882);
pub const MFTranscodeContainerType_MPEG4: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xdc6cd05d_b9d0_40ef_bd35_fa622c1ab28a);
pub const MFTranscodeContainerType_WAVE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x64c3453c_0f26_4741_be63_87bdf8bb935b);
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
pub const MF_ACCESS_CONTROLLED_MEDIASOURCE_SERVICE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x014a5031_2f05_4c6a_9f9c_7d0dc4eda5f4);
pub const MF_ACOUSTIC_ECHO_CANCELLATION_CONTROL_SERVICE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7f6c3b29_2d12_4f6f_ac05_c1a89b8d5288);
pub const MF_ACTIVATE_CUSTOM_MIXER_ALLOWFAIL: i32 = 1;
pub const MF_ACTIVATE_CUSTOM_PRESENTER_ALLOWFAIL: i32 = 1;
pub const MF_ACTIVATE_CUSTOM_VIDEO_MIXER_ACTIVATE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xba491361_be50_451e_95ab_6d4accc7dad8);
pub const MF_ACTIVATE_CUSTOM_VIDEO_MIXER_CLSID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xba491360_be50_451e_95ab_6d4accc7dad8);
pub const MF_ACTIVATE_CUSTOM_VIDEO_MIXER_FLAGS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xba491362_be50_451e_95ab_6d4accc7dad8);
pub const MF_ACTIVATE_CUSTOM_VIDEO_PRESENTER_ACTIVATE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xba491365_be50_451e_95ab_6d4accc7dad8);
pub const MF_ACTIVATE_CUSTOM_VIDEO_PRESENTER_CLSID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xba491364_be50_451e_95ab_6d4accc7dad8);
pub const MF_ACTIVATE_CUSTOM_VIDEO_PRESENTER_FLAGS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xba491366_be50_451e_95ab_6d4accc7dad8);
pub const MF_ACTIVATE_MFT_LOCKED: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc1f6093c_7f65_4fbd_9e39_5faec3c4fbd7);
pub const MF_ACTIVATE_VIDEO_WINDOW: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9a2dbbdd_f57e_4162_82b9_6831377682d3);
pub const MF_AUDIO_EFFECTS_MANAGER_SERVICE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1f541943_d5df_455e_a2e5_7d64d3bbbdb5);
pub const MF_AUDIO_RENDERER_ATTRIBUTE_ENDPOINT_ID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb10aaec3_ef71_4cc3_b873_05a9a08b9f8e);
pub const MF_AUDIO_RENDERER_ATTRIBUTE_ENDPOINT_ROLE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6ba644ff_27c5_4d02_9887_c28619fdb91b);
pub const MF_AUDIO_RENDERER_ATTRIBUTE_FLAGS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xede4b5e0_f805_4d6c_99b3_db01bf95dfab);
pub const MF_AUDIO_RENDERER_ATTRIBUTE_FLAGS_CROSSPROCESS: i32 = 1;
pub const MF_AUDIO_RENDERER_ATTRIBUTE_FLAGS_DONT_ALLOW_FORMAT_CHANGES: i32 = 4;
pub const MF_AUDIO_RENDERER_ATTRIBUTE_FLAGS_NOPERSIST: i32 = 2;
pub const MF_AUDIO_RENDERER_ATTRIBUTE_SESSION_ID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xede4b5e3_f805_4d6c_99b3_db01bf95dfab);
pub const MF_AUDIO_RENDERER_ATTRIBUTE_STREAM_CATEGORY: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa9770471_92ec_4df4_94fe_81c36f0c3a7a);
pub const MF_BD_MVC_PLANE_OFFSET_METADATA: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x62a654e4_b76c_4901_9823_2cb615d47318);
pub const MF_BOOT_DRIVER_VERIFICATION_FAILED: i32 = 1048576;
pub const MF_BYTESTREAMHANDLER_ACCEPTS_SHARE_WRITE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa6e1f733_3001_4915_8150_1558a2180ec8);
pub const MF_BYTESTREAM_SERVICE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xab025e2b_16d9_4180_a127_ba6c70156161);
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MF_BYTE_STREAM_CACHE_RANGE {
    pub qwStartOffset: super::QWORD,
    pub qwEndOffset: super::QWORD,
}
pub const MF_CAMERASOURCE_PROVIDE_SELECTED_PROFILE_ON_START: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa9b46058_82f2_4e5c_bf6e_25b4b09f22ed);
pub type MF_CAMERA_CONTROL_CONFIGURATION_TYPE = i32;
pub const MF_CAMERA_CONTROL_CONFIGURATION_TYPE_POSTSTART: MF_CAMERA_CONTROL_CONFIGURATION_TYPE = 1;
pub const MF_CAMERA_CONTROL_CONFIGURATION_TYPE_PRESTART: MF_CAMERA_CONTROL_CONFIGURATION_TYPE = 0;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MF_CAMERA_CONTROL_RANGE_INFO {
    pub minValue: super::LONG,
    pub maxValue: super::LONG,
    pub stepValue: super::LONG,
    pub defaultValue: super::LONG,
}
pub const MF_COMPONENT_CERT_REVOKED: i32 = 32768;
pub const MF_COMPONENT_HS_CERT_REVOKED: i32 = 131072;
pub const MF_COMPONENT_INVALID_EKU: i32 = 16384;
pub const MF_COMPONENT_INVALID_ROOT: i32 = 65536;
pub const MF_COMPONENT_LS_CERT_REVOKED: i32 = 262144;
pub const MF_COMPONENT_REVOKED: i32 = 8192;
pub const MF_CONNECT_ALLOW_CONVERTER: MF_CONNECT_METHOD = 1;
pub const MF_CONNECT_ALLOW_DECODER: MF_CONNECT_METHOD = 3;
pub const MF_CONNECT_AS_OPTIONAL: MF_CONNECT_METHOD = 65536;
pub const MF_CONNECT_AS_OPTIONAL_BRANCH: MF_CONNECT_METHOD = 131072;
pub const MF_CONNECT_DIRECT: MF_CONNECT_METHOD = 0;
pub type MF_CONNECT_METHOD = i32;
pub const MF_CONNECT_RESOLVE_INDEPENDENT_OUTPUTTYPES: MF_CONNECT_METHOD = 4;
pub const MF_CONTENT_DECRYPTOR_SERVICE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x68a72927_fc7b_44ee_85f4_7c51bd55a659);
pub const MF_CONTENT_PROTECTION_DEVICE_SERVICE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xff58436f_76a0_41fe_b566_10cc53962edd);
pub type MF_CROSS_ORIGIN_POLICY = i32;
pub const MF_CROSS_ORIGIN_POLICY_ANONYMOUS: MF_CROSS_ORIGIN_POLICY = 1;
pub const MF_CROSS_ORIGIN_POLICY_NONE: MF_CROSS_ORIGIN_POLICY = 0;
pub const MF_CROSS_ORIGIN_POLICY_USE_CREDENTIALS: MF_CROSS_ORIGIN_POLICY = 2;
pub const MF_DEVICEMFT_CONNECTED_FILTER_KSCONTROL: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6a2c4fa6_d179_41cd_9523_822371ea40e5);
pub const MF_DEVICEMFT_CONNECTED_PIN_KSCONTROL: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe63310f7_b244_4ef8_9a7d_24c74e32ebd0);
pub const MF_DEVICEMFT_EXTENSION_PLUGIN_CLSID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0844dbae_34fa_48a0_a783_8e696fb1c9a8);
pub const MF_DEVICEMFT_SENSORPROFILE_COLLECTION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x36ebdc44_b12c_441b_89f4_08b2f41a9cfc);
pub const MF_DEVICESTREAM_EXTENSION_PLUGIN_CLSID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x048e6558_60c4_4173_bd5b_6a3ca2896aee);
pub const MF_DEVICESTREAM_EXTENSION_PLUGIN_CONNECTION_POINT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x37f9375c_e664_4ea4_aae4_cb6d1daca1f4);
pub const MF_DEVICESTREAM_FILTER_KSCONTROL: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x46783cca_3df5_4923_a9ef_36b7223edde0);
pub const MF_DEVICESTREAM_FRAMESERVER_HIDDEN: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf402567b_4d91_4179_96d1_74c8480c2034);
pub const MF_DEVICESTREAM_FRAMESERVER_SHARED: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1cb378e9_b279_41d4_af97_34a243e68320);
pub const MF_DEVICESTREAM_IMAGE_STREAM: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa7ffb865_e7b2_43b0_9f6f_9af2a0e50fc0);
pub const MF_DEVICESTREAM_INDEPENDENT_IMAGE_STREAM: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x03eeec7e_d605_4576_8b29_6580b490d7d3);
pub const MF_DEVICESTREAM_MAX_FRAME_BUFFERS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1684cebe_3175_4985_882c_0efd3e8ac11e);
pub const MF_DEVICESTREAM_PIN_KSCONTROL: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xef3ef9a7_87f2_48ca_be02_674878918e98);
pub const MF_DEVICESTREAM_REQUIRED_CAPABILITIES: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6d8b957e_7cf6_43f4_af56_9c0e1e4fcbe1);
pub const MF_DEVICESTREAM_REQUIRED_SDDL: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x331ae85d_c0d3_49ba_83ba_82a12d63cdd6);
pub const MF_DEVICESTREAM_SENSORSTREAM_ID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe35b9fe4_0659_4cad_bb51_33160be7e413);
pub const MF_DEVICESTREAM_SOURCE_ATTRIBUTES: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2f8cb617_361b_434f_85ea_99a03e1ce4e0);
pub const MF_DEVICESTREAM_STREAM_CATEGORY: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2939e7b8_a62e_4579_b674_d4073dfabbba);
pub const MF_DEVICESTREAM_STREAM_ID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x11bd5120_d124_446b_88e6_17060257fff9);
pub const MF_DEVICESTREAM_TAKEPHOTO_TRIGGER: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1d180e34_538c_4fbb_a75a_859af7d261a6);
pub const MF_DEVICESTREAM_TRANSFORM_STREAM_ID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe63937b7_daaf_4d49_815f_d826f8ad31e7);
pub const MF_DEVICE_THERMAL_STATE_CHANGED: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x70ccd0af_fc9f_4deb_a875_9fecd16c5bd4);
pub const MF_DEVSOURCE_ATTRIBUTE_FRAMESERVER_SHARE_MODE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x44d1a9bc_2999_4238_ae43_0730ceb2ab1b);
pub const MF_DEVSOURCE_ATTRIBUTE_FRIENDLY_NAME: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x60d0e559_52f8_4fa2_bbce_acdb34a8ec01);
pub const MF_DEVSOURCE_ATTRIBUTE_MEDIA_TYPE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x56a819ca_0c78_4de4_a0a7_3ddaba0f24d4);
pub const MF_DEVSOURCE_ATTRIBUTE_SOURCE_PASSWORD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa0fd7e16_42d9_49df_84c0_e82c5eab8874);
pub const MF_DEVSOURCE_ATTRIBUTE_SOURCE_STREAM_URL: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9d7b40d2_3617_4043_93e3_8d6da9bb3492);
pub const MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc60ac5fe_252a_478f_a0ef_bc8fa5f7cad3);
pub const MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_AUDCAP_ENDPOINT_ID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x30da9258_feb9_47a7_a453_763a7a8e1c5f);
pub const MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_AUDCAP_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x14dd9a1c_7cff_41be_b1b9_ba1ac6ecb571);
pub const MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_AUDCAP_ROLE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xbc9d118e_8c67_4a18_85d4_12d300400552);
pub const MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_AUDCAP_SYMBOLIC_LINK: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x98d24b5e_5930_4614_b5a1_f600f9355a78);
pub const MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_CATEGORY: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x77f0ae69_c3bd_4509_941d_467e4d24899e);
pub const MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8ac3587a_4ae7_42d8_99e0_0a6013eef90f);
pub const MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_HW_SOURCE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xde7046ba_54d6_4487_a2a4_ec7c0d1bd163);
pub const MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_MAX_BUFFERS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7dd9b730_4f2d_41d5_8f95_0cc9a912ba26);
pub const MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_PROVIDER_DEVICE_ID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x36689d42_a06c_40ae_84cf_f5a034067cc4);
pub const MF_DEVSOURCE_ATTRIBUTE_SOURCE_TYPE_VIDCAP_SYMBOLIC_LINK: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x58f0aad8_22bf_4f8a_bb3d_d2c4978c6e2f);
pub const MF_DEVSOURCE_ATTRIBUTE_SOURCE_USERNAME: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x05d01add_949f_46eb_bc8e_8b0d2b32d79d);
pub const MF_DEVSOURCE_ATTRIBUTE_SOURCE_XADDRESS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xbca0be52_c327_44c7_9b7d_7fa8d9b5bcda);
pub const MF_DISABLE_LOCALLY_REGISTERED_PLUGINS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x66b16da9_add4_47e0_a16b_5af1fb483634);
pub const MF_DROP_MODE_1: MF_QUALITY_DROP_MODE = 1;
pub const MF_DROP_MODE_2: MF_QUALITY_DROP_MODE = 2;
pub const MF_DROP_MODE_3: MF_QUALITY_DROP_MODE = 3;
pub const MF_DROP_MODE_4: MF_QUALITY_DROP_MODE = 4;
pub const MF_DROP_MODE_5: MF_QUALITY_DROP_MODE = 5;
pub const MF_DROP_MODE_NONE: MF_QUALITY_DROP_MODE = 0;
pub const MF_GRL_ABSENT: i32 = 4096;
pub const MF_GRL_LOAD_FAILED: i32 = 16;
pub const MF_INDEPENDENT_STILL_IMAGE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xea12af41_0710_42c9_a127_daa3e78483a5);
pub const MF_INVALID_GRL_SIGNATURE: i32 = 32;
pub const MF_KERNEL_MODE_COMPONENT_LOAD: i32 = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MF_LEAKY_BUCKET_PAIR {
    pub dwBitrate: super::DWORD,
    pub msBufferWindow: super::DWORD,
}
pub const MF_LICENSE_URL_TAMPERED: MF_URL_TRUST_STATUS = 2;
pub const MF_LICENSE_URL_TRUSTED: MF_URL_TRUST_STATUS = 1;
pub const MF_LICENSE_URL_UNTRUSTED: MF_URL_TRUST_STATUS = 0;
pub const MF_LOCAL_MFT_REGISTRATION_SERVICE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xddf5cf9c_4506_45aa_abf0_6d5d94dd1b4a);
pub const MF_LOCAL_PLUGIN_CONTROL_POLICY: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd91b0085_c86d_4f81_8822_8c68e1d7fa04);
pub const MF_LUMA_KEY_ENABLE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7369820f_76de_43ca_9284_47b8f37e0649);
pub const MF_LUMA_KEY_LOWER: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x93d7b8d5_0b81_4715_aea0_8725871621e9);
pub const MF_LUMA_KEY_UPPER: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd09f39bb_4602_4c31_a706_a12171a5110a);
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
pub const MF_MEDIASOURCE_EXPOSE_ALL_STREAMS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe7f250b8_8fd9_4a09_b6c1_6a315c7c720e);
pub const MF_MEDIASOURCE_SERVICE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf09992f7_9fba_4c4a_a37f_8c47b4e1dfe7);
pub type MF_MEDIASOURCE_STATUS_INFO = i32;
pub const MF_MEDIASOURCE_STATUS_INFO_FULLYSUPPORTED: MF_MEDIASOURCE_STATUS_INFO = 0;
pub const MF_MEDIASOURCE_STATUS_INFO_UNKNOWN: MF_MEDIASOURCE_STATUS_INFO = 1;
pub const MF_MEDIA_PROTECTION_MANAGER_PROPERTIES: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x38bd81a9_acea_4c73_89b2_5532c0aeca79);
pub const MF_METADATA_PROVIDER_SERVICE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xdb214084_58a4_4d2e_b84f_6f755b2f7a0d);
pub const MF_MINCRYPT_FAILURE: i32 = 268435456;
pub const MF_MPEG4SINK_MAX_CODED_SEQUENCES_PER_FRAGMENT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xfc1b3bd6_692d_4ce5_9299_738aa5463e9a);
pub const MF_MPEG4SINK_MINIMUM_PROPERTIES_SIZE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xdca1ed52_450e_4a22_8c62_4ed452f7a187);
pub const MF_MPEG4SINK_MIN_FRAGMENT_DURATION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa30b570c_8efd_45e8_94fe_27c84b5bdff6);
pub const MF_MPEG4SINK_MOOV_BEFORE_MDAT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf672e3ac_e1e6_4f10_b5ec_5f3b30828816);
pub const MF_MPEG4SINK_SPSPPS_PASSTHROUGH: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5601a134_2005_4ad2_b37d_22a6c554deb2);
pub const MF_NALU_LENGTH_INFORMATION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x19124e7c_ad4b_465f_bb18_20186287b6af);
pub const MF_NALU_LENGTH_SET: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa7911d53_12a4_4965_ae70_6eadd6ff0551);
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
pub const MF_PD_APP_CONTEXT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6c990d32_bb8e_477a_8598_0d5d96fcd88a);
pub const MF_PD_AUDIO_ENCODING_BITRATE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6c990d35_bb8e_477a_8598_0d5d96fcd88a);
pub const MF_PD_AUDIO_ISVARIABLEBITRATE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x33026ee0_e387_4582_ae0a_34a2ad3baa18);
pub const MF_PD_DURATION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6c990d33_bb8e_477a_8598_0d5d96fcd88a);
pub const MF_PD_LAST_MODIFIED_TIME: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6c990d38_bb8e_477a_8598_0d5d96fcd88a);
pub const MF_PD_MIME_TYPE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6c990d37_bb8e_477a_8598_0d5d96fcd88a);
pub const MF_PD_PLAYBACK_BOUNDARY_TIME: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6c990d3b_bb8e_477a_8598_0d5d96fcd88a);
pub const MF_PD_PLAYBACK_ELEMENT_ID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6c990d39_bb8e_477a_8598_0d5d96fcd88a);
pub const MF_PD_PMPHOST_CONTEXT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6c990d31_bb8e_477a_8598_0d5d96fcd88a);
pub const MF_PD_PREFERRED_LANGUAGE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6c990d3a_bb8e_477a_8598_0d5d96fcd88a);
pub const MF_PD_SAMI_STYLELIST: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe0b73c7f_486d_484e_9872_4de5192a7bf8);
pub const MF_PD_TOTAL_FILE_SIZE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6c990d34_bb8e_477a_8598_0d5d96fcd88a);
pub const MF_PD_VIDEO_ENCODING_BITRATE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6c990d36_bb8e_477a_8598_0d5d96fcd88a);
pub const MF_PMP_SERVER_CONTEXT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2f00c910_d2cf_4278_8b6a_d077fac3a25f);
pub const MF_POLICY_ID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb160c24d_c059_48f1_a901_9ee298a9a8c3);
pub const MF_PROGRESSIVE_CODING_CONTENT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8f020eea_1508_471f_9da6_507d7cfa40db);
pub const MF_PROPERTY_HANDLER_SERVICE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa3face02_32b8_41dd_90e7_5fef7c8991b5);
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
pub const MF_QUALITY_NOTIFY_PROCESSING_LATENCY: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf6b44af8_604d_46fe_a95d_45479b10c9bc);
pub const MF_QUALITY_NOTIFY_SAMPLE_LAG: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x30d15206_ed2a_4760_be17_eb4a9f12295c);
pub const MF_QUALITY_SERVICES: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb7e2be11_2f96_4640_b52c_282365bdf16c);
pub const MF_RATE_CONTROL_SERVICE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x866fa297_b802_4bf8_9dc9_5e3b6a9f53c9);
pub const MF_REMOTE_PROXY: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2f00c90e_d2cf_4278_8b6a_d077fac3a25f);
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
pub const MF_SAMI_SERVICE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x49a89ae7_b4d9_4ef2_aa5c_f65a3e05ae4e);
pub const MF_SAMPLEGRABBERSINK_IGNORE_CLOCK: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0efda2c0_2b69_4e2e_ab8d_46dcbff7d25d);
pub const MF_SAMPLEGRABBERSINK_SAMPLE_TIME_OFFSET: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x62e3d776_8100_4e03_a6e8_bd3857ac9c47);
pub const MF_SD_AUDIO_ENCODER_DELAY: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8e85422c_73de_403f_9a35_550ad6e8b951);
pub const MF_SD_AUDIO_ENCODER_PADDING: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x529c7f2c_ac4b_4e3f_bfc3_0902194982cb);
pub const MF_SD_LANGUAGE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00af2180_bdc2_423c_abca_f503593bc121);
pub const MF_SD_MEDIASOURCE_STATUS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1913678b_fc0f_44da_8f43_1ba3b526f4ae);
pub const MF_SD_MUTUALLY_EXCLUSIVE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x023ef79c_388d_487f_ac17_696cd6e3c6f5);
pub const MF_SD_PROTECTED: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00af2181_bdc2_423c_abca_f503593bc121);
pub const MF_SD_SAMI_LANGUAGE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x36fcb98a_6cd0_44cb_acb9_a8f5600dd0bb);
pub const MF_SD_STREAM_NAME: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4f1b099d_d314_41e5_a781_7fefaa4c501f);
pub const MF_SD_SUPPORTS_PROTECTED_CODEC_SWITCH: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8fb6b117_862e_4b31_8dab_5e0a434caef0);
pub const MF_SD_VIDEO_SPHERICAL: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa51da449_3fdc_478c_bcb5_30be76595f55);
pub const MF_SD_VIDEO_SPHERICAL_FORMAT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4a8fc407_6ea1_46c8_b567_6971d4a139c3);
pub const MF_SD_VIDEO_SPHERICAL_INITIAL_VIEWDIRECTION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x11d25a49_bb62_467f_9db1_c17165716c49);
pub const MF_SESSION_APPROX_EVENT_OCCURRENCE_TIME: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x190e852f_6238_42d1_b5af_69ea338ef850);
pub const MF_SESSION_CONTENT_PROTECTION_MANAGER: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1e83d482_1f1c_4571_8405_88f4b2181f74);
pub const MF_SESSION_GLOBAL_TIME: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1e83d482_1f1c_4571_8405_88f4b2181f72);
pub const MF_SESSION_QUALITY_MANAGER: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1e83d482_1f1c_4571_8405_88f4b2181f73);
pub const MF_SESSION_REMOTE_SOURCE_MODE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf4033ef4_9bb3_4378_941f_85a0856bc244);
pub const MF_SESSION_SERVER_CONTEXT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xafe5b291_50fa_46e8_b9be_0c0c3ce4b3a5);
pub const MF_SESSION_TOPOLOADER: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1e83d482_1f1c_4571_8405_88f4b2181f71);
pub const MF_SINK_VIDEO_DISPLAY_ASPECT_RATIO_DENOMINATOR: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6ea1eb97_1fe0_4f10_a6e4_1f4f661564e0);
pub const MF_SINK_VIDEO_DISPLAY_ASPECT_RATIO_NUMERATOR: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd0f33b22_b78a_4879_b455_f03ef3fa82cd);
pub const MF_SINK_VIDEO_NATIVE_HEIGHT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf0ca6705_490c_43e8_941c_c0b3206b9a65);
pub const MF_SINK_VIDEO_NATIVE_WIDTH: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe6d6a707_1505_4747_9b10_72d2d158cb3a);
pub const MF_SINK_VIDEO_PTS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2162bde7_421e_4b90_9b33_e58fbf1d58b6);
pub const MF_SOURCE_PRESENTATION_PROVIDER_SERVICE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe002aadc_f4af_4ee5_9847_053edf840426);
pub const MF_SOURCE_STREAM_SUPPORTS_HW_CONNECTION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa38253aa_6314_42fd_a3ce_bb27b6859946);
pub const MF_STF_VERSION_DATE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x31a165d5_df67_4095_8e44_8868fc20dbfd);
pub const MF_STF_VERSION_INFO: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6770bd39_ef82_44ee_a49b_934beb24aef7);
pub const MF_STREAM_SINK_SUPPORTS_HW_CONNECTION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9b465cbf_0597_4f9e_9f3c_b97eeef90359);
pub const MF_STREAM_SINK_SUPPORTS_ROTATION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb3e96280_bd05_41a5_97ad_8a7fee24b912);
pub const MF_ST_MEDIASOURCE_COLLECTION: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x616de972_83ad_4950_8170_630d19cbe307);
pub const MF_SampleProtectionSalt: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5403deee_b9ee_438f_aa83_3804997e569d);
pub const MF_TEST_SIGNED_COMPONENT_LOADING: i32 = 16777216;
pub const MF_TIMECODE_SERVICE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa0d502a7_0eb3_4885_b1b9_9feb0d083454);
pub const MF_TIME_FORMAT_ENTRY_RELATIVE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4399f178_46d3_4504_afda_20d32e9ba360);
pub const MF_TIME_FORMAT_SEGMENT_OFFSET: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc8b8be77_869c_431d_812e_169693f65a39);
pub const MF_TOPOLOGY_DXVA_MODE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1e8d34f6_f5ab_4e23_bb88_874aa3a1a74d);
pub const MF_TOPOLOGY_DYNAMIC_CHANGE_NOT_ALLOWED: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd529950b_d484_4527_a9cd_b1909532b5b0);
pub const MF_TOPOLOGY_ENABLE_XVP_FOR_PLAYBACK: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1967731f_cd78_42fc_b026_0992a56e5693);
pub const MF_TOPOLOGY_ENUMERATE_SOURCE_TYPES: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6248c36d_5d0b_4f40_a0bb_b0b305f77698);
pub const MF_TOPOLOGY_HARDWARE_MODE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd2d362fd_4e4f_4191_a579_c618b66706af);
pub const MF_TOPOLOGY_MAX: MF_TOPOLOGY_TYPE = -1;
pub const MF_TOPOLOGY_NO_MARKIN_MARKOUT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7ed3f804_86bb_4b3f_b7e4_7cb43afd4b80);
pub const MF_TOPOLOGY_OUTPUT_NODE: MF_TOPOLOGY_TYPE = 0;
pub const MF_TOPOLOGY_PLAYBACK_FRAMERATE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc164737a_c2b1_4553_83bb_5a526072448f);
pub const MF_TOPOLOGY_PLAYBACK_MAX_DIMS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5715cf19_5768_44aa_ad6e_8721f1b0f9bb);
pub const MF_TOPOLOGY_PROJECTSTART: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7ed3f802_86bb_4b3f_b7e4_7cb43afd4b80);
pub const MF_TOPOLOGY_PROJECTSTOP: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7ed3f803_86bb_4b3f_b7e4_7cb43afd4b80);
pub const MF_TOPOLOGY_RESOLUTION_STATUS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x494bbcde_b031_4e38_97c4_d5422dd618dc);
pub type MF_TOPOLOGY_RESOLUTION_STATUS_FLAGS = i32;
pub const MF_TOPOLOGY_RESOLUTION_SUCCEEDED: MF_TOPOLOGY_RESOLUTION_STATUS_FLAGS = 0;
pub const MF_TOPOLOGY_SOURCESTREAM_NODE: MF_TOPOLOGY_TYPE = 1;
pub const MF_TOPOLOGY_START_TIME_ON_PRESENTATION_SWITCH: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc8cc113f_7951_4548_aad6_9ed6202e62b3);
pub const MF_TOPOLOGY_STATIC_PLAYBACK_OPTIMIZATIONS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb86cac42_41a6_4b79_897a_1ab0e52b4a1b);
pub const MF_TOPOLOGY_TEE_NODE: MF_TOPOLOGY_TYPE = 3;
pub const MF_TOPOLOGY_TRANSFORM_NODE: MF_TOPOLOGY_TYPE = 2;
pub type MF_TOPOLOGY_TYPE = i32;
pub const MF_TOPONODE_ATTRIBUTE_EDITOR_SERVICE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x65656e1a_077f_4472_83ef_316f11d5087a);
pub const MF_TOPONODE_CONNECT_METHOD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x494bbcf1_b031_4e38_97c4_d5422dd618dc);
pub const MF_TOPONODE_D3DAWARE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x494bbced_b031_4e38_97c4_d5422dd618dc);
pub const MF_TOPONODE_DECODER: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x494bbd02_b031_4e38_97c4_d5422dd618dc);
pub const MF_TOPONODE_DECRYPTOR: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x494bbcfa_b031_4e38_97c4_d5422dd618dc);
pub const MF_TOPONODE_DISABLE_PREROLL: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x14932f9e_9087_4bb4_8412_5167145cbe04);
pub const MF_TOPONODE_DISCARDABLE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x494bbcfb_b031_4e38_97c4_d5422dd618dc);
pub const MF_TOPONODE_DRAIN: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x494bbce9_b031_4e38_97c4_d5422dd618dc);
pub const MF_TOPONODE_DRAIN_ALWAYS: MF_TOPONODE_DRAIN_MODE = 1;
pub const MF_TOPONODE_DRAIN_DEFAULT: MF_TOPONODE_DRAIN_MODE = 0;
pub type MF_TOPONODE_DRAIN_MODE = i32;
pub const MF_TOPONODE_DRAIN_NEVER: MF_TOPONODE_DRAIN_MODE = 2;
pub const MF_TOPONODE_ERRORCODE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x494bbcee_b031_4e38_97c4_d5422dd618dc);
pub const MF_TOPONODE_ERROR_MAJORTYPE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x494bbcfd_b031_4e38_97c4_d5422dd618dc);
pub const MF_TOPONODE_ERROR_SUBTYPE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x494bbcfe_b031_4e38_97c4_d5422dd618dc);
pub const MF_TOPONODE_FLUSH: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x494bbce8_b031_4e38_97c4_d5422dd618dc);
pub const MF_TOPONODE_FLUSH_ALWAYS: MF_TOPONODE_FLUSH_MODE = 0;
pub type MF_TOPONODE_FLUSH_MODE = i32;
pub const MF_TOPONODE_FLUSH_NEVER: MF_TOPONODE_FLUSH_MODE = 2;
pub const MF_TOPONODE_FLUSH_SEEK: MF_TOPONODE_FLUSH_MODE = 1;
pub const MF_TOPONODE_LOCKED: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x494bbcf7_b031_4e38_97c4_d5422dd618dc);
pub const MF_TOPONODE_MARKIN_HERE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x494bbd00_b031_4e38_97c4_d5422dd618dc);
pub const MF_TOPONODE_MARKOUT_HERE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x494bbd01_b031_4e38_97c4_d5422dd618dc);
pub const MF_TOPONODE_MEDIASTART: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x835c58ea_e075_4bc7_bcba_4de000df9ae6);
pub const MF_TOPONODE_MEDIASTOP: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x835c58eb_e075_4bc7_bcba_4de000df9ae6);
pub const MF_TOPONODE_NOSHUTDOWN_ON_REMOVE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x14932f9c_9087_4bb4_8412_5167145cbe04);
pub const MF_TOPONODE_PRESENTATION_DESCRIPTOR: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x835c58ed_e075_4bc7_bcba_4de000df9ae6);
pub const MF_TOPONODE_PRIMARYOUTPUT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6304ef99_16b2_4ebe_9d67_e4c539b3a259);
pub const MF_TOPONODE_RATELESS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x14932f9d_9087_4bb4_8412_5167145cbe04);
pub const MF_TOPONODE_SEQUENCE_ELEMENTID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x835c58ef_e075_4bc7_bcba_4de000df9ae6);
pub const MF_TOPONODE_SOURCE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x835c58ec_e075_4bc7_bcba_4de000df9ae6);
pub const MF_TOPONODE_STREAMID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x14932f9b_9087_4bb4_8412_5167145cbe04);
pub const MF_TOPONODE_STREAM_DESCRIPTOR: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x835c58ee_e075_4bc7_bcba_4de000df9ae6);
pub const MF_TOPONODE_TRANSFORM_OBJECTID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x88dcc0c9_293e_4e8b_9aeb_0ad64cc016b0);
pub const MF_TOPONODE_WORKQUEUE_ID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x494bbcf8_b031_4e38_97c4_d5422dd618dc);
pub const MF_TOPONODE_WORKQUEUE_ITEM_PRIORITY: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa1ff99be_5e97_4a53_b494_568c642c0ff3);
pub const MF_TOPONODE_WORKQUEUE_MMCSS_CLASS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x494bbcf9_b031_4e38_97c4_d5422dd618dc);
pub const MF_TOPONODE_WORKQUEUE_MMCSS_PRIORITY: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5001f840_2816_48f4_9364_ad1ef661a123);
pub const MF_TOPONODE_WORKQUEUE_MMCSS_TASKID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x494bbcff_b031_4e38_97c4_d5422dd618dc);
pub const MF_TRANSCODE_ADJUST_PROFILE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9c37c21b_060f_487c_a690_80d7f50d1c72);
pub const MF_TRANSCODE_ADJUST_PROFILE_DEFAULT: MF_TRANSCODE_ADJUST_PROFILE_FLAGS = 0;
pub type MF_TRANSCODE_ADJUST_PROFILE_FLAGS = i32;
pub const MF_TRANSCODE_ADJUST_PROFILE_USE_SOURCE_ATTRIBUTES: MF_TRANSCODE_ADJUST_PROFILE_FLAGS = 1;
pub const MF_TRANSCODE_CONTAINERTYPE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x150ff23f_4abc_478b_ac4f_e1916fba1cca);
pub const MF_TRANSCODE_DONOT_INSERT_ENCODER: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf45aa7ce_ab24_4012_a11b_dc8220201410);
pub const MF_TRANSCODE_ENCODINGPROFILE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6947787c_f508_4ea9_b1e9_a1fe3a49fbc9);
pub const MF_TRANSCODE_QUALITYVSSPEED: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x98332df8_03cd_476b_89fa_3f9e442dec9f);
#[repr(C)]
#[cfg(all(feature = "mfobjects", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct MF_TRANSCODE_SINK_INFO {
    pub dwVideoStreamID: super::DWORD,
    pub pVideoMediaType: *mut core::ffi::c_void,
    pub dwAudioStreamID: super::DWORD,
    pub pAudioMediaType: *mut core::ffi::c_void,
}
pub const MF_TRANSCODE_SKIP_METADATA_TRANSFER: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4e4469ef_b571_4959_8f83_3dcfba33a393);
pub const MF_TRANSCODE_TOPOLOGYMODE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3e3df610_394a_40b2_9dea_3bab650bebf2);
pub type MF_TRANSCODE_TOPOLOGYMODE_FLAGS = i32;
pub const MF_TRANSCODE_TOPOLOGYMODE_HARDWARE_ALLOWED: MF_TRANSCODE_TOPOLOGYMODE_FLAGS = 1;
pub const MF_TRANSCODE_TOPOLOGYMODE_SOFTWARE_ONLY: MF_TRANSCODE_TOPOLOGYMODE_FLAGS = 0;
pub const MF_TRANSFER_VIDEO_FRAME_DEFAULT: MF_TRANSFER_VIDEO_FRAME_FLAGS = 0;
pub type MF_TRANSFER_VIDEO_FRAME_FLAGS = i32;
pub const MF_TRANSFER_VIDEO_FRAME_IGNORE_PAR: MF_TRANSFER_VIDEO_FRAME_FLAGS = 2;
pub const MF_TRANSFER_VIDEO_FRAME_STRETCH: MF_TRANSFER_VIDEO_FRAME_FLAGS = 1;
pub const MF_UNKNOWN_DURATION: i32 = 0;
pub type MF_URL_TRUST_STATUS = i32;
pub const MF_USER_DATA_PAYLOAD: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd1d4985d_dc92_457a_b3a0_651a33a31047);
pub const MF_USER_EXTENDED_ATTRIBUTES: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc02abac6_feb2_4541_922f_920b43702722);
pub const MF_USER_MODE_COMPONENT_LOAD: i32 = 1;
pub const MF_VIDEO_PROCESSOR_ALGORITHM: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4a0a1e1f_272c_4fb6_9eb1_db330cbc97ca);
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
pub const MF_WORKQUEUE_SERVICES: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8e37d489_41e0_413a_9068_287c886d8dda);
pub const MF_WRAPPED_BUFFER_SERVICE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xab544072_c269_4ebc_a552_1c3b32bed5ca);
pub const MF_WRAPPED_OBJECT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2b182c4c_d6ac_49f4_8915_f71887db70cd);
pub const MF_WRAPPED_SAMPLE_SERVICE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x31f52bf2_d03e_4048_80d0_9c1046d87c61);
pub const MF_WVC1_PROG_SINGLE_SLICE_CONTENT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x67ec2559_0f2f_4420_a4dd_2f8ee7a5738b);
pub const MF_XVP_CALLER_ALLOCATES_OUTPUT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x04a2cabc_0cab_40b1_a1b9_75bc3658f000);
pub const MF_XVP_DISABLE_FRC: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2c0afa19_7a97_4d5a_9ee8_16d4fc518d8c);
pub const MF_XVP_SAMPLE_LOCK_TIMEOUT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xaa4ddb29_5134_4363_ac72_83ec4bc10426);
pub const MFaudioConstriction14_14: MFAudioConstriction = 3;
pub const MFaudioConstriction44_16: MFAudioConstriction = 2;
pub const MFaudioConstriction48_16: MFAudioConstriction = 1;
pub const MFaudioConstrictionMute: MFAudioConstriction = 4;
pub const MFaudioConstrictionOff: MFAudioConstriction = 0;
pub const MIRROR_HORIZONTAL: MF_VIDEO_PROCESSOR_MIRROR = 1;
pub const MIRROR_NONE: MF_VIDEO_PROCESSOR_MIRROR = 0;
pub const MIRROR_VERTICAL: MF_VIDEO_PROCESSOR_MIRROR = 2;
pub const MR_AUDIO_POLICY_SERVICE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x911fd737_6775_4ab0_a614_297862fdac88);
pub const MR_CAPTURE_POLICY_VOLUME_SERVICE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x24030acd_107a_4265_975c_414e33e65f2a);
pub const MR_POLICY_VOLUME_SERVICE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1abaa2ac_9d3b_47c6_ab48_c59506de784d);
pub const MR_STREAM_VOLUME_SERVICE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf8b5fa2f_32ef_46f5_b172_1321212fb2c4);
pub const PEACTION_COPY: MFPOLICYMANAGER_ACTION = 2;
pub const PEACTION_EXPORT: MFPOLICYMANAGER_ACTION = 3;
pub const PEACTION_EXTRACT: MFPOLICYMANAGER_ACTION = 4;
pub const PEACTION_LAST: MFPOLICYMANAGER_ACTION = 7;
pub const PEACTION_NO: MFPOLICYMANAGER_ACTION = 0;
pub const PEACTION_PLAY: MFPOLICYMANAGER_ACTION = 1;
pub const PEACTION_RESERVED1: MFPOLICYMANAGER_ACTION = 5;
pub const PEACTION_RESERVED2: MFPOLICYMANAGER_ACTION = 6;
pub const PEACTION_RESERVED3: MFPOLICYMANAGER_ACTION = 7;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMFRR_COMPONENTS = *mut MFRR_COMPONENTS;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMFRR_COMPONENT_HASH_INFO = *mut MFRR_COMPONENT_HASH_INFO;
pub const PRESENTATION_CURRENT_POSITION: i64 = 9223372036854775807;
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
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct SENSORPROFILEID {
    pub Type: windows_sys::core::GUID,
    pub Index: super::UINT32,
    pub Unused: super::UINT32,
}
pub const SHA_HASH_LEN: i32 = 20;
pub const STR_HASH_LEN: i32 = 43;
pub const SequencerTopologyFlags_Last: MFSequencerTopologyFlags = 1;
pub type TOPOID = u64;
