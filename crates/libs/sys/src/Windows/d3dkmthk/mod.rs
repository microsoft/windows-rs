#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTAcquireKeyedMutex(param0 : *mut D3DKMT_ACQUIREKEYEDMUTEX) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTAcquireKeyedMutex2(param0 : *mut D3DKMT_ACQUIREKEYEDMUTEX2) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTAdjustFullscreenGamma(param0 : *const D3DKMT_ADJUSTFULLSCREENGAMMA) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTCancelPresents(param0 : *const D3DKMT_CANCEL_PRESENTS) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTChangeVideoMemoryReservation(param0 : *const D3DKMT_CHANGEVIDEOMEMORYRESERVATION) -> super::bcrypt::NTSTATUS);
windows_link::link!("gdi32.dll" "system" fn D3DKMTCheckExclusiveOwnership() -> bool);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTCheckMonitorPowerState(param0 : *const D3DKMT_CHECKMONITORPOWERSTATE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTCheckMultiPlaneOverlaySupport(param0 : *mut D3DKMT_CHECKMULTIPLANEOVERLAYSUPPORT) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTCheckMultiPlaneOverlaySupport2(param0 : *mut D3DKMT_CHECKMULTIPLANEOVERLAYSUPPORT2) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTCheckMultiPlaneOverlaySupport3(param0 : *mut D3DKMT_CHECKMULTIPLANEOVERLAYSUPPORT3) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTCheckOcclusion(param0 : *const D3DKMT_CHECKOCCLUSION) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTCheckSharedResourceAccess(param0 : *const D3DKMT_CHECKSHAREDRESOURCEACCESS) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTCheckVidPnExclusiveOwnership(param0 : *const D3DKMT_CHECKVIDPNEXCLUSIVEOWNERSHIP) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTCloseAdapter(param0 : *const D3DKMT_CLOSEADAPTER) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTConfigureSharedResource(param0 : *const D3DKMT_CONFIGURESHAREDRESOURCE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTConnectDoorbell(param0 : *const D3DKMT_CONNECT_DOORBELL) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTCreateAllocation(param0 : *mut D3DKMT_CREATEALLOCATION) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTCreateAllocation2(param0 : *mut D3DKMT_CREATEALLOCATION) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTCreateContext(param0 : *mut D3DKMT_CREATECONTEXT) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTCreateContextVirtual(param0 : *const D3DKMT_CREATECONTEXTVIRTUAL) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "windef", feature = "wingdi", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTCreateDCFromMemory(param0 : *mut D3DKMT_CREATEDCFROMMEMORY) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTCreateDevice(param0 : *mut D3DKMT_CREATEDEVICE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTCreateDoorbell(param0 : *const D3DKMT_CREATE_DOORBELL) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTCreateHwContext(param0 : *mut D3DKMT_CREATEHWCONTEXT) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTCreateHwQueue(param0 : *mut D3DKMT_CREATEHWQUEUE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTCreateHwQueueForUserModeSubmission(param0 : *mut D3DKMT_CREATEHWQUEUEFORUSERMODESUBMISSION) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTCreateKeyedMutex(param0 : *mut D3DKMT_CREATEKEYEDMUTEX) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTCreateKeyedMutex2(param0 : *mut D3DKMT_CREATEKEYEDMUTEX2) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTCreateNativeFence(param0 : *mut D3DKMT_CREATENATIVEFENCE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTCreateOutputDupl(param0 : *const D3DKMT_CREATE_OUTPUTDUPL) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTCreateOverlay(param0 : *mut D3DKMT_CREATEOVERLAY) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTCreatePagingQueue(param0 : *mut D3DKMT_CREATEPAGINGQUEUE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTCreateProtectedSession(param0 : *mut D3DKMT_CREATEPROTECTEDSESSION) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTCreateSynchronizationObject(param0 : *mut D3DKMT_CREATESYNCHRONIZATIONOBJECT) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTCreateSynchronizationObject2(param0 : *mut D3DKMT_CREATESYNCHRONIZATIONOBJECT2) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTDestroyAllocation(param0 : *const D3DKMT_DESTROYALLOCATION) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTDestroyAllocation2(param0 : *const D3DKMT_DESTROYALLOCATION2) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTDestroyContext(param0 : *const D3DKMT_DESTROYCONTEXT) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "windef", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTDestroyDCFromMemory(param0 : *const D3DKMT_DESTROYDCFROMMEMORY) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTDestroyDevice(param0 : *const D3DKMT_DESTROYDEVICE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTDestroyDoorbell(param0 : *const D3DKMT_DESTROY_DOORBELL) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTDestroyHwContext(param0 : *const D3DKMT_DESTROYHWCONTEXT) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTDestroyHwQueue(param0 : *const D3DKMT_DESTROYHWQUEUE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTDestroyKeyedMutex(param0 : *const D3DKMT_DESTROYKEYEDMUTEX) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTDestroyOutputDupl(param0 : *const D3DKMT_DESTROY_OUTPUTDUPL) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTDestroyOverlay(param0 : *const D3DKMT_DESTROYOVERLAY) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTDestroyPagingQueue(param0 : *mut super::d3dukmdt::D3DDDI_DESTROYPAGINGQUEUE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTDestroyProtectedSession(param0 : *mut D3DKMT_DESTROYPROTECTEDSESSION) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTDestroySynchronizationObject(param0 : *const D3DKMT_DESTROYSYNCHRONIZATIONOBJECT) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTEnumAdapters(param0 : *mut D3DKMT_ENUMADAPTERS) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTEnumAdapters2(param0 : *mut D3DKMT_ENUMADAPTERS2) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
windows_link::link!("api-ms-win-dx-d3dkmt-l1-1-6.dll" "system" fn D3DKMTEnumAdapters3(param0 : *mut D3DKMT_ENUMADAPTERS3) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTEscape(param0 : *const D3DKMT_ESCAPE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTEvict(param0 : *mut D3DKMT_EVICT) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTFlipOverlay(param0 : *const D3DKMT_FLIPOVERLAY) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTFlushHeapTransitions(param0 : *const D3DKMT_FLUSHHEAPTRANSITIONS) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTFreeGpuVirtualAddress(param0 : *const D3DKMT_FREEGPUVIRTUALADDRESS) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTGetAllocationPriority(param0 : *const D3DKMT_GETALLOCATIONPRIORITY) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTGetContextInProcessSchedulingPriority(param0 : *mut D3DKMT_GETCONTEXTINPROCESSSCHEDULINGPRIORITY) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTGetContextSchedulingPriority(param0 : *mut D3DKMT_GETCONTEXTSCHEDULINGPRIORITY) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTGetDWMVerticalBlankEvent(param0 : *const D3DKMT_GETVERTICALBLANKEVENT) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dkmdt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTGetDeviceState(param0 : *mut D3DKMT_GETDEVICESTATE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTGetDisplayModeList(param0 : *mut D3DKMT_GETDISPLAYMODELIST) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTGetMultiPlaneOverlayCaps(param0 : *mut D3DKMT_GET_MULTIPLANE_OVERLAY_CAPS) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTGetMultisampleMethodList(param0 : *mut D3DKMT_GETMULTISAMPLEMETHODLIST) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTGetNativeFenceLogDetail(param0 : *mut D3DKMT_GETNATIVEFENCELOGDETAIL) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTGetOverlayState(param0 : *mut D3DKMT_GETOVERLAYSTATE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTGetPostCompositionCaps(param0 : *mut D3DKMT_GET_POST_COMPOSITION_CAPS) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTGetPresentHistory(param0 : *mut D3DKMT_GETPRESENTHISTORY) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTGetPresentQueueEvent(hadapter : super::d3dukmdt::D3DKMT_HANDLE, param1 : *mut super::winnt::HANDLE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTGetProcessDeviceRemovalSupport(param0 : *mut D3DKMT_GETPROCESSDEVICEREMOVALSUPPORT) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTGetProcessSchedulingPriorityClass(param0 : super::winnt::HANDLE, param1 : *mut D3DKMT_SCHEDULINGPRIORITYCLASS) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTGetResourcePresentPrivateDriverData(param0 : *mut super::d3dukmdt::D3DDDI_GETRESOURCEPRESENTPRIVATEDRIVERDATA) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTGetRuntimeData(param0 : *mut D3DKMT_GETRUNTIMEDATA) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTGetScanLine(param0 : *mut D3DKMT_GETSCANLINE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTGetSharedPrimaryHandle(param0 : *mut D3DKMT_GETSHAREDPRIMARYHANDLE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTGetSharedResourceAdapterLuid(param0 : *mut D3DKMT_GETSHAREDRESOURCEADAPTERLUID) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTInvalidateActiveVidPn(param0 : *const D3DKMT_INVALIDATEACTIVEVIDPN) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTInvalidateCache(param0 : *const D3DKMT_INVALIDATECACHE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTIsFeatureEnabled(param0 : *mut D3DKMT_ISFEATUREENABLED) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTLock(param0 : *mut D3DKMT_LOCK) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTLock2(param0 : *mut D3DKMT_LOCK2) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTMakeResident(param0 : *mut super::d3dukmdt::D3DDDI_MAKERESIDENT) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTMapGpuVirtualAddress(param0 : *mut super::d3dukmdt::D3DDDI_MAPGPUVIRTUALADDRESS) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTMarkDeviceAsError(param0 : *const D3DKMT_MARKDEVICEASERROR) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTNotifyWorkSubmission(param0 : *const D3DKMT_NOTIFY_WORK_SUBMISSION) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTOfferAllocations(param0 : *const D3DKMT_OFFERALLOCATIONS) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTOpenAdapterFromDeviceName(param0 : *mut D3DKMT_OPENADAPTERFROMDEVICENAME) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTOpenAdapterFromGdiDisplayName(param0 : *mut D3DKMT_OPENADAPTERFROMGDIDISPLAYNAME) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTOpenAdapterFromHdc(param0 : *mut D3DKMT_OPENADAPTERFROMHDC) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTOpenAdapterFromLuid(param0 : *mut D3DKMT_OPENADAPTERFROMLUID) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTOpenKeyedMutex(param0 : *mut D3DKMT_OPENKEYEDMUTEX) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTOpenKeyedMutex2(param0 : *mut D3DKMT_OPENKEYEDMUTEX2) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTOpenKeyedMutexFromNtHandle(param0 : *mut D3DKMT_OPENKEYEDMUTEXFROMNTHANDLE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTOpenNativeFenceFromNtHandle(param0 : *mut D3DKMT_OPENNATIVEFENCEFROMNTHANDLE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTOpenNtHandleFromName(param0 : *mut D3DKMT_OPENNTHANDLEFROMNAME) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTOpenProtectedSessionFromNtHandle(param0 : *mut D3DKMT_OPENPROTECTEDSESSIONFROMNTHANDLE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTOpenResource(param0 : *mut D3DKMT_OPENRESOURCE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTOpenResource2(param0 : *mut D3DKMT_OPENRESOURCE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTOpenResourceFromNtHandle(param0 : *mut D3DKMT_OPENRESOURCEFROMNTHANDLE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTOpenSyncObjectFromNtHandle(param0 : *mut D3DKMT_OPENSYNCOBJECTFROMNTHANDLE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTOpenSyncObjectFromNtHandle2(param0 : *mut D3DKMT_OPENSYNCOBJECTFROMNTHANDLE2) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTOpenSyncObjectNtHandleFromName(param0 : *mut D3DKMT_OPENSYNCOBJECTNTHANDLEFROMNAME) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTOpenSynchronizationObject(param0 : *mut D3DKMT_OPENSYNCHRONIZATIONOBJECT) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTOutputDuplGetFrameInfo(param0 : *mut D3DKMT_OUTPUTDUPL_GET_FRAMEINFO) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTOutputDuplGetMetaData(param0 : *mut D3DKMT_OUTPUTDUPL_METADATA) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTOutputDuplGetPointerShapeData(param0 : *mut D3DKMT_OUTPUTDUPL_GET_POINTER_SHAPE_DATA) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dkmdt", feature = "d3dukmdt", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTOutputDuplPresent(param0 : *const D3DKMT_OUTPUTDUPLPRESENT) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dkmdt", feature = "d3dukmdt", feature = "windef"))]
windows_link::link!("api-ms-win-dx-d3dkmt-l1-1-4.dll" "system" fn D3DKMTOutputDuplPresentToHwQueue(param0 : *const D3DKMT_OUTPUTDUPLPRESENTTOHWQUEUE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTOutputDuplReleaseFrame(param0 : *mut D3DKMT_OUTPUTDUPL_RELEASE_FRAME) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTPollDisplayChildren(param0 : *const D3DKMT_POLLDISPLAYCHILDREN) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dkmdt", feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTPresent(param0 : *mut D3DKMT_PRESENT) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTPresentMultiPlaneOverlay(param0 : *const D3DKMT_PRESENT_MULTIPLANE_OVERLAY) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTPresentMultiPlaneOverlay2(param0 : *const D3DKMT_PRESENT_MULTIPLANE_OVERLAY2) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "windef"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTPresentMultiPlaneOverlay3(param0 : *const D3DKMT_PRESENT_MULTIPLANE_OVERLAY3) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTPresentRedirected(param0 : *const D3DKMT_PRESENT_REDIRECTED) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTQueryAdapterInfo(param0 : *mut D3DKMT_QUERYADAPTERINFO) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTQueryAllocationResidency(param0 : *const D3DKMT_QUERYALLOCATIONRESIDENCY) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dkmdt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTQueryClockCalibration(param0 : *mut super::d3dkmdt::D3DKMT_QUERYCLOCKCALIBRATION) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTQueryFSEBlock(param0 : *mut D3DKMT_QUERYFSEBLOCK) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "minwindef"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTQueryFeatureInterface(param0 : *mut D3DKMT_QUERYFEATUREINTERFACE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTQueryProcessOfferInfo(param0 : *mut D3DKMT_QUERYPROCESSOFFERINFO) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTQueryProtectedSessionInfoFromNtHandle(param0 : *mut D3DKMT_QUERYPROTECTEDSESSIONINFOFROMNTHANDLE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTQueryProtectedSessionStatus(param0 : *mut D3DKMT_QUERYPROTECTEDSESSIONSTATUS) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTQueryRemoteVidPnSourceFromGdiDisplayName(param0 : *mut D3DKMT_QUERYREMOTEVIDPNSOURCEFROMGDIDISPLAYNAME) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTQueryResourceInfo(param0 : *mut D3DKMT_QUERYRESOURCEINFO) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTQueryResourceInfoFromNtHandle(param0 : *mut D3DKMT_QUERYRESOURCEINFOFROMNTHANDLE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTQueryStatistics(param0 : *const D3DKMT_QUERYSTATISTICS) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTQueryVidPnExclusiveOwnership(param0 : *mut D3DKMT_QUERYVIDPNEXCLUSIVEOWNERSHIP) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTQueryVideoMemoryInfo(param0 : *mut D3DKMT_QUERYVIDEOMEMORYINFO) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTReclaimAllocations(param0 : *mut D3DKMT_RECLAIMALLOCATIONS) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTReclaimAllocations2(param0 : *mut D3DKMT_RECLAIMALLOCATIONS2) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTRegisterTrimNotification(param0 : *mut D3DKMT_REGISTERTRIMNOTIFICATION) -> super::bcrypt::NTSTATUS);
#[cfg(feature = "bcrypt")]
windows_link::link!("gdi32.dll" "system" fn D3DKMTRegisterVailProcess(param0 : *const windows_sys::core::GUID) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTReleaseKeyedMutex(param0 : *mut D3DKMT_RELEASEKEYEDMUTEX) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTReleaseKeyedMutex2(param0 : *mut D3DKMT_RELEASEKEYEDMUTEX2) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTReleaseProcessVidPnSourceOwners(param0 : super::winnt::HANDLE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTRender(param0 : *mut D3DKMT_RENDER) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTReserveGpuVirtualAddress(param0 : *mut super::d3dukmdt::D3DDDI_RESERVEGPUVIRTUALADDRESS) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTResizeRingBuffer(param0 : *mut D3DKMT_RESIZERINGBUFFER) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTSetAllocationPriority(param0 : *const D3DKMT_SETALLOCATIONPRIORITY) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTSetContextInProcessSchedulingPriority(param0 : *const D3DKMT_SETCONTEXTINPROCESSSCHEDULINGPRIORITY) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTSetContextSchedulingPriority(param0 : *const D3DKMT_SETCONTEXTSCHEDULINGPRIORITY) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTSetDisplayMode(param0 : *mut D3DKMT_SETDISPLAYMODE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTSetDisplayPrivateDriverFormat(param0 : *const D3DKMT_SETDISPLAYPRIVATEDRIVERFORMAT) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTSetFSEBlock(param0 : *const D3DKMT_SETFSEBLOCK) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTSetGammaRamp(param0 : *const D3DKMT_SETGAMMARAMP) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTSetHwProtectionTeardownRecovery(param0 : *const D3DKMT_SETHWPROTECTIONTEARDOWNRECOVERY) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTSetMonitorColorSpaceTransform(param0 : *const D3DKMT_SET_COLORSPACE_TRANSFORM) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTSetProcessSchedulingPriorityClass(param0 : super::winnt::HANDLE, param1 : D3DKMT_SCHEDULINGPRIORITYCLASS) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTSetQueuedLimit(param0 : *const D3DKMT_SETQUEUEDLIMIT) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTSetStablePowerState(param0 : *const D3DKMT_SETSTABLEPOWERSTATE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTSetSyncRefreshCountWaitTarget(param0 : *const D3DKMT_SETSYNCREFRESHCOUNTWAITTARGET) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTSetVidPnSourceHwProtection(param0 : *const D3DKMT_SETVIDPNSOURCEHWPROTECTION) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTSetVidPnSourceOwner(param0 : *const D3DKMT_SETVIDPNSOURCEOWNER) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTSetVidPnSourceOwner1(param0 : *const D3DKMT_SETVIDPNSOURCEOWNER1) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTSetVidPnSourceOwner2(param0 : *const D3DKMT_SETVIDPNSOURCEOWNER2) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTShareObjects(cobjects : u32, hobjects : *const super::d3dukmdt::D3DKMT_HANDLE, pobjectattributes : *const OBJECT_ATTRIBUTES, dwdesiredaccess : u32, phsharednthandle : *mut super::winnt::HANDLE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTSharedPrimaryLockNotification(param0 : *const D3DKMT_SHAREDPRIMARYLOCKNOTIFICATION) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTSharedPrimaryUnLockNotification(param0 : *const D3DKMT_SHAREDPRIMARYUNLOCKNOTIFICATION) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTSignalSynchronizationObject(param0 : *const D3DKMT_SIGNALSYNCHRONIZATIONOBJECT) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTSignalSynchronizationObject2(param0 : *const D3DKMT_SIGNALSYNCHRONIZATIONOBJECT2) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTSignalSynchronizationObjectFromCpu(param0 : *const D3DKMT_SIGNALSYNCHRONIZATIONOBJECTFROMCPU) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTSignalSynchronizationObjectFromGpu(param0 : *const D3DKMT_SIGNALSYNCHRONIZATIONOBJECTFROMGPU) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTSignalSynchronizationObjectFromGpu2(param0 : *const D3DKMT_SIGNALSYNCHRONIZATIONOBJECTFROMGPU2) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTSubmitCommand(param0 : *const D3DKMT_SUBMITCOMMAND) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTSubmitCommandToHwQueue(param0 : *const D3DKMT_SUBMITCOMMANDTOHWQUEUE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dkmdt", feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
windows_link::link!("api-ms-win-dx-d3dkmt-l1-1-4.dll" "system" fn D3DKMTSubmitPresentBltToHwQueue(param0 : *const D3DKMT_SUBMITPRESENTBLTTOHWQUEUE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dkmdt", feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
windows_link::link!("api-ms-win-dx-d3dkmt-l1-1-4.dll" "system" fn D3DKMTSubmitPresentToHwQueue(param0 : *mut D3DKMT_SUBMITPRESENTTOHWQUEUE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTSubmitSignalSyncObjectsToHwQueue(param0 : *const D3DKMT_SUBMITSIGNALSYNCOBJECTSTOHWQUEUE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTSubmitWaitForSyncObjectsToHwQueue(param0 : *const D3DKMT_SUBMITWAITFORSYNCOBJECTSTOHWQUEUE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTTrimProcessCommitment(param0 : *mut D3DKMT_TRIMPROCESSCOMMITMENT) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTUnlock(param0 : *const D3DKMT_UNLOCK) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTUnlock2(param0 : *const D3DKMT_UNLOCK2) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTUnregisterTrimNotification(param0 : *mut D3DKMT_UNREGISTERTRIMNOTIFICATION) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTUpdateAllocationProperty(param0 : *mut super::d3dukmdt::D3DDDI_UPDATEALLOCPROPERTY) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTUpdateGpuVirtualAddress(param0 : *const D3DKMT_UPDATEGPUVIRTUALADDRESS) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTUpdateOverlay(param0 : *const D3DKMT_UPDATEOVERLAY) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTWaitForIdle(param0 : *const D3DKMT_WAITFORIDLE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTWaitForSynchronizationObject(param0 : *const D3DKMT_WAITFORSYNCHRONIZATIONOBJECT) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTWaitForSynchronizationObject2(param0 : *const D3DKMT_WAITFORSYNCHRONIZATIONOBJECT2) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTWaitForSynchronizationObjectFromCpu(param0 : *const D3DKMT_WAITFORSYNCHRONIZATIONOBJECTFROMCPU) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTWaitForSynchronizationObjectFromGpu(param0 : *const D3DKMT_WAITFORSYNCHRONIZATIONOBJECTFROMGPU) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTWaitForVerticalBlankEvent(param0 : *const D3DKMT_WAITFORVERTICALBLANKEVENT) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
windows_link::link!("gdi32.dll" "system" fn D3DKMTWaitForVerticalBlankEvent2(param0 : *const D3DKMT_WAITFORVERTICALBLANKEVENT2) -> super::bcrypt::NTSTATUS);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMDT_DISPLAYMODE_FLAGS {
    pub _bitfield1: u32,
    pub _bitfield2: D3DKMDT_MODE_PRUNING_REASON,
}
pub type D3DKMDT_MODE_PRUNING_REASON = i32;
pub const D3DKMDT_MPR_ALLCAPS: D3DKMDT_MODE_PRUNING_REASON = 1;
pub const D3DKMDT_MPR_CLONE_PATH_PRUNED: D3DKMDT_MODE_PRUNING_REASON = 9;
pub const D3DKMDT_MPR_DEFAULT_PROFILE_MONITOR_SOURCE_MODE: D3DKMDT_MODE_PRUNING_REASON = 6;
pub const D3DKMDT_MPR_DESCRIPTOR_MONITOR_FREQUENCY_RANGE: D3DKMDT_MODE_PRUNING_REASON = 3;
pub const D3DKMDT_MPR_DESCRIPTOR_MONITOR_SOURCE_MODE: D3DKMDT_MODE_PRUNING_REASON = 2;
pub const D3DKMDT_MPR_DESCRIPTOR_OVERRIDE_MONITOR_FREQUENCY_RANGE: D3DKMDT_MODE_PRUNING_REASON = 5;
pub const D3DKMDT_MPR_DESCRIPTOR_OVERRIDE_MONITOR_SOURCE_MODE: D3DKMDT_MODE_PRUNING_REASON = 4;
pub const D3DKMDT_MPR_DRIVER_RECOMMENDED_MONITOR_SOURCE_MODE: D3DKMDT_MODE_PRUNING_REASON = 7;
pub const D3DKMDT_MPR_MAXVALID: D3DKMDT_MODE_PRUNING_REASON = 10;
pub const D3DKMDT_MPR_MONITOR_FREQUENCY_RANGE_OVERRIDE: D3DKMDT_MODE_PRUNING_REASON = 8;
pub const D3DKMDT_MPR_UNINITIALIZED: D3DKMDT_MODE_PRUNING_REASON = 0;
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_ACQUIREKEYEDMUTEX {
    pub hKeyedMutex: super::d3dukmdt::D3DKMT_HANDLE,
    pub Key: u64,
    pub pTimeout: super::winnt::PLARGE_INTEGER,
    pub FenceValue: u64,
}
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_ACQUIREKEYEDMUTEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_ACQUIREKEYEDMUTEX2 {
    pub hKeyedMutex: super::d3dukmdt::D3DKMT_HANDLE,
    pub Key: u64,
    pub pTimeout: super::winnt::PLARGE_INTEGER,
    pub FenceValue: u64,
    pub pPrivateRuntimeData: *mut core::ffi::c_void,
    pub PrivateRuntimeDataSize: u32,
}
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_ACQUIREKEYEDMUTEX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_ACTIVATE_SPECIFIC_DIAG_ESCAPE {
    pub Type: D3DKMT_ACTIVATE_SPECIFIC_DIAG_TYPE,
    pub Activate: windows_sys::core::BOOL,
}
pub type D3DKMT_ACTIVATE_SPECIFIC_DIAG_TYPE = i32;
pub const D3DKMT_ACTIVATE_SPECIFIC_DIAG_TYPE_EXTRA_CCD_DATABASE_INFO: D3DKMT_ACTIVATE_SPECIFIC_DIAG_TYPE = 0;
pub const D3DKMT_ACTIVATE_SPECIFIC_DIAG_TYPE_MODES_PRUNED: D3DKMT_ACTIVATE_SPECIFIC_DIAG_TYPE = 15;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_ADAPTERADDRESS {
    pub BusNumber: u32,
    pub DeviceNumber: u32,
    pub FunctionNumber: u32,
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_ADAPTERINFO {
    pub hAdapter: super::d3dukmdt::D3DKMT_HANDLE,
    pub AdapterLuid: super::winnt::LUID,
    pub NumOfSources: u32,
    pub bPrecisePresentRegionsPreferred: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_ADAPTERREGISTRYINFO {
    pub AdapterString: [u16; 260],
    pub BiosString: [u16; 260],
    pub DacType: [u16; 260],
    pub ChipType: [u16; 260],
}
impl Default for D3DKMT_ADAPTERREGISTRYINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_ADAPTERTYPE {
    pub Anonymous: D3DKMT_ADAPTERTYPE_0,
}
impl Default for D3DKMT_ADAPTERTYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_ADAPTERTYPE_0 {
    pub Anonymous: D3DKMT_ADAPTERTYPE_0_0,
    pub Value: u32,
}
impl Default for D3DKMT_ADAPTERTYPE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_ADAPTERTYPE_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_ADAPTER_PERFDATA {
    pub PhysicalAdapterIndex: u32,
    pub MemoryFrequency: u64,
    pub MaxMemoryFrequency: u64,
    pub MaxMemoryFrequencyOC: u64,
    pub MemoryBandwidth: u64,
    pub PCIEBandwidth: u64,
    pub FanRPM: u32,
    pub Power: u32,
    pub Temperature: u32,
    pub PowerStateOverride: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_ADAPTER_PERFDATACAPS {
    pub PhysicalAdapterIndex: u32,
    pub MaxMemoryBandwidth: u64,
    pub MaxPCIEBandwidth: u64,
    pub MaxFanRPM: u32,
    pub TemperatureMax: u32,
    pub TemperatureWarning: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_ADAPTER_VERIFIER_OPTION {
    pub Type: D3DKMT_ADAPTER_VERIFIER_OPTION_TYPE,
    pub Mode: D3DKMT_VERIFIER_OPTION_MODE,
    pub Data: D3DKMT_ADAPTER_VERIFIER_OPTION_DATA,
}
impl Default for D3DKMT_ADAPTER_VERIFIER_OPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_ADAPTER_VERIFIER_OPTION_DATA {
    pub VidMmFlags: D3DKMT_ADAPTER_VERIFIER_VIDMM_FLAGS,
    pub VidMmTrimInterval: D3DKMT_ADAPTER_VERIFIER_VIDMM_TRIM_INTERVAL,
    pub VidMmPagingInfo: D3DKMT_ADAPTER_VERIFIER_VIDMM_PAGING_INFO,
}
impl Default for D3DKMT_ADAPTER_VERIFIER_OPTION_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3DKMT_ADAPTER_VERIFIER_OPTION_TYPE = i32;
pub const D3DKMT_ADAPTER_VERIFIER_OPTION_VIDMM_FLAGS: D3DKMT_ADAPTER_VERIFIER_OPTION_TYPE = 1000;
pub const D3DKMT_ADAPTER_VERIFIER_OPTION_VIDMM_PAGING_INFO: D3DKMT_ADAPTER_VERIFIER_OPTION_TYPE = 1002;
pub const D3DKMT_ADAPTER_VERIFIER_OPTION_VIDMM_TRIM_INTERVAL: D3DKMT_ADAPTER_VERIFIER_OPTION_TYPE = 1001;
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_ADAPTER_VERIFIER_VIDMM_FLAGS {
    pub Anonymous: D3DKMT_ADAPTER_VERIFIER_VIDMM_FLAGS_0,
    pub Value: u32,
}
impl Default for D3DKMT_ADAPTER_VERIFIER_VIDMM_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_ADAPTER_VERIFIER_VIDMM_FLAGS_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_ADAPTER_VERIFIER_VIDMM_PAGING_INFO {
    pub FillPattern: u32,
    pub MaxTransferChunkSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_ADAPTER_VERIFIER_VIDMM_TRIM_INTERVAL {
    pub MinimumTrimInterval: u64,
    pub MaximumTrimInterval: u64,
    pub IdleTrimInterval: u64,
    pub ForegroundTrimInterval: u64,
    pub StartPeriodicTrimThreshold: u32,
    pub CriticalPeriodicTrimThreshold: u32,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_ADJUSTFULLSCREENGAMMA {
    pub hAdapter: super::d3dukmdt::D3DKMT_HANDLE,
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub Scale: super::d3dukmdt::D3DDDI_DXGI_RGB,
    pub Offset: super::d3dukmdt::D3DDDI_DXGI_RGB,
}
pub type D3DKMT_ALLOCATIONRESIDENCYSTATUS = i32;
pub const D3DKMT_ALLOCATIONRESIDENCYSTATUS_NOTRESIDENT: D3DKMT_ALLOCATIONRESIDENCYSTATUS = 3;
pub const D3DKMT_ALLOCATIONRESIDENCYSTATUS_RESIDENTINGPUMEMORY: D3DKMT_ALLOCATIONRESIDENCYSTATUS = 1;
pub const D3DKMT_ALLOCATIONRESIDENCYSTATUS_RESIDENTINSHAREDMEMORY: D3DKMT_ALLOCATIONRESIDENCYSTATUS = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_AUXILIARYPRESENTINFO {
    pub size: u32,
    pub r#type: D3DKMT_AUXILIARYPRESENTINFO_TYPE,
}
pub type D3DKMT_AUXILIARYPRESENTINFO_TYPE = i32;
pub const D3DKMT_AUXILIARYPRESENTINFO_TYPE_FLIPMANAGER: D3DKMT_AUXILIARYPRESENTINFO_TYPE = 0;
pub const D3DKMT_AllocationPriorityClassHigh: D3DKMT_QUERYSTATISTICS_ALLOCATION_PRIORITY_CLASS = 3;
pub const D3DKMT_AllocationPriorityClassLow: D3DKMT_QUERYSTATISTICS_ALLOCATION_PRIORITY_CLASS = 1;
pub const D3DKMT_AllocationPriorityClassMaximum: D3DKMT_QUERYSTATISTICS_ALLOCATION_PRIORITY_CLASS = 4;
pub const D3DKMT_AllocationPriorityClassMinimum: D3DKMT_QUERYSTATISTICS_ALLOCATION_PRIORITY_CLASS = 0;
pub const D3DKMT_AllocationPriorityClassNormal: D3DKMT_QUERYSTATISTICS_ALLOCATION_PRIORITY_CLASS = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_BDDFALLBACK_CTL {
    pub ForceBddHeadlessNextFallback: bool,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_BLOCKLIST_INFO {
    pub Size: u32,
    pub BlockList: [u16; 1],
}
impl Default for D3DKMT_BLOCKLIST_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_BLTMODEL_PRESENTHISTORYTOKEN {
    pub hLogicalSurface: u64,
    pub hPhysicalSurface: u64,
    pub EventId: u64,
    pub DirtyRegions: D3DKMT_DIRTYREGIONS,
}
#[repr(C)]
#[cfg(all(feature = "d3dkmdt", feature = "d3dukmdt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_BRIGHTNESS_INFO {
    pub Type: D3DKMT_BRIGHTNESS_INFO_TYPE,
    pub ChildUid: u32,
    pub Anonymous: D3DKMT_BRIGHTNESS_INFO_0,
}
#[cfg(all(feature = "d3dkmdt", feature = "d3dukmdt"))]
impl Default for D3DKMT_BRIGHTNESS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dkmdt", feature = "d3dukmdt"))]
#[derive(Clone, Copy)]
pub union D3DKMT_BRIGHTNESS_INFO_0 {
    pub PossibleLevels: D3DKMT_BRIGHTNESS_POSSIBLE_LEVELS,
    pub Brightness: u8,
    pub BrightnessCaps: super::d3dkmdt::DXGK_BRIGHTNESS_CAPS,
    pub BrightnessState: super::d3dkmdt::DXGK_BRIGHTNESS_STATE,
    pub OptimizationLevel: super::d3dkmdt::DXGK_BACKLIGHT_OPTIMIZATION_LEVEL,
    pub ReductionInfo: super::d3dkmdt::DXGK_BACKLIGHT_INFO,
    pub VerboseLogging: bool,
    pub NitRanges: super::d3dkmdt::DXGK_BRIGHTNESS_GET_NIT_RANGES_OUT,
    pub GetBrightnessMillinits: super::d3dkmdt::DXGK_BRIGHTNESS_GET_OUT,
    pub SetBrightnessMillinits: super::d3dkmdt::DXGK_BRIGHTNESS_SET_IN,
    pub BrightnessInterfaceSupported: D3DKMT_BRIGHTNESS_INTERFACE_VERSION,
}
#[cfg(all(feature = "d3dkmdt", feature = "d3dukmdt"))]
impl Default for D3DKMT_BRIGHTNESS_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3DKMT_BRIGHTNESS_INFO_BEGIN_MANUAL_MODE: D3DKMT_BRIGHTNESS_INFO_TYPE = 8;
pub const D3DKMT_BRIGHTNESS_INFO_END_MANUAL_MODE: D3DKMT_BRIGHTNESS_INFO_TYPE = 9;
pub const D3DKMT_BRIGHTNESS_INFO_GET: D3DKMT_BRIGHTNESS_INFO_TYPE = 2;
pub const D3DKMT_BRIGHTNESS_INFO_GET_CAPS: D3DKMT_BRIGHTNESS_INFO_TYPE = 4;
pub const D3DKMT_BRIGHTNESS_INFO_GET_NIT_RANGES: D3DKMT_BRIGHTNESS_INFO_TYPE = 11;
pub const D3DKMT_BRIGHTNESS_INFO_GET_POSSIBLE_LEVELS: D3DKMT_BRIGHTNESS_INFO_TYPE = 1;
pub const D3DKMT_BRIGHTNESS_INFO_GET_REDUCTION: D3DKMT_BRIGHTNESS_INFO_TYPE = 7;
pub const D3DKMT_BRIGHTNESS_INFO_SET: D3DKMT_BRIGHTNESS_INFO_TYPE = 3;
pub const D3DKMT_BRIGHTNESS_INFO_SET_OPTIMIZATION: D3DKMT_BRIGHTNESS_INFO_TYPE = 6;
pub const D3DKMT_BRIGHTNESS_INFO_SET_STATE: D3DKMT_BRIGHTNESS_INFO_TYPE = 5;
pub const D3DKMT_BRIGHTNESS_INFO_SUPPORTED_INTERFACE: D3DKMT_BRIGHTNESS_INFO_TYPE = 12;
pub const D3DKMT_BRIGHTNESS_INFO_TOGGLE_LOGGING: D3DKMT_BRIGHTNESS_INFO_TYPE = 10;
pub type D3DKMT_BRIGHTNESS_INFO_TYPE = i32;
pub type D3DKMT_BRIGHTNESS_INTERFACE_VERSION = i32;
pub const D3DKMT_BRIGHTNESS_INTERFACE_VERSION_1: D3DKMT_BRIGHTNESS_INTERFACE_VERSION = 1;
pub const D3DKMT_BRIGHTNESS_INTERFACE_VERSION_2: D3DKMT_BRIGHTNESS_INTERFACE_VERSION = 2;
pub const D3DKMT_BRIGHTNESS_INTERFACE_VERSION_3: D3DKMT_BRIGHTNESS_INTERFACE_VERSION = 3;
pub const D3DKMT_BRIGHTNESS_INTERFACE_VERSION_UNINITIALIZED: D3DKMT_BRIGHTNESS_INTERFACE_VERSION = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_BRIGHTNESS_POSSIBLE_LEVELS {
    pub LevelCount: u8,
    pub BrightnessLevels: [u8; 256],
}
impl Default for D3DKMT_BRIGHTNESS_POSSIBLE_LEVELS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_BUDGETCHANGENOTIFICATION {
    pub Context: *mut core::ffi::c_void,
    pub Budget: u64,
}
impl Default for D3DKMT_BUDGETCHANGENOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_CANCEL_PRESENTS {
    pub cbSize: u32,
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub Flags: D3DKMT_CANCEL_PRESENTS_FLAGS,
    pub Operation: D3DKMT_CANCEL_PRESENTS_OPERATION,
    pub CancelFromPresentId: u64,
    pub CompSurfaceLuid: super::winnt::LUID,
    pub BindId: u64,
    pub hFlipManagerProcessedEvent: super::winnt::HANDLE,
}
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_CANCEL_PRESENTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CANCEL_PRESENTS_FLAGS {
    pub Anonymous: D3DKMT_CANCEL_PRESENTS_FLAGS_0,
}
impl Default for D3DKMT_CANCEL_PRESENTS_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_CANCEL_PRESENTS_FLAGS_0 {
    pub ReprogramInterrupt: D3DKMT_CANCEL_PRESENTS_FLAGS_0_0,
    pub Value: u32,
}
impl Default for D3DKMT_CANCEL_PRESENTS_FLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_CANCEL_PRESENTS_FLAGS_0_0 {
    pub _bitfield: u32,
}
pub type D3DKMT_CANCEL_PRESENTS_OPERATION = i32;
pub const D3DKMT_CANCEL_PRESENTS_OPERATION_CANCEL_FROM: D3DKMT_CANCEL_PRESENTS_OPERATION = 0;
pub const D3DKMT_CANCEL_PRESENTS_OPERATION_FLUSH_COMPLETED_PRESENTS: D3DKMT_CANCEL_PRESENTS_OPERATION = 2;
pub const D3DKMT_CANCEL_PRESENTS_OPERATION_REPROGRAM_INTERRUPT: D3DKMT_CANCEL_PRESENTS_OPERATION = 1;
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_CHANGESURFACEPOINTER {
    pub hDC: super::windef::HDC,
    pub hBitmap: super::winnt::HANDLE,
    pub pSurfacePointer: *mut core::ffi::c_void,
    pub Width: u32,
    pub Height: u32,
    pub Pitch: u32,
}
#[cfg(all(feature = "windef", feature = "winnt"))]
impl Default for D3DKMT_CHANGESURFACEPOINTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_CHANGEVIDEOMEMORYRESERVATION {
    pub hProcess: super::winnt::HANDLE,
    pub hAdapter: super::d3dukmdt::D3DKMT_HANDLE,
    pub MemorySegmentGroup: D3DKMT_MEMORY_SEGMENT_GROUP,
    pub Reservation: u64,
    pub PhysicalAdapterIndex: u32,
}
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_CHANGEVIDEOMEMORYRESERVATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_CHECKMONITORPOWERSTATE {
    pub hAdapter: super::d3dukmdt::D3DKMT_HANDLE,
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_CHECKMULTIPLANEOVERLAYSUPPORT {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub PlaneCount: u32,
    pub pOverlayPlanes: *mut D3DKMT_CHECK_MULTIPLANE_OVERLAY_PLANE,
    pub Supported: windows_sys::core::BOOL,
    pub ReturnInfo: D3DKMT_CHECK_MULTIPLANE_OVERLAY_SUPPORT_RETURN_INFO,
}
#[cfg(all(feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
impl Default for D3DKMT_CHECKMULTIPLANEOVERLAYSUPPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_CHECKMULTIPLANEOVERLAYSUPPORT2 {
    pub hAdapter: super::d3dukmdt::D3DKMT_HANDLE,
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub PlaneCount: u32,
    pub pOverlayPlanes: *mut D3DKMT_CHECK_MULTIPLANE_OVERLAY_PLANE2,
    pub Supported: windows_sys::core::BOOL,
    pub ReturnInfo: D3DKMT_CHECK_MULTIPLANE_OVERLAY_SUPPORT_RETURN_INFO,
}
#[cfg(all(feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
impl Default for D3DKMT_CHECKMULTIPLANEOVERLAYSUPPORT2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_CHECKMULTIPLANEOVERLAYSUPPORT3 {
    pub hAdapter: super::d3dukmdt::D3DKMT_HANDLE,
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub PlaneCount: u32,
    pub ppOverlayPlanes: *mut *mut D3DKMT_CHECK_MULTIPLANE_OVERLAY_PLANE3,
    pub PostCompositionCount: u32,
    pub ppPostComposition: *mut *mut D3DKMT_MULTIPLANE_OVERLAY_POST_COMPOSITION_WITH_SOURCE,
    pub Supported: windows_sys::core::BOOL,
    pub ReturnInfo: D3DKMT_CHECK_MULTIPLANE_OVERLAY_SUPPORT_RETURN_INFO,
}
#[cfg(all(feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
impl Default for D3DKMT_CHECKMULTIPLANEOVERLAYSUPPORT3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct D3DKMT_CHECKOCCLUSION {
    pub hWindow: super::windef::HWND,
}
#[cfg(feature = "windef")]
impl Default for D3DKMT_CHECKOCCLUSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_CHECKSHAREDRESOURCEACCESS {
    pub hResource: super::d3dukmdt::D3DKMT_HANDLE,
    pub ClientPid: u32,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_CHECKVIDPNEXCLUSIVEOWNERSHIP {
    pub hAdapter: super::d3dukmdt::D3DKMT_HANDLE,
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_CHECK_MULTIPLANE_OVERLAY_PLANE {
    pub hResource: super::d3dukmdt::D3DKMT_HANDLE,
    pub CompSurfaceLuid: super::winnt::LUID,
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub PlaneAttributes: D3DKMT_MULTIPLANE_OVERLAY_ATTRIBUTES,
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_CHECK_MULTIPLANE_OVERLAY_PLANE2 {
    pub LayerIndex: u32,
    pub hResource: super::d3dukmdt::D3DKMT_HANDLE,
    pub CompSurfaceLuid: super::winnt::LUID,
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub PlaneAttributes: D3DKMT_MULTIPLANE_OVERLAY_ATTRIBUTES2,
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_CHECK_MULTIPLANE_OVERLAY_PLANE3 {
    pub LayerIndex: u32,
    pub hResource: super::d3dukmdt::D3DKMT_HANDLE,
    pub CompSurfaceLuid: super::winnt::LUID,
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub pPlaneAttributes: *mut D3DKMT_MULTIPLANE_OVERLAY_ATTRIBUTES3,
}
#[cfg(all(feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
impl Default for D3DKMT_CHECK_MULTIPLANE_OVERLAY_PLANE3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CHECK_MULTIPLANE_OVERLAY_SUPPORT_RETURN_INFO {
    pub Anonymous: D3DKMT_CHECK_MULTIPLANE_OVERLAY_SUPPORT_RETURN_INFO_0,
}
impl Default for D3DKMT_CHECK_MULTIPLANE_OVERLAY_SUPPORT_RETURN_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_CHECK_MULTIPLANE_OVERLAY_SUPPORT_RETURN_INFO_0 {
    pub Anonymous: D3DKMT_CHECK_MULTIPLANE_OVERLAY_SUPPORT_RETURN_INFO_0_0,
    pub Value: u32,
}
impl Default for D3DKMT_CHECK_MULTIPLANE_OVERLAY_SUPPORT_RETURN_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_CHECK_MULTIPLANE_OVERLAY_SUPPORT_RETURN_INFO_0_0 {
    pub _bitfield: u32,
}
pub type D3DKMT_CLIENTHINT = i32;
pub const D3DKMT_CLIENTHINT_11ON12: D3DKMT_CLIENTHINT = 14;
pub const D3DKMT_CLIENTHINT_9ON12: D3DKMT_CLIENTHINT = 13;
pub const D3DKMT_CLIENTHINT_CDD: D3DKMT_CLIENTHINT = 2;
pub const D3DKMT_CLIENTHINT_CLON12: D3DKMT_CLIENTHINT = 17;
pub const D3DKMT_CLIENTHINT_CUDA: D3DKMT_CLIENTHINT = 5;
pub const D3DKMT_CLIENTHINT_DML_PYTORCH: D3DKMT_CLIENTHINT = 20;
pub const D3DKMT_CLIENTHINT_DML_TENSORFLOW: D3DKMT_CLIENTHINT = 18;
pub const D3DKMT_CLIENTHINT_DML_WEBNN: D3DKMT_CLIENTHINT = 28;
pub const D3DKMT_CLIENTHINT_DX10: D3DKMT_CLIENTHINT = 10;
pub const D3DKMT_CLIENTHINT_DX11: D3DKMT_CLIENTHINT = 11;
pub const D3DKMT_CLIENTHINT_DX12: D3DKMT_CLIENTHINT = 12;
pub const D3DKMT_CLIENTHINT_DX7: D3DKMT_CLIENTHINT = 7;
pub const D3DKMT_CLIENTHINT_DX8: D3DKMT_CLIENTHINT = 8;
pub const D3DKMT_CLIENTHINT_DX9: D3DKMT_CLIENTHINT = 9;
pub const D3DKMT_CLIENTHINT_FASTRPC: D3DKMT_CLIENTHINT = 22;
pub const D3DKMT_CLIENTHINT_FFMPEG: D3DKMT_CLIENTHINT = 26;
pub const D3DKMT_CLIENTHINT_GLON12: D3DKMT_CLIENTHINT = 16;
pub const D3DKMT_CLIENTHINT_MAX: D3DKMT_CLIENTHINT = 29;
pub const D3DKMT_CLIENTHINT_MFT_ENCODE: D3DKMT_CLIENTHINT = 15;
pub const D3DKMT_CLIENTHINT_ONEAPI_LEVEL0: D3DKMT_CLIENTHINT = 19;
pub const D3DKMT_CLIENTHINT_OPENCL: D3DKMT_CLIENTHINT = 3;
pub const D3DKMT_CLIENTHINT_OPENGL: D3DKMT_CLIENTHINT = 1;
pub const D3DKMT_CLIENTHINT_OPEN_VINO: D3DKMT_CLIENTHINT = 27;
pub const D3DKMT_CLIENTHINT_QNN: D3DKMT_CLIENTHINT = 24;
pub const D3DKMT_CLIENTHINT_RESERVED: D3DKMT_CLIENTHINT = 6;
pub const D3DKMT_CLIENTHINT_SNPE: D3DKMT_CLIENTHINT = 23;
pub const D3DKMT_CLIENTHINT_UNKNOWN: D3DKMT_CLIENTHINT = 0;
pub const D3DKMT_CLIENTHINT_VITIS: D3DKMT_CLIENTHINT = 25;
pub const D3DKMT_CLIENTHINT_VKON12: D3DKMT_CLIENTHINT = 21;
pub const D3DKMT_CLIENTHINT_VULKAN: D3DKMT_CLIENTHINT = 4;
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_CLOSEADAPTER {
    pub hAdapter: super::d3dukmdt::D3DKMT_HANDLE,
}
pub const D3DKMT_COMPONENTIZED_INDICATOR: u32 = 35;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_COMPOSITION_PRESENTHISTORYTOKEN {
    pub hPrivateData: u64,
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_CONFIGURESHAREDRESOURCE {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub hResource: super::d3dukmdt::D3DKMT_HANDLE,
    pub IsDwm: bool,
    pub hProcess: super::winnt::HANDLE,
    pub AllowAccess: bool,
}
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_CONFIGURESHAREDRESOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_CONNECT_DOORBELL {
    pub hHwQueue: super::d3dukmdt::D3DKMT_HANDLE,
    pub Flags: D3DKMT_CONNECT_DOORBELL_FLAGS,
    pub DoorbellMapping: super::d3dukmdt::D3DDDI_DOORBELLMAPPING,
    pub Reserved: [u8; 64],
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_CONNECT_DOORBELL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CONNECT_DOORBELL_FLAGS {
    pub Anonymous: D3DKMT_CONNECT_DOORBELL_FLAGS_0,
}
impl Default for D3DKMT_CONNECT_DOORBELL_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_CONNECT_DOORBELL_FLAGS_0 {
    pub Anonymous: D3DKMT_CONNECT_DOORBELL_FLAGS_0_0,
    pub Value: u32,
}
impl Default for D3DKMT_CONNECT_DOORBELL_FLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_CONNECT_DOORBELL_FLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CPDRIVERNAME {
    pub ContentProtectionFileName: [u16; 260],
}
impl Default for D3DKMT_CPDRIVERNAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_CREATEALLOCATION {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub hResource: super::d3dukmdt::D3DKMT_HANDLE,
    pub hGlobalShare: super::d3dukmdt::D3DKMT_HANDLE,
    pub pPrivateRuntimeData: *const core::ffi::c_void,
    pub PrivateRuntimeDataSize: u32,
    pub Anonymous: D3DKMT_CREATEALLOCATION_0,
    pub PrivateDriverDataSize: u32,
    pub NumAllocations: u32,
    pub Anonymous2: D3DKMT_CREATEALLOCATION_1,
    pub Flags: D3DKMT_CREATEALLOCATIONFLAGS,
    pub hPrivateRuntimeResourceHandle: super::winnt::HANDLE,
}
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_CREATEALLOCATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union D3DKMT_CREATEALLOCATION_0 {
    pub pStandardAllocation: *mut D3DKMT_CREATESTANDARDALLOCATION,
    pub pPrivateDriverData: *const core::ffi::c_void,
}
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_CREATEALLOCATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union D3DKMT_CREATEALLOCATION_1 {
    pub pAllocationInfo: *mut super::d3dukmdt::D3DDDI_ALLOCATIONINFO,
    pub pAllocationInfo2: *mut super::d3dukmdt::D3DDDI_ALLOCATIONINFO2,
}
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_CREATEALLOCATION_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_CREATEALLOCATIONFLAGS {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_CREATECONTEXT {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub NodeOrdinal: u32,
    pub EngineAffinity: u32,
    pub Flags: super::d3dukmdt::D3DDDI_CREATECONTEXTFLAGS,
    pub pPrivateDriverData: *mut core::ffi::c_void,
    pub PrivateDriverDataSize: u32,
    pub ClientHint: D3DKMT_CLIENTHINT,
    pub hContext: super::d3dukmdt::D3DKMT_HANDLE,
    pub pCommandBuffer: *mut core::ffi::c_void,
    pub CommandBufferSize: u32,
    pub pAllocationList: *mut super::d3dukmdt::D3DDDI_ALLOCATIONLIST,
    pub AllocationListSize: u32,
    pub pPatchLocationList: *mut super::d3dukmdt::D3DDDI_PATCHLOCATIONLIST,
    pub PatchLocationListSize: u32,
    pub CommandBuffer: super::d3dukmdt::D3DGPU_VIRTUAL_ADDRESS,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_CREATECONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_CREATECONTEXTVIRTUAL {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub NodeOrdinal: u32,
    pub EngineAffinity: u32,
    pub Flags: super::d3dukmdt::D3DDDI_CREATECONTEXTFLAGS,
    pub pPrivateDriverData: *mut core::ffi::c_void,
    pub PrivateDriverDataSize: u32,
    pub ClientHint: D3DKMT_CLIENTHINT,
    pub hContext: super::d3dukmdt::D3DKMT_HANDLE,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_CREATECONTEXTVIRTUAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "windef", feature = "wingdi", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_CREATEDCFROMMEMORY {
    pub pMemory: *mut core::ffi::c_void,
    pub Format: super::d3dukmdt::D3DDDIFORMAT,
    pub Width: u32,
    pub Height: u32,
    pub Pitch: u32,
    pub hDeviceDc: super::windef::HDC,
    pub pColorTable: *mut super::wingdi::PALETTEENTRY,
    pub hDc: super::windef::HDC,
    pub hBitmap: super::winnt::HANDLE,
}
#[cfg(all(feature = "d3dukmdt", feature = "windef", feature = "wingdi", feature = "winnt"))]
impl Default for D3DKMT_CREATEDCFROMMEMORY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_CREATEDEVICE {
    pub Anonymous: D3DKMT_CREATEDEVICE_0,
    pub Flags: D3DKMT_CREATEDEVICEFLAGS,
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub pCommandBuffer: *mut core::ffi::c_void,
    pub CommandBufferSize: u32,
    pub pAllocationList: *mut super::d3dukmdt::D3DDDI_ALLOCATIONLIST,
    pub AllocationListSize: u32,
    pub pPatchLocationList: *mut super::d3dukmdt::D3DDDI_PATCHLOCATIONLIST,
    pub PatchLocationListSize: u32,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_CREATEDEVICE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub union D3DKMT_CREATEDEVICE_0 {
    pub hAdapter: super::d3dukmdt::D3DKMT_HANDLE,
    pub pAdapter: *mut core::ffi::c_void,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_CREATEDEVICE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_CREATEDEVICEFLAGS {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_CREATEHWCONTEXT {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub NodeOrdinal: u32,
    pub EngineAffinity: u32,
    pub Flags: super::d3dukmdt::D3DDDI_CREATEHWCONTEXTFLAGS,
    pub PrivateDriverDataSize: u32,
    pub pPrivateDriverData: *mut core::ffi::c_void,
    pub hHwContext: super::d3dukmdt::D3DKMT_HANDLE,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_CREATEHWCONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_CREATEHWQUEUE {
    pub hHwContext: super::d3dukmdt::D3DKMT_HANDLE,
    pub Flags: super::d3dukmdt::D3DDDI_CREATEHWQUEUEFLAGS,
    pub PrivateDriverDataSize: u32,
    pub pPrivateDriverData: *mut core::ffi::c_void,
    pub hHwQueue: super::d3dukmdt::D3DKMT_HANDLE,
    pub hHwQueueProgressFence: super::d3dukmdt::D3DKMT_HANDLE,
    pub HwQueueProgressFenceCPUVirtualAddress: *mut core::ffi::c_void,
    pub HwQueueProgressFenceGPUVirtualAddress: super::d3dukmdt::D3DGPU_VIRTUAL_ADDRESS,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_CREATEHWQUEUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_CREATEHWQUEUEFORUSERMODESUBMISSION {
    pub hHwContext: super::d3dukmdt::D3DKMT_HANDLE,
    pub Flags: super::d3dukmdt::D3DDDI_CREATEHWQUEUEFORUSERMODESUBMISSION_FLAGS,
    pub hRingBuffer: super::d3dukmdt::D3DKMT_HANDLE,
    pub hRingBufferControl: super::d3dukmdt::D3DKMT_HANDLE,
    pub PrivateDriverData: [u8; 64],
    pub hHwQueue: super::d3dukmdt::D3DKMT_HANDLE,
    pub DoorbellStatusCpuVa: *mut core::ffi::c_void,
    pub hProgressFence: super::d3dukmdt::D3DKMT_HANDLE,
    pub ProgressFenceMapping: super::d3dukmdt::D3DDDI_NATIVEFENCEMAPPING,
    pub ProgressFenceLastQueuedValueCpuVa: *mut core::ffi::c_void,
    pub LogBufferInfo: super::d3dukmdt::D3DDDI_NATIVEFENCELOGDETAIL,
    pub Reserved: [u8; 64],
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_CREATEHWQUEUEFORUSERMODESUBMISSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_CREATEKEYEDMUTEX {
    pub InitialValue: u64,
    pub hSharedHandle: super::d3dukmdt::D3DKMT_HANDLE,
    pub hKeyedMutex: super::d3dukmdt::D3DKMT_HANDLE,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_CREATEKEYEDMUTEX2 {
    pub InitialValue: u64,
    pub hSharedHandle: super::d3dukmdt::D3DKMT_HANDLE,
    pub hKeyedMutex: super::d3dukmdt::D3DKMT_HANDLE,
    pub pPrivateRuntimeData: *mut core::ffi::c_void,
    pub PrivateRuntimeDataSize: u32,
    pub Flags: D3DKMT_CREATEKEYEDMUTEX2_FLAGS,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_CREATEKEYEDMUTEX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CREATEKEYEDMUTEX2_FLAGS {
    pub Anonymous: D3DKMT_CREATEKEYEDMUTEX2_FLAGS_0,
}
impl Default for D3DKMT_CREATEKEYEDMUTEX2_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_CREATEKEYEDMUTEX2_FLAGS_0 {
    pub Anonymous: D3DKMT_CREATEKEYEDMUTEX2_FLAGS_0_0,
    pub Value: u32,
}
impl Default for D3DKMT_CREATEKEYEDMUTEX2_FLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_CREATEKEYEDMUTEX2_FLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_CREATENATIVEFENCE {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub hSyncObject: super::d3dukmdt::D3DKMT_HANDLE,
    pub PrivateDriverData: [u8; 64],
    pub Info: super::d3dukmdt::D3DDDI_NATIVEFENCEINFO,
    pub Flags: D3DKMT_CREATENATIVEFENCE_FLAGS,
    pub Reserved: [u8; 28],
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_CREATENATIVEFENCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CREATENATIVEFENCE_FLAGS {
    pub Anonymous: D3DKMT_CREATENATIVEFENCE_FLAGS_0,
}
impl Default for D3DKMT_CREATENATIVEFENCE_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_CREATENATIVEFENCE_FLAGS_0 {
    pub Anonymous: D3DKMT_CREATENATIVEFENCE_FLAGS_0_0,
    pub Value: u32,
}
impl Default for D3DKMT_CREATENATIVEFENCE_FLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_CREATENATIVEFENCE_FLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_CREATEOVERLAY {
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub OverlayInfo: super::d3dukmdt::D3DDDI_KERNELOVERLAYINFO,
    pub hOverlay: super::d3dukmdt::D3DKMT_HANDLE,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_CREATEPAGINGQUEUE {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub Priority: super::d3dukmdt::D3DDDI_PAGINGQUEUE_PRIORITY,
    pub hPagingQueue: super::d3dukmdt::D3DKMT_HANDLE,
    pub hSyncObject: super::d3dukmdt::D3DKMT_HANDLE,
    pub FenceValueCPUVirtualAddress: *mut core::ffi::c_void,
    pub PhysicalAdapterIndex: u32,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_CREATEPAGINGQUEUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_CREATEPROTECTEDSESSION {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub hSyncObject: super::d3dukmdt::D3DKMT_HANDLE,
    pub pPrivateDriverData: *const core::ffi::c_void,
    pub PrivateDriverDataSize: u32,
    pub pPrivateRuntimeData: *const core::ffi::c_void,
    pub PrivateRuntimeDataSize: u32,
    pub hHandle: super::d3dukmdt::D3DKMT_HANDLE,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_CREATEPROTECTEDSESSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_CREATESTANDARDALLOCATION {
    pub Type: D3DKMT_STANDARDALLOCATIONTYPE,
    pub Anonymous: D3DKMT_CREATESTANDARDALLOCATION_0,
    pub Flags: D3DKMT_CREATESTANDARDALLOCATIONFLAGS,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_CREATESTANDARDALLOCATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub union D3DKMT_CREATESTANDARDALLOCATION_0 {
    pub ExistingHeapData: D3DKMT_STANDARDALLOCATION_EXISTINGHEAP,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_CREATESTANDARDALLOCATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CREATESTANDARDALLOCATIONFLAGS {
    pub Anonymous: D3DKMT_CREATESTANDARDALLOCATIONFLAGS_0,
}
impl Default for D3DKMT_CREATESTANDARDALLOCATIONFLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_CREATESTANDARDALLOCATIONFLAGS_0 {
    pub Anonymous: D3DKMT_CREATESTANDARDALLOCATIONFLAGS_0_0,
    pub Value: u32,
}
impl Default for D3DKMT_CREATESTANDARDALLOCATIONFLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_CREATESTANDARDALLOCATIONFLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_CREATESYNCFILE {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub hMonitoredFence: super::d3dukmdt::D3DKMT_HANDLE,
    pub FenceValue: u64,
    pub hSyncFile: u64,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_CREATESYNCHRONIZATIONOBJECT {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub Info: super::d3dukmdt::D3DDDI_SYNCHRONIZATIONOBJECTINFO,
    pub hSyncObject: super::d3dukmdt::D3DKMT_HANDLE,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_CREATESYNCHRONIZATIONOBJECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_CREATESYNCHRONIZATIONOBJECT2 {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub Info: super::d3dukmdt::D3DDDI_SYNCHRONIZATIONOBJECTINFO2,
    pub hSyncObject: super::d3dukmdt::D3DKMT_HANDLE,
}
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_CREATESYNCHRONIZATIONOBJECT2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_CREATE_DOORBELL {
    pub hHwQueue: super::d3dukmdt::D3DKMT_HANDLE,
    pub hRingBuffer: super::d3dukmdt::D3DKMT_HANDLE,
    pub hRingBufferControl: super::d3dukmdt::D3DKMT_HANDLE,
    pub Flags: D3DKMT_CREATE_DOORBELL_FLAGS,
    pub PrivateDriverDataSize: u32,
    pub PrivateDriverData: *mut core::ffi::c_void,
    pub DoorbellCPUVirtualAddress: *mut core::ffi::c_void,
    pub DoorbellSecondaryCPUVirtualAddress: *mut core::ffi::c_void,
    pub DoorbellStatusCPUVirtualAddress: *mut core::ffi::c_void,
    pub HwQueueProgressFenceLastQueuedValueCPUVirtualAddress: *mut core::ffi::c_void,
    pub hDoorbell: super::d3dukmdt::D3DKMT_HANDLE,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_CREATE_DOORBELL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_CREATE_DOORBELL_FLAGS {
    pub Anonymous: D3DKMT_CREATE_DOORBELL_FLAGS_0,
}
impl Default for D3DKMT_CREATE_DOORBELL_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_CREATE_DOORBELL_FLAGS_0 {
    pub Anonymous: D3DKMT_CREATE_DOORBELL_FLAGS_0_0,
    pub Value: u32,
}
impl Default for D3DKMT_CREATE_DOORBELL_FLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_CREATE_DOORBELL_FLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_CREATE_OUTPUTDUPL {
    pub hAdapter: super::d3dukmdt::D3DKMT_HANDLE,
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub KeyedMutexCount: u32,
    pub RequiredKeyedMutexCount: u32,
    pub KeyedMutexs: [D3DKMT_OUTPUTDUPL_KEYEDMUTEX; 3],
    pub Flags: D3DKMT_OUTPUTDUPLCREATIONFLAGS,
}
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_CREATE_OUTPUTDUPL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_CROSSADAPTERRESOURCE_SUPPORT {
    pub SupportTier: D3DKMT_CROSSADAPTERRESOURCE_SUPPORT_TIER,
}
pub type D3DKMT_CROSSADAPTERRESOURCE_SUPPORT_TIER = i32;
pub const D3DKMT_CROSSADAPTERRESOURCE_SUPPORT_TIER_COPY: D3DKMT_CROSSADAPTERRESOURCE_SUPPORT_TIER = 1;
pub const D3DKMT_CROSSADAPTERRESOURCE_SUPPORT_TIER_NONE: D3DKMT_CROSSADAPTERRESOURCE_SUPPORT_TIER = 0;
pub const D3DKMT_CROSSADAPTERRESOURCE_SUPPORT_TIER_SCANOUT: D3DKMT_CROSSADAPTERRESOURCE_SUPPORT_TIER = 3;
pub const D3DKMT_CROSSADAPTERRESOURCE_SUPPORT_TIER_TEXTURE: D3DKMT_CROSSADAPTERRESOURCE_SUPPORT_TIER = 2;
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_CURRENTDISPLAYMODE {
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub DisplayMode: D3DKMT_DISPLAYMODE,
}
pub const D3DKMT_ClientPagingBuffer: D3DKMT_QUERYSTATISTICS_DMA_PACKET_TYPE = 1;
pub const D3DKMT_ClientRenderBuffer: D3DKMT_QUERYSTATISTICS_DMA_PACKET_TYPE = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_DEBUG_SNAPSHOT_ESCAPE {
    pub Length: u32,
    pub Buffer: [u8; 1],
}
impl Default for D3DKMT_DEBUG_SNAPSHOT_ESCAPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3DKMT_DEFRAG_ESCAPE_DEFRAG_DOWNWARD: D3DKMT_DEFRAG_ESCAPE_OPERATION = 2;
pub const D3DKMT_DEFRAG_ESCAPE_DEFRAG_PASS: D3DKMT_DEFRAG_ESCAPE_OPERATION = 3;
pub const D3DKMT_DEFRAG_ESCAPE_DEFRAG_UPWARD: D3DKMT_DEFRAG_ESCAPE_OPERATION = 1;
pub const D3DKMT_DEFRAG_ESCAPE_GET_FRAGMENTATION_STATS: D3DKMT_DEFRAG_ESCAPE_OPERATION = 0;
pub type D3DKMT_DEFRAG_ESCAPE_OPERATION = i32;
pub const D3DKMT_DEFRAG_ESCAPE_VERIFY_TRANSFER: D3DKMT_DEFRAG_ESCAPE_OPERATION = 4;
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_DESTROYALLOCATION {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub hResource: super::d3dukmdt::D3DKMT_HANDLE,
    pub phAllocationList: *const super::d3dukmdt::D3DKMT_HANDLE,
    pub AllocationCount: u32,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_DESTROYALLOCATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_DESTROYALLOCATION2 {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub hResource: super::d3dukmdt::D3DKMT_HANDLE,
    pub phAllocationList: *const super::d3dukmdt::D3DKMT_HANDLE,
    pub AllocationCount: u32,
    pub Flags: super::d3dukmdt::D3DDDICB_DESTROYALLOCATION2FLAGS,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_DESTROYALLOCATION2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_DESTROYCONTEXT {
    pub hContext: super::d3dukmdt::D3DKMT_HANDLE,
}
#[repr(C)]
#[cfg(all(feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_DESTROYDCFROMMEMORY {
    pub hDc: super::windef::HDC,
    pub hBitmap: super::winnt::HANDLE,
}
#[cfg(all(feature = "windef", feature = "winnt"))]
impl Default for D3DKMT_DESTROYDCFROMMEMORY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_DESTROYDEVICE {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_DESTROYHWCONTEXT {
    pub hHwContext: super::d3dukmdt::D3DKMT_HANDLE,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_DESTROYHWQUEUE {
    pub hHwQueue: super::d3dukmdt::D3DKMT_HANDLE,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_DESTROYKEYEDMUTEX {
    pub hKeyedMutex: super::d3dukmdt::D3DKMT_HANDLE,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_DESTROYOVERLAY {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub hOverlay: super::d3dukmdt::D3DKMT_HANDLE,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_DESTROYPROTECTEDSESSION {
    pub hHandle: super::d3dukmdt::D3DKMT_HANDLE,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_DESTROYSYNCHRONIZATIONOBJECT {
    pub hSyncObject: super::d3dukmdt::D3DKMT_HANDLE,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_DESTROY_DOORBELL {
    pub hDoorbell: super::d3dukmdt::D3DKMT_HANDLE,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_DESTROY_OUTPUTDUPL {
    pub hAdapter: super::d3dukmdt::D3DKMT_HANDLE,
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub bDestroyAllContexts: windows_sys::core::BOOL,
}
pub const D3DKMT_DEVICEESCAPE_RESTOREGAMMA: D3DKMT_DEVICEESCAPE_TYPE = 1;
pub type D3DKMT_DEVICEESCAPE_TYPE = i32;
pub const D3DKMT_DEVICEESCAPE_VIDPNFROMALLOCATION: D3DKMT_DEVICEESCAPE_TYPE = 0;
pub const D3DKMT_DEVICEEXECUTION_ACTIVE: D3DKMT_DEVICEEXECUTION_STATE = 1;
pub const D3DKMT_DEVICEEXECUTION_ERROR_DMAFAULT: D3DKMT_DEVICEEXECUTION_STATE = 6;
pub const D3DKMT_DEVICEEXECUTION_ERROR_DMAPAGEFAULT: D3DKMT_DEVICEEXECUTION_STATE = 7;
pub const D3DKMT_DEVICEEXECUTION_ERROR_OUTOFMEMORY: D3DKMT_DEVICEEXECUTION_STATE = 5;
pub const D3DKMT_DEVICEEXECUTION_HUNG: D3DKMT_DEVICEEXECUTION_STATE = 3;
pub const D3DKMT_DEVICEEXECUTION_RESET: D3DKMT_DEVICEEXECUTION_STATE = 2;
pub type D3DKMT_DEVICEEXECUTION_STATE = i32;
pub const D3DKMT_DEVICEEXECUTION_STOPPED: D3DKMT_DEVICEEXECUTION_STATE = 4;
#[repr(C)]
#[cfg(all(feature = "d3dkmdt", feature = "d3dukmdt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_DEVICEPAGEFAULT_STATE {
    pub FaultedPrimitiveAPISequenceNumber: u64,
    pub FaultedPipelineStage: super::d3dkmdt::DXGK_RENDER_PIPELINE_STAGE,
    pub FaultedBindTableEntry: u32,
    pub PageFaultFlags: super::d3dkmdt::DXGK_PAGE_FAULT_FLAGS,
    pub FaultErrorCode: super::d3dkmdt::DXGK_FAULT_ERROR_CODE,
    pub FaultedVirtualAddress: super::d3dukmdt::D3DGPU_VIRTUAL_ADDRESS,
}
#[cfg(all(feature = "d3dkmdt", feature = "d3dukmdt"))]
impl Default for D3DKMT_DEVICEPAGEFAULT_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_DEVICEPRESENT_QUEUE_STATE {
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub bQueuedPresentLimitReached: bool,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_DEVICEPRESENT_STATE {
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub PresentStats: D3DKMT_PRESENT_STATS,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_DEVICEPRESENT_STATE_DWM {
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub PresentStatsDWM: D3DKMT_PRESENT_STATS_DWM,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_DEVICERESET_STATE {
    pub Anonymous: D3DKMT_DEVICERESET_STATE_0,
}
impl Default for D3DKMT_DEVICERESET_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_DEVICERESET_STATE_0 {
    pub Anonymous: D3DKMT_DEVICERESET_STATE_0_0,
    pub Value: u32,
}
impl Default for D3DKMT_DEVICERESET_STATE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_DEVICERESET_STATE_0_0 {
    pub _bitfield: u32,
}
pub const D3DKMT_DEVICESTATE_EXECUTION: D3DKMT_DEVICESTATE_TYPE = 1;
pub const D3DKMT_DEVICESTATE_PAGE_FAULT: D3DKMT_DEVICESTATE_TYPE = 5;
pub const D3DKMT_DEVICESTATE_PRESENT: D3DKMT_DEVICESTATE_TYPE = 2;
pub const D3DKMT_DEVICESTATE_PRESENT_DWM: D3DKMT_DEVICESTATE_TYPE = 4;
pub const D3DKMT_DEVICESTATE_PRESENT_QUEUE: D3DKMT_DEVICESTATE_TYPE = 6;
pub const D3DKMT_DEVICESTATE_RESET: D3DKMT_DEVICESTATE_TYPE = 3;
pub type D3DKMT_DEVICESTATE_TYPE = i32;
pub type D3DKMT_DEVICE_ERROR_REASON = i32;
pub const D3DKMT_DEVICE_ERROR_REASON_DRIVER_ERROR: D3DKMT_DEVICE_ERROR_REASON = -2147483642;
pub const D3DKMT_DEVICE_ERROR_REASON_GENERIC: D3DKMT_DEVICE_ERROR_REASON = -2147483648;
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_DEVICE_ESCAPE {
    pub Type: D3DKMT_DEVICEESCAPE_TYPE,
    pub Anonymous: D3DKMT_DEVICE_ESCAPE_0,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_DEVICE_ESCAPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub union D3DKMT_DEVICE_ESCAPE_0 {
    pub VidPnFromAllocation: D3DKMT_DEVICE_ESCAPE_0_0,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_DEVICE_ESCAPE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_DEVICE_ESCAPE_0_0 {
    pub hPrimaryAllocation: super::d3dukmdt::D3DKMT_HANDLE,
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_DEVICE_IDS {
    pub VendorID: u32,
    pub DeviceID: u32,
    pub SubVendorID: u32,
    pub SubSystemID: u32,
    pub RevisionID: u32,
    pub BusType: u32,
}
pub const D3DKMT_DEVICE_MARKED_AS_ERROR_DEVICE_HUNG: D3DKMT_DEVICE_MARKED_AS_ERROR_FAILURE_CODE = 7;
pub const D3DKMT_DEVICE_MARKED_AS_ERROR_DEVICE_NOT_RECOVERED: D3DKMT_DEVICE_MARKED_AS_ERROR_FAILURE_CODE = 1;
pub const D3DKMT_DEVICE_MARKED_AS_ERROR_DEVICE_RESET: D3DKMT_DEVICE_MARKED_AS_ERROR_FAILURE_CODE = 22;
pub const D3DKMT_DEVICE_MARKED_AS_ERROR_DEVICE_TERMINATED_FOR_PROCESS_CLEANUP: D3DKMT_DEVICE_MARKED_AS_ERROR_FAILURE_CODE = 14;
pub const D3DKMT_DEVICE_MARKED_AS_ERROR_DISPLAY_DISCONNECT: D3DKMT_DEVICE_MARKED_AS_ERROR_FAILURE_CODE = 26;
pub const D3DKMT_DEVICE_MARKED_AS_ERROR_DRIVER_FAULTED: D3DKMT_DEVICE_MARKED_AS_ERROR_FAILURE_CODE = 6;
pub const D3DKMT_DEVICE_MARKED_AS_ERROR_EVICTING_WHILE_IN_USE: D3DKMT_DEVICE_MARKED_AS_ERROR_FAILURE_CODE = 15;
pub const D3DKMT_DEVICE_MARKED_AS_ERROR_EVICT_VALIDATION_FAILURE: D3DKMT_DEVICE_MARKED_AS_ERROR_FAILURE_CODE = 11;
pub const D3DKMT_DEVICE_MARKED_AS_ERROR_FAILED_ASYNC_VMBUS_COMMAND: D3DKMT_DEVICE_MARKED_AS_ERROR_FAILURE_CODE = 25;
pub const D3DKMT_DEVICE_MARKED_AS_ERROR_FAILED_MOVE_PAGING: D3DKMT_DEVICE_MARKED_AS_ERROR_FAILURE_CODE = 24;
pub const D3DKMT_DEVICE_MARKED_AS_ERROR_FAILED_MOVE_VA_COMMIT: D3DKMT_DEVICE_MARKED_AS_ERROR_FAILURE_CODE = 23;
pub const D3DKMT_DEVICE_MARKED_AS_ERROR_FAILED_PROBE_AND_LOCK_TRANSFER: D3DKMT_DEVICE_MARKED_AS_ERROR_FAILURE_CODE = 13;
pub const D3DKMT_DEVICE_MARKED_AS_ERROR_FAILED_SUBMIT_COMMAND_VIRTUAL: D3DKMT_DEVICE_MARKED_AS_ERROR_FAILURE_CODE = 19;
pub const D3DKMT_DEVICE_MARKED_AS_ERROR_FAILED_TO_PAGE_IN_REQUIRED_RESOURCE: D3DKMT_DEVICE_MARKED_AS_ERROR_FAILURE_CODE = 12;
pub const D3DKMT_DEVICE_MARKED_AS_ERROR_FAILED_UPDATE_ALLOCATION_PROPERTY: D3DKMT_DEVICE_MARKED_AS_ERROR_FAILURE_CODE = 21;
pub type D3DKMT_DEVICE_MARKED_AS_ERROR_FAILURE_CODE = i32;
pub const D3DKMT_DEVICE_MARKED_AS_ERROR_FAILURE_CODE_KERNEL_MAX: D3DKMT_DEVICE_MARKED_AS_ERROR_FAILURE_CODE = 28;
pub const D3DKMT_DEVICE_MARKED_AS_ERROR_FLIP_TO_ADDRESS: D3DKMT_DEVICE_MARKED_AS_ERROR_FAILURE_CODE = 5;
pub const D3DKMT_DEVICE_MARKED_AS_ERROR_FROM_USER_MODE: D3DKMT_DEVICE_MARKED_AS_ERROR_FAILURE_CODE = -2147483648;
pub const D3DKMT_DEVICE_MARKED_AS_ERROR_GPU_VA_COMMITMENT_ERROR: D3DKMT_DEVICE_MARKED_AS_ERROR_FAILURE_CODE = 27;
pub const D3DKMT_DEVICE_MARKED_AS_ERROR_HARDWARE_PAGE_FAULT: D3DKMT_DEVICE_MARKED_AS_ERROR_FAILURE_CODE = 9;
pub const D3DKMT_DEVICE_MARKED_AS_ERROR_HISTORY_BUFFER_NOT_RESIDENT: D3DKMT_DEVICE_MARKED_AS_ERROR_FAILURE_CODE = 17;
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_DEVICE_MARKED_AS_ERROR_INFO {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub DumpAssociationGUID: windows_sys::core::GUID,
    pub FailureCode: D3DKMT_DEVICE_MARKED_AS_ERROR_FAILURE_CODE,
}
pub const D3DKMT_DEVICE_MARKED_AS_ERROR_INVALID_RECLAIM_USAGE: D3DKMT_DEVICE_MARKED_AS_ERROR_FAILURE_CODE = 20;
pub const D3DKMT_DEVICE_MARKED_AS_ERROR_MISSED_PAGING_FENCE_SYNCHRONIZATION_FOR_RESIDENT_ALLOCATION: D3DKMT_DEVICE_MARKED_AS_ERROR_FAILURE_CODE = 18;
pub const D3DKMT_DEVICE_MARKED_AS_ERROR_NO_ERROR: D3DKMT_DEVICE_MARKED_AS_ERROR_FAILURE_CODE = 0;
pub const D3DKMT_DEVICE_MARKED_AS_ERROR_PREPARE_DMA_BUFFER: D3DKMT_DEVICE_MARKED_AS_ERROR_FAILURE_CODE = 2;
pub const D3DKMT_DEVICE_MARKED_AS_ERROR_PREPARE_DMA_BUFFER_SPLIT_PACKET_PREEMPTED: D3DKMT_DEVICE_MARKED_AS_ERROR_FAILURE_CODE = 8;
pub const D3DKMT_DEVICE_MARKED_AS_ERROR_PREPARE_DMA_BUFFER_TRY_AGAIN_LATER: D3DKMT_DEVICE_MARKED_AS_ERROR_FAILURE_CODE = 3;
pub const D3DKMT_DEVICE_MARKED_AS_ERROR_PROCESS_DEFERRED_COMMAND: D3DKMT_DEVICE_MARKED_AS_ERROR_FAILURE_CODE = 4;
pub const D3DKMT_DEVICE_MARKED_AS_ERROR_SOFTWARE_PAGE_FAULT: D3DKMT_DEVICE_MARKED_AS_ERROR_FAILURE_CODE = 10;
pub const D3DKMT_DEVICE_MARKED_AS_ERROR_SUBMITTING_RENDER_FOR_NON_RESIDENT_ALLOCATION: D3DKMT_DEVICE_MARKED_AS_ERROR_FAILURE_CODE = 16;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_DIRECTFLIP_SUPPORT {
    pub Supported: windows_sys::core::BOOL,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct D3DKMT_DIRTYREGIONS {
    pub NumRects: u32,
    pub Rects: [super::windef::RECT; 16],
}
#[cfg(feature = "windef")]
impl Default for D3DKMT_DIRTYREGIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_DISPLAYMODE {
    pub Width: u32,
    pub Height: u32,
    pub Format: super::d3dukmdt::D3DDDIFORMAT,
    pub IntegerRefreshRate: u32,
    pub RefreshRate: super::d3dukmdt::D3DDDI_RATIONAL,
    pub ScanLineOrdering: super::d3dukmdt::D3DDDI_VIDEO_SIGNAL_SCANLINE_ORDERING,
    pub DisplayOrientation: super::d3dukmdt::D3DDDI_ROTATION,
    pub DisplayFixedOutput: u32,
    pub Flags: D3DKMDT_DISPLAYMODE_FLAGS,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_DISPLAYMODELIST {
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub ModeCount: u32,
    pub pModeList: [D3DKMT_DISPLAYMODE; 0],
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_DISPLAYMODELIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_DISPLAY_UMD_FILENAMEINFO {
    pub Version: KMT_DISPLAY_UMD_VERSION,
    pub UmdFileName: [u16; 260],
}
impl Default for D3DKMT_DISPLAY_UMD_FILENAMEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_DLIST_DRIVER_NAME {
    pub DListFileName: [u16; 260],
}
impl Default for D3DKMT_DLIST_DRIVER_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3DKMT_DMMESCAPETYPE = i32;
pub const D3DKMT_DMMESCAPETYPE_ACTIVEVIDPN_COFUNCPATHMODALITY_INFO: D3DKMT_DMMESCAPETYPE = 11;
pub const D3DKMT_DMMESCAPETYPE_ACTIVEVIDPN_SOURCEMODESET_INFO: D3DKMT_DMMESCAPETYPE = 10;
pub const D3DKMT_DMMESCAPETYPE_GET_ACTIVEVIDPN_INFO: D3DKMT_DMMESCAPETYPE = 4;
pub const D3DKMT_DMMESCAPETYPE_GET_LASTCLIENTCOMMITTEDVIDPN_INFO: D3DKMT_DMMESCAPETYPE = 12;
pub const D3DKMT_DMMESCAPETYPE_GET_MONITORS_INFO: D3DKMT_DMMESCAPETYPE = 5;
pub const D3DKMT_DMMESCAPETYPE_GET_SUMMARY_INFO: D3DKMT_DMMESCAPETYPE = 1;
pub const D3DKMT_DMMESCAPETYPE_GET_VERSION_INFO: D3DKMT_DMMESCAPETYPE = 13;
pub const D3DKMT_DMMESCAPETYPE_GET_VIDEO_PRESENT_SOURCES_INFO: D3DKMT_DMMESCAPETYPE = 2;
pub const D3DKMT_DMMESCAPETYPE_GET_VIDEO_PRESENT_TARGETS_INFO: D3DKMT_DMMESCAPETYPE = 3;
pub const D3DKMT_DMMESCAPETYPE_RECENTLY_COMMITTED_VIDPNS_INFO: D3DKMT_DMMESCAPETYPE = 6;
pub const D3DKMT_DMMESCAPETYPE_RECENTLY_RECOMMENDED_VIDPNS_INFO: D3DKMT_DMMESCAPETYPE = 8;
pub const D3DKMT_DMMESCAPETYPE_RECENT_MODECHANGE_REQUESTS_INFO: D3DKMT_DMMESCAPETYPE = 7;
pub const D3DKMT_DMMESCAPETYPE_RECENT_MONITOR_PRESENCE_EVENTS_INFO: D3DKMT_DMMESCAPETYPE = 9;
pub const D3DKMT_DMMESCAPETYPE_UNINITIALIZED: D3DKMT_DMMESCAPETYPE = 0;
pub const D3DKMT_DMMESCAPETYPE_VIDPN_MGR_DIAGNOSTICS: D3DKMT_DMMESCAPETYPE = 14;
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_DMM_ESCAPE {
    pub Type: D3DKMT_DMMESCAPETYPE,
    pub ProvidedBufferSize: super::d3dukmdt::D3DKMT_SIZE_T,
    pub MinRequiredBufferSize: super::d3dukmdt::D3DKMT_SIZE_T,
    pub Data: [u8; 1],
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_DMM_ESCAPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_DOD_SET_DIRTYRECT_MODE {
    pub bForceFullScreenDirty: windows_sys::core::BOOL,
}
pub type D3DKMT_DRIVERVERSION = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_DRIVER_DESCRIPTION {
    pub DriverDescription: [u16; 4096],
}
impl Default for D3DKMT_DRIVER_DESCRIPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3DKMT_DeferredCommandBuffer: D3DKMT_QUERYSTATISTICS_QUEUE_PACKET_TYPE = 1;
pub const D3DKMT_DeviceCommandBuffer: D3DKMT_QUERYSTATISTICS_QUEUE_PACKET_TYPE = 6;
pub const D3DKMT_DmaPacketTypeMax: D3DKMT_QUERYSTATISTICS_DMA_PACKET_TYPE = 4;
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_ENUMADAPTERS {
    pub NumAdapters: u32,
    pub Adapters: [D3DKMT_ADAPTERINFO; 16],
}
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_ENUMADAPTERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_ENUMADAPTERS2 {
    pub NumAdapters: u32,
    pub pAdapters: *mut D3DKMT_ADAPTERINFO,
}
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_ENUMADAPTERS2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_ENUMADAPTERS3 {
    pub Filter: D3DKMT_ENUMADAPTERS_FILTER,
    pub NumAdapters: u32,
    pub pAdapters: *mut D3DKMT_ADAPTERINFO,
}
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_ENUMADAPTERS3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_ENUMADAPTERS_FILTER {
    pub Anonymous: D3DKMT_ENUMADAPTERS_FILTER_0,
    pub Value: u64,
}
impl Default for D3DKMT_ENUMADAPTERS_FILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_ENUMADAPTERS_FILTER_0 {
    pub _bitfield: u64,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_ESCAPE {
    pub hAdapter: super::d3dukmdt::D3DKMT_HANDLE,
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub Type: D3DKMT_ESCAPETYPE,
    pub Flags: super::d3dukmdt::D3DDDI_ESCAPEFLAGS,
    pub pPrivateDriverData: *mut core::ffi::c_void,
    pub PrivateDriverDataSize: u32,
    pub hContext: super::d3dukmdt::D3DKMT_HANDLE,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_ESCAPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3DKMT_ESCAPETYPE = i32;
pub const D3DKMT_ESCAPE_ACTIVATE_SPECIFIC_DIAG: D3DKMT_ESCAPETYPE = 14;
pub const D3DKMT_ESCAPE_ADAPTER_VERIFIER_OPTION: D3DKMT_ESCAPETYPE = 29;
pub const D3DKMT_ESCAPE_BDD_FALLBACK: D3DKMT_ESCAPETYPE = 13;
pub const D3DKMT_ESCAPE_BDD_PNP: D3DKMT_ESCAPETYPE = 12;
pub const D3DKMT_ESCAPE_BRIGHTNESS: D3DKMT_ESCAPETYPE = 17;
pub const D3DKMT_ESCAPE_CCD_DATABASE: D3DKMT_ESCAPETYPE = 38;
pub type D3DKMT_ESCAPE_COPY_CONTENT_DIRECTION = i32;
pub const D3DKMT_ESCAPE_COPY_CONTENT_DIRECTION_FROM_ALLOCATION: D3DKMT_ESCAPE_COPY_CONTENT_DIRECTION = 1;
pub const D3DKMT_ESCAPE_COPY_CONTENT_DIRECTION_TO_ALLOCATION: D3DKMT_ESCAPE_COPY_CONTENT_DIRECTION = 0;
pub const D3DKMT_ESCAPE_DEBUG_SNAPSHOT: D3DKMT_ESCAPETYPE = 6;
pub const D3DKMT_ESCAPE_DEVICE: D3DKMT_ESCAPETYPE = 4;
pub const D3DKMT_ESCAPE_DIAGNOSTICS: D3DKMT_ESCAPETYPE = 9;
pub const D3DKMT_ESCAPE_DMM: D3DKMT_ESCAPETYPE = 5;
pub const D3DKMT_ESCAPE_DOD_SET_DIRTYRECT_MODE: D3DKMT_ESCAPETYPE = 31;
pub const D3DKMT_ESCAPE_DRIVERPRIVATE: D3DKMT_ESCAPETYPE = 0;
pub const D3DKMT_ESCAPE_DRT_TEST: D3DKMT_ESCAPETYPE = 8;
pub const D3DKMT_ESCAPE_EDID_CACHE: D3DKMT_ESCAPETYPE = 18;
pub const D3DKMT_ESCAPE_FORCE_BDDFALLBACK_HEADLESS: D3DKMT_ESCAPETYPE = 24;
pub const D3DKMT_ESCAPE_GET_DISPLAY_CONFIGURATIONS: D3DKMT_ESCAPETYPE = 36;
pub const D3DKMT_ESCAPE_GET_EXTERNAL_DIAGNOSTICS: D3DKMT_ESCAPETYPE = 34;
pub const D3DKMT_ESCAPE_HISTORY_BUFFER_STATUS: D3DKMT_ESCAPETYPE = 21;
pub const D3DKMT_ESCAPE_IDD_REQUEST: D3DKMT_ESCAPETYPE = 30;
pub const D3DKMT_ESCAPE_LOG_CODEPOINT_PACKET: D3DKMT_ESCAPETYPE = 32;
pub const D3DKMT_ESCAPE_LOG_USERMODE_DAIG_PACKET: D3DKMT_ESCAPETYPE = 33;
pub const D3DKMT_ESCAPE_MIRACAST_ADAPTER_DIAG_INFO: D3DKMT_ESCAPETYPE = 23;
pub const D3DKMT_ESCAPE_MIRACAST_DISPLAY_REQUEST: D3DKMT_ESCAPETYPE = 20;
pub const D3DKMT_ESCAPE_MODES_PRUNED_OUT: D3DKMT_ESCAPETYPE = 15;
pub const D3DKMT_ESCAPE_OUTPUTDUPL_DIAGNOSTICS: D3DKMT_ESCAPETYPE = 11;
pub const D3DKMT_ESCAPE_OUTPUTDUPL_SNAPSHOT: D3DKMT_ESCAPETYPE = 10;
pub type D3DKMT_ESCAPE_PFN_CONTROL_COMMAND = i32;
pub const D3DKMT_ESCAPE_PFN_CONTROL_DEFAULT: D3DKMT_ESCAPE_PFN_CONTROL_COMMAND = 0;
pub const D3DKMT_ESCAPE_PFN_CONTROL_FORCE_CPU: D3DKMT_ESCAPE_PFN_CONTROL_COMMAND = 1;
pub const D3DKMT_ESCAPE_PFN_CONTROL_FORCE_GPU: D3DKMT_ESCAPE_PFN_CONTROL_COMMAND = 2;
pub const D3DKMT_ESCAPE_PROCESS_VERIFIER_OPTION: D3DKMT_ESCAPETYPE = 28;
pub const D3DKMT_ESCAPE_QUERY_DMA_REMAPPING_STATUS: D3DKMT_ESCAPETYPE = 39;
pub const D3DKMT_ESCAPE_QUERY_IOMMU_STATUS: D3DKMT_ESCAPETYPE = 37;
pub const D3DKMT_ESCAPE_QUERY_PHYSICAL_ADAPTER: D3DKMT_ESCAPETYPE = 40;
pub const D3DKMT_ESCAPE_REQUEST_MACHINE_CRASH: D3DKMT_ESCAPETYPE = 25;
pub const D3DKMT_ESCAPE_SOFTGPU_ENABLE_DISABLE_HMD: D3DKMT_ESCAPETYPE = 27;
pub const D3DKMT_ESCAPE_TDRDBGCTRL: D3DKMT_ESCAPETYPE = 2;
pub const D3DKMT_ESCAPE_VIDMM: D3DKMT_ESCAPETYPE = 1;
pub const D3DKMT_ESCAPE_VIDSCH: D3DKMT_ESCAPETYPE = 3;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_ESCAPE_VIRTUAL_REFRESH_RATE {
    pub Type: D3DKMT_ESCAPE_VIRTUAL_REFRESH_RATE_TYPE,
    pub VidPnSourceId: u32,
    pub ProcessBoostEligible: bool,
    pub VSyncMultiplier: u32,
    pub BaseDesktopDuration: u32,
    pub Reserved: [u8; 16],
}
impl Default for D3DKMT_ESCAPE_VIRTUAL_REFRESH_RATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3DKMT_ESCAPE_VIRTUAL_REFRESH_RATE_TYPE = i32;
pub const D3DKMT_ESCAPE_VIRTUAL_REFRESH_RATE_TYPE_SET_BASE_DESKTOP_DURATION: D3DKMT_ESCAPE_VIRTUAL_REFRESH_RATE_TYPE = 0;
pub const D3DKMT_ESCAPE_VIRTUAL_REFRESH_RATE_TYPE_SET_PROCESS_BOOST_ELIGIBLE: D3DKMT_ESCAPE_VIRTUAL_REFRESH_RATE_TYPE = 2;
pub const D3DKMT_ESCAPE_VIRTUAL_REFRESH_RATE_TYPE_SET_VSYNC_MULTIPLIER: D3DKMT_ESCAPE_VIRTUAL_REFRESH_RATE_TYPE = 1;
pub const D3DKMT_ESCAPE_WHQL_INFO: D3DKMT_ESCAPETYPE = 16;
pub const D3DKMT_ESCAPE_WIN32K_BDD_FALLBACK: D3DKMT_ESCAPETYPE = 1029;
pub const D3DKMT_ESCAPE_WIN32K_COLOR_PROFILE_INFO: D3DKMT_ESCAPETYPE = 1036;
pub const D3DKMT_ESCAPE_WIN32K_DDA_TEST_CTL: D3DKMT_ESCAPETYPE = 1030;
pub const D3DKMT_ESCAPE_WIN32K_DISPBROKER_TEST: D3DKMT_ESCAPETYPE = 1035;
pub const D3DKMT_ESCAPE_WIN32K_DPI_INFO: D3DKMT_ESCAPETYPE = 1026;
pub const D3DKMT_ESCAPE_WIN32K_HIP_DEVICE_INFO: D3DKMT_ESCAPETYPE = 1024;
pub const D3DKMT_ESCAPE_WIN32K_PRESENTER_VIEW_INFO: D3DKMT_ESCAPETYPE = 1027;
pub const D3DKMT_ESCAPE_WIN32K_QUERY_CD_ROTATION_BLOCK: D3DKMT_ESCAPETYPE = 1025;
pub const D3DKMT_ESCAPE_WIN32K_SET_DIMMED_STATE: D3DKMT_ESCAPETYPE = 1037;
pub const D3DKMT_ESCAPE_WIN32K_SPECIALIZED_DISPLAY_TEST: D3DKMT_ESCAPETYPE = 1038;
pub const D3DKMT_ESCAPE_WIN32K_START: D3DKMT_ESCAPETYPE = 1024;
pub const D3DKMT_ESCAPE_WIN32K_SYSTEM_DPI: D3DKMT_ESCAPETYPE = 1028;
pub const D3DKMT_ESCAPE_WIN32K_USER_DETECTED_BLACK_SCREEN: D3DKMT_ESCAPETYPE = 1031;
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_EVICT {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub NumAllocations: u32,
    pub AllocationList: *const super::d3dukmdt::D3DKMT_HANDLE,
    pub Flags: super::d3dukmdt::D3DDDI_EVICT_FLAGS,
    pub NumBytesToTrim: u64,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_EVICT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_EVICTION_CRITERIA {
    pub MinimumSize: u64,
    pub MaximumSize: u64,
    pub Anonymous: D3DKMT_EVICTION_CRITERIA_0,
}
impl Default for D3DKMT_EVICTION_CRITERIA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_EVICTION_CRITERIA_0 {
    pub Anonymous: D3DKMT_EVICTION_CRITERIA_0_0,
}
impl Default for D3DKMT_EVICTION_CRITERIA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_EVICTION_CRITERIA_0_0 {
    pub Flags: D3DKMT_EVICTION_CRITERIA_0_0_0,
    pub Value: u32,
}
impl Default for D3DKMT_EVICTION_CRITERIA_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_EVICTION_CRITERIA_0_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_FENCE_PRESENTHISTORYTOKEN {
    pub Key: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_FLIPINFOFLAGS {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_FLIPMANAGER_AUXILIARYPRESENTINFO {
    pub auxiliaryPresentInfo: D3DKMT_AUXILIARYPRESENTINFO,
    pub flipManagerTracingId: u32,
    pub customDurationChanged: windows_sys::core::BOOL,
    pub pFlipManagerProcessedEvent: *mut core::ffi::c_void,
    pub FlipAdapterLuid: super::winnt::LUID,
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub independentFlipStage: D3DKMT_FLIPMODEL_INDEPENDENT_FLIP_STAGE,
    pub FlipCompletedQpc: u64,
    pub HwPresentDurationQpc: u32,
    pub WasCanceled: windows_sys::core::BOOL,
    pub ConvertedToNonIFlip: windows_sys::core::BOOL,
}
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_FLIPMANAGER_AUXILIARYPRESENTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_FLIPMANAGER_PRESENTHISTORYTOKEN {
    pub hPrivateData: u64,
    pub PresentAtQpc: u64,
    pub Flags: D3DKMT_FLIPMANAGER_PRESENTHISTORYTOKEN_0,
}
impl Default for D3DKMT_FLIPMANAGER_PRESENTHISTORYTOKEN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_FLIPMANAGER_PRESENTHISTORYTOKEN_0 {
    pub Anonymous: D3DKMT_FLIPMANAGER_PRESENTHISTORYTOKEN_0_0,
    pub Value: u32,
}
impl Default for D3DKMT_FLIPMANAGER_PRESENTHISTORYTOKEN_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_FLIPMANAGER_PRESENTHISTORYTOKEN_0_0 {
    pub _bitfield: u32,
}
pub type D3DKMT_FLIPMODEL_INDEPENDENT_FLIP_STAGE = i32;
pub const D3DKMT_FLIPMODEL_INDEPENDENT_FLIP_STAGE_FLIP_COMPLETE: D3DKMT_FLIPMODEL_INDEPENDENT_FLIP_STAGE = 1;
pub const D3DKMT_FLIPMODEL_INDEPENDENT_FLIP_STAGE_FLIP_SUBMITTED: D3DKMT_FLIPMODEL_INDEPENDENT_FLIP_STAGE = 0;
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_FLIPMODEL_PRESENTHISTORYTOKEN {
    pub FenceValue: u64,
    pub hLogicalSurface: u64,
    pub dxgContext: super::d3dukmdt::D3DKMT_UINT_PTR,
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub SwapChainIndex: u32,
    pub PresentLimitSemaphoreId: u64,
    pub FlipInterval: super::d3dukmdt::D3DDDI_FLIPINTERVAL_TYPE,
    pub Flags: D3DKMT_FLIPMODEL_PRESENTHISTORYTOKENFLAGS,
    pub hCompSurf: i64,
    pub compSurfLuid: super::winnt::LUID,
    pub confirmationCookie: u64,
    pub CompositionSyncKey: u64,
    pub ScrollRect: super::windef::RECT,
    pub ScrollOffset: super::windef::POINT,
    pub PresentCount: u32,
    pub RevealColor: [f32; 4],
    pub Rotation: super::d3dukmdt::D3DDDI_ROTATION,
    pub Anonymous: D3DKMT_FLIPMODEL_PRESENTHISTORYTOKEN_0,
    pub InkCookie: u32,
    pub SourceRect: super::windef::RECT,
    pub DestWidth: u32,
    pub DestHeight: u32,
    pub TargetRect: super::windef::RECT,
    pub Transform: [f32; 6],
    pub CustomDuration: u32,
    pub CustomDurationFlipInterval: super::d3dukmdt::D3DDDI_FLIPINTERVAL_TYPE,
    pub PlaneIndex: u32,
    pub ColorSpace: super::d3dukmdt::D3DDDI_COLOR_SPACE_TYPE,
    pub DirtyRegions: D3DKMT_DIRTYREGIONS,
}
#[cfg(all(feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
impl Default for D3DKMT_FLIPMODEL_PRESENTHISTORYTOKEN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union D3DKMT_FLIPMODEL_PRESENTHISTORYTOKEN_0 {
    pub ScatterBlts: D3DKMT_SCATTERBLTS,
    pub Anonymous: D3DKMT_FLIPMODEL_PRESENTHISTORYTOKEN_0_0,
}
#[cfg(all(feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
impl Default for D3DKMT_FLIPMODEL_PRESENTHISTORYTOKEN_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_FLIPMODEL_PRESENTHISTORYTOKEN_0_0 {
    pub hSyncObject: super::winnt::HANDLE,
    pub HDRMetaDataType: super::d3dukmdt::D3DDDI_HDR_METADATA_TYPE,
    pub Anonymous: D3DKMT_FLIPMODEL_PRESENTHISTORYTOKEN_0_0_0,
}
#[cfg(all(feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
impl Default for D3DKMT_FLIPMODEL_PRESENTHISTORYTOKEN_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union D3DKMT_FLIPMODEL_PRESENTHISTORYTOKEN_0_0_0 {
    pub HDRMetaDataHDR10: super::d3dukmdt::D3DDDI_HDR_METADATA_HDR10,
    pub HDRMetaDataHDR10Plus: super::d3dukmdt::D3DDDI_HDR_METADATA_HDR10PLUS,
}
#[cfg(all(feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
impl Default for D3DKMT_FLIPMODEL_PRESENTHISTORYTOKEN_0_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_FLIPMODEL_PRESENTHISTORYTOKENFLAGS {
    pub Anonymous: D3DKMT_FLIPMODEL_PRESENTHISTORYTOKENFLAGS_0,
}
impl Default for D3DKMT_FLIPMODEL_PRESENTHISTORYTOKENFLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_FLIPMODEL_PRESENTHISTORYTOKENFLAGS_0 {
    pub Anonymous: D3DKMT_FLIPMODEL_PRESENTHISTORYTOKENFLAGS_0_0,
    pub Value: u32,
}
impl Default for D3DKMT_FLIPMODEL_PRESENTHISTORYTOKENFLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_FLIPMODEL_PRESENTHISTORYTOKENFLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_FLIPOVERLAY {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub hOverlay: super::d3dukmdt::D3DKMT_HANDLE,
    pub hSource: super::d3dukmdt::D3DKMT_HANDLE,
    pub pPrivateDriverData: *mut core::ffi::c_void,
    pub PrivateDriverDataSize: u32,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_FLIPOVERLAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_FLIPQUEUEINFO {
    pub MaxHardwareFlipQueueLength: u32,
    pub MaxSoftwareFlipQueueLength: u32,
    pub FlipFlags: D3DKMT_FLIPINFOFLAGS,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_FLUSHHEAPTRANSITIONS {
    pub hAdapter: super::d3dukmdt::D3DKMT_HANDLE,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_FREEGPUVIRTUALADDRESS {
    pub hAdapter: super::d3dukmdt::D3DKMT_HANDLE,
    pub BaseAddress: super::d3dukmdt::D3DGPU_VIRTUAL_ADDRESS,
    pub Size: super::d3dukmdt::D3DGPU_SIZE_T,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_GDIMODEL_PRESENTHISTORYTOKEN {
    pub hLogicalSurface: u64,
    pub hPhysicalSurface: u64,
    pub ScrollRect: super::windef::RECT,
    pub ScrollOffset: super::windef::POINT,
    pub DirtyRegions: D3DKMT_DIRTYREGIONS,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_GDIMODEL_SYSMEM_PRESENTHISTORYTOKEN {
    pub hlsurf: u64,
    pub dwDirtyFlags: u32,
    pub uiCookie: u64,
}
pub const D3DKMT_GDI_STYLE_HANDLE_DECORATION: u32 = 2;
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_GETALLOCATIONPRIORITY {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub hResource: super::d3dukmdt::D3DKMT_HANDLE,
    pub phAllocationList: *const super::d3dukmdt::D3DKMT_HANDLE,
    pub AllocationCount: u32,
    pub pPriorities: *mut u32,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_GETALLOCATIONPRIORITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_GETCONTEXTINPROCESSSCHEDULINGPRIORITY {
    pub hContext: super::d3dukmdt::D3DKMT_HANDLE,
    pub Priority: i32,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_GETCONTEXTSCHEDULINGPRIORITY {
    pub hContext: super::d3dukmdt::D3DKMT_HANDLE,
    pub Priority: i32,
}
#[repr(C)]
#[cfg(all(feature = "d3dkmdt", feature = "d3dukmdt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_GETDEVICESTATE {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub StateType: D3DKMT_DEVICESTATE_TYPE,
    pub Anonymous: D3DKMT_GETDEVICESTATE_0,
}
#[cfg(all(feature = "d3dkmdt", feature = "d3dukmdt"))]
impl Default for D3DKMT_GETDEVICESTATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dkmdt", feature = "d3dukmdt"))]
#[derive(Clone, Copy)]
pub union D3DKMT_GETDEVICESTATE_0 {
    pub ExecutionState: D3DKMT_DEVICEEXECUTION_STATE,
    pub PresentState: D3DKMT_DEVICEPRESENT_STATE,
    pub ResetState: D3DKMT_DEVICERESET_STATE,
    pub PresentStateDWM: D3DKMT_DEVICEPRESENT_STATE_DWM,
    pub PageFaultState: D3DKMT_DEVICEPAGEFAULT_STATE,
    pub PresentQueueState: D3DKMT_DEVICEPRESENT_QUEUE_STATE,
}
#[cfg(all(feature = "d3dkmdt", feature = "d3dukmdt"))]
impl Default for D3DKMT_GETDEVICESTATE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_GETDISPLAYMODELIST {
    pub hAdapter: super::d3dukmdt::D3DKMT_HANDLE,
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub pModeList: *mut D3DKMT_DISPLAYMODE,
    pub ModeCount: u32,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_GETDISPLAYMODELIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_GETMULTISAMPLEMETHODLIST {
    pub hAdapter: super::d3dukmdt::D3DKMT_HANDLE,
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub Width: u32,
    pub Height: u32,
    pub Format: super::d3dukmdt::D3DDDIFORMAT,
    pub pMethodList: *mut D3DKMT_MULTISAMPLEMETHOD,
    pub MethodCount: u32,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_GETMULTISAMPLEMETHODLIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_GETNATIVEFENCELOGDETAIL {
    pub hHwQueue: super::d3dukmdt::D3DKMT_HANDLE,
    pub Flags: D3DKMT_GETNATIVEFENCELOGDETAIL_FLAGS,
    pub WaitLogNumberOfEntries: u32,
    pub SignalLogNumberOfEntries: u32,
    pub WaitLogGpuBaseAddress: super::d3dukmdt::D3DGPU_VIRTUAL_ADDRESS,
    pub SignalLogGpuBaseAddress: super::d3dukmdt::D3DGPU_VIRTUAL_ADDRESS,
    pub Reserved: [u8; 64],
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_GETNATIVEFENCELOGDETAIL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_GETNATIVEFENCELOGDETAIL_FLAGS {
    pub Anonymous: D3DKMT_GETNATIVEFENCELOGDETAIL_FLAGS_0,
}
impl Default for D3DKMT_GETNATIVEFENCELOGDETAIL_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_GETNATIVEFENCELOGDETAIL_FLAGS_0 {
    pub Anonymous: D3DKMT_GETNATIVEFENCELOGDETAIL_FLAGS_0_0,
    pub Value: u32,
}
impl Default for D3DKMT_GETNATIVEFENCELOGDETAIL_FLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_GETNATIVEFENCELOGDETAIL_FLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_GETOVERLAYSTATE {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub hOverlay: super::d3dukmdt::D3DKMT_HANDLE,
    pub OverlayEnabled: bool,
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_GETPRESENTHISTORY {
    pub hAdapter: super::d3dukmdt::D3DKMT_HANDLE,
    pub ProvidedSize: u32,
    pub WrittenSize: u32,
    pub pTokens: *mut D3DKMT_PRESENTHISTORYTOKEN,
    pub NumTokens: u32,
}
#[cfg(all(feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
impl Default for D3DKMT_GETPRESENTHISTORY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3DKMT_GETPRESENTHISTORY_MAXTOKENS: u32 = 2048;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_GETPROCESSDEVICEREMOVALSUPPORT {
    pub hProcess: super::winnt::HANDLE,
    pub AdapterLuid: super::winnt::LUID,
    pub Support: bool,
}
#[cfg(feature = "winnt")]
impl Default for D3DKMT_GETPROCESSDEVICEREMOVALSUPPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_GETRUNTIMEDATA {
    pub hAdapter: super::d3dukmdt::D3DKMT_HANDLE,
    pub hGlobalShare: super::d3dukmdt::D3DKMT_HANDLE,
    pub pRuntimeData: *mut core::ffi::c_void,
    pub RuntimeDataSize: u32,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_GETRUNTIMEDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_GETSCANLINE {
    pub hAdapter: super::d3dukmdt::D3DKMT_HANDLE,
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub InVerticalBlank: bool,
    pub ScanLine: u32,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_GETSHAREDPRIMARYHANDLE {
    pub hAdapter: super::d3dukmdt::D3DKMT_HANDLE,
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub hSharedPrimary: super::d3dukmdt::D3DKMT_HANDLE,
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_GETSHAREDRESOURCEADAPTERLUID {
    pub hGlobalShare: super::d3dukmdt::D3DKMT_HANDLE,
    pub hNtHandle: super::winnt::HANDLE,
    pub AdapterLuid: super::winnt::LUID,
}
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_GETSHAREDRESOURCEADAPTERLUID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_GETVERTICALBLANKEVENT {
    pub hAdapter: super::d3dukmdt::D3DKMT_HANDLE,
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub phEvent: *mut super::d3dukmdt::D3DKMT_PTR_TYPE,
}
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_GETVERTICALBLANKEVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_GET_DEVICE_VIDPN_OWNERSHIP_INFO {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub bFailedDwmAcquireVidPn: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_GET_GPUMMU_CAPS {
    pub PhysicalAdapterIndex: u32,
    pub GpuMmuCaps: DXGK_ESCAPE_GPUMMUCAPS,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_GET_MULTIPLANE_OVERLAY_CAPS {
    pub hAdapter: super::d3dukmdt::D3DKMT_HANDLE,
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub MaxPlanes: u32,
    pub MaxRGBPlanes: u32,
    pub MaxYUVPlanes: u32,
    pub OverlayCaps: D3DKMT_MULTIPLANE_OVERLAY_CAPS,
    pub MaxStretchFactor: f32,
    pub MaxShrinkFactor: f32,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_GET_MULTIPLANE_OVERLAY_CAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_GET_POST_COMPOSITION_CAPS {
    pub hAdapter: super::d3dukmdt::D3DKMT_HANDLE,
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub MaxStretchFactor: f32,
    pub MaxShrinkFactor: f32,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_GET_PTE {
    pub PhysicalAdapterIndex: u32,
    pub PageTableLevel: u32,
    pub PageTableIndex: [u32; 6],
    pub b64KBPte: bool,
    pub NumPtes: u32,
    pub Pte: [super::d3dukmdt::DXGK_PTE; 64],
    pub NumValidEntries: u32,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_GET_PTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_GET_PTE_EXT {
    pub DriverProtection: [u64; 64],
    pub AllocationData: [u64; 64],
}
impl Default for D3DKMT_GET_PTE_EXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3DKMT_GET_PTE_MAX: u32 = 64;
pub const D3DKMT_GET_QUEUEDLIMIT_PRESENT: D3DKMT_QUEUEDLIMIT_TYPE = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_GET_SEGMENT_CAPS {
    pub PhysicalAdapterIndex: u32,
    pub NumSegments: u32,
    pub SegmentCaps: [D3DKMT_SEGMENT_CAPS; 32],
}
impl Default for D3DKMT_GET_SEGMENT_CAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_GPUMMU_CAPS {
    pub Flags: D3DKMT_GPUMMU_CAPS_0,
    pub VirtualAddressBitCount: u32,
}
impl Default for D3DKMT_GPUMMU_CAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_GPUMMU_CAPS_0 {
    pub Anonymous: D3DKMT_GPUMMU_CAPS_0_0,
    pub Value: u32,
}
impl Default for D3DKMT_GPUMMU_CAPS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_GPUMMU_CAPS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_GPUVERSION {
    pub PhysicalAdapterIndex: u32,
    pub BiosVersion: [u16; 32],
    pub GpuArchitecture: [u16; 32],
}
impl Default for D3DKMT_GPUVERSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3DKMT_GPU_PREFERENCE_QUERY_STATE = i32;
pub type D3DKMT_GPU_PREFERENCE_QUERY_TYPE = i32;
pub const D3DKMT_GPU_PREFERENCE_STATE_HIGH_PERFORMANCE: D3DKMT_GPU_PREFERENCE_QUERY_STATE = 1;
pub const D3DKMT_GPU_PREFERENCE_STATE_MINIMUM_POWER: D3DKMT_GPU_PREFERENCE_QUERY_STATE = 2;
pub const D3DKMT_GPU_PREFERENCE_STATE_NOT_FOUND: D3DKMT_GPU_PREFERENCE_QUERY_STATE = 4;
pub const D3DKMT_GPU_PREFERENCE_STATE_UNINITIALIZED: D3DKMT_GPU_PREFERENCE_QUERY_STATE = 0;
pub const D3DKMT_GPU_PREFERENCE_STATE_UNSPECIFIED: D3DKMT_GPU_PREFERENCE_QUERY_STATE = 3;
pub const D3DKMT_GPU_PREFERENCE_STATE_USER_SPECIFIED_GPU: D3DKMT_GPU_PREFERENCE_QUERY_STATE = 5;
pub const D3DKMT_GPU_PREFERENCE_TYPE_DX_DATABASE: D3DKMT_GPU_PREFERENCE_QUERY_TYPE = 1;
pub const D3DKMT_GPU_PREFERENCE_TYPE_IHV_DLIST: D3DKMT_GPU_PREFERENCE_QUERY_TYPE = 0;
pub const D3DKMT_GPU_PREFERENCE_TYPE_USER_PREFERENCE: D3DKMT_GPU_PREFERENCE_QUERY_TYPE = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_HISTORY_BUFFER_STATUS {
    pub Enabled: bool,
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_HWDRM_SUPPORT {
    pub Supported: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_HYBRID_DLIST_DLL_MUX_SUPPORT {
    pub Supported: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_HYBRID_DLIST_DLL_SUPPORT {
    pub Supported: windows_sys::core::BOOL,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_HYBRID_LIST {
    pub State: D3DKMT_GPU_PREFERENCE_QUERY_STATE,
    pub AdapterLuid: super::winnt::LUID,
    pub bUserPreferenceQuery: windows_sys::core::BOOL,
    pub QueryType: D3DKMT_GPU_PREFERENCE_QUERY_TYPE,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_INDEPENDENTFLIP_SECONDARY_SUPPORT {
    pub Supported: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_INDEPENDENTFLIP_SUPPORT {
    pub Supported: windows_sys::core::BOOL,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_INVALIDATEACTIVEVIDPN {
    pub hAdapter: super::d3dukmdt::D3DKMT_HANDLE,
    pub pPrivateDriverData: *mut core::ffi::c_void,
    pub PrivateDriverDataSize: u32,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_INVALIDATEACTIVEVIDPN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_INVALIDATECACHE {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub hAllocation: super::d3dukmdt::D3DKMT_HANDLE,
    pub Offset: super::d3dukmdt::D3DKMT_SIZE_T,
    pub Length: super::d3dukmdt::D3DKMT_SIZE_T,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_ISBADDRIVERFORHWPROTECTIONDISABLED {
    pub Disabled: windows_sys::core::BOOL,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_ISFEATUREENABLED {
    pub hAdapter: super::d3dukmdt::D3DKMT_HANDLE,
    pub FeatureId: super::d3dukmdt::DXGK_FEATURE_ID,
    pub Result: super::d3dukmdt::DXGK_ISFEATUREENABLED_RESULT,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_ISFEATUREENABLED {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_KMD_DRIVER_VERSION {
    pub DriverVersion: i64,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_LOCK {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub hAllocation: super::d3dukmdt::D3DKMT_HANDLE,
    pub PrivateDriverData: u32,
    pub NumPages: u32,
    pub pPages: *const u32,
    pub pData: *mut core::ffi::c_void,
    pub Flags: super::d3dukmdt::D3DDDICB_LOCKFLAGS,
    pub GpuVirtualAddress: super::d3dukmdt::D3DGPU_VIRTUAL_ADDRESS,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_LOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_LOCK2 {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub hAllocation: super::d3dukmdt::D3DKMT_HANDLE,
    pub Flags: super::d3dukmdt::D3DDDICB_LOCK2FLAGS,
    pub pData: *mut core::ffi::c_void,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_LOCK2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_MAPPROCESSDEBUGBLOB {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub Flags: D3DKMT_MAPPROCESSDEBUGBLOB_FLAGS,
    pub BufferSize: usize,
    pub pBuffer: *mut core::ffi::c_void,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_MAPPROCESSDEBUGBLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_MAPPROCESSDEBUGBLOB_FLAGS {
    pub Anonymous: D3DKMT_MAPPROCESSDEBUGBLOB_FLAGS_0,
}
impl Default for D3DKMT_MAPPROCESSDEBUGBLOB_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_MAPPROCESSDEBUGBLOB_FLAGS_0 {
    pub Anonymous: D3DKMT_MAPPROCESSDEBUGBLOB_FLAGS_0_0,
    pub Value: u32,
}
impl Default for D3DKMT_MAPPROCESSDEBUGBLOB_FLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_MAPPROCESSDEBUGBLOB_FLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_MARKDEVICEASERROR {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub Reason: D3DKMT_DEVICE_ERROR_REASON,
}
pub const D3DKMT_MAX_BUNDLE_OBJECTS_PER_HANDLE: u32 = 16;
pub const D3DKMT_MAX_DMM_ESCAPE_DATASIZE: i32 = 102400;
pub const D3DKMT_MAX_MULTIPLANE_OVERLAY_ALLOCATIONS_PER_PLANE: u32 = 256;
pub const D3DKMT_MAX_MULTIPLANE_OVERLAY_PLANES: u32 = 8;
pub const D3DKMT_MAX_OBJECTS_PER_HANDLE: u32 = 3;
pub const D3DKMT_MAX_PRESENT_HISTORY_RECTS: u32 = 16;
pub const D3DKMT_MAX_PRESENT_HISTORY_SCATTERBLTS: u32 = 12;
pub const D3DKMT_MAX_SEGMENT_COUNT: u32 = 32;
pub const D3DKMT_MAX_WAITFORVERTICALBLANK_OBJECTS: u32 = 8;
pub type D3DKMT_MEMORY_SEGMENT_GROUP = i32;
pub const D3DKMT_MEMORY_SEGMENT_GROUP_LOCAL: D3DKMT_MEMORY_SEGMENT_GROUP = 0;
pub const D3DKMT_MEMORY_SEGMENT_GROUP_NON_LOCAL: D3DKMT_MEMORY_SEGMENT_GROUP = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_MIRACASTCOMPANIONDRIVERNAME {
    pub MiracastCompanionDriverName: [u16; 260],
}
impl Default for D3DKMT_MIRACASTCOMPANIONDRIVERNAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_MIRACAST_CHUNK_DATA {
    pub ChunkInfo: super::d3dukmdt::DXGK_MIRACAST_CHUNK_INFO,
    pub PrivateDriverDataSize: u32,
    pub PrivateDriverData: [u8; 1],
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_MIRACAST_CHUNK_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3DKMT_MIRACAST_DEVICE_STATUS = i32;
pub const D3DKMT_MIRACAST_DEVICE_STATUS_CANCELLED: D3DKMT_MIRACAST_DEVICE_STATUS = -2147483637;
pub const D3DKMT_MIRACAST_DEVICE_STATUS_DEVICE_ERROR: D3DKMT_MIRACAST_DEVICE_STATUS = -2147483645;
pub const D3DKMT_MIRACAST_DEVICE_STATUS_DEVICE_NOT_FOUND: D3DKMT_MIRACAST_DEVICE_STATUS = -2147483642;
pub const D3DKMT_MIRACAST_DEVICE_STATUS_DEVICE_NOT_STARTED: D3DKMT_MIRACAST_DEVICE_STATUS = -2147483641;
pub const D3DKMT_MIRACAST_DEVICE_STATUS_GPU_RESOURCE_IN_USE: D3DKMT_MIRACAST_DEVICE_STATUS = -2147483646;
pub const D3DKMT_MIRACAST_DEVICE_STATUS_INSUFFICIENT_BANDWIDTH: D3DKMT_MIRACAST_DEVICE_STATUS = -2147483639;
pub const D3DKMT_MIRACAST_DEVICE_STATUS_INSUFFICIENT_MEMORY: D3DKMT_MIRACAST_DEVICE_STATUS = -2147483638;
pub const D3DKMT_MIRACAST_DEVICE_STATUS_INVALID_PARAMETER: D3DKMT_MIRACAST_DEVICE_STATUS = -2147483640;
pub const D3DKMT_MIRACAST_DEVICE_STATUS_PENDING: D3DKMT_MIRACAST_DEVICE_STATUS = 2;
pub const D3DKMT_MIRACAST_DEVICE_STATUS_REMOTE_SESSION: D3DKMT_MIRACAST_DEVICE_STATUS = -2147483643;
pub const D3DKMT_MIRACAST_DEVICE_STATUS_SUCCESS: D3DKMT_MIRACAST_DEVICE_STATUS = 0;
pub const D3DKMT_MIRACAST_DEVICE_STATUS_SUCCESS_NO_MONITOR: D3DKMT_MIRACAST_DEVICE_STATUS = 1;
pub const D3DKMT_MIRACAST_DEVICE_STATUS_UNKOWN_ERROR: D3DKMT_MIRACAST_DEVICE_STATUS = -2147483647;
pub const D3DKMT_MIRACAST_DEVICE_STATUS_UNKOWN_PAIRING: D3DKMT_MIRACAST_DEVICE_STATUS = -2147483644;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_MIRACAST_DISPLAY_DEVICE_CAPS {
    pub HdcpSupported: bool,
    pub DefaultControlPort: u32,
    pub UsesIhvSolution: bool,
}
pub type D3DKMT_MIRACAST_DISPLAY_DEVICE_STATE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_MIRACAST_DISPLAY_DEVICE_STATUS {
    pub State: D3DKMT_MIRACAST_DISPLAY_DEVICE_STATE,
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_MIRACAST_DISPLAY_STOP_SESSIONS {
    pub AdapterLuid: super::winnt::LUID,
    pub TargetId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_TARGET_ID,
    pub StopReason: u32,
}
pub const D3DKMT_MIRACAST_DRIVER_IHV: D3DKMT_MIRACAST_DRIVER_TYPE = 1;
pub const D3DKMT_MIRACAST_DRIVER_MS: D3DKMT_MIRACAST_DRIVER_TYPE = 2;
pub const D3DKMT_MIRACAST_DRIVER_NOT_SUPPORTED: D3DKMT_MIRACAST_DRIVER_TYPE = 0;
pub type D3DKMT_MIRACAST_DRIVER_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_MPO3DDI_SUPPORT {
    pub Supported: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_MPOKERNELCAPS_SUPPORT {
    pub Supported: windows_sys::core::BOOL,
}
pub const D3DKMT_MULIIPLANE_OVERLAY_VIDEO_FRAME_FORMAT_PROGRESSIVE: D3DKMT_MULTIPLANE_OVERLAY_VIDEO_FRAME_FORMAT = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_MULTIPLANEOVERLAY_DECODE_SUPPORT {
    pub Supported: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_MULTIPLANEOVERLAY_HUD_SUPPORT {
    pub VidPnSourceId: u32,
    pub Update: windows_sys::core::BOOL,
    pub KernelSupported: windows_sys::core::BOOL,
    pub HudSupported: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_MULTIPLANEOVERLAY_SECONDARY_SUPPORT {
    pub Supported: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_MULTIPLANEOVERLAY_STRETCH_SUPPORT {
    pub VidPnSourceId: u32,
    pub Update: windows_sys::core::BOOL,
    pub Supported: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_MULTIPLANEOVERLAY_SUPPORT {
    pub Supported: windows_sys::core::BOOL,
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_MULTIPLANE_OVERLAY {
    pub LayerIndex: u32,
    pub Enabled: windows_sys::core::BOOL,
    pub hAllocation: super::d3dukmdt::D3DKMT_HANDLE,
    pub PlaneAttributes: D3DKMT_MULTIPLANE_OVERLAY_ATTRIBUTES,
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_MULTIPLANE_OVERLAY2 {
    pub LayerIndex: u32,
    pub Enabled: windows_sys::core::BOOL,
    pub hAllocation: super::d3dukmdt::D3DKMT_HANDLE,
    pub PlaneAttributes: D3DKMT_MULTIPLANE_OVERLAY_ATTRIBUTES2,
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_MULTIPLANE_OVERLAY3 {
    pub LayerIndex: u32,
    pub InputFlags: D3DKMT_PLANE_SPECIFIC_INPUT_FLAGS,
    pub FlipInterval: super::d3dukmdt::D3DDDI_FLIPINTERVAL_TYPE,
    pub MaxImmediateFlipLine: u32,
    pub AllocationCount: u32,
    pub pAllocationList: *mut super::d3dukmdt::D3DKMT_HANDLE,
    pub DriverPrivateDataSize: u32,
    pub pDriverPrivateData: *mut core::ffi::c_void,
    pub pPlaneAttributes: *const D3DKMT_MULTIPLANE_OVERLAY_ATTRIBUTES3,
    pub hFlipToFence: super::d3dukmdt::D3DKMT_HANDLE,
    pub hFlipAwayFence: super::d3dukmdt::D3DKMT_HANDLE,
    pub FlipToFenceValue: u64,
    pub FlipAwayFenceValue: u64,
}
#[cfg(all(feature = "d3dukmdt", feature = "windef"))]
impl Default for D3DKMT_MULTIPLANE_OVERLAY3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_MULTIPLANE_OVERLAY_ATTRIBUTES {
    pub Flags: u32,
    pub SrcRect: super::windef::RECT,
    pub DstRect: super::windef::RECT,
    pub ClipRect: super::windef::RECT,
    pub Rotation: super::d3dukmdt::D3DDDI_ROTATION,
    pub Blend: D3DKMT_MULTIPLANE_OVERLAY_BLEND,
    pub DirtyRectCount: u32,
    pub pDirtyRects: *mut super::windef::RECT,
    pub VideoFrameFormat: D3DKMT_MULTIPLANE_OVERLAY_VIDEO_FRAME_FORMAT,
    pub YCbCrFlags: u32,
    pub StereoFormat: D3DKMT_MULTIPLANE_OVERLAY_STEREO_FORMAT,
    pub StereoLeftViewFrame0: windows_sys::core::BOOL,
    pub StereoBaseViewFrame0: windows_sys::core::BOOL,
    pub StereoFlipMode: DXGKMT_MULTIPLANE_OVERLAY_STEREO_FLIP_MODE,
    pub StretchQuality: DXGKMT_MULTIPLANE_OVERLAY_STRETCH_QUALITY,
}
#[cfg(all(feature = "d3dukmdt", feature = "windef"))]
impl Default for D3DKMT_MULTIPLANE_OVERLAY_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_MULTIPLANE_OVERLAY_ATTRIBUTES2 {
    pub Flags: u32,
    pub SrcRect: super::windef::RECT,
    pub DstRect: super::windef::RECT,
    pub ClipRect: super::windef::RECT,
    pub Rotation: super::d3dukmdt::D3DDDI_ROTATION,
    pub Blend: D3DKMT_MULTIPLANE_OVERLAY_BLEND,
    pub DirtyRectCount: u32,
    pub pDirtyRects: *mut super::windef::RECT,
    pub VideoFrameFormat: D3DKMT_MULTIPLANE_OVERLAY_VIDEO_FRAME_FORMAT,
    pub ColorSpace: super::d3dukmdt::D3DDDI_COLOR_SPACE_TYPE,
    pub StereoFormat: D3DKMT_MULTIPLANE_OVERLAY_STEREO_FORMAT,
    pub StereoLeftViewFrame0: windows_sys::core::BOOL,
    pub StereoBaseViewFrame0: windows_sys::core::BOOL,
    pub StereoFlipMode: DXGKMT_MULTIPLANE_OVERLAY_STEREO_FLIP_MODE,
    pub StretchQuality: DXGKMT_MULTIPLANE_OVERLAY_STRETCH_QUALITY,
    pub Reserved1: u32,
}
#[cfg(all(feature = "d3dukmdt", feature = "windef"))]
impl Default for D3DKMT_MULTIPLANE_OVERLAY_ATTRIBUTES2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_MULTIPLANE_OVERLAY_ATTRIBUTES3 {
    pub Flags: u32,
    pub SrcRect: super::windef::RECT,
    pub DstRect: super::windef::RECT,
    pub ClipRect: super::windef::RECT,
    pub Rotation: super::d3dukmdt::D3DDDI_ROTATION,
    pub Blend: D3DKMT_MULTIPLANE_OVERLAY_BLEND,
    pub DirtyRectCount: u32,
    pub pDirtyRects: *mut super::windef::RECT,
    pub ColorSpace: super::d3dukmdt::D3DDDI_COLOR_SPACE_TYPE,
    pub StretchQuality: DXGKMT_MULTIPLANE_OVERLAY_STRETCH_QUALITY,
    pub SDRWhiteLevel: u32,
}
#[cfg(all(feature = "d3dukmdt", feature = "windef"))]
impl Default for D3DKMT_MULTIPLANE_OVERLAY_ATTRIBUTES3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3DKMT_MULTIPLANE_OVERLAY_BLEND = i32;
pub const D3DKMT_MULTIPLANE_OVERLAY_BLEND_ALPHABLEND: D3DKMT_MULTIPLANE_OVERLAY_BLEND = 1;
pub const D3DKMT_MULTIPLANE_OVERLAY_BLEND_OPAQUE: D3DKMT_MULTIPLANE_OVERLAY_BLEND = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_MULTIPLANE_OVERLAY_CAPS {
    pub Anonymous: D3DKMT_MULTIPLANE_OVERLAY_CAPS_0,
}
impl Default for D3DKMT_MULTIPLANE_OVERLAY_CAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_MULTIPLANE_OVERLAY_CAPS_0 {
    pub Anonymous: D3DKMT_MULTIPLANE_OVERLAY_CAPS_0_0,
    pub Value: u32,
}
impl Default for D3DKMT_MULTIPLANE_OVERLAY_CAPS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_MULTIPLANE_OVERLAY_CAPS_0_0 {
    pub _bitfield: u32,
}
pub type D3DKMT_MULTIPLANE_OVERLAY_FLAGS = i32;
pub const D3DKMT_MULTIPLANE_OVERLAY_FLAG_HORIZONTAL_FLIP: D3DKMT_MULTIPLANE_OVERLAY_FLAGS = 2;
pub const D3DKMT_MULTIPLANE_OVERLAY_FLAG_STATIC_CHECK: D3DKMT_MULTIPLANE_OVERLAY_FLAGS = 4;
pub const D3DKMT_MULTIPLANE_OVERLAY_FLAG_VERTICAL_FLIP: D3DKMT_MULTIPLANE_OVERLAY_FLAGS = 1;
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_MULTIPLANE_OVERLAY_POST_COMPOSITION {
    pub Flags: D3DKMT_MULTIPLANE_OVERLAY_POST_COMPOSITION_FLAGS,
    pub SrcRect: super::windef::RECT,
    pub DstRect: super::windef::RECT,
    pub Rotation: super::d3dukmdt::D3DDDI_ROTATION,
}
#[cfg(all(feature = "d3dukmdt", feature = "windef"))]
impl Default for D3DKMT_MULTIPLANE_OVERLAY_POST_COMPOSITION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_MULTIPLANE_OVERLAY_POST_COMPOSITION_FLAGS {
    pub Anonymous: D3DKMT_MULTIPLANE_OVERLAY_POST_COMPOSITION_FLAGS_0,
}
impl Default for D3DKMT_MULTIPLANE_OVERLAY_POST_COMPOSITION_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_MULTIPLANE_OVERLAY_POST_COMPOSITION_FLAGS_0 {
    pub Anonymous: D3DKMT_MULTIPLANE_OVERLAY_POST_COMPOSITION_FLAGS_0_0,
    pub Value: u32,
}
impl Default for D3DKMT_MULTIPLANE_OVERLAY_POST_COMPOSITION_FLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_MULTIPLANE_OVERLAY_POST_COMPOSITION_FLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_MULTIPLANE_OVERLAY_POST_COMPOSITION_WITH_SOURCE {
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub PostComposition: D3DKMT_MULTIPLANE_OVERLAY_POST_COMPOSITION,
}
#[cfg(all(feature = "d3dukmdt", feature = "windef"))]
impl Default for D3DKMT_MULTIPLANE_OVERLAY_POST_COMPOSITION_WITH_SOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3DKMT_MULTIPLANE_OVERLAY_STEREO_FORMAT = i32;
pub const D3DKMT_MULTIPLANE_OVERLAY_STEREO_FORMAT_HORIZONTAL: D3DKMT_MULTIPLANE_OVERLAY_STEREO_FORMAT = 1;
pub const D3DKMT_MULTIPLANE_OVERLAY_STEREO_FORMAT_VERTICAL: D3DKMT_MULTIPLANE_OVERLAY_STEREO_FORMAT = 2;
pub type D3DKMT_MULTIPLANE_OVERLAY_VIDEO_FRAME_FORMAT = i32;
pub const D3DKMT_MULTIPLANE_OVERLAY_VIDEO_FRAME_FORMAT_INTERLACED_BOTTOM_FIELD_FIRST: D3DKMT_MULTIPLANE_OVERLAY_VIDEO_FRAME_FORMAT = 2;
pub const D3DKMT_MULTIPLANE_OVERLAY_VIDEO_FRAME_FORMAT_INTERLACED_TOP_FIELD_FIRST: D3DKMT_MULTIPLANE_OVERLAY_VIDEO_FRAME_FORMAT = 1;
pub type D3DKMT_MULTIPLANE_OVERLAY_YCbCr_FLAGS = i32;
pub const D3DKMT_MULTIPLANE_OVERLAY_YCbCr_FLAG_BT709: D3DKMT_MULTIPLANE_OVERLAY_YCbCr_FLAGS = 2;
pub const D3DKMT_MULTIPLANE_OVERLAY_YCbCr_FLAG_NOMINAL_RANGE: D3DKMT_MULTIPLANE_OVERLAY_YCbCr_FLAGS = 1;
pub const D3DKMT_MULTIPLANE_OVERLAY_YCbCr_FLAG_xvYCC: D3DKMT_MULTIPLANE_OVERLAY_YCbCr_FLAGS = 4;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_MULTISAMPLEMETHOD {
    pub NumSamples: u32,
    pub NumQualityLevels: u32,
    pub Reserved: u32,
}
pub const D3DKMT_MaxAllocationPriorityClass: D3DKMT_QUERYSTATISTICS_ALLOCATION_PRIORITY_CLASS = 5;
pub const D3DKMT_MmIoFlipCommandBuffer: D3DKMT_QUERYSTATISTICS_QUEUE_PACKET_TYPE = 3;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_NODE_PERFDATA {
    pub NodeOrdinal: u32,
    pub PhysicalAdapterIndex: u32,
    pub Frequency: u64,
    pub MaxFrequency: u64,
    pub MaxFrequencyOC: u64,
    pub Voltage: u32,
    pub VoltageMax: u32,
    pub VoltageMaxOC: u32,
    pub MaxTransitionLatency: u64,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_NOTIFY_WORK_SUBMISSION {
    pub hHwQueue: super::d3dukmdt::D3DKMT_HANDLE,
    pub Flags: D3DKMT_NOTIFY_WORK_SUBMISSION_FLAGS,
    pub PrivateDriverData: [u8; 64],
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_NOTIFY_WORK_SUBMISSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_NOTIFY_WORK_SUBMISSION_FLAGS {
    pub Anonymous: D3DKMT_NOTIFY_WORK_SUBMISSION_FLAGS_0,
}
impl Default for D3DKMT_NOTIFY_WORK_SUBMISSION_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_NOTIFY_WORK_SUBMISSION_FLAGS_0 {
    pub Anonymous: D3DKMT_NOTIFY_WORK_SUBMISSION_FLAGS_0_0,
    pub Value: u32,
}
impl Default for D3DKMT_NOTIFY_WORK_SUBMISSION_FLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_NOTIFY_WORK_SUBMISSION_FLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_OFFERALLOCATIONS {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub pResources: *mut super::d3dukmdt::D3DKMT_HANDLE,
    pub HandleList: *const super::d3dukmdt::D3DKMT_HANDLE,
    pub NumAllocations: u32,
    pub Priority: D3DKMT_OFFER_PRIORITY,
    pub Flags: D3DKMT_OFFER_FLAGS,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_OFFERALLOCATIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_OFFER_FLAGS {
    pub Anonymous: D3DKMT_OFFER_FLAGS_0,
}
impl Default for D3DKMT_OFFER_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_OFFER_FLAGS_0 {
    pub Anonymous: D3DKMT_OFFER_FLAGS_0_0,
    pub Value: u32,
}
impl Default for D3DKMT_OFFER_FLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_OFFER_FLAGS_0_0 {
    pub _bitfield: u32,
}
pub type D3DKMT_OFFER_PRIORITY = i32;
pub const D3DKMT_OFFER_PRIORITY_AUTO: D3DKMT_OFFER_PRIORITY = 4;
pub const D3DKMT_OFFER_PRIORITY_HIGH: D3DKMT_OFFER_PRIORITY = 3;
pub const D3DKMT_OFFER_PRIORITY_LOW: D3DKMT_OFFER_PRIORITY = 1;
pub const D3DKMT_OFFER_PRIORITY_NORMAL: D3DKMT_OFFER_PRIORITY = 2;
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_OPENADAPTERFROMDEVICENAME {
    pub pDeviceName: windows_sys::core::PCWSTR,
    pub hAdapter: super::d3dukmdt::D3DKMT_HANDLE,
    pub AdapterLuid: super::winnt::LUID,
}
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_OPENADAPTERFROMDEVICENAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_OPENADAPTERFROMGDIDISPLAYNAME {
    pub DeviceName: [u16; 32],
    pub hAdapter: super::d3dukmdt::D3DKMT_HANDLE,
    pub AdapterLuid: super::winnt::LUID,
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
}
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_OPENADAPTERFROMGDIDISPLAYNAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_OPENADAPTERFROMHDC {
    pub hDc: super::windef::HDC,
    pub hAdapter: super::d3dukmdt::D3DKMT_HANDLE,
    pub AdapterLuid: super::winnt::LUID,
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
}
#[cfg(all(feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
impl Default for D3DKMT_OPENADAPTERFROMHDC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_OPENADAPTERFROMLUID {
    pub AdapterLuid: super::winnt::LUID,
    pub hAdapter: super::d3dukmdt::D3DKMT_HANDLE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_OPENGLINFO {
    pub UmdOpenGlIcdFileName: [u16; 260],
    pub Version: u32,
    pub Flags: u32,
}
impl Default for D3DKMT_OPENGLINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_OPENKEYEDMUTEX {
    pub hSharedHandle: super::d3dukmdt::D3DKMT_HANDLE,
    pub hKeyedMutex: super::d3dukmdt::D3DKMT_HANDLE,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_OPENKEYEDMUTEX2 {
    pub hSharedHandle: super::d3dukmdt::D3DKMT_HANDLE,
    pub hKeyedMutex: super::d3dukmdt::D3DKMT_HANDLE,
    pub pPrivateRuntimeData: *mut core::ffi::c_void,
    pub PrivateRuntimeDataSize: u32,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_OPENKEYEDMUTEX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_OPENKEYEDMUTEXFROMNTHANDLE {
    pub hNtHandle: super::winnt::HANDLE,
    pub hKeyedMutex: super::d3dukmdt::D3DKMT_HANDLE,
    pub pPrivateRuntimeData: *mut core::ffi::c_void,
    pub PrivateRuntimeDataSize: u32,
}
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_OPENKEYEDMUTEXFROMNTHANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_OPENNATIVEFENCEFROMNTHANDLE {
    pub hNtHandle: super::winnt::HANDLE,
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub EngineAffinity: u32,
    pub Flags: super::d3dukmdt::D3DDDI_SYNCHRONIZATIONOBJECT_FLAGS,
    pub hSyncObject: super::d3dukmdt::D3DKMT_HANDLE,
    pub NativeFenceMapping: super::d3dukmdt::D3DDDI_NATIVEFENCEMAPPING,
    pub PrivateDriverData: [u8; 64],
    pub Reserved: [u8; 32],
}
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_OPENNATIVEFENCEFROMNTHANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_OPENNTHANDLEFROMNAME {
    pub dwDesiredAccess: u32,
    pub pObjAttrib: *mut OBJECT_ATTRIBUTES,
    pub hNtHandle: super::winnt::HANDLE,
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
impl Default for D3DKMT_OPENNTHANDLEFROMNAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_OPENPROTECTEDSESSIONFROMNTHANDLE {
    pub hNtHandle: super::winnt::HANDLE,
    pub hHandle: super::d3dukmdt::D3DKMT_HANDLE,
}
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_OPENPROTECTEDSESSIONFROMNTHANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_OPENRESOURCE {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub hGlobalShare: super::d3dukmdt::D3DKMT_HANDLE,
    pub NumAllocations: u32,
    pub Anonymous: D3DKMT_OPENRESOURCE_0,
    pub pPrivateRuntimeData: *mut core::ffi::c_void,
    pub PrivateRuntimeDataSize: u32,
    pub pResourcePrivateDriverData: *mut core::ffi::c_void,
    pub ResourcePrivateDriverDataSize: u32,
    pub pTotalPrivateDriverDataBuffer: *mut core::ffi::c_void,
    pub TotalPrivateDriverDataBufferSize: u32,
    pub hResource: super::d3dukmdt::D3DKMT_HANDLE,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_OPENRESOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub union D3DKMT_OPENRESOURCE_0 {
    pub pOpenAllocationInfo: *mut super::d3dukmdt::D3DDDI_OPENALLOCATIONINFO,
    pub pOpenAllocationInfo2: *mut super::d3dukmdt::D3DDDI_OPENALLOCATIONINFO2,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_OPENRESOURCE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_OPENRESOURCEFROMNTHANDLE {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub hNtHandle: super::winnt::HANDLE,
    pub NumAllocations: u32,
    pub pOpenAllocationInfo2: *mut super::d3dukmdt::D3DDDI_OPENALLOCATIONINFO2,
    pub PrivateRuntimeDataSize: u32,
    pub pPrivateRuntimeData: *mut core::ffi::c_void,
    pub ResourcePrivateDriverDataSize: u32,
    pub pResourcePrivateDriverData: *mut core::ffi::c_void,
    pub TotalPrivateDriverDataBufferSize: u32,
    pub pTotalPrivateDriverDataBuffer: *mut core::ffi::c_void,
    pub hResource: super::d3dukmdt::D3DKMT_HANDLE,
    pub hKeyedMutex: super::d3dukmdt::D3DKMT_HANDLE,
    pub pKeyedMutexPrivateRuntimeData: *mut core::ffi::c_void,
    pub KeyedMutexPrivateRuntimeDataSize: u32,
    pub hSyncObject: super::d3dukmdt::D3DKMT_HANDLE,
}
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_OPENRESOURCEFROMNTHANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_OPENSYNCHRONIZATIONOBJECT {
    pub hSharedHandle: super::d3dukmdt::D3DKMT_HANDLE,
    pub hSyncObject: super::d3dukmdt::D3DKMT_HANDLE,
    pub Reserved: [u64; 8],
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_OPENSYNCHRONIZATIONOBJECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_OPENSYNCOBJECTFROMNTHANDLE {
    pub hNtHandle: super::winnt::HANDLE,
    pub hSyncObject: super::d3dukmdt::D3DKMT_HANDLE,
}
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_OPENSYNCOBJECTFROMNTHANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_OPENSYNCOBJECTFROMNTHANDLE2 {
    pub hNtHandle: super::winnt::HANDLE,
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub Flags: super::d3dukmdt::D3DDDI_SYNCHRONIZATIONOBJECT_FLAGS,
    pub hSyncObject: super::d3dukmdt::D3DKMT_HANDLE,
    pub Anonymous: D3DKMT_OPENSYNCOBJECTFROMNTHANDLE2_0,
}
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_OPENSYNCOBJECTFROMNTHANDLE2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union D3DKMT_OPENSYNCOBJECTFROMNTHANDLE2_0 {
    pub MonitoredFence: D3DKMT_OPENSYNCOBJECTFROMNTHANDLE2_0_0,
    pub Reserved: [u64; 8],
}
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_OPENSYNCOBJECTFROMNTHANDLE2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_OPENSYNCOBJECTFROMNTHANDLE2_0_0 {
    pub FenceValueCPUVirtualAddress: *mut core::ffi::c_void,
    pub FenceValueGPUVirtualAddress: super::d3dukmdt::D3DGPU_VIRTUAL_ADDRESS,
    pub EngineAffinity: u32,
}
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_OPENSYNCOBJECTFROMNTHANDLE2_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_OPENSYNCOBJECTFROMSYNCFILE {
    pub hSyncFile: u64,
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub hSyncObject: super::d3dukmdt::D3DKMT_HANDLE,
    pub FenceValue: u64,
    pub FenceValueCPUVirtualAddress: *mut core::ffi::c_void,
    pub FenceValueGPUVirtualAddress: super::d3dukmdt::D3DGPU_VIRTUAL_ADDRESS,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_OPENSYNCOBJECTFROMSYNCFILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_OPENSYNCOBJECTNTHANDLEFROMNAME {
    pub dwDesiredAccess: u32,
    pub pObjAttrib: *mut OBJECT_ATTRIBUTES,
    pub hNtHandle: super::winnt::HANDLE,
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
impl Default for D3DKMT_OPENSYNCOBJECTNTHANDLEFROMNAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_OUTDUPL_POINTER_SHAPE_INFO {
    pub Type: D3DKMT_OUTDUPL_POINTER_SHAPE_TYPE,
    pub Width: u32,
    pub Height: u32,
    pub Pitch: u32,
    pub HotSpot: super::windef::POINT,
}
pub type D3DKMT_OUTDUPL_POINTER_SHAPE_TYPE = i32;
pub const D3DKMT_OUTDUPL_POINTER_SHAPE_TYPE_COLOR: D3DKMT_OUTDUPL_POINTER_SHAPE_TYPE = 2;
pub const D3DKMT_OUTDUPL_POINTER_SHAPE_TYPE_MASKED_COLOR: D3DKMT_OUTDUPL_POINTER_SHAPE_TYPE = 4;
pub const D3DKMT_OUTDUPL_POINTER_SHAPE_TYPE_MONOCHROME: D3DKMT_OUTDUPL_POINTER_SHAPE_TYPE = 1;
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_OUTPUTDUPLCONTEXTSCOUNT {
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub OutputDuplicationCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_OUTPUTDUPLCREATIONFLAGS {
    pub Anonymous: D3DKMT_OUTPUTDUPLCREATIONFLAGS_0,
}
impl Default for D3DKMT_OUTPUTDUPLCREATIONFLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_OUTPUTDUPLCREATIONFLAGS_0 {
    pub Anonymous: D3DKMT_OUTPUTDUPLCREATIONFLAGS_0_0,
    pub Value: u32,
}
impl Default for D3DKMT_OUTPUTDUPLCREATIONFLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_OUTPUTDUPLCREATIONFLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(all(feature = "d3dkmdt", feature = "d3dukmdt", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_OUTPUTDUPLPRESENT {
    pub hContext: super::d3dukmdt::D3DKMT_HANDLE,
    pub hSource: super::d3dukmdt::D3DKMT_HANDLE,
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub BroadcastContextCount: u32,
    pub BroadcastContext: [super::d3dukmdt::D3DKMT_HANDLE; 64],
    pub PresentRegions: D3DKMT_PRESENT_RGNS,
    pub Flags: D3DKMT_OUTPUTDUPLPRESENTFLAGS,
    pub hIndirectContext: super::d3dukmdt::D3DKMT_HANDLE,
}
#[cfg(all(feature = "d3dkmdt", feature = "d3dukmdt", feature = "windef"))]
impl Default for D3DKMT_OUTPUTDUPLPRESENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_OUTPUTDUPLPRESENTFLAGS {
    pub Anonymous: D3DKMT_OUTPUTDUPLPRESENTFLAGS_0,
}
impl Default for D3DKMT_OUTPUTDUPLPRESENTFLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_OUTPUTDUPLPRESENTFLAGS_0 {
    pub Anonymous: D3DKMT_OUTPUTDUPLPRESENTFLAGS_0_0,
    pub Value: u32,
}
impl Default for D3DKMT_OUTPUTDUPLPRESENTFLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_OUTPUTDUPLPRESENTFLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(all(feature = "d3dkmdt", feature = "d3dukmdt", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_OUTPUTDUPLPRESENTTOHWQUEUE {
    pub hSource: super::d3dukmdt::D3DKMT_HANDLE,
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub BroadcastHwQueueCount: u32,
    pub hHwQueues: *mut super::d3dukmdt::D3DKMT_HANDLE,
    pub PresentRegions: D3DKMT_PRESENT_RGNS,
    pub Flags: D3DKMT_OUTPUTDUPLPRESENTFLAGS,
    pub hIndirectHwQueue: super::d3dukmdt::D3DKMT_HANDLE,
}
#[cfg(all(feature = "d3dkmdt", feature = "d3dukmdt", feature = "windef"))]
impl Default for D3DKMT_OUTPUTDUPLPRESENTTOHWQUEUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_OUTPUTDUPL_FRAMEINFO {
    pub LastPresentTime: i64,
    pub LastMouseUpdateTime: i64,
    pub AccumulatedFrames: u32,
    pub RectsCoalesced: windows_sys::core::BOOL,
    pub ProtectedContentMaskedOut: windows_sys::core::BOOL,
    pub PointerPosition: D3DKMT_OUTPUTDUPL_POINTER_POSITION,
    pub TotalMetadataBufferSize: u32,
    pub PointerShapeBufferSize: u32,
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "windef"))]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_OUTPUTDUPL_GET_FRAMEINFO {
    pub hAdapter: super::d3dukmdt::D3DKMT_HANDLE,
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub FrameInfo: D3DKMT_OUTPUTDUPL_FRAMEINFO,
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_OUTPUTDUPL_GET_POINTER_SHAPE_DATA {
    pub hAdapter: super::d3dukmdt::D3DKMT_HANDLE,
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub BufferSizeSupplied: u32,
    pub pShapeBuffer: *mut core::ffi::c_void,
    pub BufferSizeRequired: u32,
    pub ShapeInfo: D3DKMT_OUTDUPL_POINTER_SHAPE_INFO,
}
#[cfg(all(feature = "d3dukmdt", feature = "windef"))]
impl Default for D3DKMT_OUTPUTDUPL_GET_POINTER_SHAPE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_OUTPUTDUPL_KEYEDMUTEX {
    pub hSharedSurfaceNt: super::winnt::HANDLE,
}
#[cfg(feature = "winnt")]
impl Default for D3DKMT_OUTPUTDUPL_KEYEDMUTEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_OUTPUTDUPL_METADATA {
    pub hAdapter: super::d3dukmdt::D3DKMT_HANDLE,
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub Type: D3DKMT_OUTPUTDUPL_METADATATYPE,
    pub BufferSizeSupplied: u32,
    pub pBuffer: *mut core::ffi::c_void,
    pub BufferSizeRequired: u32,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_OUTPUTDUPL_METADATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3DKMT_OUTPUTDUPL_METADATATYPE = i32;
pub const D3DKMT_OUTPUTDUPL_METADATATYPE_DIRTY_RECTS: D3DKMT_OUTPUTDUPL_METADATATYPE = 0;
pub const D3DKMT_OUTPUTDUPL_METADATATYPE_MOVE_RECTS: D3DKMT_OUTPUTDUPL_METADATATYPE = 1;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_OUTPUTDUPL_POINTER_POSITION {
    pub Position: super::windef::POINT,
    pub Visible: windows_sys::core::BOOL,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_OUTPUTDUPL_RELEASE_FRAME {
    pub hAdapter: super::d3dukmdt::D3DKMT_HANDLE,
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub NextKeyMutexIdx: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_OUTPUTDUPL_SNAPSHOT {
    pub Size: u32,
    pub SessionProcessCount: u32,
    pub SessionActiveConnectionsCount: u32,
    pub NumVidPnSources: u32,
    pub NumOutputDuplContexts: u32,
    pub Padding: u32,
    pub OutputDuplDebugInfos: [OUTPUTDUPL_CONTEXT_DEBUG_INFO; 0],
}
#[cfg(feature = "winnt")]
impl Default for D3DKMT_OUTPUTDUPL_SNAPSHOT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_PAGE_TABLE_LEVEL_DESC {
    pub IndexBitCount: u32,
    pub IndexMask: u64,
    pub IndexShift: u64,
    pub LowerLevelsMask: u64,
    pub EntryCoverageInPages: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_PANELFITTER_SUPPORT {
    pub Supported: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_PARAVIRTUALIZATION {
    pub SecureContainer: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_PHYSICAL_ADAPTER_COUNT {
    pub Count: u32,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_PINDIRECTFLIPRESOURCES {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub ResourceCount: u32,
    pub pResourceList: *mut super::d3dukmdt::D3DKMT_HANDLE,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_PINDIRECTFLIPRESOURCES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_PLANE_SPECIFIC_INPUT_FLAGS {
    pub Anonymous: D3DKMT_PLANE_SPECIFIC_INPUT_FLAGS_0,
}
impl Default for D3DKMT_PLANE_SPECIFIC_INPUT_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_PLANE_SPECIFIC_INPUT_FLAGS_0 {
    pub Anonymous: D3DKMT_PLANE_SPECIFIC_INPUT_FLAGS_0_0,
    pub Value: u32,
}
impl Default for D3DKMT_PLANE_SPECIFIC_INPUT_FLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_PLANE_SPECIFIC_INPUT_FLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_PLANE_SPECIFIC_OUTPUT_FLAGS {
    pub Anonymous: D3DKMT_PLANE_SPECIFIC_OUTPUT_FLAGS_0,
}
impl Default for D3DKMT_PLANE_SPECIFIC_OUTPUT_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_PLANE_SPECIFIC_OUTPUT_FLAGS_0 {
    pub Anonymous: D3DKMT_PLANE_SPECIFIC_OUTPUT_FLAGS_0_0,
    pub Value: u32,
}
impl Default for D3DKMT_PLANE_SPECIFIC_OUTPUT_FLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_PLANE_SPECIFIC_OUTPUT_FLAGS_0_0 {
    pub _bitfield: u32,
}
pub const D3DKMT_PM_FLIPMANAGER: D3DKMT_PRESENT_MODEL = 9;
pub const D3DKMT_PM_REDIRECTED_BLT: D3DKMT_PRESENT_MODEL = 3;
pub const D3DKMT_PM_REDIRECTED_COMPOSITION: D3DKMT_PRESENT_MODEL = 7;
pub const D3DKMT_PM_REDIRECTED_FLIP: D3DKMT_PRESENT_MODEL = 2;
pub const D3DKMT_PM_REDIRECTED_GDI: D3DKMT_PRESENT_MODEL = 1;
pub const D3DKMT_PM_REDIRECTED_GDI_SYSMEM: D3DKMT_PRESENT_MODEL = 6;
pub const D3DKMT_PM_REDIRECTED_VISTABLT: D3DKMT_PRESENT_MODEL = 4;
pub const D3DKMT_PM_SCREENCAPTUREFENCE: D3DKMT_PRESENT_MODEL = 5;
pub const D3DKMT_PM_SURFACECOMPLETE: D3DKMT_PRESENT_MODEL = 8;
pub const D3DKMT_PM_UNINITIALIZED: D3DKMT_PRESENT_MODEL = 0;
pub const D3DKMT_PNP_KEY_HARDWARE: D3DKMT_PNP_KEY_TYPE = 1;
pub const D3DKMT_PNP_KEY_SOFTWARE: D3DKMT_PNP_KEY_TYPE = 2;
pub type D3DKMT_PNP_KEY_TYPE = i32;
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_POLLDISPLAYCHILDREN {
    pub hAdapter: super::d3dukmdt::D3DKMT_HANDLE,
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(all(feature = "d3dkmdt", feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_PRESENT {
    pub Anonymous: D3DKMT_PRESENT_0,
    pub hWindow: super::windef::HWND,
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub hSource: super::d3dukmdt::D3DKMT_HANDLE,
    pub hDestination: super::d3dukmdt::D3DKMT_HANDLE,
    pub Color: u32,
    pub DstRect: super::windef::RECT,
    pub SrcRect: super::windef::RECT,
    pub SubRectCnt: u32,
    pub pSrcSubRects: *const super::windef::RECT,
    pub PresentCount: u32,
    pub FlipInterval: super::d3dukmdt::D3DDDI_FLIPINTERVAL_TYPE,
    pub Flags: D3DKMT_PRESENTFLAGS,
    pub BroadcastContextCount: u32,
    pub BroadcastContext: [super::d3dukmdt::D3DKMT_HANDLE; 64],
    pub PresentLimitSemaphore: super::winnt::HANDLE,
    pub PresentHistoryToken: D3DKMT_PRESENTHISTORYTOKEN,
    pub pPresentRegions: *mut D3DKMT_PRESENT_RGNS,
    pub Anonymous2: D3DKMT_PRESENT_1,
    pub Duration: u32,
    pub BroadcastSrcAllocation: *mut super::d3dukmdt::D3DKMT_HANDLE,
    pub BroadcastDstAllocation: *mut super::d3dukmdt::D3DKMT_HANDLE,
    pub PrivateDriverDataSize: u32,
    pub pPrivateDriverData: *mut core::ffi::c_void,
    pub bOptimizeForComposition: bool,
}
#[cfg(all(feature = "d3dkmdt", feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
impl Default for D3DKMT_PRESENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dkmdt", feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union D3DKMT_PRESENT_0 {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub hContext: super::d3dukmdt::D3DKMT_HANDLE,
}
#[cfg(all(feature = "d3dkmdt", feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
impl Default for D3DKMT_PRESENT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dkmdt", feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union D3DKMT_PRESENT_1 {
    pub hAdapter: super::d3dukmdt::D3DKMT_HANDLE,
    pub hIndirectContext: super::d3dukmdt::D3DKMT_HANDLE,
}
#[cfg(all(feature = "d3dkmdt", feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
impl Default for D3DKMT_PRESENT_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_PRESENTFLAGS {
    pub Anonymous: D3DKMT_PRESENTFLAGS_0,
}
impl Default for D3DKMT_PRESENTFLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_PRESENTFLAGS_0 {
    pub Anonymous: D3DKMT_PRESENTFLAGS_0_0,
    pub Value: u32,
}
impl Default for D3DKMT_PRESENTFLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_PRESENTFLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_PRESENTHISTORYTOKEN {
    pub Model: D3DKMT_PRESENT_MODEL,
    pub TokenSize: u32,
    pub CompositionBindingId: u64,
    pub Token: D3DKMT_PRESENTHISTORYTOKEN_0,
}
#[cfg(all(feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
impl Default for D3DKMT_PRESENTHISTORYTOKEN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union D3DKMT_PRESENTHISTORYTOKEN_0 {
    pub MaxSize: [u8; 1064],
    pub Flip: D3DKMT_FLIPMODEL_PRESENTHISTORYTOKEN,
    pub Blt: D3DKMT_BLTMODEL_PRESENTHISTORYTOKEN,
    pub VistaBlt: D3DKMT_VISTABLTMODEL_PRESENTHISTORYTOKEN,
    pub Gdi: D3DKMT_GDIMODEL_PRESENTHISTORYTOKEN,
    pub Fence: D3DKMT_FENCE_PRESENTHISTORYTOKEN,
    pub GdiSysMem: D3DKMT_GDIMODEL_SYSMEM_PRESENTHISTORYTOKEN,
    pub Composition: D3DKMT_COMPOSITION_PRESENTHISTORYTOKEN,
    pub FlipManager: D3DKMT_FLIPMANAGER_PRESENTHISTORYTOKEN,
    pub SurfaceComplete: D3DKMT_SURFACECOMPLETE_PRESENTHISTORYTOKEN,
}
#[cfg(all(feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
impl Default for D3DKMT_PRESENTHISTORYTOKEN_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3DKMT_PRESENTHISTORYTOKEN_SIZE: u32 = 1080;
pub type D3DKMT_PRESENT_MODEL = i32;
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_PRESENT_MULTIPLANE_OVERLAY {
    pub Anonymous: D3DKMT_PRESENT_MULTIPLANE_OVERLAY_0,
    pub BroadcastContextCount: u32,
    pub BroadcastContext: [super::d3dukmdt::D3DKMT_HANDLE; 64],
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub PresentCount: u32,
    pub FlipInterval: super::d3dukmdt::D3DDDI_FLIPINTERVAL_TYPE,
    pub Flags: D3DKMT_PRESENTFLAGS,
    pub PresentPlaneCount: u32,
    pub pPresentPlanes: *mut D3DKMT_MULTIPLANE_OVERLAY,
    pub Duration: u32,
}
#[cfg(all(feature = "d3dukmdt", feature = "windef"))]
impl Default for D3DKMT_PRESENT_MULTIPLANE_OVERLAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "windef"))]
#[derive(Clone, Copy)]
pub union D3DKMT_PRESENT_MULTIPLANE_OVERLAY_0 {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub hContext: super::d3dukmdt::D3DKMT_HANDLE,
}
#[cfg(all(feature = "d3dukmdt", feature = "windef"))]
impl Default for D3DKMT_PRESENT_MULTIPLANE_OVERLAY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_PRESENT_MULTIPLANE_OVERLAY2 {
    pub hAdapter: super::d3dukmdt::D3DKMT_HANDLE,
    pub Anonymous: D3DKMT_PRESENT_MULTIPLANE_OVERLAY2_0,
    pub BroadcastContextCount: u32,
    pub BroadcastContext: [super::d3dukmdt::D3DKMT_HANDLE; 64],
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub PresentCount: u32,
    pub FlipInterval: super::d3dukmdt::D3DDDI_FLIPINTERVAL_TYPE,
    pub Flags: D3DKMT_PRESENTFLAGS,
    pub PresentPlaneCount: u32,
    pub pPresentPlanes: *mut D3DKMT_MULTIPLANE_OVERLAY2,
    pub Duration: u32,
}
#[cfg(all(feature = "d3dukmdt", feature = "windef"))]
impl Default for D3DKMT_PRESENT_MULTIPLANE_OVERLAY2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "windef"))]
#[derive(Clone, Copy)]
pub union D3DKMT_PRESENT_MULTIPLANE_OVERLAY2_0 {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub hContext: super::d3dukmdt::D3DKMT_HANDLE,
}
#[cfg(all(feature = "d3dukmdt", feature = "windef"))]
impl Default for D3DKMT_PRESENT_MULTIPLANE_OVERLAY2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_PRESENT_MULTIPLANE_OVERLAY3 {
    pub hAdapter: super::d3dukmdt::D3DKMT_HANDLE,
    pub ContextCount: u32,
    pub pContextList: *mut super::d3dukmdt::D3DKMT_HANDLE,
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub PresentCount: u32,
    pub Flags: D3DKMT_PRESENT_MULTIPLANE_OVERLAY_FLAGS,
    pub PresentPlaneCount: u32,
    pub ppPresentPlanes: *mut *mut D3DKMT_MULTIPLANE_OVERLAY3,
    pub pPostComposition: *mut D3DKMT_MULTIPLANE_OVERLAY_POST_COMPOSITION,
    pub Duration: u32,
    pub HDRMetaDataType: super::d3dukmdt::D3DDDI_HDR_METADATA_TYPE,
    pub HDRMetaDataSize: u32,
    pub pHDRMetaData: *const core::ffi::c_void,
    pub BoostRefreshRateMultiplier: u32,
}
#[cfg(all(feature = "d3dukmdt", feature = "windef"))]
impl Default for D3DKMT_PRESENT_MULTIPLANE_OVERLAY3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_PRESENT_MULTIPLANE_OVERLAY_FLAGS {
    pub Anonymous: D3DKMT_PRESENT_MULTIPLANE_OVERLAY_FLAGS_0,
}
impl Default for D3DKMT_PRESENT_MULTIPLANE_OVERLAY_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_PRESENT_MULTIPLANE_OVERLAY_FLAGS_0 {
    pub Anonymous: D3DKMT_PRESENT_MULTIPLANE_OVERLAY_FLAGS_0_0,
    pub Value: u32,
}
impl Default for D3DKMT_PRESENT_MULTIPLANE_OVERLAY_FLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_PRESENT_MULTIPLANE_OVERLAY_FLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_PRESENT_REDIRECTED {
    pub hSyncObj: super::d3dukmdt::D3DKMT_HANDLE,
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub WaitedFenceValue: u64,
    pub PresentHistoryToken: D3DKMT_PRESENTHISTORYTOKEN,
    pub Flags: D3DKMT_PRESENT_REDIRECTED_FLAGS,
    pub hSource: super::d3dukmdt::D3DKMT_HANDLE,
    pub PrivateDriverDataSize: u32,
    pub pPrivateDriverData: *mut core::ffi::c_void,
}
#[cfg(all(feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
impl Default for D3DKMT_PRESENT_REDIRECTED {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_PRESENT_REDIRECTED_FLAGS {
    pub Anonymous: D3DKMT_PRESENT_REDIRECTED_FLAGS_0,
}
impl Default for D3DKMT_PRESENT_REDIRECTED_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_PRESENT_REDIRECTED_FLAGS_0 {
    pub Anonymous: D3DKMT_PRESENT_REDIRECTED_FLAGS_0_0,
    pub Value: u32,
}
impl Default for D3DKMT_PRESENT_REDIRECTED_FLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_PRESENT_REDIRECTED_FLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(all(feature = "d3dkmdt", feature = "windef"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_PRESENT_RGNS {
    pub DirtyRectCount: u32,
    pub pDirtyRects: *const super::windef::RECT,
    pub MoveRectCount: u32,
    pub pMoveRects: *const super::d3dkmdt::D3DKMT_MOVE_RECT,
}
#[cfg(all(feature = "d3dkmdt", feature = "windef"))]
impl Default for D3DKMT_PRESENT_RGNS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_PRESENT_STATS {
    pub PresentCount: u32,
    pub PresentRefreshCount: u32,
    pub SyncRefreshCount: u32,
    pub SyncQPCTime: i64,
    pub SyncGPUTime: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_PRESENT_STATS_DWM {
    pub PresentCount: u32,
    pub PresentRefreshCount: u32,
    pub PresentQPCTime: i64,
    pub SyncRefreshCount: u32,
    pub SyncQPCTime: i64,
    pub CustomPresentDuration: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_PRESENT_STATS_DWM2 {
    pub cbSize: u32,
    pub PresentCount: u32,
    pub PresentRefreshCount: u32,
    pub PresentQPCTime: i64,
    pub SyncRefreshCount: u32,
    pub SyncQPCTime: i64,
    pub CustomPresentDuration: u32,
    pub VirtualSyncRefreshCount: u32,
    pub VirtualSyncQPCTime: i64,
    pub VSyncDurationQPCTime: i64,
    pub VSyncMultiplier: u32,
    pub VirtualPresentRefreshCount: u32,
    pub VirtualPresentQPCTime: i64,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_PROCESS_VERIFIER_OPTION {
    pub hProcess: super::winnt::HANDLE,
    pub Type: D3DKMT_PROCESS_VERIFIER_OPTION_TYPE,
    pub Mode: D3DKMT_VERIFIER_OPTION_MODE,
    pub Data: D3DKMT_PROCESS_VERIFIER_OPTION_DATA,
}
#[cfg(feature = "winnt")]
impl Default for D3DKMT_PROCESS_VERIFIER_OPTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_PROCESS_VERIFIER_OPTION_DATA {
    pub VidMmFlags: D3DKMT_PROCESS_VERIFIER_VIDMM_FLAGS,
    pub VidMmRestrictBudget: D3DKMT_PROCESS_VERIFIER_VIDMM_RESTRICT_BUDGET,
}
impl Default for D3DKMT_PROCESS_VERIFIER_OPTION_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3DKMT_PROCESS_VERIFIER_OPTION_TYPE = i32;
pub const D3DKMT_PROCESS_VERIFIER_OPTION_VIDMM_FLAGS: D3DKMT_PROCESS_VERIFIER_OPTION_TYPE = 1000;
pub const D3DKMT_PROCESS_VERIFIER_OPTION_VIDMM_RESTRICT_BUDGET: D3DKMT_PROCESS_VERIFIER_OPTION_TYPE = 1001;
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_PROCESS_VERIFIER_VIDMM_FLAGS {
    pub Anonymous: D3DKMT_PROCESS_VERIFIER_VIDMM_FLAGS_0,
    pub Value: u32,
}
impl Default for D3DKMT_PROCESS_VERIFIER_VIDMM_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_PROCESS_VERIFIER_VIDMM_FLAGS_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_PROCESS_VERIFIER_VIDMM_RESTRICT_BUDGET {
    pub LocalBudget: u64,
    pub NonLocalBudget: u64,
}
pub type D3DKMT_PROTECTED_SESSION_STATUS = i32;
pub const D3DKMT_PROTECTED_SESSION_STATUS_INVALID: D3DKMT_PROTECTED_SESSION_STATUS = 1;
pub const D3DKMT_PROTECTED_SESSION_STATUS_OK: D3DKMT_PROTECTED_SESSION_STATUS = 0;
pub const D3DKMT_PreemptionAttempt: D3DKMT_QUERYRESULT_PREEMPTION_ATTEMPT_RESULT = 0;
pub const D3DKMT_PreemptionAttemptMissAlreadyPreempting: D3DKMT_QUERYRESULT_PREEMPTION_ATTEMPT_RESULT = 13;
pub const D3DKMT_PreemptionAttemptMissAlreadyRunning: D3DKMT_QUERYRESULT_PREEMPTION_ATTEMPT_RESULT = 15;
pub const D3DKMT_PreemptionAttemptMissFenceCommand: D3DKMT_QUERYRESULT_PREEMPTION_ATTEMPT_RESULT = 7;
pub const D3DKMT_PreemptionAttemptMissGlobalBlock: D3DKMT_QUERYRESULT_PREEMPTION_ATTEMPT_RESULT = 14;
pub const D3DKMT_PreemptionAttemptMissLessPriority: D3DKMT_QUERYRESULT_PREEMPTION_ATTEMPT_RESULT = 10;
pub const D3DKMT_PreemptionAttemptMissNextFence: D3DKMT_QUERYRESULT_PREEMPTION_ATTEMPT_RESULT = 4;
pub const D3DKMT_PreemptionAttemptMissNoCommand: D3DKMT_QUERYRESULT_PREEMPTION_ATTEMPT_RESULT = 2;
pub const D3DKMT_PreemptionAttemptMissNotEnabled: D3DKMT_QUERYRESULT_PREEMPTION_ATTEMPT_RESULT = 3;
pub const D3DKMT_PreemptionAttemptMissNotMakingProgress: D3DKMT_QUERYRESULT_PREEMPTION_ATTEMPT_RESULT = 9;
pub const D3DKMT_PreemptionAttemptMissPagingCommand: D3DKMT_QUERYRESULT_PREEMPTION_ATTEMPT_RESULT = 5;
pub const D3DKMT_PreemptionAttemptMissRemainingPreemptionQuantum: D3DKMT_QUERYRESULT_PREEMPTION_ATTEMPT_RESULT = 12;
pub const D3DKMT_PreemptionAttemptMissRemainingQuantum: D3DKMT_QUERYRESULT_PREEMPTION_ATTEMPT_RESULT = 11;
pub const D3DKMT_PreemptionAttemptMissRenderPendingFlip: D3DKMT_QUERYRESULT_PREEMPTION_ATTEMPT_RESULT = 8;
pub const D3DKMT_PreemptionAttemptMissSplittedCommand: D3DKMT_QUERYRESULT_PREEMPTION_ATTEMPT_RESULT = 6;
pub const D3DKMT_PreemptionAttemptStatisticsMax: D3DKMT_QUERYRESULT_PREEMPTION_ATTEMPT_RESULT = 16;
pub const D3DKMT_PreemptionAttemptSuccess: D3DKMT_QUERYRESULT_PREEMPTION_ATTEMPT_RESULT = 1;
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYADAPTERINFO {
    pub hAdapter: super::d3dukmdt::D3DKMT_HANDLE,
    pub Type: KMTQUERYADAPTERINFOTYPE,
    pub pPrivateDriverData: *mut core::ffi::c_void,
    pub PrivateDriverDataSize: u32,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_QUERYADAPTERINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYALLOCATIONRESIDENCY {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub hResource: super::d3dukmdt::D3DKMT_HANDLE,
    pub phAllocationList: *const super::d3dukmdt::D3DKMT_HANDLE,
    pub AllocationCount: u32,
    pub pResidencyStatus: *mut D3DKMT_ALLOCATIONRESIDENCYSTATUS,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_QUERYALLOCATIONRESIDENCY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "minwindef"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYFEATUREINTERFACE {
    pub hAdapter: super::d3dukmdt::D3DKMT_HANDLE,
    pub FeatureId: super::d3dukmdt::DXGK_FEATURE_ID,
    pub Result: super::d3dukmdt::DXGK_ISFEATUREENABLED_RESULT,
    pub InterfaceTableSize: u32,
    pub InterfaceTable: *const super::minwindef::FARPROC,
}
#[cfg(all(feature = "d3dukmdt", feature = "minwindef"))]
impl Default for D3DKMT_QUERYFEATUREINTERFACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYFSEBLOCK {
    pub AdapterLuid: super::winnt::LUID,
    pub hAdapter: super::d3dukmdt::D3DKMT_HANDLE,
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub Flags: D3DKMT_QUERYFSEBLOCKFLAGS,
}
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_QUERYFSEBLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_QUERYFSEBLOCKFLAGS {
    pub Anonymous: D3DKMT_QUERYFSEBLOCKFLAGS_0,
    pub Value: u32,
}
impl Default for D3DKMT_QUERYFSEBLOCKFLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_QUERYFSEBLOCKFLAGS_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYPROCESSOFFERINFO {
    pub cbSize: u32,
    pub hProcess: super::winnt::HANDLE,
    pub DecommitUniqueness: u64,
    pub DecommittableBytes: u64,
}
#[cfg(feature = "winnt")]
impl Default for D3DKMT_QUERYPROCESSOFFERINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYPROTECTEDSESSIONINFOFROMNTHANDLE {
    pub hNtHandle: super::winnt::HANDLE,
    pub pPrivateDriverData: *const core::ffi::c_void,
    pub PrivateDriverDataSize: u32,
    pub pPrivateRuntimeData: *const core::ffi::c_void,
    pub PrivateRuntimeDataSize: u32,
}
#[cfg(feature = "winnt")]
impl Default for D3DKMT_QUERYPROTECTEDSESSIONINFOFROMNTHANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_QUERYPROTECTEDSESSIONSTATUS {
    pub hHandle: super::d3dukmdt::D3DKMT_HANDLE,
    pub Status: D3DKMT_PROTECTED_SESSION_STATUS,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYREMOTEVIDPNSOURCEFROMGDIDISPLAYNAME {
    pub DeviceName: [u16; 32],
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_QUERYREMOTEVIDPNSOURCEFROMGDIDISPLAYNAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYRESOURCEINFO {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub hGlobalShare: super::d3dukmdt::D3DKMT_HANDLE,
    pub pPrivateRuntimeData: *mut core::ffi::c_void,
    pub PrivateRuntimeDataSize: u32,
    pub TotalPrivateDriverDataSize: u32,
    pub ResourcePrivateDriverDataSize: u32,
    pub NumAllocations: u32,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_QUERYRESOURCEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYRESOURCEINFOFROMNTHANDLE {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub hNtHandle: super::winnt::HANDLE,
    pub pPrivateRuntimeData: *mut core::ffi::c_void,
    pub PrivateRuntimeDataSize: u32,
    pub TotalPrivateDriverDataSize: u32,
    pub ResourcePrivateDriverDataSize: u32,
    pub NumAllocations: u32,
}
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_QUERYRESOURCEINFOFROMNTHANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3DKMT_QUERYRESULT_PREEMPTION_ATTEMPT_RESULT = i32;
pub const D3DKMT_QUERYRESULT_PREEMPTION_ATTEMPT_RESULT_MAX: u32 = 16;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS {
    pub Type: D3DKMT_QUERYSTATISTICS_TYPE,
    pub AdapterLuid: super::winnt::LUID,
    pub hProcess: super::winnt::HANDLE,
    pub QueryResult: D3DKMT_QUERYSTATISTICS_RESULT,
    pub Anonymous: D3DKMT_QUERYSTATISTICS_0,
}
#[cfg(feature = "winnt")]
impl Default for D3DKMT_QUERYSTATISTICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union D3DKMT_QUERYSTATISTICS_0 {
    pub QuerySegment: D3DKMT_QUERYSTATISTICS_QUERY_SEGMENT,
    pub QueryProcessSegment: D3DKMT_QUERYSTATISTICS_QUERY_SEGMENT,
    pub QueryProcessSegmentGroup: D3DKMT_MEMORY_SEGMENT_GROUP,
    pub QueryNode: D3DKMT_QUERYSTATISTICS_QUERY_NODE,
    pub QueryProcessNode: D3DKMT_QUERYSTATISTICS_QUERY_NODE,
    pub QueryVidPnSource: D3DKMT_QUERYSTATISTICS_QUERY_VIDPNSOURCE,
    pub QueryProcessVidPnSource: D3DKMT_QUERYSTATISTICS_QUERY_VIDPNSOURCE,
    pub QueryPhysAdapter: D3DKMT_QUERYSTATISTICS_QUERY_PHYSICAL_ADAPTER,
    pub QueryAdapter2: D3DKMT_QUERYSTATISTICS_QUERY_ADAPTER2,
    pub QuerySegment2: D3DKMT_QUERYSTATISTICS_QUERY_SEGMENT2,
    pub QueryProcessAdapter2: D3DKMT_QUERYSTATISTICS_QUERY_ADAPTER2,
    pub QueryProcessSegment2: D3DKMT_QUERYSTATISTICS_QUERY_SEGMENT2,
    pub QueryProcessSegmentGroup2: D3DKMT_QUERYSTATISTICS_QUERY_PROCESS_SEGMENT_GROUP2,
    pub QuerySegmentUsage: D3DKMT_QUERYSTATISTICS_QUERY_SEGMENT_USAGE,
    pub QuerySegmentGroupUsage: D3DKMT_QUERYSTATISTICS_QUERY_SEGMENT_GROUP_USAGE,
    pub QueryNode2: D3DKMT_QUERYSTATISTICS_QUERY_NODE2,
    pub QueryProcessNode2: D3DKMT_QUERYSTATISTICS_QUERY_NODE2,
}
#[cfg(feature = "winnt")]
impl Default for D3DKMT_QUERYSTATISTICS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3DKMT_QUERYSTATISTICS_ADAPTER: D3DKMT_QUERYSTATISTICS_TYPE = 0;
pub const D3DKMT_QUERYSTATISTICS_ADAPTER2: D3DKMT_QUERYSTATISTICS_TYPE = 11;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_ADAPTER_INFORMATION {
    pub NbSegments: u32,
    pub NodeCount: u32,
    pub VidPnSourceCount: u32,
    pub VSyncEnabled: u32,
    pub TdrDetectedCount: u32,
    pub ZeroLengthDmaBuffers: i64,
    pub RestartedPeriod: u64,
    pub ReferenceDmaBuffer: D3DKMT_QUERYSTATSTICS_REFERENCE_DMA_BUFFER,
    pub Renaming: D3DKMT_QUERYSTATSTICS_RENAMING,
    pub Preparation: D3DKMT_QUERYSTATSTICS_PREPRATION,
    pub PagingFault: D3DKMT_QUERYSTATSTICS_PAGING_FAULT,
    pub PagingTransfer: D3DKMT_QUERYSTATSTICS_PAGING_TRANSFER,
    pub SwizzlingRange: D3DKMT_QUERYSTATSTICS_SWIZZLING_RANGE,
    pub Locks: D3DKMT_QUERYSTATSTICS_LOCKS,
    pub Allocations: D3DKMT_QUERYSTATSTICS_ALLOCATIONS,
    pub Terminations: D3DKMT_QUERYSTATSTICS_TERMINATIONS,
    pub Flags: D3DKMT_QUERYSTATISTICS_ADAPTER_INFORMATION_FLAGS,
    pub Reserved: [u64; 7],
}
impl Default for D3DKMT_QUERYSTATISTICS_ADAPTER_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_ADAPTER_INFORMATION_FLAGS {
    pub Anonymous: D3DKMT_QUERYSTATISTICS_ADAPTER_INFORMATION_FLAGS_0,
}
impl Default for D3DKMT_QUERYSTATISTICS_ADAPTER_INFORMATION_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_QUERYSTATISTICS_ADAPTER_INFORMATION_FLAGS_0 {
    pub Anonymous: D3DKMT_QUERYSTATISTICS_ADAPTER_INFORMATION_FLAGS_0_0,
    pub Value: u64,
}
impl Default for D3DKMT_QUERYSTATISTICS_ADAPTER_INFORMATION_FLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_QUERYSTATISTICS_ADAPTER_INFORMATION_FLAGS_0_0 {
    pub _bitfield: u64,
}
pub type D3DKMT_QUERYSTATISTICS_ALLOCATION_PRIORITY_CLASS = i32;
pub const D3DKMT_QUERYSTATISTICS_ALLOCATION_PRIORITY_CLASS_MAX: u32 = 5;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_COMMITMENT_DATA {
    pub TotalBytesEvictedFromProcess: u64,
    pub BytesBySegmentPreference: [u64; 5],
}
impl Default for D3DKMT_QUERYSTATISTICS_COMMITMENT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_QUERYSTATISTICS_COUNTER {
    pub Count: u32,
    pub Bytes: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_QUERYSTATISTICS_DMA_BUFFER {
    pub Size: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub AllocationListBytes: u32,
    pub PatchLocationListBytes: u32,
}
pub type D3DKMT_QUERYSTATISTICS_DMA_PACKET_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_QUERYSTATISTICS_DMA_PACKET_TYPE_INFORMATION {
    pub PacketSubmited: u32,
    pub PacketCompleted: u32,
    pub PacketPreempted: u32,
    pub PacketFaulted: u32,
}
pub const D3DKMT_QUERYSTATISTICS_DMA_PACKET_TYPE_MAX: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_QUERYSTATISTICS_MEMORY {
    pub TotalBytesEvicted: u64,
    pub AllocsCommitted: u32,
    pub AllocsResident: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_QUERYSTATISTICS_MEMORY_USAGE {
    pub AllocatedBytes: u64,
    pub FreeBytes: u64,
    pub ZeroBytes: u64,
    pub ModifiedBytes: u64,
    pub StandbyBytes: u64,
}
pub const D3DKMT_QUERYSTATISTICS_NODE: D3DKMT_QUERYSTATISTICS_TYPE = 5;
pub const D3DKMT_QUERYSTATISTICS_NODE2: D3DKMT_QUERYSTATISTICS_TYPE = 18;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_NODE_INFORMATION {
    pub GlobalInformation: D3DKMT_QUERYSTATISTICS_PROCESS_NODE_INFORMATION,
    pub SystemInformation: D3DKMT_QUERYSTATISTICS_PROCESS_NODE_INFORMATION,
    pub NodePerfData: D3DKMT_NODE_PERFDATA,
    pub Reserved: [u32; 3],
}
impl Default for D3DKMT_QUERYSTATISTICS_NODE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_PACKET_INFORMATION {
    pub QueuePacket: [D3DKMT_QUERYSTATISTICS_QUEUE_PACKET_TYPE_INFORMATION; 8],
    pub DmaPacket: [D3DKMT_QUERYSTATISTICS_DMA_PACKET_TYPE_INFORMATION; 4],
}
impl Default for D3DKMT_QUERYSTATISTICS_PACKET_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3DKMT_QUERYSTATISTICS_PHYSICAL_ADAPTER: D3DKMT_QUERYSTATISTICS_TYPE = 10;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_QUERYSTATISTICS_PHYSICAL_ADAPTER_INFORMATION {
    pub AdapterPerfData: D3DKMT_ADAPTER_PERFDATA,
    pub AdapterPerfDataCaps: D3DKMT_ADAPTER_PERFDATACAPS,
    pub GpuVersion: D3DKMT_GPUVERSION,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_POLICY {
    pub PreferApertureForRead: [u64; 5],
    pub PreferAperture: [u64; 5],
    pub MemResetOnPaging: u64,
    pub RemovePagesFromWorkingSetOnPaging: u64,
    pub MigrationEnabled: u64,
}
impl Default for D3DKMT_QUERYSTATISTICS_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_PREEMPTION_INFORMATION {
    pub PreemptionCounter: [u32; 16],
}
impl Default for D3DKMT_QUERYSTATISTICS_PREEMPTION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3DKMT_QUERYSTATISTICS_PROCESS: D3DKMT_QUERYSTATISTICS_TYPE = 1;
pub const D3DKMT_QUERYSTATISTICS_PROCESS_ADAPTER: D3DKMT_QUERYSTATISTICS_TYPE = 2;
pub const D3DKMT_QUERYSTATISTICS_PROCESS_ADAPTER2: D3DKMT_QUERYSTATISTICS_TYPE = 13;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_QUERYSTATISTICS_PROCESS_ADAPTER_INFORMATION {
    pub NbSegments: u32,
    pub NodeCount: u32,
    pub VidPnSourceCount: u32,
    pub VirtualMemoryUsage: u32,
    pub DmaBuffer: D3DKMT_QUERYSTATISTICS_DMA_BUFFER,
    pub CommitmentData: D3DKMT_QUERYSTATISTICS_COMMITMENT_DATA,
    pub _Policy: D3DKMT_QUERYSTATISTICS_POLICY,
    pub ProcessInterferenceCounters: D3DKMT_QUERYSTATISTICS_PROCESS_INTERFERENCE_COUNTERS,
    pub ClientHint: D3DKMT_CLIENTHINT,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_PROCESS_INFORMATION {
    pub NodeCount: u32,
    pub VidPnSourceCount: u32,
    pub SystemMemory: D3DKMT_QUERYSTATISTICS_SYSTEM_MEMORY,
    pub Reserved: [u64; 7],
}
impl Default for D3DKMT_QUERYSTATISTICS_PROCESS_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3DKMT_QUERYSTATISTICS_PROCESS_INTERFERENCE_BUCKET_COUNT: u32 = 9;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_PROCESS_INTERFERENCE_COUNTERS {
    pub InterferenceCount: [u64; 9],
}
impl Default for D3DKMT_QUERYSTATISTICS_PROCESS_INTERFERENCE_COUNTERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3DKMT_QUERYSTATISTICS_PROCESS_NODE: D3DKMT_QUERYSTATISTICS_TYPE = 6;
pub const D3DKMT_QUERYSTATISTICS_PROCESS_NODE2: D3DKMT_QUERYSTATISTICS_TYPE = 19;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_PROCESS_NODE_INFORMATION {
    pub RunningTime: i64,
    pub ContextSwitch: u32,
    pub PreemptionStatistics: D3DKMT_QUERYSTATISTICS_PREEMPTION_INFORMATION,
    pub PacketStatistics: D3DKMT_QUERYSTATISTICS_PACKET_INFORMATION,
    pub Reserved: [u64; 8],
}
impl Default for D3DKMT_QUERYSTATISTICS_PROCESS_NODE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3DKMT_QUERYSTATISTICS_PROCESS_SEGMENT: D3DKMT_QUERYSTATISTICS_TYPE = 4;
pub const D3DKMT_QUERYSTATISTICS_PROCESS_SEGMENT2: D3DKMT_QUERYSTATISTICS_TYPE = 14;
pub const D3DKMT_QUERYSTATISTICS_PROCESS_SEGMENT_GROUP: D3DKMT_QUERYSTATISTICS_TYPE = 9;
pub const D3DKMT_QUERYSTATISTICS_PROCESS_SEGMENT_GROUP2: D3DKMT_QUERYSTATISTICS_TYPE = 15;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_PROCESS_SEGMENT_GROUP_INFORMATION {
    pub Budget: u64,
    pub Requested: u64,
    pub Usage: u64,
    pub Demoted: [u64; 5],
}
impl Default for D3DKMT_QUERYSTATISTICS_PROCESS_SEGMENT_GROUP_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_PROCESS_SEGMENT_INFORMATION {
    pub BytesCommitted: u64,
    pub MaximumWorkingSet: u64,
    pub MinimumWorkingSet: u64,
    pub NbReferencedAllocationEvictedInPeriod: u32,
    pub Padding: u32,
    pub VideoMemory: D3DKMT_QUERYSTATISTICS_VIDEO_MEMORY,
    pub _Policy: D3DKMT_QUERYSTATISTICS_PROCESS_SEGMENT_POLICY,
    pub Reserved: [u64; 8],
}
impl Default for D3DKMT_QUERYSTATISTICS_PROCESS_SEGMENT_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_QUERYSTATISTICS_PROCESS_SEGMENT_POLICY {
    pub UseMRU: u64,
}
pub const D3DKMT_QUERYSTATISTICS_PROCESS_VIDPNSOURCE: D3DKMT_QUERYSTATISTICS_TYPE = 8;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_PROCESS_VIDPNSOURCE_INFORMATION {
    pub Frame: u32,
    pub CancelledFrame: u32,
    pub QueuedPresent: u32,
    pub Padding: u32,
    pub IsVSyncEnabled: u64,
    pub VSyncOnTotalTimeMs: u64,
    pub VSyncOffKeepPhaseTotalTimeMs: u64,
    pub VSyncOffNoPhaseTotalTimeMs: u64,
    pub Reserved: [u64; 4],
}
impl Default for D3DKMT_QUERYSTATISTICS_PROCESS_VIDPNSOURCE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_QUERYSTATISTICS_QUERY_ADAPTER2 {
    pub PhysicalAdapterIndex: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_QUERYSTATISTICS_QUERY_ADAPTER_INFORMATION2 {
    pub PhysicalAdapterIndex: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_QUERYSTATISTICS_QUERY_NODE {
    pub NodeId: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_QUERYSTATISTICS_QUERY_NODE2 {
    pub PhysicalAdapterIndex: u16,
    pub NodeOrdinal: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_QUERYSTATISTICS_QUERY_PHYSICAL_ADAPTER {
    pub PhysicalAdapterIndex: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_QUERYSTATISTICS_QUERY_PROCESS_SEGMENT_GROUP2 {
    pub PhysicalAdapterIndex: u16,
    pub SegmentGroup: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_QUERYSTATISTICS_QUERY_SEGMENT {
    pub SegmentId: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_QUERYSTATISTICS_QUERY_SEGMENT2 {
    pub PhysicalAdapterIndex: u16,
    pub SegmentId: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_QUERYSTATISTICS_QUERY_SEGMENT_GROUP_USAGE {
    pub PhysicalAdapterIndex: u16,
    pub SegmentGroup: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_QUERYSTATISTICS_QUERY_SEGMENT_USAGE {
    pub PhysicalAdapterIndex: u16,
    pub SegmentId: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_QUERYSTATISTICS_QUERY_VIDPNSOURCE {
    pub VidPnSourceId: u32,
}
pub type D3DKMT_QUERYSTATISTICS_QUEUE_PACKET_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_QUERYSTATISTICS_QUEUE_PACKET_TYPE_INFORMATION {
    pub PacketSubmited: u32,
    pub PacketCompleted: u32,
}
pub const D3DKMT_QUERYSTATISTICS_QUEUE_PACKET_TYPE_MAX: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_QUERYSTATISTICS_RESULT {
    pub AdapterInformation: D3DKMT_QUERYSTATISTICS_ADAPTER_INFORMATION,
    pub PhysAdapterInformation: D3DKMT_QUERYSTATISTICS_PHYSICAL_ADAPTER_INFORMATION,
    pub SegmentInformation: D3DKMT_QUERYSTATISTICS_SEGMENT_INFORMATION,
    pub NodeInformation: D3DKMT_QUERYSTATISTICS_NODE_INFORMATION,
    pub VidPnSourceInformation: D3DKMT_QUERYSTATISTICS_VIDPNSOURCE_INFORMATION,
    pub ProcessInformation: D3DKMT_QUERYSTATISTICS_PROCESS_INFORMATION,
    pub ProcessAdapterInformation: D3DKMT_QUERYSTATISTICS_PROCESS_ADAPTER_INFORMATION,
    pub ProcessSegmentInformation: D3DKMT_QUERYSTATISTICS_PROCESS_SEGMENT_INFORMATION,
    pub ProcessNodeInformation: D3DKMT_QUERYSTATISTICS_PROCESS_NODE_INFORMATION,
    pub ProcessVidPnSourceInformation: D3DKMT_QUERYSTATISTICS_PROCESS_VIDPNSOURCE_INFORMATION,
    pub ProcessSegmentGroupInformation: D3DKMT_QUERYSTATISTICS_PROCESS_SEGMENT_GROUP_INFORMATION,
    pub SegmentUsageInformation: D3DKMT_QUERYSTATISTICS_MEMORY_USAGE,
    pub SegmentGroupUsageInformation: D3DKMT_QUERYSTATISTICS_MEMORY_USAGE,
}
impl Default for D3DKMT_QUERYSTATISTICS_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3DKMT_QUERYSTATISTICS_SEGMENT: D3DKMT_QUERYSTATISTICS_TYPE = 3;
pub const D3DKMT_QUERYSTATISTICS_SEGMENT2: D3DKMT_QUERYSTATISTICS_TYPE = 12;
pub const D3DKMT_QUERYSTATISTICS_SEGMENT_GROUP_USAGE: D3DKMT_QUERYSTATISTICS_TYPE = 17;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_SEGMENT_INFORMATION {
    pub CommitLimit: u64,
    pub BytesCommitted: u64,
    pub BytesResident: u64,
    pub Memory: D3DKMT_QUERYSTATISTICS_MEMORY,
    pub Aperture: u32,
    pub TotalBytesEvictedByPriority: [u64; 5],
    pub SystemMemoryEndAddress: u64,
    pub PowerFlags: D3DKMT_QUERYSTATISTICS_SEGMENT_INFORMATION_0,
    pub SegmentProperties: D3DKMT_QUERYSTATISTICS_SEGMENT_INFORMATION_1,
    pub Reserved: [u64; 5],
}
impl Default for D3DKMT_QUERYSTATISTICS_SEGMENT_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_QUERYSTATISTICS_SEGMENT_INFORMATION_0 {
    pub _bitfield: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_QUERYSTATISTICS_SEGMENT_INFORMATION_1 {
    pub _bitfield: u64,
}
pub const D3DKMT_QUERYSTATISTICS_SEGMENT_PREFERENCE_MAX: u32 = 5;
pub type D3DKMT_QUERYSTATISTICS_SEGMENT_TYPE = i32;
pub const D3DKMT_QUERYSTATISTICS_SEGMENT_TYPE_APERTURE: D3DKMT_QUERYSTATISTICS_SEGMENT_TYPE = 0;
pub const D3DKMT_QUERYSTATISTICS_SEGMENT_TYPE_MEMORY: D3DKMT_QUERYSTATISTICS_SEGMENT_TYPE = 1;
pub const D3DKMT_QUERYSTATISTICS_SEGMENT_TYPE_SYSMEM: D3DKMT_QUERYSTATISTICS_SEGMENT_TYPE = 2;
pub const D3DKMT_QUERYSTATISTICS_SEGMENT_USAGE: D3DKMT_QUERYSTATISTICS_TYPE = 16;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_QUERYSTATISTICS_SYSTEM_MEMORY {
    pub BytesAllocated: u64,
    pub BytesReserved: u64,
    pub SmallAllocationBlocks: u32,
    pub LargeAllocationBlocks: u32,
    pub WriteCombinedBytesAllocated: u64,
    pub WriteCombinedBytesReserved: u64,
    pub CachedBytesAllocated: u64,
    pub CachedBytesReserved: u64,
    pub SectionBytesAllocated: u64,
    pub SectionBytesReserved: u64,
    pub BytesZeroed: u64,
}
pub type D3DKMT_QUERYSTATISTICS_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_VIDEO_MEMORY {
    pub AllocsCommitted: u32,
    pub AllocsResidentInP: [D3DKMT_QUERYSTATISTICS_COUNTER; 5],
    pub AllocsResidentInNonPreferred: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub TotalBytesEvictedDueToPreparation: u64,
}
impl Default for D3DKMT_QUERYSTATISTICS_VIDEO_MEMORY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3DKMT_QUERYSTATISTICS_VIDPNSOURCE: D3DKMT_QUERYSTATISTICS_TYPE = 7;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYSTATISTICS_VIDPNSOURCE_INFORMATION {
    pub GlobalInformation: D3DKMT_QUERYSTATISTICS_PROCESS_VIDPNSOURCE_INFORMATION,
    pub SystemInformation: D3DKMT_QUERYSTATISTICS_PROCESS_VIDPNSOURCE_INFORMATION,
    pub Reserved: [u64; 8],
}
impl Default for D3DKMT_QUERYSTATISTICS_VIDPNSOURCE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_QUERYSTATSTICS_ALLOCATIONS {
    pub Created: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub Destroyed: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub Opened: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub Closed: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub MigratedSuccess: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub MigratedFail: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub MigratedAbandoned: D3DKMT_QUERYSTATISTICS_COUNTER,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_QUERYSTATSTICS_LOCKS {
    pub NbLocks: u32,
    pub NbLocksWaitFlag: u32,
    pub NbLocksDiscardFlag: u32,
    pub NbLocksNoOverwrite: u32,
    pub NbLocksNoReadSync: u32,
    pub NbLocksLinearization: u32,
    pub NbComplexLocks: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_QUERYSTATSTICS_PAGING_FAULT {
    pub Faults: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub FaultsFirstTimeAccess: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub FaultsReclaimed: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub FaultsMigration: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub FaultsIncorrectResource: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub FaultsLostContent: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub FaultsEvicted: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub AllocationsMEM_RESET: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub AllocationsUnresetSuccess: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub AllocationsUnresetFail: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub AllocationsUnresetSuccessRead: u32,
    pub AllocationsUnresetFailRead: u32,
    pub Evictions: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub EvictionsDueToPreparation: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub EvictionsDueToLock: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub EvictionsDueToClose: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub EvictionsDueToPurge: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub EvictionsDueToSuspendCPUAccess: D3DKMT_QUERYSTATISTICS_COUNTER,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_QUERYSTATSTICS_PAGING_TRANSFER {
    pub BytesFilled: u64,
    pub BytesDiscarded: u64,
    pub BytesMappedIntoAperture: u64,
    pub BytesUnmappedFromAperture: u64,
    pub BytesTransferredFromMdlToMemory: u64,
    pub BytesTransferredFromMemoryToMdl: u64,
    pub BytesTransferredFromApertureToMemory: u64,
    pub BytesTransferredFromMemoryToAperture: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_QUERYSTATSTICS_PREPRATION {
    pub BroadcastStall: u32,
    pub NbDMAPrepared: u32,
    pub NbDMAPreparedLongPath: u32,
    pub ImmediateHighestPreparationPass: u32,
    pub AllocationsTrimmed: D3DKMT_QUERYSTATISTICS_COUNTER,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_QUERYSTATSTICS_REFERENCE_DMA_BUFFER {
    pub NbCall: u32,
    pub NbAllocationsReferenced: u32,
    pub MaxNbAllocationsReferenced: u32,
    pub NbNULLReference: u32,
    pub NbWriteReference: u32,
    pub NbRenamedAllocationsReferenced: u32,
    pub NbIterationSearchingRenamedAllocation: u32,
    pub NbLockedAllocationReferenced: u32,
    pub NbAllocationWithValidPrepatchingInfoReferenced: u32,
    pub NbAllocationWithInvalidPrepatchingInfoReferenced: u32,
    pub NbDMABufferSuccessfullyPrePatched: u32,
    pub NbPrimariesReferencesOverflow: u32,
    pub NbAllocationWithNonPreferredResources: u32,
    pub NbAllocationInsertedInMigrationTable: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_QUERYSTATSTICS_RENAMING {
    pub NbAllocationsRenamed: u32,
    pub NbAllocationsShrinked: u32,
    pub NbRenamedBuffer: u32,
    pub MaxRenamingListLength: u32,
    pub NbFailuresDueToRenamingLimit: u32,
    pub NbFailuresDueToCreateAllocation: u32,
    pub NbFailuresDueToOpenAllocation: u32,
    pub NbFailuresDueToLowResource: u32,
    pub NbFailuresDueToNonRetiredLimit: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_QUERYSTATSTICS_SWIZZLING_RANGE {
    pub NbRangesAcquired: u32,
    pub NbRangesReleased: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_QUERYSTATSTICS_TERMINATIONS {
    pub TerminatedShared: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub TerminatedNonShared: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub DestroyedShared: D3DKMT_QUERYSTATISTICS_COUNTER,
    pub DestroyedNonShared: D3DKMT_QUERYSTATISTICS_COUNTER,
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYVIDEOMEMORYINFO {
    pub hProcess: super::winnt::HANDLE,
    pub hAdapter: super::d3dukmdt::D3DKMT_HANDLE,
    pub MemorySegmentGroup: D3DKMT_MEMORY_SEGMENT_GROUP,
    pub Budget: u64,
    pub CurrentUsage: u64,
    pub CurrentReservation: u64,
    pub AvailableForReservation: u64,
    pub PhysicalAdapterIndex: u32,
}
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_QUERYVIDEOMEMORYINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERYVIDPNEXCLUSIVEOWNERSHIP {
    pub hProcess: super::winnt::HANDLE,
    pub hWindow: super::windef::HWND,
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub AdapterLuid: super::winnt::LUID,
    pub OwnerType: D3DKMT_VIDPNSOURCEOWNER_TYPE,
}
#[cfg(all(feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
impl Default for D3DKMT_QUERYVIDPNEXCLUSIVEOWNERSHIP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERY_ADAPTER_UNIQUE_GUID {
    pub AdapterUniqueGUID: [u16; 40],
}
impl Default for D3DKMT_QUERY_ADAPTER_UNIQUE_GUID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_QUERY_DEVICE_IDS {
    pub PhysicalAdapterIndex: u32,
    pub DeviceIds: D3DKMT_DEVICE_IDS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERY_GPUMMU_CAPS {
    pub PhysicalAdapterIndex: u32,
    pub Caps: D3DKMT_GPUMMU_CAPS,
}
impl Default for D3DKMT_QUERY_GPUMMU_CAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_QUERY_MIRACAST_DRIVER_TYPE {
    pub MiracastDriverType: D3DKMT_MIRACAST_DRIVER_TYPE,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_QUERY_PHYSICAL_ADAPTER {
    pub PhysicalAdapterIndex: u32,
    pub NumExecutionNodes: u32,
    pub PagingNodeIndex: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_QUERY_PHYSICAL_ADAPTER_1 {
    pub PhysicalAdapterIndex: u32,
    pub NumExecutionNodes: u32,
    pub PagingNodeIndex: u32,
    pub GdiNodeIndex: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_QUERY_PHYSICAL_ADAPTER_PNP_KEY {
    pub PhysicalAdapterIndex: u32,
    pub PnPKeyType: D3DKMT_PNP_KEY_TYPE,
    pub pDest: *mut u16,
    pub pCchDest: *mut u32,
}
impl Default for D3DKMT_QUERY_PHYSICAL_ADAPTER_PNP_KEY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_QUERY_SCANOUT_CAPS {
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub Caps: u32,
}
pub type D3DKMT_QUEUEDLIMIT_TYPE = i32;
pub const D3DKMT_QueuePacketTypeMax: D3DKMT_QUERYSTATISTICS_QUEUE_PACKET_TYPE = 8;
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_RECLAIMALLOCATIONS {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub pResources: *mut super::d3dukmdt::D3DKMT_HANDLE,
    pub HandleList: *const super::d3dukmdt::D3DKMT_HANDLE,
    pub pDiscarded: *mut windows_sys::core::BOOL,
    pub NumAllocations: u32,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_RECLAIMALLOCATIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_RECLAIMALLOCATIONS2 {
    pub hPagingQueue: super::d3dukmdt::D3DKMT_HANDLE,
    pub NumAllocations: u32,
    pub pResources: *mut super::d3dukmdt::D3DKMT_HANDLE,
    pub HandleList: *const super::d3dukmdt::D3DKMT_HANDLE,
    pub Anonymous: D3DKMT_RECLAIMALLOCATIONS2_0,
    pub PagingFenceValue: u64,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_RECLAIMALLOCATIONS2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub union D3DKMT_RECLAIMALLOCATIONS2_0 {
    pub pDiscarded: *mut windows_sys::core::BOOL,
    pub pResults: *mut super::d3dukmdt::D3DDDI_RECLAIM_RESULT,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_RECLAIMALLOCATIONS2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_REGISTERBUDGETCHANGENOTIFICATION {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub Callback: PFND3DKMT_BUDGETCHANGENOTIFICATIONCALLBACK,
    pub Context: *mut core::ffi::c_void,
    pub Handle: *mut core::ffi::c_void,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_REGISTERBUDGETCHANGENOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_REGISTERTRIMNOTIFICATION {
    pub AdapterLuid: super::winnt::LUID,
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub Callback: PFND3DKMT_TRIMNOTIFICATIONCALLBACK,
    pub Context: *mut core::ffi::c_void,
    pub Handle: *mut core::ffi::c_void,
}
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_REGISTERTRIMNOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_RELEASEKEYEDMUTEX {
    pub hKeyedMutex: super::d3dukmdt::D3DKMT_HANDLE,
    pub Key: u64,
    pub FenceValue: u64,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_RELEASEKEYEDMUTEX2 {
    pub hKeyedMutex: super::d3dukmdt::D3DKMT_HANDLE,
    pub Key: u64,
    pub FenceValue: u64,
    pub pPrivateRuntimeData: *mut core::ffi::c_void,
    pub PrivateRuntimeDataSize: u32,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_RELEASEKEYEDMUTEX2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_RENDER {
    pub Anonymous: D3DKMT_RENDER_0,
    pub CommandOffset: u32,
    pub CommandLength: u32,
    pub AllocationCount: u32,
    pub PatchLocationCount: u32,
    pub pNewCommandBuffer: *mut core::ffi::c_void,
    pub NewCommandBufferSize: u32,
    pub pNewAllocationList: *mut super::d3dukmdt::D3DDDI_ALLOCATIONLIST,
    pub NewAllocationListSize: u32,
    pub pNewPatchLocationList: *mut super::d3dukmdt::D3DDDI_PATCHLOCATIONLIST,
    pub NewPatchLocationListSize: u32,
    pub Flags: D3DKMT_RENDERFLAGS,
    pub PresentHistoryToken: u64,
    pub BroadcastContextCount: u32,
    pub BroadcastContext: [super::d3dukmdt::D3DKMT_HANDLE; 64],
    pub QueuedBufferCount: u32,
    pub NewCommandBuffer: super::d3dukmdt::D3DGPU_VIRTUAL_ADDRESS,
    pub pPrivateDriverData: *mut core::ffi::c_void,
    pub PrivateDriverDataSize: u32,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_RENDER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub union D3DKMT_RENDER_0 {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub hContext: super::d3dukmdt::D3DKMT_HANDLE,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_RENDER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_RENDERFLAGS {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_REQUEST_MACHINE_CRASH_ESCAPE {
    pub Param1: super::d3dukmdt::D3DKMT_ULONG_PTR,
    pub Param2: super::d3dukmdt::D3DKMT_ULONG_PTR,
    pub Param3: super::d3dukmdt::D3DKMT_ULONG_PTR,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_RESIZERINGBUFFER {
    pub hHwQueue: super::d3dukmdt::D3DKMT_HANDLE,
    pub hRingBuffer: super::d3dukmdt::D3DKMT_HANDLE,
    pub hRingBufferControl: super::d3dukmdt::D3DKMT_HANDLE,
    pub Flags: D3DKMT_RESIZERINGBUFFER_FLAGS,
    pub PrivateDriverData: [u8; 64],
    pub Reserved: [u8; 64],
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_RESIZERINGBUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_RESIZERINGBUFFER_FLAGS {
    pub Anonymous: D3DKMT_RESIZERINGBUFFER_FLAGS_0,
}
impl Default for D3DKMT_RESIZERINGBUFFER_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_RESIZERINGBUFFER_FLAGS_0 {
    pub Anonymous: D3DKMT_RESIZERINGBUFFER_FLAGS_0_0,
    pub Value: u32,
}
impl Default for D3DKMT_RESIZERINGBUFFER_FLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_RESIZERINGBUFFER_FLAGS_0_0 {
    pub _bitfield: u32,
}
pub const D3DKMT_RenderCommandBuffer: D3DKMT_QUERYSTATISTICS_QUEUE_PACKET_TYPE = 0;
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_SCATTERBLT {
    pub hLogicalSurfaceDestination: u64,
    pub hDestinationCompSurfDWM: i64,
    pub DestinationCompositionBindingId: u64,
    pub SourceRect: super::windef::RECT,
    pub DestinationOffset: super::windef::POINT,
}
#[repr(C)]
#[cfg(feature = "windef")]
#[derive(Clone, Copy)]
pub struct D3DKMT_SCATTERBLTS {
    pub NumBlts: u32,
    pub Blts: [D3DKMT_SCATTERBLT; 12],
}
#[cfg(feature = "windef")]
impl Default for D3DKMT_SCATTERBLTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3DKMT_SCHEDULINGPRIORITYCLASS = i32;
pub const D3DKMT_SCHEDULINGPRIORITYCLASS_ABOVE_NORMAL: D3DKMT_SCHEDULINGPRIORITYCLASS = 3;
pub const D3DKMT_SCHEDULINGPRIORITYCLASS_BELOW_NORMAL: D3DKMT_SCHEDULINGPRIORITYCLASS = 1;
pub const D3DKMT_SCHEDULINGPRIORITYCLASS_HIGH: D3DKMT_SCHEDULINGPRIORITYCLASS = 4;
pub const D3DKMT_SCHEDULINGPRIORITYCLASS_IDLE: D3DKMT_SCHEDULINGPRIORITYCLASS = 0;
pub const D3DKMT_SCHEDULINGPRIORITYCLASS_NORMAL: D3DKMT_SCHEDULINGPRIORITYCLASS = 2;
pub const D3DKMT_SCHEDULINGPRIORITYCLASS_REALTIME: D3DKMT_SCHEDULINGPRIORITYCLASS = 5;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_SEGMENTGROUPSIZEINFO {
    pub PhysicalAdapterIndex: u32,
    pub LegacyInfo: D3DKMT_SEGMENTSIZEINFO,
    pub LocalMemory: u64,
    pub NonLocalMemory: u64,
    pub NonBudgetMemory: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_SEGMENTSIZEINFO {
    pub DedicatedVideoMemorySize: u64,
    pub DedicatedSystemMemorySize: u64,
    pub SharedSystemMemorySize: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_SEGMENT_CAPS {
    pub Size: u64,
    pub PageSize: u32,
    pub SegmentId: u32,
    pub bAperture: bool,
    pub bReservedSysMem: bool,
    pub BudgetGroup: D3DKMT_MEMORY_SEGMENT_GROUP,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_SETALLOCATIONPRIORITY {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub hResource: super::d3dukmdt::D3DKMT_HANDLE,
    pub phAllocationList: *const super::d3dukmdt::D3DKMT_HANDLE,
    pub AllocationCount: u32,
    pub pPriorities: *const u32,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_SETALLOCATIONPRIORITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_SETCONTEXTINPROCESSSCHEDULINGPRIORITY {
    pub hContext: super::d3dukmdt::D3DKMT_HANDLE,
    pub Priority: i32,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_SETCONTEXTSCHEDULINGPRIORITY {
    pub hContext: super::d3dukmdt::D3DKMT_HANDLE,
    pub Priority: i32,
}
pub const D3DKMT_SETCONTEXTSCHEDULINGPRIORITY_ABSOLUTE: u32 = 1073741824;
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_SETDISPLAYMODE {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub hPrimaryAllocation: super::d3dukmdt::D3DKMT_HANDLE,
    pub ScanLineOrdering: super::d3dukmdt::D3DDDI_VIDEO_SIGNAL_SCANLINE_ORDERING,
    pub DisplayOrientation: super::d3dukmdt::D3DDDI_ROTATION,
    pub PrivateDriverFormatAttribute: u32,
    pub Flags: D3DKMT_SETDISPLAYMODE_FLAGS,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_SETDISPLAYMODE_FLAGS {
    pub _bitfield1: bool,
    pub _bitfield2: u32,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_SETDISPLAYPRIVATEDRIVERFORMAT {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub PrivateDriverFormatAttribute: u32,
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_SETFSEBLOCK {
    pub AdapterLuid: super::winnt::LUID,
    pub hAdapter: super::d3dukmdt::D3DKMT_HANDLE,
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub Flags: D3DKMT_SETFSEBLOCKFLAGS,
}
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_SETFSEBLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_SETFSEBLOCKFLAGS {
    pub Anonymous: D3DKMT_SETFSEBLOCKFLAGS_0,
    pub Value: u32,
}
impl Default for D3DKMT_SETFSEBLOCKFLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_SETFSEBLOCKFLAGS_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_SETGAMMARAMP {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub Type: super::d3dukmdt::D3DDDI_GAMMARAMP_TYPE,
    pub Anonymous: D3DKMT_SETGAMMARAMP_0,
    pub Size: u32,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_SETGAMMARAMP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub union D3DKMT_SETGAMMARAMP_0 {
    pub pGammaRampRgb256x3x16: *mut super::d3dukmdt::D3DDDI_GAMMA_RAMP_RGB256x3x16,
    pub pGammaRampDXGI1: *mut super::d3dukmdt::D3DDDI_GAMMA_RAMP_DXGI_1,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_SETGAMMARAMP_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_SETHWPROTECTIONTEARDOWNRECOVERY {
    pub hAdapter: super::d3dukmdt::D3DKMT_HANDLE,
    pub Recovered: windows_sys::core::BOOL,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_SETQUEUEDLIMIT {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub Type: D3DKMT_QUEUEDLIMIT_TYPE,
    pub Anonymous: D3DKMT_SETQUEUEDLIMIT_0,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_SETQUEUEDLIMIT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub union D3DKMT_SETQUEUEDLIMIT_0 {
    pub QueuedPresentLimit: u32,
    pub Anonymous: D3DKMT_SETQUEUEDLIMIT_0_0,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_SETQUEUEDLIMIT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_SETQUEUEDLIMIT_0_0 {
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub QueuedPendingFlipLimit: u32,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_SETSTABLEPOWERSTATE {
    pub hAdapter: super::d3dukmdt::D3DKMT_HANDLE,
    pub Enabled: windows_sys::core::BOOL,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_SETSYNCREFRESHCOUNTWAITTARGET {
    pub hAdapter: super::d3dukmdt::D3DKMT_HANDLE,
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub TargetSyncRefreshCount: u32,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_SETVIDPNSOURCEHWPROTECTION {
    pub hAdapter: super::d3dukmdt::D3DKMT_HANDLE,
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub HwProtected: windows_sys::core::BOOL,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_SETVIDPNSOURCEOWNER {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub pType: *const D3DKMT_VIDPNSOURCEOWNER_TYPE,
    pub pVidPnSourceId: *const super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub VidPnSourceCount: u32,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_SETVIDPNSOURCEOWNER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_SETVIDPNSOURCEOWNER1 {
    pub Version0: D3DKMT_SETVIDPNSOURCEOWNER,
    pub Flags: D3DKMT_VIDPNSOURCEOWNER_FLAGS,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_SETVIDPNSOURCEOWNER1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_SETVIDPNSOURCEOWNER2 {
    pub Version1: D3DKMT_SETVIDPNSOURCEOWNER1,
    pub pVidPnSourceNtHandles: *const super::d3dukmdt::D3DKMT_PTR_TYPE,
}
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_SETVIDPNSOURCEOWNER2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_SET_COLORSPACE_TRANSFORM {
    pub AdapterLuid: super::winnt::LUID,
    pub VidPnTargetId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_TARGET_ID,
    pub Type: super::d3dukmdt::D3DDDI_GAMMARAMP_TYPE,
    pub Size: u32,
    pub Anonymous: D3DKMT_SET_COLORSPACE_TRANSFORM_0,
}
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_SET_COLORSPACE_TRANSFORM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union D3DKMT_SET_COLORSPACE_TRANSFORM_0 {
    pub pColorSpaceTransform: *mut super::d3dukmdt::D3DKMDT_3x4_COLORSPACE_TRANSFORM,
}
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_SET_COLORSPACE_TRANSFORM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3DKMT_SET_QUEUEDLIMIT_PRESENT: D3DKMT_QUEUEDLIMIT_TYPE = 1;
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_SHAREDPRIMARYLOCKNOTIFICATION {
    pub AdapterLuid: super::winnt::LUID,
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub LockRect: super::windef::RECTL,
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_SHAREDPRIMARYUNLOCKNOTIFICATION {
    pub AdapterLuid: super::winnt::LUID,
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_SHAREOBJECTWITHHOST {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub hObject: super::d3dukmdt::D3DKMT_HANDLE,
    pub Reserved: u64,
    pub hVailProcessNtHandle: u64,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_SIGNALSYNCHRONIZATIONOBJECT {
    pub hContext: super::d3dukmdt::D3DKMT_HANDLE,
    pub ObjectCount: u32,
    pub ObjectHandleArray: [super::d3dukmdt::D3DKMT_HANDLE; 32],
    pub Flags: super::d3dukmdt::D3DDDICB_SIGNALFLAGS,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_SIGNALSYNCHRONIZATIONOBJECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_SIGNALSYNCHRONIZATIONOBJECT2 {
    pub hContext: super::d3dukmdt::D3DKMT_HANDLE,
    pub ObjectCount: u32,
    pub ObjectHandleArray: [super::d3dukmdt::D3DKMT_HANDLE; 32],
    pub Flags: super::d3dukmdt::D3DDDICB_SIGNALFLAGS,
    pub BroadcastContextCount: u32,
    pub BroadcastContext: [super::d3dukmdt::D3DKMT_HANDLE; 64],
    pub Anonymous: D3DKMT_SIGNALSYNCHRONIZATIONOBJECT2_0,
}
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_SIGNALSYNCHRONIZATIONOBJECT2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union D3DKMT_SIGNALSYNCHRONIZATIONOBJECT2_0 {
    pub Fence: D3DKMT_SIGNALSYNCHRONIZATIONOBJECT2_0_0,
    pub CpuEventHandle: super::winnt::HANDLE,
    pub Reserved: [u64; 8],
}
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_SIGNALSYNCHRONIZATIONOBJECT2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_SIGNALSYNCHRONIZATIONOBJECT2_0_0 {
    pub FenceValue: u64,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_SIGNALSYNCHRONIZATIONOBJECTFROMCPU {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub ObjectCount: u32,
    pub ObjectHandleArray: *const super::d3dukmdt::D3DKMT_HANDLE,
    pub FenceValueArray: *const u64,
    pub Flags: super::d3dukmdt::D3DDDICB_SIGNALFLAGS,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_SIGNALSYNCHRONIZATIONOBJECTFROMCPU {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_SIGNALSYNCHRONIZATIONOBJECTFROMGPU {
    pub hContext: super::d3dukmdt::D3DKMT_HANDLE,
    pub ObjectCount: u32,
    pub ObjectHandleArray: *const super::d3dukmdt::D3DKMT_HANDLE,
    pub Anonymous: D3DKMT_SIGNALSYNCHRONIZATIONOBJECTFROMGPU_0,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_SIGNALSYNCHRONIZATIONOBJECTFROMGPU {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub union D3DKMT_SIGNALSYNCHRONIZATIONOBJECTFROMGPU_0 {
    pub MonitoredFenceValueArray: *const u64,
    pub Reserved: [u64; 8],
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_SIGNALSYNCHRONIZATIONOBJECTFROMGPU_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_SIGNALSYNCHRONIZATIONOBJECTFROMGPU2 {
    pub ObjectCount: u32,
    pub ObjectHandleArray: *const super::d3dukmdt::D3DKMT_HANDLE,
    pub Flags: super::d3dukmdt::D3DDDICB_SIGNALFLAGS,
    pub BroadcastContextCount: u32,
    pub BroadcastContextArray: *const super::d3dukmdt::D3DKMT_HANDLE,
    pub Anonymous: D3DKMT_SIGNALSYNCHRONIZATIONOBJECTFROMGPU2_0,
}
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_SIGNALSYNCHRONIZATIONOBJECTFROMGPU2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union D3DKMT_SIGNALSYNCHRONIZATIONOBJECTFROMGPU2_0 {
    pub FenceValue: u64,
    pub CpuEventHandle: super::winnt::HANDLE,
    pub MonitoredFenceValueArray: *const u64,
    pub Reserved: [u64; 8],
}
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_SIGNALSYNCHRONIZATIONOBJECTFROMGPU2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3DKMT_STANDARDALLOCATIONTYPE = i32;
pub const D3DKMT_STANDARDALLOCATIONTYPE_EXISTINGHEAP: D3DKMT_STANDARDALLOCATIONTYPE = 1;
pub const D3DKMT_STANDARDALLOCATIONTYPE_INTERNALBACKINGSTORE: D3DKMT_STANDARDALLOCATIONTYPE = 2;
pub const D3DKMT_STANDARDALLOCATIONTYPE_MAX: D3DKMT_STANDARDALLOCATIONTYPE = 3;
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_STANDARDALLOCATION_EXISTINGHEAP {
    pub Size: super::d3dukmdt::D3DKMT_SIZE_T,
}
pub const D3DKMT_SUBKEY_DX9: windows_sys::core::PCWSTR = windows_sys::core::w!("DX9");
pub const D3DKMT_SUBKEY_OPENGL: windows_sys::core::PCWSTR = windows_sys::core::w!("OpenGL");
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_SUBMITCOMMAND {
    pub Commands: super::d3dukmdt::D3DGPU_VIRTUAL_ADDRESS,
    pub CommandLength: u32,
    pub Flags: D3DKMT_SUBMITCOMMANDFLAGS,
    pub PresentHistoryToken: u64,
    pub BroadcastContextCount: u32,
    pub BroadcastContext: [super::d3dukmdt::D3DKMT_HANDLE; 64],
    pub pPrivateDriverData: *mut core::ffi::c_void,
    pub PrivateDriverDataSize: u32,
    pub NumPrimaries: u32,
    pub WrittenPrimaries: [super::d3dukmdt::D3DKMT_HANDLE; 16],
    pub NumHistoryBuffers: u32,
    pub HistoryBufferArray: *mut super::d3dukmdt::D3DKMT_HANDLE,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_SUBMITCOMMAND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_SUBMITCOMMANDFLAGS {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_SUBMITCOMMANDTOHWQUEUE {
    pub hHwQueue: super::d3dukmdt::D3DKMT_HANDLE,
    pub HwQueueProgressFenceId: u64,
    pub CommandBuffer: super::d3dukmdt::D3DGPU_VIRTUAL_ADDRESS,
    pub CommandLength: u32,
    pub PrivateDriverDataSize: u32,
    pub pPrivateDriverData: *mut core::ffi::c_void,
    pub NumPrimaries: u32,
    pub WrittenPrimaries: *const super::d3dukmdt::D3DKMT_HANDLE,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_SUBMITCOMMANDTOHWQUEUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dkmdt", feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_SUBMITPRESENTBLTTOHWQUEUE {
    pub hHwQueue: super::d3dukmdt::D3DKMT_HANDLE,
    pub HwQueueProgressFenceId: u64,
    pub PrivatePresentData: D3DKMT_PRESENT,
}
#[cfg(all(feature = "d3dkmdt", feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
impl Default for D3DKMT_SUBMITPRESENTBLTTOHWQUEUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "d3dkmdt", feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_SUBMITPRESENTTOHWQUEUE {
    pub hHwQueues: *mut super::d3dukmdt::D3DKMT_HANDLE,
    pub PrivatePresentData: D3DKMT_PRESENT,
}
#[cfg(all(feature = "d3dkmdt", feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
impl Default for D3DKMT_SUBMITPRESENTTOHWQUEUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_SUBMITSIGNALSYNCOBJECTSTOHWQUEUE {
    pub Flags: super::d3dukmdt::D3DDDICB_SIGNALFLAGS,
    pub BroadcastHwQueueCount: u32,
    pub BroadcastHwQueueArray: *const super::d3dukmdt::D3DKMT_HANDLE,
    pub ObjectCount: u32,
    pub ObjectHandleArray: *const super::d3dukmdt::D3DKMT_HANDLE,
    pub FenceValueArray: *const u64,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_SUBMITSIGNALSYNCOBJECTSTOHWQUEUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_SUBMITWAITFORSYNCOBJECTSTOHWQUEUE {
    pub hHwQueue: super::d3dukmdt::D3DKMT_HANDLE,
    pub ObjectCount: u32,
    pub ObjectHandleArray: *const super::d3dukmdt::D3DKMT_HANDLE,
    pub FenceValueArray: *const u64,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_SUBMITWAITFORSYNCOBJECTSTOHWQUEUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_SURFACECOMPLETE_PRESENTHISTORYTOKEN {
    pub hLogicalSurface: u64,
}
pub const D3DKMT_SignalCommandBuffer: D3DKMT_QUERYSTATISTICS_QUEUE_PACKET_TYPE = 5;
pub const D3DKMT_SoftwareCommandBuffer: D3DKMT_QUERYSTATISTICS_QUEUE_PACKET_TYPE = 7;
pub const D3DKMT_SystemCommandBuffer: D3DKMT_QUERYSTATISTICS_QUEUE_PACKET_TYPE = 2;
pub const D3DKMT_SystemPagingBuffer: D3DKMT_QUERYSTATISTICS_DMA_PACKET_TYPE = 2;
pub const D3DKMT_SystemPreemptionBuffer: D3DKMT_QUERYSTATISTICS_DMA_PACKET_TYPE = 3;
pub type D3DKMT_TDRDBGCTRLTYPE = i32;
pub const D3DKMT_TDRDBGCTRLTYPE_DISABLEBREAK: D3DKMT_TDRDBGCTRLTYPE = 1;
pub const D3DKMT_TDRDBGCTRLTYPE_ENABLEBREAK: D3DKMT_TDRDBGCTRLTYPE = 2;
pub const D3DKMT_TDRDBGCTRLTYPE_ENGINETDR: D3DKMT_TDRDBGCTRLTYPE = 8;
pub const D3DKMT_TDRDBGCTRLTYPE_FORCEDODTDR: D3DKMT_TDRDBGCTRLTYPE = 6;
pub const D3DKMT_TDRDBGCTRLTYPE_FORCEDODVSYNCTDR: D3DKMT_TDRDBGCTRLTYPE = 7;
pub const D3DKMT_TDRDBGCTRLTYPE_FORCETDR: D3DKMT_TDRDBGCTRLTYPE = 0;
pub const D3DKMT_TDRDBGCTRLTYPE_GPUTDR: D3DKMT_TDRDBGCTRLTYPE = 5;
pub const D3DKMT_TDRDBGCTRLTYPE_UNCONDITIONAL: D3DKMT_TDRDBGCTRLTYPE = 3;
pub const D3DKMT_TDRDBGCTRLTYPE_VSYNCTDR: D3DKMT_TDRDBGCTRLTYPE = 4;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_TDRDBGCTRL_ESCAPE {
    pub TdrControl: D3DKMT_TDRDBGCTRLTYPE,
    pub Anonymous: D3DKMT_TDRDBGCTRL_ESCAPE_0,
}
impl Default for D3DKMT_TDRDBGCTRL_ESCAPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_TDRDBGCTRL_ESCAPE_0 {
    pub NodeOrdinal: u32,
}
impl Default for D3DKMT_TDRDBGCTRL_ESCAPE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_TRIMNOTIFICATION {
    pub Context: *mut core::ffi::c_void,
    pub Flags: super::d3dukmdt::D3DDDI_TRIMRESIDENCYSET_FLAGS,
    pub NumBytesToTrim: u64,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_TRIMNOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_TRIMPROCESSCOMMITMENT {
    pub cbSize: u32,
    pub hProcess: super::winnt::HANDLE,
    pub Flags: D3DKMT_TRIMPROCESSCOMMITMENT_FLAGS,
    pub DecommitRequested: u64,
    pub NumBytesDecommitted: u64,
}
#[cfg(feature = "winnt")]
impl Default for D3DKMT_TRIMPROCESSCOMMITMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_TRIMPROCESSCOMMITMENT_FLAGS {
    pub Anonymous: D3DKMT_TRIMPROCESSCOMMITMENT_FLAGS_0,
    pub Value: u32,
}
impl Default for D3DKMT_TRIMPROCESSCOMMITMENT_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_TRIMPROCESSCOMMITMENT_FLAGS_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_UMDFILENAMEINFO {
    pub Version: KMTUMDVERSION,
    pub UmdFileName: [u16; 260],
}
impl Default for D3DKMT_UMDFILENAMEINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_UMD_DRIVER_VERSION {
    pub DriverVersion: i64,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_UNLOCK {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub NumAllocations: u32,
    pub phAllocations: *const super::d3dukmdt::D3DKMT_HANDLE,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_UNLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_UNLOCK2 {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub hAllocation: super::d3dukmdt::D3DKMT_HANDLE,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_UNMAPPROCESSDEBUGBLOB {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub pBuffer: *mut core::ffi::c_void,
    pub Flags: D3DKMT_UNMAPPROCESSDEBUGBLOB_FLAGS,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_UNMAPPROCESSDEBUGBLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_UNMAPPROCESSDEBUGBLOB_FLAGS {
    pub Anonymous: D3DKMT_UNMAPPROCESSDEBUGBLOB_FLAGS_0,
}
impl Default for D3DKMT_UNMAPPROCESSDEBUGBLOB_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_UNMAPPROCESSDEBUGBLOB_FLAGS_0 {
    pub Anonymous: D3DKMT_UNMAPPROCESSDEBUGBLOB_FLAGS_0_0,
    pub Value: u32,
}
impl Default for D3DKMT_UNMAPPROCESSDEBUGBLOB_FLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_UNMAPPROCESSDEBUGBLOB_FLAGS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_UNPINDIRECTFLIPRESOURCES {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub ResourceCount: u32,
    pub pResourceList: *mut super::d3dukmdt::D3DKMT_HANDLE,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_UNPINDIRECTFLIPRESOURCES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_UNREGISTERBUDGETCHANGENOTIFICATION {
    pub Handle: *mut core::ffi::c_void,
}
impl Default for D3DKMT_UNREGISTERBUDGETCHANGENOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_UNREGISTERTRIMNOTIFICATION {
    pub Handle: *mut core::ffi::c_void,
    pub Callback: PFND3DKMT_TRIMNOTIFICATIONCALLBACK,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_UNREGISTERTRIMNOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_UPDATEGPUVIRTUALADDRESS {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub hContext: super::d3dukmdt::D3DKMT_HANDLE,
    pub hFenceObject: super::d3dukmdt::D3DKMT_HANDLE,
    pub NumOperations: u32,
    pub Operations: *mut super::d3dukmdt::D3DDDI_UPDATEGPUVIRTUALADDRESS_OPERATION,
    pub Reserved0: super::d3dukmdt::D3DKMT_SIZE_T,
    pub Reserved1: u64,
    pub FenceValue: u64,
    pub Flags: D3DKMT_UPDATEGPUVIRTUALADDRESS_0,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_UPDATEGPUVIRTUALADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub union D3DKMT_UPDATEGPUVIRTUALADDRESS_0 {
    pub Anonymous: D3DKMT_UPDATEGPUVIRTUALADDRESS_0_0,
    pub Value: u32,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_UPDATEGPUVIRTUALADDRESS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_UPDATEGPUVIRTUALADDRESS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_UPDATEOVERLAY {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub hOverlay: super::d3dukmdt::D3DKMT_HANDLE,
    pub OverlayInfo: super::d3dukmdt::D3DDDI_KERNELOVERLAYINFO,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_VAD_DESC {
    pub VadIndex: u32,
    pub VadAddress: u64,
    pub NumMappedRanges: u32,
    pub VadType: u32,
    pub StartAddress: u64,
    pub EndAddress: u64,
}
pub type D3DKMT_VAD_ESCAPE_COMMAND = i32;
pub const D3DKMT_VAD_ESCAPE_GETNUMVADS: D3DKMT_VAD_ESCAPE_COMMAND = 0;
pub const D3DKMT_VAD_ESCAPE_GETVAD: D3DKMT_VAD_ESCAPE_COMMAND = 1;
pub const D3DKMT_VAD_ESCAPE_GETVADRANGE: D3DKMT_VAD_ESCAPE_COMMAND = 2;
pub const D3DKMT_VAD_ESCAPE_GET_GPUMMU_CAPS: D3DKMT_VAD_ESCAPE_COMMAND = 4;
pub const D3DKMT_VAD_ESCAPE_GET_PTE: D3DKMT_VAD_ESCAPE_COMMAND = 3;
pub const D3DKMT_VAD_ESCAPE_GET_PTE_DATA: D3DKMT_VAD_ESCAPE_COMMAND = 6;
pub const D3DKMT_VAD_ESCAPE_GET_SEGMENT_CAPS: D3DKMT_VAD_ESCAPE_COMMAND = 5;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_VA_RANGE_DESC {
    pub VadAddress: u64,
    pub VaRangeIndex: u32,
    pub PhysicalAdapterIndex: u32,
    pub StartAddress: u64,
    pub EndAddress: u64,
    pub DriverProtection: u64,
    pub OwnerType: u32,
    pub pOwner: u64,
    pub OwnerOffset: u64,
    pub Protection: u32,
}
pub type D3DKMT_VERIFIER_OPTION_MODE = i32;
pub const D3DKMT_VERIFIER_OPTION_QUERY: D3DKMT_VERIFIER_OPTION_MODE = 0;
pub const D3DKMT_VERIFIER_OPTION_SET: D3DKMT_VERIFIER_OPTION_MODE = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_VGPUINTERFACEID {
    pub VirtualGpuIntefaceId: [u16; 260],
}
impl Default for D3DKMT_VGPUINTERFACEID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3DKMT_VIDMMESCAPETYPE = i32;
pub const D3DKMT_VIDMMESCAPETYPE_APERTURE_CORRUPTION_CHECK: D3DKMT_VIDMMESCAPETYPE = 3;
pub const D3DKMT_VIDMMESCAPETYPE_COPY_CONTENT: D3DKMT_VIDMMESCAPETYPE = 19;
pub const D3DKMT_VIDMMESCAPETYPE_DEBUG: D3DKMT_VIDMMESCAPETYPE = 20;
pub const D3DKMT_VIDMMESCAPETYPE_DEFRAG: D3DKMT_VIDMMESCAPETYPE = 15;
pub const D3DKMT_VIDMMESCAPETYPE_DELAYEXECUTION: D3DKMT_VIDMMESCAPETYPE = 16;
pub const D3DKMT_VIDMMESCAPETYPE_EVICT: D3DKMT_VIDMMESCAPETYPE = 5;
pub const D3DKMT_VIDMMESCAPETYPE_EVICT_BY_CRITERIA: D3DKMT_VIDMMESCAPETYPE = 13;
pub const D3DKMT_VIDMMESCAPETYPE_EVICT_BY_NT_HANDLE: D3DKMT_VIDMMESCAPETYPE = 6;
pub const D3DKMT_VIDMMESCAPETYPE_GET_BUDGET: D3DKMT_VIDMMESCAPETYPE = 11;
pub const D3DKMT_VIDMMESCAPETYPE_GET_VAD_INFO: D3DKMT_VIDMMESCAPETYPE = 7;
pub const D3DKMT_VIDMMESCAPETYPE_QUERYSECTION: D3DKMT_VIDMMESCAPETYPE = 21;
pub const D3DKMT_VIDMMESCAPETYPE_RESUME_PROCESS: D3DKMT_VIDMMESCAPETYPE = 10;
pub const D3DKMT_VIDMMESCAPETYPE_RUN_COHERENCY_TEST: D3DKMT_VIDMMESCAPETYPE = 1;
pub const D3DKMT_VIDMMESCAPETYPE_RUN_UNMAP_TO_DUMMY_PAGE_TEST: D3DKMT_VIDMMESCAPETYPE = 2;
pub const D3DKMT_VIDMMESCAPETYPE_SETFAULT: D3DKMT_VIDMMESCAPETYPE = 0;
pub const D3DKMT_VIDMMESCAPETYPE_SET_BUDGET: D3DKMT_VIDMMESCAPETYPE = 8;
pub const D3DKMT_VIDMMESCAPETYPE_SET_EVICTION_CONFIG: D3DKMT_VIDMMESCAPETYPE = 18;
pub const D3DKMT_VIDMMESCAPETYPE_SET_TRIM_INTERVALS: D3DKMT_VIDMMESCAPETYPE = 12;
pub const D3DKMT_VIDMMESCAPETYPE_SUSPEND_CPU_ACCESS_TEST: D3DKMT_VIDMMESCAPETYPE = 4;
pub const D3DKMT_VIDMMESCAPETYPE_SUSPEND_PROCESS: D3DKMT_VIDMMESCAPETYPE = 9;
pub const D3DKMT_VIDMMESCAPETYPE_VALIDATE_INTEGRITY: D3DKMT_VIDMMESCAPETYPE = 17;
pub const D3DKMT_VIDMMESCAPETYPE_WAKE: D3DKMT_VIDMMESCAPETYPE = 14;
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_VIDMM_ESCAPE {
    pub Type: D3DKMT_VIDMMESCAPETYPE,
    pub Anonymous: D3DKMT_VIDMM_ESCAPE_0,
}
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_VIDMM_ESCAPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union D3DKMT_VIDMM_ESCAPE_0 {
    pub SetFault: D3DKMT_VIDMM_ESCAPE_0_0,
    pub Debug: D3DKMT_VIDMM_ESCAPE_0_1,
    pub Evict: D3DKMT_VIDMM_ESCAPE_0_2,
    pub EvictByNtHandle: D3DKMT_VIDMM_ESCAPE_0_3,
    pub GetVads: D3DKMT_VIDMM_ESCAPE_0_4,
    pub SetBudget: D3DKMT_VIDMM_ESCAPE_0_5,
    pub SuspendProcess: D3DKMT_VIDMM_ESCAPE_0_6,
    pub ResumeProcess: D3DKMT_VIDMM_ESCAPE_0_7,
    pub GetBudget: D3DKMT_VIDMM_ESCAPE_0_8,
    pub SetTrimIntervals: D3DKMT_VIDMM_ESCAPE_0_9,
    pub EvictByCriteria: D3DKMT_EVICTION_CRITERIA,
    pub Wake: D3DKMT_VIDMM_ESCAPE_0_10,
    pub Defrag: D3DKMT_VIDMM_ESCAPE_0_11,
    pub DelayExecution: D3DKMT_VIDMM_ESCAPE_0_12,
    pub VerifyIntegrity: D3DKMT_VIDMM_ESCAPE_0_13,
    pub DelayedEvictionConfig: D3DKMT_VIDMM_ESCAPE_0_14,
    pub CopyContent: D3DKMT_VIDMM_ESCAPE_0_15,
    pub QuerySection: D3DKMT_VIDMM_ESCAPE_0_16,
}
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_VIDMM_ESCAPE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_VIDMM_ESCAPE_0_0 {
    pub Anonymous: D3DKMT_VIDMM_ESCAPE_0_0_0,
}
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_VIDMM_ESCAPE_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union D3DKMT_VIDMM_ESCAPE_0_0_0 {
    pub Anonymous: D3DKMT_VIDMM_ESCAPE_0_0_0_0,
    pub Value: u32,
}
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_VIDMM_ESCAPE_0_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_VIDMM_ESCAPE_0_0_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_VIDMM_ESCAPE_0_1 {
    pub Anonymous: D3DKMT_VIDMM_ESCAPE_0_1_0,
}
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_VIDMM_ESCAPE_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union D3DKMT_VIDMM_ESCAPE_0_1_0 {
    pub Anonymous: D3DKMT_VIDMM_ESCAPE_0_1_0_0,
    pub Value: u32,
}
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_VIDMM_ESCAPE_0_1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_VIDMM_ESCAPE_0_1_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_VIDMM_ESCAPE_0_10 {
    pub bFlush: windows_sys::core::BOOL,
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_VIDMM_ESCAPE_0_11 {
    pub Operation: D3DKMT_DEFRAG_ESCAPE_OPERATION,
    pub SegmentId: u32,
    pub TotalCommitted: u64,
    pub TotalFree: u64,
    pub LargestGapBefore: u64,
    pub LargestGapAfter: u64,
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_VIDMM_ESCAPE_0_12 {
    pub hPagingQueue: super::d3dukmdt::D3DKMT_HANDLE,
    pub PhysicalAdapterIndex: u32,
    pub Milliseconds: u32,
    pub PagingFenceValue: u64,
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_VIDMM_ESCAPE_0_13 {
    pub PhysicalAdapterIndex: u16,
    pub SegmentId: u16,
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_VIDMM_ESCAPE_0_14 {
    pub TimerValue: i64,
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_VIDMM_ESCAPE_0_15 {
    pub UserBuffer: *mut core::ffi::c_void,
    pub Direction: D3DKMT_ESCAPE_COPY_CONTENT_DIRECTION,
    pub hAllocation: super::d3dukmdt::D3DKMT_HANDLE,
    pub Offset: u64,
    pub Size: u64,
}
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_VIDMM_ESCAPE_0_15 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_VIDMM_ESCAPE_0_16 {
    pub hAllocation: super::d3dukmdt::D3DKMT_HANDLE,
    pub hSection: super::winnt::HANDLE,
}
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_VIDMM_ESCAPE_0_16 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_VIDMM_ESCAPE_0_2 {
    pub ResourceHandle: super::d3dukmdt::D3DKMT_HANDLE,
    pub AllocationHandle: super::d3dukmdt::D3DKMT_HANDLE,
    pub hProcess: super::winnt::HANDLE,
}
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_VIDMM_ESCAPE_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_VIDMM_ESCAPE_0_3 {
    pub NtHandle: u64,
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_VIDMM_ESCAPE_0_4 {
    pub Anonymous: D3DKMT_VIDMM_ESCAPE_0_4_0,
    pub Command: D3DKMT_VAD_ESCAPE_COMMAND,
    pub Status: super::bcrypt::NTSTATUS,
    pub Anonymous2: D3DKMT_VIDMM_ESCAPE_0_4_1,
}
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_VIDMM_ESCAPE_0_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union D3DKMT_VIDMM_ESCAPE_0_4_0 {
    pub GetNumVads: D3DKMT_VIDMM_ESCAPE_0_4_0_0,
    pub GetVad: D3DKMT_VAD_DESC,
    pub GetVadRange: D3DKMT_VA_RANGE_DESC,
    pub GetGpuMmuCaps: D3DKMT_GET_GPUMMU_CAPS,
    pub GetPte: D3DKMT_GET_PTE,
    pub GetSegmentCaps: D3DKMT_GET_SEGMENT_CAPS,
}
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_VIDMM_ESCAPE_0_4_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_VIDMM_ESCAPE_0_4_0_0 {
    pub NumVads: u32,
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union D3DKMT_VIDMM_ESCAPE_0_4_1 {
    pub GetPteExt: D3DKMT_GET_PTE_EXT,
}
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_VIDMM_ESCAPE_0_4_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_VIDMM_ESCAPE_0_5 {
    pub LocalMemoryBudget: u64,
    pub SystemMemoryBudget: u64,
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_VIDMM_ESCAPE_0_6 {
    pub hProcess: super::winnt::HANDLE,
    pub bAllowWakeOnSubmission: windows_sys::core::BOOL,
}
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_VIDMM_ESCAPE_0_6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_VIDMM_ESCAPE_0_7 {
    pub hProcess: super::winnt::HANDLE,
}
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_VIDMM_ESCAPE_0_7 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_VIDMM_ESCAPE_0_8 {
    pub NumBytesToTrim: u64,
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_VIDMM_ESCAPE_0_9 {
    pub MinTrimInterval: u32,
    pub MaxTrimInterval: u32,
    pub IdleTrimInterval: u32,
}
pub const D3DKMT_VIDPNSOURCEOWNER_EMULATED: D3DKMT_VIDPNSOURCEOWNER_TYPE = 4;
pub const D3DKMT_VIDPNSOURCEOWNER_EXCLUSIVE: D3DKMT_VIDPNSOURCEOWNER_TYPE = 2;
pub const D3DKMT_VIDPNSOURCEOWNER_EXCLUSIVEGDI: D3DKMT_VIDPNSOURCEOWNER_TYPE = 3;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_VIDPNSOURCEOWNER_FLAGS {
    pub Anonymous: D3DKMT_VIDPNSOURCEOWNER_FLAGS_0,
}
impl Default for D3DKMT_VIDPNSOURCEOWNER_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_VIDPNSOURCEOWNER_FLAGS_0 {
    pub Anonymous: D3DKMT_VIDPNSOURCEOWNER_FLAGS_0_0,
    pub Value: u32,
}
impl Default for D3DKMT_VIDPNSOURCEOWNER_FLAGS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_VIDPNSOURCEOWNER_FLAGS_0_0 {
    pub _bitfield: u32,
}
pub const D3DKMT_VIDPNSOURCEOWNER_SHARED: D3DKMT_VIDPNSOURCEOWNER_TYPE = 1;
pub type D3DKMT_VIDPNSOURCEOWNER_TYPE = i32;
pub const D3DKMT_VIDPNSOURCEOWNER_UNOWNED: D3DKMT_VIDPNSOURCEOWNER_TYPE = 0;
pub type D3DKMT_VIDSCHESCAPETYPE = i32;
pub const D3DKMT_VIDSCHESCAPETYPE_CONFIGURE_TDR_LIMIT: D3DKMT_VIDSCHESCAPETYPE = 5;
pub const D3DKMT_VIDSCHESCAPETYPE_ENABLECONTEXTDELAY: D3DKMT_VIDSCHESCAPETYPE = 4;
pub const D3DKMT_VIDSCHESCAPETYPE_PFN_CONTROL: D3DKMT_VIDSCHESCAPETYPE = 7;
pub const D3DKMT_VIDSCHESCAPETYPE_PREEMPTIONCONTROL: D3DKMT_VIDSCHESCAPETYPE = 0;
pub const D3DKMT_VIDSCHESCAPETYPE_SUSPENDRESUME: D3DKMT_VIDSCHESCAPETYPE = 3;
pub const D3DKMT_VIDSCHESCAPETYPE_SUSPENDSCHEDULER: D3DKMT_VIDSCHESCAPETYPE = 1;
pub const D3DKMT_VIDSCHESCAPETYPE_TDRCONTROL: D3DKMT_VIDSCHESCAPETYPE = 2;
pub const D3DKMT_VIDSCHESCAPETYPE_VGPU_RESET: D3DKMT_VIDSCHESCAPETYPE = 6;
pub const D3DKMT_VIDSCHESCAPETYPE_VIRTUAL_REFRESH_RATE: D3DKMT_VIDSCHESCAPETYPE = 8;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_VIDSCH_ESCAPE {
    pub Type: D3DKMT_VIDSCHESCAPETYPE,
    pub Anonymous: D3DKMT_VIDSCH_ESCAPE_0,
    pub VirtualRefreshRateControl: D3DKMT_ESCAPE_VIRTUAL_REFRESH_RATE,
}
impl Default for D3DKMT_VIDSCH_ESCAPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_VIDSCH_ESCAPE_0 {
    pub PreemptionControl: windows_sys::core::BOOL,
    pub EnableContextDelay: windows_sys::core::BOOL,
    pub TdrControl2: D3DKMT_VIDSCH_ESCAPE_0_0,
    pub SuspendScheduler: windows_sys::core::BOOL,
    pub TdrControl: u32,
    pub SuspendTime: u32,
    pub TdrLimit: D3DKMT_VIDSCH_ESCAPE_0_1,
    pub PfnControl: D3DKMT_ESCAPE_PFN_CONTROL_COMMAND,
}
impl Default for D3DKMT_VIDSCH_ESCAPE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_VIDSCH_ESCAPE_0_0 {
    pub TdrControl: u32,
    pub Anonymous: D3DKMT_VIDSCH_ESCAPE_0_0_0,
}
impl Default for D3DKMT_VIDSCH_ESCAPE_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union D3DKMT_VIDSCH_ESCAPE_0_0_0 {
    pub NodeOrdinal: u32,
}
impl Default for D3DKMT_VIDSCH_ESCAPE_0_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_VIDSCH_ESCAPE_0_1 {
    pub Count: u32,
    pub Time: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_VIRTUALADDRESSFLAGS {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_VIRTUALADDRESSINFO {
    pub VirtualAddressFlags: D3DKMT_VIRTUALADDRESSFLAGS,
}
pub type D3DKMT_VISTABLTMODEL_PRESENTHISTORYTOKEN = u64;
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_WAITFORIDLE {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_WAITFORSYNCHRONIZATIONOBJECT {
    pub hContext: super::d3dukmdt::D3DKMT_HANDLE,
    pub ObjectCount: u32,
    pub ObjectHandleArray: [super::d3dukmdt::D3DKMT_HANDLE; 32],
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_WAITFORSYNCHRONIZATIONOBJECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_WAITFORSYNCHRONIZATIONOBJECT2 {
    pub hContext: super::d3dukmdt::D3DKMT_HANDLE,
    pub ObjectCount: u32,
    pub ObjectHandleArray: [super::d3dukmdt::D3DKMT_HANDLE; 32],
    pub Anonymous: D3DKMT_WAITFORSYNCHRONIZATIONOBJECT2_0,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_WAITFORSYNCHRONIZATIONOBJECT2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub union D3DKMT_WAITFORSYNCHRONIZATIONOBJECT2_0 {
    pub Fence: D3DKMT_WAITFORSYNCHRONIZATIONOBJECT2_0_0,
    pub Reserved: [u64; 8],
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_WAITFORSYNCHRONIZATIONOBJECT2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_WAITFORSYNCHRONIZATIONOBJECT2_0_0 {
    pub FenceValue: u64,
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_WAITFORSYNCHRONIZATIONOBJECTFROMCPU {
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub ObjectCount: u32,
    pub ObjectHandleArray: *const super::d3dukmdt::D3DKMT_HANDLE,
    pub FenceValueArray: *const u64,
    pub hAsyncEvent: super::winnt::HANDLE,
    pub Flags: super::d3dukmdt::D3DDDI_WAITFORSYNCHRONIZATIONOBJECTFROMCPU_FLAGS,
}
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_WAITFORSYNCHRONIZATIONOBJECTFROMCPU {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub struct D3DKMT_WAITFORSYNCHRONIZATIONOBJECTFROMGPU {
    pub hContext: super::d3dukmdt::D3DKMT_HANDLE,
    pub ObjectCount: u32,
    pub ObjectHandleArray: *const super::d3dukmdt::D3DKMT_HANDLE,
    pub Anonymous: D3DKMT_WAITFORSYNCHRONIZATIONOBJECTFROMGPU_0,
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_WAITFORSYNCHRONIZATIONOBJECTFROMGPU {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy)]
pub union D3DKMT_WAITFORSYNCHRONIZATIONOBJECTFROMGPU_0 {
    pub MonitoredFenceValueArray: *const u64,
    pub FenceValue: u64,
    pub Reserved: [u64; 8],
}
#[cfg(feature = "d3dukmdt")]
impl Default for D3DKMT_WAITFORSYNCHRONIZATIONOBJECTFROMGPU_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_WAITFORVERTICALBLANKEVENT {
    pub hAdapter: super::d3dukmdt::D3DKMT_HANDLE,
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
}
#[repr(C)]
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct D3DKMT_WAITFORVERTICALBLANKEVENT2 {
    pub hAdapter: super::d3dukmdt::D3DKMT_HANDLE,
    pub hDevice: super::d3dukmdt::D3DKMT_HANDLE,
    pub VidPnSourceId: super::d3dukmdt::D3DDDI_VIDEO_PRESENT_SOURCE_ID,
    pub NumObjects: u32,
    pub ObjectHandleArray: [super::d3dukmdt::D3DKMT_PTR_TYPE; 8],
}
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
impl Default for D3DKMT_WAITFORVERTICALBLANKEVENT2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "d3dukmdt")]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_WAITSYNCFILE {
    pub hSyncFile: u64,
    pub hContext: super::d3dukmdt::D3DKMT_HANDLE,
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_WORKINGSETFLAGS {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_WORKINGSETINFO {
    pub Flags: D3DKMT_WORKINGSETFLAGS,
    pub MinimumWorkingSetPercentile: u32,
    pub MaximumWorkingSetPercentile: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3DKMT_WSAUMDIMAGENAME {
    pub WsaUmdImageName: [u16; 260],
}
impl Default for D3DKMT_WSAUMDIMAGENAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3DKMT_WaitCommandBuffer: D3DKMT_QUERYSTATISTICS_QUEUE_PACKET_TYPE = 4;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct D3DKMT_XBOX {
    pub IsXBOX: windows_sys::core::BOOL,
}
pub const DXGKMT_MULTIPLANE_OVERLAY_STEREO_FLIP_FRAME0: DXGKMT_MULTIPLANE_OVERLAY_STEREO_FLIP_MODE = 1;
pub const DXGKMT_MULTIPLANE_OVERLAY_STEREO_FLIP_FRAME1: DXGKMT_MULTIPLANE_OVERLAY_STEREO_FLIP_MODE = 2;
pub type DXGKMT_MULTIPLANE_OVERLAY_STEREO_FLIP_MODE = i32;
pub const DXGKMT_MULTIPLANE_OVERLAY_STEREO_FLIP_NONE: DXGKMT_MULTIPLANE_OVERLAY_STEREO_FLIP_MODE = 0;
pub const DXGKMT_MULTIPLANE_OVERLAY_STEREO_FORMAT_CHECKERBOARD: D3DKMT_MULTIPLANE_OVERLAY_STEREO_FORMAT = 7;
pub const DXGKMT_MULTIPLANE_OVERLAY_STEREO_FORMAT_COLUMN_INTERLEAVED: D3DKMT_MULTIPLANE_OVERLAY_STEREO_FORMAT = 6;
pub const DXGKMT_MULTIPLANE_OVERLAY_STEREO_FORMAT_MONO: D3DKMT_MULTIPLANE_OVERLAY_STEREO_FORMAT = 0;
pub const DXGKMT_MULTIPLANE_OVERLAY_STEREO_FORMAT_MONO_OFFSET: D3DKMT_MULTIPLANE_OVERLAY_STEREO_FORMAT = 4;
pub const DXGKMT_MULTIPLANE_OVERLAY_STEREO_FORMAT_ROW_INTERLEAVED: D3DKMT_MULTIPLANE_OVERLAY_STEREO_FORMAT = 5;
pub const DXGKMT_MULTIPLANE_OVERLAY_STEREO_FORMAT_SEPARATE: D3DKMT_MULTIPLANE_OVERLAY_STEREO_FORMAT = 3;
pub type DXGKMT_MULTIPLANE_OVERLAY_STRETCH_QUALITY = i32;
pub const DXGKMT_MULTIPLANE_OVERLAY_STRETCH_QUALITY_BILINEAR: DXGKMT_MULTIPLANE_OVERLAY_STRETCH_QUALITY = 1;
pub const DXGKMT_MULTIPLANE_OVERLAY_STRETCH_QUALITY_HIGH: DXGKMT_MULTIPLANE_OVERLAY_STRETCH_QUALITY = 2;
pub type DXGKMT_POWER_SHARED_TYPE = i32;
pub const DXGKMT_POWER_SHARED_TYPE_AUDIO: DXGKMT_POWER_SHARED_TYPE = 0;
pub const DXGK_DIAG_PROCESS_NAME_LENGTH: u32 = 16;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DXGK_ESCAPE_GPUMMUCAPS {
    pub ReadOnlyMemorySupported: bool,
    pub NoExecuteMemorySupported: bool,
    pub ZeroInPteSupported: bool,
    pub CacheCoherentMemorySupported: bool,
    pub LargePageSupported: bool,
    pub DualPteSupported: bool,
    pub AllowNonAlignedLargePageAddress: bool,
    pub _bitfield: bool,
    pub VirtualAddressBitCount: u32,
    pub PageTableLevelCount: u32,
    pub PageTableLevelDesk: [D3DKMT_PAGE_TABLE_LEVEL_DESC; 6],
}
impl Default for DXGK_ESCAPE_GPUMMUCAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
#[derive(Clone, Copy, Default)]
pub struct DXGK_FEATURE_PROCESS_DEBUG_BLOB_COLLECTION_INTERFACE_V1 {
    pub pfnEnableProcessDebugBlobCollection: PFND3DKMT_ENABLEPROCESSDEBUGBLOBCOLLECTION,
    pub pfnDisableProcessDebugBlobCollection: PFND3DKMT_DISABLEPROCESSDEBUGBLOBCOLLECTION,
    pub pfnMapProcessDebugBlob: PFND3DKMT_MAPPROCESSDEBUGBLOB,
    pub pfnUnmapProcessDebugBlob: PFND3DKMT_UNMAPPROCESSDEBUGBLOB,
}
#[cfg(feature = "winnt")]
pub type DXGK_GRAPHICSPOWER_REGISTER_INPUT = DXGK_GRAPHICSPOWER_REGISTER_INPUT_V_1_2;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct DXGK_GRAPHICSPOWER_REGISTER_INPUT_V_1_2 {
    pub Version: u32,
    pub PrivateHandle: *mut core::ffi::c_void,
    pub PowerNotificationCb: PDXGK_POWER_NOTIFICATION,
    pub RemovalNotificationCb: PDXGK_REMOVAL_NOTIFICATION,
    pub FStateNotificationCb: PDXGK_FSTATE_NOTIFICATION,
    pub InitialComponentStateCb: PDXGK_INITIAL_COMPONENT_STATE,
}
#[cfg(feature = "winnt")]
impl Default for DXGK_GRAPHICSPOWER_REGISTER_INPUT_V_1_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DXGK_GRAPHICSPOWER_REGISTER_OUTPUT {
    pub DeviceHandle: *mut core::ffi::c_void,
    pub InitialGrfxPowerState: super::winnt::DEVICE_POWER_STATE,
    pub SetSharedPowerComponentStateCb: PDXGK_SET_SHARED_POWER_COMPONENT_STATE,
    pub UnregisterCb: PDXGK_GRAPHICSPOWER_UNREGISTER,
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
impl Default for DXGK_GRAPHICSPOWER_REGISTER_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DXGK_GRAPHICSPOWER_VERSION: u32 = 4098;
pub const DXGK_GRAPHICSPOWER_VERSION_1_0: u32 = 4096;
pub const DXGK_GRAPHICSPOWER_VERSION_1_1: u32 = 4097;
pub const DXGK_GRAPHICSPOWER_VERSION_1_2: u32 = 4098;
pub const FLIPEX_TIMEOUT_KERNEL: u32 = 20000000;
pub const FLIPEX_TIMEOUT_USER: u32 = 2000;
pub const GUID_DEVINTERFACE_GRAPHICSPOWER: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xea5c6870_e93c_4588_bef1_fec42fc9429a);
pub const IOCTL_INTERNAL_GRAPHICSPOWER_REGISTER: u32 = 2304007;
pub const KMTQAITYPE_ADAPTERADDRESS: KMTQUERYADAPTERINFOTYPE = 6;
pub const KMTQAITYPE_ADAPTERADDRESS_RENDER: KMTQUERYADAPTERINFOTYPE = 53;
pub const KMTQAITYPE_ADAPTERGUID: KMTQUERYADAPTERINFOTYPE = 4;
pub const KMTQAITYPE_ADAPTERGUID_RENDER: KMTQUERYADAPTERINFOTYPE = 52;
pub const KMTQAITYPE_ADAPTERPERFDATA: KMTQUERYADAPTERINFOTYPE = 62;
pub const KMTQAITYPE_ADAPTERPERFDATA_CAPS: KMTQUERYADAPTERINFOTYPE = 63;
pub const KMTQAITYPE_ADAPTERREGISTRYINFO: KMTQUERYADAPTERINFOTYPE = 8;
pub const KMTQAITYPE_ADAPTERREGISTRYINFO_RENDER: KMTQUERYADAPTERINFOTYPE = 54;
pub const KMTQAITYPE_ADAPTERTYPE: KMTQUERYADAPTERINFOTYPE = 15;
pub const KMTQAITYPE_ADAPTERTYPE_RENDER: KMTQUERYADAPTERINFOTYPE = 57;
pub const KMTQAITYPE_BLOCKLIST_KERNEL: KMTQUERYADAPTERINFOTYPE = 50;
pub const KMTQAITYPE_BLOCKLIST_RUNTIME: KMTQUERYADAPTERINFOTYPE = 51;
pub const KMTQAITYPE_CHECKDRIVERUPDATESTATUS: KMTQUERYADAPTERINFOTYPE = 11;
pub const KMTQAITYPE_CHECKDRIVERUPDATESTATUS_RENDER: KMTQUERYADAPTERINFOTYPE = 55;
pub const KMTQAITYPE_CPDRIVERNAME: KMTQUERYADAPTERINFOTYPE = 26;
pub const KMTQAITYPE_CROSSADAPTERRESOURCE_SUPPORT: KMTQUERYADAPTERINFOTYPE = 76;
pub const KMTQAITYPE_CURRENTDISPLAYMODE: KMTQUERYADAPTERINFOTYPE = 9;
pub const KMTQAITYPE_DIRECTFLIP_SUPPORT: KMTQUERYADAPTERINFOTYPE = 19;
pub const KMTQAITYPE_DISPLAY_CAPS: KMTQUERYADAPTERINFOTYPE = 74;
pub const KMTQAITYPE_DISPLAY_UMDRIVERNAME: KMTQUERYADAPTERINFOTYPE = 71;
pub const KMTQAITYPE_DLIST_DRIVER_NAME: KMTQUERYADAPTERINFOTYPE = 21;
pub const KMTQAITYPE_DRIVERCAPS_EXT: KMTQUERYADAPTERINFOTYPE = 32;
pub const KMTQAITYPE_DRIVERVERSION: KMTQUERYADAPTERINFOTYPE = 13;
pub const KMTQAITYPE_DRIVERVERSION_RENDER: KMTQUERYADAPTERINFOTYPE = 56;
pub const KMTQAITYPE_DRIVER_DESCRIPTION: KMTQUERYADAPTERINFOTYPE = 65;
pub const KMTQAITYPE_DRIVER_DESCRIPTION_RENDER: KMTQUERYADAPTERINFOTYPE = 66;
pub const KMTQAITYPE_FLIPQUEUEINFO: KMTQUERYADAPTERINFOTYPE = 5;
pub const KMTQAITYPE_GETSEGMENTGROUPSIZE: KMTQUERYADAPTERINFOTYPE = 42;
pub const KMTQAITYPE_GETSEGMENTSIZE: KMTQUERYADAPTERINFOTYPE = 3;
pub const KMTQAITYPE_GET_DEVICE_VIDPN_OWNERSHIP_INFO: KMTQUERYADAPTERINFOTYPE = 47;
pub const KMTQAITYPE_HWDRM_SUPPORT: KMTQUERYADAPTERINFOTYPE = 44;
pub const KMTQAITYPE_HYBRID_DLIST_DLL_MUX_SUPPORT: KMTQUERYADAPTERINFOTYPE = 81;
pub const KMTQAITYPE_HYBRID_DLIST_DLL_SUPPORT: KMTQUERYADAPTERINFOTYPE = 73;
pub const KMTQAITYPE_INDEPENDENTFLIP_SECONDARY_SUPPORT: KMTQUERYADAPTERINFOTYPE = 39;
pub const KMTQAITYPE_INDEPENDENTFLIP_SUPPORT: KMTQUERYADAPTERINFOTYPE = 28;
pub const KMTQAITYPE_KMD_DRIVER_VERSION: KMTQUERYADAPTERINFOTYPE = 49;
pub const KMTQAITYPE_MIRACASTCOMPANIONDRIVERNAME: KMTQUERYADAPTERINFOTYPE = 29;
pub const KMTQAITYPE_MODELIST: KMTQUERYADAPTERINFOTYPE = 10;
pub const KMTQAITYPE_MPO3DDI_SUPPORT: KMTQUERYADAPTERINFOTYPE = 43;
pub const KMTQAITYPE_MPOKERNELCAPS_SUPPORT: KMTQUERYADAPTERINFOTYPE = 45;
pub const KMTQAITYPE_MULTIPLANEOVERLAY_HUD_SUPPORT: KMTQUERYADAPTERINFOTYPE = 23;
pub const KMTQAITYPE_MULTIPLANEOVERLAY_SECONDARY_SUPPORT: KMTQUERYADAPTERINFOTYPE = 38;
pub const KMTQAITYPE_MULTIPLANEOVERLAY_STRETCH_SUPPORT: KMTQUERYADAPTERINFOTYPE = 46;
pub const KMTQAITYPE_MULTIPLANEOVERLAY_SUPPORT: KMTQUERYADAPTERINFOTYPE = 20;
pub const KMTQAITYPE_NODEMETADATA: KMTQUERYADAPTERINFOTYPE = 25;
pub const KMTQAITYPE_NODEPERFDATA: KMTQUERYADAPTERINFOTYPE = 61;
pub const KMTQAITYPE_OUTPUTDUPLCONTEXTSCOUNT: KMTQUERYADAPTERINFOTYPE = 16;
pub const KMTQAITYPE_PANELFITTER_SUPPORT: KMTQUERYADAPTERINFOTYPE = 40;
pub const KMTQAITYPE_PARAVIRTUALIZATION_RENDER: KMTQUERYADAPTERINFOTYPE = 68;
pub const KMTQAITYPE_PHYSICALADAPTERCOUNT: KMTQUERYADAPTERINFOTYPE = 30;
pub const KMTQAITYPE_PHYSICALADAPTERDEVICEIDS: KMTQUERYADAPTERINFOTYPE = 31;
pub const KMTQAITYPE_PHYSICALADAPTERPNPKEY: KMTQUERYADAPTERINFOTYPE = 41;
pub const KMTQAITYPE_QUERYREGISTRY: KMTQUERYADAPTERINFOTYPE = 48;
pub const KMTQAITYPE_QUERY_ADAPTER_UNIQUE_GUID: KMTQUERYADAPTERINFOTYPE = 60;
pub const KMTQAITYPE_QUERY_GPUMMU_CAPS: KMTQUERYADAPTERINFOTYPE = 34;
pub const KMTQAITYPE_QUERY_HW_PROTECTION_TEARDOWN_COUNT: KMTQUERYADAPTERINFOTYPE = 36;
pub const KMTQAITYPE_QUERY_ISBADDRIVERFORHWPROTECTIONDISABLED: KMTQUERYADAPTERINFOTYPE = 37;
pub const KMTQAITYPE_QUERY_MIRACAST_DRIVER_TYPE: KMTQUERYADAPTERINFOTYPE = 33;
pub const KMTQAITYPE_QUERY_MULTIPLANEOVERLAY_DECODE_SUPPORT: KMTQUERYADAPTERINFOTYPE = 35;
pub const KMTQAITYPE_SCANOUT_CAPS: KMTQUERYADAPTERINFOTYPE = 67;
pub const KMTQAITYPE_SERVICENAME: KMTQUERYADAPTERINFOTYPE = 69;
pub const KMTQAITYPE_SETWORKINGSETINFO: KMTQUERYADAPTERINFOTYPE = 7;
pub const KMTQAITYPE_TRACKEDWORKLOAD_SUPPORT: KMTQUERYADAPTERINFOTYPE = 72;
pub const KMTQAITYPE_UMDRIVERNAME: KMTQUERYADAPTERINFOTYPE = 1;
pub const KMTQAITYPE_UMDRIVERPRIVATE: KMTQUERYADAPTERINFOTYPE = 0;
pub const KMTQAITYPE_UMD_DRIVER_VERSION: KMTQUERYADAPTERINFOTYPE = 18;
pub const KMTQAITYPE_UMOPENGLINFO: KMTQUERYADAPTERINFOTYPE = 2;
pub const KMTQAITYPE_VGPUINTERFACEID: KMTQUERYADAPTERINFOTYPE = 79;
pub const KMTQAITYPE_VIRTUALADDRESSINFO: KMTQUERYADAPTERINFOTYPE = 12;
pub const KMTQAITYPE_WDDM_1_2_CAPS: KMTQUERYADAPTERINFOTYPE = 17;
pub const KMTQAITYPE_WDDM_1_2_CAPS_RENDER: KMTQUERYADAPTERINFOTYPE = 58;
pub const KMTQAITYPE_WDDM_1_3_CAPS: KMTQUERYADAPTERINFOTYPE = 22;
pub const KMTQAITYPE_WDDM_1_3_CAPS_RENDER: KMTQUERYADAPTERINFOTYPE = 59;
pub const KMTQAITYPE_WDDM_2_0_CAPS: KMTQUERYADAPTERINFOTYPE = 24;
pub const KMTQAITYPE_WDDM_2_7_CAPS: KMTQUERYADAPTERINFOTYPE = 70;
pub const KMTQAITYPE_WDDM_2_9_CAPS: KMTQUERYADAPTERINFOTYPE = 75;
pub const KMTQAITYPE_WDDM_3_0_CAPS: KMTQUERYADAPTERINFOTYPE = 77;
pub const KMTQAITYPE_WDDM_3_1_CAPS: KMTQUERYADAPTERINFOTYPE = 80;
pub const KMTQAITYPE_WSAUMDIMAGENAME: KMTQUERYADAPTERINFOTYPE = 78;
pub const KMTQAITYPE_XBOX: KMTQUERYADAPTERINFOTYPE = 27;
pub type KMTQUERYADAPTERINFOTYPE = i32;
pub const KMTQUITYPE_GPUVERSION: KMTQUERYADAPTERINFOTYPE = 64;
pub type KMTUMDVERSION = i32;
pub const KMTUMDVERSION_DX10: KMTUMDVERSION = 1;
pub const KMTUMDVERSION_DX11: KMTUMDVERSION = 2;
pub const KMTUMDVERSION_DX12: KMTUMDVERSION = 3;
pub const KMTUMDVERSION_DX12_WSA32: KMTUMDVERSION = 4;
pub const KMTUMDVERSION_DX12_WSA64: KMTUMDVERSION = 5;
pub const KMTUMDVERSION_DX9: KMTUMDVERSION = 0;
pub const KMT_DISPLAY_UMDVERSION_1: KMT_DISPLAY_UMD_VERSION = 0;
pub type KMT_DISPLAY_UMD_VERSION = i32;
pub const KMT_DRIVERVERSION_WDDM_1_0: D3DKMT_DRIVERVERSION = 1000;
pub const KMT_DRIVERVERSION_WDDM_1_1: D3DKMT_DRIVERVERSION = 1105;
pub const KMT_DRIVERVERSION_WDDM_1_1_PRERELEASE: D3DKMT_DRIVERVERSION = 1102;
pub const KMT_DRIVERVERSION_WDDM_1_2: D3DKMT_DRIVERVERSION = 1200;
pub const KMT_DRIVERVERSION_WDDM_1_3: D3DKMT_DRIVERVERSION = 1300;
pub const KMT_DRIVERVERSION_WDDM_2_0: D3DKMT_DRIVERVERSION = 2000;
pub const KMT_DRIVERVERSION_WDDM_2_1: D3DKMT_DRIVERVERSION = 2100;
pub const KMT_DRIVERVERSION_WDDM_2_2: D3DKMT_DRIVERVERSION = 2200;
pub const KMT_DRIVERVERSION_WDDM_2_3: D3DKMT_DRIVERVERSION = 2300;
pub const KMT_DRIVERVERSION_WDDM_2_4: D3DKMT_DRIVERVERSION = 2400;
pub const KMT_DRIVERVERSION_WDDM_2_5: D3DKMT_DRIVERVERSION = 2500;
pub const KMT_DRIVERVERSION_WDDM_2_6: D3DKMT_DRIVERVERSION = 2600;
pub const KMT_DRIVERVERSION_WDDM_2_7: D3DKMT_DRIVERVERSION = 2700;
pub const KMT_DRIVERVERSION_WDDM_2_8: D3DKMT_DRIVERVERSION = 2800;
pub const KMT_DRIVERVERSION_WDDM_2_9: D3DKMT_DRIVERVERSION = 2900;
pub const KMT_DRIVERVERSION_WDDM_3_0: D3DKMT_DRIVERVERSION = 3000;
pub const KMT_DRIVERVERSION_WDDM_3_1: D3DKMT_DRIVERVERSION = 3100;
pub const KMT_DRIVERVERSION_WDDM_3_2: D3DKMT_DRIVERVERSION = 3200;
pub const MAX_ENUM_ADAPTERS: u32 = 16;
pub const MiracastStartPending: D3DKMT_MIRACAST_DISPLAY_DEVICE_STATE = 1;
pub const MiracastStarted: D3DKMT_MIRACAST_DISPLAY_DEVICE_STATE = 2;
pub const MiracastStopPending: D3DKMT_MIRACAST_DISPLAY_DEVICE_STATE = 3;
pub const MiracastStopped: D3DKMT_MIRACAST_DISPLAY_DEVICE_STATE = 0;
pub const NUM_KMTUMDVERSIONS: KMTUMDVERSION = 6;
pub const NUM_KMT_DISPLAY_UMDVERSIONS: KMT_DISPLAY_UMD_VERSION = 1;
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct OBJECT_ATTRIBUTES {
    pub Length: u32,
    pub RootDirectory: super::winnt::HANDLE,
    pub ObjectName: super::ntsecapi::PUNICODE_STRING,
    pub Attributes: u32,
    pub SecurityDescriptor: *mut core::ffi::c_void,
    pub SecurityQualityOfService: *mut core::ffi::c_void,
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
impl Default for OBJECT_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct OUTPUTDUPL_CONTEXT_DEBUG_INFO {
    pub Status: OUTPUTDUPL_CONTEXT_DEBUG_STATUS,
    pub ProcessID: super::winnt::HANDLE,
    pub AccumulatedPresents: u32,
    pub LastPresentTime: i64,
    pub LastMouseTime: i64,
    pub ProcessName: [i8; 16],
}
#[cfg(feature = "winnt")]
impl Default for OUTPUTDUPL_CONTEXT_DEBUG_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type OUTPUTDUPL_CONTEXT_DEBUG_STATUS = i32;
pub const OUTPUTDUPL_CONTEXT_DEBUG_STATUS_ACTIVE: OUTPUTDUPL_CONTEXT_DEBUG_STATUS = 1;
pub const OUTPUTDUPL_CONTEXT_DEBUG_STATUS_FORCE_UINT32: OUTPUTDUPL_CONTEXT_DEBUG_STATUS = -1;
pub const OUTPUTDUPL_CONTEXT_DEBUG_STATUS_INACTIVE: OUTPUTDUPL_CONTEXT_DEBUG_STATUS = 0;
pub const OUTPUTDUPL_CONTEXT_DEBUG_STATUS_PENDING_DESTROY: OUTPUTDUPL_CONTEXT_DEBUG_STATUS = 2;
pub const OUTPUTDUPL_CREATE_MAX_KEYEDMUTXES: u32 = 3;
pub type PD3DKMT_MIRACAST_DISPLAY_DEVICE_CAPS = *mut D3DKMT_MIRACAST_DISPLAY_DEVICE_CAPS;
pub type PD3DKMT_MIRACAST_DISPLAY_DEVICE_STATUS = *mut D3DKMT_MIRACAST_DISPLAY_DEVICE_STATUS;
#[cfg(all(feature = "d3dukmdt", feature = "winnt"))]
pub type PD3DKMT_MIRACAST_DISPLAY_STOP_SESSIONS = *mut D3DKMT_MIRACAST_DISPLAY_STOP_SESSIONS;
pub type PDXGK_FSTATE_NOTIFICATION = Option<unsafe extern "system" fn(graphicsdevicehandle: *mut core::ffi::c_void, componentindex: u32, newfstate: u32, prenotification: bool, privatehandle: *mut core::ffi::c_void)>;
#[cfg(feature = "winnt")]
pub type PDXGK_GRAPHICSPOWER_REGISTER_INPUT = *mut DXGK_GRAPHICSPOWER_REGISTER_INPUT;
#[cfg(feature = "winnt")]
pub type PDXGK_GRAPHICSPOWER_REGISTER_INPUT_V_1_2 = *mut DXGK_GRAPHICSPOWER_REGISTER_INPUT_V_1_2;
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
pub type PDXGK_GRAPHICSPOWER_REGISTER_OUTPUT = *mut DXGK_GRAPHICSPOWER_REGISTER_OUTPUT;
#[cfg(feature = "bcrypt")]
pub type PDXGK_GRAPHICSPOWER_UNREGISTER = Option<unsafe extern "system" fn(devicehandle: *mut core::ffi::c_void, privatehandle: *mut core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
pub type PDXGK_INITIAL_COMPONENT_STATE = Option<unsafe extern "system" fn(graphicsdevicehandle: *mut core::ffi::c_void, privatehandle: *mut core::ffi::c_void, componentindex: u32, isblockingtype: bool, initialfstate: u32, componentguid: windows_sys::core::GUID, powercomponentmappingflag: u32)>;
#[cfg(feature = "winnt")]
pub type PDXGK_POWER_NOTIFICATION = Option<unsafe extern "system" fn(graphicsdevicehandle: *mut core::ffi::c_void, newgrfxpowerstate: super::winnt::DEVICE_POWER_STATE, prenotification: bool, privatehandle: *mut core::ffi::c_void)>;
pub type PDXGK_REMOVAL_NOTIFICATION = Option<unsafe extern "system" fn(graphicsdevicehandle: *mut core::ffi::c_void, privatehandle: *mut core::ffi::c_void)>;
#[cfg(feature = "bcrypt")]
pub type PDXGK_SET_SHARED_POWER_COMPONENT_STATE = Option<unsafe extern "system" fn(devicehandle: *mut core::ffi::c_void, privatehandle: *mut core::ffi::c_void, componentindex: u32, active: bool) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
pub type PFND3DKMT_ACQUIREKEYEDMUTEX = Option<unsafe extern "system" fn(param0: *mut D3DKMT_ACQUIREKEYEDMUTEX) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
pub type PFND3DKMT_ACQUIREKEYEDMUTEX2 = Option<unsafe extern "system" fn(param0: *mut D3DKMT_ACQUIREKEYEDMUTEX2) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_ADJUSTFULLSCREENGAMMA = Option<unsafe extern "system" fn(param0: *const D3DKMT_ADJUSTFULLSCREENGAMMA) -> super::bcrypt::NTSTATUS>;
pub type PFND3DKMT_BUDGETCHANGENOTIFICATIONCALLBACK = Option<unsafe extern "system" fn(param0: *const D3DKMT_BUDGETCHANGENOTIFICATION)>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
pub type PFND3DKMT_CANCELPRESENTS = Option<unsafe extern "system" fn(param0: *const D3DKMT_CANCEL_PRESENTS) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "windef", feature = "winnt"))]
pub type PFND3DKMT_CHANGESURFACEPOINTER = Option<unsafe extern "system" fn(param0: *const D3DKMT_CHANGESURFACEPOINTER) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
pub type PFND3DKMT_CHANGEVIDEOMEMORYRESERVATION = Option<unsafe extern "system" fn(param0: *const D3DKMT_CHANGEVIDEOMEMORYRESERVATION) -> super::bcrypt::NTSTATUS>;
pub type PFND3DKMT_CHECKEXCLUSIVEOWNERSHIP = Option<unsafe extern "system" fn() -> bool>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_CHECKMONITORPOWERSTATE = Option<unsafe extern "system" fn(param0: *const D3DKMT_CHECKMONITORPOWERSTATE) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
pub type PFND3DKMT_CHECKMULTIPLANEOVERLAYSUPPORT = Option<unsafe extern "system" fn(param0: *mut D3DKMT_CHECKMULTIPLANEOVERLAYSUPPORT) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
pub type PFND3DKMT_CHECKMULTIPLANEOVERLAYSUPPORT2 = Option<unsafe extern "system" fn(param0: *mut D3DKMT_CHECKMULTIPLANEOVERLAYSUPPORT2) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
pub type PFND3DKMT_CHECKMULTIPLANEOVERLAYSUPPORT3 = Option<unsafe extern "system" fn(param0: *mut D3DKMT_CHECKMULTIPLANEOVERLAYSUPPORT3) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "windef"))]
pub type PFND3DKMT_CHECKOCCLUSION = Option<unsafe extern "system" fn(param0: *const D3DKMT_CHECKOCCLUSION) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_CHECKSHAREDRESOURCEACCESS = Option<unsafe extern "system" fn(param0: *const D3DKMT_CHECKSHAREDRESOURCEACCESS) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_CHECKVIDPNEXCLUSIVEOWNERSHIP = Option<unsafe extern "system" fn(param0: *const D3DKMT_CHECKVIDPNEXCLUSIVEOWNERSHIP) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_CLOSEADAPTER = Option<unsafe extern "system" fn(param0: *const D3DKMT_CLOSEADAPTER) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
pub type PFND3DKMT_CONFIGURESHAREDRESOURCE = Option<unsafe extern "system" fn(param0: *const D3DKMT_CONFIGURESHAREDRESOURCE) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_CONNECTDOORBELL = Option<unsafe extern "system" fn(param0: *const D3DKMT_CONNECT_DOORBELL) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
pub type PFND3DKMT_CREATEALLOCATION = Option<unsafe extern "system" fn(param0: *mut D3DKMT_CREATEALLOCATION) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
pub type PFND3DKMT_CREATEALLOCATION2 = Option<unsafe extern "system" fn(param0: *mut D3DKMT_CREATEALLOCATION) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_CREATECONTEXT = Option<unsafe extern "system" fn(param0: *mut D3DKMT_CREATECONTEXT) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_CREATECONTEXTVIRTUAL = Option<unsafe extern "system" fn(param0: *mut D3DKMT_CREATECONTEXTVIRTUAL) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "windef", feature = "wingdi", feature = "winnt"))]
pub type PFND3DKMT_CREATEDCFROMMEMORY = Option<unsafe extern "system" fn(param0: *mut D3DKMT_CREATEDCFROMMEMORY) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_CREATEDEVICE = Option<unsafe extern "system" fn(param0: *mut D3DKMT_CREATEDEVICE) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_CREATEDOORBELL = Option<unsafe extern "system" fn(param0: *const D3DKMT_CREATE_DOORBELL) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_CREATEHWQUEUE = Option<unsafe extern "system" fn(param0: *mut D3DKMT_CREATEHWQUEUE) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_CREATEHWQUEUEFORUSERMODESUBMISSION = Option<unsafe extern "system" fn(param0: *mut D3DKMT_CREATEHWQUEUEFORUSERMODESUBMISSION) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_CREATEKEYEDMUTEX = Option<unsafe extern "system" fn(param0: *mut D3DKMT_CREATEKEYEDMUTEX) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_CREATEKEYEDMUTEX2 = Option<unsafe extern "system" fn(param0: *mut D3DKMT_CREATEKEYEDMUTEX2) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_CREATENATIVEFENCE = Option<unsafe extern "system" fn(param0: *mut D3DKMT_CREATENATIVEFENCE) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
pub type PFND3DKMT_CREATEOUTPUTDUPL = Option<unsafe extern "system" fn(param0: *const D3DKMT_CREATE_OUTPUTDUPL) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_CREATEOVERLAY = Option<unsafe extern "system" fn(param0: *mut D3DKMT_CREATEOVERLAY) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_CREATEPAGINGQUEUE = Option<unsafe extern "system" fn(param0: *mut D3DKMT_CREATEPAGINGQUEUE) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_CREATEPROTECTEDSESSION = Option<unsafe extern "system" fn(param0: *mut D3DKMT_CREATEPROTECTEDSESSION) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_CREATESYNCHRONIZATIONOBJECT = Option<unsafe extern "system" fn(param0: *mut D3DKMT_CREATESYNCHRONIZATIONOBJECT) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
pub type PFND3DKMT_CREATESYNCHRONIZATIONOBJECT2 = Option<unsafe extern "system" fn(param0: *mut D3DKMT_CREATESYNCHRONIZATIONOBJECT2) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_DESTROYALLOCATION = Option<unsafe extern "system" fn(param0: *const D3DKMT_DESTROYALLOCATION) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_DESTROYALLOCATION2 = Option<unsafe extern "system" fn(param0: *const D3DKMT_DESTROYALLOCATION2) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_DESTROYCONTEXT = Option<unsafe extern "system" fn(param0: *const D3DKMT_DESTROYCONTEXT) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "windef", feature = "winnt"))]
pub type PFND3DKMT_DESTROYDCFROMMEMORY = Option<unsafe extern "system" fn(param0: *const D3DKMT_DESTROYDCFROMMEMORY) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_DESTROYDEVICE = Option<unsafe extern "system" fn(param0: *const D3DKMT_DESTROYDEVICE) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_DESTROYDOORBELL = Option<unsafe extern "system" fn(param0: *const D3DKMT_DESTROY_DOORBELL) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_DESTROYHWQUEUE = Option<unsafe extern "system" fn(param0: *const D3DKMT_DESTROYHWQUEUE) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_DESTROYKEYEDMUTEX = Option<unsafe extern "system" fn(param0: *const D3DKMT_DESTROYKEYEDMUTEX) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_DESTROYOUTPUTDUPL = Option<unsafe extern "system" fn(param0: *const D3DKMT_DESTROY_OUTPUTDUPL) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_DESTROYOVERLAY = Option<unsafe extern "system" fn(param0: *const D3DKMT_DESTROYOVERLAY) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_DESTROYPAGINGQUEUE = Option<unsafe extern "system" fn(param0: *mut super::d3dukmdt::D3DDDI_DESTROYPAGINGQUEUE) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_DESTROYPROTECTEDSESSION = Option<unsafe extern "system" fn(param0: *mut D3DKMT_DESTROYPROTECTEDSESSION) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_DESTROYSYNCHRONIZATIONOBJECT = Option<unsafe extern "system" fn(param0: *const D3DKMT_DESTROYSYNCHRONIZATIONOBJECT) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "bcrypt")]
pub type PFND3DKMT_DISABLEPROCESSDEBUGBLOBCOLLECTION = Option<unsafe extern "system" fn() -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "bcrypt")]
pub type PFND3DKMT_ENABLEPROCESSDEBUGBLOBCOLLECTION = Option<unsafe extern "system" fn() -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
pub type PFND3DKMT_ENUMADAPTERS = Option<unsafe extern "system" fn(param0: *mut D3DKMT_ENUMADAPTERS) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
pub type PFND3DKMT_ENUMADAPTERS2 = Option<unsafe extern "system" fn(param0: *mut D3DKMT_ENUMADAPTERS2) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
pub type PFND3DKMT_ENUMADAPTERS3 = Option<unsafe extern "system" fn(param0: *mut D3DKMT_ENUMADAPTERS3) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_ESCAPE = Option<unsafe extern "system" fn(param0: *const D3DKMT_ESCAPE) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_EVICT = Option<unsafe extern "system" fn(param0: *mut D3DKMT_EVICT) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_FLIPOVERLAY = Option<unsafe extern "system" fn(param0: *const D3DKMT_FLIPOVERLAY) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_FLUSHHEAPTRANSITIONS = Option<unsafe extern "system" fn(param0: *const D3DKMT_FLUSHHEAPTRANSITIONS) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_FREEGPUVIRTUALADDRESS = Option<unsafe extern "system" fn(param0: *const D3DKMT_FREEGPUVIRTUALADDRESS) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_GETALLOCATIONPRIORITY = Option<unsafe extern "system" fn(param0: *const D3DKMT_GETALLOCATIONPRIORITY) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_GETCONTEXTINPROCESSSCHEDULINGPRIORITY = Option<unsafe extern "system" fn(param0: *mut D3DKMT_GETCONTEXTINPROCESSSCHEDULINGPRIORITY) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_GETCONTEXTSCHEDULINGPRIORITY = Option<unsafe extern "system" fn(param0: *mut D3DKMT_GETCONTEXTSCHEDULINGPRIORITY) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dkmdt", feature = "d3dukmdt"))]
pub type PFND3DKMT_GETDEVICESTATE = Option<unsafe extern "system" fn(param0: *mut D3DKMT_GETDEVICESTATE) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_GETDISPLAYMODELIST = Option<unsafe extern "system" fn(param0: *mut D3DKMT_GETDISPLAYMODELIST) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
pub type PFND3DKMT_GETDWMVERTICALBLANKEVENT = Option<unsafe extern "system" fn(param0: *const D3DKMT_GETVERTICALBLANKEVENT) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_GETMULTIPLANEOVERLAYCAPS = Option<unsafe extern "system" fn(param0: *mut D3DKMT_GET_MULTIPLANE_OVERLAY_CAPS) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_GETMULTISAMPLEMETHODLIST = Option<unsafe extern "system" fn(param0: *mut D3DKMT_GETMULTISAMPLEMETHODLIST) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_GETNATIVEFENCELOGDETAIL = Option<unsafe extern "system" fn(param0: *mut D3DKMT_GETNATIVEFENCELOGDETAIL) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_GETOVERLAYSTATE = Option<unsafe extern "system" fn(param0: *mut D3DKMT_GETOVERLAYSTATE) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_GETPOSTCOMPOSITIONCAPS = Option<unsafe extern "system" fn(param0: *mut D3DKMT_GET_POST_COMPOSITION_CAPS) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
pub type PFND3DKMT_GETPRESENTHISTORY = Option<unsafe extern "system" fn(param0: *mut D3DKMT_GETPRESENTHISTORY) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
pub type PFND3DKMT_GETPROCESSDEVICEREMOVALSUPPORT = Option<unsafe extern "system" fn(param0: *mut D3DKMT_GETPROCESSDEVICEREMOVALSUPPORT) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
pub type PFND3DKMT_GETPROCESSSCHEDULINGPRIORITYCLASS = Option<unsafe extern "system" fn(param0: super::winnt::HANDLE, param1: *mut D3DKMT_SCHEDULINGPRIORITYCLASS) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_GETRESOURCEPRESENTPRIVATEDRIVERDATA = Option<unsafe extern "system" fn(param0: *mut super::d3dukmdt::D3DDDI_GETRESOURCEPRESENTPRIVATEDRIVERDATA) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_GETRUNTIMEDATA = Option<unsafe extern "system" fn(param0: *mut D3DKMT_GETRUNTIMEDATA) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_GETSCANLINE = Option<unsafe extern "system" fn(param0: *mut D3DKMT_GETSCANLINE) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_GETSHAREDPRIMARYHANDLE = Option<unsafe extern "system" fn(param0: *mut D3DKMT_GETSHAREDPRIMARYHANDLE) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
pub type PFND3DKMT_GETSHAREDRESOURCEADAPTERLUID = Option<unsafe extern "system" fn(param0: *mut D3DKMT_GETSHAREDRESOURCEADAPTERLUID) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_INVALIDATEACTIVEVIDPN = Option<unsafe extern "system" fn(param0: *const D3DKMT_INVALIDATEACTIVEVIDPN) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_INVALIDATECACHE = Option<unsafe extern "system" fn(param0: *const D3DKMT_INVALIDATECACHE) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_ISFEATUREENABLED = Option<unsafe extern "system" fn(param0: *mut D3DKMT_ISFEATUREENABLED) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_LOCK = Option<unsafe extern "system" fn(param0: *mut D3DKMT_LOCK) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_LOCK2 = Option<unsafe extern "system" fn(param0: *mut D3DKMT_LOCK2) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_MAKERESIDENT = Option<unsafe extern "system" fn(param0: *mut super::d3dukmdt::D3DDDI_MAKERESIDENT) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_MAPGPUVIRTUALADDRESS = Option<unsafe extern "system" fn(param0: *mut super::d3dukmdt::D3DDDI_MAPGPUVIRTUALADDRESS) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_MAPPROCESSDEBUGBLOB = Option<unsafe extern "system" fn(param0: *mut D3DKMT_MAPPROCESSDEBUGBLOB) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_MARKDEVICEASERROR = Option<unsafe extern "system" fn(param0: *const D3DKMT_MARKDEVICEASERROR) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_NOTIFYWORKSUBMISSION = Option<unsafe extern "system" fn(param0: *const D3DKMT_NOTIFY_WORK_SUBMISSION) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_OFFERALLOCATIONS = Option<unsafe extern "system" fn(param0: *const D3DKMT_OFFERALLOCATIONS) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
pub type PFND3DKMT_OPENADAPTERFROMDEVICENAME = Option<unsafe extern "system" fn(param0: *mut D3DKMT_OPENADAPTERFROMDEVICENAME) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
pub type PFND3DKMT_OPENADAPTERFROMGDIDISPLAYNAME = Option<unsafe extern "system" fn(param0: *mut D3DKMT_OPENADAPTERFROMGDIDISPLAYNAME) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
pub type PFND3DKMT_OPENADAPTERFROMHDC = Option<unsafe extern "system" fn(param0: *mut D3DKMT_OPENADAPTERFROMHDC) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
pub type PFND3DKMT_OPENADAPTERFROMLUID = Option<unsafe extern "system" fn(param0: *mut D3DKMT_OPENADAPTERFROMLUID) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_OPENKEYEDMUTEX = Option<unsafe extern "system" fn(param0: *mut D3DKMT_OPENKEYEDMUTEX) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_OPENKEYEDMUTEX2 = Option<unsafe extern "system" fn(param0: *mut D3DKMT_OPENKEYEDMUTEX2) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
pub type PFND3DKMT_OPENKEYEDMUTEXFROMNTHANDLE = Option<unsafe extern "system" fn(param0: *mut D3DKMT_OPENKEYEDMUTEXFROMNTHANDLE) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
pub type PFND3DKMT_OPENNATIVEFENCEFROMNTHANDLE = Option<unsafe extern "system" fn(param0: *mut D3DKMT_OPENNATIVEFENCEFROMNTHANDLE) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
pub type PFND3DKMT_OPENNTHANDLEFROMNAME = Option<unsafe extern "system" fn(param0: *mut D3DKMT_OPENNTHANDLEFROMNAME) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
pub type PFND3DKMT_OPENPROTECTEDSESSIONFROMNTHANDLE = Option<unsafe extern "system" fn(param0: *mut D3DKMT_OPENPROTECTEDSESSIONFROMNTHANDLE) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_OPENRESOURCE = Option<unsafe extern "system" fn(param0: *mut D3DKMT_OPENRESOURCE) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_OPENRESOURCE2 = Option<unsafe extern "system" fn(param0: *mut D3DKMT_OPENRESOURCE) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
pub type PFND3DKMT_OPENRESOURCEFROMNTHANDLE = Option<unsafe extern "system" fn(param0: *mut D3DKMT_OPENRESOURCEFROMNTHANDLE) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_OPENSYNCHRONIZATIONOBJECT = Option<unsafe extern "system" fn(param0: *mut D3DKMT_OPENSYNCHRONIZATIONOBJECT) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
pub type PFND3DKMT_OPENSYNCOBJECTFROMNTHANDLE = Option<unsafe extern "system" fn(param0: *mut D3DKMT_OPENSYNCOBJECTFROMNTHANDLE) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
pub type PFND3DKMT_OPENSYNCOBJECTFROMNTHANDLE2 = Option<unsafe extern "system" fn(param0: *mut D3DKMT_OPENSYNCOBJECTFROMNTHANDLE2) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
pub type PFND3DKMT_OPENSYNCOBJECTNTHANDLEFROMNAME = Option<unsafe extern "system" fn(param0: *mut D3DKMT_OPENSYNCOBJECTNTHANDLEFROMNAME) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "windef"))]
pub type PFND3DKMT_OUTPUTDUPLGETFRAMEINFO = Option<unsafe extern "system" fn(param0: *mut D3DKMT_OUTPUTDUPL_GET_FRAMEINFO) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_OUTPUTDUPLGETMETADATA = Option<unsafe extern "system" fn(param0: *mut D3DKMT_OUTPUTDUPL_METADATA) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "windef"))]
pub type PFND3DKMT_OUTPUTDUPLGETPOINTERSHAPEDATA = Option<unsafe extern "system" fn(param0: *mut D3DKMT_OUTPUTDUPL_GET_POINTER_SHAPE_DATA) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dkmdt", feature = "d3dukmdt", feature = "windef"))]
pub type PFND3DKMT_OUTPUTDUPLPRESENT = Option<unsafe extern "system" fn(param0: *const D3DKMT_OUTPUTDUPLPRESENT) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dkmdt", feature = "d3dukmdt", feature = "windef"))]
pub type PFND3DKMT_OUTPUTDUPLPRESENTTOHWQUEUE = Option<unsafe extern "system" fn(param0: *const D3DKMT_OUTPUTDUPLPRESENTTOHWQUEUE) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_OUTPUTDUPLRELEASEFRAME = Option<unsafe extern "system" fn(param0: *const D3DKMT_OUTPUTDUPL_RELEASE_FRAME) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_PINDIRECTFLIPRESOURCES = Option<unsafe extern "system" fn(param0: *const D3DKMT_PINDIRECTFLIPRESOURCES) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_POLLDISPLAYCHILDREN = Option<unsafe extern "system" fn(param0: *const D3DKMT_POLLDISPLAYCHILDREN) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dkmdt", feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
pub type PFND3DKMT_PRESENT = Option<unsafe extern "system" fn(param0: *mut D3DKMT_PRESENT) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "windef"))]
pub type PFND3DKMT_PRESENTMULTIPLANEOVERLAY = Option<unsafe extern "system" fn(param0: *const D3DKMT_PRESENT_MULTIPLANE_OVERLAY) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "windef"))]
pub type PFND3DKMT_PRESENTMULTIPLANEOVERLAY2 = Option<unsafe extern "system" fn(param0: *const D3DKMT_PRESENT_MULTIPLANE_OVERLAY2) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "windef"))]
pub type PFND3DKMT_PRESENTMULTIPLANEOVERLAY3 = Option<unsafe extern "system" fn(param0: *const D3DKMT_PRESENT_MULTIPLANE_OVERLAY3) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_QUERYADAPTERINFO = Option<unsafe extern "system" fn(param0: *mut D3DKMT_QUERYADAPTERINFO) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_QUERYALLOCATIONRESIDENCY = Option<unsafe extern "system" fn(param0: *const D3DKMT_QUERYALLOCATIONRESIDENCY) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dkmdt", feature = "d3dukmdt"))]
pub type PFND3DKMT_QUERYCLOCKCALIBRATION = Option<unsafe extern "system" fn(param0: *mut super::d3dkmdt::D3DKMT_QUERYCLOCKCALIBRATION) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "minwindef"))]
pub type PFND3DKMT_QUERYFEATUREINTERFACE = Option<unsafe extern "system" fn(param0: *mut D3DKMT_QUERYFEATUREINTERFACE) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
pub type PFND3DKMT_QUERYFSEBLOCK = Option<unsafe extern "system" fn(param0: *mut D3DKMT_QUERYFSEBLOCK) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
pub type PFND3DKMT_QUERYHYBRIDLISTVALUE = Option<unsafe extern "system" fn(param0: *mut D3DKMT_HYBRID_LIST) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
pub type PFND3DKMT_QUERYPROCESSOFFERINFO = Option<unsafe extern "system" fn(param0: *mut D3DKMT_QUERYPROCESSOFFERINFO) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
pub type PFND3DKMT_QUERYPROTECTEDSESSIONINFOFROMNTHANDLE = Option<unsafe extern "system" fn(param0: *mut D3DKMT_QUERYPROTECTEDSESSIONINFOFROMNTHANDLE) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_QUERYPROTECTEDSESSIONSTATUS = Option<unsafe extern "system" fn(param0: *mut D3DKMT_QUERYPROTECTEDSESSIONSTATUS) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_QUERYREMOTEVIDPNSOURCEFROMGDIDISPLAYNAME = Option<unsafe extern "system" fn(param0: *mut D3DKMT_QUERYREMOTEVIDPNSOURCEFROMGDIDISPLAYNAME) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_QUERYRESOURCEINFO = Option<unsafe extern "system" fn(param0: *mut D3DKMT_QUERYRESOURCEINFO) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
pub type PFND3DKMT_QUERYRESOURCEINFOFROMNTHANDLE = Option<unsafe extern "system" fn(param0: *mut D3DKMT_QUERYRESOURCEINFOFROMNTHANDLE) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
pub type PFND3DKMT_QUERYSTATISTICS = Option<unsafe extern "system" fn(param0: *const D3DKMT_QUERYSTATISTICS) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
pub type PFND3DKMT_QUERYVIDEOMEMORYINFO = Option<unsafe extern "system" fn(param0: *mut D3DKMT_QUERYVIDEOMEMORYINFO) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
pub type PFND3DKMT_QUERYVIDPNEXCLUSIVEOWNERSHIP = Option<unsafe extern "system" fn(param0: *mut D3DKMT_QUERYVIDPNEXCLUSIVEOWNERSHIP) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_RECLAIMALLOCATIONS = Option<unsafe extern "system" fn(param0: *mut D3DKMT_RECLAIMALLOCATIONS) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_RECLAIMALLOCATIONS2 = Option<unsafe extern "system" fn(param0: *mut D3DKMT_RECLAIMALLOCATIONS2) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_REGISTERBUDGETCHANGENOTIFICATION = Option<unsafe extern "system" fn(param0: *mut D3DKMT_REGISTERBUDGETCHANGENOTIFICATION) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
pub type PFND3DKMT_REGISTERTRIMNOTIFICATION = Option<unsafe extern "system" fn(param0: *mut D3DKMT_REGISTERTRIMNOTIFICATION) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_RELEASEKEYEDMUTEX = Option<unsafe extern "system" fn(param0: *mut D3DKMT_RELEASEKEYEDMUTEX) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_RELEASEKEYEDMUTEX2 = Option<unsafe extern "system" fn(param0: *mut D3DKMT_RELEASEKEYEDMUTEX2) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
pub type PFND3DKMT_RELEASEPROCESSVIDPNSOURCEOWNERS = Option<unsafe extern "system" fn(param0: super::winnt::HANDLE) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_RENDER = Option<unsafe extern "system" fn(param0: *mut D3DKMT_RENDER) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_RESERVEGPUVIRTUALADDRESS = Option<unsafe extern "system" fn(param0: *mut super::d3dukmdt::D3DDDI_RESERVEGPUVIRTUALADDRESS) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_RESIZERINGBUFFER = Option<unsafe extern "system" fn(param0: *mut D3DKMT_RESIZERINGBUFFER) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_SETALLOCATIONPRIORITY = Option<unsafe extern "system" fn(param0: *const D3DKMT_SETALLOCATIONPRIORITY) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_SETCONTEXTINPROCESSSCHEDULINGPRIORITY = Option<unsafe extern "system" fn(param0: *const D3DKMT_SETCONTEXTINPROCESSSCHEDULINGPRIORITY) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_SETCONTEXTSCHEDULINGPRIORITY = Option<unsafe extern "system" fn(param0: *const D3DKMT_SETCONTEXTSCHEDULINGPRIORITY) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_SETDISPLAYMODE = Option<unsafe extern "system" fn(param0: *mut D3DKMT_SETDISPLAYMODE) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_SETDISPLAYPRIVATEDRIVERFORMAT = Option<unsafe extern "system" fn(param0: *const D3DKMT_SETDISPLAYPRIVATEDRIVERFORMAT) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
pub type PFND3DKMT_SETFSEBLOCK = Option<unsafe extern "system" fn(param0: *const D3DKMT_SETFSEBLOCK) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_SETGAMMARAMP = Option<unsafe extern "system" fn(param0: *const D3DKMT_SETGAMMARAMP) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_SETHWPROTECTIONTEARDOWNRECOVERY = Option<unsafe extern "system" fn(param0: *const D3DKMT_SETHWPROTECTIONTEARDOWNRECOVERY) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
pub type PFND3DKMT_SETHYBRIDLISTVVALUE = Option<unsafe extern "system" fn(param0: *mut D3DKMT_HYBRID_LIST) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
pub type PFND3DKMT_SETPROCESSSCHEDULINGPRIORITYCLASS = Option<unsafe extern "system" fn(param0: super::winnt::HANDLE, param1: D3DKMT_SCHEDULINGPRIORITYCLASS) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_SETQUEUEDLIMIT = Option<unsafe extern "system" fn(param0: *const D3DKMT_SETQUEUEDLIMIT) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_SETSTABLEPOWERSTATE = Option<unsafe extern "system" fn(param0: *const D3DKMT_SETSTABLEPOWERSTATE) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "bcrypt")]
pub type PFND3DKMT_SETSTEREOENABLED = Option<unsafe extern "system" fn(param0: windows_sys::core::BOOL) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_SETSYNCREFRESHCOUNTWAITTARGET = Option<unsafe extern "system" fn(param0: *const D3DKMT_SETSYNCREFRESHCOUNTWAITTARGET) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_SETVIDPNSOURCEHWPROTECTION = Option<unsafe extern "system" fn(param0: *const D3DKMT_SETVIDPNSOURCEHWPROTECTION) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_SETVIDPNSOURCEOWNER = Option<unsafe extern "system" fn(param0: *const D3DKMT_SETVIDPNSOURCEOWNER) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_SETVIDPNSOURCEOWNER1 = Option<unsafe extern "system" fn(param0: *const D3DKMT_SETVIDPNSOURCEOWNER1) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
pub type PFND3DKMT_SETVIDPNSOURCEOWNER2 = Option<unsafe extern "system" fn(param0: *const D3DKMT_SETVIDPNSOURCEOWNER2) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
pub type PFND3DKMT_SHAREDPRIMARYLOCKNOTIFICATION = Option<unsafe extern "system" fn(param0: *const D3DKMT_SHAREDPRIMARYLOCKNOTIFICATION) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
pub type PFND3DKMT_SHAREDPRIMARYUNLOCKNOTIFICATION = Option<unsafe extern "system" fn(param0: *const D3DKMT_SHAREDPRIMARYUNLOCKNOTIFICATION) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
pub type PFND3DKMT_SHAREOBJECTS = Option<unsafe extern "system" fn(cobjects: u32, hobjects: *const super::d3dukmdt::D3DKMT_HANDLE, pobjectattributes: *const OBJECT_ATTRIBUTES, dwdesiredaccess: u32, phsharednthandle: *mut super::winnt::HANDLE) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_SIGNALSYNCHRONIZATIONOBJECT = Option<unsafe extern "system" fn(param0: *const D3DKMT_SIGNALSYNCHRONIZATIONOBJECT) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
pub type PFND3DKMT_SIGNALSYNCHRONIZATIONOBJECT2 = Option<unsafe extern "system" fn(param0: *const D3DKMT_SIGNALSYNCHRONIZATIONOBJECT2) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_SIGNALSYNCHRONIZATIONOBJECTFROMCPU = Option<unsafe extern "system" fn(param0: *const D3DKMT_SIGNALSYNCHRONIZATIONOBJECTFROMCPU) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_SIGNALSYNCHRONIZATIONOBJECTFROMGPU = Option<unsafe extern "system" fn(param0: *const D3DKMT_SIGNALSYNCHRONIZATIONOBJECTFROMGPU) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
pub type PFND3DKMT_SIGNALSYNCHRONIZATIONOBJECTFROMGPU2 = Option<unsafe extern "system" fn(param0: *const D3DKMT_SIGNALSYNCHRONIZATIONOBJECTFROMGPU2) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_SUBMITCOMMAND = Option<unsafe extern "system" fn(param0: *const D3DKMT_SUBMITCOMMAND) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_SUBMITCOMMANDTOHWQUEUE = Option<unsafe extern "system" fn(param0: *const D3DKMT_SUBMITCOMMANDTOHWQUEUE) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dkmdt", feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
pub type PFND3DKMT_SUBMITPRESENTBLTTOHWQUEUE = Option<unsafe extern "system" fn(param0: *const D3DKMT_SUBMITPRESENTBLTTOHWQUEUE) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dkmdt", feature = "d3dukmdt", feature = "windef", feature = "winnt"))]
pub type PFND3DKMT_SUBMITPRESENTTOHWQUEUE = Option<unsafe extern "system" fn(param0: *mut D3DKMT_SUBMITPRESENTTOHWQUEUE) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_SUBMITSIGNALSYNCOBJECTSTOHWQUEUE = Option<unsafe extern "system" fn(param0: *const D3DKMT_SUBMITSIGNALSYNCOBJECTSTOHWQUEUE) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_SUBMITWAITFORSYNCOBJECTSTOHWQUEUE = Option<unsafe extern "system" fn(param0: *const D3DKMT_SUBMITWAITFORSYNCOBJECTSTOHWQUEUE) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "d3dukmdt")]
pub type PFND3DKMT_TRIMNOTIFICATIONCALLBACK = Option<unsafe extern "system" fn(param0: *mut D3DKMT_TRIMNOTIFICATION)>;
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
pub type PFND3DKMT_TRIMPROCESSCOMMITMENT = Option<unsafe extern "system" fn(param0: *mut D3DKMT_TRIMPROCESSCOMMITMENT) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_UNLOCK = Option<unsafe extern "system" fn(param0: *const D3DKMT_UNLOCK) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_UNLOCK2 = Option<unsafe extern "system" fn(param0: *const D3DKMT_UNLOCK2) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_UNMAPPROCESSDEBUGBLOB = Option<unsafe extern "system" fn(param0: *mut D3DKMT_UNMAPPROCESSDEBUGBLOB) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_UNPINDIRECTFLIPRESOURCES = Option<unsafe extern "system" fn(param0: *const D3DKMT_UNPINDIRECTFLIPRESOURCES) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "bcrypt")]
pub type PFND3DKMT_UNREGISTERBUDGETCHANGENOTIFICATION = Option<unsafe extern "system" fn(param0: *mut D3DKMT_UNREGISTERBUDGETCHANGENOTIFICATION) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_UNREGISTERTRIMNOTIFICATION = Option<unsafe extern "system" fn(param0: *mut D3DKMT_UNREGISTERTRIMNOTIFICATION) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_UPDATEALLOCATIONPROPERTY = Option<unsafe extern "system" fn(param0: *mut super::d3dukmdt::D3DDDI_UPDATEALLOCPROPERTY) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_UPDATEGPUVIRTUALADDRESS = Option<unsafe extern "system" fn(param0: *const D3DKMT_UPDATEGPUVIRTUALADDRESS) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_UPDATEOVERLAY = Option<unsafe extern "system" fn(param0: *const D3DKMT_UPDATEOVERLAY) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_WAITFORIDLE = Option<unsafe extern "system" fn(param0: *const D3DKMT_WAITFORIDLE) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_WAITFORSYNCHRONIZATIONOBJECT = Option<unsafe extern "system" fn(param0: *const D3DKMT_WAITFORSYNCHRONIZATIONOBJECT) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_WAITFORSYNCHRONIZATIONOBJECT2 = Option<unsafe extern "system" fn(param0: *const D3DKMT_WAITFORSYNCHRONIZATIONOBJECT2) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
pub type PFND3DKMT_WAITFORSYNCHRONIZATIONOBJECTFROMCPU = Option<unsafe extern "system" fn(param0: *const D3DKMT_WAITFORSYNCHRONIZATIONOBJECTFROMCPU) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_WAITFORSYNCHRONIZATIONOBJECTFROMGPU = Option<unsafe extern "system" fn(param0: *const D3DKMT_WAITFORSYNCHRONIZATIONOBJECTFROMGPU) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt"))]
pub type PFND3DKMT_WAITFORVERTICALBLANKEVENT = Option<unsafe extern "system" fn(param0: *const D3DKMT_WAITFORVERTICALBLANKEVENT) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dukmdt", feature = "winnt"))]
pub type PFND3DKMT_WAITFORVERTICALBLANKEVENT2 = Option<unsafe extern "system" fn(param0: *const D3DKMT_WAITFORVERTICALBLANKEVENT2) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
pub type POBJECT_ATTRIBUTES = *mut OBJECT_ATTRIBUTES;
pub const SHARED_ALLOCATION_ALL_ACCESS: u32 = 983041;
pub const SHARED_ALLOCATION_WRITE: u32 = 1;
