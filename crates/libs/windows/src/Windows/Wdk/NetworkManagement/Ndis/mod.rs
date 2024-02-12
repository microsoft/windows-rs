#[inline]
pub unsafe fn NdisAcquireReadWriteLock<P0>(lock: *mut NDIS_RW_LOCK, fwrite: P0, lockstate: *mut LOCK_STATE)
where
    P0: ::windows_core::IntoParam<super::super::super::Win32::Foundation::BOOLEAN>,
{
    ::windows_targets::link!("ndis.sys" "system" fn NdisAcquireReadWriteLock(lock : *mut NDIS_RW_LOCK, fwrite : super::super::super::Win32::Foundation:: BOOLEAN, lockstate : *mut LOCK_STATE));
    NdisAcquireReadWriteLock(lock, fwrite.into_param().abi(), lockstate)
}
#[inline]
pub unsafe fn NdisAllocateMemoryWithTag(virtualaddress: *mut *mut ::core::ffi::c_void, length: u32, tag: u32) -> i32 {
    ::windows_targets::link!("ndis.sys" "system" fn NdisAllocateMemoryWithTag(virtualaddress : *mut *mut ::core::ffi::c_void, length : u32, tag : u32) -> i32);
    NdisAllocateMemoryWithTag(virtualaddress, length, tag)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn NdisCancelTimer(timer: *const NDIS_TIMER) -> super::super::super::Win32::Foundation::BOOLEAN {
    ::windows_targets::link!("ndis.sys" "system" fn NdisCancelTimer(timer : *const NDIS_TIMER, timercancelled : *mut super::super::super::Win32::Foundation:: BOOLEAN));
    let mut result__ = ::std::mem::zeroed();
    NdisCancelTimer(timer, &mut result__);
    result__
}
#[inline]
pub unsafe fn NdisClAddParty(ndisvchandle: *const ::core::ffi::c_void, protocolpartycontext: *const ::core::ffi::c_void, callparameters: *mut CO_CALL_PARAMETERS, ndispartyhandle: *mut *mut ::core::ffi::c_void) -> i32 {
    ::windows_targets::link!("ndis.sys" "system" fn NdisClAddParty(ndisvchandle : *const ::core::ffi::c_void, protocolpartycontext : *const ::core::ffi::c_void, callparameters : *mut CO_CALL_PARAMETERS, ndispartyhandle : *mut *mut ::core::ffi::c_void) -> i32);
    NdisClAddParty(ndisvchandle, protocolpartycontext, callparameters, ndispartyhandle)
}
#[inline]
pub unsafe fn NdisClCloseAddressFamily(ndisafhandle: *const ::core::ffi::c_void) -> i32 {
    ::windows_targets::link!("ndis.sys" "system" fn NdisClCloseAddressFamily(ndisafhandle : *const ::core::ffi::c_void) -> i32);
    NdisClCloseAddressFamily(ndisafhandle)
}
#[inline]
pub unsafe fn NdisClCloseCall(ndisvchandle: *const ::core::ffi::c_void, ndispartyhandle: ::core::option::Option<*const ::core::ffi::c_void>, buffer: ::core::option::Option<*const ::core::ffi::c_void>, size: u32) -> i32 {
    ::windows_targets::link!("ndis.sys" "system" fn NdisClCloseCall(ndisvchandle : *const ::core::ffi::c_void, ndispartyhandle : *const ::core::ffi::c_void, buffer : *const ::core::ffi::c_void, size : u32) -> i32);
    NdisClCloseCall(ndisvchandle, ::core::mem::transmute(ndispartyhandle.unwrap_or(::std::ptr::null())), ::core::mem::transmute(buffer.unwrap_or(::std::ptr::null())), size)
}
#[inline]
pub unsafe fn NdisClDeregisterSap(ndissaphandle: *const ::core::ffi::c_void) -> i32 {
    ::windows_targets::link!("ndis.sys" "system" fn NdisClDeregisterSap(ndissaphandle : *const ::core::ffi::c_void) -> i32);
    NdisClDeregisterSap(ndissaphandle)
}
#[inline]
pub unsafe fn NdisClDropParty(ndispartyhandle: *const ::core::ffi::c_void, buffer: ::core::option::Option<*const ::core::ffi::c_void>, size: u32) -> i32 {
    ::windows_targets::link!("ndis.sys" "system" fn NdisClDropParty(ndispartyhandle : *const ::core::ffi::c_void, buffer : *const ::core::ffi::c_void, size : u32) -> i32);
    NdisClDropParty(ndispartyhandle, ::core::mem::transmute(buffer.unwrap_or(::std::ptr::null())), size)
}
#[inline]
pub unsafe fn NdisClGetProtocolVcContextFromTapiCallId(tapicallid: super::super::super::Win32::Foundation::UNICODE_STRING, protocolvccontext: *mut *mut ::core::ffi::c_void) -> i32 {
    ::windows_targets::link!("ndis.sys" "system" fn NdisClGetProtocolVcContextFromTapiCallId(tapicallid : super::super::super::Win32::Foundation:: UNICODE_STRING, protocolvccontext : *mut *mut ::core::ffi::c_void) -> i32);
    NdisClGetProtocolVcContextFromTapiCallId(::core::mem::transmute(tapicallid), protocolvccontext)
}
#[inline]
pub unsafe fn NdisClIncomingCallComplete(status: i32, ndisvchandle: *const ::core::ffi::c_void, callparameters: *const CO_CALL_PARAMETERS) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisClIncomingCallComplete(status : i32, ndisvchandle : *const ::core::ffi::c_void, callparameters : *const CO_CALL_PARAMETERS));
    NdisClIncomingCallComplete(status, ndisvchandle, callparameters)
}
#[inline]
pub unsafe fn NdisClMakeCall(ndisvchandle: *const ::core::ffi::c_void, callparameters: *mut CO_CALL_PARAMETERS, protocolpartycontext: ::core::option::Option<*const ::core::ffi::c_void>, ndispartyhandle: ::core::option::Option<*mut *mut ::core::ffi::c_void>) -> i32 {
    ::windows_targets::link!("ndis.sys" "system" fn NdisClMakeCall(ndisvchandle : *const ::core::ffi::c_void, callparameters : *mut CO_CALL_PARAMETERS, protocolpartycontext : *const ::core::ffi::c_void, ndispartyhandle : *mut *mut ::core::ffi::c_void) -> i32);
    NdisClMakeCall(ndisvchandle, callparameters, ::core::mem::transmute(protocolpartycontext.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ndispartyhandle.unwrap_or(::std::ptr::null_mut())))
}
#[inline]
pub unsafe fn NdisClModifyCallQoS(ndisvchandle: *const ::core::ffi::c_void, callparameters: *const CO_CALL_PARAMETERS) -> i32 {
    ::windows_targets::link!("ndis.sys" "system" fn NdisClModifyCallQoS(ndisvchandle : *const ::core::ffi::c_void, callparameters : *const CO_CALL_PARAMETERS) -> i32);
    NdisClModifyCallQoS(ndisvchandle, callparameters)
}
#[inline]
pub unsafe fn NdisClRegisterSap(ndisafhandle: *const ::core::ffi::c_void, protocolsapcontext: *const ::core::ffi::c_void, sap: *const CO_SAP, ndissaphandle: *mut *mut ::core::ffi::c_void) -> i32 {
    ::windows_targets::link!("ndis.sys" "system" fn NdisClRegisterSap(ndisafhandle : *const ::core::ffi::c_void, protocolsapcontext : *const ::core::ffi::c_void, sap : *const CO_SAP, ndissaphandle : *mut *mut ::core::ffi::c_void) -> i32);
    NdisClRegisterSap(ndisafhandle, protocolsapcontext, sap, ndissaphandle)
}
#[inline]
pub unsafe fn NdisCloseConfiguration(configurationhandle: *const ::core::ffi::c_void) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisCloseConfiguration(configurationhandle : *const ::core::ffi::c_void));
    NdisCloseConfiguration(configurationhandle)
}
#[inline]
pub unsafe fn NdisCloseFile(filehandle: *const ::core::ffi::c_void) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisCloseFile(filehandle : *const ::core::ffi::c_void));
    NdisCloseFile(filehandle)
}
#[inline]
pub unsafe fn NdisCmActivateVc(ndisvchandle: *const ::core::ffi::c_void, callparameters: *mut CO_CALL_PARAMETERS) -> i32 {
    ::windows_targets::link!("ndis.sys" "system" fn NdisCmActivateVc(ndisvchandle : *const ::core::ffi::c_void, callparameters : *mut CO_CALL_PARAMETERS) -> i32);
    NdisCmActivateVc(ndisvchandle, callparameters)
}
#[inline]
pub unsafe fn NdisCmAddPartyComplete(status: i32, ndispartyhandle: *const ::core::ffi::c_void, callmgrpartycontext: ::core::option::Option<*const ::core::ffi::c_void>, callparameters: *const CO_CALL_PARAMETERS) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisCmAddPartyComplete(status : i32, ndispartyhandle : *const ::core::ffi::c_void, callmgrpartycontext : *const ::core::ffi::c_void, callparameters : *const CO_CALL_PARAMETERS));
    NdisCmAddPartyComplete(status, ndispartyhandle, ::core::mem::transmute(callmgrpartycontext.unwrap_or(::std::ptr::null())), callparameters)
}
#[inline]
pub unsafe fn NdisCmCloseAddressFamilyComplete(status: i32, ndisafhandle: *const ::core::ffi::c_void) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisCmCloseAddressFamilyComplete(status : i32, ndisafhandle : *const ::core::ffi::c_void));
    NdisCmCloseAddressFamilyComplete(status, ndisafhandle)
}
#[inline]
pub unsafe fn NdisCmCloseCallComplete(status: i32, ndisvchandle: *const ::core::ffi::c_void, ndispartyhandle: ::core::option::Option<*const ::core::ffi::c_void>) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisCmCloseCallComplete(status : i32, ndisvchandle : *const ::core::ffi::c_void, ndispartyhandle : *const ::core::ffi::c_void));
    NdisCmCloseCallComplete(status, ndisvchandle, ::core::mem::transmute(ndispartyhandle.unwrap_or(::std::ptr::null())))
}
#[inline]
pub unsafe fn NdisCmDeactivateVc(ndisvchandle: *const ::core::ffi::c_void) -> i32 {
    ::windows_targets::link!("ndis.sys" "system" fn NdisCmDeactivateVc(ndisvchandle : *const ::core::ffi::c_void) -> i32);
    NdisCmDeactivateVc(ndisvchandle)
}
#[inline]
pub unsafe fn NdisCmDeregisterSapComplete(status: i32, ndissaphandle: *const ::core::ffi::c_void) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisCmDeregisterSapComplete(status : i32, ndissaphandle : *const ::core::ffi::c_void));
    NdisCmDeregisterSapComplete(status, ndissaphandle)
}
#[inline]
pub unsafe fn NdisCmDispatchCallConnected(ndisvchandle: *const ::core::ffi::c_void) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisCmDispatchCallConnected(ndisvchandle : *const ::core::ffi::c_void));
    NdisCmDispatchCallConnected(ndisvchandle)
}
#[inline]
pub unsafe fn NdisCmDispatchIncomingCall(ndissaphandle: *const ::core::ffi::c_void, ndisvchandle: *const ::core::ffi::c_void, callparameters: *const CO_CALL_PARAMETERS) -> i32 {
    ::windows_targets::link!("ndis.sys" "system" fn NdisCmDispatchIncomingCall(ndissaphandle : *const ::core::ffi::c_void, ndisvchandle : *const ::core::ffi::c_void, callparameters : *const CO_CALL_PARAMETERS) -> i32);
    NdisCmDispatchIncomingCall(ndissaphandle, ndisvchandle, callparameters)
}
#[inline]
pub unsafe fn NdisCmDispatchIncomingCallQoSChange(ndisvchandle: *const ::core::ffi::c_void, callparameters: *const CO_CALL_PARAMETERS) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisCmDispatchIncomingCallQoSChange(ndisvchandle : *const ::core::ffi::c_void, callparameters : *const CO_CALL_PARAMETERS));
    NdisCmDispatchIncomingCallQoSChange(ndisvchandle, callparameters)
}
#[inline]
pub unsafe fn NdisCmDispatchIncomingCloseCall(closestatus: i32, ndisvchandle: *const ::core::ffi::c_void, buffer: ::core::option::Option<*const ::core::ffi::c_void>, size: u32) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisCmDispatchIncomingCloseCall(closestatus : i32, ndisvchandle : *const ::core::ffi::c_void, buffer : *const ::core::ffi::c_void, size : u32));
    NdisCmDispatchIncomingCloseCall(closestatus, ndisvchandle, ::core::mem::transmute(buffer.unwrap_or(::std::ptr::null())), size)
}
#[inline]
pub unsafe fn NdisCmDispatchIncomingDropParty(dropstatus: i32, ndispartyhandle: *const ::core::ffi::c_void, buffer: ::core::option::Option<*const ::core::ffi::c_void>, size: u32) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisCmDispatchIncomingDropParty(dropstatus : i32, ndispartyhandle : *const ::core::ffi::c_void, buffer : *const ::core::ffi::c_void, size : u32));
    NdisCmDispatchIncomingDropParty(dropstatus, ndispartyhandle, ::core::mem::transmute(buffer.unwrap_or(::std::ptr::null())), size)
}
#[inline]
pub unsafe fn NdisCmDropPartyComplete(status: i32, ndispartyhandle: *const ::core::ffi::c_void) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisCmDropPartyComplete(status : i32, ndispartyhandle : *const ::core::ffi::c_void));
    NdisCmDropPartyComplete(status, ndispartyhandle)
}
#[inline]
pub unsafe fn NdisCmMakeCallComplete(status: i32, ndisvchandle: *const ::core::ffi::c_void, ndispartyhandle: ::core::option::Option<*const ::core::ffi::c_void>, callmgrpartycontext: ::core::option::Option<*const ::core::ffi::c_void>, callparameters: *const CO_CALL_PARAMETERS) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisCmMakeCallComplete(status : i32, ndisvchandle : *const ::core::ffi::c_void, ndispartyhandle : *const ::core::ffi::c_void, callmgrpartycontext : *const ::core::ffi::c_void, callparameters : *const CO_CALL_PARAMETERS));
    NdisCmMakeCallComplete(status, ndisvchandle, ::core::mem::transmute(ndispartyhandle.unwrap_or(::std::ptr::null())), ::core::mem::transmute(callmgrpartycontext.unwrap_or(::std::ptr::null())), callparameters)
}
#[inline]
pub unsafe fn NdisCmModifyCallQoSComplete(status: i32, ndisvchandle: *const ::core::ffi::c_void, callparameters: *const CO_CALL_PARAMETERS) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisCmModifyCallQoSComplete(status : i32, ndisvchandle : *const ::core::ffi::c_void, callparameters : *const CO_CALL_PARAMETERS));
    NdisCmModifyCallQoSComplete(status, ndisvchandle, callparameters)
}
#[inline]
pub unsafe fn NdisCmOpenAddressFamilyComplete(status: i32, ndisafhandle: *const ::core::ffi::c_void, callmgrafcontext: *const ::core::ffi::c_void) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisCmOpenAddressFamilyComplete(status : i32, ndisafhandle : *const ::core::ffi::c_void, callmgrafcontext : *const ::core::ffi::c_void));
    NdisCmOpenAddressFamilyComplete(status, ndisafhandle, callmgrafcontext)
}
#[inline]
pub unsafe fn NdisCmRegisterSapComplete(status: i32, ndissaphandle: *const ::core::ffi::c_void, callmgrsapcontext: *const ::core::ffi::c_void) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisCmRegisterSapComplete(status : i32, ndissaphandle : *const ::core::ffi::c_void, callmgrsapcontext : *const ::core::ffi::c_void));
    NdisCmRegisterSapComplete(status, ndissaphandle, callmgrsapcontext)
}
#[inline]
pub unsafe fn NdisCoAssignInstanceName(ndisvchandle: *const ::core::ffi::c_void, baseinstancename: *const super::super::super::Win32::Foundation::UNICODE_STRING, vcinstancename: ::core::option::Option<*mut super::super::super::Win32::Foundation::UNICODE_STRING>) -> i32 {
    ::windows_targets::link!("ndis.sys" "system" fn NdisCoAssignInstanceName(ndisvchandle : *const ::core::ffi::c_void, baseinstancename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, vcinstancename : *mut super::super::super::Win32::Foundation:: UNICODE_STRING) -> i32);
    NdisCoAssignInstanceName(ndisvchandle, baseinstancename, ::core::mem::transmute(vcinstancename.unwrap_or(::std::ptr::null_mut())))
}
#[inline]
pub unsafe fn NdisCoCreateVc(ndisbindinghandle: *const ::core::ffi::c_void, ndisafhandle: ::core::option::Option<*const ::core::ffi::c_void>, protocolvccontext: *const ::core::ffi::c_void, ndisvchandle: *mut *mut ::core::ffi::c_void) -> i32 {
    ::windows_targets::link!("ndis.sys" "system" fn NdisCoCreateVc(ndisbindinghandle : *const ::core::ffi::c_void, ndisafhandle : *const ::core::ffi::c_void, protocolvccontext : *const ::core::ffi::c_void, ndisvchandle : *mut *mut ::core::ffi::c_void) -> i32);
    NdisCoCreateVc(ndisbindinghandle, ::core::mem::transmute(ndisafhandle.unwrap_or(::std::ptr::null())), protocolvccontext, ndisvchandle)
}
#[inline]
pub unsafe fn NdisCoDeleteVc(ndisvchandle: *const ::core::ffi::c_void) -> i32 {
    ::windows_targets::link!("ndis.sys" "system" fn NdisCoDeleteVc(ndisvchandle : *const ::core::ffi::c_void) -> i32);
    NdisCoDeleteVc(ndisvchandle)
}
#[inline]
pub unsafe fn NdisCoGetTapiCallId(ndisvchandle: *const ::core::ffi::c_void, tapicallid: *mut VAR_STRING) -> i32 {
    ::windows_targets::link!("ndis.sys" "system" fn NdisCoGetTapiCallId(ndisvchandle : *const ::core::ffi::c_void, tapicallid : *mut VAR_STRING) -> i32);
    NdisCoGetTapiCallId(ndisvchandle, tapicallid)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn NdisCompleteDmaTransfer<P0>(status: *mut i32, ndisdmahandle: *mut ::core::ffi::c_void, buffer: *mut super::super::Foundation::MDL, offset: u32, length: u32, writetodevice: P0)
where
    P0: ::windows_core::IntoParam<super::super::super::Win32::Foundation::BOOLEAN>,
{
    ::windows_targets::link!("ndis.sys" "system" fn NdisCompleteDmaTransfer(status : *mut i32, ndisdmahandle : *mut ::core::ffi::c_void, buffer : *mut super::super::Foundation:: MDL, offset : u32, length : u32, writetodevice : super::super::super::Win32::Foundation:: BOOLEAN));
    NdisCompleteDmaTransfer(status, ndisdmahandle, buffer, offset, length, writetodevice.into_param().abi())
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn NdisCopyBuffer(status: *mut i32, buffer: *mut *mut super::super::Foundation::MDL, poolhandle: *const ::core::ffi::c_void, memorydescriptor: *const ::core::ffi::c_void, offset: u32, length: u32) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisCopyBuffer(status : *mut i32, buffer : *mut *mut super::super::Foundation:: MDL, poolhandle : *const ::core::ffi::c_void, memorydescriptor : *const ::core::ffi::c_void, offset : u32, length : u32));
    NdisCopyBuffer(status, buffer, poolhandle, memorydescriptor, offset, length)
}
#[inline]
pub unsafe fn NdisDeregisterTdiCallBack() {
    ::windows_targets::link!("ndis.sys" "system" fn NdisDeregisterTdiCallBack());
    NdisDeregisterTdiCallBack()
}
#[inline]
pub unsafe fn NdisFreeMemory(virtualaddress: *const ::core::ffi::c_void, length: u32, memoryflags: u32) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisFreeMemory(virtualaddress : *const ::core::ffi::c_void, length : u32, memoryflags : u32));
    NdisFreeMemory(virtualaddress, length, memoryflags)
}
#[inline]
pub unsafe fn NdisGeneratePartialCancelId() -> u8 {
    ::windows_targets::link!("ndis.sys" "system" fn NdisGeneratePartialCancelId() -> u8);
    NdisGeneratePartialCancelId()
}
#[inline]
pub unsafe fn NdisGetCurrentProcessorCounts(pidlecount: *mut u32, pkernelanduser: *mut u32, pindex: *mut u32) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisGetCurrentProcessorCounts(pidlecount : *mut u32, pkernelanduser : *mut u32, pindex : *mut u32));
    NdisGetCurrentProcessorCounts(pidlecount, pkernelanduser, pindex)
}
#[inline]
pub unsafe fn NdisGetCurrentProcessorCpuUsage() -> u32 {
    ::windows_targets::link!("ndis.sys" "system" fn NdisGetCurrentProcessorCpuUsage(pcpuusage : *mut u32));
    let mut result__ = ::std::mem::zeroed();
    NdisGetCurrentProcessorCpuUsage(&mut result__);
    result__
}
#[inline]
pub unsafe fn NdisGetRoutineAddress(ndisroutinename: *const super::super::super::Win32::Foundation::UNICODE_STRING) -> *mut ::core::ffi::c_void {
    ::windows_targets::link!("ndis.sys" "system" fn NdisGetRoutineAddress(ndisroutinename : *const super::super::super::Win32::Foundation:: UNICODE_STRING) -> *mut ::core::ffi::c_void);
    NdisGetRoutineAddress(ndisroutinename)
}
#[inline]
pub unsafe fn NdisGetSharedDataAlignment() -> u32 {
    ::windows_targets::link!("ndis.sys" "system" fn NdisGetSharedDataAlignment() -> u32);
    NdisGetSharedDataAlignment()
}
#[inline]
pub unsafe fn NdisGetVersion() -> u32 {
    ::windows_targets::link!("ndis.sys" "system" fn NdisGetVersion() -> u32);
    NdisGetVersion()
}
#[inline]
pub unsafe fn NdisIMAssociateMiniport(driverhandle: *const ::core::ffi::c_void, protocolhandle: *const ::core::ffi::c_void) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisIMAssociateMiniport(driverhandle : *const ::core::ffi::c_void, protocolhandle : *const ::core::ffi::c_void));
    NdisIMAssociateMiniport(driverhandle, protocolhandle)
}
#[inline]
pub unsafe fn NdisIMCancelInitializeDeviceInstance(driverhandle: *const ::core::ffi::c_void, deviceinstance: *const super::super::super::Win32::Foundation::UNICODE_STRING) -> i32 {
    ::windows_targets::link!("ndis.sys" "system" fn NdisIMCancelInitializeDeviceInstance(driverhandle : *const ::core::ffi::c_void, deviceinstance : *const super::super::super::Win32::Foundation:: UNICODE_STRING) -> i32);
    NdisIMCancelInitializeDeviceInstance(driverhandle, deviceinstance)
}
#[inline]
pub unsafe fn NdisIMDeInitializeDeviceInstance(ndisminiporthandle: *const ::core::ffi::c_void) -> i32 {
    ::windows_targets::link!("ndis.sys" "system" fn NdisIMDeInitializeDeviceInstance(ndisminiporthandle : *const ::core::ffi::c_void) -> i32);
    NdisIMDeInitializeDeviceInstance(ndisminiporthandle)
}
#[inline]
pub unsafe fn NdisIMGetBindingContext(ndisbindinghandle: *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void {
    ::windows_targets::link!("ndis.sys" "system" fn NdisIMGetBindingContext(ndisbindinghandle : *const ::core::ffi::c_void) -> *mut ::core::ffi::c_void);
    NdisIMGetBindingContext(ndisbindinghandle)
}
#[inline]
pub unsafe fn NdisIMInitializeDeviceInstanceEx(driverhandle: *const ::core::ffi::c_void, driverinstance: *const super::super::super::Win32::Foundation::UNICODE_STRING, devicecontext: ::core::option::Option<*const ::core::ffi::c_void>) -> i32 {
    ::windows_targets::link!("ndis.sys" "system" fn NdisIMInitializeDeviceInstanceEx(driverhandle : *const ::core::ffi::c_void, driverinstance : *const super::super::super::Win32::Foundation:: UNICODE_STRING, devicecontext : *const ::core::ffi::c_void) -> i32);
    NdisIMInitializeDeviceInstanceEx(driverhandle, driverinstance, ::core::mem::transmute(devicecontext.unwrap_or(::std::ptr::null())))
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn NdisInitializeEvent() -> NDIS_EVENT {
    ::windows_targets::link!("ndis.sys" "system" fn NdisInitializeEvent(event : *mut NDIS_EVENT));
    let mut result__ = ::std::mem::zeroed();
    NdisInitializeEvent(&mut result__);
    result__
}
#[inline]
pub unsafe fn NdisInitializeReadWriteLock(lock: *mut NDIS_RW_LOCK) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisInitializeReadWriteLock(lock : *mut NDIS_RW_LOCK));
    NdisInitializeReadWriteLock(lock)
}
#[inline]
pub unsafe fn NdisInitializeString(destination: *mut super::super::super::Win32::Foundation::UNICODE_STRING, source: *const u8) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisInitializeString(destination : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, source : *const u8));
    NdisInitializeString(destination, source)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn NdisInitializeTimer(timer: *mut NDIS_TIMER, timerfunction: PNDIS_TIMER_FUNCTION, functioncontext: ::core::option::Option<*const ::core::ffi::c_void>) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisInitializeTimer(timer : *mut NDIS_TIMER, timerfunction : PNDIS_TIMER_FUNCTION, functioncontext : *const ::core::ffi::c_void));
    NdisInitializeTimer(timer, timerfunction, ::core::mem::transmute(functioncontext.unwrap_or(::std::ptr::null())))
}
#[inline]
pub unsafe fn NdisMAllocateSharedMemory<P0>(miniportadapterhandle: *const ::core::ffi::c_void, length: u32, cached: P0, virtualaddress: *mut *mut ::core::ffi::c_void, physicaladdress: *mut i64)
where
    P0: ::windows_core::IntoParam<super::super::super::Win32::Foundation::BOOLEAN>,
{
    ::windows_targets::link!("ndis.sys" "system" fn NdisMAllocateSharedMemory(miniportadapterhandle : *const ::core::ffi::c_void, length : u32, cached : super::super::super::Win32::Foundation:: BOOLEAN, virtualaddress : *mut *mut ::core::ffi::c_void, physicaladdress : *mut i64));
    NdisMAllocateSharedMemory(miniportadapterhandle, length, cached.into_param().abi(), virtualaddress, physicaladdress)
}
#[inline]
pub unsafe fn NdisMAllocateSharedMemoryAsync<P0>(miniportadapterhandle: *const ::core::ffi::c_void, length: u32, cached: P0, context: *const ::core::ffi::c_void) -> i32
where
    P0: ::windows_core::IntoParam<super::super::super::Win32::Foundation::BOOLEAN>,
{
    ::windows_targets::link!("ndis.sys" "system" fn NdisMAllocateSharedMemoryAsync(miniportadapterhandle : *const ::core::ffi::c_void, length : u32, cached : super::super::super::Win32::Foundation:: BOOLEAN, context : *const ::core::ffi::c_void) -> i32);
    NdisMAllocateSharedMemoryAsync(miniportadapterhandle, length, cached.into_param().abi(), context)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn NdisMCancelTimer(timer: *const NDIS_MINIPORT_TIMER) -> super::super::super::Win32::Foundation::BOOLEAN {
    ::windows_targets::link!("ndis.sys" "system" fn NdisMCancelTimer(timer : *const NDIS_MINIPORT_TIMER, timercancelled : *mut super::super::super::Win32::Foundation:: BOOLEAN));
    let mut result__ = ::std::mem::zeroed();
    NdisMCancelTimer(timer, &mut result__);
    result__
}
#[inline]
pub unsafe fn NdisMCloseLog(loghandle: *const ::core::ffi::c_void) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisMCloseLog(loghandle : *const ::core::ffi::c_void));
    NdisMCloseLog(loghandle)
}
#[inline]
pub unsafe fn NdisMCmActivateVc(ndisvchandle: *const ::core::ffi::c_void, callparameters: *const CO_CALL_PARAMETERS) -> i32 {
    ::windows_targets::link!("ndis.sys" "system" fn NdisMCmActivateVc(ndisvchandle : *const ::core::ffi::c_void, callparameters : *const CO_CALL_PARAMETERS) -> i32);
    NdisMCmActivateVc(ndisvchandle, callparameters)
}
#[inline]
pub unsafe fn NdisMCmCreateVc(miniportadapterhandle: *const ::core::ffi::c_void, ndisafhandle: *const ::core::ffi::c_void, miniportvccontext: *const ::core::ffi::c_void, ndisvchandle: *mut *mut ::core::ffi::c_void) -> i32 {
    ::windows_targets::link!("ndis.sys" "system" fn NdisMCmCreateVc(miniportadapterhandle : *const ::core::ffi::c_void, ndisafhandle : *const ::core::ffi::c_void, miniportvccontext : *const ::core::ffi::c_void, ndisvchandle : *mut *mut ::core::ffi::c_void) -> i32);
    NdisMCmCreateVc(miniportadapterhandle, ndisafhandle, miniportvccontext, ndisvchandle)
}
#[inline]
pub unsafe fn NdisMCmDeactivateVc(ndisvchandle: *const ::core::ffi::c_void) -> i32 {
    ::windows_targets::link!("ndis.sys" "system" fn NdisMCmDeactivateVc(ndisvchandle : *const ::core::ffi::c_void) -> i32);
    NdisMCmDeactivateVc(ndisvchandle)
}
#[inline]
pub unsafe fn NdisMCmDeleteVc(ndisvchandle: *const ::core::ffi::c_void) -> i32 {
    ::windows_targets::link!("ndis.sys" "system" fn NdisMCmDeleteVc(ndisvchandle : *const ::core::ffi::c_void) -> i32);
    NdisMCmDeleteVc(ndisvchandle)
}
#[inline]
pub unsafe fn NdisMCmRegisterAddressFamily(miniportadapterhandle: *mut ::core::ffi::c_void, addressfamily: *mut CO_ADDRESS_FAMILY, cmcharacteristics: *mut NDIS_CALL_MANAGER_CHARACTERISTICS, sizeofcmcharacteristics: u32) -> i32 {
    ::windows_targets::link!("ndis.sys" "system" fn NdisMCmRegisterAddressFamily(miniportadapterhandle : *mut ::core::ffi::c_void, addressfamily : *mut CO_ADDRESS_FAMILY, cmcharacteristics : *mut NDIS_CALL_MANAGER_CHARACTERISTICS, sizeofcmcharacteristics : u32) -> i32);
    NdisMCmRegisterAddressFamily(miniportadapterhandle, addressfamily, cmcharacteristics, sizeofcmcharacteristics)
}
#[inline]
pub unsafe fn NdisMCoActivateVcComplete(status: i32, ndisvchandle: *const ::core::ffi::c_void, callparameters: *const CO_CALL_PARAMETERS) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisMCoActivateVcComplete(status : i32, ndisvchandle : *const ::core::ffi::c_void, callparameters : *const CO_CALL_PARAMETERS));
    NdisMCoActivateVcComplete(status, ndisvchandle, callparameters)
}
#[inline]
pub unsafe fn NdisMCoDeactivateVcComplete(status: i32, ndisvchandle: *const ::core::ffi::c_void) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisMCoDeactivateVcComplete(status : i32, ndisvchandle : *const ::core::ffi::c_void));
    NdisMCoDeactivateVcComplete(status, ndisvchandle)
}
#[inline]
pub unsafe fn NdisMCreateLog(miniportadapterhandle: *const ::core::ffi::c_void, size: u32, loghandle: *mut *mut ::core::ffi::c_void) -> i32 {
    ::windows_targets::link!("ndis.sys" "system" fn NdisMCreateLog(miniportadapterhandle : *const ::core::ffi::c_void, size : u32, loghandle : *mut *mut ::core::ffi::c_void) -> i32);
    NdisMCreateLog(miniportadapterhandle, size, loghandle)
}
#[inline]
pub unsafe fn NdisMDeregisterDmaChannel(miniportdmahandle: *const ::core::ffi::c_void) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisMDeregisterDmaChannel(miniportdmahandle : *const ::core::ffi::c_void));
    NdisMDeregisterDmaChannel(miniportdmahandle)
}
#[inline]
pub unsafe fn NdisMDeregisterIoPortRange(miniportadapterhandle: *const ::core::ffi::c_void, initialport: u32, numberofports: u32, portoffset: *const ::core::ffi::c_void) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisMDeregisterIoPortRange(miniportadapterhandle : *const ::core::ffi::c_void, initialport : u32, numberofports : u32, portoffset : *const ::core::ffi::c_void));
    NdisMDeregisterIoPortRange(miniportadapterhandle, initialport, numberofports, portoffset)
}
#[inline]
pub unsafe fn NdisMFlushLog(loghandle: *const ::core::ffi::c_void) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisMFlushLog(loghandle : *const ::core::ffi::c_void));
    NdisMFlushLog(loghandle)
}
#[inline]
pub unsafe fn NdisMFreeSharedMemory<P0>(miniportadapterhandle: *const ::core::ffi::c_void, length: u32, cached: P0, virtualaddress: *const ::core::ffi::c_void, physicaladdress: i64)
where
    P0: ::windows_core::IntoParam<super::super::super::Win32::Foundation::BOOLEAN>,
{
    ::windows_targets::link!("ndis.sys" "system" fn NdisMFreeSharedMemory(miniportadapterhandle : *const ::core::ffi::c_void, length : u32, cached : super::super::super::Win32::Foundation:: BOOLEAN, virtualaddress : *const ::core::ffi::c_void, physicaladdress : i64));
    NdisMFreeSharedMemory(miniportadapterhandle, length, cached.into_param().abi(), virtualaddress, physicaladdress)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn NdisMGetDeviceProperty(miniportadapterhandle: *const ::core::ffi::c_void, physicaldeviceobject: ::core::option::Option<*mut *mut super::super::Foundation::DEVICE_OBJECT>, functionaldeviceobject: ::core::option::Option<*mut *mut super::super::Foundation::DEVICE_OBJECT>, nextdeviceobject: ::core::option::Option<*mut *mut super::super::Foundation::DEVICE_OBJECT>, allocatedresources: ::core::option::Option<*mut *mut super::super::System::SystemServices::CM_RESOURCE_LIST>, allocatedresourcestranslated: ::core::option::Option<*mut *mut super::super::System::SystemServices::CM_RESOURCE_LIST>) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisMGetDeviceProperty(miniportadapterhandle : *const ::core::ffi::c_void, physicaldeviceobject : *mut *mut super::super::Foundation:: DEVICE_OBJECT, functionaldeviceobject : *mut *mut super::super::Foundation:: DEVICE_OBJECT, nextdeviceobject : *mut *mut super::super::Foundation:: DEVICE_OBJECT, allocatedresources : *mut *mut super::super::System::SystemServices:: CM_RESOURCE_LIST, allocatedresourcestranslated : *mut *mut super::super::System::SystemServices:: CM_RESOURCE_LIST));
    NdisMGetDeviceProperty(miniportadapterhandle, ::core::mem::transmute(physicaldeviceobject.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(functionaldeviceobject.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(nextdeviceobject.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(allocatedresources.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(allocatedresourcestranslated.unwrap_or(::std::ptr::null_mut())))
}
#[inline]
pub unsafe fn NdisMGetDmaAlignment(miniportadapterhandle: *const ::core::ffi::c_void) -> u32 {
    ::windows_targets::link!("ndis.sys" "system" fn NdisMGetDmaAlignment(miniportadapterhandle : *const ::core::ffi::c_void) -> u32);
    NdisMGetDmaAlignment(miniportadapterhandle)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn NdisMInitializeTimer(timer: *const NDIS_MINIPORT_TIMER, miniportadapterhandle: *const ::core::ffi::c_void, timerfunction: PNDIS_TIMER_FUNCTION, functioncontext: *const ::core::ffi::c_void) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisMInitializeTimer(timer : *const NDIS_MINIPORT_TIMER, miniportadapterhandle : *const ::core::ffi::c_void, timerfunction : PNDIS_TIMER_FUNCTION, functioncontext : *const ::core::ffi::c_void));
    NdisMInitializeTimer(timer, miniportadapterhandle, timerfunction, functioncontext)
}
#[inline]
pub unsafe fn NdisMMapIoSpace(virtualaddress: *mut *mut ::core::ffi::c_void, miniportadapterhandle: *const ::core::ffi::c_void, physicaladdress: i64, length: u32) -> i32 {
    ::windows_targets::link!("ndis.sys" "system" fn NdisMMapIoSpace(virtualaddress : *mut *mut ::core::ffi::c_void, miniportadapterhandle : *const ::core::ffi::c_void, physicaladdress : i64, length : u32) -> i32);
    NdisMMapIoSpace(virtualaddress, miniportadapterhandle, physicaladdress, length)
}
#[inline]
pub unsafe fn NdisMQueryAdapterInstanceName(padapterinstancename: *mut super::super::super::Win32::Foundation::UNICODE_STRING, miniporthandle: *const ::core::ffi::c_void) -> i32 {
    ::windows_targets::link!("ndis.sys" "system" fn NdisMQueryAdapterInstanceName(padapterinstancename : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, miniporthandle : *const ::core::ffi::c_void) -> i32);
    NdisMQueryAdapterInstanceName(padapterinstancename, miniporthandle)
}
#[inline]
pub unsafe fn NdisMReadDmaCounter(miniportdmahandle: *const ::core::ffi::c_void) -> u32 {
    ::windows_targets::link!("ndis.sys" "system" fn NdisMReadDmaCounter(miniportdmahandle : *const ::core::ffi::c_void) -> u32);
    NdisMReadDmaCounter(miniportdmahandle)
}
#[cfg(feature = "Wdk_System_SystemServices")]
#[inline]
pub unsafe fn NdisMRegisterDmaChannel<P0>(miniportdmahandle: *mut *mut ::core::ffi::c_void, miniportadapterhandle: *const ::core::ffi::c_void, dmachannel: u32, dma32bitaddresses: P0, dmadescription: *const NDIS_DMA_DESCRIPTION, maximumlength: u32) -> i32
where
    P0: ::windows_core::IntoParam<super::super::super::Win32::Foundation::BOOLEAN>,
{
    ::windows_targets::link!("ndis.sys" "system" fn NdisMRegisterDmaChannel(miniportdmahandle : *mut *mut ::core::ffi::c_void, miniportadapterhandle : *const ::core::ffi::c_void, dmachannel : u32, dma32bitaddresses : super::super::super::Win32::Foundation:: BOOLEAN, dmadescription : *const NDIS_DMA_DESCRIPTION, maximumlength : u32) -> i32);
    NdisMRegisterDmaChannel(miniportdmahandle, miniportadapterhandle, dmachannel, dma32bitaddresses.into_param().abi(), dmadescription, maximumlength)
}
#[inline]
pub unsafe fn NdisMRegisterIoPortRange(portoffset: *mut *mut ::core::ffi::c_void, miniportadapterhandle: *const ::core::ffi::c_void, initialport: u32, numberofports: u32) -> i32 {
    ::windows_targets::link!("ndis.sys" "system" fn NdisMRegisterIoPortRange(portoffset : *mut *mut ::core::ffi::c_void, miniportadapterhandle : *const ::core::ffi::c_void, initialport : u32, numberofports : u32) -> i32);
    NdisMRegisterIoPortRange(portoffset, miniportadapterhandle, initialport, numberofports)
}
#[inline]
pub unsafe fn NdisMRemoveMiniport(miniporthandle: *const ::core::ffi::c_void) -> i32 {
    ::windows_targets::link!("ndis.sys" "system" fn NdisMRemoveMiniport(miniporthandle : *const ::core::ffi::c_void) -> i32);
    NdisMRemoveMiniport(miniporthandle)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn NdisMSetPeriodicTimer(timer: *const NDIS_MINIPORT_TIMER, millisecondperiod: u32) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisMSetPeriodicTimer(timer : *const NDIS_MINIPORT_TIMER, millisecondperiod : u32));
    NdisMSetPeriodicTimer(timer, millisecondperiod)
}
#[inline]
pub unsafe fn NdisMSleep(microsecondstosleep: u32) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisMSleep(microsecondstosleep : u32));
    NdisMSleep(microsecondstosleep)
}
#[inline]
pub unsafe fn NdisMUnmapIoSpace(miniportadapterhandle: *const ::core::ffi::c_void, virtualaddress: *const ::core::ffi::c_void, length: u32) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisMUnmapIoSpace(miniportadapterhandle : *const ::core::ffi::c_void, virtualaddress : *const ::core::ffi::c_void, length : u32));
    NdisMUnmapIoSpace(miniportadapterhandle, virtualaddress, length)
}
#[inline]
pub unsafe fn NdisMWriteLogData(loghandle: *const ::core::ffi::c_void, logbuffer: *const ::core::ffi::c_void, logbuffersize: u32) -> i32 {
    ::windows_targets::link!("ndis.sys" "system" fn NdisMWriteLogData(loghandle : *const ::core::ffi::c_void, logbuffer : *const ::core::ffi::c_void, logbuffersize : u32) -> i32);
    NdisMWriteLogData(loghandle, logbuffer, logbuffersize)
}
#[inline]
pub unsafe fn NdisMapFile(status: *mut i32, mappedbuffer: *mut *mut ::core::ffi::c_void, filehandle: *const ::core::ffi::c_void) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisMapFile(status : *mut i32, mappedbuffer : *mut *mut ::core::ffi::c_void, filehandle : *const ::core::ffi::c_void));
    NdisMapFile(status, mappedbuffer, filehandle)
}
#[inline]
pub unsafe fn NdisOpenConfigurationKeyByIndex(status: *mut i32, configurationhandle: *const ::core::ffi::c_void, index: u32, keyname: *mut super::super::super::Win32::Foundation::UNICODE_STRING, keyhandle: *mut *mut ::core::ffi::c_void) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisOpenConfigurationKeyByIndex(status : *mut i32, configurationhandle : *const ::core::ffi::c_void, index : u32, keyname : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, keyhandle : *mut *mut ::core::ffi::c_void));
    NdisOpenConfigurationKeyByIndex(status, configurationhandle, index, keyname, keyhandle)
}
#[inline]
pub unsafe fn NdisOpenConfigurationKeyByName(status: *mut i32, configurationhandle: *const ::core::ffi::c_void, subkeyname: *const super::super::super::Win32::Foundation::UNICODE_STRING, subkeyhandle: *mut *mut ::core::ffi::c_void) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisOpenConfigurationKeyByName(status : *mut i32, configurationhandle : *const ::core::ffi::c_void, subkeyname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, subkeyhandle : *mut *mut ::core::ffi::c_void));
    NdisOpenConfigurationKeyByName(status, configurationhandle, subkeyname, subkeyhandle)
}
#[inline]
pub unsafe fn NdisOpenFile(status: *mut i32, filehandle: *mut *mut ::core::ffi::c_void, filelength: *mut u32, filename: *const super::super::super::Win32::Foundation::UNICODE_STRING, highestacceptableaddress: i64) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisOpenFile(status : *mut i32, filehandle : *mut *mut ::core::ffi::c_void, filelength : *mut u32, filename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, highestacceptableaddress : i64));
    NdisOpenFile(status, filehandle, filelength, filename, highestacceptableaddress)
}
#[inline]
pub unsafe fn NdisQueryAdapterInstanceName(padapterinstancename: *mut super::super::super::Win32::Foundation::UNICODE_STRING, ndisbindinghandle: *const ::core::ffi::c_void) -> i32 {
    ::windows_targets::link!("ndis.sys" "system" fn NdisQueryAdapterInstanceName(padapterinstancename : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, ndisbindinghandle : *const ::core::ffi::c_void) -> i32);
    NdisQueryAdapterInstanceName(padapterinstancename, ndisbindinghandle)
}
#[inline]
pub unsafe fn NdisQueryBindInstanceName(padapterinstancename: *mut super::super::super::Win32::Foundation::UNICODE_STRING, bindingcontext: *const ::core::ffi::c_void) -> i32 {
    ::windows_targets::link!("ndis.sys" "system" fn NdisQueryBindInstanceName(padapterinstancename : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, bindingcontext : *const ::core::ffi::c_void) -> i32);
    NdisQueryBindInstanceName(padapterinstancename, bindingcontext)
}
#[inline]
pub unsafe fn NdisReEnumerateProtocolBindings(ndisprotocolhandle: *const ::core::ffi::c_void) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisReEnumerateProtocolBindings(ndisprotocolhandle : *const ::core::ffi::c_void));
    NdisReEnumerateProtocolBindings(ndisprotocolhandle)
}
#[inline]
pub unsafe fn NdisReadConfiguration(status: *mut i32, parametervalue: *mut *mut NDIS_CONFIGURATION_PARAMETER, configurationhandle: *const ::core::ffi::c_void, keyword: *const super::super::super::Win32::Foundation::UNICODE_STRING, parametertype: NDIS_PARAMETER_TYPE) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisReadConfiguration(status : *mut i32, parametervalue : *mut *mut NDIS_CONFIGURATION_PARAMETER, configurationhandle : *const ::core::ffi::c_void, keyword : *const super::super::super::Win32::Foundation:: UNICODE_STRING, parametertype : NDIS_PARAMETER_TYPE));
    NdisReadConfiguration(status, parametervalue, configurationhandle, keyword, parametertype)
}
#[inline]
pub unsafe fn NdisReadNetworkAddress(status: *mut i32, networkaddress: *mut *mut ::core::ffi::c_void, networkaddresslength: *mut u32, configurationhandle: *const ::core::ffi::c_void) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisReadNetworkAddress(status : *mut i32, networkaddress : *mut *mut ::core::ffi::c_void, networkaddresslength : *mut u32, configurationhandle : *const ::core::ffi::c_void));
    NdisReadNetworkAddress(status, networkaddress, networkaddresslength, configurationhandle)
}
#[inline]
pub unsafe fn NdisRegisterTdiCallBack(registercallback: TDI_REGISTER_CALLBACK, pnphandler: TDI_PNP_HANDLER) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisRegisterTdiCallBack(registercallback : TDI_REGISTER_CALLBACK, pnphandler : TDI_PNP_HANDLER));
    NdisRegisterTdiCallBack(registercallback, pnphandler)
}
#[inline]
pub unsafe fn NdisReleaseReadWriteLock(lock: *mut NDIS_RW_LOCK, lockstate: *const LOCK_STATE) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisReleaseReadWriteLock(lock : *mut NDIS_RW_LOCK, lockstate : *const LOCK_STATE));
    NdisReleaseReadWriteLock(lock, lockstate)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn NdisResetEvent(event: *const NDIS_EVENT) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisResetEvent(event : *const NDIS_EVENT));
    NdisResetEvent(event)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn NdisSetEvent(event: *const NDIS_EVENT) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisSetEvent(event : *const NDIS_EVENT));
    NdisSetEvent(event)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn NdisSetPeriodicTimer(ndistimer: *const NDIS_TIMER, millisecondsperiod: u32) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisSetPeriodicTimer(ndistimer : *const NDIS_TIMER, millisecondsperiod : u32));
    NdisSetPeriodicTimer(ndistimer, millisecondsperiod)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn NdisSetTimer(timer: *const NDIS_TIMER, millisecondstodelay: u32) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisSetTimer(timer : *const NDIS_TIMER, millisecondstodelay : u32));
    NdisSetTimer(timer, millisecondstodelay)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn NdisSetTimerEx(ndistimer: *const NDIS_TIMER, millisecondstodelay: u32, functioncontext: *const ::core::ffi::c_void) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisSetTimerEx(ndistimer : *const NDIS_TIMER, millisecondstodelay : u32, functioncontext : *const ::core::ffi::c_void));
    NdisSetTimerEx(ndistimer, millisecondstodelay, functioncontext)
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn NdisSetupDmaTransfer<P0>(status: *mut i32, ndisdmahandle: *mut ::core::ffi::c_void, buffer: *mut super::super::Foundation::MDL, offset: u32, length: u32, writetodevice: P0)
where
    P0: ::windows_core::IntoParam<super::super::super::Win32::Foundation::BOOLEAN>,
{
    ::windows_targets::link!("ndis.sys" "system" fn NdisSetupDmaTransfer(status : *mut i32, ndisdmahandle : *mut ::core::ffi::c_void, buffer : *mut super::super::Foundation:: MDL, offset : u32, length : u32, writetodevice : super::super::super::Win32::Foundation:: BOOLEAN));
    NdisSetupDmaTransfer(status, ndisdmahandle, buffer, offset, length, writetodevice.into_param().abi())
}
#[inline]
pub unsafe fn NdisSystemProcessorCount() -> i8 {
    ::windows_targets::link!("ndis.sys" "system" fn NdisSystemProcessorCount() -> i8);
    NdisSystemProcessorCount()
}
#[inline]
pub unsafe fn NdisUnmapFile(filehandle: *const ::core::ffi::c_void) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisUnmapFile(filehandle : *const ::core::ffi::c_void));
    NdisUnmapFile(filehandle)
}
#[inline]
pub unsafe fn NdisUpdateSharedMemory(ndisadapterhandle: *mut ::core::ffi::c_void, length: u32, virtualaddress: *mut ::core::ffi::c_void, physicaladdress: i64) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisUpdateSharedMemory(ndisadapterhandle : *mut ::core::ffi::c_void, length : u32, virtualaddress : *mut ::core::ffi::c_void, physicaladdress : i64));
    NdisUpdateSharedMemory(ndisadapterhandle, length, virtualaddress, physicaladdress)
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn NdisWaitEvent(event: *const NDIS_EVENT, mstowait: u32) -> super::super::super::Win32::Foundation::BOOLEAN {
    ::windows_targets::link!("ndis.sys" "system" fn NdisWaitEvent(event : *const NDIS_EVENT, mstowait : u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
    NdisWaitEvent(event, mstowait)
}
#[inline]
pub unsafe fn NdisWriteConfiguration(status: *mut i32, configurationhandle: *const ::core::ffi::c_void, keyword: *const super::super::super::Win32::Foundation::UNICODE_STRING, parametervalue: *const NDIS_CONFIGURATION_PARAMETER) {
    ::windows_targets::link!("ndis.sys" "system" fn NdisWriteConfiguration(status : *mut i32, configurationhandle : *const ::core::ffi::c_void, keyword : *const super::super::super::Win32::Foundation:: UNICODE_STRING, parametervalue : *const NDIS_CONFIGURATION_PARAMETER));
    NdisWriteConfiguration(status, configurationhandle, keyword, parametervalue)
}
#[inline]
pub unsafe fn NdisWriteErrorLogEntry(ndisadapterhandle: *const ::core::ffi::c_void, errorcode: u32, numberoferrorvalues: u32) {
    ::windows_targets::link!("ndis.sys" "cdecl" fn NdisWriteErrorLogEntry(ndisadapterhandle : *const ::core::ffi::c_void, errorcode : u32, numberoferrorvalues : u32));
    NdisWriteErrorLogEntry(ndisadapterhandle, errorcode, numberoferrorvalues)
}
#[inline]
pub unsafe fn NdisWriteEventLogEntry(loghandle: *const ::core::ffi::c_void, eventcode: i32, uniqueeventvalue: u32, numstrings: u16, stringslist: ::core::option::Option<*const ::core::ffi::c_void>, datasize: u32, data: ::core::option::Option<*const ::core::ffi::c_void>) -> i32 {
    ::windows_targets::link!("ndis.sys" "system" fn NdisWriteEventLogEntry(loghandle : *const ::core::ffi::c_void, eventcode : i32, uniqueeventvalue : u32, numstrings : u16, stringslist : *const ::core::ffi::c_void, datasize : u32, data : *const ::core::ffi::c_void) -> i32);
    NdisWriteEventLogEntry(loghandle, eventcode, uniqueeventvalue, numstrings, ::core::mem::transmute(stringslist.unwrap_or(::std::ptr::null())), datasize, ::core::mem::transmute(data.unwrap_or(::std::ptr::null())))
}
pub const AUTHENTICATE: OFFLOAD_OPERATION_E = OFFLOAD_OPERATION_E(1i32);
pub const BINARY_COMPATIBLE: u32 = 0u32;
pub const BROADCAST_VC: u32 = 8u32;
pub const CALL_PARAMETERS_CHANGED: u32 = 2u32;
pub const CLOCK_NETWORK_DERIVED: u32 = 2u32;
pub const CLOCK_PRECISION: u32 = 4u32;
pub const CO_ADDRESS_FAMILY_PROXY: u32 = 2147483648u32;
pub const CO_SEND_FLAG_SET_DISCARD_ELIBILITY: u32 = 1u32;
pub const CRYPTO_GENERIC_ERROR: u32 = 1u32;
pub const CRYPTO_INVALID_PACKET_SYNTAX: u32 = 6u32;
pub const CRYPTO_INVALID_PROTOCOL: u32 = 7u32;
pub const CRYPTO_SUCCESS: u32 = 0u32;
pub const CRYPTO_TRANSPORT_AH_AUTH_FAILED: u32 = 2u32;
pub const CRYPTO_TRANSPORT_ESP_AUTH_FAILED: u32 = 3u32;
pub const CRYPTO_TUNNEL_AH_AUTH_FAILED: u32 = 4u32;
pub const CRYPTO_TUNNEL_ESP_AUTH_FAILED: u32 = 5u32;
pub const CachedNetBufferList: NDIS_PER_PACKET_INFO = NDIS_PER_PACKET_INFO(10i32);
pub const ClassificationHandlePacketInfo: NDIS_PER_PACKET_INFO = NDIS_PER_PACKET_INFO(3i32);
pub const DD_NDIS_DEVICE_NAME: ::windows_core::PCWSTR = ::windows_core::w!("\\Device\\NDIS");
pub const DOT11_RSN_KCK_LENGTH: u32 = 16u32;
pub const DOT11_RSN_KEK_LENGTH: u32 = 16u32;
pub const DOT11_RSN_MAX_CIPHER_KEY_LENGTH: u32 = 32u32;
pub const EAPOL_REQUEST_ID_WOL_FLAG_MUST_ENCRYPT: u32 = 1u32;
pub const ENCRYPT: OFFLOAD_OPERATION_E = OFFLOAD_OPERATION_E(2i32);
pub const ERRED_PACKET_INDICATION: u32 = 1u32;
pub const ETHERNET_LENGTH_OF_ADDRESS: u32 = 6u32;
pub const GUID_NDIS_NDK_CAPABILITIES: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7969ba4d_dd80_4bc7_b3e6_68043997e519);
pub const GUID_NDIS_NDK_STATE: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x530c69c9_2f51_49de_a1af_088d54ffa474);
pub const INDICATE_END_OF_TX: u32 = 32u32;
pub const INDICATE_ERRED_PACKETS: u32 = 16u32;
pub const IOCTL_NDIS_RESERVED5: u32 = 1507380u32;
pub const IOCTL_NDIS_RESERVED6: u32 = 1540152u32;
pub const IPSEC_OFFLOAD_V2_AND_TCP_CHECKSUM_COEXISTENCE: u32 = 2u32;
pub const IPSEC_OFFLOAD_V2_AND_UDP_CHECKSUM_COEXISTENCE: u32 = 4u32;
pub const IPSEC_OFFLOAD_V2_AUTHENTICATION_AES_GCM_128: u32 = 8u32;
pub const IPSEC_OFFLOAD_V2_AUTHENTICATION_AES_GCM_192: u32 = 16u32;
pub const IPSEC_OFFLOAD_V2_AUTHENTICATION_AES_GCM_256: u32 = 32u32;
pub const IPSEC_OFFLOAD_V2_AUTHENTICATION_MD5: u32 = 1u32;
pub const IPSEC_OFFLOAD_V2_AUTHENTICATION_SHA_1: u32 = 2u32;
pub const IPSEC_OFFLOAD_V2_AUTHENTICATION_SHA_256: u32 = 4u32;
pub const IPSEC_OFFLOAD_V2_ENCRYPTION_3_DES_CBC: u32 = 4u32;
pub const IPSEC_OFFLOAD_V2_ENCRYPTION_AES_CBC_128: u32 = 64u32;
pub const IPSEC_OFFLOAD_V2_ENCRYPTION_AES_CBC_192: u32 = 128u32;
pub const IPSEC_OFFLOAD_V2_ENCRYPTION_AES_CBC_256: u32 = 256u32;
pub const IPSEC_OFFLOAD_V2_ENCRYPTION_AES_GCM_128: u32 = 8u32;
pub const IPSEC_OFFLOAD_V2_ENCRYPTION_AES_GCM_192: u32 = 16u32;
pub const IPSEC_OFFLOAD_V2_ENCRYPTION_AES_GCM_256: u32 = 32u32;
pub const IPSEC_OFFLOAD_V2_ENCRYPTION_DES_CBC: u32 = 2u32;
pub const IPSEC_OFFLOAD_V2_ENCRYPTION_NONE: u32 = 1u32;
pub const IPSEC_OFFLOAD_V2_ESN_SA: u32 = 1u32;
pub const IPSEC_OFFLOAD_V2_INBOUND: u32 = 1u32;
pub const IPSEC_OFFLOAD_V2_IPv6: u32 = 16u32;
pub const IPSEC_OFFLOAD_V2_MAX_EXTENSION_HEADERS: u32 = 2u32;
pub const IPSEC_OFFLOAD_V2_TRANSPORT_OVER_UDP_ESP_ENCAPSULATION_TUNNEL: u32 = 4u32;
pub const IPSEC_OFFLOAD_V2_UDP_ESP_ENCAPSULATION_NONE: u32 = 0u32;
pub const IPSEC_OFFLOAD_V2_UDP_ESP_ENCAPSULATION_TRANSPORT: u32 = 1u32;
pub const IPSEC_OFFLOAD_V2_UDP_ESP_ENCAPSULATION_TRANSPORT_OVER_TUNNEL: u32 = 8u32;
pub const IPSEC_OFFLOAD_V2_UDP_ESP_ENCAPSULATION_TUNNEL: u32 = 2u32;
pub const IPSEC_TPTOVERTUN_UDPESP_ENCAPTYPE_IKE: u32 = 4u32;
pub const IPSEC_TPTOVERTUN_UDPESP_ENCAPTYPE_OTHER: u32 = 64u32;
pub const IPSEC_TPT_UDPESP_ENCAPTYPE_IKE: u32 = 1u32;
pub const IPSEC_TPT_UDPESP_ENCAPTYPE_OTHER: u32 = 16u32;
pub const IPSEC_TPT_UDPESP_OVER_PURE_TUN_ENCAPTYPE_IKE: u32 = 8u32;
pub const IPSEC_TPT_UDPESP_OVER_PURE_TUN_ENCAPTYPE_OTHER: u32 = 128u32;
pub const IPSEC_TUN_UDPESP_ENCAPTYPE_IKE: u32 = 2u32;
pub const IPSEC_TUN_UDPESP_ENCAPTYPE_OTHER: u32 = 32u32;
pub const Ieee8021QInfo: NDIS_PER_PACKET_INFO = NDIS_PER_PACKET_INFO(6i32);
pub const IpSecPacketInfo: NDIS_PER_PACKET_INFO = NDIS_PER_PACKET_INFO(1i32);
pub const MAXIMUM_IP_OPER_STATUS_ADDRESS_FAMILIES_SUPPORTED: u32 = 32u32;
pub const MAX_HASHES: u32 = 4u32;
pub const MULTIPOINT_VC: u32 = 16u32;
pub const MaxPerPacketInfo: NDIS_PER_PACKET_INFO = NDIS_PER_PACKET_INFO(12i32);
pub const NBL_FLAGS_MINIPORT_RESERVED: u32 = 61440u32;
pub const NBL_FLAGS_NDIS_RESERVED: u32 = 4092u32;
pub const NBL_FLAGS_PROTOCOL_RESERVED: u32 = 4293918723u32;
pub const NBL_FLAGS_SCRATCH: u32 = 983040u32;
pub const NBL_PROT_RSVD_FLAGS: u32 = 4293918723u32;
pub const NDIS630_MINIPORT: u32 = 1u32;
pub const NDIS685_MINIPORT: u32 = 1u32;
pub const NDIS_802_11_AI_REQFI_CAPABILITIES: u32 = 1u32;
pub const NDIS_802_11_AI_REQFI_CURRENTAPADDRESS: u32 = 4u32;
pub const NDIS_802_11_AI_REQFI_LISTENINTERVAL: u32 = 2u32;
pub const NDIS_802_11_AI_RESFI_ASSOCIATIONID: u32 = 4u32;
pub const NDIS_802_11_AI_RESFI_CAPABILITIES: u32 = 1u32;
pub const NDIS_802_11_AI_RESFI_STATUSCODE: u32 = 2u32;
pub const NDIS_802_11_AUTH_REQUEST_AUTH_FIELDS: u32 = 15u32;
pub const NDIS_802_11_AUTH_REQUEST_GROUP_ERROR: u32 = 14u32;
pub const NDIS_802_11_AUTH_REQUEST_KEYUPDATE: u32 = 2u32;
pub const NDIS_802_11_AUTH_REQUEST_PAIRWISE_ERROR: u32 = 6u32;
pub const NDIS_802_11_AUTH_REQUEST_REAUTH: u32 = 1u32;
pub const NDIS_802_11_LENGTH_RATES: u32 = 8u32;
pub const NDIS_802_11_LENGTH_RATES_EX: u32 = 16u32;
pub const NDIS_802_11_LENGTH_SSID: u32 = 32u32;
pub const NDIS_802_11_PMKID_CANDIDATE_PREAUTH_ENABLED: u32 = 1u32;
pub const NDIS_802_3_MAC_OPTION_PRIORITY: u32 = 1u32;
pub const NDIS_ANY_NUMBER_OF_NBLS: u32 = 4294967295u32;
pub const NDIS_ATTRIBUTE_BUS_MASTER: u32 = 8u32;
pub const NDIS_ATTRIBUTE_DESERIALIZE: u32 = 32u32;
pub const NDIS_ATTRIBUTE_DO_NOT_BIND_TO_ALL_CO: u32 = 1024u32;
pub const NDIS_ATTRIBUTE_IGNORE_PACKET_TIMEOUT: u32 = 1u32;
pub const NDIS_ATTRIBUTE_IGNORE_REQUEST_TIMEOUT: u32 = 2u32;
pub const NDIS_ATTRIBUTE_IGNORE_TOKEN_RING_ERRORS: u32 = 4u32;
pub const NDIS_ATTRIBUTE_INTERMEDIATE_DRIVER: u32 = 16u32;
pub const NDIS_ATTRIBUTE_MINIPORT_PADS_SHORT_PACKETS: u32 = 2048u32;
pub const NDIS_ATTRIBUTE_NOT_CO_NDIS: u32 = 256u32;
pub const NDIS_ATTRIBUTE_NO_HALT_ON_SUSPEND: u32 = 64u32;
pub const NDIS_ATTRIBUTE_SURPRISE_REMOVE_OK: u32 = 128u32;
pub const NDIS_ATTRIBUTE_USES_SAFE_BUFFER_APIS: u32 = 512u32;
pub const NDIS_BIND_FAILED_NOTIFICATION_REVISION_1: u32 = 1u32;
pub const NDIS_BIND_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_BIND_PARAMETERS_REVISION_2: u32 = 2u32;
pub const NDIS_BIND_PARAMETERS_REVISION_3: u32 = 3u32;
pub const NDIS_BIND_PARAMETERS_REVISION_4: u32 = 4u32;
pub const NDIS_CLONE_FLAGS_RESERVED: u32 = 1u32;
pub const NDIS_CLONE_FLAGS_USE_ORIGINAL_MDLS: u32 = 2u32;
pub const NDIS_CONFIGURATION_OBJECT_REVISION_1: u32 = 1u32;
pub const NDIS_CONFIG_FLAG_FILTER_INSTANCE_CONFIGURATION: u32 = 1u32;
pub const NDIS_CO_CALL_MANAGER_OPTIONAL_HANDLERS_REVISION_1: u32 = 1u32;
pub const NDIS_CO_CLIENT_OPTIONAL_HANDLERS_REVISION_1: u32 = 1u32;
pub const NDIS_CO_MAC_OPTION_DYNAMIC_LINK_SPEED: u32 = 1u32;
pub const NDIS_DEFAULT_RECEIVE_FILTER_ID: u32 = 0u32;
pub const NDIS_DEFAULT_RECEIVE_QUEUE_GROUP_ID: u32 = 0u32;
pub const NDIS_DEFAULT_RECEIVE_QUEUE_ID: u32 = 0u32;
pub const NDIS_DEFAULT_SWITCH_ID: u32 = 0u32;
pub const NDIS_DEFAULT_VPORT_ID: u32 = 0u32;
pub const NDIS_DEVICE_OBJECT_ATTRIBUTES_REVISION_1: u32 = 1u32;
pub const NDIS_DEVICE_TYPE_ENDPOINT: u32 = 1u32;
pub const NDIS_DEVICE_WAKE_ON_MAGIC_PACKET_ENABLE: u32 = 4u32;
pub const NDIS_DEVICE_WAKE_ON_PATTERN_MATCH_ENABLE: u32 = 2u32;
pub const NDIS_DEVICE_WAKE_UP_ENABLE: u32 = 1u32;
pub const NDIS_DRIVER_FLAGS_RESERVED: u32 = 8u32;
pub const NDIS_ENCAPSULATED_PACKET_TASK_OFFLOAD_INNER_IPV4: u32 = 1u32;
pub const NDIS_ENCAPSULATED_PACKET_TASK_OFFLOAD_INNER_IPV6: u32 = 4u32;
pub const NDIS_ENCAPSULATED_PACKET_TASK_OFFLOAD_NOT_SUPPORTED: u32 = 0u32;
pub const NDIS_ENCAPSULATED_PACKET_TASK_OFFLOAD_OUTER_IPV4: u32 = 2u32;
pub const NDIS_ENCAPSULATED_PACKET_TASK_OFFLOAD_OUTER_IPV6: u32 = 8u32;
pub const NDIS_ENCAPSULATION_IEEE_802_3: u32 = 2u32;
pub const NDIS_ENCAPSULATION_IEEE_802_3_P_AND_Q: u32 = 4u32;
pub const NDIS_ENCAPSULATION_IEEE_802_3_P_AND_Q_IN_OOB: u32 = 8u32;
pub const NDIS_ENCAPSULATION_IEEE_LLC_SNAP_ROUTED: u32 = 16u32;
pub const NDIS_ENCAPSULATION_NOT_SUPPORTED: u32 = 0u32;
pub const NDIS_ENCAPSULATION_NULL: u32 = 1u32;
pub const NDIS_ENCAPSULATION_TYPE_GRE_MAC: u32 = 1u32;
pub const NDIS_ENCAPSULATION_TYPE_VXLAN: u32 = 2u32;
pub const NDIS_ENUM_FILTERS_REVISION_1: u32 = 1u32;
pub const NDIS_ETH_TYPE_802_1Q: u32 = 33024u32;
pub const NDIS_ETH_TYPE_802_1X: u32 = 34958u32;
pub const NDIS_ETH_TYPE_ARP: u32 = 2054u32;
pub const NDIS_ETH_TYPE_IPV4: u32 = 2048u32;
pub const NDIS_ETH_TYPE_IPV6: u32 = 34525u32;
pub const NDIS_ETH_TYPE_SLOW_PROTOCOL: u32 = 34825u32;
pub const NDIS_FILTER_ATTACH_FLAGS_IGNORE_MANDATORY: u32 = 1u32;
pub const NDIS_FILTER_ATTACH_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_FILTER_ATTACH_PARAMETERS_REVISION_2: u32 = 2u32;
pub const NDIS_FILTER_ATTACH_PARAMETERS_REVISION_3: u32 = 3u32;
pub const NDIS_FILTER_ATTACH_PARAMETERS_REVISION_4: u32 = 4u32;
pub const NDIS_FILTER_ATTRIBUTES_REVISION_1: u32 = 1u32;
pub const NDIS_FILTER_CHARACTERISTICS_REVISION_1: u32 = 1u32;
pub const NDIS_FILTER_CHARACTERISTICS_REVISION_2: u32 = 2u32;
pub const NDIS_FILTER_CHARACTERISTICS_REVISION_3: u32 = 3u32;
pub const NDIS_FILTER_DRIVER_MANDATORY: u32 = 1u32;
pub const NDIS_FILTER_DRIVER_SUPPORTS_CURRENT_MAC_ADDRESS_CHANGE: u32 = 2u32;
pub const NDIS_FILTER_DRIVER_SUPPORTS_L2_MTU_SIZE_CHANGE: u32 = 4u32;
pub const NDIS_FILTER_INTERFACE_IM_FILTER: u32 = 1u32;
pub const NDIS_FILTER_INTERFACE_LW_FILTER: u32 = 2u32;
pub const NDIS_FILTER_INTERFACE_RECEIVE_BYPASS: u32 = 8u32;
pub const NDIS_FILTER_INTERFACE_REVISION_1: u32 = 1u32;
pub const NDIS_FILTER_INTERFACE_REVISION_2: u32 = 2u32;
pub const NDIS_FILTER_INTERFACE_SEND_BYPASS: u32 = 4u32;
pub const NDIS_FILTER_MAJOR_VERSION: u32 = 6u32;
pub const NDIS_FILTER_MINIMUM_MAJOR_VERSION: u32 = 6u32;
pub const NDIS_FILTER_MINIMUM_MINOR_VERSION: u32 = 0u32;
pub const NDIS_FILTER_MINOR_VERSION: u32 = 87u32;
pub const NDIS_FILTER_PARTIAL_CHARACTERISTICS_REVISION_1: u32 = 1u32;
pub const NDIS_FILTER_PAUSE_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_FILTER_RESTART_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_FLAGS_DONT_LOOPBACK: u32 = 128u32;
pub const NDIS_FLAGS_DOUBLE_BUFFERED: u32 = 2048u32;
pub const NDIS_FLAGS_IS_LOOPBACK_PACKET: u32 = 256u32;
pub const NDIS_FLAGS_LOOPBACK_ONLY: u32 = 512u32;
pub const NDIS_FLAGS_MULTICAST_PACKET: u32 = 16u32;
pub const NDIS_FLAGS_PADDED: u32 = 65536u32;
pub const NDIS_FLAGS_PROTOCOL_ID_MASK: u32 = 15u32;
pub const NDIS_FLAGS_RESERVED2: u32 = 32u32;
pub const NDIS_FLAGS_RESERVED3: u32 = 64u32;
pub const NDIS_FLAGS_RESERVED4: u32 = 1024u32;
pub const NDIS_FLAGS_SENT_AT_DPC: u32 = 4096u32;
pub const NDIS_FLAGS_USES_ORIGINAL_PACKET: u32 = 16384u32;
pub const NDIS_FLAGS_USES_SG_BUFFER_LIST: u32 = 8192u32;
pub const NDIS_FLAGS_XLATE_AT_TOP: u32 = 131072u32;
pub const NDIS_GFP_ENCAPSULATION_TYPE_IP_IN_GRE: u32 = 4u32;
pub const NDIS_GFP_ENCAPSULATION_TYPE_IP_IN_IP: u32 = 2u32;
pub const NDIS_GFP_ENCAPSULATION_TYPE_NOT_ENCAPSULATED: u32 = 1u32;
pub const NDIS_GFP_ENCAPSULATION_TYPE_NVGRE: u32 = 8u32;
pub const NDIS_GFP_ENCAPSULATION_TYPE_VXLAN: u32 = 16u32;
pub const NDIS_GFP_EXACT_MATCH_PROFILE_RDMA_FLOW: u32 = 1u32;
pub const NDIS_GFP_EXACT_MATCH_PROFILE_REVISION_1: u32 = 1u32;
pub const NDIS_GFP_HEADER_GROUP_EXACT_MATCH_IS_TTL_ONE: u32 = 1u32;
pub const NDIS_GFP_HEADER_GROUP_EXACT_MATCH_PROFILE_IS_TTL_ONE: u32 = 1u32;
pub const NDIS_GFP_HEADER_GROUP_EXACT_MATCH_PROFILE_REVISION_1: u32 = 1u32;
pub const NDIS_GFP_HEADER_GROUP_EXACT_MATCH_REVISION_1: u32 = 1u32;
pub const NDIS_GFP_HEADER_GROUP_WILDCARD_MATCH_IS_TTL_ONE: u32 = 1u32;
pub const NDIS_GFP_HEADER_GROUP_WILDCARD_MATCH_PROFILE_IS_TTL_ONE: u32 = 1u32;
pub const NDIS_GFP_HEADER_GROUP_WILDCARD_MATCH_PROFILE_REVISION_1: u32 = 1u32;
pub const NDIS_GFP_HEADER_GROUP_WILDCARD_MATCH_REVISION_1: u32 = 1u32;
pub const NDIS_GFP_HEADER_PRESENT_ESP: u32 = 2048u32;
pub const NDIS_GFP_HEADER_PRESENT_ETHERNET: u32 = 1u32;
pub const NDIS_GFP_HEADER_PRESENT_ICMP: u32 = 32u32;
pub const NDIS_GFP_HEADER_PRESENT_IPV4: u32 = 2u32;
pub const NDIS_GFP_HEADER_PRESENT_IPV6: u32 = 4u32;
pub const NDIS_GFP_HEADER_PRESENT_IP_IN_GRE_ENCAP: u32 = 256u32;
pub const NDIS_GFP_HEADER_PRESENT_IP_IN_IP_ENCAP: u32 = 128u32;
pub const NDIS_GFP_HEADER_PRESENT_NO_ENCAP: u32 = 64u32;
pub const NDIS_GFP_HEADER_PRESENT_NVGRE_ENCAP: u32 = 512u32;
pub const NDIS_GFP_HEADER_PRESENT_TCP: u32 = 8u32;
pub const NDIS_GFP_HEADER_PRESENT_UDP: u32 = 16u32;
pub const NDIS_GFP_HEADER_PRESENT_VXLAN_ENCAP: u32 = 1024u32;
pub const NDIS_GFP_UNDEFINED_PROFILE_ID: u32 = 0u32;
pub const NDIS_GFP_WILDCARD_MATCH_PROFILE_REVISION_1: u32 = 1u32;
pub const NDIS_GFT_COUNTER_INFO_ARRAY_REVISION_1: u32 = 1u32;
pub const NDIS_GFT_COUNTER_INFO_REVISION_1: u32 = 1u32;
pub const NDIS_GFT_COUNTER_PARAMETERS_CLIENT_SPECIFIED_ADDRESS: u32 = 1u32;
pub const NDIS_GFT_COUNTER_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_GFT_COUNTER_VALUE_ARRAY_GET_VALUES: u32 = 2u32;
pub const NDIS_GFT_COUNTER_VALUE_ARRAY_REVISION_1: u32 = 1u32;
pub const NDIS_GFT_COUNTER_VALUE_ARRAY_UPDATE_MEMORY_MAPPED_COUNTERS: u32 = 1u32;
pub const NDIS_GFT_CUSTOM_ACTION_LAST_ACTION: u32 = 1u32;
pub const NDIS_GFT_CUSTOM_ACTION_PROFILE_REVISION_1: u32 = 1u32;
pub const NDIS_GFT_CUSTOM_ACTION_REVISION_1: u32 = 1u32;
pub const NDIS_GFT_DELETE_PROFILE_ALL_PROFILES: u32 = 1u32;
pub const NDIS_GFT_DELETE_PROFILE_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_GFT_DELETE_TABLE_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_GFT_EMFE_ADD_IN_ACTIVATED_STATE: u32 = 1u32;
pub const NDIS_GFT_EMFE_ALL_VPORT_FLOW_ENTRIES: u32 = 33554432u32;
pub const NDIS_GFT_EMFE_COPY_AFTER_TCP_FIN_FLAG_SET: u32 = 2097152u32;
pub const NDIS_GFT_EMFE_COPY_AFTER_TCP_RST_FLAG_SET: u32 = 4194304u32;
pub const NDIS_GFT_EMFE_COPY_ALL_PACKETS: u32 = 65536u32;
pub const NDIS_GFT_EMFE_COPY_CONDITION_CHANGED: u32 = 16777216u32;
pub const NDIS_GFT_EMFE_COPY_FIRST_PACKET: u32 = 131072u32;
pub const NDIS_GFT_EMFE_COPY_WHEN_TCP_FLAG_SET: u32 = 262144u32;
pub const NDIS_GFT_EMFE_COUNTER_ALLOCATE: u32 = 1u32;
pub const NDIS_GFT_EMFE_COUNTER_CLIENT_SPECIFIED_ADDRESS: u32 = 4u32;
pub const NDIS_GFT_EMFE_COUNTER_MEMORY_MAPPED: u32 = 2u32;
pub const NDIS_GFT_EMFE_COUNTER_TRACK_TCP_FLOW: u32 = 8u32;
pub const NDIS_GFT_EMFE_CUSTOM_ACTION_PRESENT: u32 = 524288u32;
pub const NDIS_GFT_EMFE_MATCH_AND_ACTION_MUST_BE_SUPPORTED: u32 = 2u32;
pub const NDIS_GFT_EMFE_META_ACTION_BEFORE_HEADER_TRANSPOSITION: u32 = 1048576u32;
pub const NDIS_GFT_EMFE_RDMA_FLOW: u32 = 4u32;
pub const NDIS_GFT_EMFE_REDIRECT_TO_EGRESS_QUEUE_OF_VPORT: u32 = 8192u32;
pub const NDIS_GFT_EMFE_REDIRECT_TO_EGRESS_QUEUE_OF_VPORT_IF_TTL_IS_ONE: u32 = 32768u32;
pub const NDIS_GFT_EMFE_REDIRECT_TO_INGRESS_QUEUE_OF_VPORT: u32 = 4096u32;
pub const NDIS_GFT_EMFE_REDIRECT_TO_INGRESS_QUEUE_OF_VPORT_IF_TTL_IS_ONE: u32 = 16384u32;
pub const NDIS_GFT_EXACT_MATCH_FLOW_ENTRY_REVISION_1: u32 = 1u32;
pub const NDIS_GFT_FLOW_ENTRY_ARRAY_REVISION_1: u32 = 1u32;
pub const NDIS_GFT_FLOW_ENTRY_ID_ALL_NIC_SWITCH_FLOW_ENTRIES: u32 = 1u32;
pub const NDIS_GFT_FLOW_ENTRY_ID_ALL_TABLE_FLOW_ENTRIES: u32 = 2u32;
pub const NDIS_GFT_FLOW_ENTRY_ID_ALL_VPORT_FLOW_ENTRIES: u32 = 4u32;
pub const NDIS_GFT_FLOW_ENTRY_ID_ARRAY_COUNTER_VALUES: u32 = 65536u32;
pub const NDIS_GFT_FLOW_ENTRY_ID_ARRAY_DEFINED: u32 = 16u32;
pub const NDIS_GFT_FLOW_ENTRY_ID_ARRAY_REVISION_1: u32 = 1u32;
pub const NDIS_GFT_FLOW_ENTRY_ID_RANGE_DEFINED: u32 = 8u32;
pub const NDIS_GFT_FLOW_ENTRY_INFO_ALL_FLOW_ENTRIES: u32 = 1u32;
pub const NDIS_GFT_FLOW_ENTRY_INFO_ARRAY_REVISION_1: u32 = 1u32;
pub const NDIS_GFT_FREE_COUNTER_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_GFT_HEADER_GROUP_TRANSPOSITION_DECREMENT_TTL_IF_NOT_ONE: u32 = 1u32;
pub const NDIS_GFT_HEADER_GROUP_TRANSPOSITION_PROFILE_DECREMENT_TTL_IF_NOT_ONE: u32 = 1u32;
pub const NDIS_GFT_HEADER_GROUP_TRANSPOSITION_PROFILE_REVISION_1: u32 = 1u32;
pub const NDIS_GFT_HEADER_GROUP_TRANSPOSITION_REVISION_1: u32 = 1u32;
pub const NDIS_GFT_HEADER_TRANSPOSITION_PROFILE_REVISION_1: u32 = 1u32;
pub const NDIS_GFT_HTP_COPY_ALL_PACKETS: u32 = 16u32;
pub const NDIS_GFT_HTP_COPY_FIRST_PACKET: u32 = 32u32;
pub const NDIS_GFT_HTP_COPY_WHEN_TCP_FLAG_SET: u32 = 64u32;
pub const NDIS_GFT_HTP_CUSTOM_ACTION_PRESENT: u32 = 128u32;
pub const NDIS_GFT_HTP_META_ACTION_BEFORE_HEADER_TRANSPOSITION: u32 = 256u32;
pub const NDIS_GFT_HTP_REDIRECT_TO_EGRESS_QUEUE_OF_VPORT: u32 = 2u32;
pub const NDIS_GFT_HTP_REDIRECT_TO_EGRESS_QUEUE_OF_VPORT_IF_TTL_IS_ONE: u32 = 8u32;
pub const NDIS_GFT_HTP_REDIRECT_TO_INGRESS_QUEUE_OF_VPORT: u32 = 1u32;
pub const NDIS_GFT_HTP_REDIRECT_TO_INGRESS_QUEUE_OF_VPORT_IF_TTL_IS_ONE: u32 = 4u32;
pub const NDIS_GFT_MAX_COUNTER_OBJECTS_PER_FLOW_ENTRY: u32 = 8u32;
pub const NDIS_GFT_OFFLOAD_CAPABILITIES_REVISION_1: u32 = 1u32;
pub const NDIS_GFT_OFFLOAD_CAPS_8021P_PRIORITY_MASK: u32 = 131072u32;
pub const NDIS_GFT_OFFLOAD_CAPS_ADD_FLOW_ENTRY_DEACTIVATED_PREFERRED: u32 = 1u32;
pub const NDIS_GFT_OFFLOAD_CAPS_ALLOW: u32 = 262144u32;
pub const NDIS_GFT_OFFLOAD_CAPS_CLIENT_SPECIFIED_MEMORY_MAPPED_COUNTERS: u32 = 16u32;
pub const NDIS_GFT_OFFLOAD_CAPS_COMBINED_COUNTER_AND_STATE: u32 = 256u32;
pub const NDIS_GFT_OFFLOAD_CAPS_COPY_ALL: u32 = 256u32;
pub const NDIS_GFT_OFFLOAD_CAPS_COPY_FIRST: u32 = 512u32;
pub const NDIS_GFT_OFFLOAD_CAPS_COPY_WHEN_TCP_FLAG_SET: u32 = 1024u32;
pub const NDIS_GFT_OFFLOAD_CAPS_DESIGNATED_EXCEPTION_VPORT: u32 = 32768u32;
pub const NDIS_GFT_OFFLOAD_CAPS_DROP: u32 = 524288u32;
pub const NDIS_GFT_OFFLOAD_CAPS_DSCP_MASK: u32 = 65536u32;
pub const NDIS_GFT_OFFLOAD_CAPS_EGRESS_AGGREGATE_COUNTERS: u32 = 64u32;
pub const NDIS_GFT_OFFLOAD_CAPS_EGRESS_EXACT_MATCH: u32 = 8u32;
pub const NDIS_GFT_OFFLOAD_CAPS_EGRESS_WILDCARD_MATCH: u32 = 2u32;
pub const NDIS_GFT_OFFLOAD_CAPS_EXT_VPORT_EGRESS_EXACT_MATCH: u32 = 128u32;
pub const NDIS_GFT_OFFLOAD_CAPS_EXT_VPORT_EGRESS_WILDCARD_MATCH: u32 = 32u32;
pub const NDIS_GFT_OFFLOAD_CAPS_EXT_VPORT_INGRESS_EXACT_MATCH: u32 = 64u32;
pub const NDIS_GFT_OFFLOAD_CAPS_EXT_VPORT_INGRESS_WILDCARD_MATCH: u32 = 16u32;
pub const NDIS_GFT_OFFLOAD_CAPS_IGNORE_ACTION_SUPPORTED: u32 = 8u32;
pub const NDIS_GFT_OFFLOAD_CAPS_INGRESS_AGGREGATE_COUNTERS: u32 = 32u32;
pub const NDIS_GFT_OFFLOAD_CAPS_INGRESS_EXACT_MATCH: u32 = 4u32;
pub const NDIS_GFT_OFFLOAD_CAPS_INGRESS_WILDCARD_MATCH: u32 = 1u32;
pub const NDIS_GFT_OFFLOAD_CAPS_MEMORY_MAPPED_COUNTERS: u32 = 1u32;
pub const NDIS_GFT_OFFLOAD_CAPS_MEMORY_MAPPED_PAKCET_AND_BYTE_COUNTERS: u32 = 2u32;
pub const NDIS_GFT_OFFLOAD_CAPS_META_ACTION_AFTER_HEADER_TRANSPOSITION: u32 = 8192u32;
pub const NDIS_GFT_OFFLOAD_CAPS_META_ACTION_BEFORE_HEADER_TRANSPOSITION: u32 = 4096u32;
pub const NDIS_GFT_OFFLOAD_CAPS_MODIFY: u32 = 4u32;
pub const NDIS_GFT_OFFLOAD_CAPS_PER_FLOW_ENTRY_COUNTERS: u32 = 4u32;
pub const NDIS_GFT_OFFLOAD_CAPS_PER_PACKET_COUNTER_UPDATE: u32 = 8u32;
pub const NDIS_GFT_OFFLOAD_CAPS_PER_VPORT_EXCEPTION_VPORT: u32 = 16384u32;
pub const NDIS_GFT_OFFLOAD_CAPS_POP: u32 = 1u32;
pub const NDIS_GFT_OFFLOAD_CAPS_PUSH: u32 = 2u32;
pub const NDIS_GFT_OFFLOAD_CAPS_RATE_LIMITING_QUEUE_SUPPORTED: u32 = 2u32;
pub const NDIS_GFT_OFFLOAD_CAPS_REDIRECT_TO_EGRESS_QUEUE_OF_VPORT: u32 = 32u32;
pub const NDIS_GFT_OFFLOAD_CAPS_REDIRECT_TO_EGRESS_QUEUE_OF_VPORT_IF_TTL_IS_ONE: u32 = 128u32;
pub const NDIS_GFT_OFFLOAD_CAPS_REDIRECT_TO_INGRESS_QUEUE_OF_VPORT: u32 = 16u32;
pub const NDIS_GFT_OFFLOAD_CAPS_REDIRECT_TO_INGRESS_QUEUE_OF_VPORT_IF_TTL_IS_ONE: u32 = 64u32;
pub const NDIS_GFT_OFFLOAD_CAPS_SAMPLE: u32 = 2048u32;
pub const NDIS_GFT_OFFLOAD_CAPS_TRACK_TCP_FLOW_STATE: u32 = 128u32;
pub const NDIS_GFT_OFFLOAD_INFO_COPY_PACKET: u32 = 4u32;
pub const NDIS_GFT_OFFLOAD_INFO_DIRECTION_INGRESS: u32 = 1u32;
pub const NDIS_GFT_OFFLOAD_INFO_EXCEPTION_PACKET: u32 = 2u32;
pub const NDIS_GFT_OFFLOAD_INFO_SAMPLE_PACKET: u32 = 8u32;
pub const NDIS_GFT_OFFLOAD_PARAMETERS_CUSTOM_PROVIDER_RESERVED: u32 = 4278190080u32;
pub const NDIS_GFT_OFFLOAD_PARAMETERS_ENABLE_OFFLOAD: u32 = 1u32;
pub const NDIS_GFT_OFFLOAD_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_GFT_PROFILE_INFO_ARRAY_REVISION_1: u32 = 1u32;
pub const NDIS_GFT_PROFILE_INFO_REVISION_1: u32 = 1u32;
pub const NDIS_GFT_RESERVED_CUSTOM_ACTIONS: u32 = 256u32;
pub const NDIS_GFT_STATISTICS_REVISION_1: u32 = 1u32;
pub const NDIS_GFT_TABLE_INCLUDE_EXTERNAL_VPPORT: u32 = 1u32;
pub const NDIS_GFT_TABLE_INFO_ARRAY_REVISION_1: u32 = 1u32;
pub const NDIS_GFT_TABLE_INFO_REVISION_1: u32 = 1u32;
pub const NDIS_GFT_TABLE_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_GFT_UNDEFINED_COUNTER_ID: u32 = 0u32;
pub const NDIS_GFT_UNDEFINED_CUSTOM_ACTION: u32 = 0u32;
pub const NDIS_GFT_UNDEFINED_FLOW_ENTRY_ID: u32 = 0u32;
pub const NDIS_GFT_UNDEFINED_TABLE_ID: u32 = 0u32;
pub const NDIS_GFT_VPORT_DSCP_FLAGS_CHANGED: u32 = 67108864u32;
pub const NDIS_GFT_VPORT_DSCP_GUARD_ENABLE_RX: u32 = 1u32;
pub const NDIS_GFT_VPORT_DSCP_GUARD_ENABLE_TX: u32 = 2u32;
pub const NDIS_GFT_VPORT_DSCP_MASK_CHANGED: u32 = 8388608u32;
pub const NDIS_GFT_VPORT_DSCP_MASK_ENABLE_RX: u32 = 4u32;
pub const NDIS_GFT_VPORT_DSCP_MASK_ENABLE_TX: u32 = 8u32;
pub const NDIS_GFT_VPORT_ENABLE: u32 = 1u32;
pub const NDIS_GFT_VPORT_ENABLE_STATE_CHANGED: u32 = 1048576u32;
pub const NDIS_GFT_VPORT_EXCEPTION_VPORT_CHANGED: u32 = 2097152u32;
pub const NDIS_GFT_VPORT_MAX_DSCP_MASK_COUNTER_OBJECTS: u32 = 64u32;
pub const NDIS_GFT_VPORT_MAX_PRIORITY_MASK_COUNTER_OBJECTS: u32 = 8u32;
pub const NDIS_GFT_VPORT_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_GFT_VPORT_PARAMS_CHANGE_MASK: u32 = 4293918720u32;
pub const NDIS_GFT_VPORT_PARAMS_CUSTOM_PROVIDER_RESERVED: u32 = 1044480u32;
pub const NDIS_GFT_VPORT_PARSE_VXLAN: u32 = 2u32;
pub const NDIS_GFT_VPORT_PARSE_VXLAN_NOT_IN_SRC_PORT_RANGE: u32 = 4u32;
pub const NDIS_GFT_VPORT_PRIORITY_MASK_CHANGED: u32 = 16777216u32;
pub const NDIS_GFT_VPORT_SAMPLING_RATE_CHANGED: u32 = 4194304u32;
pub const NDIS_GFT_VPORT_VXLAN_SETTINGS_CHANGED: u32 = 33554432u32;
pub const NDIS_GFT_WCFE_ADD_IN_ACTIVATED_STATE: u32 = 1u32;
pub const NDIS_GFT_WCFE_COPY_ALL_PACKETS: u32 = 32u32;
pub const NDIS_GFT_WCFE_COUNTER_ALLOCATE: u32 = 1u32;
pub const NDIS_GFT_WCFE_COUNTER_CLIENT_SPECIFIED_ADDRESS: u32 = 4u32;
pub const NDIS_GFT_WCFE_COUNTER_MEMORY_MAPPED: u32 = 2u32;
pub const NDIS_GFT_WCFE_CUSTOM_ACTION_PRESENT: u32 = 64u32;
pub const NDIS_GFT_WCFE_REDIRECT_TO_EGRESS_QUEUE_OF_VPORT: u32 = 4u32;
pub const NDIS_GFT_WCFE_REDIRECT_TO_EGRESS_QUEUE_OF_VPORT_IF_TTL_IS_ONE: u32 = 16u32;
pub const NDIS_GFT_WCFE_REDIRECT_TO_INGRESS_QUEUE_OF_VPORT: u32 = 2u32;
pub const NDIS_GFT_WCFE_REDIRECT_TO_INGRESS_QUEUE_OF_VPORT_IF_TTL_IS_ONE: u32 = 8u32;
pub const NDIS_GFT_WILDCARD_MATCH_FLOW_ENTRY_REVISION_1: u32 = 1u32;
pub const NDIS_HARDWARE_CROSSTIMESTAMP_REVISION_1: u32 = 1u32;
pub const NDIS_HASH_FUNCTION_MASK: u32 = 255u32;
pub const NDIS_HASH_IPV4: u32 = 256u32;
pub const NDIS_HASH_IPV6: u32 = 1024u32;
pub const NDIS_HASH_IPV6_EX: u32 = 2048u32;
pub const NDIS_HASH_TCP_IPV4: u32 = 512u32;
pub const NDIS_HASH_TCP_IPV6: u32 = 4096u32;
pub const NDIS_HASH_TCP_IPV6_EX: u32 = 8192u32;
pub const NDIS_HASH_TYPE_MASK: u32 = 16776960u32;
pub const NDIS_HASH_UDP_IPV4: u32 = 16384u32;
pub const NDIS_HASH_UDP_IPV6: u32 = 32768u32;
pub const NDIS_HASH_UDP_IPV6_EX: u32 = 65536u32;
pub const NDIS_HD_SPLIT_ATTRIBUTES_REVISION_1: u32 = 1u32;
pub const NDIS_HD_SPLIT_CAPS_SUPPORTS_HEADER_DATA_SPLIT: u32 = 1u32;
pub const NDIS_HD_SPLIT_CAPS_SUPPORTS_IPV4_OPTIONS: u32 = 2u32;
pub const NDIS_HD_SPLIT_CAPS_SUPPORTS_IPV6_EXTENSION_HEADERS: u32 = 4u32;
pub const NDIS_HD_SPLIT_CAPS_SUPPORTS_TCP_OPTIONS: u32 = 8u32;
pub const NDIS_HD_SPLIT_COMBINE_ALL_HEADERS: u32 = 1u32;
pub const NDIS_HD_SPLIT_CURRENT_CONFIG_REVISION_1: u32 = 1u32;
pub const NDIS_HD_SPLIT_ENABLE_HEADER_DATA_SPLIT: u32 = 1u32;
pub const NDIS_HD_SPLIT_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_HYPERVISOR_INFO_FLAG_HYPERVISOR_PRESENT: u32 = 1u32;
pub const NDIS_HYPERVISOR_INFO_REVISION_1: u32 = 1u32;
pub const NDIS_INTERMEDIATE_DRIVER: u32 = 1u32;
pub const NDIS_INTERRUPT_MODERATION_CHANGE_NEEDS_REINITIALIZE: u32 = 2u32;
pub const NDIS_INTERRUPT_MODERATION_CHANGE_NEEDS_RESET: u32 = 1u32;
pub const NDIS_INTERRUPT_MODERATION_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_IPSEC_OFFLOAD_V2_ADD_SA_EX_REVISION_1: u32 = 1u32;
pub const NDIS_IPSEC_OFFLOAD_V2_ADD_SA_REVISION_1: u32 = 1u32;
pub const NDIS_IPSEC_OFFLOAD_V2_DELETE_SA_REVISION_1: u32 = 1u32;
pub const NDIS_IPSEC_OFFLOAD_V2_UPDATE_SA_REVISION_1: u32 = 1u32;
pub const NDIS_IP_OPER_STATE_REVISION_1: u32 = 1u32;
pub const NDIS_IP_OPER_STATUS_INFO_REVISION_1: u32 = 1u32;
pub const NDIS_ISOLATION_NAME_MAX_STRING_SIZE: u32 = 127u32;
pub const NDIS_ISOLATION_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_KDNET_ADD_PF_REVISION_1: u32 = 1u32;
pub const NDIS_KDNET_ENUMERATE_PFS_REVISION_1: u32 = 1u32;
pub const NDIS_KDNET_PF_ENUM_ELEMENT_REVISION_1: u32 = 1u32;
pub const NDIS_KDNET_QUERY_PF_INFORMATION_REVISION_1: u32 = 1u32;
pub const NDIS_KDNET_REMOVE_PF_REVISION_1: u32 = 1u32;
pub const NDIS_LARGE_SEND_OFFLOAD_MAX_HEADER_LENGTH: u32 = 128u32;
pub const NDIS_LEGACY_DRIVER: u32 = 1u32;
pub const NDIS_LEGACY_MINIPORT: u32 = 1u32;
pub const NDIS_LEGACY_PROTOCOL: u32 = 1u32;
pub const NDIS_LINK_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_LINK_STATE_DUPLEX_AUTO_NEGOTIATED: u32 = 4u32;
pub const NDIS_LINK_STATE_PAUSE_FUNCTIONS_AUTO_NEGOTIATED: u32 = 8u32;
pub const NDIS_LINK_STATE_RCV_LINK_SPEED_AUTO_NEGOTIATED: u32 = 2u32;
pub const NDIS_LINK_STATE_REVISION_1: u32 = 1u32;
pub const NDIS_LINK_STATE_XMIT_LINK_SPEED_AUTO_NEGOTIATED: u32 = 1u32;
pub const NDIS_MAC_OPTION_8021P_PRIORITY: u32 = 64u32;
pub const NDIS_MAC_OPTION_8021Q_VLAN: u32 = 512u32;
pub const NDIS_MAC_OPTION_COPY_LOOKAHEAD_DATA: u32 = 1u32;
pub const NDIS_MAC_OPTION_EOTX_INDICATION: u32 = 32u32;
pub const NDIS_MAC_OPTION_FULL_DUPLEX: u32 = 16u32;
pub const NDIS_MAC_OPTION_NO_LOOPBACK: u32 = 8u32;
pub const NDIS_MAC_OPTION_RECEIVE_AT_DPC: u32 = 256u32;
pub const NDIS_MAC_OPTION_RECEIVE_SERIALIZED: u32 = 2u32;
pub const NDIS_MAC_OPTION_RESERVED: u32 = 2147483648u32;
pub const NDIS_MAC_OPTION_SUPPORTS_MAC_ADDRESS_OVERWRITE: u32 = 128u32;
pub const NDIS_MAC_OPTION_TRANSFERS_NOT_PEND: u32 = 4u32;
pub const NDIS_MAXIMUM_PORTS: u32 = 16777216u32;
pub const NDIS_MAX_LOOKAHEAD_SIZE_ACCESSED_UNDEFINED: i32 = -1i32;
pub const NDIS_MAX_PROCESSOR_COUNT: u32 = 64u32;
pub const NDIS_MEDIA_CAP_RECEIVE: u32 = 2u32;
pub const NDIS_MEDIA_CAP_TRANSMIT: u32 = 1u32;
pub const NDIS_MEDIA_SPECIFIC_INFO_EAPOL: u32 = 2147549185u32;
pub const NDIS_MEDIA_SPECIFIC_INFO_FCOE: u32 = 2147549184u32;
pub const NDIS_MEDIA_SPECIFIC_INFO_LLDP: u32 = 2147549186u32;
pub const NDIS_MEDIA_SPECIFIC_INFO_TIMESYNC: u32 = 2147549187u32;
pub const NDIS_MEDIA_SPECIFIC_INFO_TUNDL: u32 = 65537u32;
pub const NDIS_MEMORY_CONTIGUOUS: u32 = 1u32;
pub const NDIS_MEMORY_NONCACHED: u32 = 2u32;
pub const NDIS_MINIPORT_ADAPTER_802_11_ATTRIBUTES_REVISION_1: u32 = 1u32;
pub const NDIS_MINIPORT_ADAPTER_802_11_ATTRIBUTES_REVISION_2: u32 = 2u32;
pub const NDIS_MINIPORT_ADAPTER_802_11_ATTRIBUTES_REVISION_3: u32 = 3u32;
pub const NDIS_MINIPORT_ADAPTER_GENERAL_ATTRIBUTES_REVISION_1: u32 = 1u32;
pub const NDIS_MINIPORT_ADAPTER_GENERAL_ATTRIBUTES_REVISION_2: u32 = 2u32;
pub const NDIS_MINIPORT_ADAPTER_HARDWARE_ASSIST_ATTRIBUTES_REVISION_1: u32 = 1u32;
pub const NDIS_MINIPORT_ADAPTER_HARDWARE_ASSIST_ATTRIBUTES_REVISION_2: u32 = 2u32;
pub const NDIS_MINIPORT_ADAPTER_HARDWARE_ASSIST_ATTRIBUTES_REVISION_3: u32 = 3u32;
pub const NDIS_MINIPORT_ADAPTER_HARDWARE_ASSIST_ATTRIBUTES_REVISION_4: u32 = 4u32;
pub const NDIS_MINIPORT_ADAPTER_NDK_ATTRIBUTES_REVISION_1: u32 = 1u32;
pub const NDIS_MINIPORT_ADAPTER_OFFLOAD_ATTRIBUTES_REVISION_1: u32 = 1u32;
pub const NDIS_MINIPORT_ADAPTER_PACKET_DIRECT_ATTRIBUTES_REVISION_1: u32 = 1u32;
pub const NDIS_MINIPORT_ADAPTER_REGISTRATION_ATTRIBUTES_REVISION_1: u32 = 1u32;
pub const NDIS_MINIPORT_ADAPTER_REGISTRATION_ATTRIBUTES_REVISION_2: u32 = 2u32;
pub const NDIS_MINIPORT_ADD_DEVICE_REGISTRATION_ATTRIBUTES_REVISION_1: u32 = 1u32;
pub const NDIS_MINIPORT_ATTRIBUTES_BUS_MASTER: u32 = 64u32;
pub const NDIS_MINIPORT_ATTRIBUTES_CONTROLS_DEFAULT_PORT: u32 = 128u32;
pub const NDIS_MINIPORT_ATTRIBUTES_DO_NOT_BIND_TO_ALL_CO: u32 = 16u32;
pub const NDIS_MINIPORT_ATTRIBUTES_HARDWARE_DEVICE: u32 = 1u32;
pub const NDIS_MINIPORT_ATTRIBUTES_NDIS_WDM: u32 = 2u32;
pub const NDIS_MINIPORT_ATTRIBUTES_NOT_CO_NDIS: u32 = 8u32;
pub const NDIS_MINIPORT_ATTRIBUTES_NO_HALT_ON_SUSPEND: u32 = 32u32;
pub const NDIS_MINIPORT_ATTRIBUTES_NO_OID_INTERCEPT_ON_NONDEFAULT_PORTS: u32 = 512u32;
pub const NDIS_MINIPORT_ATTRIBUTES_NO_PAUSE_ON_SUSPEND: u32 = 256u32;
pub const NDIS_MINIPORT_ATTRIBUTES_REGISTER_BUGCHECK_CALLBACK: u32 = 1024u32;
pub const NDIS_MINIPORT_ATTRIBUTES_SURPRISE_REMOVE_OK: u32 = 4u32;
pub const NDIS_MINIPORT_CO_CHARACTERISTICS_REVISION_1: u32 = 1u32;
pub const NDIS_MINIPORT_DRIVER: u32 = 1u32;
pub const NDIS_MINIPORT_DRIVER_CHARACTERISTICS_REVISION_1: u32 = 1u32;
pub const NDIS_MINIPORT_DRIVER_CHARACTERISTICS_REVISION_2: u32 = 2u32;
pub const NDIS_MINIPORT_DRIVER_CHARACTERISTICS_REVISION_3: u32 = 3u32;
pub const NDIS_MINIPORT_INIT_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_MINIPORT_INTERRUPT_REVISION_1: u32 = 1u32;
pub const NDIS_MINIPORT_MAJOR_VERSION: u32 = 6u32;
pub const NDIS_MINIPORT_MINIMUM_MAJOR_VERSION: u32 = 5u32;
pub const NDIS_MINIPORT_MINIMUM_MINOR_VERSION: u32 = 0u32;
pub const NDIS_MINIPORT_MINOR_VERSION: u32 = 87u32;
pub const NDIS_MINIPORT_PAUSE_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_MINIPORT_PNP_CHARACTERISTICS_REVISION_1: u32 = 1u32;
pub const NDIS_MINIPORT_RESTART_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_MINIPORT_SS_CHARACTERISTICS_REVISION_1: u32 = 1u32;
pub const NDIS_MIN_API: u32 = 1024u32;
pub const NDIS_MONITOR_CONFIG_REVISION_1: u32 = 1u32;
pub const NDIS_MSIX_CONFIG_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_M_MAX_LOOKAHEAD: u32 = 526u32;
pub const NDIS_NBL_FLAGS_CAPTURE_TIMESTAMP_ON_TRANSMIT: u32 = 65536u32;
pub const NDIS_NBL_FLAGS_HD_SPLIT: u32 = 256u32;
pub const NDIS_NBL_FLAGS_IS_IPV4: u32 = 512u32;
pub const NDIS_NBL_FLAGS_IS_IPV6: u32 = 1024u32;
pub const NDIS_NBL_FLAGS_IS_LOOPBACK_PACKET: u32 = 32768u32;
pub const NDIS_NBL_FLAGS_IS_TCP: u32 = 2048u32;
pub const NDIS_NBL_FLAGS_IS_UDP: u32 = 4096u32;
pub const NDIS_NBL_FLAGS_RECV_READ_ONLY: u32 = 2u32;
pub const NDIS_NBL_FLAGS_SEND_READ_ONLY: u32 = 1u32;
pub const NDIS_NBL_FLAGS_SPLIT_AT_UPPER_LAYER_PROTOCOL_HEADER: u32 = 8192u32;
pub const NDIS_NBL_FLAGS_SPLIT_AT_UPPER_LAYER_PROTOCOL_PAYLOAD: u32 = 16384u32;
pub const NDIS_NBL_MEDIA_SPECIFIC_INFO_REVISION_1: u32 = 1u32;
pub const NDIS_NDK_CAPABILITIES_REVISION_1: u32 = 1u32;
pub const NDIS_NDK_CONNECTIONS_REVISION_1: u32 = 1u32;
pub const NDIS_NDK_LOCAL_ENDPOINTS_REVISION_1: u32 = 1u32;
pub const NDIS_NDK_STATISTICS_INFO_REVISION_1: u32 = 1u32;
pub const NDIS_NIC_SWITCH_CAPABILITIES_REVISION_1: u32 = 1u32;
pub const NDIS_NIC_SWITCH_CAPABILITIES_REVISION_2: u32 = 2u32;
pub const NDIS_NIC_SWITCH_CAPABILITIES_REVISION_3: u32 = 3u32;
pub const NDIS_NIC_SWITCH_CAPS_ASYMMETRIC_QUEUE_PAIRS_FOR_NONDEFAULT_VPORT_SUPPORTED: u32 = 4u32;
pub const NDIS_NIC_SWITCH_CAPS_NIC_SWITCH_WITHOUT_IOV_SUPPORTED: u32 = 64u32;
pub const NDIS_NIC_SWITCH_CAPS_PER_VPORT_INTERRUPT_MODERATION_SUPPORTED: u32 = 2u32;
pub const NDIS_NIC_SWITCH_CAPS_RSS_ON_PF_VPORTS_SUPPORTED: u32 = 128u32;
pub const NDIS_NIC_SWITCH_CAPS_RSS_PARAMETERS_PER_PF_VPORT_SUPPORTED: u32 = 32u32;
pub const NDIS_NIC_SWITCH_CAPS_RSS_PER_PF_VPORT_HASH_FUNCTION_SUPPORTED: u32 = 512u32;
pub const NDIS_NIC_SWITCH_CAPS_RSS_PER_PF_VPORT_HASH_KEY_SUPPORTED: u32 = 2048u32;
pub const NDIS_NIC_SWITCH_CAPS_RSS_PER_PF_VPORT_HASH_TYPE_SUPPORTED: u32 = 1024u32;
pub const NDIS_NIC_SWITCH_CAPS_RSS_PER_PF_VPORT_INDIRECTION_TABLE_SIZE_RESTRICTED: u32 = 4096u32;
pub const NDIS_NIC_SWITCH_CAPS_RSS_PER_PF_VPORT_INDIRECTION_TABLE_SUPPORTED: u32 = 256u32;
pub const NDIS_NIC_SWITCH_CAPS_SINGLE_VPORT_POOL: u32 = 16u32;
pub const NDIS_NIC_SWITCH_CAPS_VF_RSS_SUPPORTED: u32 = 8u32;
pub const NDIS_NIC_SWITCH_CAPS_VLAN_SUPPORTED: u32 = 1u32;
pub const NDIS_NIC_SWITCH_DELETE_SWITCH_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_NIC_SWITCH_DELETE_VPORT_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_NIC_SWITCH_FREE_VF_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_NIC_SWITCH_INFO_ARRAY_REVISION_1: u32 = 1u32;
pub const NDIS_NIC_SWITCH_INFO_REVISION_1: u32 = 1u32;
pub const NDIS_NIC_SWITCH_PARAMETERS_CHANGE_MASK: u32 = 4294901760u32;
pub const NDIS_NIC_SWITCH_PARAMETERS_DEFAULT_NUMBER_OF_QUEUE_PAIRS_FOR_DEFAULT_VPORT: u32 = 1u32;
pub const NDIS_NIC_SWITCH_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_NIC_SWITCH_PARAMETERS_REVISION_2: u32 = 2u32;
pub const NDIS_NIC_SWITCH_PARAMETERS_SWITCH_NAME_CHANGED: u32 = 65536u32;
pub const NDIS_NIC_SWITCH_VF_INFO_ARRAY_ENUM_ON_SPECIFIC_SWITCH: u32 = 1u32;
pub const NDIS_NIC_SWITCH_VF_INFO_ARRAY_REVISION_1: u32 = 1u32;
pub const NDIS_NIC_SWITCH_VF_INFO_REVISION_1: u32 = 1u32;
pub const NDIS_NIC_SWITCH_VF_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_NIC_SWITCH_VPORT_INFO_ARRAY_ENUM_ON_SPECIFIC_FUNCTION: u32 = 1u32;
pub const NDIS_NIC_SWITCH_VPORT_INFO_ARRAY_ENUM_ON_SPECIFIC_SWITCH: u32 = 2u32;
pub const NDIS_NIC_SWITCH_VPORT_INFO_ARRAY_REVISION_1: u32 = 1u32;
pub const NDIS_NIC_SWITCH_VPORT_INFO_GFT_ENABLED: u32 = 4u32;
pub const NDIS_NIC_SWITCH_VPORT_INFO_LOOKAHEAD_SPLIT_ENABLED: u32 = 1u32;
pub const NDIS_NIC_SWITCH_VPORT_INFO_PACKET_DIRECT_RX_ONLY: u32 = 2u32;
pub const NDIS_NIC_SWITCH_VPORT_INFO_REVISION_1: u32 = 1u32;
pub const NDIS_NIC_SWITCH_VPORT_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_NIC_SWITCH_VPORT_PARAMETERS_REVISION_2: u32 = 2u32;
pub const NDIS_NIC_SWITCH_VPORT_PARAMS_CHANGE_MASK: u32 = 4294901760u32;
pub const NDIS_NIC_SWITCH_VPORT_PARAMS_ENFORCE_MAX_SG_LIST: u32 = 32768u32;
pub const NDIS_NIC_SWITCH_VPORT_PARAMS_FLAGS_CHANGED: u32 = 65536u32;
pub const NDIS_NIC_SWITCH_VPORT_PARAMS_INT_MOD_CHANGED: u32 = 262144u32;
pub const NDIS_NIC_SWITCH_VPORT_PARAMS_LOOKAHEAD_SPLIT_ENABLED: u32 = 1u32;
pub const NDIS_NIC_SWITCH_VPORT_PARAMS_NAME_CHANGED: u32 = 131072u32;
pub const NDIS_NIC_SWITCH_VPORT_PARAMS_NDK_PARAMS_CHANGED: u32 = 2097152u32;
pub const NDIS_NIC_SWITCH_VPORT_PARAMS_NUM_QUEUE_PAIRS_CHANGED: u32 = 8388608u32;
pub const NDIS_NIC_SWITCH_VPORT_PARAMS_PACKET_DIRECT_RX_ONLY: u32 = 2u32;
pub const NDIS_NIC_SWITCH_VPORT_PARAMS_PROCESSOR_AFFINITY_CHANGED: u32 = 1048576u32;
pub const NDIS_NIC_SWITCH_VPORT_PARAMS_QOS_SQ_ID_CHANGED: u32 = 4194304u32;
pub const NDIS_NIC_SWITCH_VPORT_PARAMS_STATE_CHANGED: u32 = 524288u32;
pub const NDIS_NT: u32 = 1u32;
pub const NDIS_OBJECT_REVISION_1: u32 = 1u32;
pub const NDIS_OBJECT_TYPE_BIND_PARAMETERS: u32 = 134u32;
pub const NDIS_OBJECT_TYPE_CLIENT_CHIMNEY_OFFLOAD_CHARACTERISTICS: u32 = 147u32;
pub const NDIS_OBJECT_TYPE_CLIENT_CHIMNEY_OFFLOAD_GENERIC_CHARACTERISTICS: u32 = 142u32;
pub const NDIS_OBJECT_TYPE_CONFIGURATION_OBJECT: u32 = 169u32;
pub const NDIS_OBJECT_TYPE_CO_CALL_MANAGER_OPTIONAL_HANDLERS: u32 = 165u32;
pub const NDIS_OBJECT_TYPE_CO_CLIENT_OPTIONAL_HANDLERS: u32 = 166u32;
pub const NDIS_OBJECT_TYPE_CO_MINIPORT_CHARACTERISTICS: u32 = 145u32;
pub const NDIS_OBJECT_TYPE_CO_PROTOCOL_CHARACTERISTICS: u32 = 144u32;
pub const NDIS_OBJECT_TYPE_DEFAULT: u32 = 128u32;
pub const NDIS_OBJECT_TYPE_DEVICE_OBJECT_ATTRIBUTES: u32 = 133u32;
pub const NDIS_OBJECT_TYPE_DRIVER_WRAPPER_OBJECT: u32 = 170u32;
pub const NDIS_OBJECT_TYPE_DRIVER_WRAPPER_REVISION_1: u32 = 1u32;
pub const NDIS_OBJECT_TYPE_FILTER_ATTACH_PARAMETERS: u32 = 153u32;
pub const NDIS_OBJECT_TYPE_FILTER_ATTRIBUTES: u32 = 141u32;
pub const NDIS_OBJECT_TYPE_FILTER_DRIVER_CHARACTERISTICS: u32 = 139u32;
pub const NDIS_OBJECT_TYPE_FILTER_PARTIAL_CHARACTERISTICS: u32 = 140u32;
pub const NDIS_OBJECT_TYPE_FILTER_PAUSE_PARAMETERS: u32 = 154u32;
pub const NDIS_OBJECT_TYPE_FILTER_RESTART_PARAMETERS: u32 = 155u32;
pub const NDIS_OBJECT_TYPE_HD_SPLIT_ATTRIBUTES: u32 = 171u32;
pub const NDIS_OBJECT_TYPE_MINIPORT_ADAPTER_GENERAL_ATTRIBUTES: u32 = 159u32;
pub const NDIS_OBJECT_TYPE_MINIPORT_ADAPTER_HARDWARE_ASSIST_ATTRIBUTES: u32 = 175u32;
pub const NDIS_OBJECT_TYPE_MINIPORT_ADAPTER_NATIVE_802_11_ATTRIBUTES: u32 = 161u32;
pub const NDIS_OBJECT_TYPE_MINIPORT_ADAPTER_NDK_ATTRIBUTES: u32 = 179u32;
pub const NDIS_OBJECT_TYPE_MINIPORT_ADAPTER_OFFLOAD_ATTRIBUTES: u32 = 160u32;
pub const NDIS_OBJECT_TYPE_MINIPORT_ADAPTER_PACKET_DIRECT_ATTRIBUTES: u32 = 197u32;
pub const NDIS_OBJECT_TYPE_MINIPORT_ADAPTER_REGISTRATION_ATTRIBUTES: u32 = 158u32;
pub const NDIS_OBJECT_TYPE_MINIPORT_ADD_DEVICE_REGISTRATION_ATTRIBUTES: u32 = 164u32;
pub const NDIS_OBJECT_TYPE_MINIPORT_DEVICE_POWER_NOTIFICATION: u32 = 198u32;
pub const NDIS_OBJECT_TYPE_MINIPORT_DRIVER_CHARACTERISTICS: u32 = 138u32;
pub const NDIS_OBJECT_TYPE_MINIPORT_INIT_PARAMETERS: u32 = 129u32;
pub const NDIS_OBJECT_TYPE_MINIPORT_INTERRUPT: u32 = 132u32;
pub const NDIS_OBJECT_TYPE_MINIPORT_PNP_CHARACTERISTICS: u32 = 146u32;
pub const NDIS_OBJECT_TYPE_MINIPORT_SS_CHARACTERISTICS: u32 = 180u32;
pub const NDIS_OBJECT_TYPE_NDK_PROVIDER_CHARACTERISTICS: u32 = 178u32;
pub const NDIS_OBJECT_TYPE_NSI_COMPARTMENT_RW_STRUCT: u32 = 173u32;
pub const NDIS_OBJECT_TYPE_NSI_INTERFACE_PERSIST_RW_STRUCT: u32 = 174u32;
pub const NDIS_OBJECT_TYPE_NSI_NETWORK_RW_STRUCT: u32 = 172u32;
pub const NDIS_OBJECT_TYPE_OFFLOAD: u32 = 167u32;
pub const NDIS_OBJECT_TYPE_OFFLOAD_ENCAPSULATION: u32 = 168u32;
pub const NDIS_OBJECT_TYPE_OID_REQUEST: u32 = 150u32;
pub const NDIS_OBJECT_TYPE_OPEN_PARAMETERS: u32 = 135u32;
pub const NDIS_OBJECT_TYPE_PCI_DEVICE_CUSTOM_PROPERTIES_REVISION_1: u32 = 1u32;
pub const NDIS_OBJECT_TYPE_PCI_DEVICE_CUSTOM_PROPERTIES_REVISION_2: u32 = 2u32;
pub const NDIS_OBJECT_TYPE_PD_RECEIVE_QUEUE: u32 = 191u32;
pub const NDIS_OBJECT_TYPE_PD_TRANSMIT_QUEUE: u32 = 190u32;
pub const NDIS_OBJECT_TYPE_PORT_CHARACTERISTICS: u32 = 156u32;
pub const NDIS_OBJECT_TYPE_PORT_STATE: u32 = 157u32;
pub const NDIS_OBJECT_TYPE_PROTOCOL_DRIVER_CHARACTERISTICS: u32 = 149u32;
pub const NDIS_OBJECT_TYPE_PROTOCOL_RESTART_PARAMETERS: u32 = 163u32;
pub const NDIS_OBJECT_TYPE_PROVIDER_CHIMNEY_OFFLOAD_CHARACTERISTICS: u32 = 148u32;
pub const NDIS_OBJECT_TYPE_PROVIDER_CHIMNEY_OFFLOAD_GENERIC_CHARACTERISTICS: u32 = 143u32;
pub const NDIS_OBJECT_TYPE_QOS_CAPABILITIES: u32 = 181u32;
pub const NDIS_OBJECT_TYPE_QOS_CLASSIFICATION_ELEMENT: u32 = 183u32;
pub const NDIS_OBJECT_TYPE_QOS_PARAMETERS: u32 = 182u32;
pub const NDIS_OBJECT_TYPE_REQUEST_EX: u32 = 150u32;
pub const NDIS_OBJECT_TYPE_RESTART_GENERAL_ATTRIBUTES: u32 = 162u32;
pub const NDIS_OBJECT_TYPE_RSS_CAPABILITIES: u32 = 136u32;
pub const NDIS_OBJECT_TYPE_RSS_PARAMETERS: u32 = 137u32;
pub const NDIS_OBJECT_TYPE_RSS_PARAMETERS_V2: u32 = 200u32;
pub const NDIS_OBJECT_TYPE_RSS_PROCESSOR_INFO: u32 = 177u32;
pub const NDIS_OBJECT_TYPE_RSS_SET_INDIRECTION_ENTRIES: u32 = 201u32;
pub const NDIS_OBJECT_TYPE_SG_DMA_DESCRIPTION: u32 = 131u32;
pub const NDIS_OBJECT_TYPE_SHARED_MEMORY_PROVIDER_CHARACTERISTICS: u32 = 176u32;
pub const NDIS_OBJECT_TYPE_STATUS_INDICATION: u32 = 152u32;
pub const NDIS_OBJECT_TYPE_SWITCH_OPTIONAL_HANDLERS: u32 = 184u32;
pub const NDIS_OBJECT_TYPE_TIMER_CHARACTERISTICS: u32 = 151u32;
pub const NDIS_OFFLOAD_ENCAPSULATION_REVISION_1: u32 = 1u32;
pub const NDIS_OFFLOAD_FLAGS_GROUP_CHECKSUM_CAPABILITIES: u32 = 1u32;
pub const NDIS_OFFLOAD_NOT_SUPPORTED: u32 = 0u32;
pub const NDIS_OFFLOAD_PARAMETERS_CONNECTION_OFFLOAD_DISABLED: u32 = 1u32;
pub const NDIS_OFFLOAD_PARAMETERS_CONNECTION_OFFLOAD_ENABLED: u32 = 2u32;
pub const NDIS_OFFLOAD_PARAMETERS_IPSECV1_AH_AND_ESP_ENABLED: u32 = 4u32;
pub const NDIS_OFFLOAD_PARAMETERS_IPSECV1_AH_ENABLED: u32 = 2u32;
pub const NDIS_OFFLOAD_PARAMETERS_IPSECV1_DISABLED: u32 = 1u32;
pub const NDIS_OFFLOAD_PARAMETERS_IPSECV1_ESP_ENABLED: u32 = 3u32;
pub const NDIS_OFFLOAD_PARAMETERS_IPSECV2_AH_AND_ESP_ENABLED: u32 = 4u32;
pub const NDIS_OFFLOAD_PARAMETERS_IPSECV2_AH_ENABLED: u32 = 2u32;
pub const NDIS_OFFLOAD_PARAMETERS_IPSECV2_DISABLED: u32 = 1u32;
pub const NDIS_OFFLOAD_PARAMETERS_IPSECV2_ESP_ENABLED: u32 = 3u32;
pub const NDIS_OFFLOAD_PARAMETERS_LSOV1_DISABLED: u32 = 1u32;
pub const NDIS_OFFLOAD_PARAMETERS_LSOV1_ENABLED: u32 = 2u32;
pub const NDIS_OFFLOAD_PARAMETERS_LSOV2_DISABLED: u32 = 1u32;
pub const NDIS_OFFLOAD_PARAMETERS_LSOV2_ENABLED: u32 = 2u32;
pub const NDIS_OFFLOAD_PARAMETERS_NO_CHANGE: u32 = 0u32;
pub const NDIS_OFFLOAD_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_OFFLOAD_PARAMETERS_REVISION_2: u32 = 2u32;
pub const NDIS_OFFLOAD_PARAMETERS_REVISION_3: u32 = 3u32;
pub const NDIS_OFFLOAD_PARAMETERS_REVISION_4: u32 = 4u32;
pub const NDIS_OFFLOAD_PARAMETERS_REVISION_5: u32 = 5u32;
pub const NDIS_OFFLOAD_PARAMETERS_RSC_DISABLED: u32 = 1u32;
pub const NDIS_OFFLOAD_PARAMETERS_RSC_ENABLED: u32 = 2u32;
pub const NDIS_OFFLOAD_PARAMETERS_RX_ENABLED_TX_DISABLED: u32 = 3u32;
pub const NDIS_OFFLOAD_PARAMETERS_SKIP_REGISTRY_UPDATE: u32 = 1u32;
pub const NDIS_OFFLOAD_PARAMETERS_TX_ENABLED_RX_DISABLED: u32 = 2u32;
pub const NDIS_OFFLOAD_PARAMETERS_TX_RX_DISABLED: u32 = 1u32;
pub const NDIS_OFFLOAD_PARAMETERS_TX_RX_ENABLED: u32 = 4u32;
pub const NDIS_OFFLOAD_PARAMETERS_USO_DISABLED: u32 = 1u32;
pub const NDIS_OFFLOAD_PARAMETERS_USO_ENABLED: u32 = 2u32;
pub const NDIS_OFFLOAD_REVISION_1: u32 = 1u32;
pub const NDIS_OFFLOAD_REVISION_2: u32 = 2u32;
pub const NDIS_OFFLOAD_REVISION_3: u32 = 3u32;
pub const NDIS_OFFLOAD_REVISION_4: u32 = 4u32;
pub const NDIS_OFFLOAD_REVISION_5: u32 = 5u32;
pub const NDIS_OFFLOAD_REVISION_6: u32 = 6u32;
pub const NDIS_OFFLOAD_REVISION_7: u32 = 7u32;
pub const NDIS_OFFLOAD_SET_NO_CHANGE: u32 = 0u32;
pub const NDIS_OFFLOAD_SET_OFF: u32 = 2u32;
pub const NDIS_OFFLOAD_SET_ON: u32 = 1u32;
pub const NDIS_OFFLOAD_SUPPORTED: u32 = 1u32;
pub const NDIS_OID_REQUEST_FLAGS_VPORT_ID_VALID: u32 = 1u32;
pub const NDIS_OID_REQUEST_NDIS_RESERVED_SIZE: u32 = 16u32;
pub const NDIS_OID_REQUEST_REVISION_1: u32 = 1u32;
pub const NDIS_OID_REQUEST_REVISION_2: u32 = 2u32;
pub const NDIS_OID_REQUEST_TIMEOUT_INFINITE: u32 = 0u32;
pub const NDIS_OPEN_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_OPEN_RECEIVE_NOT_REENTRANT: u32 = 1u32;
pub const NDIS_OPER_STATE_REVISION_1: u32 = 1u32;
pub const NDIS_PACKET_TYPE_ALL_FUNCTIONAL: u32 = 8192u32;
pub const NDIS_PACKET_TYPE_ALL_LOCAL: u32 = 128u32;
pub const NDIS_PACKET_TYPE_ALL_MULTICAST: u32 = 4u32;
pub const NDIS_PACKET_TYPE_BROADCAST: u32 = 8u32;
pub const NDIS_PACKET_TYPE_DIRECTED: u32 = 1u32;
pub const NDIS_PACKET_TYPE_FUNCTIONAL: u32 = 16384u32;
pub const NDIS_PACKET_TYPE_GROUP: u32 = 4096u32;
pub const NDIS_PACKET_TYPE_MAC_FRAME: u32 = 32768u32;
pub const NDIS_PACKET_TYPE_MULTICAST: u32 = 2u32;
pub const NDIS_PACKET_TYPE_NO_LOCAL: u32 = 65536u32;
pub const NDIS_PACKET_TYPE_PROMISCUOUS: u32 = 32u32;
pub const NDIS_PACKET_TYPE_SMT: u32 = 64u32;
pub const NDIS_PACKET_TYPE_SOURCE_ROUTING: u32 = 16u32;
pub const NDIS_PAUSE_ATTACH_FILTER: u32 = 16u32;
pub const NDIS_PAUSE_BIND_PROTOCOL: u32 = 4u32;
pub const NDIS_PAUSE_DETACH_FILTER: u32 = 32u32;
pub const NDIS_PAUSE_FILTER_RESTART_STACK: u32 = 64u32;
pub const NDIS_PAUSE_LOW_POWER: u32 = 2u32;
pub const NDIS_PAUSE_MINIPORT_DEVICE_REMOVE: u32 = 128u32;
pub const NDIS_PAUSE_NDIS_INTERNAL: u32 = 1u32;
pub const NDIS_PAUSE_UNBIND_PROTOCOL: u32 = 8u32;
pub const NDIS_PD_ACQUIRE_QUEUES_FLAG_DRAIN_NOTIFICATION: u32 = 1u32;
pub const NDIS_PD_ACQUIRE_QUEUES_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_PD_CAPABILITIES_REVISION_1: u32 = 1u32;
pub const NDIS_PD_CAPS_DRAIN_NOTIFICATIONS_SUPPORTED: u32 = 2u32;
pub const NDIS_PD_CAPS_NOTIFICATION_MODERATION_COUNT_SUPPORTED: u32 = 8u32;
pub const NDIS_PD_CAPS_NOTIFICATION_MODERATION_INTERVAL_SUPPORTED: u32 = 4u32;
pub const NDIS_PD_CAPS_RECEIVE_FILTER_COUNTERS_SUPPORTED: u32 = 1u32;
pub const NDIS_PD_CLOSE_PROVIDER_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_PD_CONFIG_REVISION_1: u32 = 1u32;
pub const NDIS_PD_COUNTER_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_PD_FILTER_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_PD_OPEN_PROVIDER_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_PD_PROVIDER_DISPATCH_REVISION_1: u32 = 1u32;
pub const NDIS_PD_QUEUE_DISPATCH_REVISION_1: u32 = 1u32;
pub const NDIS_PD_QUEUE_FLAG_DRAIN_NOTIFICATION: u32 = 1u32;
pub const NDIS_PD_QUEUE_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_PD_QUEUE_REVISION_1: u32 = 1u32;
pub const NDIS_PM_CAPABILITIES_REVISION_1: u32 = 1u32;
pub const NDIS_PM_CAPABILITIES_REVISION_2: u32 = 2u32;
pub const NDIS_PM_MAX_PATTERN_ID: u32 = 65535u32;
pub const NDIS_PM_MAX_STRING_SIZE: u32 = 64u32;
pub const NDIS_PM_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_PM_PARAMETERS_REVISION_2: u32 = 2u32;
pub const NDIS_PM_PRIVATE_PATTERN_ID: u32 = 1u32;
pub const NDIS_PM_PROTOCOL_OFFLOAD_80211_RSN_REKEY_ENABLED: u32 = 128u32;
pub const NDIS_PM_PROTOCOL_OFFLOAD_80211_RSN_REKEY_SUPPORTED: u32 = 128u32;
pub const NDIS_PM_PROTOCOL_OFFLOAD_ARP_ENABLED: u32 = 1u32;
pub const NDIS_PM_PROTOCOL_OFFLOAD_ARP_SUPPORTED: u32 = 1u32;
pub const NDIS_PM_PROTOCOL_OFFLOAD_NS_ENABLED: u32 = 2u32;
pub const NDIS_PM_PROTOCOL_OFFLOAD_NS_SUPPORTED: u32 = 2u32;
pub const NDIS_PM_PROTOCOL_OFFLOAD_PRIORITY_HIGHEST: u32 = 1u32;
pub const NDIS_PM_PROTOCOL_OFFLOAD_PRIORITY_LOWEST: u32 = 4294967295u32;
pub const NDIS_PM_PROTOCOL_OFFLOAD_PRIORITY_NORMAL: u32 = 268435456u32;
pub const NDIS_PM_PROTOCOL_OFFLOAD_REVISION_1: u32 = 1u32;
pub const NDIS_PM_PROTOCOL_OFFLOAD_REVISION_2: u32 = 2u32;
pub const NDIS_PM_SELECTIVE_SUSPEND_ENABLED: u32 = 16u32;
pub const NDIS_PM_SELECTIVE_SUSPEND_SUPPORTED: u32 = 2u32;
pub const NDIS_PM_WAKE_ON_LINK_CHANGE_ENABLED: u32 = 1u32;
pub const NDIS_PM_WAKE_ON_MEDIA_CONNECT_SUPPORTED: u32 = 1u32;
pub const NDIS_PM_WAKE_ON_MEDIA_DISCONNECT_ENABLED: u32 = 2u32;
pub const NDIS_PM_WAKE_ON_MEDIA_DISCONNECT_SUPPORTED: u32 = 2u32;
pub const NDIS_PM_WAKE_PACKET_INDICATION_SUPPORTED: u32 = 1u32;
pub const NDIS_PM_WAKE_PACKET_REVISION_1: u32 = 1u32;
pub const NDIS_PM_WAKE_REASON_REVISION_1: u32 = 1u32;
pub const NDIS_PM_WOL_BITMAP_PATTERN_ENABLED: u32 = 1u32;
pub const NDIS_PM_WOL_BITMAP_PATTERN_SUPPORTED: u32 = 1u32;
pub const NDIS_PM_WOL_EAPOL_REQUEST_ID_MESSAGE_ENABLED: u32 = 65536u32;
pub const NDIS_PM_WOL_EAPOL_REQUEST_ID_MESSAGE_SUPPORTED: u32 = 65536u32;
pub const NDIS_PM_WOL_IPV4_DEST_ADDR_WILDCARD_ENABLED: u32 = 512u32;
pub const NDIS_PM_WOL_IPV4_DEST_ADDR_WILDCARD_SUPPORTED: u32 = 512u32;
pub const NDIS_PM_WOL_IPV4_TCP_SYN_ENABLED: u32 = 4u32;
pub const NDIS_PM_WOL_IPV4_TCP_SYN_SUPPORTED: u32 = 4u32;
pub const NDIS_PM_WOL_IPV6_DEST_ADDR_WILDCARD_ENABLED: u32 = 2048u32;
pub const NDIS_PM_WOL_IPV6_DEST_ADDR_WILDCARD_SUPPORTED: u32 = 2048u32;
pub const NDIS_PM_WOL_IPV6_TCP_SYN_ENABLED: u32 = 8u32;
pub const NDIS_PM_WOL_IPV6_TCP_SYN_SUPPORTED: u32 = 8u32;
pub const NDIS_PM_WOL_MAGIC_PACKET_ENABLED: u32 = 2u32;
pub const NDIS_PM_WOL_MAGIC_PACKET_SUPPORTED: u32 = 2u32;
pub const NDIS_PM_WOL_PATTERN_REVISION_1: u32 = 1u32;
pub const NDIS_PM_WOL_PATTERN_REVISION_2: u32 = 2u32;
pub const NDIS_PM_WOL_PRIORITY_HIGHEST: u32 = 1u32;
pub const NDIS_PM_WOL_PRIORITY_LOWEST: u32 = 4294967295u32;
pub const NDIS_PM_WOL_PRIORITY_NORMAL: u32 = 268435456u32;
pub const NDIS_PNP_WAKE_UP_LINK_CHANGE: u32 = 4u32;
pub const NDIS_PNP_WAKE_UP_MAGIC_PACKET: u32 = 1u32;
pub const NDIS_PNP_WAKE_UP_PATTERN_MATCH: u32 = 2u32;
pub const NDIS_POLL_CHARACTERISTICS_REVISION_1: u32 = 1u32;
pub const NDIS_POLL_DATA_REVISION_1: u32 = 1u32;
pub const NDIS_POLL_NOTIFICATION_REVISION_1: u32 = 1u32;
pub const NDIS_PORT_ARRAY_REVISION_1: u32 = 1u32;
pub const NDIS_PORT_AUTHENTICATION_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_PORT_CHARACTERISTICS_REVISION_1: u32 = 1u32;
pub const NDIS_PORT_CHAR_USE_DEFAULT_AUTH_SETTINGS: u32 = 1u32;
pub const NDIS_PORT_STATE_REVISION_1: u32 = 1u32;
pub const NDIS_PROTOCOL_CO_CHARACTERISTICS_REVISION_1: u32 = 1u32;
pub const NDIS_PROTOCOL_DRIVER_CHARACTERISTICS_REVISION_1: u32 = 1u32;
pub const NDIS_PROTOCOL_DRIVER_CHARACTERISTICS_REVISION_2: u32 = 2u32;
pub const NDIS_PROTOCOL_DRIVER_SUPPORTS_CURRENT_MAC_ADDRESS_CHANGE: u32 = 2u32;
pub const NDIS_PROTOCOL_DRIVER_SUPPORTS_L2_MTU_SIZE_CHANGE: u32 = 4u32;
pub const NDIS_PROTOCOL_ID_DEFAULT: u32 = 0u32;
pub const NDIS_PROTOCOL_ID_IP6: u32 = 3u32;
pub const NDIS_PROTOCOL_ID_IPX: u32 = 6u32;
pub const NDIS_PROTOCOL_ID_MASK: u32 = 15u32;
pub const NDIS_PROTOCOL_ID_MAX: u32 = 15u32;
pub const NDIS_PROTOCOL_ID_NBF: u32 = 7u32;
pub const NDIS_PROTOCOL_ID_TCP_IP: u32 = 2u32;
pub const NDIS_PROTOCOL_MAJOR_VERSION: u32 = 6u32;
pub const NDIS_PROTOCOL_MINIMUM_MAJOR_VERSION: u32 = 4u32;
pub const NDIS_PROTOCOL_MINIMUM_MINOR_VERSION: u32 = 0u32;
pub const NDIS_PROTOCOL_MINOR_VERSION: u32 = 87u32;
pub const NDIS_PROTOCOL_PAUSE_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_PROTOCOL_RESTART_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_PROT_OPTION_ESTIMATED_LENGTH: u32 = 1u32;
pub const NDIS_PROT_OPTION_NO_LOOPBACK: u32 = 2u32;
pub const NDIS_PROT_OPTION_NO_RSVD_ON_RCVPKT: u32 = 4u32;
pub const NDIS_PROT_OPTION_SEND_RESTRICTED: u32 = 8u32;
pub const NDIS_QOS_ACTION_MAXIMUM: u32 = 1u32;
pub const NDIS_QOS_ACTION_PRIORITY: u32 = 0u32;
pub const NDIS_QOS_CAPABILITIES_CEE_DCBX_SUPPORTED: u32 = 4u32;
pub const NDIS_QOS_CAPABILITIES_IEEE_DCBX_SUPPORTED: u32 = 8u32;
pub const NDIS_QOS_CAPABILITIES_MACSEC_BYPASS_SUPPORTED: u32 = 2u32;
pub const NDIS_QOS_CAPABILITIES_REVISION_1: u32 = 1u32;
pub const NDIS_QOS_CAPABILITIES_STRICT_TSA_SUPPORTED: u32 = 1u32;
pub const NDIS_QOS_CLASSIFICATION_ELEMENT_REVISION_1: u32 = 1u32;
pub const NDIS_QOS_CLASSIFICATION_ENFORCED_BY_MINIPORT: u32 = 16777216u32;
pub const NDIS_QOS_CLASSIFICATION_SET_BY_MINIPORT_MASK: u32 = 4278190080u32;
pub const NDIS_QOS_CONDITION_DEFAULT: u32 = 1u32;
pub const NDIS_QOS_CONDITION_ETHERTYPE: u32 = 5u32;
pub const NDIS_QOS_CONDITION_MAXIMUM: u32 = 7u32;
pub const NDIS_QOS_CONDITION_NETDIRECT_PORT: u32 = 6u32;
pub const NDIS_QOS_CONDITION_RESERVED: u32 = 0u32;
pub const NDIS_QOS_CONDITION_TCP_OR_UDP_PORT: u32 = 4u32;
pub const NDIS_QOS_CONDITION_TCP_PORT: u32 = 2u32;
pub const NDIS_QOS_CONDITION_UDP_PORT: u32 = 3u32;
pub const NDIS_QOS_DEFAULT_SQ_ID: u32 = 0u32;
pub const NDIS_QOS_MAXIMUM_PRIORITIES: u32 = 8u32;
pub const NDIS_QOS_MAXIMUM_TRAFFIC_CLASSES: u32 = 8u32;
pub const NDIS_QOS_OFFLOAD_CAPABILITIES_REVISION_1: u32 = 1u32;
pub const NDIS_QOS_OFFLOAD_CAPABILITIES_REVISION_2: u32 = 2u32;
pub const NDIS_QOS_OFFLOAD_CAPS_GFT_SQ: u32 = 2u32;
pub const NDIS_QOS_OFFLOAD_CAPS_STANDARD_SQ: u32 = 1u32;
pub const NDIS_QOS_PARAMETERS_CLASSIFICATION_CHANGED: u32 = 65536u32;
pub const NDIS_QOS_PARAMETERS_CLASSIFICATION_CONFIGURED: u32 = 131072u32;
pub const NDIS_QOS_PARAMETERS_ETS_CHANGED: u32 = 1u32;
pub const NDIS_QOS_PARAMETERS_ETS_CONFIGURED: u32 = 2u32;
pub const NDIS_QOS_PARAMETERS_PFC_CHANGED: u32 = 256u32;
pub const NDIS_QOS_PARAMETERS_PFC_CONFIGURED: u32 = 512u32;
pub const NDIS_QOS_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_QOS_PARAMETERS_WILLING: u32 = 2147483648u32;
pub const NDIS_QOS_SQ_ARRAY_REVISION_1: u32 = 1u32;
pub const NDIS_QOS_SQ_PARAMETERS_ARRAY_REVISION_1: u32 = 1u32;
pub const NDIS_QOS_SQ_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_QOS_SQ_PARAMETERS_REVISION_2: u32 = 2u32;
pub const NDIS_QOS_SQ_RECEIVE_CAP_ENABLED: u32 = 4u32;
pub const NDIS_QOS_SQ_STATS_REVISION_1: u32 = 1u32;
pub const NDIS_QOS_SQ_TRANSMIT_CAP_ENABLED: u32 = 1u32;
pub const NDIS_QOS_SQ_TRANSMIT_RESERVATION_ENABLED: u32 = 2u32;
pub const NDIS_QOS_TSA_CBS: u32 = 1u32;
pub const NDIS_QOS_TSA_ETS: u32 = 2u32;
pub const NDIS_QOS_TSA_MAXIMUM: u32 = 3u32;
pub const NDIS_QOS_TSA_STRICT: u32 = 0u32;
pub const NDIS_RECEIVE_FILTER_ANY_VLAN_SUPPORTED: u32 = 32u32;
pub const NDIS_RECEIVE_FILTER_ARP_HEADER_OPERATION_SUPPORTED: u32 = 1u32;
pub const NDIS_RECEIVE_FILTER_ARP_HEADER_SPA_SUPPORTED: u32 = 2u32;
pub const NDIS_RECEIVE_FILTER_ARP_HEADER_SUPPORTED: u32 = 8u32;
pub const NDIS_RECEIVE_FILTER_ARP_HEADER_TPA_SUPPORTED: u32 = 4u32;
pub const NDIS_RECEIVE_FILTER_CAPABILITIES_REVISION_1: u32 = 1u32;
pub const NDIS_RECEIVE_FILTER_CAPABILITIES_REVISION_2: u32 = 2u32;
pub const NDIS_RECEIVE_FILTER_CLEAR_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_RECEIVE_FILTER_DYNAMIC_PROCESSOR_AFFINITY_CHANGE_FOR_DEFAULT_QUEUE_SUPPORTED: u32 = 64u32;
pub const NDIS_RECEIVE_FILTER_DYNAMIC_PROCESSOR_AFFINITY_CHANGE_SUPPORTED: u32 = 8u32;
pub const NDIS_RECEIVE_FILTER_FIELD_MAC_HEADER_VLAN_UNTAGGED_OR_ZERO: u32 = 1u32;
pub const NDIS_RECEIVE_FILTER_FIELD_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_RECEIVE_FILTER_FIELD_PARAMETERS_REVISION_2: u32 = 2u32;
pub const NDIS_RECEIVE_FILTER_FLAGS_RESERVED: u32 = 1u32;
pub const NDIS_RECEIVE_FILTER_GLOBAL_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_RECEIVE_FILTER_IMPLAT_MIN_OF_QUEUES_MODE: u32 = 64u32;
pub const NDIS_RECEIVE_FILTER_IMPLAT_SUM_OF_QUEUES_MODE: u32 = 128u32;
pub const NDIS_RECEIVE_FILTER_INFO_ARRAY_REVISION_1: u32 = 1u32;
pub const NDIS_RECEIVE_FILTER_INFO_ARRAY_REVISION_2: u32 = 2u32;
pub const NDIS_RECEIVE_FILTER_INFO_ARRAY_VPORT_ID_SPECIFIED: u32 = 1u32;
pub const NDIS_RECEIVE_FILTER_INFO_REVISION_1: u32 = 1u32;
pub const NDIS_RECEIVE_FILTER_INTERRUPT_VECTOR_COALESCING_SUPPORTED: u32 = 16u32;
pub const NDIS_RECEIVE_FILTER_IPV4_HEADER_PROTOCOL_SUPPORTED: u32 = 1u32;
pub const NDIS_RECEIVE_FILTER_IPV4_HEADER_SUPPORTED: u32 = 2u32;
pub const NDIS_RECEIVE_FILTER_IPV6_HEADER_PROTOCOL_SUPPORTED: u32 = 1u32;
pub const NDIS_RECEIVE_FILTER_IPV6_HEADER_SUPPORTED: u32 = 4u32;
pub const NDIS_RECEIVE_FILTER_LOOKAHEAD_SPLIT_SUPPORTED: u32 = 4u32;
pub const NDIS_RECEIVE_FILTER_MAC_HEADER_DEST_ADDR_SUPPORTED: u32 = 1u32;
pub const NDIS_RECEIVE_FILTER_MAC_HEADER_PACKET_TYPE_SUPPORTED: u32 = 32u32;
pub const NDIS_RECEIVE_FILTER_MAC_HEADER_PRIORITY_SUPPORTED: u32 = 16u32;
pub const NDIS_RECEIVE_FILTER_MAC_HEADER_PROTOCOL_SUPPORTED: u32 = 4u32;
pub const NDIS_RECEIVE_FILTER_MAC_HEADER_SOURCE_ADDR_SUPPORTED: u32 = 2u32;
pub const NDIS_RECEIVE_FILTER_MAC_HEADER_SUPPORTED: u32 = 1u32;
pub const NDIS_RECEIVE_FILTER_MAC_HEADER_VLAN_ID_SUPPORTED: u32 = 8u32;
pub const NDIS_RECEIVE_FILTER_MOVE_FILTER_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_RECEIVE_FILTER_MSI_X_SUPPORTED: u32 = 1u32;
pub const NDIS_RECEIVE_FILTER_PACKET_COALESCING_FILTERS_ENABLED: u32 = 2u32;
pub const NDIS_RECEIVE_FILTER_PACKET_COALESCING_SUPPORTED_ON_DEFAULT_QUEUE: u32 = 256u32;
pub const NDIS_RECEIVE_FILTER_PACKET_ENCAPSULATION: u32 = 2u32;
pub const NDIS_RECEIVE_FILTER_PACKET_ENCAPSULATION_GRE: u32 = 2u32;
pub const NDIS_RECEIVE_FILTER_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_RECEIVE_FILTER_PARAMETERS_REVISION_2: u32 = 2u32;
pub const NDIS_RECEIVE_FILTER_QUEUE_STATE_CHANGE_REVISION_1: u32 = 1u32;
pub const NDIS_RECEIVE_FILTER_RESERVED: u32 = 254u32;
pub const NDIS_RECEIVE_FILTER_TEST_HEADER_FIELD_EQUAL_SUPPORTED: u32 = 1u32;
pub const NDIS_RECEIVE_FILTER_TEST_HEADER_FIELD_MASK_EQUAL_SUPPORTED: u32 = 2u32;
pub const NDIS_RECEIVE_FILTER_TEST_HEADER_FIELD_NOT_EQUAL_SUPPORTED: u32 = 4u32;
pub const NDIS_RECEIVE_FILTER_UDP_HEADER_DEST_PORT_SUPPORTED: u32 = 1u32;
pub const NDIS_RECEIVE_FILTER_UDP_HEADER_SUPPORTED: u32 = 16u32;
pub const NDIS_RECEIVE_FILTER_VMQ_FILTERS_ENABLED: u32 = 1u32;
pub const NDIS_RECEIVE_FILTER_VM_QUEUES_ENABLED: u32 = 1u32;
pub const NDIS_RECEIVE_FILTER_VM_QUEUE_SUPPORTED: u32 = 2u32;
pub const NDIS_RECEIVE_FLAGS_DISPATCH_LEVEL: u32 = 1u32;
pub const NDIS_RECEIVE_FLAGS_MORE_NBLS: u32 = 8192u32;
pub const NDIS_RECEIVE_FLAGS_PERFECT_FILTERED: u32 = 1024u32;
pub const NDIS_RECEIVE_FLAGS_RESOURCES: u32 = 2u32;
pub const NDIS_RECEIVE_FLAGS_SHARED_MEMORY_INFO_VALID: u32 = 4096u32;
pub const NDIS_RECEIVE_FLAGS_SINGLE_ETHER_TYPE: u32 = 256u32;
pub const NDIS_RECEIVE_FLAGS_SINGLE_QUEUE: u32 = 2048u32;
pub const NDIS_RECEIVE_FLAGS_SINGLE_VLAN: u32 = 512u32;
pub const NDIS_RECEIVE_FLAGS_SWITCH_DESTINATION_GROUP: u32 = 16384u32;
pub const NDIS_RECEIVE_FLAGS_SWITCH_SINGLE_SOURCE: u32 = 32768u32;
pub const NDIS_RECEIVE_HASH_FLAG_ENABLE_HASH: u32 = 1u32;
pub const NDIS_RECEIVE_HASH_FLAG_HASH_INFO_UNCHANGED: u32 = 2u32;
pub const NDIS_RECEIVE_HASH_FLAG_HASH_KEY_UNCHANGED: u32 = 4u32;
pub const NDIS_RECEIVE_HASH_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_RECEIVE_QUEUE_ALLOCATION_COMPLETE_ARRAY_REVISION_1: u32 = 1u32;
pub const NDIS_RECEIVE_QUEUE_ALLOCATION_COMPLETE_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_RECEIVE_QUEUE_FREE_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_RECEIVE_QUEUE_INFO_ARRAY_REVISION_1: u32 = 1u32;
pub const NDIS_RECEIVE_QUEUE_INFO_REVISION_1: u32 = 1u32;
pub const NDIS_RECEIVE_QUEUE_INFO_REVISION_2: u32 = 2u32;
pub const NDIS_RECEIVE_QUEUE_PARAMETERS_CHANGE_MASK: u32 = 4294901760u32;
pub const NDIS_RECEIVE_QUEUE_PARAMETERS_FLAGS_CHANGED: u32 = 65536u32;
pub const NDIS_RECEIVE_QUEUE_PARAMETERS_INTERRUPT_COALESCING_DOMAIN_ID_CHANGED: u32 = 1048576u32;
pub const NDIS_RECEIVE_QUEUE_PARAMETERS_LOOKAHEAD_SPLIT_REQUIRED: u32 = 2u32;
pub const NDIS_RECEIVE_QUEUE_PARAMETERS_NAME_CHANGED: u32 = 524288u32;
pub const NDIS_RECEIVE_QUEUE_PARAMETERS_PER_QUEUE_RECEIVE_INDICATION: u32 = 1u32;
pub const NDIS_RECEIVE_QUEUE_PARAMETERS_PROCESSOR_AFFINITY_CHANGED: u32 = 131072u32;
pub const NDIS_RECEIVE_QUEUE_PARAMETERS_QOS_SQ_ID_CHANGED: u32 = 2097152u32;
pub const NDIS_RECEIVE_QUEUE_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_RECEIVE_QUEUE_PARAMETERS_REVISION_2: u32 = 2u32;
pub const NDIS_RECEIVE_QUEUE_PARAMETERS_REVISION_3: u32 = 3u32;
pub const NDIS_RECEIVE_QUEUE_PARAMETERS_SUGGESTED_RECV_BUFFER_NUMBERS_CHANGED: u32 = 262144u32;
pub const NDIS_RECEIVE_QUEUE_STATE_REVISION_1: u32 = 1u32;
pub const NDIS_RECEIVE_SCALE_CAPABILITIES_REVISION_1: u32 = 1u32;
pub const NDIS_RECEIVE_SCALE_CAPABILITIES_REVISION_2: u32 = 2u32;
pub const NDIS_RECEIVE_SCALE_CAPABILITIES_REVISION_3: u32 = 3u32;
pub const NDIS_RECEIVE_SCALE_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_RECEIVE_SCALE_PARAMETERS_REVISION_2: u32 = 2u32;
pub const NDIS_RECEIVE_SCALE_PARAMETERS_REVISION_3: u32 = 3u32;
pub const NDIS_RECEIVE_SCALE_PARAMETERS_V2_REVISION_1: u32 = 1u32;
pub const NDIS_RECEIVE_SCALE_PARAM_ENABLE_RSS: u32 = 1u32;
pub const NDIS_RECEIVE_SCALE_PARAM_HASH_INFO_CHANGED: u32 = 2u32;
pub const NDIS_RECEIVE_SCALE_PARAM_HASH_KEY_CHANGED: u32 = 4u32;
pub const NDIS_RECEIVE_SCALE_PARAM_NUMBER_OF_ENTRIES_CHANGED: u32 = 16u32;
pub const NDIS_RECEIVE_SCALE_PARAM_NUMBER_OF_QUEUES_CHANGED: u32 = 8u32;
pub const NDIS_RESTART_GENERAL_ATTRIBUTES_MAX_LOOKAHEAD_ACCESSED_DEFINED: u32 = 1u32;
pub const NDIS_RESTART_GENERAL_ATTRIBUTES_REVISION_1: u32 = 1u32;
pub const NDIS_RESTART_GENERAL_ATTRIBUTES_REVISION_2: u32 = 2u32;
pub const NDIS_RETURN_FLAGS_DISPATCH_LEVEL: u32 = 1u32;
pub const NDIS_RETURN_FLAGS_SINGLE_QUEUE: u32 = 2u32;
pub const NDIS_RETURN_FLAGS_SWITCH_SINGLE_SOURCE: u32 = 4u32;
pub const NDIS_RING_AUTO_REMOVAL_ERROR: u32 = 1024u32;
pub const NDIS_RING_COUNTER_OVERFLOW: u32 = 256u32;
pub const NDIS_RING_HARD_ERROR: u32 = 16384u32;
pub const NDIS_RING_LOBE_WIRE_FAULT: u32 = 2048u32;
pub const NDIS_RING_REMOVE_RECEIVED: u32 = 512u32;
pub const NDIS_RING_RING_RECOVERY: u32 = 64u32;
pub const NDIS_RING_SIGNAL_LOSS: u32 = 32768u32;
pub const NDIS_RING_SINGLE_STATION: u32 = 128u32;
pub const NDIS_RING_SOFT_ERROR: u32 = 8192u32;
pub const NDIS_RING_TRANSMIT_BEACON: u32 = 4096u32;
pub const NDIS_ROUTING_DOMAIN_ENTRY_REVISION_1: u32 = 1u32;
pub const NDIS_ROUTING_DOMAIN_ISOLATION_ENTRY_REVISION_1: u32 = 1u32;
pub const NDIS_RSC_STATISTICS_REVISION_1: u32 = 1u32;
pub const NDIS_RSS_CAPS_CLASSIFICATION_AT_DPC: u32 = 67108864u32;
pub const NDIS_RSS_CAPS_CLASSIFICATION_AT_ISR: u32 = 33554432u32;
pub const NDIS_RSS_CAPS_HASH_TYPE_TCP_IPV4: u32 = 256u32;
pub const NDIS_RSS_CAPS_HASH_TYPE_TCP_IPV6: u32 = 512u32;
pub const NDIS_RSS_CAPS_HASH_TYPE_TCP_IPV6_EX: u32 = 1024u32;
pub const NDIS_RSS_CAPS_HASH_TYPE_UDP_IPV4: u32 = 2048u32;
pub const NDIS_RSS_CAPS_HASH_TYPE_UDP_IPV6: u32 = 4096u32;
pub const NDIS_RSS_CAPS_HASH_TYPE_UDP_IPV6_EX: u32 = 8192u32;
pub const NDIS_RSS_CAPS_MESSAGE_SIGNALED_INTERRUPTS: u32 = 16777216u32;
pub const NDIS_RSS_CAPS_RSS_AVAILABLE_ON_PORTS: u32 = 268435456u32;
pub const NDIS_RSS_CAPS_SUPPORTS_INDEPENDENT_ENTRY_MOVE: u32 = 1073741824u32;
pub const NDIS_RSS_CAPS_SUPPORTS_MSI_X: u32 = 536870912u32;
pub const NDIS_RSS_CAPS_USING_MSI_X: u32 = 134217728u32;
pub const NDIS_RSS_HASH_SECRET_KEY_MAX_SIZE_REVISION_1: u32 = 40u32;
pub const NDIS_RSS_HASH_SECRET_KEY_MAX_SIZE_REVISION_2: u32 = 40u32;
pub const NDIS_RSS_HASH_SECRET_KEY_MAX_SIZE_REVISION_3: u32 = 40u32;
pub const NDIS_RSS_HASH_SECRET_KEY_SIZE_REVISION_1: u32 = 40u32;
pub const NDIS_RSS_INDIRECTION_TABLE_MAX_SIZE_REVISION_1: u32 = 128u32;
pub const NDIS_RSS_INDIRECTION_TABLE_SIZE_REVISION_1: u32 = 128u32;
pub const NDIS_RSS_PARAM_FLAG_BASE_CPU_UNCHANGED: u32 = 1u32;
pub const NDIS_RSS_PARAM_FLAG_DEFAULT_PROCESSOR_UNCHANGED: u32 = 32u32;
pub const NDIS_RSS_PARAM_FLAG_DISABLE_RSS: u32 = 16u32;
pub const NDIS_RSS_PARAM_FLAG_HASH_INFO_UNCHANGED: u32 = 2u32;
pub const NDIS_RSS_PARAM_FLAG_HASH_KEY_UNCHANGED: u32 = 8u32;
pub const NDIS_RSS_PARAM_FLAG_ITABLE_UNCHANGED: u32 = 4u32;
pub const NDIS_RSS_PROCESSOR_INFO_REVISION_1: u32 = 1u32;
pub const NDIS_RSS_PROCESSOR_INFO_REVISION_2: u32 = 2u32;
pub const NDIS_RSS_SET_INDIRECTION_ENTRIES_REVISION_1: u32 = 1u32;
pub const NDIS_RSS_SET_INDIRECTION_ENTRY_FLAG_DEFAULT_PROCESSOR: u32 = 2u32;
pub const NDIS_RSS_SET_INDIRECTION_ENTRY_FLAG_PRIMARY_PROCESSOR: u32 = 1u32;
pub const NDIS_RUNTIME_VERSION_60: u32 = 393216u32;
pub const NDIS_RWL_AT_DISPATCH_LEVEL: u32 = 1u32;
pub const NDIS_SCATTER_GATHER_LIST_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_SEND_COMPLETE_FLAGS_DISPATCH_LEVEL: u32 = 1u32;
pub const NDIS_SEND_COMPLETE_FLAGS_SINGLE_QUEUE: u32 = 2u32;
pub const NDIS_SEND_COMPLETE_FLAGS_SWITCH_SINGLE_SOURCE: u32 = 4u32;
pub const NDIS_SEND_FLAGS_CHECK_FOR_LOOPBACK: u32 = 2u32;
pub const NDIS_SEND_FLAGS_DISPATCH_LEVEL: u32 = 1u32;
pub const NDIS_SEND_FLAGS_SINGLE_QUEUE: u32 = 4u32;
pub const NDIS_SEND_FLAGS_SWITCH_DESTINATION_GROUP: u32 = 16u32;
pub const NDIS_SEND_FLAGS_SWITCH_SINGLE_SOURCE: u32 = 32u32;
pub const NDIS_SG_DMA_64_BIT_ADDRESS: u32 = 1u32;
pub const NDIS_SG_DMA_DESCRIPTION_REVISION_1: u32 = 1u32;
pub const NDIS_SG_DMA_DESCRIPTION_REVISION_2: u32 = 2u32;
pub const NDIS_SG_DMA_HYBRID_DMA: u32 = 4u32;
pub const NDIS_SG_DMA_V3_HAL_API: u32 = 2u32;
pub const NDIS_SG_LIST_WRITE_TO_DEVICE: u64 = 1u64;
pub const NDIS_SHARED_MEMORY_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_SHARED_MEMORY_PARAMETERS_REVISION_2: u32 = 2u32;
pub const NDIS_SHARED_MEMORY_PROVIDER_CHARACTERISTICS_REVISION_1: u32 = 1u32;
pub const NDIS_SHARED_MEMORY_PROVIDER_CHAR_SUPPORTS_PF_VPORTS: u32 = 1u32;
pub const NDIS_SHARED_MEM_PARAMETERS_CONTIGOUS: u32 = 1u32;
pub const NDIS_SHARED_MEM_PARAMETERS_CONTIGUOUS: u32 = 1u32;
pub const NDIS_SIZEOF_NDIS_PM_PROTOCOL_OFFLOAD_REVISION_1: u32 = 240u32;
pub const NDIS_SRIOV_BAR_RESOURCES_INFO_REVISION_1: u32 = 1u32;
pub const NDIS_SRIOV_CAPABILITIES_REVISION_1: u32 = 1u32;
pub const NDIS_SRIOV_CAPS_PF_MINIPORT: u32 = 2u32;
pub const NDIS_SRIOV_CAPS_SRIOV_SUPPORTED: u32 = 1u32;
pub const NDIS_SRIOV_CAPS_VF_MINIPORT: u32 = 4u32;
pub const NDIS_SRIOV_CONFIG_STATE_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_SRIOV_OVERLYING_ADAPTER_INFO_VERSION_1: u32 = 1u32;
pub const NDIS_SRIOV_PF_LUID_INFO_REVISION_1: u32 = 1u32;
pub const NDIS_SRIOV_PROBED_BARS_INFO_REVISION_1: u32 = 1u32;
pub const NDIS_SRIOV_READ_VF_CONFIG_BLOCK_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_SRIOV_READ_VF_CONFIG_SPACE_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_SRIOV_RESET_VF_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_SRIOV_SET_VF_POWER_STATE_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_SRIOV_VF_INVALIDATE_CONFIG_BLOCK_INFO_REVISION_1: u32 = 1u32;
pub const NDIS_SRIOV_VF_SERIAL_NUMBER_INFO_REVISION_1: u32 = 1u32;
pub const NDIS_SRIOV_VF_VENDOR_DEVICE_ID_INFO_REVISION_1: u32 = 1u32;
pub const NDIS_SRIOV_WRITE_VF_CONFIG_BLOCK_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_SRIOV_WRITE_VF_CONFIG_SPACE_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_STATISTICS_BROADCAST_BYTES_RCV_SUPPORTED: u32 = 32768u32;
pub const NDIS_STATISTICS_BROADCAST_BYTES_XMIT_SUPPORTED: u32 = 512u32;
pub const NDIS_STATISTICS_BROADCAST_FRAMES_RCV_SUPPORTED: u32 = 65536u32;
pub const NDIS_STATISTICS_BROADCAST_FRAMES_XMIT_SUPPORTED: u32 = 1024u32;
pub const NDIS_STATISTICS_BYTES_RCV_SUPPORTED: u32 = 524288u32;
pub const NDIS_STATISTICS_BYTES_XMIT_SUPPORTED: u32 = 1048576u32;
pub const NDIS_STATISTICS_DIRECTED_BYTES_RCV_SUPPORTED: u32 = 2048u32;
pub const NDIS_STATISTICS_DIRECTED_BYTES_XMIT_SUPPORTED: u32 = 32u32;
pub const NDIS_STATISTICS_DIRECTED_FRAMES_RCV_SUPPORTED: u32 = 4096u32;
pub const NDIS_STATISTICS_DIRECTED_FRAMES_XMIT_SUPPORTED: u32 = 64u32;
pub const NDIS_STATISTICS_FLAGS_VALID_BROADCAST_BYTES_RCV: u32 = 262144u32;
pub const NDIS_STATISTICS_FLAGS_VALID_BROADCAST_BYTES_XMIT: u32 = 2097152u32;
pub const NDIS_STATISTICS_FLAGS_VALID_BROADCAST_FRAMES_RCV: u32 = 4u32;
pub const NDIS_STATISTICS_FLAGS_VALID_BROADCAST_FRAMES_XMIT: u32 = 256u32;
pub const NDIS_STATISTICS_FLAGS_VALID_BYTES_RCV: u32 = 8u32;
pub const NDIS_STATISTICS_FLAGS_VALID_BYTES_XMIT: u32 = 512u32;
pub const NDIS_STATISTICS_FLAGS_VALID_DIRECTED_BYTES_RCV: u32 = 65536u32;
pub const NDIS_STATISTICS_FLAGS_VALID_DIRECTED_BYTES_XMIT: u32 = 524288u32;
pub const NDIS_STATISTICS_FLAGS_VALID_DIRECTED_FRAMES_RCV: u32 = 1u32;
pub const NDIS_STATISTICS_FLAGS_VALID_DIRECTED_FRAMES_XMIT: u32 = 64u32;
pub const NDIS_STATISTICS_FLAGS_VALID_MULTICAST_BYTES_RCV: u32 = 131072u32;
pub const NDIS_STATISTICS_FLAGS_VALID_MULTICAST_BYTES_XMIT: u32 = 1048576u32;
pub const NDIS_STATISTICS_FLAGS_VALID_MULTICAST_FRAMES_RCV: u32 = 2u32;
pub const NDIS_STATISTICS_FLAGS_VALID_MULTICAST_FRAMES_XMIT: u32 = 128u32;
pub const NDIS_STATISTICS_FLAGS_VALID_RCV_DISCARDS: u32 = 16u32;
pub const NDIS_STATISTICS_FLAGS_VALID_RCV_ERROR: u32 = 32u32;
pub const NDIS_STATISTICS_FLAGS_VALID_XMIT_DISCARDS: u32 = 32768u32;
pub const NDIS_STATISTICS_FLAGS_VALID_XMIT_ERROR: u32 = 1024u32;
pub const NDIS_STATISTICS_GEN_STATISTICS_SUPPORTED: u32 = 4194304u32;
pub const NDIS_STATISTICS_INFO_REVISION_1: u32 = 1u32;
pub const NDIS_STATISTICS_MULTICAST_BYTES_RCV_SUPPORTED: u32 = 8192u32;
pub const NDIS_STATISTICS_MULTICAST_BYTES_XMIT_SUPPORTED: u32 = 128u32;
pub const NDIS_STATISTICS_MULTICAST_FRAMES_RCV_SUPPORTED: u32 = 16384u32;
pub const NDIS_STATISTICS_MULTICAST_FRAMES_XMIT_SUPPORTED: u32 = 256u32;
pub const NDIS_STATISTICS_RCV_CRC_ERROR_SUPPORTED: u32 = 131072u32;
pub const NDIS_STATISTICS_RCV_DISCARDS_SUPPORTED: u32 = 2097152u32;
pub const NDIS_STATISTICS_RCV_ERROR_SUPPORTED: u32 = 8u32;
pub const NDIS_STATISTICS_RCV_NO_BUFFER_SUPPORTED: u32 = 16u32;
pub const NDIS_STATISTICS_RCV_OK_SUPPORTED: u32 = 2u32;
pub const NDIS_STATISTICS_TRANSMIT_QUEUE_LENGTH_SUPPORTED: u32 = 262144u32;
pub const NDIS_STATISTICS_XMIT_DISCARDS_SUPPORTED: u32 = 134217728u32;
pub const NDIS_STATISTICS_XMIT_ERROR_SUPPORTED: u32 = 4u32;
pub const NDIS_STATISTICS_XMIT_OK_SUPPORTED: u32 = 1u32;
pub const NDIS_STATUS_INDICATION_FLAGS_MEDIA_CONNECT_TO_CONNECT: u32 = 4096u32;
pub const NDIS_STATUS_INDICATION_FLAGS_NDIS_RESERVED: u32 = 4095u32;
pub const NDIS_STATUS_INDICATION_REVISION_1: u32 = 1u32;
pub const NDIS_SUPPORT_60_COMPATIBLE_API: u32 = 1u32;
pub const NDIS_SUPPORT_NDIS6: u32 = 1u32;
pub const NDIS_SUPPORT_NDIS61: u32 = 1u32;
pub const NDIS_SUPPORT_NDIS620: u32 = 1u32;
pub const NDIS_SUPPORT_NDIS630: u32 = 1u32;
pub const NDIS_SUPPORT_NDIS640: u32 = 1u32;
pub const NDIS_SUPPORT_NDIS650: u32 = 1u32;
pub const NDIS_SUPPORT_NDIS651: u32 = 1u32;
pub const NDIS_SUPPORT_NDIS660: u32 = 1u32;
pub const NDIS_SUPPORT_NDIS670: u32 = 1u32;
pub const NDIS_SUPPORT_NDIS680: u32 = 1u32;
pub const NDIS_SUPPORT_NDIS681: u32 = 1u32;
pub const NDIS_SUPPORT_NDIS682: u32 = 1u32;
pub const NDIS_SUPPORT_NDIS683: u32 = 1u32;
pub const NDIS_SUPPORT_NDIS684: u32 = 1u32;
pub const NDIS_SUPPORT_NDIS685: u32 = 1u32;
pub const NDIS_SUPPORT_NDIS686: u32 = 1u32;
pub const NDIS_SUPPORT_NDIS687: u32 = 1u32;
pub const NDIS_SWITCH_COPY_NBL_INFO_FLAGS_PRESERVE_DESTINATIONS: u32 = 1u32;
pub const NDIS_SWITCH_COPY_NBL_INFO_FLAGS_PRESERVE_SWITCH_INFO_ONLY: u32 = 2u32;
pub const NDIS_SWITCH_DEFAULT_NIC_INDEX: u32 = 0u32;
pub const NDIS_SWITCH_DEFAULT_PORT_ID: u32 = 0u32;
pub const NDIS_SWITCH_FEATURE_STATUS_CUSTOM_REVISION_1: u32 = 1u32;
pub const NDIS_SWITCH_FEATURE_STATUS_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_SWITCH_FORWARDING_DESTINATION_ARRAY_REVISION_1: u32 = 1u32;
pub const NDIS_SWITCH_NET_BUFFER_LIST_CONTEXT_TYPE_INFO_REVISION_1: u32 = 1u32;
pub const NDIS_SWITCH_NIC_ARRAY_REVISION_1: u32 = 1u32;
pub const NDIS_SWITCH_NIC_FLAGS_MAPPED_NIC_UPDATED: u32 = 4u32;
pub const NDIS_SWITCH_NIC_FLAGS_NIC_INITIALIZING: u32 = 1u32;
pub const NDIS_SWITCH_NIC_FLAGS_NIC_SUSPENDED: u32 = 2u32;
pub const NDIS_SWITCH_NIC_FLAGS_NIC_SUSPENDED_LM: u32 = 16u32;
pub const NDIS_SWITCH_NIC_OID_REQUEST_REVISION_1: u32 = 1u32;
pub const NDIS_SWITCH_NIC_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_SWITCH_NIC_PARAMETERS_REVISION_2: u32 = 2u32;
pub const NDIS_SWITCH_NIC_SAVE_STATE_REVISION_1: u32 = 1u32;
pub const NDIS_SWITCH_NIC_SAVE_STATE_REVISION_2: u32 = 2u32;
pub const NDIS_SWITCH_NIC_STATUS_INDICATION_REVISION_1: u32 = 1u32;
pub const NDIS_SWITCH_OBJECT_SERIALIZATION_VERSION_1: u32 = 1u32;
pub const NDIS_SWITCH_OPTIONAL_HANDLERS_PD_RESERVED_SIZE: u32 = 14u32;
pub const NDIS_SWITCH_OPTIONAL_HANDLERS_REVISION_1: u32 = 1u32;
pub const NDIS_SWITCH_OPTIONAL_HANDLERS_REVISION_2: u32 = 2u32;
pub const NDIS_SWITCH_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_SWITCH_PORT_ARRAY_REVISION_1: u32 = 1u32;
pub const NDIS_SWITCH_PORT_FEATURE_STATUS_CUSTOM_REVISION_1: u32 = 1u32;
pub const NDIS_SWITCH_PORT_FEATURE_STATUS_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_SWITCH_PORT_PARAMETERS_FLAG_RESTORING_PORT: u32 = 2u32;
pub const NDIS_SWITCH_PORT_PARAMETERS_FLAG_UNTRUSTED_INTERNAL_PORT: u32 = 1u32;
pub const NDIS_SWITCH_PORT_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_SWITCH_PORT_PROPERTY_CUSTOM_REVISION_1: u32 = 1u32;
pub const NDIS_SWITCH_PORT_PROPERTY_DELETE_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_SWITCH_PORT_PROPERTY_ENUM_INFO_REVISION_1: u32 = 1u32;
pub const NDIS_SWITCH_PORT_PROPERTY_ENUM_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_SWITCH_PORT_PROPERTY_ISOLATION_REVISION_1: u32 = 1u32;
pub const NDIS_SWITCH_PORT_PROPERTY_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_SWITCH_PORT_PROPERTY_PROFILE_REVISION_1: u32 = 1u32;
pub const NDIS_SWITCH_PORT_PROPERTY_ROUTING_DOMAIN_REVISION_1: u32 = 1u32;
pub const NDIS_SWITCH_PORT_PROPERTY_SECURITY_REVISION_1: u32 = 1u32;
pub const NDIS_SWITCH_PORT_PROPERTY_SECURITY_REVISION_2: u32 = 2u32;
pub const NDIS_SWITCH_PORT_PROPERTY_VLAN_REVISION_1: u32 = 1u32;
pub const NDIS_SWITCH_PROPERTY_CUSTOM_REVISION_1: u32 = 1u32;
pub const NDIS_SWITCH_PROPERTY_DELETE_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_SWITCH_PROPERTY_ENUM_INFO_REVISION_1: u32 = 1u32;
pub const NDIS_SWITCH_PROPERTY_ENUM_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_SWITCH_PROPERTY_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_SWITCH_REPORT_FILTERED_NBL_FLAGS_IS_INCOMING: u32 = 1u32;
pub const NDIS_SYSTEM_PROCESSOR_INFO_EX_REVISION_1: u32 = 1u32;
pub const NDIS_SYSTEM_PROCESSOR_INFO_REVISION_1: u32 = 1u32;
pub const NDIS_TASK_OFFLOAD_VERSION: u32 = 1u32;
pub const NDIS_TASK_TCP_LARGE_SEND_V0: u32 = 0u32;
pub const NDIS_TCP_CONNECTION_OFFLOAD_REVISION_1: u32 = 1u32;
pub const NDIS_TCP_CONNECTION_OFFLOAD_REVISION_2: u32 = 2u32;
pub const NDIS_TCP_LARGE_SEND_OFFLOAD_IPv4: u32 = 0u32;
pub const NDIS_TCP_LARGE_SEND_OFFLOAD_IPv6: u32 = 1u32;
pub const NDIS_TCP_LARGE_SEND_OFFLOAD_V1_TYPE: u32 = 0u32;
pub const NDIS_TCP_LARGE_SEND_OFFLOAD_V2_TYPE: u32 = 1u32;
pub const NDIS_TCP_RECV_SEG_COALESC_OFFLOAD_REVISION_1: u32 = 1u32;
pub const NDIS_TIMEOUT_DPC_REQUEST_CAPABILITIES_REVISION_1: u32 = 1u32;
pub const NDIS_TIMER_CHARACTERISTICS_REVISION_1: u32 = 1u32;
pub const NDIS_TIMESTAMP_CAPABILITIES_REVISION_1: u32 = 1u32;
pub const NDIS_UDP_SEGMENTATION_OFFLOAD_IPV4: u32 = 0u32;
pub const NDIS_UDP_SEGMENTATION_OFFLOAD_IPV6: u32 = 1u32;
pub const NDIS_WDF: u32 = 1u32;
pub const NDIS_WDM: u32 = 0u32;
pub const NDIS_WDM_DRIVER: u32 = 2u32;
pub const NDIS_WLAN_WAKE_ON_4WAY_HANDSHAKE_REQUEST_ENABLED: u32 = 8u32;
pub const NDIS_WLAN_WAKE_ON_4WAY_HANDSHAKE_REQUEST_SUPPORTED: u32 = 8u32;
pub const NDIS_WLAN_WAKE_ON_AP_ASSOCIATION_LOST_ENABLED: u32 = 2u32;
pub const NDIS_WLAN_WAKE_ON_AP_ASSOCIATION_LOST_SUPPORTED: u32 = 2u32;
pub const NDIS_WLAN_WAKE_ON_GTK_HANDSHAKE_ERROR_ENABLED: u32 = 4u32;
pub const NDIS_WLAN_WAKE_ON_GTK_HANDSHAKE_ERROR_SUPPORTED: u32 = 4u32;
pub const NDIS_WLAN_WAKE_ON_NLO_DISCOVERY_ENABLED: u32 = 1u32;
pub const NDIS_WLAN_WAKE_ON_NLO_DISCOVERY_SUPPORTED: u32 = 1u32;
pub const NDIS_WMI_DEFAULT_METHOD_ID: u32 = 1u32;
pub const NDIS_WMI_ENUM_ADAPTER_REVISION_1: u32 = 1u32;
pub const NDIS_WMI_EVENT_HEADER_REVISION_1: u32 = 1u32;
pub const NDIS_WMI_METHOD_HEADER_REVISION_1: u32 = 1u32;
pub const NDIS_WMI_OBJECT_TYPE_ENUM_ADAPTER: u32 = 4u32;
pub const NDIS_WMI_OBJECT_TYPE_EVENT: u32 = 3u32;
pub const NDIS_WMI_OBJECT_TYPE_METHOD: u32 = 2u32;
pub const NDIS_WMI_OBJECT_TYPE_OUTPUT_INFO: u32 = 5u32;
pub const NDIS_WMI_OBJECT_TYPE_SET: u32 = 1u32;
pub const NDIS_WMI_PM_ACTIVE_CAPABILITIES_REVISION_1: u32 = 1u32;
pub const NDIS_WMI_PM_ADMIN_CONFIG_REVISION_1: u32 = 1u32;
pub const NDIS_WMI_RECEIVE_QUEUE_INFO_REVISION_1: u32 = 1u32;
pub const NDIS_WMI_RECEIVE_QUEUE_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NDIS_WMI_SET_HEADER_REVISION_1: u32 = 1u32;
pub const NDIS_WWAN_WAKE_ON_PACKET_STATE_ENABLED: u32 = 8u32;
pub const NDIS_WWAN_WAKE_ON_PACKET_STATE_SUPPORTED: u32 = 8u32;
pub const NDIS_WWAN_WAKE_ON_REGISTER_STATE_ENABLED: u32 = 1u32;
pub const NDIS_WWAN_WAKE_ON_REGISTER_STATE_SUPPORTED: u32 = 1u32;
pub const NDIS_WWAN_WAKE_ON_SMS_RECEIVE_ENABLED: u32 = 2u32;
pub const NDIS_WWAN_WAKE_ON_SMS_RECEIVE_SUPPORTED: u32 = 2u32;
pub const NDIS_WWAN_WAKE_ON_UICC_CHANGE_ENABLED: u32 = 16u32;
pub const NDIS_WWAN_WAKE_ON_UICC_CHANGE_SUPPORTED: u32 = 16u32;
pub const NDIS_WWAN_WAKE_ON_USSD_RECEIVE_ENABLED: u32 = 4u32;
pub const NDIS_WWAN_WAKE_ON_USSD_RECEIVE_SUPPORTED: u32 = 4u32;
pub const NET_BUFFER_LIST_POOL_FLAG_VERIFY: u32 = 1u32;
pub const NET_BUFFER_LIST_POOL_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NET_BUFFER_LIST_POOL_PARAMETERS_REVISION_2: u32 = 2u32;
pub const NET_BUFFER_POOL_FLAG_VERIFY: u32 = 1u32;
pub const NET_BUFFER_POOL_PARAMETERS_REVISION_1: u32 = 1u32;
pub const NET_BUFFER_POOL_PARAMETERS_REVISION_2: u32 = 2u32;
pub const NET_DEVICE_PNP_EVENT_REVISION_1: u32 = 1u32;
pub const NET_EVENT_FLAGS_VPORT_ID_VALID: u32 = 2u32;
pub const NET_EVENT_HALT_MINIPORT_ON_LOW_POWER: u32 = 1u32;
pub const NET_PNP_EVENT_NOTIFICATION_REVISION_1: u32 = 1u32;
pub const NET_PNP_EVENT_NOTIFICATION_REVISION_2: u32 = 2u32;
pub const Ndis802_11AuthModeAutoSwitch: NDIS_802_11_AUTHENTICATION_MODE = NDIS_802_11_AUTHENTICATION_MODE(2i32);
pub const Ndis802_11AuthModeMax: NDIS_802_11_AUTHENTICATION_MODE = NDIS_802_11_AUTHENTICATION_MODE(11i32);
pub const Ndis802_11AuthModeOpen: NDIS_802_11_AUTHENTICATION_MODE = NDIS_802_11_AUTHENTICATION_MODE(0i32);
pub const Ndis802_11AuthModeShared: NDIS_802_11_AUTHENTICATION_MODE = NDIS_802_11_AUTHENTICATION_MODE(1i32);
pub const Ndis802_11AuthModeWPA: NDIS_802_11_AUTHENTICATION_MODE = NDIS_802_11_AUTHENTICATION_MODE(3i32);
pub const Ndis802_11AuthModeWPA2: NDIS_802_11_AUTHENTICATION_MODE = NDIS_802_11_AUTHENTICATION_MODE(6i32);
pub const Ndis802_11AuthModeWPA2PSK: NDIS_802_11_AUTHENTICATION_MODE = NDIS_802_11_AUTHENTICATION_MODE(7i32);
pub const Ndis802_11AuthModeWPA3: NDIS_802_11_AUTHENTICATION_MODE = NDIS_802_11_AUTHENTICATION_MODE(8i32);
pub const Ndis802_11AuthModeWPA3Ent: NDIS_802_11_AUTHENTICATION_MODE = NDIS_802_11_AUTHENTICATION_MODE(10i32);
pub const Ndis802_11AuthModeWPA3Ent192: NDIS_802_11_AUTHENTICATION_MODE = NDIS_802_11_AUTHENTICATION_MODE(8i32);
pub const Ndis802_11AuthModeWPA3SAE: NDIS_802_11_AUTHENTICATION_MODE = NDIS_802_11_AUTHENTICATION_MODE(9i32);
pub const Ndis802_11AuthModeWPANone: NDIS_802_11_AUTHENTICATION_MODE = NDIS_802_11_AUTHENTICATION_MODE(5i32);
pub const Ndis802_11AuthModeWPAPSK: NDIS_802_11_AUTHENTICATION_MODE = NDIS_802_11_AUTHENTICATION_MODE(4i32);
pub const Ndis802_11AutoUnknown: NDIS_802_11_NETWORK_INFRASTRUCTURE = NDIS_802_11_NETWORK_INFRASTRUCTURE(2i32);
pub const Ndis802_11Automode: NDIS_802_11_NETWORK_TYPE = NDIS_802_11_NETWORK_TYPE(4i32);
pub const Ndis802_11DS: NDIS_802_11_NETWORK_TYPE = NDIS_802_11_NETWORK_TYPE(1i32);
pub const Ndis802_11Encryption1Enabled: NDIS_802_11_WEP_STATUS = NDIS_802_11_WEP_STATUS(0i32);
pub const Ndis802_11Encryption1KeyAbsent: NDIS_802_11_WEP_STATUS = NDIS_802_11_WEP_STATUS(2i32);
pub const Ndis802_11Encryption2Enabled: NDIS_802_11_WEP_STATUS = NDIS_802_11_WEP_STATUS(4i32);
pub const Ndis802_11Encryption2KeyAbsent: NDIS_802_11_WEP_STATUS = NDIS_802_11_WEP_STATUS(5i32);
pub const Ndis802_11Encryption3Enabled: NDIS_802_11_WEP_STATUS = NDIS_802_11_WEP_STATUS(6i32);
pub const Ndis802_11Encryption3KeyAbsent: NDIS_802_11_WEP_STATUS = NDIS_802_11_WEP_STATUS(7i32);
pub const Ndis802_11EncryptionDisabled: NDIS_802_11_WEP_STATUS = NDIS_802_11_WEP_STATUS(1i32);
pub const Ndis802_11EncryptionNotSupported: NDIS_802_11_WEP_STATUS = NDIS_802_11_WEP_STATUS(3i32);
pub const Ndis802_11FH: NDIS_802_11_NETWORK_TYPE = NDIS_802_11_NETWORK_TYPE(0i32);
pub const Ndis802_11IBSS: NDIS_802_11_NETWORK_INFRASTRUCTURE = NDIS_802_11_NETWORK_INFRASTRUCTURE(0i32);
pub const Ndis802_11Infrastructure: NDIS_802_11_NETWORK_INFRASTRUCTURE = NDIS_802_11_NETWORK_INFRASTRUCTURE(1i32);
pub const Ndis802_11InfrastructureMax: NDIS_802_11_NETWORK_INFRASTRUCTURE = NDIS_802_11_NETWORK_INFRASTRUCTURE(3i32);
pub const Ndis802_11MediaStreamOff: NDIS_802_11_MEDIA_STREAM_MODE = NDIS_802_11_MEDIA_STREAM_MODE(0i32);
pub const Ndis802_11MediaStreamOn: NDIS_802_11_MEDIA_STREAM_MODE = NDIS_802_11_MEDIA_STREAM_MODE(1i32);
pub const Ndis802_11NetworkTypeMax: NDIS_802_11_NETWORK_TYPE = NDIS_802_11_NETWORK_TYPE(5i32);
pub const Ndis802_11OFDM24: NDIS_802_11_NETWORK_TYPE = NDIS_802_11_NETWORK_TYPE(3i32);
pub const Ndis802_11OFDM5: NDIS_802_11_NETWORK_TYPE = NDIS_802_11_NETWORK_TYPE(2i32);
pub const Ndis802_11PowerModeCAM: NDIS_802_11_POWER_MODE = NDIS_802_11_POWER_MODE(0i32);
pub const Ndis802_11PowerModeFast_PSP: NDIS_802_11_POWER_MODE = NDIS_802_11_POWER_MODE(2i32);
pub const Ndis802_11PowerModeMAX_PSP: NDIS_802_11_POWER_MODE = NDIS_802_11_POWER_MODE(1i32);
pub const Ndis802_11PowerModeMax: NDIS_802_11_POWER_MODE = NDIS_802_11_POWER_MODE(3i32);
pub const Ndis802_11PrivFilter8021xWEP: NDIS_802_11_PRIVACY_FILTER = NDIS_802_11_PRIVACY_FILTER(1i32);
pub const Ndis802_11PrivFilterAcceptAll: NDIS_802_11_PRIVACY_FILTER = NDIS_802_11_PRIVACY_FILTER(0i32);
pub const Ndis802_11RadioStatusHardwareOff: NDIS_802_11_RADIO_STATUS = NDIS_802_11_RADIO_STATUS(1i32);
pub const Ndis802_11RadioStatusHardwareSoftwareOff: NDIS_802_11_RADIO_STATUS = NDIS_802_11_RADIO_STATUS(3i32);
pub const Ndis802_11RadioStatusMax: NDIS_802_11_RADIO_STATUS = NDIS_802_11_RADIO_STATUS(4i32);
pub const Ndis802_11RadioStatusOn: NDIS_802_11_RADIO_STATUS = NDIS_802_11_RADIO_STATUS(0i32);
pub const Ndis802_11RadioStatusSoftwareOff: NDIS_802_11_RADIO_STATUS = NDIS_802_11_RADIO_STATUS(2i32);
pub const Ndis802_11ReloadWEPKeys: NDIS_802_11_RELOAD_DEFAULTS = NDIS_802_11_RELOAD_DEFAULTS(0i32);
pub const Ndis802_11StatusTypeMax: NDIS_802_11_STATUS_TYPE = NDIS_802_11_STATUS_TYPE(3i32);
pub const Ndis802_11StatusType_Authentication: NDIS_802_11_STATUS_TYPE = NDIS_802_11_STATUS_TYPE(0i32);
pub const Ndis802_11StatusType_MediaStreamMode: NDIS_802_11_STATUS_TYPE = NDIS_802_11_STATUS_TYPE(1i32);
pub const Ndis802_11StatusType_PMKID_CandidateList: NDIS_802_11_STATUS_TYPE = NDIS_802_11_STATUS_TYPE(2i32);
pub const Ndis802_11WEPDisabled: NDIS_802_11_WEP_STATUS = NDIS_802_11_WEP_STATUS(1i32);
pub const Ndis802_11WEPEnabled: NDIS_802_11_WEP_STATUS = NDIS_802_11_WEP_STATUS(0i32);
pub const Ndis802_11WEPKeyAbsent: NDIS_802_11_WEP_STATUS = NDIS_802_11_WEP_STATUS(2i32);
pub const Ndis802_11WEPNotSupported: NDIS_802_11_WEP_STATUS = NDIS_802_11_WEP_STATUS(3i32);
pub const NdisClass802_3Priority: NDIS_CLASS_ID = NDIS_CLASS_ID(0i32);
pub const NdisClassAtmAALInfo: NDIS_CLASS_ID = NDIS_CLASS_ID(3i32);
pub const NdisClassIrdaPacketInfo: NDIS_CLASS_ID = NDIS_CLASS_ID(2i32);
pub const NdisClassWirelessWanMbxMailbox: NDIS_CLASS_ID = NDIS_CLASS_ID(1i32);
pub const NdisDefinitelyNetworkChange: NDIS_NETWORK_CHANGE_TYPE = NDIS_NETWORK_CHANGE_TYPE(2i32);
pub const NdisDevicePnPEventMaximum: NDIS_DEVICE_PNP_EVENT = NDIS_DEVICE_PNP_EVENT(6i32);
pub const NdisDevicePnPEventPowerProfileChanged: NDIS_DEVICE_PNP_EVENT = NDIS_DEVICE_PNP_EVENT(5i32);
pub const NdisDevicePnPEventQueryRemoved: NDIS_DEVICE_PNP_EVENT = NDIS_DEVICE_PNP_EVENT(0i32);
pub const NdisDevicePnPEventQueryStopped: NDIS_DEVICE_PNP_EVENT = NDIS_DEVICE_PNP_EVENT(3i32);
pub const NdisDevicePnPEventRemoved: NDIS_DEVICE_PNP_EVENT = NDIS_DEVICE_PNP_EVENT(1i32);
pub const NdisDevicePnPEventStopped: NDIS_DEVICE_PNP_EVENT = NDIS_DEVICE_PNP_EVENT(4i32);
pub const NdisDevicePnPEventSurpriseRemoved: NDIS_DEVICE_PNP_EVENT = NDIS_DEVICE_PNP_EVENT(2i32);
pub const NdisDeviceStateD0: NDIS_DEVICE_POWER_STATE = NDIS_DEVICE_POWER_STATE(1i32);
pub const NdisDeviceStateD1: NDIS_DEVICE_POWER_STATE = NDIS_DEVICE_POWER_STATE(2i32);
pub const NdisDeviceStateD2: NDIS_DEVICE_POWER_STATE = NDIS_DEVICE_POWER_STATE(3i32);
pub const NdisDeviceStateD3: NDIS_DEVICE_POWER_STATE = NDIS_DEVICE_POWER_STATE(4i32);
pub const NdisDeviceStateMaximum: NDIS_DEVICE_POWER_STATE = NDIS_DEVICE_POWER_STATE(5i32);
pub const NdisDeviceStateUnspecified: NDIS_DEVICE_POWER_STATE = NDIS_DEVICE_POWER_STATE(0i32);
pub const NdisEnvironmentWindows: NDIS_ENVIRONMENT_TYPE = NDIS_ENVIRONMENT_TYPE(0i32);
pub const NdisEnvironmentWindowsNt: NDIS_ENVIRONMENT_TYPE = NDIS_ENVIRONMENT_TYPE(1i32);
pub const NdisFddiRingDetect: NDIS_FDDI_RING_MGT_STATE = NDIS_FDDI_RING_MGT_STATE(4i32);
pub const NdisFddiRingDirected: NDIS_FDDI_RING_MGT_STATE = NDIS_FDDI_RING_MGT_STATE(7i32);
pub const NdisFddiRingIsolated: NDIS_FDDI_RING_MGT_STATE = NDIS_FDDI_RING_MGT_STATE(1i32);
pub const NdisFddiRingNonOperational: NDIS_FDDI_RING_MGT_STATE = NDIS_FDDI_RING_MGT_STATE(2i32);
pub const NdisFddiRingNonOperationalDup: NDIS_FDDI_RING_MGT_STATE = NDIS_FDDI_RING_MGT_STATE(5i32);
pub const NdisFddiRingOperational: NDIS_FDDI_RING_MGT_STATE = NDIS_FDDI_RING_MGT_STATE(3i32);
pub const NdisFddiRingOperationalDup: NDIS_FDDI_RING_MGT_STATE = NDIS_FDDI_RING_MGT_STATE(6i32);
pub const NdisFddiRingTrace: NDIS_FDDI_RING_MGT_STATE = NDIS_FDDI_RING_MGT_STATE(8i32);
pub const NdisFddiStateActive: NDIS_FDDI_LCONNECTION_STATE = NDIS_FDDI_LCONNECTION_STATE(9i32);
pub const NdisFddiStateBreak: NDIS_FDDI_LCONNECTION_STATE = NDIS_FDDI_LCONNECTION_STATE(2i32);
pub const NdisFddiStateConnect: NDIS_FDDI_LCONNECTION_STATE = NDIS_FDDI_LCONNECTION_STATE(4i32);
pub const NdisFddiStateJoin: NDIS_FDDI_LCONNECTION_STATE = NDIS_FDDI_LCONNECTION_STATE(7i32);
pub const NdisFddiStateMaintenance: NDIS_FDDI_LCONNECTION_STATE = NDIS_FDDI_LCONNECTION_STATE(10i32);
pub const NdisFddiStateNext: NDIS_FDDI_LCONNECTION_STATE = NDIS_FDDI_LCONNECTION_STATE(5i32);
pub const NdisFddiStateOff: NDIS_FDDI_LCONNECTION_STATE = NDIS_FDDI_LCONNECTION_STATE(1i32);
pub const NdisFddiStateSignal: NDIS_FDDI_LCONNECTION_STATE = NDIS_FDDI_LCONNECTION_STATE(6i32);
pub const NdisFddiStateTrace: NDIS_FDDI_LCONNECTION_STATE = NDIS_FDDI_LCONNECTION_STATE(3i32);
pub const NdisFddiStateVerify: NDIS_FDDI_LCONNECTION_STATE = NDIS_FDDI_LCONNECTION_STATE(8i32);
pub const NdisFddiTypeCWrapA: NDIS_FDDI_ATTACHMENT_TYPE = NDIS_FDDI_ATTACHMENT_TYPE(10i32);
pub const NdisFddiTypeCWrapB: NDIS_FDDI_ATTACHMENT_TYPE = NDIS_FDDI_ATTACHMENT_TYPE(11i32);
pub const NdisFddiTypeCWrapS: NDIS_FDDI_ATTACHMENT_TYPE = NDIS_FDDI_ATTACHMENT_TYPE(12i32);
pub const NdisFddiTypeIsolated: NDIS_FDDI_ATTACHMENT_TYPE = NDIS_FDDI_ATTACHMENT_TYPE(1i32);
pub const NdisFddiTypeLocalA: NDIS_FDDI_ATTACHMENT_TYPE = NDIS_FDDI_ATTACHMENT_TYPE(2i32);
pub const NdisFddiTypeLocalAB: NDIS_FDDI_ATTACHMENT_TYPE = NDIS_FDDI_ATTACHMENT_TYPE(4i32);
pub const NdisFddiTypeLocalB: NDIS_FDDI_ATTACHMENT_TYPE = NDIS_FDDI_ATTACHMENT_TYPE(3i32);
pub const NdisFddiTypeLocalS: NDIS_FDDI_ATTACHMENT_TYPE = NDIS_FDDI_ATTACHMENT_TYPE(5i32);
pub const NdisFddiTypeThrough: NDIS_FDDI_ATTACHMENT_TYPE = NDIS_FDDI_ATTACHMENT_TYPE(13i32);
pub const NdisFddiTypeWrapA: NDIS_FDDI_ATTACHMENT_TYPE = NDIS_FDDI_ATTACHMENT_TYPE(6i32);
pub const NdisFddiTypeWrapAB: NDIS_FDDI_ATTACHMENT_TYPE = NDIS_FDDI_ATTACHMENT_TYPE(8i32);
pub const NdisFddiTypeWrapB: NDIS_FDDI_ATTACHMENT_TYPE = NDIS_FDDI_ATTACHMENT_TYPE(7i32);
pub const NdisFddiTypeWrapS: NDIS_FDDI_ATTACHMENT_TYPE = NDIS_FDDI_ATTACHMENT_TYPE(9i32);
pub const NdisHardwareStatusClosing: NDIS_HARDWARE_STATUS = NDIS_HARDWARE_STATUS(3i32);
pub const NdisHardwareStatusInitializing: NDIS_HARDWARE_STATUS = NDIS_HARDWARE_STATUS(1i32);
pub const NdisHardwareStatusNotReady: NDIS_HARDWARE_STATUS = NDIS_HARDWARE_STATUS(4i32);
pub const NdisHardwareStatusReady: NDIS_HARDWARE_STATUS = NDIS_HARDWARE_STATUS(0i32);
pub const NdisHardwareStatusReset: NDIS_HARDWARE_STATUS = NDIS_HARDWARE_STATUS(2i32);
pub const NdisHashFunctionReserved1: u32 = 2u32;
pub const NdisHashFunctionReserved2: u32 = 4u32;
pub const NdisHashFunctionReserved3: u32 = 8u32;
pub const NdisHashFunctionToeplitz: u32 = 1u32;
pub const NdisInterface1394: NDIS_INTERFACE_TYPE = NDIS_INTERFACE_TYPE(18i32);
pub const NdisInterfaceCBus: NDIS_INTERFACE_TYPE = NDIS_INTERFACE_TYPE(9i32);
pub const NdisInterfaceEisa: NDIS_INTERFACE_TYPE = NDIS_INTERFACE_TYPE(2i32);
pub const NdisInterfaceInternal: NDIS_INTERFACE_TYPE = NDIS_INTERFACE_TYPE(0i32);
pub const NdisInterfaceInternalPowerBus: NDIS_INTERFACE_TYPE = NDIS_INTERFACE_TYPE(13i32);
pub const NdisInterfaceIrda: NDIS_INTERFACE_TYPE = NDIS_INTERFACE_TYPE(17i32);
pub const NdisInterfaceIsa: NDIS_INTERFACE_TYPE = NDIS_INTERFACE_TYPE(1i32);
pub const NdisInterfaceMPIBus: NDIS_INTERFACE_TYPE = NDIS_INTERFACE_TYPE(10i32);
pub const NdisInterfaceMPSABus: NDIS_INTERFACE_TYPE = NDIS_INTERFACE_TYPE(11i32);
pub const NdisInterfaceMca: NDIS_INTERFACE_TYPE = NDIS_INTERFACE_TYPE(3i32);
pub const NdisInterfacePNPBus: NDIS_INTERFACE_TYPE = NDIS_INTERFACE_TYPE(15i32);
pub const NdisInterfacePNPISABus: NDIS_INTERFACE_TYPE = NDIS_INTERFACE_TYPE(14i32);
pub const NdisInterfacePcMcia: NDIS_INTERFACE_TYPE = NDIS_INTERFACE_TYPE(8i32);
pub const NdisInterfacePci: NDIS_INTERFACE_TYPE = NDIS_INTERFACE_TYPE(5i32);
pub const NdisInterfaceProcessorInternal: NDIS_INTERFACE_TYPE = NDIS_INTERFACE_TYPE(12i32);
pub const NdisInterfaceTurboChannel: NDIS_INTERFACE_TYPE = NDIS_INTERFACE_TYPE(4i32);
pub const NdisInterfaceUSB: NDIS_INTERFACE_TYPE = NDIS_INTERFACE_TYPE(16i32);
pub const NdisInterruptModerationDisabled: NDIS_INTERRUPT_MODERATION = NDIS_INTERRUPT_MODERATION(3i32);
pub const NdisInterruptModerationEnabled: NDIS_INTERRUPT_MODERATION = NDIS_INTERRUPT_MODERATION(2i32);
pub const NdisInterruptModerationNotSupported: NDIS_INTERRUPT_MODERATION = NDIS_INTERRUPT_MODERATION(1i32);
pub const NdisInterruptModerationUnknown: NDIS_INTERRUPT_MODERATION = NDIS_INTERRUPT_MODERATION(0i32);
pub const NdisMaximumInterfaceType: NDIS_INTERFACE_TYPE = NDIS_INTERFACE_TYPE(19i32);
pub const NdisMediaStateConnected: NDIS_MEDIA_STATE = NDIS_MEDIA_STATE(0i32);
pub const NdisMediaStateDisconnected: NDIS_MEDIA_STATE = NDIS_MEDIA_STATE(1i32);
pub const NdisMedium1394: NDIS_MEDIUM = NDIS_MEDIUM(13i32);
pub const NdisMedium802_3: NDIS_MEDIUM = NDIS_MEDIUM(0i32);
pub const NdisMedium802_5: NDIS_MEDIUM = NDIS_MEDIUM(1i32);
pub const NdisMediumArcnet878_2: NDIS_MEDIUM = NDIS_MEDIUM(7i32);
pub const NdisMediumArcnetRaw: NDIS_MEDIUM = NDIS_MEDIUM(6i32);
pub const NdisMediumAtm: NDIS_MEDIUM = NDIS_MEDIUM(8i32);
pub const NdisMediumBpc: NDIS_MEDIUM = NDIS_MEDIUM(11i32);
pub const NdisMediumCoWan: NDIS_MEDIUM = NDIS_MEDIUM(12i32);
pub const NdisMediumDix: NDIS_MEDIUM = NDIS_MEDIUM(5i32);
pub const NdisMediumFddi: NDIS_MEDIUM = NDIS_MEDIUM(2i32);
pub const NdisMediumIP: NDIS_MEDIUM = NDIS_MEDIUM(19i32);
pub const NdisMediumInfiniBand: NDIS_MEDIUM = NDIS_MEDIUM(14i32);
pub const NdisMediumIrda: NDIS_MEDIUM = NDIS_MEDIUM(10i32);
pub const NdisMediumLocalTalk: NDIS_MEDIUM = NDIS_MEDIUM(4i32);
pub const NdisMediumLoopback: NDIS_MEDIUM = NDIS_MEDIUM(17i32);
pub const NdisMediumMax: NDIS_MEDIUM = NDIS_MEDIUM(20i32);
pub const NdisMediumNative802_11: NDIS_MEDIUM = NDIS_MEDIUM(16i32);
pub const NdisMediumTunnel: NDIS_MEDIUM = NDIS_MEDIUM(15i32);
pub const NdisMediumWan: NDIS_MEDIUM = NDIS_MEDIUM(3i32);
pub const NdisMediumWiMAX: NDIS_MEDIUM = NDIS_MEDIUM(18i32);
pub const NdisMediumWirelessWan: NDIS_MEDIUM = NDIS_MEDIUM(9i32);
pub const NdisNetworkChangeFromMediaConnect: NDIS_NETWORK_CHANGE_TYPE = NDIS_NETWORK_CHANGE_TYPE(3i32);
pub const NdisNetworkChangeMax: NDIS_NETWORK_CHANGE_TYPE = NDIS_NETWORK_CHANGE_TYPE(4i32);
pub const NdisParameterBinary: NDIS_PARAMETER_TYPE = NDIS_PARAMETER_TYPE(4i32);
pub const NdisParameterHexInteger: NDIS_PARAMETER_TYPE = NDIS_PARAMETER_TYPE(1i32);
pub const NdisParameterInteger: NDIS_PARAMETER_TYPE = NDIS_PARAMETER_TYPE(0i32);
pub const NdisParameterMultiString: NDIS_PARAMETER_TYPE = NDIS_PARAMETER_TYPE(3i32);
pub const NdisParameterString: NDIS_PARAMETER_TYPE = NDIS_PARAMETER_TYPE(2i32);
pub const NdisPauseFunctionsReceiveOnly: NDIS_SUPPORTED_PAUSE_FUNCTIONS = NDIS_SUPPORTED_PAUSE_FUNCTIONS(2i32);
pub const NdisPauseFunctionsSendAndReceive: NDIS_SUPPORTED_PAUSE_FUNCTIONS = NDIS_SUPPORTED_PAUSE_FUNCTIONS(3i32);
pub const NdisPauseFunctionsSendOnly: NDIS_SUPPORTED_PAUSE_FUNCTIONS = NDIS_SUPPORTED_PAUSE_FUNCTIONS(1i32);
pub const NdisPauseFunctionsUnknown: NDIS_SUPPORTED_PAUSE_FUNCTIONS = NDIS_SUPPORTED_PAUSE_FUNCTIONS(4i32);
pub const NdisPauseFunctionsUnsupported: NDIS_SUPPORTED_PAUSE_FUNCTIONS = NDIS_SUPPORTED_PAUSE_FUNCTIONS(0i32);
pub const NdisPhysicalMedium1394: NDIS_PHYSICAL_MEDIUM = NDIS_PHYSICAL_MEDIUM(7i32);
pub const NdisPhysicalMedium802_3: NDIS_PHYSICAL_MEDIUM = NDIS_PHYSICAL_MEDIUM(14i32);
pub const NdisPhysicalMedium802_5: NDIS_PHYSICAL_MEDIUM = NDIS_PHYSICAL_MEDIUM(15i32);
pub const NdisPhysicalMediumBluetooth: NDIS_PHYSICAL_MEDIUM = NDIS_PHYSICAL_MEDIUM(10i32);
pub const NdisPhysicalMediumCableModem: NDIS_PHYSICAL_MEDIUM = NDIS_PHYSICAL_MEDIUM(2i32);
pub const NdisPhysicalMediumDSL: NDIS_PHYSICAL_MEDIUM = NDIS_PHYSICAL_MEDIUM(5i32);
pub const NdisPhysicalMediumFibreChannel: NDIS_PHYSICAL_MEDIUM = NDIS_PHYSICAL_MEDIUM(6i32);
pub const NdisPhysicalMediumInfiniband: NDIS_PHYSICAL_MEDIUM = NDIS_PHYSICAL_MEDIUM(11i32);
pub const NdisPhysicalMediumIrda: NDIS_PHYSICAL_MEDIUM = NDIS_PHYSICAL_MEDIUM(16i32);
pub const NdisPhysicalMediumMax: NDIS_PHYSICAL_MEDIUM = NDIS_PHYSICAL_MEDIUM(21i32);
pub const NdisPhysicalMediumNative802_11: NDIS_PHYSICAL_MEDIUM = NDIS_PHYSICAL_MEDIUM(9i32);
pub const NdisPhysicalMediumNative802_15_4: NDIS_PHYSICAL_MEDIUM = NDIS_PHYSICAL_MEDIUM(20i32);
pub const NdisPhysicalMediumOther: NDIS_PHYSICAL_MEDIUM = NDIS_PHYSICAL_MEDIUM(19i32);
pub const NdisPhysicalMediumPhoneLine: NDIS_PHYSICAL_MEDIUM = NDIS_PHYSICAL_MEDIUM(3i32);
pub const NdisPhysicalMediumPowerLine: NDIS_PHYSICAL_MEDIUM = NDIS_PHYSICAL_MEDIUM(4i32);
pub const NdisPhysicalMediumUWB: NDIS_PHYSICAL_MEDIUM = NDIS_PHYSICAL_MEDIUM(13i32);
pub const NdisPhysicalMediumUnspecified: NDIS_PHYSICAL_MEDIUM = NDIS_PHYSICAL_MEDIUM(0i32);
pub const NdisPhysicalMediumWiMax: NDIS_PHYSICAL_MEDIUM = NDIS_PHYSICAL_MEDIUM(12i32);
pub const NdisPhysicalMediumWiredCoWan: NDIS_PHYSICAL_MEDIUM = NDIS_PHYSICAL_MEDIUM(18i32);
pub const NdisPhysicalMediumWiredWAN: NDIS_PHYSICAL_MEDIUM = NDIS_PHYSICAL_MEDIUM(17i32);
pub const NdisPhysicalMediumWirelessLan: NDIS_PHYSICAL_MEDIUM = NDIS_PHYSICAL_MEDIUM(1i32);
pub const NdisPhysicalMediumWirelessWan: NDIS_PHYSICAL_MEDIUM = NDIS_PHYSICAL_MEDIUM(8i32);
pub const NdisPortAuthorizationUnknown: NDIS_PORT_AUTHORIZATION_STATE = NDIS_PORT_AUTHORIZATION_STATE(0i32);
pub const NdisPortAuthorized: NDIS_PORT_AUTHORIZATION_STATE = NDIS_PORT_AUTHORIZATION_STATE(1i32);
pub const NdisPortControlStateControlled: NDIS_PORT_CONTROL_STATE = NDIS_PORT_CONTROL_STATE(1i32);
pub const NdisPortControlStateUncontrolled: NDIS_PORT_CONTROL_STATE = NDIS_PORT_CONTROL_STATE(2i32);
pub const NdisPortControlStateUnknown: NDIS_PORT_CONTROL_STATE = NDIS_PORT_CONTROL_STATE(0i32);
pub const NdisPortReauthorizing: NDIS_PORT_AUTHORIZATION_STATE = NDIS_PORT_AUTHORIZATION_STATE(3i32);
pub const NdisPortType8021xSupplicant: NDIS_PORT_TYPE = NDIS_PORT_TYPE(3i32);
pub const NdisPortTypeBridge: NDIS_PORT_TYPE = NDIS_PORT_TYPE(1i32);
pub const NdisPortTypeMax: NDIS_PORT_TYPE = NDIS_PORT_TYPE(4i32);
pub const NdisPortTypeRasConnection: NDIS_PORT_TYPE = NDIS_PORT_TYPE(2i32);
pub const NdisPortTypeUndefined: NDIS_PORT_TYPE = NDIS_PORT_TYPE(0i32);
pub const NdisPortUnauthorized: NDIS_PORT_AUTHORIZATION_STATE = NDIS_PORT_AUTHORIZATION_STATE(2i32);
pub const NdisPossibleNetworkChange: NDIS_NETWORK_CHANGE_TYPE = NDIS_NETWORK_CHANGE_TYPE(1i32);
pub const NdisPowerProfileAcOnLine: NDIS_POWER_PROFILE = NDIS_POWER_PROFILE(1i32);
pub const NdisPowerProfileBattery: NDIS_POWER_PROFILE = NDIS_POWER_PROFILE(0i32);
pub const NdisProcessorAlpha: NDIS_PROCESSOR_TYPE = NDIS_PROCESSOR_TYPE(2i32);
pub const NdisProcessorAmd64: NDIS_PROCESSOR_TYPE = NDIS_PROCESSOR_TYPE(4i32);
pub const NdisProcessorArm: NDIS_PROCESSOR_TYPE = NDIS_PROCESSOR_TYPE(6i32);
pub const NdisProcessorArm64: NDIS_PROCESSOR_TYPE = NDIS_PROCESSOR_TYPE(7i32);
pub const NdisProcessorIA64: NDIS_PROCESSOR_TYPE = NDIS_PROCESSOR_TYPE(5i32);
pub const NdisProcessorMips: NDIS_PROCESSOR_TYPE = NDIS_PROCESSOR_TYPE(1i32);
pub const NdisProcessorPpc: NDIS_PROCESSOR_TYPE = NDIS_PROCESSOR_TYPE(3i32);
pub const NdisProcessorVendorAuthenticAMD: NDIS_PROCESSOR_VENDOR = NDIS_PROCESSOR_VENDOR(2i32);
pub const NdisProcessorVendorGenuinIntel: NDIS_PROCESSOR_VENDOR = NDIS_PROCESSOR_VENDOR(1i32);
pub const NdisProcessorVendorGenuineIntel: NDIS_PROCESSOR_VENDOR = NDIS_PROCESSOR_VENDOR(1i32);
pub const NdisProcessorVendorUnknown: NDIS_PROCESSOR_VENDOR = NDIS_PROCESSOR_VENDOR(0i32);
pub const NdisProcessorX86: NDIS_PROCESSOR_TYPE = NDIS_PROCESSOR_TYPE(0i32);
pub const NdisRequestClose: NDIS_REQUEST_TYPE = NDIS_REQUEST_TYPE(4i32);
pub const NdisRequestGeneric1: NDIS_REQUEST_TYPE = NDIS_REQUEST_TYPE(8i32);
pub const NdisRequestGeneric2: NDIS_REQUEST_TYPE = NDIS_REQUEST_TYPE(9i32);
pub const NdisRequestGeneric3: NDIS_REQUEST_TYPE = NDIS_REQUEST_TYPE(10i32);
pub const NdisRequestGeneric4: NDIS_REQUEST_TYPE = NDIS_REQUEST_TYPE(11i32);
pub const NdisRequestOpen: NDIS_REQUEST_TYPE = NDIS_REQUEST_TYPE(3i32);
pub const NdisRequestQueryInformation: NDIS_REQUEST_TYPE = NDIS_REQUEST_TYPE(0i32);
pub const NdisRequestQueryStatistics: NDIS_REQUEST_TYPE = NDIS_REQUEST_TYPE(2i32);
pub const NdisRequestReset: NDIS_REQUEST_TYPE = NDIS_REQUEST_TYPE(7i32);
pub const NdisRequestSend: NDIS_REQUEST_TYPE = NDIS_REQUEST_TYPE(5i32);
pub const NdisRequestSetInformation: NDIS_REQUEST_TYPE = NDIS_REQUEST_TYPE(1i32);
pub const NdisRequestTransferData: NDIS_REQUEST_TYPE = NDIS_REQUEST_TYPE(6i32);
pub const NdisReserved: NDIS_PER_PACKET_INFO = NDIS_PER_PACKET_INFO(4i32);
pub const NdisRingStateClosed: NDIS_802_5_RING_STATE = NDIS_802_5_RING_STATE(2i32);
pub const NdisRingStateClosing: NDIS_802_5_RING_STATE = NDIS_802_5_RING_STATE(4i32);
pub const NdisRingStateOpenFailure: NDIS_802_5_RING_STATE = NDIS_802_5_RING_STATE(5i32);
pub const NdisRingStateOpened: NDIS_802_5_RING_STATE = NDIS_802_5_RING_STATE(1i32);
pub const NdisRingStateOpening: NDIS_802_5_RING_STATE = NDIS_802_5_RING_STATE(3i32);
pub const NdisRingStateRingFailure: NDIS_802_5_RING_STATE = NDIS_802_5_RING_STATE(6i32);
pub const NdisWanErrorControl: NDIS_WAN_QUALITY = NDIS_WAN_QUALITY(1i32);
pub const NdisWanHeaderEthernet: NDIS_WAN_HEADER_FORMAT = NDIS_WAN_HEADER_FORMAT(1i32);
pub const NdisWanHeaderNative: NDIS_WAN_HEADER_FORMAT = NDIS_WAN_HEADER_FORMAT(0i32);
pub const NdisWanMediumAgileVPN: NDIS_WAN_MEDIUM_SUBTYPE = NDIS_WAN_MEDIUM_SUBTYPE(14i32);
pub const NdisWanMediumAtm: NDIS_WAN_MEDIUM_SUBTYPE = NDIS_WAN_MEDIUM_SUBTYPE(5i32);
pub const NdisWanMediumFrameRelay: NDIS_WAN_MEDIUM_SUBTYPE = NDIS_WAN_MEDIUM_SUBTYPE(4i32);
pub const NdisWanMediumGre: NDIS_WAN_MEDIUM_SUBTYPE = NDIS_WAN_MEDIUM_SUBTYPE(15i32);
pub const NdisWanMediumHub: NDIS_WAN_MEDIUM_SUBTYPE = NDIS_WAN_MEDIUM_SUBTYPE(0i32);
pub const NdisWanMediumIrda: NDIS_WAN_MEDIUM_SUBTYPE = NDIS_WAN_MEDIUM_SUBTYPE(10i32);
pub const NdisWanMediumIsdn: NDIS_WAN_MEDIUM_SUBTYPE = NDIS_WAN_MEDIUM_SUBTYPE(2i32);
pub const NdisWanMediumL2TP: NDIS_WAN_MEDIUM_SUBTYPE = NDIS_WAN_MEDIUM_SUBTYPE(9i32);
pub const NdisWanMediumPPTP: NDIS_WAN_MEDIUM_SUBTYPE = NDIS_WAN_MEDIUM_SUBTYPE(8i32);
pub const NdisWanMediumParallel: NDIS_WAN_MEDIUM_SUBTYPE = NDIS_WAN_MEDIUM_SUBTYPE(11i32);
pub const NdisWanMediumPppoe: NDIS_WAN_MEDIUM_SUBTYPE = NDIS_WAN_MEDIUM_SUBTYPE(12i32);
pub const NdisWanMediumSSTP: NDIS_WAN_MEDIUM_SUBTYPE = NDIS_WAN_MEDIUM_SUBTYPE(13i32);
pub const NdisWanMediumSW56K: NDIS_WAN_MEDIUM_SUBTYPE = NDIS_WAN_MEDIUM_SUBTYPE(7i32);
pub const NdisWanMediumSerial: NDIS_WAN_MEDIUM_SUBTYPE = NDIS_WAN_MEDIUM_SUBTYPE(3i32);
pub const NdisWanMediumSonet: NDIS_WAN_MEDIUM_SUBTYPE = NDIS_WAN_MEDIUM_SUBTYPE(6i32);
pub const NdisWanMediumSubTypeMax: NDIS_WAN_MEDIUM_SUBTYPE = NDIS_WAN_MEDIUM_SUBTYPE(16i32);
pub const NdisWanMediumX_25: NDIS_WAN_MEDIUM_SUBTYPE = NDIS_WAN_MEDIUM_SUBTYPE(1i32);
pub const NdisWanRaw: NDIS_WAN_QUALITY = NDIS_WAN_QUALITY(0i32);
pub const NdisWanReliable: NDIS_WAN_QUALITY = NDIS_WAN_QUALITY(2i32);
pub const OFFLOAD_INBOUND_SA: u32 = 1u32;
pub const OFFLOAD_IPSEC_CONF_3_DES: OFFLOAD_CONF_ALGO = OFFLOAD_CONF_ALGO(3i32);
pub const OFFLOAD_IPSEC_CONF_DES: OFFLOAD_CONF_ALGO = OFFLOAD_CONF_ALGO(1i32);
pub const OFFLOAD_IPSEC_CONF_MAX: OFFLOAD_CONF_ALGO = OFFLOAD_CONF_ALGO(4i32);
pub const OFFLOAD_IPSEC_CONF_NONE: OFFLOAD_CONF_ALGO = OFFLOAD_CONF_ALGO(0i32);
pub const OFFLOAD_IPSEC_CONF_RESERVED: OFFLOAD_CONF_ALGO = OFFLOAD_CONF_ALGO(2i32);
pub const OFFLOAD_IPSEC_INTEGRITY_MAX: OFFLOAD_INTEGRITY_ALGO = OFFLOAD_INTEGRITY_ALGO(3i32);
pub const OFFLOAD_IPSEC_INTEGRITY_MD5: OFFLOAD_INTEGRITY_ALGO = OFFLOAD_INTEGRITY_ALGO(1i32);
pub const OFFLOAD_IPSEC_INTEGRITY_NONE: OFFLOAD_INTEGRITY_ALGO = OFFLOAD_INTEGRITY_ALGO(0i32);
pub const OFFLOAD_IPSEC_INTEGRITY_SHA: OFFLOAD_INTEGRITY_ALGO = OFFLOAD_INTEGRITY_ALGO(2i32);
pub const OFFLOAD_IPSEC_UDPESP_ENCAPTYPE_IKE: UDP_ENCAP_TYPE = UDP_ENCAP_TYPE(0i32);
pub const OFFLOAD_IPSEC_UDPESP_ENCAPTYPE_OTHER: UDP_ENCAP_TYPE = UDP_ENCAP_TYPE(1i32);
pub const OFFLOAD_MAX_SAS: u32 = 3u32;
pub const OFFLOAD_OUTBOUND_SA: u32 = 2u32;
pub const OID_1394_LOCAL_NODE_INFO: u32 = 201392385u32;
pub const OID_1394_VC_INFO: u32 = 201392386u32;
pub const OID_802_11_ADD_KEY: u32 = 218169629u32;
pub const OID_802_11_ADD_WEP: u32 = 218169619u32;
pub const OID_802_11_ASSOCIATION_INFORMATION: u32 = 218169631u32;
pub const OID_802_11_AUTHENTICATION_MODE: u32 = 218169624u32;
pub const OID_802_11_BSSID: u32 = 218169601u32;
pub const OID_802_11_BSSID_LIST: u32 = 218169879u32;
pub const OID_802_11_BSSID_LIST_SCAN: u32 = 218169626u32;
pub const OID_802_11_CAPABILITY: u32 = 218169634u32;
pub const OID_802_11_CONFIGURATION: u32 = 218169873u32;
pub const OID_802_11_DESIRED_RATES: u32 = 218169872u32;
pub const OID_802_11_DISASSOCIATE: u32 = 218169621u32;
pub const OID_802_11_ENCRYPTION_STATUS: u32 = 218169627u32;
pub const OID_802_11_FRAGMENTATION_THRESHOLD: u32 = 218169865u32;
pub const OID_802_11_INFRASTRUCTURE_MODE: u32 = 218169608u32;
pub const OID_802_11_MEDIA_STREAM_MODE: u32 = 218169633u32;
pub const OID_802_11_NETWORK_TYPES_SUPPORTED: u32 = 218169859u32;
pub const OID_802_11_NETWORK_TYPE_IN_USE: u32 = 218169860u32;
pub const OID_802_11_NON_BCAST_SSID_LIST: u32 = 218169636u32;
pub const OID_802_11_NUMBER_OF_ANTENNAS: u32 = 218169867u32;
pub const OID_802_11_PMKID: u32 = 218169635u32;
pub const OID_802_11_POWER_MODE: u32 = 218169878u32;
pub const OID_802_11_PRIVACY_FILTER: u32 = 218169625u32;
pub const OID_802_11_RADIO_STATUS: u32 = 218169637u32;
pub const OID_802_11_RELOAD_DEFAULTS: u32 = 218169628u32;
pub const OID_802_11_REMOVE_KEY: u32 = 218169630u32;
pub const OID_802_11_REMOVE_WEP: u32 = 218169620u32;
pub const OID_802_11_RSSI: u32 = 218169862u32;
pub const OID_802_11_RSSI_TRIGGER: u32 = 218169863u32;
pub const OID_802_11_RTS_THRESHOLD: u32 = 218169866u32;
pub const OID_802_11_RX_ANTENNA_SELECTED: u32 = 218169868u32;
pub const OID_802_11_SSID: u32 = 218169602u32;
pub const OID_802_11_STATISTICS: u32 = 218235410u32;
pub const OID_802_11_SUPPORTED_RATES: u32 = 218169870u32;
pub const OID_802_11_TEST: u32 = 218169632u32;
pub const OID_802_11_TX_ANTENNA_SELECTED: u32 = 218169869u32;
pub const OID_802_11_TX_POWER_LEVEL: u32 = 218169861u32;
pub const OID_802_11_WEP_STATUS: u32 = 218169627u32;
pub const OID_802_3_ADD_MULTICAST_ADDRESS: u32 = 16843272u32;
pub const OID_802_3_CURRENT_ADDRESS: u32 = 16843010u32;
pub const OID_802_3_DELETE_MULTICAST_ADDRESS: u32 = 16843273u32;
pub const OID_802_3_MAC_OPTIONS: u32 = 16843013u32;
pub const OID_802_3_MAXIMUM_LIST_SIZE: u32 = 16843012u32;
pub const OID_802_3_MULTICAST_LIST: u32 = 16843011u32;
pub const OID_802_3_PERMANENT_ADDRESS: u32 = 16843009u32;
pub const OID_802_3_RCV_ERROR_ALIGNMENT: u32 = 16908545u32;
pub const OID_802_3_RCV_OVERRUN: u32 = 16908803u32;
pub const OID_802_3_XMIT_DEFERRED: u32 = 16908801u32;
pub const OID_802_3_XMIT_HEARTBEAT_FAILURE: u32 = 16908805u32;
pub const OID_802_3_XMIT_LATE_COLLISIONS: u32 = 16908807u32;
pub const OID_802_3_XMIT_MAX_COLLISIONS: u32 = 16908802u32;
pub const OID_802_3_XMIT_MORE_COLLISIONS: u32 = 16908547u32;
pub const OID_802_3_XMIT_ONE_COLLISION: u32 = 16908546u32;
pub const OID_802_3_XMIT_TIMES_CRS_LOST: u32 = 16908806u32;
pub const OID_802_3_XMIT_UNDERRUN: u32 = 16908804u32;
pub const OID_802_5_ABORT_DELIMETERS: u32 = 33686019u32;
pub const OID_802_5_AC_ERRORS: u32 = 33686018u32;
pub const OID_802_5_BURST_ERRORS: u32 = 33686017u32;
pub const OID_802_5_CURRENT_ADDRESS: u32 = 33620226u32;
pub const OID_802_5_CURRENT_FUNCTIONAL: u32 = 33620227u32;
pub const OID_802_5_CURRENT_GROUP: u32 = 33620228u32;
pub const OID_802_5_CURRENT_RING_STATE: u32 = 33620231u32;
pub const OID_802_5_CURRENT_RING_STATUS: u32 = 33620230u32;
pub const OID_802_5_FRAME_COPIED_ERRORS: u32 = 33686020u32;
pub const OID_802_5_FREQUENCY_ERRORS: u32 = 33686021u32;
pub const OID_802_5_INTERNAL_ERRORS: u32 = 33686023u32;
pub const OID_802_5_LAST_OPEN_STATUS: u32 = 33620229u32;
pub const OID_802_5_LINE_ERRORS: u32 = 33685761u32;
pub const OID_802_5_LOST_FRAMES: u32 = 33685762u32;
pub const OID_802_5_PERMANENT_ADDRESS: u32 = 33620225u32;
pub const OID_802_5_TOKEN_ERRORS: u32 = 33686022u32;
pub const OID_ARCNET_CURRENT_ADDRESS: u32 = 100729090u32;
pub const OID_ARCNET_PERMANENT_ADDRESS: u32 = 100729089u32;
pub const OID_ARCNET_RECONFIGURATIONS: u32 = 100794881u32;
pub const OID_ATM_ACQUIRE_ACCESS_NET_RESOURCES: u32 = 134283779u32;
pub const OID_ATM_ALIGNMENT_REQUIRED: u32 = 134283784u32;
pub const OID_ATM_ASSIGNED_VPI: u32 = 134283778u32;
pub const OID_ATM_CALL_ALERTING: u32 = 134283788u32;
pub const OID_ATM_CALL_NOTIFY: u32 = 134283790u32;
pub const OID_ATM_CALL_PROCEEDING: u32 = 134283787u32;
pub const OID_ATM_CELLS_HEC_ERROR: u32 = 134349314u32;
pub const OID_ATM_DIGITAL_BROADCAST_VPIVCI: u32 = 134283782u32;
pub const OID_ATM_GET_NEAREST_FLOW: u32 = 134283783u32;
pub const OID_ATM_HW_CURRENT_ADDRESS: u32 = 134283524u32;
pub const OID_ATM_ILMI_VPIVCI: u32 = 134283781u32;
pub const OID_ATM_LECS_ADDRESS: u32 = 134283785u32;
pub const OID_ATM_MAX_AAL0_PACKET_SIZE: u32 = 134283528u32;
pub const OID_ATM_MAX_AAL1_PACKET_SIZE: u32 = 134283529u32;
pub const OID_ATM_MAX_AAL34_PACKET_SIZE: u32 = 134283530u32;
pub const OID_ATM_MAX_AAL5_PACKET_SIZE: u32 = 134283531u32;
pub const OID_ATM_MAX_ACTIVE_VCI_BITS: u32 = 134283526u32;
pub const OID_ATM_MAX_ACTIVE_VCS: u32 = 134283525u32;
pub const OID_ATM_MAX_ACTIVE_VPI_BITS: u32 = 134283527u32;
pub const OID_ATM_MY_IP_NM_ADDRESS: u32 = 134283791u32;
pub const OID_ATM_PARTY_ALERTING: u32 = 134283789u32;
pub const OID_ATM_RCV_CELLS_DROPPED: u32 = 134349059u32;
pub const OID_ATM_RCV_CELLS_OK: u32 = 134349057u32;
pub const OID_ATM_RCV_INVALID_VPI_VCI: u32 = 134349313u32;
pub const OID_ATM_RCV_REASSEMBLY_ERROR: u32 = 134349315u32;
pub const OID_ATM_RELEASE_ACCESS_NET_RESOURCES: u32 = 134283780u32;
pub const OID_ATM_SERVICE_ADDRESS: u32 = 134283786u32;
pub const OID_ATM_SIGNALING_VPIVCI: u32 = 134283777u32;
pub const OID_ATM_SUPPORTED_AAL_TYPES: u32 = 134283523u32;
pub const OID_ATM_SUPPORTED_SERVICE_CATEGORY: u32 = 134283522u32;
pub const OID_ATM_SUPPORTED_VC_RATES: u32 = 134283521u32;
pub const OID_ATM_XMIT_CELLS_OK: u32 = 134349058u32;
pub const OID_CO_ADDRESS_CHANGE: u32 = 4261412871u32;
pub const OID_CO_ADD_ADDRESS: u32 = 4261412868u32;
pub const OID_CO_ADD_PVC: u32 = 4261412865u32;
pub const OID_CO_AF_CLOSE: u32 = 4261412874u32;
pub const OID_CO_DELETE_ADDRESS: u32 = 4261412869u32;
pub const OID_CO_DELETE_PVC: u32 = 4261412866u32;
pub const OID_CO_GET_ADDRESSES: u32 = 4261412870u32;
pub const OID_CO_GET_CALL_INFORMATION: u32 = 4261412867u32;
pub const OID_CO_SIGNALING_DISABLED: u32 = 4261412873u32;
pub const OID_CO_SIGNALING_ENABLED: u32 = 4261412872u32;
pub const OID_CO_TAPI_ADDRESS_CAPS: u32 = 4261416963u32;
pub const OID_CO_TAPI_CM_CAPS: u32 = 4261416961u32;
pub const OID_CO_TAPI_DONT_REPORT_DIGITS: u32 = 4261416969u32;
pub const OID_CO_TAPI_GET_CALL_DIAGNOSTICS: u32 = 4261416967u32;
pub const OID_CO_TAPI_LINE_CAPS: u32 = 4261416962u32;
pub const OID_CO_TAPI_REPORT_DIGITS: u32 = 4261416968u32;
pub const OID_CO_TAPI_TRANSLATE_NDIS_CALLPARAMS: u32 = 4261416965u32;
pub const OID_CO_TAPI_TRANSLATE_TAPI_CALLPARAMS: u32 = 4261416964u32;
pub const OID_CO_TAPI_TRANSLATE_TAPI_SAP: u32 = 4261416966u32;
pub const OID_FDDI_ATTACHMENT_TYPE: u32 = 50462977u32;
pub const OID_FDDI_DOWNSTREAM_NODE_LONG: u32 = 50462979u32;
pub const OID_FDDI_FRAMES_LOST: u32 = 50462981u32;
pub const OID_FDDI_FRAME_ERRORS: u32 = 50462980u32;
pub const OID_FDDI_IF_ADMIN_STATUS: u32 = 50528894u32;
pub const OID_FDDI_IF_DESCR: u32 = 50528889u32;
pub const OID_FDDI_IF_IN_DISCARDS: u32 = 50528900u32;
pub const OID_FDDI_IF_IN_ERRORS: u32 = 50528901u32;
pub const OID_FDDI_IF_IN_NUCAST_PKTS: u32 = 50528899u32;
pub const OID_FDDI_IF_IN_OCTETS: u32 = 50528897u32;
pub const OID_FDDI_IF_IN_UCAST_PKTS: u32 = 50528898u32;
pub const OID_FDDI_IF_IN_UNKNOWN_PROTOS: u32 = 50528902u32;
pub const OID_FDDI_IF_LAST_CHANGE: u32 = 50528896u32;
pub const OID_FDDI_IF_MTU: u32 = 50528891u32;
pub const OID_FDDI_IF_OPER_STATUS: u32 = 50528895u32;
pub const OID_FDDI_IF_OUT_DISCARDS: u32 = 50528906u32;
pub const OID_FDDI_IF_OUT_ERRORS: u32 = 50528907u32;
pub const OID_FDDI_IF_OUT_NUCAST_PKTS: u32 = 50528905u32;
pub const OID_FDDI_IF_OUT_OCTETS: u32 = 50528903u32;
pub const OID_FDDI_IF_OUT_QLEN: u32 = 50528908u32;
pub const OID_FDDI_IF_OUT_UCAST_PKTS: u32 = 50528904u32;
pub const OID_FDDI_IF_PHYS_ADDRESS: u32 = 50528893u32;
pub const OID_FDDI_IF_SPECIFIC: u32 = 50528909u32;
pub const OID_FDDI_IF_SPEED: u32 = 50528892u32;
pub const OID_FDDI_IF_TYPE: u32 = 50528890u32;
pub const OID_FDDI_LCONNECTION_STATE: u32 = 50462985u32;
pub const OID_FDDI_LCT_FAILURES: u32 = 50462983u32;
pub const OID_FDDI_LEM_REJECTS: u32 = 50462984u32;
pub const OID_FDDI_LONG_CURRENT_ADDR: u32 = 50397442u32;
pub const OID_FDDI_LONG_MAX_LIST_SIZE: u32 = 50397444u32;
pub const OID_FDDI_LONG_MULTICAST_LIST: u32 = 50397443u32;
pub const OID_FDDI_LONG_PERMANENT_ADDR: u32 = 50397441u32;
pub const OID_FDDI_MAC_AVAILABLE_PATHS: u32 = 50528803u32;
pub const OID_FDDI_MAC_BRIDGE_FUNCTIONS: u32 = 50528800u32;
pub const OID_FDDI_MAC_COPIED_CT: u32 = 50528828u32;
pub const OID_FDDI_MAC_CURRENT_PATH: u32 = 50528804u32;
pub const OID_FDDI_MAC_DA_FLAG: u32 = 50528842u32;
pub const OID_FDDI_MAC_DOWNSTREAM_NBR: u32 = 50528806u32;
pub const OID_FDDI_MAC_DOWNSTREAM_PORT_TYPE: u32 = 50528811u32;
pub const OID_FDDI_MAC_DUP_ADDRESS_TEST: u32 = 50528809u32;
pub const OID_FDDI_MAC_ERROR_CT: u32 = 50528831u32;
pub const OID_FDDI_MAC_FRAME_CT: u32 = 50528827u32;
pub const OID_FDDI_MAC_FRAME_ERROR_FLAG: u32 = 50528844u32;
pub const OID_FDDI_MAC_FRAME_ERROR_RATIO: u32 = 50528838u32;
pub const OID_FDDI_MAC_FRAME_ERROR_THRESHOLD: u32 = 50528837u32;
pub const OID_FDDI_MAC_FRAME_STATUS_FUNCTIONS: u32 = 50528799u32;
pub const OID_FDDI_MAC_HARDWARE_PRESENT: u32 = 50528847u32;
pub const OID_FDDI_MAC_INDEX: u32 = 50528812u32;
pub const OID_FDDI_MAC_LATE_CT: u32 = 50528835u32;
pub const OID_FDDI_MAC_LONG_GRP_ADDRESS: u32 = 50528814u32;
pub const OID_FDDI_MAC_LOST_CT: u32 = 50528832u32;
pub const OID_FDDI_MAC_MA_UNITDATA_AVAILABLE: u32 = 50528846u32;
pub const OID_FDDI_MAC_MA_UNITDATA_ENABLE: u32 = 50528848u32;
pub const OID_FDDI_MAC_NOT_COPIED_CT: u32 = 50528834u32;
pub const OID_FDDI_MAC_NOT_COPIED_FLAG: u32 = 50528845u32;
pub const OID_FDDI_MAC_NOT_COPIED_RATIO: u32 = 50528840u32;
pub const OID_FDDI_MAC_NOT_COPIED_THRESHOLD: u32 = 50528839u32;
pub const OID_FDDI_MAC_OLD_DOWNSTREAM_NBR: u32 = 50528808u32;
pub const OID_FDDI_MAC_OLD_UPSTREAM_NBR: u32 = 50528807u32;
pub const OID_FDDI_MAC_REQUESTED_PATHS: u32 = 50528810u32;
pub const OID_FDDI_MAC_RING_OP_CT: u32 = 50528836u32;
pub const OID_FDDI_MAC_RMT_STATE: u32 = 50528841u32;
pub const OID_FDDI_MAC_SHORT_GRP_ADDRESS: u32 = 50528815u32;
pub const OID_FDDI_MAC_SMT_ADDRESS: u32 = 50528813u32;
pub const OID_FDDI_MAC_TOKEN_CT: u32 = 50528830u32;
pub const OID_FDDI_MAC_TRANSMIT_CT: u32 = 50528829u32;
pub const OID_FDDI_MAC_TVX_CAPABILITY: u32 = 50528802u32;
pub const OID_FDDI_MAC_TVX_EXPIRED_CT: u32 = 50528833u32;
pub const OID_FDDI_MAC_TVX_VALUE: u32 = 50528819u32;
pub const OID_FDDI_MAC_T_MAX: u32 = 50528818u32;
pub const OID_FDDI_MAC_T_MAX_CAPABILITY: u32 = 50528801u32;
pub const OID_FDDI_MAC_T_NEG: u32 = 50528817u32;
pub const OID_FDDI_MAC_T_PRI0: u32 = 50528820u32;
pub const OID_FDDI_MAC_T_PRI1: u32 = 50528821u32;
pub const OID_FDDI_MAC_T_PRI2: u32 = 50528822u32;
pub const OID_FDDI_MAC_T_PRI3: u32 = 50528823u32;
pub const OID_FDDI_MAC_T_PRI4: u32 = 50528824u32;
pub const OID_FDDI_MAC_T_PRI5: u32 = 50528825u32;
pub const OID_FDDI_MAC_T_PRI6: u32 = 50528826u32;
pub const OID_FDDI_MAC_T_REQ: u32 = 50528816u32;
pub const OID_FDDI_MAC_UNDA_FLAG: u32 = 50528843u32;
pub const OID_FDDI_MAC_UPSTREAM_NBR: u32 = 50528805u32;
pub const OID_FDDI_PATH_CONFIGURATION: u32 = 50528854u32;
pub const OID_FDDI_PATH_INDEX: u32 = 50528849u32;
pub const OID_FDDI_PATH_MAX_T_REQ: u32 = 50528859u32;
pub const OID_FDDI_PATH_RING_LATENCY: u32 = 50528850u32;
pub const OID_FDDI_PATH_SBA_AVAILABLE: u32 = 50528856u32;
pub const OID_FDDI_PATH_SBA_OVERHEAD: u32 = 50528853u32;
pub const OID_FDDI_PATH_SBA_PAYLOAD: u32 = 50528852u32;
pub const OID_FDDI_PATH_TRACE_STATUS: u32 = 50528851u32;
pub const OID_FDDI_PATH_TVX_LOWER_BOUND: u32 = 50528857u32;
pub const OID_FDDI_PATH_T_MAX_LOWER_BOUND: u32 = 50528858u32;
pub const OID_FDDI_PATH_T_R_MODE: u32 = 50528855u32;
pub const OID_FDDI_PORT_ACTION: u32 = 50528888u32;
pub const OID_FDDI_PORT_AVAILABLE_PATHS: u32 = 50528867u32;
pub const OID_FDDI_PORT_BS_FLAG: u32 = 50528873u32;
pub const OID_FDDI_PORT_CONNECTION_CAPABILITIES: u32 = 50528870u32;
pub const OID_FDDI_PORT_CONNECTION_POLICIES: u32 = 50528862u32;
pub const OID_FDDI_PORT_CONNNECT_STATE: u32 = 50528882u32;
pub const OID_FDDI_PORT_CURRENT_PATH: u32 = 50528864u32;
pub const OID_FDDI_PORT_EB_ERROR_CT: u32 = 50528875u32;
pub const OID_FDDI_PORT_HARDWARE_PRESENT: u32 = 50528886u32;
pub const OID_FDDI_PORT_INDEX: u32 = 50528871u32;
pub const OID_FDDI_PORT_LCT_FAIL_CT: u32 = 50528876u32;
pub const OID_FDDI_PORT_LEM_CT: u32 = 50528879u32;
pub const OID_FDDI_PORT_LEM_REJECT_CT: u32 = 50528878u32;
pub const OID_FDDI_PORT_LER_ALARM: u32 = 50528881u32;
pub const OID_FDDI_PORT_LER_CUTOFF: u32 = 50528880u32;
pub const OID_FDDI_PORT_LER_ESTIMATE: u32 = 50528877u32;
pub const OID_FDDI_PORT_LER_FLAG: u32 = 50528885u32;
pub const OID_FDDI_PORT_MAC_INDICATED: u32 = 50528863u32;
pub const OID_FDDI_PORT_MAC_LOOP_TIME: u32 = 50528868u32;
pub const OID_FDDI_PORT_MAC_PLACEMENT: u32 = 50528866u32;
pub const OID_FDDI_PORT_MAINT_LS: u32 = 50528872u32;
pub const OID_FDDI_PORT_MY_TYPE: u32 = 50528860u32;
pub const OID_FDDI_PORT_NEIGHBOR_TYPE: u32 = 50528861u32;
pub const OID_FDDI_PORT_PCM_STATE: u32 = 50528883u32;
pub const OID_FDDI_PORT_PC_LS: u32 = 50528874u32;
pub const OID_FDDI_PORT_PC_WITHHOLD: u32 = 50528884u32;
pub const OID_FDDI_PORT_PMD_CLASS: u32 = 50528869u32;
pub const OID_FDDI_PORT_REQUESTED_PATHS: u32 = 50528865u32;
pub const OID_FDDI_RING_MGT_STATE: u32 = 50462982u32;
pub const OID_FDDI_SHORT_CURRENT_ADDR: u32 = 50397446u32;
pub const OID_FDDI_SHORT_MAX_LIST_SIZE: u32 = 50397448u32;
pub const OID_FDDI_SHORT_MULTICAST_LIST: u32 = 50397447u32;
pub const OID_FDDI_SHORT_PERMANENT_ADDR: u32 = 50397445u32;
pub const OID_FDDI_SMT_AVAILABLE_PATHS: u32 = 50528779u32;
pub const OID_FDDI_SMT_BYPASS_PRESENT: u32 = 50528788u32;
pub const OID_FDDI_SMT_CF_STATE: u32 = 50528790u32;
pub const OID_FDDI_SMT_CONFIG_CAPABILITIES: u32 = 50528780u32;
pub const OID_FDDI_SMT_CONFIG_POLICY: u32 = 50528781u32;
pub const OID_FDDI_SMT_CONNECTION_POLICY: u32 = 50528782u32;
pub const OID_FDDI_SMT_ECM_STATE: u32 = 50528789u32;
pub const OID_FDDI_SMT_HI_VERSION_ID: u32 = 50528771u32;
pub const OID_FDDI_SMT_HOLD_STATE: u32 = 50528791u32;
pub const OID_FDDI_SMT_LAST_SET_STATION_ID: u32 = 50528798u32;
pub const OID_FDDI_SMT_LO_VERSION_ID: u32 = 50528772u32;
pub const OID_FDDI_SMT_MAC_CT: u32 = 50528776u32;
pub const OID_FDDI_SMT_MAC_INDEXES: u32 = 50528787u32;
pub const OID_FDDI_SMT_MANUFACTURER_DATA: u32 = 50528773u32;
pub const OID_FDDI_SMT_MASTER_CT: u32 = 50528778u32;
pub const OID_FDDI_SMT_MIB_VERSION_ID: u32 = 50528775u32;
pub const OID_FDDI_SMT_MSG_TIME_STAMP: u32 = 50528795u32;
pub const OID_FDDI_SMT_NON_MASTER_CT: u32 = 50528777u32;
pub const OID_FDDI_SMT_OP_VERSION_ID: u32 = 50528770u32;
pub const OID_FDDI_SMT_PEER_WRAP_FLAG: u32 = 50528794u32;
pub const OID_FDDI_SMT_PORT_INDEXES: u32 = 50528786u32;
pub const OID_FDDI_SMT_REMOTE_DISCONNECT_FLAG: u32 = 50528792u32;
pub const OID_FDDI_SMT_SET_COUNT: u32 = 50528797u32;
pub const OID_FDDI_SMT_STATION_ACTION: u32 = 50528887u32;
pub const OID_FDDI_SMT_STATION_ID: u32 = 50528769u32;
pub const OID_FDDI_SMT_STATION_STATUS: u32 = 50528793u32;
pub const OID_FDDI_SMT_STAT_RPT_POLICY: u32 = 50528784u32;
pub const OID_FDDI_SMT_TRACE_MAX_EXPIRATION: u32 = 50528785u32;
pub const OID_FDDI_SMT_TRANSITION_TIME_STAMP: u32 = 50528796u32;
pub const OID_FDDI_SMT_T_NOTIFY: u32 = 50528783u32;
pub const OID_FDDI_SMT_USER_DATA: u32 = 50528774u32;
pub const OID_FDDI_UPSTREAM_NODE_LONG: u32 = 50462978u32;
pub const OID_FFP_ADAPTER_STATS: u32 = 4227990033u32;
pub const OID_FFP_CONTROL: u32 = 4227924498u32;
pub const OID_FFP_DATA: u32 = 4227924500u32;
pub const OID_FFP_DRIVER_STATS: u32 = 4227990032u32;
pub const OID_FFP_FLUSH: u32 = 4227924497u32;
pub const OID_FFP_PARAMS: u32 = 4227924499u32;
pub const OID_FFP_SUPPORT: u32 = 4227924496u32;
pub const OID_GEN_ADMIN_STATUS: u32 = 66184u32;
pub const OID_GEN_ALIAS: u32 = 66185u32;
pub const OID_GEN_BROADCAST_BYTES_RCV: u32 = 131595u32;
pub const OID_GEN_BROADCAST_BYTES_XMIT: u32 = 131589u32;
pub const OID_GEN_BROADCAST_FRAMES_RCV: u32 = 131596u32;
pub const OID_GEN_BROADCAST_FRAMES_XMIT: u32 = 131590u32;
pub const OID_GEN_BYTES_RCV: u32 = 131609u32;
pub const OID_GEN_BYTES_XMIT: u32 = 131610u32;
pub const OID_GEN_CO_BYTES_RCV: u32 = 131591u32;
pub const OID_GEN_CO_BYTES_XMIT: u32 = 131585u32;
pub const OID_GEN_CO_BYTES_XMIT_OUTSTANDING: u32 = 131617u32;
pub const OID_GEN_CO_DEVICE_PROFILE: u32 = 131602u32;
pub const OID_GEN_CO_DRIVER_VERSION: u32 = 65808u32;
pub const OID_GEN_CO_GET_NETCARD_TIME: u32 = 131600u32;
pub const OID_GEN_CO_GET_TIME_CAPS: u32 = 131599u32;
pub const OID_GEN_CO_HARDWARE_STATUS: u32 = 65794u32;
pub const OID_GEN_CO_LINK_SPEED: u32 = 65799u32;
pub const OID_GEN_CO_MAC_OPTIONS: u32 = 65811u32;
pub const OID_GEN_CO_MEDIA_CONNECT_STATUS: u32 = 65812u32;
pub const OID_GEN_CO_MEDIA_IN_USE: u32 = 65796u32;
pub const OID_GEN_CO_MEDIA_SUPPORTED: u32 = 65795u32;
pub const OID_GEN_CO_MINIMUM_LINK_SPEED: u32 = 131360u32;
pub const OID_GEN_CO_NETCARD_LOAD: u32 = 131601u32;
pub const OID_GEN_CO_PROTOCOL_OPTIONS: u32 = 65810u32;
pub const OID_GEN_CO_RCV_CRC_ERROR: u32 = 131597u32;
pub const OID_GEN_CO_RCV_PDUS_ERROR: u32 = 131332u32;
pub const OID_GEN_CO_RCV_PDUS_NO_BUFFER: u32 = 131333u32;
pub const OID_GEN_CO_RCV_PDUS_OK: u32 = 131330u32;
pub const OID_GEN_CO_SUPPORTED_GUIDS: u32 = 65815u32;
pub const OID_GEN_CO_SUPPORTED_LIST: u32 = 65793u32;
pub const OID_GEN_CO_TRANSMIT_QUEUE_LENGTH: u32 = 131598u32;
pub const OID_GEN_CO_VENDOR_DESCRIPTION: u32 = 65805u32;
pub const OID_GEN_CO_VENDOR_DRIVER_VERSION: u32 = 65814u32;
pub const OID_GEN_CO_VENDOR_ID: u32 = 65804u32;
pub const OID_GEN_CO_XMIT_PDUS_ERROR: u32 = 131331u32;
pub const OID_GEN_CO_XMIT_PDUS_OK: u32 = 131329u32;
pub const OID_GEN_CURRENT_LOOKAHEAD: u32 = 65807u32;
pub const OID_GEN_CURRENT_PACKET_FILTER: u32 = 65806u32;
pub const OID_GEN_DEVICE_PROFILE: u32 = 131602u32;
pub const OID_GEN_DIRECTED_BYTES_RCV: u32 = 131591u32;
pub const OID_GEN_DIRECTED_BYTES_XMIT: u32 = 131585u32;
pub const OID_GEN_DIRECTED_FRAMES_RCV: u32 = 131592u32;
pub const OID_GEN_DIRECTED_FRAMES_XMIT: u32 = 131586u32;
pub const OID_GEN_DISCONTINUITY_TIME: u32 = 66178u32;
pub const OID_GEN_DRIVER_VERSION: u32 = 65808u32;
pub const OID_GEN_ENUMERATE_PORTS: u32 = 66061u32;
pub const OID_GEN_FRIENDLY_NAME: u32 = 131606u32;
pub const OID_GEN_GET_NETCARD_TIME: u32 = 131600u32;
pub const OID_GEN_GET_TIME_CAPS: u32 = 131599u32;
pub const OID_GEN_HARDWARE_STATUS: u32 = 65794u32;
pub const OID_GEN_HD_SPLIT_CURRENT_CONFIG: u32 = 66080u32;
pub const OID_GEN_HD_SPLIT_PARAMETERS: u32 = 66078u32;
pub const OID_GEN_INIT_TIME_MS: u32 = 131603u32;
pub const OID_GEN_INTERFACE_INFO: u32 = 66183u32;
pub const OID_GEN_INTERRUPT_MODERATION: u32 = 66057u32;
pub const OID_GEN_IP_OPER_STATUS: u32 = 66189u32;
pub const OID_GEN_ISOLATION_PARAMETERS: u32 = 66304u32;
pub const OID_GEN_LAST_CHANGE: u32 = 66177u32;
pub const OID_GEN_LINK_PARAMETERS: u32 = 66056u32;
pub const OID_GEN_LINK_SPEED: u32 = 65799u32;
pub const OID_GEN_LINK_SPEED_EX: u32 = 66187u32;
pub const OID_GEN_LINK_STATE: u32 = 66055u32;
pub const OID_GEN_MACHINE_NAME: u32 = 66074u32;
pub const OID_GEN_MAC_ADDRESS: u32 = 66053u32;
pub const OID_GEN_MAC_OPTIONS: u32 = 65811u32;
pub const OID_GEN_MAXIMUM_FRAME_SIZE: u32 = 65798u32;
pub const OID_GEN_MAXIMUM_LOOKAHEAD: u32 = 65797u32;
pub const OID_GEN_MAXIMUM_SEND_PACKETS: u32 = 65813u32;
pub const OID_GEN_MAXIMUM_TOTAL_SIZE: u32 = 65809u32;
pub const OID_GEN_MAX_LINK_SPEED: u32 = 66054u32;
pub const OID_GEN_MEDIA_CAPABILITIES: u32 = 66049u32;
pub const OID_GEN_MEDIA_CONNECT_STATUS: u32 = 65812u32;
pub const OID_GEN_MEDIA_CONNECT_STATUS_EX: u32 = 66186u32;
pub const OID_GEN_MEDIA_DUPLEX_STATE: u32 = 66188u32;
pub const OID_GEN_MEDIA_IN_USE: u32 = 65796u32;
pub const OID_GEN_MEDIA_SENSE_COUNTS: u32 = 131605u32;
pub const OID_GEN_MEDIA_SUPPORTED: u32 = 65795u32;
pub const OID_GEN_MINIPORT_RESTART_ATTRIBUTES: u32 = 66077u32;
pub const OID_GEN_MULTICAST_BYTES_RCV: u32 = 131593u32;
pub const OID_GEN_MULTICAST_BYTES_XMIT: u32 = 131587u32;
pub const OID_GEN_MULTICAST_FRAMES_RCV: u32 = 131594u32;
pub const OID_GEN_MULTICAST_FRAMES_XMIT: u32 = 131588u32;
pub const OID_GEN_NDIS_RESERVED_1: u32 = 131607u32;
pub const OID_GEN_NDIS_RESERVED_2: u32 = 131608u32;
pub const OID_GEN_NDIS_RESERVED_3: u32 = 66058u32;
pub const OID_GEN_NDIS_RESERVED_4: u32 = 66059u32;
pub const OID_GEN_NDIS_RESERVED_5: u32 = 66060u32;
pub const OID_GEN_NDIS_RESERVED_6: u32 = 66066u32;
pub const OID_GEN_NDIS_RESERVED_7: u32 = 131614u32;
pub const OID_GEN_NETCARD_LOAD: u32 = 131601u32;
pub const OID_GEN_NETWORK_LAYER_ADDRESSES: u32 = 65816u32;
pub const OID_GEN_OPERATIONAL_STATUS: u32 = 66179u32;
pub const OID_GEN_PACKET_MONITOR: u32 = 66257u32;
pub const OID_GEN_PCI_DEVICE_CUSTOM_PROPERTIES: u32 = 66065u32;
pub const OID_GEN_PHYSICAL_MEDIUM: u32 = 66050u32;
pub const OID_GEN_PHYSICAL_MEDIUM_EX: u32 = 66067u32;
pub const OID_GEN_PORT_AUTHENTICATION_PARAMETERS: u32 = 66063u32;
pub const OID_GEN_PORT_STATE: u32 = 66062u32;
pub const OID_GEN_PROMISCUOUS_MODE: u32 = 66176u32;
pub const OID_GEN_PROTOCOL_OPTIONS: u32 = 65810u32;
pub const OID_GEN_RCV_CRC_ERROR: u32 = 131597u32;
pub const OID_GEN_RCV_DISCARDS: u32 = 131611u32;
pub const OID_GEN_RCV_ERROR: u32 = 131332u32;
pub const OID_GEN_RCV_LINK_SPEED: u32 = 66181u32;
pub const OID_GEN_RCV_NO_BUFFER: u32 = 131333u32;
pub const OID_GEN_RCV_OK: u32 = 131330u32;
pub const OID_GEN_RECEIVE_BLOCK_SIZE: u32 = 65803u32;
pub const OID_GEN_RECEIVE_BUFFER_SPACE: u32 = 65801u32;
pub const OID_GEN_RECEIVE_HASH: u32 = 66079u32;
pub const OID_GEN_RECEIVE_SCALE_CAPABILITIES: u32 = 66051u32;
pub const OID_GEN_RECEIVE_SCALE_PARAMETERS: u32 = 66052u32;
pub const OID_GEN_RECEIVE_SCALE_PARAMETERS_V2: u32 = 66068u32;
pub const OID_GEN_RESET_COUNTS: u32 = 131604u32;
pub const OID_GEN_RNDIS_CONFIG_PARAMETER: u32 = 66075u32;
pub const OID_GEN_RSS_SET_INDIRECTION_TABLE_ENTRIES: u32 = 66240u32;
pub const OID_GEN_STATISTICS: u32 = 131334u32;
pub const OID_GEN_SUPPORTED_GUIDS: u32 = 65815u32;
pub const OID_GEN_SUPPORTED_LIST: u32 = 65793u32;
pub const OID_GEN_TIMEOUT_DPC_REQUEST_CAPABILITIES: u32 = 66064u32;
pub const OID_GEN_TRANSMIT_BLOCK_SIZE: u32 = 65802u32;
pub const OID_GEN_TRANSMIT_BUFFER_SPACE: u32 = 65800u32;
pub const OID_GEN_TRANSMIT_QUEUE_LENGTH: u32 = 131598u32;
pub const OID_GEN_TRANSPORT_HEADER_OFFSET: u32 = 65817u32;
pub const OID_GEN_UNKNOWN_PROTOS: u32 = 66182u32;
pub const OID_GEN_VENDOR_DESCRIPTION: u32 = 65805u32;
pub const OID_GEN_VENDOR_DRIVER_VERSION: u32 = 65814u32;
pub const OID_GEN_VENDOR_ID: u32 = 65804u32;
pub const OID_GEN_VLAN_ID: u32 = 66076u32;
pub const OID_GEN_XMIT_DISCARDS: u32 = 131612u32;
pub const OID_GEN_XMIT_ERROR: u32 = 131331u32;
pub const OID_GEN_XMIT_LINK_SPEED: u32 = 66180u32;
pub const OID_GEN_XMIT_OK: u32 = 131329u32;
pub const OID_GFT_ACTIVATE_FLOW_ENTRIES: u32 = 66575u32;
pub const OID_GFT_ADD_FLOW_ENTRIES: u32 = 66572u32;
pub const OID_GFT_ALLOCATE_COUNTERS: u32 = 66567u32;
pub const OID_GFT_COUNTER_VALUES: u32 = 66570u32;
pub const OID_GFT_CREATE_LOGICAL_VPORT: u32 = 66584u32;
pub const OID_GFT_CREATE_TABLE: u32 = 66564u32;
pub const OID_GFT_CURRENT_CAPABILITIES: u32 = 66562u32;
pub const OID_GFT_DEACTIVATE_FLOW_ENTRIES: u32 = 66576u32;
pub const OID_GFT_DELETE_FLOW_ENTRIES: u32 = 66573u32;
pub const OID_GFT_DELETE_LOGICAL_VPORT: u32 = 66585u32;
pub const OID_GFT_DELETE_PROFILE: u32 = 66582u32;
pub const OID_GFT_DELETE_TABLE: u32 = 66565u32;
pub const OID_GFT_ENUM_COUNTERS: u32 = 66569u32;
pub const OID_GFT_ENUM_FLOW_ENTRIES: u32 = 66574u32;
pub const OID_GFT_ENUM_LOGICAL_VPORTS: u32 = 66586u32;
pub const OID_GFT_ENUM_PROFILES: u32 = 66581u32;
pub const OID_GFT_ENUM_TABLES: u32 = 66566u32;
pub const OID_GFT_EXACT_MATCH_PROFILE: u32 = 66578u32;
pub const OID_GFT_FLOW_ENTRY_PARAMETERS: u32 = 66577u32;
pub const OID_GFT_FREE_COUNTERS: u32 = 66568u32;
pub const OID_GFT_GLOBAL_PARAMETERS: u32 = 66563u32;
pub const OID_GFT_HARDWARE_CAPABILITIES: u32 = 66561u32;
pub const OID_GFT_HEADER_TRANSPOSITION_PROFILE: u32 = 66579u32;
pub const OID_GFT_STATISTICS: u32 = 66571u32;
pub const OID_GFT_VPORT_PARAMETERS: u32 = 66583u32;
pub const OID_GFT_WILDCARD_MATCH_PROFILE: u32 = 66580u32;
pub const OID_IP4_OFFLOAD_STATS: u32 = 4227924489u32;
pub const OID_IP6_OFFLOAD_STATS: u32 = 4227924490u32;
pub const OID_IRDA_EXTRA_RCV_BOFS: u32 = 167838208u32;
pub const OID_IRDA_LINK_SPEED: u32 = 167837955u32;
pub const OID_IRDA_MAX_RECEIVE_WINDOW_SIZE: u32 = 167838212u32;
pub const OID_IRDA_MAX_SEND_WINDOW_SIZE: u32 = 167838213u32;
pub const OID_IRDA_MAX_UNICAST_LIST_SIZE: u32 = 167838211u32;
pub const OID_IRDA_MEDIA_BUSY: u32 = 167837956u32;
pub const OID_IRDA_RATE_SNIFF: u32 = 167838209u32;
pub const OID_IRDA_RECEIVING: u32 = 167837952u32;
pub const OID_IRDA_RESERVED1: u32 = 167838218u32;
pub const OID_IRDA_RESERVED2: u32 = 167838223u32;
pub const OID_IRDA_SUPPORTED_SPEEDS: u32 = 167837954u32;
pub const OID_IRDA_TURNAROUND_TIME: u32 = 167837953u32;
pub const OID_IRDA_UNICAST_LIST: u32 = 167838210u32;
pub const OID_KDNET_ADD_PF: u32 = 131619u32;
pub const OID_KDNET_ENUMERATE_PFS: u32 = 131618u32;
pub const OID_KDNET_QUERY_PF_INFORMATION: u32 = 131621u32;
pub const OID_KDNET_REMOVE_PF: u32 = 131620u32;
pub const OID_LTALK_COLLISIONS: u32 = 84017666u32;
pub const OID_LTALK_CURRENT_NODE_ID: u32 = 83951874u32;
pub const OID_LTALK_DEFERS: u32 = 84017667u32;
pub const OID_LTALK_FCS_ERRORS: u32 = 84017670u32;
pub const OID_LTALK_IN_BROADCASTS: u32 = 84017409u32;
pub const OID_LTALK_IN_LENGTH_ERRORS: u32 = 84017410u32;
pub const OID_LTALK_NO_DATA_ERRORS: u32 = 84017668u32;
pub const OID_LTALK_OUT_NO_HANDLERS: u32 = 84017665u32;
pub const OID_LTALK_RANDOM_CTS_ERRORS: u32 = 84017669u32;
pub const OID_NDK_CONNECTIONS: u32 = 4228121091u32;
pub const OID_NDK_LOCAL_ENDPOINTS: u32 = 4228121092u32;
pub const OID_NDK_SET_STATE: u32 = 4228121089u32;
pub const OID_NDK_STATISTICS: u32 = 4228121090u32;
pub const OID_NIC_SWITCH_ALLOCATE_VF: u32 = 66117u32;
pub const OID_NIC_SWITCH_CREATE_SWITCH: u32 = 66103u32;
pub const OID_NIC_SWITCH_CREATE_VPORT: u32 = 66113u32;
pub const OID_NIC_SWITCH_CURRENT_CAPABILITIES: u32 = 66095u32;
pub const OID_NIC_SWITCH_DELETE_SWITCH: u32 = 66105u32;
pub const OID_NIC_SWITCH_DELETE_VPORT: u32 = 66116u32;
pub const OID_NIC_SWITCH_ENUM_SWITCHES: u32 = 66112u32;
pub const OID_NIC_SWITCH_ENUM_VFS: u32 = 66120u32;
pub const OID_NIC_SWITCH_ENUM_VPORTS: u32 = 66115u32;
pub const OID_NIC_SWITCH_FREE_VF: u32 = 66118u32;
pub const OID_NIC_SWITCH_HARDWARE_CAPABILITIES: u32 = 66094u32;
pub const OID_NIC_SWITCH_PARAMETERS: u32 = 66104u32;
pub const OID_NIC_SWITCH_VF_PARAMETERS: u32 = 66119u32;
pub const OID_NIC_SWITCH_VPORT_PARAMETERS: u32 = 66114u32;
pub const OID_OFFLOAD_ENCAPSULATION: u32 = 16843018u32;
pub const OID_PACKET_COALESCING_FILTER_MATCH_COUNT: u32 = 66101u32;
pub const OID_PD_CLOSE_PROVIDER: u32 = 66818u32;
pub const OID_PD_OPEN_PROVIDER: u32 = 66817u32;
pub const OID_PD_QUERY_CURRENT_CONFIG: u32 = 66819u32;
pub const OID_PM_ADD_PROTOCOL_OFFLOAD: u32 = 4244701453u32;
pub const OID_PM_ADD_WOL_PATTERN: u32 = 4244701450u32;
pub const OID_PM_CURRENT_CAPABILITIES: u32 = 4244701447u32;
pub const OID_PM_GET_PROTOCOL_OFFLOAD: u32 = 4244701454u32;
pub const OID_PM_HARDWARE_CAPABILITIES: u32 = 4244701448u32;
pub const OID_PM_PARAMETERS: u32 = 4244701449u32;
pub const OID_PM_PROTOCOL_OFFLOAD_LIST: u32 = 4244701456u32;
pub const OID_PM_REMOVE_PROTOCOL_OFFLOAD: u32 = 4244701455u32;
pub const OID_PM_REMOVE_WOL_PATTERN: u32 = 4244701451u32;
pub const OID_PM_RESERVED_1: u32 = 4244701457u32;
pub const OID_PM_WOL_PATTERN_LIST: u32 = 4244701452u32;
pub const OID_PNP_ADD_WAKE_UP_PATTERN: u32 = 4244701443u32;
pub const OID_PNP_CAPABILITIES: u32 = 4244701440u32;
pub const OID_PNP_ENABLE_WAKE_UP: u32 = 4244701446u32;
pub const OID_PNP_QUERY_POWER: u32 = 4244701442u32;
pub const OID_PNP_REMOVE_WAKE_UP_PATTERN: u32 = 4244701444u32;
pub const OID_PNP_SET_POWER: u32 = 4244701441u32;
pub const OID_PNP_WAKE_UP_ERROR: u32 = 4244767233u32;
pub const OID_PNP_WAKE_UP_OK: u32 = 4244767232u32;
pub const OID_PNP_WAKE_UP_PATTERN_LIST: u32 = 4244701445u32;
pub const OID_QOS_CURRENT_CAPABILITIES: u32 = 4228186114u32;
pub const OID_QOS_HARDWARE_CAPABILITIES: u32 = 4228186113u32;
pub const OID_QOS_OFFLOAD_CREATE_SQ: u32 = 67075u32;
pub const OID_QOS_OFFLOAD_CURRENT_CAPABILITIES: u32 = 67074u32;
pub const OID_QOS_OFFLOAD_DELETE_SQ: u32 = 67076u32;
pub const OID_QOS_OFFLOAD_ENUM_SQS: u32 = 67078u32;
pub const OID_QOS_OFFLOAD_HARDWARE_CAPABILITIES: u32 = 67073u32;
pub const OID_QOS_OFFLOAD_SQ_STATS: u32 = 67079u32;
pub const OID_QOS_OFFLOAD_UPDATE_SQ: u32 = 67077u32;
pub const OID_QOS_OPERATIONAL_PARAMETERS: u32 = 4228186116u32;
pub const OID_QOS_PARAMETERS: u32 = 4228186115u32;
pub const OID_QOS_REMOTE_PARAMETERS: u32 = 4228186117u32;
pub const OID_QOS_RESERVED1: u32 = 4211147008u32;
pub const OID_QOS_RESERVED10: u32 = 4211147017u32;
pub const OID_QOS_RESERVED11: u32 = 4211147018u32;
pub const OID_QOS_RESERVED12: u32 = 4211147019u32;
pub const OID_QOS_RESERVED13: u32 = 4211147020u32;
pub const OID_QOS_RESERVED14: u32 = 4211147021u32;
pub const OID_QOS_RESERVED15: u32 = 4211147022u32;
pub const OID_QOS_RESERVED16: u32 = 4211147023u32;
pub const OID_QOS_RESERVED17: u32 = 4211147024u32;
pub const OID_QOS_RESERVED18: u32 = 4211147025u32;
pub const OID_QOS_RESERVED19: u32 = 4211147026u32;
pub const OID_QOS_RESERVED2: u32 = 4211147009u32;
pub const OID_QOS_RESERVED20: u32 = 4211147027u32;
pub const OID_QOS_RESERVED3: u32 = 4211147010u32;
pub const OID_QOS_RESERVED4: u32 = 4211147011u32;
pub const OID_QOS_RESERVED5: u32 = 4211147012u32;
pub const OID_QOS_RESERVED6: u32 = 4211147013u32;
pub const OID_QOS_RESERVED7: u32 = 4211147014u32;
pub const OID_QOS_RESERVED8: u32 = 4211147015u32;
pub const OID_QOS_RESERVED9: u32 = 4211147016u32;
pub const OID_RECEIVE_FILTER_ALLOCATE_QUEUE: u32 = 66083u32;
pub const OID_RECEIVE_FILTER_CLEAR_FILTER: u32 = 66088u32;
pub const OID_RECEIVE_FILTER_CURRENT_CAPABILITIES: u32 = 66093u32;
pub const OID_RECEIVE_FILTER_ENUM_FILTERS: u32 = 66089u32;
pub const OID_RECEIVE_FILTER_ENUM_QUEUES: u32 = 66085u32;
pub const OID_RECEIVE_FILTER_FREE_QUEUE: u32 = 66084u32;
pub const OID_RECEIVE_FILTER_GLOBAL_PARAMETERS: u32 = 66082u32;
pub const OID_RECEIVE_FILTER_HARDWARE_CAPABILITIES: u32 = 66081u32;
pub const OID_RECEIVE_FILTER_MOVE_FILTER: u32 = 66096u32;
pub const OID_RECEIVE_FILTER_PARAMETERS: u32 = 66090u32;
pub const OID_RECEIVE_FILTER_QUEUE_ALLOCATION_COMPLETE: u32 = 66091u32;
pub const OID_RECEIVE_FILTER_QUEUE_PARAMETERS: u32 = 66086u32;
pub const OID_RECEIVE_FILTER_SET_FILTER: u32 = 66087u32;
pub const OID_SRIOV_BAR_RESOURCES: u32 = 66137u32;
pub const OID_SRIOV_CONFIG_STATE: u32 = 66145u32;
pub const OID_SRIOV_CURRENT_CAPABILITIES: u32 = 66128u32;
pub const OID_SRIOV_HARDWARE_CAPABILITIES: u32 = 66121u32;
pub const OID_SRIOV_OVERLYING_ADAPTER_INFO: u32 = 66152u32;
pub const OID_SRIOV_PF_LUID: u32 = 66144u32;
pub const OID_SRIOV_PROBED_BARS: u32 = 66136u32;
pub const OID_SRIOV_READ_VF_CONFIG_BLOCK: u32 = 66131u32;
pub const OID_SRIOV_READ_VF_CONFIG_SPACE: u32 = 66129u32;
pub const OID_SRIOV_RESET_VF: u32 = 66133u32;
pub const OID_SRIOV_SET_VF_POWER_STATE: u32 = 66134u32;
pub const OID_SRIOV_VF_INVALIDATE_CONFIG_BLOCK: u32 = 66153u32;
pub const OID_SRIOV_VF_SERIAL_NUMBER: u32 = 66146u32;
pub const OID_SRIOV_VF_VENDOR_DEVICE_ID: u32 = 66135u32;
pub const OID_SRIOV_WRITE_VF_CONFIG_BLOCK: u32 = 66132u32;
pub const OID_SRIOV_WRITE_VF_CONFIG_SPACE: u32 = 66130u32;
pub const OID_SWITCH_FEATURE_STATUS_QUERY: u32 = 66151u32;
pub const OID_SWITCH_NIC_ARRAY: u32 = 66167u32;
pub const OID_SWITCH_NIC_CONNECT: u32 = 66171u32;
pub const OID_SWITCH_NIC_CREATE: u32 = 66170u32;
pub const OID_SWITCH_NIC_DELETE: u32 = 66173u32;
pub const OID_SWITCH_NIC_DIRECT_REQUEST: u32 = 66198u32;
pub const OID_SWITCH_NIC_DISCONNECT: u32 = 66172u32;
pub const OID_SWITCH_NIC_REQUEST: u32 = 66160u32;
pub const OID_SWITCH_NIC_RESTORE: u32 = 66194u32;
pub const OID_SWITCH_NIC_RESTORE_COMPLETE: u32 = 66195u32;
pub const OID_SWITCH_NIC_RESUME: u32 = 66200u32;
pub const OID_SWITCH_NIC_SAVE: u32 = 66192u32;
pub const OID_SWITCH_NIC_SAVE_COMPLETE: u32 = 66193u32;
pub const OID_SWITCH_NIC_SUSPEND: u32 = 66199u32;
pub const OID_SWITCH_NIC_SUSPENDED_LM_SOURCE_FINISHED: u32 = 66202u32;
pub const OID_SWITCH_NIC_SUSPENDED_LM_SOURCE_STARTED: u32 = 66201u32;
pub const OID_SWITCH_NIC_UPDATED: u32 = 66196u32;
pub const OID_SWITCH_PARAMETERS: u32 = 66165u32;
pub const OID_SWITCH_PORT_ARRAY: u32 = 66166u32;
pub const OID_SWITCH_PORT_CREATE: u32 = 66168u32;
pub const OID_SWITCH_PORT_DELETE: u32 = 66169u32;
pub const OID_SWITCH_PORT_FEATURE_STATUS_QUERY: u32 = 66174u32;
pub const OID_SWITCH_PORT_PROPERTY_ADD: u32 = 66161u32;
pub const OID_SWITCH_PORT_PROPERTY_DELETE: u32 = 66163u32;
pub const OID_SWITCH_PORT_PROPERTY_ENUM: u32 = 66164u32;
pub const OID_SWITCH_PORT_PROPERTY_UPDATE: u32 = 66162u32;
pub const OID_SWITCH_PORT_TEARDOWN: u32 = 66175u32;
pub const OID_SWITCH_PORT_UPDATED: u32 = 66197u32;
pub const OID_SWITCH_PROPERTY_ADD: u32 = 66147u32;
pub const OID_SWITCH_PROPERTY_DELETE: u32 = 66149u32;
pub const OID_SWITCH_PROPERTY_ENUM: u32 = 66150u32;
pub const OID_SWITCH_PROPERTY_UPDATE: u32 = 66148u32;
pub const OID_TAPI_ACCEPT: u32 = 117637377u32;
pub const OID_TAPI_ANSWER: u32 = 117637378u32;
pub const OID_TAPI_CLOSE: u32 = 117637379u32;
pub const OID_TAPI_CLOSE_CALL: u32 = 117637380u32;
pub const OID_TAPI_CONDITIONAL_MEDIA_DETECTION: u32 = 117637381u32;
pub const OID_TAPI_CONFIG_DIALOG: u32 = 117637382u32;
pub const OID_TAPI_DEV_SPECIFIC: u32 = 117637383u32;
pub const OID_TAPI_DIAL: u32 = 117637384u32;
pub const OID_TAPI_DROP: u32 = 117637385u32;
pub const OID_TAPI_GATHER_DIGITS: u32 = 117637411u32;
pub const OID_TAPI_GET_ADDRESS_CAPS: u32 = 117637386u32;
pub const OID_TAPI_GET_ADDRESS_ID: u32 = 117637387u32;
pub const OID_TAPI_GET_ADDRESS_STATUS: u32 = 117637388u32;
pub const OID_TAPI_GET_CALL_ADDRESS_ID: u32 = 117637389u32;
pub const OID_TAPI_GET_CALL_INFO: u32 = 117637390u32;
pub const OID_TAPI_GET_CALL_STATUS: u32 = 117637391u32;
pub const OID_TAPI_GET_DEV_CAPS: u32 = 117637392u32;
pub const OID_TAPI_GET_DEV_CONFIG: u32 = 117637393u32;
pub const OID_TAPI_GET_EXTENSION_ID: u32 = 117637394u32;
pub const OID_TAPI_GET_ID: u32 = 117637395u32;
pub const OID_TAPI_GET_LINE_DEV_STATUS: u32 = 117637396u32;
pub const OID_TAPI_MAKE_CALL: u32 = 117637397u32;
pub const OID_TAPI_MONITOR_DIGITS: u32 = 117637412u32;
pub const OID_TAPI_NEGOTIATE_EXT_VERSION: u32 = 117637398u32;
pub const OID_TAPI_OPEN: u32 = 117637399u32;
pub const OID_TAPI_PROVIDER_INITIALIZE: u32 = 117637400u32;
pub const OID_TAPI_PROVIDER_SHUTDOWN: u32 = 117637401u32;
pub const OID_TAPI_SECURE_CALL: u32 = 117637402u32;
pub const OID_TAPI_SELECT_EXT_VERSION: u32 = 117637403u32;
pub const OID_TAPI_SEND_USER_USER_INFO: u32 = 117637404u32;
pub const OID_TAPI_SET_APP_SPECIFIC: u32 = 117637405u32;
pub const OID_TAPI_SET_CALL_PARAMS: u32 = 117637406u32;
pub const OID_TAPI_SET_DEFAULT_MEDIA_DETECTION: u32 = 117637407u32;
pub const OID_TAPI_SET_DEV_CONFIG: u32 = 117637408u32;
pub const OID_TAPI_SET_MEDIA_MODE: u32 = 117637409u32;
pub const OID_TAPI_SET_STATUS_MESSAGES: u32 = 117637410u32;
pub const OID_TCP4_OFFLOAD_STATS: u32 = 4227924487u32;
pub const OID_TCP6_OFFLOAD_STATS: u32 = 4227924488u32;
pub const OID_TCP_CONNECTION_OFFLOAD_CURRENT_CONFIG: u32 = 4227924494u32;
pub const OID_TCP_CONNECTION_OFFLOAD_HARDWARE_CAPABILITIES: u32 = 4227924495u32;
pub const OID_TCP_CONNECTION_OFFLOAD_PARAMETERS: u32 = 4228055553u32;
pub const OID_TCP_OFFLOAD_CURRENT_CONFIG: u32 = 4227924491u32;
pub const OID_TCP_OFFLOAD_HARDWARE_CAPABILITIES: u32 = 4227924493u32;
pub const OID_TCP_OFFLOAD_PARAMETERS: u32 = 4227924492u32;
pub const OID_TCP_RSC_STATISTICS: u32 = 131613u32;
pub const OID_TCP_SAN_SUPPORT: u32 = 4227924484u32;
pub const OID_TCP_TASK_IPSEC_ADD_SA: u32 = 4227924482u32;
pub const OID_TCP_TASK_IPSEC_ADD_UDPESP_SA: u32 = 4227924485u32;
pub const OID_TCP_TASK_IPSEC_DELETE_SA: u32 = 4227924483u32;
pub const OID_TCP_TASK_IPSEC_DELETE_UDPESP_SA: u32 = 4227924486u32;
pub const OID_TCP_TASK_IPSEC_OFFLOAD_V2_ADD_SA: u32 = 4228055554u32;
pub const OID_TCP_TASK_IPSEC_OFFLOAD_V2_ADD_SA_EX: u32 = 4228055557u32;
pub const OID_TCP_TASK_IPSEC_OFFLOAD_V2_DELETE_SA: u32 = 4228055555u32;
pub const OID_TCP_TASK_IPSEC_OFFLOAD_V2_UPDATE_SA: u32 = 4228055556u32;
pub const OID_TCP_TASK_OFFLOAD: u32 = 4227924481u32;
pub const OID_TIMESTAMP_CAPABILITY: u32 = 10485761u32;
pub const OID_TIMESTAMP_CURRENT_CONFIG: u32 = 10485762u32;
pub const OID_TIMESTAMP_GET_CROSSTIMESTAMP: u32 = 10485763u32;
pub const OID_TUNNEL_INTERFACE_RELEASE_OID: u32 = 251724039u32;
pub const OID_TUNNEL_INTERFACE_SET_OID: u32 = 251724038u32;
pub const OID_VLAN_RESERVED1: u32 = 66097u32;
pub const OID_VLAN_RESERVED2: u32 = 66098u32;
pub const OID_VLAN_RESERVED3: u32 = 66099u32;
pub const OID_VLAN_RESERVED4: u32 = 66100u32;
pub const OID_WAN_CO_GET_COMP_INFO: u32 = 67175040u32;
pub const OID_WAN_CO_GET_INFO: u32 = 67174784u32;
pub const OID_WAN_CO_GET_LINK_INFO: u32 = 67174786u32;
pub const OID_WAN_CO_GET_STATS_INFO: u32 = 67175042u32;
pub const OID_WAN_CO_SET_COMP_INFO: u32 = 67175041u32;
pub const OID_WAN_CO_SET_LINK_INFO: u32 = 67174785u32;
pub const OID_WAN_CURRENT_ADDRESS: u32 = 67174658u32;
pub const OID_WAN_GET_BRIDGE_INFO: u32 = 67174922u32;
pub const OID_WAN_GET_COMP_INFO: u32 = 67174924u32;
pub const OID_WAN_GET_INFO: u32 = 67174663u32;
pub const OID_WAN_GET_LINK_INFO: u32 = 67174665u32;
pub const OID_WAN_GET_STATS_INFO: u32 = 67174926u32;
pub const OID_WAN_HEADER_FORMAT: u32 = 67174662u32;
pub const OID_WAN_LINE_COUNT: u32 = 67174666u32;
pub const OID_WAN_MEDIUM_SUBTYPE: u32 = 67174661u32;
pub const OID_WAN_PERMANENT_ADDRESS: u32 = 67174657u32;
pub const OID_WAN_PROTOCOL_CAPS: u32 = 67174667u32;
pub const OID_WAN_PROTOCOL_TYPE: u32 = 67174660u32;
pub const OID_WAN_QUALITY_OF_SERVICE: u32 = 67174659u32;
pub const OID_WAN_SET_BRIDGE_INFO: u32 = 67174923u32;
pub const OID_WAN_SET_COMP_INFO: u32 = 67174925u32;
pub const OID_WAN_SET_LINK_INFO: u32 = 67174664u32;
pub const OID_WWAN_AUTH_CHALLENGE: u32 = 234946837u32;
pub const OID_WWAN_BASE_STATIONS_INFO: u32 = 234946888u32;
pub const OID_WWAN_CONNECT: u32 = 234946828u32;
pub const OID_WWAN_CREATE_MAC: u32 = 234946854u32;
pub const OID_WWAN_DELETE_MAC: u32 = 234946855u32;
pub const OID_WWAN_DEVICE_BINDINGS: u32 = 234946865u32;
pub const OID_WWAN_DEVICE_CAPS: u32 = 234946817u32;
pub const OID_WWAN_DEVICE_CAPS_EX: u32 = 234946862u32;
pub const OID_WWAN_DEVICE_RESET: u32 = 234946887u32;
pub const OID_WWAN_DEVICE_SERVICE_COMMAND: u32 = 234946840u32;
pub const OID_WWAN_DEVICE_SERVICE_SESSION: u32 = 234946851u32;
pub const OID_WWAN_DEVICE_SERVICE_SESSION_WRITE: u32 = 234946852u32;
pub const OID_WWAN_DRIVER_CAPS: u32 = 234946816u32;
pub const OID_WWAN_ENUMERATE_DEVICE_SERVICES: u32 = 234946838u32;
pub const OID_WWAN_ENUMERATE_DEVICE_SERVICE_COMMANDS: u32 = 234946850u32;
pub const OID_WWAN_HOME_PROVIDER: u32 = 234946822u32;
pub const OID_WWAN_IMS_VOICE_STATE: u32 = 234946867u32;
pub const OID_WWAN_LOCATION_STATE: u32 = 234946869u32;
pub const OID_WWAN_LTE_ATTACH_CONFIG: u32 = 234946882u32;
pub const OID_WWAN_LTE_ATTACH_STATUS: u32 = 234946883u32;
pub const OID_WWAN_MBIM_VERSION: u32 = 234946860u32;
pub const OID_WWAN_MODEM_CONFIG_INFO: u32 = 234946884u32;
pub const OID_WWAN_MODEM_LOGGING_CONFIG: u32 = 234946891u32;
pub const OID_WWAN_MPDP: u32 = 234946889u32;
pub const OID_WWAN_NETWORK_BLACKLIST: u32 = 234946881u32;
pub const OID_WWAN_NETWORK_IDLE_HINT: u32 = 234946871u32;
pub const OID_WWAN_NETWORK_PARAMS: u32 = 234946893u32;
pub const OID_WWAN_NITZ: u32 = 234946870u32;
pub const OID_WWAN_PACKET_SERVICE: u32 = 234946826u32;
pub const OID_WWAN_PCO: u32 = 234946885u32;
pub const OID_WWAN_PIN: u32 = 234946820u32;
pub const OID_WWAN_PIN_EX: u32 = 234946849u32;
pub const OID_WWAN_PIN_EX2: u32 = 234946859u32;
pub const OID_WWAN_PIN_LIST: u32 = 234946821u32;
pub const OID_WWAN_PREFERRED_MULTICARRIER_PROVIDERS: u32 = 234946853u32;
pub const OID_WWAN_PREFERRED_PROVIDERS: u32 = 234946823u32;
pub const OID_WWAN_PRESHUTDOWN: u32 = 234946872u32;
pub const OID_WWAN_PROVISIONED_CONTEXTS: u32 = 234946829u32;
pub const OID_WWAN_PS_MEDIA_CONFIG: u32 = 234946878u32;
pub const OID_WWAN_RADIO_STATE: u32 = 234946819u32;
pub const OID_WWAN_READY_INFO: u32 = 234946818u32;
pub const OID_WWAN_REGISTER_PARAMS: u32 = 234946892u32;
pub const OID_WWAN_REGISTER_STATE: u32 = 234946825u32;
pub const OID_WWAN_REGISTER_STATE_EX: u32 = 234946866u32;
pub const OID_WWAN_SAR_CONFIG: u32 = 234946879u32;
pub const OID_WWAN_SAR_TRANSMISSION_STATUS: u32 = 234946880u32;
pub const OID_WWAN_SERVICE_ACTIVATION: u32 = 234946830u32;
pub const OID_WWAN_SIGNAL_STATE: u32 = 234946827u32;
pub const OID_WWAN_SIGNAL_STATE_EX: u32 = 234946868u32;
pub const OID_WWAN_SLOT_INFO_STATUS: u32 = 234946864u32;
pub const OID_WWAN_SMS_CONFIGURATION: u32 = 234946831u32;
pub const OID_WWAN_SMS_DELETE: u32 = 234946834u32;
pub const OID_WWAN_SMS_READ: u32 = 234946832u32;
pub const OID_WWAN_SMS_SEND: u32 = 234946833u32;
pub const OID_WWAN_SMS_STATUS: u32 = 234946835u32;
pub const OID_WWAN_SUBSCRIBE_DEVICE_SERVICE_EVENTS: u32 = 234946839u32;
pub const OID_WWAN_SYS_CAPS: u32 = 234946861u32;
pub const OID_WWAN_SYS_SLOTMAPPINGS: u32 = 234946863u32;
pub const OID_WWAN_UE_POLICY: u32 = 234946894u32;
pub const OID_WWAN_UICC_ACCESS_BINARY: u32 = 234946857u32;
pub const OID_WWAN_UICC_ACCESS_RECORD: u32 = 234946858u32;
pub const OID_WWAN_UICC_APDU: u32 = 234946876u32;
pub const OID_WWAN_UICC_APP_LIST: u32 = 234946890u32;
pub const OID_WWAN_UICC_ATR: u32 = 234946873u32;
pub const OID_WWAN_UICC_CLOSE_CHANNEL: u32 = 234946875u32;
pub const OID_WWAN_UICC_FILE_STATUS: u32 = 234946856u32;
pub const OID_WWAN_UICC_OPEN_CHANNEL: u32 = 234946874u32;
pub const OID_WWAN_UICC_RESET: u32 = 234946886u32;
pub const OID_WWAN_UICC_TERMINAL_CAPABILITY: u32 = 234946877u32;
pub const OID_WWAN_USSD: u32 = 234946841u32;
pub const OID_WWAN_VENDOR_SPECIFIC: u32 = 234946836u32;
pub const OID_WWAN_VISIBLE_PROVIDERS: u32 = 234946824u32;
pub const OID_XBOX_ACC_RESERVED0: u32 = 4194304000u32;
pub const OriginalNetBufferList: NDIS_PER_PACKET_INFO = NDIS_PER_PACKET_INFO(9i32);
pub const OriginalPacketInfo: NDIS_PER_PACKET_INFO = NDIS_PER_PACKET_INFO(7i32);
pub const PD_BUFFER_ATTR_BUILT_IN_DATA_BUFFER: u32 = 1u32;
pub const PD_BUFFER_FLAG_PARTIAL_PACKET_HEAD: u32 = 1u32;
pub const PD_BUFFER_MIN_RX_DATA_START_ALIGNMENT: u32 = 2u32;
pub const PD_BUFFER_MIN_RX_DATA_START_VALUE: u32 = 32u32;
pub const PD_BUFFER_MIN_TX_DATA_START_ALIGNMENT: u32 = 2u32;
pub const PERMANENT_VC: u32 = 1u32;
pub const PacketCancelId: NDIS_PER_PACKET_INFO = NDIS_PER_PACKET_INFO(8i32);
pub const QUERY_CALL_PARAMETERS: u32 = 4u32;
pub const READABLE_LOCAL_CLOCK: u32 = 1u32;
pub const RECEIVE_TIME_INDICATION: u32 = 1u32;
pub const RECEIVE_TIME_INDICATION_CAPABLE: u32 = 8u32;
pub const RECEIVE_VC: u32 = 8u32;
pub const RESERVE_RESOURCES_VC: u32 = 64u32;
pub const ROUND_DOWN_FLOW: u32 = 128u32;
pub const ROUND_UP_FLOW: u32 = 256u32;
pub const STRINGFORMAT_ASCII: u32 = 1u32;
pub const STRINGFORMAT_BINARY: u32 = 4u32;
pub const STRINGFORMAT_DBCS: u32 = 2u32;
pub const STRINGFORMAT_UNICODE: u32 = 3u32;
pub const ScatterGatherListPacketInfo: NDIS_PER_PACKET_INFO = NDIS_PER_PACKET_INFO(5i32);
pub const ShortPacketPaddingInfo: NDIS_PER_PACKET_INFO = NDIS_PER_PACKET_INFO(11i32);
pub const TIMED_SEND_CAPABLE: u32 = 16u32;
pub const TIME_STAMP_CAPABLE: u32 = 32u32;
pub const TRANSMIT_VC: u32 = 4u32;
pub const TRUNCATED_HASH_LEN: u32 = 12u32;
pub const TcpIpChecksumPacketInfo: NDIS_PER_PACKET_INFO = NDIS_PER_PACKET_INFO(0i32);
pub const TcpLargeSendPacketInfo: NDIS_PER_PACKET_INFO = NDIS_PER_PACKET_INFO(2i32);
pub const USE_TIME_STAMPS: u32 = 2u32;
pub const WAN_PROTOCOL_KEEPS_STATS: u32 = 1u32;
pub const fNDIS_GUID_ALLOW_READ: u32 = 32u32;
pub const fNDIS_GUID_ALLOW_WRITE: u32 = 64u32;
pub const fNDIS_GUID_ANSI_STRING: u32 = 4u32;
pub const fNDIS_GUID_ARRAY: u32 = 16u32;
pub const fNDIS_GUID_METHOD: u32 = 128u32;
pub const fNDIS_GUID_NDIS_RESERVED: u32 = 256u32;
pub const fNDIS_GUID_SUPPORT_COMMON_HEADER: u32 = 512u32;
pub const fNDIS_GUID_TO_OID: u32 = 1u32;
pub const fNDIS_GUID_TO_STATUS: u32 = 2u32;
pub const fNDIS_GUID_UNICODE_STRING: u32 = 8u32;
pub const fPACKET_ALLOCATED_BY_NDIS: u32 = 128u32;
pub const fPACKET_CONTAINS_MEDIA_SPECIFIC_INFO: u32 = 64u32;
pub const fPACKET_WRAPPER_RESERVED: u32 = 63u32;
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct NDIS_802_11_AUTHENTICATION_MODE(pub i32);
impl ::windows_core::TypeKind for NDIS_802_11_AUTHENTICATION_MODE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NDIS_802_11_AUTHENTICATION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_802_11_AUTHENTICATION_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct NDIS_802_11_MEDIA_STREAM_MODE(pub i32);
impl ::windows_core::TypeKind for NDIS_802_11_MEDIA_STREAM_MODE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NDIS_802_11_MEDIA_STREAM_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_802_11_MEDIA_STREAM_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct NDIS_802_11_NETWORK_INFRASTRUCTURE(pub i32);
impl ::windows_core::TypeKind for NDIS_802_11_NETWORK_INFRASTRUCTURE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NDIS_802_11_NETWORK_INFRASTRUCTURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_802_11_NETWORK_INFRASTRUCTURE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct NDIS_802_11_NETWORK_TYPE(pub i32);
impl ::windows_core::TypeKind for NDIS_802_11_NETWORK_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NDIS_802_11_NETWORK_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_802_11_NETWORK_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct NDIS_802_11_POWER_MODE(pub i32);
impl ::windows_core::TypeKind for NDIS_802_11_POWER_MODE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NDIS_802_11_POWER_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_802_11_POWER_MODE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct NDIS_802_11_PRIVACY_FILTER(pub i32);
impl ::windows_core::TypeKind for NDIS_802_11_PRIVACY_FILTER {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NDIS_802_11_PRIVACY_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_802_11_PRIVACY_FILTER").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct NDIS_802_11_RADIO_STATUS(pub i32);
impl ::windows_core::TypeKind for NDIS_802_11_RADIO_STATUS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NDIS_802_11_RADIO_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_802_11_RADIO_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct NDIS_802_11_RELOAD_DEFAULTS(pub i32);
impl ::windows_core::TypeKind for NDIS_802_11_RELOAD_DEFAULTS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NDIS_802_11_RELOAD_DEFAULTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_802_11_RELOAD_DEFAULTS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct NDIS_802_11_STATUS_TYPE(pub i32);
impl ::windows_core::TypeKind for NDIS_802_11_STATUS_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NDIS_802_11_STATUS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_802_11_STATUS_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct NDIS_802_11_WEP_STATUS(pub i32);
impl ::windows_core::TypeKind for NDIS_802_11_WEP_STATUS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NDIS_802_11_WEP_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_802_11_WEP_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct NDIS_802_5_RING_STATE(pub i32);
impl ::windows_core::TypeKind for NDIS_802_5_RING_STATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NDIS_802_5_RING_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_802_5_RING_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct NDIS_CLASS_ID(pub i32);
impl ::windows_core::TypeKind for NDIS_CLASS_ID {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NDIS_CLASS_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_CLASS_ID").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct NDIS_DEVICE_PNP_EVENT(pub i32);
impl ::windows_core::TypeKind for NDIS_DEVICE_PNP_EVENT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NDIS_DEVICE_PNP_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_DEVICE_PNP_EVENT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct NDIS_DEVICE_POWER_STATE(pub i32);
impl ::windows_core::TypeKind for NDIS_DEVICE_POWER_STATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NDIS_DEVICE_POWER_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_DEVICE_POWER_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct NDIS_ENVIRONMENT_TYPE(pub i32);
impl ::windows_core::TypeKind for NDIS_ENVIRONMENT_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NDIS_ENVIRONMENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_ENVIRONMENT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct NDIS_FDDI_ATTACHMENT_TYPE(pub i32);
impl ::windows_core::TypeKind for NDIS_FDDI_ATTACHMENT_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NDIS_FDDI_ATTACHMENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_FDDI_ATTACHMENT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct NDIS_FDDI_LCONNECTION_STATE(pub i32);
impl ::windows_core::TypeKind for NDIS_FDDI_LCONNECTION_STATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NDIS_FDDI_LCONNECTION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_FDDI_LCONNECTION_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct NDIS_FDDI_RING_MGT_STATE(pub i32);
impl ::windows_core::TypeKind for NDIS_FDDI_RING_MGT_STATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NDIS_FDDI_RING_MGT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_FDDI_RING_MGT_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct NDIS_HARDWARE_STATUS(pub i32);
impl ::windows_core::TypeKind for NDIS_HARDWARE_STATUS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NDIS_HARDWARE_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_HARDWARE_STATUS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct NDIS_INTERFACE_TYPE(pub i32);
impl ::windows_core::TypeKind for NDIS_INTERFACE_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NDIS_INTERFACE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_INTERFACE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct NDIS_INTERRUPT_MODERATION(pub i32);
impl ::windows_core::TypeKind for NDIS_INTERRUPT_MODERATION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NDIS_INTERRUPT_MODERATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_INTERRUPT_MODERATION").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct NDIS_MEDIA_STATE(pub i32);
impl ::windows_core::TypeKind for NDIS_MEDIA_STATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NDIS_MEDIA_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_MEDIA_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct NDIS_MEDIUM(pub i32);
impl ::windows_core::TypeKind for NDIS_MEDIUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NDIS_MEDIUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_MEDIUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct NDIS_NETWORK_CHANGE_TYPE(pub i32);
impl ::windows_core::TypeKind for NDIS_NETWORK_CHANGE_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NDIS_NETWORK_CHANGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_NETWORK_CHANGE_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct NDIS_PARAMETER_TYPE(pub i32);
impl ::windows_core::TypeKind for NDIS_PARAMETER_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NDIS_PARAMETER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_PARAMETER_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct NDIS_PER_PACKET_INFO(pub i32);
impl ::windows_core::TypeKind for NDIS_PER_PACKET_INFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NDIS_PER_PACKET_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_PER_PACKET_INFO").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct NDIS_PHYSICAL_MEDIUM(pub i32);
impl ::windows_core::TypeKind for NDIS_PHYSICAL_MEDIUM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NDIS_PHYSICAL_MEDIUM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_PHYSICAL_MEDIUM").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct NDIS_PORT_AUTHORIZATION_STATE(pub i32);
impl ::windows_core::TypeKind for NDIS_PORT_AUTHORIZATION_STATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NDIS_PORT_AUTHORIZATION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_PORT_AUTHORIZATION_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct NDIS_PORT_CONTROL_STATE(pub i32);
impl ::windows_core::TypeKind for NDIS_PORT_CONTROL_STATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NDIS_PORT_CONTROL_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_PORT_CONTROL_STATE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct NDIS_PORT_TYPE(pub i32);
impl ::windows_core::TypeKind for NDIS_PORT_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NDIS_PORT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_PORT_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct NDIS_POWER_PROFILE(pub i32);
impl ::windows_core::TypeKind for NDIS_POWER_PROFILE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NDIS_POWER_PROFILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_POWER_PROFILE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct NDIS_PROCESSOR_TYPE(pub i32);
impl ::windows_core::TypeKind for NDIS_PROCESSOR_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NDIS_PROCESSOR_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_PROCESSOR_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct NDIS_PROCESSOR_VENDOR(pub i32);
impl ::windows_core::TypeKind for NDIS_PROCESSOR_VENDOR {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NDIS_PROCESSOR_VENDOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_PROCESSOR_VENDOR").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct NDIS_REQUEST_TYPE(pub i32);
impl ::windows_core::TypeKind for NDIS_REQUEST_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NDIS_REQUEST_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_REQUEST_TYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct NDIS_SUPPORTED_PAUSE_FUNCTIONS(pub i32);
impl ::windows_core::TypeKind for NDIS_SUPPORTED_PAUSE_FUNCTIONS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NDIS_SUPPORTED_PAUSE_FUNCTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_SUPPORTED_PAUSE_FUNCTIONS").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct NDIS_WAN_HEADER_FORMAT(pub i32);
impl ::windows_core::TypeKind for NDIS_WAN_HEADER_FORMAT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NDIS_WAN_HEADER_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_WAN_HEADER_FORMAT").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct NDIS_WAN_MEDIUM_SUBTYPE(pub i32);
impl ::windows_core::TypeKind for NDIS_WAN_MEDIUM_SUBTYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NDIS_WAN_MEDIUM_SUBTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_WAN_MEDIUM_SUBTYPE").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct NDIS_WAN_QUALITY(pub i32);
impl ::windows_core::TypeKind for NDIS_WAN_QUALITY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for NDIS_WAN_QUALITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_WAN_QUALITY").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct OFFLOAD_CONF_ALGO(pub i32);
impl ::windows_core::TypeKind for OFFLOAD_CONF_ALGO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for OFFLOAD_CONF_ALGO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OFFLOAD_CONF_ALGO").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct OFFLOAD_INTEGRITY_ALGO(pub i32);
impl ::windows_core::TypeKind for OFFLOAD_INTEGRITY_ALGO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for OFFLOAD_INTEGRITY_ALGO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OFFLOAD_INTEGRITY_ALGO").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct OFFLOAD_OPERATION_E(pub i32);
impl ::windows_core::TypeKind for OFFLOAD_OPERATION_E {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for OFFLOAD_OPERATION_E {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("OFFLOAD_OPERATION_E").field(&self.0).finish()
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq, ::core::marker::Copy, ::core::clone::Clone, ::core::default::Default)]
pub struct UDP_ENCAP_TYPE(pub i32);
impl ::windows_core::TypeKind for UDP_ENCAP_TYPE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::fmt::Debug for UDP_ENCAP_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("UDP_ENCAP_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
pub struct BINARY_DATA {
    pub Length: u16,
    pub Buffer: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for BINARY_DATA {}
impl ::core::clone::Clone for BINARY_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BINARY_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BINARY_DATA").field("Length", &self.Length).field("Buffer", &self.Buffer).finish()
    }
}
impl ::windows_core::TypeKind for BINARY_DATA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for BINARY_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.Buffer == other.Buffer
    }
}
impl ::core::cmp::Eq for BINARY_DATA {}
impl ::core::default::Default for BINARY_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct BSSID_INFO {
    pub BSSID: [u8; 6],
    pub PMKID: [u8; 16],
}
impl ::core::marker::Copy for BSSID_INFO {}
impl ::core::clone::Clone for BSSID_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BSSID_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BSSID_INFO").field("BSSID", &self.BSSID).field("PMKID", &self.PMKID).finish()
    }
}
impl ::windows_core::TypeKind for BSSID_INFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for BSSID_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.BSSID == other.BSSID && self.PMKID == other.PMKID
    }
}
impl ::core::cmp::Eq for BSSID_INFO {}
impl ::core::default::Default for BSSID_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct CO_ADDRESS {
    pub AddressSize: u32,
    pub Address: [u8; 1],
}
impl ::core::marker::Copy for CO_ADDRESS {}
impl ::core::clone::Clone for CO_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CO_ADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CO_ADDRESS").field("AddressSize", &self.AddressSize).field("Address", &self.Address).finish()
    }
}
impl ::windows_core::TypeKind for CO_ADDRESS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for CO_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.AddressSize == other.AddressSize && self.Address == other.Address
    }
}
impl ::core::cmp::Eq for CO_ADDRESS {}
impl ::core::default::Default for CO_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct CO_ADDRESS_FAMILY {
    pub AddressFamily: u32,
    pub MajorVersion: u32,
    pub MinorVersion: u32,
}
impl ::core::marker::Copy for CO_ADDRESS_FAMILY {}
impl ::core::clone::Clone for CO_ADDRESS_FAMILY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CO_ADDRESS_FAMILY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CO_ADDRESS_FAMILY").field("AddressFamily", &self.AddressFamily).field("MajorVersion", &self.MajorVersion).field("MinorVersion", &self.MinorVersion).finish()
    }
}
impl ::windows_core::TypeKind for CO_ADDRESS_FAMILY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for CO_ADDRESS_FAMILY {
    fn eq(&self, other: &Self) -> bool {
        self.AddressFamily == other.AddressFamily && self.MajorVersion == other.MajorVersion && self.MinorVersion == other.MinorVersion
    }
}
impl ::core::cmp::Eq for CO_ADDRESS_FAMILY {}
impl ::core::default::Default for CO_ADDRESS_FAMILY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct CO_ADDRESS_LIST {
    pub NumberOfAddressesAvailable: u32,
    pub NumberOfAddresses: u32,
    pub AddressList: CO_ADDRESS,
}
impl ::core::marker::Copy for CO_ADDRESS_LIST {}
impl ::core::clone::Clone for CO_ADDRESS_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CO_ADDRESS_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CO_ADDRESS_LIST").field("NumberOfAddressesAvailable", &self.NumberOfAddressesAvailable).field("NumberOfAddresses", &self.NumberOfAddresses).field("AddressList", &self.AddressList).finish()
    }
}
impl ::windows_core::TypeKind for CO_ADDRESS_LIST {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for CO_ADDRESS_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfAddressesAvailable == other.NumberOfAddressesAvailable && self.NumberOfAddresses == other.NumberOfAddresses && self.AddressList == other.AddressList
    }
}
impl ::core::cmp::Eq for CO_ADDRESS_LIST {}
impl ::core::default::Default for CO_ADDRESS_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Networking_WinSock")]
pub struct CO_CALL_MANAGER_PARAMETERS {
    pub Transmit: super::super::super::Win32::Networking::WinSock::FLOWSPEC,
    pub Receive: super::super::super::Win32::Networking::WinSock::FLOWSPEC,
    pub CallMgrSpecific: CO_SPECIFIC_PARAMETERS,
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::marker::Copy for CO_CALL_MANAGER_PARAMETERS {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::clone::Clone for CO_CALL_MANAGER_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::fmt::Debug for CO_CALL_MANAGER_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CO_CALL_MANAGER_PARAMETERS").field("Transmit", &self.Transmit).field("Receive", &self.Receive).field("CallMgrSpecific", &self.CallMgrSpecific).finish()
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::windows_core::TypeKind for CO_CALL_MANAGER_PARAMETERS {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::PartialEq for CO_CALL_MANAGER_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Transmit == other.Transmit && self.Receive == other.Receive && self.CallMgrSpecific == other.CallMgrSpecific
    }
}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::cmp::Eq for CO_CALL_MANAGER_PARAMETERS {}
#[cfg(feature = "Win32_Networking_WinSock")]
impl ::core::default::Default for CO_CALL_MANAGER_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CO_CALL_PARAMETERS(pub isize);
impl ::core::default::Default for CO_CALL_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for CO_CALL_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for CO_CALL_PARAMETERS {}
impl ::core::fmt::Debug for CO_CALL_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CO_CALL_PARAMETERS").field(&self.0).finish()
    }
}
impl ::windows_core::TypeKind for CO_CALL_PARAMETERS {
    type TypeKind = ::windows_core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CO_MEDIA_PARAMETERS(pub isize);
impl ::core::default::Default for CO_MEDIA_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for CO_MEDIA_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for CO_MEDIA_PARAMETERS {}
impl ::core::fmt::Debug for CO_MEDIA_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CO_MEDIA_PARAMETERS").field(&self.0).finish()
    }
}
impl ::windows_core::TypeKind for CO_MEDIA_PARAMETERS {
    type TypeKind = ::windows_core::CopyType;
}
#[repr(C)]
pub struct CO_PVC {
    pub NdisAfHandle: *mut ::core::ffi::c_void,
    pub PvcParameters: CO_SPECIFIC_PARAMETERS,
}
impl ::core::marker::Copy for CO_PVC {}
impl ::core::clone::Clone for CO_PVC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CO_PVC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CO_PVC").field("NdisAfHandle", &self.NdisAfHandle).field("PvcParameters", &self.PvcParameters).finish()
    }
}
impl ::windows_core::TypeKind for CO_PVC {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for CO_PVC {
    fn eq(&self, other: &Self) -> bool {
        self.NdisAfHandle == other.NdisAfHandle && self.PvcParameters == other.PvcParameters
    }
}
impl ::core::cmp::Eq for CO_PVC {}
impl ::core::default::Default for CO_PVC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct CO_SAP {
    pub SapType: u32,
    pub SapLength: u32,
    pub Sap: [u8; 1],
}
impl ::core::marker::Copy for CO_SAP {}
impl ::core::clone::Clone for CO_SAP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CO_SAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CO_SAP").field("SapType", &self.SapType).field("SapLength", &self.SapLength).field("Sap", &self.Sap).finish()
    }
}
impl ::windows_core::TypeKind for CO_SAP {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for CO_SAP {
    fn eq(&self, other: &Self) -> bool {
        self.SapType == other.SapType && self.SapLength == other.SapLength && self.Sap == other.Sap
    }
}
impl ::core::cmp::Eq for CO_SAP {}
impl ::core::default::Default for CO_SAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct CO_SPECIFIC_PARAMETERS {
    pub ParamType: u32,
    pub Length: u32,
    pub Parameters: [u8; 1],
}
impl ::core::marker::Copy for CO_SPECIFIC_PARAMETERS {}
impl ::core::clone::Clone for CO_SPECIFIC_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for CO_SPECIFIC_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("CO_SPECIFIC_PARAMETERS").field("ParamType", &self.ParamType).field("Length", &self.Length).field("Parameters", &self.Parameters).finish()
    }
}
impl ::windows_core::TypeKind for CO_SPECIFIC_PARAMETERS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for CO_SPECIFIC_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.ParamType == other.ParamType && self.Length == other.Length && self.Parameters == other.Parameters
    }
}
impl ::core::cmp::Eq for CO_SPECIFIC_PARAMETERS {}
impl ::core::default::Default for CO_SPECIFIC_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ETH_FILTER(pub isize);
impl ::core::default::Default for ETH_FILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for ETH_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for ETH_FILTER {}
impl ::core::fmt::Debug for ETH_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ETH_FILTER").field(&self.0).finish()
    }
}
impl ::windows_core::TypeKind for ETH_FILTER {
    type TypeKind = ::windows_core::CopyType;
}
#[repr(C)]
pub struct FILTERDBS {
    pub Anonymous: FILTERDBS_0,
    pub TrDB: *mut isize,
    pub YYYDB: *mut ::core::ffi::c_void,
    pub XXXDB: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for FILTERDBS {}
impl ::core::clone::Clone for FILTERDBS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for FILTERDBS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for FILTERDBS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union FILTERDBS_0 {
    pub EthDB: *mut ETH_FILTER,
    pub NullDB: *mut isize,
}
impl ::core::marker::Copy for FILTERDBS_0 {}
impl ::core::clone::Clone for FILTERDBS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for FILTERDBS_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for FILTERDBS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct GEN_GET_NETCARD_TIME {
    pub ReadTime: u64,
}
impl ::core::marker::Copy for GEN_GET_NETCARD_TIME {}
impl ::core::clone::Clone for GEN_GET_NETCARD_TIME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GEN_GET_NETCARD_TIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GEN_GET_NETCARD_TIME").field("ReadTime", &self.ReadTime).finish()
    }
}
impl ::windows_core::TypeKind for GEN_GET_NETCARD_TIME {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for GEN_GET_NETCARD_TIME {
    fn eq(&self, other: &Self) -> bool {
        self.ReadTime == other.ReadTime
    }
}
impl ::core::cmp::Eq for GEN_GET_NETCARD_TIME {}
impl ::core::default::Default for GEN_GET_NETCARD_TIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct GEN_GET_TIME_CAPS {
    pub Flags: u32,
    pub ClockPrecision: u32,
}
impl ::core::marker::Copy for GEN_GET_TIME_CAPS {}
impl ::core::clone::Clone for GEN_GET_TIME_CAPS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GEN_GET_TIME_CAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GEN_GET_TIME_CAPS").field("Flags", &self.Flags).field("ClockPrecision", &self.ClockPrecision).finish()
    }
}
impl ::windows_core::TypeKind for GEN_GET_TIME_CAPS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for GEN_GET_TIME_CAPS {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.ClockPrecision == other.ClockPrecision
    }
}
impl ::core::cmp::Eq for GEN_GET_TIME_CAPS {}
impl ::core::default::Default for GEN_GET_TIME_CAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct LOCK_STATE {
    pub LockState: u16,
    pub OldIrql: u8,
}
impl ::core::marker::Copy for LOCK_STATE {}
impl ::core::clone::Clone for LOCK_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for LOCK_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("LOCK_STATE").field("LockState", &self.LockState).field("OldIrql", &self.OldIrql).finish()
    }
}
impl ::windows_core::TypeKind for LOCK_STATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for LOCK_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.LockState == other.LockState && self.OldIrql == other.OldIrql
    }
}
impl ::core::cmp::Eq for LOCK_STATE {}
impl ::core::default::Default for LOCK_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct MEDIA_SPECIFIC_INFORMATION {
    pub NextEntryOffset: u32,
    pub ClassId: NDIS_CLASS_ID,
    pub Size: u32,
    pub ClassInformation: [u8; 1],
}
impl ::core::marker::Copy for MEDIA_SPECIFIC_INFORMATION {}
impl ::core::clone::Clone for MEDIA_SPECIFIC_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MEDIA_SPECIFIC_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MEDIA_SPECIFIC_INFORMATION").field("NextEntryOffset", &self.NextEntryOffset).field("ClassId", &self.ClassId).field("Size", &self.Size).field("ClassInformation", &self.ClassInformation).finish()
    }
}
impl ::windows_core::TypeKind for MEDIA_SPECIFIC_INFORMATION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for MEDIA_SPECIFIC_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.NextEntryOffset == other.NextEntryOffset && self.ClassId == other.ClassId && self.Size == other.Size && self.ClassInformation == other.ClassInformation
    }
}
impl ::core::cmp::Eq for MEDIA_SPECIFIC_INFORMATION {}
impl ::core::default::Default for MEDIA_SPECIFIC_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_802_11_AI_REQFI {
    pub Capabilities: u16,
    pub ListenInterval: u16,
    pub CurrentAPAddress: [u8; 6],
}
impl ::core::marker::Copy for NDIS_802_11_AI_REQFI {}
impl ::core::clone::Clone for NDIS_802_11_AI_REQFI {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_802_11_AI_REQFI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_802_11_AI_REQFI").field("Capabilities", &self.Capabilities).field("ListenInterval", &self.ListenInterval).field("CurrentAPAddress", &self.CurrentAPAddress).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_802_11_AI_REQFI {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_802_11_AI_REQFI {
    fn eq(&self, other: &Self) -> bool {
        self.Capabilities == other.Capabilities && self.ListenInterval == other.ListenInterval && self.CurrentAPAddress == other.CurrentAPAddress
    }
}
impl ::core::cmp::Eq for NDIS_802_11_AI_REQFI {}
impl ::core::default::Default for NDIS_802_11_AI_REQFI {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_802_11_AI_RESFI {
    pub Capabilities: u16,
    pub StatusCode: u16,
    pub AssociationId: u16,
}
impl ::core::marker::Copy for NDIS_802_11_AI_RESFI {}
impl ::core::clone::Clone for NDIS_802_11_AI_RESFI {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_802_11_AI_RESFI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_802_11_AI_RESFI").field("Capabilities", &self.Capabilities).field("StatusCode", &self.StatusCode).field("AssociationId", &self.AssociationId).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_802_11_AI_RESFI {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_802_11_AI_RESFI {
    fn eq(&self, other: &Self) -> bool {
        self.Capabilities == other.Capabilities && self.StatusCode == other.StatusCode && self.AssociationId == other.AssociationId
    }
}
impl ::core::cmp::Eq for NDIS_802_11_AI_RESFI {}
impl ::core::default::Default for NDIS_802_11_AI_RESFI {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_802_11_ASSOCIATION_INFORMATION {
    pub Length: u32,
    pub AvailableRequestFixedIEs: u16,
    pub RequestFixedIEs: NDIS_802_11_AI_REQFI,
    pub RequestIELength: u32,
    pub OffsetRequestIEs: u32,
    pub AvailableResponseFixedIEs: u16,
    pub ResponseFixedIEs: NDIS_802_11_AI_RESFI,
    pub ResponseIELength: u32,
    pub OffsetResponseIEs: u32,
}
impl ::core::marker::Copy for NDIS_802_11_ASSOCIATION_INFORMATION {}
impl ::core::clone::Clone for NDIS_802_11_ASSOCIATION_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_802_11_ASSOCIATION_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_802_11_ASSOCIATION_INFORMATION")
            .field("Length", &self.Length)
            .field("AvailableRequestFixedIEs", &self.AvailableRequestFixedIEs)
            .field("RequestFixedIEs", &self.RequestFixedIEs)
            .field("RequestIELength", &self.RequestIELength)
            .field("OffsetRequestIEs", &self.OffsetRequestIEs)
            .field("AvailableResponseFixedIEs", &self.AvailableResponseFixedIEs)
            .field("ResponseFixedIEs", &self.ResponseFixedIEs)
            .field("ResponseIELength", &self.ResponseIELength)
            .field("OffsetResponseIEs", &self.OffsetResponseIEs)
            .finish()
    }
}
impl ::windows_core::TypeKind for NDIS_802_11_ASSOCIATION_INFORMATION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_802_11_ASSOCIATION_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.AvailableRequestFixedIEs == other.AvailableRequestFixedIEs && self.RequestFixedIEs == other.RequestFixedIEs && self.RequestIELength == other.RequestIELength && self.OffsetRequestIEs == other.OffsetRequestIEs && self.AvailableResponseFixedIEs == other.AvailableResponseFixedIEs && self.ResponseFixedIEs == other.ResponseFixedIEs && self.ResponseIELength == other.ResponseIELength && self.OffsetResponseIEs == other.OffsetResponseIEs
    }
}
impl ::core::cmp::Eq for NDIS_802_11_ASSOCIATION_INFORMATION {}
impl ::core::default::Default for NDIS_802_11_ASSOCIATION_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_802_11_AUTHENTICATION_ENCRYPTION {
    pub AuthModeSupported: NDIS_802_11_AUTHENTICATION_MODE,
    pub EncryptStatusSupported: NDIS_802_11_WEP_STATUS,
}
impl ::core::marker::Copy for NDIS_802_11_AUTHENTICATION_ENCRYPTION {}
impl ::core::clone::Clone for NDIS_802_11_AUTHENTICATION_ENCRYPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_802_11_AUTHENTICATION_ENCRYPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_802_11_AUTHENTICATION_ENCRYPTION").field("AuthModeSupported", &self.AuthModeSupported).field("EncryptStatusSupported", &self.EncryptStatusSupported).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_802_11_AUTHENTICATION_ENCRYPTION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_802_11_AUTHENTICATION_ENCRYPTION {
    fn eq(&self, other: &Self) -> bool {
        self.AuthModeSupported == other.AuthModeSupported && self.EncryptStatusSupported == other.EncryptStatusSupported
    }
}
impl ::core::cmp::Eq for NDIS_802_11_AUTHENTICATION_ENCRYPTION {}
impl ::core::default::Default for NDIS_802_11_AUTHENTICATION_ENCRYPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_802_11_AUTHENTICATION_EVENT {
    pub Status: NDIS_802_11_STATUS_INDICATION,
    pub Request: [NDIS_802_11_AUTHENTICATION_REQUEST; 1],
}
impl ::core::marker::Copy for NDIS_802_11_AUTHENTICATION_EVENT {}
impl ::core::clone::Clone for NDIS_802_11_AUTHENTICATION_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_802_11_AUTHENTICATION_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_802_11_AUTHENTICATION_EVENT").field("Status", &self.Status).field("Request", &self.Request).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_802_11_AUTHENTICATION_EVENT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_802_11_AUTHENTICATION_EVENT {
    fn eq(&self, other: &Self) -> bool {
        self.Status == other.Status && self.Request == other.Request
    }
}
impl ::core::cmp::Eq for NDIS_802_11_AUTHENTICATION_EVENT {}
impl ::core::default::Default for NDIS_802_11_AUTHENTICATION_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_802_11_AUTHENTICATION_REQUEST {
    pub Length: u32,
    pub Bssid: [u8; 6],
    pub Flags: u32,
}
impl ::core::marker::Copy for NDIS_802_11_AUTHENTICATION_REQUEST {}
impl ::core::clone::Clone for NDIS_802_11_AUTHENTICATION_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_802_11_AUTHENTICATION_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_802_11_AUTHENTICATION_REQUEST").field("Length", &self.Length).field("Bssid", &self.Bssid).field("Flags", &self.Flags).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_802_11_AUTHENTICATION_REQUEST {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_802_11_AUTHENTICATION_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.Bssid == other.Bssid && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for NDIS_802_11_AUTHENTICATION_REQUEST {}
impl ::core::default::Default for NDIS_802_11_AUTHENTICATION_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_802_11_BSSID_LIST {
    pub NumberOfItems: u32,
    pub Bssid: [NDIS_WLAN_BSSID; 1],
}
impl ::core::marker::Copy for NDIS_802_11_BSSID_LIST {}
impl ::core::clone::Clone for NDIS_802_11_BSSID_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_802_11_BSSID_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_802_11_BSSID_LIST").field("NumberOfItems", &self.NumberOfItems).field("Bssid", &self.Bssid).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_802_11_BSSID_LIST {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_802_11_BSSID_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfItems == other.NumberOfItems && self.Bssid == other.Bssid
    }
}
impl ::core::cmp::Eq for NDIS_802_11_BSSID_LIST {}
impl ::core::default::Default for NDIS_802_11_BSSID_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_802_11_BSSID_LIST_EX {
    pub NumberOfItems: u32,
    pub Bssid: [NDIS_WLAN_BSSID_EX; 1],
}
impl ::core::marker::Copy for NDIS_802_11_BSSID_LIST_EX {}
impl ::core::clone::Clone for NDIS_802_11_BSSID_LIST_EX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_802_11_BSSID_LIST_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_802_11_BSSID_LIST_EX").field("NumberOfItems", &self.NumberOfItems).field("Bssid", &self.Bssid).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_802_11_BSSID_LIST_EX {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_802_11_BSSID_LIST_EX {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfItems == other.NumberOfItems && self.Bssid == other.Bssid
    }
}
impl ::core::cmp::Eq for NDIS_802_11_BSSID_LIST_EX {}
impl ::core::default::Default for NDIS_802_11_BSSID_LIST_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_802_11_CAPABILITY {
    pub Length: u32,
    pub Version: u32,
    pub NoOfPMKIDs: u32,
    pub NoOfAuthEncryptPairsSupported: u32,
    pub AuthenticationEncryptionSupported: [NDIS_802_11_AUTHENTICATION_ENCRYPTION; 1],
}
impl ::core::marker::Copy for NDIS_802_11_CAPABILITY {}
impl ::core::clone::Clone for NDIS_802_11_CAPABILITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_802_11_CAPABILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_802_11_CAPABILITY").field("Length", &self.Length).field("Version", &self.Version).field("NoOfPMKIDs", &self.NoOfPMKIDs).field("NoOfAuthEncryptPairsSupported", &self.NoOfAuthEncryptPairsSupported).field("AuthenticationEncryptionSupported", &self.AuthenticationEncryptionSupported).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_802_11_CAPABILITY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_802_11_CAPABILITY {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.Version == other.Version && self.NoOfPMKIDs == other.NoOfPMKIDs && self.NoOfAuthEncryptPairsSupported == other.NoOfAuthEncryptPairsSupported && self.AuthenticationEncryptionSupported == other.AuthenticationEncryptionSupported
    }
}
impl ::core::cmp::Eq for NDIS_802_11_CAPABILITY {}
impl ::core::default::Default for NDIS_802_11_CAPABILITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_802_11_CONFIGURATION {
    pub Length: u32,
    pub BeaconPeriod: u32,
    pub ATIMWindow: u32,
    pub DSConfig: u32,
    pub FHConfig: NDIS_802_11_CONFIGURATION_FH,
}
impl ::core::marker::Copy for NDIS_802_11_CONFIGURATION {}
impl ::core::clone::Clone for NDIS_802_11_CONFIGURATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_802_11_CONFIGURATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_802_11_CONFIGURATION").field("Length", &self.Length).field("BeaconPeriod", &self.BeaconPeriod).field("ATIMWindow", &self.ATIMWindow).field("DSConfig", &self.DSConfig).field("FHConfig", &self.FHConfig).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_802_11_CONFIGURATION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_802_11_CONFIGURATION {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.BeaconPeriod == other.BeaconPeriod && self.ATIMWindow == other.ATIMWindow && self.DSConfig == other.DSConfig && self.FHConfig == other.FHConfig
    }
}
impl ::core::cmp::Eq for NDIS_802_11_CONFIGURATION {}
impl ::core::default::Default for NDIS_802_11_CONFIGURATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_802_11_CONFIGURATION_FH {
    pub Length: u32,
    pub HopPattern: u32,
    pub HopSet: u32,
    pub DwellTime: u32,
}
impl ::core::marker::Copy for NDIS_802_11_CONFIGURATION_FH {}
impl ::core::clone::Clone for NDIS_802_11_CONFIGURATION_FH {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_802_11_CONFIGURATION_FH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_802_11_CONFIGURATION_FH").field("Length", &self.Length).field("HopPattern", &self.HopPattern).field("HopSet", &self.HopSet).field("DwellTime", &self.DwellTime).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_802_11_CONFIGURATION_FH {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_802_11_CONFIGURATION_FH {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.HopPattern == other.HopPattern && self.HopSet == other.HopSet && self.DwellTime == other.DwellTime
    }
}
impl ::core::cmp::Eq for NDIS_802_11_CONFIGURATION_FH {}
impl ::core::default::Default for NDIS_802_11_CONFIGURATION_FH {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_802_11_FIXED_IEs {
    pub Timestamp: [u8; 8],
    pub BeaconInterval: u16,
    pub Capabilities: u16,
}
impl ::core::marker::Copy for NDIS_802_11_FIXED_IEs {}
impl ::core::clone::Clone for NDIS_802_11_FIXED_IEs {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_802_11_FIXED_IEs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_802_11_FIXED_IEs").field("Timestamp", &self.Timestamp).field("BeaconInterval", &self.BeaconInterval).field("Capabilities", &self.Capabilities).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_802_11_FIXED_IEs {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_802_11_FIXED_IEs {
    fn eq(&self, other: &Self) -> bool {
        self.Timestamp == other.Timestamp && self.BeaconInterval == other.BeaconInterval && self.Capabilities == other.Capabilities
    }
}
impl ::core::cmp::Eq for NDIS_802_11_FIXED_IEs {}
impl ::core::default::Default for NDIS_802_11_FIXED_IEs {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_802_11_KEY {
    pub Length: u32,
    pub KeyIndex: u32,
    pub KeyLength: u32,
    pub BSSID: [u8; 6],
    pub KeyRSC: u64,
    pub KeyMaterial: [u8; 1],
}
impl ::core::marker::Copy for NDIS_802_11_KEY {}
impl ::core::clone::Clone for NDIS_802_11_KEY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_802_11_KEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_802_11_KEY").field("Length", &self.Length).field("KeyIndex", &self.KeyIndex).field("KeyLength", &self.KeyLength).field("BSSID", &self.BSSID).field("KeyRSC", &self.KeyRSC).field("KeyMaterial", &self.KeyMaterial).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_802_11_KEY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_802_11_KEY {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.KeyIndex == other.KeyIndex && self.KeyLength == other.KeyLength && self.BSSID == other.BSSID && self.KeyRSC == other.KeyRSC && self.KeyMaterial == other.KeyMaterial
    }
}
impl ::core::cmp::Eq for NDIS_802_11_KEY {}
impl ::core::default::Default for NDIS_802_11_KEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_802_11_NETWORK_TYPE_LIST {
    pub NumberOfItems: u32,
    pub NetworkType: [NDIS_802_11_NETWORK_TYPE; 1],
}
impl ::core::marker::Copy for NDIS_802_11_NETWORK_TYPE_LIST {}
impl ::core::clone::Clone for NDIS_802_11_NETWORK_TYPE_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_802_11_NETWORK_TYPE_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_802_11_NETWORK_TYPE_LIST").field("NumberOfItems", &self.NumberOfItems).field("NetworkType", &self.NetworkType).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_802_11_NETWORK_TYPE_LIST {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_802_11_NETWORK_TYPE_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfItems == other.NumberOfItems && self.NetworkType == other.NetworkType
    }
}
impl ::core::cmp::Eq for NDIS_802_11_NETWORK_TYPE_LIST {}
impl ::core::default::Default for NDIS_802_11_NETWORK_TYPE_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_802_11_NON_BCAST_SSID_LIST {
    pub NumberOfItems: u32,
    pub Non_Bcast_Ssid: [NDIS_802_11_SSID; 1],
}
impl ::core::marker::Copy for NDIS_802_11_NON_BCAST_SSID_LIST {}
impl ::core::clone::Clone for NDIS_802_11_NON_BCAST_SSID_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_802_11_NON_BCAST_SSID_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_802_11_NON_BCAST_SSID_LIST").field("NumberOfItems", &self.NumberOfItems).field("Non_Bcast_Ssid", &self.Non_Bcast_Ssid).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_802_11_NON_BCAST_SSID_LIST {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_802_11_NON_BCAST_SSID_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfItems == other.NumberOfItems && self.Non_Bcast_Ssid == other.Non_Bcast_Ssid
    }
}
impl ::core::cmp::Eq for NDIS_802_11_NON_BCAST_SSID_LIST {}
impl ::core::default::Default for NDIS_802_11_NON_BCAST_SSID_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_802_11_PMKID {
    pub Length: u32,
    pub BSSIDInfoCount: u32,
    pub BSSIDInfo: [BSSID_INFO; 1],
}
impl ::core::marker::Copy for NDIS_802_11_PMKID {}
impl ::core::clone::Clone for NDIS_802_11_PMKID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_802_11_PMKID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_802_11_PMKID").field("Length", &self.Length).field("BSSIDInfoCount", &self.BSSIDInfoCount).field("BSSIDInfo", &self.BSSIDInfo).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_802_11_PMKID {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_802_11_PMKID {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.BSSIDInfoCount == other.BSSIDInfoCount && self.BSSIDInfo == other.BSSIDInfo
    }
}
impl ::core::cmp::Eq for NDIS_802_11_PMKID {}
impl ::core::default::Default for NDIS_802_11_PMKID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_802_11_PMKID_CANDIDATE_LIST {
    pub Version: u32,
    pub NumCandidates: u32,
    pub CandidateList: [PMKID_CANDIDATE; 1],
}
impl ::core::marker::Copy for NDIS_802_11_PMKID_CANDIDATE_LIST {}
impl ::core::clone::Clone for NDIS_802_11_PMKID_CANDIDATE_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_802_11_PMKID_CANDIDATE_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_802_11_PMKID_CANDIDATE_LIST").field("Version", &self.Version).field("NumCandidates", &self.NumCandidates).field("CandidateList", &self.CandidateList).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_802_11_PMKID_CANDIDATE_LIST {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_802_11_PMKID_CANDIDATE_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.NumCandidates == other.NumCandidates && self.CandidateList == other.CandidateList
    }
}
impl ::core::cmp::Eq for NDIS_802_11_PMKID_CANDIDATE_LIST {}
impl ::core::default::Default for NDIS_802_11_PMKID_CANDIDATE_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_802_11_REMOVE_KEY {
    pub Length: u32,
    pub KeyIndex: u32,
    pub BSSID: [u8; 6],
}
impl ::core::marker::Copy for NDIS_802_11_REMOVE_KEY {}
impl ::core::clone::Clone for NDIS_802_11_REMOVE_KEY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_802_11_REMOVE_KEY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_802_11_REMOVE_KEY").field("Length", &self.Length).field("KeyIndex", &self.KeyIndex).field("BSSID", &self.BSSID).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_802_11_REMOVE_KEY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_802_11_REMOVE_KEY {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.KeyIndex == other.KeyIndex && self.BSSID == other.BSSID
    }
}
impl ::core::cmp::Eq for NDIS_802_11_REMOVE_KEY {}
impl ::core::default::Default for NDIS_802_11_REMOVE_KEY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_802_11_SSID {
    pub SsidLength: u32,
    pub Ssid: [u8; 32],
}
impl ::core::marker::Copy for NDIS_802_11_SSID {}
impl ::core::clone::Clone for NDIS_802_11_SSID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_802_11_SSID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_802_11_SSID").field("SsidLength", &self.SsidLength).field("Ssid", &self.Ssid).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_802_11_SSID {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_802_11_SSID {
    fn eq(&self, other: &Self) -> bool {
        self.SsidLength == other.SsidLength && self.Ssid == other.Ssid
    }
}
impl ::core::cmp::Eq for NDIS_802_11_SSID {}
impl ::core::default::Default for NDIS_802_11_SSID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_802_11_STATISTICS {
    pub Length: u32,
    pub TransmittedFragmentCount: i64,
    pub MulticastTransmittedFrameCount: i64,
    pub FailedCount: i64,
    pub RetryCount: i64,
    pub MultipleRetryCount: i64,
    pub RTSSuccessCount: i64,
    pub RTSFailureCount: i64,
    pub ACKFailureCount: i64,
    pub FrameDuplicateCount: i64,
    pub ReceivedFragmentCount: i64,
    pub MulticastReceivedFrameCount: i64,
    pub FCSErrorCount: i64,
    pub TKIPLocalMICFailures: i64,
    pub TKIPICVErrorCount: i64,
    pub TKIPCounterMeasuresInvoked: i64,
    pub TKIPReplays: i64,
    pub CCMPFormatErrors: i64,
    pub CCMPReplays: i64,
    pub CCMPDecryptErrors: i64,
    pub FourWayHandshakeFailures: i64,
    pub WEPUndecryptableCount: i64,
    pub WEPICVErrorCount: i64,
    pub DecryptSuccessCount: i64,
    pub DecryptFailureCount: i64,
}
impl ::core::marker::Copy for NDIS_802_11_STATISTICS {}
impl ::core::clone::Clone for NDIS_802_11_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_802_11_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_802_11_STATISTICS")
            .field("Length", &self.Length)
            .field("TransmittedFragmentCount", &self.TransmittedFragmentCount)
            .field("MulticastTransmittedFrameCount", &self.MulticastTransmittedFrameCount)
            .field("FailedCount", &self.FailedCount)
            .field("RetryCount", &self.RetryCount)
            .field("MultipleRetryCount", &self.MultipleRetryCount)
            .field("RTSSuccessCount", &self.RTSSuccessCount)
            .field("RTSFailureCount", &self.RTSFailureCount)
            .field("ACKFailureCount", &self.ACKFailureCount)
            .field("FrameDuplicateCount", &self.FrameDuplicateCount)
            .field("ReceivedFragmentCount", &self.ReceivedFragmentCount)
            .field("MulticastReceivedFrameCount", &self.MulticastReceivedFrameCount)
            .field("FCSErrorCount", &self.FCSErrorCount)
            .field("TKIPLocalMICFailures", &self.TKIPLocalMICFailures)
            .field("TKIPICVErrorCount", &self.TKIPICVErrorCount)
            .field("TKIPCounterMeasuresInvoked", &self.TKIPCounterMeasuresInvoked)
            .field("TKIPReplays", &self.TKIPReplays)
            .field("CCMPFormatErrors", &self.CCMPFormatErrors)
            .field("CCMPReplays", &self.CCMPReplays)
            .field("CCMPDecryptErrors", &self.CCMPDecryptErrors)
            .field("FourWayHandshakeFailures", &self.FourWayHandshakeFailures)
            .field("WEPUndecryptableCount", &self.WEPUndecryptableCount)
            .field("WEPICVErrorCount", &self.WEPICVErrorCount)
            .field("DecryptSuccessCount", &self.DecryptSuccessCount)
            .field("DecryptFailureCount", &self.DecryptFailureCount)
            .finish()
    }
}
impl ::windows_core::TypeKind for NDIS_802_11_STATISTICS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_802_11_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length
            && self.TransmittedFragmentCount == other.TransmittedFragmentCount
            && self.MulticastTransmittedFrameCount == other.MulticastTransmittedFrameCount
            && self.FailedCount == other.FailedCount
            && self.RetryCount == other.RetryCount
            && self.MultipleRetryCount == other.MultipleRetryCount
            && self.RTSSuccessCount == other.RTSSuccessCount
            && self.RTSFailureCount == other.RTSFailureCount
            && self.ACKFailureCount == other.ACKFailureCount
            && self.FrameDuplicateCount == other.FrameDuplicateCount
            && self.ReceivedFragmentCount == other.ReceivedFragmentCount
            && self.MulticastReceivedFrameCount == other.MulticastReceivedFrameCount
            && self.FCSErrorCount == other.FCSErrorCount
            && self.TKIPLocalMICFailures == other.TKIPLocalMICFailures
            && self.TKIPICVErrorCount == other.TKIPICVErrorCount
            && self.TKIPCounterMeasuresInvoked == other.TKIPCounterMeasuresInvoked
            && self.TKIPReplays == other.TKIPReplays
            && self.CCMPFormatErrors == other.CCMPFormatErrors
            && self.CCMPReplays == other.CCMPReplays
            && self.CCMPDecryptErrors == other.CCMPDecryptErrors
            && self.FourWayHandshakeFailures == other.FourWayHandshakeFailures
            && self.WEPUndecryptableCount == other.WEPUndecryptableCount
            && self.WEPICVErrorCount == other.WEPICVErrorCount
            && self.DecryptSuccessCount == other.DecryptSuccessCount
            && self.DecryptFailureCount == other.DecryptFailureCount
    }
}
impl ::core::cmp::Eq for NDIS_802_11_STATISTICS {}
impl ::core::default::Default for NDIS_802_11_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_802_11_STATUS_INDICATION {
    pub StatusType: NDIS_802_11_STATUS_TYPE,
}
impl ::core::marker::Copy for NDIS_802_11_STATUS_INDICATION {}
impl ::core::clone::Clone for NDIS_802_11_STATUS_INDICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_802_11_STATUS_INDICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_802_11_STATUS_INDICATION").field("StatusType", &self.StatusType).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_802_11_STATUS_INDICATION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_802_11_STATUS_INDICATION {
    fn eq(&self, other: &Self) -> bool {
        self.StatusType == other.StatusType
    }
}
impl ::core::cmp::Eq for NDIS_802_11_STATUS_INDICATION {}
impl ::core::default::Default for NDIS_802_11_STATUS_INDICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_802_11_TEST {
    pub Length: u32,
    pub Type: u32,
    pub Anonymous: NDIS_802_11_TEST_0,
}
impl ::core::marker::Copy for NDIS_802_11_TEST {}
impl ::core::clone::Clone for NDIS_802_11_TEST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for NDIS_802_11_TEST {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for NDIS_802_11_TEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union NDIS_802_11_TEST_0 {
    pub AuthenticationEvent: NDIS_802_11_AUTHENTICATION_EVENT,
    pub RssiTrigger: i32,
}
impl ::core::marker::Copy for NDIS_802_11_TEST_0 {}
impl ::core::clone::Clone for NDIS_802_11_TEST_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for NDIS_802_11_TEST_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for NDIS_802_11_TEST_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_802_11_VARIABLE_IEs {
    pub ElementID: u8,
    pub Length: u8,
    pub data: [u8; 1],
}
impl ::core::marker::Copy for NDIS_802_11_VARIABLE_IEs {}
impl ::core::clone::Clone for NDIS_802_11_VARIABLE_IEs {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_802_11_VARIABLE_IEs {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_802_11_VARIABLE_IEs").field("ElementID", &self.ElementID).field("Length", &self.Length).field("data", &self.data).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_802_11_VARIABLE_IEs {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_802_11_VARIABLE_IEs {
    fn eq(&self, other: &Self) -> bool {
        self.ElementID == other.ElementID && self.Length == other.Length && self.data == other.data
    }
}
impl ::core::cmp::Eq for NDIS_802_11_VARIABLE_IEs {}
impl ::core::default::Default for NDIS_802_11_VARIABLE_IEs {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_802_11_WEP {
    pub Length: u32,
    pub KeyIndex: u32,
    pub KeyLength: u32,
    pub KeyMaterial: [u8; 1],
}
impl ::core::marker::Copy for NDIS_802_11_WEP {}
impl ::core::clone::Clone for NDIS_802_11_WEP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_802_11_WEP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_802_11_WEP").field("Length", &self.Length).field("KeyIndex", &self.KeyIndex).field("KeyLength", &self.KeyLength).field("KeyMaterial", &self.KeyMaterial).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_802_11_WEP {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_802_11_WEP {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.KeyIndex == other.KeyIndex && self.KeyLength == other.KeyLength && self.KeyMaterial == other.KeyMaterial
    }
}
impl ::core::cmp::Eq for NDIS_802_11_WEP {}
impl ::core::default::Default for NDIS_802_11_WEP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NDIS_AF_LIST(pub isize);
impl ::core::default::Default for NDIS_AF_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for NDIS_AF_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for NDIS_AF_LIST {}
impl ::core::fmt::Debug for NDIS_AF_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_AF_LIST").field(&self.0).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_AF_LIST {
    type TypeKind = ::windows_core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NDIS_CALL_MANAGER_CHARACTERISTICS(pub isize);
impl ::core::default::Default for NDIS_CALL_MANAGER_CHARACTERISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for NDIS_CALL_MANAGER_CHARACTERISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for NDIS_CALL_MANAGER_CHARACTERISTICS {}
impl ::core::fmt::Debug for NDIS_CALL_MANAGER_CHARACTERISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_CALL_MANAGER_CHARACTERISTICS").field(&self.0).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_CALL_MANAGER_CHARACTERISTICS {
    type TypeKind = ::windows_core::CopyType;
}
#[repr(C)]
pub struct NDIS_CONFIGURATION_PARAMETER {
    pub ParameterType: NDIS_PARAMETER_TYPE,
    pub ParameterData: NDIS_CONFIGURATION_PARAMETER_0,
}
impl ::core::marker::Copy for NDIS_CONFIGURATION_PARAMETER {}
impl ::core::clone::Clone for NDIS_CONFIGURATION_PARAMETER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for NDIS_CONFIGURATION_PARAMETER {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for NDIS_CONFIGURATION_PARAMETER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union NDIS_CONFIGURATION_PARAMETER_0 {
    pub IntegerData: u32,
    pub StringData: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub BinaryData: BINARY_DATA,
}
impl ::core::marker::Copy for NDIS_CONFIGURATION_PARAMETER_0 {}
impl ::core::clone::Clone for NDIS_CONFIGURATION_PARAMETER_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for NDIS_CONFIGURATION_PARAMETER_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for NDIS_CONFIGURATION_PARAMETER_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_CO_DEVICE_PROFILE {
    pub DeviceDescription: NDIS_VAR_DATA_DESC,
    pub DevSpecificInfo: NDIS_VAR_DATA_DESC,
    pub ulTAPISupplementaryPassThru: u32,
    pub ulAddressModes: u32,
    pub ulNumAddresses: u32,
    pub ulBearerModes: u32,
    pub ulMaxTxRate: u32,
    pub ulMinTxRate: u32,
    pub ulMaxRxRate: u32,
    pub ulMinRxRate: u32,
    pub ulMediaModes: u32,
    pub ulGenerateToneModes: u32,
    pub ulGenerateToneMaxNumFreq: u32,
    pub ulGenerateDigitModes: u32,
    pub ulMonitorToneMaxNumFreq: u32,
    pub ulMonitorToneMaxNumEntries: u32,
    pub ulMonitorDigitModes: u32,
    pub ulGatherDigitsMinTimeout: u32,
    pub ulGatherDigitsMaxTimeout: u32,
    pub ulDevCapFlags: u32,
    pub ulMaxNumActiveCalls: u32,
    pub ulAnswerMode: u32,
    pub ulUUIAcceptSize: u32,
    pub ulUUIAnswerSize: u32,
    pub ulUUIMakeCallSize: u32,
    pub ulUUIDropSize: u32,
    pub ulUUISendUserUserInfoSize: u32,
    pub ulUUICallInfoSize: u32,
}
impl ::core::marker::Copy for NDIS_CO_DEVICE_PROFILE {}
impl ::core::clone::Clone for NDIS_CO_DEVICE_PROFILE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_CO_DEVICE_PROFILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_CO_DEVICE_PROFILE")
            .field("DeviceDescription", &self.DeviceDescription)
            .field("DevSpecificInfo", &self.DevSpecificInfo)
            .field("ulTAPISupplementaryPassThru", &self.ulTAPISupplementaryPassThru)
            .field("ulAddressModes", &self.ulAddressModes)
            .field("ulNumAddresses", &self.ulNumAddresses)
            .field("ulBearerModes", &self.ulBearerModes)
            .field("ulMaxTxRate", &self.ulMaxTxRate)
            .field("ulMinTxRate", &self.ulMinTxRate)
            .field("ulMaxRxRate", &self.ulMaxRxRate)
            .field("ulMinRxRate", &self.ulMinRxRate)
            .field("ulMediaModes", &self.ulMediaModes)
            .field("ulGenerateToneModes", &self.ulGenerateToneModes)
            .field("ulGenerateToneMaxNumFreq", &self.ulGenerateToneMaxNumFreq)
            .field("ulGenerateDigitModes", &self.ulGenerateDigitModes)
            .field("ulMonitorToneMaxNumFreq", &self.ulMonitorToneMaxNumFreq)
            .field("ulMonitorToneMaxNumEntries", &self.ulMonitorToneMaxNumEntries)
            .field("ulMonitorDigitModes", &self.ulMonitorDigitModes)
            .field("ulGatherDigitsMinTimeout", &self.ulGatherDigitsMinTimeout)
            .field("ulGatherDigitsMaxTimeout", &self.ulGatherDigitsMaxTimeout)
            .field("ulDevCapFlags", &self.ulDevCapFlags)
            .field("ulMaxNumActiveCalls", &self.ulMaxNumActiveCalls)
            .field("ulAnswerMode", &self.ulAnswerMode)
            .field("ulUUIAcceptSize", &self.ulUUIAcceptSize)
            .field("ulUUIAnswerSize", &self.ulUUIAnswerSize)
            .field("ulUUIMakeCallSize", &self.ulUUIMakeCallSize)
            .field("ulUUIDropSize", &self.ulUUIDropSize)
            .field("ulUUISendUserUserInfoSize", &self.ulUUISendUserUserInfoSize)
            .field("ulUUICallInfoSize", &self.ulUUICallInfoSize)
            .finish()
    }
}
impl ::windows_core::TypeKind for NDIS_CO_DEVICE_PROFILE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_CO_DEVICE_PROFILE {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceDescription == other.DeviceDescription
            && self.DevSpecificInfo == other.DevSpecificInfo
            && self.ulTAPISupplementaryPassThru == other.ulTAPISupplementaryPassThru
            && self.ulAddressModes == other.ulAddressModes
            && self.ulNumAddresses == other.ulNumAddresses
            && self.ulBearerModes == other.ulBearerModes
            && self.ulMaxTxRate == other.ulMaxTxRate
            && self.ulMinTxRate == other.ulMinTxRate
            && self.ulMaxRxRate == other.ulMaxRxRate
            && self.ulMinRxRate == other.ulMinRxRate
            && self.ulMediaModes == other.ulMediaModes
            && self.ulGenerateToneModes == other.ulGenerateToneModes
            && self.ulGenerateToneMaxNumFreq == other.ulGenerateToneMaxNumFreq
            && self.ulGenerateDigitModes == other.ulGenerateDigitModes
            && self.ulMonitorToneMaxNumFreq == other.ulMonitorToneMaxNumFreq
            && self.ulMonitorToneMaxNumEntries == other.ulMonitorToneMaxNumEntries
            && self.ulMonitorDigitModes == other.ulMonitorDigitModes
            && self.ulGatherDigitsMinTimeout == other.ulGatherDigitsMinTimeout
            && self.ulGatherDigitsMaxTimeout == other.ulGatherDigitsMaxTimeout
            && self.ulDevCapFlags == other.ulDevCapFlags
            && self.ulMaxNumActiveCalls == other.ulMaxNumActiveCalls
            && self.ulAnswerMode == other.ulAnswerMode
            && self.ulUUIAcceptSize == other.ulUUIAcceptSize
            && self.ulUUIAnswerSize == other.ulUUIAnswerSize
            && self.ulUUIMakeCallSize == other.ulUUIMakeCallSize
            && self.ulUUIDropSize == other.ulUUIDropSize
            && self.ulUUISendUserUserInfoSize == other.ulUUISendUserUserInfoSize
            && self.ulUUICallInfoSize == other.ulUUICallInfoSize
    }
}
impl ::core::cmp::Eq for NDIS_CO_DEVICE_PROFILE {}
impl ::core::default::Default for NDIS_CO_DEVICE_PROFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_CO_LINK_SPEED {
    pub Outbound: u32,
    pub Inbound: u32,
}
impl ::core::marker::Copy for NDIS_CO_LINK_SPEED {}
impl ::core::clone::Clone for NDIS_CO_LINK_SPEED {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_CO_LINK_SPEED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_CO_LINK_SPEED").field("Outbound", &self.Outbound).field("Inbound", &self.Inbound).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_CO_LINK_SPEED {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_CO_LINK_SPEED {
    fn eq(&self, other: &Self) -> bool {
        self.Outbound == other.Outbound && self.Inbound == other.Inbound
    }
}
impl ::core::cmp::Eq for NDIS_CO_LINK_SPEED {}
impl ::core::default::Default for NDIS_CO_LINK_SPEED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
pub struct NDIS_DMA_BLOCK {
    pub MapRegisterBase: *mut ::core::ffi::c_void,
    pub AllocationEvent: super::super::Foundation::KEVENT,
    pub SystemAdapterObject: *mut ::core::ffi::c_void,
    pub Miniport: *mut ::core::ffi::c_void,
    pub InProgress: super::super::super::Win32::Foundation::BOOLEAN,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for NDIS_DMA_BLOCK {}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for NDIS_DMA_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
impl ::windows_core::TypeKind for NDIS_DMA_BLOCK {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for NDIS_DMA_BLOCK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Wdk_System_SystemServices")]
pub struct NDIS_DMA_DESCRIPTION {
    pub DemandMode: super::super::super::Win32::Foundation::BOOLEAN,
    pub AutoInitialize: super::super::super::Win32::Foundation::BOOLEAN,
    pub DmaChannelSpecified: super::super::super::Win32::Foundation::BOOLEAN,
    pub DmaWidth: super::super::System::SystemServices::DMA_WIDTH,
    pub DmaSpeed: super::super::System::SystemServices::DMA_SPEED,
    pub DmaPort: u32,
    pub DmaChannel: u32,
}
#[cfg(feature = "Wdk_System_SystemServices")]
impl ::core::marker::Copy for NDIS_DMA_DESCRIPTION {}
#[cfg(feature = "Wdk_System_SystemServices")]
impl ::core::clone::Clone for NDIS_DMA_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Wdk_System_SystemServices")]
impl ::core::fmt::Debug for NDIS_DMA_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_DMA_DESCRIPTION").field("DemandMode", &self.DemandMode).field("AutoInitialize", &self.AutoInitialize).field("DmaChannelSpecified", &self.DmaChannelSpecified).field("DmaWidth", &self.DmaWidth).field("DmaSpeed", &self.DmaSpeed).field("DmaPort", &self.DmaPort).field("DmaChannel", &self.DmaChannel).finish()
    }
}
#[cfg(feature = "Wdk_System_SystemServices")]
impl ::windows_core::TypeKind for NDIS_DMA_DESCRIPTION {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Wdk_System_SystemServices")]
impl ::core::cmp::PartialEq for NDIS_DMA_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.DemandMode == other.DemandMode && self.AutoInitialize == other.AutoInitialize && self.DmaChannelSpecified == other.DmaChannelSpecified && self.DmaWidth == other.DmaWidth && self.DmaSpeed == other.DmaSpeed && self.DmaPort == other.DmaPort && self.DmaChannel == other.DmaChannel
    }
}
#[cfg(feature = "Wdk_System_SystemServices")]
impl ::core::cmp::Eq for NDIS_DMA_DESCRIPTION {}
#[cfg(feature = "Wdk_System_SystemServices")]
impl ::core::default::Default for NDIS_DMA_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
pub struct NDIS_EVENT {
    pub Event: super::super::Foundation::KEVENT,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for NDIS_EVENT {}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for NDIS_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
impl ::windows_core::TypeKind for NDIS_EVENT {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for NDIS_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_GUID {
    pub Guid: ::windows_core::GUID,
    pub Anonymous: NDIS_GUID_0,
    pub Size: u32,
    pub Flags: u32,
}
impl ::core::marker::Copy for NDIS_GUID {}
impl ::core::clone::Clone for NDIS_GUID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for NDIS_GUID {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for NDIS_GUID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union NDIS_GUID_0 {
    pub Oid: u32,
    pub Status: i32,
}
impl ::core::marker::Copy for NDIS_GUID_0 {}
impl ::core::clone::Clone for NDIS_GUID_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for NDIS_GUID_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for NDIS_GUID_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_HARDWARE_CROSSTIMESTAMP {
    pub Header: NDIS_OBJECT_HEADER,
    pub Flags: u32,
    pub SystemTimestamp1: u64,
    pub HardwareClockTimestamp: u64,
    pub SystemTimestamp2: u64,
}
impl ::core::marker::Copy for NDIS_HARDWARE_CROSSTIMESTAMP {}
impl ::core::clone::Clone for NDIS_HARDWARE_CROSSTIMESTAMP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_HARDWARE_CROSSTIMESTAMP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_HARDWARE_CROSSTIMESTAMP").field("Header", &self.Header).field("Flags", &self.Flags).field("SystemTimestamp1", &self.SystemTimestamp1).field("HardwareClockTimestamp", &self.HardwareClockTimestamp).field("SystemTimestamp2", &self.SystemTimestamp2).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_HARDWARE_CROSSTIMESTAMP {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_HARDWARE_CROSSTIMESTAMP {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Flags == other.Flags && self.SystemTimestamp1 == other.SystemTimestamp1 && self.HardwareClockTimestamp == other.HardwareClockTimestamp && self.SystemTimestamp2 == other.SystemTimestamp2
    }
}
impl ::core::cmp::Eq for NDIS_HARDWARE_CROSSTIMESTAMP {}
impl ::core::default::Default for NDIS_HARDWARE_CROSSTIMESTAMP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_INTERRUPT_MODERATION_PARAMETERS {
    pub Header: NDIS_OBJECT_HEADER,
    pub Flags: u32,
    pub InterruptModeration: NDIS_INTERRUPT_MODERATION,
}
impl ::core::marker::Copy for NDIS_INTERRUPT_MODERATION_PARAMETERS {}
impl ::core::clone::Clone for NDIS_INTERRUPT_MODERATION_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_INTERRUPT_MODERATION_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_INTERRUPT_MODERATION_PARAMETERS").field("Header", &self.Header).field("Flags", &self.Flags).field("InterruptModeration", &self.InterruptModeration).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_INTERRUPT_MODERATION_PARAMETERS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_INTERRUPT_MODERATION_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Flags == other.Flags && self.InterruptModeration == other.InterruptModeration
    }
}
impl ::core::cmp::Eq for NDIS_INTERRUPT_MODERATION_PARAMETERS {}
impl ::core::default::Default for NDIS_INTERRUPT_MODERATION_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_IPSEC_OFFLOAD_V1 {
    pub Supported: NDIS_IPSEC_OFFLOAD_V1_2,
    pub IPv4AH: NDIS_IPSEC_OFFLOAD_V1_0,
    pub IPv4ESP: NDIS_IPSEC_OFFLOAD_V1_1,
}
impl ::core::marker::Copy for NDIS_IPSEC_OFFLOAD_V1 {}
impl ::core::clone::Clone for NDIS_IPSEC_OFFLOAD_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_IPSEC_OFFLOAD_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_IPSEC_OFFLOAD_V1").field("Supported", &self.Supported).field("IPv4AH", &self.IPv4AH).field("IPv4ESP", &self.IPv4ESP).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_IPSEC_OFFLOAD_V1 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_IPSEC_OFFLOAD_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.Supported == other.Supported && self.IPv4AH == other.IPv4AH && self.IPv4ESP == other.IPv4ESP
    }
}
impl ::core::cmp::Eq for NDIS_IPSEC_OFFLOAD_V1 {}
impl ::core::default::Default for NDIS_IPSEC_OFFLOAD_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_IPSEC_OFFLOAD_V1_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for NDIS_IPSEC_OFFLOAD_V1_0 {}
impl ::core::clone::Clone for NDIS_IPSEC_OFFLOAD_V1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_IPSEC_OFFLOAD_V1_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_IPSEC_OFFLOAD_V1_0").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_IPSEC_OFFLOAD_V1_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_IPSEC_OFFLOAD_V1_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for NDIS_IPSEC_OFFLOAD_V1_0 {}
impl ::core::default::Default for NDIS_IPSEC_OFFLOAD_V1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_IPSEC_OFFLOAD_V1_1 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for NDIS_IPSEC_OFFLOAD_V1_1 {}
impl ::core::clone::Clone for NDIS_IPSEC_OFFLOAD_V1_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_IPSEC_OFFLOAD_V1_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_IPSEC_OFFLOAD_V1_1").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_IPSEC_OFFLOAD_V1_1 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_IPSEC_OFFLOAD_V1_1 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for NDIS_IPSEC_OFFLOAD_V1_1 {}
impl ::core::default::Default for NDIS_IPSEC_OFFLOAD_V1_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_IPSEC_OFFLOAD_V1_2 {
    pub Encapsulation: u32,
    pub AhEspCombined: u32,
    pub TransportTunnelCombined: u32,
    pub IPv4Options: u32,
    pub Flags: u32,
}
impl ::core::marker::Copy for NDIS_IPSEC_OFFLOAD_V1_2 {}
impl ::core::clone::Clone for NDIS_IPSEC_OFFLOAD_V1_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_IPSEC_OFFLOAD_V1_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_IPSEC_OFFLOAD_V1_2").field("Encapsulation", &self.Encapsulation).field("AhEspCombined", &self.AhEspCombined).field("TransportTunnelCombined", &self.TransportTunnelCombined).field("IPv4Options", &self.IPv4Options).field("Flags", &self.Flags).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_IPSEC_OFFLOAD_V1_2 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_IPSEC_OFFLOAD_V1_2 {
    fn eq(&self, other: &Self) -> bool {
        self.Encapsulation == other.Encapsulation && self.AhEspCombined == other.AhEspCombined && self.TransportTunnelCombined == other.TransportTunnelCombined && self.IPv4Options == other.IPv4Options && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for NDIS_IPSEC_OFFLOAD_V1_2 {}
impl ::core::default::Default for NDIS_IPSEC_OFFLOAD_V1_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct NDIS_IP_OPER_STATE {
    pub Header: NDIS_OBJECT_HEADER,
    pub Flags: u32,
    pub IpOperationalStatus: NDIS_IP_OPER_STATUS,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for NDIS_IP_OPER_STATE {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for NDIS_IP_OPER_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for NDIS_IP_OPER_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_IP_OPER_STATE").field("Header", &self.Header).field("Flags", &self.Flags).field("IpOperationalStatus", &self.IpOperationalStatus).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows_core::TypeKind for NDIS_IP_OPER_STATE {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for NDIS_IP_OPER_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Flags == other.Flags && self.IpOperationalStatus == other.IpOperationalStatus
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for NDIS_IP_OPER_STATE {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for NDIS_IP_OPER_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct NDIS_IP_OPER_STATUS {
    pub AddressFamily: u32,
    pub OperationalStatus: super::super::super::Win32::NetworkManagement::Ndis::NET_IF_OPER_STATUS,
    pub OperationalStatusFlags: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for NDIS_IP_OPER_STATUS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for NDIS_IP_OPER_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for NDIS_IP_OPER_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_IP_OPER_STATUS").field("AddressFamily", &self.AddressFamily).field("OperationalStatus", &self.OperationalStatus).field("OperationalStatusFlags", &self.OperationalStatusFlags).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows_core::TypeKind for NDIS_IP_OPER_STATUS {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for NDIS_IP_OPER_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.AddressFamily == other.AddressFamily && self.OperationalStatus == other.OperationalStatus && self.OperationalStatusFlags == other.OperationalStatusFlags
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for NDIS_IP_OPER_STATUS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for NDIS_IP_OPER_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct NDIS_IP_OPER_STATUS_INFO {
    pub Header: NDIS_OBJECT_HEADER,
    pub Flags: u32,
    pub NumberofAddressFamiliesReturned: u32,
    pub IpOperationalStatus: [NDIS_IP_OPER_STATUS; 32],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for NDIS_IP_OPER_STATUS_INFO {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for NDIS_IP_OPER_STATUS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for NDIS_IP_OPER_STATUS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_IP_OPER_STATUS_INFO").field("Header", &self.Header).field("Flags", &self.Flags).field("NumberofAddressFamiliesReturned", &self.NumberofAddressFamiliesReturned).field("IpOperationalStatus", &self.IpOperationalStatus).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows_core::TypeKind for NDIS_IP_OPER_STATUS_INFO {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for NDIS_IP_OPER_STATUS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Flags == other.Flags && self.NumberofAddressFamiliesReturned == other.NumberofAddressFamiliesReturned && self.IpOperationalStatus == other.IpOperationalStatus
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for NDIS_IP_OPER_STATUS_INFO {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for NDIS_IP_OPER_STATUS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_IRDA_PACKET_INFO {
    pub ExtraBOFs: u32,
    pub MinTurnAroundTime: u32,
}
impl ::core::marker::Copy for NDIS_IRDA_PACKET_INFO {}
impl ::core::clone::Clone for NDIS_IRDA_PACKET_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_IRDA_PACKET_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_IRDA_PACKET_INFO").field("ExtraBOFs", &self.ExtraBOFs).field("MinTurnAroundTime", &self.MinTurnAroundTime).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_IRDA_PACKET_INFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_IRDA_PACKET_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ExtraBOFs == other.ExtraBOFs && self.MinTurnAroundTime == other.MinTurnAroundTime
    }
}
impl ::core::cmp::Eq for NDIS_IRDA_PACKET_INFO {}
impl ::core::default::Default for NDIS_IRDA_PACKET_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct NDIS_LINK_PARAMETERS {
    pub Header: NDIS_OBJECT_HEADER,
    pub MediaDuplexState: super::super::super::Win32::NetworkManagement::Ndis::NET_IF_MEDIA_DUPLEX_STATE,
    pub XmitLinkSpeed: u64,
    pub RcvLinkSpeed: u64,
    pub PauseFunctions: NDIS_SUPPORTED_PAUSE_FUNCTIONS,
    pub AutoNegotiationFlags: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for NDIS_LINK_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for NDIS_LINK_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for NDIS_LINK_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_LINK_PARAMETERS").field("Header", &self.Header).field("MediaDuplexState", &self.MediaDuplexState).field("XmitLinkSpeed", &self.XmitLinkSpeed).field("RcvLinkSpeed", &self.RcvLinkSpeed).field("PauseFunctions", &self.PauseFunctions).field("AutoNegotiationFlags", &self.AutoNegotiationFlags).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows_core::TypeKind for NDIS_LINK_PARAMETERS {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for NDIS_LINK_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.MediaDuplexState == other.MediaDuplexState && self.XmitLinkSpeed == other.XmitLinkSpeed && self.RcvLinkSpeed == other.RcvLinkSpeed && self.PauseFunctions == other.PauseFunctions && self.AutoNegotiationFlags == other.AutoNegotiationFlags
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for NDIS_LINK_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for NDIS_LINK_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_LINK_SPEED {
    pub XmitLinkSpeed: u64,
    pub RcvLinkSpeed: u64,
}
impl ::core::marker::Copy for NDIS_LINK_SPEED {}
impl ::core::clone::Clone for NDIS_LINK_SPEED {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_LINK_SPEED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_LINK_SPEED").field("XmitLinkSpeed", &self.XmitLinkSpeed).field("RcvLinkSpeed", &self.RcvLinkSpeed).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_LINK_SPEED {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_LINK_SPEED {
    fn eq(&self, other: &Self) -> bool {
        self.XmitLinkSpeed == other.XmitLinkSpeed && self.RcvLinkSpeed == other.RcvLinkSpeed
    }
}
impl ::core::cmp::Eq for NDIS_LINK_SPEED {}
impl ::core::default::Default for NDIS_LINK_SPEED {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct NDIS_LINK_STATE {
    pub Header: NDIS_OBJECT_HEADER,
    pub MediaConnectState: super::super::super::Win32::NetworkManagement::Ndis::NET_IF_MEDIA_CONNECT_STATE,
    pub MediaDuplexState: super::super::super::Win32::NetworkManagement::Ndis::NET_IF_MEDIA_DUPLEX_STATE,
    pub XmitLinkSpeed: u64,
    pub RcvLinkSpeed: u64,
    pub PauseFunctions: NDIS_SUPPORTED_PAUSE_FUNCTIONS,
    pub AutoNegotiationFlags: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for NDIS_LINK_STATE {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for NDIS_LINK_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for NDIS_LINK_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_LINK_STATE").field("Header", &self.Header).field("MediaConnectState", &self.MediaConnectState).field("MediaDuplexState", &self.MediaDuplexState).field("XmitLinkSpeed", &self.XmitLinkSpeed).field("RcvLinkSpeed", &self.RcvLinkSpeed).field("PauseFunctions", &self.PauseFunctions).field("AutoNegotiationFlags", &self.AutoNegotiationFlags).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows_core::TypeKind for NDIS_LINK_STATE {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for NDIS_LINK_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.MediaConnectState == other.MediaConnectState && self.MediaDuplexState == other.MediaDuplexState && self.XmitLinkSpeed == other.XmitLinkSpeed && self.RcvLinkSpeed == other.RcvLinkSpeed && self.PauseFunctions == other.PauseFunctions && self.AutoNegotiationFlags == other.AutoNegotiationFlags
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for NDIS_LINK_STATE {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for NDIS_LINK_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NDIS_MINIPORT_BLOCK(pub isize);
impl ::core::default::Default for NDIS_MINIPORT_BLOCK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for NDIS_MINIPORT_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for NDIS_MINIPORT_BLOCK {}
impl ::core::fmt::Debug for NDIS_MINIPORT_BLOCK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_MINIPORT_BLOCK").field(&self.0).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_MINIPORT_BLOCK {
    type TypeKind = ::windows_core::CopyType;
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_System_Kernel"))]
pub struct NDIS_MINIPORT_TIMER {
    pub Timer: super::super::System::SystemServices::KTIMER,
    pub Dpc: super::super::Foundation::KDPC,
    pub MiniportTimerFunction: PNDIS_TIMER_FUNCTION,
    pub MiniportTimerContext: *mut ::core::ffi::c_void,
    pub Miniport: *mut NDIS_MINIPORT_BLOCK,
    pub NextTimer: *mut NDIS_MINIPORT_TIMER,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for NDIS_MINIPORT_TIMER {}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for NDIS_MINIPORT_TIMER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_System_Kernel"))]
impl ::windows_core::TypeKind for NDIS_MINIPORT_TIMER {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for NDIS_MINIPORT_TIMER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NDIS_M_DRIVER_BLOCK(pub isize);
impl ::core::default::Default for NDIS_M_DRIVER_BLOCK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for NDIS_M_DRIVER_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for NDIS_M_DRIVER_BLOCK {}
impl ::core::fmt::Debug for NDIS_M_DRIVER_BLOCK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_M_DRIVER_BLOCK").field(&self.0).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_M_DRIVER_BLOCK {
    type TypeKind = ::windows_core::CopyType;
}
#[repr(C)]
pub struct NDIS_OBJECT_HEADER {
    pub Type: u8,
    pub Revision: u8,
    pub Size: u16,
}
impl ::core::marker::Copy for NDIS_OBJECT_HEADER {}
impl ::core::clone::Clone for NDIS_OBJECT_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_OBJECT_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_OBJECT_HEADER").field("Type", &self.Type).field("Revision", &self.Revision).field("Size", &self.Size).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_OBJECT_HEADER {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_OBJECT_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Revision == other.Revision && self.Size == other.Size
    }
}
impl ::core::cmp::Eq for NDIS_OBJECT_HEADER {}
impl ::core::default::Default for NDIS_OBJECT_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_OFFLOAD {
    pub Header: NDIS_OBJECT_HEADER,
    pub Checksum: NDIS_TCP_IP_CHECKSUM_OFFLOAD,
    pub LsoV1: NDIS_TCP_LARGE_SEND_OFFLOAD_V1,
    pub IPsecV1: NDIS_IPSEC_OFFLOAD_V1,
    pub LsoV2: NDIS_TCP_LARGE_SEND_OFFLOAD_V2,
    pub Flags: u32,
}
impl ::core::marker::Copy for NDIS_OFFLOAD {}
impl ::core::clone::Clone for NDIS_OFFLOAD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_OFFLOAD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_OFFLOAD").field("Header", &self.Header).field("Checksum", &self.Checksum).field("LsoV1", &self.LsoV1).field("IPsecV1", &self.IPsecV1).field("LsoV2", &self.LsoV2).field("Flags", &self.Flags).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_OFFLOAD {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_OFFLOAD {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Checksum == other.Checksum && self.LsoV1 == other.LsoV1 && self.IPsecV1 == other.IPsecV1 && self.LsoV2 == other.LsoV2 && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for NDIS_OFFLOAD {}
impl ::core::default::Default for NDIS_OFFLOAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_OFFLOAD_PARAMETERS {
    pub Header: NDIS_OBJECT_HEADER,
    pub IPv4Checksum: u8,
    pub TCPIPv4Checksum: u8,
    pub UDPIPv4Checksum: u8,
    pub TCPIPv6Checksum: u8,
    pub UDPIPv6Checksum: u8,
    pub LsoV1: u8,
    pub IPsecV1: u8,
    pub LsoV2IPv4: u8,
    pub LsoV2IPv6: u8,
    pub TcpConnectionIPv4: u8,
    pub TcpConnectionIPv6: u8,
    pub Flags: u32,
}
impl ::core::marker::Copy for NDIS_OFFLOAD_PARAMETERS {}
impl ::core::clone::Clone for NDIS_OFFLOAD_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_OFFLOAD_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_OFFLOAD_PARAMETERS")
            .field("Header", &self.Header)
            .field("IPv4Checksum", &self.IPv4Checksum)
            .field("TCPIPv4Checksum", &self.TCPIPv4Checksum)
            .field("UDPIPv4Checksum", &self.UDPIPv4Checksum)
            .field("TCPIPv6Checksum", &self.TCPIPv6Checksum)
            .field("UDPIPv6Checksum", &self.UDPIPv6Checksum)
            .field("LsoV1", &self.LsoV1)
            .field("IPsecV1", &self.IPsecV1)
            .field("LsoV2IPv4", &self.LsoV2IPv4)
            .field("LsoV2IPv6", &self.LsoV2IPv6)
            .field("TcpConnectionIPv4", &self.TcpConnectionIPv4)
            .field("TcpConnectionIPv6", &self.TcpConnectionIPv6)
            .field("Flags", &self.Flags)
            .finish()
    }
}
impl ::windows_core::TypeKind for NDIS_OFFLOAD_PARAMETERS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_OFFLOAD_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.IPv4Checksum == other.IPv4Checksum && self.TCPIPv4Checksum == other.TCPIPv4Checksum && self.UDPIPv4Checksum == other.UDPIPv4Checksum && self.TCPIPv6Checksum == other.TCPIPv6Checksum && self.UDPIPv6Checksum == other.UDPIPv6Checksum && self.LsoV1 == other.LsoV1 && self.IPsecV1 == other.IPsecV1 && self.LsoV2IPv4 == other.LsoV2IPv4 && self.LsoV2IPv6 == other.LsoV2IPv6 && self.TcpConnectionIPv4 == other.TcpConnectionIPv4 && self.TcpConnectionIPv6 == other.TcpConnectionIPv6 && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for NDIS_OFFLOAD_PARAMETERS {}
impl ::core::default::Default for NDIS_OFFLOAD_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NDIS_OPEN_BLOCK(pub isize);
impl ::core::default::Default for NDIS_OPEN_BLOCK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for NDIS_OPEN_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for NDIS_OPEN_BLOCK {}
impl ::core::fmt::Debug for NDIS_OPEN_BLOCK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_OPEN_BLOCK").field(&self.0).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_OPEN_BLOCK {
    type TypeKind = ::windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct NDIS_OPER_STATE {
    pub Header: NDIS_OBJECT_HEADER,
    pub OperationalStatus: super::super::super::Win32::NetworkManagement::Ndis::NET_IF_OPER_STATUS,
    pub OperationalStatusFlags: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for NDIS_OPER_STATE {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for NDIS_OPER_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for NDIS_OPER_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_OPER_STATE").field("Header", &self.Header).field("OperationalStatus", &self.OperationalStatus).field("OperationalStatusFlags", &self.OperationalStatusFlags).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows_core::TypeKind for NDIS_OPER_STATE {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for NDIS_OPER_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.OperationalStatus == other.OperationalStatus && self.OperationalStatusFlags == other.OperationalStatusFlags
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for NDIS_OPER_STATE {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for NDIS_OPER_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_PACKET_8021Q_INFO {
    pub Anonymous: NDIS_PACKET_8021Q_INFO_0,
}
impl ::core::marker::Copy for NDIS_PACKET_8021Q_INFO {}
impl ::core::clone::Clone for NDIS_PACKET_8021Q_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for NDIS_PACKET_8021Q_INFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for NDIS_PACKET_8021Q_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union NDIS_PACKET_8021Q_INFO_0 {
    pub TagHeader: NDIS_PACKET_8021Q_INFO_0_0,
    pub Value: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for NDIS_PACKET_8021Q_INFO_0 {}
impl ::core::clone::Clone for NDIS_PACKET_8021Q_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for NDIS_PACKET_8021Q_INFO_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for NDIS_PACKET_8021Q_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_PACKET_8021Q_INFO_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for NDIS_PACKET_8021Q_INFO_0_0 {}
impl ::core::clone::Clone for NDIS_PACKET_8021Q_INFO_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_PACKET_8021Q_INFO_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_PACKET_8021Q_INFO_0_0").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_PACKET_8021Q_INFO_0_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_PACKET_8021Q_INFO_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for NDIS_PACKET_8021Q_INFO_0_0 {}
impl ::core::default::Default for NDIS_PACKET_8021Q_INFO_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_PCI_DEVICE_CUSTOM_PROPERTIES {
    pub Header: NDIS_OBJECT_HEADER,
    pub DeviceType: u32,
    pub CurrentSpeedAndMode: u32,
    pub CurrentPayloadSize: u32,
    pub MaxPayloadSize: u32,
    pub MaxReadRequestSize: u32,
    pub CurrentLinkSpeed: u32,
    pub CurrentLinkWidth: u32,
    pub MaxLinkSpeed: u32,
    pub MaxLinkWidth: u32,
    pub PciExpressVersion: u32,
    pub InterruptType: u32,
    pub MaxInterruptMessages: u32,
}
impl ::core::marker::Copy for NDIS_PCI_DEVICE_CUSTOM_PROPERTIES {}
impl ::core::clone::Clone for NDIS_PCI_DEVICE_CUSTOM_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_PCI_DEVICE_CUSTOM_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_PCI_DEVICE_CUSTOM_PROPERTIES")
            .field("Header", &self.Header)
            .field("DeviceType", &self.DeviceType)
            .field("CurrentSpeedAndMode", &self.CurrentSpeedAndMode)
            .field("CurrentPayloadSize", &self.CurrentPayloadSize)
            .field("MaxPayloadSize", &self.MaxPayloadSize)
            .field("MaxReadRequestSize", &self.MaxReadRequestSize)
            .field("CurrentLinkSpeed", &self.CurrentLinkSpeed)
            .field("CurrentLinkWidth", &self.CurrentLinkWidth)
            .field("MaxLinkSpeed", &self.MaxLinkSpeed)
            .field("MaxLinkWidth", &self.MaxLinkWidth)
            .field("PciExpressVersion", &self.PciExpressVersion)
            .field("InterruptType", &self.InterruptType)
            .field("MaxInterruptMessages", &self.MaxInterruptMessages)
            .finish()
    }
}
impl ::windows_core::TypeKind for NDIS_PCI_DEVICE_CUSTOM_PROPERTIES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_PCI_DEVICE_CUSTOM_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.DeviceType == other.DeviceType && self.CurrentSpeedAndMode == other.CurrentSpeedAndMode && self.CurrentPayloadSize == other.CurrentPayloadSize && self.MaxPayloadSize == other.MaxPayloadSize && self.MaxReadRequestSize == other.MaxReadRequestSize && self.CurrentLinkSpeed == other.CurrentLinkSpeed && self.CurrentLinkWidth == other.CurrentLinkWidth && self.MaxLinkSpeed == other.MaxLinkSpeed && self.MaxLinkWidth == other.MaxLinkWidth && self.PciExpressVersion == other.PciExpressVersion && self.InterruptType == other.InterruptType && self.MaxInterruptMessages == other.MaxInterruptMessages
    }
}
impl ::core::cmp::Eq for NDIS_PCI_DEVICE_CUSTOM_PROPERTIES {}
impl ::core::default::Default for NDIS_PCI_DEVICE_CUSTOM_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NDIS_PD_COUNTER_HANDLE(pub isize);
impl ::core::default::Default for NDIS_PD_COUNTER_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for NDIS_PD_COUNTER_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for NDIS_PD_COUNTER_HANDLE {}
impl ::core::fmt::Debug for NDIS_PD_COUNTER_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_PD_COUNTER_HANDLE").field(&self.0).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_PD_COUNTER_HANDLE {
    type TypeKind = ::windows_core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NDIS_PD_FILTER_HANDLE(pub isize);
impl ::core::default::Default for NDIS_PD_FILTER_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for NDIS_PD_FILTER_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for NDIS_PD_FILTER_HANDLE {}
impl ::core::fmt::Debug for NDIS_PD_FILTER_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_PD_FILTER_HANDLE").field(&self.0).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_PD_FILTER_HANDLE {
    type TypeKind = ::windows_core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NDIS_PD_PROVIDER_HANDLE(pub isize);
impl ::core::default::Default for NDIS_PD_PROVIDER_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for NDIS_PD_PROVIDER_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for NDIS_PD_PROVIDER_HANDLE {}
impl ::core::fmt::Debug for NDIS_PD_PROVIDER_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_PD_PROVIDER_HANDLE").field(&self.0).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_PD_PROVIDER_HANDLE {
    type TypeKind = ::windows_core::CopyType;
}
#[repr(C)]
pub struct NDIS_PHYSICAL_ADDRESS_UNIT {
    pub PhysicalAddress: i64,
    pub Length: u32,
}
impl ::core::marker::Copy for NDIS_PHYSICAL_ADDRESS_UNIT {}
impl ::core::clone::Clone for NDIS_PHYSICAL_ADDRESS_UNIT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_PHYSICAL_ADDRESS_UNIT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_PHYSICAL_ADDRESS_UNIT").field("PhysicalAddress", &self.PhysicalAddress).field("Length", &self.Length).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_PHYSICAL_ADDRESS_UNIT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_PHYSICAL_ADDRESS_UNIT {
    fn eq(&self, other: &Self) -> bool {
        self.PhysicalAddress == other.PhysicalAddress && self.Length == other.Length
    }
}
impl ::core::cmp::Eq for NDIS_PHYSICAL_ADDRESS_UNIT {}
impl ::core::default::Default for NDIS_PHYSICAL_ADDRESS_UNIT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_PM_PACKET_PATTERN {
    pub Priority: u32,
    pub Reserved: u32,
    pub MaskSize: u32,
    pub PatternOffset: u32,
    pub PatternSize: u32,
    pub PatternFlags: u32,
}
impl ::core::marker::Copy for NDIS_PM_PACKET_PATTERN {}
impl ::core::clone::Clone for NDIS_PM_PACKET_PATTERN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_PM_PACKET_PATTERN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_PM_PACKET_PATTERN").field("Priority", &self.Priority).field("Reserved", &self.Reserved).field("MaskSize", &self.MaskSize).field("PatternOffset", &self.PatternOffset).field("PatternSize", &self.PatternSize).field("PatternFlags", &self.PatternFlags).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_PM_PACKET_PATTERN {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_PM_PACKET_PATTERN {
    fn eq(&self, other: &Self) -> bool {
        self.Priority == other.Priority && self.Reserved == other.Reserved && self.MaskSize == other.MaskSize && self.PatternOffset == other.PatternOffset && self.PatternSize == other.PatternSize && self.PatternFlags == other.PatternFlags
    }
}
impl ::core::cmp::Eq for NDIS_PM_PACKET_PATTERN {}
impl ::core::default::Default for NDIS_PM_PACKET_PATTERN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_PM_WAKE_UP_CAPABILITIES {
    pub MinMagicPacketWakeUp: NDIS_DEVICE_POWER_STATE,
    pub MinPatternWakeUp: NDIS_DEVICE_POWER_STATE,
    pub MinLinkChangeWakeUp: NDIS_DEVICE_POWER_STATE,
}
impl ::core::marker::Copy for NDIS_PM_WAKE_UP_CAPABILITIES {}
impl ::core::clone::Clone for NDIS_PM_WAKE_UP_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_PM_WAKE_UP_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_PM_WAKE_UP_CAPABILITIES").field("MinMagicPacketWakeUp", &self.MinMagicPacketWakeUp).field("MinPatternWakeUp", &self.MinPatternWakeUp).field("MinLinkChangeWakeUp", &self.MinLinkChangeWakeUp).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_PM_WAKE_UP_CAPABILITIES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_PM_WAKE_UP_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.MinMagicPacketWakeUp == other.MinMagicPacketWakeUp && self.MinPatternWakeUp == other.MinPatternWakeUp && self.MinLinkChangeWakeUp == other.MinLinkChangeWakeUp
    }
}
impl ::core::cmp::Eq for NDIS_PM_WAKE_UP_CAPABILITIES {}
impl ::core::default::Default for NDIS_PM_WAKE_UP_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_PNP_CAPABILITIES {
    pub Flags: u32,
    pub WakeUpCapabilities: NDIS_PM_WAKE_UP_CAPABILITIES,
}
impl ::core::marker::Copy for NDIS_PNP_CAPABILITIES {}
impl ::core::clone::Clone for NDIS_PNP_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_PNP_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_PNP_CAPABILITIES").field("Flags", &self.Flags).field("WakeUpCapabilities", &self.WakeUpCapabilities).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_PNP_CAPABILITIES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_PNP_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.WakeUpCapabilities == other.WakeUpCapabilities
    }
}
impl ::core::cmp::Eq for NDIS_PNP_CAPABILITIES {}
impl ::core::default::Default for NDIS_PNP_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NDIS_POLL_HANDLE(pub isize);
impl ::core::default::Default for NDIS_POLL_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for NDIS_POLL_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for NDIS_POLL_HANDLE {}
impl ::core::fmt::Debug for NDIS_POLL_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_POLL_HANDLE").field(&self.0).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_POLL_HANDLE {
    type TypeKind = ::windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct NDIS_PORT {
    pub Next: *mut NDIS_PORT,
    pub NdisReserved: *mut ::core::ffi::c_void,
    pub MiniportReserved: *mut ::core::ffi::c_void,
    pub ProtocolReserved: *mut ::core::ffi::c_void,
    pub PortCharacteristics: NDIS_PORT_CHARACTERISTICS,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for NDIS_PORT {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for NDIS_PORT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for NDIS_PORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_PORT").field("Next", &self.Next).field("NdisReserved", &self.NdisReserved).field("MiniportReserved", &self.MiniportReserved).field("ProtocolReserved", &self.ProtocolReserved).field("PortCharacteristics", &self.PortCharacteristics).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows_core::TypeKind for NDIS_PORT {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for NDIS_PORT {
    fn eq(&self, other: &Self) -> bool {
        self.Next == other.Next && self.NdisReserved == other.NdisReserved && self.MiniportReserved == other.MiniportReserved && self.ProtocolReserved == other.ProtocolReserved && self.PortCharacteristics == other.PortCharacteristics
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for NDIS_PORT {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for NDIS_PORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct NDIS_PORT_ARRAY {
    pub Header: NDIS_OBJECT_HEADER,
    pub NumberOfPorts: u32,
    pub OffsetFirstPort: u32,
    pub ElementSize: u32,
    pub Ports: [NDIS_PORT_CHARACTERISTICS; 1],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for NDIS_PORT_ARRAY {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for NDIS_PORT_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for NDIS_PORT_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_PORT_ARRAY").field("Header", &self.Header).field("NumberOfPorts", &self.NumberOfPorts).field("OffsetFirstPort", &self.OffsetFirstPort).field("ElementSize", &self.ElementSize).field("Ports", &self.Ports).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows_core::TypeKind for NDIS_PORT_ARRAY {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for NDIS_PORT_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.NumberOfPorts == other.NumberOfPorts && self.OffsetFirstPort == other.OffsetFirstPort && self.ElementSize == other.ElementSize && self.Ports == other.Ports
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for NDIS_PORT_ARRAY {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for NDIS_PORT_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_PORT_AUTHENTICATION_PARAMETERS {
    pub Header: NDIS_OBJECT_HEADER,
    pub SendControlState: NDIS_PORT_CONTROL_STATE,
    pub RcvControlState: NDIS_PORT_CONTROL_STATE,
    pub SendAuthorizationState: NDIS_PORT_AUTHORIZATION_STATE,
    pub RcvAuthorizationState: NDIS_PORT_AUTHORIZATION_STATE,
}
impl ::core::marker::Copy for NDIS_PORT_AUTHENTICATION_PARAMETERS {}
impl ::core::clone::Clone for NDIS_PORT_AUTHENTICATION_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_PORT_AUTHENTICATION_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_PORT_AUTHENTICATION_PARAMETERS").field("Header", &self.Header).field("SendControlState", &self.SendControlState).field("RcvControlState", &self.RcvControlState).field("SendAuthorizationState", &self.SendAuthorizationState).field("RcvAuthorizationState", &self.RcvAuthorizationState).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_PORT_AUTHENTICATION_PARAMETERS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_PORT_AUTHENTICATION_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.SendControlState == other.SendControlState && self.RcvControlState == other.RcvControlState && self.SendAuthorizationState == other.SendAuthorizationState && self.RcvAuthorizationState == other.RcvAuthorizationState
    }
}
impl ::core::cmp::Eq for NDIS_PORT_AUTHENTICATION_PARAMETERS {}
impl ::core::default::Default for NDIS_PORT_AUTHENTICATION_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct NDIS_PORT_CHARACTERISTICS {
    pub Header: NDIS_OBJECT_HEADER,
    pub PortNumber: u32,
    pub Flags: u32,
    pub Type: NDIS_PORT_TYPE,
    pub MediaConnectState: super::super::super::Win32::NetworkManagement::Ndis::NET_IF_MEDIA_CONNECT_STATE,
    pub XmitLinkSpeed: u64,
    pub RcvLinkSpeed: u64,
    pub Direction: super::super::super::Win32::NetworkManagement::Ndis::NET_IF_DIRECTION_TYPE,
    pub SendControlState: NDIS_PORT_CONTROL_STATE,
    pub RcvControlState: NDIS_PORT_CONTROL_STATE,
    pub SendAuthorizationState: NDIS_PORT_AUTHORIZATION_STATE,
    pub RcvAuthorizationState: NDIS_PORT_AUTHORIZATION_STATE,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for NDIS_PORT_CHARACTERISTICS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for NDIS_PORT_CHARACTERISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for NDIS_PORT_CHARACTERISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_PORT_CHARACTERISTICS")
            .field("Header", &self.Header)
            .field("PortNumber", &self.PortNumber)
            .field("Flags", &self.Flags)
            .field("Type", &self.Type)
            .field("MediaConnectState", &self.MediaConnectState)
            .field("XmitLinkSpeed", &self.XmitLinkSpeed)
            .field("RcvLinkSpeed", &self.RcvLinkSpeed)
            .field("Direction", &self.Direction)
            .field("SendControlState", &self.SendControlState)
            .field("RcvControlState", &self.RcvControlState)
            .field("SendAuthorizationState", &self.SendAuthorizationState)
            .field("RcvAuthorizationState", &self.RcvAuthorizationState)
            .finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows_core::TypeKind for NDIS_PORT_CHARACTERISTICS {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for NDIS_PORT_CHARACTERISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.PortNumber == other.PortNumber && self.Flags == other.Flags && self.Type == other.Type && self.MediaConnectState == other.MediaConnectState && self.XmitLinkSpeed == other.XmitLinkSpeed && self.RcvLinkSpeed == other.RcvLinkSpeed && self.Direction == other.Direction && self.SendControlState == other.SendControlState && self.RcvControlState == other.RcvControlState && self.SendAuthorizationState == other.SendAuthorizationState && self.RcvAuthorizationState == other.RcvAuthorizationState
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for NDIS_PORT_CHARACTERISTICS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for NDIS_PORT_CHARACTERISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct NDIS_PORT_STATE {
    pub Header: NDIS_OBJECT_HEADER,
    pub MediaConnectState: super::super::super::Win32::NetworkManagement::Ndis::NET_IF_MEDIA_CONNECT_STATE,
    pub XmitLinkSpeed: u64,
    pub RcvLinkSpeed: u64,
    pub Direction: super::super::super::Win32::NetworkManagement::Ndis::NET_IF_DIRECTION_TYPE,
    pub SendControlState: NDIS_PORT_CONTROL_STATE,
    pub RcvControlState: NDIS_PORT_CONTROL_STATE,
    pub SendAuthorizationState: NDIS_PORT_AUTHORIZATION_STATE,
    pub RcvAuthorizationState: NDIS_PORT_AUTHORIZATION_STATE,
    pub Flags: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for NDIS_PORT_STATE {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for NDIS_PORT_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for NDIS_PORT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_PORT_STATE")
            .field("Header", &self.Header)
            .field("MediaConnectState", &self.MediaConnectState)
            .field("XmitLinkSpeed", &self.XmitLinkSpeed)
            .field("RcvLinkSpeed", &self.RcvLinkSpeed)
            .field("Direction", &self.Direction)
            .field("SendControlState", &self.SendControlState)
            .field("RcvControlState", &self.RcvControlState)
            .field("SendAuthorizationState", &self.SendAuthorizationState)
            .field("RcvAuthorizationState", &self.RcvAuthorizationState)
            .field("Flags", &self.Flags)
            .finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows_core::TypeKind for NDIS_PORT_STATE {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for NDIS_PORT_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.MediaConnectState == other.MediaConnectState && self.XmitLinkSpeed == other.XmitLinkSpeed && self.RcvLinkSpeed == other.RcvLinkSpeed && self.Direction == other.Direction && self.SendControlState == other.SendControlState && self.RcvControlState == other.RcvControlState && self.SendAuthorizationState == other.SendAuthorizationState && self.RcvAuthorizationState == other.RcvAuthorizationState && self.Flags == other.Flags
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for NDIS_PORT_STATE {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for NDIS_PORT_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NDIS_PROTOCOL_BLOCK(pub isize);
impl ::core::default::Default for NDIS_PROTOCOL_BLOCK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for NDIS_PROTOCOL_BLOCK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for NDIS_PROTOCOL_BLOCK {}
impl ::core::fmt::Debug for NDIS_PROTOCOL_BLOCK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_PROTOCOL_BLOCK").field(&self.0).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_PROTOCOL_BLOCK {
    type TypeKind = ::windows_core::CopyType;
}
#[repr(C)]
pub struct NDIS_RECEIVE_HASH_PARAMETERS {
    pub Header: NDIS_OBJECT_HEADER,
    pub Flags: u32,
    pub HashInformation: u32,
    pub HashSecretKeySize: u16,
    pub HashSecretKeyOffset: u32,
}
impl ::core::marker::Copy for NDIS_RECEIVE_HASH_PARAMETERS {}
impl ::core::clone::Clone for NDIS_RECEIVE_HASH_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_RECEIVE_HASH_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_RECEIVE_HASH_PARAMETERS").field("Header", &self.Header).field("Flags", &self.Flags).field("HashInformation", &self.HashInformation).field("HashSecretKeySize", &self.HashSecretKeySize).field("HashSecretKeyOffset", &self.HashSecretKeyOffset).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_RECEIVE_HASH_PARAMETERS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_RECEIVE_HASH_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Flags == other.Flags && self.HashInformation == other.HashInformation && self.HashSecretKeySize == other.HashSecretKeySize && self.HashSecretKeyOffset == other.HashSecretKeyOffset
    }
}
impl ::core::cmp::Eq for NDIS_RECEIVE_HASH_PARAMETERS {}
impl ::core::default::Default for NDIS_RECEIVE_HASH_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_RECEIVE_SCALE_CAPABILITIES {
    pub Header: NDIS_OBJECT_HEADER,
    pub CapabilitiesFlags: u32,
    pub NumberOfInterruptMessages: u32,
    pub NumberOfReceiveQueues: u32,
}
impl ::core::marker::Copy for NDIS_RECEIVE_SCALE_CAPABILITIES {}
impl ::core::clone::Clone for NDIS_RECEIVE_SCALE_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_RECEIVE_SCALE_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_RECEIVE_SCALE_CAPABILITIES").field("Header", &self.Header).field("CapabilitiesFlags", &self.CapabilitiesFlags).field("NumberOfInterruptMessages", &self.NumberOfInterruptMessages).field("NumberOfReceiveQueues", &self.NumberOfReceiveQueues).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_RECEIVE_SCALE_CAPABILITIES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_RECEIVE_SCALE_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.CapabilitiesFlags == other.CapabilitiesFlags && self.NumberOfInterruptMessages == other.NumberOfInterruptMessages && self.NumberOfReceiveQueues == other.NumberOfReceiveQueues
    }
}
impl ::core::cmp::Eq for NDIS_RECEIVE_SCALE_CAPABILITIES {}
impl ::core::default::Default for NDIS_RECEIVE_SCALE_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_RECEIVE_SCALE_PARAMETERS {
    pub Header: NDIS_OBJECT_HEADER,
    pub Flags: u16,
    pub BaseCpuNumber: u16,
    pub HashInformation: u32,
    pub IndirectionTableSize: u16,
    pub IndirectionTableOffset: u32,
    pub HashSecretKeySize: u16,
    pub HashSecretKeyOffset: u32,
}
impl ::core::marker::Copy for NDIS_RECEIVE_SCALE_PARAMETERS {}
impl ::core::clone::Clone for NDIS_RECEIVE_SCALE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_RECEIVE_SCALE_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_RECEIVE_SCALE_PARAMETERS").field("Header", &self.Header).field("Flags", &self.Flags).field("BaseCpuNumber", &self.BaseCpuNumber).field("HashInformation", &self.HashInformation).field("IndirectionTableSize", &self.IndirectionTableSize).field("IndirectionTableOffset", &self.IndirectionTableOffset).field("HashSecretKeySize", &self.HashSecretKeySize).field("HashSecretKeyOffset", &self.HashSecretKeyOffset).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_RECEIVE_SCALE_PARAMETERS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_RECEIVE_SCALE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Flags == other.Flags && self.BaseCpuNumber == other.BaseCpuNumber && self.HashInformation == other.HashInformation && self.IndirectionTableSize == other.IndirectionTableSize && self.IndirectionTableOffset == other.IndirectionTableOffset && self.HashSecretKeySize == other.HashSecretKeySize && self.HashSecretKeyOffset == other.HashSecretKeyOffset
    }
}
impl ::core::cmp::Eq for NDIS_RECEIVE_SCALE_PARAMETERS {}
impl ::core::default::Default for NDIS_RECEIVE_SCALE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_RW_LOCK {
    pub Anonymous1: NDIS_RW_LOCK_0,
    pub Anonymous2: NDIS_RW_LOCK_1,
}
impl ::core::marker::Copy for NDIS_RW_LOCK {}
impl ::core::clone::Clone for NDIS_RW_LOCK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for NDIS_RW_LOCK {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for NDIS_RW_LOCK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union NDIS_RW_LOCK_0 {
    pub Anonymous: NDIS_RW_LOCK_0_0,
    pub Reserved: [u8; 16],
}
impl ::core::marker::Copy for NDIS_RW_LOCK_0 {}
impl ::core::clone::Clone for NDIS_RW_LOCK_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for NDIS_RW_LOCK_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for NDIS_RW_LOCK_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_RW_LOCK_0_0 {
    pub SpinLock: usize,
    pub Context: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for NDIS_RW_LOCK_0_0 {}
impl ::core::clone::Clone for NDIS_RW_LOCK_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_RW_LOCK_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_RW_LOCK_0_0").field("SpinLock", &self.SpinLock).field("Context", &self.Context).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_RW_LOCK_0_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_RW_LOCK_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self.SpinLock == other.SpinLock && self.Context == other.Context
    }
}
impl ::core::cmp::Eq for NDIS_RW_LOCK_0_0 {}
impl ::core::default::Default for NDIS_RW_LOCK_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union NDIS_RW_LOCK_1 {
    pub RefCount: [NDIS_RW_LOCK_REFCOUNT; 32],
    pub RefCountEx: [u32; 128],
    pub Anonymous: NDIS_RW_LOCK_1_0,
}
impl ::core::marker::Copy for NDIS_RW_LOCK_1 {}
impl ::core::clone::Clone for NDIS_RW_LOCK_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for NDIS_RW_LOCK_1 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for NDIS_RW_LOCK_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_RW_LOCK_1_0 {
    pub RefCountLock: usize,
    pub SharedRefCount: u32,
    pub WriterWaiting: super::super::super::Win32::Foundation::BOOLEAN,
}
impl ::core::marker::Copy for NDIS_RW_LOCK_1_0 {}
impl ::core::clone::Clone for NDIS_RW_LOCK_1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_RW_LOCK_1_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_RW_LOCK_1_0").field("RefCountLock", &self.RefCountLock).field("SharedRefCount", &self.SharedRefCount).field("WriterWaiting", &self.WriterWaiting).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_RW_LOCK_1_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_RW_LOCK_1_0 {
    fn eq(&self, other: &Self) -> bool {
        self.RefCountLock == other.RefCountLock && self.SharedRefCount == other.SharedRefCount && self.WriterWaiting == other.WriterWaiting
    }
}
impl ::core::cmp::Eq for NDIS_RW_LOCK_1_0 {}
impl ::core::default::Default for NDIS_RW_LOCK_1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union NDIS_RW_LOCK_REFCOUNT {
    pub RefCount: u32,
    pub cacheLine: [u8; 16],
}
impl ::core::marker::Copy for NDIS_RW_LOCK_REFCOUNT {}
impl ::core::clone::Clone for NDIS_RW_LOCK_REFCOUNT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for NDIS_RW_LOCK_REFCOUNT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for NDIS_RW_LOCK_REFCOUNT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_SPIN_LOCK {
    pub SpinLock: usize,
    pub OldIrql: u8,
}
impl ::core::marker::Copy for NDIS_SPIN_LOCK {}
impl ::core::clone::Clone for NDIS_SPIN_LOCK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_SPIN_LOCK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_SPIN_LOCK").field("SpinLock", &self.SpinLock).field("OldIrql", &self.OldIrql).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_SPIN_LOCK {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_SPIN_LOCK {
    fn eq(&self, other: &Self) -> bool {
        self.SpinLock == other.SpinLock && self.OldIrql == other.OldIrql
    }
}
impl ::core::cmp::Eq for NDIS_SPIN_LOCK {}
impl ::core::default::Default for NDIS_SPIN_LOCK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_STATISTICS_INFO {
    pub Header: NDIS_OBJECT_HEADER,
    pub SupportedStatistics: u32,
    pub ifInDiscards: u64,
    pub ifInErrors: u64,
    pub ifHCInOctets: u64,
    pub ifHCInUcastPkts: u64,
    pub ifHCInMulticastPkts: u64,
    pub ifHCInBroadcastPkts: u64,
    pub ifHCOutOctets: u64,
    pub ifHCOutUcastPkts: u64,
    pub ifHCOutMulticastPkts: u64,
    pub ifHCOutBroadcastPkts: u64,
    pub ifOutErrors: u64,
    pub ifOutDiscards: u64,
    pub ifHCInUcastOctets: u64,
    pub ifHCInMulticastOctets: u64,
    pub ifHCInBroadcastOctets: u64,
    pub ifHCOutUcastOctets: u64,
    pub ifHCOutMulticastOctets: u64,
    pub ifHCOutBroadcastOctets: u64,
}
impl ::core::marker::Copy for NDIS_STATISTICS_INFO {}
impl ::core::clone::Clone for NDIS_STATISTICS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_STATISTICS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_STATISTICS_INFO")
            .field("Header", &self.Header)
            .field("SupportedStatistics", &self.SupportedStatistics)
            .field("ifInDiscards", &self.ifInDiscards)
            .field("ifInErrors", &self.ifInErrors)
            .field("ifHCInOctets", &self.ifHCInOctets)
            .field("ifHCInUcastPkts", &self.ifHCInUcastPkts)
            .field("ifHCInMulticastPkts", &self.ifHCInMulticastPkts)
            .field("ifHCInBroadcastPkts", &self.ifHCInBroadcastPkts)
            .field("ifHCOutOctets", &self.ifHCOutOctets)
            .field("ifHCOutUcastPkts", &self.ifHCOutUcastPkts)
            .field("ifHCOutMulticastPkts", &self.ifHCOutMulticastPkts)
            .field("ifHCOutBroadcastPkts", &self.ifHCOutBroadcastPkts)
            .field("ifOutErrors", &self.ifOutErrors)
            .field("ifOutDiscards", &self.ifOutDiscards)
            .field("ifHCInUcastOctets", &self.ifHCInUcastOctets)
            .field("ifHCInMulticastOctets", &self.ifHCInMulticastOctets)
            .field("ifHCInBroadcastOctets", &self.ifHCInBroadcastOctets)
            .field("ifHCOutUcastOctets", &self.ifHCOutUcastOctets)
            .field("ifHCOutMulticastOctets", &self.ifHCOutMulticastOctets)
            .field("ifHCOutBroadcastOctets", &self.ifHCOutBroadcastOctets)
            .finish()
    }
}
impl ::windows_core::TypeKind for NDIS_STATISTICS_INFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_STATISTICS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header
            && self.SupportedStatistics == other.SupportedStatistics
            && self.ifInDiscards == other.ifInDiscards
            && self.ifInErrors == other.ifInErrors
            && self.ifHCInOctets == other.ifHCInOctets
            && self.ifHCInUcastPkts == other.ifHCInUcastPkts
            && self.ifHCInMulticastPkts == other.ifHCInMulticastPkts
            && self.ifHCInBroadcastPkts == other.ifHCInBroadcastPkts
            && self.ifHCOutOctets == other.ifHCOutOctets
            && self.ifHCOutUcastPkts == other.ifHCOutUcastPkts
            && self.ifHCOutMulticastPkts == other.ifHCOutMulticastPkts
            && self.ifHCOutBroadcastPkts == other.ifHCOutBroadcastPkts
            && self.ifOutErrors == other.ifOutErrors
            && self.ifOutDiscards == other.ifOutDiscards
            && self.ifHCInUcastOctets == other.ifHCInUcastOctets
            && self.ifHCInMulticastOctets == other.ifHCInMulticastOctets
            && self.ifHCInBroadcastOctets == other.ifHCInBroadcastOctets
            && self.ifHCOutUcastOctets == other.ifHCOutUcastOctets
            && self.ifHCOutMulticastOctets == other.ifHCOutMulticastOctets
            && self.ifHCOutBroadcastOctets == other.ifHCOutBroadcastOctets
    }
}
impl ::core::cmp::Eq for NDIS_STATISTICS_INFO {}
impl ::core::default::Default for NDIS_STATISTICS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_STATISTICS_VALUE {
    pub Oid: u32,
    pub DataLength: u32,
    pub Data: [u8; 1],
}
impl ::core::marker::Copy for NDIS_STATISTICS_VALUE {}
impl ::core::clone::Clone for NDIS_STATISTICS_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_STATISTICS_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_STATISTICS_VALUE").field("Oid", &self.Oid).field("DataLength", &self.DataLength).field("Data", &self.Data).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_STATISTICS_VALUE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_STATISTICS_VALUE {
    fn eq(&self, other: &Self) -> bool {
        self.Oid == other.Oid && self.DataLength == other.DataLength && self.Data == other.Data
    }
}
impl ::core::cmp::Eq for NDIS_STATISTICS_VALUE {}
impl ::core::default::Default for NDIS_STATISTICS_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_STATISTICS_VALUE_EX {
    pub Oid: u32,
    pub DataLength: u32,
    pub Length: u32,
    pub Data: [u8; 1],
}
impl ::core::marker::Copy for NDIS_STATISTICS_VALUE_EX {}
impl ::core::clone::Clone for NDIS_STATISTICS_VALUE_EX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_STATISTICS_VALUE_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_STATISTICS_VALUE_EX").field("Oid", &self.Oid).field("DataLength", &self.DataLength).field("Length", &self.Length).field("Data", &self.Data).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_STATISTICS_VALUE_EX {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_STATISTICS_VALUE_EX {
    fn eq(&self, other: &Self) -> bool {
        self.Oid == other.Oid && self.DataLength == other.DataLength && self.Length == other.Length && self.Data == other.Data
    }
}
impl ::core::cmp::Eq for NDIS_STATISTICS_VALUE_EX {}
impl ::core::default::Default for NDIS_STATISTICS_VALUE_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_TCP_CONNECTION_OFFLOAD {
    pub Header: NDIS_OBJECT_HEADER,
    pub Encapsulation: u32,
    pub _bitfield: u32,
    pub TcpConnectionOffloadCapacity: u32,
    pub Flags: u32,
}
impl ::core::marker::Copy for NDIS_TCP_CONNECTION_OFFLOAD {}
impl ::core::clone::Clone for NDIS_TCP_CONNECTION_OFFLOAD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_TCP_CONNECTION_OFFLOAD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_TCP_CONNECTION_OFFLOAD").field("Header", &self.Header).field("Encapsulation", &self.Encapsulation).field("_bitfield", &self._bitfield).field("TcpConnectionOffloadCapacity", &self.TcpConnectionOffloadCapacity).field("Flags", &self.Flags).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_TCP_CONNECTION_OFFLOAD {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_TCP_CONNECTION_OFFLOAD {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Encapsulation == other.Encapsulation && self._bitfield == other._bitfield && self.TcpConnectionOffloadCapacity == other.TcpConnectionOffloadCapacity && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for NDIS_TCP_CONNECTION_OFFLOAD {}
impl ::core::default::Default for NDIS_TCP_CONNECTION_OFFLOAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_TCP_IP_CHECKSUM_OFFLOAD {
    pub IPv4Transmit: NDIS_TCP_IP_CHECKSUM_OFFLOAD_1,
    pub IPv4Receive: NDIS_TCP_IP_CHECKSUM_OFFLOAD_0,
    pub IPv6Transmit: NDIS_TCP_IP_CHECKSUM_OFFLOAD_3,
    pub IPv6Receive: NDIS_TCP_IP_CHECKSUM_OFFLOAD_2,
}
impl ::core::marker::Copy for NDIS_TCP_IP_CHECKSUM_OFFLOAD {}
impl ::core::clone::Clone for NDIS_TCP_IP_CHECKSUM_OFFLOAD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_TCP_IP_CHECKSUM_OFFLOAD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_TCP_IP_CHECKSUM_OFFLOAD").field("IPv4Transmit", &self.IPv4Transmit).field("IPv4Receive", &self.IPv4Receive).field("IPv6Transmit", &self.IPv6Transmit).field("IPv6Receive", &self.IPv6Receive).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_TCP_IP_CHECKSUM_OFFLOAD {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_TCP_IP_CHECKSUM_OFFLOAD {
    fn eq(&self, other: &Self) -> bool {
        self.IPv4Transmit == other.IPv4Transmit && self.IPv4Receive == other.IPv4Receive && self.IPv6Transmit == other.IPv6Transmit && self.IPv6Receive == other.IPv6Receive
    }
}
impl ::core::cmp::Eq for NDIS_TCP_IP_CHECKSUM_OFFLOAD {}
impl ::core::default::Default for NDIS_TCP_IP_CHECKSUM_OFFLOAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_TCP_IP_CHECKSUM_OFFLOAD_0 {
    pub Encapsulation: u32,
    pub _bitfield: u32,
}
impl ::core::marker::Copy for NDIS_TCP_IP_CHECKSUM_OFFLOAD_0 {}
impl ::core::clone::Clone for NDIS_TCP_IP_CHECKSUM_OFFLOAD_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_TCP_IP_CHECKSUM_OFFLOAD_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_TCP_IP_CHECKSUM_OFFLOAD_0").field("Encapsulation", &self.Encapsulation).field("_bitfield", &self._bitfield).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_TCP_IP_CHECKSUM_OFFLOAD_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_TCP_IP_CHECKSUM_OFFLOAD_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Encapsulation == other.Encapsulation && self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for NDIS_TCP_IP_CHECKSUM_OFFLOAD_0 {}
impl ::core::default::Default for NDIS_TCP_IP_CHECKSUM_OFFLOAD_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_TCP_IP_CHECKSUM_OFFLOAD_1 {
    pub Encapsulation: u32,
    pub _bitfield: u32,
}
impl ::core::marker::Copy for NDIS_TCP_IP_CHECKSUM_OFFLOAD_1 {}
impl ::core::clone::Clone for NDIS_TCP_IP_CHECKSUM_OFFLOAD_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_TCP_IP_CHECKSUM_OFFLOAD_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_TCP_IP_CHECKSUM_OFFLOAD_1").field("Encapsulation", &self.Encapsulation).field("_bitfield", &self._bitfield).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_TCP_IP_CHECKSUM_OFFLOAD_1 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_TCP_IP_CHECKSUM_OFFLOAD_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Encapsulation == other.Encapsulation && self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for NDIS_TCP_IP_CHECKSUM_OFFLOAD_1 {}
impl ::core::default::Default for NDIS_TCP_IP_CHECKSUM_OFFLOAD_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_TCP_IP_CHECKSUM_OFFLOAD_2 {
    pub Encapsulation: u32,
    pub _bitfield: u32,
}
impl ::core::marker::Copy for NDIS_TCP_IP_CHECKSUM_OFFLOAD_2 {}
impl ::core::clone::Clone for NDIS_TCP_IP_CHECKSUM_OFFLOAD_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_TCP_IP_CHECKSUM_OFFLOAD_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_TCP_IP_CHECKSUM_OFFLOAD_2").field("Encapsulation", &self.Encapsulation).field("_bitfield", &self._bitfield).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_TCP_IP_CHECKSUM_OFFLOAD_2 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_TCP_IP_CHECKSUM_OFFLOAD_2 {
    fn eq(&self, other: &Self) -> bool {
        self.Encapsulation == other.Encapsulation && self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for NDIS_TCP_IP_CHECKSUM_OFFLOAD_2 {}
impl ::core::default::Default for NDIS_TCP_IP_CHECKSUM_OFFLOAD_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_TCP_IP_CHECKSUM_OFFLOAD_3 {
    pub Encapsulation: u32,
    pub _bitfield: u32,
}
impl ::core::marker::Copy for NDIS_TCP_IP_CHECKSUM_OFFLOAD_3 {}
impl ::core::clone::Clone for NDIS_TCP_IP_CHECKSUM_OFFLOAD_3 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_TCP_IP_CHECKSUM_OFFLOAD_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_TCP_IP_CHECKSUM_OFFLOAD_3").field("Encapsulation", &self.Encapsulation).field("_bitfield", &self._bitfield).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_TCP_IP_CHECKSUM_OFFLOAD_3 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_TCP_IP_CHECKSUM_OFFLOAD_3 {
    fn eq(&self, other: &Self) -> bool {
        self.Encapsulation == other.Encapsulation && self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for NDIS_TCP_IP_CHECKSUM_OFFLOAD_3 {}
impl ::core::default::Default for NDIS_TCP_IP_CHECKSUM_OFFLOAD_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_TCP_IP_CHECKSUM_PACKET_INFO {
    pub Anonymous: NDIS_TCP_IP_CHECKSUM_PACKET_INFO_0,
}
impl ::core::marker::Copy for NDIS_TCP_IP_CHECKSUM_PACKET_INFO {}
impl ::core::clone::Clone for NDIS_TCP_IP_CHECKSUM_PACKET_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for NDIS_TCP_IP_CHECKSUM_PACKET_INFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for NDIS_TCP_IP_CHECKSUM_PACKET_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub union NDIS_TCP_IP_CHECKSUM_PACKET_INFO_0 {
    pub Transmit: NDIS_TCP_IP_CHECKSUM_PACKET_INFO_0_1,
    pub Receive: NDIS_TCP_IP_CHECKSUM_PACKET_INFO_0_0,
    pub Value: u32,
}
impl ::core::marker::Copy for NDIS_TCP_IP_CHECKSUM_PACKET_INFO_0 {}
impl ::core::clone::Clone for NDIS_TCP_IP_CHECKSUM_PACKET_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows_core::TypeKind for NDIS_TCP_IP_CHECKSUM_PACKET_INFO_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for NDIS_TCP_IP_CHECKSUM_PACKET_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_TCP_IP_CHECKSUM_PACKET_INFO_0_0 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for NDIS_TCP_IP_CHECKSUM_PACKET_INFO_0_0 {}
impl ::core::clone::Clone for NDIS_TCP_IP_CHECKSUM_PACKET_INFO_0_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_TCP_IP_CHECKSUM_PACKET_INFO_0_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_TCP_IP_CHECKSUM_PACKET_INFO_0_0").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_TCP_IP_CHECKSUM_PACKET_INFO_0_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_TCP_IP_CHECKSUM_PACKET_INFO_0_0 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for NDIS_TCP_IP_CHECKSUM_PACKET_INFO_0_0 {}
impl ::core::default::Default for NDIS_TCP_IP_CHECKSUM_PACKET_INFO_0_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_TCP_IP_CHECKSUM_PACKET_INFO_0_1 {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for NDIS_TCP_IP_CHECKSUM_PACKET_INFO_0_1 {}
impl ::core::clone::Clone for NDIS_TCP_IP_CHECKSUM_PACKET_INFO_0_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_TCP_IP_CHECKSUM_PACKET_INFO_0_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_TCP_IP_CHECKSUM_PACKET_INFO_0_1").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_TCP_IP_CHECKSUM_PACKET_INFO_0_1 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_TCP_IP_CHECKSUM_PACKET_INFO_0_1 {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for NDIS_TCP_IP_CHECKSUM_PACKET_INFO_0_1 {}
impl ::core::default::Default for NDIS_TCP_IP_CHECKSUM_PACKET_INFO_0_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_TCP_LARGE_SEND_OFFLOAD_V1 {
    pub IPv4: NDIS_TCP_LARGE_SEND_OFFLOAD_V1_0,
}
impl ::core::marker::Copy for NDIS_TCP_LARGE_SEND_OFFLOAD_V1 {}
impl ::core::clone::Clone for NDIS_TCP_LARGE_SEND_OFFLOAD_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_TCP_LARGE_SEND_OFFLOAD_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_TCP_LARGE_SEND_OFFLOAD_V1").field("IPv4", &self.IPv4).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_TCP_LARGE_SEND_OFFLOAD_V1 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_TCP_LARGE_SEND_OFFLOAD_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.IPv4 == other.IPv4
    }
}
impl ::core::cmp::Eq for NDIS_TCP_LARGE_SEND_OFFLOAD_V1 {}
impl ::core::default::Default for NDIS_TCP_LARGE_SEND_OFFLOAD_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_TCP_LARGE_SEND_OFFLOAD_V1_0 {
    pub Encapsulation: u32,
    pub MaxOffLoadSize: u32,
    pub MinSegmentCount: u32,
    pub _bitfield: u32,
}
impl ::core::marker::Copy for NDIS_TCP_LARGE_SEND_OFFLOAD_V1_0 {}
impl ::core::clone::Clone for NDIS_TCP_LARGE_SEND_OFFLOAD_V1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_TCP_LARGE_SEND_OFFLOAD_V1_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_TCP_LARGE_SEND_OFFLOAD_V1_0").field("Encapsulation", &self.Encapsulation).field("MaxOffLoadSize", &self.MaxOffLoadSize).field("MinSegmentCount", &self.MinSegmentCount).field("_bitfield", &self._bitfield).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_TCP_LARGE_SEND_OFFLOAD_V1_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_TCP_LARGE_SEND_OFFLOAD_V1_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Encapsulation == other.Encapsulation && self.MaxOffLoadSize == other.MaxOffLoadSize && self.MinSegmentCount == other.MinSegmentCount && self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for NDIS_TCP_LARGE_SEND_OFFLOAD_V1_0 {}
impl ::core::default::Default for NDIS_TCP_LARGE_SEND_OFFLOAD_V1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_TCP_LARGE_SEND_OFFLOAD_V2 {
    pub IPv4: NDIS_TCP_LARGE_SEND_OFFLOAD_V2_0,
    pub IPv6: NDIS_TCP_LARGE_SEND_OFFLOAD_V2_1,
}
impl ::core::marker::Copy for NDIS_TCP_LARGE_SEND_OFFLOAD_V2 {}
impl ::core::clone::Clone for NDIS_TCP_LARGE_SEND_OFFLOAD_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_TCP_LARGE_SEND_OFFLOAD_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_TCP_LARGE_SEND_OFFLOAD_V2").field("IPv4", &self.IPv4).field("IPv6", &self.IPv6).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_TCP_LARGE_SEND_OFFLOAD_V2 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_TCP_LARGE_SEND_OFFLOAD_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.IPv4 == other.IPv4 && self.IPv6 == other.IPv6
    }
}
impl ::core::cmp::Eq for NDIS_TCP_LARGE_SEND_OFFLOAD_V2 {}
impl ::core::default::Default for NDIS_TCP_LARGE_SEND_OFFLOAD_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_TCP_LARGE_SEND_OFFLOAD_V2_0 {
    pub Encapsulation: u32,
    pub MaxOffLoadSize: u32,
    pub MinSegmentCount: u32,
}
impl ::core::marker::Copy for NDIS_TCP_LARGE_SEND_OFFLOAD_V2_0 {}
impl ::core::clone::Clone for NDIS_TCP_LARGE_SEND_OFFLOAD_V2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_TCP_LARGE_SEND_OFFLOAD_V2_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_TCP_LARGE_SEND_OFFLOAD_V2_0").field("Encapsulation", &self.Encapsulation).field("MaxOffLoadSize", &self.MaxOffLoadSize).field("MinSegmentCount", &self.MinSegmentCount).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_TCP_LARGE_SEND_OFFLOAD_V2_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_TCP_LARGE_SEND_OFFLOAD_V2_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Encapsulation == other.Encapsulation && self.MaxOffLoadSize == other.MaxOffLoadSize && self.MinSegmentCount == other.MinSegmentCount
    }
}
impl ::core::cmp::Eq for NDIS_TCP_LARGE_SEND_OFFLOAD_V2_0 {}
impl ::core::default::Default for NDIS_TCP_LARGE_SEND_OFFLOAD_V2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_TCP_LARGE_SEND_OFFLOAD_V2_1 {
    pub Encapsulation: u32,
    pub MaxOffLoadSize: u32,
    pub MinSegmentCount: u32,
    pub _bitfield: u32,
}
impl ::core::marker::Copy for NDIS_TCP_LARGE_SEND_OFFLOAD_V2_1 {}
impl ::core::clone::Clone for NDIS_TCP_LARGE_SEND_OFFLOAD_V2_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_TCP_LARGE_SEND_OFFLOAD_V2_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_TCP_LARGE_SEND_OFFLOAD_V2_1").field("Encapsulation", &self.Encapsulation).field("MaxOffLoadSize", &self.MaxOffLoadSize).field("MinSegmentCount", &self.MinSegmentCount).field("_bitfield", &self._bitfield).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_TCP_LARGE_SEND_OFFLOAD_V2_1 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_TCP_LARGE_SEND_OFFLOAD_V2_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Encapsulation == other.Encapsulation && self.MaxOffLoadSize == other.MaxOffLoadSize && self.MinSegmentCount == other.MinSegmentCount && self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for NDIS_TCP_LARGE_SEND_OFFLOAD_V2_1 {}
impl ::core::default::Default for NDIS_TCP_LARGE_SEND_OFFLOAD_V2_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_TIMEOUT_DPC_REQUEST_CAPABILITIES {
    pub Header: NDIS_OBJECT_HEADER,
    pub Flags: u32,
    pub TimeoutArrayLength: u32,
    pub TimeoutArray: [u32; 1],
}
impl ::core::marker::Copy for NDIS_TIMEOUT_DPC_REQUEST_CAPABILITIES {}
impl ::core::clone::Clone for NDIS_TIMEOUT_DPC_REQUEST_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_TIMEOUT_DPC_REQUEST_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_TIMEOUT_DPC_REQUEST_CAPABILITIES").field("Header", &self.Header).field("Flags", &self.Flags).field("TimeoutArrayLength", &self.TimeoutArrayLength).field("TimeoutArray", &self.TimeoutArray).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_TIMEOUT_DPC_REQUEST_CAPABILITIES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_TIMEOUT_DPC_REQUEST_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Flags == other.Flags && self.TimeoutArrayLength == other.TimeoutArrayLength && self.TimeoutArray == other.TimeoutArray
    }
}
impl ::core::cmp::Eq for NDIS_TIMEOUT_DPC_REQUEST_CAPABILITIES {}
impl ::core::default::Default for NDIS_TIMEOUT_DPC_REQUEST_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_System_Kernel"))]
pub struct NDIS_TIMER {
    pub Timer: super::super::System::SystemServices::KTIMER,
    pub Dpc: super::super::Foundation::KDPC,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_System_Kernel"))]
impl ::core::marker::Copy for NDIS_TIMER {}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_System_Kernel"))]
impl ::core::clone::Clone for NDIS_TIMER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_System_Kernel"))]
impl ::windows_core::TypeKind for NDIS_TIMER {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_System_Kernel"))]
impl ::core::default::Default for NDIS_TIMER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_TIMESTAMP_CAPABILITIES {
    pub Header: NDIS_OBJECT_HEADER,
    pub HardwareClockFrequencyHz: u64,
    pub CrossTimestamp: super::super::super::Win32::Foundation::BOOLEAN,
    pub Reserved1: u64,
    pub Reserved2: u64,
    pub TimestampFlags: NDIS_TIMESTAMP_CAPABILITY_FLAGS,
}
impl ::core::marker::Copy for NDIS_TIMESTAMP_CAPABILITIES {}
impl ::core::clone::Clone for NDIS_TIMESTAMP_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_TIMESTAMP_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_TIMESTAMP_CAPABILITIES").field("Header", &self.Header).field("HardwareClockFrequencyHz", &self.HardwareClockFrequencyHz).field("CrossTimestamp", &self.CrossTimestamp).field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).field("TimestampFlags", &self.TimestampFlags).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_TIMESTAMP_CAPABILITIES {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_TIMESTAMP_CAPABILITIES {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.HardwareClockFrequencyHz == other.HardwareClockFrequencyHz && self.CrossTimestamp == other.CrossTimestamp && self.Reserved1 == other.Reserved1 && self.Reserved2 == other.Reserved2 && self.TimestampFlags == other.TimestampFlags
    }
}
impl ::core::cmp::Eq for NDIS_TIMESTAMP_CAPABILITIES {}
impl ::core::default::Default for NDIS_TIMESTAMP_CAPABILITIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_TIMESTAMP_CAPABILITY_FLAGS {
    pub PtpV2OverUdpIPv4EventMsgReceiveHw: super::super::super::Win32::Foundation::BOOLEAN,
    pub PtpV2OverUdpIPv4AllMsgReceiveHw: super::super::super::Win32::Foundation::BOOLEAN,
    pub PtpV2OverUdpIPv4EventMsgTransmitHw: super::super::super::Win32::Foundation::BOOLEAN,
    pub PtpV2OverUdpIPv4AllMsgTransmitHw: super::super::super::Win32::Foundation::BOOLEAN,
    pub PtpV2OverUdpIPv6EventMsgReceiveHw: super::super::super::Win32::Foundation::BOOLEAN,
    pub PtpV2OverUdpIPv6AllMsgReceiveHw: super::super::super::Win32::Foundation::BOOLEAN,
    pub PtpV2OverUdpIPv6EventMsgTransmitHw: super::super::super::Win32::Foundation::BOOLEAN,
    pub PtpV2OverUdpIPv6AllMsgTransmitHw: super::super::super::Win32::Foundation::BOOLEAN,
    pub AllReceiveHw: super::super::super::Win32::Foundation::BOOLEAN,
    pub AllTransmitHw: super::super::super::Win32::Foundation::BOOLEAN,
    pub TaggedTransmitHw: super::super::super::Win32::Foundation::BOOLEAN,
    pub AllReceiveSw: super::super::super::Win32::Foundation::BOOLEAN,
    pub AllTransmitSw: super::super::super::Win32::Foundation::BOOLEAN,
    pub TaggedTransmitSw: super::super::super::Win32::Foundation::BOOLEAN,
}
impl ::core::marker::Copy for NDIS_TIMESTAMP_CAPABILITY_FLAGS {}
impl ::core::clone::Clone for NDIS_TIMESTAMP_CAPABILITY_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_TIMESTAMP_CAPABILITY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_TIMESTAMP_CAPABILITY_FLAGS")
            .field("PtpV2OverUdpIPv4EventMsgReceiveHw", &self.PtpV2OverUdpIPv4EventMsgReceiveHw)
            .field("PtpV2OverUdpIPv4AllMsgReceiveHw", &self.PtpV2OverUdpIPv4AllMsgReceiveHw)
            .field("PtpV2OverUdpIPv4EventMsgTransmitHw", &self.PtpV2OverUdpIPv4EventMsgTransmitHw)
            .field("PtpV2OverUdpIPv4AllMsgTransmitHw", &self.PtpV2OverUdpIPv4AllMsgTransmitHw)
            .field("PtpV2OverUdpIPv6EventMsgReceiveHw", &self.PtpV2OverUdpIPv6EventMsgReceiveHw)
            .field("PtpV2OverUdpIPv6AllMsgReceiveHw", &self.PtpV2OverUdpIPv6AllMsgReceiveHw)
            .field("PtpV2OverUdpIPv6EventMsgTransmitHw", &self.PtpV2OverUdpIPv6EventMsgTransmitHw)
            .field("PtpV2OverUdpIPv6AllMsgTransmitHw", &self.PtpV2OverUdpIPv6AllMsgTransmitHw)
            .field("AllReceiveHw", &self.AllReceiveHw)
            .field("AllTransmitHw", &self.AllTransmitHw)
            .field("TaggedTransmitHw", &self.TaggedTransmitHw)
            .field("AllReceiveSw", &self.AllReceiveSw)
            .field("AllTransmitSw", &self.AllTransmitSw)
            .field("TaggedTransmitSw", &self.TaggedTransmitSw)
            .finish()
    }
}
impl ::windows_core::TypeKind for NDIS_TIMESTAMP_CAPABILITY_FLAGS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_TIMESTAMP_CAPABILITY_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self.PtpV2OverUdpIPv4EventMsgReceiveHw == other.PtpV2OverUdpIPv4EventMsgReceiveHw
            && self.PtpV2OverUdpIPv4AllMsgReceiveHw == other.PtpV2OverUdpIPv4AllMsgReceiveHw
            && self.PtpV2OverUdpIPv4EventMsgTransmitHw == other.PtpV2OverUdpIPv4EventMsgTransmitHw
            && self.PtpV2OverUdpIPv4AllMsgTransmitHw == other.PtpV2OverUdpIPv4AllMsgTransmitHw
            && self.PtpV2OverUdpIPv6EventMsgReceiveHw == other.PtpV2OverUdpIPv6EventMsgReceiveHw
            && self.PtpV2OverUdpIPv6AllMsgReceiveHw == other.PtpV2OverUdpIPv6AllMsgReceiveHw
            && self.PtpV2OverUdpIPv6EventMsgTransmitHw == other.PtpV2OverUdpIPv6EventMsgTransmitHw
            && self.PtpV2OverUdpIPv6AllMsgTransmitHw == other.PtpV2OverUdpIPv6AllMsgTransmitHw
            && self.AllReceiveHw == other.AllReceiveHw
            && self.AllTransmitHw == other.AllTransmitHw
            && self.TaggedTransmitHw == other.TaggedTransmitHw
            && self.AllReceiveSw == other.AllReceiveSw
            && self.AllTransmitSw == other.AllTransmitSw
            && self.TaggedTransmitSw == other.TaggedTransmitSw
    }
}
impl ::core::cmp::Eq for NDIS_TIMESTAMP_CAPABILITY_FLAGS {}
impl ::core::default::Default for NDIS_TIMESTAMP_CAPABILITY_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_VAR_DATA_DESC {
    pub Length: u16,
    pub MaximumLength: u16,
    pub Offset: usize,
}
impl ::core::marker::Copy for NDIS_VAR_DATA_DESC {}
impl ::core::clone::Clone for NDIS_VAR_DATA_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_VAR_DATA_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_VAR_DATA_DESC").field("Length", &self.Length).field("MaximumLength", &self.MaximumLength).field("Offset", &self.Offset).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_VAR_DATA_DESC {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_VAR_DATA_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.MaximumLength == other.MaximumLength && self.Offset == other.Offset
    }
}
impl ::core::cmp::Eq for NDIS_VAR_DATA_DESC {}
impl ::core::default::Default for NDIS_VAR_DATA_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_WAN_FRAGMENT {
    pub RemoteAddress: [u8; 6],
    pub LocalAddress: [u8; 6],
}
impl ::core::marker::Copy for NDIS_WAN_FRAGMENT {}
impl ::core::clone::Clone for NDIS_WAN_FRAGMENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_WAN_FRAGMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_WAN_FRAGMENT").field("RemoteAddress", &self.RemoteAddress).field("LocalAddress", &self.LocalAddress).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_WAN_FRAGMENT {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_WAN_FRAGMENT {
    fn eq(&self, other: &Self) -> bool {
        self.RemoteAddress == other.RemoteAddress && self.LocalAddress == other.LocalAddress
    }
}
impl ::core::cmp::Eq for NDIS_WAN_FRAGMENT {}
impl ::core::default::Default for NDIS_WAN_FRAGMENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_WAN_GET_STATS {
    pub LocalAddress: [u8; 6],
    pub BytesSent: u32,
    pub BytesRcvd: u32,
    pub FramesSent: u32,
    pub FramesRcvd: u32,
    pub CRCErrors: u32,
    pub TimeoutErrors: u32,
    pub AlignmentErrors: u32,
    pub SerialOverrunErrors: u32,
    pub FramingErrors: u32,
    pub BufferOverrunErrors: u32,
    pub BytesTransmittedUncompressed: u32,
    pub BytesReceivedUncompressed: u32,
    pub BytesTransmittedCompressed: u32,
    pub BytesReceivedCompressed: u32,
}
impl ::core::marker::Copy for NDIS_WAN_GET_STATS {}
impl ::core::clone::Clone for NDIS_WAN_GET_STATS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_WAN_GET_STATS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_WAN_GET_STATS")
            .field("LocalAddress", &self.LocalAddress)
            .field("BytesSent", &self.BytesSent)
            .field("BytesRcvd", &self.BytesRcvd)
            .field("FramesSent", &self.FramesSent)
            .field("FramesRcvd", &self.FramesRcvd)
            .field("CRCErrors", &self.CRCErrors)
            .field("TimeoutErrors", &self.TimeoutErrors)
            .field("AlignmentErrors", &self.AlignmentErrors)
            .field("SerialOverrunErrors", &self.SerialOverrunErrors)
            .field("FramingErrors", &self.FramingErrors)
            .field("BufferOverrunErrors", &self.BufferOverrunErrors)
            .field("BytesTransmittedUncompressed", &self.BytesTransmittedUncompressed)
            .field("BytesReceivedUncompressed", &self.BytesReceivedUncompressed)
            .field("BytesTransmittedCompressed", &self.BytesTransmittedCompressed)
            .field("BytesReceivedCompressed", &self.BytesReceivedCompressed)
            .finish()
    }
}
impl ::windows_core::TypeKind for NDIS_WAN_GET_STATS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_WAN_GET_STATS {
    fn eq(&self, other: &Self) -> bool {
        self.LocalAddress == other.LocalAddress && self.BytesSent == other.BytesSent && self.BytesRcvd == other.BytesRcvd && self.FramesSent == other.FramesSent && self.FramesRcvd == other.FramesRcvd && self.CRCErrors == other.CRCErrors && self.TimeoutErrors == other.TimeoutErrors && self.AlignmentErrors == other.AlignmentErrors && self.SerialOverrunErrors == other.SerialOverrunErrors && self.FramingErrors == other.FramingErrors && self.BufferOverrunErrors == other.BufferOverrunErrors && self.BytesTransmittedUncompressed == other.BytesTransmittedUncompressed && self.BytesReceivedUncompressed == other.BytesReceivedUncompressed && self.BytesTransmittedCompressed == other.BytesTransmittedCompressed && self.BytesReceivedCompressed == other.BytesReceivedCompressed
    }
}
impl ::core::cmp::Eq for NDIS_WAN_GET_STATS {}
impl ::core::default::Default for NDIS_WAN_GET_STATS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_WAN_LINE_DOWN {
    pub RemoteAddress: [u8; 6],
    pub LocalAddress: [u8; 6],
}
impl ::core::marker::Copy for NDIS_WAN_LINE_DOWN {}
impl ::core::clone::Clone for NDIS_WAN_LINE_DOWN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_WAN_LINE_DOWN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_WAN_LINE_DOWN").field("RemoteAddress", &self.RemoteAddress).field("LocalAddress", &self.LocalAddress).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_WAN_LINE_DOWN {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_WAN_LINE_DOWN {
    fn eq(&self, other: &Self) -> bool {
        self.RemoteAddress == other.RemoteAddress && self.LocalAddress == other.LocalAddress
    }
}
impl ::core::cmp::Eq for NDIS_WAN_LINE_DOWN {}
impl ::core::default::Default for NDIS_WAN_LINE_DOWN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_WAN_LINE_UP {
    pub LinkSpeed: u32,
    pub MaximumTotalSize: u32,
    pub Quality: NDIS_WAN_QUALITY,
    pub SendWindow: u16,
    pub RemoteAddress: [u8; 6],
    pub LocalAddress: [u8; 6],
    pub ProtocolBufferLength: u32,
    pub ProtocolBuffer: *mut u8,
    pub ProtocolType: u16,
    pub DeviceName: super::super::super::Win32::Foundation::UNICODE_STRING,
}
impl ::core::marker::Copy for NDIS_WAN_LINE_UP {}
impl ::core::clone::Clone for NDIS_WAN_LINE_UP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_WAN_LINE_UP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_WAN_LINE_UP").field("LinkSpeed", &self.LinkSpeed).field("MaximumTotalSize", &self.MaximumTotalSize).field("Quality", &self.Quality).field("SendWindow", &self.SendWindow).field("RemoteAddress", &self.RemoteAddress).field("LocalAddress", &self.LocalAddress).field("ProtocolBufferLength", &self.ProtocolBufferLength).field("ProtocolBuffer", &self.ProtocolBuffer).field("ProtocolType", &self.ProtocolType).field("DeviceName", &self.DeviceName).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_WAN_LINE_UP {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_WAN_LINE_UP {
    fn eq(&self, other: &Self) -> bool {
        self.LinkSpeed == other.LinkSpeed && self.MaximumTotalSize == other.MaximumTotalSize && self.Quality == other.Quality && self.SendWindow == other.SendWindow && self.RemoteAddress == other.RemoteAddress && self.LocalAddress == other.LocalAddress && self.ProtocolBufferLength == other.ProtocolBufferLength && self.ProtocolBuffer == other.ProtocolBuffer && self.ProtocolType == other.ProtocolType && self.DeviceName == other.DeviceName
    }
}
impl ::core::cmp::Eq for NDIS_WAN_LINE_UP {}
impl ::core::default::Default for NDIS_WAN_LINE_UP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_WAN_PROTOCOL_CAPS {
    pub Flags: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for NDIS_WAN_PROTOCOL_CAPS {}
impl ::core::clone::Clone for NDIS_WAN_PROTOCOL_CAPS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_WAN_PROTOCOL_CAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_WAN_PROTOCOL_CAPS").field("Flags", &self.Flags).field("Reserved", &self.Reserved).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_WAN_PROTOCOL_CAPS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_WAN_PROTOCOL_CAPS {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for NDIS_WAN_PROTOCOL_CAPS {}
impl ::core::default::Default for NDIS_WAN_PROTOCOL_CAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_WLAN_BSSID {
    pub Length: u32,
    pub MacAddress: [u8; 6],
    pub Reserved: [u8; 2],
    pub Ssid: NDIS_802_11_SSID,
    pub Privacy: u32,
    pub Rssi: i32,
    pub NetworkTypeInUse: NDIS_802_11_NETWORK_TYPE,
    pub Configuration: NDIS_802_11_CONFIGURATION,
    pub InfrastructureMode: NDIS_802_11_NETWORK_INFRASTRUCTURE,
    pub SupportedRates: [u8; 8],
}
impl ::core::marker::Copy for NDIS_WLAN_BSSID {}
impl ::core::clone::Clone for NDIS_WLAN_BSSID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_WLAN_BSSID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_WLAN_BSSID").field("Length", &self.Length).field("MacAddress", &self.MacAddress).field("Reserved", &self.Reserved).field("Ssid", &self.Ssid).field("Privacy", &self.Privacy).field("Rssi", &self.Rssi).field("NetworkTypeInUse", &self.NetworkTypeInUse).field("Configuration", &self.Configuration).field("InfrastructureMode", &self.InfrastructureMode).field("SupportedRates", &self.SupportedRates).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_WLAN_BSSID {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_WLAN_BSSID {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.MacAddress == other.MacAddress && self.Reserved == other.Reserved && self.Ssid == other.Ssid && self.Privacy == other.Privacy && self.Rssi == other.Rssi && self.NetworkTypeInUse == other.NetworkTypeInUse && self.Configuration == other.Configuration && self.InfrastructureMode == other.InfrastructureMode && self.SupportedRates == other.SupportedRates
    }
}
impl ::core::cmp::Eq for NDIS_WLAN_BSSID {}
impl ::core::default::Default for NDIS_WLAN_BSSID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_WLAN_BSSID_EX {
    pub Length: u32,
    pub MacAddress: [u8; 6],
    pub Reserved: [u8; 2],
    pub Ssid: NDIS_802_11_SSID,
    pub Privacy: u32,
    pub Rssi: i32,
    pub NetworkTypeInUse: NDIS_802_11_NETWORK_TYPE,
    pub Configuration: NDIS_802_11_CONFIGURATION,
    pub InfrastructureMode: NDIS_802_11_NETWORK_INFRASTRUCTURE,
    pub SupportedRates: [u8; 16],
    pub IELength: u32,
    pub IEs: [u8; 1],
}
impl ::core::marker::Copy for NDIS_WLAN_BSSID_EX {}
impl ::core::clone::Clone for NDIS_WLAN_BSSID_EX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_WLAN_BSSID_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_WLAN_BSSID_EX")
            .field("Length", &self.Length)
            .field("MacAddress", &self.MacAddress)
            .field("Reserved", &self.Reserved)
            .field("Ssid", &self.Ssid)
            .field("Privacy", &self.Privacy)
            .field("Rssi", &self.Rssi)
            .field("NetworkTypeInUse", &self.NetworkTypeInUse)
            .field("Configuration", &self.Configuration)
            .field("InfrastructureMode", &self.InfrastructureMode)
            .field("SupportedRates", &self.SupportedRates)
            .field("IELength", &self.IELength)
            .field("IEs", &self.IEs)
            .finish()
    }
}
impl ::windows_core::TypeKind for NDIS_WLAN_BSSID_EX {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_WLAN_BSSID_EX {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.MacAddress == other.MacAddress && self.Reserved == other.Reserved && self.Ssid == other.Ssid && self.Privacy == other.Privacy && self.Rssi == other.Rssi && self.NetworkTypeInUse == other.NetworkTypeInUse && self.Configuration == other.Configuration && self.InfrastructureMode == other.InfrastructureMode && self.SupportedRates == other.SupportedRates && self.IELength == other.IELength && self.IEs == other.IEs
    }
}
impl ::core::cmp::Eq for NDIS_WLAN_BSSID_EX {}
impl ::core::default::Default for NDIS_WLAN_BSSID_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct NDIS_WMI_ENUM_ADAPTER {
    pub Header: NDIS_OBJECT_HEADER,
    pub IfIndex: u32,
    pub NetLuid: super::super::super::Win32::NetworkManagement::Ndis::NET_LUID_LH,
    pub DeviceNameLength: u16,
    pub DeviceName: [i8; 1],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for NDIS_WMI_ENUM_ADAPTER {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for NDIS_WMI_ENUM_ADAPTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows_core::TypeKind for NDIS_WMI_ENUM_ADAPTER {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for NDIS_WMI_ENUM_ADAPTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct NDIS_WMI_EVENT_HEADER {
    pub Header: NDIS_OBJECT_HEADER,
    pub IfIndex: u32,
    pub NetLuid: super::super::super::Win32::NetworkManagement::Ndis::NET_LUID_LH,
    pub RequestId: u64,
    pub PortNumber: u32,
    pub DeviceNameLength: u32,
    pub DeviceNameOffset: u32,
    pub Padding: [u8; 4],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for NDIS_WMI_EVENT_HEADER {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for NDIS_WMI_EVENT_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows_core::TypeKind for NDIS_WMI_EVENT_HEADER {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for NDIS_WMI_EVENT_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_WMI_IPSEC_OFFLOAD_V1 {
    pub Supported: NDIS_WMI_IPSEC_OFFLOAD_V1_2,
    pub IPv4AH: NDIS_WMI_IPSEC_OFFLOAD_V1_0,
    pub IPv4ESP: NDIS_WMI_IPSEC_OFFLOAD_V1_1,
}
impl ::core::marker::Copy for NDIS_WMI_IPSEC_OFFLOAD_V1 {}
impl ::core::clone::Clone for NDIS_WMI_IPSEC_OFFLOAD_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_WMI_IPSEC_OFFLOAD_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_WMI_IPSEC_OFFLOAD_V1").field("Supported", &self.Supported).field("IPv4AH", &self.IPv4AH).field("IPv4ESP", &self.IPv4ESP).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_WMI_IPSEC_OFFLOAD_V1 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_WMI_IPSEC_OFFLOAD_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.Supported == other.Supported && self.IPv4AH == other.IPv4AH && self.IPv4ESP == other.IPv4ESP
    }
}
impl ::core::cmp::Eq for NDIS_WMI_IPSEC_OFFLOAD_V1 {}
impl ::core::default::Default for NDIS_WMI_IPSEC_OFFLOAD_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_WMI_IPSEC_OFFLOAD_V1_0 {
    pub Md5: u32,
    pub Sha_1: u32,
    pub Transport: u32,
    pub Tunnel: u32,
    pub Send: u32,
    pub Receive: u32,
}
impl ::core::marker::Copy for NDIS_WMI_IPSEC_OFFLOAD_V1_0 {}
impl ::core::clone::Clone for NDIS_WMI_IPSEC_OFFLOAD_V1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_WMI_IPSEC_OFFLOAD_V1_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_WMI_IPSEC_OFFLOAD_V1_0").field("Md5", &self.Md5).field("Sha_1", &self.Sha_1).field("Transport", &self.Transport).field("Tunnel", &self.Tunnel).field("Send", &self.Send).field("Receive", &self.Receive).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_WMI_IPSEC_OFFLOAD_V1_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_WMI_IPSEC_OFFLOAD_V1_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Md5 == other.Md5 && self.Sha_1 == other.Sha_1 && self.Transport == other.Transport && self.Tunnel == other.Tunnel && self.Send == other.Send && self.Receive == other.Receive
    }
}
impl ::core::cmp::Eq for NDIS_WMI_IPSEC_OFFLOAD_V1_0 {}
impl ::core::default::Default for NDIS_WMI_IPSEC_OFFLOAD_V1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_WMI_IPSEC_OFFLOAD_V1_1 {
    pub Des: u32,
    pub Reserved: u32,
    pub TripleDes: u32,
    pub NullEsp: u32,
    pub Transport: u32,
    pub Tunnel: u32,
    pub Send: u32,
    pub Receive: u32,
}
impl ::core::marker::Copy for NDIS_WMI_IPSEC_OFFLOAD_V1_1 {}
impl ::core::clone::Clone for NDIS_WMI_IPSEC_OFFLOAD_V1_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_WMI_IPSEC_OFFLOAD_V1_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_WMI_IPSEC_OFFLOAD_V1_1").field("Des", &self.Des).field("Reserved", &self.Reserved).field("TripleDes", &self.TripleDes).field("NullEsp", &self.NullEsp).field("Transport", &self.Transport).field("Tunnel", &self.Tunnel).field("Send", &self.Send).field("Receive", &self.Receive).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_WMI_IPSEC_OFFLOAD_V1_1 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_WMI_IPSEC_OFFLOAD_V1_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Des == other.Des && self.Reserved == other.Reserved && self.TripleDes == other.TripleDes && self.NullEsp == other.NullEsp && self.Transport == other.Transport && self.Tunnel == other.Tunnel && self.Send == other.Send && self.Receive == other.Receive
    }
}
impl ::core::cmp::Eq for NDIS_WMI_IPSEC_OFFLOAD_V1_1 {}
impl ::core::default::Default for NDIS_WMI_IPSEC_OFFLOAD_V1_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_WMI_IPSEC_OFFLOAD_V1_2 {
    pub Encapsulation: u32,
    pub AhEspCombined: u32,
    pub TransportTunnelCombined: u32,
    pub IPv4Options: u32,
    pub Flags: u32,
}
impl ::core::marker::Copy for NDIS_WMI_IPSEC_OFFLOAD_V1_2 {}
impl ::core::clone::Clone for NDIS_WMI_IPSEC_OFFLOAD_V1_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_WMI_IPSEC_OFFLOAD_V1_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_WMI_IPSEC_OFFLOAD_V1_2").field("Encapsulation", &self.Encapsulation).field("AhEspCombined", &self.AhEspCombined).field("TransportTunnelCombined", &self.TransportTunnelCombined).field("IPv4Options", &self.IPv4Options).field("Flags", &self.Flags).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_WMI_IPSEC_OFFLOAD_V1_2 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_WMI_IPSEC_OFFLOAD_V1_2 {
    fn eq(&self, other: &Self) -> bool {
        self.Encapsulation == other.Encapsulation && self.AhEspCombined == other.AhEspCombined && self.TransportTunnelCombined == other.TransportTunnelCombined && self.IPv4Options == other.IPv4Options && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for NDIS_WMI_IPSEC_OFFLOAD_V1_2 {}
impl ::core::default::Default for NDIS_WMI_IPSEC_OFFLOAD_V1_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct NDIS_WMI_METHOD_HEADER {
    pub Header: NDIS_OBJECT_HEADER,
    pub PortNumber: u32,
    pub NetLuid: super::super::super::Win32::NetworkManagement::Ndis::NET_LUID_LH,
    pub RequestId: u64,
    pub Timeout: u32,
    pub Padding: [u8; 4],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for NDIS_WMI_METHOD_HEADER {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for NDIS_WMI_METHOD_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows_core::TypeKind for NDIS_WMI_METHOD_HEADER {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for NDIS_WMI_METHOD_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_WMI_OFFLOAD {
    pub Header: NDIS_OBJECT_HEADER,
    pub Checksum: NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD,
    pub LsoV1: NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1,
    pub IPsecV1: NDIS_WMI_IPSEC_OFFLOAD_V1,
    pub LsoV2: NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2,
    pub Flags: u32,
}
impl ::core::marker::Copy for NDIS_WMI_OFFLOAD {}
impl ::core::clone::Clone for NDIS_WMI_OFFLOAD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_WMI_OFFLOAD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_WMI_OFFLOAD").field("Header", &self.Header).field("Checksum", &self.Checksum).field("LsoV1", &self.LsoV1).field("IPsecV1", &self.IPsecV1).field("LsoV2", &self.LsoV2).field("Flags", &self.Flags).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_WMI_OFFLOAD {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_WMI_OFFLOAD {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Checksum == other.Checksum && self.LsoV1 == other.LsoV1 && self.IPsecV1 == other.IPsecV1 && self.LsoV2 == other.LsoV2 && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for NDIS_WMI_OFFLOAD {}
impl ::core::default::Default for NDIS_WMI_OFFLOAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_WMI_OUTPUT_INFO {
    pub Header: NDIS_OBJECT_HEADER,
    pub Flags: u32,
    pub SupportedRevision: u8,
    pub DataOffset: u32,
}
impl ::core::marker::Copy for NDIS_WMI_OUTPUT_INFO {}
impl ::core::clone::Clone for NDIS_WMI_OUTPUT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_WMI_OUTPUT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_WMI_OUTPUT_INFO").field("Header", &self.Header).field("Flags", &self.Flags).field("SupportedRevision", &self.SupportedRevision).field("DataOffset", &self.DataOffset).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_WMI_OUTPUT_INFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_WMI_OUTPUT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Flags == other.Flags && self.SupportedRevision == other.SupportedRevision && self.DataOffset == other.DataOffset
    }
}
impl ::core::cmp::Eq for NDIS_WMI_OUTPUT_INFO {}
impl ::core::default::Default for NDIS_WMI_OUTPUT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct NDIS_WMI_SET_HEADER {
    pub Header: NDIS_OBJECT_HEADER,
    pub PortNumber: u32,
    pub NetLuid: super::super::super::Win32::NetworkManagement::Ndis::NET_LUID_LH,
    pub RequestId: u64,
    pub Timeout: u32,
    pub Padding: [u8; 4],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for NDIS_WMI_SET_HEADER {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for NDIS_WMI_SET_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows_core::TypeKind for NDIS_WMI_SET_HEADER {
    type TypeKind = ::windows_core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for NDIS_WMI_SET_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_WMI_TCP_CONNECTION_OFFLOAD {
    pub Header: NDIS_OBJECT_HEADER,
    pub Encapsulation: u32,
    pub SupportIPv4: u32,
    pub SupportIPv6: u32,
    pub SupportIPv6ExtensionHeaders: u32,
    pub SupportSack: u32,
    pub TcpConnectionOffloadCapacity: u32,
    pub Flags: u32,
}
impl ::core::marker::Copy for NDIS_WMI_TCP_CONNECTION_OFFLOAD {}
impl ::core::clone::Clone for NDIS_WMI_TCP_CONNECTION_OFFLOAD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_WMI_TCP_CONNECTION_OFFLOAD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_WMI_TCP_CONNECTION_OFFLOAD").field("Header", &self.Header).field("Encapsulation", &self.Encapsulation).field("SupportIPv4", &self.SupportIPv4).field("SupportIPv6", &self.SupportIPv6).field("SupportIPv6ExtensionHeaders", &self.SupportIPv6ExtensionHeaders).field("SupportSack", &self.SupportSack).field("TcpConnectionOffloadCapacity", &self.TcpConnectionOffloadCapacity).field("Flags", &self.Flags).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_WMI_TCP_CONNECTION_OFFLOAD {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_WMI_TCP_CONNECTION_OFFLOAD {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Encapsulation == other.Encapsulation && self.SupportIPv4 == other.SupportIPv4 && self.SupportIPv6 == other.SupportIPv6 && self.SupportIPv6ExtensionHeaders == other.SupportIPv6ExtensionHeaders && self.SupportSack == other.SupportSack && self.TcpConnectionOffloadCapacity == other.TcpConnectionOffloadCapacity && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for NDIS_WMI_TCP_CONNECTION_OFFLOAD {}
impl ::core::default::Default for NDIS_WMI_TCP_CONNECTION_OFFLOAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD {
    pub IPv4Transmit: NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_1,
    pub IPv4Receive: NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_0,
    pub IPv6Transmit: NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_3,
    pub IPv6Receive: NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_2,
}
impl ::core::marker::Copy for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD {}
impl ::core::clone::Clone for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD").field("IPv4Transmit", &self.IPv4Transmit).field("IPv4Receive", &self.IPv4Receive).field("IPv6Transmit", &self.IPv6Transmit).field("IPv6Receive", &self.IPv6Receive).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD {
    fn eq(&self, other: &Self) -> bool {
        self.IPv4Transmit == other.IPv4Transmit && self.IPv4Receive == other.IPv4Receive && self.IPv6Transmit == other.IPv6Transmit && self.IPv6Receive == other.IPv6Receive
    }
}
impl ::core::cmp::Eq for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD {}
impl ::core::default::Default for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_0 {
    pub Encapsulation: u32,
    pub IpOptionsSupported: u32,
    pub TcpOptionsSupported: u32,
    pub TcpChecksum: u32,
    pub UdpChecksum: u32,
    pub IpChecksum: u32,
}
impl ::core::marker::Copy for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_0 {}
impl ::core::clone::Clone for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_0").field("Encapsulation", &self.Encapsulation).field("IpOptionsSupported", &self.IpOptionsSupported).field("TcpOptionsSupported", &self.TcpOptionsSupported).field("TcpChecksum", &self.TcpChecksum).field("UdpChecksum", &self.UdpChecksum).field("IpChecksum", &self.IpChecksum).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Encapsulation == other.Encapsulation && self.IpOptionsSupported == other.IpOptionsSupported && self.TcpOptionsSupported == other.TcpOptionsSupported && self.TcpChecksum == other.TcpChecksum && self.UdpChecksum == other.UdpChecksum && self.IpChecksum == other.IpChecksum
    }
}
impl ::core::cmp::Eq for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_0 {}
impl ::core::default::Default for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_1 {
    pub Encapsulation: u32,
    pub IpOptionsSupported: u32,
    pub TcpOptionsSupported: u32,
    pub TcpChecksum: u32,
    pub UdpChecksum: u32,
    pub IpChecksum: u32,
}
impl ::core::marker::Copy for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_1 {}
impl ::core::clone::Clone for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_1").field("Encapsulation", &self.Encapsulation).field("IpOptionsSupported", &self.IpOptionsSupported).field("TcpOptionsSupported", &self.TcpOptionsSupported).field("TcpChecksum", &self.TcpChecksum).field("UdpChecksum", &self.UdpChecksum).field("IpChecksum", &self.IpChecksum).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_1 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Encapsulation == other.Encapsulation && self.IpOptionsSupported == other.IpOptionsSupported && self.TcpOptionsSupported == other.TcpOptionsSupported && self.TcpChecksum == other.TcpChecksum && self.UdpChecksum == other.UdpChecksum && self.IpChecksum == other.IpChecksum
    }
}
impl ::core::cmp::Eq for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_1 {}
impl ::core::default::Default for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_2 {
    pub Encapsulation: u32,
    pub IpExtensionHeadersSupported: u32,
    pub TcpOptionsSupported: u32,
    pub TcpChecksum: u32,
    pub UdpChecksum: u32,
}
impl ::core::marker::Copy for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_2 {}
impl ::core::clone::Clone for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_2").field("Encapsulation", &self.Encapsulation).field("IpExtensionHeadersSupported", &self.IpExtensionHeadersSupported).field("TcpOptionsSupported", &self.TcpOptionsSupported).field("TcpChecksum", &self.TcpChecksum).field("UdpChecksum", &self.UdpChecksum).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_2 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_2 {
    fn eq(&self, other: &Self) -> bool {
        self.Encapsulation == other.Encapsulation && self.IpExtensionHeadersSupported == other.IpExtensionHeadersSupported && self.TcpOptionsSupported == other.TcpOptionsSupported && self.TcpChecksum == other.TcpChecksum && self.UdpChecksum == other.UdpChecksum
    }
}
impl ::core::cmp::Eq for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_2 {}
impl ::core::default::Default for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_3 {
    pub Encapsulation: u32,
    pub IpExtensionHeadersSupported: u32,
    pub TcpOptionsSupported: u32,
    pub TcpChecksum: u32,
    pub UdpChecksum: u32,
}
impl ::core::marker::Copy for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_3 {}
impl ::core::clone::Clone for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_3 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_3").field("Encapsulation", &self.Encapsulation).field("IpExtensionHeadersSupported", &self.IpExtensionHeadersSupported).field("TcpOptionsSupported", &self.TcpOptionsSupported).field("TcpChecksum", &self.TcpChecksum).field("UdpChecksum", &self.UdpChecksum).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_3 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_3 {
    fn eq(&self, other: &Self) -> bool {
        self.Encapsulation == other.Encapsulation && self.IpExtensionHeadersSupported == other.IpExtensionHeadersSupported && self.TcpOptionsSupported == other.TcpOptionsSupported && self.TcpChecksum == other.TcpChecksum && self.UdpChecksum == other.UdpChecksum
    }
}
impl ::core::cmp::Eq for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_3 {}
impl ::core::default::Default for NDIS_WMI_TCP_IP_CHECKSUM_OFFLOAD_3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1 {
    pub IPv4: NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1_0,
}
impl ::core::marker::Copy for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1 {}
impl ::core::clone::Clone for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1").field("IPv4", &self.IPv4).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.IPv4 == other.IPv4
    }
}
impl ::core::cmp::Eq for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1 {}
impl ::core::default::Default for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1_0 {
    pub Encapsulation: u32,
    pub MaxOffLoadSize: u32,
    pub MinSegmentCount: u32,
    pub TcpOptions: u32,
    pub IpOptions: u32,
}
impl ::core::marker::Copy for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1_0 {}
impl ::core::clone::Clone for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1_0").field("Encapsulation", &self.Encapsulation).field("MaxOffLoadSize", &self.MaxOffLoadSize).field("MinSegmentCount", &self.MinSegmentCount).field("TcpOptions", &self.TcpOptions).field("IpOptions", &self.IpOptions).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Encapsulation == other.Encapsulation && self.MaxOffLoadSize == other.MaxOffLoadSize && self.MinSegmentCount == other.MinSegmentCount && self.TcpOptions == other.TcpOptions && self.IpOptions == other.IpOptions
    }
}
impl ::core::cmp::Eq for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1_0 {}
impl ::core::default::Default for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V1_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2 {
    pub IPv4: NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2_0,
    pub IPv6: NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2_1,
}
impl ::core::marker::Copy for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2 {}
impl ::core::clone::Clone for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2").field("IPv4", &self.IPv4).field("IPv6", &self.IPv6).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.IPv4 == other.IPv4 && self.IPv6 == other.IPv6
    }
}
impl ::core::cmp::Eq for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2 {}
impl ::core::default::Default for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2_0 {
    pub Encapsulation: u32,
    pub MaxOffLoadSize: u32,
    pub MinSegmentCount: u32,
}
impl ::core::marker::Copy for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2_0 {}
impl ::core::clone::Clone for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2_0").field("Encapsulation", &self.Encapsulation).field("MaxOffLoadSize", &self.MaxOffLoadSize).field("MinSegmentCount", &self.MinSegmentCount).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2_0 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2_0 {
    fn eq(&self, other: &Self) -> bool {
        self.Encapsulation == other.Encapsulation && self.MaxOffLoadSize == other.MaxOffLoadSize && self.MinSegmentCount == other.MinSegmentCount
    }
}
impl ::core::cmp::Eq for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2_0 {}
impl ::core::default::Default for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2_1 {
    pub Encapsulation: u32,
    pub MaxOffLoadSize: u32,
    pub MinSegmentCount: u32,
    pub IpExtensionHeadersSupported: u32,
    pub TcpOptionsSupported: u32,
}
impl ::core::marker::Copy for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2_1 {}
impl ::core::clone::Clone for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2_1").field("Encapsulation", &self.Encapsulation).field("MaxOffLoadSize", &self.MaxOffLoadSize).field("MinSegmentCount", &self.MinSegmentCount).field("IpExtensionHeadersSupported", &self.IpExtensionHeadersSupported).field("TcpOptionsSupported", &self.TcpOptionsSupported).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2_1 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2_1 {
    fn eq(&self, other: &Self) -> bool {
        self.Encapsulation == other.Encapsulation && self.MaxOffLoadSize == other.MaxOffLoadSize && self.MinSegmentCount == other.MinSegmentCount && self.IpExtensionHeadersSupported == other.IpExtensionHeadersSupported && self.TcpOptionsSupported == other.TcpOptionsSupported
    }
}
impl ::core::cmp::Eq for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2_1 {}
impl ::core::default::Default for NDIS_WMI_TCP_LARGE_SEND_OFFLOAD_V2_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NDIS_WORK_ITEM {
    pub Context: *mut ::core::ffi::c_void,
    pub Routine: NDIS_PROC,
    pub WrapperReserved: [u8; 32],
}
impl ::core::marker::Copy for NDIS_WORK_ITEM {}
impl ::core::clone::Clone for NDIS_WORK_ITEM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDIS_WORK_ITEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDIS_WORK_ITEM").field("Context", &self.Context).field("WrapperReserved", &self.WrapperReserved).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_WORK_ITEM {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::default::Default for NDIS_WORK_ITEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NDIS_WRAPPER_HANDLE(pub isize);
impl ::core::default::Default for NDIS_WRAPPER_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for NDIS_WRAPPER_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for NDIS_WRAPPER_HANDLE {}
impl ::core::fmt::Debug for NDIS_WRAPPER_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NDIS_WRAPPER_HANDLE").field(&self.0).finish()
    }
}
impl ::windows_core::TypeKind for NDIS_WRAPPER_HANDLE {
    type TypeKind = ::windows_core::CopyType;
}
#[repr(C)]
pub struct NETWORK_ADDRESS {
    pub AddressLength: u16,
    pub AddressType: u16,
    pub Address: [u8; 1],
}
impl ::core::marker::Copy for NETWORK_ADDRESS {}
impl ::core::clone::Clone for NETWORK_ADDRESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NETWORK_ADDRESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NETWORK_ADDRESS").field("AddressLength", &self.AddressLength).field("AddressType", &self.AddressType).field("Address", &self.Address).finish()
    }
}
impl ::windows_core::TypeKind for NETWORK_ADDRESS {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NETWORK_ADDRESS {
    fn eq(&self, other: &Self) -> bool {
        self.AddressLength == other.AddressLength && self.AddressType == other.AddressType && self.Address == other.Address
    }
}
impl ::core::cmp::Eq for NETWORK_ADDRESS {}
impl ::core::default::Default for NETWORK_ADDRESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NETWORK_ADDRESS_IP {
    pub sin_port: u16,
    pub IN_ADDR: u32,
    pub sin_zero: [u8; 8],
}
impl ::core::marker::Copy for NETWORK_ADDRESS_IP {}
impl ::core::clone::Clone for NETWORK_ADDRESS_IP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NETWORK_ADDRESS_IP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NETWORK_ADDRESS_IP").field("sin_port", &self.sin_port).field("IN_ADDR", &self.IN_ADDR).field("sin_zero", &self.sin_zero).finish()
    }
}
impl ::windows_core::TypeKind for NETWORK_ADDRESS_IP {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NETWORK_ADDRESS_IP {
    fn eq(&self, other: &Self) -> bool {
        self.sin_port == other.sin_port && self.IN_ADDR == other.IN_ADDR && self.sin_zero == other.sin_zero
    }
}
impl ::core::cmp::Eq for NETWORK_ADDRESS_IP {}
impl ::core::default::Default for NETWORK_ADDRESS_IP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NETWORK_ADDRESS_IP6 {
    pub sin6_port: u16,
    pub sin6_flowinfo: u32,
    pub sin6_addr: [u16; 8],
    pub sin6_scope_id: u32,
}
impl ::core::marker::Copy for NETWORK_ADDRESS_IP6 {}
impl ::core::clone::Clone for NETWORK_ADDRESS_IP6 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NETWORK_ADDRESS_IP6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NETWORK_ADDRESS_IP6").field("sin6_port", &self.sin6_port).field("sin6_flowinfo", &self.sin6_flowinfo).field("sin6_addr", &self.sin6_addr).field("sin6_scope_id", &self.sin6_scope_id).finish()
    }
}
impl ::windows_core::TypeKind for NETWORK_ADDRESS_IP6 {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NETWORK_ADDRESS_IP6 {
    fn eq(&self, other: &Self) -> bool {
        self.sin6_port == other.sin6_port && self.sin6_flowinfo == other.sin6_flowinfo && self.sin6_addr == other.sin6_addr && self.sin6_scope_id == other.sin6_scope_id
    }
}
impl ::core::cmp::Eq for NETWORK_ADDRESS_IP6 {}
impl ::core::default::Default for NETWORK_ADDRESS_IP6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NETWORK_ADDRESS_IPX {
    pub NetworkAddress: u32,
    pub NodeAddress: [u8; 6],
    pub Socket: u16,
}
impl ::core::marker::Copy for NETWORK_ADDRESS_IPX {}
impl ::core::clone::Clone for NETWORK_ADDRESS_IPX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NETWORK_ADDRESS_IPX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NETWORK_ADDRESS_IPX").field("NetworkAddress", &self.NetworkAddress).field("NodeAddress", &self.NodeAddress).field("Socket", &self.Socket).finish()
    }
}
impl ::windows_core::TypeKind for NETWORK_ADDRESS_IPX {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NETWORK_ADDRESS_IPX {
    fn eq(&self, other: &Self) -> bool {
        self.NetworkAddress == other.NetworkAddress && self.NodeAddress == other.NodeAddress && self.Socket == other.Socket
    }
}
impl ::core::cmp::Eq for NETWORK_ADDRESS_IPX {}
impl ::core::default::Default for NETWORK_ADDRESS_IPX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct NETWORK_ADDRESS_LIST {
    pub AddressCount: i32,
    pub AddressType: u16,
    pub Address: [NETWORK_ADDRESS; 1],
}
impl ::core::marker::Copy for NETWORK_ADDRESS_LIST {}
impl ::core::clone::Clone for NETWORK_ADDRESS_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NETWORK_ADDRESS_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NETWORK_ADDRESS_LIST").field("AddressCount", &self.AddressCount).field("AddressType", &self.AddressType).field("Address", &self.Address).finish()
    }
}
impl ::windows_core::TypeKind for NETWORK_ADDRESS_LIST {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for NETWORK_ADDRESS_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.AddressCount == other.AddressCount && self.AddressType == other.AddressType && self.Address == other.Address
    }
}
impl ::core::cmp::Eq for NETWORK_ADDRESS_LIST {}
impl ::core::default::Default for NETWORK_ADDRESS_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct NULL_FILTER(pub isize);
impl ::core::default::Default for NULL_FILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for NULL_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for NULL_FILTER {}
impl ::core::fmt::Debug for NULL_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("NULL_FILTER").field(&self.0).finish()
    }
}
impl ::windows_core::TypeKind for NULL_FILTER {
    type TypeKind = ::windows_core::CopyType;
}
#[repr(C)]
pub struct OFFLOAD_ALGO_INFO {
    pub algoIdentifier: u32,
    pub algoKeylen: u32,
    pub algoRounds: u32,
}
impl ::core::marker::Copy for OFFLOAD_ALGO_INFO {}
impl ::core::clone::Clone for OFFLOAD_ALGO_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OFFLOAD_ALGO_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OFFLOAD_ALGO_INFO").field("algoIdentifier", &self.algoIdentifier).field("algoKeylen", &self.algoKeylen).field("algoRounds", &self.algoRounds).finish()
    }
}
impl ::windows_core::TypeKind for OFFLOAD_ALGO_INFO {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for OFFLOAD_ALGO_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.algoIdentifier == other.algoIdentifier && self.algoKeylen == other.algoKeylen && self.algoRounds == other.algoRounds
    }
}
impl ::core::cmp::Eq for OFFLOAD_ALGO_INFO {}
impl ::core::default::Default for OFFLOAD_ALGO_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct OFFLOAD_IPSEC_ADD_SA {
    pub SrcAddr: u32,
    pub SrcMask: u32,
    pub DestAddr: u32,
    pub DestMask: u32,
    pub Protocol: u32,
    pub SrcPort: u16,
    pub DestPort: u16,
    pub SrcTunnelAddr: u32,
    pub DestTunnelAddr: u32,
    pub Flags: u16,
    pub NumSAs: i16,
    pub SecAssoc: [OFFLOAD_SECURITY_ASSOCIATION; 3],
    pub OffloadHandle: super::super::super::Win32::Foundation::HANDLE,
    pub KeyLen: u32,
    pub KeyMat: [u8; 1],
}
impl ::core::marker::Copy for OFFLOAD_IPSEC_ADD_SA {}
impl ::core::clone::Clone for OFFLOAD_IPSEC_ADD_SA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OFFLOAD_IPSEC_ADD_SA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OFFLOAD_IPSEC_ADD_SA")
            .field("SrcAddr", &self.SrcAddr)
            .field("SrcMask", &self.SrcMask)
            .field("DestAddr", &self.DestAddr)
            .field("DestMask", &self.DestMask)
            .field("Protocol", &self.Protocol)
            .field("SrcPort", &self.SrcPort)
            .field("DestPort", &self.DestPort)
            .field("SrcTunnelAddr", &self.SrcTunnelAddr)
            .field("DestTunnelAddr", &self.DestTunnelAddr)
            .field("Flags", &self.Flags)
            .field("NumSAs", &self.NumSAs)
            .field("SecAssoc", &self.SecAssoc)
            .field("OffloadHandle", &self.OffloadHandle)
            .field("KeyLen", &self.KeyLen)
            .field("KeyMat", &self.KeyMat)
            .finish()
    }
}
impl ::windows_core::TypeKind for OFFLOAD_IPSEC_ADD_SA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for OFFLOAD_IPSEC_ADD_SA {
    fn eq(&self, other: &Self) -> bool {
        self.SrcAddr == other.SrcAddr && self.SrcMask == other.SrcMask && self.DestAddr == other.DestAddr && self.DestMask == other.DestMask && self.Protocol == other.Protocol && self.SrcPort == other.SrcPort && self.DestPort == other.DestPort && self.SrcTunnelAddr == other.SrcTunnelAddr && self.DestTunnelAddr == other.DestTunnelAddr && self.Flags == other.Flags && self.NumSAs == other.NumSAs && self.SecAssoc == other.SecAssoc && self.OffloadHandle == other.OffloadHandle && self.KeyLen == other.KeyLen && self.KeyMat == other.KeyMat
    }
}
impl ::core::cmp::Eq for OFFLOAD_IPSEC_ADD_SA {}
impl ::core::default::Default for OFFLOAD_IPSEC_ADD_SA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct OFFLOAD_IPSEC_ADD_UDPESP_SA {
    pub SrcAddr: u32,
    pub SrcMask: u32,
    pub DstAddr: u32,
    pub DstMask: u32,
    pub Protocol: u32,
    pub SrcPort: u16,
    pub DstPort: u16,
    pub SrcTunnelAddr: u32,
    pub DstTunnelAddr: u32,
    pub Flags: u16,
    pub NumSAs: i16,
    pub SecAssoc: [OFFLOAD_SECURITY_ASSOCIATION; 3],
    pub OffloadHandle: super::super::super::Win32::Foundation::HANDLE,
    pub EncapTypeEntry: OFFLOAD_IPSEC_UDPESP_ENCAPTYPE_ENTRY,
    pub EncapTypeEntryOffldHandle: super::super::super::Win32::Foundation::HANDLE,
    pub KeyLen: u32,
    pub KeyMat: [u8; 1],
}
impl ::core::marker::Copy for OFFLOAD_IPSEC_ADD_UDPESP_SA {}
impl ::core::clone::Clone for OFFLOAD_IPSEC_ADD_UDPESP_SA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OFFLOAD_IPSEC_ADD_UDPESP_SA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OFFLOAD_IPSEC_ADD_UDPESP_SA")
            .field("SrcAddr", &self.SrcAddr)
            .field("SrcMask", &self.SrcMask)
            .field("DstAddr", &self.DstAddr)
            .field("DstMask", &self.DstMask)
            .field("Protocol", &self.Protocol)
            .field("SrcPort", &self.SrcPort)
            .field("DstPort", &self.DstPort)
            .field("SrcTunnelAddr", &self.SrcTunnelAddr)
            .field("DstTunnelAddr", &self.DstTunnelAddr)
            .field("Flags", &self.Flags)
            .field("NumSAs", &self.NumSAs)
            .field("SecAssoc", &self.SecAssoc)
            .field("OffloadHandle", &self.OffloadHandle)
            .field("EncapTypeEntry", &self.EncapTypeEntry)
            .field("EncapTypeEntryOffldHandle", &self.EncapTypeEntryOffldHandle)
            .field("KeyLen", &self.KeyLen)
            .field("KeyMat", &self.KeyMat)
            .finish()
    }
}
impl ::windows_core::TypeKind for OFFLOAD_IPSEC_ADD_UDPESP_SA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for OFFLOAD_IPSEC_ADD_UDPESP_SA {
    fn eq(&self, other: &Self) -> bool {
        self.SrcAddr == other.SrcAddr && self.SrcMask == other.SrcMask && self.DstAddr == other.DstAddr && self.DstMask == other.DstMask && self.Protocol == other.Protocol && self.SrcPort == other.SrcPort && self.DstPort == other.DstPort && self.SrcTunnelAddr == other.SrcTunnelAddr && self.DstTunnelAddr == other.DstTunnelAddr && self.Flags == other.Flags && self.NumSAs == other.NumSAs && self.SecAssoc == other.SecAssoc && self.OffloadHandle == other.OffloadHandle && self.EncapTypeEntry == other.EncapTypeEntry && self.EncapTypeEntryOffldHandle == other.EncapTypeEntryOffldHandle && self.KeyLen == other.KeyLen && self.KeyMat == other.KeyMat
    }
}
impl ::core::cmp::Eq for OFFLOAD_IPSEC_ADD_UDPESP_SA {}
impl ::core::default::Default for OFFLOAD_IPSEC_ADD_UDPESP_SA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct OFFLOAD_IPSEC_DELETE_SA {
    pub OffloadHandle: super::super::super::Win32::Foundation::HANDLE,
}
impl ::core::marker::Copy for OFFLOAD_IPSEC_DELETE_SA {}
impl ::core::clone::Clone for OFFLOAD_IPSEC_DELETE_SA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OFFLOAD_IPSEC_DELETE_SA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OFFLOAD_IPSEC_DELETE_SA").field("OffloadHandle", &self.OffloadHandle).finish()
    }
}
impl ::windows_core::TypeKind for OFFLOAD_IPSEC_DELETE_SA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for OFFLOAD_IPSEC_DELETE_SA {
    fn eq(&self, other: &Self) -> bool {
        self.OffloadHandle == other.OffloadHandle
    }
}
impl ::core::cmp::Eq for OFFLOAD_IPSEC_DELETE_SA {}
impl ::core::default::Default for OFFLOAD_IPSEC_DELETE_SA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct OFFLOAD_IPSEC_DELETE_UDPESP_SA {
    pub OffloadHandle: super::super::super::Win32::Foundation::HANDLE,
    pub EncapTypeEntryOffldHandle: super::super::super::Win32::Foundation::HANDLE,
}
impl ::core::marker::Copy for OFFLOAD_IPSEC_DELETE_UDPESP_SA {}
impl ::core::clone::Clone for OFFLOAD_IPSEC_DELETE_UDPESP_SA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OFFLOAD_IPSEC_DELETE_UDPESP_SA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OFFLOAD_IPSEC_DELETE_UDPESP_SA").field("OffloadHandle", &self.OffloadHandle).field("EncapTypeEntryOffldHandle", &self.EncapTypeEntryOffldHandle).finish()
    }
}
impl ::windows_core::TypeKind for OFFLOAD_IPSEC_DELETE_UDPESP_SA {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for OFFLOAD_IPSEC_DELETE_UDPESP_SA {
    fn eq(&self, other: &Self) -> bool {
        self.OffloadHandle == other.OffloadHandle && self.EncapTypeEntryOffldHandle == other.EncapTypeEntryOffldHandle
    }
}
impl ::core::cmp::Eq for OFFLOAD_IPSEC_DELETE_UDPESP_SA {}
impl ::core::default::Default for OFFLOAD_IPSEC_DELETE_UDPESP_SA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct OFFLOAD_IPSEC_UDPESP_ENCAPTYPE_ENTRY {
    pub UdpEncapType: UDP_ENCAP_TYPE,
    pub DstEncapPort: u16,
}
impl ::core::marker::Copy for OFFLOAD_IPSEC_UDPESP_ENCAPTYPE_ENTRY {}
impl ::core::clone::Clone for OFFLOAD_IPSEC_UDPESP_ENCAPTYPE_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OFFLOAD_IPSEC_UDPESP_ENCAPTYPE_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OFFLOAD_IPSEC_UDPESP_ENCAPTYPE_ENTRY").field("UdpEncapType", &self.UdpEncapType).field("DstEncapPort", &self.DstEncapPort).finish()
    }
}
impl ::windows_core::TypeKind for OFFLOAD_IPSEC_UDPESP_ENCAPTYPE_ENTRY {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for OFFLOAD_IPSEC_UDPESP_ENCAPTYPE_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.UdpEncapType == other.UdpEncapType && self.DstEncapPort == other.DstEncapPort
    }
}
impl ::core::cmp::Eq for OFFLOAD_IPSEC_UDPESP_ENCAPTYPE_ENTRY {}
impl ::core::default::Default for OFFLOAD_IPSEC_UDPESP_ENCAPTYPE_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct OFFLOAD_SECURITY_ASSOCIATION {
    pub Operation: OFFLOAD_OPERATION_E,
    pub SPI: u32,
    pub IntegrityAlgo: OFFLOAD_ALGO_INFO,
    pub ConfAlgo: OFFLOAD_ALGO_INFO,
    pub Reserved: OFFLOAD_ALGO_INFO,
}
impl ::core::marker::Copy for OFFLOAD_SECURITY_ASSOCIATION {}
impl ::core::clone::Clone for OFFLOAD_SECURITY_ASSOCIATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for OFFLOAD_SECURITY_ASSOCIATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("OFFLOAD_SECURITY_ASSOCIATION").field("Operation", &self.Operation).field("SPI", &self.SPI).field("IntegrityAlgo", &self.IntegrityAlgo).field("ConfAlgo", &self.ConfAlgo).field("Reserved", &self.Reserved).finish()
    }
}
impl ::windows_core::TypeKind for OFFLOAD_SECURITY_ASSOCIATION {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for OFFLOAD_SECURITY_ASSOCIATION {
    fn eq(&self, other: &Self) -> bool {
        self.Operation == other.Operation && self.SPI == other.SPI && self.IntegrityAlgo == other.IntegrityAlgo && self.ConfAlgo == other.ConfAlgo && self.Reserved == other.Reserved
    }
}
impl ::core::cmp::Eq for OFFLOAD_SECURITY_ASSOCIATION {}
impl ::core::default::Default for OFFLOAD_SECURITY_ASSOCIATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct PMKID_CANDIDATE {
    pub BSSID: [u8; 6],
    pub Flags: u32,
}
impl ::core::marker::Copy for PMKID_CANDIDATE {}
impl ::core::clone::Clone for PMKID_CANDIDATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for PMKID_CANDIDATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("PMKID_CANDIDATE").field("BSSID", &self.BSSID).field("Flags", &self.Flags).finish()
    }
}
impl ::windows_core::TypeKind for PMKID_CANDIDATE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for PMKID_CANDIDATE {
    fn eq(&self, other: &Self) -> bool {
        self.BSSID == other.BSSID && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for PMKID_CANDIDATE {}
impl ::core::default::Default for PMKID_CANDIDATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct REFERENCE {
    pub SpinLock: usize,
    pub ReferenceCount: u16,
    pub Closing: super::super::super::Win32::Foundation::BOOLEAN,
}
impl ::core::marker::Copy for REFERENCE {}
impl ::core::clone::Clone for REFERENCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for REFERENCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("REFERENCE").field("SpinLock", &self.SpinLock).field("ReferenceCount", &self.ReferenceCount).field("Closing", &self.Closing).finish()
    }
}
impl ::windows_core::TypeKind for REFERENCE {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for REFERENCE {
    fn eq(&self, other: &Self) -> bool {
        self.SpinLock == other.SpinLock && self.ReferenceCount == other.ReferenceCount && self.Closing == other.Closing
    }
}
impl ::core::cmp::Eq for REFERENCE {}
impl ::core::default::Default for REFERENCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct TRANSPORT_HEADER_OFFSET {
    pub ProtocolType: u16,
    pub HeaderOffset: u16,
}
impl ::core::marker::Copy for TRANSPORT_HEADER_OFFSET {}
impl ::core::clone::Clone for TRANSPORT_HEADER_OFFSET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for TRANSPORT_HEADER_OFFSET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("TRANSPORT_HEADER_OFFSET").field("ProtocolType", &self.ProtocolType).field("HeaderOffset", &self.HeaderOffset).finish()
    }
}
impl ::windows_core::TypeKind for TRANSPORT_HEADER_OFFSET {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for TRANSPORT_HEADER_OFFSET {
    fn eq(&self, other: &Self) -> bool {
        self.ProtocolType == other.ProtocolType && self.HeaderOffset == other.HeaderOffset
    }
}
impl ::core::cmp::Eq for TRANSPORT_HEADER_OFFSET {}
impl ::core::default::Default for TRANSPORT_HEADER_OFFSET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct TR_FILTER(pub isize);
impl ::core::default::Default for TR_FILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for TR_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for TR_FILTER {}
impl ::core::fmt::Debug for TR_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("TR_FILTER").field(&self.0).finish()
    }
}
impl ::windows_core::TypeKind for TR_FILTER {
    type TypeKind = ::windows_core::CopyType;
}
#[repr(C)]
pub struct VAR_STRING {
    pub ulTotalSize: u32,
    pub ulNeededSize: u32,
    pub ulUsedSize: u32,
    pub ulStringFormat: u32,
    pub ulStringSize: u32,
    pub ulStringOffset: u32,
}
impl ::core::marker::Copy for VAR_STRING {}
impl ::core::clone::Clone for VAR_STRING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for VAR_STRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("VAR_STRING").field("ulTotalSize", &self.ulTotalSize).field("ulNeededSize", &self.ulNeededSize).field("ulUsedSize", &self.ulUsedSize).field("ulStringFormat", &self.ulStringFormat).field("ulStringSize", &self.ulStringSize).field("ulStringOffset", &self.ulStringOffset).finish()
    }
}
impl ::windows_core::TypeKind for VAR_STRING {
    type TypeKind = ::windows_core::CopyType;
}
impl ::core::cmp::PartialEq for VAR_STRING {
    fn eq(&self, other: &Self) -> bool {
        self.ulTotalSize == other.ulTotalSize && self.ulNeededSize == other.ulNeededSize && self.ulUsedSize == other.ulUsedSize && self.ulStringFormat == other.ulStringFormat && self.ulStringSize == other.ulStringSize && self.ulStringOffset == other.ulStringOffset
    }
}
impl ::core::cmp::Eq for VAR_STRING {}
impl ::core::default::Default for VAR_STRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub type CL_ADD_PARTY_COMPLETE_HANDLER = ::core::option::Option<unsafe extern "system" fn()>;
pub type CL_CALL_CONNECTED_HANDLER = ::core::option::Option<unsafe extern "system" fn()>;
pub type CL_CLOSE_AF_COMPLETE_HANDLER = ::core::option::Option<unsafe extern "system" fn()>;
pub type CL_CLOSE_CALL_COMPLETE_HANDLER = ::core::option::Option<unsafe extern "system" fn()>;
pub type CL_DEREG_SAP_COMPLETE_HANDLER = ::core::option::Option<unsafe extern "system" fn()>;
pub type CL_DROP_PARTY_COMPLETE_HANDLER = ::core::option::Option<unsafe extern "system" fn()>;
pub type CL_INCOMING_CALL_HANDLER = ::core::option::Option<unsafe extern "system" fn() -> i32>;
pub type CL_INCOMING_CALL_QOS_CHANGE_HANDLER = ::core::option::Option<unsafe extern "system" fn()>;
pub type CL_INCOMING_CLOSE_CALL_HANDLER = ::core::option::Option<unsafe extern "system" fn()>;
pub type CL_INCOMING_DROP_PARTY_HANDLER = ::core::option::Option<unsafe extern "system" fn()>;
pub type CL_MAKE_CALL_COMPLETE_HANDLER = ::core::option::Option<unsafe extern "system" fn()>;
pub type CL_MODIFY_CALL_QOS_COMPLETE_HANDLER = ::core::option::Option<unsafe extern "system" fn()>;
pub type CL_OPEN_AF_COMPLETE_HANDLER = ::core::option::Option<unsafe extern "system" fn()>;
pub type CL_REG_SAP_COMPLETE_HANDLER = ::core::option::Option<unsafe extern "system" fn()>;
pub type CM_ACTIVATE_VC_COMPLETE_HANDLER = ::core::option::Option<unsafe extern "system" fn()>;
pub type CM_ADD_PARTY_HANDLER = ::core::option::Option<unsafe extern "system" fn() -> i32>;
pub type CM_CLOSE_AF_HANDLER = ::core::option::Option<unsafe extern "system" fn() -> i32>;
pub type CM_CLOSE_CALL_HANDLER = ::core::option::Option<unsafe extern "system" fn() -> i32>;
pub type CM_DEACTIVATE_VC_COMPLETE_HANDLER = ::core::option::Option<unsafe extern "system" fn()>;
pub type CM_DEREG_SAP_HANDLER = ::core::option::Option<unsafe extern "system" fn() -> i32>;
pub type CM_DROP_PARTY_HANDLER = ::core::option::Option<unsafe extern "system" fn() -> i32>;
pub type CM_INCOMING_CALL_COMPLETE_HANDLER = ::core::option::Option<unsafe extern "system" fn()>;
pub type CM_MAKE_CALL_HANDLER = ::core::option::Option<unsafe extern "system" fn() -> i32>;
pub type CM_MODIFY_CALL_QOS_HANDLER = ::core::option::Option<unsafe extern "system" fn() -> i32>;
pub type CM_OPEN_AF_HANDLER = ::core::option::Option<unsafe extern "system" fn() -> i32>;
pub type CM_REG_SAP_HANDLER = ::core::option::Option<unsafe extern "system" fn() -> i32>;
pub type CO_AF_REGISTER_NOTIFY_HANDLER = ::core::option::Option<unsafe extern "system" fn()>;
pub type CO_CREATE_VC_HANDLER = ::core::option::Option<unsafe extern "system" fn() -> i32>;
pub type CO_DELETE_VC_HANDLER = ::core::option::Option<unsafe extern "system" fn() -> i32>;
pub type MINIPORT_CO_ACTIVATE_VC = ::core::option::Option<unsafe extern "system" fn(miniportvccontext: *const ::core::ffi::c_void, callparameters: *mut CO_CALL_PARAMETERS) -> i32>;
pub type MINIPORT_CO_CREATE_VC = ::core::option::Option<unsafe extern "system" fn(miniportadaptercontext: *const ::core::ffi::c_void, ndisvchandle: *const ::core::ffi::c_void, miniportvccontext: *mut *mut ::core::ffi::c_void) -> i32>;
pub type MINIPORT_CO_DEACTIVATE_VC = ::core::option::Option<unsafe extern "system" fn(miniportvccontext: *const ::core::ffi::c_void) -> i32>;
pub type MINIPORT_CO_DELETE_VC = ::core::option::Option<unsafe extern "system" fn(miniportvccontext: *const ::core::ffi::c_void) -> i32>;
pub type NDIS_PROC = ::core::option::Option<unsafe extern "system" fn()>;
pub type NDIS_PROC_CALLBACK = ::core::option::Option<unsafe extern "system" fn(workitem: *const NDIS_WORK_ITEM, context: *const ::core::ffi::c_void)>;
pub type NDIS_TIMER_FUNCTION = ::core::option::Option<unsafe extern "system" fn(systemspecific1: *const ::core::ffi::c_void, functioncontext: *const ::core::ffi::c_void, systemspecific2: *const ::core::ffi::c_void, systemspecific3: *const ::core::ffi::c_void)>;
pub type PNDIS_TIMER_FUNCTION = ::core::option::Option<unsafe extern "system" fn()>;
pub type PROTCOL_CO_AF_REGISTER_NOTIFY = ::core::option::Option<unsafe extern "system" fn()>;
pub type PROTOCOL_CL_ADD_PARTY_COMPLETE = ::core::option::Option<unsafe extern "system" fn(status: i32, protocolpartycontext: *const ::core::ffi::c_void, ndispartyhandle: *const ::core::ffi::c_void, callparameters: *const CO_CALL_PARAMETERS)>;
pub type PROTOCOL_CL_CALL_CONNECTED = ::core::option::Option<unsafe extern "system" fn(protocolvccontext: *const ::core::ffi::c_void)>;
pub type PROTOCOL_CL_CLOSE_AF_COMPLETE = ::core::option::Option<unsafe extern "system" fn(status: i32, protocolafcontext: *const ::core::ffi::c_void)>;
pub type PROTOCOL_CL_CLOSE_CALL_COMPLETE = ::core::option::Option<unsafe extern "system" fn(status: i32, protocolvccontext: *const ::core::ffi::c_void, protocolpartycontext: *const ::core::ffi::c_void)>;
pub type PROTOCOL_CL_DEREGISTER_SAP_COMPLETE = ::core::option::Option<unsafe extern "system" fn(status: i32, protocolsapcontext: *const ::core::ffi::c_void)>;
pub type PROTOCOL_CL_DROP_PARTY_COMPLETE = ::core::option::Option<unsafe extern "system" fn(status: i32, protocolpartycontext: *const ::core::ffi::c_void)>;
pub type PROTOCOL_CL_INCOMING_CALL = ::core::option::Option<unsafe extern "system" fn(protocolsapcontext: *const ::core::ffi::c_void, protocolvccontext: *const ::core::ffi::c_void, callparameters: *mut CO_CALL_PARAMETERS) -> i32>;
pub type PROTOCOL_CL_INCOMING_CALL_QOS_CHANGE = ::core::option::Option<unsafe extern "system" fn(protocolvccontext: *const ::core::ffi::c_void, callparameters: *const CO_CALL_PARAMETERS)>;
pub type PROTOCOL_CL_INCOMING_CLOSE_CALL = ::core::option::Option<unsafe extern "system" fn(closestatus: i32, protocolvccontext: *const ::core::ffi::c_void, closedata: *const ::core::ffi::c_void, size: u32)>;
pub type PROTOCOL_CL_INCOMING_DROP_PARTY = ::core::option::Option<unsafe extern "system" fn(dropstatus: i32, protocolpartycontext: *const ::core::ffi::c_void, closedata: *const ::core::ffi::c_void, size: u32)>;
pub type PROTOCOL_CL_MAKE_CALL_COMPLETE = ::core::option::Option<unsafe extern "system" fn(status: i32, protocolvccontext: *const ::core::ffi::c_void, ndispartyhandle: *const ::core::ffi::c_void, callparameters: *const CO_CALL_PARAMETERS)>;
pub type PROTOCOL_CL_MODIFY_CALL_QOS_COMPLETE = ::core::option::Option<unsafe extern "system" fn(status: i32, protocolvccontext: *const ::core::ffi::c_void, callparameters: *const CO_CALL_PARAMETERS)>;
pub type PROTOCOL_CL_OPEN_AF_COMPLETE = ::core::option::Option<unsafe extern "system" fn(status: i32, protocolafcontext: *const ::core::ffi::c_void, ndisafhandle: *const ::core::ffi::c_void)>;
pub type PROTOCOL_CL_REGISTER_SAP_COMPLETE = ::core::option::Option<unsafe extern "system" fn(status: i32, protocolsapcontext: *const ::core::ffi::c_void, sap: *const CO_SAP, ndissaphandle: *const ::core::ffi::c_void)>;
pub type PROTOCOL_CM_ACTIVATE_VC_COMPLETE = ::core::option::Option<unsafe extern "system" fn(status: i32, callmgrvccontext: *const ::core::ffi::c_void, callparameters: *const CO_CALL_PARAMETERS)>;
pub type PROTOCOL_CM_ADD_PARTY = ::core::option::Option<unsafe extern "system" fn(callmgrvccontext: *const ::core::ffi::c_void, callparameters: *mut CO_CALL_PARAMETERS, ndispartyhandle: *const ::core::ffi::c_void, callmgrpartycontext: *mut *mut ::core::ffi::c_void) -> i32>;
pub type PROTOCOL_CM_CLOSE_AF = ::core::option::Option<unsafe extern "system" fn(callmgrafcontext: *const ::core::ffi::c_void) -> i32>;
pub type PROTOCOL_CM_CLOSE_CALL = ::core::option::Option<unsafe extern "system" fn(callmgrvccontext: *const ::core::ffi::c_void, callmgrpartycontext: *const ::core::ffi::c_void, closedata: *const ::core::ffi::c_void, size: u32) -> i32>;
pub type PROTOCOL_CM_DEACTIVATE_VC_COMPLETE = ::core::option::Option<unsafe extern "system" fn(status: i32, callmgrvccontext: *const ::core::ffi::c_void)>;
pub type PROTOCOL_CM_DEREGISTER_SAP = ::core::option::Option<unsafe extern "system" fn(callmgrsapcontext: *const ::core::ffi::c_void) -> i32>;
pub type PROTOCOL_CM_DROP_PARTY = ::core::option::Option<unsafe extern "system" fn(callmgrpartycontext: *const ::core::ffi::c_void, closedata: *const ::core::ffi::c_void, size: u32) -> i32>;
pub type PROTOCOL_CM_INCOMING_CALL_COMPLETE = ::core::option::Option<unsafe extern "system" fn(status: i32, callmgrvccontext: *const ::core::ffi::c_void, callparameters: *const CO_CALL_PARAMETERS)>;
pub type PROTOCOL_CM_MAKE_CALL = ::core::option::Option<unsafe extern "system" fn(callmgrvccontext: *const ::core::ffi::c_void, callparameters: *mut CO_CALL_PARAMETERS, ndispartyhandle: *const ::core::ffi::c_void, callmgrpartycontext: *mut *mut ::core::ffi::c_void) -> i32>;
pub type PROTOCOL_CM_MODIFY_QOS_CALL = ::core::option::Option<unsafe extern "system" fn(callmgrvccontext: *const ::core::ffi::c_void, callparameters: *const CO_CALL_PARAMETERS) -> i32>;
pub type PROTOCOL_CM_OPEN_AF = ::core::option::Option<unsafe extern "system" fn(callmgrbindingcontext: *const ::core::ffi::c_void, addressfamily: *const CO_ADDRESS_FAMILY, ndisafhandle: *const ::core::ffi::c_void, callmgrafcontext: *mut *mut ::core::ffi::c_void) -> i32>;
pub type PROTOCOL_CM_REG_SAP = ::core::option::Option<unsafe extern "system" fn(callmgrafcontext: *const ::core::ffi::c_void, sap: *const CO_SAP, ndissaphandle: *const ::core::ffi::c_void, callmgrsapcontext: *mut *mut ::core::ffi::c_void) -> i32>;
pub type PROTOCOL_CO_AF_REGISTER_NOTIFY = ::core::option::Option<unsafe extern "system" fn(protocolbindingcontext: *const ::core::ffi::c_void, addressfamily: *const CO_ADDRESS_FAMILY)>;
pub type PROTOCOL_CO_CREATE_VC = ::core::option::Option<unsafe extern "system" fn(protocolafcontext: *const ::core::ffi::c_void, ndisvchandle: *const ::core::ffi::c_void, protocolvccontext: *mut *mut ::core::ffi::c_void) -> i32>;
pub type PROTOCOL_CO_DELETE_VC = ::core::option::Option<unsafe extern "system" fn(protocolvccontext: *const ::core::ffi::c_void) -> i32>;
pub type TDI_PNP_HANDLER = ::core::option::Option<unsafe extern "system" fn(uppercomponent: *const super::super::super::Win32::Foundation::UNICODE_STRING, lowercomponent: *const super::super::super::Win32::Foundation::UNICODE_STRING, bindlist: *const super::super::super::Win32::Foundation::UNICODE_STRING, reconfigbuffer: *const ::core::ffi::c_void, reconfigbuffersize: u32, operation: u32) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type TDI_REGISTER_CALLBACK = ::core::option::Option<unsafe extern "system" fn(devicename: *const super::super::super::Win32::Foundation::UNICODE_STRING, tdihandle: *mut super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type W_CO_ACTIVATE_VC_HANDLER = ::core::option::Option<unsafe extern "system" fn() -> i32>;
pub type W_CO_CREATE_VC_HANDLER = ::core::option::Option<unsafe extern "system" fn() -> i32>;
pub type W_CO_DEACTIVATE_VC_HANDLER = ::core::option::Option<unsafe extern "system" fn() -> i32>;
pub type W_CO_DELETE_VC_HANDLER = ::core::option::Option<unsafe extern "system" fn() -> i32>;
