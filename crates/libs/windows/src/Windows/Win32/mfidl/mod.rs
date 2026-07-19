#[cfg(feature = "propsys")]
#[inline]
pub unsafe fn CreateNamedPropertyStore() -> windows_core::Result<super::INamedPropertyStore> {
    windows_core::link!("mf.dll" "system" fn CreateNamedPropertyStore(ppstore : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateNamedPropertyStore(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "propsys")]
#[inline]
pub unsafe fn CreatePropertyStore() -> windows_core::Result<super::IPropertyStore> {
    windows_core::link!("mfplat.dll" "system" fn CreatePropertyStore(ppstore : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreatePropertyStore(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreate3GPMediaSink<P0, P1, P2>(pibytestream: P0, pvideomediatype: P1, paudiomediatype: P2) -> windows_core::Result<IMFMediaSink>
where
    P0: windows_core::Param<super::IMFByteStream>,
    P1: windows_core::Param<super::IMFMediaType>,
    P2: windows_core::Param<super::IMFMediaType>,
{
    windows_core::link!("mf.dll" "system" fn MFCreate3GPMediaSink(pibytestream : *mut core::ffi::c_void, pvideomediatype : *mut core::ffi::c_void, paudiomediatype : *mut core::ffi::c_void, ppimediasink : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreate3GPMediaSink(pibytestream.param().abi(), pvideomediatype.param().abi(), paudiomediatype.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateAC3MediaSink<P0, P1>(ptargetbytestream: P0, paudiomediatype: P1) -> windows_core::Result<IMFMediaSink>
where
    P0: windows_core::Param<super::IMFByteStream>,
    P1: windows_core::Param<super::IMFMediaType>,
{
    windows_core::link!("mf.dll" "system" fn MFCreateAC3MediaSink(ptargetbytestream : *mut core::ffi::c_void, paudiomediatype : *mut core::ffi::c_void, ppmediasink : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateAC3MediaSink(ptargetbytestream.param().abi(), paudiomediatype.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateADTSMediaSink<P0, P1>(ptargetbytestream: P0, paudiomediatype: P1) -> windows_core::Result<IMFMediaSink>
where
    P0: windows_core::Param<super::IMFByteStream>,
    P1: windows_core::Param<super::IMFMediaType>,
{
    windows_core::link!("mf.dll" "system" fn MFCreateADTSMediaSink(ptargetbytestream : *mut core::ffi::c_void, paudiomediatype : *mut core::ffi::c_void, ppmediasink : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateADTSMediaSink(ptargetbytestream.param().abi(), paudiomediatype.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateAVIMediaSink<P0, P1, P2>(pibytestream: P0, pvideomediatype: P1, paudiomediatype: P2) -> windows_core::Result<IMFMediaSink>
where
    P0: windows_core::Param<super::IMFByteStream>,
    P1: windows_core::Param<super::IMFMediaType>,
    P2: windows_core::Param<super::IMFMediaType>,
{
    windows_core::link!("mfsrcsnk.dll" "system" fn MFCreateAVIMediaSink(pibytestream : *mut core::ffi::c_void, pvideomediatype : *mut core::ffi::c_void, paudiomediatype : *mut core::ffi::c_void, ppimediasink : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateAVIMediaSink(pibytestream.param().abi(), pvideomediatype.param().abi(), paudiomediatype.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateAggregateSource<P0>(psourcecollection: P0) -> windows_core::Result<IMFMediaSource>
where
    P0: windows_core::Param<super::IMFCollection>,
{
    windows_core::link!("mf.dll" "system" fn MFCreateAggregateSource(psourcecollection : *mut core::ffi::c_void, ppaggsource : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateAggregateSource(psourcecollection.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateAudioRenderer<P0>(paudioattributes: P0) -> windows_core::Result<IMFMediaSink>
where
    P0: windows_core::Param<super::IMFAttributes>,
{
    windows_core::link!("mf.dll" "system" fn MFCreateAudioRenderer(paudioattributes : *mut core::ffi::c_void, ppsink : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateAudioRenderer(paudioattributes.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateAudioRendererActivate() -> windows_core::Result<super::IMFActivate> {
    windows_core::link!("mf.dll" "system" fn MFCreateAudioRendererActivate(ppactivate : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateAudioRendererActivate(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn MFCreateCameraControlMonitor<P0, P1>(symboliclink: P0, callback: P1) -> windows_core::Result<IMFCameraControlMonitor>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<IMFCameraControlNotify>,
{
    windows_core::link!("mfsensorgroup.dll" "system" fn MFCreateCameraControlMonitor(symboliclink : windows_core::PCWSTR, callback : *mut core::ffi::c_void, ppcameracontrolmonitor : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateCameraControlMonitor(symboliclink.param().abi(), callback.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn MFCreateCameraOcclusionStateMonitor<P0, P1>(symboliclink: P0, callback: P1) -> windows_core::Result<IMFCameraOcclusionStateMonitor>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<IMFCameraOcclusionStateReportCallback>,
{
    windows_core::link!("mfsensorgroup.dll" "system" fn MFCreateCameraOcclusionStateMonitor(symboliclink : windows_core::PCWSTR, callback : *mut core::ffi::c_void, occlusionstatemonitor : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateCameraOcclusionStateMonitor(symboliclink.param().abi(), callback.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateContentDecryptorContext<P1, P2>(guidmediaprotectionsystemid: *const windows_core::GUID, pd3dmanager: P1, pcontentprotectiondevice: P2) -> windows_core::Result<IMFContentDecryptorContext>
where
    P1: windows_core::Param<super::IMFDXGIDeviceManager>,
    P2: windows_core::Param<IMFContentProtectionDevice>,
{
    windows_core::link!("mfplat.dll" "system" fn MFCreateContentDecryptorContext(guidmediaprotectionsystemid : *const windows_core::GUID, pd3dmanager : *mut core::ffi::c_void, pcontentprotectiondevice : *mut core::ffi::c_void, ppcontentdecryptorcontext : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateContentDecryptorContext(guidmediaprotectionsystemid, pd3dmanager.param().abi(), pcontentprotectiondevice.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn MFCreateContentProtectionDevice(protectionsystemid: *const windows_core::GUID) -> windows_core::Result<IMFContentProtectionDevice> {
    windows_core::link!("mfplat.dll" "system" fn MFCreateContentProtectionDevice(protectionsystemid : *const windows_core::GUID, contentprotectiondevice : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateContentProtectionDevice(protectionsystemid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn MFCreateCredentialCache() -> windows_core::Result<IMFNetCredentialCache> {
    windows_core::link!("mf.dll" "system" fn MFCreateCredentialCache(ppcache : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateCredentialCache(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateDeviceSource<P0>(pattributes: P0) -> windows_core::Result<IMFMediaSource>
where
    P0: windows_core::Param<super::IMFAttributes>,
{
    windows_core::link!("mf.dll" "system" fn MFCreateDeviceSource(pattributes : *mut core::ffi::c_void, ppsource : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateDeviceSource(pattributes.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateDeviceSourceActivate<P0>(pattributes: P0) -> windows_core::Result<super::IMFActivate>
where
    P0: windows_core::Param<super::IMFAttributes>,
{
    windows_core::link!("mf.dll" "system" fn MFCreateDeviceSourceActivate(pattributes : *mut core::ffi::c_void, ppactivate : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateDeviceSourceActivate(pattributes.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn MFCreateExtendedCameraIntrinsicModel(distortionmodeltype: MFCameraIntrinsic_DistortionModelType) -> windows_core::Result<IMFExtendedCameraIntrinsicModel> {
    windows_core::link!("mfcore.dll" "system" fn MFCreateExtendedCameraIntrinsicModel(distortionmodeltype : MFCameraIntrinsic_DistortionModelType, ppextendedcameraintrinsicmodel : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateExtendedCameraIntrinsicModel(distortionmodeltype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn MFCreateExtendedCameraIntrinsics() -> windows_core::Result<IMFExtendedCameraIntrinsics> {
    windows_core::link!("mfcore.dll" "system" fn MFCreateExtendedCameraIntrinsics(ppextendedcameraintrinsics : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateExtendedCameraIntrinsics(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateFMPEG4MediaSink<P0, P1, P2>(pibytestream: P0, pvideomediatype: P1, paudiomediatype: P2) -> windows_core::Result<IMFMediaSink>
where
    P0: windows_core::Param<super::IMFByteStream>,
    P1: windows_core::Param<super::IMFMediaType>,
    P2: windows_core::Param<super::IMFMediaType>,
{
    windows_core::link!("mf.dll" "system" fn MFCreateFMPEG4MediaSink(pibytestream : *mut core::ffi::c_void, pvideomediatype : *mut core::ffi::c_void, paudiomediatype : *mut core::ffi::c_void, ppimediasink : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateFMPEG4MediaSink(pibytestream.param().abi(), pvideomediatype.param().abi(), paudiomediatype.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "mfobjects", feature = "objidlbase"))]
#[inline]
pub unsafe fn MFCreateMFByteStreamOnStream<P0>(pstream: P0) -> windows_core::Result<super::IMFByteStream>
where
    P0: windows_core::Param<super::IStream>,
{
    windows_core::link!("mfplat.dll" "system" fn MFCreateMFByteStreamOnStream(pstream : *mut core::ffi::c_void, ppbytestream : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateMFByteStreamOnStream(pstream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateMFByteStreamOnStreamEx<P0>(punkstream: P0) -> windows_core::Result<super::IMFByteStream>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("mfplat.dll" "system" fn MFCreateMFByteStreamOnStreamEx(punkstream : *mut core::ffi::c_void, ppbytestream : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateMFByteStreamOnStreamEx(punkstream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateMP3MediaSink<P0>(ptargetbytestream: P0) -> windows_core::Result<IMFMediaSink>
where
    P0: windows_core::Param<super::IMFByteStream>,
{
    windows_core::link!("mf.dll" "system" fn MFCreateMP3MediaSink(ptargetbytestream : *mut core::ffi::c_void, ppmediasink : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateMP3MediaSink(ptargetbytestream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateMPEG4MediaSink<P0, P1, P2>(pibytestream: P0, pvideomediatype: P1, paudiomediatype: P2) -> windows_core::Result<IMFMediaSink>
where
    P0: windows_core::Param<super::IMFByteStream>,
    P1: windows_core::Param<super::IMFMediaType>,
    P2: windows_core::Param<super::IMFMediaType>,
{
    windows_core::link!("mf.dll" "system" fn MFCreateMPEG4MediaSink(pibytestream : *mut core::ffi::c_void, pvideomediatype : *mut core::ffi::c_void, paudiomediatype : *mut core::ffi::c_void, ppimediasink : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateMPEG4MediaSink(pibytestream.param().abi(), pvideomediatype.param().abi(), paudiomediatype.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateMediaSession<P0>(pconfiguration: P0) -> windows_core::Result<IMFMediaSession>
where
    P0: windows_core::Param<super::IMFAttributes>,
{
    windows_core::link!("mf.dll" "system" fn MFCreateMediaSession(pconfiguration : *mut core::ffi::c_void, ppmediasession : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateMediaSession(pconfiguration.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateMediaTypeFromProperties<P0>(punkstream: P0) -> windows_core::Result<super::IMFMediaType>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("mfplat.dll" "system" fn MFCreateMediaTypeFromProperties(punkstream : *mut core::ffi::c_void, ppmediatype : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateMediaTypeFromProperties(punkstream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateMuxSink<P1, P2>(guidoutputsubtype: windows_core::GUID, poutputattributes: P1, poutputbytestream: P2) -> windows_core::Result<IMFMediaSink>
where
    P1: windows_core::Param<super::IMFAttributes>,
    P2: windows_core::Param<super::IMFByteStream>,
{
    windows_core::link!("mf.dll" "system" fn MFCreateMuxSink(guidoutputsubtype : windows_core::GUID, poutputattributes : *mut core::ffi::c_void, poutputbytestream : *mut core::ffi::c_void, ppmuxsink : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateMuxSink(guidoutputsubtype, poutputattributes.param().abi(), poutputbytestream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn MFCreateNetSchemePlugin<T>() -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("mf.dll" "system" fn MFCreateNetSchemePlugin(riid : *const windows_core::GUID, ppvhandler : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { MFCreateNetSchemePlugin(&T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreatePMPMediaSession<P1>(dwcreationflags: u32, pconfiguration: P1, ppmediasession: *mut Option<IMFMediaSession>, ppenableractivate: *mut Option<super::IMFActivate>) -> windows_core::HRESULT
where
    P1: windows_core::Param<super::IMFAttributes>,
{
    windows_core::link!("mf.dll" "system" fn MFCreatePMPMediaSession(dwcreationflags : u32, pconfiguration : *mut core::ffi::c_void, ppmediasession : *mut *mut core::ffi::c_void, ppenableractivate : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { MFCreatePMPMediaSession(dwcreationflags, pconfiguration.param().abi(), core::mem::transmute(ppmediasession), core::mem::transmute(ppenableractivate)) }
}
#[inline]
pub unsafe fn MFCreatePMPServer(dwcreationflags: u32) -> windows_core::Result<IMFPMPServer> {
    windows_core::link!("mf.dll" "system" fn MFCreatePMPServer(dwcreationflags : u32, pppmpserver : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreatePMPServer(dwcreationflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn MFCreatePresentationClock() -> windows_core::Result<IMFPresentationClock> {
    windows_core::link!("mf.dll" "system" fn MFCreatePresentationClock(pppresentationclock : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreatePresentationClock(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreatePresentationDescriptor(apstreamdescriptors: Option<&[Option<IMFStreamDescriptor>]>) -> windows_core::Result<IMFPresentationDescriptor> {
    windows_core::link!("mfplat.dll" "system" fn MFCreatePresentationDescriptor(cstreamdescriptors : u32, apstreamdescriptors : *const *mut core::ffi::c_void, pppresentationdescriptor : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreatePresentationDescriptor(apstreamdescriptors.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(apstreamdescriptors.map_or(core::ptr::null(), |slice| slice.as_ptr())), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreatePropertiesFromMediaType<P0, T>(pmediatype: P0) -> windows_core::Result<T>
where
    P0: windows_core::Param<super::IMFMediaType>,
    T: windows_core::Interface,
{
    windows_core::link!("mfplat.dll" "system" fn MFCreatePropertiesFromMediaType(pmediatype : *mut core::ffi::c_void, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { MFCreatePropertiesFromMediaType(pmediatype.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[inline]
pub unsafe fn MFCreateProtectedEnvironmentAccess() -> windows_core::Result<IMFProtectedEnvironmentAccess> {
    windows_core::link!("mf.dll" "system" fn MFCreateProtectedEnvironmentAccess(ppaccess : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateProtectedEnvironmentAccess(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "propsys")]
#[inline]
pub unsafe fn MFCreateProxyLocator<P0, P1>(pszprotocol: P0, pproxyconfig: P1) -> windows_core::Result<IMFNetProxyLocator>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<super::IPropertyStore>,
{
    windows_core::link!("mf.dll" "system" fn MFCreateProxyLocator(pszprotocol : windows_core::PCWSTR, pproxyconfig : *mut core::ffi::c_void, ppproxylocator : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateProxyLocator(pszprotocol.param().abi(), pproxyconfig.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn MFCreateRelativePanelWatcher<P0, P1>(videodeviceid: P0, displaymonitordeviceid: P1) -> windows_core::Result<IMFRelativePanelWatcher>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mfsensorgroup.dll" "system" fn MFCreateRelativePanelWatcher(videodeviceid : windows_core::PCWSTR, displaymonitordeviceid : windows_core::PCWSTR, pprelativepanelwatcher : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateRelativePanelWatcher(videodeviceid.param().abi(), displaymonitordeviceid.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn MFCreateRemoteDesktopPlugin() -> windows_core::Result<IMFRemoteDesktopPlugin> {
    windows_core::link!("mf.dll" "system" fn MFCreateRemoteDesktopPlugin(ppplugin : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateRemoteDesktopPlugin(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mftransform")]
#[inline]
pub unsafe fn MFCreateSampleCopierMFT() -> windows_core::Result<super::IMFTransform> {
    windows_core::link!("mf.dll" "system" fn MFCreateSampleCopierMFT(ppcopiermft : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateSampleCopierMFT(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateSampleGrabberSinkActivate<P0, P1>(pimfmediatype: P0, pimfsamplegrabbersinkcallback: P1) -> windows_core::Result<super::IMFActivate>
where
    P0: windows_core::Param<super::IMFMediaType>,
    P1: windows_core::Param<IMFSampleGrabberSinkCallback>,
{
    windows_core::link!("mf.dll" "system" fn MFCreateSampleGrabberSinkActivate(pimfmediatype : *mut core::ffi::c_void, pimfsamplegrabbersinkcallback : *mut core::ffi::c_void, ppiactivate : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateSampleGrabberSinkActivate(pimfmediatype.param().abi(), pimfsamplegrabbersinkcallback.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn MFCreateSensorActivityMonitor<P0>(pcallback: P0) -> windows_core::Result<IMFSensorActivityMonitor>
where
    P0: windows_core::Param<IMFSensorActivitiesReportCallback>,
{
    windows_core::link!("mfsensorgroup.dll" "system" fn MFCreateSensorActivityMonitor(pcallback : *mut core::ffi::c_void, ppactivitymonitor : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateSensorActivityMonitor(pcallback.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn MFCreateSensorGroup<P0>(sensorgroupsymboliclink: P0) -> windows_core::Result<IMFSensorGroup>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mfsensorgroup.dll" "system" fn MFCreateSensorGroup(sensorgroupsymboliclink : windows_core::PCWSTR, ppsensorgroup : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateSensorGroup(sensorgroupsymboliclink.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn MFCreateSensorProfile<P2>(profiletype: *const windows_core::GUID, profileindex: u32, constraints: P2) -> windows_core::Result<IMFSensorProfile>
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mfsensorgroup.dll" "system" fn MFCreateSensorProfile(profiletype : *const windows_core::GUID, profileindex : u32, constraints : windows_core::PCWSTR, ppprofile : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateSensorProfile(profiletype, profileindex, constraints.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn MFCreateSensorProfileCollection() -> windows_core::Result<IMFSensorProfileCollection> {
    windows_core::link!("mfsensorgroup.dll" "system" fn MFCreateSensorProfileCollection(ppsensorprofile : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateSensorProfileCollection(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateSensorStream<P1, P2>(streamid: u32, pattributes: P1, pmediatypecollection: P2) -> windows_core::Result<IMFSensorStream>
where
    P1: windows_core::Param<super::IMFAttributes>,
    P2: windows_core::Param<super::IMFCollection>,
{
    windows_core::link!("mfsensorgroup.dll" "system" fn MFCreateSensorStream(streamid : u32, pattributes : *mut core::ffi::c_void, pmediatypecollection : *mut core::ffi::c_void, ppstream : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateSensorStream(streamid, pattributes.param().abi(), pmediatypecollection.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn MFCreateSequencerSegmentOffset(dwid: MFSequencerElementId, hnsoffset: MFTIME) -> windows_core::Result<super::PROPVARIANT> {
    windows_core::link!("mf.dll" "system" fn MFCreateSequencerSegmentOffset(dwid : MFSequencerElementId, hnsoffset : MFTIME, pvarsegmentoffset : *mut super::PROPVARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateSequencerSegmentOffset(dwid, hnsoffset, &mut result__).map(|| core::mem::transmute(result__))
    }
}
#[inline]
pub unsafe fn MFCreateSequencerSource<P0>(preserved: P0) -> windows_core::Result<IMFSequencerSource>
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("mf.dll" "system" fn MFCreateSequencerSource(preserved : *mut core::ffi::c_void, ppsequencersource : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateSequencerSource(preserved.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn MFCreateSimpleTypeHandler() -> windows_core::Result<IMFMediaTypeHandler> {
    windows_core::link!("mf.dll" "system" fn MFCreateSimpleTypeHandler(pphandler : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateSimpleTypeHandler(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn MFCreateSourceResolver() -> windows_core::Result<IMFSourceResolver> {
    windows_core::link!("mfplat.dll" "system" fn MFCreateSourceResolver(ppisourceresolver : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateSourceResolver(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn MFCreateStandardQualityManager() -> windows_core::Result<IMFQualityManager> {
    windows_core::link!("mf.dll" "system" fn MFCreateStandardQualityManager(ppqualitymanager : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateStandardQualityManager(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateStreamDescriptor(dwstreamidentifier: u32, apmediatypes: &[Option<super::IMFMediaType>]) -> windows_core::Result<IMFStreamDescriptor> {
    windows_core::link!("mfplat.dll" "system" fn MFCreateStreamDescriptor(dwstreamidentifier : u32, cmediatypes : u32, apmediatypes : *const *mut core::ffi::c_void, ppdescriptor : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateStreamDescriptor(dwstreamidentifier, apmediatypes.len().try_into().unwrap(), core::mem::transmute(apmediatypes.as_ptr()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "mfobjects", feature = "objidlbase"))]
#[inline]
pub unsafe fn MFCreateStreamOnMFByteStream<P0>(pbytestream: P0) -> windows_core::Result<super::IStream>
where
    P0: windows_core::Param<super::IMFByteStream>,
{
    windows_core::link!("mfplat.dll" "system" fn MFCreateStreamOnMFByteStream(pbytestream : *mut core::ffi::c_void, ppstream : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateStreamOnMFByteStream(pbytestream.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateStreamOnMFByteStreamEx<P0, T>(pbytestream: P0) -> windows_core::Result<T>
where
    P0: windows_core::Param<super::IMFByteStream>,
    T: windows_core::Interface,
{
    windows_core::link!("mfplat.dll" "system" fn MFCreateStreamOnMFByteStreamEx(pbytestream : *mut core::ffi::c_void, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { MFCreateStreamOnMFByteStreamEx(pbytestream.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[inline]
pub unsafe fn MFCreateSystemTimeSource() -> windows_core::Result<IMFPresentationTimeSource> {
    windows_core::link!("mfplat.dll" "system" fn MFCreateSystemTimeSource(ppsystemtimesource : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateSystemTimeSource(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn MFCreateTopoLoader() -> windows_core::Result<IMFTopoLoader> {
    windows_core::link!("mf.dll" "system" fn MFCreateTopoLoader(ppobj : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateTopoLoader(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateTopology() -> windows_core::Result<IMFTopology> {
    windows_core::link!("mf.dll" "system" fn MFCreateTopology(pptopo : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateTopology(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateTopologyNode(nodetype: MF_TOPOLOGY_TYPE) -> windows_core::Result<IMFTopologyNode> {
    windows_core::link!("mf.dll" "system" fn MFCreateTopologyNode(nodetype : MF_TOPOLOGY_TYPE, ppnode : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateTopologyNode(nodetype, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn MFCreateTrackedSample() -> windows_core::Result<IMFTrackedSample> {
    windows_core::link!("mfplat.dll" "system" fn MFCreateTrackedSample(ppmfsample : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateTrackedSample(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn MFCreateTranscodeProfile() -> windows_core::Result<IMFTranscodeProfile> {
    windows_core::link!("mf.dll" "system" fn MFCreateTranscodeProfile(pptranscodeprofile : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateTranscodeProfile(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateTranscodeSinkActivate() -> windows_core::Result<super::IMFActivate> {
    windows_core::link!("mf.dll" "system" fn MFCreateTranscodeSinkActivate(ppactivate : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateTranscodeSinkActivate(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateTranscodeTopology<P0, P1, P2>(psrc: P0, pwszoutputfilepath: P1, pprofile: P2) -> windows_core::Result<IMFTopology>
where
    P0: windows_core::Param<IMFMediaSource>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<IMFTranscodeProfile>,
{
    windows_core::link!("mf.dll" "system" fn MFCreateTranscodeTopology(psrc : *mut core::ffi::c_void, pwszoutputfilepath : windows_core::PCWSTR, pprofile : *mut core::ffi::c_void, pptranscodetopo : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateTranscodeTopology(psrc.param().abi(), pwszoutputfilepath.param().abi(), pprofile.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateTranscodeTopologyFromByteStream<P0, P1, P2>(psrc: P0, poutputstream: P1, pprofile: P2) -> windows_core::Result<IMFTopology>
where
    P0: windows_core::Param<IMFMediaSource>,
    P1: windows_core::Param<super::IMFByteStream>,
    P2: windows_core::Param<IMFTranscodeProfile>,
{
    windows_core::link!("mf.dll" "system" fn MFCreateTranscodeTopologyFromByteStream(psrc : *mut core::ffi::c_void, poutputstream : *mut core::ffi::c_void, pprofile : *mut core::ffi::c_void, pptranscodetopo : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateTranscodeTopologyFromByteStream(psrc.param().abi(), poutputstream.param().abi(), pprofile.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(all(feature = "mfobjects", feature = "windef"))]
#[inline]
pub unsafe fn MFCreateVideoRendererActivate(hwndvideo: super::HWND) -> windows_core::Result<super::IMFActivate> {
    windows_core::link!("mf.dll" "system" fn MFCreateVideoRendererActivate(hwndvideo : super::HWND, ppactivate : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateVideoRendererActivate(hwndvideo, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFCreateWAVEMediaSink<P0, P1>(ptargetbytestream: P0, paudiomediatype: P1) -> windows_core::Result<IMFMediaSink>
where
    P0: windows_core::Param<super::IMFByteStream>,
    P1: windows_core::Param<super::IMFMediaType>,
{
    windows_core::link!("mfsrcsnk.dll" "system" fn MFCreateWAVEMediaSink(ptargetbytestream : *mut core::ffi::c_void, paudiomediatype : *mut core::ffi::c_void, ppmediasink : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFCreateWAVEMediaSink(ptargetbytestream.param().abi(), paudiomediatype.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFDeserializePresentationDescriptor(pbdata: &[u8]) -> windows_core::Result<IMFPresentationDescriptor> {
    windows_core::link!("mfplat.dll" "system" fn MFDeserializePresentationDescriptor(cbdata : u32, pbdata : *const u8, pppd : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFDeserializePresentationDescriptor(pbdata.len().try_into().unwrap(), pbdata.as_ptr(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFEnumDeviceSources<P0>(pattributes: P0, pppsourceactivate: *mut *mut Option<super::IMFActivate>, pcsourceactivate: *mut u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IMFAttributes>,
{
    windows_core::link!("mf.dll" "system" fn MFEnumDeviceSources(pattributes : *mut core::ffi::c_void, pppsourceactivate : *mut *mut *mut core::ffi::c_void, pcsourceactivate : *mut u32) -> windows_core::HRESULT);
    unsafe { MFEnumDeviceSources(pattributes.param().abi(), pppsourceactivate as _, pcsourceactivate as _) }
}
#[inline]
pub unsafe fn MFGetLocalId(verifier: &[u8]) -> windows_core::Result<windows_core::PWSTR> {
    windows_core::link!("mf.dll" "system" fn MFGetLocalId(verifier : *const u8, size : u32, id : *mut windows_core::PWSTR) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFGetLocalId(verifier.as_ptr(), verifier.len().try_into().unwrap(), &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn MFGetService<P0, T>(punkobject: P0, guidservice: *const windows_core::GUID) -> windows_core::Result<T>
where
    P0: windows_core::Param<windows_core::IUnknown>,
    T: windows_core::Interface,
{
    windows_core::link!("mf.dll" "system" fn MFGetService(punkobject : *mut core::ffi::c_void, guidservice : *const windows_core::GUID, riid : *const windows_core::GUID, ppvobject : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { MFGetService(punkobject.param().abi(), guidservice, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn MFGetSupportedMimeTypes() -> windows_core::Result<super::PROPVARIANT> {
    windows_core::link!("mfplat.dll" "system" fn MFGetSupportedMimeTypes(ppropvarmimetypearray : *mut super::PROPVARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFGetSupportedMimeTypes(&mut result__).map(|| core::mem::transmute(result__))
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn MFGetSupportedSchemes() -> windows_core::Result<super::PROPVARIANT> {
    windows_core::link!("mfplat.dll" "system" fn MFGetSupportedSchemes(ppropvarschemearray : *mut super::PROPVARIANT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFGetSupportedSchemes(&mut result__).map(|| core::mem::transmute(result__))
    }
}
#[inline]
pub unsafe fn MFGetSystemId() -> windows_core::Result<IMFSystemId> {
    windows_core::link!("mf.dll" "system" fn MFGetSystemId(ppid : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFGetSystemId(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn MFGetSystemTime() -> MFTIME {
    windows_core::link!("mfplat.dll" "system" fn MFGetSystemTime() -> MFTIME);
    unsafe { MFGetSystemTime() }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFGetTopoNodeCurrentType<P0>(pnode: P0, dwstreamindex: u32, foutput: bool) -> windows_core::Result<super::IMFMediaType>
where
    P0: windows_core::Param<IMFTopologyNode>,
{
    windows_core::link!("mf.dll" "system" fn MFGetTopoNodeCurrentType(pnode : *mut core::ffi::c_void, dwstreamindex : u32, foutput : windows_core::BOOL, pptype : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFGetTopoNodeCurrentType(pnode.param().abi(), dwstreamindex, foutput.into(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn MFIsContentProtectionDeviceSupported(protectionsystemid: *const windows_core::GUID) -> windows_core::Result<windows_core::BOOL> {
    windows_core::link!("mfplat.dll" "system" fn MFIsContentProtectionDeviceSupported(protectionsystemid : *const windows_core::GUID, issupported : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFIsContentProtectionDeviceSupported(protectionsystemid, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn MFLoadSignedLibrary<P0>(pszname: P0) -> windows_core::Result<IMFSignedLibrary>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mf.dll" "system" fn MFLoadSignedLibrary(pszname : windows_core::PCWSTR, pplib : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFLoadSignedLibrary(pszname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFRequireProtectedEnvironment<P0>(ppresentationdescriptor: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<IMFPresentationDescriptor>,
{
    windows_core::link!("mf.dll" "system" fn MFRequireProtectedEnvironment(ppresentationdescriptor : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { MFRequireProtectedEnvironment(ppresentationdescriptor.param().abi()) }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFSerializePresentationDescriptor<P0>(ppd: P0, pcbdata: *mut u32, ppbdata: *mut *mut u8) -> windows_core::HRESULT
where
    P0: windows_core::Param<IMFPresentationDescriptor>,
{
    windows_core::link!("mfplat.dll" "system" fn MFSerializePresentationDescriptor(ppd : *mut core::ffi::c_void, pcbdata : *mut u32, ppbdata : *mut *mut u8) -> windows_core::HRESULT);
    unsafe { MFSerializePresentationDescriptor(ppd.param().abi(), pcbdata as _, ppbdata as _) }
}
#[inline]
pub unsafe fn MFShutdownObject<P0>(punk: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("mf.dll" "system" fn MFShutdownObject(punk : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { MFShutdownObject(punk.param().abi()) }
}
#[cfg(feature = "mfobjects")]
#[inline]
pub unsafe fn MFTranscodeGetAudioOutputAvailableTypes<P2>(guidsubtype: *const windows_core::GUID, dwmftflags: u32, pcodecconfig: P2) -> windows_core::Result<super::IMFCollection>
where
    P2: windows_core::Param<super::IMFAttributes>,
{
    windows_core::link!("mf.dll" "system" fn MFTranscodeGetAudioOutputAvailableTypes(guidsubtype : *const windows_core::GUID, dwmftflags : u32, pcodecconfig : *mut core::ffi::c_void, ppavailabletypes : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MFTranscodeGetAudioOutputAvailableTypes(guidsubtype, dwmftflags, pcodecconfig.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
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
pub const CLSID_MPEG2ByteStreamPlugin: windows_core::GUID = windows_core::GUID::from_u128(0x40871c59_ab40_471f_8dc3_1f259d862479);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DetectedFaceBound {
    pub sizeInBytes: u32,
    pub normalizedXPosition: f32,
    pub normalizedYPosition: f32,
    pub normalizedWidth: f32,
    pub normalizedHeight: f32,
    pub confidenceValue: i32,
    pub flags: u64,
}
windows_core::imp::define_interface!(IMFAudioPolicy, IMFAudioPolicy_Vtbl, 0xa0638c2b_6465_4395_9ae7_a321a9fd2856);
windows_core::imp::interface_hierarchy!(IMFAudioPolicy, windows_core::IUnknown);
impl IMFAudioPolicy {
    pub unsafe fn SetGroupingParam(&self, rguidclass: *const windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetGroupingParam)(windows_core::Interface::as_raw(self), rguidclass) }
    }
    pub unsafe fn GetGroupingParam(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGroupingParam)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDisplayName<P0>(&self, pszname: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDisplayName)(windows_core::Interface::as_raw(self), pszname.param().abi()) }
    }
    pub unsafe fn GetDisplayName(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDisplayName)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetIconPath<P0>(&self, pszpath: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetIconPath)(windows_core::Interface::as_raw(self), pszpath.param().abi()) }
    }
    pub unsafe fn GetIconPath(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIconPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFAudioPolicy_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetGroupingParam: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID) -> windows_core::HRESULT,
    pub GetGroupingParam: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub SetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetDisplayName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub SetIconPath: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetIconPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
pub trait IMFAudioPolicy_Impl: windows_core::IUnknownImpl {
    fn SetGroupingParam(&self, rguidclass: *const windows_core::GUID) -> windows_core::Result<()>;
    fn GetGroupingParam(&self) -> windows_core::Result<windows_core::GUID>;
    fn SetDisplayName(&self, pszname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetDisplayName(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn SetIconPath(&self, pszpath: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetIconPath(&self) -> windows_core::Result<windows_core::PWSTR>;
}
impl IMFAudioPolicy_Vtbl {
    pub const fn new<Identity: IMFAudioPolicy_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetGroupingParam<Identity: IMFAudioPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rguidclass: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAudioPolicy_Impl::SetGroupingParam(this, core::mem::transmute_copy(&rguidclass)).into()
            }
        }
        unsafe extern "system" fn GetGroupingParam<Identity: IMFAudioPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidclass: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFAudioPolicy_Impl::GetGroupingParam(this) {
                    Ok(ok__) => {
                        pguidclass.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDisplayName<Identity: IMFAudioPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAudioPolicy_Impl::SetDisplayName(this, core::mem::transmute(&pszname)).into()
            }
        }
        unsafe extern "system" fn GetDisplayName<Identity: IMFAudioPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFAudioPolicy_Impl::GetDisplayName(this) {
                    Ok(ok__) => {
                        pszname.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetIconPath<Identity: IMFAudioPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAudioPolicy_Impl::SetIconPath(this, core::mem::transmute(&pszpath)).into()
            }
        }
        unsafe extern "system" fn GetIconPath<Identity: IMFAudioPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszpath: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFAudioPolicy_Impl::GetIconPath(this) {
                    Ok(ok__) => {
                        pszpath.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetGroupingParam: SetGroupingParam::<Identity, OFFSET>,
            GetGroupingParam: GetGroupingParam::<Identity, OFFSET>,
            SetDisplayName: SetDisplayName::<Identity, OFFSET>,
            GetDisplayName: GetDisplayName::<Identity, OFFSET>,
            SetIconPath: SetIconPath::<Identity, OFFSET>,
            GetIconPath: GetIconPath::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFAudioPolicy as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFAudioPolicy {}
windows_core::imp::define_interface!(IMFAudioStreamVolume, IMFAudioStreamVolume_Vtbl, 0x76b1bbdb_4ec8_4f36_b106_70a9316df593);
windows_core::imp::interface_hierarchy!(IMFAudioStreamVolume, windows_core::IUnknown);
impl IMFAudioStreamVolume {
    pub unsafe fn GetChannelCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetChannelCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetChannelVolume(&self, dwindex: u32, flevel: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetChannelVolume)(windows_core::Interface::as_raw(self), dwindex, flevel) }
    }
    pub unsafe fn GetChannelVolume(&self, dwindex: u32) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetChannelVolume)(windows_core::Interface::as_raw(self), dwindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAllVolumes(&self, dwcount: u32, pfvolumes: *const f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAllVolumes)(windows_core::Interface::as_raw(self), dwcount, pfvolumes) }
    }
    pub unsafe fn GetAllVolumes(&self, dwcount: u32) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAllVolumes)(windows_core::Interface::as_raw(self), dwcount, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFAudioStreamVolume_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetChannelCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetChannelVolume: unsafe extern "system" fn(*mut core::ffi::c_void, u32, f32) -> windows_core::HRESULT,
    pub GetChannelVolume: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32) -> windows_core::HRESULT,
    pub SetAllVolumes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const f32) -> windows_core::HRESULT,
    pub GetAllVolumes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut f32) -> windows_core::HRESULT,
}
pub trait IMFAudioStreamVolume_Impl: windows_core::IUnknownImpl {
    fn GetChannelCount(&self) -> windows_core::Result<u32>;
    fn SetChannelVolume(&self, dwindex: u32, flevel: f32) -> windows_core::Result<()>;
    fn GetChannelVolume(&self, dwindex: u32) -> windows_core::Result<f32>;
    fn SetAllVolumes(&self, dwcount: u32, pfvolumes: *const f32) -> windows_core::Result<()>;
    fn GetAllVolumes(&self, dwcount: u32) -> windows_core::Result<f32>;
}
impl IMFAudioStreamVolume_Vtbl {
    pub const fn new<Identity: IMFAudioStreamVolume_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetChannelCount<Identity: IMFAudioStreamVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFAudioStreamVolume_Impl::GetChannelCount(this) {
                    Ok(ok__) => {
                        pdwcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetChannelVolume<Identity: IMFAudioStreamVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, flevel: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAudioStreamVolume_Impl::SetChannelVolume(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&flevel)).into()
            }
        }
        unsafe extern "system" fn GetChannelVolume<Identity: IMFAudioStreamVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, pflevel: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFAudioStreamVolume_Impl::GetChannelVolume(this, core::mem::transmute_copy(&dwindex)) {
                    Ok(ok__) => {
                        pflevel.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAllVolumes<Identity: IMFAudioStreamVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcount: u32, pfvolumes: *const f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFAudioStreamVolume_Impl::SetAllVolumes(this, core::mem::transmute_copy(&dwcount), core::mem::transmute_copy(&pfvolumes)).into()
            }
        }
        unsafe extern "system" fn GetAllVolumes<Identity: IMFAudioStreamVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwcount: u32, pfvolumes: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFAudioStreamVolume_Impl::GetAllVolumes(this, core::mem::transmute_copy(&dwcount)) {
                    Ok(ok__) => {
                        pfvolumes.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetChannelCount: GetChannelCount::<Identity, OFFSET>,
            SetChannelVolume: SetChannelVolume::<Identity, OFFSET>,
            GetChannelVolume: GetChannelVolume::<Identity, OFFSET>,
            SetAllVolumes: SetAllVolumes::<Identity, OFFSET>,
            GetAllVolumes: GetAllVolumes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFAudioStreamVolume as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFAudioStreamVolume {}
windows_core::imp::define_interface!(IMFByteStreamBuffering, IMFByteStreamBuffering_Vtbl, 0x6d66d782_1d4f_4db7_8c63_cb8c77f1ef5e);
windows_core::imp::interface_hierarchy!(IMFByteStreamBuffering, windows_core::IUnknown);
impl IMFByteStreamBuffering {
    pub unsafe fn SetBufferingParams(&self, pparams: *const MFBYTESTREAM_BUFFERING_PARAMS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBufferingParams)(windows_core::Interface::as_raw(self), pparams) }
    }
    pub unsafe fn EnableBuffering(&self, fenable: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnableBuffering)(windows_core::Interface::as_raw(self), fenable.into()) }
    }
    pub unsafe fn StopBuffering(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).StopBuffering)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFByteStreamBuffering_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetBufferingParams: unsafe extern "system" fn(*mut core::ffi::c_void, *const MFBYTESTREAM_BUFFERING_PARAMS) -> windows_core::HRESULT,
    pub EnableBuffering: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub StopBuffering: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFByteStreamBuffering_Impl: windows_core::IUnknownImpl {
    fn SetBufferingParams(&self, pparams: *const MFBYTESTREAM_BUFFERING_PARAMS) -> windows_core::Result<()>;
    fn EnableBuffering(&self, fenable: windows_core::BOOL) -> windows_core::Result<()>;
    fn StopBuffering(&self) -> windows_core::Result<()>;
}
impl IMFByteStreamBuffering_Vtbl {
    pub const fn new<Identity: IMFByteStreamBuffering_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetBufferingParams<Identity: IMFByteStreamBuffering_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparams: *const MFBYTESTREAM_BUFFERING_PARAMS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFByteStreamBuffering_Impl::SetBufferingParams(this, core::mem::transmute_copy(&pparams)).into()
            }
        }
        unsafe extern "system" fn EnableBuffering<Identity: IMFByteStreamBuffering_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenable: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFByteStreamBuffering_Impl::EnableBuffering(this, core::mem::transmute_copy(&fenable)).into()
            }
        }
        unsafe extern "system" fn StopBuffering<Identity: IMFByteStreamBuffering_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFByteStreamBuffering_Impl::StopBuffering(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetBufferingParams: SetBufferingParams::<Identity, OFFSET>,
            EnableBuffering: EnableBuffering::<Identity, OFFSET>,
            StopBuffering: StopBuffering::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFByteStreamBuffering as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFByteStreamBuffering {}
windows_core::imp::define_interface!(IMFByteStreamCacheControl, IMFByteStreamCacheControl_Vtbl, 0xf5042ea4_7a96_4a75_aa7b_2be1ef7f88d5);
windows_core::imp::interface_hierarchy!(IMFByteStreamCacheControl, windows_core::IUnknown);
impl IMFByteStreamCacheControl {
    pub unsafe fn StopBackgroundTransfer(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).StopBackgroundTransfer)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFByteStreamCacheControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub StopBackgroundTransfer: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFByteStreamCacheControl_Impl: windows_core::IUnknownImpl {
    fn StopBackgroundTransfer(&self) -> windows_core::Result<()>;
}
impl IMFByteStreamCacheControl_Vtbl {
    pub const fn new<Identity: IMFByteStreamCacheControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StopBackgroundTransfer<Identity: IMFByteStreamCacheControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFByteStreamCacheControl_Impl::StopBackgroundTransfer(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), StopBackgroundTransfer: StopBackgroundTransfer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFByteStreamCacheControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFByteStreamCacheControl {}
windows_core::imp::define_interface!(IMFByteStreamCacheControl2, IMFByteStreamCacheControl2_Vtbl, 0x71ce469c_f34b_49ea_a56b_2d2a10e51149);
impl core::ops::Deref for IMFByteStreamCacheControl2 {
    type Target = IMFByteStreamCacheControl;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMFByteStreamCacheControl2, windows_core::IUnknown, IMFByteStreamCacheControl);
impl IMFByteStreamCacheControl2 {
    pub unsafe fn GetByteRanges(&self, pcranges: *mut u32, ppranges: *mut *mut MF_BYTE_STREAM_CACHE_RANGE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetByteRanges)(windows_core::Interface::as_raw(self), pcranges as _, ppranges as _) }
    }
    pub unsafe fn SetCacheLimit(&self, qwbytes: u64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCacheLimit)(windows_core::Interface::as_raw(self), qwbytes) }
    }
    pub unsafe fn IsBackgroundTransferActive(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsBackgroundTransferActive)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFByteStreamCacheControl2_Vtbl {
    pub base__: IMFByteStreamCacheControl_Vtbl,
    pub GetByteRanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut MF_BYTE_STREAM_CACHE_RANGE) -> windows_core::HRESULT,
    pub SetCacheLimit: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub IsBackgroundTransferActive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IMFByteStreamCacheControl2_Impl: IMFByteStreamCacheControl_Impl {
    fn GetByteRanges(&self, pcranges: *mut u32, ppranges: *mut *mut MF_BYTE_STREAM_CACHE_RANGE) -> windows_core::Result<()>;
    fn SetCacheLimit(&self, qwbytes: u64) -> windows_core::Result<()>;
    fn IsBackgroundTransferActive(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IMFByteStreamCacheControl2_Vtbl {
    pub const fn new<Identity: IMFByteStreamCacheControl2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetByteRanges<Identity: IMFByteStreamCacheControl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcranges: *mut u32, ppranges: *mut *mut MF_BYTE_STREAM_CACHE_RANGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFByteStreamCacheControl2_Impl::GetByteRanges(this, core::mem::transmute_copy(&pcranges), core::mem::transmute_copy(&ppranges)).into()
            }
        }
        unsafe extern "system" fn SetCacheLimit<Identity: IMFByteStreamCacheControl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, qwbytes: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFByteStreamCacheControl2_Impl::SetCacheLimit(this, core::mem::transmute_copy(&qwbytes)).into()
            }
        }
        unsafe extern "system" fn IsBackgroundTransferActive<Identity: IMFByteStreamCacheControl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfactive: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFByteStreamCacheControl2_Impl::IsBackgroundTransferActive(this) {
                    Ok(ok__) => {
                        pfactive.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IMFByteStreamCacheControl_Vtbl::new::<Identity, OFFSET>(),
            GetByteRanges: GetByteRanges::<Identity, OFFSET>,
            SetCacheLimit: SetCacheLimit::<Identity, OFFSET>,
            IsBackgroundTransferActive: IsBackgroundTransferActive::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFByteStreamCacheControl2 as windows_core::Interface>::IID || iid == &<IMFByteStreamCacheControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFByteStreamCacheControl2 {}
windows_core::imp::define_interface!(IMFByteStreamHandler, IMFByteStreamHandler_Vtbl, 0xbb420aa4_765b_4a1f_91fe_d6a8a143924c);
windows_core::imp::interface_hierarchy!(IMFByteStreamHandler, windows_core::IUnknown);
impl IMFByteStreamHandler {
    #[cfg(all(feature = "mfobjects", feature = "propsys"))]
    pub unsafe fn BeginCreateObject<P0, P1, P3, P5, P6>(&self, pbytestream: P0, pwszurl: P1, dwflags: u32, pprops: P3, ppiunknowncancelcookie: *mut Option<windows_core::IUnknown>, pcallback: P5, punkstate: P6) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMFByteStream>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<super::IPropertyStore>,
        P5: windows_core::Param<super::IMFAsyncCallback>,
        P6: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).BeginCreateObject)(windows_core::Interface::as_raw(self), pbytestream.param().abi(), pwszurl.param().abi(), dwflags, pprops.param().abi(), core::mem::transmute(ppiunknowncancelcookie), pcallback.param().abi(), punkstate.param().abi()) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn EndCreateObject<P0>(&self, presult: P0, pobjecttype: *mut MF_OBJECT_TYPE, ppobject: *mut Option<windows_core::IUnknown>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMFAsyncResult>,
    {
        unsafe { (windows_core::Interface::vtable(self).EndCreateObject)(windows_core::Interface::as_raw(self), presult.param().abi(), pobjecttype as _, core::mem::transmute(ppobject)) }
    }
    pub unsafe fn CancelObjectCreation<P0>(&self, piunknowncancelcookie: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CancelObjectCreation)(windows_core::Interface::as_raw(self), piunknowncancelcookie.param().abi()) }
    }
    pub unsafe fn GetMaxNumberOfBytesRequiredForResolution(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaxNumberOfBytesRequiredForResolution)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFByteStreamHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "mfobjects", feature = "propsys"))]
    pub BeginCreateObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "propsys")))]
    BeginCreateObject: usize,
    #[cfg(feature = "mfobjects")]
    pub EndCreateObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut MF_OBJECT_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    EndCreateObject: usize,
    pub CancelObjectCreation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetMaxNumberOfBytesRequiredForResolution: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
}
#[cfg(all(feature = "mfobjects", feature = "propsys"))]
pub trait IMFByteStreamHandler_Impl: windows_core::IUnknownImpl {
    fn BeginCreateObject(&self, pbytestream: windows_core::Ref<super::IMFByteStream>, pwszurl: &windows_core::PCWSTR, dwflags: u32, pprops: windows_core::Ref<super::IPropertyStore>, ppiunknowncancelcookie: windows_core::OutRef<windows_core::IUnknown>, pcallback: windows_core::Ref<super::IMFAsyncCallback>, punkstate: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn EndCreateObject(&self, presult: windows_core::Ref<super::IMFAsyncResult>, pobjecttype: *mut MF_OBJECT_TYPE, ppobject: windows_core::OutRef<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn CancelObjectCreation(&self, piunknowncancelcookie: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetMaxNumberOfBytesRequiredForResolution(&self) -> windows_core::Result<u64>;
}
#[cfg(all(feature = "mfobjects", feature = "propsys"))]
impl IMFByteStreamHandler_Vtbl {
    pub const fn new<Identity: IMFByteStreamHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BeginCreateObject<Identity: IMFByteStreamHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbytestream: *mut core::ffi::c_void, pwszurl: windows_core::PCWSTR, dwflags: u32, pprops: *mut core::ffi::c_void, ppiunknowncancelcookie: *mut *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void, punkstate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFByteStreamHandler_Impl::BeginCreateObject(this, core::mem::transmute_copy(&pbytestream), core::mem::transmute(&pwszurl), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pprops), core::mem::transmute_copy(&ppiunknowncancelcookie), core::mem::transmute_copy(&pcallback), core::mem::transmute_copy(&punkstate)).into()
            }
        }
        unsafe extern "system" fn EndCreateObject<Identity: IMFByteStreamHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presult: *mut core::ffi::c_void, pobjecttype: *mut MF_OBJECT_TYPE, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFByteStreamHandler_Impl::EndCreateObject(this, core::mem::transmute_copy(&presult), core::mem::transmute_copy(&pobjecttype), core::mem::transmute_copy(&ppobject)).into()
            }
        }
        unsafe extern "system" fn CancelObjectCreation<Identity: IMFByteStreamHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piunknowncancelcookie: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFByteStreamHandler_Impl::CancelObjectCreation(this, core::mem::transmute_copy(&piunknowncancelcookie)).into()
            }
        }
        unsafe extern "system" fn GetMaxNumberOfBytesRequiredForResolution<Identity: IMFByteStreamHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqwbytes: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFByteStreamHandler_Impl::GetMaxNumberOfBytesRequiredForResolution(this) {
                    Ok(ok__) => {
                        pqwbytes.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginCreateObject: BeginCreateObject::<Identity, OFFSET>,
            EndCreateObject: EndCreateObject::<Identity, OFFSET>,
            CancelObjectCreation: CancelObjectCreation::<Identity, OFFSET>,
            GetMaxNumberOfBytesRequiredForResolution: GetMaxNumberOfBytesRequiredForResolution::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFByteStreamHandler as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mfobjects", feature = "propsys"))]
impl windows_core::RuntimeName for IMFByteStreamHandler {}
windows_core::imp::define_interface!(IMFByteStreamTimeSeek, IMFByteStreamTimeSeek_Vtbl, 0x64976bfa_fb61_4041_9069_8c9a5f659beb);
windows_core::imp::interface_hierarchy!(IMFByteStreamTimeSeek, windows_core::IUnknown);
impl IMFByteStreamTimeSeek {
    pub unsafe fn IsTimeSeekSupported(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsTimeSeekSupported)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TimeSeek(&self, qwtimeposition: u64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).TimeSeek)(windows_core::Interface::as_raw(self), qwtimeposition) }
    }
    pub unsafe fn GetTimeSeekResult(&self, pqwstarttime: *mut u64, pqwstoptime: *mut u64, pqwduration: *mut u64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTimeSeekResult)(windows_core::Interface::as_raw(self), pqwstarttime as _, pqwstoptime as _, pqwduration as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFByteStreamTimeSeek_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsTimeSeekSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub TimeSeek: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub GetTimeSeekResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64, *mut u64, *mut u64) -> windows_core::HRESULT,
}
pub trait IMFByteStreamTimeSeek_Impl: windows_core::IUnknownImpl {
    fn IsTimeSeekSupported(&self) -> windows_core::Result<windows_core::BOOL>;
    fn TimeSeek(&self, qwtimeposition: u64) -> windows_core::Result<()>;
    fn GetTimeSeekResult(&self, pqwstarttime: *mut u64, pqwstoptime: *mut u64, pqwduration: *mut u64) -> windows_core::Result<()>;
}
impl IMFByteStreamTimeSeek_Vtbl {
    pub const fn new<Identity: IMFByteStreamTimeSeek_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsTimeSeekSupported<Identity: IMFByteStreamTimeSeek_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pftimeseekissupported: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFByteStreamTimeSeek_Impl::IsTimeSeekSupported(this) {
                    Ok(ok__) => {
                        pftimeseekissupported.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TimeSeek<Identity: IMFByteStreamTimeSeek_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, qwtimeposition: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFByteStreamTimeSeek_Impl::TimeSeek(this, core::mem::transmute_copy(&qwtimeposition)).into()
            }
        }
        unsafe extern "system" fn GetTimeSeekResult<Identity: IMFByteStreamTimeSeek_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqwstarttime: *mut u64, pqwstoptime: *mut u64, pqwduration: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFByteStreamTimeSeek_Impl::GetTimeSeekResult(this, core::mem::transmute_copy(&pqwstarttime), core::mem::transmute_copy(&pqwstoptime), core::mem::transmute_copy(&pqwduration)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsTimeSeekSupported: IsTimeSeekSupported::<Identity, OFFSET>,
            TimeSeek: TimeSeek::<Identity, OFFSET>,
            GetTimeSeekResult: GetTimeSeekResult::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFByteStreamTimeSeek as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFByteStreamTimeSeek {}
windows_core::imp::define_interface!(IMFCameraConfigurationManager, IMFCameraConfigurationManager_Vtbl, 0xa624f617_4704_4206_8a6d_ebda4a093985);
windows_core::imp::interface_hierarchy!(IMFCameraConfigurationManager, windows_core::IUnknown);
impl IMFCameraConfigurationManager {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn LoadDefaults<P0>(&self, cameraattributes: P0) -> windows_core::Result<IMFCameraControlDefaultsCollection>
    where
        P0: windows_core::Param<super::IMFAttributes>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LoadDefaults)(windows_core::Interface::as_raw(self), cameraattributes.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn SaveDefaults<P0>(&self, configurations: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFCameraControlDefaultsCollection>,
    {
        unsafe { (windows_core::Interface::vtable(self).SaveDefaults)(windows_core::Interface::as_raw(self), configurations.param().abi()) }
    }
    pub unsafe fn Shutdown(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).Shutdown)(windows_core::Interface::as_raw(self));
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFCameraConfigurationManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "mfobjects")]
    pub LoadDefaults: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    LoadDefaults: usize,
    #[cfg(feature = "mfobjects")]
    pub SaveDefaults: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    SaveDefaults: usize,
    pub Shutdown: unsafe extern "system" fn(*mut core::ffi::c_void),
}
#[cfg(feature = "mfobjects")]
pub trait IMFCameraConfigurationManager_Impl: windows_core::IUnknownImpl {
    fn LoadDefaults(&self, cameraattributes: windows_core::Ref<super::IMFAttributes>) -> windows_core::Result<IMFCameraControlDefaultsCollection>;
    fn SaveDefaults(&self, configurations: windows_core::Ref<IMFCameraControlDefaultsCollection>) -> windows_core::Result<()>;
    fn Shutdown(&self);
}
#[cfg(feature = "mfobjects")]
impl IMFCameraConfigurationManager_Vtbl {
    pub const fn new<Identity: IMFCameraConfigurationManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LoadDefaults<Identity: IMFCameraConfigurationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cameraattributes: *mut core::ffi::c_void, configurations: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFCameraConfigurationManager_Impl::LoadDefaults(this, core::mem::transmute_copy(&cameraattributes)) {
                    Ok(ok__) => {
                        configurations.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SaveDefaults<Identity: IMFCameraConfigurationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, configurations: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCameraConfigurationManager_Impl::SaveDefaults(this, core::mem::transmute_copy(&configurations)).into()
            }
        }
        unsafe extern "system" fn Shutdown<Identity: IMFCameraConfigurationManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCameraConfigurationManager_Impl::Shutdown(this);
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            LoadDefaults: LoadDefaults::<Identity, OFFSET>,
            SaveDefaults: SaveDefaults::<Identity, OFFSET>,
            Shutdown: Shutdown::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFCameraConfigurationManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFCameraConfigurationManager {}
windows_core::imp::define_interface!(IMFCameraControlDefaults, IMFCameraControlDefaults_Vtbl, 0x75510662_b034_48f4_88a7_8de61daa4af9);
windows_core::imp::interface_hierarchy!(IMFCameraControlDefaults, windows_core::IUnknown);
impl IMFCameraControlDefaults {
    pub unsafe fn GetType(&self) -> MF_CAMERA_CONTROL_CONFIGURATION_TYPE {
        unsafe { (windows_core::Interface::vtable(self).GetType)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetRangeInfo(&self) -> windows_core::Result<MF_CAMERA_CONTROL_RANGE_INFO> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRangeInfo)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn LockControlData(&self, control: *mut *mut core::ffi::c_void, controlsize: *mut u32, data: *mut *mut core::ffi::c_void, datasize: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LockControlData)(windows_core::Interface::as_raw(self), control as _, controlsize as _, data as _, datasize.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn UnlockControlData(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnlockControlData)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFCameraControlDefaults_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetType: unsafe extern "system" fn(*mut core::ffi::c_void) -> MF_CAMERA_CONTROL_CONFIGURATION_TYPE,
    pub GetRangeInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MF_CAMERA_CONTROL_RANGE_INFO) -> windows_core::HRESULT,
    pub LockControlData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub UnlockControlData: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFCameraControlDefaults_Impl: windows_core::IUnknownImpl {
    fn GetType(&self) -> MF_CAMERA_CONTROL_CONFIGURATION_TYPE;
    fn GetRangeInfo(&self) -> windows_core::Result<MF_CAMERA_CONTROL_RANGE_INFO>;
    fn LockControlData(&self, control: *mut *mut core::ffi::c_void, controlsize: *mut u32, data: *mut *mut core::ffi::c_void, datasize: *mut u32) -> windows_core::Result<()>;
    fn UnlockControlData(&self) -> windows_core::Result<()>;
}
impl IMFCameraControlDefaults_Vtbl {
    pub const fn new<Identity: IMFCameraControlDefaults_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetType<Identity: IMFCameraControlDefaults_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> MF_CAMERA_CONTROL_CONFIGURATION_TYPE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCameraControlDefaults_Impl::GetType(this)
            }
        }
        unsafe extern "system" fn GetRangeInfo<Identity: IMFCameraControlDefaults_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rangeinfo: *mut MF_CAMERA_CONTROL_RANGE_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFCameraControlDefaults_Impl::GetRangeInfo(this) {
                    Ok(ok__) => {
                        rangeinfo.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LockControlData<Identity: IMFCameraControlDefaults_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, control: *mut *mut core::ffi::c_void, controlsize: *mut u32, data: *mut *mut core::ffi::c_void, datasize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCameraControlDefaults_Impl::LockControlData(this, core::mem::transmute_copy(&control), core::mem::transmute_copy(&controlsize), core::mem::transmute_copy(&data), core::mem::transmute_copy(&datasize)).into()
            }
        }
        unsafe extern "system" fn UnlockControlData<Identity: IMFCameraControlDefaults_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCameraControlDefaults_Impl::UnlockControlData(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetType: GetType::<Identity, OFFSET>,
            GetRangeInfo: GetRangeInfo::<Identity, OFFSET>,
            LockControlData: LockControlData::<Identity, OFFSET>,
            UnlockControlData: UnlockControlData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFCameraControlDefaults as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFCameraControlDefaults {}
#[cfg(feature = "mfobjects")]
windows_core::imp::define_interface!(IMFCameraControlDefaultsCollection, IMFCameraControlDefaultsCollection_Vtbl, 0x92d43d0f_54a8_4bae_96da_356d259a5c26);
#[cfg(feature = "mfobjects")]
impl core::ops::Deref for IMFCameraControlDefaultsCollection {
    type Target = super::IMFAttributes;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "mfobjects")]
windows_core::imp::interface_hierarchy!(IMFCameraControlDefaultsCollection, windows_core::IUnknown, super::IMFAttributes);
#[cfg(feature = "mfobjects")]
impl IMFCameraControlDefaultsCollection {
    pub unsafe fn GetControlCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetControlCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetControl(&self, index: u32) -> windows_core::Result<IMFCameraControlDefaults> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetControl)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetOrAddExtendedControl(&self, configtype: MF_CAMERA_CONTROL_CONFIGURATION_TYPE, constrolid: u32, streamid: u32, datasize: u32) -> windows_core::Result<IMFCameraControlDefaults> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOrAddExtendedControl)(windows_core::Interface::as_raw(self), configtype, constrolid, streamid, datasize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetOrAddControl(&self, configtype: MF_CAMERA_CONTROL_CONFIGURATION_TYPE, controlset: *const windows_core::GUID, constrolid: u32, controlsize: u32, datasize: u32) -> windows_core::Result<IMFCameraControlDefaults> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOrAddControl)(windows_core::Interface::as_raw(self), configtype, controlset, constrolid, controlsize, datasize, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RemoveControl(&self, controlset: *const windows_core::GUID, constrolid: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveControl)(windows_core::Interface::as_raw(self), controlset, constrolid) }
    }
    pub unsafe fn RemoveAllControls(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveAllControls)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "mfobjects")]
#[repr(C)]
#[doc(hidden)]
pub struct IMFCameraControlDefaultsCollection_Vtbl {
    pub base__: super::IMFAttributes_Vtbl,
    pub GetControlCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetControl: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetOrAddExtendedControl: unsafe extern "system" fn(*mut core::ffi::c_void, MF_CAMERA_CONTROL_CONFIGURATION_TYPE, u32, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetOrAddControl: unsafe extern "system" fn(*mut core::ffi::c_void, MF_CAMERA_CONTROL_CONFIGURATION_TYPE, *const windows_core::GUID, u32, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveControl: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32) -> windows_core::HRESULT,
    pub RemoveAllControls: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMFCameraControlDefaultsCollection_Impl: super::IMFAttributes_Impl {
    fn GetControlCount(&self) -> u32;
    fn GetControl(&self, index: u32) -> windows_core::Result<IMFCameraControlDefaults>;
    fn GetOrAddExtendedControl(&self, configtype: MF_CAMERA_CONTROL_CONFIGURATION_TYPE, constrolid: u32, streamid: u32, datasize: u32) -> windows_core::Result<IMFCameraControlDefaults>;
    fn GetOrAddControl(&self, configtype: MF_CAMERA_CONTROL_CONFIGURATION_TYPE, controlset: *const windows_core::GUID, constrolid: u32, controlsize: u32, datasize: u32) -> windows_core::Result<IMFCameraControlDefaults>;
    fn RemoveControl(&self, controlset: *const windows_core::GUID, constrolid: u32) -> windows_core::Result<()>;
    fn RemoveAllControls(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl IMFCameraControlDefaultsCollection_Vtbl {
    pub const fn new<Identity: IMFCameraControlDefaultsCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetControlCount<Identity: IMFCameraControlDefaultsCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCameraControlDefaultsCollection_Impl::GetControlCount(this)
            }
        }
        unsafe extern "system" fn GetControl<Identity: IMFCameraControlDefaultsCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, configuration: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFCameraControlDefaultsCollection_Impl::GetControl(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        configuration.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOrAddExtendedControl<Identity: IMFCameraControlDefaultsCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, configtype: MF_CAMERA_CONTROL_CONFIGURATION_TYPE, constrolid: u32, streamid: u32, datasize: u32, defaults: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFCameraControlDefaultsCollection_Impl::GetOrAddExtendedControl(this, core::mem::transmute_copy(&configtype), core::mem::transmute_copy(&constrolid), core::mem::transmute_copy(&streamid), core::mem::transmute_copy(&datasize)) {
                    Ok(ok__) => {
                        defaults.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOrAddControl<Identity: IMFCameraControlDefaultsCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, configtype: MF_CAMERA_CONTROL_CONFIGURATION_TYPE, controlset: *const windows_core::GUID, constrolid: u32, controlsize: u32, datasize: u32, defaults: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFCameraControlDefaultsCollection_Impl::GetOrAddControl(this, core::mem::transmute_copy(&configtype), core::mem::transmute_copy(&controlset), core::mem::transmute_copy(&constrolid), core::mem::transmute_copy(&controlsize), core::mem::transmute_copy(&datasize)) {
                    Ok(ok__) => {
                        defaults.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveControl<Identity: IMFCameraControlDefaultsCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, controlset: *const windows_core::GUID, constrolid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCameraControlDefaultsCollection_Impl::RemoveControl(this, core::mem::transmute_copy(&controlset), core::mem::transmute_copy(&constrolid)).into()
            }
        }
        unsafe extern "system" fn RemoveAllControls<Identity: IMFCameraControlDefaultsCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCameraControlDefaultsCollection_Impl::RemoveAllControls(this).into()
            }
        }
        Self {
            base__: super::IMFAttributes_Vtbl::new::<Identity, OFFSET>(),
            GetControlCount: GetControlCount::<Identity, OFFSET>,
            GetControl: GetControl::<Identity, OFFSET>,
            GetOrAddExtendedControl: GetOrAddExtendedControl::<Identity, OFFSET>,
            GetOrAddControl: GetOrAddControl::<Identity, OFFSET>,
            RemoveControl: RemoveControl::<Identity, OFFSET>,
            RemoveAllControls: RemoveAllControls::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFCameraControlDefaultsCollection as windows_core::Interface>::IID || iid == &<super::IMFAttributes as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMFCameraControlDefaultsCollection {}
windows_core::imp::define_interface!(IMFCameraControlMonitor, IMFCameraControlMonitor_Vtbl, 0x4d46f2c9_28ba_4970_8c7b_1f0c9d80af69);
windows_core::imp::interface_hierarchy!(IMFCameraControlMonitor, windows_core::IUnknown);
impl IMFCameraControlMonitor {
    pub unsafe fn Start(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Stop(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AddControlSubscription(&self, controlset: windows_core::GUID, id: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddControlSubscription)(windows_core::Interface::as_raw(self), controlset, id) }
    }
    pub unsafe fn RemoveControlSubscription(&self, controlset: windows_core::GUID, id: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveControlSubscription)(windows_core::Interface::as_raw(self), controlset, id) }
    }
    pub unsafe fn Shutdown(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).Shutdown)(windows_core::Interface::as_raw(self));
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFCameraControlMonitor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddControlSubscription: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, u32) -> windows_core::HRESULT,
    pub RemoveControlSubscription: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, u32) -> windows_core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(*mut core::ffi::c_void),
}
pub trait IMFCameraControlMonitor_Impl: windows_core::IUnknownImpl {
    fn Start(&self) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn AddControlSubscription(&self, controlset: &windows_core::GUID, id: u32) -> windows_core::Result<()>;
    fn RemoveControlSubscription(&self, controlset: &windows_core::GUID, id: u32) -> windows_core::Result<()>;
    fn Shutdown(&self);
}
impl IMFCameraControlMonitor_Vtbl {
    pub const fn new<Identity: IMFCameraControlMonitor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Start<Identity: IMFCameraControlMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCameraControlMonitor_Impl::Start(this).into()
            }
        }
        unsafe extern "system" fn Stop<Identity: IMFCameraControlMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCameraControlMonitor_Impl::Stop(this).into()
            }
        }
        unsafe extern "system" fn AddControlSubscription<Identity: IMFCameraControlMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, controlset: windows_core::GUID, id: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCameraControlMonitor_Impl::AddControlSubscription(this, core::mem::transmute(&controlset), core::mem::transmute_copy(&id)).into()
            }
        }
        unsafe extern "system" fn RemoveControlSubscription<Identity: IMFCameraControlMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, controlset: windows_core::GUID, id: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCameraControlMonitor_Impl::RemoveControlSubscription(this, core::mem::transmute(&controlset), core::mem::transmute_copy(&id)).into()
            }
        }
        unsafe extern "system" fn Shutdown<Identity: IMFCameraControlMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCameraControlMonitor_Impl::Shutdown(this);
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Start: Start::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            AddControlSubscription: AddControlSubscription::<Identity, OFFSET>,
            RemoveControlSubscription: RemoveControlSubscription::<Identity, OFFSET>,
            Shutdown: Shutdown::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFCameraControlMonitor as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFCameraControlMonitor {}
windows_core::imp::define_interface!(IMFCameraControlNotify, IMFCameraControlNotify_Vtbl, 0xe8f2540d_558a_4449_8b64_4863467a9fe8);
windows_core::imp::interface_hierarchy!(IMFCameraControlNotify, windows_core::IUnknown);
impl IMFCameraControlNotify {
    pub unsafe fn OnChange(&self, controlset: *const windows_core::GUID, id: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).OnChange)(windows_core::Interface::as_raw(self), controlset, id);
        }
    }
    pub unsafe fn OnError(&self, hrstatus: windows_core::HRESULT) {
        unsafe {
            (windows_core::Interface::vtable(self).OnError)(windows_core::Interface::as_raw(self), hrstatus);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFCameraControlNotify_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnChange: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32),
    pub OnError: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT),
}
pub trait IMFCameraControlNotify_Impl: windows_core::IUnknownImpl {
    fn OnChange(&self, controlset: *const windows_core::GUID, id: u32);
    fn OnError(&self, hrstatus: windows_core::HRESULT);
}
impl IMFCameraControlNotify_Vtbl {
    pub const fn new<Identity: IMFCameraControlNotify_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnChange<Identity: IMFCameraControlNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, controlset: *const windows_core::GUID, id: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCameraControlNotify_Impl::OnChange(this, core::mem::transmute_copy(&controlset), core::mem::transmute_copy(&id));
            }
        }
        unsafe extern "system" fn OnError<Identity: IMFCameraControlNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrstatus: windows_core::HRESULT) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCameraControlNotify_Impl::OnError(this, core::mem::transmute_copy(&hrstatus));
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnChange: OnChange::<Identity, OFFSET>, OnError: OnError::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFCameraControlNotify as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFCameraControlNotify {}
windows_core::imp::define_interface!(IMFCameraOcclusionStateMonitor, IMFCameraOcclusionStateMonitor_Vtbl, 0xcc692f46_c697_47e2_a72d_7b064617749b);
windows_core::imp::interface_hierarchy!(IMFCameraOcclusionStateMonitor, windows_core::IUnknown);
impl IMFCameraOcclusionStateMonitor {
    pub unsafe fn Start(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Stop(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetSupportedStates(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetSupportedStates)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFCameraOcclusionStateMonitor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSupportedStates: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
}
pub trait IMFCameraOcclusionStateMonitor_Impl: windows_core::IUnknownImpl {
    fn Start(&self) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn GetSupportedStates(&self) -> u32;
}
impl IMFCameraOcclusionStateMonitor_Vtbl {
    pub const fn new<Identity: IMFCameraOcclusionStateMonitor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Start<Identity: IMFCameraOcclusionStateMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCameraOcclusionStateMonitor_Impl::Start(this).into()
            }
        }
        unsafe extern "system" fn Stop<Identity: IMFCameraOcclusionStateMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCameraOcclusionStateMonitor_Impl::Stop(this).into()
            }
        }
        unsafe extern "system" fn GetSupportedStates<Identity: IMFCameraOcclusionStateMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCameraOcclusionStateMonitor_Impl::GetSupportedStates(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Start: Start::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            GetSupportedStates: GetSupportedStates::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFCameraOcclusionStateMonitor as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFCameraOcclusionStateMonitor {}
windows_core::imp::define_interface!(IMFCameraOcclusionStateReport, IMFCameraOcclusionStateReport_Vtbl, 0x1640b2cf_74da_4462_a43b_b76d3bdc1434);
windows_core::imp::interface_hierarchy!(IMFCameraOcclusionStateReport, windows_core::IUnknown);
impl IMFCameraOcclusionStateReport {
    pub unsafe fn GetOcclusionState(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOcclusionState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFCameraOcclusionStateReport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetOcclusionState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IMFCameraOcclusionStateReport_Impl: windows_core::IUnknownImpl {
    fn GetOcclusionState(&self) -> windows_core::Result<u32>;
}
impl IMFCameraOcclusionStateReport_Vtbl {
    pub const fn new<Identity: IMFCameraOcclusionStateReport_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetOcclusionState<Identity: IMFCameraOcclusionStateReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, occlusionstate: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFCameraOcclusionStateReport_Impl::GetOcclusionState(this) {
                    Ok(ok__) => {
                        occlusionstate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetOcclusionState: GetOcclusionState::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFCameraOcclusionStateReport as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFCameraOcclusionStateReport {}
windows_core::imp::define_interface!(IMFCameraOcclusionStateReportCallback, IMFCameraOcclusionStateReportCallback_Vtbl, 0x6e5841c7_3889_4019_9035_783fb19b5948);
windows_core::imp::interface_hierarchy!(IMFCameraOcclusionStateReportCallback, windows_core::IUnknown);
impl IMFCameraOcclusionStateReportCallback {
    pub unsafe fn OnOcclusionStateReport<P0>(&self, occlusionstatereport: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFCameraOcclusionStateReport>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnOcclusionStateReport)(windows_core::Interface::as_raw(self), occlusionstatereport.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFCameraOcclusionStateReportCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnOcclusionStateReport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFCameraOcclusionStateReportCallback_Impl: windows_core::IUnknownImpl {
    fn OnOcclusionStateReport(&self, occlusionstatereport: windows_core::Ref<IMFCameraOcclusionStateReport>) -> windows_core::Result<()>;
}
impl IMFCameraOcclusionStateReportCallback_Vtbl {
    pub const fn new<Identity: IMFCameraOcclusionStateReportCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnOcclusionStateReport<Identity: IMFCameraOcclusionStateReportCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, occlusionstatereport: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCameraOcclusionStateReportCallback_Impl::OnOcclusionStateReport(this, core::mem::transmute_copy(&occlusionstatereport)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnOcclusionStateReport: OnOcclusionStateReport::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFCameraOcclusionStateReportCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFCameraOcclusionStateReportCallback {}
windows_core::imp::define_interface!(IMFCapturePhotoConfirmation, IMFCapturePhotoConfirmation_Vtbl, 0x19f68549_ca8a_4706_a4ef_481dbc95e12c);
windows_core::imp::interface_hierarchy!(IMFCapturePhotoConfirmation, windows_core::IUnknown);
impl IMFCapturePhotoConfirmation {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn SetPhotoConfirmationCallback<P0>(&self, pnotificationcallback: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMFAsyncCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPhotoConfirmationCallback)(windows_core::Interface::as_raw(self), pnotificationcallback.param().abi()) }
    }
    pub unsafe fn SetPixelFormat(&self, subtype: windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPixelFormat)(windows_core::Interface::as_raw(self), subtype) }
    }
    pub unsafe fn GetPixelFormat(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPixelFormat)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFCapturePhotoConfirmation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "mfobjects")]
    pub SetPhotoConfirmationCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    SetPhotoConfirmationCallback: usize,
    pub SetPixelFormat: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub GetPixelFormat: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
#[cfg(feature = "mfobjects")]
pub trait IMFCapturePhotoConfirmation_Impl: windows_core::IUnknownImpl {
    fn SetPhotoConfirmationCallback(&self, pnotificationcallback: windows_core::Ref<super::IMFAsyncCallback>) -> windows_core::Result<()>;
    fn SetPixelFormat(&self, subtype: &windows_core::GUID) -> windows_core::Result<()>;
    fn GetPixelFormat(&self) -> windows_core::Result<windows_core::GUID>;
}
#[cfg(feature = "mfobjects")]
impl IMFCapturePhotoConfirmation_Vtbl {
    pub const fn new<Identity: IMFCapturePhotoConfirmation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetPhotoConfirmationCallback<Identity: IMFCapturePhotoConfirmation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnotificationcallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCapturePhotoConfirmation_Impl::SetPhotoConfirmationCallback(this, core::mem::transmute_copy(&pnotificationcallback)).into()
            }
        }
        unsafe extern "system" fn SetPixelFormat<Identity: IMFCapturePhotoConfirmation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, subtype: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFCapturePhotoConfirmation_Impl::SetPixelFormat(this, core::mem::transmute(&subtype)).into()
            }
        }
        unsafe extern "system" fn GetPixelFormat<Identity: IMFCapturePhotoConfirmation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, subtype: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFCapturePhotoConfirmation_Impl::GetPixelFormat(this) {
                    Ok(ok__) => {
                        subtype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetPhotoConfirmationCallback: SetPhotoConfirmationCallback::<Identity, OFFSET>,
            SetPixelFormat: SetPixelFormat::<Identity, OFFSET>,
            GetPixelFormat: GetPixelFormat::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFCapturePhotoConfirmation as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFCapturePhotoConfirmation {}
windows_core::imp::define_interface!(IMFClock, IMFClock_Vtbl, 0x2eb1e945_18b8_4139_9b1a_d5d584818530);
windows_core::imp::interface_hierarchy!(IMFClock, windows_core::IUnknown);
impl IMFClock {
    pub unsafe fn GetClockCharacteristics(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetClockCharacteristics)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCorrelatedTime(&self, dwreserved: u32, pllclocktime: *mut i64, phnssystemtime: *mut MFTIME) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCorrelatedTime)(windows_core::Interface::as_raw(self), dwreserved, pllclocktime as _, phnssystemtime as _) }
    }
    pub unsafe fn GetContinuityKey(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetContinuityKey)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetState(&self, dwreserved: u32) -> windows_core::Result<MFCLOCK_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetState)(windows_core::Interface::as_raw(self), dwreserved, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetProperties(&self, pclockproperties: *mut MFCLOCK_PROPERTIES) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetProperties)(windows_core::Interface::as_raw(self), pclockproperties as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFClock_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetClockCharacteristics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetCorrelatedTime: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut i64, *mut MFTIME) -> windows_core::HRESULT,
    pub GetContinuityKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetState: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut MFCLOCK_STATE) -> windows_core::HRESULT,
    pub GetProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MFCLOCK_PROPERTIES) -> windows_core::HRESULT,
}
pub trait IMFClock_Impl: windows_core::IUnknownImpl {
    fn GetClockCharacteristics(&self) -> windows_core::Result<u32>;
    fn GetCorrelatedTime(&self, dwreserved: u32, pllclocktime: *mut i64, phnssystemtime: *mut MFTIME) -> windows_core::Result<()>;
    fn GetContinuityKey(&self) -> windows_core::Result<u32>;
    fn GetState(&self, dwreserved: u32) -> windows_core::Result<MFCLOCK_STATE>;
    fn GetProperties(&self, pclockproperties: *mut MFCLOCK_PROPERTIES) -> windows_core::Result<()>;
}
impl IMFClock_Vtbl {
    pub const fn new<Identity: IMFClock_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetClockCharacteristics<Identity: IMFClock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcharacteristics: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFClock_Impl::GetClockCharacteristics(this) {
                    Ok(ok__) => {
                        pdwcharacteristics.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCorrelatedTime<Identity: IMFClock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwreserved: u32, pllclocktime: *mut i64, phnssystemtime: *mut MFTIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFClock_Impl::GetCorrelatedTime(this, core::mem::transmute_copy(&dwreserved), core::mem::transmute_copy(&pllclocktime), core::mem::transmute_copy(&phnssystemtime)).into()
            }
        }
        unsafe extern "system" fn GetContinuityKey<Identity: IMFClock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcontinuitykey: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFClock_Impl::GetContinuityKey(this) {
                    Ok(ok__) => {
                        pdwcontinuitykey.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetState<Identity: IMFClock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwreserved: u32, peclockstate: *mut MFCLOCK_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFClock_Impl::GetState(this, core::mem::transmute_copy(&dwreserved)) {
                    Ok(ok__) => {
                        peclockstate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProperties<Identity: IMFClock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclockproperties: *mut MFCLOCK_PROPERTIES) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFClock_Impl::GetProperties(this, core::mem::transmute_copy(&pclockproperties)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetClockCharacteristics: GetClockCharacteristics::<Identity, OFFSET>,
            GetCorrelatedTime: GetCorrelatedTime::<Identity, OFFSET>,
            GetContinuityKey: GetContinuityKey::<Identity, OFFSET>,
            GetState: GetState::<Identity, OFFSET>,
            GetProperties: GetProperties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFClock as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFClock {}
windows_core::imp::define_interface!(IMFClockConsumer, IMFClockConsumer_Vtbl, 0x6ef2a662_47c0_4666_b13d_cbb717f2fa2c);
windows_core::imp::interface_hierarchy!(IMFClockConsumer, windows_core::IUnknown);
impl IMFClockConsumer {
    pub unsafe fn SetPresentationClock<P0>(&self, ppresentationclock: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFPresentationClock>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPresentationClock)(windows_core::Interface::as_raw(self), ppresentationclock.param().abi()) }
    }
    pub unsafe fn GetPresentationClock(&self) -> windows_core::Result<IMFPresentationClock> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPresentationClock)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFClockConsumer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetPresentationClock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPresentationClock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFClockConsumer_Impl: windows_core::IUnknownImpl {
    fn SetPresentationClock(&self, ppresentationclock: windows_core::Ref<IMFPresentationClock>) -> windows_core::Result<()>;
    fn GetPresentationClock(&self) -> windows_core::Result<IMFPresentationClock>;
}
impl IMFClockConsumer_Vtbl {
    pub const fn new<Identity: IMFClockConsumer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetPresentationClock<Identity: IMFClockConsumer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresentationclock: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFClockConsumer_Impl::SetPresentationClock(this, core::mem::transmute_copy(&ppresentationclock)).into()
            }
        }
        unsafe extern "system" fn GetPresentationClock<Identity: IMFClockConsumer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppresentationclock: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFClockConsumer_Impl::GetPresentationClock(this) {
                    Ok(ok__) => {
                        pppresentationclock.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetPresentationClock: SetPresentationClock::<Identity, OFFSET>,
            GetPresentationClock: GetPresentationClock::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFClockConsumer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFClockConsumer {}
windows_core::imp::define_interface!(IMFClockStateSink, IMFClockStateSink_Vtbl, 0xf6696e82_74f7_4f3d_a178_8a5e09c3659f);
windows_core::imp::interface_hierarchy!(IMFClockStateSink, windows_core::IUnknown);
impl IMFClockStateSink {
    pub unsafe fn OnClockStart(&self, hnssystemtime: MFTIME, llclockstartoffset: i64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnClockStart)(windows_core::Interface::as_raw(self), hnssystemtime, llclockstartoffset) }
    }
    pub unsafe fn OnClockStop(&self, hnssystemtime: MFTIME) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnClockStop)(windows_core::Interface::as_raw(self), hnssystemtime) }
    }
    pub unsafe fn OnClockPause(&self, hnssystemtime: MFTIME) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnClockPause)(windows_core::Interface::as_raw(self), hnssystemtime) }
    }
    pub unsafe fn OnClockRestart(&self, hnssystemtime: MFTIME) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnClockRestart)(windows_core::Interface::as_raw(self), hnssystemtime) }
    }
    pub unsafe fn OnClockSetRate(&self, hnssystemtime: MFTIME, flrate: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnClockSetRate)(windows_core::Interface::as_raw(self), hnssystemtime, flrate) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFClockStateSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnClockStart: unsafe extern "system" fn(*mut core::ffi::c_void, MFTIME, i64) -> windows_core::HRESULT,
    pub OnClockStop: unsafe extern "system" fn(*mut core::ffi::c_void, MFTIME) -> windows_core::HRESULT,
    pub OnClockPause: unsafe extern "system" fn(*mut core::ffi::c_void, MFTIME) -> windows_core::HRESULT,
    pub OnClockRestart: unsafe extern "system" fn(*mut core::ffi::c_void, MFTIME) -> windows_core::HRESULT,
    pub OnClockSetRate: unsafe extern "system" fn(*mut core::ffi::c_void, MFTIME, f32) -> windows_core::HRESULT,
}
pub trait IMFClockStateSink_Impl: windows_core::IUnknownImpl {
    fn OnClockStart(&self, hnssystemtime: MFTIME, llclockstartoffset: i64) -> windows_core::Result<()>;
    fn OnClockStop(&self, hnssystemtime: MFTIME) -> windows_core::Result<()>;
    fn OnClockPause(&self, hnssystemtime: MFTIME) -> windows_core::Result<()>;
    fn OnClockRestart(&self, hnssystemtime: MFTIME) -> windows_core::Result<()>;
    fn OnClockSetRate(&self, hnssystemtime: MFTIME, flrate: f32) -> windows_core::Result<()>;
}
impl IMFClockStateSink_Vtbl {
    pub const fn new<Identity: IMFClockStateSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnClockStart<Identity: IMFClockStateSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hnssystemtime: MFTIME, llclockstartoffset: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFClockStateSink_Impl::OnClockStart(this, core::mem::transmute_copy(&hnssystemtime), core::mem::transmute_copy(&llclockstartoffset)).into()
            }
        }
        unsafe extern "system" fn OnClockStop<Identity: IMFClockStateSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hnssystemtime: MFTIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFClockStateSink_Impl::OnClockStop(this, core::mem::transmute_copy(&hnssystemtime)).into()
            }
        }
        unsafe extern "system" fn OnClockPause<Identity: IMFClockStateSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hnssystemtime: MFTIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFClockStateSink_Impl::OnClockPause(this, core::mem::transmute_copy(&hnssystemtime)).into()
            }
        }
        unsafe extern "system" fn OnClockRestart<Identity: IMFClockStateSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hnssystemtime: MFTIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFClockStateSink_Impl::OnClockRestart(this, core::mem::transmute_copy(&hnssystemtime)).into()
            }
        }
        unsafe extern "system" fn OnClockSetRate<Identity: IMFClockStateSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hnssystemtime: MFTIME, flrate: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFClockStateSink_Impl::OnClockSetRate(this, core::mem::transmute_copy(&hnssystemtime), core::mem::transmute_copy(&flrate)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnClockStart: OnClockStart::<Identity, OFFSET>,
            OnClockStop: OnClockStop::<Identity, OFFSET>,
            OnClockPause: OnClockPause::<Identity, OFFSET>,
            OnClockRestart: OnClockRestart::<Identity, OFFSET>,
            OnClockSetRate: OnClockSetRate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFClockStateSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFClockStateSink {}
windows_core::imp::define_interface!(IMFContentDecryptorContext, IMFContentDecryptorContext_Vtbl, 0x7ec4b1bd_43fb_4763_85d2_64fcb5c5f4cb);
windows_core::imp::interface_hierarchy!(IMFContentDecryptorContext, windows_core::IUnknown);
impl IMFContentDecryptorContext {
    pub unsafe fn InitializeHardwareKey(&self, inputprivatedata: Option<&[u8]>) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InitializeHardwareKey)(windows_core::Interface::as_raw(self), inputprivatedata.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(inputprivatedata.map_or(core::ptr::null(), |slice| slice.as_ptr())), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFContentDecryptorContext_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InitializeHardwareKey: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
}
pub trait IMFContentDecryptorContext_Impl: windows_core::IUnknownImpl {
    fn InitializeHardwareKey(&self, inputprivatedatabytecount: u32, inputprivatedata: *const core::ffi::c_void) -> windows_core::Result<u64>;
}
impl IMFContentDecryptorContext_Vtbl {
    pub const fn new<Identity: IMFContentDecryptorContext_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InitializeHardwareKey<Identity: IMFContentDecryptorContext_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputprivatedatabytecount: u32, inputprivatedata: *const core::ffi::c_void, outputprivatedata: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFContentDecryptorContext_Impl::InitializeHardwareKey(this, core::mem::transmute_copy(&inputprivatedatabytecount), core::mem::transmute_copy(&inputprivatedata)) {
                    Ok(ok__) => {
                        outputprivatedata.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), InitializeHardwareKey: InitializeHardwareKey::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFContentDecryptorContext as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFContentDecryptorContext {}
windows_core::imp::define_interface!(IMFContentEnabler, IMFContentEnabler_Vtbl, 0xd3c4ef59_49ce_4381_9071_d5bcd044c770);
windows_core::imp::interface_hierarchy!(IMFContentEnabler, windows_core::IUnknown);
impl IMFContentEnabler {
    pub unsafe fn GetEnableType(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEnableType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetEnableURL(&self, ppwszurl: *mut windows_core::PWSTR, pcchurl: *mut u32, ptruststatus: *mut MF_URL_TRUST_STATUS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetEnableURL)(windows_core::Interface::as_raw(self), ppwszurl as _, pcchurl as _, ptruststatus as _) }
    }
    pub unsafe fn GetEnableData(&self, ppbdata: *mut *mut u8, pcbdata: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetEnableData)(windows_core::Interface::as_raw(self), ppbdata as _, pcbdata as _) }
    }
    pub unsafe fn IsAutomaticSupported(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsAutomaticSupported)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AutomaticEnable(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AutomaticEnable)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn MonitorEnable(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).MonitorEnable)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Cancel(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFContentEnabler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetEnableType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetEnableURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR, *mut u32, *mut MF_URL_TRUST_STATUS) -> windows_core::HRESULT,
    pub GetEnableData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub IsAutomaticSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub AutomaticEnable: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MonitorEnable: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFContentEnabler_Impl: windows_core::IUnknownImpl {
    fn GetEnableType(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetEnableURL(&self, ppwszurl: *mut windows_core::PWSTR, pcchurl: *mut u32, ptruststatus: *mut MF_URL_TRUST_STATUS) -> windows_core::Result<()>;
    fn GetEnableData(&self, ppbdata: *mut *mut u8, pcbdata: *mut u32) -> windows_core::Result<()>;
    fn IsAutomaticSupported(&self) -> windows_core::Result<windows_core::BOOL>;
    fn AutomaticEnable(&self) -> windows_core::Result<()>;
    fn MonitorEnable(&self) -> windows_core::Result<()>;
    fn Cancel(&self) -> windows_core::Result<()>;
}
impl IMFContentEnabler_Vtbl {
    pub const fn new<Identity: IMFContentEnabler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetEnableType<Identity: IMFContentEnabler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFContentEnabler_Impl::GetEnableType(this) {
                    Ok(ok__) => {
                        ptype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetEnableURL<Identity: IMFContentEnabler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwszurl: *mut windows_core::PWSTR, pcchurl: *mut u32, ptruststatus: *mut MF_URL_TRUST_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFContentEnabler_Impl::GetEnableURL(this, core::mem::transmute_copy(&ppwszurl), core::mem::transmute_copy(&pcchurl), core::mem::transmute_copy(&ptruststatus)).into()
            }
        }
        unsafe extern "system" fn GetEnableData<Identity: IMFContentEnabler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppbdata: *mut *mut u8, pcbdata: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFContentEnabler_Impl::GetEnableData(this, core::mem::transmute_copy(&ppbdata), core::mem::transmute_copy(&pcbdata)).into()
            }
        }
        unsafe extern "system" fn IsAutomaticSupported<Identity: IMFContentEnabler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfautomatic: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFContentEnabler_Impl::IsAutomaticSupported(this) {
                    Ok(ok__) => {
                        pfautomatic.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AutomaticEnable<Identity: IMFContentEnabler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFContentEnabler_Impl::AutomaticEnable(this).into()
            }
        }
        unsafe extern "system" fn MonitorEnable<Identity: IMFContentEnabler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFContentEnabler_Impl::MonitorEnable(this).into()
            }
        }
        unsafe extern "system" fn Cancel<Identity: IMFContentEnabler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFContentEnabler_Impl::Cancel(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetEnableType: GetEnableType::<Identity, OFFSET>,
            GetEnableURL: GetEnableURL::<Identity, OFFSET>,
            GetEnableData: GetEnableData::<Identity, OFFSET>,
            IsAutomaticSupported: IsAutomaticSupported::<Identity, OFFSET>,
            AutomaticEnable: AutomaticEnable::<Identity, OFFSET>,
            MonitorEnable: MonitorEnable::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFContentEnabler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFContentEnabler {}
windows_core::imp::define_interface!(IMFContentProtectionDevice, IMFContentProtectionDevice_Vtbl, 0xe6257174_a060_4c9a_a088_3b1b471cad28);
windows_core::imp::interface_hierarchy!(IMFContentProtectionDevice, windows_core::IUnknown);
impl IMFContentProtectionDevice {
    pub unsafe fn InvokeFunction(&self, functionid: u32, inputbuffer: &[u8], outputbufferbytecount: *mut u32, outputbuffer: *mut u8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InvokeFunction)(windows_core::Interface::as_raw(self), functionid, inputbuffer.len().try_into().unwrap(), inputbuffer.as_ptr(), outputbufferbytecount as _, outputbuffer as _) }
    }
    pub unsafe fn GetPrivateDataByteCount(&self, privateinputbytecount: *mut u32, privateoutputbytecount: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPrivateDataByteCount)(windows_core::Interface::as_raw(self), privateinputbytecount as _, privateoutputbytecount as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFContentProtectionDevice_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InvokeFunction: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const u8, *mut u32, *mut u8) -> windows_core::HRESULT,
    pub GetPrivateDataByteCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut u32) -> windows_core::HRESULT,
}
pub trait IMFContentProtectionDevice_Impl: windows_core::IUnknownImpl {
    fn InvokeFunction(&self, functionid: u32, inputbufferbytecount: u32, inputbuffer: *const u8, outputbufferbytecount: *mut u32, outputbuffer: *mut u8) -> windows_core::Result<()>;
    fn GetPrivateDataByteCount(&self, privateinputbytecount: *mut u32, privateoutputbytecount: *mut u32) -> windows_core::Result<()>;
}
impl IMFContentProtectionDevice_Vtbl {
    pub const fn new<Identity: IMFContentProtectionDevice_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InvokeFunction<Identity: IMFContentProtectionDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, functionid: u32, inputbufferbytecount: u32, inputbuffer: *const u8, outputbufferbytecount: *mut u32, outputbuffer: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFContentProtectionDevice_Impl::InvokeFunction(this, core::mem::transmute_copy(&functionid), core::mem::transmute_copy(&inputbufferbytecount), core::mem::transmute_copy(&inputbuffer), core::mem::transmute_copy(&outputbufferbytecount), core::mem::transmute_copy(&outputbuffer)).into()
            }
        }
        unsafe extern "system" fn GetPrivateDataByteCount<Identity: IMFContentProtectionDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, privateinputbytecount: *mut u32, privateoutputbytecount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFContentProtectionDevice_Impl::GetPrivateDataByteCount(this, core::mem::transmute_copy(&privateinputbytecount), core::mem::transmute_copy(&privateoutputbytecount)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InvokeFunction: InvokeFunction::<Identity, OFFSET>,
            GetPrivateDataByteCount: GetPrivateDataByteCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFContentProtectionDevice as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFContentProtectionDevice {}
windows_core::imp::define_interface!(IMFContentProtectionManager, IMFContentProtectionManager_Vtbl, 0xacf92459_6a61_42bd_b57c_b43e51203cb0);
windows_core::imp::interface_hierarchy!(IMFContentProtectionManager, windows_core::IUnknown);
impl IMFContentProtectionManager {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn BeginEnableContent<P0, P1, P2, P3>(&self, penableractivate: P0, ptopo: P1, pcallback: P2, punkstate: P3) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMFActivate>,
        P1: windows_core::Param<IMFTopology>,
        P2: windows_core::Param<super::IMFAsyncCallback>,
        P3: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).BeginEnableContent)(windows_core::Interface::as_raw(self), penableractivate.param().abi(), ptopo.param().abi(), pcallback.param().abi(), punkstate.param().abi()) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn EndEnableContent<P0>(&self, presult: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMFAsyncResult>,
    {
        unsafe { (windows_core::Interface::vtable(self).EndEnableContent)(windows_core::Interface::as_raw(self), presult.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFContentProtectionManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "mfobjects")]
    pub BeginEnableContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    BeginEnableContent: usize,
    #[cfg(feature = "mfobjects")]
    pub EndEnableContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    EndEnableContent: usize,
}
#[cfg(feature = "mfobjects")]
pub trait IMFContentProtectionManager_Impl: windows_core::IUnknownImpl {
    fn BeginEnableContent(&self, penableractivate: windows_core::Ref<super::IMFActivate>, ptopo: windows_core::Ref<IMFTopology>, pcallback: windows_core::Ref<super::IMFAsyncCallback>, punkstate: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn EndEnableContent(&self, presult: windows_core::Ref<super::IMFAsyncResult>) -> windows_core::Result<()>;
}
#[cfg(feature = "mfobjects")]
impl IMFContentProtectionManager_Vtbl {
    pub const fn new<Identity: IMFContentProtectionManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BeginEnableContent<Identity: IMFContentProtectionManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, penableractivate: *mut core::ffi::c_void, ptopo: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void, punkstate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFContentProtectionManager_Impl::BeginEnableContent(this, core::mem::transmute_copy(&penableractivate), core::mem::transmute_copy(&ptopo), core::mem::transmute_copy(&pcallback), core::mem::transmute_copy(&punkstate)).into()
            }
        }
        unsafe extern "system" fn EndEnableContent<Identity: IMFContentProtectionManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presult: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFContentProtectionManager_Impl::EndEnableContent(this, core::mem::transmute_copy(&presult)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginEnableContent: BeginEnableContent::<Identity, OFFSET>,
            EndEnableContent: EndEnableContent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFContentProtectionManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFContentProtectionManager {}
windows_core::imp::define_interface!(IMFDXGIDeviceManagerSource, IMFDXGIDeviceManagerSource_Vtbl, 0x20bc074b_7a8d_4609_8c3b_64a0a3b5d7ce);
windows_core::imp::interface_hierarchy!(IMFDXGIDeviceManagerSource, windows_core::IUnknown);
impl IMFDXGIDeviceManagerSource {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn GetManager(&self) -> windows_core::Result<super::IMFDXGIDeviceManager> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetManager)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFDXGIDeviceManagerSource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "mfobjects")]
    pub GetManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    GetManager: usize,
}
#[cfg(feature = "mfobjects")]
pub trait IMFDXGIDeviceManagerSource_Impl: windows_core::IUnknownImpl {
    fn GetManager(&self) -> windows_core::Result<super::IMFDXGIDeviceManager>;
}
#[cfg(feature = "mfobjects")]
impl IMFDXGIDeviceManagerSource_Vtbl {
    pub const fn new<Identity: IMFDXGIDeviceManagerSource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetManager<Identity: IMFDXGIDeviceManagerSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmanager: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFDXGIDeviceManagerSource_Impl::GetManager(this) {
                    Ok(ok__) => {
                        ppmanager.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetManager: GetManager::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFDXGIDeviceManagerSource as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFDXGIDeviceManagerSource {}
windows_core::imp::define_interface!(IMFExtendedCameraControl, IMFExtendedCameraControl_Vtbl, 0x38e33520_fca1_4845_a27a_68b7c6ab3789);
windows_core::imp::interface_hierarchy!(IMFExtendedCameraControl, windows_core::IUnknown);
impl IMFExtendedCameraControl {
    pub unsafe fn GetCapabilities(&self) -> u64 {
        unsafe { (windows_core::Interface::vtable(self).GetCapabilities)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetFlags(&self, ulflags: u64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFlags)(windows_core::Interface::as_raw(self), ulflags) }
    }
    pub unsafe fn GetFlags(&self) -> u64 {
        unsafe { (windows_core::Interface::vtable(self).GetFlags)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn LockPayload(&self, pppayload: *mut *mut u8, pulpayload: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LockPayload)(windows_core::Interface::as_raw(self), pppayload as _, pulpayload as _) }
    }
    pub unsafe fn UnlockPayload(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnlockPayload)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn CommitSettings(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CommitSettings)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFExtendedCameraControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void) -> u64,
    pub SetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(*mut core::ffi::c_void) -> u64,
    pub LockPayload: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub UnlockPayload: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CommitSettings: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFExtendedCameraControl_Impl: windows_core::IUnknownImpl {
    fn GetCapabilities(&self) -> u64;
    fn SetFlags(&self, ulflags: u64) -> windows_core::Result<()>;
    fn GetFlags(&self) -> u64;
    fn LockPayload(&self, pppayload: *mut *mut u8, pulpayload: *mut u32) -> windows_core::Result<()>;
    fn UnlockPayload(&self) -> windows_core::Result<()>;
    fn CommitSettings(&self) -> windows_core::Result<()>;
}
impl IMFExtendedCameraControl_Vtbl {
    pub const fn new<Identity: IMFExtendedCameraControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCapabilities<Identity: IMFExtendedCameraControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFExtendedCameraControl_Impl::GetCapabilities(this)
            }
        }
        unsafe extern "system" fn SetFlags<Identity: IMFExtendedCameraControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulflags: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFExtendedCameraControl_Impl::SetFlags(this, core::mem::transmute_copy(&ulflags)).into()
            }
        }
        unsafe extern "system" fn GetFlags<Identity: IMFExtendedCameraControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u64 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFExtendedCameraControl_Impl::GetFlags(this)
            }
        }
        unsafe extern "system" fn LockPayload<Identity: IMFExtendedCameraControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppayload: *mut *mut u8, pulpayload: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFExtendedCameraControl_Impl::LockPayload(this, core::mem::transmute_copy(&pppayload), core::mem::transmute_copy(&pulpayload)).into()
            }
        }
        unsafe extern "system" fn UnlockPayload<Identity: IMFExtendedCameraControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFExtendedCameraControl_Impl::UnlockPayload(this).into()
            }
        }
        unsafe extern "system" fn CommitSettings<Identity: IMFExtendedCameraControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFExtendedCameraControl_Impl::CommitSettings(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCapabilities: GetCapabilities::<Identity, OFFSET>,
            SetFlags: SetFlags::<Identity, OFFSET>,
            GetFlags: GetFlags::<Identity, OFFSET>,
            LockPayload: LockPayload::<Identity, OFFSET>,
            UnlockPayload: UnlockPayload::<Identity, OFFSET>,
            CommitSettings: CommitSettings::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFExtendedCameraControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFExtendedCameraControl {}
windows_core::imp::define_interface!(IMFExtendedCameraController, IMFExtendedCameraController_Vtbl, 0xb91ebfee_ca03_4af4_8a82_a31752f4a0fc);
windows_core::imp::interface_hierarchy!(IMFExtendedCameraController, windows_core::IUnknown);
impl IMFExtendedCameraController {
    pub unsafe fn GetExtendedCameraControl(&self, dwstreamindex: u32, ulpropertyid: u32) -> windows_core::Result<IMFExtendedCameraControl> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetExtendedCameraControl)(windows_core::Interface::as_raw(self), dwstreamindex, ulpropertyid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFExtendedCameraController_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetExtendedCameraControl: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFExtendedCameraController_Impl: windows_core::IUnknownImpl {
    fn GetExtendedCameraControl(&self, dwstreamindex: u32, ulpropertyid: u32) -> windows_core::Result<IMFExtendedCameraControl>;
}
impl IMFExtendedCameraController_Vtbl {
    pub const fn new<Identity: IMFExtendedCameraController_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetExtendedCameraControl<Identity: IMFExtendedCameraController_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamindex: u32, ulpropertyid: u32, ppcontrol: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFExtendedCameraController_Impl::GetExtendedCameraControl(this, core::mem::transmute_copy(&dwstreamindex), core::mem::transmute_copy(&ulpropertyid)) {
                    Ok(ok__) => {
                        ppcontrol.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetExtendedCameraControl: GetExtendedCameraControl::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFExtendedCameraController as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFExtendedCameraController {}
windows_core::imp::define_interface!(IMFExtendedCameraIntrinsicModel, IMFExtendedCameraIntrinsicModel_Vtbl, 0x5c595e64_4630_4231_855a_12842f733245);
windows_core::imp::interface_hierarchy!(IMFExtendedCameraIntrinsicModel, windows_core::IUnknown);
impl IMFExtendedCameraIntrinsicModel {
    pub unsafe fn GetModel(&self, pintrinsicmodel: *mut MFExtendedCameraIntrinsic_IntrinsicModel) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetModel)(windows_core::Interface::as_raw(self), pintrinsicmodel as _) }
    }
    pub unsafe fn SetModel(&self, pintrinsicmodel: *const MFExtendedCameraIntrinsic_IntrinsicModel) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetModel)(windows_core::Interface::as_raw(self), pintrinsicmodel) }
    }
    pub unsafe fn GetDistortionModelType(&self) -> windows_core::Result<MFCameraIntrinsic_DistortionModelType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDistortionModelType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFExtendedCameraIntrinsicModel_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetModel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MFExtendedCameraIntrinsic_IntrinsicModel) -> windows_core::HRESULT,
    pub SetModel: unsafe extern "system" fn(*mut core::ffi::c_void, *const MFExtendedCameraIntrinsic_IntrinsicModel) -> windows_core::HRESULT,
    pub GetDistortionModelType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MFCameraIntrinsic_DistortionModelType) -> windows_core::HRESULT,
}
pub trait IMFExtendedCameraIntrinsicModel_Impl: windows_core::IUnknownImpl {
    fn GetModel(&self, pintrinsicmodel: *mut MFExtendedCameraIntrinsic_IntrinsicModel) -> windows_core::Result<()>;
    fn SetModel(&self, pintrinsicmodel: *const MFExtendedCameraIntrinsic_IntrinsicModel) -> windows_core::Result<()>;
    fn GetDistortionModelType(&self) -> windows_core::Result<MFCameraIntrinsic_DistortionModelType>;
}
impl IMFExtendedCameraIntrinsicModel_Vtbl {
    pub const fn new<Identity: IMFExtendedCameraIntrinsicModel_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetModel<Identity: IMFExtendedCameraIntrinsicModel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pintrinsicmodel: *mut MFExtendedCameraIntrinsic_IntrinsicModel) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFExtendedCameraIntrinsicModel_Impl::GetModel(this, core::mem::transmute_copy(&pintrinsicmodel)).into()
            }
        }
        unsafe extern "system" fn SetModel<Identity: IMFExtendedCameraIntrinsicModel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pintrinsicmodel: *const MFExtendedCameraIntrinsic_IntrinsicModel) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFExtendedCameraIntrinsicModel_Impl::SetModel(this, core::mem::transmute_copy(&pintrinsicmodel)).into()
            }
        }
        unsafe extern "system" fn GetDistortionModelType<Identity: IMFExtendedCameraIntrinsicModel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdistortionmodeltype: *mut MFCameraIntrinsic_DistortionModelType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFExtendedCameraIntrinsicModel_Impl::GetDistortionModelType(this) {
                    Ok(ok__) => {
                        pdistortionmodeltype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetModel: GetModel::<Identity, OFFSET>,
            SetModel: SetModel::<Identity, OFFSET>,
            GetDistortionModelType: GetDistortionModelType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFExtendedCameraIntrinsicModel as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFExtendedCameraIntrinsicModel {}
windows_core::imp::define_interface!(IMFExtendedCameraIntrinsics, IMFExtendedCameraIntrinsics_Vtbl, 0x687f6dac_6987_4750_a16a_734d1e7a10fe);
windows_core::imp::interface_hierarchy!(IMFExtendedCameraIntrinsics, windows_core::IUnknown);
impl IMFExtendedCameraIntrinsics {
    pub unsafe fn InitializeFromBuffer(&self, pbbuffer: &[u8]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InitializeFromBuffer)(windows_core::Interface::as_raw(self), pbbuffer.as_ptr(), pbbuffer.len().try_into().unwrap()) }
    }
    pub unsafe fn GetBufferSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBufferSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SerializeToBuffer(&self, pbbuffer: *mut u8, pdwbuffersize: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SerializeToBuffer)(windows_core::Interface::as_raw(self), pbbuffer as _, pdwbuffersize as _) }
    }
    pub unsafe fn GetIntrinsicModelCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIntrinsicModelCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetIntrinsicModelByIndex(&self, dwindex: u32) -> windows_core::Result<IMFExtendedCameraIntrinsicModel> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIntrinsicModelByIndex)(windows_core::Interface::as_raw(self), dwindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AddIntrinsicModel<P0>(&self, pintrinsicmodel: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFExtendedCameraIntrinsicModel>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddIntrinsicModel)(windows_core::Interface::as_raw(self), pintrinsicmodel.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFExtendedCameraIntrinsics_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub InitializeFromBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
    pub GetBufferSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SerializeToBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetIntrinsicModelCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetIntrinsicModelByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddIntrinsicModel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFExtendedCameraIntrinsics_Impl: windows_core::IUnknownImpl {
    fn InitializeFromBuffer(&self, pbbuffer: *const u8, dwbuffersize: u32) -> windows_core::Result<()>;
    fn GetBufferSize(&self) -> windows_core::Result<u32>;
    fn SerializeToBuffer(&self, pbbuffer: *mut u8, pdwbuffersize: *mut u32) -> windows_core::Result<()>;
    fn GetIntrinsicModelCount(&self) -> windows_core::Result<u32>;
    fn GetIntrinsicModelByIndex(&self, dwindex: u32) -> windows_core::Result<IMFExtendedCameraIntrinsicModel>;
    fn AddIntrinsicModel(&self, pintrinsicmodel: windows_core::Ref<IMFExtendedCameraIntrinsicModel>) -> windows_core::Result<()>;
}
impl IMFExtendedCameraIntrinsics_Vtbl {
    pub const fn new<Identity: IMFExtendedCameraIntrinsics_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InitializeFromBuffer<Identity: IMFExtendedCameraIntrinsics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbbuffer: *const u8, dwbuffersize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFExtendedCameraIntrinsics_Impl::InitializeFromBuffer(this, core::mem::transmute_copy(&pbbuffer), core::mem::transmute_copy(&dwbuffersize)).into()
            }
        }
        unsafe extern "system" fn GetBufferSize<Identity: IMFExtendedCameraIntrinsics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwbuffersize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFExtendedCameraIntrinsics_Impl::GetBufferSize(this) {
                    Ok(ok__) => {
                        pdwbuffersize.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SerializeToBuffer<Identity: IMFExtendedCameraIntrinsics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbbuffer: *mut u8, pdwbuffersize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFExtendedCameraIntrinsics_Impl::SerializeToBuffer(this, core::mem::transmute_copy(&pbbuffer), core::mem::transmute_copy(&pdwbuffersize)).into()
            }
        }
        unsafe extern "system" fn GetIntrinsicModelCount<Identity: IMFExtendedCameraIntrinsics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFExtendedCameraIntrinsics_Impl::GetIntrinsicModelCount(this) {
                    Ok(ok__) => {
                        pdwcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetIntrinsicModelByIndex<Identity: IMFExtendedCameraIntrinsics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, ppintrinsicmodel: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFExtendedCameraIntrinsics_Impl::GetIntrinsicModelByIndex(this, core::mem::transmute_copy(&dwindex)) {
                    Ok(ok__) => {
                        ppintrinsicmodel.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddIntrinsicModel<Identity: IMFExtendedCameraIntrinsics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pintrinsicmodel: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFExtendedCameraIntrinsics_Impl::AddIntrinsicModel(this, core::mem::transmute_copy(&pintrinsicmodel)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            InitializeFromBuffer: InitializeFromBuffer::<Identity, OFFSET>,
            GetBufferSize: GetBufferSize::<Identity, OFFSET>,
            SerializeToBuffer: SerializeToBuffer::<Identity, OFFSET>,
            GetIntrinsicModelCount: GetIntrinsicModelCount::<Identity, OFFSET>,
            GetIntrinsicModelByIndex: GetIntrinsicModelByIndex::<Identity, OFFSET>,
            AddIntrinsicModel: AddIntrinsicModel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFExtendedCameraIntrinsics as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFExtendedCameraIntrinsics {}
windows_core::imp::define_interface!(IMFExtendedCameraIntrinsicsDistortionModel6KT, IMFExtendedCameraIntrinsicsDistortionModel6KT_Vtbl, 0x74c2653b_5f55_4eb1_9f0f_18b8f68b7d3d);
windows_core::imp::interface_hierarchy!(IMFExtendedCameraIntrinsicsDistortionModel6KT, windows_core::IUnknown);
impl IMFExtendedCameraIntrinsicsDistortionModel6KT {
    pub unsafe fn GetDistortionModel(&self, pdistortionmodel: *mut MFCameraIntrinsic_DistortionModel6KT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDistortionModel)(windows_core::Interface::as_raw(self), pdistortionmodel as _) }
    }
    pub unsafe fn SetDistortionModel(&self, pdistortionmodel: *const MFCameraIntrinsic_DistortionModel6KT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDistortionModel)(windows_core::Interface::as_raw(self), pdistortionmodel) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFExtendedCameraIntrinsicsDistortionModel6KT_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDistortionModel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MFCameraIntrinsic_DistortionModel6KT) -> windows_core::HRESULT,
    pub SetDistortionModel: unsafe extern "system" fn(*mut core::ffi::c_void, *const MFCameraIntrinsic_DistortionModel6KT) -> windows_core::HRESULT,
}
pub trait IMFExtendedCameraIntrinsicsDistortionModel6KT_Impl: windows_core::IUnknownImpl {
    fn GetDistortionModel(&self, pdistortionmodel: *mut MFCameraIntrinsic_DistortionModel6KT) -> windows_core::Result<()>;
    fn SetDistortionModel(&self, pdistortionmodel: *const MFCameraIntrinsic_DistortionModel6KT) -> windows_core::Result<()>;
}
impl IMFExtendedCameraIntrinsicsDistortionModel6KT_Vtbl {
    pub const fn new<Identity: IMFExtendedCameraIntrinsicsDistortionModel6KT_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDistortionModel<Identity: IMFExtendedCameraIntrinsicsDistortionModel6KT_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdistortionmodel: *mut MFCameraIntrinsic_DistortionModel6KT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFExtendedCameraIntrinsicsDistortionModel6KT_Impl::GetDistortionModel(this, core::mem::transmute_copy(&pdistortionmodel)).into()
            }
        }
        unsafe extern "system" fn SetDistortionModel<Identity: IMFExtendedCameraIntrinsicsDistortionModel6KT_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdistortionmodel: *const MFCameraIntrinsic_DistortionModel6KT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFExtendedCameraIntrinsicsDistortionModel6KT_Impl::SetDistortionModel(this, core::mem::transmute_copy(&pdistortionmodel)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDistortionModel: GetDistortionModel::<Identity, OFFSET>,
            SetDistortionModel: SetDistortionModel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFExtendedCameraIntrinsicsDistortionModel6KT as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFExtendedCameraIntrinsicsDistortionModel6KT {}
windows_core::imp::define_interface!(IMFExtendedCameraIntrinsicsDistortionModelArcTan, IMFExtendedCameraIntrinsicsDistortionModelArcTan_Vtbl, 0x812d5f95_b572_45dc_bafc_ae24199ddda8);
windows_core::imp::interface_hierarchy!(IMFExtendedCameraIntrinsicsDistortionModelArcTan, windows_core::IUnknown);
impl IMFExtendedCameraIntrinsicsDistortionModelArcTan {
    pub unsafe fn GetDistortionModel(&self, pdistortionmodel: *mut MFCameraIntrinsic_DistortionModelArcTan) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDistortionModel)(windows_core::Interface::as_raw(self), pdistortionmodel as _) }
    }
    pub unsafe fn SetDistortionModel(&self, pdistortionmodel: *const MFCameraIntrinsic_DistortionModelArcTan) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDistortionModel)(windows_core::Interface::as_raw(self), pdistortionmodel) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFExtendedCameraIntrinsicsDistortionModelArcTan_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDistortionModel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MFCameraIntrinsic_DistortionModelArcTan) -> windows_core::HRESULT,
    pub SetDistortionModel: unsafe extern "system" fn(*mut core::ffi::c_void, *const MFCameraIntrinsic_DistortionModelArcTan) -> windows_core::HRESULT,
}
pub trait IMFExtendedCameraIntrinsicsDistortionModelArcTan_Impl: windows_core::IUnknownImpl {
    fn GetDistortionModel(&self, pdistortionmodel: *mut MFCameraIntrinsic_DistortionModelArcTan) -> windows_core::Result<()>;
    fn SetDistortionModel(&self, pdistortionmodel: *const MFCameraIntrinsic_DistortionModelArcTan) -> windows_core::Result<()>;
}
impl IMFExtendedCameraIntrinsicsDistortionModelArcTan_Vtbl {
    pub const fn new<Identity: IMFExtendedCameraIntrinsicsDistortionModelArcTan_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDistortionModel<Identity: IMFExtendedCameraIntrinsicsDistortionModelArcTan_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdistortionmodel: *mut MFCameraIntrinsic_DistortionModelArcTan) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFExtendedCameraIntrinsicsDistortionModelArcTan_Impl::GetDistortionModel(this, core::mem::transmute_copy(&pdistortionmodel)).into()
            }
        }
        unsafe extern "system" fn SetDistortionModel<Identity: IMFExtendedCameraIntrinsicsDistortionModelArcTan_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdistortionmodel: *const MFCameraIntrinsic_DistortionModelArcTan) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFExtendedCameraIntrinsicsDistortionModelArcTan_Impl::SetDistortionModel(this, core::mem::transmute_copy(&pdistortionmodel)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDistortionModel: GetDistortionModel::<Identity, OFFSET>,
            SetDistortionModel: SetDistortionModel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFExtendedCameraIntrinsicsDistortionModelArcTan as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFExtendedCameraIntrinsicsDistortionModelArcTan {}
windows_core::imp::define_interface!(IMFFaceDetectionTransform, IMFFaceDetectionTransform_Vtbl, 0xddd59578_d0e7_46e2_be8c_1ce76ad147c0);
windows_core::imp::interface_hierarchy!(IMFFaceDetectionTransform, windows_core::IUnknown);
impl IMFFaceDetectionTransform {
    pub unsafe fn SetDetectionCallback<P0>(&self, callback: P0, callbacktoken: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFFaceDetectionTransformCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDetectionCallback)(windows_core::Interface::as_raw(self), callback.param().abi(), callbacktoken as _) }
    }
    pub unsafe fn ClearDetectionCallback(&self, callbacktoken: *const core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ClearDetectionCallback)(windows_core::Interface::as_raw(self), callbacktoken) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFFaceDetectionTransform_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetDetectionCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClearDetectionCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *const core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFFaceDetectionTransform_Impl: windows_core::IUnknownImpl {
    fn SetDetectionCallback(&self, callback: windows_core::Ref<IMFFaceDetectionTransformCallback>, callbacktoken: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn ClearDetectionCallback(&self, callbacktoken: *const core::ffi::c_void) -> windows_core::Result<()>;
}
impl IMFFaceDetectionTransform_Vtbl {
    pub const fn new<Identity: IMFFaceDetectionTransform_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetDetectionCallback<Identity: IMFFaceDetectionTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, callback: *mut core::ffi::c_void, callbacktoken: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFFaceDetectionTransform_Impl::SetDetectionCallback(this, core::mem::transmute_copy(&callback), core::mem::transmute_copy(&callbacktoken)).into()
            }
        }
        unsafe extern "system" fn ClearDetectionCallback<Identity: IMFFaceDetectionTransform_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, callbacktoken: *const core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFFaceDetectionTransform_Impl::ClearDetectionCallback(this, core::mem::transmute_copy(&callbacktoken)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetDetectionCallback: SetDetectionCallback::<Identity, OFFSET>,
            ClearDetectionCallback: ClearDetectionCallback::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFFaceDetectionTransform as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFFaceDetectionTransform {}
windows_core::imp::define_interface!(IMFFaceDetectionTransformCallback, IMFFaceDetectionTransformCallback_Vtbl, 0x0bfd1ade_0421_4909_acb7_7a7125416881);
windows_core::imp::interface_hierarchy!(IMFFaceDetectionTransformCallback, windows_core::IUnknown);
impl IMFFaceDetectionTransformCallback {
    pub unsafe fn OnFaceDetectionResult(&self, detectedfacebounds: &[DetectedFaceBound]) {
        unsafe {
            (windows_core::Interface::vtable(self).OnFaceDetectionResult)(windows_core::Interface::as_raw(self), detectedfacebounds.len().try_into().unwrap(), detectedfacebounds.as_ptr());
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFFaceDetectionTransformCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnFaceDetectionResult: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const DetectedFaceBound),
}
pub trait IMFFaceDetectionTransformCallback_Impl: windows_core::IUnknownImpl {
    fn OnFaceDetectionResult(&self, countofbounds: u32, detectedfacebounds: *const DetectedFaceBound);
}
impl IMFFaceDetectionTransformCallback_Vtbl {
    pub const fn new<Identity: IMFFaceDetectionTransformCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnFaceDetectionResult<Identity: IMFFaceDetectionTransformCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, countofbounds: u32, detectedfacebounds: *const DetectedFaceBound) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFFaceDetectionTransformCallback_Impl::OnFaceDetectionResult(this, core::mem::transmute_copy(&countofbounds), core::mem::transmute_copy(&detectedfacebounds));
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnFaceDetectionResult: OnFaceDetectionResult::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFFaceDetectionTransformCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFFaceDetectionTransformCallback {}
windows_core::imp::define_interface!(IMFFieldOfUseMFTUnlock, IMFFieldOfUseMFTUnlock_Vtbl, 0x508e71d3_ec66_4fc3_8775_b4b9ed6ba847);
windows_core::imp::interface_hierarchy!(IMFFieldOfUseMFTUnlock, windows_core::IUnknown);
impl IMFFieldOfUseMFTUnlock {
    pub unsafe fn Unlock<P0>(&self, punkmft: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).Unlock)(windows_core::Interface::as_raw(self), punkmft.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFFieldOfUseMFTUnlock_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Unlock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFFieldOfUseMFTUnlock_Impl: windows_core::IUnknownImpl {
    fn Unlock(&self, punkmft: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl IMFFieldOfUseMFTUnlock_Vtbl {
    pub const fn new<Identity: IMFFieldOfUseMFTUnlock_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Unlock<Identity: IMFFieldOfUseMFTUnlock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkmft: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFFieldOfUseMFTUnlock_Impl::Unlock(this, core::mem::transmute_copy(&punkmft)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Unlock: Unlock::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFFieldOfUseMFTUnlock as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFFieldOfUseMFTUnlock {}
windows_core::imp::define_interface!(IMFFinalizableMediaSink, IMFFinalizableMediaSink_Vtbl, 0xeaecb74a_9a50_42ce_9541_6a7f57aa4ad7);
impl core::ops::Deref for IMFFinalizableMediaSink {
    type Target = IMFMediaSink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMFFinalizableMediaSink, windows_core::IUnknown, IMFMediaSink);
impl IMFFinalizableMediaSink {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn BeginFinalize<P0, P1>(&self, pcallback: P0, punkstate: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMFAsyncCallback>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).BeginFinalize)(windows_core::Interface::as_raw(self), pcallback.param().abi(), punkstate.param().abi()) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn EndFinalize<P0>(&self, presult: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMFAsyncResult>,
    {
        unsafe { (windows_core::Interface::vtable(self).EndFinalize)(windows_core::Interface::as_raw(self), presult.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFFinalizableMediaSink_Vtbl {
    pub base__: IMFMediaSink_Vtbl,
    #[cfg(feature = "mfobjects")]
    pub BeginFinalize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    BeginFinalize: usize,
    #[cfg(feature = "mfobjects")]
    pub EndFinalize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    EndFinalize: usize,
}
#[cfg(feature = "mfobjects")]
pub trait IMFFinalizableMediaSink_Impl: IMFMediaSink_Impl {
    fn BeginFinalize(&self, pcallback: windows_core::Ref<super::IMFAsyncCallback>, punkstate: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn EndFinalize(&self, presult: windows_core::Ref<super::IMFAsyncResult>) -> windows_core::Result<()>;
}
#[cfg(feature = "mfobjects")]
impl IMFFinalizableMediaSink_Vtbl {
    pub const fn new<Identity: IMFFinalizableMediaSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BeginFinalize<Identity: IMFFinalizableMediaSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void, punkstate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFFinalizableMediaSink_Impl::BeginFinalize(this, core::mem::transmute_copy(&pcallback), core::mem::transmute_copy(&punkstate)).into()
            }
        }
        unsafe extern "system" fn EndFinalize<Identity: IMFFinalizableMediaSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presult: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFFinalizableMediaSink_Impl::EndFinalize(this, core::mem::transmute_copy(&presult)).into()
            }
        }
        Self {
            base__: IMFMediaSink_Vtbl::new::<Identity, OFFSET>(),
            BeginFinalize: BeginFinalize::<Identity, OFFSET>,
            EndFinalize: EndFinalize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFFinalizableMediaSink as windows_core::Interface>::IID || iid == &<IMFMediaSink as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFFinalizableMediaSink {}
windows_core::imp::define_interface!(IMFGetService, IMFGetService_Vtbl, 0xfa993888_4383_415a_a930_dd472a8cf6f7);
windows_core::imp::interface_hierarchy!(IMFGetService, windows_core::IUnknown);
impl IMFGetService {
    pub unsafe fn GetService<T>(&self, guidservice: *const windows_core::GUID) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetService)(windows_core::Interface::as_raw(self), guidservice, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFGetService_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetService: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFGetService_Impl: windows_core::IUnknownImpl {
    fn GetService(&self, guidservice: *const windows_core::GUID, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IMFGetService_Vtbl {
    pub const fn new<Identity: IMFGetService_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetService<Identity: IMFGetService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidservice: *const windows_core::GUID, riid: *const windows_core::GUID, ppvobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFGetService_Impl::GetService(this, core::mem::transmute_copy(&guidservice), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppvobject)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetService: GetService::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFGetService as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFGetService {}
windows_core::imp::define_interface!(IMFHttpDownloadRequest, IMFHttpDownloadRequest_Vtbl, 0xf779fddf_26e7_4270_8a8b_b983d1859de0);
windows_core::imp::interface_hierarchy!(IMFHttpDownloadRequest, windows_core::IUnknown);
impl IMFHttpDownloadRequest {
    pub unsafe fn AddHeader<P0>(&self, szheader: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddHeader)(windows_core::Interface::as_raw(self), szheader.param().abi()) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn BeginSendRequest<P2, P3>(&self, pbpayload: Option<&[u8]>, pcallback: P2, punkstate: P3) -> windows_core::HRESULT
    where
        P2: windows_core::Param<super::IMFAsyncCallback>,
        P3: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).BeginSendRequest)(windows_core::Interface::as_raw(self), pbpayload.map_or(core::ptr::null(), |slice| slice.as_ptr()), pbpayload.map_or(0, |slice| slice.len().try_into().unwrap()), pcallback.param().abi(), punkstate.param().abi()) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn EndSendRequest<P0>(&self, presult: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMFAsyncResult>,
    {
        unsafe { (windows_core::Interface::vtable(self).EndSendRequest)(windows_core::Interface::as_raw(self), presult.param().abi()) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn BeginReceiveResponse<P0, P1>(&self, pcallback: P0, punkstate: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMFAsyncCallback>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).BeginReceiveResponse)(windows_core::Interface::as_raw(self), pcallback.param().abi(), punkstate.param().abi()) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn EndReceiveResponse<P0>(&self, presult: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMFAsyncResult>,
    {
        unsafe { (windows_core::Interface::vtable(self).EndReceiveResponse)(windows_core::Interface::as_raw(self), presult.param().abi()) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn BeginReadPayload<P2, P3>(&self, pb: &mut [u8], pcallback: P2, punkstate: P3) -> windows_core::HRESULT
    where
        P2: windows_core::Param<super::IMFAsyncCallback>,
        P3: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).BeginReadPayload)(windows_core::Interface::as_raw(self), pb.as_mut_ptr(), pb.len().try_into().unwrap(), pcallback.param().abi(), punkstate.param().abi()) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn EndReadPayload<P0>(&self, presult: P0, pqwoffset: *mut u64, pcbread: *mut u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMFAsyncResult>,
    {
        unsafe { (windows_core::Interface::vtable(self).EndReadPayload)(windows_core::Interface::as_raw(self), presult.param().abi(), pqwoffset as _, pcbread as _) }
    }
    pub unsafe fn QueryHeader<P0>(&self, szheadername: P0, dwindex: u32) -> windows_core::Result<windows_core::PWSTR>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).QueryHeader)(windows_core::Interface::as_raw(self), szheadername.param().abi(), dwindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetURL(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetURL)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn HasNullSourceOrigin(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HasNullSourceOrigin)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetTimeSeekResult(&self, pqwstarttime: *mut u64, pqwstoptime: *mut u64, pqwduration: *mut u64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTimeSeekResult)(windows_core::Interface::as_raw(self), pqwstarttime as _, pqwstoptime as _, pqwduration as _) }
    }
    pub unsafe fn GetHttpStatus(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHttpStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAtEndOfPayload(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAtEndOfPayload)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetTotalLength(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTotalLength)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetRangeEndOffset(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRangeEndOffset)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Close(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFHttpDownloadRequest_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddHeader: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "mfobjects")]
    pub BeginSendRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    BeginSendRequest: usize,
    #[cfg(feature = "mfobjects")]
    pub EndSendRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    EndSendRequest: usize,
    #[cfg(feature = "mfobjects")]
    pub BeginReceiveResponse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    BeginReceiveResponse: usize,
    #[cfg(feature = "mfobjects")]
    pub EndReceiveResponse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    EndReceiveResponse: usize,
    #[cfg(feature = "mfobjects")]
    pub BeginReadPayload: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, u32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    BeginReadPayload: usize,
    #[cfg(feature = "mfobjects")]
    pub EndReadPayload: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u64, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    EndReadPayload: usize,
    pub QueryHeader: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub GetURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub HasNullSourceOrigin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetTimeSeekResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64, *mut u64, *mut u64) -> windows_core::HRESULT,
    pub GetHttpStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAtEndOfPayload: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetTotalLength: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub GetRangeEndOffset: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "mfobjects")]
pub trait IMFHttpDownloadRequest_Impl: windows_core::IUnknownImpl {
    fn AddHeader(&self, szheader: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn BeginSendRequest(&self, pbpayload: *const u8, cbpayload: u32, pcallback: windows_core::Ref<super::IMFAsyncCallback>, punkstate: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn EndSendRequest(&self, presult: windows_core::Ref<super::IMFAsyncResult>) -> windows_core::Result<()>;
    fn BeginReceiveResponse(&self, pcallback: windows_core::Ref<super::IMFAsyncCallback>, punkstate: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn EndReceiveResponse(&self, presult: windows_core::Ref<super::IMFAsyncResult>) -> windows_core::Result<()>;
    fn BeginReadPayload(&self, pb: *mut u8, cb: u32, pcallback: windows_core::Ref<super::IMFAsyncCallback>, punkstate: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn EndReadPayload(&self, presult: windows_core::Ref<super::IMFAsyncResult>, pqwoffset: *mut u64, pcbread: *mut u32) -> windows_core::Result<()>;
    fn QueryHeader(&self, szheadername: &windows_core::PCWSTR, dwindex: u32) -> windows_core::Result<windows_core::PWSTR>;
    fn GetURL(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn HasNullSourceOrigin(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetTimeSeekResult(&self, pqwstarttime: *mut u64, pqwstoptime: *mut u64, pqwduration: *mut u64) -> windows_core::Result<()>;
    fn GetHttpStatus(&self) -> windows_core::Result<u32>;
    fn GetAtEndOfPayload(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetTotalLength(&self) -> windows_core::Result<u64>;
    fn GetRangeEndOffset(&self) -> windows_core::Result<u64>;
    fn Close(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "mfobjects")]
impl IMFHttpDownloadRequest_Vtbl {
    pub const fn new<Identity: IMFHttpDownloadRequest_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddHeader<Identity: IMFHttpDownloadRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szheader: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFHttpDownloadRequest_Impl::AddHeader(this, core::mem::transmute(&szheader)).into()
            }
        }
        unsafe extern "system" fn BeginSendRequest<Identity: IMFHttpDownloadRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbpayload: *const u8, cbpayload: u32, pcallback: *mut core::ffi::c_void, punkstate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFHttpDownloadRequest_Impl::BeginSendRequest(this, core::mem::transmute_copy(&pbpayload), core::mem::transmute_copy(&cbpayload), core::mem::transmute_copy(&pcallback), core::mem::transmute_copy(&punkstate)).into()
            }
        }
        unsafe extern "system" fn EndSendRequest<Identity: IMFHttpDownloadRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presult: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFHttpDownloadRequest_Impl::EndSendRequest(this, core::mem::transmute_copy(&presult)).into()
            }
        }
        unsafe extern "system" fn BeginReceiveResponse<Identity: IMFHttpDownloadRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void, punkstate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFHttpDownloadRequest_Impl::BeginReceiveResponse(this, core::mem::transmute_copy(&pcallback), core::mem::transmute_copy(&punkstate)).into()
            }
        }
        unsafe extern "system" fn EndReceiveResponse<Identity: IMFHttpDownloadRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presult: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFHttpDownloadRequest_Impl::EndReceiveResponse(this, core::mem::transmute_copy(&presult)).into()
            }
        }
        unsafe extern "system" fn BeginReadPayload<Identity: IMFHttpDownloadRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pb: *mut u8, cb: u32, pcallback: *mut core::ffi::c_void, punkstate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFHttpDownloadRequest_Impl::BeginReadPayload(this, core::mem::transmute_copy(&pb), core::mem::transmute_copy(&cb), core::mem::transmute_copy(&pcallback), core::mem::transmute_copy(&punkstate)).into()
            }
        }
        unsafe extern "system" fn EndReadPayload<Identity: IMFHttpDownloadRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presult: *mut core::ffi::c_void, pqwoffset: *mut u64, pcbread: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFHttpDownloadRequest_Impl::EndReadPayload(this, core::mem::transmute_copy(&presult), core::mem::transmute_copy(&pqwoffset), core::mem::transmute_copy(&pcbread)).into()
            }
        }
        unsafe extern "system" fn QueryHeader<Identity: IMFHttpDownloadRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szheadername: windows_core::PCWSTR, dwindex: u32, ppszheadervalue: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFHttpDownloadRequest_Impl::QueryHeader(this, core::mem::transmute(&szheadername), core::mem::transmute_copy(&dwindex)) {
                    Ok(ok__) => {
                        ppszheadervalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetURL<Identity: IMFHttpDownloadRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppszurl: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFHttpDownloadRequest_Impl::GetURL(this) {
                    Ok(ok__) => {
                        ppszurl.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HasNullSourceOrigin<Identity: IMFHttpDownloadRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfnullsourceorigin: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFHttpDownloadRequest_Impl::HasNullSourceOrigin(this) {
                    Ok(ok__) => {
                        pfnullsourceorigin.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTimeSeekResult<Identity: IMFHttpDownloadRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqwstarttime: *mut u64, pqwstoptime: *mut u64, pqwduration: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFHttpDownloadRequest_Impl::GetTimeSeekResult(this, core::mem::transmute_copy(&pqwstarttime), core::mem::transmute_copy(&pqwstoptime), core::mem::transmute_copy(&pqwduration)).into()
            }
        }
        unsafe extern "system" fn GetHttpStatus<Identity: IMFHttpDownloadRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwhttpstatus: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFHttpDownloadRequest_Impl::GetHttpStatus(this) {
                    Ok(ok__) => {
                        pdwhttpstatus.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAtEndOfPayload<Identity: IMFHttpDownloadRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfatendofpayload: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFHttpDownloadRequest_Impl::GetAtEndOfPayload(this) {
                    Ok(ok__) => {
                        pfatendofpayload.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTotalLength<Identity: IMFHttpDownloadRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqwtotallength: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFHttpDownloadRequest_Impl::GetTotalLength(this) {
                    Ok(ok__) => {
                        pqwtotallength.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRangeEndOffset<Identity: IMFHttpDownloadRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pqwrangeend: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFHttpDownloadRequest_Impl::GetRangeEndOffset(this) {
                    Ok(ok__) => {
                        pqwrangeend.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Close<Identity: IMFHttpDownloadRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFHttpDownloadRequest_Impl::Close(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddHeader: AddHeader::<Identity, OFFSET>,
            BeginSendRequest: BeginSendRequest::<Identity, OFFSET>,
            EndSendRequest: EndSendRequest::<Identity, OFFSET>,
            BeginReceiveResponse: BeginReceiveResponse::<Identity, OFFSET>,
            EndReceiveResponse: EndReceiveResponse::<Identity, OFFSET>,
            BeginReadPayload: BeginReadPayload::<Identity, OFFSET>,
            EndReadPayload: EndReadPayload::<Identity, OFFSET>,
            QueryHeader: QueryHeader::<Identity, OFFSET>,
            GetURL: GetURL::<Identity, OFFSET>,
            HasNullSourceOrigin: HasNullSourceOrigin::<Identity, OFFSET>,
            GetTimeSeekResult: GetTimeSeekResult::<Identity, OFFSET>,
            GetHttpStatus: GetHttpStatus::<Identity, OFFSET>,
            GetAtEndOfPayload: GetAtEndOfPayload::<Identity, OFFSET>,
            GetTotalLength: GetTotalLength::<Identity, OFFSET>,
            GetRangeEndOffset: GetRangeEndOffset::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFHttpDownloadRequest as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFHttpDownloadRequest {}
windows_core::imp::define_interface!(IMFHttpDownloadSession, IMFHttpDownloadSession_Vtbl, 0x71fa9a2c_53ce_4662_a132_1a7e8cbf62db);
windows_core::imp::interface_hierarchy!(IMFHttpDownloadSession, windows_core::IUnknown);
impl IMFHttpDownloadSession {
    pub unsafe fn SetServer<P0>(&self, szservername: P0, nport: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetServer)(windows_core::Interface::as_raw(self), szservername.param().abi(), nport) }
    }
    pub unsafe fn CreateRequest<P0, P3, P4>(&self, szobjectname: P0, fbypassproxycache: bool, fsecure: bool, szverb: P3, szreferrer: P4) -> windows_core::Result<IMFHttpDownloadRequest>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateRequest)(windows_core::Interface::as_raw(self), szobjectname.param().abi(), fbypassproxycache.into(), fsecure.into(), szverb.param().abi(), szreferrer.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Close(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFHttpDownloadSession_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetServer: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub CreateRequest: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::BOOL, windows_core::BOOL, windows_core::PCWSTR, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFHttpDownloadSession_Impl: windows_core::IUnknownImpl {
    fn SetServer(&self, szservername: &windows_core::PCWSTR, nport: u32) -> windows_core::Result<()>;
    fn CreateRequest(&self, szobjectname: &windows_core::PCWSTR, fbypassproxycache: windows_core::BOOL, fsecure: windows_core::BOOL, szverb: &windows_core::PCWSTR, szreferrer: &windows_core::PCWSTR) -> windows_core::Result<IMFHttpDownloadRequest>;
    fn Close(&self) -> windows_core::Result<()>;
}
impl IMFHttpDownloadSession_Vtbl {
    pub const fn new<Identity: IMFHttpDownloadSession_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetServer<Identity: IMFHttpDownloadSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szservername: windows_core::PCWSTR, nport: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFHttpDownloadSession_Impl::SetServer(this, core::mem::transmute(&szservername), core::mem::transmute_copy(&nport)).into()
            }
        }
        unsafe extern "system" fn CreateRequest<Identity: IMFHttpDownloadSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szobjectname: windows_core::PCWSTR, fbypassproxycache: windows_core::BOOL, fsecure: windows_core::BOOL, szverb: windows_core::PCWSTR, szreferrer: windows_core::PCWSTR, pprequest: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFHttpDownloadSession_Impl::CreateRequest(this, core::mem::transmute(&szobjectname), core::mem::transmute_copy(&fbypassproxycache), core::mem::transmute_copy(&fsecure), core::mem::transmute(&szverb), core::mem::transmute(&szreferrer)) {
                    Ok(ok__) => {
                        pprequest.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Close<Identity: IMFHttpDownloadSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFHttpDownloadSession_Impl::Close(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetServer: SetServer::<Identity, OFFSET>,
            CreateRequest: CreateRequest::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFHttpDownloadSession as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFHttpDownloadSession {}
windows_core::imp::define_interface!(IMFHttpDownloadSessionProvider, IMFHttpDownloadSessionProvider_Vtbl, 0x1b4cf4b9_3a16_4115_839d_03cc5c99df01);
windows_core::imp::interface_hierarchy!(IMFHttpDownloadSessionProvider, windows_core::IUnknown);
impl IMFHttpDownloadSessionProvider {
    pub unsafe fn CreateHttpDownloadSession<P0>(&self, wszscheme: P0) -> windows_core::Result<IMFHttpDownloadSession>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateHttpDownloadSession)(windows_core::Interface::as_raw(self), wszscheme.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFHttpDownloadSessionProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateHttpDownloadSession: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFHttpDownloadSessionProvider_Impl: windows_core::IUnknownImpl {
    fn CreateHttpDownloadSession(&self, wszscheme: &windows_core::PCWSTR) -> windows_core::Result<IMFHttpDownloadSession>;
}
impl IMFHttpDownloadSessionProvider_Vtbl {
    pub const fn new<Identity: IMFHttpDownloadSessionProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateHttpDownloadSession<Identity: IMFHttpDownloadSessionProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszscheme: windows_core::PCWSTR, ppdownloadsession: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFHttpDownloadSessionProvider_Impl::CreateHttpDownloadSession(this, core::mem::transmute(&wszscheme)) {
                    Ok(ok__) => {
                        ppdownloadsession.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateHttpDownloadSession: CreateHttpDownloadSession::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFHttpDownloadSessionProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFHttpDownloadSessionProvider {}
windows_core::imp::define_interface!(IMFInputTrustAuthority, IMFInputTrustAuthority_Vtbl, 0xd19f8e98_b126_4446_890c_5dcb7ad71453);
windows_core::imp::interface_hierarchy!(IMFInputTrustAuthority, windows_core::IUnknown);
impl IMFInputTrustAuthority {
    pub unsafe fn GetDecrypter<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetDecrypter)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn RequestAccess(&self, action: MFPOLICYMANAGER_ACTION) -> windows_core::Result<super::IMFActivate> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RequestAccess)(windows_core::Interface::as_raw(self), action, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn GetPolicy(&self, action: MFPOLICYMANAGER_ACTION) -> windows_core::Result<IMFOutputPolicy> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPolicy)(windows_core::Interface::as_raw(self), action, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn BindAccess(&self, pparam: *const MFINPUTTRUSTAUTHORITY_ACCESS_PARAMS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).BindAccess)(windows_core::Interface::as_raw(self), pparam) }
    }
    pub unsafe fn UpdateAccess(&self, pparam: *const MFINPUTTRUSTAUTHORITY_ACCESS_PARAMS) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UpdateAccess)(windows_core::Interface::as_raw(self), pparam) }
    }
    pub unsafe fn Reset(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Reset)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFInputTrustAuthority_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDecrypter: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "mfobjects")]
    pub RequestAccess: unsafe extern "system" fn(*mut core::ffi::c_void, MFPOLICYMANAGER_ACTION, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    RequestAccess: usize,
    #[cfg(feature = "mfobjects")]
    pub GetPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, MFPOLICYMANAGER_ACTION, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    GetPolicy: usize,
    pub BindAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *const MFINPUTTRUSTAUTHORITY_ACCESS_PARAMS) -> windows_core::HRESULT,
    pub UpdateAccess: unsafe extern "system" fn(*mut core::ffi::c_void, *const MFINPUTTRUSTAUTHORITY_ACCESS_PARAMS) -> windows_core::HRESULT,
    pub Reset: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "mfobjects")]
pub trait IMFInputTrustAuthority_Impl: windows_core::IUnknownImpl {
    fn GetDecrypter(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn RequestAccess(&self, action: MFPOLICYMANAGER_ACTION) -> windows_core::Result<super::IMFActivate>;
    fn GetPolicy(&self, action: MFPOLICYMANAGER_ACTION) -> windows_core::Result<IMFOutputPolicy>;
    fn BindAccess(&self, pparam: *const MFINPUTTRUSTAUTHORITY_ACCESS_PARAMS) -> windows_core::Result<()>;
    fn UpdateAccess(&self, pparam: *const MFINPUTTRUSTAUTHORITY_ACCESS_PARAMS) -> windows_core::Result<()>;
    fn Reset(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "mfobjects")]
impl IMFInputTrustAuthority_Vtbl {
    pub const fn new<Identity: IMFInputTrustAuthority_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDecrypter<Identity: IMFInputTrustAuthority_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFInputTrustAuthority_Impl::GetDecrypter(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn RequestAccess<Identity: IMFInputTrustAuthority_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, action: MFPOLICYMANAGER_ACTION, ppcontentenableractivate: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFInputTrustAuthority_Impl::RequestAccess(this, core::mem::transmute_copy(&action)) {
                    Ok(ok__) => {
                        ppcontentenableractivate.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPolicy<Identity: IMFInputTrustAuthority_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, action: MFPOLICYMANAGER_ACTION, pppolicy: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFInputTrustAuthority_Impl::GetPolicy(this, core::mem::transmute_copy(&action)) {
                    Ok(ok__) => {
                        pppolicy.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BindAccess<Identity: IMFInputTrustAuthority_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparam: *const MFINPUTTRUSTAUTHORITY_ACCESS_PARAMS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFInputTrustAuthority_Impl::BindAccess(this, core::mem::transmute_copy(&pparam)).into()
            }
        }
        unsafe extern "system" fn UpdateAccess<Identity: IMFInputTrustAuthority_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparam: *const MFINPUTTRUSTAUTHORITY_ACCESS_PARAMS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFInputTrustAuthority_Impl::UpdateAccess(this, core::mem::transmute_copy(&pparam)).into()
            }
        }
        unsafe extern "system" fn Reset<Identity: IMFInputTrustAuthority_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFInputTrustAuthority_Impl::Reset(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDecrypter: GetDecrypter::<Identity, OFFSET>,
            RequestAccess: RequestAccess::<Identity, OFFSET>,
            GetPolicy: GetPolicy::<Identity, OFFSET>,
            BindAccess: BindAccess::<Identity, OFFSET>,
            UpdateAccess: UpdateAccess::<Identity, OFFSET>,
            Reset: Reset::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFInputTrustAuthority as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFInputTrustAuthority {}
windows_core::imp::define_interface!(IMFLocalMFTRegistration, IMFLocalMFTRegistration_Vtbl, 0x149c4d73_b4be_4f8d_8b87_079e926b6add);
windows_core::imp::interface_hierarchy!(IMFLocalMFTRegistration, windows_core::IUnknown);
impl IMFLocalMFTRegistration {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn RegisterMFTs(&self, pmfts: *const MFT_REGISTRATION_INFO, cmfts: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RegisterMFTs)(windows_core::Interface::as_raw(self), pmfts, cmfts) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFLocalMFTRegistration_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "mfobjects")]
    pub RegisterMFTs: unsafe extern "system" fn(*mut core::ffi::c_void, *const MFT_REGISTRATION_INFO, u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    RegisterMFTs: usize,
}
#[cfg(feature = "mfobjects")]
pub trait IMFLocalMFTRegistration_Impl: windows_core::IUnknownImpl {
    fn RegisterMFTs(&self, pmfts: *const MFT_REGISTRATION_INFO, cmfts: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "mfobjects")]
impl IMFLocalMFTRegistration_Vtbl {
    pub const fn new<Identity: IMFLocalMFTRegistration_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterMFTs<Identity: IMFLocalMFTRegistration_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmfts: *const MFT_REGISTRATION_INFO, cmfts: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFLocalMFTRegistration_Impl::RegisterMFTs(this, core::mem::transmute_copy(&pmfts), core::mem::transmute_copy(&cmfts)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), RegisterMFTs: RegisterMFTs::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFLocalMFTRegistration as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFLocalMFTRegistration {}
#[cfg(feature = "mfobjects")]
windows_core::imp::define_interface!(IMFMediaSession, IMFMediaSession_Vtbl, 0x90377834_21d0_4dee_8214_ba2e3e6c1127);
#[cfg(feature = "mfobjects")]
impl core::ops::Deref for IMFMediaSession {
    type Target = super::IMFMediaEventGenerator;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "mfobjects")]
windows_core::imp::interface_hierarchy!(IMFMediaSession, windows_core::IUnknown, super::IMFMediaEventGenerator);
#[cfg(feature = "mfobjects")]
impl IMFMediaSession {
    pub unsafe fn SetTopology<P1>(&self, dwsettopologyflags: u32, ptopology: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IMFTopology>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTopology)(windows_core::Interface::as_raw(self), dwsettopologyflags, ptopology.param().abi()) }
    }
    pub unsafe fn ClearTopologies(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ClearTopologies)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Start(&self, pguidtimeformat: *const windows_core::GUID, pvarstartposition: *const super::PROPVARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self), pguidtimeformat, pvarstartposition) }
    }
    pub unsafe fn Pause(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Stop(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Close(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Shutdown(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Shutdown)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetClock(&self) -> windows_core::Result<IMFClock> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetClock)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSessionCapabilities(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSessionCapabilities)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetFullTopology(&self, dwgetfulltopologyflags: u32, topoid: TOPOID) -> windows_core::Result<IMFTopology> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFullTopology)(windows_core::Interface::as_raw(self), dwgetfulltopologyflags, topoid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "mfobjects")]
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaSession_Vtbl {
    pub base__: super::IMFMediaEventGenerator_Vtbl,
    pub SetTopology: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClearTopologies: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const super::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    Start: usize,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetClock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSessionCapabilities: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetFullTopology: unsafe extern "system" fn(*mut core::ffi::c_void, u32, TOPOID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMFMediaSession_Impl: super::IMFMediaEventGenerator_Impl {
    fn SetTopology(&self, dwsettopologyflags: u32, ptopology: windows_core::Ref<IMFTopology>) -> windows_core::Result<()>;
    fn ClearTopologies(&self) -> windows_core::Result<()>;
    fn Start(&self, pguidtimeformat: *const windows_core::GUID, pvarstartposition: *const super::PROPVARIANT) -> windows_core::Result<()>;
    fn Pause(&self) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
    fn Shutdown(&self) -> windows_core::Result<()>;
    fn GetClock(&self) -> windows_core::Result<IMFClock>;
    fn GetSessionCapabilities(&self) -> windows_core::Result<u32>;
    fn GetFullTopology(&self, dwgetfulltopologyflags: u32, topoid: TOPOID) -> windows_core::Result<IMFTopology>;
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl IMFMediaSession_Vtbl {
    pub const fn new<Identity: IMFMediaSession_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetTopology<Identity: IMFMediaSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsettopologyflags: u32, ptopology: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaSession_Impl::SetTopology(this, core::mem::transmute_copy(&dwsettopologyflags), core::mem::transmute_copy(&ptopology)).into()
            }
        }
        unsafe extern "system" fn ClearTopologies<Identity: IMFMediaSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaSession_Impl::ClearTopologies(this).into()
            }
        }
        unsafe extern "system" fn Start<Identity: IMFMediaSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidtimeformat: *const windows_core::GUID, pvarstartposition: *const super::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaSession_Impl::Start(this, core::mem::transmute_copy(&pguidtimeformat), core::mem::transmute_copy(&pvarstartposition)).into()
            }
        }
        unsafe extern "system" fn Pause<Identity: IMFMediaSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaSession_Impl::Pause(this).into()
            }
        }
        unsafe extern "system" fn Stop<Identity: IMFMediaSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaSession_Impl::Stop(this).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IMFMediaSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaSession_Impl::Close(this).into()
            }
        }
        unsafe extern "system" fn Shutdown<Identity: IMFMediaSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaSession_Impl::Shutdown(this).into()
            }
        }
        unsafe extern "system" fn GetClock<Identity: IMFMediaSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppclock: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaSession_Impl::GetClock(this) {
                    Ok(ok__) => {
                        ppclock.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSessionCapabilities<Identity: IMFMediaSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcaps: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaSession_Impl::GetSessionCapabilities(this) {
                    Ok(ok__) => {
                        pdwcaps.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFullTopology<Identity: IMFMediaSession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwgetfulltopologyflags: u32, topoid: TOPOID, ppfulltopology: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaSession_Impl::GetFullTopology(this, core::mem::transmute_copy(&dwgetfulltopologyflags), core::mem::transmute_copy(&topoid)) {
                    Ok(ok__) => {
                        ppfulltopology.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IMFMediaEventGenerator_Vtbl::new::<Identity, OFFSET>(),
            SetTopology: SetTopology::<Identity, OFFSET>,
            ClearTopologies: ClearTopologies::<Identity, OFFSET>,
            Start: Start::<Identity, OFFSET>,
            Pause: Pause::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            Shutdown: Shutdown::<Identity, OFFSET>,
            GetClock: GetClock::<Identity, OFFSET>,
            GetSessionCapabilities: GetSessionCapabilities::<Identity, OFFSET>,
            GetFullTopology: GetFullTopology::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaSession as windows_core::Interface>::IID || iid == &<super::IMFMediaEventGenerator as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMFMediaSession {}
windows_core::imp::define_interface!(IMFMediaSink, IMFMediaSink_Vtbl, 0x6ef2a660_47c0_4666_b13d_cbb717f2fa2c);
windows_core::imp::interface_hierarchy!(IMFMediaSink, windows_core::IUnknown);
impl IMFMediaSink {
    pub unsafe fn GetCharacteristics(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCharacteristics)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn AddStreamSink<P1>(&self, dwstreamsinkidentifier: u32, pmediatype: P1) -> windows_core::Result<IMFStreamSink>
    where
        P1: windows_core::Param<super::IMFMediaType>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddStreamSink)(windows_core::Interface::as_raw(self), dwstreamsinkidentifier, pmediatype.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RemoveStreamSink(&self, dwstreamsinkidentifier: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveStreamSink)(windows_core::Interface::as_raw(self), dwstreamsinkidentifier) }
    }
    pub unsafe fn GetStreamSinkCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStreamSinkCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn GetStreamSinkByIndex(&self, dwindex: u32) -> windows_core::Result<IMFStreamSink> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStreamSinkByIndex)(windows_core::Interface::as_raw(self), dwindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn GetStreamSinkById(&self, dwstreamsinkidentifier: u32) -> windows_core::Result<IMFStreamSink> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStreamSinkById)(windows_core::Interface::as_raw(self), dwstreamsinkidentifier, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetPresentationClock<P0>(&self, ppresentationclock: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFPresentationClock>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPresentationClock)(windows_core::Interface::as_raw(self), ppresentationclock.param().abi()) }
    }
    pub unsafe fn GetPresentationClock(&self) -> windows_core::Result<IMFPresentationClock> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPresentationClock)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Shutdown(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Shutdown)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCharacteristics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "mfobjects")]
    pub AddStreamSink: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    AddStreamSink: usize,
    pub RemoveStreamSink: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetStreamSinkCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "mfobjects")]
    pub GetStreamSinkByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    GetStreamSinkByIndex: usize,
    #[cfg(feature = "mfobjects")]
    pub GetStreamSinkById: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    GetStreamSinkById: usize,
    pub SetPresentationClock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPresentationClock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "mfobjects")]
pub trait IMFMediaSink_Impl: windows_core::IUnknownImpl {
    fn GetCharacteristics(&self) -> windows_core::Result<u32>;
    fn AddStreamSink(&self, dwstreamsinkidentifier: u32, pmediatype: windows_core::Ref<super::IMFMediaType>) -> windows_core::Result<IMFStreamSink>;
    fn RemoveStreamSink(&self, dwstreamsinkidentifier: u32) -> windows_core::Result<()>;
    fn GetStreamSinkCount(&self) -> windows_core::Result<u32>;
    fn GetStreamSinkByIndex(&self, dwindex: u32) -> windows_core::Result<IMFStreamSink>;
    fn GetStreamSinkById(&self, dwstreamsinkidentifier: u32) -> windows_core::Result<IMFStreamSink>;
    fn SetPresentationClock(&self, ppresentationclock: windows_core::Ref<IMFPresentationClock>) -> windows_core::Result<()>;
    fn GetPresentationClock(&self) -> windows_core::Result<IMFPresentationClock>;
    fn Shutdown(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "mfobjects")]
impl IMFMediaSink_Vtbl {
    pub const fn new<Identity: IMFMediaSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCharacteristics<Identity: IMFMediaSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcharacteristics: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaSink_Impl::GetCharacteristics(this) {
                    Ok(ok__) => {
                        pdwcharacteristics.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddStreamSink<Identity: IMFMediaSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamsinkidentifier: u32, pmediatype: *mut core::ffi::c_void, ppstreamsink: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaSink_Impl::AddStreamSink(this, core::mem::transmute_copy(&dwstreamsinkidentifier), core::mem::transmute_copy(&pmediatype)) {
                    Ok(ok__) => {
                        ppstreamsink.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveStreamSink<Identity: IMFMediaSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamsinkidentifier: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaSink_Impl::RemoveStreamSink(this, core::mem::transmute_copy(&dwstreamsinkidentifier)).into()
            }
        }
        unsafe extern "system" fn GetStreamSinkCount<Identity: IMFMediaSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcstreamsinkcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaSink_Impl::GetStreamSinkCount(this) {
                    Ok(ok__) => {
                        pcstreamsinkcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStreamSinkByIndex<Identity: IMFMediaSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, ppstreamsink: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaSink_Impl::GetStreamSinkByIndex(this, core::mem::transmute_copy(&dwindex)) {
                    Ok(ok__) => {
                        ppstreamsink.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStreamSinkById<Identity: IMFMediaSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamsinkidentifier: u32, ppstreamsink: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaSink_Impl::GetStreamSinkById(this, core::mem::transmute_copy(&dwstreamsinkidentifier)) {
                    Ok(ok__) => {
                        ppstreamsink.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPresentationClock<Identity: IMFMediaSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresentationclock: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaSink_Impl::SetPresentationClock(this, core::mem::transmute_copy(&ppresentationclock)).into()
            }
        }
        unsafe extern "system" fn GetPresentationClock<Identity: IMFMediaSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppresentationclock: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaSink_Impl::GetPresentationClock(this) {
                    Ok(ok__) => {
                        pppresentationclock.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Shutdown<Identity: IMFMediaSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaSink_Impl::Shutdown(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCharacteristics: GetCharacteristics::<Identity, OFFSET>,
            AddStreamSink: AddStreamSink::<Identity, OFFSET>,
            RemoveStreamSink: RemoveStreamSink::<Identity, OFFSET>,
            GetStreamSinkCount: GetStreamSinkCount::<Identity, OFFSET>,
            GetStreamSinkByIndex: GetStreamSinkByIndex::<Identity, OFFSET>,
            GetStreamSinkById: GetStreamSinkById::<Identity, OFFSET>,
            SetPresentationClock: SetPresentationClock::<Identity, OFFSET>,
            GetPresentationClock: GetPresentationClock::<Identity, OFFSET>,
            Shutdown: Shutdown::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaSink as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFMediaSink {}
windows_core::imp::define_interface!(IMFMediaSinkPreroll, IMFMediaSinkPreroll_Vtbl, 0x5dfd4b2a_7674_4110_a4e6_8a68fd5f3688);
windows_core::imp::interface_hierarchy!(IMFMediaSinkPreroll, windows_core::IUnknown);
impl IMFMediaSinkPreroll {
    pub unsafe fn NotifyPreroll(&self, hnsupcomingstarttime: MFTIME) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).NotifyPreroll)(windows_core::Interface::as_raw(self), hnsupcomingstarttime) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaSinkPreroll_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub NotifyPreroll: unsafe extern "system" fn(*mut core::ffi::c_void, MFTIME) -> windows_core::HRESULT,
}
pub trait IMFMediaSinkPreroll_Impl: windows_core::IUnknownImpl {
    fn NotifyPreroll(&self, hnsupcomingstarttime: MFTIME) -> windows_core::Result<()>;
}
impl IMFMediaSinkPreroll_Vtbl {
    pub const fn new<Identity: IMFMediaSinkPreroll_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn NotifyPreroll<Identity: IMFMediaSinkPreroll_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hnsupcomingstarttime: MFTIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaSinkPreroll_Impl::NotifyPreroll(this, core::mem::transmute_copy(&hnsupcomingstarttime)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), NotifyPreroll: NotifyPreroll::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaSinkPreroll as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFMediaSinkPreroll {}
#[cfg(feature = "mfobjects")]
windows_core::imp::define_interface!(IMFMediaSource, IMFMediaSource_Vtbl, 0x279a808d_aec7_40c8_9c6b_a6b492c78a66);
#[cfg(feature = "mfobjects")]
impl core::ops::Deref for IMFMediaSource {
    type Target = super::IMFMediaEventGenerator;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "mfobjects")]
windows_core::imp::interface_hierarchy!(IMFMediaSource, windows_core::IUnknown, super::IMFMediaEventGenerator);
#[cfg(feature = "mfobjects")]
impl IMFMediaSource {
    pub unsafe fn GetCharacteristics(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCharacteristics)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CreatePresentationDescriptor(&self) -> windows_core::Result<IMFPresentationDescriptor> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePresentationDescriptor)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn Start<P0>(&self, ppresentationdescriptor: P0, pguidtimeformat: *const windows_core::GUID, pvarstartposition: *const super::PROPVARIANT) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFPresentationDescriptor>,
    {
        unsafe { (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self), ppresentationdescriptor.param().abi(), pguidtimeformat, pvarstartposition) }
    }
    pub unsafe fn Stop(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Pause(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Shutdown(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Shutdown)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "mfobjects")]
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaSource_Vtbl {
    pub base__: super::IMFMediaEventGenerator_Vtbl,
    pub GetCharacteristics: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub CreatePresentationDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const windows_core::GUID, *const super::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    Start: usize,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Shutdown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMFMediaSource_Impl: super::IMFMediaEventGenerator_Impl {
    fn GetCharacteristics(&self) -> windows_core::Result<u32>;
    fn CreatePresentationDescriptor(&self) -> windows_core::Result<IMFPresentationDescriptor>;
    fn Start(&self, ppresentationdescriptor: windows_core::Ref<IMFPresentationDescriptor>, pguidtimeformat: *const windows_core::GUID, pvarstartposition: *const super::PROPVARIANT) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn Pause(&self) -> windows_core::Result<()>;
    fn Shutdown(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl IMFMediaSource_Vtbl {
    pub const fn new<Identity: IMFMediaSource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCharacteristics<Identity: IMFMediaSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcharacteristics: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaSource_Impl::GetCharacteristics(this) {
                    Ok(ok__) => {
                        pdwcharacteristics.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreatePresentationDescriptor<Identity: IMFMediaSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppresentationdescriptor: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaSource_Impl::CreatePresentationDescriptor(this) {
                    Ok(ok__) => {
                        pppresentationdescriptor.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Start<Identity: IMFMediaSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresentationdescriptor: *mut core::ffi::c_void, pguidtimeformat: *const windows_core::GUID, pvarstartposition: *const super::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaSource_Impl::Start(this, core::mem::transmute_copy(&ppresentationdescriptor), core::mem::transmute_copy(&pguidtimeformat), core::mem::transmute_copy(&pvarstartposition)).into()
            }
        }
        unsafe extern "system" fn Stop<Identity: IMFMediaSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaSource_Impl::Stop(this).into()
            }
        }
        unsafe extern "system" fn Pause<Identity: IMFMediaSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaSource_Impl::Pause(this).into()
            }
        }
        unsafe extern "system" fn Shutdown<Identity: IMFMediaSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaSource_Impl::Shutdown(this).into()
            }
        }
        Self {
            base__: super::IMFMediaEventGenerator_Vtbl::new::<Identity, OFFSET>(),
            GetCharacteristics: GetCharacteristics::<Identity, OFFSET>,
            CreatePresentationDescriptor: CreatePresentationDescriptor::<Identity, OFFSET>,
            Start: Start::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            Pause: Pause::<Identity, OFFSET>,
            Shutdown: Shutdown::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaSource as windows_core::Interface>::IID || iid == &<super::IMFMediaEventGenerator as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMFMediaSource {}
#[cfg(feature = "mfobjects")]
windows_core::imp::define_interface!(IMFMediaSource2, IMFMediaSource2_Vtbl, 0xfbb03414_d13b_4786_8319_5ac51fc0a136);
#[cfg(feature = "mfobjects")]
impl core::ops::Deref for IMFMediaSource2 {
    type Target = IMFMediaSourceEx;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "mfobjects")]
windows_core::imp::interface_hierarchy!(IMFMediaSource2, windows_core::IUnknown, super::IMFMediaEventGenerator, IMFMediaSource, IMFMediaSourceEx);
#[cfg(feature = "mfobjects")]
impl IMFMediaSource2 {
    pub unsafe fn SetMediaType<P1>(&self, dwstreamid: u32, pmediatype: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IMFMediaType>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetMediaType)(windows_core::Interface::as_raw(self), dwstreamid, pmediatype.param().abi()) }
    }
}
#[cfg(feature = "mfobjects")]
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaSource2_Vtbl {
    pub base__: IMFMediaSourceEx_Vtbl,
    pub SetMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMFMediaSource2_Impl: IMFMediaSourceEx_Impl {
    fn SetMediaType(&self, dwstreamid: u32, pmediatype: windows_core::Ref<super::IMFMediaType>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl IMFMediaSource2_Vtbl {
    pub const fn new<Identity: IMFMediaSource2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetMediaType<Identity: IMFMediaSource2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamid: u32, pmediatype: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaSource2_Impl::SetMediaType(this, core::mem::transmute_copy(&dwstreamid), core::mem::transmute_copy(&pmediatype)).into()
            }
        }
        Self { base__: IMFMediaSourceEx_Vtbl::new::<Identity, OFFSET>(), SetMediaType: SetMediaType::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaSource2 as windows_core::Interface>::IID || iid == &<super::IMFMediaEventGenerator as windows_core::Interface>::IID || iid == &<IMFMediaSource as windows_core::Interface>::IID || iid == &<IMFMediaSourceEx as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMFMediaSource2 {}
#[cfg(feature = "mfobjects")]
windows_core::imp::define_interface!(IMFMediaSourceEx, IMFMediaSourceEx_Vtbl, 0x3c9b2eb9_86d5_4514_a394_f56664f9f0d8);
#[cfg(feature = "mfobjects")]
impl core::ops::Deref for IMFMediaSourceEx {
    type Target = IMFMediaSource;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "mfobjects")]
windows_core::imp::interface_hierarchy!(IMFMediaSourceEx, windows_core::IUnknown, super::IMFMediaEventGenerator, IMFMediaSource);
#[cfg(feature = "mfobjects")]
impl IMFMediaSourceEx {
    pub unsafe fn GetSourceAttributes(&self) -> windows_core::Result<super::IMFAttributes> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSourceAttributes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetStreamAttributes(&self, dwstreamidentifier: u32) -> windows_core::Result<super::IMFAttributes> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStreamAttributes)(windows_core::Interface::as_raw(self), dwstreamidentifier, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetD3DManager<P0>(&self, pmanager: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetD3DManager)(windows_core::Interface::as_raw(self), pmanager.param().abi()) }
    }
}
#[cfg(feature = "mfobjects")]
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaSourceEx_Vtbl {
    pub base__: IMFMediaSource_Vtbl,
    pub GetSourceAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStreamAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetD3DManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMFMediaSourceEx_Impl: IMFMediaSource_Impl {
    fn GetSourceAttributes(&self) -> windows_core::Result<super::IMFAttributes>;
    fn GetStreamAttributes(&self, dwstreamidentifier: u32) -> windows_core::Result<super::IMFAttributes>;
    fn SetD3DManager(&self, pmanager: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl IMFMediaSourceEx_Vtbl {
    pub const fn new<Identity: IMFMediaSourceEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSourceAttributes<Identity: IMFMediaSourceEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppattributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaSourceEx_Impl::GetSourceAttributes(this) {
                    Ok(ok__) => {
                        ppattributes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStreamAttributes<Identity: IMFMediaSourceEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamidentifier: u32, ppattributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaSourceEx_Impl::GetStreamAttributes(this, core::mem::transmute_copy(&dwstreamidentifier)) {
                    Ok(ok__) => {
                        ppattributes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetD3DManager<Identity: IMFMediaSourceEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmanager: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaSourceEx_Impl::SetD3DManager(this, core::mem::transmute_copy(&pmanager)).into()
            }
        }
        Self {
            base__: IMFMediaSource_Vtbl::new::<Identity, OFFSET>(),
            GetSourceAttributes: GetSourceAttributes::<Identity, OFFSET>,
            GetStreamAttributes: GetStreamAttributes::<Identity, OFFSET>,
            SetD3DManager: SetD3DManager::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaSourceEx as windows_core::Interface>::IID || iid == &<super::IMFMediaEventGenerator as windows_core::Interface>::IID || iid == &<IMFMediaSource as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMFMediaSourceEx {}
windows_core::imp::define_interface!(IMFMediaSourcePresentationProvider, IMFMediaSourcePresentationProvider_Vtbl, 0x0e1d600a_c9f3_442d_8c51_a42d2d49452f);
windows_core::imp::interface_hierarchy!(IMFMediaSourcePresentationProvider, windows_core::IUnknown);
impl IMFMediaSourcePresentationProvider {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn ForceEndOfPresentation<P0>(&self, ppresentationdescriptor: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFPresentationDescriptor>,
    {
        unsafe { (windows_core::Interface::vtable(self).ForceEndOfPresentation)(windows_core::Interface::as_raw(self), ppresentationdescriptor.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaSourcePresentationProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "mfobjects")]
    pub ForceEndOfPresentation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    ForceEndOfPresentation: usize,
}
#[cfg(feature = "mfobjects")]
pub trait IMFMediaSourcePresentationProvider_Impl: windows_core::IUnknownImpl {
    fn ForceEndOfPresentation(&self, ppresentationdescriptor: windows_core::Ref<IMFPresentationDescriptor>) -> windows_core::Result<()>;
}
#[cfg(feature = "mfobjects")]
impl IMFMediaSourcePresentationProvider_Vtbl {
    pub const fn new<Identity: IMFMediaSourcePresentationProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ForceEndOfPresentation<Identity: IMFMediaSourcePresentationProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresentationdescriptor: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaSourcePresentationProvider_Impl::ForceEndOfPresentation(this, core::mem::transmute_copy(&ppresentationdescriptor)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ForceEndOfPresentation: ForceEndOfPresentation::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaSourcePresentationProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFMediaSourcePresentationProvider {}
windows_core::imp::define_interface!(IMFMediaSourceTopologyProvider, IMFMediaSourceTopologyProvider_Vtbl, 0x0e1d6009_c9f3_442d_8c51_a42d2d49452f);
windows_core::imp::interface_hierarchy!(IMFMediaSourceTopologyProvider, windows_core::IUnknown);
impl IMFMediaSourceTopologyProvider {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn GetMediaSourceTopology<P0>(&self, ppresentationdescriptor: P0) -> windows_core::Result<IMFTopology>
    where
        P0: windows_core::Param<IMFPresentationDescriptor>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMediaSourceTopology)(windows_core::Interface::as_raw(self), ppresentationdescriptor.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaSourceTopologyProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "mfobjects")]
    pub GetMediaSourceTopology: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    GetMediaSourceTopology: usize,
}
#[cfg(feature = "mfobjects")]
pub trait IMFMediaSourceTopologyProvider_Impl: windows_core::IUnknownImpl {
    fn GetMediaSourceTopology(&self, ppresentationdescriptor: windows_core::Ref<IMFPresentationDescriptor>) -> windows_core::Result<IMFTopology>;
}
#[cfg(feature = "mfobjects")]
impl IMFMediaSourceTopologyProvider_Vtbl {
    pub const fn new<Identity: IMFMediaSourceTopologyProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMediaSourceTopology<Identity: IMFMediaSourceTopologyProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresentationdescriptor: *mut core::ffi::c_void, pptopology: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaSourceTopologyProvider_Impl::GetMediaSourceTopology(this, core::mem::transmute_copy(&ppresentationdescriptor)) {
                    Ok(ok__) => {
                        pptopology.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetMediaSourceTopology: GetMediaSourceTopology::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaSourceTopologyProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFMediaSourceTopologyProvider {}
#[cfg(feature = "mfobjects")]
windows_core::imp::define_interface!(IMFMediaStream, IMFMediaStream_Vtbl, 0xd182108f_4ec6_443f_aa42_a71106ec825f);
#[cfg(feature = "mfobjects")]
impl core::ops::Deref for IMFMediaStream {
    type Target = super::IMFMediaEventGenerator;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "mfobjects")]
windows_core::imp::interface_hierarchy!(IMFMediaStream, windows_core::IUnknown, super::IMFMediaEventGenerator);
#[cfg(feature = "mfobjects")]
impl IMFMediaStream {
    pub unsafe fn GetMediaSource(&self) -> windows_core::Result<IMFMediaSource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMediaSource)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetStreamDescriptor(&self) -> windows_core::Result<IMFStreamDescriptor> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStreamDescriptor)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RequestSample<P0>(&self, ptoken: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).RequestSample)(windows_core::Interface::as_raw(self), ptoken.param().abi()) }
    }
}
#[cfg(feature = "mfobjects")]
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaStream_Vtbl {
    pub base__: super::IMFMediaEventGenerator_Vtbl,
    pub GetMediaSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetStreamDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestSample: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMFMediaStream_Impl: super::IMFMediaEventGenerator_Impl {
    fn GetMediaSource(&self) -> windows_core::Result<IMFMediaSource>;
    fn GetStreamDescriptor(&self) -> windows_core::Result<IMFStreamDescriptor>;
    fn RequestSample(&self, ptoken: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl IMFMediaStream_Vtbl {
    pub const fn new<Identity: IMFMediaStream_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMediaSource<Identity: IMFMediaStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmediasource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaStream_Impl::GetMediaSource(this) {
                    Ok(ok__) => {
                        ppmediasource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStreamDescriptor<Identity: IMFMediaStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstreamdescriptor: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaStream_Impl::GetStreamDescriptor(this) {
                    Ok(ok__) => {
                        ppstreamdescriptor.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RequestSample<Identity: IMFMediaStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptoken: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaStream_Impl::RequestSample(this, core::mem::transmute_copy(&ptoken)).into()
            }
        }
        Self {
            base__: super::IMFMediaEventGenerator_Vtbl::new::<Identity, OFFSET>(),
            GetMediaSource: GetMediaSource::<Identity, OFFSET>,
            GetStreamDescriptor: GetStreamDescriptor::<Identity, OFFSET>,
            RequestSample: RequestSample::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaStream as windows_core::Interface>::IID || iid == &<super::IMFMediaEventGenerator as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMFMediaStream {}
#[cfg(feature = "mfobjects")]
windows_core::imp::define_interface!(IMFMediaStream2, IMFMediaStream2_Vtbl, 0xc5bc37d6_75c7_46a1_a132_81b5f723c20f);
#[cfg(feature = "mfobjects")]
impl core::ops::Deref for IMFMediaStream2 {
    type Target = IMFMediaStream;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "mfobjects")]
windows_core::imp::interface_hierarchy!(IMFMediaStream2, windows_core::IUnknown, super::IMFMediaEventGenerator, IMFMediaStream);
#[cfg(feature = "mfobjects")]
impl IMFMediaStream2 {
    pub unsafe fn SetStreamState(&self, value: super::MF_STREAM_STATE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetStreamState)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn GetStreamState(&self) -> windows_core::Result<super::MF_STREAM_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStreamState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "mfobjects")]
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaStream2_Vtbl {
    pub base__: IMFMediaStream_Vtbl,
    pub SetStreamState: unsafe extern "system" fn(*mut core::ffi::c_void, super::MF_STREAM_STATE) -> windows_core::HRESULT,
    pub GetStreamState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::MF_STREAM_STATE) -> windows_core::HRESULT,
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMFMediaStream2_Impl: IMFMediaStream_Impl {
    fn SetStreamState(&self, value: super::MF_STREAM_STATE) -> windows_core::Result<()>;
    fn GetStreamState(&self) -> windows_core::Result<super::MF_STREAM_STATE>;
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl IMFMediaStream2_Vtbl {
    pub const fn new<Identity: IMFMediaStream2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetStreamState<Identity: IMFMediaStream2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::MF_STREAM_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaStream2_Impl::SetStreamState(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn GetStreamState<Identity: IMFMediaStream2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::MF_STREAM_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaStream2_Impl::GetStreamState(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IMFMediaStream_Vtbl::new::<Identity, OFFSET>(),
            SetStreamState: SetStreamState::<Identity, OFFSET>,
            GetStreamState: GetStreamState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaStream2 as windows_core::Interface>::IID || iid == &<super::IMFMediaEventGenerator as windows_core::Interface>::IID || iid == &<IMFMediaStream as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMFMediaStream2 {}
windows_core::imp::define_interface!(IMFMediaStreamSourceSampleRequest, IMFMediaStreamSourceSampleRequest_Vtbl, 0x380b9af9_a85b_4e78_a2af_ea5ce645c6b4);
windows_core::imp::interface_hierarchy!(IMFMediaStreamSourceSampleRequest, windows_core::IUnknown);
impl IMFMediaStreamSourceSampleRequest {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn SetSample<P0>(&self, value: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMFSample>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSample)(windows_core::Interface::as_raw(self), value.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaStreamSourceSampleRequest_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "mfobjects")]
    pub SetSample: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    SetSample: usize,
}
#[cfg(feature = "mfobjects")]
pub trait IMFMediaStreamSourceSampleRequest_Impl: windows_core::IUnknownImpl {
    fn SetSample(&self, value: windows_core::Ref<super::IMFSample>) -> windows_core::Result<()>;
}
#[cfg(feature = "mfobjects")]
impl IMFMediaStreamSourceSampleRequest_Vtbl {
    pub const fn new<Identity: IMFMediaStreamSourceSampleRequest_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetSample<Identity: IMFMediaStreamSourceSampleRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaStreamSourceSampleRequest_Impl::SetSample(this, core::mem::transmute_copy(&value)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetSample: SetSample::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaStreamSourceSampleRequest as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFMediaStreamSourceSampleRequest {}
windows_core::imp::define_interface!(IMFMediaTypeHandler, IMFMediaTypeHandler_Vtbl, 0xe93dcf6c_4b07_4e1e_8123_aa16ed6eadf5);
windows_core::imp::interface_hierarchy!(IMFMediaTypeHandler, windows_core::IUnknown);
impl IMFMediaTypeHandler {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn IsMediaTypeSupported<P0>(&self, pmediatype: P0) -> windows_core::Result<super::IMFMediaType>
    where
        P0: windows_core::Param<super::IMFMediaType>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsMediaTypeSupported)(windows_core::Interface::as_raw(self), pmediatype.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetMediaTypeCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMediaTypeCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn GetMediaTypeByIndex(&self, dwindex: u32) -> windows_core::Result<super::IMFMediaType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMediaTypeByIndex)(windows_core::Interface::as_raw(self), dwindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn SetCurrentMediaType<P0>(&self, pmediatype: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMFMediaType>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCurrentMediaType)(windows_core::Interface::as_raw(self), pmediatype.param().abi()) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn GetCurrentMediaType(&self) -> windows_core::Result<super::IMFMediaType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentMediaType)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetMajorType(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMajorType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMediaTypeHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "mfobjects")]
    pub IsMediaTypeSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    IsMediaTypeSupported: usize,
    pub GetMediaTypeCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "mfobjects")]
    pub GetMediaTypeByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    GetMediaTypeByIndex: usize,
    #[cfg(feature = "mfobjects")]
    pub SetCurrentMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    SetCurrentMediaType: usize,
    #[cfg(feature = "mfobjects")]
    pub GetCurrentMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    GetCurrentMediaType: usize,
    pub GetMajorType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
#[cfg(feature = "mfobjects")]
pub trait IMFMediaTypeHandler_Impl: windows_core::IUnknownImpl {
    fn IsMediaTypeSupported(&self, pmediatype: windows_core::Ref<super::IMFMediaType>) -> windows_core::Result<super::IMFMediaType>;
    fn GetMediaTypeCount(&self) -> windows_core::Result<u32>;
    fn GetMediaTypeByIndex(&self, dwindex: u32) -> windows_core::Result<super::IMFMediaType>;
    fn SetCurrentMediaType(&self, pmediatype: windows_core::Ref<super::IMFMediaType>) -> windows_core::Result<()>;
    fn GetCurrentMediaType(&self) -> windows_core::Result<super::IMFMediaType>;
    fn GetMajorType(&self) -> windows_core::Result<windows_core::GUID>;
}
#[cfg(feature = "mfobjects")]
impl IMFMediaTypeHandler_Vtbl {
    pub const fn new<Identity: IMFMediaTypeHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsMediaTypeSupported<Identity: IMFMediaTypeHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmediatype: *mut core::ffi::c_void, ppmediatype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaTypeHandler_Impl::IsMediaTypeSupported(this, core::mem::transmute_copy(&pmediatype)) {
                    Ok(ok__) => {
                        ppmediatype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMediaTypeCount<Identity: IMFMediaTypeHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwtypecount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaTypeHandler_Impl::GetMediaTypeCount(this) {
                    Ok(ok__) => {
                        pdwtypecount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMediaTypeByIndex<Identity: IMFMediaTypeHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, pptype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaTypeHandler_Impl::GetMediaTypeByIndex(this, core::mem::transmute_copy(&dwindex)) {
                    Ok(ok__) => {
                        pptype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCurrentMediaType<Identity: IMFMediaTypeHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmediatype: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMediaTypeHandler_Impl::SetCurrentMediaType(this, core::mem::transmute_copy(&pmediatype)).into()
            }
        }
        unsafe extern "system" fn GetCurrentMediaType<Identity: IMFMediaTypeHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmediatype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaTypeHandler_Impl::GetCurrentMediaType(this) {
                    Ok(ok__) => {
                        ppmediatype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMajorType<Identity: IMFMediaTypeHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidmajortype: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMediaTypeHandler_Impl::GetMajorType(this) {
                    Ok(ok__) => {
                        pguidmajortype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsMediaTypeSupported: IsMediaTypeSupported::<Identity, OFFSET>,
            GetMediaTypeCount: GetMediaTypeCount::<Identity, OFFSET>,
            GetMediaTypeByIndex: GetMediaTypeByIndex::<Identity, OFFSET>,
            SetCurrentMediaType: SetCurrentMediaType::<Identity, OFFSET>,
            GetCurrentMediaType: GetCurrentMediaType::<Identity, OFFSET>,
            GetMajorType: GetMajorType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMediaTypeHandler as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFMediaTypeHandler {}
windows_core::imp::define_interface!(IMFMetadata, IMFMetadata_Vtbl, 0xf88cfb8c_ef16_4991_b450_cb8c69e51704);
windows_core::imp::interface_hierarchy!(IMFMetadata, windows_core::IUnknown);
impl IMFMetadata {
    pub unsafe fn SetLanguage<P0>(&self, pwszrfc1766: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetLanguage)(windows_core::Interface::as_raw(self), pwszrfc1766.param().abi()) }
    }
    pub unsafe fn GetLanguage(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLanguage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetAllLanguages(&self) -> windows_core::Result<super::PROPVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAllLanguages)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn SetProperty<P0>(&self, pwszname: P0, ppvvalue: *const super::PROPVARIANT) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), pwszname.param().abi(), ppvvalue) }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetProperty<P0>(&self, pwszname: P0) -> windows_core::Result<super::PROPVARIANT>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), pwszname.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn DeleteProperty<P0>(&self, pwszname: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).DeleteProperty)(windows_core::Interface::as_raw(self), pwszname.param().abi()) }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetAllPropertyNames(&self) -> windows_core::Result<super::PROPVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAllPropertyNames)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMetadata_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetLanguage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub GetAllLanguages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    GetAllLanguages: usize,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const super::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    SetProperty: usize,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut super::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    GetProperty: usize,
    pub DeleteProperty: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub GetAllPropertyNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    GetAllPropertyNames: usize,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMFMetadata_Impl: windows_core::IUnknownImpl {
    fn SetLanguage(&self, pwszrfc1766: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetLanguage(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn GetAllLanguages(&self) -> windows_core::Result<super::PROPVARIANT>;
    fn SetProperty(&self, pwszname: &windows_core::PCWSTR, ppvvalue: *const super::PROPVARIANT) -> windows_core::Result<()>;
    fn GetProperty(&self, pwszname: &windows_core::PCWSTR) -> windows_core::Result<super::PROPVARIANT>;
    fn DeleteProperty(&self, pwszname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetAllPropertyNames(&self) -> windows_core::Result<super::PROPVARIANT>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl IMFMetadata_Vtbl {
    pub const fn new<Identity: IMFMetadata_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetLanguage<Identity: IMFMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszrfc1766: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMetadata_Impl::SetLanguage(this, core::mem::transmute(&pwszrfc1766)).into()
            }
        }
        unsafe extern "system" fn GetLanguage<Identity: IMFMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwszrfc1766: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMetadata_Impl::GetLanguage(this) {
                    Ok(ok__) => {
                        ppwszrfc1766.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAllLanguages<Identity: IMFMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvlanguages: *mut super::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMetadata_Impl::GetAllLanguages(this) {
                    Ok(ok__) => {
                        ppvlanguages.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetProperty<Identity: IMFMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PCWSTR, ppvvalue: *const super::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMetadata_Impl::SetProperty(this, core::mem::transmute(&pwszname), core::mem::transmute_copy(&ppvvalue)).into()
            }
        }
        unsafe extern "system" fn GetProperty<Identity: IMFMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PCWSTR, ppvvalue: *mut super::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMetadata_Impl::GetProperty(this, core::mem::transmute(&pwszname)) {
                    Ok(ok__) => {
                        ppvvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteProperty<Identity: IMFMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFMetadata_Impl::DeleteProperty(this, core::mem::transmute(&pwszname)).into()
            }
        }
        unsafe extern "system" fn GetAllPropertyNames<Identity: IMFMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppvnames: *mut super::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMetadata_Impl::GetAllPropertyNames(this) {
                    Ok(ok__) => {
                        ppvnames.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetLanguage: SetLanguage::<Identity, OFFSET>,
            GetLanguage: GetLanguage::<Identity, OFFSET>,
            GetAllLanguages: GetAllLanguages::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            DeleteProperty: DeleteProperty::<Identity, OFFSET>,
            GetAllPropertyNames: GetAllPropertyNames::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMetadata as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMFMetadata {}
windows_core::imp::define_interface!(IMFMetadataProvider, IMFMetadataProvider_Vtbl, 0x56181d2d_e221_4adb_b1c8_3cee6a53f76f);
windows_core::imp::interface_hierarchy!(IMFMetadataProvider, windows_core::IUnknown);
impl IMFMetadataProvider {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn GetMFMetadata<P0>(&self, ppresentationdescriptor: P0, dwstreamidentifier: u32, dwflags: u32) -> windows_core::Result<IMFMetadata>
    where
        P0: windows_core::Param<IMFPresentationDescriptor>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMFMetadata)(windows_core::Interface::as_raw(self), ppresentationdescriptor.param().abi(), dwstreamidentifier, dwflags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFMetadataProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "mfobjects")]
    pub GetMFMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    GetMFMetadata: usize,
}
#[cfg(feature = "mfobjects")]
pub trait IMFMetadataProvider_Impl: windows_core::IUnknownImpl {
    fn GetMFMetadata(&self, ppresentationdescriptor: windows_core::Ref<IMFPresentationDescriptor>, dwstreamidentifier: u32, dwflags: u32) -> windows_core::Result<IMFMetadata>;
}
#[cfg(feature = "mfobjects")]
impl IMFMetadataProvider_Vtbl {
    pub const fn new<Identity: IMFMetadataProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMFMetadata<Identity: IMFMetadataProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresentationdescriptor: *mut core::ffi::c_void, dwstreamidentifier: u32, dwflags: u32, ppmfmetadata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFMetadataProvider_Impl::GetMFMetadata(this, core::mem::transmute_copy(&ppresentationdescriptor), core::mem::transmute_copy(&dwstreamidentifier), core::mem::transmute_copy(&dwflags)) {
                    Ok(ok__) => {
                        ppmfmetadata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetMFMetadata: GetMFMetadata::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFMetadataProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFMetadataProvider {}
windows_core::imp::define_interface!(IMFNetCredential, IMFNetCredential_Vtbl, 0x5b87ef6a_7ed8_434f_ba0e_184fac1628d1);
windows_core::imp::interface_hierarchy!(IMFNetCredential, windows_core::IUnknown);
impl IMFNetCredential {
    pub unsafe fn SetUser(&self, pbdata: &[u8], fdataisencrypted: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetUser)(windows_core::Interface::as_raw(self), pbdata.as_ptr(), pbdata.len().try_into().unwrap(), fdataisencrypted.into()) }
    }
    pub unsafe fn SetPassword(&self, pbdata: &[u8], fdataisencrypted: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPassword)(windows_core::Interface::as_raw(self), pbdata.as_ptr(), pbdata.len().try_into().unwrap(), fdataisencrypted.into()) }
    }
    pub unsafe fn GetUser(&self, pbdata: Option<*mut u8>, pcbdata: *mut u32, fencryptdata: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetUser)(windows_core::Interface::as_raw(self), pbdata.unwrap_or(core::mem::zeroed()) as _, pcbdata as _, fencryptdata.into()) }
    }
    pub unsafe fn GetPassword(&self, pbdata: Option<*mut u8>, pcbdata: *mut u32, fencryptdata: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPassword)(windows_core::Interface::as_raw(self), pbdata.unwrap_or(core::mem::zeroed()) as _, pcbdata as _, fencryptdata.into()) }
    }
    pub unsafe fn LoggedOnUser(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LoggedOnUser)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFNetCredential_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetUser: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetPassword: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u32, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetPassword: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u8, *mut u32, windows_core::BOOL) -> windows_core::HRESULT,
    pub LoggedOnUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IMFNetCredential_Impl: windows_core::IUnknownImpl {
    fn SetUser(&self, pbdata: *const u8, cbdata: u32, fdataisencrypted: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetPassword(&self, pbdata: *const u8, cbdata: u32, fdataisencrypted: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetUser(&self, pbdata: *mut u8, pcbdata: *mut u32, fencryptdata: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetPassword(&self, pbdata: *mut u8, pcbdata: *mut u32, fencryptdata: windows_core::BOOL) -> windows_core::Result<()>;
    fn LoggedOnUser(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IMFNetCredential_Vtbl {
    pub const fn new<Identity: IMFNetCredential_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetUser<Identity: IMFNetCredential_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbdata: *const u8, cbdata: u32, fdataisencrypted: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFNetCredential_Impl::SetUser(this, core::mem::transmute_copy(&pbdata), core::mem::transmute_copy(&cbdata), core::mem::transmute_copy(&fdataisencrypted)).into()
            }
        }
        unsafe extern "system" fn SetPassword<Identity: IMFNetCredential_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbdata: *const u8, cbdata: u32, fdataisencrypted: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFNetCredential_Impl::SetPassword(this, core::mem::transmute_copy(&pbdata), core::mem::transmute_copy(&cbdata), core::mem::transmute_copy(&fdataisencrypted)).into()
            }
        }
        unsafe extern "system" fn GetUser<Identity: IMFNetCredential_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbdata: *mut u8, pcbdata: *mut u32, fencryptdata: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFNetCredential_Impl::GetUser(this, core::mem::transmute_copy(&pbdata), core::mem::transmute_copy(&pcbdata), core::mem::transmute_copy(&fencryptdata)).into()
            }
        }
        unsafe extern "system" fn GetPassword<Identity: IMFNetCredential_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbdata: *mut u8, pcbdata: *mut u32, fencryptdata: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFNetCredential_Impl::GetPassword(this, core::mem::transmute_copy(&pbdata), core::mem::transmute_copy(&pcbdata), core::mem::transmute_copy(&fencryptdata)).into()
            }
        }
        unsafe extern "system" fn LoggedOnUser<Identity: IMFNetCredential_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfloggedonuser: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFNetCredential_Impl::LoggedOnUser(this) {
                    Ok(ok__) => {
                        pfloggedonuser.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetUser: SetUser::<Identity, OFFSET>,
            SetPassword: SetPassword::<Identity, OFFSET>,
            GetUser: GetUser::<Identity, OFFSET>,
            GetPassword: GetPassword::<Identity, OFFSET>,
            LoggedOnUser: LoggedOnUser::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFNetCredential as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFNetCredential {}
windows_core::imp::define_interface!(IMFNetCredentialCache, IMFNetCredentialCache_Vtbl, 0x5b87ef6c_7ed8_434f_ba0e_184fac1628d1);
windows_core::imp::interface_hierarchy!(IMFNetCredentialCache, windows_core::IUnknown);
impl IMFNetCredentialCache {
    pub unsafe fn GetCredential<P0, P1>(&self, pszurl: P0, pszrealm: P1, dwauthenticationflags: u32, ppcred: *mut Option<IMFNetCredential>, pdwrequirementsflags: *mut u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetCredential)(windows_core::Interface::as_raw(self), pszurl.param().abi(), pszrealm.param().abi(), dwauthenticationflags, core::mem::transmute(ppcred), pdwrequirementsflags as _) }
    }
    pub unsafe fn SetGood<P0>(&self, pcred: P0, fgood: bool) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFNetCredential>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetGood)(windows_core::Interface::as_raw(self), pcred.param().abi(), fgood.into()) }
    }
    pub unsafe fn SetUserOptions<P0>(&self, pcred: P0, dwoptionsflags: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFNetCredential>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetUserOptions)(windows_core::Interface::as_raw(self), pcred.param().abi(), dwoptionsflags) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFNetCredentialCache_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCredential: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetGood: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetUserOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IMFNetCredentialCache_Impl: windows_core::IUnknownImpl {
    fn GetCredential(&self, pszurl: &windows_core::PCWSTR, pszrealm: &windows_core::PCWSTR, dwauthenticationflags: u32, ppcred: windows_core::OutRef<IMFNetCredential>, pdwrequirementsflags: *mut u32) -> windows_core::Result<()>;
    fn SetGood(&self, pcred: windows_core::Ref<IMFNetCredential>, fgood: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetUserOptions(&self, pcred: windows_core::Ref<IMFNetCredential>, dwoptionsflags: u32) -> windows_core::Result<()>;
}
impl IMFNetCredentialCache_Vtbl {
    pub const fn new<Identity: IMFNetCredentialCache_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCredential<Identity: IMFNetCredentialCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszurl: windows_core::PCWSTR, pszrealm: windows_core::PCWSTR, dwauthenticationflags: u32, ppcred: *mut *mut core::ffi::c_void, pdwrequirementsflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFNetCredentialCache_Impl::GetCredential(this, core::mem::transmute(&pszurl), core::mem::transmute(&pszrealm), core::mem::transmute_copy(&dwauthenticationflags), core::mem::transmute_copy(&ppcred), core::mem::transmute_copy(&pdwrequirementsflags)).into()
            }
        }
        unsafe extern "system" fn SetGood<Identity: IMFNetCredentialCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcred: *mut core::ffi::c_void, fgood: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFNetCredentialCache_Impl::SetGood(this, core::mem::transmute_copy(&pcred), core::mem::transmute_copy(&fgood)).into()
            }
        }
        unsafe extern "system" fn SetUserOptions<Identity: IMFNetCredentialCache_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcred: *mut core::ffi::c_void, dwoptionsflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFNetCredentialCache_Impl::SetUserOptions(this, core::mem::transmute_copy(&pcred), core::mem::transmute_copy(&dwoptionsflags)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCredential: GetCredential::<Identity, OFFSET>,
            SetGood: SetGood::<Identity, OFFSET>,
            SetUserOptions: SetUserOptions::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFNetCredentialCache as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFNetCredentialCache {}
windows_core::imp::define_interface!(IMFNetCredentialManager, IMFNetCredentialManager_Vtbl, 0x5b87ef6b_7ed8_434f_ba0e_184fac1628d1);
windows_core::imp::interface_hierarchy!(IMFNetCredentialManager, windows_core::IUnknown);
impl IMFNetCredentialManager {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn BeginGetCredentials<P1, P2>(&self, pparam: *const MFNetCredentialManagerGetParam, pcallback: P1, pstate: P2) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IMFAsyncCallback>,
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).BeginGetCredentials)(windows_core::Interface::as_raw(self), pparam, pcallback.param().abi(), pstate.param().abi()) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn EndGetCredentials<P0>(&self, presult: P0) -> windows_core::Result<IMFNetCredential>
    where
        P0: windows_core::Param<super::IMFAsyncResult>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EndGetCredentials)(windows_core::Interface::as_raw(self), presult.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetGood<P0>(&self, pcred: P0, fgood: bool) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFNetCredential>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetGood)(windows_core::Interface::as_raw(self), pcred.param().abi(), fgood.into()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFNetCredentialManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "mfobjects")]
    pub BeginGetCredentials: unsafe extern "system" fn(*mut core::ffi::c_void, *const MFNetCredentialManagerGetParam, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    BeginGetCredentials: usize,
    #[cfg(feature = "mfobjects")]
    pub EndGetCredentials: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    EndGetCredentials: usize,
    pub SetGood: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "mfobjects")]
pub trait IMFNetCredentialManager_Impl: windows_core::IUnknownImpl {
    fn BeginGetCredentials(&self, pparam: *const MFNetCredentialManagerGetParam, pcallback: windows_core::Ref<super::IMFAsyncCallback>, pstate: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn EndGetCredentials(&self, presult: windows_core::Ref<super::IMFAsyncResult>) -> windows_core::Result<IMFNetCredential>;
    fn SetGood(&self, pcred: windows_core::Ref<IMFNetCredential>, fgood: windows_core::BOOL) -> windows_core::Result<()>;
}
#[cfg(feature = "mfobjects")]
impl IMFNetCredentialManager_Vtbl {
    pub const fn new<Identity: IMFNetCredentialManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BeginGetCredentials<Identity: IMFNetCredentialManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pparam: *const MFNetCredentialManagerGetParam, pcallback: *mut core::ffi::c_void, pstate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFNetCredentialManager_Impl::BeginGetCredentials(this, core::mem::transmute_copy(&pparam), core::mem::transmute_copy(&pcallback), core::mem::transmute_copy(&pstate)).into()
            }
        }
        unsafe extern "system" fn EndGetCredentials<Identity: IMFNetCredentialManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presult: *mut core::ffi::c_void, ppcred: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFNetCredentialManager_Impl::EndGetCredentials(this, core::mem::transmute_copy(&presult)) {
                    Ok(ok__) => {
                        ppcred.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetGood<Identity: IMFNetCredentialManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcred: *mut core::ffi::c_void, fgood: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFNetCredentialManager_Impl::SetGood(this, core::mem::transmute_copy(&pcred), core::mem::transmute_copy(&fgood)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginGetCredentials: BeginGetCredentials::<Identity, OFFSET>,
            EndGetCredentials: EndGetCredentials::<Identity, OFFSET>,
            SetGood: SetGood::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFNetCredentialManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFNetCredentialManager {}
windows_core::imp::define_interface!(IMFNetCrossOriginSupport, IMFNetCrossOriginSupport_Vtbl, 0xbc2b7d44_a72d_49d5_8376_1480dee58b22);
windows_core::imp::interface_hierarchy!(IMFNetCrossOriginSupport, windows_core::IUnknown);
impl IMFNetCrossOriginSupport {
    pub unsafe fn GetCrossOriginPolicy(&self) -> windows_core::Result<MF_CROSS_ORIGIN_POLICY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCrossOriginPolicy)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSourceOrigin(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSourceOrigin)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsSameOrigin<P0>(&self, wszurl: P0) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsSameOrigin)(windows_core::Interface::as_raw(self), wszurl.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFNetCrossOriginSupport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCrossOriginPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MF_CROSS_ORIGIN_POLICY) -> windows_core::HRESULT,
    pub GetSourceOrigin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
    pub IsSameOrigin: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IMFNetCrossOriginSupport_Impl: windows_core::IUnknownImpl {
    fn GetCrossOriginPolicy(&self) -> windows_core::Result<MF_CROSS_ORIGIN_POLICY>;
    fn GetSourceOrigin(&self) -> windows_core::Result<windows_core::PWSTR>;
    fn IsSameOrigin(&self, wszurl: &windows_core::PCWSTR) -> windows_core::Result<windows_core::BOOL>;
}
impl IMFNetCrossOriginSupport_Vtbl {
    pub const fn new<Identity: IMFNetCrossOriginSupport_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCrossOriginPolicy<Identity: IMFNetCrossOriginSupport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppolicy: *mut MF_CROSS_ORIGIN_POLICY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFNetCrossOriginSupport_Impl::GetCrossOriginPolicy(this) {
                    Ok(ok__) => {
                        ppolicy.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSourceOrigin<Identity: IMFNetCrossOriginSupport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszsourceorigin: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFNetCrossOriginSupport_Impl::GetSourceOrigin(this) {
                    Ok(ok__) => {
                        wszsourceorigin.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsSameOrigin<Identity: IMFNetCrossOriginSupport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszurl: windows_core::PCWSTR, pfissameorigin: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFNetCrossOriginSupport_Impl::IsSameOrigin(this, core::mem::transmute(&wszurl)) {
                    Ok(ok__) => {
                        pfissameorigin.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCrossOriginPolicy: GetCrossOriginPolicy::<Identity, OFFSET>,
            GetSourceOrigin: GetSourceOrigin::<Identity, OFFSET>,
            IsSameOrigin: IsSameOrigin::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFNetCrossOriginSupport as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFNetCrossOriginSupport {}
windows_core::imp::define_interface!(IMFNetProxyLocator, IMFNetProxyLocator_Vtbl, 0xe9cd0383_a268_4bb4_82de_658d53574d41);
windows_core::imp::interface_hierarchy!(IMFNetProxyLocator, windows_core::IUnknown);
impl IMFNetProxyLocator {
    pub unsafe fn FindFirstProxy<P0, P1>(&self, pszhost: P0, pszurl: P1, freserved: bool) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).FindFirstProxy)(windows_core::Interface::as_raw(self), pszhost.param().abi(), pszurl.param().abi(), freserved.into()) }
    }
    pub unsafe fn FindNextProxy(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).FindNextProxy)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn RegisterProxyResult(&self, hrop: windows_core::HRESULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RegisterProxyResult)(windows_core::Interface::as_raw(self), hrop) }
    }
    pub unsafe fn GetCurrentProxy(&self, pszstr: Option<windows_core::PWSTR>, pcchstr: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCurrentProxy)(windows_core::Interface::as_raw(self), pszstr.unwrap_or(core::mem::zeroed()) as _, pcchstr as _) }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFNetProxyLocator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub FindFirstProxy: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::BOOL) -> windows_core::HRESULT,
    pub FindNextProxy: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RegisterProxyResult: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    pub GetCurrentProxy: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFNetProxyLocator_Impl: windows_core::IUnknownImpl {
    fn FindFirstProxy(&self, pszhost: &windows_core::PCWSTR, pszurl: &windows_core::PCWSTR, freserved: windows_core::BOOL) -> windows_core::Result<()>;
    fn FindNextProxy(&self) -> windows_core::Result<()>;
    fn RegisterProxyResult(&self, hrop: windows_core::HRESULT) -> windows_core::Result<()>;
    fn GetCurrentProxy(&self, pszstr: windows_core::PWSTR, pcchstr: *mut u32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IMFNetProxyLocator>;
}
impl IMFNetProxyLocator_Vtbl {
    pub const fn new<Identity: IMFNetProxyLocator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FindFirstProxy<Identity: IMFNetProxyLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszhost: windows_core::PCWSTR, pszurl: windows_core::PCWSTR, freserved: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFNetProxyLocator_Impl::FindFirstProxy(this, core::mem::transmute(&pszhost), core::mem::transmute(&pszurl), core::mem::transmute_copy(&freserved)).into()
            }
        }
        unsafe extern "system" fn FindNextProxy<Identity: IMFNetProxyLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFNetProxyLocator_Impl::FindNextProxy(this).into()
            }
        }
        unsafe extern "system" fn RegisterProxyResult<Identity: IMFNetProxyLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hrop: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFNetProxyLocator_Impl::RegisterProxyResult(this, core::mem::transmute_copy(&hrop)).into()
            }
        }
        unsafe extern "system" fn GetCurrentProxy<Identity: IMFNetProxyLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszstr: windows_core::PWSTR, pcchstr: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFNetProxyLocator_Impl::GetCurrentProxy(this, core::mem::transmute_copy(&pszstr), core::mem::transmute_copy(&pcchstr)).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IMFNetProxyLocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppproxylocator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFNetProxyLocator_Impl::Clone(this) {
                    Ok(ok__) => {
                        ppproxylocator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            FindFirstProxy: FindFirstProxy::<Identity, OFFSET>,
            FindNextProxy: FindNextProxy::<Identity, OFFSET>,
            RegisterProxyResult: RegisterProxyResult::<Identity, OFFSET>,
            GetCurrentProxy: GetCurrentProxy::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFNetProxyLocator as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFNetProxyLocator {}
windows_core::imp::define_interface!(IMFNetProxyLocatorFactory, IMFNetProxyLocatorFactory_Vtbl, 0xe9cd0384_a268_4bb4_82de_658d53574d41);
windows_core::imp::interface_hierarchy!(IMFNetProxyLocatorFactory, windows_core::IUnknown);
impl IMFNetProxyLocatorFactory {
    pub unsafe fn CreateProxyLocator<P0>(&self, pszprotocol: P0) -> windows_core::Result<IMFNetProxyLocator>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateProxyLocator)(windows_core::Interface::as_raw(self), pszprotocol.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFNetProxyLocatorFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateProxyLocator: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFNetProxyLocatorFactory_Impl: windows_core::IUnknownImpl {
    fn CreateProxyLocator(&self, pszprotocol: &windows_core::PCWSTR) -> windows_core::Result<IMFNetProxyLocator>;
}
impl IMFNetProxyLocatorFactory_Vtbl {
    pub const fn new<Identity: IMFNetProxyLocatorFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateProxyLocator<Identity: IMFNetProxyLocatorFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszprotocol: windows_core::PCWSTR, ppproxylocator: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFNetProxyLocatorFactory_Impl::CreateProxyLocator(this, core::mem::transmute(&pszprotocol)) {
                    Ok(ok__) => {
                        ppproxylocator.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), CreateProxyLocator: CreateProxyLocator::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFNetProxyLocatorFactory as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFNetProxyLocatorFactory {}
windows_core::imp::define_interface!(IMFNetResourceFilter, IMFNetResourceFilter_Vtbl, 0x091878a3_bf11_4a5c_bc9f_33995b06ef2d);
windows_core::imp::interface_hierarchy!(IMFNetResourceFilter, windows_core::IUnknown);
impl IMFNetResourceFilter {
    #[cfg(feature = "wtypes")]
    pub unsafe fn OnRedirect<P0>(&self, pszurl: P0) -> windows_core::Result<super::VARIANT_BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OnRedirect)(windows_core::Interface::as_raw(self), pszurl.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn OnSendingRequest<P0>(&self, pszurl: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnSendingRequest)(windows_core::Interface::as_raw(self), pszurl.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFNetResourceFilter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "wtypes")]
    pub OnRedirect: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    OnRedirect: usize,
    pub OnSendingRequest: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
#[cfg(feature = "wtypes")]
pub trait IMFNetResourceFilter_Impl: windows_core::IUnknownImpl {
    fn OnRedirect(&self, pszurl: &windows_core::PCWSTR) -> windows_core::Result<super::VARIANT_BOOL>;
    fn OnSendingRequest(&self, pszurl: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "wtypes")]
impl IMFNetResourceFilter_Vtbl {
    pub const fn new<Identity: IMFNetResourceFilter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnRedirect<Identity: IMFNetResourceFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszurl: windows_core::PCWSTR, pvbcancel: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFNetResourceFilter_Impl::OnRedirect(this, core::mem::transmute(&pszurl)) {
                    Ok(ok__) => {
                        pvbcancel.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn OnSendingRequest<Identity: IMFNetResourceFilter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszurl: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFNetResourceFilter_Impl::OnSendingRequest(this, core::mem::transmute(&pszurl)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnRedirect: OnRedirect::<Identity, OFFSET>,
            OnSendingRequest: OnSendingRequest::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFNetResourceFilter as windows_core::Interface>::IID
    }
}
#[cfg(feature = "wtypes")]
impl windows_core::RuntimeName for IMFNetResourceFilter {}
windows_core::imp::define_interface!(IMFNetSchemeHandlerConfig, IMFNetSchemeHandlerConfig_Vtbl, 0x7be19e73_c9bf_468a_ac5a_a5e8653bec87);
windows_core::imp::interface_hierarchy!(IMFNetSchemeHandlerConfig, windows_core::IUnknown);
impl IMFNetSchemeHandlerConfig {
    pub unsafe fn GetNumberOfSupportedProtocols(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNumberOfSupportedProtocols)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSupportedProtocolType(&self, nprotocolindex: u32) -> windows_core::Result<MFNETSOURCE_PROTOCOL_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSupportedProtocolType)(windows_core::Interface::as_raw(self), nprotocolindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ResetProtocolRolloverSettings(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ResetProtocolRolloverSettings)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFNetSchemeHandlerConfig_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetNumberOfSupportedProtocols: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetSupportedProtocolType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut MFNETSOURCE_PROTOCOL_TYPE) -> windows_core::HRESULT,
    pub ResetProtocolRolloverSettings: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFNetSchemeHandlerConfig_Impl: windows_core::IUnknownImpl {
    fn GetNumberOfSupportedProtocols(&self) -> windows_core::Result<u32>;
    fn GetSupportedProtocolType(&self, nprotocolindex: u32) -> windows_core::Result<MFNETSOURCE_PROTOCOL_TYPE>;
    fn ResetProtocolRolloverSettings(&self) -> windows_core::Result<()>;
}
impl IMFNetSchemeHandlerConfig_Vtbl {
    pub const fn new<Identity: IMFNetSchemeHandlerConfig_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetNumberOfSupportedProtocols<Identity: IMFNetSchemeHandlerConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcprotocols: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFNetSchemeHandlerConfig_Impl::GetNumberOfSupportedProtocols(this) {
                    Ok(ok__) => {
                        pcprotocols.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSupportedProtocolType<Identity: IMFNetSchemeHandlerConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, nprotocolindex: u32, pnprotocoltype: *mut MFNETSOURCE_PROTOCOL_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFNetSchemeHandlerConfig_Impl::GetSupportedProtocolType(this, core::mem::transmute_copy(&nprotocolindex)) {
                    Ok(ok__) => {
                        pnprotocoltype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ResetProtocolRolloverSettings<Identity: IMFNetSchemeHandlerConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFNetSchemeHandlerConfig_Impl::ResetProtocolRolloverSettings(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetNumberOfSupportedProtocols: GetNumberOfSupportedProtocols::<Identity, OFFSET>,
            GetSupportedProtocolType: GetSupportedProtocolType::<Identity, OFFSET>,
            ResetProtocolRolloverSettings: ResetProtocolRolloverSettings::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFNetSchemeHandlerConfig as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFNetSchemeHandlerConfig {}
windows_core::imp::define_interface!(IMFObjectReferenceStream, IMFObjectReferenceStream_Vtbl, 0x09ef5be3_c8a7_469e_8b70_73bf25bb193f);
windows_core::imp::interface_hierarchy!(IMFObjectReferenceStream, windows_core::IUnknown);
impl IMFObjectReferenceStream {
    pub unsafe fn SaveReference<P1>(&self, riid: *const windows_core::GUID, punk: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SaveReference)(windows_core::Interface::as_raw(self), riid, punk.param().abi()) }
    }
    pub unsafe fn LoadReference<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).LoadReference)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFObjectReferenceStream_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SaveReference: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LoadReference: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFObjectReferenceStream_Impl: windows_core::IUnknownImpl {
    fn SaveReference(&self, riid: *const windows_core::GUID, punk: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn LoadReference(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IMFObjectReferenceStream_Vtbl {
    pub const fn new<Identity: IMFObjectReferenceStream_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SaveReference<Identity: IMFObjectReferenceStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, punk: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFObjectReferenceStream_Impl::SaveReference(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&punk)).into()
            }
        }
        unsafe extern "system" fn LoadReference<Identity: IMFObjectReferenceStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFObjectReferenceStream_Impl::LoadReference(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SaveReference: SaveReference::<Identity, OFFSET>,
            LoadReference: LoadReference::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFObjectReferenceStream as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFObjectReferenceStream {}
#[cfg(feature = "mfobjects")]
windows_core::imp::define_interface!(IMFOutputPolicy, IMFOutputPolicy_Vtbl, 0x7f00f10a_daed_41af_ab26_5fdfa4dfba3c);
#[cfg(feature = "mfobjects")]
impl core::ops::Deref for IMFOutputPolicy {
    type Target = super::IMFAttributes;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "mfobjects")]
windows_core::imp::interface_hierarchy!(IMFOutputPolicy, windows_core::IUnknown, super::IMFAttributes);
#[cfg(feature = "mfobjects")]
impl IMFOutputPolicy {
    pub unsafe fn GenerateRequiredSchemas(&self, dwattributes: u32, guidoutputsubtype: windows_core::GUID, rgguidprotectionschemassupported: *const windows_core::GUID, cprotectionschemassupported: u32) -> windows_core::Result<super::IMFCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GenerateRequiredSchemas)(windows_core::Interface::as_raw(self), dwattributes, guidoutputsubtype, rgguidprotectionschemassupported, cprotectionschemassupported, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetOriginatorID(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOriginatorID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMinimumGRLVersion(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMinimumGRLVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "mfobjects")]
#[repr(C)]
#[doc(hidden)]
pub struct IMFOutputPolicy_Vtbl {
    pub base__: super::IMFAttributes_Vtbl,
    pub GenerateRequiredSchemas: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::GUID, *const windows_core::GUID, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetOriginatorID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetMinimumGRLVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMFOutputPolicy_Impl: super::IMFAttributes_Impl {
    fn GenerateRequiredSchemas(&self, dwattributes: u32, guidoutputsubtype: &windows_core::GUID, rgguidprotectionschemassupported: *const windows_core::GUID, cprotectionschemassupported: u32) -> windows_core::Result<super::IMFCollection>;
    fn GetOriginatorID(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetMinimumGRLVersion(&self) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl IMFOutputPolicy_Vtbl {
    pub const fn new<Identity: IMFOutputPolicy_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GenerateRequiredSchemas<Identity: IMFOutputPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwattributes: u32, guidoutputsubtype: windows_core::GUID, rgguidprotectionschemassupported: *const windows_core::GUID, cprotectionschemassupported: u32, pprequiredprotectionschemas: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFOutputPolicy_Impl::GenerateRequiredSchemas(this, core::mem::transmute_copy(&dwattributes), core::mem::transmute(&guidoutputsubtype), core::mem::transmute_copy(&rgguidprotectionschemassupported), core::mem::transmute_copy(&cprotectionschemassupported)) {
                    Ok(ok__) => {
                        pprequiredprotectionschemas.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOriginatorID<Identity: IMFOutputPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidoriginatorid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFOutputPolicy_Impl::GetOriginatorID(this) {
                    Ok(ok__) => {
                        pguidoriginatorid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMinimumGRLVersion<Identity: IMFOutputPolicy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwminimumgrlversion: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFOutputPolicy_Impl::GetMinimumGRLVersion(this) {
                    Ok(ok__) => {
                        pdwminimumgrlversion.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IMFAttributes_Vtbl::new::<Identity, OFFSET>(),
            GenerateRequiredSchemas: GenerateRequiredSchemas::<Identity, OFFSET>,
            GetOriginatorID: GetOriginatorID::<Identity, OFFSET>,
            GetMinimumGRLVersion: GetMinimumGRLVersion::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFOutputPolicy as windows_core::Interface>::IID || iid == &<super::IMFAttributes as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMFOutputPolicy {}
#[cfg(feature = "mfobjects")]
windows_core::imp::define_interface!(IMFOutputSchema, IMFOutputSchema_Vtbl, 0x7be0fc5b_abd9_44fb_a5c8_f50136e71599);
#[cfg(feature = "mfobjects")]
impl core::ops::Deref for IMFOutputSchema {
    type Target = super::IMFAttributes;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "mfobjects")]
windows_core::imp::interface_hierarchy!(IMFOutputSchema, windows_core::IUnknown, super::IMFAttributes);
#[cfg(feature = "mfobjects")]
impl IMFOutputSchema {
    pub unsafe fn GetSchemaType(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSchemaType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetConfigurationData(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConfigurationData)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetOriginatorID(&self) -> windows_core::Result<windows_core::GUID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOriginatorID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "mfobjects")]
#[repr(C)]
#[doc(hidden)]
pub struct IMFOutputSchema_Vtbl {
    pub base__: super::IMFAttributes_Vtbl,
    pub GetSchemaType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
    pub GetConfigurationData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetOriginatorID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::GUID) -> windows_core::HRESULT,
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMFOutputSchema_Impl: super::IMFAttributes_Impl {
    fn GetSchemaType(&self) -> windows_core::Result<windows_core::GUID>;
    fn GetConfigurationData(&self) -> windows_core::Result<u32>;
    fn GetOriginatorID(&self) -> windows_core::Result<windows_core::GUID>;
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl IMFOutputSchema_Vtbl {
    pub const fn new<Identity: IMFOutputSchema_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSchemaType<Identity: IMFOutputSchema_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidschematype: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFOutputSchema_Impl::GetSchemaType(this) {
                    Ok(ok__) => {
                        pguidschematype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetConfigurationData<Identity: IMFOutputSchema_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwval: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFOutputSchema_Impl::GetConfigurationData(this) {
                    Ok(ok__) => {
                        pdwval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOriginatorID<Identity: IMFOutputSchema_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidoriginatorid: *mut windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFOutputSchema_Impl::GetOriginatorID(this) {
                    Ok(ok__) => {
                        pguidoriginatorid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IMFAttributes_Vtbl::new::<Identity, OFFSET>(),
            GetSchemaType: GetSchemaType::<Identity, OFFSET>,
            GetConfigurationData: GetConfigurationData::<Identity, OFFSET>,
            GetOriginatorID: GetOriginatorID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFOutputSchema as windows_core::Interface>::IID || iid == &<super::IMFAttributes as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMFOutputSchema {}
windows_core::imp::define_interface!(IMFOutputTrustAuthority, IMFOutputTrustAuthority_Vtbl, 0xd19f8e94_b126_4446_890c_5dcb7ad71453);
windows_core::imp::interface_hierarchy!(IMFOutputTrustAuthority, windows_core::IUnknown);
impl IMFOutputTrustAuthority {
    pub unsafe fn GetAction(&self) -> windows_core::Result<MFPOLICYMANAGER_ACTION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn SetPolicy(&self, pppolicy: Option<&[Option<IMFOutputPolicy>]>, ppbticket: *mut *mut u8, pcbticket: Option<*mut u32>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPolicy)(windows_core::Interface::as_raw(self), core::mem::transmute(pppolicy.map_or(core::ptr::null(), |slice| slice.as_ptr())), pppolicy.map_or(0, |slice| slice.len().try_into().unwrap()), ppbticket as _, pcbticket.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFOutputTrustAuthority_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MFPOLICYMANAGER_ACTION) -> windows_core::HRESULT,
    #[cfg(feature = "mfobjects")]
    pub SetPolicy: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void, u32, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    SetPolicy: usize,
}
#[cfg(feature = "mfobjects")]
pub trait IMFOutputTrustAuthority_Impl: windows_core::IUnknownImpl {
    fn GetAction(&self) -> windows_core::Result<MFPOLICYMANAGER_ACTION>;
    fn SetPolicy(&self, pppolicy: *const Option<IMFOutputPolicy>, npolicy: u32, ppbticket: *mut *mut u8, pcbticket: *mut u32) -> windows_core::Result<()>;
}
#[cfg(feature = "mfobjects")]
impl IMFOutputTrustAuthority_Vtbl {
    pub const fn new<Identity: IMFOutputTrustAuthority_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAction<Identity: IMFOutputTrustAuthority_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, paction: *mut MFPOLICYMANAGER_ACTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFOutputTrustAuthority_Impl::GetAction(this) {
                    Ok(ok__) => {
                        paction.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPolicy<Identity: IMFOutputTrustAuthority_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppolicy: *const *mut core::ffi::c_void, npolicy: u32, ppbticket: *mut *mut u8, pcbticket: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFOutputTrustAuthority_Impl::SetPolicy(this, core::mem::transmute_copy(&pppolicy), core::mem::transmute_copy(&npolicy), core::mem::transmute_copy(&ppbticket), core::mem::transmute_copy(&pcbticket)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAction: GetAction::<Identity, OFFSET>,
            SetPolicy: SetPolicy::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFOutputTrustAuthority as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFOutputTrustAuthority {}
windows_core::imp::define_interface!(IMFPMPClient, IMFPMPClient_Vtbl, 0x6c4e655d_ead8_4421_b6b9_54dcdbbdf820);
windows_core::imp::interface_hierarchy!(IMFPMPClient, windows_core::IUnknown);
impl IMFPMPClient {
    pub unsafe fn SetPMPHost<P0>(&self, ppmphost: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFPMPHost>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPMPHost)(windows_core::Interface::as_raw(self), ppmphost.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFPMPClient_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetPMPHost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFPMPClient_Impl: windows_core::IUnknownImpl {
    fn SetPMPHost(&self, ppmphost: windows_core::Ref<IMFPMPHost>) -> windows_core::Result<()>;
}
impl IMFPMPClient_Vtbl {
    pub const fn new<Identity: IMFPMPClient_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetPMPHost<Identity: IMFPMPClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmphost: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPMPClient_Impl::SetPMPHost(this, core::mem::transmute_copy(&ppmphost)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetPMPHost: SetPMPHost::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFPMPClient as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFPMPClient {}
windows_core::imp::define_interface!(IMFPMPClientApp, IMFPMPClientApp_Vtbl, 0xc004f646_be2c_48f3_93a2_a0983eba1108);
windows_core::imp::interface_hierarchy!(IMFPMPClientApp, windows_core::IUnknown);
impl IMFPMPClientApp {
    pub unsafe fn SetPMPHost<P0>(&self, ppmphost: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFPMPHostApp>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPMPHost)(windows_core::Interface::as_raw(self), ppmphost.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFPMPClientApp_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetPMPHost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFPMPClientApp_Impl: windows_core::IUnknownImpl {
    fn SetPMPHost(&self, ppmphost: windows_core::Ref<IMFPMPHostApp>) -> windows_core::Result<()>;
}
impl IMFPMPClientApp_Vtbl {
    pub const fn new<Identity: IMFPMPClientApp_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetPMPHost<Identity: IMFPMPClientApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmphost: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPMPClientApp_Impl::SetPMPHost(this, core::mem::transmute_copy(&ppmphost)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetPMPHost: SetPMPHost::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFPMPClientApp as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFPMPClientApp {}
windows_core::imp::define_interface!(IMFPMPHost, IMFPMPHost_Vtbl, 0xf70ca1a9_fdc7_4782_b994_adffb1c98606);
windows_core::imp::interface_hierarchy!(IMFPMPHost, windows_core::IUnknown);
impl IMFPMPHost {
    pub unsafe fn LockProcess(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LockProcess)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn UnlockProcess(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnlockProcess)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn CreateObjectByCLSID<P1, T>(&self, clsid: *const windows_core::GUID, pstream: P1) -> windows_core::Result<T>
    where
        P1: windows_core::Param<super::IStream>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateObjectByCLSID)(windows_core::Interface::as_raw(self), clsid, pstream.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFPMPHost_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub LockProcess: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnlockProcess: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "objidlbase")]
    pub CreateObjectByCLSID: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    CreateObjectByCLSID: usize,
}
#[cfg(feature = "objidlbase")]
pub trait IMFPMPHost_Impl: windows_core::IUnknownImpl {
    fn LockProcess(&self) -> windows_core::Result<()>;
    fn UnlockProcess(&self) -> windows_core::Result<()>;
    fn CreateObjectByCLSID(&self, clsid: *const windows_core::GUID, pstream: windows_core::Ref<super::IStream>, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "objidlbase")]
impl IMFPMPHost_Vtbl {
    pub const fn new<Identity: IMFPMPHost_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LockProcess<Identity: IMFPMPHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPMPHost_Impl::LockProcess(this).into()
            }
        }
        unsafe extern "system" fn UnlockProcess<Identity: IMFPMPHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPMPHost_Impl::UnlockProcess(this).into()
            }
        }
        unsafe extern "system" fn CreateObjectByCLSID<Identity: IMFPMPHost_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *const windows_core::GUID, pstream: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPMPHost_Impl::CreateObjectByCLSID(this, core::mem::transmute_copy(&clsid), core::mem::transmute_copy(&pstream), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            LockProcess: LockProcess::<Identity, OFFSET>,
            UnlockProcess: UnlockProcess::<Identity, OFFSET>,
            CreateObjectByCLSID: CreateObjectByCLSID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFPMPHost as windows_core::Interface>::IID
    }
}
#[cfg(feature = "objidlbase")]
impl windows_core::RuntimeName for IMFPMPHost {}
windows_core::imp::define_interface!(IMFPMPHostApp, IMFPMPHostApp_Vtbl, 0x84d2054a_3aa1_4728_a3b0_440a418cf49c);
windows_core::imp::interface_hierarchy!(IMFPMPHostApp, windows_core::IUnknown);
impl IMFPMPHostApp {
    pub unsafe fn LockProcess(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LockProcess)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn UnlockProcess(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnlockProcess)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn ActivateClassById<P0, P1, T>(&self, id: P0, pstream: P1) -> windows_core::Result<T>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::IStream>,
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).ActivateClassById)(windows_core::Interface::as_raw(self), id.param().abi(), pstream.param().abi(), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFPMPHostApp_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub LockProcess: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnlockProcess: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "objidlbase")]
    pub ActivateClassById: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    ActivateClassById: usize,
}
#[cfg(feature = "objidlbase")]
pub trait IMFPMPHostApp_Impl: windows_core::IUnknownImpl {
    fn LockProcess(&self) -> windows_core::Result<()>;
    fn UnlockProcess(&self) -> windows_core::Result<()>;
    fn ActivateClassById(&self, id: &windows_core::PCWSTR, pstream: windows_core::Ref<super::IStream>, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
#[cfg(feature = "objidlbase")]
impl IMFPMPHostApp_Vtbl {
    pub const fn new<Identity: IMFPMPHostApp_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LockProcess<Identity: IMFPMPHostApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPMPHostApp_Impl::LockProcess(this).into()
            }
        }
        unsafe extern "system" fn UnlockProcess<Identity: IMFPMPHostApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPMPHostApp_Impl::UnlockProcess(this).into()
            }
        }
        unsafe extern "system" fn ActivateClassById<Identity: IMFPMPHostApp_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, id: windows_core::PCWSTR, pstream: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPMPHostApp_Impl::ActivateClassById(this, core::mem::transmute(&id), core::mem::transmute_copy(&pstream), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            LockProcess: LockProcess::<Identity, OFFSET>,
            UnlockProcess: UnlockProcess::<Identity, OFFSET>,
            ActivateClassById: ActivateClassById::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFPMPHostApp as windows_core::Interface>::IID
    }
}
#[cfg(feature = "objidlbase")]
impl windows_core::RuntimeName for IMFPMPHostApp {}
windows_core::imp::define_interface!(IMFPMPServer, IMFPMPServer_Vtbl, 0x994e23af_1cc2_493c_b9fa_46f1cb040fa4);
windows_core::imp::interface_hierarchy!(IMFPMPServer, windows_core::IUnknown);
impl IMFPMPServer {
    pub unsafe fn LockProcess(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).LockProcess)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn UnlockProcess(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnlockProcess)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn CreateObjectByCLSID<T>(&self, clsid: *const windows_core::GUID) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).CreateObjectByCLSID)(windows_core::Interface::as_raw(self), clsid, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFPMPServer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub LockProcess: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnlockProcess: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateObjectByCLSID: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFPMPServer_Impl: windows_core::IUnknownImpl {
    fn LockProcess(&self) -> windows_core::Result<()>;
    fn UnlockProcess(&self) -> windows_core::Result<()>;
    fn CreateObjectByCLSID(&self, clsid: *const windows_core::GUID, riid: *const windows_core::GUID, ppobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IMFPMPServer_Vtbl {
    pub const fn new<Identity: IMFPMPServer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn LockProcess<Identity: IMFPMPServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPMPServer_Impl::LockProcess(this).into()
            }
        }
        unsafe extern "system" fn UnlockProcess<Identity: IMFPMPServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPMPServer_Impl::UnlockProcess(this).into()
            }
        }
        unsafe extern "system" fn CreateObjectByCLSID<Identity: IMFPMPServer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clsid: *const windows_core::GUID, riid: *const windows_core::GUID, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPMPServer_Impl::CreateObjectByCLSID(this, core::mem::transmute_copy(&clsid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppobject)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            LockProcess: LockProcess::<Identity, OFFSET>,
            UnlockProcess: UnlockProcess::<Identity, OFFSET>,
            CreateObjectByCLSID: CreateObjectByCLSID::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFPMPServer as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFPMPServer {}
windows_core::imp::define_interface!(IMFPresentationClock, IMFPresentationClock_Vtbl, 0x868ce85c_8ea9_4f55_ab82_b009a910a805);
impl core::ops::Deref for IMFPresentationClock {
    type Target = IMFClock;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMFPresentationClock, windows_core::IUnknown, IMFClock);
impl IMFPresentationClock {
    pub unsafe fn SetTimeSource<P0>(&self, ptimesource: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFPresentationTimeSource>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTimeSource)(windows_core::Interface::as_raw(self), ptimesource.param().abi()) }
    }
    pub unsafe fn GetTimeSource(&self) -> windows_core::Result<IMFPresentationTimeSource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTimeSource)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetTime(&self) -> windows_core::Result<MFTIME> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AddClockStateSink<P0>(&self, pstatesink: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFClockStateSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddClockStateSink)(windows_core::Interface::as_raw(self), pstatesink.param().abi()) }
    }
    pub unsafe fn RemoveClockStateSink<P0>(&self, pstatesink: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFClockStateSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveClockStateSink)(windows_core::Interface::as_raw(self), pstatesink.param().abi()) }
    }
    pub unsafe fn Start(&self, llclockstartoffset: i64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self), llclockstartoffset) }
    }
    pub unsafe fn Stop(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Pause(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Pause)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFPresentationClock_Vtbl {
    pub base__: IMFClock_Vtbl,
    pub SetTimeSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTimeSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MFTIME) -> windows_core::HRESULT,
    pub AddClockStateSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveClockStateSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Pause: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFPresentationClock_Impl: IMFClock_Impl {
    fn SetTimeSource(&self, ptimesource: windows_core::Ref<IMFPresentationTimeSource>) -> windows_core::Result<()>;
    fn GetTimeSource(&self) -> windows_core::Result<IMFPresentationTimeSource>;
    fn GetTime(&self) -> windows_core::Result<MFTIME>;
    fn AddClockStateSink(&self, pstatesink: windows_core::Ref<IMFClockStateSink>) -> windows_core::Result<()>;
    fn RemoveClockStateSink(&self, pstatesink: windows_core::Ref<IMFClockStateSink>) -> windows_core::Result<()>;
    fn Start(&self, llclockstartoffset: i64) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
    fn Pause(&self) -> windows_core::Result<()>;
}
impl IMFPresentationClock_Vtbl {
    pub const fn new<Identity: IMFPresentationClock_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetTimeSource<Identity: IMFPresentationClock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptimesource: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPresentationClock_Impl::SetTimeSource(this, core::mem::transmute_copy(&ptimesource)).into()
            }
        }
        unsafe extern "system" fn GetTimeSource<Identity: IMFPresentationClock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptimesource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFPresentationClock_Impl::GetTimeSource(this) {
                    Ok(ok__) => {
                        pptimesource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTime<Identity: IMFPresentationClock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phnsclocktime: *mut MFTIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFPresentationClock_Impl::GetTime(this) {
                    Ok(ok__) => {
                        phnsclocktime.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddClockStateSink<Identity: IMFPresentationClock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatesink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPresentationClock_Impl::AddClockStateSink(this, core::mem::transmute_copy(&pstatesink)).into()
            }
        }
        unsafe extern "system" fn RemoveClockStateSink<Identity: IMFPresentationClock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatesink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPresentationClock_Impl::RemoveClockStateSink(this, core::mem::transmute_copy(&pstatesink)).into()
            }
        }
        unsafe extern "system" fn Start<Identity: IMFPresentationClock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, llclockstartoffset: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPresentationClock_Impl::Start(this, core::mem::transmute_copy(&llclockstartoffset)).into()
            }
        }
        unsafe extern "system" fn Stop<Identity: IMFPresentationClock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPresentationClock_Impl::Stop(this).into()
            }
        }
        unsafe extern "system" fn Pause<Identity: IMFPresentationClock_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPresentationClock_Impl::Pause(this).into()
            }
        }
        Self {
            base__: IMFClock_Vtbl::new::<Identity, OFFSET>(),
            SetTimeSource: SetTimeSource::<Identity, OFFSET>,
            GetTimeSource: GetTimeSource::<Identity, OFFSET>,
            GetTime: GetTime::<Identity, OFFSET>,
            AddClockStateSink: AddClockStateSink::<Identity, OFFSET>,
            RemoveClockStateSink: RemoveClockStateSink::<Identity, OFFSET>,
            Start: Start::<Identity, OFFSET>,
            Stop: Stop::<Identity, OFFSET>,
            Pause: Pause::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFPresentationClock as windows_core::Interface>::IID || iid == &<IMFClock as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFPresentationClock {}
#[cfg(feature = "mfobjects")]
windows_core::imp::define_interface!(IMFPresentationDescriptor, IMFPresentationDescriptor_Vtbl, 0x03cb2711_24d7_4db6_a17f_f3a7a479a536);
#[cfg(feature = "mfobjects")]
impl core::ops::Deref for IMFPresentationDescriptor {
    type Target = super::IMFAttributes;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "mfobjects")]
windows_core::imp::interface_hierarchy!(IMFPresentationDescriptor, windows_core::IUnknown, super::IMFAttributes);
#[cfg(feature = "mfobjects")]
impl IMFPresentationDescriptor {
    pub unsafe fn GetStreamDescriptorCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStreamDescriptorCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetStreamDescriptorByIndex(&self, dwindex: u32, pfselected: *mut windows_core::BOOL, ppdescriptor: *mut Option<IMFStreamDescriptor>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetStreamDescriptorByIndex)(windows_core::Interface::as_raw(self), dwindex, pfselected as _, core::mem::transmute(ppdescriptor)) }
    }
    pub unsafe fn SelectStream(&self, dwdescriptorindex: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SelectStream)(windows_core::Interface::as_raw(self), dwdescriptorindex) }
    }
    pub unsafe fn DeselectStream(&self, dwdescriptorindex: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeselectStream)(windows_core::Interface::as_raw(self), dwdescriptorindex) }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "mfobjects")]
#[repr(C)]
#[doc(hidden)]
pub struct IMFPresentationDescriptor_Vtbl {
    pub base__: super::IMFAttributes_Vtbl,
    pub GetStreamDescriptorCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetStreamDescriptorByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SelectStream: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub DeselectStream: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMFPresentationDescriptor_Impl: super::IMFAttributes_Impl {
    fn GetStreamDescriptorCount(&self) -> windows_core::Result<u32>;
    fn GetStreamDescriptorByIndex(&self, dwindex: u32, pfselected: *mut windows_core::BOOL, ppdescriptor: windows_core::OutRef<IMFStreamDescriptor>) -> windows_core::Result<()>;
    fn SelectStream(&self, dwdescriptorindex: u32) -> windows_core::Result<()>;
    fn DeselectStream(&self, dwdescriptorindex: u32) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IMFPresentationDescriptor>;
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl IMFPresentationDescriptor_Vtbl {
    pub const fn new<Identity: IMFPresentationDescriptor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStreamDescriptorCount<Identity: IMFPresentationDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwdescriptorcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFPresentationDescriptor_Impl::GetStreamDescriptorCount(this) {
                    Ok(ok__) => {
                        pdwdescriptorcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStreamDescriptorByIndex<Identity: IMFPresentationDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, pfselected: *mut windows_core::BOOL, ppdescriptor: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPresentationDescriptor_Impl::GetStreamDescriptorByIndex(this, core::mem::transmute_copy(&dwindex), core::mem::transmute_copy(&pfselected), core::mem::transmute_copy(&ppdescriptor)).into()
            }
        }
        unsafe extern "system" fn SelectStream<Identity: IMFPresentationDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdescriptorindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPresentationDescriptor_Impl::SelectStream(this, core::mem::transmute_copy(&dwdescriptorindex)).into()
            }
        }
        unsafe extern "system" fn DeselectStream<Identity: IMFPresentationDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwdescriptorindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFPresentationDescriptor_Impl::DeselectStream(this, core::mem::transmute_copy(&dwdescriptorindex)).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IMFPresentationDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pppresentationdescriptor: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFPresentationDescriptor_Impl::Clone(this) {
                    Ok(ok__) => {
                        pppresentationdescriptor.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IMFAttributes_Vtbl::new::<Identity, OFFSET>(),
            GetStreamDescriptorCount: GetStreamDescriptorCount::<Identity, OFFSET>,
            GetStreamDescriptorByIndex: GetStreamDescriptorByIndex::<Identity, OFFSET>,
            SelectStream: SelectStream::<Identity, OFFSET>,
            DeselectStream: DeselectStream::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFPresentationDescriptor as windows_core::Interface>::IID || iid == &<super::IMFAttributes as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMFPresentationDescriptor {}
windows_core::imp::define_interface!(IMFPresentationTimeSource, IMFPresentationTimeSource_Vtbl, 0x7ff12cce_f76f_41c2_863b_1666c8e5e139);
impl core::ops::Deref for IMFPresentationTimeSource {
    type Target = IMFClock;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMFPresentationTimeSource, windows_core::IUnknown, IMFClock);
impl IMFPresentationTimeSource {
    pub unsafe fn GetUnderlyingClock(&self) -> windows_core::Result<IMFClock> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUnderlyingClock)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFPresentationTimeSource_Vtbl {
    pub base__: IMFClock_Vtbl,
    pub GetUnderlyingClock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFPresentationTimeSource_Impl: IMFClock_Impl {
    fn GetUnderlyingClock(&self) -> windows_core::Result<IMFClock>;
}
impl IMFPresentationTimeSource_Vtbl {
    pub const fn new<Identity: IMFPresentationTimeSource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetUnderlyingClock<Identity: IMFPresentationTimeSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppclock: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFPresentationTimeSource_Impl::GetUnderlyingClock(this) {
                    Ok(ok__) => {
                        ppclock.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IMFClock_Vtbl::new::<Identity, OFFSET>(), GetUnderlyingClock: GetUnderlyingClock::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFPresentationTimeSource as windows_core::Interface>::IID || iid == &<IMFClock as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFPresentationTimeSource {}
windows_core::imp::define_interface!(IMFProtectedEnvironmentAccess, IMFProtectedEnvironmentAccess_Vtbl, 0xef5dc845_f0d9_4ec9_b00c_cb5183d38434);
windows_core::imp::interface_hierarchy!(IMFProtectedEnvironmentAccess, windows_core::IUnknown);
impl IMFProtectedEnvironmentAccess {
    pub unsafe fn Call(&self, input: &[u8], output: &mut [u8]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Call)(windows_core::Interface::as_raw(self), input.len().try_into().unwrap(), input.as_ptr(), output.len().try_into().unwrap(), output.as_mut_ptr()) }
    }
    pub unsafe fn ReadGRL(&self, outputlength: *mut u32, output: *mut *mut u8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReadGRL)(windows_core::Interface::as_raw(self), outputlength as _, output as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFProtectedEnvironmentAccess_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Call: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, u32, *mut u8) -> windows_core::HRESULT,
    pub ReadGRL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
}
pub trait IMFProtectedEnvironmentAccess_Impl: windows_core::IUnknownImpl {
    fn Call(&self, inputlength: u32, input: *const u8, outputlength: u32, output: *mut u8) -> windows_core::Result<()>;
    fn ReadGRL(&self, outputlength: *mut u32, output: *mut *mut u8) -> windows_core::Result<()>;
}
impl IMFProtectedEnvironmentAccess_Vtbl {
    pub const fn new<Identity: IMFProtectedEnvironmentAccess_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Call<Identity: IMFProtectedEnvironmentAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputlength: u32, input: *const u8, outputlength: u32, output: *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFProtectedEnvironmentAccess_Impl::Call(this, core::mem::transmute_copy(&inputlength), core::mem::transmute_copy(&input), core::mem::transmute_copy(&outputlength), core::mem::transmute_copy(&output)).into()
            }
        }
        unsafe extern "system" fn ReadGRL<Identity: IMFProtectedEnvironmentAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, outputlength: *mut u32, output: *mut *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFProtectedEnvironmentAccess_Impl::ReadGRL(this, core::mem::transmute_copy(&outputlength), core::mem::transmute_copy(&output)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Call: Call::<Identity, OFFSET>, ReadGRL: ReadGRL::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFProtectedEnvironmentAccess as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFProtectedEnvironmentAccess {}
windows_core::imp::define_interface!(IMFQualityAdvise, IMFQualityAdvise_Vtbl, 0xec15e2e9_e36b_4f7c_8758_77d452ef4ce7);
windows_core::imp::interface_hierarchy!(IMFQualityAdvise, windows_core::IUnknown);
impl IMFQualityAdvise {
    pub unsafe fn SetDropMode(&self, edropmode: MF_QUALITY_DROP_MODE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDropMode)(windows_core::Interface::as_raw(self), edropmode) }
    }
    pub unsafe fn SetQualityLevel(&self, equalitylevel: MF_QUALITY_LEVEL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetQualityLevel)(windows_core::Interface::as_raw(self), equalitylevel) }
    }
    pub unsafe fn GetDropMode(&self) -> windows_core::Result<MF_QUALITY_DROP_MODE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDropMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetQualityLevel(&self) -> windows_core::Result<MF_QUALITY_LEVEL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetQualityLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DropTime(&self, hnsamounttodrop: i64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DropTime)(windows_core::Interface::as_raw(self), hnsamounttodrop) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFQualityAdvise_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetDropMode: unsafe extern "system" fn(*mut core::ffi::c_void, MF_QUALITY_DROP_MODE) -> windows_core::HRESULT,
    pub SetQualityLevel: unsafe extern "system" fn(*mut core::ffi::c_void, MF_QUALITY_LEVEL) -> windows_core::HRESULT,
    pub GetDropMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MF_QUALITY_DROP_MODE) -> windows_core::HRESULT,
    pub GetQualityLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MF_QUALITY_LEVEL) -> windows_core::HRESULT,
    pub DropTime: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
pub trait IMFQualityAdvise_Impl: windows_core::IUnknownImpl {
    fn SetDropMode(&self, edropmode: MF_QUALITY_DROP_MODE) -> windows_core::Result<()>;
    fn SetQualityLevel(&self, equalitylevel: MF_QUALITY_LEVEL) -> windows_core::Result<()>;
    fn GetDropMode(&self) -> windows_core::Result<MF_QUALITY_DROP_MODE>;
    fn GetQualityLevel(&self) -> windows_core::Result<MF_QUALITY_LEVEL>;
    fn DropTime(&self, hnsamounttodrop: i64) -> windows_core::Result<()>;
}
impl IMFQualityAdvise_Vtbl {
    pub const fn new<Identity: IMFQualityAdvise_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetDropMode<Identity: IMFQualityAdvise_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, edropmode: MF_QUALITY_DROP_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFQualityAdvise_Impl::SetDropMode(this, core::mem::transmute_copy(&edropmode)).into()
            }
        }
        unsafe extern "system" fn SetQualityLevel<Identity: IMFQualityAdvise_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, equalitylevel: MF_QUALITY_LEVEL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFQualityAdvise_Impl::SetQualityLevel(this, core::mem::transmute_copy(&equalitylevel)).into()
            }
        }
        unsafe extern "system" fn GetDropMode<Identity: IMFQualityAdvise_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pedropmode: *mut MF_QUALITY_DROP_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFQualityAdvise_Impl::GetDropMode(this) {
                    Ok(ok__) => {
                        pedropmode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetQualityLevel<Identity: IMFQualityAdvise_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pequalitylevel: *mut MF_QUALITY_LEVEL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFQualityAdvise_Impl::GetQualityLevel(this) {
                    Ok(ok__) => {
                        pequalitylevel.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DropTime<Identity: IMFQualityAdvise_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hnsamounttodrop: i64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFQualityAdvise_Impl::DropTime(this, core::mem::transmute_copy(&hnsamounttodrop)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetDropMode: SetDropMode::<Identity, OFFSET>,
            SetQualityLevel: SetQualityLevel::<Identity, OFFSET>,
            GetDropMode: GetDropMode::<Identity, OFFSET>,
            GetQualityLevel: GetQualityLevel::<Identity, OFFSET>,
            DropTime: DropTime::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFQualityAdvise as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFQualityAdvise {}
windows_core::imp::define_interface!(IMFQualityAdvise2, IMFQualityAdvise2_Vtbl, 0xf3706f0d_8ea2_4886_8000_7155e9ec2eae);
impl core::ops::Deref for IMFQualityAdvise2 {
    type Target = IMFQualityAdvise;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMFQualityAdvise2, windows_core::IUnknown, IMFQualityAdvise);
impl IMFQualityAdvise2 {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn NotifyQualityEvent<P0>(&self, pevent: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<super::IMFMediaEvent>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NotifyQualityEvent)(windows_core::Interface::as_raw(self), pevent.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFQualityAdvise2_Vtbl {
    pub base__: IMFQualityAdvise_Vtbl,
    #[cfg(feature = "mfobjects")]
    pub NotifyQualityEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    NotifyQualityEvent: usize,
}
#[cfg(feature = "mfobjects")]
pub trait IMFQualityAdvise2_Impl: IMFQualityAdvise_Impl {
    fn NotifyQualityEvent(&self, pevent: windows_core::Ref<super::IMFMediaEvent>) -> windows_core::Result<u32>;
}
#[cfg(feature = "mfobjects")]
impl IMFQualityAdvise2_Vtbl {
    pub const fn new<Identity: IMFQualityAdvise2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn NotifyQualityEvent<Identity: IMFQualityAdvise2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pevent: *mut core::ffi::c_void, pdwflags: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFQualityAdvise2_Impl::NotifyQualityEvent(this, core::mem::transmute_copy(&pevent)) {
                    Ok(ok__) => {
                        pdwflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IMFQualityAdvise_Vtbl::new::<Identity, OFFSET>(), NotifyQualityEvent: NotifyQualityEvent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFQualityAdvise2 as windows_core::Interface>::IID || iid == &<IMFQualityAdvise as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFQualityAdvise2 {}
windows_core::imp::define_interface!(IMFQualityAdviseLimits, IMFQualityAdviseLimits_Vtbl, 0xdfcd8e4d_30b5_4567_acaa_8eb5b7853dc9);
windows_core::imp::interface_hierarchy!(IMFQualityAdviseLimits, windows_core::IUnknown);
impl IMFQualityAdviseLimits {
    pub unsafe fn GetMaximumDropMode(&self) -> windows_core::Result<MF_QUALITY_DROP_MODE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaximumDropMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMinimumQualityLevel(&self) -> windows_core::Result<MF_QUALITY_LEVEL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMinimumQualityLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFQualityAdviseLimits_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetMaximumDropMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MF_QUALITY_DROP_MODE) -> windows_core::HRESULT,
    pub GetMinimumQualityLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MF_QUALITY_LEVEL) -> windows_core::HRESULT,
}
pub trait IMFQualityAdviseLimits_Impl: windows_core::IUnknownImpl {
    fn GetMaximumDropMode(&self) -> windows_core::Result<MF_QUALITY_DROP_MODE>;
    fn GetMinimumQualityLevel(&self) -> windows_core::Result<MF_QUALITY_LEVEL>;
}
impl IMFQualityAdviseLimits_Vtbl {
    pub const fn new<Identity: IMFQualityAdviseLimits_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMaximumDropMode<Identity: IMFQualityAdviseLimits_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pedropmode: *mut MF_QUALITY_DROP_MODE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFQualityAdviseLimits_Impl::GetMaximumDropMode(this) {
                    Ok(ok__) => {
                        pedropmode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMinimumQualityLevel<Identity: IMFQualityAdviseLimits_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pequalitylevel: *mut MF_QUALITY_LEVEL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFQualityAdviseLimits_Impl::GetMinimumQualityLevel(this) {
                    Ok(ok__) => {
                        pequalitylevel.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetMaximumDropMode: GetMaximumDropMode::<Identity, OFFSET>,
            GetMinimumQualityLevel: GetMinimumQualityLevel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFQualityAdviseLimits as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFQualityAdviseLimits {}
windows_core::imp::define_interface!(IMFQualityManager, IMFQualityManager_Vtbl, 0x8d009d86_5b9f_4115_b1fc_9f80d52ab8ab);
windows_core::imp::interface_hierarchy!(IMFQualityManager, windows_core::IUnknown);
impl IMFQualityManager {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn NotifyTopology<P0>(&self, ptopology: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFTopology>,
    {
        unsafe { (windows_core::Interface::vtable(self).NotifyTopology)(windows_core::Interface::as_raw(self), ptopology.param().abi()) }
    }
    pub unsafe fn NotifyPresentationClock<P0>(&self, pclock: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFPresentationClock>,
    {
        unsafe { (windows_core::Interface::vtable(self).NotifyPresentationClock)(windows_core::Interface::as_raw(self), pclock.param().abi()) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn NotifyProcessInput<P0, P2>(&self, pnode: P0, linputindex: i32, psample: P2) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFTopologyNode>,
        P2: windows_core::Param<super::IMFSample>,
    {
        unsafe { (windows_core::Interface::vtable(self).NotifyProcessInput)(windows_core::Interface::as_raw(self), pnode.param().abi(), linputindex, psample.param().abi()) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn NotifyProcessOutput<P0, P2>(&self, pnode: P0, loutputindex: i32, psample: P2) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFTopologyNode>,
        P2: windows_core::Param<super::IMFSample>,
    {
        unsafe { (windows_core::Interface::vtable(self).NotifyProcessOutput)(windows_core::Interface::as_raw(self), pnode.param().abi(), loutputindex, psample.param().abi()) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn NotifyQualityEvent<P0, P1>(&self, pobject: P0, pevent: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
        P1: windows_core::Param<super::IMFMediaEvent>,
    {
        unsafe { (windows_core::Interface::vtable(self).NotifyQualityEvent)(windows_core::Interface::as_raw(self), pobject.param().abi(), pevent.param().abi()) }
    }
    pub unsafe fn Shutdown(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Shutdown)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFQualityManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "mfobjects")]
    pub NotifyTopology: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    NotifyTopology: usize,
    pub NotifyPresentationClock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "mfobjects")]
    pub NotifyProcessInput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    NotifyProcessInput: usize,
    #[cfg(feature = "mfobjects")]
    pub NotifyProcessOutput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    NotifyProcessOutput: usize,
    #[cfg(feature = "mfobjects")]
    pub NotifyQualityEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    NotifyQualityEvent: usize,
    pub Shutdown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "mfobjects")]
pub trait IMFQualityManager_Impl: windows_core::IUnknownImpl {
    fn NotifyTopology(&self, ptopology: windows_core::Ref<IMFTopology>) -> windows_core::Result<()>;
    fn NotifyPresentationClock(&self, pclock: windows_core::Ref<IMFPresentationClock>) -> windows_core::Result<()>;
    fn NotifyProcessInput(&self, pnode: windows_core::Ref<IMFTopologyNode>, linputindex: i32, psample: windows_core::Ref<super::IMFSample>) -> windows_core::Result<()>;
    fn NotifyProcessOutput(&self, pnode: windows_core::Ref<IMFTopologyNode>, loutputindex: i32, psample: windows_core::Ref<super::IMFSample>) -> windows_core::Result<()>;
    fn NotifyQualityEvent(&self, pobject: windows_core::Ref<windows_core::IUnknown>, pevent: windows_core::Ref<super::IMFMediaEvent>) -> windows_core::Result<()>;
    fn Shutdown(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "mfobjects")]
impl IMFQualityManager_Vtbl {
    pub const fn new<Identity: IMFQualityManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn NotifyTopology<Identity: IMFQualityManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptopology: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFQualityManager_Impl::NotifyTopology(this, core::mem::transmute_copy(&ptopology)).into()
            }
        }
        unsafe extern "system" fn NotifyPresentationClock<Identity: IMFQualityManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pclock: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFQualityManager_Impl::NotifyPresentationClock(this, core::mem::transmute_copy(&pclock)).into()
            }
        }
        unsafe extern "system" fn NotifyProcessInput<Identity: IMFQualityManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnode: *mut core::ffi::c_void, linputindex: i32, psample: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFQualityManager_Impl::NotifyProcessInput(this, core::mem::transmute_copy(&pnode), core::mem::transmute_copy(&linputindex), core::mem::transmute_copy(&psample)).into()
            }
        }
        unsafe extern "system" fn NotifyProcessOutput<Identity: IMFQualityManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnode: *mut core::ffi::c_void, loutputindex: i32, psample: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFQualityManager_Impl::NotifyProcessOutput(this, core::mem::transmute_copy(&pnode), core::mem::transmute_copy(&loutputindex), core::mem::transmute_copy(&psample)).into()
            }
        }
        unsafe extern "system" fn NotifyQualityEvent<Identity: IMFQualityManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobject: *mut core::ffi::c_void, pevent: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFQualityManager_Impl::NotifyQualityEvent(this, core::mem::transmute_copy(&pobject), core::mem::transmute_copy(&pevent)).into()
            }
        }
        unsafe extern "system" fn Shutdown<Identity: IMFQualityManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFQualityManager_Impl::Shutdown(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            NotifyTopology: NotifyTopology::<Identity, OFFSET>,
            NotifyPresentationClock: NotifyPresentationClock::<Identity, OFFSET>,
            NotifyProcessInput: NotifyProcessInput::<Identity, OFFSET>,
            NotifyProcessOutput: NotifyProcessOutput::<Identity, OFFSET>,
            NotifyQualityEvent: NotifyQualityEvent::<Identity, OFFSET>,
            Shutdown: Shutdown::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFQualityManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFQualityManager {}
windows_core::imp::define_interface!(IMFRateControl, IMFRateControl_Vtbl, 0x88ddcd21_03c3_4275_91ed_55ee3929328f);
windows_core::imp::interface_hierarchy!(IMFRateControl, windows_core::IUnknown);
impl IMFRateControl {
    pub unsafe fn SetRate(&self, fthin: bool, flrate: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRate)(windows_core::Interface::as_raw(self), fthin.into(), flrate) }
    }
    pub unsafe fn GetRate(&self, pfthin: *mut windows_core::BOOL, pflrate: *mut f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRate)(windows_core::Interface::as_raw(self), pfthin as _, pflrate as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFRateControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetRate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, f32) -> windows_core::HRESULT,
    pub GetRate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL, *mut f32) -> windows_core::HRESULT,
}
pub trait IMFRateControl_Impl: windows_core::IUnknownImpl {
    fn SetRate(&self, fthin: windows_core::BOOL, flrate: f32) -> windows_core::Result<()>;
    fn GetRate(&self, pfthin: *mut windows_core::BOOL, pflrate: *mut f32) -> windows_core::Result<()>;
}
impl IMFRateControl_Vtbl {
    pub const fn new<Identity: IMFRateControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetRate<Identity: IMFRateControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fthin: windows_core::BOOL, flrate: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFRateControl_Impl::SetRate(this, core::mem::transmute_copy(&fthin), core::mem::transmute_copy(&flrate)).into()
            }
        }
        unsafe extern "system" fn GetRate<Identity: IMFRateControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfthin: *mut windows_core::BOOL, pflrate: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFRateControl_Impl::GetRate(this, core::mem::transmute_copy(&pfthin), core::mem::transmute_copy(&pflrate)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetRate: SetRate::<Identity, OFFSET>, GetRate: GetRate::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFRateControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFRateControl {}
windows_core::imp::define_interface!(IMFRateSupport, IMFRateSupport_Vtbl, 0x0a9ccdbc_d797_4563_9667_94ec5d79292d);
windows_core::imp::interface_hierarchy!(IMFRateSupport, windows_core::IUnknown);
impl IMFRateSupport {
    pub unsafe fn GetSlowestRate(&self, edirection: MFRATE_DIRECTION, fthin: bool) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSlowestRate)(windows_core::Interface::as_raw(self), edirection, fthin.into(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetFastestRate(&self, edirection: MFRATE_DIRECTION, fthin: bool) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFastestRate)(windows_core::Interface::as_raw(self), edirection, fthin.into(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsRateSupported(&self, fthin: bool, flrate: f32, pflnearestsupportedrate: *mut f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).IsRateSupported)(windows_core::Interface::as_raw(self), fthin.into(), flrate, pflnearestsupportedrate as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFRateSupport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSlowestRate: unsafe extern "system" fn(*mut core::ffi::c_void, MFRATE_DIRECTION, windows_core::BOOL, *mut f32) -> windows_core::HRESULT,
    pub GetFastestRate: unsafe extern "system" fn(*mut core::ffi::c_void, MFRATE_DIRECTION, windows_core::BOOL, *mut f32) -> windows_core::HRESULT,
    pub IsRateSupported: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, f32, *mut f32) -> windows_core::HRESULT,
}
pub trait IMFRateSupport_Impl: windows_core::IUnknownImpl {
    fn GetSlowestRate(&self, edirection: MFRATE_DIRECTION, fthin: windows_core::BOOL) -> windows_core::Result<f32>;
    fn GetFastestRate(&self, edirection: MFRATE_DIRECTION, fthin: windows_core::BOOL) -> windows_core::Result<f32>;
    fn IsRateSupported(&self, fthin: windows_core::BOOL, flrate: f32, pflnearestsupportedrate: *mut f32) -> windows_core::Result<()>;
}
impl IMFRateSupport_Vtbl {
    pub const fn new<Identity: IMFRateSupport_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSlowestRate<Identity: IMFRateSupport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, edirection: MFRATE_DIRECTION, fthin: windows_core::BOOL, pflrate: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFRateSupport_Impl::GetSlowestRate(this, core::mem::transmute_copy(&edirection), core::mem::transmute_copy(&fthin)) {
                    Ok(ok__) => {
                        pflrate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFastestRate<Identity: IMFRateSupport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, edirection: MFRATE_DIRECTION, fthin: windows_core::BOOL, pflrate: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFRateSupport_Impl::GetFastestRate(this, core::mem::transmute_copy(&edirection), core::mem::transmute_copy(&fthin)) {
                    Ok(ok__) => {
                        pflrate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsRateSupported<Identity: IMFRateSupport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fthin: windows_core::BOOL, flrate: f32, pflnearestsupportedrate: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFRateSupport_Impl::IsRateSupported(this, core::mem::transmute_copy(&fthin), core::mem::transmute_copy(&flrate), core::mem::transmute_copy(&pflnearestsupportedrate)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSlowestRate: GetSlowestRate::<Identity, OFFSET>,
            GetFastestRate: GetFastestRate::<Identity, OFFSET>,
            IsRateSupported: IsRateSupported::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFRateSupport as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFRateSupport {}
windows_core::imp::define_interface!(IMFRealTimeClient, IMFRealTimeClient_Vtbl, 0x2347d60b_3fb5_480c_8803_8df3adcd3ef0);
windows_core::imp::interface_hierarchy!(IMFRealTimeClient, windows_core::IUnknown);
impl IMFRealTimeClient {
    pub unsafe fn RegisterThreads<P1>(&self, dwtaskindex: u32, wszclass: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterThreads)(windows_core::Interface::as_raw(self), dwtaskindex, wszclass.param().abi()) }
    }
    pub unsafe fn UnregisterThreads(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnregisterThreads)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetWorkQueue(&self, dwworkqueueid: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetWorkQueue)(windows_core::Interface::as_raw(self), dwworkqueueid) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFRealTimeClient_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterThreads: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub UnregisterThreads: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetWorkQueue: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
pub trait IMFRealTimeClient_Impl: windows_core::IUnknownImpl {
    fn RegisterThreads(&self, dwtaskindex: u32, wszclass: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn UnregisterThreads(&self) -> windows_core::Result<()>;
    fn SetWorkQueue(&self, dwworkqueueid: u32) -> windows_core::Result<()>;
}
impl IMFRealTimeClient_Vtbl {
    pub const fn new<Identity: IMFRealTimeClient_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterThreads<Identity: IMFRealTimeClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwtaskindex: u32, wszclass: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFRealTimeClient_Impl::RegisterThreads(this, core::mem::transmute_copy(&dwtaskindex), core::mem::transmute(&wszclass)).into()
            }
        }
        unsafe extern "system" fn UnregisterThreads<Identity: IMFRealTimeClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFRealTimeClient_Impl::UnregisterThreads(this).into()
            }
        }
        unsafe extern "system" fn SetWorkQueue<Identity: IMFRealTimeClient_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwworkqueueid: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFRealTimeClient_Impl::SetWorkQueue(this, core::mem::transmute_copy(&dwworkqueueid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterThreads: RegisterThreads::<Identity, OFFSET>,
            UnregisterThreads: UnregisterThreads::<Identity, OFFSET>,
            SetWorkQueue: SetWorkQueue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFRealTimeClient as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFRealTimeClient {}
windows_core::imp::define_interface!(IMFRealTimeClientEx, IMFRealTimeClientEx_Vtbl, 0x03910848_ab16_4611_b100_17b88ae2f248);
windows_core::imp::interface_hierarchy!(IMFRealTimeClientEx, windows_core::IUnknown);
impl IMFRealTimeClientEx {
    pub unsafe fn RegisterThreadsEx<P1>(&self, pdwtaskindex: *mut u32, wszclassname: P1, lbasepriority: i32) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).RegisterThreadsEx)(windows_core::Interface::as_raw(self), pdwtaskindex as _, wszclassname.param().abi(), lbasepriority) }
    }
    pub unsafe fn UnregisterThreads(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnregisterThreads)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetWorkQueueEx(&self, dwmultithreadedworkqueueid: u32, lworkitembasepriority: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetWorkQueueEx)(windows_core::Interface::as_raw(self), dwmultithreadedworkqueueid, lworkitembasepriority) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFRealTimeClientEx_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterThreadsEx: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, windows_core::PCWSTR, i32) -> windows_core::HRESULT,
    pub UnregisterThreads: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetWorkQueueEx: unsafe extern "system" fn(*mut core::ffi::c_void, u32, i32) -> windows_core::HRESULT,
}
pub trait IMFRealTimeClientEx_Impl: windows_core::IUnknownImpl {
    fn RegisterThreadsEx(&self, pdwtaskindex: *mut u32, wszclassname: &windows_core::PCWSTR, lbasepriority: i32) -> windows_core::Result<()>;
    fn UnregisterThreads(&self) -> windows_core::Result<()>;
    fn SetWorkQueueEx(&self, dwmultithreadedworkqueueid: u32, lworkitembasepriority: i32) -> windows_core::Result<()>;
}
impl IMFRealTimeClientEx_Vtbl {
    pub const fn new<Identity: IMFRealTimeClientEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterThreadsEx<Identity: IMFRealTimeClientEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwtaskindex: *mut u32, wszclassname: windows_core::PCWSTR, lbasepriority: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFRealTimeClientEx_Impl::RegisterThreadsEx(this, core::mem::transmute_copy(&pdwtaskindex), core::mem::transmute(&wszclassname), core::mem::transmute_copy(&lbasepriority)).into()
            }
        }
        unsafe extern "system" fn UnregisterThreads<Identity: IMFRealTimeClientEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFRealTimeClientEx_Impl::UnregisterThreads(this).into()
            }
        }
        unsafe extern "system" fn SetWorkQueueEx<Identity: IMFRealTimeClientEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmultithreadedworkqueueid: u32, lworkitembasepriority: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFRealTimeClientEx_Impl::SetWorkQueueEx(this, core::mem::transmute_copy(&dwmultithreadedworkqueueid), core::mem::transmute_copy(&lworkitembasepriority)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterThreadsEx: RegisterThreadsEx::<Identity, OFFSET>,
            UnregisterThreads: UnregisterThreads::<Identity, OFFSET>,
            SetWorkQueueEx: SetWorkQueueEx::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFRealTimeClientEx as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFRealTimeClientEx {}
windows_core::imp::define_interface!(IMFRelativePanelReport, IMFRelativePanelReport_Vtbl, 0xf25362ea_2c0e_447f_81e2_755914cdc0c3);
windows_core::imp::interface_hierarchy!(IMFRelativePanelReport, windows_core::IUnknown);
impl IMFRelativePanelReport {
    pub unsafe fn GetRelativePanel(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRelativePanel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFRelativePanelReport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetRelativePanel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IMFRelativePanelReport_Impl: windows_core::IUnknownImpl {
    fn GetRelativePanel(&self) -> windows_core::Result<u32>;
}
impl IMFRelativePanelReport_Vtbl {
    pub const fn new<Identity: IMFRelativePanelReport_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRelativePanel<Identity: IMFRelativePanelReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, panel: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFRelativePanelReport_Impl::GetRelativePanel(this) {
                    Ok(ok__) => {
                        panel.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetRelativePanel: GetRelativePanel::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFRelativePanelReport as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFRelativePanelReport {}
windows_core::imp::define_interface!(IMFRelativePanelWatcher, IMFRelativePanelWatcher_Vtbl, 0x421af7f6_573e_4ad0_8fda_2e57cedb18c6);
impl core::ops::Deref for IMFRelativePanelWatcher {
    type Target = IMFShutdown;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMFRelativePanelWatcher, windows_core::IUnknown, IMFShutdown);
impl IMFRelativePanelWatcher {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn BeginGetReport<P0, P1>(&self, pcallback: P0, pstate: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMFAsyncCallback>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).BeginGetReport)(windows_core::Interface::as_raw(self), pcallback.param().abi(), pstate.param().abi()) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn EndGetReport<P0>(&self, presult: P0) -> windows_core::Result<IMFRelativePanelReport>
    where
        P0: windows_core::Param<super::IMFAsyncResult>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EndGetReport)(windows_core::Interface::as_raw(self), presult.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetReport(&self) -> windows_core::Result<IMFRelativePanelReport> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetReport)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFRelativePanelWatcher_Vtbl {
    pub base__: IMFShutdown_Vtbl,
    #[cfg(feature = "mfobjects")]
    pub BeginGetReport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    BeginGetReport: usize,
    #[cfg(feature = "mfobjects")]
    pub EndGetReport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    EndGetReport: usize,
    pub GetReport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "mfobjects")]
pub trait IMFRelativePanelWatcher_Impl: IMFShutdown_Impl {
    fn BeginGetReport(&self, pcallback: windows_core::Ref<super::IMFAsyncCallback>, pstate: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn EndGetReport(&self, presult: windows_core::Ref<super::IMFAsyncResult>) -> windows_core::Result<IMFRelativePanelReport>;
    fn GetReport(&self) -> windows_core::Result<IMFRelativePanelReport>;
}
#[cfg(feature = "mfobjects")]
impl IMFRelativePanelWatcher_Vtbl {
    pub const fn new<Identity: IMFRelativePanelWatcher_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BeginGetReport<Identity: IMFRelativePanelWatcher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void, pstate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFRelativePanelWatcher_Impl::BeginGetReport(this, core::mem::transmute_copy(&pcallback), core::mem::transmute_copy(&pstate)).into()
            }
        }
        unsafe extern "system" fn EndGetReport<Identity: IMFRelativePanelWatcher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presult: *mut core::ffi::c_void, pprelativepanelreport: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFRelativePanelWatcher_Impl::EndGetReport(this, core::mem::transmute_copy(&presult)) {
                    Ok(ok__) => {
                        pprelativepanelreport.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetReport<Identity: IMFRelativePanelWatcher_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprelativepanelreport: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFRelativePanelWatcher_Impl::GetReport(this) {
                    Ok(ok__) => {
                        pprelativepanelreport.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IMFShutdown_Vtbl::new::<Identity, OFFSET>(),
            BeginGetReport: BeginGetReport::<Identity, OFFSET>,
            EndGetReport: EndGetReport::<Identity, OFFSET>,
            GetReport: GetReport::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFRelativePanelWatcher as windows_core::Interface>::IID || iid == &<IMFShutdown as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFRelativePanelWatcher {}
windows_core::imp::define_interface!(IMFRemoteDesktopPlugin, IMFRemoteDesktopPlugin_Vtbl, 0x1cde6309_cae0_4940_907e_c1ec9c3d1d4a);
windows_core::imp::interface_hierarchy!(IMFRemoteDesktopPlugin, windows_core::IUnknown);
impl IMFRemoteDesktopPlugin {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn UpdateTopology(&self, ptopology: &Option<IMFTopology>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UpdateTopology)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(ptopology)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFRemoteDesktopPlugin_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "mfobjects")]
    pub UpdateTopology: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    UpdateTopology: usize,
}
#[cfg(feature = "mfobjects")]
pub trait IMFRemoteDesktopPlugin_Impl: windows_core::IUnknownImpl {
    fn UpdateTopology(&self, ptopology: windows_core::OutRef<IMFTopology>) -> windows_core::Result<()>;
}
#[cfg(feature = "mfobjects")]
impl IMFRemoteDesktopPlugin_Vtbl {
    pub const fn new<Identity: IMFRemoteDesktopPlugin_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn UpdateTopology<Identity: IMFRemoteDesktopPlugin_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptopology: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFRemoteDesktopPlugin_Impl::UpdateTopology(this, core::mem::transmute(&ptopology)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), UpdateTopology: UpdateTopology::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFRemoteDesktopPlugin as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFRemoteDesktopPlugin {}
windows_core::imp::define_interface!(IMFRemoteProxy, IMFRemoteProxy_Vtbl, 0x994e23ad_1cc2_493c_b9fa_46f1cb040fa4);
windows_core::imp::interface_hierarchy!(IMFRemoteProxy, windows_core::IUnknown);
impl IMFRemoteProxy {
    pub unsafe fn GetRemoteObject<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetRemoteObject)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    pub unsafe fn GetRemoteHost<T>(&self) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetRemoteHost)(windows_core::Interface::as_raw(self), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFRemoteProxy_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetRemoteObject: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRemoteHost: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFRemoteProxy_Impl: windows_core::IUnknownImpl {
    fn GetRemoteObject(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetRemoteHost(&self, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IMFRemoteProxy_Vtbl {
    pub const fn new<Identity: IMFRemoteProxy_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRemoteObject<Identity: IMFRemoteProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFRemoteProxy_Impl::GetRemoteObject(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        unsafe extern "system" fn GetRemoteHost<Identity: IMFRemoteProxy_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFRemoteProxy_Impl::GetRemoteHost(this, core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppv)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRemoteObject: GetRemoteObject::<Identity, OFFSET>,
            GetRemoteHost: GetRemoteHost::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFRemoteProxy as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFRemoteProxy {}
windows_core::imp::define_interface!(IMFSAMIStyle, IMFSAMIStyle_Vtbl, 0xa7e025dd_5303_4a62_89d6_e747e1efac73);
windows_core::imp::interface_hierarchy!(IMFSAMIStyle, windows_core::IUnknown);
impl IMFSAMIStyle {
    pub unsafe fn GetStyleCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStyleCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetStyles(&self) -> windows_core::Result<super::PROPVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStyles)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetSelectedStyle<P0>(&self, pwszstyle: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetSelectedStyle)(windows_core::Interface::as_raw(self), pwszstyle.param().abi()) }
    }
    pub unsafe fn GetSelectedStyle(&self) -> windows_core::Result<windows_core::PWSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSelectedStyle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFSAMIStyle_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetStyleCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub GetStyles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    GetStyles: usize,
    pub SetSelectedStyle: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetSelectedStyle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::PWSTR) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMFSAMIStyle_Impl: windows_core::IUnknownImpl {
    fn GetStyleCount(&self) -> windows_core::Result<u32>;
    fn GetStyles(&self) -> windows_core::Result<super::PROPVARIANT>;
    fn SetSelectedStyle(&self, pwszstyle: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetSelectedStyle(&self) -> windows_core::Result<windows_core::PWSTR>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl IMFSAMIStyle_Vtbl {
    pub const fn new<Identity: IMFSAMIStyle_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStyleCount<Identity: IMFSAMIStyle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSAMIStyle_Impl::GetStyleCount(this) {
                    Ok(ok__) => {
                        pdwcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStyles<Identity: IMFSAMIStyle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppropvarstylearray: *mut super::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSAMIStyle_Impl::GetStyles(this) {
                    Ok(ok__) => {
                        ppropvarstylearray.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSelectedStyle<Identity: IMFSAMIStyle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszstyle: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSAMIStyle_Impl::SetSelectedStyle(this, core::mem::transmute(&pwszstyle)).into()
            }
        }
        unsafe extern "system" fn GetSelectedStyle<Identity: IMFSAMIStyle_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppwszstyle: *mut windows_core::PWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSAMIStyle_Impl::GetSelectedStyle(this) {
                    Ok(ok__) => {
                        ppwszstyle.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetStyleCount: GetStyleCount::<Identity, OFFSET>,
            GetStyles: GetStyles::<Identity, OFFSET>,
            SetSelectedStyle: SetSelectedStyle::<Identity, OFFSET>,
            GetSelectedStyle: GetSelectedStyle::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFSAMIStyle as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMFSAMIStyle {}
windows_core::imp::define_interface!(IMFSSLCertificateManager, IMFSSLCertificateManager_Vtbl, 0x61f7d887_1230_4a8b_aeba_8ad434d1a64d);
windows_core::imp::interface_hierarchy!(IMFSSLCertificateManager, windows_core::IUnknown);
impl IMFSSLCertificateManager {
    pub unsafe fn GetClientCertificate<P0>(&self, pszurl: P0, ppbdata: *mut *mut u8, pcbdata: *mut u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetClientCertificate)(windows_core::Interface::as_raw(self), pszurl.param().abi(), ppbdata as _, pcbdata as _) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn BeginGetClientCertificate<P0, P1, P2>(&self, pszurl: P0, pcallback: P1, pstate: P2) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<super::IMFAsyncCallback>,
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).BeginGetClientCertificate)(windows_core::Interface::as_raw(self), pszurl.param().abi(), pcallback.param().abi(), pstate.param().abi()) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn EndGetClientCertificate<P0>(&self, presult: P0, ppbdata: *mut *mut u8, pcbdata: *mut u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMFAsyncResult>,
    {
        unsafe { (windows_core::Interface::vtable(self).EndGetClientCertificate)(windows_core::Interface::as_raw(self), presult.param().abi(), ppbdata as _, pcbdata as _) }
    }
    pub unsafe fn GetCertificatePolicy<P0>(&self, pszurl: P0, pfoverrideautomaticcheck: *mut windows_core::BOOL, pfclientcertificateavailable: *mut windows_core::BOOL) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetCertificatePolicy)(windows_core::Interface::as_raw(self), pszurl.param().abi(), pfoverrideautomaticcheck as _, pfclientcertificateavailable as _) }
    }
    pub unsafe fn OnServerCertificate<P0>(&self, pszurl: P0, pbdata: &[u8]) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).OnServerCertificate)(windows_core::Interface::as_raw(self), pszurl.param().abi(), pbdata.as_ptr(), pbdata.len().try_into().unwrap(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFSSLCertificateManager_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetClientCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "mfobjects")]
    pub BeginGetClientCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    BeginGetClientCertificate: usize,
    #[cfg(feature = "mfobjects")]
    pub EndGetClientCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    EndGetClientCertificate: usize,
    pub GetCertificatePolicy: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::BOOL, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub OnServerCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *const u8, u32, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "mfobjects")]
pub trait IMFSSLCertificateManager_Impl: windows_core::IUnknownImpl {
    fn GetClientCertificate(&self, pszurl: &windows_core::PCWSTR, ppbdata: *mut *mut u8, pcbdata: *mut u32) -> windows_core::Result<()>;
    fn BeginGetClientCertificate(&self, pszurl: &windows_core::PCWSTR, pcallback: windows_core::Ref<super::IMFAsyncCallback>, pstate: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn EndGetClientCertificate(&self, presult: windows_core::Ref<super::IMFAsyncResult>, ppbdata: *mut *mut u8, pcbdata: *mut u32) -> windows_core::Result<()>;
    fn GetCertificatePolicy(&self, pszurl: &windows_core::PCWSTR, pfoverrideautomaticcheck: *mut windows_core::BOOL, pfclientcertificateavailable: *mut windows_core::BOOL) -> windows_core::Result<()>;
    fn OnServerCertificate(&self, pszurl: &windows_core::PCWSTR, pbdata: *const u8, cbdata: u32) -> windows_core::Result<windows_core::BOOL>;
}
#[cfg(feature = "mfobjects")]
impl IMFSSLCertificateManager_Vtbl {
    pub const fn new<Identity: IMFSSLCertificateManager_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetClientCertificate<Identity: IMFSSLCertificateManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszurl: windows_core::PCWSTR, ppbdata: *mut *mut u8, pcbdata: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSSLCertificateManager_Impl::GetClientCertificate(this, core::mem::transmute(&pszurl), core::mem::transmute_copy(&ppbdata), core::mem::transmute_copy(&pcbdata)).into()
            }
        }
        unsafe extern "system" fn BeginGetClientCertificate<Identity: IMFSSLCertificateManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszurl: windows_core::PCWSTR, pcallback: *mut core::ffi::c_void, pstate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSSLCertificateManager_Impl::BeginGetClientCertificate(this, core::mem::transmute(&pszurl), core::mem::transmute_copy(&pcallback), core::mem::transmute_copy(&pstate)).into()
            }
        }
        unsafe extern "system" fn EndGetClientCertificate<Identity: IMFSSLCertificateManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presult: *mut core::ffi::c_void, ppbdata: *mut *mut u8, pcbdata: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSSLCertificateManager_Impl::EndGetClientCertificate(this, core::mem::transmute_copy(&presult), core::mem::transmute_copy(&ppbdata), core::mem::transmute_copy(&pcbdata)).into()
            }
        }
        unsafe extern "system" fn GetCertificatePolicy<Identity: IMFSSLCertificateManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszurl: windows_core::PCWSTR, pfoverrideautomaticcheck: *mut windows_core::BOOL, pfclientcertificateavailable: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSSLCertificateManager_Impl::GetCertificatePolicy(this, core::mem::transmute(&pszurl), core::mem::transmute_copy(&pfoverrideautomaticcheck), core::mem::transmute_copy(&pfclientcertificateavailable)).into()
            }
        }
        unsafe extern "system" fn OnServerCertificate<Identity: IMFSSLCertificateManager_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszurl: windows_core::PCWSTR, pbdata: *const u8, cbdata: u32, pfisgood: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSSLCertificateManager_Impl::OnServerCertificate(this, core::mem::transmute(&pszurl), core::mem::transmute_copy(&pbdata), core::mem::transmute_copy(&cbdata)) {
                    Ok(ok__) => {
                        pfisgood.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetClientCertificate: GetClientCertificate::<Identity, OFFSET>,
            BeginGetClientCertificate: BeginGetClientCertificate::<Identity, OFFSET>,
            EndGetClientCertificate: EndGetClientCertificate::<Identity, OFFSET>,
            GetCertificatePolicy: GetCertificatePolicy::<Identity, OFFSET>,
            OnServerCertificate: OnServerCertificate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFSSLCertificateManager as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFSSLCertificateManager {}
windows_core::imp::define_interface!(IMFSampleAllocatorControl, IMFSampleAllocatorControl_Vtbl, 0xda62b958_3a38_4a97_bd27_149c640c0771);
windows_core::imp::interface_hierarchy!(IMFSampleAllocatorControl, windows_core::IUnknown);
impl IMFSampleAllocatorControl {
    pub unsafe fn SetDefaultAllocator<P1>(&self, dwoutputstreamid: u32, pallocator: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDefaultAllocator)(windows_core::Interface::as_raw(self), dwoutputstreamid, pallocator.param().abi()) }
    }
    pub unsafe fn GetAllocatorUsage(&self, dwoutputstreamid: u32, pdwinputstreamid: *mut u32, peusage: *mut MFSampleAllocatorUsage) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAllocatorUsage)(windows_core::Interface::as_raw(self), dwoutputstreamid, pdwinputstreamid as _, peusage as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFSampleAllocatorControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetDefaultAllocator: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAllocatorUsage: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32, *mut MFSampleAllocatorUsage) -> windows_core::HRESULT,
}
pub trait IMFSampleAllocatorControl_Impl: windows_core::IUnknownImpl {
    fn SetDefaultAllocator(&self, dwoutputstreamid: u32, pallocator: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetAllocatorUsage(&self, dwoutputstreamid: u32, pdwinputstreamid: *mut u32, peusage: *mut MFSampleAllocatorUsage) -> windows_core::Result<()>;
}
impl IMFSampleAllocatorControl_Vtbl {
    pub const fn new<Identity: IMFSampleAllocatorControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetDefaultAllocator<Identity: IMFSampleAllocatorControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputstreamid: u32, pallocator: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSampleAllocatorControl_Impl::SetDefaultAllocator(this, core::mem::transmute_copy(&dwoutputstreamid), core::mem::transmute_copy(&pallocator)).into()
            }
        }
        unsafe extern "system" fn GetAllocatorUsage<Identity: IMFSampleAllocatorControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputstreamid: u32, pdwinputstreamid: *mut u32, peusage: *mut MFSampleAllocatorUsage) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSampleAllocatorControl_Impl::GetAllocatorUsage(this, core::mem::transmute_copy(&dwoutputstreamid), core::mem::transmute_copy(&pdwinputstreamid), core::mem::transmute_copy(&peusage)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetDefaultAllocator: SetDefaultAllocator::<Identity, OFFSET>,
            GetAllocatorUsage: GetAllocatorUsage::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFSampleAllocatorControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFSampleAllocatorControl {}
windows_core::imp::define_interface!(IMFSampleGrabberSinkCallback, IMFSampleGrabberSinkCallback_Vtbl, 0x8c7b80bf_ee42_4b59_b1df_55668e1bdca8);
impl core::ops::Deref for IMFSampleGrabberSinkCallback {
    type Target = IMFClockStateSink;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMFSampleGrabberSinkCallback, windows_core::IUnknown, IMFClockStateSink);
impl IMFSampleGrabberSinkCallback {
    pub unsafe fn OnSetPresentationClock<P0>(&self, ppresentationclock: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFPresentationClock>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnSetPresentationClock)(windows_core::Interface::as_raw(self), ppresentationclock.param().abi()) }
    }
    pub unsafe fn OnProcessSample(&self, guidmajormediatype: *const windows_core::GUID, dwsampleflags: u32, llsampletime: i64, llsampleduration: i64, psamplebuffer: &[u8]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnProcessSample)(windows_core::Interface::as_raw(self), guidmajormediatype, dwsampleflags, llsampletime, llsampleduration, psamplebuffer.as_ptr(), psamplebuffer.len().try_into().unwrap()) }
    }
    pub unsafe fn OnShutdown(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnShutdown)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFSampleGrabberSinkCallback_Vtbl {
    pub base__: IMFClockStateSink_Vtbl,
    pub OnSetPresentationClock: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnProcessSample: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, i64, i64, *const u8, u32) -> windows_core::HRESULT,
    pub OnShutdown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFSampleGrabberSinkCallback_Impl: IMFClockStateSink_Impl {
    fn OnSetPresentationClock(&self, ppresentationclock: windows_core::Ref<IMFPresentationClock>) -> windows_core::Result<()>;
    fn OnProcessSample(&self, guidmajormediatype: *const windows_core::GUID, dwsampleflags: u32, llsampletime: i64, llsampleduration: i64, psamplebuffer: *const u8, dwsamplesize: u32) -> windows_core::Result<()>;
    fn OnShutdown(&self) -> windows_core::Result<()>;
}
impl IMFSampleGrabberSinkCallback_Vtbl {
    pub const fn new<Identity: IMFSampleGrabberSinkCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnSetPresentationClock<Identity: IMFSampleGrabberSinkCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppresentationclock: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSampleGrabberSinkCallback_Impl::OnSetPresentationClock(this, core::mem::transmute_copy(&ppresentationclock)).into()
            }
        }
        unsafe extern "system" fn OnProcessSample<Identity: IMFSampleGrabberSinkCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidmajormediatype: *const windows_core::GUID, dwsampleflags: u32, llsampletime: i64, llsampleduration: i64, psamplebuffer: *const u8, dwsamplesize: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSampleGrabberSinkCallback_Impl::OnProcessSample(this, core::mem::transmute_copy(&guidmajormediatype), core::mem::transmute_copy(&dwsampleflags), core::mem::transmute_copy(&llsampletime), core::mem::transmute_copy(&llsampleduration), core::mem::transmute_copy(&psamplebuffer), core::mem::transmute_copy(&dwsamplesize)).into()
            }
        }
        unsafe extern "system" fn OnShutdown<Identity: IMFSampleGrabberSinkCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSampleGrabberSinkCallback_Impl::OnShutdown(this).into()
            }
        }
        Self {
            base__: IMFClockStateSink_Vtbl::new::<Identity, OFFSET>(),
            OnSetPresentationClock: OnSetPresentationClock::<Identity, OFFSET>,
            OnProcessSample: OnProcessSample::<Identity, OFFSET>,
            OnShutdown: OnShutdown::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFSampleGrabberSinkCallback as windows_core::Interface>::IID || iid == &<IMFClockStateSink as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFSampleGrabberSinkCallback {}
windows_core::imp::define_interface!(IMFSampleGrabberSinkCallback2, IMFSampleGrabberSinkCallback2_Vtbl, 0xca86aa50_c46e_429e_ab27_16d6ac6844cb);
impl core::ops::Deref for IMFSampleGrabberSinkCallback2 {
    type Target = IMFSampleGrabberSinkCallback;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMFSampleGrabberSinkCallback2, windows_core::IUnknown, IMFClockStateSink, IMFSampleGrabberSinkCallback);
impl IMFSampleGrabberSinkCallback2 {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn OnProcessSampleEx<P6>(&self, guidmajormediatype: *const windows_core::GUID, dwsampleflags: u32, llsampletime: i64, llsampleduration: i64, psamplebuffer: &[u8], pattributes: P6) -> windows_core::HRESULT
    where
        P6: windows_core::Param<super::IMFAttributes>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnProcessSampleEx)(windows_core::Interface::as_raw(self), guidmajormediatype, dwsampleflags, llsampletime, llsampleduration, psamplebuffer.as_ptr(), psamplebuffer.len().try_into().unwrap(), pattributes.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFSampleGrabberSinkCallback2_Vtbl {
    pub base__: IMFSampleGrabberSinkCallback_Vtbl,
    #[cfg(feature = "mfobjects")]
    pub OnProcessSampleEx: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, u32, i64, i64, *const u8, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    OnProcessSampleEx: usize,
}
#[cfg(feature = "mfobjects")]
pub trait IMFSampleGrabberSinkCallback2_Impl: IMFSampleGrabberSinkCallback_Impl {
    fn OnProcessSampleEx(&self, guidmajormediatype: *const windows_core::GUID, dwsampleflags: u32, llsampletime: i64, llsampleduration: i64, psamplebuffer: *const u8, dwsamplesize: u32, pattributes: windows_core::Ref<super::IMFAttributes>) -> windows_core::Result<()>;
}
#[cfg(feature = "mfobjects")]
impl IMFSampleGrabberSinkCallback2_Vtbl {
    pub const fn new<Identity: IMFSampleGrabberSinkCallback2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnProcessSampleEx<Identity: IMFSampleGrabberSinkCallback2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidmajormediatype: *const windows_core::GUID, dwsampleflags: u32, llsampletime: i64, llsampleduration: i64, psamplebuffer: *const u8, dwsamplesize: u32, pattributes: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSampleGrabberSinkCallback2_Impl::OnProcessSampleEx(this, core::mem::transmute_copy(&guidmajormediatype), core::mem::transmute_copy(&dwsampleflags), core::mem::transmute_copy(&llsampletime), core::mem::transmute_copy(&llsampleduration), core::mem::transmute_copy(&psamplebuffer), core::mem::transmute_copy(&dwsamplesize), core::mem::transmute_copy(&pattributes)).into()
            }
        }
        Self { base__: IMFSampleGrabberSinkCallback_Vtbl::new::<Identity, OFFSET>(), OnProcessSampleEx: OnProcessSampleEx::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFSampleGrabberSinkCallback2 as windows_core::Interface>::IID || iid == &<IMFClockStateSink as windows_core::Interface>::IID || iid == &<IMFSampleGrabberSinkCallback as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFSampleGrabberSinkCallback2 {}
windows_core::imp::define_interface!(IMFSampleProtection, IMFSampleProtection_Vtbl, 0x8e36395f_c7b9_43c4_a54d_512b4af63c95);
windows_core::imp::interface_hierarchy!(IMFSampleProtection, windows_core::IUnknown);
impl IMFSampleProtection {
    pub unsafe fn GetInputProtectionVersion(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInputProtectionVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetOutputProtectionVersion(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputProtectionVersion)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetProtectionCertificate(&self, dwversion: u32, ppcert: *mut *mut u8, pcbcert: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetProtectionCertificate)(windows_core::Interface::as_raw(self), dwversion, ppcert as _, pcbcert as _) }
    }
    pub unsafe fn InitOutputProtection(&self, dwversion: u32, dwoutputid: u32, pbcert: *const u8, cbcert: u32, ppbseed: *mut *mut u8, pcbseed: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InitOutputProtection)(windows_core::Interface::as_raw(self), dwversion, dwoutputid, pbcert, cbcert, ppbseed as _, pcbseed as _) }
    }
    pub unsafe fn InitInputProtection(&self, dwversion: u32, dwinputid: u32, pbseed: *const u8, cbseed: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InitInputProtection)(windows_core::Interface::as_raw(self), dwversion, dwinputid, pbseed, cbseed) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFSampleProtection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetInputProtectionVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetOutputProtectionVersion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetProtectionCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub InitOutputProtection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const u8, u32, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub InitInputProtection: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const u8, u32) -> windows_core::HRESULT,
}
pub trait IMFSampleProtection_Impl: windows_core::IUnknownImpl {
    fn GetInputProtectionVersion(&self) -> windows_core::Result<u32>;
    fn GetOutputProtectionVersion(&self) -> windows_core::Result<u32>;
    fn GetProtectionCertificate(&self, dwversion: u32, ppcert: *mut *mut u8, pcbcert: *mut u32) -> windows_core::Result<()>;
    fn InitOutputProtection(&self, dwversion: u32, dwoutputid: u32, pbcert: *const u8, cbcert: u32, ppbseed: *mut *mut u8, pcbseed: *mut u32) -> windows_core::Result<()>;
    fn InitInputProtection(&self, dwversion: u32, dwinputid: u32, pbseed: *const u8, cbseed: u32) -> windows_core::Result<()>;
}
impl IMFSampleProtection_Vtbl {
    pub const fn new<Identity: IMFSampleProtection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetInputProtectionVersion<Identity: IMFSampleProtection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwversion: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSampleProtection_Impl::GetInputProtectionVersion(this) {
                    Ok(ok__) => {
                        pdwversion.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOutputProtectionVersion<Identity: IMFSampleProtection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwversion: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSampleProtection_Impl::GetOutputProtectionVersion(this) {
                    Ok(ok__) => {
                        pdwversion.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProtectionCertificate<Identity: IMFSampleProtection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwversion: u32, ppcert: *mut *mut u8, pcbcert: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSampleProtection_Impl::GetProtectionCertificate(this, core::mem::transmute_copy(&dwversion), core::mem::transmute_copy(&ppcert), core::mem::transmute_copy(&pcbcert)).into()
            }
        }
        unsafe extern "system" fn InitOutputProtection<Identity: IMFSampleProtection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwversion: u32, dwoutputid: u32, pbcert: *const u8, cbcert: u32, ppbseed: *mut *mut u8, pcbseed: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSampleProtection_Impl::InitOutputProtection(this, core::mem::transmute_copy(&dwversion), core::mem::transmute_copy(&dwoutputid), core::mem::transmute_copy(&pbcert), core::mem::transmute_copy(&cbcert), core::mem::transmute_copy(&ppbseed), core::mem::transmute_copy(&pcbseed)).into()
            }
        }
        unsafe extern "system" fn InitInputProtection<Identity: IMFSampleProtection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwversion: u32, dwinputid: u32, pbseed: *const u8, cbseed: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSampleProtection_Impl::InitInputProtection(this, core::mem::transmute_copy(&dwversion), core::mem::transmute_copy(&dwinputid), core::mem::transmute_copy(&pbseed), core::mem::transmute_copy(&cbseed)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetInputProtectionVersion: GetInputProtectionVersion::<Identity, OFFSET>,
            GetOutputProtectionVersion: GetOutputProtectionVersion::<Identity, OFFSET>,
            GetProtectionCertificate: GetProtectionCertificate::<Identity, OFFSET>,
            InitOutputProtection: InitOutputProtection::<Identity, OFFSET>,
            InitInputProtection: InitInputProtection::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFSampleProtection as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFSampleProtection {}
windows_core::imp::define_interface!(IMFSaveJob, IMFSaveJob_Vtbl, 0xe9931663_80bf_4c6e_98af_5dcf58747d1f);
windows_core::imp::interface_hierarchy!(IMFSaveJob, windows_core::IUnknown);
impl IMFSaveJob {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn BeginSave<P0, P1, P2>(&self, pstream: P0, pcallback: P1, pstate: P2) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMFByteStream>,
        P1: windows_core::Param<super::IMFAsyncCallback>,
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).BeginSave)(windows_core::Interface::as_raw(self), pstream.param().abi(), pcallback.param().abi(), pstate.param().abi()) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn EndSave<P0>(&self, presult: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMFAsyncResult>,
    {
        unsafe { (windows_core::Interface::vtable(self).EndSave)(windows_core::Interface::as_raw(self), presult.param().abi()) }
    }
    pub unsafe fn CancelSave(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CancelSave)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetProgress(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProgress)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFSaveJob_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "mfobjects")]
    pub BeginSave: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    BeginSave: usize,
    #[cfg(feature = "mfobjects")]
    pub EndSave: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    EndSave: usize,
    pub CancelSave: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "mfobjects")]
pub trait IMFSaveJob_Impl: windows_core::IUnknownImpl {
    fn BeginSave(&self, pstream: windows_core::Ref<super::IMFByteStream>, pcallback: windows_core::Ref<super::IMFAsyncCallback>, pstate: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn EndSave(&self, presult: windows_core::Ref<super::IMFAsyncResult>) -> windows_core::Result<()>;
    fn CancelSave(&self) -> windows_core::Result<()>;
    fn GetProgress(&self) -> windows_core::Result<u32>;
}
#[cfg(feature = "mfobjects")]
impl IMFSaveJob_Vtbl {
    pub const fn new<Identity: IMFSaveJob_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BeginSave<Identity: IMFSaveJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstream: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void, pstate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSaveJob_Impl::BeginSave(this, core::mem::transmute_copy(&pstream), core::mem::transmute_copy(&pcallback), core::mem::transmute_copy(&pstate)).into()
            }
        }
        unsafe extern "system" fn EndSave<Identity: IMFSaveJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presult: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSaveJob_Impl::EndSave(this, core::mem::transmute_copy(&presult)).into()
            }
        }
        unsafe extern "system" fn CancelSave<Identity: IMFSaveJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSaveJob_Impl::CancelSave(this).into()
            }
        }
        unsafe extern "system" fn GetProgress<Identity: IMFSaveJob_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwpercentcomplete: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSaveJob_Impl::GetProgress(this) {
                    Ok(ok__) => {
                        pdwpercentcomplete.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginSave: BeginSave::<Identity, OFFSET>,
            EndSave: EndSave::<Identity, OFFSET>,
            CancelSave: CancelSave::<Identity, OFFSET>,
            GetProgress: GetProgress::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFSaveJob as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFSaveJob {}
windows_core::imp::define_interface!(IMFSchemeHandler, IMFSchemeHandler_Vtbl, 0x6d4c7b74_52a0_4bb7_b0db_55f29f47a668);
windows_core::imp::interface_hierarchy!(IMFSchemeHandler, windows_core::IUnknown);
impl IMFSchemeHandler {
    #[cfg(all(feature = "mfobjects", feature = "propsys"))]
    pub unsafe fn BeginCreateObject<P0, P2, P4, P5>(&self, pwszurl: P0, dwflags: u32, pprops: P2, ppiunknowncancelcookie: *mut Option<windows_core::IUnknown>, pcallback: P4, punkstate: P5) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<super::IPropertyStore>,
        P4: windows_core::Param<super::IMFAsyncCallback>,
        P5: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).BeginCreateObject)(windows_core::Interface::as_raw(self), pwszurl.param().abi(), dwflags, pprops.param().abi(), core::mem::transmute(ppiunknowncancelcookie), pcallback.param().abi(), punkstate.param().abi()) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn EndCreateObject<P0>(&self, presult: P0, pobjecttype: *mut MF_OBJECT_TYPE, ppobject: *mut Option<windows_core::IUnknown>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMFAsyncResult>,
    {
        unsafe { (windows_core::Interface::vtable(self).EndCreateObject)(windows_core::Interface::as_raw(self), presult.param().abi(), pobjecttype as _, core::mem::transmute(ppobject)) }
    }
    pub unsafe fn CancelObjectCreation<P0>(&self, piunknowncancelcookie: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CancelObjectCreation)(windows_core::Interface::as_raw(self), piunknowncancelcookie.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFSchemeHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "mfobjects", feature = "propsys"))]
    pub BeginCreateObject: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "propsys")))]
    BeginCreateObject: usize,
    #[cfg(feature = "mfobjects")]
    pub EndCreateObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut MF_OBJECT_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    EndCreateObject: usize,
    pub CancelObjectCreation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "mfobjects", feature = "propsys"))]
pub trait IMFSchemeHandler_Impl: windows_core::IUnknownImpl {
    fn BeginCreateObject(&self, pwszurl: &windows_core::PCWSTR, dwflags: u32, pprops: windows_core::Ref<super::IPropertyStore>, ppiunknowncancelcookie: windows_core::OutRef<windows_core::IUnknown>, pcallback: windows_core::Ref<super::IMFAsyncCallback>, punkstate: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn EndCreateObject(&self, presult: windows_core::Ref<super::IMFAsyncResult>, pobjecttype: *mut MF_OBJECT_TYPE, ppobject: windows_core::OutRef<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn CancelObjectCreation(&self, piunknowncancelcookie: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "mfobjects", feature = "propsys"))]
impl IMFSchemeHandler_Vtbl {
    pub const fn new<Identity: IMFSchemeHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BeginCreateObject<Identity: IMFSchemeHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszurl: windows_core::PCWSTR, dwflags: u32, pprops: *mut core::ffi::c_void, ppiunknowncancelcookie: *mut *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void, punkstate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSchemeHandler_Impl::BeginCreateObject(this, core::mem::transmute(&pwszurl), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pprops), core::mem::transmute_copy(&ppiunknowncancelcookie), core::mem::transmute_copy(&pcallback), core::mem::transmute_copy(&punkstate)).into()
            }
        }
        unsafe extern "system" fn EndCreateObject<Identity: IMFSchemeHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presult: *mut core::ffi::c_void, pobjecttype: *mut MF_OBJECT_TYPE, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSchemeHandler_Impl::EndCreateObject(this, core::mem::transmute_copy(&presult), core::mem::transmute_copy(&pobjecttype), core::mem::transmute_copy(&ppobject)).into()
            }
        }
        unsafe extern "system" fn CancelObjectCreation<Identity: IMFSchemeHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piunknowncancelcookie: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSchemeHandler_Impl::CancelObjectCreation(this, core::mem::transmute_copy(&piunknowncancelcookie)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginCreateObject: BeginCreateObject::<Identity, OFFSET>,
            EndCreateObject: EndCreateObject::<Identity, OFFSET>,
            CancelObjectCreation: CancelObjectCreation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFSchemeHandler as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mfobjects", feature = "propsys"))]
impl windows_core::RuntimeName for IMFSchemeHandler {}
windows_core::imp::define_interface!(IMFSecureChannel, IMFSecureChannel_Vtbl, 0xd0ae555d_3b12_4d97_b060_0990bc5aeb67);
windows_core::imp::interface_hierarchy!(IMFSecureChannel, windows_core::IUnknown);
impl IMFSecureChannel {
    pub unsafe fn GetCertificate(&self, ppcert: *mut *mut u8, pcbcert: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCertificate)(windows_core::Interface::as_raw(self), ppcert as _, pcbcert as _) }
    }
    pub unsafe fn SetupSession(&self, pbencryptedsessionkey: &[u8]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetupSession)(windows_core::Interface::as_raw(self), pbencryptedsessionkey.as_ptr(), pbencryptedsessionkey.len().try_into().unwrap()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFSecureChannel_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCertificate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut u32) -> windows_core::HRESULT,
    pub SetupSession: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32) -> windows_core::HRESULT,
}
pub trait IMFSecureChannel_Impl: windows_core::IUnknownImpl {
    fn GetCertificate(&self, ppcert: *mut *mut u8, pcbcert: *mut u32) -> windows_core::Result<()>;
    fn SetupSession(&self, pbencryptedsessionkey: *const u8, cbsessionkey: u32) -> windows_core::Result<()>;
}
impl IMFSecureChannel_Vtbl {
    pub const fn new<Identity: IMFSecureChannel_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCertificate<Identity: IMFSecureChannel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcert: *mut *mut u8, pcbcert: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSecureChannel_Impl::GetCertificate(this, core::mem::transmute_copy(&ppcert), core::mem::transmute_copy(&pcbcert)).into()
            }
        }
        unsafe extern "system" fn SetupSession<Identity: IMFSecureChannel_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbencryptedsessionkey: *const u8, cbsessionkey: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSecureChannel_Impl::SetupSession(this, core::mem::transmute_copy(&pbencryptedsessionkey), core::mem::transmute_copy(&cbsessionkey)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCertificate: GetCertificate::<Identity, OFFSET>,
            SetupSession: SetupSession::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFSecureChannel as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFSecureChannel {}
windows_core::imp::define_interface!(IMFSeekInfo, IMFSeekInfo_Vtbl, 0x26afea53_d9ed_42b5_ab80_e64f9ee34779);
windows_core::imp::interface_hierarchy!(IMFSeekInfo, windows_core::IUnknown);
impl IMFSeekInfo {
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetNearestKeyFrames(&self, pguidtimeformat: *const windows_core::GUID, pvarstartposition: *const super::PROPVARIANT, pvarpreviouskeyframe: *mut super::PROPVARIANT, pvarnextkeyframe: *mut super::PROPVARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetNearestKeyFrames)(windows_core::Interface::as_raw(self), pguidtimeformat, pvarstartposition, pvarpreviouskeyframe, pvarnextkeyframe) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFSeekInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub GetNearestKeyFrames: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *const super::PROPVARIANT, *mut super::PROPVARIANT, *mut super::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    GetNearestKeyFrames: usize,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMFSeekInfo_Impl: windows_core::IUnknownImpl {
    fn GetNearestKeyFrames(&self, pguidtimeformat: *const windows_core::GUID, pvarstartposition: *const super::PROPVARIANT, pvarpreviouskeyframe: *mut super::PROPVARIANT, pvarnextkeyframe: *mut super::PROPVARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl IMFSeekInfo_Vtbl {
    pub const fn new<Identity: IMFSeekInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetNearestKeyFrames<Identity: IMFSeekInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pguidtimeformat: *const windows_core::GUID, pvarstartposition: *const super::PROPVARIANT, pvarpreviouskeyframe: *mut super::PROPVARIANT, pvarnextkeyframe: *mut super::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSeekInfo_Impl::GetNearestKeyFrames(this, core::mem::transmute_copy(&pguidtimeformat), core::mem::transmute_copy(&pvarstartposition), core::mem::transmute_copy(&pvarpreviouskeyframe), core::mem::transmute_copy(&pvarnextkeyframe)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetNearestKeyFrames: GetNearestKeyFrames::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFSeekInfo as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMFSeekInfo {}
windows_core::imp::define_interface!(IMFSensorActivitiesReport, IMFSensorActivitiesReport_Vtbl, 0x683f7a5e_4a19_43cd_b1a9_dbf4ab3f7777);
windows_core::imp::interface_hierarchy!(IMFSensorActivitiesReport, windows_core::IUnknown);
impl IMFSensorActivitiesReport {
    pub unsafe fn GetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetActivityReport(&self, index: u32) -> windows_core::Result<IMFSensorActivityReport> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetActivityReport)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetActivityReportByDeviceName<P0>(&self, symbolicname: P0) -> windows_core::Result<IMFSensorActivityReport>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetActivityReportByDeviceName)(windows_core::Interface::as_raw(self), symbolicname.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFSensorActivitiesReport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetActivityReport: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetActivityReportByDeviceName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFSensorActivitiesReport_Impl: windows_core::IUnknownImpl {
    fn GetCount(&self) -> windows_core::Result<u32>;
    fn GetActivityReport(&self, index: u32) -> windows_core::Result<IMFSensorActivityReport>;
    fn GetActivityReportByDeviceName(&self, symbolicname: &windows_core::PCWSTR) -> windows_core::Result<IMFSensorActivityReport>;
}
impl IMFSensorActivitiesReport_Vtbl {
    pub const fn new<Identity: IMFSensorActivitiesReport_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCount<Identity: IMFSensorActivitiesReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pccount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSensorActivitiesReport_Impl::GetCount(this) {
                    Ok(ok__) => {
                        pccount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetActivityReport<Identity: IMFSensorActivitiesReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, sensoractivityreport: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSensorActivitiesReport_Impl::GetActivityReport(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        sensoractivityreport.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetActivityReportByDeviceName<Identity: IMFSensorActivitiesReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, symbolicname: windows_core::PCWSTR, sensoractivityreport: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSensorActivitiesReport_Impl::GetActivityReportByDeviceName(this, core::mem::transmute(&symbolicname)) {
                    Ok(ok__) => {
                        sensoractivityreport.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCount: GetCount::<Identity, OFFSET>,
            GetActivityReport: GetActivityReport::<Identity, OFFSET>,
            GetActivityReportByDeviceName: GetActivityReportByDeviceName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFSensorActivitiesReport as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFSensorActivitiesReport {}
windows_core::imp::define_interface!(IMFSensorActivitiesReportCallback, IMFSensorActivitiesReportCallback_Vtbl, 0xde5072ee_dbe3_46dc_8a87_b6f631194751);
windows_core::imp::interface_hierarchy!(IMFSensorActivitiesReportCallback, windows_core::IUnknown);
impl IMFSensorActivitiesReportCallback {
    pub unsafe fn OnActivitiesReport<P0>(&self, sensoractivitiesreport: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFSensorActivitiesReport>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnActivitiesReport)(windows_core::Interface::as_raw(self), sensoractivitiesreport.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFSensorActivitiesReportCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnActivitiesReport: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFSensorActivitiesReportCallback_Impl: windows_core::IUnknownImpl {
    fn OnActivitiesReport(&self, sensoractivitiesreport: windows_core::Ref<IMFSensorActivitiesReport>) -> windows_core::Result<()>;
}
impl IMFSensorActivitiesReportCallback_Vtbl {
    pub const fn new<Identity: IMFSensorActivitiesReportCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnActivitiesReport<Identity: IMFSensorActivitiesReportCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sensoractivitiesreport: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSensorActivitiesReportCallback_Impl::OnActivitiesReport(this, core::mem::transmute_copy(&sensoractivitiesreport)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnActivitiesReport: OnActivitiesReport::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFSensorActivitiesReportCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFSensorActivitiesReportCallback {}
windows_core::imp::define_interface!(IMFSensorActivityMonitor, IMFSensorActivityMonitor_Vtbl, 0xd0cef145_b3f4_4340_a2e5_7a5080ca05cb);
windows_core::imp::interface_hierarchy!(IMFSensorActivityMonitor, windows_core::IUnknown);
impl IMFSensorActivityMonitor {
    pub unsafe fn Start(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Start)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Stop(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Stop)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFSensorActivityMonitor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Start: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Stop: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFSensorActivityMonitor_Impl: windows_core::IUnknownImpl {
    fn Start(&self) -> windows_core::Result<()>;
    fn Stop(&self) -> windows_core::Result<()>;
}
impl IMFSensorActivityMonitor_Vtbl {
    pub const fn new<Identity: IMFSensorActivityMonitor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Start<Identity: IMFSensorActivityMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSensorActivityMonitor_Impl::Start(this).into()
            }
        }
        unsafe extern "system" fn Stop<Identity: IMFSensorActivityMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSensorActivityMonitor_Impl::Stop(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Start: Start::<Identity, OFFSET>, Stop: Stop::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFSensorActivityMonitor as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFSensorActivityMonitor {}
windows_core::imp::define_interface!(IMFSensorActivityReport, IMFSensorActivityReport_Vtbl, 0x3e8c4be1_a8c2_4528_90de_2851bde5fead);
windows_core::imp::interface_hierarchy!(IMFSensorActivityReport, windows_core::IUnknown);
impl IMFSensorActivityReport {
    pub unsafe fn GetFriendlyName(&self, friendlyname: &mut [u16], pcchwritten: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFriendlyName)(windows_core::Interface::as_raw(self), core::mem::transmute(friendlyname.as_mut_ptr()), friendlyname.len().try_into().unwrap(), pcchwritten as _) }
    }
    pub unsafe fn GetSymbolicLink(&self, symboliclink: &mut [u16], pcchwritten: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSymbolicLink)(windows_core::Interface::as_raw(self), core::mem::transmute(symboliclink.as_mut_ptr()), symboliclink.len().try_into().unwrap(), pcchwritten as _) }
    }
    pub unsafe fn GetProcessCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProcessCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetProcessActivity(&self, index: u32) -> windows_core::Result<IMFSensorProcessActivity> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProcessActivity)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFSensorActivityReport_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetFriendlyName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32, *mut u32) -> windows_core::HRESULT,
    pub GetSymbolicLink: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, u32, *mut u32) -> windows_core::HRESULT,
    pub GetProcessCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetProcessActivity: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFSensorActivityReport_Impl: windows_core::IUnknownImpl {
    fn GetFriendlyName(&self, friendlyname: windows_core::PWSTR, cchfriendlyname: u32, pcchwritten: *mut u32) -> windows_core::Result<()>;
    fn GetSymbolicLink(&self, symboliclink: windows_core::PWSTR, cchsymboliclink: u32, pcchwritten: *mut u32) -> windows_core::Result<()>;
    fn GetProcessCount(&self) -> windows_core::Result<u32>;
    fn GetProcessActivity(&self, index: u32) -> windows_core::Result<IMFSensorProcessActivity>;
}
impl IMFSensorActivityReport_Vtbl {
    pub const fn new<Identity: IMFSensorActivityReport_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFriendlyName<Identity: IMFSensorActivityReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, friendlyname: windows_core::PWSTR, cchfriendlyname: u32, pcchwritten: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSensorActivityReport_Impl::GetFriendlyName(this, core::mem::transmute_copy(&friendlyname), core::mem::transmute_copy(&cchfriendlyname), core::mem::transmute_copy(&pcchwritten)).into()
            }
        }
        unsafe extern "system" fn GetSymbolicLink<Identity: IMFSensorActivityReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, symboliclink: windows_core::PWSTR, cchsymboliclink: u32, pcchwritten: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSensorActivityReport_Impl::GetSymbolicLink(this, core::mem::transmute_copy(&symboliclink), core::mem::transmute_copy(&cchsymboliclink), core::mem::transmute_copy(&pcchwritten)).into()
            }
        }
        unsafe extern "system" fn GetProcessCount<Identity: IMFSensorActivityReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pccount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSensorActivityReport_Impl::GetProcessCount(this) {
                    Ok(ok__) => {
                        pccount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProcessActivity<Identity: IMFSensorActivityReport_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, ppprocessactivity: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSensorActivityReport_Impl::GetProcessActivity(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        ppprocessactivity.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFriendlyName: GetFriendlyName::<Identity, OFFSET>,
            GetSymbolicLink: GetSymbolicLink::<Identity, OFFSET>,
            GetProcessCount: GetProcessCount::<Identity, OFFSET>,
            GetProcessActivity: GetProcessActivity::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFSensorActivityReport as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFSensorActivityReport {}
windows_core::imp::define_interface!(IMFSensorDevice, IMFSensorDevice_Vtbl, 0xfb9f48f2_2a18_4e28_9730_786f30f04dc4);
windows_core::imp::interface_hierarchy!(IMFSensorDevice, windows_core::IUnknown);
impl IMFSensorDevice {
    pub unsafe fn GetDeviceId(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDeviceId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDeviceType(&self) -> windows_core::Result<MFSensorDeviceType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDeviceType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetFlags(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSymbolicLink(&self, symboliclink: &mut [u16], pcchwritten: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSymbolicLink)(windows_core::Interface::as_raw(self), core::mem::transmute(symboliclink.as_mut_ptr()), symboliclink.len().try_into().unwrap(), pcchwritten as _) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn GetDeviceAttributes(&self) -> windows_core::Result<super::IMFAttributes> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDeviceAttributes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetStreamAttributesCount(&self, etype: MFSensorStreamType) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStreamAttributesCount)(windows_core::Interface::as_raw(self), etype, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn GetStreamAttributes(&self, etype: MFSensorStreamType, dwindex: u32) -> windows_core::Result<super::IMFAttributes> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStreamAttributes)(windows_core::Interface::as_raw(self), etype, dwindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetSensorDeviceMode(&self, emode: MFSensorDeviceMode) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSensorDeviceMode)(windows_core::Interface::as_raw(self), emode) }
    }
    pub unsafe fn GetSensorDeviceMode(&self) -> windows_core::Result<MFSensorDeviceMode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSensorDeviceMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFSensorDevice_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDeviceId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub GetDeviceType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MFSensorDeviceType) -> windows_core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub GetSymbolicLink: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "mfobjects")]
    pub GetDeviceAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    GetDeviceAttributes: usize,
    pub GetStreamAttributesCount: unsafe extern "system" fn(*mut core::ffi::c_void, MFSensorStreamType, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "mfobjects")]
    pub GetStreamAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, MFSensorStreamType, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    GetStreamAttributes: usize,
    pub SetSensorDeviceMode: unsafe extern "system" fn(*mut core::ffi::c_void, MFSensorDeviceMode) -> windows_core::HRESULT,
    pub GetSensorDeviceMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MFSensorDeviceMode) -> windows_core::HRESULT,
}
#[cfg(feature = "mfobjects")]
pub trait IMFSensorDevice_Impl: windows_core::IUnknownImpl {
    fn GetDeviceId(&self) -> windows_core::Result<u64>;
    fn GetDeviceType(&self) -> windows_core::Result<MFSensorDeviceType>;
    fn GetFlags(&self) -> windows_core::Result<u64>;
    fn GetSymbolicLink(&self, symboliclink: windows_core::PWSTR, cchsymboliclink: i32, pcchwritten: *mut i32) -> windows_core::Result<()>;
    fn GetDeviceAttributes(&self) -> windows_core::Result<super::IMFAttributes>;
    fn GetStreamAttributesCount(&self, etype: MFSensorStreamType) -> windows_core::Result<u32>;
    fn GetStreamAttributes(&self, etype: MFSensorStreamType, dwindex: u32) -> windows_core::Result<super::IMFAttributes>;
    fn SetSensorDeviceMode(&self, emode: MFSensorDeviceMode) -> windows_core::Result<()>;
    fn GetSensorDeviceMode(&self) -> windows_core::Result<MFSensorDeviceMode>;
}
#[cfg(feature = "mfobjects")]
impl IMFSensorDevice_Vtbl {
    pub const fn new<Identity: IMFSensorDevice_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDeviceId<Identity: IMFSensorDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdeviceid: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSensorDevice_Impl::GetDeviceId(this) {
                    Ok(ok__) => {
                        pdeviceid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDeviceType<Identity: IMFSensorDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut MFSensorDeviceType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSensorDevice_Impl::GetDeviceType(this) {
                    Ok(ok__) => {
                        ptype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFlags<Identity: IMFSensorDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflags: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSensorDevice_Impl::GetFlags(this) {
                    Ok(ok__) => {
                        pflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSymbolicLink<Identity: IMFSensorDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, symboliclink: windows_core::PWSTR, cchsymboliclink: i32, pcchwritten: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSensorDevice_Impl::GetSymbolicLink(this, core::mem::transmute_copy(&symboliclink), core::mem::transmute_copy(&cchsymboliclink), core::mem::transmute_copy(&pcchwritten)).into()
            }
        }
        unsafe extern "system" fn GetDeviceAttributes<Identity: IMFSensorDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppattributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSensorDevice_Impl::GetDeviceAttributes(this) {
                    Ok(ok__) => {
                        ppattributes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStreamAttributesCount<Identity: IMFSensorDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, etype: MFSensorStreamType, pdwcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSensorDevice_Impl::GetStreamAttributesCount(this, core::mem::transmute_copy(&etype)) {
                    Ok(ok__) => {
                        pdwcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStreamAttributes<Identity: IMFSensorDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, etype: MFSensorStreamType, dwindex: u32, ppattributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSensorDevice_Impl::GetStreamAttributes(this, core::mem::transmute_copy(&etype), core::mem::transmute_copy(&dwindex)) {
                    Ok(ok__) => {
                        ppattributes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetSensorDeviceMode<Identity: IMFSensorDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, emode: MFSensorDeviceMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSensorDevice_Impl::SetSensorDeviceMode(this, core::mem::transmute_copy(&emode)).into()
            }
        }
        unsafe extern "system" fn GetSensorDeviceMode<Identity: IMFSensorDevice_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pemode: *mut MFSensorDeviceMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSensorDevice_Impl::GetSensorDeviceMode(this) {
                    Ok(ok__) => {
                        pemode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDeviceId: GetDeviceId::<Identity, OFFSET>,
            GetDeviceType: GetDeviceType::<Identity, OFFSET>,
            GetFlags: GetFlags::<Identity, OFFSET>,
            GetSymbolicLink: GetSymbolicLink::<Identity, OFFSET>,
            GetDeviceAttributes: GetDeviceAttributes::<Identity, OFFSET>,
            GetStreamAttributesCount: GetStreamAttributesCount::<Identity, OFFSET>,
            GetStreamAttributes: GetStreamAttributes::<Identity, OFFSET>,
            SetSensorDeviceMode: SetSensorDeviceMode::<Identity, OFFSET>,
            GetSensorDeviceMode: GetSensorDeviceMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFSensorDevice as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFSensorDevice {}
windows_core::imp::define_interface!(IMFSensorGroup, IMFSensorGroup_Vtbl, 0x4110243a_9757_461f_89f1_f22345bcab4e);
windows_core::imp::interface_hierarchy!(IMFSensorGroup, windows_core::IUnknown);
impl IMFSensorGroup {
    pub unsafe fn GetSymbolicLink(&self, symboliclink: &mut [u16], pcchwritten: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSymbolicLink)(windows_core::Interface::as_raw(self), core::mem::transmute(symboliclink.as_mut_ptr()), symboliclink.len().try_into().unwrap(), pcchwritten as _) }
    }
    pub unsafe fn GetFlags(&self) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn GetSensorGroupAttributes(&self) -> windows_core::Result<super::IMFAttributes> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSensorGroupAttributes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSensorDeviceCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSensorDeviceCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetSensorDevice(&self, dwindex: u32) -> windows_core::Result<IMFSensorDevice> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSensorDevice)(windows_core::Interface::as_raw(self), dwindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetDefaultSensorDeviceIndex(&self, dwindex: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDefaultSensorDeviceIndex)(windows_core::Interface::as_raw(self), dwindex) }
    }
    pub unsafe fn GetDefaultSensorDeviceIndex(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDefaultSensorDeviceIndex)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn CreateMediaSource(&self) -> windows_core::Result<IMFMediaSource> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateMediaSource)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFSensorGroup_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetSymbolicLink: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PWSTR, i32, *mut i32) -> windows_core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    #[cfg(feature = "mfobjects")]
    pub GetSensorGroupAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    GetSensorGroupAttributes: usize,
    pub GetSensorDeviceCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetSensorDevice: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetDefaultSensorDeviceIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetDefaultSensorDeviceIndex: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "mfobjects")]
    pub CreateMediaSource: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    CreateMediaSource: usize,
}
#[cfg(feature = "mfobjects")]
pub trait IMFSensorGroup_Impl: windows_core::IUnknownImpl {
    fn GetSymbolicLink(&self, symboliclink: windows_core::PWSTR, cchsymboliclink: i32, pcchwritten: *mut i32) -> windows_core::Result<()>;
    fn GetFlags(&self) -> windows_core::Result<u64>;
    fn GetSensorGroupAttributes(&self) -> windows_core::Result<super::IMFAttributes>;
    fn GetSensorDeviceCount(&self) -> windows_core::Result<u32>;
    fn GetSensorDevice(&self, dwindex: u32) -> windows_core::Result<IMFSensorDevice>;
    fn SetDefaultSensorDeviceIndex(&self, dwindex: u32) -> windows_core::Result<()>;
    fn GetDefaultSensorDeviceIndex(&self) -> windows_core::Result<u32>;
    fn CreateMediaSource(&self) -> windows_core::Result<IMFMediaSource>;
}
#[cfg(feature = "mfobjects")]
impl IMFSensorGroup_Vtbl {
    pub const fn new<Identity: IMFSensorGroup_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSymbolicLink<Identity: IMFSensorGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, symboliclink: windows_core::PWSTR, cchsymboliclink: i32, pcchwritten: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSensorGroup_Impl::GetSymbolicLink(this, core::mem::transmute_copy(&symboliclink), core::mem::transmute_copy(&cchsymboliclink), core::mem::transmute_copy(&pcchwritten)).into()
            }
        }
        unsafe extern "system" fn GetFlags<Identity: IMFSensorGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflags: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSensorGroup_Impl::GetFlags(this) {
                    Ok(ok__) => {
                        pflags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSensorGroupAttributes<Identity: IMFSensorGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppattributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSensorGroup_Impl::GetSensorGroupAttributes(this) {
                    Ok(ok__) => {
                        ppattributes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSensorDeviceCount<Identity: IMFSensorGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSensorGroup_Impl::GetSensorDeviceCount(this) {
                    Ok(ok__) => {
                        pdwcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSensorDevice<Identity: IMFSensorGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, ppdevice: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSensorGroup_Impl::GetSensorDevice(this, core::mem::transmute_copy(&dwindex)) {
                    Ok(ok__) => {
                        ppdevice.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDefaultSensorDeviceIndex<Identity: IMFSensorGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSensorGroup_Impl::SetDefaultSensorDeviceIndex(this, core::mem::transmute_copy(&dwindex)).into()
            }
        }
        unsafe extern "system" fn GetDefaultSensorDeviceIndex<Identity: IMFSensorGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwindex: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSensorGroup_Impl::GetDefaultSensorDeviceIndex(this) {
                    Ok(ok__) => {
                        pdwindex.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateMediaSource<Identity: IMFSensorGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsource: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSensorGroup_Impl::CreateMediaSource(this) {
                    Ok(ok__) => {
                        ppsource.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSymbolicLink: GetSymbolicLink::<Identity, OFFSET>,
            GetFlags: GetFlags::<Identity, OFFSET>,
            GetSensorGroupAttributes: GetSensorGroupAttributes::<Identity, OFFSET>,
            GetSensorDeviceCount: GetSensorDeviceCount::<Identity, OFFSET>,
            GetSensorDevice: GetSensorDevice::<Identity, OFFSET>,
            SetDefaultSensorDeviceIndex: SetDefaultSensorDeviceIndex::<Identity, OFFSET>,
            GetDefaultSensorDeviceIndex: GetDefaultSensorDeviceIndex::<Identity, OFFSET>,
            CreateMediaSource: CreateMediaSource::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFSensorGroup as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFSensorGroup {}
windows_core::imp::define_interface!(IMFSensorProcessActivity, IMFSensorProcessActivity_Vtbl, 0x39dc7f4a_b141_4719_813c_a7f46162a2b8);
windows_core::imp::interface_hierarchy!(IMFSensorProcessActivity, windows_core::IUnknown);
impl IMFSensorProcessActivity {
    pub unsafe fn GetProcessId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProcessId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetStreamingState(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStreamingState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetStreamingMode(&self) -> windows_core::Result<MFSensorDeviceMode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStreamingMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "minwindef")]
    pub unsafe fn GetReportTime(&self) -> windows_core::Result<super::FILETIME> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetReportTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFSensorProcessActivity_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetProcessId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetStreamingState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetStreamingMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MFSensorDeviceMode) -> windows_core::HRESULT,
    #[cfg(feature = "minwindef")]
    pub GetReportTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::FILETIME) -> windows_core::HRESULT,
    #[cfg(not(feature = "minwindef"))]
    GetReportTime: usize,
}
#[cfg(feature = "minwindef")]
pub trait IMFSensorProcessActivity_Impl: windows_core::IUnknownImpl {
    fn GetProcessId(&self) -> windows_core::Result<u32>;
    fn GetStreamingState(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetStreamingMode(&self) -> windows_core::Result<MFSensorDeviceMode>;
    fn GetReportTime(&self) -> windows_core::Result<super::FILETIME>;
}
#[cfg(feature = "minwindef")]
impl IMFSensorProcessActivity_Vtbl {
    pub const fn new<Identity: IMFSensorProcessActivity_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetProcessId<Identity: IMFSensorProcessActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSensorProcessActivity_Impl::GetProcessId(this) {
                    Ok(ok__) => {
                        ppid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStreamingState<Identity: IMFSensorProcessActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfstreaming: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSensorProcessActivity_Impl::GetStreamingState(this) {
                    Ok(ok__) => {
                        pfstreaming.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetStreamingMode<Identity: IMFSensorProcessActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmode: *mut MFSensorDeviceMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSensorProcessActivity_Impl::GetStreamingMode(this) {
                    Ok(ok__) => {
                        pmode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetReportTime<Identity: IMFSensorProcessActivity_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pft: *mut super::FILETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSensorProcessActivity_Impl::GetReportTime(this) {
                    Ok(ok__) => {
                        pft.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProcessId: GetProcessId::<Identity, OFFSET>,
            GetStreamingState: GetStreamingState::<Identity, OFFSET>,
            GetStreamingMode: GetStreamingMode::<Identity, OFFSET>,
            GetReportTime: GetReportTime::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFSensorProcessActivity as windows_core::Interface>::IID
    }
}
#[cfg(feature = "minwindef")]
impl windows_core::RuntimeName for IMFSensorProcessActivity {}
windows_core::imp::define_interface!(IMFSensorProfile, IMFSensorProfile_Vtbl, 0x22f765d1_8dab_4107_846d_56baf72215e7);
windows_core::imp::interface_hierarchy!(IMFSensorProfile, windows_core::IUnknown);
impl IMFSensorProfile {
    pub unsafe fn GetProfileId(&self, pid: *mut SENSORPROFILEID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetProfileId)(windows_core::Interface::as_raw(self), pid as _) }
    }
    pub unsafe fn AddProfileFilter<P1>(&self, streamid: u32, wzfiltersetstring: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddProfileFilter)(windows_core::Interface::as_raw(self), streamid, wzfiltersetstring.param().abi()) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn IsMediaTypeSupported<P1>(&self, streamid: u32, pmediatype: P1) -> windows_core::Result<windows_core::BOOL>
    where
        P1: windows_core::Param<super::IMFMediaType>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsMediaTypeSupported)(windows_core::Interface::as_raw(self), streamid, pmediatype.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AddBlockedControl<P0>(&self, wzblockedcontrol: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddBlockedControl)(windows_core::Interface::as_raw(self), wzblockedcontrol.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFSensorProfile_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetProfileId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SENSORPROFILEID) -> windows_core::HRESULT,
    pub AddProfileFilter: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "mfobjects")]
    pub IsMediaTypeSupported: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    IsMediaTypeSupported: usize,
    pub AddBlockedControl: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
}
#[cfg(feature = "mfobjects")]
pub trait IMFSensorProfile_Impl: windows_core::IUnknownImpl {
    fn GetProfileId(&self, pid: *mut SENSORPROFILEID) -> windows_core::Result<()>;
    fn AddProfileFilter(&self, streamid: u32, wzfiltersetstring: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn IsMediaTypeSupported(&self, streamid: u32, pmediatype: windows_core::Ref<super::IMFMediaType>) -> windows_core::Result<windows_core::BOOL>;
    fn AddBlockedControl(&self, wzblockedcontrol: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "mfobjects")]
impl IMFSensorProfile_Vtbl {
    pub const fn new<Identity: IMFSensorProfile_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetProfileId<Identity: IMFSensorProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pid: *mut SENSORPROFILEID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSensorProfile_Impl::GetProfileId(this, core::mem::transmute_copy(&pid)).into()
            }
        }
        unsafe extern "system" fn AddProfileFilter<Identity: IMFSensorProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streamid: u32, wzfiltersetstring: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSensorProfile_Impl::AddProfileFilter(this, core::mem::transmute_copy(&streamid), core::mem::transmute(&wzfiltersetstring)).into()
            }
        }
        unsafe extern "system" fn IsMediaTypeSupported<Identity: IMFSensorProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streamid: u32, pmediatype: *mut core::ffi::c_void, pfsupported: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSensorProfile_Impl::IsMediaTypeSupported(this, core::mem::transmute_copy(&streamid), core::mem::transmute_copy(&pmediatype)) {
                    Ok(ok__) => {
                        pfsupported.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddBlockedControl<Identity: IMFSensorProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wzblockedcontrol: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSensorProfile_Impl::AddBlockedControl(this, core::mem::transmute(&wzblockedcontrol)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProfileId: GetProfileId::<Identity, OFFSET>,
            AddProfileFilter: AddProfileFilter::<Identity, OFFSET>,
            IsMediaTypeSupported: IsMediaTypeSupported::<Identity, OFFSET>,
            AddBlockedControl: AddBlockedControl::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFSensorProfile as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFSensorProfile {}
windows_core::imp::define_interface!(IMFSensorProfileCollection, IMFSensorProfileCollection_Vtbl, 0xc95ea55b_0187_48be_9353_8d2507662351);
windows_core::imp::interface_hierarchy!(IMFSensorProfileCollection, windows_core::IUnknown);
impl IMFSensorProfileCollection {
    pub unsafe fn GetProfileCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetProfileCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetProfile(&self, index: u32) -> windows_core::Result<IMFSensorProfile> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProfile)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AddProfile<P0>(&self, pprofile: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFSensorProfile>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddProfile)(windows_core::Interface::as_raw(self), pprofile.param().abi()) }
    }
    pub unsafe fn FindProfile(&self, profileid: *const SENSORPROFILEID) -> windows_core::Result<IMFSensorProfile> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindProfile)(windows_core::Interface::as_raw(self), profileid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RemoveProfileByIndex(&self, index: u32) {
        unsafe {
            (windows_core::Interface::vtable(self).RemoveProfileByIndex)(windows_core::Interface::as_raw(self), index);
        }
    }
    pub unsafe fn RemoveProfile(&self, profileid: *const SENSORPROFILEID) {
        unsafe {
            (windows_core::Interface::vtable(self).RemoveProfile)(windows_core::Interface::as_raw(self), profileid);
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFSensorProfileCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetProfileCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetProfile: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *const SENSORPROFILEID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveProfileByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32),
    pub RemoveProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *const SENSORPROFILEID),
}
pub trait IMFSensorProfileCollection_Impl: windows_core::IUnknownImpl {
    fn GetProfileCount(&self) -> u32;
    fn GetProfile(&self, index: u32) -> windows_core::Result<IMFSensorProfile>;
    fn AddProfile(&self, pprofile: windows_core::Ref<IMFSensorProfile>) -> windows_core::Result<()>;
    fn FindProfile(&self, profileid: *const SENSORPROFILEID) -> windows_core::Result<IMFSensorProfile>;
    fn RemoveProfileByIndex(&self, index: u32);
    fn RemoveProfile(&self, profileid: *const SENSORPROFILEID);
}
impl IMFSensorProfileCollection_Vtbl {
    pub const fn new<Identity: IMFSensorProfileCollection_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetProfileCount<Identity: IMFSensorProfileCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSensorProfileCollection_Impl::GetProfileCount(this)
            }
        }
        unsafe extern "system" fn GetProfile<Identity: IMFSensorProfileCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, ppprofile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSensorProfileCollection_Impl::GetProfile(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        ppprofile.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddProfile<Identity: IMFSensorProfileCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprofile: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSensorProfileCollection_Impl::AddProfile(this, core::mem::transmute_copy(&pprofile)).into()
            }
        }
        unsafe extern "system" fn FindProfile<Identity: IMFSensorProfileCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profileid: *const SENSORPROFILEID, ppprofile: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSensorProfileCollection_Impl::FindProfile(this, core::mem::transmute_copy(&profileid)) {
                    Ok(ok__) => {
                        ppprofile.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemoveProfileByIndex<Identity: IMFSensorProfileCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSensorProfileCollection_Impl::RemoveProfileByIndex(this, core::mem::transmute_copy(&index));
            }
        }
        unsafe extern "system" fn RemoveProfile<Identity: IMFSensorProfileCollection_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, profileid: *const SENSORPROFILEID) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSensorProfileCollection_Impl::RemoveProfile(this, core::mem::transmute_copy(&profileid));
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProfileCount: GetProfileCount::<Identity, OFFSET>,
            GetProfile: GetProfile::<Identity, OFFSET>,
            AddProfile: AddProfile::<Identity, OFFSET>,
            FindProfile: FindProfile::<Identity, OFFSET>,
            RemoveProfileByIndex: RemoveProfileByIndex::<Identity, OFFSET>,
            RemoveProfile: RemoveProfile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFSensorProfileCollection as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFSensorProfileCollection {}
#[cfg(feature = "mfobjects")]
windows_core::imp::define_interface!(IMFSensorStream, IMFSensorStream_Vtbl, 0xe9a42171_c56e_498a_8b39_eda5a070b7fc);
#[cfg(feature = "mfobjects")]
impl core::ops::Deref for IMFSensorStream {
    type Target = super::IMFAttributes;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "mfobjects")]
windows_core::imp::interface_hierarchy!(IMFSensorStream, windows_core::IUnknown, super::IMFAttributes);
#[cfg(feature = "mfobjects")]
impl IMFSensorStream {
    pub unsafe fn GetMediaTypeCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMediaTypeCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMediaType(&self, dwindex: u32) -> windows_core::Result<super::IMFMediaType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMediaType)(windows_core::Interface::as_raw(self), dwindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CloneSensorStream(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CloneSensorStream)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "mfobjects")]
#[repr(C)]
#[doc(hidden)]
pub struct IMFSensorStream_Vtbl {
    pub base__: super::IMFAttributes_Vtbl,
    pub GetMediaTypeCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CloneSensorStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMFSensorStream_Impl: super::IMFAttributes_Impl {
    fn GetMediaTypeCount(&self) -> windows_core::Result<u32>;
    fn GetMediaType(&self, dwindex: u32) -> windows_core::Result<super::IMFMediaType>;
    fn CloneSensorStream(&self) -> windows_core::Result<IMFSensorStream>;
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl IMFSensorStream_Vtbl {
    pub const fn new<Identity: IMFSensorStream_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMediaTypeCount<Identity: IMFSensorStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSensorStream_Impl::GetMediaTypeCount(this) {
                    Ok(ok__) => {
                        pdwcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMediaType<Identity: IMFSensorStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, ppmediatype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSensorStream_Impl::GetMediaType(this, core::mem::transmute_copy(&dwindex)) {
                    Ok(ok__) => {
                        ppmediatype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CloneSensorStream<Identity: IMFSensorStream_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSensorStream_Impl::CloneSensorStream(this) {
                    Ok(ok__) => {
                        ppstream.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IMFAttributes_Vtbl::new::<Identity, OFFSET>(),
            GetMediaTypeCount: GetMediaTypeCount::<Identity, OFFSET>,
            GetMediaType: GetMediaType::<Identity, OFFSET>,
            CloneSensorStream: CloneSensorStream::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFSensorStream as windows_core::Interface>::IID || iid == &<super::IMFAttributes as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMFSensorStream {}
windows_core::imp::define_interface!(IMFSensorTransformFactory, IMFSensorTransformFactory_Vtbl, 0xeed9c2ee_66b4_4f18_a697_ac7d3960215c);
windows_core::imp::interface_hierarchy!(IMFSensorTransformFactory, windows_core::IUnknown);
impl IMFSensorTransformFactory {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn GetFactoryAttributes(&self) -> windows_core::Result<super::IMFAttributes> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFactoryAttributes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn InitializeFactory<P1, P2>(&self, dwmaxtransformcount: u32, psensordevices: P1, pattributes: P2) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IMFCollection>,
        P2: windows_core::Param<super::IMFAttributes>,
    {
        unsafe { (windows_core::Interface::vtable(self).InitializeFactory)(windows_core::Interface::as_raw(self), dwmaxtransformcount, psensordevices.param().abi(), pattributes.param().abi()) }
    }
    pub unsafe fn GetTransformCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTransformCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn GetTransformInformation(&self, transformindex: u32, pguidtransformid: *mut windows_core::GUID, ppattributes: *mut Option<super::IMFAttributes>, ppstreaminformation: *mut Option<super::IMFCollection>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTransformInformation)(windows_core::Interface::as_raw(self), transformindex, pguidtransformid as _, core::mem::transmute(ppattributes), core::mem::transmute(ppstreaminformation)) }
    }
    #[cfg(all(feature = "mfobjects", feature = "mftransform"))]
    pub unsafe fn CreateTransform<P1>(&self, guidsensortransformid: *const windows_core::GUID, pattributes: P1) -> windows_core::Result<super::IMFDeviceTransform>
    where
        P1: windows_core::Param<super::IMFAttributes>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTransform)(windows_core::Interface::as_raw(self), guidsensortransformid, pattributes.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFSensorTransformFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "mfobjects")]
    pub GetFactoryAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    GetFactoryAttributes: usize,
    #[cfg(feature = "mfobjects")]
    pub InitializeFactory: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    InitializeFactory: usize,
    pub GetTransformCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "mfobjects")]
    pub GetTransformInformation: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut windows_core::GUID, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    GetTransformInformation: usize,
    #[cfg(all(feature = "mfobjects", feature = "mftransform"))]
    pub CreateTransform: unsafe extern "system" fn(*mut core::ffi::c_void, *const windows_core::GUID, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "mftransform")))]
    CreateTransform: usize,
}
#[cfg(all(feature = "mfobjects", feature = "mftransform"))]
pub trait IMFSensorTransformFactory_Impl: windows_core::IUnknownImpl {
    fn GetFactoryAttributes(&self) -> windows_core::Result<super::IMFAttributes>;
    fn InitializeFactory(&self, dwmaxtransformcount: u32, psensordevices: windows_core::Ref<super::IMFCollection>, pattributes: windows_core::Ref<super::IMFAttributes>) -> windows_core::Result<()>;
    fn GetTransformCount(&self) -> windows_core::Result<u32>;
    fn GetTransformInformation(&self, transformindex: u32, pguidtransformid: *mut windows_core::GUID, ppattributes: windows_core::OutRef<super::IMFAttributes>, ppstreaminformation: windows_core::OutRef<super::IMFCollection>) -> windows_core::Result<()>;
    fn CreateTransform(&self, guidsensortransformid: *const windows_core::GUID, pattributes: windows_core::Ref<super::IMFAttributes>) -> windows_core::Result<super::IMFDeviceTransform>;
}
#[cfg(all(feature = "mfobjects", feature = "mftransform"))]
impl IMFSensorTransformFactory_Vtbl {
    pub const fn new<Identity: IMFSensorTransformFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetFactoryAttributes<Identity: IMFSensorTransformFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppattributes: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSensorTransformFactory_Impl::GetFactoryAttributes(this) {
                    Ok(ok__) => {
                        ppattributes.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn InitializeFactory<Identity: IMFSensorTransformFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwmaxtransformcount: u32, psensordevices: *mut core::ffi::c_void, pattributes: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSensorTransformFactory_Impl::InitializeFactory(this, core::mem::transmute_copy(&dwmaxtransformcount), core::mem::transmute_copy(&psensordevices), core::mem::transmute_copy(&pattributes)).into()
            }
        }
        unsafe extern "system" fn GetTransformCount<Identity: IMFSensorTransformFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwcount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSensorTransformFactory_Impl::GetTransformCount(this) {
                    Ok(ok__) => {
                        pdwcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTransformInformation<Identity: IMFSensorTransformFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, transformindex: u32, pguidtransformid: *mut windows_core::GUID, ppattributes: *mut *mut core::ffi::c_void, ppstreaminformation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSensorTransformFactory_Impl::GetTransformInformation(this, core::mem::transmute_copy(&transformindex), core::mem::transmute_copy(&pguidtransformid), core::mem::transmute_copy(&ppattributes), core::mem::transmute_copy(&ppstreaminformation)).into()
            }
        }
        unsafe extern "system" fn CreateTransform<Identity: IMFSensorTransformFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, guidsensortransformid: *const windows_core::GUID, pattributes: *mut core::ffi::c_void, ppdevicemft: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSensorTransformFactory_Impl::CreateTransform(this, core::mem::transmute_copy(&guidsensortransformid), core::mem::transmute_copy(&pattributes)) {
                    Ok(ok__) => {
                        ppdevicemft.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetFactoryAttributes: GetFactoryAttributes::<Identity, OFFSET>,
            InitializeFactory: InitializeFactory::<Identity, OFFSET>,
            GetTransformCount: GetTransformCount::<Identity, OFFSET>,
            GetTransformInformation: GetTransformInformation::<Identity, OFFSET>,
            CreateTransform: CreateTransform::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFSensorTransformFactory as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mfobjects", feature = "mftransform"))]
impl windows_core::RuntimeName for IMFSensorTransformFactory {}
windows_core::imp::define_interface!(IMFSequencerSource, IMFSequencerSource_Vtbl, 0x197cd219_19cb_4de1_a64c_acf2edcbe59e);
windows_core::imp::interface_hierarchy!(IMFSequencerSource, windows_core::IUnknown);
impl IMFSequencerSource {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn AppendTopology<P0>(&self, ptopology: P0, dwflags: u32) -> windows_core::Result<MFSequencerElementId>
    where
        P0: windows_core::Param<IMFTopology>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AppendTopology)(windows_core::Interface::as_raw(self), ptopology.param().abi(), dwflags, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DeleteTopology(&self, dwid: MFSequencerElementId) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DeleteTopology)(windows_core::Interface::as_raw(self), dwid) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn GetPresentationContext<P0>(&self, ppd: P0, pid: Option<*mut MFSequencerElementId>, pptopology: Option<*mut Option<IMFTopology>>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFPresentationDescriptor>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetPresentationContext)(windows_core::Interface::as_raw(self), ppd.param().abi(), pid.unwrap_or(core::mem::zeroed()) as _, pptopology.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn UpdateTopology<P1>(&self, dwid: MFSequencerElementId, ptopology: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IMFTopology>,
    {
        unsafe { (windows_core::Interface::vtable(self).UpdateTopology)(windows_core::Interface::as_raw(self), dwid, ptopology.param().abi()) }
    }
    pub unsafe fn UpdateTopologyFlags(&self, dwid: MFSequencerElementId, dwflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UpdateTopologyFlags)(windows_core::Interface::as_raw(self), dwid, dwflags) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFSequencerSource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "mfobjects")]
    pub AppendTopology: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *mut MFSequencerElementId) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    AppendTopology: usize,
    pub DeleteTopology: unsafe extern "system" fn(*mut core::ffi::c_void, MFSequencerElementId) -> windows_core::HRESULT,
    #[cfg(feature = "mfobjects")]
    pub GetPresentationContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut MFSequencerElementId, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    GetPresentationContext: usize,
    #[cfg(feature = "mfobjects")]
    pub UpdateTopology: unsafe extern "system" fn(*mut core::ffi::c_void, MFSequencerElementId, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    UpdateTopology: usize,
    pub UpdateTopologyFlags: unsafe extern "system" fn(*mut core::ffi::c_void, MFSequencerElementId, u32) -> windows_core::HRESULT,
}
#[cfg(feature = "mfobjects")]
pub trait IMFSequencerSource_Impl: windows_core::IUnknownImpl {
    fn AppendTopology(&self, ptopology: windows_core::Ref<IMFTopology>, dwflags: u32) -> windows_core::Result<MFSequencerElementId>;
    fn DeleteTopology(&self, dwid: MFSequencerElementId) -> windows_core::Result<()>;
    fn GetPresentationContext(&self, ppd: windows_core::Ref<IMFPresentationDescriptor>, pid: *mut MFSequencerElementId, pptopology: windows_core::OutRef<IMFTopology>) -> windows_core::Result<()>;
    fn UpdateTopology(&self, dwid: MFSequencerElementId, ptopology: windows_core::Ref<IMFTopology>) -> windows_core::Result<()>;
    fn UpdateTopologyFlags(&self, dwid: MFSequencerElementId, dwflags: u32) -> windows_core::Result<()>;
}
#[cfg(feature = "mfobjects")]
impl IMFSequencerSource_Vtbl {
    pub const fn new<Identity: IMFSequencerSource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AppendTopology<Identity: IMFSequencerSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptopology: *mut core::ffi::c_void, dwflags: u32, pdwid: *mut MFSequencerElementId) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSequencerSource_Impl::AppendTopology(this, core::mem::transmute_copy(&ptopology), core::mem::transmute_copy(&dwflags)) {
                    Ok(ok__) => {
                        pdwid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeleteTopology<Identity: IMFSequencerSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwid: MFSequencerElementId) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSequencerSource_Impl::DeleteTopology(this, core::mem::transmute_copy(&dwid)).into()
            }
        }
        unsafe extern "system" fn GetPresentationContext<Identity: IMFSequencerSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppd: *mut core::ffi::c_void, pid: *mut MFSequencerElementId, pptopology: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSequencerSource_Impl::GetPresentationContext(this, core::mem::transmute_copy(&ppd), core::mem::transmute_copy(&pid), core::mem::transmute_copy(&pptopology)).into()
            }
        }
        unsafe extern "system" fn UpdateTopology<Identity: IMFSequencerSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwid: MFSequencerElementId, ptopology: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSequencerSource_Impl::UpdateTopology(this, core::mem::transmute_copy(&dwid), core::mem::transmute_copy(&ptopology)).into()
            }
        }
        unsafe extern "system" fn UpdateTopologyFlags<Identity: IMFSequencerSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwid: MFSequencerElementId, dwflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSequencerSource_Impl::UpdateTopologyFlags(this, core::mem::transmute_copy(&dwid), core::mem::transmute_copy(&dwflags)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AppendTopology: AppendTopology::<Identity, OFFSET>,
            DeleteTopology: DeleteTopology::<Identity, OFFSET>,
            GetPresentationContext: GetPresentationContext::<Identity, OFFSET>,
            UpdateTopology: UpdateTopology::<Identity, OFFSET>,
            UpdateTopologyFlags: UpdateTopologyFlags::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFSequencerSource as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFSequencerSource {}
windows_core::imp::define_interface!(IMFShutdown, IMFShutdown_Vtbl, 0x97ec2ea4_0e42_4937_97ac_9d6d328824e1);
windows_core::imp::interface_hierarchy!(IMFShutdown, windows_core::IUnknown);
impl IMFShutdown {
    pub unsafe fn Shutdown(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Shutdown)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetShutdownStatus(&self) -> windows_core::Result<MFSHUTDOWN_STATUS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetShutdownStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFShutdown_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Shutdown: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetShutdownStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MFSHUTDOWN_STATUS) -> windows_core::HRESULT,
}
pub trait IMFShutdown_Impl: windows_core::IUnknownImpl {
    fn Shutdown(&self) -> windows_core::Result<()>;
    fn GetShutdownStatus(&self) -> windows_core::Result<MFSHUTDOWN_STATUS>;
}
impl IMFShutdown_Vtbl {
    pub const fn new<Identity: IMFShutdown_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Shutdown<Identity: IMFShutdown_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFShutdown_Impl::Shutdown(this).into()
            }
        }
        unsafe extern "system" fn GetShutdownStatus<Identity: IMFShutdown_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatus: *mut MFSHUTDOWN_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFShutdown_Impl::GetShutdownStatus(this) {
                    Ok(ok__) => {
                        pstatus.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Shutdown: Shutdown::<Identity, OFFSET>,
            GetShutdownStatus: GetShutdownStatus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFShutdown as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFShutdown {}
windows_core::imp::define_interface!(IMFSignedLibrary, IMFSignedLibrary_Vtbl, 0x4a724bca_ff6a_4c07_8e0d_7a358421cf06);
windows_core::imp::interface_hierarchy!(IMFSignedLibrary, windows_core::IUnknown);
impl IMFSignedLibrary {
    pub unsafe fn GetProcedureAddress<P0>(&self, name: P0, address: *mut *mut core::ffi::c_void) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).GetProcedureAddress)(windows_core::Interface::as_raw(self), name.param().abi(), address as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFSignedLibrary_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetProcedureAddress: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFSignedLibrary_Impl: windows_core::IUnknownImpl {
    fn GetProcedureAddress(&self, name: &windows_core::PCSTR, address: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IMFSignedLibrary_Vtbl {
    pub const fn new<Identity: IMFSignedLibrary_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetProcedureAddress<Identity: IMFSignedLibrary_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCSTR, address: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSignedLibrary_Impl::GetProcedureAddress(this, core::mem::transmute(&name), core::mem::transmute_copy(&address)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetProcedureAddress: GetProcedureAddress::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFSignedLibrary as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFSignedLibrary {}
windows_core::imp::define_interface!(IMFSimpleAudioVolume, IMFSimpleAudioVolume_Vtbl, 0x089edf13_cf71_4338_8d13_9e569dbdc319);
windows_core::imp::interface_hierarchy!(IMFSimpleAudioVolume, windows_core::IUnknown);
impl IMFSimpleAudioVolume {
    pub unsafe fn SetMasterVolume(&self, flevel: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMasterVolume)(windows_core::Interface::as_raw(self), flevel) }
    }
    pub unsafe fn GetMasterVolume(&self) -> windows_core::Result<f32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMasterVolume)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMute(&self, bmute: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMute)(windows_core::Interface::as_raw(self), bmute.into()) }
    }
    pub unsafe fn GetMute(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMute)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFSimpleAudioVolume_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetMasterVolume: unsafe extern "system" fn(*mut core::ffi::c_void, f32) -> windows_core::HRESULT,
    pub GetMasterVolume: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f32) -> windows_core::HRESULT,
    pub SetMute: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetMute: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IMFSimpleAudioVolume_Impl: windows_core::IUnknownImpl {
    fn SetMasterVolume(&self, flevel: f32) -> windows_core::Result<()>;
    fn GetMasterVolume(&self) -> windows_core::Result<f32>;
    fn SetMute(&self, bmute: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetMute(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IMFSimpleAudioVolume_Vtbl {
    pub const fn new<Identity: IMFSimpleAudioVolume_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetMasterVolume<Identity: IMFSimpleAudioVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flevel: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSimpleAudioVolume_Impl::SetMasterVolume(this, core::mem::transmute_copy(&flevel)).into()
            }
        }
        unsafe extern "system" fn GetMasterVolume<Identity: IMFSimpleAudioVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pflevel: *mut f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSimpleAudioVolume_Impl::GetMasterVolume(this) {
                    Ok(ok__) => {
                        pflevel.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMute<Identity: IMFSimpleAudioVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, bmute: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSimpleAudioVolume_Impl::SetMute(this, core::mem::transmute_copy(&bmute)).into()
            }
        }
        unsafe extern "system" fn GetMute<Identity: IMFSimpleAudioVolume_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbmute: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFSimpleAudioVolume_Impl::GetMute(this) {
                    Ok(ok__) => {
                        pbmute.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetMasterVolume: SetMasterVolume::<Identity, OFFSET>,
            GetMasterVolume: GetMasterVolume::<Identity, OFFSET>,
            SetMute: SetMute::<Identity, OFFSET>,
            GetMute: GetMute::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFSimpleAudioVolume as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFSimpleAudioVolume {}
windows_core::imp::define_interface!(IMFSourceOpenMonitor, IMFSourceOpenMonitor_Vtbl, 0x059054b3_027c_494c_a27d_9113291cf87f);
windows_core::imp::interface_hierarchy!(IMFSourceOpenMonitor, windows_core::IUnknown);
impl IMFSourceOpenMonitor {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn OnSourceEvent<P0>(&self, pevent: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMFMediaEvent>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnSourceEvent)(windows_core::Interface::as_raw(self), pevent.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFSourceOpenMonitor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "mfobjects")]
    pub OnSourceEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    OnSourceEvent: usize,
}
#[cfg(feature = "mfobjects")]
pub trait IMFSourceOpenMonitor_Impl: windows_core::IUnknownImpl {
    fn OnSourceEvent(&self, pevent: windows_core::Ref<super::IMFMediaEvent>) -> windows_core::Result<()>;
}
#[cfg(feature = "mfobjects")]
impl IMFSourceOpenMonitor_Vtbl {
    pub const fn new<Identity: IMFSourceOpenMonitor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnSourceEvent<Identity: IMFSourceOpenMonitor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pevent: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceOpenMonitor_Impl::OnSourceEvent(this, core::mem::transmute_copy(&pevent)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnSourceEvent: OnSourceEvent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFSourceOpenMonitor as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFSourceOpenMonitor {}
windows_core::imp::define_interface!(IMFSourceResolver, IMFSourceResolver_Vtbl, 0xfbe5a32d_a497_4b61_bb85_97b1a848a6e3);
windows_core::imp::interface_hierarchy!(IMFSourceResolver, windows_core::IUnknown);
impl IMFSourceResolver {
    #[cfg(feature = "propsys")]
    pub unsafe fn CreateObjectFromURL<P0, P2>(&self, pwszurl: P0, dwflags: u32, pprops: P2, pobjecttype: *mut MF_OBJECT_TYPE, ppobject: *mut Option<windows_core::IUnknown>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<super::IPropertyStore>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateObjectFromURL)(windows_core::Interface::as_raw(self), pwszurl.param().abi(), dwflags, pprops.param().abi(), pobjecttype as _, core::mem::transmute(ppobject)) }
    }
    #[cfg(all(feature = "mfobjects", feature = "propsys"))]
    pub unsafe fn CreateObjectFromByteStream<P0, P1, P3>(&self, pbytestream: P0, pwszurl: P1, dwflags: u32, pprops: P3, pobjecttype: *mut MF_OBJECT_TYPE, ppobject: *mut Option<windows_core::IUnknown>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMFByteStream>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<super::IPropertyStore>,
    {
        unsafe { (windows_core::Interface::vtable(self).CreateObjectFromByteStream)(windows_core::Interface::as_raw(self), pbytestream.param().abi(), pwszurl.param().abi(), dwflags, pprops.param().abi(), pobjecttype as _, core::mem::transmute(ppobject)) }
    }
    #[cfg(all(feature = "mfobjects", feature = "propsys"))]
    pub unsafe fn BeginCreateObjectFromURL<P0, P2, P4, P5>(&self, pwszurl: P0, dwflags: u32, pprops: P2, ppiunknowncancelcookie: *mut Option<windows_core::IUnknown>, pcallback: P4, punkstate: P5) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<super::IPropertyStore>,
        P4: windows_core::Param<super::IMFAsyncCallback>,
        P5: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).BeginCreateObjectFromURL)(windows_core::Interface::as_raw(self), pwszurl.param().abi(), dwflags, pprops.param().abi(), core::mem::transmute(ppiunknowncancelcookie), pcallback.param().abi(), punkstate.param().abi()) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn EndCreateObjectFromURL<P0>(&self, presult: P0, pobjecttype: *mut MF_OBJECT_TYPE, ppobject: *mut Option<windows_core::IUnknown>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMFAsyncResult>,
    {
        unsafe { (windows_core::Interface::vtable(self).EndCreateObjectFromURL)(windows_core::Interface::as_raw(self), presult.param().abi(), pobjecttype as _, core::mem::transmute(ppobject)) }
    }
    #[cfg(all(feature = "mfobjects", feature = "propsys"))]
    pub unsafe fn BeginCreateObjectFromByteStream<P0, P1, P3, P5, P6>(&self, pbytestream: P0, pwszurl: P1, dwflags: u32, pprops: P3, ppiunknowncancelcookie: *mut Option<windows_core::IUnknown>, pcallback: P5, punkstate: P6) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMFByteStream>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<super::IPropertyStore>,
        P5: windows_core::Param<super::IMFAsyncCallback>,
        P6: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).BeginCreateObjectFromByteStream)(windows_core::Interface::as_raw(self), pbytestream.param().abi(), pwszurl.param().abi(), dwflags, pprops.param().abi(), core::mem::transmute(ppiunknowncancelcookie), pcallback.param().abi(), punkstate.param().abi()) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn EndCreateObjectFromByteStream<P0>(&self, presult: P0, pobjecttype: *mut MF_OBJECT_TYPE, ppobject: *mut Option<windows_core::IUnknown>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMFAsyncResult>,
    {
        unsafe { (windows_core::Interface::vtable(self).EndCreateObjectFromByteStream)(windows_core::Interface::as_raw(self), presult.param().abi(), pobjecttype as _, core::mem::transmute(ppobject)) }
    }
    pub unsafe fn CancelObjectCreation<P0>(&self, piunknowncancelcookie: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CancelObjectCreation)(windows_core::Interface::as_raw(self), piunknowncancelcookie.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFSourceResolver_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "propsys")]
    pub CreateObjectFromURL: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut core::ffi::c_void, *mut MF_OBJECT_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "propsys"))]
    CreateObjectFromURL: usize,
    #[cfg(all(feature = "mfobjects", feature = "propsys"))]
    pub CreateObjectFromByteStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut core::ffi::c_void, *mut MF_OBJECT_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "propsys")))]
    CreateObjectFromByteStream: usize,
    #[cfg(all(feature = "mfobjects", feature = "propsys"))]
    pub BeginCreateObjectFromURL: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "propsys")))]
    BeginCreateObjectFromURL: usize,
    #[cfg(feature = "mfobjects")]
    pub EndCreateObjectFromURL: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut MF_OBJECT_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    EndCreateObjectFromURL: usize,
    #[cfg(all(feature = "mfobjects", feature = "propsys"))]
    pub BeginCreateObjectFromByteStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::PCWSTR, u32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "propsys")))]
    BeginCreateObjectFromByteStream: usize,
    #[cfg(feature = "mfobjects")]
    pub EndCreateObjectFromByteStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut MF_OBJECT_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    EndCreateObjectFromByteStream: usize,
    pub CancelObjectCreation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "mfobjects", feature = "propsys"))]
pub trait IMFSourceResolver_Impl: windows_core::IUnknownImpl {
    fn CreateObjectFromURL(&self, pwszurl: &windows_core::PCWSTR, dwflags: u32, pprops: windows_core::Ref<super::IPropertyStore>, pobjecttype: *mut MF_OBJECT_TYPE, ppobject: windows_core::OutRef<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn CreateObjectFromByteStream(&self, pbytestream: windows_core::Ref<super::IMFByteStream>, pwszurl: &windows_core::PCWSTR, dwflags: u32, pprops: windows_core::Ref<super::IPropertyStore>, pobjecttype: *mut MF_OBJECT_TYPE, ppobject: windows_core::OutRef<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn BeginCreateObjectFromURL(&self, pwszurl: &windows_core::PCWSTR, dwflags: u32, pprops: windows_core::Ref<super::IPropertyStore>, ppiunknowncancelcookie: windows_core::OutRef<windows_core::IUnknown>, pcallback: windows_core::Ref<super::IMFAsyncCallback>, punkstate: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn EndCreateObjectFromURL(&self, presult: windows_core::Ref<super::IMFAsyncResult>, pobjecttype: *mut MF_OBJECT_TYPE, ppobject: windows_core::OutRef<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn BeginCreateObjectFromByteStream(&self, pbytestream: windows_core::Ref<super::IMFByteStream>, pwszurl: &windows_core::PCWSTR, dwflags: u32, pprops: windows_core::Ref<super::IPropertyStore>, ppiunknowncancelcookie: windows_core::OutRef<windows_core::IUnknown>, pcallback: windows_core::Ref<super::IMFAsyncCallback>, punkstate: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn EndCreateObjectFromByteStream(&self, presult: windows_core::Ref<super::IMFAsyncResult>, pobjecttype: *mut MF_OBJECT_TYPE, ppobject: windows_core::OutRef<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn CancelObjectCreation(&self, piunknowncancelcookie: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "mfobjects", feature = "propsys"))]
impl IMFSourceResolver_Vtbl {
    pub const fn new<Identity: IMFSourceResolver_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateObjectFromURL<Identity: IMFSourceResolver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszurl: windows_core::PCWSTR, dwflags: u32, pprops: *mut core::ffi::c_void, pobjecttype: *mut MF_OBJECT_TYPE, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceResolver_Impl::CreateObjectFromURL(this, core::mem::transmute(&pwszurl), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pprops), core::mem::transmute_copy(&pobjecttype), core::mem::transmute_copy(&ppobject)).into()
            }
        }
        unsafe extern "system" fn CreateObjectFromByteStream<Identity: IMFSourceResolver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbytestream: *mut core::ffi::c_void, pwszurl: windows_core::PCWSTR, dwflags: u32, pprops: *mut core::ffi::c_void, pobjecttype: *mut MF_OBJECT_TYPE, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceResolver_Impl::CreateObjectFromByteStream(this, core::mem::transmute_copy(&pbytestream), core::mem::transmute(&pwszurl), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pprops), core::mem::transmute_copy(&pobjecttype), core::mem::transmute_copy(&ppobject)).into()
            }
        }
        unsafe extern "system" fn BeginCreateObjectFromURL<Identity: IMFSourceResolver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszurl: windows_core::PCWSTR, dwflags: u32, pprops: *mut core::ffi::c_void, ppiunknowncancelcookie: *mut *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void, punkstate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceResolver_Impl::BeginCreateObjectFromURL(this, core::mem::transmute(&pwszurl), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pprops), core::mem::transmute_copy(&ppiunknowncancelcookie), core::mem::transmute_copy(&pcallback), core::mem::transmute_copy(&punkstate)).into()
            }
        }
        unsafe extern "system" fn EndCreateObjectFromURL<Identity: IMFSourceResolver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presult: *mut core::ffi::c_void, pobjecttype: *mut MF_OBJECT_TYPE, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceResolver_Impl::EndCreateObjectFromURL(this, core::mem::transmute_copy(&presult), core::mem::transmute_copy(&pobjecttype), core::mem::transmute_copy(&ppobject)).into()
            }
        }
        unsafe extern "system" fn BeginCreateObjectFromByteStream<Identity: IMFSourceResolver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbytestream: *mut core::ffi::c_void, pwszurl: windows_core::PCWSTR, dwflags: u32, pprops: *mut core::ffi::c_void, ppiunknowncancelcookie: *mut *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void, punkstate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceResolver_Impl::BeginCreateObjectFromByteStream(this, core::mem::transmute_copy(&pbytestream), core::mem::transmute(&pwszurl), core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&pprops), core::mem::transmute_copy(&ppiunknowncancelcookie), core::mem::transmute_copy(&pcallback), core::mem::transmute_copy(&punkstate)).into()
            }
        }
        unsafe extern "system" fn EndCreateObjectFromByteStream<Identity: IMFSourceResolver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presult: *mut core::ffi::c_void, pobjecttype: *mut MF_OBJECT_TYPE, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceResolver_Impl::EndCreateObjectFromByteStream(this, core::mem::transmute_copy(&presult), core::mem::transmute_copy(&pobjecttype), core::mem::transmute_copy(&ppobject)).into()
            }
        }
        unsafe extern "system" fn CancelObjectCreation<Identity: IMFSourceResolver_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, piunknowncancelcookie: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSourceResolver_Impl::CancelObjectCreation(this, core::mem::transmute_copy(&piunknowncancelcookie)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateObjectFromURL: CreateObjectFromURL::<Identity, OFFSET>,
            CreateObjectFromByteStream: CreateObjectFromByteStream::<Identity, OFFSET>,
            BeginCreateObjectFromURL: BeginCreateObjectFromURL::<Identity, OFFSET>,
            EndCreateObjectFromURL: EndCreateObjectFromURL::<Identity, OFFSET>,
            BeginCreateObjectFromByteStream: BeginCreateObjectFromByteStream::<Identity, OFFSET>,
            EndCreateObjectFromByteStream: EndCreateObjectFromByteStream::<Identity, OFFSET>,
            CancelObjectCreation: CancelObjectCreation::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFSourceResolver as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mfobjects", feature = "propsys"))]
impl windows_core::RuntimeName for IMFSourceResolver {}
#[cfg(feature = "mfobjects")]
windows_core::imp::define_interface!(IMFStreamDescriptor, IMFStreamDescriptor_Vtbl, 0x56c03d9c_9dbb_45f5_ab4b_d80f47c05938);
#[cfg(feature = "mfobjects")]
impl core::ops::Deref for IMFStreamDescriptor {
    type Target = super::IMFAttributes;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "mfobjects")]
windows_core::imp::interface_hierarchy!(IMFStreamDescriptor, windows_core::IUnknown, super::IMFAttributes);
#[cfg(feature = "mfobjects")]
impl IMFStreamDescriptor {
    pub unsafe fn GetStreamIdentifier(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStreamIdentifier)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMediaTypeHandler(&self) -> windows_core::Result<IMFMediaTypeHandler> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMediaTypeHandler)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "mfobjects")]
#[repr(C)]
#[doc(hidden)]
pub struct IMFStreamDescriptor_Vtbl {
    pub base__: super::IMFAttributes_Vtbl,
    pub GetStreamIdentifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetMediaTypeHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMFStreamDescriptor_Impl: super::IMFAttributes_Impl {
    fn GetStreamIdentifier(&self) -> windows_core::Result<u32>;
    fn GetMediaTypeHandler(&self) -> windows_core::Result<IMFMediaTypeHandler>;
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl IMFStreamDescriptor_Vtbl {
    pub const fn new<Identity: IMFStreamDescriptor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetStreamIdentifier<Identity: IMFStreamDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstreamidentifier: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFStreamDescriptor_Impl::GetStreamIdentifier(this) {
                    Ok(ok__) => {
                        pdwstreamidentifier.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMediaTypeHandler<Identity: IMFStreamDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmediatypehandler: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFStreamDescriptor_Impl::GetMediaTypeHandler(this) {
                    Ok(ok__) => {
                        ppmediatypehandler.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IMFAttributes_Vtbl::new::<Identity, OFFSET>(),
            GetStreamIdentifier: GetStreamIdentifier::<Identity, OFFSET>,
            GetMediaTypeHandler: GetMediaTypeHandler::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFStreamDescriptor as windows_core::Interface>::IID || iid == &<super::IMFAttributes as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMFStreamDescriptor {}
#[cfg(feature = "mfobjects")]
windows_core::imp::define_interface!(IMFStreamSink, IMFStreamSink_Vtbl, 0x0a97b3cf_8e7c_4a3d_8f8c_0c843dc247fb);
#[cfg(feature = "mfobjects")]
impl core::ops::Deref for IMFStreamSink {
    type Target = super::IMFMediaEventGenerator;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "mfobjects")]
windows_core::imp::interface_hierarchy!(IMFStreamSink, windows_core::IUnknown, super::IMFMediaEventGenerator);
#[cfg(feature = "mfobjects")]
impl IMFStreamSink {
    pub unsafe fn GetMediaSink(&self) -> windows_core::Result<IMFMediaSink> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMediaSink)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetIdentifier(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIdentifier)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMediaTypeHandler(&self) -> windows_core::Result<IMFMediaTypeHandler> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMediaTypeHandler)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ProcessSample<P0>(&self, psample: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMFSample>,
    {
        unsafe { (windows_core::Interface::vtable(self).ProcessSample)(windows_core::Interface::as_raw(self), psample.param().abi()) }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn PlaceMarker(&self, emarkertype: MFSTREAMSINK_MARKER_TYPE, pvarmarkervalue: *const super::PROPVARIANT, pvarcontextvalue: *const super::PROPVARIANT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PlaceMarker)(windows_core::Interface::as_raw(self), emarkertype, pvarmarkervalue, pvarcontextvalue) }
    }
    pub unsafe fn Flush(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Flush)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "mfobjects")]
#[repr(C)]
#[doc(hidden)]
pub struct IMFStreamSink_Vtbl {
    pub base__: super::IMFMediaEventGenerator_Vtbl,
    pub GetMediaSink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetIdentifier: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetMediaTypeHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProcessSample: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub PlaceMarker: unsafe extern "system" fn(*mut core::ffi::c_void, MFSTREAMSINK_MARKER_TYPE, *const super::PROPVARIANT, *const super::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    PlaceMarker: usize,
    pub Flush: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMFStreamSink_Impl: super::IMFMediaEventGenerator_Impl {
    fn GetMediaSink(&self) -> windows_core::Result<IMFMediaSink>;
    fn GetIdentifier(&self) -> windows_core::Result<u32>;
    fn GetMediaTypeHandler(&self) -> windows_core::Result<IMFMediaTypeHandler>;
    fn ProcessSample(&self, psample: windows_core::Ref<super::IMFSample>) -> windows_core::Result<()>;
    fn PlaceMarker(&self, emarkertype: MFSTREAMSINK_MARKER_TYPE, pvarmarkervalue: *const super::PROPVARIANT, pvarcontextvalue: *const super::PROPVARIANT) -> windows_core::Result<()>;
    fn Flush(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl IMFStreamSink_Vtbl {
    pub const fn new<Identity: IMFStreamSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMediaSink<Identity: IMFStreamSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppmediasink: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFStreamSink_Impl::GetMediaSink(this) {
                    Ok(ok__) => {
                        ppmediasink.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetIdentifier<Identity: IMFStreamSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwidentifier: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFStreamSink_Impl::GetIdentifier(this) {
                    Ok(ok__) => {
                        pdwidentifier.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMediaTypeHandler<Identity: IMFStreamSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pphandler: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFStreamSink_Impl::GetMediaTypeHandler(this) {
                    Ok(ok__) => {
                        pphandler.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ProcessSample<Identity: IMFStreamSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psample: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFStreamSink_Impl::ProcessSample(this, core::mem::transmute_copy(&psample)).into()
            }
        }
        unsafe extern "system" fn PlaceMarker<Identity: IMFStreamSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, emarkertype: MFSTREAMSINK_MARKER_TYPE, pvarmarkervalue: *const super::PROPVARIANT, pvarcontextvalue: *const super::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFStreamSink_Impl::PlaceMarker(this, core::mem::transmute_copy(&emarkertype), core::mem::transmute_copy(&pvarmarkervalue), core::mem::transmute_copy(&pvarcontextvalue)).into()
            }
        }
        unsafe extern "system" fn Flush<Identity: IMFStreamSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFStreamSink_Impl::Flush(this).into()
            }
        }
        Self {
            base__: super::IMFMediaEventGenerator_Vtbl::new::<Identity, OFFSET>(),
            GetMediaSink: GetMediaSink::<Identity, OFFSET>,
            GetIdentifier: GetIdentifier::<Identity, OFFSET>,
            GetMediaTypeHandler: GetMediaTypeHandler::<Identity, OFFSET>,
            ProcessSample: ProcessSample::<Identity, OFFSET>,
            PlaceMarker: PlaceMarker::<Identity, OFFSET>,
            Flush: Flush::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFStreamSink as windows_core::Interface>::IID || iid == &<super::IMFMediaEventGenerator as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMFStreamSink {}
windows_core::imp::define_interface!(IMFStreamingSinkConfig, IMFStreamingSinkConfig_Vtbl, 0x9db7aa41_3cc5_40d4_8509_555804ad34cc);
windows_core::imp::interface_hierarchy!(IMFStreamingSinkConfig, windows_core::IUnknown);
impl IMFStreamingSinkConfig {
    pub unsafe fn StartStreaming(&self, fseekoffsetisbyteoffset: bool, qwseekoffset: u64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).StartStreaming)(windows_core::Interface::as_raw(self), fseekoffsetisbyteoffset.into(), qwseekoffset) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFStreamingSinkConfig_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub StartStreaming: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, u64) -> windows_core::HRESULT,
}
pub trait IMFStreamingSinkConfig_Impl: windows_core::IUnknownImpl {
    fn StartStreaming(&self, fseekoffsetisbyteoffset: windows_core::BOOL, qwseekoffset: u64) -> windows_core::Result<()>;
}
impl IMFStreamingSinkConfig_Vtbl {
    pub const fn new<Identity: IMFStreamingSinkConfig_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StartStreaming<Identity: IMFStreamingSinkConfig_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fseekoffsetisbyteoffset: windows_core::BOOL, qwseekoffset: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFStreamingSinkConfig_Impl::StartStreaming(this, core::mem::transmute_copy(&fseekoffsetisbyteoffset), core::mem::transmute_copy(&qwseekoffset)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), StartStreaming: StartStreaming::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFStreamingSinkConfig as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFStreamingSinkConfig {}
windows_core::imp::define_interface!(IMFSystemId, IMFSystemId_Vtbl, 0xfff4af3a_1fc1_4ef9_a29b_d26c49e2f31a);
windows_core::imp::interface_hierarchy!(IMFSystemId, windows_core::IUnknown);
impl IMFSystemId {
    pub unsafe fn GetData(&self, size: *mut u32, data: *mut *mut u8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetData)(windows_core::Interface::as_raw(self), size as _, data as _) }
    }
    pub unsafe fn Setup(&self, stage: u32, pbin: &[u8], pcbout: *mut u32, ppbout: *mut *mut u8) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Setup)(windows_core::Interface::as_raw(self), stage, pbin.len().try_into().unwrap(), pbin.as_ptr(), pcbout as _, ppbout as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFSystemId_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
    pub Setup: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *const u8, *mut u32, *mut *mut u8) -> windows_core::HRESULT,
}
pub trait IMFSystemId_Impl: windows_core::IUnknownImpl {
    fn GetData(&self, size: *mut u32, data: *mut *mut u8) -> windows_core::Result<()>;
    fn Setup(&self, stage: u32, cbin: u32, pbin: *const u8, pcbout: *mut u32, ppbout: *mut *mut u8) -> windows_core::Result<()>;
}
impl IMFSystemId_Vtbl {
    pub const fn new<Identity: IMFSystemId_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetData<Identity: IMFSystemId_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, size: *mut u32, data: *mut *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSystemId_Impl::GetData(this, core::mem::transmute_copy(&size), core::mem::transmute_copy(&data)).into()
            }
        }
        unsafe extern "system" fn Setup<Identity: IMFSystemId_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stage: u32, cbin: u32, pbin: *const u8, pcbout: *mut u32, ppbout: *mut *mut u8) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFSystemId_Impl::Setup(this, core::mem::transmute_copy(&stage), core::mem::transmute_copy(&cbin), core::mem::transmute_copy(&pbin), core::mem::transmute_copy(&pcbout), core::mem::transmute_copy(&ppbout)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetData: GetData::<Identity, OFFSET>, Setup: Setup::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFSystemId as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFSystemId {}
windows_core::imp::define_interface!(IMFTimecodeTranslate, IMFTimecodeTranslate_Vtbl, 0xab9d8661_f7e8_4ef4_9861_89f334f94e74);
windows_core::imp::interface_hierarchy!(IMFTimecodeTranslate, windows_core::IUnknown);
impl IMFTimecodeTranslate {
    #[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn BeginConvertTimecodeToHNS<P1, P2>(&self, ppropvartimecode: *const super::PROPVARIANT, pcallback: P1, punkstate: P2) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IMFAsyncCallback>,
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).BeginConvertTimecodeToHNS)(windows_core::Interface::as_raw(self), ppropvartimecode, pcallback.param().abi(), punkstate.param().abi()) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn EndConvertTimecodeToHNS<P0>(&self, presult: P0) -> windows_core::Result<MFTIME>
    where
        P0: windows_core::Param<super::IMFAsyncResult>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EndConvertTimecodeToHNS)(windows_core::Interface::as_raw(self), presult.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn BeginConvertHNSToTimecode<P1, P2>(&self, hnstime: MFTIME, pcallback: P1, punkstate: P2) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IMFAsyncCallback>,
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).BeginConvertHNSToTimecode)(windows_core::Interface::as_raw(self), hnstime, pcallback.param().abi(), punkstate.param().abi()) }
    }
    #[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn EndConvertHNSToTimecode<P0>(&self, presult: P0) -> windows_core::Result<super::PROPVARIANT>
    where
        P0: windows_core::Param<super::IMFAsyncResult>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EndConvertHNSToTimecode)(windows_core::Interface::as_raw(self), presult.param().abi(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFTimecodeTranslate_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub BeginConvertTimecodeToHNS: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::PROPVARIANT, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    BeginConvertTimecodeToHNS: usize,
    #[cfg(feature = "mfobjects")]
    pub EndConvertTimecodeToHNS: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut MFTIME) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    EndConvertTimecodeToHNS: usize,
    #[cfg(feature = "mfobjects")]
    pub BeginConvertHNSToTimecode: unsafe extern "system" fn(*mut core::ffi::c_void, MFTIME, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    BeginConvertHNSToTimecode: usize,
    #[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
    pub EndConvertHNSToTimecode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::PROPVARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase")))]
    EndConvertHNSToTimecode: usize,
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMFTimecodeTranslate_Impl: windows_core::IUnknownImpl {
    fn BeginConvertTimecodeToHNS(&self, ppropvartimecode: *const super::PROPVARIANT, pcallback: windows_core::Ref<super::IMFAsyncCallback>, punkstate: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn EndConvertTimecodeToHNS(&self, presult: windows_core::Ref<super::IMFAsyncResult>) -> windows_core::Result<MFTIME>;
    fn BeginConvertHNSToTimecode(&self, hnstime: MFTIME, pcallback: windows_core::Ref<super::IMFAsyncCallback>, punkstate: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn EndConvertHNSToTimecode(&self, presult: windows_core::Ref<super::IMFAsyncResult>) -> windows_core::Result<super::PROPVARIANT>;
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl IMFTimecodeTranslate_Vtbl {
    pub const fn new<Identity: IMFTimecodeTranslate_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BeginConvertTimecodeToHNS<Identity: IMFTimecodeTranslate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppropvartimecode: *const super::PROPVARIANT, pcallback: *mut core::ffi::c_void, punkstate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTimecodeTranslate_Impl::BeginConvertTimecodeToHNS(this, core::mem::transmute_copy(&ppropvartimecode), core::mem::transmute_copy(&pcallback), core::mem::transmute_copy(&punkstate)).into()
            }
        }
        unsafe extern "system" fn EndConvertTimecodeToHNS<Identity: IMFTimecodeTranslate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presult: *mut core::ffi::c_void, phnstime: *mut MFTIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimecodeTranslate_Impl::EndConvertTimecodeToHNS(this, core::mem::transmute_copy(&presult)) {
                    Ok(ok__) => {
                        phnstime.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BeginConvertHNSToTimecode<Identity: IMFTimecodeTranslate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hnstime: MFTIME, pcallback: *mut core::ffi::c_void, punkstate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTimecodeTranslate_Impl::BeginConvertHNSToTimecode(this, core::mem::transmute_copy(&hnstime), core::mem::transmute_copy(&pcallback), core::mem::transmute_copy(&punkstate)).into()
            }
        }
        unsafe extern "system" fn EndConvertHNSToTimecode<Identity: IMFTimecodeTranslate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presult: *mut core::ffi::c_void, ppropvartimecode: *mut super::PROPVARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimecodeTranslate_Impl::EndConvertHNSToTimecode(this, core::mem::transmute_copy(&presult)) {
                    Ok(ok__) => {
                        ppropvartimecode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginConvertTimecodeToHNS: BeginConvertTimecodeToHNS::<Identity, OFFSET>,
            EndConvertTimecodeToHNS: EndConvertTimecodeToHNS::<Identity, OFFSET>,
            BeginConvertHNSToTimecode: BeginConvertHNSToTimecode::<Identity, OFFSET>,
            EndConvertHNSToTimecode: EndConvertHNSToTimecode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFTimecodeTranslate as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMFTimecodeTranslate {}
windows_core::imp::define_interface!(IMFTimer, IMFTimer_Vtbl, 0xe56e4cbd_8f70_49d8_a0f8_edb3d6ab9bf2);
windows_core::imp::interface_hierarchy!(IMFTimer, windows_core::IUnknown);
impl IMFTimer {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn SetTimer<P2, P3>(&self, dwflags: u32, llclocktime: i64, pcallback: P2, punkstate: P3) -> windows_core::Result<windows_core::IUnknown>
    where
        P2: windows_core::Param<super::IMFAsyncCallback>,
        P3: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SetTimer)(windows_core::Interface::as_raw(self), dwflags, llclocktime, pcallback.param().abi(), punkstate.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CancelTimer<P0>(&self, punkkey: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).CancelTimer)(windows_core::Interface::as_raw(self), punkkey.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFTimer_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "mfobjects")]
    pub SetTimer: unsafe extern "system" fn(*mut core::ffi::c_void, u32, i64, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    SetTimer: usize,
    pub CancelTimer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "mfobjects")]
pub trait IMFTimer_Impl: windows_core::IUnknownImpl {
    fn SetTimer(&self, dwflags: u32, llclocktime: i64, pcallback: windows_core::Ref<super::IMFAsyncCallback>, punkstate: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<windows_core::IUnknown>;
    fn CancelTimer(&self, punkkey: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(feature = "mfobjects")]
impl IMFTimer_Vtbl {
    pub const fn new<Identity: IMFTimer_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetTimer<Identity: IMFTimer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwflags: u32, llclocktime: i64, pcallback: *mut core::ffi::c_void, punkstate: *mut core::ffi::c_void, ppunkkey: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTimer_Impl::SetTimer(this, core::mem::transmute_copy(&dwflags), core::mem::transmute_copy(&llclocktime), core::mem::transmute_copy(&pcallback), core::mem::transmute_copy(&punkstate)) {
                    Ok(ok__) => {
                        ppunkkey.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CancelTimer<Identity: IMFTimer_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, punkkey: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTimer_Impl::CancelTimer(this, core::mem::transmute_copy(&punkkey)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetTimer: SetTimer::<Identity, OFFSET>,
            CancelTimer: CancelTimer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFTimer as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFTimer {}
windows_core::imp::define_interface!(IMFTopoLoader, IMFTopoLoader_Vtbl, 0xde9a6157_f660_4643_b56a_df9f7998c7cd);
windows_core::imp::interface_hierarchy!(IMFTopoLoader, windows_core::IUnknown);
impl IMFTopoLoader {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn Load<P0, P2>(&self, pinputtopo: P0, ppoutputtopo: *mut Option<IMFTopology>, pcurrenttopo: P2) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFTopology>,
        P2: windows_core::Param<IMFTopology>,
    {
        unsafe { (windows_core::Interface::vtable(self).Load)(windows_core::Interface::as_raw(self), pinputtopo.param().abi(), core::mem::transmute(ppoutputtopo), pcurrenttopo.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFTopoLoader_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "mfobjects")]
    pub Load: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    Load: usize,
}
#[cfg(feature = "mfobjects")]
pub trait IMFTopoLoader_Impl: windows_core::IUnknownImpl {
    fn Load(&self, pinputtopo: windows_core::Ref<IMFTopology>, ppoutputtopo: windows_core::OutRef<IMFTopology>, pcurrenttopo: windows_core::Ref<IMFTopology>) -> windows_core::Result<()>;
}
#[cfg(feature = "mfobjects")]
impl IMFTopoLoader_Vtbl {
    pub const fn new<Identity: IMFTopoLoader_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Load<Identity: IMFTopoLoader_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinputtopo: *mut core::ffi::c_void, ppoutputtopo: *mut *mut core::ffi::c_void, pcurrenttopo: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTopoLoader_Impl::Load(this, core::mem::transmute_copy(&pinputtopo), core::mem::transmute_copy(&ppoutputtopo), core::mem::transmute_copy(&pcurrenttopo)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Load: Load::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFTopoLoader as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFTopoLoader {}
#[cfg(feature = "mfobjects")]
windows_core::imp::define_interface!(IMFTopology, IMFTopology_Vtbl, 0x83cf873a_f6da_4bc8_823f_bacfd55dc433);
#[cfg(feature = "mfobjects")]
impl core::ops::Deref for IMFTopology {
    type Target = super::IMFAttributes;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "mfobjects")]
windows_core::imp::interface_hierarchy!(IMFTopology, windows_core::IUnknown, super::IMFAttributes);
#[cfg(feature = "mfobjects")]
impl IMFTopology {
    pub unsafe fn GetTopologyID(&self) -> windows_core::Result<TOPOID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTopologyID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AddNode<P0>(&self, pnode: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFTopologyNode>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddNode)(windows_core::Interface::as_raw(self), pnode.param().abi()) }
    }
    pub unsafe fn RemoveNode<P0>(&self, pnode: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFTopologyNode>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveNode)(windows_core::Interface::as_raw(self), pnode.param().abi()) }
    }
    pub unsafe fn GetNodeCount(&self) -> windows_core::Result<u16> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNodeCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetNode(&self, windex: u16) -> windows_core::Result<IMFTopologyNode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNode)(windows_core::Interface::as_raw(self), windex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Clear(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Clear)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn CloneFrom<P0>(&self, ptopology: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).CloneFrom)(windows_core::Interface::as_raw(self), ptopology.param().abi()) }
    }
    pub unsafe fn GetNodeByID(&self, qwtoponodeid: TOPOID) -> windows_core::Result<IMFTopologyNode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNodeByID)(windows_core::Interface::as_raw(self), qwtoponodeid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSourceNodeCollection(&self) -> windows_core::Result<super::IMFCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSourceNodeCollection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetOutputNodeCollection(&self) -> windows_core::Result<super::IMFCollection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputNodeCollection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "mfobjects")]
#[repr(C)]
#[doc(hidden)]
pub struct IMFTopology_Vtbl {
    pub base__: super::IMFAttributes_Vtbl,
    pub GetTopologyID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TOPOID) -> windows_core::HRESULT,
    pub AddNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveNode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNodeCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u16) -> windows_core::HRESULT,
    pub GetNode: unsafe extern "system" fn(*mut core::ffi::c_void, u16, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Clear: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CloneFrom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNodeByID: unsafe extern "system" fn(*mut core::ffi::c_void, TOPOID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSourceNodeCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetOutputNodeCollection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMFTopology_Impl: super::IMFAttributes_Impl {
    fn GetTopologyID(&self) -> windows_core::Result<TOPOID>;
    fn AddNode(&self, pnode: windows_core::Ref<IMFTopologyNode>) -> windows_core::Result<()>;
    fn RemoveNode(&self, pnode: windows_core::Ref<IMFTopologyNode>) -> windows_core::Result<()>;
    fn GetNodeCount(&self) -> windows_core::Result<u16>;
    fn GetNode(&self, windex: u16) -> windows_core::Result<IMFTopologyNode>;
    fn Clear(&self) -> windows_core::Result<()>;
    fn CloneFrom(&self, ptopology: windows_core::Ref<IMFTopology>) -> windows_core::Result<()>;
    fn GetNodeByID(&self, qwtoponodeid: TOPOID) -> windows_core::Result<IMFTopologyNode>;
    fn GetSourceNodeCollection(&self) -> windows_core::Result<super::IMFCollection>;
    fn GetOutputNodeCollection(&self) -> windows_core::Result<super::IMFCollection>;
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl IMFTopology_Vtbl {
    pub const fn new<Identity: IMFTopology_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetTopologyID<Identity: IMFTopology_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pid: *mut TOPOID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTopology_Impl::GetTopologyID(this) {
                    Ok(ok__) => {
                        pid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddNode<Identity: IMFTopology_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnode: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTopology_Impl::AddNode(this, core::mem::transmute_copy(&pnode)).into()
            }
        }
        unsafe extern "system" fn RemoveNode<Identity: IMFTopology_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnode: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTopology_Impl::RemoveNode(this, core::mem::transmute_copy(&pnode)).into()
            }
        }
        unsafe extern "system" fn GetNodeCount<Identity: IMFTopology_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwnodes: *mut u16) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTopology_Impl::GetNodeCount(this) {
                    Ok(ok__) => {
                        pwnodes.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNode<Identity: IMFTopology_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, windex: u16, ppnode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTopology_Impl::GetNode(this, core::mem::transmute_copy(&windex)) {
                    Ok(ok__) => {
                        ppnode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Clear<Identity: IMFTopology_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTopology_Impl::Clear(this).into()
            }
        }
        unsafe extern "system" fn CloneFrom<Identity: IMFTopology_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptopology: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTopology_Impl::CloneFrom(this, core::mem::transmute_copy(&ptopology)).into()
            }
        }
        unsafe extern "system" fn GetNodeByID<Identity: IMFTopology_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, qwtoponodeid: TOPOID, ppnode: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTopology_Impl::GetNodeByID(this, core::mem::transmute_copy(&qwtoponodeid)) {
                    Ok(ok__) => {
                        ppnode.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSourceNodeCollection<Identity: IMFTopology_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTopology_Impl::GetSourceNodeCollection(this) {
                    Ok(ok__) => {
                        ppcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOutputNodeCollection<Identity: IMFTopology_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppcollection: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTopology_Impl::GetOutputNodeCollection(this) {
                    Ok(ok__) => {
                        ppcollection.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IMFAttributes_Vtbl::new::<Identity, OFFSET>(),
            GetTopologyID: GetTopologyID::<Identity, OFFSET>,
            AddNode: AddNode::<Identity, OFFSET>,
            RemoveNode: RemoveNode::<Identity, OFFSET>,
            GetNodeCount: GetNodeCount::<Identity, OFFSET>,
            GetNode: GetNode::<Identity, OFFSET>,
            Clear: Clear::<Identity, OFFSET>,
            CloneFrom: CloneFrom::<Identity, OFFSET>,
            GetNodeByID: GetNodeByID::<Identity, OFFSET>,
            GetSourceNodeCollection: GetSourceNodeCollection::<Identity, OFFSET>,
            GetOutputNodeCollection: GetOutputNodeCollection::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFTopology as windows_core::Interface>::IID || iid == &<super::IMFAttributes as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMFTopology {}
#[cfg(feature = "mfobjects")]
windows_core::imp::define_interface!(IMFTopologyNode, IMFTopologyNode_Vtbl, 0x83cf873a_f6da_4bc8_823f_bacfd55dc430);
#[cfg(feature = "mfobjects")]
impl core::ops::Deref for IMFTopologyNode {
    type Target = super::IMFAttributes;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "mfobjects")]
windows_core::imp::interface_hierarchy!(IMFTopologyNode, windows_core::IUnknown, super::IMFAttributes);
#[cfg(feature = "mfobjects")]
impl IMFTopologyNode {
    pub unsafe fn SetObject<P0>(&self, pobject: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetObject)(windows_core::Interface::as_raw(self), pobject.param().abi()) }
    }
    pub unsafe fn GetObject(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetObject)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetNodeType(&self) -> windows_core::Result<MF_TOPOLOGY_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNodeType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetTopoNodeID(&self) -> windows_core::Result<TOPOID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTopoNodeID)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetTopoNodeID(&self, ulltopoid: TOPOID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTopoNodeID)(windows_core::Interface::as_raw(self), ulltopoid) }
    }
    pub unsafe fn GetInputCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInputCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetOutputCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ConnectOutput<P1>(&self, dwoutputindex: u32, pdownstreamnode: P1, dwinputindexondownstreamnode: u32) -> windows_core::HRESULT
    where
        P1: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).ConnectOutput)(windows_core::Interface::as_raw(self), dwoutputindex, pdownstreamnode.param().abi(), dwinputindexondownstreamnode) }
    }
    pub unsafe fn DisconnectOutput(&self, dwoutputindex: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DisconnectOutput)(windows_core::Interface::as_raw(self), dwoutputindex) }
    }
    pub unsafe fn GetInput(&self, dwinputindex: u32, ppupstreamnode: *mut Option<Self>, pdwoutputindexonupstreamnode: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetInput)(windows_core::Interface::as_raw(self), dwinputindex, core::mem::transmute(ppupstreamnode), pdwoutputindexonupstreamnode as _) }
    }
    pub unsafe fn GetOutput(&self, dwoutputindex: u32, ppdownstreamnode: *mut Option<Self>, pdwinputindexondownstreamnode: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetOutput)(windows_core::Interface::as_raw(self), dwoutputindex, core::mem::transmute(ppdownstreamnode), pdwinputindexondownstreamnode as _) }
    }
    pub unsafe fn SetOutputPrefType<P1>(&self, dwoutputindex: u32, ptype: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IMFMediaType>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetOutputPrefType)(windows_core::Interface::as_raw(self), dwoutputindex, ptype.param().abi()) }
    }
    pub unsafe fn GetOutputPrefType(&self, dwoutputindex: u32) -> windows_core::Result<super::IMFMediaType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputPrefType)(windows_core::Interface::as_raw(self), dwoutputindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetInputPrefType<P1>(&self, dwinputindex: u32, ptype: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IMFMediaType>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetInputPrefType)(windows_core::Interface::as_raw(self), dwinputindex, ptype.param().abi()) }
    }
    pub unsafe fn GetInputPrefType(&self, dwinputindex: u32) -> windows_core::Result<super::IMFMediaType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInputPrefType)(windows_core::Interface::as_raw(self), dwinputindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CloneFrom<P0>(&self, pnode: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).CloneFrom)(windows_core::Interface::as_raw(self), pnode.param().abi()) }
    }
}
#[cfg(feature = "mfobjects")]
#[repr(C)]
#[doc(hidden)]
pub struct IMFTopologyNode_Vtbl {
    pub base__: super::IMFAttributes_Vtbl,
    pub SetObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetObject: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNodeType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MF_TOPOLOGY_TYPE) -> windows_core::HRESULT,
    pub GetTopoNodeID: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TOPOID) -> windows_core::HRESULT,
    pub SetTopoNodeID: unsafe extern "system" fn(*mut core::ffi::c_void, TOPOID) -> windows_core::HRESULT,
    pub GetInputCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetOutputCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ConnectOutput: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub DisconnectOutput: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub GetInput: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetOutput: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetOutputPrefType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetOutputPrefType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetInputPrefType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetInputPrefType: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CloneFrom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMFTopologyNode_Impl: super::IMFAttributes_Impl {
    fn SetObject(&self, pobject: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetObject(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn GetNodeType(&self) -> windows_core::Result<MF_TOPOLOGY_TYPE>;
    fn GetTopoNodeID(&self) -> windows_core::Result<TOPOID>;
    fn SetTopoNodeID(&self, ulltopoid: TOPOID) -> windows_core::Result<()>;
    fn GetInputCount(&self) -> windows_core::Result<u32>;
    fn GetOutputCount(&self) -> windows_core::Result<u32>;
    fn ConnectOutput(&self, dwoutputindex: u32, pdownstreamnode: windows_core::Ref<IMFTopologyNode>, dwinputindexondownstreamnode: u32) -> windows_core::Result<()>;
    fn DisconnectOutput(&self, dwoutputindex: u32) -> windows_core::Result<()>;
    fn GetInput(&self, dwinputindex: u32, ppupstreamnode: windows_core::OutRef<IMFTopologyNode>, pdwoutputindexonupstreamnode: *mut u32) -> windows_core::Result<()>;
    fn GetOutput(&self, dwoutputindex: u32, ppdownstreamnode: windows_core::OutRef<IMFTopologyNode>, pdwinputindexondownstreamnode: *mut u32) -> windows_core::Result<()>;
    fn SetOutputPrefType(&self, dwoutputindex: u32, ptype: windows_core::Ref<super::IMFMediaType>) -> windows_core::Result<()>;
    fn GetOutputPrefType(&self, dwoutputindex: u32) -> windows_core::Result<super::IMFMediaType>;
    fn SetInputPrefType(&self, dwinputindex: u32, ptype: windows_core::Ref<super::IMFMediaType>) -> windows_core::Result<()>;
    fn GetInputPrefType(&self, dwinputindex: u32) -> windows_core::Result<super::IMFMediaType>;
    fn CloneFrom(&self, pnode: windows_core::Ref<IMFTopologyNode>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl IMFTopologyNode_Vtbl {
    pub const fn new<Identity: IMFTopologyNode_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetObject<Identity: IMFTopologyNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pobject: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTopologyNode_Impl::SetObject(this, core::mem::transmute_copy(&pobject)).into()
            }
        }
        unsafe extern "system" fn GetObject<Identity: IMFTopologyNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTopologyNode_Impl::GetObject(this) {
                    Ok(ok__) => {
                        ppobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNodeType<Identity: IMFTopologyNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptype: *mut MF_TOPOLOGY_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTopologyNode_Impl::GetNodeType(this) {
                    Ok(ok__) => {
                        ptype.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTopoNodeID<Identity: IMFTopologyNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pid: *mut TOPOID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTopologyNode_Impl::GetTopoNodeID(this) {
                    Ok(ok__) => {
                        pid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTopoNodeID<Identity: IMFTopologyNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ulltopoid: TOPOID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTopologyNode_Impl::SetTopoNodeID(this, core::mem::transmute_copy(&ulltopoid)).into()
            }
        }
        unsafe extern "system" fn GetInputCount<Identity: IMFTopologyNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcinputs: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTopologyNode_Impl::GetInputCount(this) {
                    Ok(ok__) => {
                        pcinputs.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOutputCount<Identity: IMFTopologyNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcoutputs: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTopologyNode_Impl::GetOutputCount(this) {
                    Ok(ok__) => {
                        pcoutputs.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ConnectOutput<Identity: IMFTopologyNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputindex: u32, pdownstreamnode: *mut core::ffi::c_void, dwinputindexondownstreamnode: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTopologyNode_Impl::ConnectOutput(this, core::mem::transmute_copy(&dwoutputindex), core::mem::transmute_copy(&pdownstreamnode), core::mem::transmute_copy(&dwinputindexondownstreamnode)).into()
            }
        }
        unsafe extern "system" fn DisconnectOutput<Identity: IMFTopologyNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTopologyNode_Impl::DisconnectOutput(this, core::mem::transmute_copy(&dwoutputindex)).into()
            }
        }
        unsafe extern "system" fn GetInput<Identity: IMFTopologyNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputindex: u32, ppupstreamnode: *mut *mut core::ffi::c_void, pdwoutputindexonupstreamnode: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTopologyNode_Impl::GetInput(this, core::mem::transmute_copy(&dwinputindex), core::mem::transmute_copy(&ppupstreamnode), core::mem::transmute_copy(&pdwoutputindexonupstreamnode)).into()
            }
        }
        unsafe extern "system" fn GetOutput<Identity: IMFTopologyNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputindex: u32, ppdownstreamnode: *mut *mut core::ffi::c_void, pdwinputindexondownstreamnode: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTopologyNode_Impl::GetOutput(this, core::mem::transmute_copy(&dwoutputindex), core::mem::transmute_copy(&ppdownstreamnode), core::mem::transmute_copy(&pdwinputindexondownstreamnode)).into()
            }
        }
        unsafe extern "system" fn SetOutputPrefType<Identity: IMFTopologyNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputindex: u32, ptype: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTopologyNode_Impl::SetOutputPrefType(this, core::mem::transmute_copy(&dwoutputindex), core::mem::transmute_copy(&ptype)).into()
            }
        }
        unsafe extern "system" fn GetOutputPrefType<Identity: IMFTopologyNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwoutputindex: u32, pptype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTopologyNode_Impl::GetOutputPrefType(this, core::mem::transmute_copy(&dwoutputindex)) {
                    Ok(ok__) => {
                        pptype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetInputPrefType<Identity: IMFTopologyNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputindex: u32, ptype: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTopologyNode_Impl::SetInputPrefType(this, core::mem::transmute_copy(&dwinputindex), core::mem::transmute_copy(&ptype)).into()
            }
        }
        unsafe extern "system" fn GetInputPrefType<Identity: IMFTopologyNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwinputindex: u32, pptype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTopologyNode_Impl::GetInputPrefType(this, core::mem::transmute_copy(&dwinputindex)) {
                    Ok(ok__) => {
                        pptype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CloneFrom<Identity: IMFTopologyNode_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnode: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTopologyNode_Impl::CloneFrom(this, core::mem::transmute_copy(&pnode)).into()
            }
        }
        Self {
            base__: super::IMFAttributes_Vtbl::new::<Identity, OFFSET>(),
            SetObject: SetObject::<Identity, OFFSET>,
            GetObject: GetObject::<Identity, OFFSET>,
            GetNodeType: GetNodeType::<Identity, OFFSET>,
            GetTopoNodeID: GetTopoNodeID::<Identity, OFFSET>,
            SetTopoNodeID: SetTopoNodeID::<Identity, OFFSET>,
            GetInputCount: GetInputCount::<Identity, OFFSET>,
            GetOutputCount: GetOutputCount::<Identity, OFFSET>,
            ConnectOutput: ConnectOutput::<Identity, OFFSET>,
            DisconnectOutput: DisconnectOutput::<Identity, OFFSET>,
            GetInput: GetInput::<Identity, OFFSET>,
            GetOutput: GetOutput::<Identity, OFFSET>,
            SetOutputPrefType: SetOutputPrefType::<Identity, OFFSET>,
            GetOutputPrefType: GetOutputPrefType::<Identity, OFFSET>,
            SetInputPrefType: SetInputPrefType::<Identity, OFFSET>,
            GetInputPrefType: GetInputPrefType::<Identity, OFFSET>,
            CloneFrom: CloneFrom::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFTopologyNode as windows_core::Interface>::IID || iid == &<super::IMFAttributes as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mfobjects", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMFTopologyNode {}
windows_core::imp::define_interface!(IMFTopologyNodeAttributeEditor, IMFTopologyNodeAttributeEditor_Vtbl, 0x676aa6dd_238a_410d_bb99_65668d01605a);
windows_core::imp::interface_hierarchy!(IMFTopologyNodeAttributeEditor, windows_core::IUnknown);
impl IMFTopologyNodeAttributeEditor {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn UpdateNodeAttributes(&self, topoid: TOPOID, cupdates: u32, pupdates: *const MFTOPONODE_ATTRIBUTE_UPDATE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UpdateNodeAttributes)(windows_core::Interface::as_raw(self), topoid, cupdates, pupdates) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFTopologyNodeAttributeEditor_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "mfobjects")]
    pub UpdateNodeAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, TOPOID, u32, *const MFTOPONODE_ATTRIBUTE_UPDATE) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    UpdateNodeAttributes: usize,
}
#[cfg(feature = "mfobjects")]
pub trait IMFTopologyNodeAttributeEditor_Impl: windows_core::IUnknownImpl {
    fn UpdateNodeAttributes(&self, topoid: TOPOID, cupdates: u32, pupdates: *const MFTOPONODE_ATTRIBUTE_UPDATE) -> windows_core::Result<()>;
}
#[cfg(feature = "mfobjects")]
impl IMFTopologyNodeAttributeEditor_Vtbl {
    pub const fn new<Identity: IMFTopologyNodeAttributeEditor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn UpdateNodeAttributes<Identity: IMFTopologyNodeAttributeEditor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, topoid: TOPOID, cupdates: u32, pupdates: *const MFTOPONODE_ATTRIBUTE_UPDATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTopologyNodeAttributeEditor_Impl::UpdateNodeAttributes(this, core::mem::transmute_copy(&topoid), core::mem::transmute_copy(&cupdates), core::mem::transmute_copy(&pupdates)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), UpdateNodeAttributes: UpdateNodeAttributes::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFTopologyNodeAttributeEditor as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFTopologyNodeAttributeEditor {}
windows_core::imp::define_interface!(IMFTrackedSample, IMFTrackedSample_Vtbl, 0x245bf8e9_0755_40f7_88a5_ae0f18d55e17);
windows_core::imp::interface_hierarchy!(IMFTrackedSample, windows_core::IUnknown);
impl IMFTrackedSample {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn SetAllocator<P0, P1>(&self, psampleallocator: P0, punkstate: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMFAsyncCallback>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAllocator)(windows_core::Interface::as_raw(self), psampleallocator.param().abi(), punkstate.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFTrackedSample_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "mfobjects")]
    pub SetAllocator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    SetAllocator: usize,
}
#[cfg(feature = "mfobjects")]
pub trait IMFTrackedSample_Impl: windows_core::IUnknownImpl {
    fn SetAllocator(&self, psampleallocator: windows_core::Ref<super::IMFAsyncCallback>, punkstate: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(feature = "mfobjects")]
impl IMFTrackedSample_Vtbl {
    pub const fn new<Identity: IMFTrackedSample_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetAllocator<Identity: IMFTrackedSample_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psampleallocator: *mut core::ffi::c_void, punkstate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTrackedSample_Impl::SetAllocator(this, core::mem::transmute_copy(&psampleallocator), core::mem::transmute_copy(&punkstate)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), SetAllocator: SetAllocator::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFTrackedSample as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFTrackedSample {}
windows_core::imp::define_interface!(IMFTranscodeProfile, IMFTranscodeProfile_Vtbl, 0x4adfdba3_7ab0_4953_a62b_461e7ff3da1e);
windows_core::imp::interface_hierarchy!(IMFTranscodeProfile, windows_core::IUnknown);
impl IMFTranscodeProfile {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn SetAudioAttributes<P0>(&self, pattrs: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMFAttributes>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetAudioAttributes)(windows_core::Interface::as_raw(self), pattrs.param().abi()) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn GetAudioAttributes(&self) -> windows_core::Result<super::IMFAttributes> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAudioAttributes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn SetVideoAttributes<P0>(&self, pattrs: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMFAttributes>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetVideoAttributes)(windows_core::Interface::as_raw(self), pattrs.param().abi()) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn GetVideoAttributes(&self) -> windows_core::Result<super::IMFAttributes> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVideoAttributes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn SetContainerAttributes<P0>(&self, pattrs: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMFAttributes>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetContainerAttributes)(windows_core::Interface::as_raw(self), pattrs.param().abi()) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn GetContainerAttributes(&self) -> windows_core::Result<super::IMFAttributes> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetContainerAttributes)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFTranscodeProfile_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "mfobjects")]
    pub SetAudioAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    SetAudioAttributes: usize,
    #[cfg(feature = "mfobjects")]
    pub GetAudioAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    GetAudioAttributes: usize,
    #[cfg(feature = "mfobjects")]
    pub SetVideoAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    SetVideoAttributes: usize,
    #[cfg(feature = "mfobjects")]
    pub GetVideoAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    GetVideoAttributes: usize,
    #[cfg(feature = "mfobjects")]
    pub SetContainerAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    SetContainerAttributes: usize,
    #[cfg(feature = "mfobjects")]
    pub GetContainerAttributes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    GetContainerAttributes: usize,
}
#[cfg(feature = "mfobjects")]
pub trait IMFTranscodeProfile_Impl: windows_core::IUnknownImpl {
    fn SetAudioAttributes(&self, pattrs: windows_core::Ref<super::IMFAttributes>) -> windows_core::Result<()>;
    fn GetAudioAttributes(&self) -> windows_core::Result<super::IMFAttributes>;
    fn SetVideoAttributes(&self, pattrs: windows_core::Ref<super::IMFAttributes>) -> windows_core::Result<()>;
    fn GetVideoAttributes(&self) -> windows_core::Result<super::IMFAttributes>;
    fn SetContainerAttributes(&self, pattrs: windows_core::Ref<super::IMFAttributes>) -> windows_core::Result<()>;
    fn GetContainerAttributes(&self) -> windows_core::Result<super::IMFAttributes>;
}
#[cfg(feature = "mfobjects")]
impl IMFTranscodeProfile_Vtbl {
    pub const fn new<Identity: IMFTranscodeProfile_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetAudioAttributes<Identity: IMFTranscodeProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattrs: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTranscodeProfile_Impl::SetAudioAttributes(this, core::mem::transmute_copy(&pattrs)).into()
            }
        }
        unsafe extern "system" fn GetAudioAttributes<Identity: IMFTranscodeProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppattrs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTranscodeProfile_Impl::GetAudioAttributes(this) {
                    Ok(ok__) => {
                        ppattrs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetVideoAttributes<Identity: IMFTranscodeProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattrs: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTranscodeProfile_Impl::SetVideoAttributes(this, core::mem::transmute_copy(&pattrs)).into()
            }
        }
        unsafe extern "system" fn GetVideoAttributes<Identity: IMFTranscodeProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppattrs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTranscodeProfile_Impl::GetVideoAttributes(this) {
                    Ok(ok__) => {
                        ppattrs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetContainerAttributes<Identity: IMFTranscodeProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattrs: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTranscodeProfile_Impl::SetContainerAttributes(this, core::mem::transmute_copy(&pattrs)).into()
            }
        }
        unsafe extern "system" fn GetContainerAttributes<Identity: IMFTranscodeProfile_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppattrs: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTranscodeProfile_Impl::GetContainerAttributes(this) {
                    Ok(ok__) => {
                        ppattrs.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetAudioAttributes: SetAudioAttributes::<Identity, OFFSET>,
            GetAudioAttributes: GetAudioAttributes::<Identity, OFFSET>,
            SetVideoAttributes: SetVideoAttributes::<Identity, OFFSET>,
            GetVideoAttributes: GetVideoAttributes::<Identity, OFFSET>,
            SetContainerAttributes: SetContainerAttributes::<Identity, OFFSET>,
            GetContainerAttributes: GetContainerAttributes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFTranscodeProfile as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFTranscodeProfile {}
windows_core::imp::define_interface!(IMFTranscodeSinkInfoProvider, IMFTranscodeSinkInfoProvider_Vtbl, 0x8cffcd2e_5a03_4a3a_aff7_edcd107c620e);
windows_core::imp::interface_hierarchy!(IMFTranscodeSinkInfoProvider, windows_core::IUnknown);
impl IMFTranscodeSinkInfoProvider {
    pub unsafe fn SetOutputFile<P0>(&self, pwszfilename: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetOutputFile)(windows_core::Interface::as_raw(self), pwszfilename.param().abi()) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn SetOutputByteStream<P0>(&self, pbytestreamactivate: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMFActivate>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetOutputByteStream)(windows_core::Interface::as_raw(self), pbytestreamactivate.param().abi()) }
    }
    pub unsafe fn SetProfile<P0>(&self, pprofile: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFTranscodeProfile>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetProfile)(windows_core::Interface::as_raw(self), pprofile.param().abi()) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn GetSinkInfo(&self) -> windows_core::Result<MF_TRANSCODE_SINK_INFO> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSinkInfo)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFTranscodeSinkInfoProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetOutputFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "mfobjects")]
    pub SetOutputByteStream: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    SetOutputByteStream: usize,
    pub SetProfile: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "mfobjects")]
    pub GetSinkInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut MF_TRANSCODE_SINK_INFO) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    GetSinkInfo: usize,
}
#[cfg(feature = "mfobjects")]
pub trait IMFTranscodeSinkInfoProvider_Impl: windows_core::IUnknownImpl {
    fn SetOutputFile(&self, pwszfilename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetOutputByteStream(&self, pbytestreamactivate: windows_core::Ref<super::IMFActivate>) -> windows_core::Result<()>;
    fn SetProfile(&self, pprofile: windows_core::Ref<IMFTranscodeProfile>) -> windows_core::Result<()>;
    fn GetSinkInfo(&self) -> windows_core::Result<MF_TRANSCODE_SINK_INFO>;
}
#[cfg(feature = "mfobjects")]
impl IMFTranscodeSinkInfoProvider_Vtbl {
    pub const fn new<Identity: IMFTranscodeSinkInfoProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetOutputFile<Identity: IMFTranscodeSinkInfoProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwszfilename: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTranscodeSinkInfoProvider_Impl::SetOutputFile(this, core::mem::transmute(&pwszfilename)).into()
            }
        }
        unsafe extern "system" fn SetOutputByteStream<Identity: IMFTranscodeSinkInfoProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbytestreamactivate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTranscodeSinkInfoProvider_Impl::SetOutputByteStream(this, core::mem::transmute_copy(&pbytestreamactivate)).into()
            }
        }
        unsafe extern "system" fn SetProfile<Identity: IMFTranscodeSinkInfoProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprofile: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTranscodeSinkInfoProvider_Impl::SetProfile(this, core::mem::transmute_copy(&pprofile)).into()
            }
        }
        unsafe extern "system" fn GetSinkInfo<Identity: IMFTranscodeSinkInfoProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psinkinfo: *mut MF_TRANSCODE_SINK_INFO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTranscodeSinkInfoProvider_Impl::GetSinkInfo(this) {
                    Ok(ok__) => {
                        psinkinfo.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetOutputFile: SetOutputFile::<Identity, OFFSET>,
            SetOutputByteStream: SetOutputByteStream::<Identity, OFFSET>,
            SetProfile: SetProfile::<Identity, OFFSET>,
            GetSinkInfo: GetSinkInfo::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFTranscodeSinkInfoProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFTranscodeSinkInfoProvider {}
windows_core::imp::define_interface!(IMFTrustedInput, IMFTrustedInput_Vtbl, 0x542612c4_a1b8_4632_b521_de11ea64a0b0);
windows_core::imp::interface_hierarchy!(IMFTrustedInput, windows_core::IUnknown);
impl IMFTrustedInput {
    pub unsafe fn GetInputTrustAuthority<T>(&self, dwstreamid: u32) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetInputTrustAuthority)(windows_core::Interface::as_raw(self), dwstreamid, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFTrustedInput_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetInputTrustAuthority: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFTrustedInput_Impl: windows_core::IUnknownImpl {
    fn GetInputTrustAuthority(&self, dwstreamid: u32, riid: *const windows_core::GUID, ppunkobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
}
impl IMFTrustedInput_Vtbl {
    pub const fn new<Identity: IMFTrustedInput_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetInputTrustAuthority<Identity: IMFTrustedInput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwstreamid: u32, riid: *const windows_core::GUID, ppunkobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFTrustedInput_Impl::GetInputTrustAuthority(this, core::mem::transmute_copy(&dwstreamid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&ppunkobject)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetInputTrustAuthority: GetInputTrustAuthority::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFTrustedInput as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFTrustedInput {}
windows_core::imp::define_interface!(IMFTrustedOutput, IMFTrustedOutput_Vtbl, 0xd19f8e95_b126_4446_890c_5dcb7ad71453);
windows_core::imp::interface_hierarchy!(IMFTrustedOutput, windows_core::IUnknown);
impl IMFTrustedOutput {
    pub unsafe fn GetOutputTrustAuthorityCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputTrustAuthorityCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetOutputTrustAuthorityByIndex(&self, dwindex: u32) -> windows_core::Result<IMFOutputTrustAuthority> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOutputTrustAuthorityByIndex)(windows_core::Interface::as_raw(self), dwindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn IsFinal(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsFinal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFTrustedOutput_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetOutputTrustAuthorityCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetOutputTrustAuthorityByIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsFinal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IMFTrustedOutput_Impl: windows_core::IUnknownImpl {
    fn GetOutputTrustAuthorityCount(&self) -> windows_core::Result<u32>;
    fn GetOutputTrustAuthorityByIndex(&self, dwindex: u32) -> windows_core::Result<IMFOutputTrustAuthority>;
    fn IsFinal(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IMFTrustedOutput_Vtbl {
    pub const fn new<Identity: IMFTrustedOutput_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetOutputTrustAuthorityCount<Identity: IMFTrustedOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcoutputtrustauthorities: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTrustedOutput_Impl::GetOutputTrustAuthorityCount(this) {
                    Ok(ok__) => {
                        pcoutputtrustauthorities.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetOutputTrustAuthorityByIndex<Identity: IMFTrustedOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwindex: u32, ppauthority: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTrustedOutput_Impl::GetOutputTrustAuthorityByIndex(this, core::mem::transmute_copy(&dwindex)) {
                    Ok(ok__) => {
                        ppauthority.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsFinal<Identity: IMFTrustedOutput_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pfisfinal: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFTrustedOutput_Impl::IsFinal(this) {
                    Ok(ok__) => {
                        pfisfinal.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetOutputTrustAuthorityCount: GetOutputTrustAuthorityCount::<Identity, OFFSET>,
            GetOutputTrustAuthorityByIndex: GetOutputTrustAuthorityByIndex::<Identity, OFFSET>,
            IsFinal: IsFinal::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFTrustedOutput as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFTrustedOutput {}
windows_core::imp::define_interface!(IMFVideoCaptureSampleAllocator, IMFVideoCaptureSampleAllocator_Vtbl, 0x725b77c7_ca9f_4fe5_9d72_9946bf9b3c70);
impl core::ops::Deref for IMFVideoCaptureSampleAllocator {
    type Target = IMFVideoSampleAllocator;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMFVideoCaptureSampleAllocator, windows_core::IUnknown, IMFVideoSampleAllocator);
impl IMFVideoCaptureSampleAllocator {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn InitializeCaptureSampleAllocator<P4, P5>(&self, cbsamplesize: u32, cbcapturemetadatasize: u32, cbalignment: u32, cminimumsamples: u32, pattributes: P4, pmediatype: P5) -> windows_core::HRESULT
    where
        P4: windows_core::Param<super::IMFAttributes>,
        P5: windows_core::Param<super::IMFMediaType>,
    {
        unsafe { (windows_core::Interface::vtable(self).InitializeCaptureSampleAllocator)(windows_core::Interface::as_raw(self), cbsamplesize, cbcapturemetadatasize, cbalignment, cminimumsamples, pattributes.param().abi(), pmediatype.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFVideoCaptureSampleAllocator_Vtbl {
    pub base__: IMFVideoSampleAllocator_Vtbl,
    #[cfg(feature = "mfobjects")]
    pub InitializeCaptureSampleAllocator: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, u32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    InitializeCaptureSampleAllocator: usize,
}
#[cfg(feature = "mfobjects")]
pub trait IMFVideoCaptureSampleAllocator_Impl: IMFVideoSampleAllocator_Impl {
    fn InitializeCaptureSampleAllocator(&self, cbsamplesize: u32, cbcapturemetadatasize: u32, cbalignment: u32, cminimumsamples: u32, pattributes: windows_core::Ref<super::IMFAttributes>, pmediatype: windows_core::Ref<super::IMFMediaType>) -> windows_core::Result<()>;
}
#[cfg(feature = "mfobjects")]
impl IMFVideoCaptureSampleAllocator_Vtbl {
    pub const fn new<Identity: IMFVideoCaptureSampleAllocator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InitializeCaptureSampleAllocator<Identity: IMFVideoCaptureSampleAllocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cbsamplesize: u32, cbcapturemetadatasize: u32, cbalignment: u32, cminimumsamples: u32, pattributes: *mut core::ffi::c_void, pmediatype: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFVideoCaptureSampleAllocator_Impl::InitializeCaptureSampleAllocator(this, core::mem::transmute_copy(&cbsamplesize), core::mem::transmute_copy(&cbcapturemetadatasize), core::mem::transmute_copy(&cbalignment), core::mem::transmute_copy(&cminimumsamples), core::mem::transmute_copy(&pattributes), core::mem::transmute_copy(&pmediatype)).into()
            }
        }
        Self {
            base__: IMFVideoSampleAllocator_Vtbl::new::<Identity, OFFSET>(),
            InitializeCaptureSampleAllocator: InitializeCaptureSampleAllocator::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFVideoCaptureSampleAllocator as windows_core::Interface>::IID || iid == &<IMFVideoSampleAllocator as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFVideoCaptureSampleAllocator {}
windows_core::imp::define_interface!(IMFVideoProcessorControl, IMFVideoProcessorControl_Vtbl, 0xa3f675d5_6119_4f7f_a100_1d8b280f0efb);
windows_core::imp::interface_hierarchy!(IMFVideoProcessorControl, windows_core::IUnknown);
impl IMFVideoProcessorControl {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn SetBorderColor(&self, pbordercolor: Option<*const super::MFARGB>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBorderColor)(windows_core::Interface::as_raw(self), pbordercolor.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn SetSourceRectangle(&self, psrcrect: Option<*const super::RECT>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSourceRectangle)(windows_core::Interface::as_raw(self), psrcrect.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn SetDestinationRectangle(&self, pdstrect: Option<*const super::RECT>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDestinationRectangle)(windows_core::Interface::as_raw(self), pdstrect.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn SetMirror(&self, emirror: MF_VIDEO_PROCESSOR_MIRROR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMirror)(windows_core::Interface::as_raw(self), emirror) }
    }
    pub unsafe fn SetRotation(&self, erotation: MF_VIDEO_PROCESSOR_ROTATION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRotation)(windows_core::Interface::as_raw(self), erotation) }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn SetConstrictionSize(&self, pconstrictionsize: Option<*const super::SIZE>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetConstrictionSize)(windows_core::Interface::as_raw(self), pconstrictionsize.unwrap_or(core::mem::zeroed()) as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFVideoProcessorControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "mfobjects")]
    pub SetBorderColor: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::MFARGB) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    SetBorderColor: usize,
    #[cfg(feature = "windef")]
    pub SetSourceRectangle: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SetSourceRectangle: usize,
    #[cfg(feature = "windef")]
    pub SetDestinationRectangle: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SetDestinationRectangle: usize,
    pub SetMirror: unsafe extern "system" fn(*mut core::ffi::c_void, MF_VIDEO_PROCESSOR_MIRROR) -> windows_core::HRESULT,
    pub SetRotation: unsafe extern "system" fn(*mut core::ffi::c_void, MF_VIDEO_PROCESSOR_ROTATION) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub SetConstrictionSize: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SIZE) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    SetConstrictionSize: usize,
}
#[cfg(all(feature = "mfobjects", feature = "windef"))]
pub trait IMFVideoProcessorControl_Impl: windows_core::IUnknownImpl {
    fn SetBorderColor(&self, pbordercolor: *const super::MFARGB) -> windows_core::Result<()>;
    fn SetSourceRectangle(&self, psrcrect: *const super::RECT) -> windows_core::Result<()>;
    fn SetDestinationRectangle(&self, pdstrect: *const super::RECT) -> windows_core::Result<()>;
    fn SetMirror(&self, emirror: MF_VIDEO_PROCESSOR_MIRROR) -> windows_core::Result<()>;
    fn SetRotation(&self, erotation: MF_VIDEO_PROCESSOR_ROTATION) -> windows_core::Result<()>;
    fn SetConstrictionSize(&self, pconstrictionsize: *const super::SIZE) -> windows_core::Result<()>;
}
#[cfg(all(feature = "mfobjects", feature = "windef"))]
impl IMFVideoProcessorControl_Vtbl {
    pub const fn new<Identity: IMFVideoProcessorControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetBorderColor<Identity: IMFVideoProcessorControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbordercolor: *const super::MFARGB) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFVideoProcessorControl_Impl::SetBorderColor(this, core::mem::transmute_copy(&pbordercolor)).into()
            }
        }
        unsafe extern "system" fn SetSourceRectangle<Identity: IMFVideoProcessorControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, psrcrect: *const super::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFVideoProcessorControl_Impl::SetSourceRectangle(this, core::mem::transmute_copy(&psrcrect)).into()
            }
        }
        unsafe extern "system" fn SetDestinationRectangle<Identity: IMFVideoProcessorControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdstrect: *const super::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFVideoProcessorControl_Impl::SetDestinationRectangle(this, core::mem::transmute_copy(&pdstrect)).into()
            }
        }
        unsafe extern "system" fn SetMirror<Identity: IMFVideoProcessorControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, emirror: MF_VIDEO_PROCESSOR_MIRROR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFVideoProcessorControl_Impl::SetMirror(this, core::mem::transmute_copy(&emirror)).into()
            }
        }
        unsafe extern "system" fn SetRotation<Identity: IMFVideoProcessorControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, erotation: MF_VIDEO_PROCESSOR_ROTATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFVideoProcessorControl_Impl::SetRotation(this, core::mem::transmute_copy(&erotation)).into()
            }
        }
        unsafe extern "system" fn SetConstrictionSize<Identity: IMFVideoProcessorControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pconstrictionsize: *const super::SIZE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFVideoProcessorControl_Impl::SetConstrictionSize(this, core::mem::transmute_copy(&pconstrictionsize)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetBorderColor: SetBorderColor::<Identity, OFFSET>,
            SetSourceRectangle: SetSourceRectangle::<Identity, OFFSET>,
            SetDestinationRectangle: SetDestinationRectangle::<Identity, OFFSET>,
            SetMirror: SetMirror::<Identity, OFFSET>,
            SetRotation: SetRotation::<Identity, OFFSET>,
            SetConstrictionSize: SetConstrictionSize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFVideoProcessorControl as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mfobjects", feature = "windef"))]
impl windows_core::RuntimeName for IMFVideoProcessorControl {}
windows_core::imp::define_interface!(IMFVideoProcessorControl2, IMFVideoProcessorControl2_Vtbl, 0xbde633d3_e1dc_4a7f_a693_bbae399c4a20);
impl core::ops::Deref for IMFVideoProcessorControl2 {
    type Target = IMFVideoProcessorControl;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMFVideoProcessorControl2, windows_core::IUnknown, IMFVideoProcessorControl);
impl IMFVideoProcessorControl2 {
    pub unsafe fn SetRotationOverride(&self, uirotation: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRotationOverride)(windows_core::Interface::as_raw(self), uirotation) }
    }
    pub unsafe fn EnableHardwareEffects(&self, fenabled: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnableHardwareEffects)(windows_core::Interface::as_raw(self), fenabled.into()) }
    }
    pub unsafe fn GetSupportedHardwareEffects(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSupportedHardwareEffects)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFVideoProcessorControl2_Vtbl {
    pub base__: IMFVideoProcessorControl_Vtbl,
    pub SetRotationOverride: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub EnableHardwareEffects: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetSupportedHardwareEffects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "mfobjects", feature = "windef"))]
pub trait IMFVideoProcessorControl2_Impl: IMFVideoProcessorControl_Impl {
    fn SetRotationOverride(&self, uirotation: u32) -> windows_core::Result<()>;
    fn EnableHardwareEffects(&self, fenabled: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetSupportedHardwareEffects(&self) -> windows_core::Result<u32>;
}
#[cfg(all(feature = "mfobjects", feature = "windef"))]
impl IMFVideoProcessorControl2_Vtbl {
    pub const fn new<Identity: IMFVideoProcessorControl2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetRotationOverride<Identity: IMFVideoProcessorControl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, uirotation: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFVideoProcessorControl2_Impl::SetRotationOverride(this, core::mem::transmute_copy(&uirotation)).into()
            }
        }
        unsafe extern "system" fn EnableHardwareEffects<Identity: IMFVideoProcessorControl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenabled: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFVideoProcessorControl2_Impl::EnableHardwareEffects(this, core::mem::transmute_copy(&fenabled)).into()
            }
        }
        unsafe extern "system" fn GetSupportedHardwareEffects<Identity: IMFVideoProcessorControl2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, puisupport: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFVideoProcessorControl2_Impl::GetSupportedHardwareEffects(this) {
                    Ok(ok__) => {
                        puisupport.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IMFVideoProcessorControl_Vtbl::new::<Identity, OFFSET>(),
            SetRotationOverride: SetRotationOverride::<Identity, OFFSET>,
            EnableHardwareEffects: EnableHardwareEffects::<Identity, OFFSET>,
            GetSupportedHardwareEffects: GetSupportedHardwareEffects::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFVideoProcessorControl2 as windows_core::Interface>::IID || iid == &<IMFVideoProcessorControl as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mfobjects", feature = "windef"))]
impl windows_core::RuntimeName for IMFVideoProcessorControl2 {}
windows_core::imp::define_interface!(IMFVideoProcessorControl3, IMFVideoProcessorControl3_Vtbl, 0x2424b3f2_eb23_40f1_91aa_74bddeea0883);
impl core::ops::Deref for IMFVideoProcessorControl3 {
    type Target = IMFVideoProcessorControl2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMFVideoProcessorControl3, windows_core::IUnknown, IMFVideoProcessorControl, IMFVideoProcessorControl2);
impl IMFVideoProcessorControl3 {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn GetNaturalOutputType(&self) -> windows_core::Result<super::IMFMediaType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNaturalOutputType)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn EnableSphericalVideoProcessing(&self, fenable: bool, eformat: MFVideoSphericalFormat, eprojectionmode: MFVideoSphericalProjectionMode) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnableSphericalVideoProcessing)(windows_core::Interface::as_raw(self), fenable.into(), eformat, eprojectionmode) }
    }
    pub unsafe fn SetSphericalVideoProperties(&self, x: f32, y: f32, z: f32, w: f32, fieldofview: f32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetSphericalVideoProperties)(windows_core::Interface::as_raw(self), x, y, z, w, fieldofview) }
    }
    pub unsafe fn SetOutputDevice<P0>(&self, poutputdevice: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetOutputDevice)(windows_core::Interface::as_raw(self), poutputdevice.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFVideoProcessorControl3_Vtbl {
    pub base__: IMFVideoProcessorControl2_Vtbl,
    #[cfg(feature = "mfobjects")]
    pub GetNaturalOutputType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    GetNaturalOutputType: usize,
    pub EnableSphericalVideoProcessing: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL, MFVideoSphericalFormat, MFVideoSphericalProjectionMode) -> windows_core::HRESULT,
    pub SetSphericalVideoProperties: unsafe extern "system" fn(*mut core::ffi::c_void, f32, f32, f32, f32, f32) -> windows_core::HRESULT,
    pub SetOutputDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "mfobjects", feature = "windef"))]
pub trait IMFVideoProcessorControl3_Impl: IMFVideoProcessorControl2_Impl {
    fn GetNaturalOutputType(&self) -> windows_core::Result<super::IMFMediaType>;
    fn EnableSphericalVideoProcessing(&self, fenable: windows_core::BOOL, eformat: MFVideoSphericalFormat, eprojectionmode: MFVideoSphericalProjectionMode) -> windows_core::Result<()>;
    fn SetSphericalVideoProperties(&self, x: f32, y: f32, z: f32, w: f32, fieldofview: f32) -> windows_core::Result<()>;
    fn SetOutputDevice(&self, poutputdevice: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "mfobjects", feature = "windef"))]
impl IMFVideoProcessorControl3_Vtbl {
    pub const fn new<Identity: IMFVideoProcessorControl3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetNaturalOutputType<Identity: IMFVideoProcessorControl3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pptype: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFVideoProcessorControl3_Impl::GetNaturalOutputType(this) {
                    Ok(ok__) => {
                        pptype.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EnableSphericalVideoProcessing<Identity: IMFVideoProcessorControl3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, fenable: windows_core::BOOL, eformat: MFVideoSphericalFormat, eprojectionmode: MFVideoSphericalProjectionMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFVideoProcessorControl3_Impl::EnableSphericalVideoProcessing(this, core::mem::transmute_copy(&fenable), core::mem::transmute_copy(&eformat), core::mem::transmute_copy(&eprojectionmode)).into()
            }
        }
        unsafe extern "system" fn SetSphericalVideoProperties<Identity: IMFVideoProcessorControl3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: f32, y: f32, z: f32, w: f32, fieldofview: f32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFVideoProcessorControl3_Impl::SetSphericalVideoProperties(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y), core::mem::transmute_copy(&z), core::mem::transmute_copy(&w), core::mem::transmute_copy(&fieldofview)).into()
            }
        }
        unsafe extern "system" fn SetOutputDevice<Identity: IMFVideoProcessorControl3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, poutputdevice: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFVideoProcessorControl3_Impl::SetOutputDevice(this, core::mem::transmute_copy(&poutputdevice)).into()
            }
        }
        Self {
            base__: IMFVideoProcessorControl2_Vtbl::new::<Identity, OFFSET>(),
            GetNaturalOutputType: GetNaturalOutputType::<Identity, OFFSET>,
            EnableSphericalVideoProcessing: EnableSphericalVideoProcessing::<Identity, OFFSET>,
            SetSphericalVideoProperties: SetSphericalVideoProperties::<Identity, OFFSET>,
            SetOutputDevice: SetOutputDevice::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFVideoProcessorControl3 as windows_core::Interface>::IID || iid == &<IMFVideoProcessorControl as windows_core::Interface>::IID || iid == &<IMFVideoProcessorControl2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "mfobjects", feature = "windef"))]
impl windows_core::RuntimeName for IMFVideoProcessorControl3 {}
windows_core::imp::define_interface!(IMFVideoRendererEffectControl, IMFVideoRendererEffectControl_Vtbl, 0x604d33d7_cf23_41d5_8224_5bbbb1a87475);
windows_core::imp::interface_hierarchy!(IMFVideoRendererEffectControl, windows_core::IUnknown);
impl IMFVideoRendererEffectControl {
    pub unsafe fn OnAppServiceConnectionEstablished<P0>(&self, pappserviceconnection: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnAppServiceConnectionEstablished)(windows_core::Interface::as_raw(self), pappserviceconnection.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFVideoRendererEffectControl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnAppServiceConnectionEstablished: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFVideoRendererEffectControl_Impl: windows_core::IUnknownImpl {
    fn OnAppServiceConnectionEstablished(&self, pappserviceconnection: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
}
impl IMFVideoRendererEffectControl_Vtbl {
    pub const fn new<Identity: IMFVideoRendererEffectControl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnAppServiceConnectionEstablished<Identity: IMFVideoRendererEffectControl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pappserviceconnection: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFVideoRendererEffectControl_Impl::OnAppServiceConnectionEstablished(this, core::mem::transmute_copy(&pappserviceconnection)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnAppServiceConnectionEstablished: OnAppServiceConnectionEstablished::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFVideoRendererEffectControl as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFVideoRendererEffectControl {}
windows_core::imp::define_interface!(IMFVideoSampleAllocator, IMFVideoSampleAllocator_Vtbl, 0x86cbc910_e533_4751_8e3b_f19b5b806a03);
windows_core::imp::interface_hierarchy!(IMFVideoSampleAllocator, windows_core::IUnknown);
impl IMFVideoSampleAllocator {
    pub unsafe fn SetDirectXManager<P0>(&self, pmanager: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetDirectXManager)(windows_core::Interface::as_raw(self), pmanager.param().abi()) }
    }
    pub unsafe fn UninitializeSampleAllocator(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UninitializeSampleAllocator)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn InitializeSampleAllocator<P1>(&self, crequestedframes: u32, pmediatype: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IMFMediaType>,
    {
        unsafe { (windows_core::Interface::vtable(self).InitializeSampleAllocator)(windows_core::Interface::as_raw(self), crequestedframes, pmediatype.param().abi()) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn AllocateSample(&self) -> windows_core::Result<super::IMFSample> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllocateSample)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFVideoSampleAllocator_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetDirectXManager: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UninitializeSampleAllocator: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "mfobjects")]
    pub InitializeSampleAllocator: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    InitializeSampleAllocator: usize,
    #[cfg(feature = "mfobjects")]
    pub AllocateSample: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    AllocateSample: usize,
}
#[cfg(feature = "mfobjects")]
pub trait IMFVideoSampleAllocator_Impl: windows_core::IUnknownImpl {
    fn SetDirectXManager(&self, pmanager: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn UninitializeSampleAllocator(&self) -> windows_core::Result<()>;
    fn InitializeSampleAllocator(&self, crequestedframes: u32, pmediatype: windows_core::Ref<super::IMFMediaType>) -> windows_core::Result<()>;
    fn AllocateSample(&self) -> windows_core::Result<super::IMFSample>;
}
#[cfg(feature = "mfobjects")]
impl IMFVideoSampleAllocator_Vtbl {
    pub const fn new<Identity: IMFVideoSampleAllocator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetDirectXManager<Identity: IMFVideoSampleAllocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pmanager: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFVideoSampleAllocator_Impl::SetDirectXManager(this, core::mem::transmute_copy(&pmanager)).into()
            }
        }
        unsafe extern "system" fn UninitializeSampleAllocator<Identity: IMFVideoSampleAllocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFVideoSampleAllocator_Impl::UninitializeSampleAllocator(this).into()
            }
        }
        unsafe extern "system" fn InitializeSampleAllocator<Identity: IMFVideoSampleAllocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, crequestedframes: u32, pmediatype: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFVideoSampleAllocator_Impl::InitializeSampleAllocator(this, core::mem::transmute_copy(&crequestedframes), core::mem::transmute_copy(&pmediatype)).into()
            }
        }
        unsafe extern "system" fn AllocateSample<Identity: IMFVideoSampleAllocator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppsample: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFVideoSampleAllocator_Impl::AllocateSample(this) {
                    Ok(ok__) => {
                        ppsample.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetDirectXManager: SetDirectXManager::<Identity, OFFSET>,
            UninitializeSampleAllocator: UninitializeSampleAllocator::<Identity, OFFSET>,
            InitializeSampleAllocator: InitializeSampleAllocator::<Identity, OFFSET>,
            AllocateSample: AllocateSample::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFVideoSampleAllocator as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFVideoSampleAllocator {}
windows_core::imp::define_interface!(IMFVideoSampleAllocatorCallback, IMFVideoSampleAllocatorCallback_Vtbl, 0x992388b4_3372_4f67_8b6f_c84c071f4751);
windows_core::imp::interface_hierarchy!(IMFVideoSampleAllocatorCallback, windows_core::IUnknown);
impl IMFVideoSampleAllocatorCallback {
    pub unsafe fn SetCallback<P0>(&self, pnotify: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IMFVideoSampleAllocatorNotify>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetCallback)(windows_core::Interface::as_raw(self), pnotify.param().abi()) }
    }
    pub unsafe fn GetFreeSampleCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFreeSampleCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFVideoSampleAllocatorCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFreeSampleCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
pub trait IMFVideoSampleAllocatorCallback_Impl: windows_core::IUnknownImpl {
    fn SetCallback(&self, pnotify: windows_core::Ref<IMFVideoSampleAllocatorNotify>) -> windows_core::Result<()>;
    fn GetFreeSampleCount(&self) -> windows_core::Result<i32>;
}
impl IMFVideoSampleAllocatorCallback_Vtbl {
    pub const fn new<Identity: IMFVideoSampleAllocatorCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetCallback<Identity: IMFVideoSampleAllocatorCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pnotify: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFVideoSampleAllocatorCallback_Impl::SetCallback(this, core::mem::transmute_copy(&pnotify)).into()
            }
        }
        unsafe extern "system" fn GetFreeSampleCount<Identity: IMFVideoSampleAllocatorCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, plsamples: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFVideoSampleAllocatorCallback_Impl::GetFreeSampleCount(this) {
                    Ok(ok__) => {
                        plsamples.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetCallback: SetCallback::<Identity, OFFSET>,
            GetFreeSampleCount: GetFreeSampleCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFVideoSampleAllocatorCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFVideoSampleAllocatorCallback {}
windows_core::imp::define_interface!(IMFVideoSampleAllocatorEx, IMFVideoSampleAllocatorEx_Vtbl, 0x545b3a48_3283_4f62_866f_a62d8f598f9f);
impl core::ops::Deref for IMFVideoSampleAllocatorEx {
    type Target = IMFVideoSampleAllocator;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMFVideoSampleAllocatorEx, windows_core::IUnknown, IMFVideoSampleAllocator);
impl IMFVideoSampleAllocatorEx {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn InitializeSampleAllocatorEx<P2, P3>(&self, cinitialsamples: u32, cmaximumsamples: u32, pattributes: P2, pmediatype: P3) -> windows_core::HRESULT
    where
        P2: windows_core::Param<super::IMFAttributes>,
        P3: windows_core::Param<super::IMFMediaType>,
    {
        unsafe { (windows_core::Interface::vtable(self).InitializeSampleAllocatorEx)(windows_core::Interface::as_raw(self), cinitialsamples, cmaximumsamples, pattributes.param().abi(), pmediatype.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFVideoSampleAllocatorEx_Vtbl {
    pub base__: IMFVideoSampleAllocator_Vtbl,
    #[cfg(feature = "mfobjects")]
    pub InitializeSampleAllocatorEx: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    InitializeSampleAllocatorEx: usize,
}
#[cfg(feature = "mfobjects")]
pub trait IMFVideoSampleAllocatorEx_Impl: IMFVideoSampleAllocator_Impl {
    fn InitializeSampleAllocatorEx(&self, cinitialsamples: u32, cmaximumsamples: u32, pattributes: windows_core::Ref<super::IMFAttributes>, pmediatype: windows_core::Ref<super::IMFMediaType>) -> windows_core::Result<()>;
}
#[cfg(feature = "mfobjects")]
impl IMFVideoSampleAllocatorEx_Vtbl {
    pub const fn new<Identity: IMFVideoSampleAllocatorEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn InitializeSampleAllocatorEx<Identity: IMFVideoSampleAllocatorEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cinitialsamples: u32, cmaximumsamples: u32, pattributes: *mut core::ffi::c_void, pmediatype: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFVideoSampleAllocatorEx_Impl::InitializeSampleAllocatorEx(this, core::mem::transmute_copy(&cinitialsamples), core::mem::transmute_copy(&cmaximumsamples), core::mem::transmute_copy(&pattributes), core::mem::transmute_copy(&pmediatype)).into()
            }
        }
        Self { base__: IMFVideoSampleAllocator_Vtbl::new::<Identity, OFFSET>(), InitializeSampleAllocatorEx: InitializeSampleAllocatorEx::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFVideoSampleAllocatorEx as windows_core::Interface>::IID || iid == &<IMFVideoSampleAllocator as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFVideoSampleAllocatorEx {}
windows_core::imp::define_interface!(IMFVideoSampleAllocatorNotify, IMFVideoSampleAllocatorNotify_Vtbl, 0xa792cdbe_c374_4e89_8335_278e7b9956a4);
windows_core::imp::interface_hierarchy!(IMFVideoSampleAllocatorNotify, windows_core::IUnknown);
impl IMFVideoSampleAllocatorNotify {
    pub unsafe fn NotifyRelease(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).NotifyRelease)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFVideoSampleAllocatorNotify_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub NotifyRelease: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IMFVideoSampleAllocatorNotify_Impl: windows_core::IUnknownImpl {
    fn NotifyRelease(&self) -> windows_core::Result<()>;
}
impl IMFVideoSampleAllocatorNotify_Vtbl {
    pub const fn new<Identity: IMFVideoSampleAllocatorNotify_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn NotifyRelease<Identity: IMFVideoSampleAllocatorNotify_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFVideoSampleAllocatorNotify_Impl::NotifyRelease(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), NotifyRelease: NotifyRelease::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFVideoSampleAllocatorNotify as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IMFVideoSampleAllocatorNotify {}
windows_core::imp::define_interface!(IMFVideoSampleAllocatorNotifyEx, IMFVideoSampleAllocatorNotifyEx_Vtbl, 0x3978aa1a_6d5b_4b7f_a340_90899189ae34);
impl core::ops::Deref for IMFVideoSampleAllocatorNotifyEx {
    type Target = IMFVideoSampleAllocatorNotify;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMFVideoSampleAllocatorNotifyEx, windows_core::IUnknown, IMFVideoSampleAllocatorNotify);
impl IMFVideoSampleAllocatorNotifyEx {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn NotifyPrune<P0>(&self, param0: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMFSample>,
    {
        unsafe { (windows_core::Interface::vtable(self).NotifyPrune)(windows_core::Interface::as_raw(self), param0.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFVideoSampleAllocatorNotifyEx_Vtbl {
    pub base__: IMFVideoSampleAllocatorNotify_Vtbl,
    #[cfg(feature = "mfobjects")]
    pub NotifyPrune: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    NotifyPrune: usize,
}
#[cfg(feature = "mfobjects")]
pub trait IMFVideoSampleAllocatorNotifyEx_Impl: IMFVideoSampleAllocatorNotify_Impl {
    fn NotifyPrune(&self, param0: windows_core::Ref<super::IMFSample>) -> windows_core::Result<()>;
}
#[cfg(feature = "mfobjects")]
impl IMFVideoSampleAllocatorNotifyEx_Vtbl {
    pub const fn new<Identity: IMFVideoSampleAllocatorNotifyEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn NotifyPrune<Identity: IMFVideoSampleAllocatorNotifyEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, param0: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFVideoSampleAllocatorNotifyEx_Impl::NotifyPrune(this, core::mem::transmute_copy(&param0)).into()
            }
        }
        Self { base__: IMFVideoSampleAllocatorNotify_Vtbl::new::<Identity, OFFSET>(), NotifyPrune: NotifyPrune::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFVideoSampleAllocatorNotifyEx as windows_core::Interface>::IID || iid == &<IMFVideoSampleAllocatorNotify as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFVideoSampleAllocatorNotifyEx {}
windows_core::imp::define_interface!(IMFWorkQueueServices, IMFWorkQueueServices_Vtbl, 0x35fe1bb8_a3a9_40fe_bbec_eb569c9ccca3);
windows_core::imp::interface_hierarchy!(IMFWorkQueueServices, windows_core::IUnknown);
impl IMFWorkQueueServices {
    #[cfg(feature = "mfobjects")]
    pub unsafe fn BeginRegisterTopologyWorkQueuesWithMMCSS<P0, P1>(&self, pcallback: P0, pstate: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMFAsyncCallback>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).BeginRegisterTopologyWorkQueuesWithMMCSS)(windows_core::Interface::as_raw(self), pcallback.param().abi(), pstate.param().abi()) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn EndRegisterTopologyWorkQueuesWithMMCSS<P0>(&self, presult: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMFAsyncResult>,
    {
        unsafe { (windows_core::Interface::vtable(self).EndRegisterTopologyWorkQueuesWithMMCSS)(windows_core::Interface::as_raw(self), presult.param().abi()) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn BeginUnregisterTopologyWorkQueuesWithMMCSS<P0, P1>(&self, pcallback: P0, pstate: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMFAsyncCallback>,
        P1: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).BeginUnregisterTopologyWorkQueuesWithMMCSS)(windows_core::Interface::as_raw(self), pcallback.param().abi(), pstate.param().abi()) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn EndUnregisterTopologyWorkQueuesWithMMCSS<P0>(&self, presult: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMFAsyncResult>,
    {
        unsafe { (windows_core::Interface::vtable(self).EndUnregisterTopologyWorkQueuesWithMMCSS)(windows_core::Interface::as_raw(self), presult.param().abi()) }
    }
    pub unsafe fn GetTopologyWorkQueueMMCSSClass(&self, dwtopologyworkqueueid: u32, pwszclass: windows_core::PWSTR, pcchclass: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTopologyWorkQueueMMCSSClass)(windows_core::Interface::as_raw(self), dwtopologyworkqueueid, pwszclass, pcchclass as _) }
    }
    pub unsafe fn GetTopologyWorkQueueMMCSSTaskId(&self, dwtopologyworkqueueid: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTopologyWorkQueueMMCSSTaskId)(windows_core::Interface::as_raw(self), dwtopologyworkqueueid, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn BeginRegisterPlatformWorkQueueWithMMCSS<P1, P3, P4>(&self, dwplatformworkqueue: u32, wszclass: P1, dwtaskid: u32, pcallback: P3, pstate: P4) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<super::IMFAsyncCallback>,
        P4: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).BeginRegisterPlatformWorkQueueWithMMCSS)(windows_core::Interface::as_raw(self), dwplatformworkqueue, wszclass.param().abi(), dwtaskid, pcallback.param().abi(), pstate.param().abi()) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn EndRegisterPlatformWorkQueueWithMMCSS<P0>(&self, presult: P0) -> windows_core::Result<u32>
    where
        P0: windows_core::Param<super::IMFAsyncResult>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EndRegisterPlatformWorkQueueWithMMCSS)(windows_core::Interface::as_raw(self), presult.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn BeginUnregisterPlatformWorkQueueWithMMCSS<P1, P2>(&self, dwplatformworkqueue: u32, pcallback: P1, pstate: P2) -> windows_core::HRESULT
    where
        P1: windows_core::Param<super::IMFAsyncCallback>,
        P2: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).BeginUnregisterPlatformWorkQueueWithMMCSS)(windows_core::Interface::as_raw(self), dwplatformworkqueue, pcallback.param().abi(), pstate.param().abi()) }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn EndUnregisterPlatformWorkQueueWithMMCSS<P0>(&self, presult: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IMFAsyncResult>,
    {
        unsafe { (windows_core::Interface::vtable(self).EndUnregisterPlatformWorkQueueWithMMCSS)(windows_core::Interface::as_raw(self), presult.param().abi()) }
    }
    pub unsafe fn GetPlaftormWorkQueueMMCSSClass(&self, dwplatformworkqueueid: u32, pwszclass: windows_core::PWSTR, pcchclass: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPlaftormWorkQueueMMCSSClass)(windows_core::Interface::as_raw(self), dwplatformworkqueueid, pwszclass, pcchclass as _) }
    }
    pub unsafe fn GetPlatformWorkQueueMMCSSTaskId(&self, dwplatformworkqueueid: u32) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPlatformWorkQueueMMCSSTaskId)(windows_core::Interface::as_raw(self), dwplatformworkqueueid, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFWorkQueueServices_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "mfobjects")]
    pub BeginRegisterTopologyWorkQueuesWithMMCSS: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    BeginRegisterTopologyWorkQueuesWithMMCSS: usize,
    #[cfg(feature = "mfobjects")]
    pub EndRegisterTopologyWorkQueuesWithMMCSS: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    EndRegisterTopologyWorkQueuesWithMMCSS: usize,
    #[cfg(feature = "mfobjects")]
    pub BeginUnregisterTopologyWorkQueuesWithMMCSS: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    BeginUnregisterTopologyWorkQueuesWithMMCSS: usize,
    #[cfg(feature = "mfobjects")]
    pub EndUnregisterTopologyWorkQueuesWithMMCSS: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    EndUnregisterTopologyWorkQueuesWithMMCSS: usize,
    pub GetTopologyWorkQueueMMCSSClass: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub GetTopologyWorkQueueMMCSSTaskId: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "mfobjects")]
    pub BeginRegisterPlatformWorkQueueWithMMCSS: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, u32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    BeginRegisterPlatformWorkQueueWithMMCSS: usize,
    #[cfg(feature = "mfobjects")]
    pub EndRegisterPlatformWorkQueueWithMMCSS: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    EndRegisterPlatformWorkQueueWithMMCSS: usize,
    #[cfg(feature = "mfobjects")]
    pub BeginUnregisterPlatformWorkQueueWithMMCSS: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    BeginUnregisterPlatformWorkQueueWithMMCSS: usize,
    #[cfg(feature = "mfobjects")]
    pub EndUnregisterPlatformWorkQueueWithMMCSS: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    EndUnregisterPlatformWorkQueueWithMMCSS: usize,
    pub GetPlaftormWorkQueueMMCSSClass: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PWSTR, *mut u32) -> windows_core::HRESULT,
    pub GetPlatformWorkQueueMMCSSTaskId: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut u32) -> windows_core::HRESULT,
}
#[cfg(feature = "mfobjects")]
pub trait IMFWorkQueueServices_Impl: windows_core::IUnknownImpl {
    fn BeginRegisterTopologyWorkQueuesWithMMCSS(&self, pcallback: windows_core::Ref<super::IMFAsyncCallback>, pstate: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn EndRegisterTopologyWorkQueuesWithMMCSS(&self, presult: windows_core::Ref<super::IMFAsyncResult>) -> windows_core::Result<()>;
    fn BeginUnregisterTopologyWorkQueuesWithMMCSS(&self, pcallback: windows_core::Ref<super::IMFAsyncCallback>, pstate: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn EndUnregisterTopologyWorkQueuesWithMMCSS(&self, presult: windows_core::Ref<super::IMFAsyncResult>) -> windows_core::Result<()>;
    fn GetTopologyWorkQueueMMCSSClass(&self, dwtopologyworkqueueid: u32, pwszclass: windows_core::PWSTR, pcchclass: *mut u32) -> windows_core::Result<()>;
    fn GetTopologyWorkQueueMMCSSTaskId(&self, dwtopologyworkqueueid: u32) -> windows_core::Result<u32>;
    fn BeginRegisterPlatformWorkQueueWithMMCSS(&self, dwplatformworkqueue: u32, wszclass: &windows_core::PCWSTR, dwtaskid: u32, pcallback: windows_core::Ref<super::IMFAsyncCallback>, pstate: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn EndRegisterPlatformWorkQueueWithMMCSS(&self, presult: windows_core::Ref<super::IMFAsyncResult>) -> windows_core::Result<u32>;
    fn BeginUnregisterPlatformWorkQueueWithMMCSS(&self, dwplatformworkqueue: u32, pcallback: windows_core::Ref<super::IMFAsyncCallback>, pstate: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn EndUnregisterPlatformWorkQueueWithMMCSS(&self, presult: windows_core::Ref<super::IMFAsyncResult>) -> windows_core::Result<()>;
    fn GetPlaftormWorkQueueMMCSSClass(&self, dwplatformworkqueueid: u32, pwszclass: windows_core::PWSTR, pcchclass: *mut u32) -> windows_core::Result<()>;
    fn GetPlatformWorkQueueMMCSSTaskId(&self, dwplatformworkqueueid: u32) -> windows_core::Result<u32>;
}
#[cfg(feature = "mfobjects")]
impl IMFWorkQueueServices_Vtbl {
    pub const fn new<Identity: IMFWorkQueueServices_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BeginRegisterTopologyWorkQueuesWithMMCSS<Identity: IMFWorkQueueServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void, pstate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFWorkQueueServices_Impl::BeginRegisterTopologyWorkQueuesWithMMCSS(this, core::mem::transmute_copy(&pcallback), core::mem::transmute_copy(&pstate)).into()
            }
        }
        unsafe extern "system" fn EndRegisterTopologyWorkQueuesWithMMCSS<Identity: IMFWorkQueueServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presult: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFWorkQueueServices_Impl::EndRegisterTopologyWorkQueuesWithMMCSS(this, core::mem::transmute_copy(&presult)).into()
            }
        }
        unsafe extern "system" fn BeginUnregisterTopologyWorkQueuesWithMMCSS<Identity: IMFWorkQueueServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void, pstate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFWorkQueueServices_Impl::BeginUnregisterTopologyWorkQueuesWithMMCSS(this, core::mem::transmute_copy(&pcallback), core::mem::transmute_copy(&pstate)).into()
            }
        }
        unsafe extern "system" fn EndUnregisterTopologyWorkQueuesWithMMCSS<Identity: IMFWorkQueueServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presult: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFWorkQueueServices_Impl::EndUnregisterTopologyWorkQueuesWithMMCSS(this, core::mem::transmute_copy(&presult)).into()
            }
        }
        unsafe extern "system" fn GetTopologyWorkQueueMMCSSClass<Identity: IMFWorkQueueServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwtopologyworkqueueid: u32, pwszclass: windows_core::PWSTR, pcchclass: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFWorkQueueServices_Impl::GetTopologyWorkQueueMMCSSClass(this, core::mem::transmute_copy(&dwtopologyworkqueueid), core::mem::transmute_copy(&pwszclass), core::mem::transmute_copy(&pcchclass)).into()
            }
        }
        unsafe extern "system" fn GetTopologyWorkQueueMMCSSTaskId<Identity: IMFWorkQueueServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwtopologyworkqueueid: u32, pdwtaskid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFWorkQueueServices_Impl::GetTopologyWorkQueueMMCSSTaskId(this, core::mem::transmute_copy(&dwtopologyworkqueueid)) {
                    Ok(ok__) => {
                        pdwtaskid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BeginRegisterPlatformWorkQueueWithMMCSS<Identity: IMFWorkQueueServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwplatformworkqueue: u32, wszclass: windows_core::PCWSTR, dwtaskid: u32, pcallback: *mut core::ffi::c_void, pstate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFWorkQueueServices_Impl::BeginRegisterPlatformWorkQueueWithMMCSS(this, core::mem::transmute_copy(&dwplatformworkqueue), core::mem::transmute(&wszclass), core::mem::transmute_copy(&dwtaskid), core::mem::transmute_copy(&pcallback), core::mem::transmute_copy(&pstate)).into()
            }
        }
        unsafe extern "system" fn EndRegisterPlatformWorkQueueWithMMCSS<Identity: IMFWorkQueueServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presult: *mut core::ffi::c_void, pdwtaskid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFWorkQueueServices_Impl::EndRegisterPlatformWorkQueueWithMMCSS(this, core::mem::transmute_copy(&presult)) {
                    Ok(ok__) => {
                        pdwtaskid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BeginUnregisterPlatformWorkQueueWithMMCSS<Identity: IMFWorkQueueServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwplatformworkqueue: u32, pcallback: *mut core::ffi::c_void, pstate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFWorkQueueServices_Impl::BeginUnregisterPlatformWorkQueueWithMMCSS(this, core::mem::transmute_copy(&dwplatformworkqueue), core::mem::transmute_copy(&pcallback), core::mem::transmute_copy(&pstate)).into()
            }
        }
        unsafe extern "system" fn EndUnregisterPlatformWorkQueueWithMMCSS<Identity: IMFWorkQueueServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, presult: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFWorkQueueServices_Impl::EndUnregisterPlatformWorkQueueWithMMCSS(this, core::mem::transmute_copy(&presult)).into()
            }
        }
        unsafe extern "system" fn GetPlaftormWorkQueueMMCSSClass<Identity: IMFWorkQueueServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwplatformworkqueueid: u32, pwszclass: windows_core::PWSTR, pcchclass: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFWorkQueueServices_Impl::GetPlaftormWorkQueueMMCSSClass(this, core::mem::transmute_copy(&dwplatformworkqueueid), core::mem::transmute_copy(&pwszclass), core::mem::transmute_copy(&pcchclass)).into()
            }
        }
        unsafe extern "system" fn GetPlatformWorkQueueMMCSSTaskId<Identity: IMFWorkQueueServices_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwplatformworkqueueid: u32, pdwtaskid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFWorkQueueServices_Impl::GetPlatformWorkQueueMMCSSTaskId(this, core::mem::transmute_copy(&dwplatformworkqueueid)) {
                    Ok(ok__) => {
                        pdwtaskid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BeginRegisterTopologyWorkQueuesWithMMCSS: BeginRegisterTopologyWorkQueuesWithMMCSS::<Identity, OFFSET>,
            EndRegisterTopologyWorkQueuesWithMMCSS: EndRegisterTopologyWorkQueuesWithMMCSS::<Identity, OFFSET>,
            BeginUnregisterTopologyWorkQueuesWithMMCSS: BeginUnregisterTopologyWorkQueuesWithMMCSS::<Identity, OFFSET>,
            EndUnregisterTopologyWorkQueuesWithMMCSS: EndUnregisterTopologyWorkQueuesWithMMCSS::<Identity, OFFSET>,
            GetTopologyWorkQueueMMCSSClass: GetTopologyWorkQueueMMCSSClass::<Identity, OFFSET>,
            GetTopologyWorkQueueMMCSSTaskId: GetTopologyWorkQueueMMCSSTaskId::<Identity, OFFSET>,
            BeginRegisterPlatformWorkQueueWithMMCSS: BeginRegisterPlatformWorkQueueWithMMCSS::<Identity, OFFSET>,
            EndRegisterPlatformWorkQueueWithMMCSS: EndRegisterPlatformWorkQueueWithMMCSS::<Identity, OFFSET>,
            BeginUnregisterPlatformWorkQueueWithMMCSS: BeginUnregisterPlatformWorkQueueWithMMCSS::<Identity, OFFSET>,
            EndUnregisterPlatformWorkQueueWithMMCSS: EndUnregisterPlatformWorkQueueWithMMCSS::<Identity, OFFSET>,
            GetPlaftormWorkQueueMMCSSClass: GetPlaftormWorkQueueMMCSSClass::<Identity, OFFSET>,
            GetPlatformWorkQueueMMCSSTaskId: GetPlatformWorkQueueMMCSSTaskId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFWorkQueueServices as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFWorkQueueServices {}
windows_core::imp::define_interface!(IMFWorkQueueServicesEx, IMFWorkQueueServicesEx_Vtbl, 0x96bf961b_40fe_42f1_ba9d_320238b49700);
impl core::ops::Deref for IMFWorkQueueServicesEx {
    type Target = IMFWorkQueueServices;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IMFWorkQueueServicesEx, windows_core::IUnknown, IMFWorkQueueServices);
impl IMFWorkQueueServicesEx {
    pub unsafe fn GetTopologyWorkQueueMMCSSPriority(&self, dwtopologyworkqueueid: u32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTopologyWorkQueueMMCSSPriority)(windows_core::Interface::as_raw(self), dwtopologyworkqueueid, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "mfobjects")]
    pub unsafe fn BeginRegisterPlatformWorkQueueWithMMCSSEx<P1, P4, P5>(&self, dwplatformworkqueue: u32, wszclass: P1, dwtaskid: u32, lpriority: i32, pcallback: P4, pstate: P5) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<super::IMFAsyncCallback>,
        P5: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).BeginRegisterPlatformWorkQueueWithMMCSSEx)(windows_core::Interface::as_raw(self), dwplatformworkqueue, wszclass.param().abi(), dwtaskid, lpriority, pcallback.param().abi(), pstate.param().abi()) }
    }
    pub unsafe fn GetPlatformWorkQueueMMCSSPriority(&self, dwplatformworkqueueid: u32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPlatformWorkQueueMMCSSPriority)(windows_core::Interface::as_raw(self), dwplatformworkqueueid, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMFWorkQueueServicesEx_Vtbl {
    pub base__: IMFWorkQueueServices_Vtbl,
    pub GetTopologyWorkQueueMMCSSPriority: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "mfobjects")]
    pub BeginRegisterPlatformWorkQueueWithMMCSSEx: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::PCWSTR, u32, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "mfobjects"))]
    BeginRegisterPlatformWorkQueueWithMMCSSEx: usize,
    pub GetPlatformWorkQueueMMCSSPriority: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "mfobjects")]
pub trait IMFWorkQueueServicesEx_Impl: IMFWorkQueueServices_Impl {
    fn GetTopologyWorkQueueMMCSSPriority(&self, dwtopologyworkqueueid: u32) -> windows_core::Result<i32>;
    fn BeginRegisterPlatformWorkQueueWithMMCSSEx(&self, dwplatformworkqueue: u32, wszclass: &windows_core::PCWSTR, dwtaskid: u32, lpriority: i32, pcallback: windows_core::Ref<super::IMFAsyncCallback>, pstate: windows_core::Ref<windows_core::IUnknown>) -> windows_core::Result<()>;
    fn GetPlatformWorkQueueMMCSSPriority(&self, dwplatformworkqueueid: u32) -> windows_core::Result<i32>;
}
#[cfg(feature = "mfobjects")]
impl IMFWorkQueueServicesEx_Vtbl {
    pub const fn new<Identity: IMFWorkQueueServicesEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetTopologyWorkQueueMMCSSPriority<Identity: IMFWorkQueueServicesEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwtopologyworkqueueid: u32, plpriority: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFWorkQueueServicesEx_Impl::GetTopologyWorkQueueMMCSSPriority(this, core::mem::transmute_copy(&dwtopologyworkqueueid)) {
                    Ok(ok__) => {
                        plpriority.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BeginRegisterPlatformWorkQueueWithMMCSSEx<Identity: IMFWorkQueueServicesEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwplatformworkqueue: u32, wszclass: windows_core::PCWSTR, dwtaskid: u32, lpriority: i32, pcallback: *mut core::ffi::c_void, pstate: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMFWorkQueueServicesEx_Impl::BeginRegisterPlatformWorkQueueWithMMCSSEx(this, core::mem::transmute_copy(&dwplatformworkqueue), core::mem::transmute(&wszclass), core::mem::transmute_copy(&dwtaskid), core::mem::transmute_copy(&lpriority), core::mem::transmute_copy(&pcallback), core::mem::transmute_copy(&pstate)).into()
            }
        }
        unsafe extern "system" fn GetPlatformWorkQueueMMCSSPriority<Identity: IMFWorkQueueServicesEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwplatformworkqueueid: u32, plpriority: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMFWorkQueueServicesEx_Impl::GetPlatformWorkQueueMMCSSPriority(this, core::mem::transmute_copy(&dwplatformworkqueueid)) {
                    Ok(ok__) => {
                        plpriority.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IMFWorkQueueServices_Vtbl::new::<Identity, OFFSET>(),
            GetTopologyWorkQueueMMCSSPriority: GetTopologyWorkQueueMMCSSPriority::<Identity, OFFSET>,
            BeginRegisterPlatformWorkQueueWithMMCSSEx: BeginRegisterPlatformWorkQueueWithMMCSSEx::<Identity, OFFSET>,
            GetPlatformWorkQueueMMCSSPriority: GetPlatformWorkQueueMMCSSPriority::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMFWorkQueueServicesEx as windows_core::Interface>::IID || iid == &<IMFWorkQueueServices as windows_core::Interface>::IID
    }
}
#[cfg(feature = "mfobjects")]
impl windows_core::RuntimeName for IMFWorkQueueServicesEx {}
pub const MEDIASINK_CANNOT_MATCH_CLOCK: u32 = 2;
pub const MEDIASINK_CAN_PREROLL: u32 = 16;
pub const MEDIASINK_CLOCK_REQUIRED: u32 = 8;
pub const MEDIASINK_FIXED_STREAMS: u32 = 1;
pub const MEDIASINK_RATELESS: u32 = 4;
pub const MEDIASINK_REQUIRE_REFERENCE_MEDIATYPE: u32 = 32;
pub type MFAudioConstriction = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MFCLOCK_PROPERTIES {
    pub qwCorrelationRate: u64,
    pub guidClockId: windows_core::GUID,
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MFCONTENTPROTECTIONDEVICE_OUTPUT_DATA {
    pub PrivateDataByteCount: u32,
    pub MaxHWProtectionDataByteCount: u32,
    pub HWProtectionDataByteCount: u32,
    pub Status: windows_core::HRESULT,
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MFCameraIntrinsic_CameraModel {
    pub FocalLength_x: f32,
    pub FocalLength_y: f32,
    pub PrincipalPoint_x: f32,
    pub PrincipalPoint_y: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MFExtendedCameraIntrinsic_IntrinsicModel {
    pub Width: u32,
    pub Height: u32,
    pub SplitFrameId: u32,
    pub CameraModel: MFCameraIntrinsic_CameraModel,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MFNetCredentialManagerGetParam {
    pub hrOp: windows_core::HRESULT,
    pub fAllowLoggedOnUser: windows_core::BOOL,
    pub fClearTextPackage: windows_core::BOOL,
    pub pszUrl: windows_core::PCWSTR,
    pub pszSite: windows_core::PCWSTR,
    pub pszRealm: windows_core::PCWSTR,
    pub pszPackage: windows_core::PCWSTR,
    pub nRetries: i32,
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MFRR_COMPONENTS {
    pub dwRRInfoVersion: u32,
    pub dwRRComponents: u32,
    pub pRRComponents: PMFRR_COMPONENT_HASH_INFO,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
pub const MFSampleExtension_ExtendedCameraIntrinsics: windows_core::GUID = windows_core::GUID::from_u128(0x560bc4a5_4de0_4113_9cdc_832db9740f3d);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct MFSequencerElementId(pub u32);
pub type MFSequencerTopologyFlags = i32;
pub const MFStreamExtension_ExtendedCameraIntrinsics: windows_core::GUID = windows_core::GUID::from_u128(0xaa74b3df_9a2c_48d6_8393_5bd1c1a81e6e);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct MFTIME(pub i64);
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
    pub guidAttributeKey: windows_core::GUID,
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MFT_REGISTRATION_INFO {
    pub clsid: windows_core::GUID,
    pub guidCategory: windows_core::GUID,
    pub uiFlags: u32,
    pub pszName: windows_core::PCWSTR,
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MF_BYTE_STREAM_CACHE_RANGE {
    pub qwStartOffset: u64,
    pub qwEndOffset: u64,
}
pub type MF_CAMERA_CONTROL_CONFIGURATION_TYPE = i32;
pub const MF_CAMERA_CONTROL_CONFIGURATION_TYPE_POSTSTART: MF_CAMERA_CONTROL_CONFIGURATION_TYPE = 1;
pub const MF_CAMERA_CONTROL_CONFIGURATION_TYPE_PRESTART: MF_CAMERA_CONTROL_CONFIGURATION_TYPE = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
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
pub const MF_PD_ADAPTIVE_STREAMING: windows_core::GUID = windows_core::GUID::from_u128(0xea0d5d97_29f9_488b_ae6b_7d6b4136112b);
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
pub const MF_SD_SUPPORTS_PROTECTED_CODEC_SWITCH: windows_core::GUID = windows_core::GUID::from_u128(0x8fb6b117_862e_4b31_8dab_5e0a434caef0);
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
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MF_TRANSCODE_SINK_INFO {
    pub dwVideoStreamID: u32,
    pub pVideoMediaType: core::mem::ManuallyDrop<Option<super::IMFMediaType>>,
    pub dwAudioStreamID: u32,
    pub pAudioMediaType: core::mem::ManuallyDrop<Option<super::IMFMediaType>>,
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MF_VIDEO_SPHERICAL_VIEWDIRECTION {
    pub iHeading: i32,
    pub iPitch: i32,
    pub iRoll: i32,
}
pub const MF_WRAPPED_BUFFER_SERVICE: windows_core::GUID = windows_core::GUID::from_u128(0xab544072_c269_4ebc_a552_1c3b32bed5ca);
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SENSORPROFILEID {
    pub Type: windows_core::GUID,
    pub Index: u32,
    pub Unused: u32,
}
pub const SHA_HASH_LEN: u32 = 20;
pub const STR_HASH_LEN: u32 = 43;
pub const SequencerTopologyFlags_Last: MFSequencerTopologyFlags = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct TOPOID(pub u64);
