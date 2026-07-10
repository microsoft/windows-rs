#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn RtlAddFunctionTable(functiontable: &[RUNTIME_FUNCTION], baseaddress: u64) -> bool {
    windows_core::link!("kernel32.dll" "C" fn RtlAddFunctionTable(functiontable : *const RUNTIME_FUNCTION, entrycount : u32, baseaddress : u64) -> bool);
    unsafe { RtlAddFunctionTable(core::mem::transmute(functiontable.as_ptr()), functiontable.len().try_into().unwrap(), baseaddress) }
}
#[cfg(target_arch = "aarch64")]
#[inline]
pub unsafe fn RtlAddFunctionTable(functiontable: &[ARM64_RUNTIME_FUNCTION], baseaddress: usize) -> bool {
    windows_core::link!("kernel32.dll" "C" fn RtlAddFunctionTable(functiontable : *const ARM64_RUNTIME_FUNCTION, entrycount : u32, baseaddress : usize) -> bool);
    unsafe { RtlAddFunctionTable(core::mem::transmute(functiontable.as_ptr()), functiontable.len().try_into().unwrap(), baseaddress) }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn RtlAddGrowableFunctionTable(dynamictable: *mut *mut core::ffi::c_void, functiontable: &[RUNTIME_FUNCTION], entrycount: u32, rangebase: usize, rangeend: usize) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlAddGrowableFunctionTable(dynamictable : *mut *mut core::ffi::c_void, functiontable : *const RUNTIME_FUNCTION, entrycount : u32, maximumentrycount : u32, rangebase : usize, rangeend : usize) -> u32);
    unsafe { RtlAddGrowableFunctionTable(dynamictable as _, core::mem::transmute(functiontable.as_ptr()), entrycount, functiontable.len().try_into().unwrap(), rangebase, rangeend) }
}
#[cfg(target_arch = "aarch64")]
#[inline]
pub unsafe fn RtlAddGrowableFunctionTable(dynamictable: *mut *mut core::ffi::c_void, functiontable: &[ARM64_RUNTIME_FUNCTION], entrycount: u32, rangebase: usize, rangeend: usize) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlAddGrowableFunctionTable(dynamictable : *mut *mut core::ffi::c_void, functiontable : *const ARM64_RUNTIME_FUNCTION, entrycount : u32, maximumentrycount : u32, rangebase : usize, rangeend : usize) -> u32);
    unsafe { RtlAddGrowableFunctionTable(dynamictable as _, core::mem::transmute(functiontable.as_ptr()), entrycount, functiontable.len().try_into().unwrap(), rangebase, rangeend) }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[inline]
pub unsafe fn RtlCaptureContext(contextrecord: *mut CONTEXT) {
    windows_core::link!("kernel32.dll" "system" fn RtlCaptureContext(contextrecord : *mut CONTEXT));
    unsafe { RtlCaptureContext(contextrecord as _) }
}
#[cfg(target_arch = "aarch64")]
#[inline]
pub unsafe fn RtlCaptureContext(contextrecord: *mut ARM64_NT_CONTEXT) {
    windows_core::link!("kernel32.dll" "system" fn RtlCaptureContext(contextrecord : *mut ARM64_NT_CONTEXT));
    unsafe { RtlCaptureContext(contextrecord as _) }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[inline]
pub unsafe fn RtlCaptureContext2(contextrecord: *mut CONTEXT) {
    windows_core::link!("ntdll.dll" "system" fn RtlCaptureContext2(contextrecord : *mut CONTEXT));
    unsafe { RtlCaptureContext2(contextrecord as _) }
}
#[cfg(target_arch = "aarch64")]
#[inline]
pub unsafe fn RtlCaptureContext2(contextrecord: *mut ARM64_NT_CONTEXT) {
    windows_core::link!("ntdll.dll" "system" fn RtlCaptureContext2(contextrecord : *mut ARM64_NT_CONTEXT));
    unsafe { RtlCaptureContext2(contextrecord as _) }
}
#[inline]
pub unsafe fn RtlCaptureStackBackTrace(framestoskip: u32, backtrace: &mut [*mut core::ffi::c_void], backtracehash: Option<*mut u32>) -> u16 {
    windows_core::link!("kernel32.dll" "system" fn RtlCaptureStackBackTrace(framestoskip : u32, framestocapture : u32, backtrace : *mut *mut core::ffi::c_void, backtracehash : *mut u32) -> u16);
    unsafe { RtlCaptureStackBackTrace(framestoskip, backtrace.len().try_into().unwrap(), core::mem::transmute(backtrace.as_ptr()), backtracehash.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RtlCompareMemory(source1: *const core::ffi::c_void, source2: *const core::ffi::c_void, length: usize) -> usize {
    windows_core::link!("kernel32.dll" "system" fn RtlCompareMemory(source1 : *const core::ffi::c_void, source2 : *const core::ffi::c_void, length : usize) -> usize);
    unsafe { RtlCompareMemory(source1, source2, length) }
}
#[inline]
pub unsafe fn RtlConvertDeviceFamilyInfoToString(puldevicefamilybuffersize: *mut u32, puldeviceformbuffersize: *mut u32, devicefamily: windows_core::PWSTR, deviceform: windows_core::PWSTR) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlConvertDeviceFamilyInfoToString(puldevicefamilybuffersize : *mut u32, puldeviceformbuffersize : *mut u32, devicefamily : windows_core::PWSTR, deviceform : windows_core::PWSTR) -> u32);
    unsafe { RtlConvertDeviceFamilyInfoToString(puldevicefamilybuffersize as _, puldeviceformbuffersize as _, core::mem::transmute(devicefamily), core::mem::transmute(deviceform)) }
}
#[inline]
pub unsafe fn RtlCrc32(buffer: *const core::ffi::c_void, size: usize, initialcrc: u32) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlCrc32(buffer : *const core::ffi::c_void, size : usize, initialcrc : u32) -> u32);
    unsafe { RtlCrc32(buffer, size, initialcrc) }
}
#[inline]
pub unsafe fn RtlCrc64(buffer: *const core::ffi::c_void, size: usize, initialcrc: u64) -> u64 {
    windows_core::link!("ntdll.dll" "system" fn RtlCrc64(buffer : *const core::ffi::c_void, size : usize, initialcrc : u64) -> u64);
    unsafe { RtlCrc64(buffer, size, initialcrc) }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn RtlDeleteFunctionTable(functiontable: *const RUNTIME_FUNCTION) -> bool {
    windows_core::link!("kernel32.dll" "C" fn RtlDeleteFunctionTable(functiontable : *const RUNTIME_FUNCTION) -> bool);
    unsafe { RtlDeleteFunctionTable(functiontable) }
}
#[cfg(target_arch = "aarch64")]
#[inline]
pub unsafe fn RtlDeleteFunctionTable(functiontable: *const ARM64_RUNTIME_FUNCTION) -> bool {
    windows_core::link!("kernel32.dll" "C" fn RtlDeleteFunctionTable(functiontable : *const ARM64_RUNTIME_FUNCTION) -> bool);
    unsafe { RtlDeleteFunctionTable(functiontable) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn RtlDeleteGrowableFunctionTable(dynamictable: *const core::ffi::c_void) {
    windows_core::link!("ntdll.dll" "system" fn RtlDeleteGrowableFunctionTable(dynamictable : *const core::ffi::c_void));
    unsafe { RtlDeleteGrowableFunctionTable(dynamictable) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn RtlDrainNonVolatileFlush(nvtoken: *const core::ffi::c_void) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlDrainNonVolatileFlush(nvtoken : *const core::ffi::c_void) -> u32);
    unsafe { RtlDrainNonVolatileFlush(nvtoken) }
}
#[inline]
pub unsafe fn RtlExtendCorrelationVector(correlationvector: *mut CORRELATION_VECTOR) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlExtendCorrelationVector(correlationvector : *mut CORRELATION_VECTOR) -> u32);
    unsafe { RtlExtendCorrelationVector(correlationvector as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn RtlFillNonVolatileMemory(nvtoken: *const core::ffi::c_void, nvdestination: *mut core::ffi::c_void, size: usize, value: u8, flags: u32) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlFillNonVolatileMemory(nvtoken : *const core::ffi::c_void, nvdestination : *mut core::ffi::c_void, size : usize, value : u8, flags : u32) -> u32);
    unsafe { RtlFillNonVolatileMemory(nvtoken, nvdestination as _, size, value, flags) }
}
#[inline]
pub unsafe fn RtlFirstEntrySList(listhead: *const SLIST_HEADER) -> PSLIST_ENTRY {
    windows_core::link!("ntdll.dll" "system" fn RtlFirstEntrySList(listhead : *const SLIST_HEADER) -> PSLIST_ENTRY);
    unsafe { RtlFirstEntrySList(listhead) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn RtlFlushNonVolatileMemory(nvtoken: *const core::ffi::c_void, nvbuffer: *const core::ffi::c_void, size: usize, flags: u32) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlFlushNonVolatileMemory(nvtoken : *const core::ffi::c_void, nvbuffer : *const core::ffi::c_void, size : usize, flags : u32) -> u32);
    unsafe { RtlFlushNonVolatileMemory(nvtoken, nvbuffer, size, flags) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn RtlFlushNonVolatileMemoryRanges(nvtoken: *const core::ffi::c_void, nvranges: &[NV_MEMORY_RANGE], flags: u32) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlFlushNonVolatileMemoryRanges(nvtoken : *const core::ffi::c_void, nvranges : *const NV_MEMORY_RANGE, numranges : usize, flags : u32) -> u32);
    unsafe { RtlFlushNonVolatileMemoryRanges(nvtoken, core::mem::transmute(nvranges.as_ptr()), nvranges.len().try_into().unwrap(), flags) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn RtlFreeNonVolatileToken(nvtoken: *const core::ffi::c_void) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlFreeNonVolatileToken(nvtoken : *const core::ffi::c_void) -> u32);
    unsafe { RtlFreeNonVolatileToken(nvtoken) }
}
#[inline]
pub unsafe fn RtlGetDeviceFamilyInfoEnum(pulluapinfo: Option<*mut u64>, puldevicefamily: Option<*mut u32>, puldeviceform: Option<*mut u32>) {
    windows_core::link!("ntdll.dll" "system" fn RtlGetDeviceFamilyInfoEnum(pulluapinfo : *mut u64, puldevicefamily : *mut u32, puldeviceform : *mut u32));
    unsafe { RtlGetDeviceFamilyInfoEnum(pulluapinfo.unwrap_or(core::mem::zeroed()) as _, puldevicefamily.unwrap_or(core::mem::zeroed()) as _, puldeviceform.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RtlGetImageFileMachines<P0>(dosfilename: P0, machinetypeflags: *mut IMAGE_FILE_MACHINES) -> u32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdll.dll" "C" fn RtlGetImageFileMachines(dosfilename : windows_core::PCWSTR, machinetypeflags : *mut IMAGE_FILE_MACHINES) -> u32);
    unsafe { RtlGetImageFileMachines(dosfilename.param().abi(), machinetypeflags as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn RtlGetNonVolatileToken(nvbuffer: *const core::ffi::c_void, size: usize, nvtoken: *mut *mut core::ffi::c_void) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlGetNonVolatileToken(nvbuffer : *const core::ffi::c_void, size : usize, nvtoken : *mut *mut core::ffi::c_void) -> u32);
    unsafe { RtlGetNonVolatileToken(nvbuffer, size, nvtoken as _) }
}
#[inline]
pub unsafe fn RtlGetProductInfo(osmajorversion: u32, osminorversion: u32, spmajorversion: u32, spminorversion: u32, returnedproducttype: *mut u32) -> bool {
    windows_core::link!("ntdll.dll" "system" fn RtlGetProductInfo(osmajorversion : u32, osminorversion : u32, spmajorversion : u32, spminorversion : u32, returnedproducttype : *mut u32) -> bool);
    unsafe { RtlGetProductInfo(osmajorversion, osminorversion, spmajorversion, spminorversion, returnedproducttype as _) }
}
#[inline]
pub unsafe fn RtlGetReturnAddressHijackTarget() -> usize {
    windows_core::link!("ntdll.dll" "system" fn RtlGetReturnAddressHijackTarget() -> usize);
    unsafe { RtlGetReturnAddressHijackTarget() }
}
#[inline]
pub unsafe fn RtlGetSystemGlobalData(dataid: RTL_SYSTEM_GLOBAL_DATA_ID, buffer: *mut core::ffi::c_void, size: u32) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlGetSystemGlobalData(dataid : RTL_SYSTEM_GLOBAL_DATA_ID, buffer : *mut core::ffi::c_void, size : u32) -> u32);
    unsafe { RtlGetSystemGlobalData(dataid, buffer as _, size) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn RtlGrowFunctionTable(dynamictable: *mut core::ffi::c_void, newentrycount: u32) {
    windows_core::link!("ntdll.dll" "system" fn RtlGrowFunctionTable(dynamictable : *mut core::ffi::c_void, newentrycount : u32));
    unsafe { RtlGrowFunctionTable(dynamictable as _, newentrycount) }
}
#[inline]
pub unsafe fn RtlIncrementCorrelationVector(correlationvector: *mut CORRELATION_VECTOR) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlIncrementCorrelationVector(correlationvector : *mut CORRELATION_VECTOR) -> u32);
    unsafe { RtlIncrementCorrelationVector(correlationvector as _) }
}
#[inline]
pub unsafe fn RtlInitializeCorrelationVector(correlationvector: *mut CORRELATION_VECTOR, version: i32, guid: Option<*const windows_core::GUID>) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlInitializeCorrelationVector(correlationvector : *mut CORRELATION_VECTOR, version : i32, guid : *const windows_core::GUID) -> u32);
    unsafe { RtlInitializeCorrelationVector(correlationvector as _, version, guid.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RtlInitializeSListHead() -> SLIST_HEADER {
    windows_core::link!("ntdll.dll" "system" fn RtlInitializeSListHead(listhead : *mut SLIST_HEADER));
    unsafe {
        let mut result__ = core::mem::zeroed();
        RtlInitializeSListHead(&mut result__);
        result__
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn RtlInstallFunctionTableCallback<P5>(tableidentifier: u64, baseaddress: u64, length: u32, callback: PGET_RUNTIME_FUNCTION_CALLBACK, context: Option<*const core::ffi::c_void>, outofprocesscallbackdll: P5) -> bool
where
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "C" fn RtlInstallFunctionTableCallback(tableidentifier : u64, baseaddress : u64, length : u32, callback : PGET_RUNTIME_FUNCTION_CALLBACK, context : *const core::ffi::c_void, outofprocesscallbackdll : windows_core::PCWSTR) -> bool);
    unsafe { RtlInstallFunctionTableCallback(tableidentifier, baseaddress, length, callback, context.unwrap_or(core::mem::zeroed()) as _, outofprocesscallbackdll.param().abi()) }
}
#[cfg(target_arch = "aarch64")]
#[inline]
pub unsafe fn RtlInstallFunctionTableCallback<P5>(tableidentifier: usize, baseaddress: usize, length: u32, callback: PGET_RUNTIME_FUNCTION_CALLBACK, context: Option<*const core::ffi::c_void>, outofprocesscallbackdll: P5) -> bool
where
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "C" fn RtlInstallFunctionTableCallback(tableidentifier : usize, baseaddress : usize, length : u32, callback : PGET_RUNTIME_FUNCTION_CALLBACK, context : *const core::ffi::c_void, outofprocesscallbackdll : windows_core::PCWSTR) -> bool);
    unsafe { RtlInstallFunctionTableCallback(tableidentifier, baseaddress, length, callback, context.unwrap_or(core::mem::zeroed()) as _, outofprocesscallbackdll.param().abi()) }
}
#[inline]
pub unsafe fn RtlInterlockedFlushSList(listhead: *mut SLIST_HEADER) -> PSLIST_ENTRY {
    windows_core::link!("ntdll.dll" "system" fn RtlInterlockedFlushSList(listhead : *mut SLIST_HEADER) -> PSLIST_ENTRY);
    unsafe { RtlInterlockedFlushSList(listhead as _) }
}
#[inline]
pub unsafe fn RtlInterlockedPopEntrySList(listhead: *mut SLIST_HEADER) -> PSLIST_ENTRY {
    windows_core::link!("ntdll.dll" "system" fn RtlInterlockedPopEntrySList(listhead : *mut SLIST_HEADER) -> PSLIST_ENTRY);
    unsafe { RtlInterlockedPopEntrySList(listhead as _) }
}
#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn RtlInterlockedPushEntrySList(listhead: *mut SLIST_HEADER, listentry: *mut SINGLE_LIST_ENTRY) -> PSLIST_ENTRY {
    windows_core::link!("ntdll.dll" "system" fn RtlInterlockedPushEntrySList(listhead : *mut SLIST_HEADER, listentry : *mut SINGLE_LIST_ENTRY) -> PSLIST_ENTRY);
    unsafe { RtlInterlockedPushEntrySList(listhead as _, listentry as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn RtlInterlockedPushEntrySList(listhead: *mut SLIST_HEADER, listentry: *mut SLIST_ENTRY) -> PSLIST_ENTRY {
    windows_core::link!("ntdll.dll" "system" fn RtlInterlockedPushEntrySList(listhead : *mut SLIST_HEADER, listentry : *mut SLIST_ENTRY) -> PSLIST_ENTRY);
    unsafe { RtlInterlockedPushEntrySList(listhead as _, listentry as _) }
}
#[cfg(target_arch = "x86")]
#[inline]
pub unsafe fn RtlInterlockedPushListSListEx(listhead: *mut SLIST_HEADER, list: *mut SINGLE_LIST_ENTRY, listend: *mut SINGLE_LIST_ENTRY, count: u32) -> PSLIST_ENTRY {
    windows_core::link!("ntdll.dll" "system" fn RtlInterlockedPushListSListEx(listhead : *mut SLIST_HEADER, list : *mut SINGLE_LIST_ENTRY, listend : *mut SINGLE_LIST_ENTRY, count : u32) -> PSLIST_ENTRY);
    unsafe { RtlInterlockedPushListSListEx(listhead as _, list as _, listend as _, count) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn RtlInterlockedPushListSListEx(listhead: *mut SLIST_HEADER, list: *mut SLIST_ENTRY, listend: *mut SLIST_ENTRY, count: u32) -> PSLIST_ENTRY {
    windows_core::link!("ntdll.dll" "system" fn RtlInterlockedPushListSListEx(listhead : *mut SLIST_HEADER, list : *mut SLIST_ENTRY, listend : *mut SLIST_ENTRY, count : u32) -> PSLIST_ENTRY);
    unsafe { RtlInterlockedPushListSListEx(listhead as _, list as _, listend as _, count) }
}
#[inline]
pub unsafe fn RtlIsZeroMemory(buffer: *const core::ffi::c_void, length: usize) -> bool {
    windows_core::link!("ntdll.dll" "system" fn RtlIsZeroMemory(buffer : *const core::ffi::c_void, length : usize) -> bool);
    unsafe { RtlIsZeroMemory(buffer, length) }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn RtlLookupFunctionEntry(controlpc: u64, imagebase: *mut u64, historytable: Option<*mut UNWIND_HISTORY_TABLE>) -> PRUNTIME_FUNCTION {
    windows_core::link!("kernel32.dll" "system" fn RtlLookupFunctionEntry(controlpc : u64, imagebase : *mut u64, historytable : *mut UNWIND_HISTORY_TABLE) -> PRUNTIME_FUNCTION);
    unsafe { RtlLookupFunctionEntry(controlpc, imagebase as _, historytable.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(target_arch = "aarch64")]
#[inline]
pub unsafe fn RtlLookupFunctionEntry(controlpc: usize, imagebase: *mut u64, historytable: Option<*mut UNWIND_HISTORY_TABLE>) -> PRUNTIME_FUNCTION {
    windows_core::link!("kernel32.dll" "system" fn RtlLookupFunctionEntry(controlpc : usize, imagebase : *mut u64, historytable : *mut UNWIND_HISTORY_TABLE) -> PRUNTIME_FUNCTION);
    unsafe { RtlLookupFunctionEntry(controlpc, imagebase as _, historytable.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RtlNormalizeSecurityDescriptor(securitydescriptor: *mut PSECURITY_DESCRIPTOR, securitydescriptorlength: u32, newsecuritydescriptor: Option<*mut PSECURITY_DESCRIPTOR>, newsecuritydescriptorlength: Option<*mut u32>, checkonly: bool) -> bool {
    windows_core::link!("ntdll.dll" "system" fn RtlNormalizeSecurityDescriptor(securitydescriptor : *mut PSECURITY_DESCRIPTOR, securitydescriptorlength : u32, newsecuritydescriptor : *mut PSECURITY_DESCRIPTOR, newsecuritydescriptorlength : *mut u32, checkonly : bool) -> bool);
    unsafe { RtlNormalizeSecurityDescriptor(securitydescriptor as _, securitydescriptorlength, newsecuritydescriptor.unwrap_or(core::mem::zeroed()) as _, newsecuritydescriptorlength.unwrap_or(core::mem::zeroed()) as _, checkonly) }
}
#[inline]
pub unsafe fn RtlOsDeploymentState(flags: u32) -> OS_DEPLOYEMENT_STATE_VALUES {
    windows_core::link!("ntdll.dll" "system" fn RtlOsDeploymentState(flags : u32) -> OS_DEPLOYEMENT_STATE_VALUES);
    unsafe { RtlOsDeploymentState(flags) }
}
#[inline]
pub unsafe fn RtlPcToFileHeader(pcvalue: *const core::ffi::c_void, baseofimage: *mut *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    windows_core::link!("kernel32.dll" "system" fn RtlPcToFileHeader(pcvalue : *const core::ffi::c_void, baseofimage : *mut *mut core::ffi::c_void) -> *mut core::ffi::c_void);
    unsafe { RtlPcToFileHeader(pcvalue, baseofimage as _) }
}
#[inline]
pub unsafe fn RtlQueryDepthSList(listhead: *const SLIST_HEADER) -> u16 {
    windows_core::link!("ntdll.dll" "system" fn RtlQueryDepthSList(listhead : *const SLIST_HEADER) -> u16);
    unsafe { RtlQueryDepthSList(listhead) }
}
#[inline]
pub unsafe fn RtlRaiseCustomSystemEventTrigger(triggerconfig: *const CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlRaiseCustomSystemEventTrigger(triggerconfig : *const CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG) -> u32);
    unsafe { RtlRaiseCustomSystemEventTrigger(triggerconfig) }
}
#[inline]
pub unsafe fn RtlRaiseException(exceptionrecord: *const EXCEPTION_RECORD) {
    windows_core::link!("kernel32.dll" "system" fn RtlRaiseException(exceptionrecord : *const EXCEPTION_RECORD));
    unsafe { RtlRaiseException(exceptionrecord) }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[inline]
pub unsafe fn RtlRestoreContext(contextrecord: *const CONTEXT, exceptionrecord: Option<*const EXCEPTION_RECORD>) {
    windows_core::link!("kernel32.dll" "C" fn RtlRestoreContext(contextrecord : *const CONTEXT, exceptionrecord : *const EXCEPTION_RECORD));
    unsafe { RtlRestoreContext(contextrecord, exceptionrecord.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(target_arch = "aarch64")]
#[inline]
pub unsafe fn RtlRestoreContext(contextrecord: *const ARM64_NT_CONTEXT, exceptionrecord: Option<*const EXCEPTION_RECORD>) {
    windows_core::link!("kernel32.dll" "C" fn RtlRestoreContext(contextrecord : *const ARM64_NT_CONTEXT, exceptionrecord : *const EXCEPTION_RECORD));
    unsafe { RtlRestoreContext(contextrecord, exceptionrecord.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RtlSwitchedVVI(versioninfo: *const OSVERSIONINFOEXW, typemask: u32, conditionmask: u64) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlSwitchedVVI(versioninfo : *const OSVERSIONINFOEXW, typemask : u32, conditionmask : u64) -> u32);
    unsafe { RtlSwitchedVVI(versioninfo, typemask, conditionmask) }
}
#[inline]
pub unsafe fn RtlUnwind(targetframe: Option<*const core::ffi::c_void>, targetip: Option<*const core::ffi::c_void>, exceptionrecord: Option<*const EXCEPTION_RECORD>, returnvalue: *const core::ffi::c_void) {
    windows_core::link!("kernel32.dll" "system" fn RtlUnwind(targetframe : *const core::ffi::c_void, targetip : *const core::ffi::c_void, exceptionrecord : *const EXCEPTION_RECORD, returnvalue : *const core::ffi::c_void));
    unsafe { RtlUnwind(targetframe.unwrap_or(core::mem::zeroed()) as _, targetip.unwrap_or(core::mem::zeroed()) as _, exceptionrecord.unwrap_or(core::mem::zeroed()) as _, returnvalue) }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn RtlUnwindEx(targetframe: Option<*const core::ffi::c_void>, targetip: Option<*const core::ffi::c_void>, exceptionrecord: Option<*const EXCEPTION_RECORD>, returnvalue: *const core::ffi::c_void, contextrecord: *const CONTEXT, historytable: Option<*const UNWIND_HISTORY_TABLE>) {
    windows_core::link!("kernel32.dll" "system" fn RtlUnwindEx(targetframe : *const core::ffi::c_void, targetip : *const core::ffi::c_void, exceptionrecord : *const EXCEPTION_RECORD, returnvalue : *const core::ffi::c_void, contextrecord : *const CONTEXT, historytable : *const UNWIND_HISTORY_TABLE));
    unsafe { RtlUnwindEx(targetframe.unwrap_or(core::mem::zeroed()) as _, targetip.unwrap_or(core::mem::zeroed()) as _, exceptionrecord.unwrap_or(core::mem::zeroed()) as _, returnvalue, contextrecord, historytable.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(target_arch = "aarch64")]
#[inline]
pub unsafe fn RtlUnwindEx(targetframe: Option<*const core::ffi::c_void>, targetip: Option<*const core::ffi::c_void>, exceptionrecord: Option<*const EXCEPTION_RECORD>, returnvalue: *const core::ffi::c_void, contextrecord: *const ARM64_NT_CONTEXT, historytable: Option<*const UNWIND_HISTORY_TABLE>) {
    windows_core::link!("kernel32.dll" "system" fn RtlUnwindEx(targetframe : *const core::ffi::c_void, targetip : *const core::ffi::c_void, exceptionrecord : *const EXCEPTION_RECORD, returnvalue : *const core::ffi::c_void, contextrecord : *const ARM64_NT_CONTEXT, historytable : *const UNWIND_HISTORY_TABLE));
    unsafe { RtlUnwindEx(targetframe.unwrap_or(core::mem::zeroed()) as _, targetip.unwrap_or(core::mem::zeroed()) as _, exceptionrecord.unwrap_or(core::mem::zeroed()) as _, returnvalue, contextrecord, historytable.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RtlValidateCorrelationVector(vector: *const CORRELATION_VECTOR) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlValidateCorrelationVector(vector : *const CORRELATION_VECTOR) -> u32);
    unsafe { RtlValidateCorrelationVector(vector) }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "excpt"))]
#[inline]
pub unsafe fn RtlVirtualUnwind(handlertype: u32, imagebase: u64, controlpc: u64, functionentry: *const RUNTIME_FUNCTION, contextrecord: *mut CONTEXT, handlerdata: *mut *mut core::ffi::c_void, establisherframe: *mut u64, contextpointers: Option<*mut KNONVOLATILE_CONTEXT_POINTERS>) -> PEXCEPTION_ROUTINE {
    windows_core::link!("kernel32.dll" "system" fn RtlVirtualUnwind(handlertype : u32, imagebase : u64, controlpc : u64, functionentry : *const RUNTIME_FUNCTION, contextrecord : *mut CONTEXT, handlerdata : *mut *mut core::ffi::c_void, establisherframe : *mut u64, contextpointers : *mut KNONVOLATILE_CONTEXT_POINTERS) -> PEXCEPTION_ROUTINE);
    unsafe { RtlVirtualUnwind(handlertype, imagebase, controlpc, functionentry, contextrecord as _, handlerdata as _, establisherframe as _, contextpointers.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(target_arch = "aarch64")]
#[cfg(all(feature = "basetsd", feature = "excpt"))]
#[inline]
pub unsafe fn RtlVirtualUnwind(handlertype: u32, imagebase: usize, controlpc: usize, functionentry: *const ARM64_RUNTIME_FUNCTION, contextrecord: *mut ARM64_NT_CONTEXT, handlerdata: *mut *mut core::ffi::c_void, establisherframe: *mut u64, contextpointers: Option<*mut KNONVOLATILE_CONTEXT_POINTERS_ARM64>) -> PEXCEPTION_ROUTINE {
    windows_core::link!("kernel32.dll" "system" fn RtlVirtualUnwind(handlertype : u32, imagebase : usize, controlpc : usize, functionentry : *const ARM64_RUNTIME_FUNCTION, contextrecord : *mut ARM64_NT_CONTEXT, handlerdata : *mut *mut core::ffi::c_void, establisherframe : *mut u64, contextpointers : *mut KNONVOLATILE_CONTEXT_POINTERS_ARM64) -> PEXCEPTION_ROUTINE);
    unsafe { RtlVirtualUnwind(handlertype, imagebase, controlpc, functionentry, contextrecord as _, handlerdata as _, establisherframe as _, contextpointers.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "excpt"))]
#[inline]
pub unsafe fn RtlVirtualUnwind2(handlertype: u32, imagebase: u64, controlpc: u64, functionentry: Option<*const RUNTIME_FUNCTION>, contextrecord: *mut CONTEXT, machineframeunwound: Option<*mut bool>, handlerdata: *mut *mut core::ffi::c_void, establisherframe: *mut u64, contextpointers: Option<*mut KNONVOLATILE_CONTEXT_POINTERS>, lowlimit: Option<*const u64>, highlimit: Option<*const u64>, handlerroutine: *mut PEXCEPTION_ROUTINE, unwindflags: u32) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn RtlVirtualUnwind2(handlertype : u32, imagebase : u64, controlpc : u64, functionentry : *const RUNTIME_FUNCTION, contextrecord : *mut CONTEXT, machineframeunwound : *mut bool, handlerdata : *mut *mut core::ffi::c_void, establisherframe : *mut u64, contextpointers : *mut KNONVOLATILE_CONTEXT_POINTERS, lowlimit : *const u64, highlimit : *const u64, handlerroutine : *mut PEXCEPTION_ROUTINE, unwindflags : u32) -> u32);
    unsafe { RtlVirtualUnwind2(handlertype, imagebase, controlpc, functionentry.unwrap_or(core::mem::zeroed()) as _, contextrecord as _, machineframeunwound.unwrap_or(core::mem::zeroed()) as _, handlerdata as _, establisherframe as _, contextpointers.unwrap_or(core::mem::zeroed()) as _, lowlimit.unwrap_or(core::mem::zeroed()) as _, highlimit.unwrap_or(core::mem::zeroed()) as _, handlerroutine as _, unwindflags) }
}
#[cfg(target_arch = "aarch64")]
#[cfg(all(feature = "basetsd", feature = "excpt"))]
#[inline]
pub unsafe fn RtlVirtualUnwind2(handlertype: u32, imagebase: usize, controlpc: usize, functionentry: Option<*const ARM64_RUNTIME_FUNCTION>, contextrecord: *mut ARM64_NT_CONTEXT, machineframeunwound: Option<*mut bool>, handlerdata: *mut *mut core::ffi::c_void, establisherframe: *mut u64, contextpointers: Option<*mut KNONVOLATILE_CONTEXT_POINTERS_ARM64>, lowlimit: Option<*const u64>, highlimit: Option<*const u64>, handlerroutine: *mut PEXCEPTION_ROUTINE, unwindflags: u32) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn RtlVirtualUnwind2(handlertype : u32, imagebase : usize, controlpc : usize, functionentry : *const ARM64_RUNTIME_FUNCTION, contextrecord : *mut ARM64_NT_CONTEXT, machineframeunwound : *mut bool, handlerdata : *mut *mut core::ffi::c_void, establisherframe : *mut u64, contextpointers : *mut KNONVOLATILE_CONTEXT_POINTERS_ARM64, lowlimit : *const u64, highlimit : *const u64, handlerroutine : *mut PEXCEPTION_ROUTINE, unwindflags : u32) -> u32);
    unsafe { RtlVirtualUnwind2(handlertype, imagebase, controlpc, functionentry.unwrap_or(core::mem::zeroed()) as _, contextrecord as _, machineframeunwound.unwrap_or(core::mem::zeroed()) as _, handlerdata as _, establisherframe as _, contextpointers.unwrap_or(core::mem::zeroed()) as _, lowlimit.unwrap_or(core::mem::zeroed()) as _, highlimit.unwrap_or(core::mem::zeroed()) as _, handlerroutine as _, unwindflags) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn RtlWriteNonVolatileMemory(nvtoken: *const core::ffi::c_void, nvdestination: *mut core::ffi::c_void, source: *const core::ffi::c_void, size: usize, flags: u32) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlWriteNonVolatileMemory(nvtoken : *const core::ffi::c_void, nvdestination : *mut core::ffi::c_void, source : *const core::ffi::c_void, size : usize, flags : u32) -> u32);
    unsafe { RtlWriteNonVolatileMemory(nvtoken, nvdestination as _, source, size, flags) }
}
#[inline]
pub unsafe fn VerSetConditionMask(conditionmask: u64, typemask: u32, condition: u8) -> u64 {
    windows_core::link!("kernel32.dll" "system" fn VerSetConditionMask(conditionmask : u64, typemask : u32, condition : u8) -> u64);
    unsafe { VerSetConditionMask(conditionmask, typemask, condition) }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ACCESS_ALLOWED_ACE {
    pub Header: ACE_HEADER,
    pub Mask: ACCESS_MASK,
    pub SidStart: u32,
}
pub const ACCESS_ALLOWED_ACE_TYPE: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ACCESS_ALLOWED_CALLBACK_ACE {
    pub Header: ACE_HEADER,
    pub Mask: ACCESS_MASK,
    pub SidStart: u32,
}
pub const ACCESS_ALLOWED_CALLBACK_ACE_TYPE: u32 = 9;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ACCESS_ALLOWED_CALLBACK_OBJECT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: ACCESS_MASK,
    pub Flags: u32,
    pub ObjectType: windows_core::GUID,
    pub InheritedObjectType: windows_core::GUID,
    pub SidStart: u32,
}
pub const ACCESS_ALLOWED_CALLBACK_OBJECT_ACE_TYPE: u32 = 11;
pub const ACCESS_ALLOWED_COMPOUND_ACE_TYPE: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ACCESS_ALLOWED_OBJECT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: ACCESS_MASK,
    pub Flags: u32,
    pub ObjectType: windows_core::GUID,
    pub InheritedObjectType: windows_core::GUID,
    pub SidStart: u32,
}
pub const ACCESS_ALLOWED_OBJECT_ACE_TYPE: u32 = 5;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ACCESS_DENIED_ACE {
    pub Header: ACE_HEADER,
    pub Mask: ACCESS_MASK,
    pub SidStart: u32,
}
pub const ACCESS_DENIED_ACE_TYPE: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ACCESS_DENIED_CALLBACK_ACE {
    pub Header: ACE_HEADER,
    pub Mask: ACCESS_MASK,
    pub SidStart: u32,
}
pub const ACCESS_DENIED_CALLBACK_ACE_TYPE: u32 = 10;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ACCESS_DENIED_CALLBACK_OBJECT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: ACCESS_MASK,
    pub Flags: u32,
    pub ObjectType: windows_core::GUID,
    pub InheritedObjectType: windows_core::GUID,
    pub SidStart: u32,
}
pub const ACCESS_DENIED_CALLBACK_OBJECT_ACE_TYPE: u32 = 12;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ACCESS_DENIED_OBJECT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: ACCESS_MASK,
    pub Flags: u32,
    pub ObjectType: windows_core::GUID,
    pub InheritedObjectType: windows_core::GUID,
    pub SidStart: u32,
}
pub const ACCESS_DENIED_OBJECT_ACE_TYPE: u32 = 6;
pub const ACCESS_DS_OBJECT_TYPE_NAME_A: windows_core::PCSTR = windows_core::s!("Directory Service Object");
pub const ACCESS_DS_OBJECT_TYPE_NAME_W: windows_core::PCWSTR = windows_core::w!("Directory Service Object");
pub const ACCESS_DS_SOURCE_A: windows_core::PCSTR = windows_core::s!("DS");
pub const ACCESS_DS_SOURCE_W: windows_core::PCWSTR = windows_core::w!("DS");
pub const ACCESS_FILTER_SECURITY_INFORMATION: u32 = 256;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ACCESS_MASK(pub u32);
pub const ACCESS_MAX_LEVEL: u32 = 4;
pub const ACCESS_MAX_MS_ACE_TYPE: u32 = 8;
pub const ACCESS_MAX_MS_OBJECT_ACE_TYPE: u32 = 8;
pub const ACCESS_MAX_MS_V2_ACE_TYPE: u32 = 3;
pub const ACCESS_MAX_MS_V3_ACE_TYPE: u32 = 4;
pub const ACCESS_MAX_MS_V4_ACE_TYPE: u32 = 8;
pub const ACCESS_MAX_MS_V5_ACE_TYPE: u32 = 21;
pub const ACCESS_MIN_MS_ACE_TYPE: u32 = 0;
pub const ACCESS_MIN_MS_OBJECT_ACE_TYPE: u32 = 5;
pub const ACCESS_OBJECT_GUID: u32 = 0;
pub const ACCESS_PROPERTY_GUID: u32 = 2;
pub const ACCESS_PROPERTY_SET_GUID: u32 = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ACCESS_REASON(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ACCESS_REASONS {
    pub Data: [ACCESS_REASON; 32],
}
impl Default for ACCESS_REASONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ACCESS_REASON_DATA_MASK: u32 = 65535;
pub const ACCESS_REASON_EXDATA_MASK: u32 = 2130706432;
pub const ACCESS_REASON_STAGING_MASK: u32 = 2147483648;
pub type ACCESS_REASON_TYPE = i32;
pub const ACCESS_REASON_TYPE_MASK: u32 = 16711680;
pub const ACCESS_SYSTEM_SECURITY: u32 = 16777216;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ACE_HEADER {
    pub AceType: u8,
    pub AceFlags: u8,
    pub AceSize: u16,
}
pub const ACE_INHERITED_OBJECT_TYPE_PRESENT: u32 = 2;
pub const ACE_OBJECT_TYPE_PRESENT: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ACL {
    pub AclRevision: u8,
    pub Sbz1: u8,
    pub AclSize: u16,
    pub AceCount: u16,
    pub Sbz2: u16,
}
pub type ACL_INFORMATION_CLASS = i32;
pub const ACL_REVISION: u32 = 2;
pub const ACL_REVISION1: u32 = 1;
pub const ACL_REVISION2: u32 = 2;
pub const ACL_REVISION3: u32 = 3;
pub const ACL_REVISION4: u32 = 4;
pub const ACL_REVISION_DS: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ACL_REVISION_INFORMATION {
    pub AclRevision: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ACL_SIZE_INFORMATION {
    pub AceCount: u32,
    pub AclBytesInUse: u32,
    pub AclBytesFree: u32,
}
pub const ACPI_PPM_HARDWARE_ALL: u32 = 254;
pub const ACPI_PPM_SOFTWARE_ALL: u32 = 252;
pub const ACPI_PPM_SOFTWARE_ANY: u32 = 253;
pub type ACTCTX_COMPATIBILITY_ELEMENT_TYPE = i32;
pub const ACTCTX_COMPATIBILITY_ELEMENT_TYPE_MAXVERSIONTESTED: ACTCTX_COMPATIBILITY_ELEMENT_TYPE = 3;
pub const ACTCTX_COMPATIBILITY_ELEMENT_TYPE_MITIGATION: ACTCTX_COMPATIBILITY_ELEMENT_TYPE = 2;
pub const ACTCTX_COMPATIBILITY_ELEMENT_TYPE_OS: ACTCTX_COMPATIBILITY_ELEMENT_TYPE = 1;
pub const ACTCTX_COMPATIBILITY_ELEMENT_TYPE_UNKNOWN: ACTCTX_COMPATIBILITY_ELEMENT_TYPE = 0;
pub type ACTCTX_REQUESTED_RUN_LEVEL = i32;
pub const ACTCTX_RUN_LEVEL_AS_INVOKER: ACTCTX_REQUESTED_RUN_LEVEL = 1;
pub const ACTCTX_RUN_LEVEL_HIGHEST_AVAILABLE: ACTCTX_REQUESTED_RUN_LEVEL = 2;
pub const ACTCTX_RUN_LEVEL_NUMBERS: ACTCTX_REQUESTED_RUN_LEVEL = 4;
pub const ACTCTX_RUN_LEVEL_REQUIRE_ADMIN: ACTCTX_REQUESTED_RUN_LEVEL = 3;
pub const ACTCTX_RUN_LEVEL_UNSPECIFIED: ACTCTX_REQUESTED_RUN_LEVEL = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION {
    pub ulFlags: u32,
    pub ulEncodedAssemblyIdentityLength: u32,
    pub ulManifestPathType: u32,
    pub ulManifestPathLength: u32,
    pub liManifestLastWriteTime: i64,
    pub ulPolicyPathType: u32,
    pub ulPolicyPathLength: u32,
    pub liPolicyLastWriteTime: i64,
    pub ulMetadataSatelliteRosterIndex: u32,
    pub ulManifestVersionMajor: u32,
    pub ulManifestVersionMinor: u32,
    pub ulPolicyVersionMajor: u32,
    pub ulPolicyVersionMinor: u32,
    pub ulAssemblyDirectoryNameLength: u32,
    pub lpAssemblyEncodedAssemblyIdentity: windows_core::PCWSTR,
    pub lpAssemblyManifestPath: windows_core::PCWSTR,
    pub lpAssemblyPolicyPath: windows_core::PCWSTR,
    pub lpAssemblyDirectoryName: windows_core::PCWSTR,
    pub ulFileCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION {
    pub ElementCount: u32,
    pub Elements: [COMPATIBILITY_CONTEXT_ELEMENT; 0],
}
impl Default for ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ACTIVATION_CONTEXT_DETAILED_INFORMATION {
    pub dwFlags: u32,
    pub ulFormatVersion: u32,
    pub ulAssemblyCount: u32,
    pub ulRootManifestPathType: u32,
    pub ulRootManifestPathChars: u32,
    pub ulRootConfigurationPathType: u32,
    pub ulRootConfigurationPathChars: u32,
    pub ulAppDirPathType: u32,
    pub ulAppDirPathChars: u32,
    pub lpRootManifestPath: windows_core::PCWSTR,
    pub lpRootConfigurationPath: windows_core::PCWSTR,
    pub lpAppDirPath: windows_core::PCWSTR,
}
pub type ACTIVATION_CONTEXT_INFO_CLASS = i32;
pub const ACTIVATION_CONTEXT_PATH_TYPE_ASSEMBLYREF: u32 = 4;
pub const ACTIVATION_CONTEXT_PATH_TYPE_NONE: u32 = 1;
pub const ACTIVATION_CONTEXT_PATH_TYPE_URL: u32 = 3;
pub const ACTIVATION_CONTEXT_PATH_TYPE_WIN32_FILE: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ACTIVATION_CONTEXT_QUERY_INDEX {
    pub ulAssemblyIndex: u32,
    pub ulFileIndexInAssembly: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION {
    pub ulFlags: u32,
    pub RunLevel: ACTCTX_REQUESTED_RUN_LEVEL,
    pub UiAccess: u32,
}
pub const ACTIVATION_CONTEXT_SECTION_APPLICATION_SETTINGS: u32 = 10;
pub const ACTIVATION_CONTEXT_SECTION_ASSEMBLY_INFORMATION: u32 = 1;
pub const ACTIVATION_CONTEXT_SECTION_CLR_SURROGATES: u32 = 9;
pub const ACTIVATION_CONTEXT_SECTION_COMPATIBILITY_INFO: u32 = 11;
pub const ACTIVATION_CONTEXT_SECTION_COM_INTERFACE_REDIRECTION: u32 = 5;
pub const ACTIVATION_CONTEXT_SECTION_COM_PROGID_REDIRECTION: u32 = 7;
pub const ACTIVATION_CONTEXT_SECTION_COM_SERVER_REDIRECTION: u32 = 4;
pub const ACTIVATION_CONTEXT_SECTION_COM_TYPE_LIBRARY_REDIRECTION: u32 = 6;
pub const ACTIVATION_CONTEXT_SECTION_DLL_REDIRECTION: u32 = 2;
pub const ACTIVATION_CONTEXT_SECTION_GLOBAL_OBJECT_RENAME_TABLE: u32 = 8;
pub const ACTIVATION_CONTEXT_SECTION_WINDOW_CLASS_REDIRECTION: u32 = 3;
pub const ACTIVATION_CONTEXT_SECTION_WINRT_ACTIVATABLE_CLASSES: u32 = 12;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ADMINISTRATOR_POWER_POLICY {
    pub MinSleep: SYSTEM_POWER_STATE,
    pub MaxSleep: SYSTEM_POWER_STATE,
    pub MinVideoTimeout: u32,
    pub MaxVideoTimeout: u32,
    pub MinSpindownTimeout: u32,
    pub MaxSpindownTimeout: u32,
}
pub const ALL_POWERSCHEMES_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x68a1e95e_13ea_41e1_8011_0c496ca490b0);
pub const ALL_PROCESSOR_GROUPS: u32 = 65535;
pub const ALTITUDE_GROUP_POLICY: POWER_SETTING_ALTITUDE = 0;
pub const ALTITUDE_INTERNAL_OVERRIDE: POWER_SETTING_ALTITUDE = 5;
pub const ALTITUDE_OEM_CUSTOMIZATION: POWER_SETTING_ALTITUDE = 4;
pub const ALTITUDE_OS_DEFAULT: POWER_SETTING_ALTITUDE = 6;
pub const ALTITUDE_PROVISIONING: POWER_SETTING_ALTITUDE = 3;
pub const ALTITUDE_RUNTIME_OVERRIDE: POWER_SETTING_ALTITUDE = 2;
pub const ALTITUDE_USER: POWER_SETTING_ALTITUDE = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ANON_OBJECT_HEADER {
    pub Sig1: u16,
    pub Sig2: u16,
    pub Version: u16,
    pub Machine: u16,
    pub TimeDateStamp: u32,
    pub ClassID: windows_core::GUID,
    pub SizeOfData: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ANON_OBJECT_HEADER_BIGOBJ {
    pub Sig1: u16,
    pub Sig2: u16,
    pub Version: u16,
    pub Machine: u16,
    pub TimeDateStamp: u32,
    pub ClassID: windows_core::GUID,
    pub SizeOfData: u32,
    pub Flags: u32,
    pub MetaDataSize: u32,
    pub MetaDataOffset: u32,
    pub NumberOfSections: u32,
    pub PointerToSymbolTable: u32,
    pub NumberOfSymbols: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ANON_OBJECT_HEADER_V2 {
    pub Sig1: u16,
    pub Sig2: u16,
    pub Version: u16,
    pub Machine: u16,
    pub TimeDateStamp: u32,
    pub ClassID: windows_core::GUID,
    pub SizeOfData: u32,
    pub Flags: u32,
    pub MetaDataSize: u32,
    pub MetaDataOffset: u32,
}
pub const ANSI_NULL: i8 = 0;
pub const ANYSIZE_ARRAY: u32 = 1;
pub type APC_CALLBACK_FUNCTION = Option<unsafe extern "system" fn(param0: u32, param1: *mut core::ffi::c_void, param2: *mut core::ffi::c_void)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct APPLICATIONLAUNCH_SETTING_VALUE {
    pub ActivationTime: i64,
    pub Flags: u32,
    pub ButtonInstanceID: u32,
}
pub const APPLICATION_ERROR_MASK: u32 = 536870912;
#[repr(C, align(16))]
#[derive(Clone, Copy)]
pub struct ARM64EC_NT_CONTEXT {
    pub Anonymous: ARM64EC_NT_CONTEXT_0,
}
impl Default for ARM64EC_NT_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union ARM64EC_NT_CONTEXT_0 {
    pub Anonymous: ARM64EC_NT_CONTEXT_0_0,
}
impl Default for ARM64EC_NT_CONTEXT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ARM64EC_NT_CONTEXT_0_0 {
    pub AMD64_P1Home: u64,
    pub AMD64_P2Home: u64,
    pub AMD64_P3Home: u64,
    pub AMD64_P4Home: u64,
    pub AMD64_P5Home: u64,
    pub AMD64_P6Home: u64,
    pub ContextFlags: u32,
    pub AMD64_MxCsr_copy: u32,
    pub AMD64_SegCs: u16,
    pub AMD64_SegDs: u16,
    pub AMD64_SegEs: u16,
    pub AMD64_SegFs: u16,
    pub AMD64_SegGs: u16,
    pub AMD64_SegSs: u16,
    pub AMD64_EFlags: u32,
    pub AMD64_Dr0: u64,
    pub AMD64_Dr1: u64,
    pub AMD64_Dr2: u64,
    pub AMD64_Dr3: u64,
    pub AMD64_Dr6: u64,
    pub AMD64_Dr7: u64,
    pub X8: u64,
    pub X0: u64,
    pub X1: u64,
    pub X27: u64,
    pub Sp: u64,
    pub Fp: u64,
    pub X25: u64,
    pub X26: u64,
    pub X2: u64,
    pub X3: u64,
    pub X4: u64,
    pub X5: u64,
    pub X19: u64,
    pub X20: u64,
    pub X21: u64,
    pub X22: u64,
    pub Pc: u64,
    pub Anonymous: ARM64EC_NT_CONTEXT_0_0_0,
    pub AMD64_VectorRegister: [ARM64_NT_NEON128; 26],
    pub AMD64_VectorControl: u64,
    pub AMD64_DebugControl: u64,
    pub AMD64_LastBranchToRip: u64,
    pub AMD64_LastBranchFromRip: u64,
    pub AMD64_LastExceptionToRip: u64,
    pub AMD64_LastExceptionFromRip: u64,
}
impl Default for ARM64EC_NT_CONTEXT_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ARM64EC_NT_CONTEXT_0_0_0 {
    pub AMD64_ControlWord: u16,
    pub AMD64_StatusWord: u16,
    pub AMD64_TagWord: u8,
    pub AMD64_Reserved1: u8,
    pub AMD64_ErrorOpcode: u16,
    pub AMD64_ErrorOffset: u32,
    pub AMD64_ErrorSelector: u16,
    pub AMD64_Reserved2: u16,
    pub AMD64_DataOffset: u32,
    pub AMD64_DataSelector: u16,
    pub AMD64_Reserved3: u16,
    pub AMD64_MxCsr: u32,
    pub AMD64_MxCsr_Mask: u32,
    pub Lr: u64,
    pub X16_0: u16,
    pub AMD64_St0_Reserved1: u16,
    pub AMD64_St0_Reserved2: u32,
    pub X6: u64,
    pub X16_1: u16,
    pub AMD64_St1_Reserved1: u16,
    pub AMD64_St1_Reserved2: u32,
    pub X7: u64,
    pub X16_2: u16,
    pub AMD64_St2_Reserved1: u16,
    pub AMD64_St2_Reserved2: u32,
    pub X9: u64,
    pub X16_3: u16,
    pub AMD64_St3_Reserved1: u16,
    pub AMD64_St3_Reserved2: u32,
    pub X10: u64,
    pub X17_0: u16,
    pub AMD64_St4_Reserved1: u16,
    pub AMD64_St4_Reserved2: u32,
    pub X11: u64,
    pub X17_1: u16,
    pub AMD64_St5_Reserved1: u16,
    pub AMD64_St5_Reserved2: u32,
    pub X12: u64,
    pub X17_2: u16,
    pub AMD64_St6_Reserved1: u16,
    pub AMD64_St6_Reserved2: u32,
    pub X15: u64,
    pub X17_3: u16,
    pub AMD64_St7_Reserved1: u16,
    pub AMD64_St7_Reserved2: u32,
    pub V: [ARM64_NT_NEON128; 16],
    pub AMD64_XSAVE_FORMAT_Reserved4: [u8; 96],
}
impl Default for ARM64EC_NT_CONTEXT_0_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "aarch64")]
pub const ARM64_CNTFRQ_EL0: u32 = 24320;
#[cfg(target_arch = "aarch64")]
pub const ARM64_CNTVCT: u32 = 24322;
#[cfg(target_arch = "aarch64")]
pub const ARM64_CNTVCT_EL0: u32 = 24322;
pub type ARM64_FNPDATA_CR = i32;
pub type ARM64_FNPDATA_FLAGS = i32;
pub const ARM64_MAX_BREAKPOINTS: u32 = 8;
pub const ARM64_MAX_WATCHPOINTS: u32 = 2;
#[repr(C, align(16))]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct ARM64_NT_CONTEXT {
    pub ContextFlags: u32,
    pub Cpsr: u32,
    pub Anonymous: ARM64_NT_CONTEXT_0,
    pub Sp: u64,
    pub Pc: u64,
    pub V: [ARM64_NT_NEON128; 32],
    pub Fpcr: u32,
    pub Fpsr: u32,
    pub Bcr: [u32; 8],
    pub Bvr: [u64; 8],
    pub Wcr: [u32; 2],
    pub Wvr: [u64; 2],
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
impl Default for ARM64_NT_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union ARM64_NT_CONTEXT_0 {
    pub Anonymous: ARM64_NT_CONTEXT_0_0,
    pub X: [u64; 31],
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
impl Default for ARM64_NT_CONTEXT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ARM64_NT_CONTEXT_0_0 {
    pub X0: u64,
    pub X1: u64,
    pub X2: u64,
    pub X3: u64,
    pub X4: u64,
    pub X5: u64,
    pub X6: u64,
    pub X7: u64,
    pub X8: u64,
    pub X9: u64,
    pub X10: u64,
    pub X11: u64,
    pub X12: u64,
    pub X13: u64,
    pub X14: u64,
    pub X15: u64,
    pub X16: u64,
    pub X17: u64,
    pub X18: u64,
    pub X19: u64,
    pub X20: u64,
    pub X21: u64,
    pub X22: u64,
    pub X23: u64,
    pub X24: u64,
    pub X25: u64,
    pub X26: u64,
    pub X27: u64,
    pub X28: u64,
    pub Fp: u64,
    pub Lr: u64,
}
#[repr(C, align(16))]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy)]
pub struct ARM64_NT_CONTEXT {
    pub ContextFlags: u32,
    pub Cpsr: u32,
    pub Anonymous: ARM64_NT_CONTEXT_0,
    pub Sp: u64,
    pub Pc: u64,
    pub V: [NEON128; 32],
    pub Fpcr: u32,
    pub Fpsr: u32,
    pub Bcr: [u32; 8],
    pub Bvr: [u64; 8],
    pub Wcr: [u32; 2],
    pub Wvr: [u64; 2],
}
#[cfg(target_arch = "aarch64")]
impl Default for ARM64_NT_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy)]
pub union ARM64_NT_CONTEXT_0 {
    pub Anonymous: ARM64_NT_CONTEXT_0_0,
    pub X: [u64; 31],
}
#[cfg(target_arch = "aarch64")]
impl Default for ARM64_NT_CONTEXT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ARM64_NT_CONTEXT_0_0 {
    pub X0: u64,
    pub X1: u64,
    pub X2: u64,
    pub X3: u64,
    pub X4: u64,
    pub X5: u64,
    pub X6: u64,
    pub X7: u64,
    pub X8: u64,
    pub X9: u64,
    pub X10: u64,
    pub X11: u64,
    pub X12: u64,
    pub X13: u64,
    pub X14: u64,
    pub X15: u64,
    pub X16: u64,
    pub X17: u64,
    pub X18: u64,
    pub X19: u64,
    pub X20: u64,
    pub X21: u64,
    pub X22: u64,
    pub X23: u64,
    pub X24: u64,
    pub X25: u64,
    pub X26: u64,
    pub X27: u64,
    pub X28: u64,
    pub Fp: u64,
    pub Lr: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union ARM64_NT_NEON128 {
    pub Anonymous: ARM64_NT_NEON128_0,
    pub D: [f64; 2],
    pub S: [f32; 4],
    pub H: [u16; 8],
    pub B: [u8; 16],
}
impl Default for ARM64_NT_NEON128 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ARM64_NT_NEON128_0 {
    pub Low: u64,
    pub High: i64,
}
#[cfg(target_arch = "aarch64")]
pub const ARM64_PMCCNTR_EL0: u32 = 23784;
#[cfg(target_arch = "aarch64")]
pub const ARM64_PMSELR_EL0: u32 = 23781;
#[cfg(target_arch = "aarch64")]
pub const ARM64_PMXEVCNTR_EL0: u32 = 23786;
#[cfg(target_arch = "aarch64")]
pub const ARM64_PREFETCH_KEEP: u32 = 0;
#[cfg(target_arch = "aarch64")]
pub const ARM64_PREFETCH_L1: u32 = 0;
#[cfg(target_arch = "aarch64")]
pub const ARM64_PREFETCH_L2: u32 = 2;
#[cfg(target_arch = "aarch64")]
pub const ARM64_PREFETCH_L3: u32 = 4;
#[cfg(target_arch = "aarch64")]
pub const ARM64_PREFETCH_PLD: u32 = 0;
#[cfg(target_arch = "aarch64")]
pub const ARM64_PREFETCH_PLI: u32 = 8;
#[cfg(target_arch = "aarch64")]
pub const ARM64_PREFETCH_PST: u32 = 16;
#[cfg(target_arch = "aarch64")]
pub const ARM64_PREFETCH_STRM: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ARM64_RUNTIME_FUNCTION {
    pub BeginAddress: u32,
    pub Anonymous: ARM64_RUNTIME_FUNCTION_0,
}
impl Default for ARM64_RUNTIME_FUNCTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union ARM64_RUNTIME_FUNCTION_0 {
    pub UnwindData: u32,
    pub Anonymous: ARM64_RUNTIME_FUNCTION_0_0,
}
impl Default for ARM64_RUNTIME_FUNCTION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ARM64_RUNTIME_FUNCTION_0_0 {
    pub _bitfield: u32,
}
#[cfg(target_arch = "aarch64")]
pub const ARM64_SVCR: u32 = 23058;
#[cfg(target_arch = "aarch64")]
pub const ARM64_SVCR_SM: u32 = 1;
#[cfg(target_arch = "aarch64")]
pub const ARM64_SVCR_ZA: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ARM64_TPIDR2_BLOCK {
    pub ZaSaveBuffer: *mut core::ffi::c_void,
    pub NumZaSaveSlices: u16,
    pub Reserved1: u16,
    pub Reserved2: u32,
}
impl Default for ARM64_TPIDR2_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ARM64_TPIDR2_BLOCK_ALIGN: u32 = 16;
#[cfg(target_arch = "aarch64")]
pub const ARM64_TPIDR2_EL0: u32 = 24197;
#[cfg(target_arch = "aarch64")]
pub const ARM64_TPIDRRO_EL0: u32 = 24195;
#[cfg(target_arch = "aarch64")]
pub const ARM64_TPIDR_EL0: u32 = 24194;
#[cfg(target_arch = "aarch64")]
pub const ARM64_TPIDR_EL1: u32 = 18052;
pub const ARM_CACHE_ALIGNMENT_SIZE: u32 = 128;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ASSEMBLY_FILE_DETAILED_INFORMATION {
    pub ulFlags: u32,
    pub ulFilenameLength: u32,
    pub ulPathLength: u32,
    pub lpFileName: windows_core::PCWSTR,
    pub lpFilePath: windows_core::PCWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ATTRIBUTES_AND_SID {
    pub Attributes: u32,
    pub SidStart: u32,
}
pub const ATTRIBUTE_SECURITY_INFORMATION: u32 = 32;
pub const AUDIT_ALLOW_NO_PRIVILEGE: u32 = 1;
pub type AUDIT_EVENT_TYPE = i32;
pub const AccessReasonAllowedAce: ACCESS_REASON_TYPE = 65536;
pub const AccessReasonAllowedParentAce: ACCESS_REASON_TYPE = 196608;
pub const AccessReasonDeniedAce: ACCESS_REASON_TYPE = 131072;
pub const AccessReasonDeniedParentAce: ACCESS_REASON_TYPE = 262144;
pub const AccessReasonEmptyDacl: ACCESS_REASON_TYPE = 6291456;
pub const AccessReasonFilterAce: ACCESS_REASON_TYPE = 10485760;
pub const AccessReasonFromPrivilege: ACCESS_REASON_TYPE = 2097152;
pub const AccessReasonIntegrityLevel: ACCESS_REASON_TYPE = 3145728;
pub const AccessReasonMissingPrivilege: ACCESS_REASON_TYPE = 1048576;
pub const AccessReasonNoGrant: ACCESS_REASON_TYPE = 8388608;
pub const AccessReasonNoSD: ACCESS_REASON_TYPE = 7340032;
pub const AccessReasonNone: ACCESS_REASON_TYPE = 0;
pub const AccessReasonNotGrantedByCape: ACCESS_REASON_TYPE = 327680;
pub const AccessReasonNotGrantedByParentCape: ACCESS_REASON_TYPE = 393216;
pub const AccessReasonNotGrantedToAppContainer: ACCESS_REASON_TYPE = 458752;
pub const AccessReasonNullDacl: ACCESS_REASON_TYPE = 5242880;
pub const AccessReasonOwnership: ACCESS_REASON_TYPE = 4194304;
pub const AccessReasonTrustLabel: ACCESS_REASON_TYPE = 9437184;
pub const AclRevisionInformation: ACL_INFORMATION_CLASS = 1;
pub const AclSizeInformation: ACL_INFORMATION_CLASS = 2;
pub const ActivationContextBasicInformation: ACTIVATION_CONTEXT_INFO_CLASS = 1;
pub const ActivationContextDetailedInformation: ACTIVATION_CONTEXT_INFO_CLASS = 2;
pub const ActivationContextManifestResourceName: ACTIVATION_CONTEXT_INFO_CLASS = 7;
pub const AdapterType: SERVICE_NODE_TYPE = 4;
pub const AdministratorPowerPolicy: POWER_INFORMATION_LEVEL = 9;
pub const AssemblyDetailedInformationInActivationContext: ACTIVATION_CONTEXT_INFO_CLASS = 3;
pub const AssemblyDetailedInformationInActivationContxt: ACTIVATION_CONTEXT_INFO_CLASS = 3;
pub const AuditEventDirectoryServiceAccess: AUDIT_EVENT_TYPE = 1;
pub const AuditEventObjectAccess: AUDIT_EVENT_TYPE = 0;
pub const AutoLoad: SERVICE_LOAD_TYPE = 2;
pub const BACKUP_SECURITY_INFORMATION: u32 = 65536;
pub const BATTERY_DISCHARGE_FLAGS_ENABLE: u32 = 2147483648;
pub const BATTERY_DISCHARGE_FLAGS_EVENTCODE_MASK: u32 = 7;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BATTERY_REPORTING_SCALE {
    pub Granularity: u32,
    pub Capacity: u32,
}
pub const BatteryDeviceState: POWER_INFORMATION_LEVEL = 86;
pub const BlackBoxRecorderDirectAccessBuffer: POWER_INFORMATION_LEVEL = 97;
pub const BootLoad: SERVICE_LOAD_TYPE = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CACHE_DESCRIPTOR {
    pub Level: u8,
    pub Associativity: u8,
    pub LineSize: u16,
    pub Size: u32,
    pub Type: PROCESSOR_CACHE_TYPE,
}
pub const CACHE_FULLY_ASSOCIATIVE: u32 = 255;
#[repr(C)]
#[cfg(feature = "basetsd")]
#[derive(Clone, Copy)]
pub struct CACHE_RELATIONSHIP {
    pub Level: u8,
    pub Associativity: u8,
    pub LineSize: u16,
    pub CacheSize: u32,
    pub Type: PROCESSOR_CACHE_TYPE,
    pub Reserved: [u8; 18],
    pub GroupCount: u16,
    pub Anonymous: CACHE_RELATIONSHIP_0,
}
#[cfg(feature = "basetsd")]
impl Default for CACHE_RELATIONSHIP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "basetsd")]
#[derive(Clone, Copy)]
pub union CACHE_RELATIONSHIP_0 {
    pub GroupMask: GROUP_AFFINITY,
    pub GroupMasks: [GROUP_AFFINITY; 1],
}
#[cfg(feature = "basetsd")]
impl Default for CACHE_RELATIONSHIP_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct CCHAR(pub i8);
pub const CFG_CALL_TARGET_CONVERT_EXPORT_SUPPRESSED_TO_VALID: u32 = 4;
pub const CFG_CALL_TARGET_CONVERT_XFG_TO_CFG: u32 = 16;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CFG_CALL_TARGET_INFO {
    pub Offset: usize,
    pub Flags: usize,
}
pub const CFG_CALL_TARGET_PROCESSED: u32 = 2;
pub const CFG_CALL_TARGET_VALID: u32 = 1;
pub const CFG_CALL_TARGET_VALID_XFG: u32 = 8;
#[repr(C)]
#[cfg(feature = "basetsd")]
#[derive(Clone, Copy)]
pub struct CLAIM_SECURITY_ATTRIBUTES_INFORMATION {
    pub Version: u16,
    pub Reserved: u16,
    pub AttributeCount: u32,
    pub Attribute: CLAIM_SECURITY_ATTRIBUTES_INFORMATION_0,
}
#[cfg(feature = "basetsd")]
impl Default for CLAIM_SECURITY_ATTRIBUTES_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "basetsd")]
#[derive(Clone, Copy)]
pub union CLAIM_SECURITY_ATTRIBUTES_INFORMATION_0 {
    pub pAttributeV1: PCLAIM_SECURITY_ATTRIBUTE_V1,
}
#[cfg(feature = "basetsd")]
impl Default for CLAIM_SECURITY_ATTRIBUTES_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CLAIM_SECURITY_ATTRIBUTES_INFORMATION_VERSION: u32 = 1;
pub const CLAIM_SECURITY_ATTRIBUTES_INFORMATION_VERSION_V1: u32 = 1;
pub const CLAIM_SECURITY_ATTRIBUTE_CUSTOM_FLAGS: u32 = 4294901760;
pub const CLAIM_SECURITY_ATTRIBUTE_DISABLED: u32 = 16;
pub const CLAIM_SECURITY_ATTRIBUTE_DISABLED_BY_DEFAULT: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE {
    pub Version: u64,
    pub Name: windows_core::PWSTR,
}
pub const CLAIM_SECURITY_ATTRIBUTE_MANDATORY: u32 = 32;
pub const CLAIM_SECURITY_ATTRIBUTE_NON_INHERITABLE: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    pub pValue: *mut core::ffi::c_void,
    pub ValueLength: u32,
}
impl Default for CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1 {
    pub Name: u32,
    pub ValueType: u16,
    pub Reserved: u16,
    pub Flags: u32,
    pub ValueCount: u32,
    pub Values: CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1_0,
}
impl Default for CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1_0 {
    pub pInt64: [u32; 1],
    pub pUint64: [u32; 1],
    pub ppString: [u32; 1],
    pub pFqbn: [u32; 1],
    pub pOctetString: [u32; 1],
}
impl Default for CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_BOOLEAN: u32 = 6;
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_FQBN: u32 = 4;
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_INT64: u32 = 1;
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_INVALID: u32 = 0;
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_OCTET_STRING: u32 = 16;
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_SID: u32 = 5;
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_STRING: u32 = 3;
pub const CLAIM_SECURITY_ATTRIBUTE_TYPE_UINT64: u32 = 2;
pub const CLAIM_SECURITY_ATTRIBUTE_USE_FOR_DENY_ONLY: u32 = 4;
#[repr(C)]
#[cfg(feature = "basetsd")]
#[derive(Clone, Copy)]
pub struct CLAIM_SECURITY_ATTRIBUTE_V1 {
    pub Name: windows_core::PWSTR,
    pub ValueType: u16,
    pub Reserved: u16,
    pub Flags: u32,
    pub ValueCount: u32,
    pub Values: CLAIM_SECURITY_ATTRIBUTE_V1_0,
}
#[cfg(feature = "basetsd")]
impl Default for CLAIM_SECURITY_ATTRIBUTE_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "basetsd")]
#[derive(Clone, Copy)]
pub union CLAIM_SECURITY_ATTRIBUTE_V1_0 {
    pub pInt64: super::basetsd::PLONG64,
    pub pUint64: super::basetsd::PDWORD64,
    pub ppString: *mut windows_core::PWSTR,
    pub pFqbn: PCLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE,
    pub pOctetString: PCLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE,
}
#[cfg(feature = "basetsd")]
impl Default for CLAIM_SECURITY_ATTRIBUTE_V1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CLAIM_SECURITY_ATTRIBUTE_VALID_FLAGS: u32 = 63;
pub const CLAIM_SECURITY_ATTRIBUTE_VALUE_CASE_SENSITIVE: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CM_POWER_DATA {
    pub PD_Size: u32,
    pub PD_MostRecentPowerState: DEVICE_POWER_STATE,
    pub PD_Capabilities: u32,
    pub PD_D1Latency: u32,
    pub PD_D2Latency: u32,
    pub PD_D3Latency: u32,
    pub PD_PowerStateMapping: [DEVICE_POWER_STATE; 7],
    pub PD_DeepestSystemWake: SYSTEM_POWER_STATE,
}
impl Default for CM_POWER_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CM_SERVICE_MEASURED_BOOT_LOAD: u32 = 32;
pub const CM_SERVICE_NETWORK_BOOT_LOAD: u32 = 1;
pub const CM_SERVICE_RAM_DISK_BOOT_LOAD: u32 = 256;
pub const CM_SERVICE_SD_DISK_BOOT_LOAD: u32 = 8;
pub const CM_SERVICE_USB3_DISK_BOOT_LOAD: u32 = 16;
pub const CM_SERVICE_USB_DISK_BOOT_LOAD: u32 = 4;
pub const CM_SERVICE_VALID_PROMOTION_MASK: u32 = 511;
pub const CM_SERVICE_VERIFIER_BOOT_LOAD: u32 = 64;
pub const CM_SERVICE_VIRTUAL_DISK_BOOT_LOAD: u32 = 2;
pub const CM_SERVICE_WINPE_BOOT_LOAD: u32 = 128;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODE_INTEGRITY_REPORT_GENERATION_HEADER {
    pub Version: u16,
    pub Reserved: u16,
    pub RecordSize: u32,
    pub CommitTime: u64,
}
pub const CODE_INTEGRITY_REPORT_GENERATION_VERSION_CURRENT: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODE_INTEGRITY_REPORT_RECORD_HEADER {
    pub Version: u16,
    pub Reserved: u16,
    pub RecordSize: u32,
    pub SipaEventCode: u32,
}
pub const CODE_INTEGRITY_REPORT_RECORD_VERSION_CURRENT: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CODE_INTEGRITY_RUNTIME_REPORT {
    pub Header: RUNTIME_REPORT_HEADER,
    pub CurrentGeneration: u64,
    pub NumberOfGenerations: u32,
}
pub const COMIMAGE_FLAGS_32BITPREFERRED: ReplacesCorHdrNumericDefines = 131072;
pub const COMIMAGE_FLAGS_32BITREQUIRED: ReplacesCorHdrNumericDefines = 2;
pub const COMIMAGE_FLAGS_ILONLY: ReplacesCorHdrNumericDefines = 1;
pub const COMIMAGE_FLAGS_IL_LIBRARY: ReplacesCorHdrNumericDefines = 4;
pub const COMIMAGE_FLAGS_NATIVE_ENTRYPOINT: ReplacesCorHdrNumericDefines = 16;
pub const COMIMAGE_FLAGS_STRONGNAMESIGNED: ReplacesCorHdrNumericDefines = 8;
pub const COMIMAGE_FLAGS_TRACKDEBUGDATA: ReplacesCorHdrNumericDefines = 65536;
pub type COMPARTMENT_ID = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COMPATIBILITY_CONTEXT_ELEMENT {
    pub Id: windows_core::GUID,
    pub Type: ACTCTX_COMPATIBILITY_ELEMENT_TYPE,
    pub MaxVersionTested: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COMPONENT_FILTER {
    pub ComponentFlags: u32,
}
pub const COMPONENT_KTM: u32 = 1;
pub const COMPONENT_VALID_FLAGS: u32 = 1;
pub const COMPRESSION_ENGINE_HIBER: u32 = 512;
pub const COMPRESSION_ENGINE_MAXIMUM: u32 = 256;
pub const COMPRESSION_ENGINE_STANDARD: u32 = 0;
pub const COMPRESSION_FORMAT_DEFAULT: u32 = 1;
pub const COMPRESSION_FORMAT_DEFLATE: u32 = 7;
pub const COMPRESSION_FORMAT_LZ4: u32 = 6;
pub const COMPRESSION_FORMAT_LZNT1: u32 = 2;
pub const COMPRESSION_FORMAT_NONE: u32 = 0;
pub const COMPRESSION_FORMAT_XP10: u32 = 5;
pub const COMPRESSION_FORMAT_XPRESS: u32 = 3;
pub const COMPRESSION_FORMAT_XPRESS_HUFF: u32 = 4;
pub const COMPRESSION_FORMAT_ZLIB: u32 = 8;
pub const CONTAINER_INHERIT_ACE: u32 = 2;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CONTEXT {
    pub ContextFlags: u32,
    pub Dr0: u32,
    pub Dr1: u32,
    pub Dr2: u32,
    pub Dr3: u32,
    pub Dr6: u32,
    pub Dr7: u32,
    pub FloatSave: FLOATING_SAVE_AREA,
    pub SegGs: u32,
    pub SegFs: u32,
    pub SegEs: u32,
    pub SegDs: u32,
    pub Edi: u32,
    pub Esi: u32,
    pub Ebx: u32,
    pub Edx: u32,
    pub Ecx: u32,
    pub Eax: u32,
    pub Ebp: u32,
    pub Eip: u32,
    pub SegCs: u32,
    pub EFlags: u32,
    pub Esp: u32,
    pub SegSs: u32,
    pub ExtendedRegisters: [u8; 512],
}
#[cfg(target_arch = "x86")]
impl Default for CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct CONTEXT {
    pub P1Home: u64,
    pub P2Home: u64,
    pub P3Home: u64,
    pub P4Home: u64,
    pub P5Home: u64,
    pub P6Home: u64,
    pub ContextFlags: u32,
    pub MxCsr: u32,
    pub SegCs: u16,
    pub SegDs: u16,
    pub SegEs: u16,
    pub SegFs: u16,
    pub SegGs: u16,
    pub SegSs: u16,
    pub EFlags: u32,
    pub Dr0: u64,
    pub Dr1: u64,
    pub Dr2: u64,
    pub Dr3: u64,
    pub Dr6: u64,
    pub Dr7: u64,
    pub Rax: u64,
    pub Rcx: u64,
    pub Rdx: u64,
    pub Rbx: u64,
    pub Rsp: u64,
    pub Rbp: u64,
    pub Rsi: u64,
    pub Rdi: u64,
    pub R8: u64,
    pub R9: u64,
    pub R10: u64,
    pub R11: u64,
    pub R12: u64,
    pub R13: u64,
    pub R14: u64,
    pub R15: u64,
    pub Rip: u64,
    pub Anonymous: CONTEXT_0,
    pub VectorRegister: [M128A; 26],
    pub VectorControl: u64,
    pub DebugControl: u64,
    pub LastBranchToRip: u64,
    pub LastBranchFromRip: u64,
    pub LastExceptionToRip: u64,
    pub LastExceptionFromRip: u64,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union CONTEXT_0 {
    pub FltSave: XMM_SAVE_AREA32,
    pub Anonymous: CONTEXT_0_0,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for CONTEXT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CONTEXT_0_0 {
    pub Header: [M128A; 2],
    pub Legacy: [M128A; 8],
    pub Xmm0: M128A,
    pub Xmm1: M128A,
    pub Xmm2: M128A,
    pub Xmm3: M128A,
    pub Xmm4: M128A,
    pub Xmm5: M128A,
    pub Xmm6: M128A,
    pub Xmm7: M128A,
    pub Xmm8: M128A,
    pub Xmm9: M128A,
    pub Xmm10: M128A,
    pub Xmm11: M128A,
    pub Xmm12: M128A,
    pub Xmm13: M128A,
    pub Xmm14: M128A,
    pub Xmm15: M128A,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for CONTEXT_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "aarch64")]
pub type CONTEXT = ARM64_NT_CONTEXT;
#[cfg(target_arch = "x86")]
pub const CONTEXT_ALL: u32 = 65599;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const CONTEXT_ALL: u32 = 1048607;
#[cfg(target_arch = "aarch64")]
pub const CONTEXT_ALL: u32 = 4194527;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const CONTEXT_AMD64: u32 = 1048576;
pub const CONTEXT_ARM64: u32 = 4194304;
pub const CONTEXT_ARM64_ALL: u32 = 4194527;
pub const CONTEXT_ARM64_CONTROL: u32 = 4194305;
pub const CONTEXT_ARM64_DEBUG_REGISTERS: u32 = 4194312;
pub const CONTEXT_ARM64_FLOATING_POINT: u32 = 4194308;
pub const CONTEXT_ARM64_FLOATING_POINT_HIGH: u32 = 4194432;
pub const CONTEXT_ARM64_FLOATING_POINT_LOW: u32 = 4194368;
pub const CONTEXT_ARM64_FULL: u32 = 4194311;
pub const CONTEXT_ARM64_INTEGER: u32 = 4194306;
pub const CONTEXT_ARM64_RET_TO_GUEST: u32 = 67108864;
pub const CONTEXT_ARM64_UNWOUND_TO_CALL: u32 = 536870912;
pub const CONTEXT_ARM64_X18: u32 = 4194320;
pub const CONTEXT_ARM64_XSTATE: u32 = 4194336;
#[cfg(target_arch = "x86")]
pub const CONTEXT_CONTROL: u32 = 65537;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const CONTEXT_CONTROL: u32 = 1048577;
#[cfg(target_arch = "aarch64")]
pub const CONTEXT_CONTROL: u32 = 4194305;
#[cfg(target_arch = "x86")]
pub const CONTEXT_DEBUG_REGISTERS: u32 = 65552;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const CONTEXT_DEBUG_REGISTERS: u32 = 1048592;
#[cfg(target_arch = "aarch64")]
pub const CONTEXT_DEBUG_REGISTERS: u32 = 4194312;
pub const CONTEXT_EXCEPTION_ACTIVE: u32 = 134217728;
pub const CONTEXT_EXCEPTION_REPORTING: u32 = 2147483648;
pub const CONTEXT_EXCEPTION_REQUEST: u32 = 1073741824;
#[cfg(target_arch = "x86")]
pub const CONTEXT_EXTENDED_REGISTERS: u32 = 65568;
#[cfg(target_arch = "x86")]
pub const CONTEXT_FLOATING_POINT: u32 = 65544;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const CONTEXT_FLOATING_POINT: u32 = 1048584;
#[cfg(target_arch = "aarch64")]
pub const CONTEXT_FLOATING_POINT: u32 = 4194308;
#[cfg(target_arch = "x86")]
pub const CONTEXT_FULL: u32 = 65543;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const CONTEXT_FULL: u32 = 1048587;
#[cfg(target_arch = "aarch64")]
pub const CONTEXT_FULL: u32 = 4194311;
#[cfg(target_arch = "x86")]
pub const CONTEXT_INTEGER: u32 = 65538;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const CONTEXT_INTEGER: u32 = 1048578;
#[cfg(target_arch = "aarch64")]
pub const CONTEXT_INTEGER: u32 = 4194306;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const CONTEXT_KERNEL_CET: u32 = 1048704;
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
pub const CONTEXT_RET_TO_GUEST: u32 = 67108864;
#[cfg(target_arch = "x86")]
pub const CONTEXT_SEGMENTS: u32 = 65540;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const CONTEXT_SEGMENTS: u32 = 1048580;
pub const CONTEXT_SERVICE_ACTIVE: u32 = 268435456;
pub const CONTEXT_UNWOUND_TO_CALL: u32 = 536870912;
#[cfg(target_arch = "x86")]
pub const CONTEXT_XSTATE: u32 = 65600;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const CONTEXT_XSTATE: u32 = 1048640;
#[cfg(target_arch = "aarch64")]
pub const CONTEXT_XSTATE: u32 = 4194336;
#[cfg(target_arch = "x86")]
pub const CONTEXT_i386: u32 = 65536;
#[cfg(target_arch = "x86")]
pub const CONTEXT_i486: u32 = 65536;
pub const CORE_PARKING_POLICY_CHANGE_IDEAL: u32 = 0;
pub const CORE_PARKING_POLICY_CHANGE_MAX: u32 = 3;
pub const CORE_PARKING_POLICY_CHANGE_MULTISTEP: u32 = 3;
pub const CORE_PARKING_POLICY_CHANGE_ROCKET: u32 = 2;
pub const CORE_PARKING_POLICY_CHANGE_SINGLE: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CORRELATION_VECTOR {
    pub Version: i8,
    pub Vector: [i8; 129],
}
impl Default for CORRELATION_VECTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const COR_DELETED_NAME_LENGTH: ReplacesCorHdrNumericDefines = 8;
pub const COR_ILMETHOD_SECT_SMALL_MAX_DATASIZE: ReplacesCorHdrNumericDefines = 255;
pub const COR_VERSION_MAJOR: ReplacesCorHdrNumericDefines = 2;
pub const COR_VERSION_MAJOR_V2: ReplacesCorHdrNumericDefines = 2;
pub const COR_VERSION_MINOR: ReplacesCorHdrNumericDefines = 5;
pub const COR_VTABLEGAP_NAME_LENGTH: ReplacesCorHdrNumericDefines = 8;
pub const COR_VTABLE_32BIT: ReplacesCorHdrNumericDefines = 1;
pub const COR_VTABLE_64BIT: ReplacesCorHdrNumericDefines = 2;
pub const COR_VTABLE_CALL_MOST_DERIVED: ReplacesCorHdrNumericDefines = 16;
pub const COR_VTABLE_FROM_UNMANAGED: ReplacesCorHdrNumericDefines = 4;
pub const COR_VTABLE_FROM_UNMANAGED_RETAIN_APPDOMAIN: ReplacesCorHdrNumericDefines = 8;
pub type CPU_SET_INFORMATION_TYPE = i32;
pub const CREATE_BOUNDARY_DESCRIPTOR_ADD_APPCONTAINER_SID: u32 = 1;
pub const CRITICAL_ACE_FLAG: u32 = 32;
pub const CTMF_INCLUDE_APPCONTAINER: u32 = 1;
pub const CTMF_INCLUDE_LPAC: u32 = 2;
pub const CTMF_VALID_FLAGS: u32 = 3;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG {
    pub Size: u32,
    pub TriggerId: windows_core::PCWSTR,
}
pub const CacheData: PROCESSOR_CACHE_TYPE = 2;
pub const CacheInstruction: PROCESSOR_CACHE_TYPE = 1;
pub const CacheTrace: PROCESSOR_CACHE_TYPE = 3;
pub const CacheUnified: PROCESSOR_CACHE_TYPE = 0;
pub const CacheUnknown: PROCESSOR_CACHE_TYPE = 4;
pub const CompatibilityInformationInActivationContext: ACTIVATION_CONTEXT_INFO_CLASS = 6;
pub const CpuSetInformation: CPU_SET_INFORMATION_TYPE = 0;
pub const CriticalError: SERVICE_ERROR_TYPE = 3;
pub const CsDeviceNotification: POWER_INFORMATION_LEVEL = 74;
pub const DACL_SECURITY_INFORMATION: u32 = 4;
pub const DBG_COMMAND_EXCEPTION: u32 = 1073807369;
pub const DBG_CONTINUE: u32 = 65538;
pub const DBG_CONTROL_BREAK: u32 = 1073807368;
pub const DBG_CONTROL_C: u32 = 1073807365;
pub const DBG_EXCEPTION_HANDLED: u32 = 65537;
pub const DBG_EXCEPTION_NOT_HANDLED: u32 = 2147549185;
pub const DBG_PRINTEXCEPTION_C: u32 = 1073807366;
pub const DBG_PRINTEXCEPTION_WIDE_C: u32 = 1073807370;
pub const DBG_REPLY_LATER: u32 = 1073807361;
pub const DBG_RIPEXCEPTION: u32 = 1073807367;
pub const DBG_TERMINATE_PROCESS: u32 = 1073807364;
pub const DBG_TERMINATE_THREAD: u32 = 1073807363;
pub const DEDICATED_MEMORY_CACHE_ELIGIBLE: u32 = 1;
pub const DEFAULT_COMPARTMENT_ID: COMPARTMENT_ID = 1;
pub const DEFAULT_IMPERSONATION_LEVEL: u32 = 2;
pub const DELETE: u32 = 65536;
pub const DEVICEFAMILYDEVICEFORM_ALLINONE: u32 = 7;
pub const DEVICEFAMILYDEVICEFORM_BANKING: u32 = 14;
pub const DEVICEFAMILYDEVICEFORM_BUILDING_AUTOMATION: u32 = 15;
pub const DEVICEFAMILYDEVICEFORM_CONVERTIBLE: u32 = 5;
pub const DEVICEFAMILYDEVICEFORM_DESKTOP: u32 = 3;
pub const DEVICEFAMILYDEVICEFORM_DETACHABLE: u32 = 6;
pub const DEVICEFAMILYDEVICEFORM_DIGITAL_SIGNAGE: u32 = 16;
pub const DEVICEFAMILYDEVICEFORM_GAMING: u32 = 17;
pub const DEVICEFAMILYDEVICEFORM_GAMING_CONSOLE: u32 = 47;
pub const DEVICEFAMILYDEVICEFORM_GAMING_HANDHELD: u32 = 46;
pub const DEVICEFAMILYDEVICEFORM_HMD: u32 = 11;
pub const DEVICEFAMILYDEVICEFORM_HOME_AUTOMATION: u32 = 18;
pub const DEVICEFAMILYDEVICEFORM_INDUSTRIAL_AUTOMATION: u32 = 19;
pub const DEVICEFAMILYDEVICEFORM_INDUSTRY_HANDHELD: u32 = 12;
pub const DEVICEFAMILYDEVICEFORM_INDUSTRY_OTHER: u32 = 29;
pub const DEVICEFAMILYDEVICEFORM_INDUSTRY_TABLET: u32 = 13;
pub const DEVICEFAMILYDEVICEFORM_KEY: windows_core::PCWSTR = windows_core::w!("\\Registry\\Machine\\Software\\Microsoft\\Windows NT\\CurrentVersion\\OEM");
pub const DEVICEFAMILYDEVICEFORM_KIOSK: u32 = 20;
pub const DEVICEFAMILYDEVICEFORM_LARGESCREEN: u32 = 10;
pub const DEVICEFAMILYDEVICEFORM_MAKER_BOARD: u32 = 21;
pub const DEVICEFAMILYDEVICEFORM_MAX: u32 = 47;
pub const DEVICEFAMILYDEVICEFORM_MEDICAL: u32 = 22;
pub const DEVICEFAMILYDEVICEFORM_NETWORKING: u32 = 23;
pub const DEVICEFAMILYDEVICEFORM_NOTEBOOK: u32 = 4;
pub const DEVICEFAMILYDEVICEFORM_PHONE: u32 = 1;
pub const DEVICEFAMILYDEVICEFORM_POINT_OF_SERVICE: u32 = 24;
pub const DEVICEFAMILYDEVICEFORM_PRINTING: u32 = 25;
pub const DEVICEFAMILYDEVICEFORM_PUCK: u32 = 9;
pub const DEVICEFAMILYDEVICEFORM_STICKPC: u32 = 8;
pub const DEVICEFAMILYDEVICEFORM_TABLET: u32 = 2;
pub const DEVICEFAMILYDEVICEFORM_THIN_CLIENT: u32 = 26;
pub const DEVICEFAMILYDEVICEFORM_TOY: u32 = 27;
pub const DEVICEFAMILYDEVICEFORM_UNKNOWN: u32 = 0;
pub const DEVICEFAMILYDEVICEFORM_VALUE: windows_core::PCWSTR = windows_core::w!("DeviceForm");
pub const DEVICEFAMILYDEVICEFORM_VENDING: u32 = 28;
pub const DEVICEFAMILYDEVICEFORM_XBOX_ONE: u32 = 30;
pub const DEVICEFAMILYDEVICEFORM_XBOX_ONE_S: u32 = 31;
pub const DEVICEFAMILYDEVICEFORM_XBOX_ONE_X: u32 = 32;
pub const DEVICEFAMILYDEVICEFORM_XBOX_ONE_X_DEVKIT: u32 = 33;
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_01: u32 = 37;
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_02: u32 = 38;
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_03: u32 = 39;
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_04: u32 = 40;
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_05: u32 = 41;
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_06: u32 = 42;
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_07: u32 = 43;
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_08: u32 = 44;
pub const DEVICEFAMILYDEVICEFORM_XBOX_RESERVED_09: u32 = 45;
pub const DEVICEFAMILYDEVICEFORM_XBOX_SERIES_S: u32 = 36;
pub const DEVICEFAMILYDEVICEFORM_XBOX_SERIES_X: u32 = 34;
pub const DEVICEFAMILYDEVICEFORM_XBOX_SERIES_X_DEVKIT: u32 = 35;
pub const DEVICEFAMILYINFOENUM_7067329: u32 = 15;
pub const DEVICEFAMILYINFOENUM_8828080: u32 = 14;
pub const DEVICEFAMILYINFOENUM_DESKTOP: u32 = 3;
pub const DEVICEFAMILYINFOENUM_HOLOGRAPHIC: u32 = 10;
pub const DEVICEFAMILYINFOENUM_IOT: u32 = 7;
pub const DEVICEFAMILYINFOENUM_IOT_HEADLESS: u32 = 8;
pub const DEVICEFAMILYINFOENUM_MAX: u32 = 17;
pub const DEVICEFAMILYINFOENUM_MOBILE: u32 = 4;
pub const DEVICEFAMILYINFOENUM_SERVER: u32 = 9;
pub const DEVICEFAMILYINFOENUM_SERVER_NANO: u32 = 13;
pub const DEVICEFAMILYINFOENUM_TEAM: u32 = 6;
pub const DEVICEFAMILYINFOENUM_UAP: u32 = 0;
pub const DEVICEFAMILYINFOENUM_WINDOWS_8X: u32 = 1;
pub const DEVICEFAMILYINFOENUM_WINDOWS_CORE: u32 = 16;
pub const DEVICEFAMILYINFOENUM_WINDOWS_CORE_HEADLESS: u32 = 17;
pub const DEVICEFAMILYINFOENUM_WINDOWS_PHONE_8X: u32 = 2;
pub const DEVICEFAMILYINFOENUM_XBOX: u32 = 5;
pub const DEVICEFAMILYINFOENUM_XBOXERA: u32 = 12;
pub const DEVICEFAMILYINFOENUM_XBOXSRA: u32 = 11;
pub type DEVICE_POWER_STATE = i32;
pub const DIAGNOSTIC_REASON_DETAILED_STRING: u32 = 2;
pub const DIAGNOSTIC_REASON_INVALID_FLAGS: u32 = 2147483640;
pub const DIAGNOSTIC_REASON_NOT_SPECIFIED: u32 = 2147483648;
pub const DIAGNOSTIC_REASON_SIMPLE_STRING: u32 = 1;
pub const DIAGNOSTIC_REASON_VERSION: u32 = 0;
pub const DISABLE_MAX_PRIVILEGE: u32 = 1;
pub const DISCHARGE_POLICY_CRITICAL: u32 = 0;
pub const DISCHARGE_POLICY_LOW: u32 = 1;
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "excpt")]
#[derive(Clone, Copy, Debug)]
pub struct DISPATCHER_CONTEXT {
    pub ControlPc: u64,
    pub ImageBase: u64,
    pub FunctionEntry: PRUNTIME_FUNCTION,
    pub EstablisherFrame: u64,
    pub TargetIp: u64,
    pub ContextRecord: PCONTEXT,
    pub LanguageHandler: PEXCEPTION_ROUTINE,
    pub HandlerData: *mut core::ffi::c_void,
    pub HistoryTable: *mut UNWIND_HISTORY_TABLE,
    pub ScopeIndex: u32,
    pub Fill0: u32,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "excpt")]
impl Default for DISPATCHER_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "aarch64")]
#[cfg(all(feature = "excpt", feature = "minwindef"))]
pub type DISPATCHER_CONTEXT = DISPATCHER_CONTEXT_ARM64;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "excpt", feature = "minwindef"))]
#[derive(Clone, Copy, Debug)]
pub struct DISPATCHER_CONTEXT_ARM64 {
    pub ControlPc: usize,
    pub ImageBase: usize,
    pub FunctionEntry: PARM64_RUNTIME_FUNCTION,
    pub EstablisherFrame: usize,
    pub TargetPc: usize,
    pub ContextRecord: PARM64_NT_CONTEXT,
    pub LanguageHandler: PEXCEPTION_ROUTINE,
    pub HandlerData: *mut core::ffi::c_void,
    pub HistoryTable: *mut _UNWIND_HISTORY_TABLE,
    pub ScopeIndex: u32,
    pub ControlPcIsUnwound: bool,
    pub NonVolatileRegisters: super::minwindef::PBYTE,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "excpt", feature = "minwindef"))]
impl Default for DISPATCHER_CONTEXT_ARM64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "excpt", feature = "minwindef"))]
#[derive(Clone, Copy, Debug)]
pub struct DISPATCHER_CONTEXT_ARM64 {
    pub ControlPc: usize,
    pub ImageBase: usize,
    pub FunctionEntry: PARM64_RUNTIME_FUNCTION,
    pub EstablisherFrame: usize,
    pub TargetPc: usize,
    pub ContextRecord: PARM64_NT_CONTEXT,
    pub LanguageHandler: PEXCEPTION_ROUTINE,
    pub HandlerData: *mut core::ffi::c_void,
    pub HistoryTable: *mut UNWIND_HISTORY_TABLE,
    pub ScopeIndex: u32,
    pub ControlPcIsUnwound: bool,
    pub NonVolatileRegisters: super::minwindef::PBYTE,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "excpt", feature = "minwindef"))]
impl Default for DISPATCHER_CONTEXT_ARM64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DISPATCHER_CONTEXT_NONVOLREG_ARM64 {
    pub Buffer: [u8; 152],
    pub Anonymous: DISPATCHER_CONTEXT_NONVOLREG_ARM64_0,
}
impl Default for DISPATCHER_CONTEXT_NONVOLREG_ARM64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DISPATCHER_CONTEXT_NONVOLREG_ARM64_0 {
    pub GpNvRegs: [u64; 11],
    pub FpNvRegs: [f64; 8],
}
impl Default for DISPATCHER_CONTEXT_NONVOLREG_ARM64_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DLL_PROCESS_ATTACH: u32 = 1;
pub const DLL_PROCESS_DETACH: u32 = 0;
pub const DLL_THREAD_ATTACH: u32 = 2;
pub const DLL_THREAD_DETACH: u32 = 3;
pub const DOMAIN_ALIAS_RID_ACCESS_CONTROL_ASSISTANCE_OPS: u32 = 579;
pub const DOMAIN_ALIAS_RID_ACCOUNT_OPS: u32 = 548;
pub const DOMAIN_ALIAS_RID_ADMINS: u32 = 544;
pub const DOMAIN_ALIAS_RID_AUTHORIZATIONACCESS: u32 = 560;
pub const DOMAIN_ALIAS_RID_BACKUP_OPS: u32 = 551;
pub const DOMAIN_ALIAS_RID_CACHEABLE_PRINCIPALS_GROUP: u32 = 571;
pub const DOMAIN_ALIAS_RID_CERTSVC_DCOM_ACCESS_GROUP: u32 = 574;
pub const DOMAIN_ALIAS_RID_CRYPTO_OPERATORS: u32 = 569;
pub const DOMAIN_ALIAS_RID_CUA_USERS: u32 = 586;
pub const DOMAIN_ALIAS_RID_DCOM_USERS: u32 = 562;
pub const DOMAIN_ALIAS_RID_DEFAULT_ACCOUNT: u32 = 581;
pub const DOMAIN_ALIAS_RID_DEVICE_OWNERS: u32 = 583;
pub const DOMAIN_ALIAS_RID_EVENT_LOG_READERS_GROUP: u32 = 573;
pub const DOMAIN_ALIAS_RID_GUESTS: u32 = 546;
pub const DOMAIN_ALIAS_RID_HYPER_V_ADMINS: u32 = 578;
pub const DOMAIN_ALIAS_RID_INCOMING_FOREST_TRUST_BUILDERS: u32 = 557;
pub const DOMAIN_ALIAS_RID_IUSERS: u32 = 568;
pub const DOMAIN_ALIAS_RID_LOGGING_USERS: u32 = 559;
pub const DOMAIN_ALIAS_RID_MONITORING_USERS: u32 = 558;
pub const DOMAIN_ALIAS_RID_NETWORK_CONFIGURATION_OPS: u32 = 556;
pub const DOMAIN_ALIAS_RID_NON_CACHEABLE_PRINCIPALS_GROUP: u32 = 572;
pub const DOMAIN_ALIAS_RID_OPENSSH_USERS: u32 = 585;
pub const DOMAIN_ALIAS_RID_POWER_USERS: u32 = 547;
pub const DOMAIN_ALIAS_RID_PREW2KCOMPACCESS: u32 = 554;
pub const DOMAIN_ALIAS_RID_PRINT_OPS: u32 = 550;
pub const DOMAIN_ALIAS_RID_RAS_SERVERS: u32 = 553;
pub const DOMAIN_ALIAS_RID_RDS_ENDPOINT_SERVERS: u32 = 576;
pub const DOMAIN_ALIAS_RID_RDS_MANAGEMENT_SERVERS: u32 = 577;
pub const DOMAIN_ALIAS_RID_RDS_REMOTE_ACCESS_SERVERS: u32 = 575;
pub const DOMAIN_ALIAS_RID_REMOTE_DESKTOP_USERS: u32 = 555;
pub const DOMAIN_ALIAS_RID_REMOTE_MANAGEMENT_USERS: u32 = 580;
pub const DOMAIN_ALIAS_RID_REPLICATOR: u32 = 552;
pub const DOMAIN_ALIAS_RID_STORAGE_REPLICA_ADMINS: u32 = 582;
pub const DOMAIN_ALIAS_RID_SYSTEM_OPS: u32 = 549;
pub const DOMAIN_ALIAS_RID_TS_LICENSE_SERVERS: u32 = 561;
pub const DOMAIN_ALIAS_RID_USERS: u32 = 545;
pub const DOMAIN_ALIAS_RID_USER_MODE_HARDWARE_OPERATORS: u32 = 584;
pub const DOMAIN_GROUP_RID_ADMINS: u32 = 512;
pub const DOMAIN_GROUP_RID_AUTHORIZATION_DATA_CONTAINS_CLAIMS: u32 = 497;
pub const DOMAIN_GROUP_RID_AUTHORIZATION_DATA_IS_COMPOUNDED: u32 = 496;
pub const DOMAIN_GROUP_RID_CDC_RESERVED: u32 = 524;
pub const DOMAIN_GROUP_RID_CERT_ADMINS: u32 = 517;
pub const DOMAIN_GROUP_RID_CLONEABLE_CONTROLLERS: u32 = 522;
pub const DOMAIN_GROUP_RID_COMPUTERS: u32 = 515;
pub const DOMAIN_GROUP_RID_CONTROLLERS: u32 = 516;
pub const DOMAIN_GROUP_RID_ENTERPRISE_ADMINS: u32 = 519;
pub const DOMAIN_GROUP_RID_ENTERPRISE_KEY_ADMINS: u32 = 527;
pub const DOMAIN_GROUP_RID_ENTERPRISE_READONLY_DOMAIN_CONTROLLERS: u32 = 498;
pub const DOMAIN_GROUP_RID_EXTERNAL_TRUSTS: u32 = 529;
pub const DOMAIN_GROUP_RID_FOREST_TRUSTS: u32 = 528;
pub const DOMAIN_GROUP_RID_GUESTS: u32 = 514;
pub const DOMAIN_GROUP_RID_KEY_ADMINS: u32 = 526;
pub const DOMAIN_GROUP_RID_POLICY_ADMINS: u32 = 520;
pub const DOMAIN_GROUP_RID_PROTECTED_USERS: u32 = 525;
pub const DOMAIN_GROUP_RID_READONLY_CONTROLLERS: u32 = 521;
pub const DOMAIN_GROUP_RID_SCHEMA_ADMINS: u32 = 518;
pub const DOMAIN_GROUP_RID_USERS: u32 = 513;
pub const DOMAIN_USER_RID_ADMIN: u32 = 500;
pub const DOMAIN_USER_RID_DEFAULT_ACCOUNT: u32 = 503;
pub const DOMAIN_USER_RID_GUEST: u32 = 501;
pub const DOMAIN_USER_RID_KRBTGT: u32 = 502;
pub const DOMAIN_USER_RID_MAX: u32 = 999;
pub const DOMAIN_USER_RID_WDAG_ACCOUNT: u32 = 504;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DRIVER_INFO_ENTRY {
    pub InternalName: [i8; 32],
    pub ImageHashAlgorithm: u16,
    pub PublisherThumbprintHashAlgorithm: u16,
    pub ImageHashOffset: u32,
    pub PublisherThumbprintOffset: u32,
    pub NumberOfLoadingTimes: u16,
    pub OemNameSize: u16,
    pub OemNameOffset: u32,
    pub Flags: DRIVER_INFO_ENTRY_0,
    pub Padding: u16,
}
impl Default for DRIVER_INFO_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DRIVER_INFO_ENTRY_0 {
    pub Anonymous: DRIVER_INFO_ENTRY_0_0,
    pub AsUInt16: u16,
}
impl Default for DRIVER_INFO_ENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DRIVER_INFO_ENTRY_0_0 {
    pub _bitfield: u16,
}
pub const DRIVER_REPORT_DIGEST_MAX_SIZE: u32 = 64;
pub const DRIVER_REPORT_NAME_MAX_LENGTH: u32 = 32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DRIVER_RUNTIME_REPORT {
    pub Header: RUNTIME_REPORT_HEADER,
    pub NumberOfDrivers: u16,
    pub Flags: DRIVER_RUNTIME_REPORT_0,
    pub DriverEntries: [DRIVER_INFO_ENTRY; 1],
}
impl Default for DRIVER_RUNTIME_REPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DRIVER_RUNTIME_REPORT_0 {
    pub Anonymous: DRIVER_RUNTIME_REPORT_0_0,
    pub AsUInt16: u16,
}
impl Default for DRIVER_RUNTIME_REPORT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DRIVER_RUNTIME_REPORT_0_0 {
    pub _bitfield: u16,
}
pub const DUPLICATE_CLOSE_SOURCE: u32 = 1;
pub const DUPLICATE_SAME_ACCESS: u32 = 2;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DWORDLONG(pub u64);
pub const DYNAMIC_EH_CONTINUATION_TARGET_ADD: u32 = 1;
pub const DYNAMIC_EH_CONTINUATION_TARGET_PROCESSED: u32 = 2;
pub const DYNAMIC_ENFORCED_ADDRESS_RANGE_ADD: u32 = 1;
pub const DYNAMIC_ENFORCED_ADDRESS_RANGE_PROCESSED: u32 = 2;
pub const DemandLoad: SERVICE_LOAD_TYPE = 3;
pub const DisableLoad: SERVICE_LOAD_TYPE = 4;
pub const DisplayBurst: POWER_INFORMATION_LEVEL = 77;
pub const DriverType: SERVICE_NODE_TYPE = 1;
pub const EMARCH_ENC_I17_IC_INST_WORD_POS_X: u32 = 12;
pub const EMARCH_ENC_I17_IC_INST_WORD_X: u32 = 3;
pub const EMARCH_ENC_I17_IC_SIZE_X: u32 = 1;
pub const EMARCH_ENC_I17_IC_VAL_POS_X: u32 = 21;
pub const EMARCH_ENC_I17_IMM41a_INST_WORD_POS_X: u32 = 14;
pub const EMARCH_ENC_I17_IMM41a_INST_WORD_X: u32 = 1;
pub const EMARCH_ENC_I17_IMM41a_SIZE_X: u32 = 10;
pub const EMARCH_ENC_I17_IMM41a_VAL_POS_X: u32 = 22;
pub const EMARCH_ENC_I17_IMM41b_INST_WORD_POS_X: u32 = 24;
pub const EMARCH_ENC_I17_IMM41b_INST_WORD_X: u32 = 1;
pub const EMARCH_ENC_I17_IMM41b_SIZE_X: u32 = 8;
pub const EMARCH_ENC_I17_IMM41b_VAL_POS_X: u32 = 32;
pub const EMARCH_ENC_I17_IMM41c_INST_WORD_POS_X: u32 = 0;
pub const EMARCH_ENC_I17_IMM41c_INST_WORD_X: u32 = 2;
pub const EMARCH_ENC_I17_IMM41c_SIZE_X: u32 = 23;
pub const EMARCH_ENC_I17_IMM41c_VAL_POS_X: u32 = 40;
pub const EMARCH_ENC_I17_IMM5C_INST_WORD_POS_X: u32 = 13;
pub const EMARCH_ENC_I17_IMM5C_INST_WORD_X: u32 = 3;
pub const EMARCH_ENC_I17_IMM5C_SIZE_X: u32 = 5;
pub const EMARCH_ENC_I17_IMM5C_VAL_POS_X: u32 = 16;
pub const EMARCH_ENC_I17_IMM7B_INST_WORD_POS_X: u32 = 4;
pub const EMARCH_ENC_I17_IMM7B_INST_WORD_X: u32 = 3;
pub const EMARCH_ENC_I17_IMM7B_SIZE_X: u32 = 7;
pub const EMARCH_ENC_I17_IMM7B_VAL_POS_X: u32 = 0;
pub const EMARCH_ENC_I17_IMM9D_INST_WORD_POS_X: u32 = 18;
pub const EMARCH_ENC_I17_IMM9D_INST_WORD_X: u32 = 3;
pub const EMARCH_ENC_I17_IMM9D_SIZE_X: u32 = 9;
pub const EMARCH_ENC_I17_IMM9D_VAL_POS_X: u32 = 7;
pub const EMARCH_ENC_I17_SIGN_INST_WORD_POS_X: u32 = 27;
pub const EMARCH_ENC_I17_SIGN_INST_WORD_X: u32 = 3;
pub const EMARCH_ENC_I17_SIGN_SIZE_X: u32 = 1;
pub const EMARCH_ENC_I17_SIGN_VAL_POS_X: u32 = 63;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ENCLAVE_CREATE_INFO_SGX {
    pub Secs: [u8; 4096],
}
impl Default for ENCLAVE_CREATE_INFO_SGX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ENCLAVE_CREATE_INFO_VBS {
    pub Flags: u32,
    pub OwnerID: [u8; 32],
}
impl Default for ENCLAVE_CREATE_INFO_VBS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ENCLAVE_CREATE_INFO_VBS_BASIC {
    pub Flags: u32,
    pub OwnerID: [u8; 32],
}
impl Default for ENCLAVE_CREATE_INFO_VBS_BASIC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ENCLAVE_INIT_INFO_SGX {
    pub SigStruct: [u8; 1808],
    pub Reserved1: [u8; 240],
    pub EInitToken: [u8; 304],
    pub Reserved2: [u8; 1744],
}
impl Default for ENCLAVE_INIT_INFO_SGX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ENCLAVE_INIT_INFO_VBS {
    pub Length: u32,
    pub ThreadCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ENCLAVE_INIT_INFO_VBS_BASIC {
    pub FamilyId: [u8; 16],
    pub ImageId: [u8; 16],
    pub EnclaveSize: u64,
    pub EnclaveSvn: u32,
    pub Reserved: u32,
    pub Anonymous: ENCLAVE_INIT_INFO_VBS_BASIC_0,
}
impl Default for ENCLAVE_INIT_INFO_VBS_BASIC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union ENCLAVE_INIT_INFO_VBS_BASIC_0 {
    pub SignatureInfoHandle: HANDLE,
    pub Unused: u64,
}
impl Default for ENCLAVE_INIT_INFO_VBS_BASIC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ENCLAVE_LOAD_DATA_VBS_BASIC {
    pub PageType: u32,
}
pub const ENCLAVE_LONG_ID_LENGTH: u32 = 32;
pub const ENCLAVE_SHORT_ID_LENGTH: u32 = 16;
pub type ENCLAVE_TARGET_FUNCTION = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void) -> *mut core::ffi::c_void>;
pub const ENCLAVE_TYPE_SGX: u32 = 1;
pub const ENCLAVE_TYPE_SGX2: u32 = 2;
pub const ENCLAVE_TYPE_VBS: u32 = 16;
pub const ENCLAVE_TYPE_VBS_BASIC: u32 = 17;
pub const ENCLAVE_VBS_FLAG_DEBUG: u32 = 1;
pub const ENERGY_SAVER_HIGH_SAVINGS: ENERGY_SAVER_STATUS = 2;
pub const ENERGY_SAVER_OFF: ENERGY_SAVER_STATUS = 0;
pub const ENERGY_SAVER_STANDARD: ENERGY_SAVER_STATUS = 1;
pub type ENERGY_SAVER_STATUS = i32;
pub const ENLISTMENT_ALL_ACCESS: u32 = 983071;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ENLISTMENT_BASIC_INFORMATION {
    pub EnlistmentId: windows_core::GUID,
    pub TransactionId: windows_core::GUID,
    pub ResourceManagerId: windows_core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ENLISTMENT_CRM_INFORMATION {
    pub CrmTransactionManagerId: windows_core::GUID,
    pub CrmResourceManagerId: windows_core::GUID,
    pub CrmEnlistmentId: windows_core::GUID,
}
pub const ENLISTMENT_GENERIC_EXECUTE: u32 = 131100;
pub const ENLISTMENT_GENERIC_READ: u32 = 131073;
pub const ENLISTMENT_GENERIC_WRITE: u32 = 131102;
pub type ENLISTMENT_INFORMATION_CLASS = i32;
pub const ENLISTMENT_QUERY_INFORMATION: u32 = 1;
pub const ENLISTMENT_RECOVER: u32 = 4;
pub const ENLISTMENT_SET_INFORMATION: u32 = 2;
pub const ENLISTMENT_SUBORDINATE_RIGHTS: u32 = 8;
pub const ENLISTMENT_SUPERIOR_RIGHTS: u32 = 16;
pub const ERROR_SEVERITY_ERROR: u32 = 3221225472;
pub const ERROR_SEVERITY_INFORMATIONAL: u32 = 1073741824;
pub const ERROR_SEVERITY_SUCCESS: u32 = 0;
pub const ERROR_SEVERITY_WARNING: u32 = 2147483648;
pub const ES_AWAYMODE_REQUIRED: u32 = 64;
pub const ES_CONTINUOUS: u32 = 2147483648;
pub const ES_DISPLAY_REQUIRED: u32 = 2;
pub const ES_SYSTEM_REQUIRED: u32 = 1;
pub const ES_USER_PRESENT: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EVENTLOGRECORD {
    pub Length: u32,
    pub Reserved: u32,
    pub RecordNumber: u32,
    pub TimeGenerated: u32,
    pub TimeWritten: u32,
    pub EventID: u32,
    pub EventType: u16,
    pub NumStrings: u16,
    pub EventCategory: u16,
    pub ReservedFlags: u16,
    pub ClosingRecordNumber: u32,
    pub StringOffset: u32,
    pub UserSidLength: u32,
    pub UserSidOffset: u32,
    pub DataLength: u32,
    pub DataOffset: u32,
}
pub const EVENTLOG_AUDIT_FAILURE: u32 = 16;
pub const EVENTLOG_AUDIT_SUCCESS: u32 = 8;
pub const EVENTLOG_BACKWARDS_READ: u32 = 8;
pub const EVENTLOG_END_ALL_PAIRED_EVENTS: u32 = 4;
pub const EVENTLOG_END_PAIRED_EVENT: u32 = 2;
pub const EVENTLOG_ERROR_TYPE: u32 = 1;
pub const EVENTLOG_FORWARDS_READ: u32 = 4;
pub const EVENTLOG_INFORMATION_TYPE: u32 = 4;
pub const EVENTLOG_PAIRED_EVENT_ACTIVE: u32 = 8;
pub const EVENTLOG_PAIRED_EVENT_INACTIVE: u32 = 16;
pub const EVENTLOG_SEEK_READ: u32 = 2;
pub const EVENTLOG_SEQUENTIAL_READ: u32 = 1;
pub const EVENTLOG_START_PAIRED_EVENT: u32 = 1;
pub const EVENTLOG_SUCCESS: u32 = 0;
pub const EVENTLOG_WARNING_TYPE: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EVENTSFORLOGFILE {
    pub ulSize: u32,
    pub szLogicalLogFile: [u16; 256],
    pub ulNumRecords: u32,
    pub pEventLogRecords: [EVENTLOGRECORD; 0],
}
impl Default for EVENTSFORLOGFILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EVENT_ALL_ACCESS: u32 = 2031619;
pub const EVENT_MODIFY_STATE: u32 = 2;
pub const EXCEPTION_COLLIDED_UNWIND: u32 = 64;
pub const EXCEPTION_EXECUTE_FAULT: u32 = 8;
pub const EXCEPTION_EXIT_UNWIND: u32 = 4;
pub const EXCEPTION_MAXIMUM_PARAMETERS: u32 = 15;
pub const EXCEPTION_NESTED_CALL: u32 = 16;
pub const EXCEPTION_NONCONTINUABLE: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EXCEPTION_POINTERS {
    pub ExceptionRecord: PEXCEPTION_RECORD,
    pub ContextRecord: PCONTEXT,
}
pub const EXCEPTION_READ_FAULT: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EXCEPTION_RECORD {
    pub ExceptionCode: u32,
    pub ExceptionFlags: u32,
    pub ExceptionRecord: *mut Self,
    pub ExceptionAddress: *mut core::ffi::c_void,
    pub NumberParameters: u32,
    pub ExceptionInformation: [usize; 15],
}
impl Default for EXCEPTION_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EXCEPTION_RECORD32 {
    pub ExceptionCode: u32,
    pub ExceptionFlags: u32,
    pub ExceptionRecord: u32,
    pub ExceptionAddress: u32,
    pub NumberParameters: u32,
    pub ExceptionInformation: [u32; 15],
}
impl Default for EXCEPTION_RECORD32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EXCEPTION_RECORD64 {
    pub ExceptionCode: u32,
    pub ExceptionFlags: u32,
    pub ExceptionRecord: u64,
    pub ExceptionAddress: u64,
    pub NumberParameters: u32,
    pub __unusedAlignment: u32,
    pub ExceptionInformation: [u64; 15],
}
impl Default for EXCEPTION_RECORD64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "excpt")]
#[derive(Clone, Copy, Debug)]
pub struct EXCEPTION_REGISTRATION_RECORD {
    pub Next: *mut Self,
    pub Handler: PEXCEPTION_ROUTINE,
}
#[cfg(feature = "excpt")]
impl Default for EXCEPTION_REGISTRATION_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[cfg(feature = "excpt")]
pub type EXCEPTION_ROUTINE = Option<unsafe extern "system" fn(exceptionrecord: *mut EXCEPTION_RECORD, establisherframe: *const core::ffi::c_void, contextrecord: *mut CONTEXT, dispatchercontext: *const core::ffi::c_void) -> super::excpt::EXCEPTION_DISPOSITION>;
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "excpt")]
pub type EXCEPTION_ROUTINE = Option<unsafe extern "system" fn(exceptionrecord: *mut EXCEPTION_RECORD, establisherframe: *const core::ffi::c_void, contextrecord: *mut ARM64_NT_CONTEXT, dispatchercontext: *const core::ffi::c_void) -> super::excpt::EXCEPTION_DISPOSITION>;
pub const EXCEPTION_SOFTWARE_ORIGINATE: u32 = 128;
pub const EXCEPTION_STACK_INVALID: u32 = 8;
pub const EXCEPTION_TARGET_UNWIND: u32 = 32;
pub const EXCEPTION_UNWIND: u32 = 102;
pub const EXCEPTION_UNWINDING: u32 = 2;
pub const EXCEPTION_WRITE_FAULT: u32 = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct EXECUTION_STATE(pub u32);
pub const EndpointParamRegNone: IMAGE_HOTSWAP_X64_ENDPOINT_INFO_CC_REG = 0;
pub const EndpointParamRegR8: IMAGE_HOTSWAP_X64_ENDPOINT_INFO_CC_REG = 9;
pub const EndpointParamRegR9: IMAGE_HOTSWAP_X64_ENDPOINT_INFO_CC_REG = 10;
pub const EndpointParamRegRAX: IMAGE_HOTSWAP_X64_ENDPOINT_INFO_CC_REG = 1;
pub const EndpointParamRegRCX: IMAGE_HOTSWAP_X64_ENDPOINT_INFO_CC_REG = 2;
pub const EndpointParamRegRDX: IMAGE_HOTSWAP_X64_ENDPOINT_INFO_CC_REG = 3;
pub const EndpointParamRegXMM0: IMAGE_HOTSWAP_X64_ENDPOINT_INFO_CC_REG = 200;
pub const EndpointParamRegXMM1: IMAGE_HOTSWAP_X64_ENDPOINT_INFO_CC_REG = 201;
pub const EndpointParamRegXMM2: IMAGE_HOTSWAP_X64_ENDPOINT_INFO_CC_REG = 202;
pub const EndpointParamRegXMM3: IMAGE_HOTSWAP_X64_ENDPOINT_INFO_CC_REG = 203;
pub const EndpointReturnTypeNone: IMAGE_HOTSWAP_ARM64_ENDPOINT_INFO_CC_RETURN = 0;
pub const EndpointReturnTypeQ0: IMAGE_HOTSWAP_ARM64_ENDPOINT_INFO_CC_RETURN = 5;
pub const EndpointReturnTypeQ0_Q1: IMAGE_HOTSWAP_ARM64_ENDPOINT_INFO_CC_RETURN = 6;
pub const EndpointReturnTypeQ0_Q2: IMAGE_HOTSWAP_ARM64_ENDPOINT_INFO_CC_RETURN = 7;
pub const EndpointReturnTypeQ0_Q3: IMAGE_HOTSWAP_ARM64_ENDPOINT_INFO_CC_RETURN = 8;
pub const EndpointReturnTypeX0: IMAGE_HOTSWAP_ARM64_ENDPOINT_INFO_CC_RETURN = 1;
pub const EndpointReturnTypeX0_X1: IMAGE_HOTSWAP_ARM64_ENDPOINT_INFO_CC_RETURN = 2;
pub const EndpointReturnTypeX0_X2: IMAGE_HOTSWAP_ARM64_ENDPOINT_INFO_CC_RETURN = 3;
pub const EndpointReturnTypeX0_X3: IMAGE_HOTSWAP_ARM64_ENDPOINT_INFO_CC_RETURN = 4;
pub const EndpointReturnTypeX8: IMAGE_HOTSWAP_ARM64_ENDPOINT_INFO_CC_RETURN = 9;
pub const EnergyTrackerCreate: POWER_INFORMATION_LEVEL = 92;
pub const EnergyTrackerQuery: POWER_INFORMATION_LEVEL = 93;
pub const EnlistmentBasicInformation: ENLISTMENT_INFORMATION_CLASS = 0;
pub const EnlistmentCrmInformation: ENLISTMENT_INFORMATION_CLASS = 2;
pub const EnlistmentRecoveryInformation: ENLISTMENT_INFORMATION_CLASS = 1;
pub const ExitLatencySamplingPercentage: POWER_INFORMATION_LEVEL = 78;
pub const FAILED_ACCESS_ACE_FLAG: u32 = 128;
pub const FAST_FAIL_ADMINLESS_ACCESS_DENIED: u32 = 55;
pub const FAST_FAIL_APCS_DISABLED: u32 = 32;
pub const FAST_FAIL_ASAN_ERROR: u32 = 71;
pub const FAST_FAIL_CAST_GUARD: u32 = 65;
pub const FAST_FAIL_CERTIFICATION_FAILURE: u32 = 20;
pub const FAST_FAIL_CLR_EXCEPTION_AOT: u32 = 72;
pub const FAST_FAIL_CONTROL_INVALID_RETURN_ADDRESS: u32 = 57;
pub const FAST_FAIL_CORRUPT_LIST_ENTRY: u32 = 3;
pub const FAST_FAIL_CORRUPT_WOW64_STATE: u32 = 75;
pub const FAST_FAIL_CRYPTO_LIBRARY: u32 = 22;
pub const FAST_FAIL_DEPRECATED_SERVICE_INVOKED: u32 = 27;
pub const FAST_FAIL_DLOAD_PROTECTION_FAILURE: u32 = 25;
pub const FAST_FAIL_ENCLAVE_CALL_FAILURE: u32 = 53;
pub const FAST_FAIL_ETW_CORRUPTION: u32 = 61;
pub const FAST_FAIL_FATAL_APP_EXIT: u32 = 7;
pub const FAST_FAIL_FLAGS_CORRUPTION: u32 = 59;
pub const FAST_FAIL_GS_COOKIE_INIT: u32 = 6;
pub const FAST_FAIL_GUARD_EXPORT_SUPPRESSION_FAILURE: u32 = 46;
pub const FAST_FAIL_GUARD_ICALL_CHECK_FAILURE: u32 = 10;
pub const FAST_FAIL_GUARD_ICALL_CHECK_FAILURE_XFG: u32 = 64;
pub const FAST_FAIL_GUARD_ICALL_CHECK_SUPPRESSED: u32 = 31;
pub const FAST_FAIL_GUARD_JUMPTABLE: u32 = 37;
pub const FAST_FAIL_GUARD_SS_FAILURE: u32 = 44;
pub const FAST_FAIL_GUARD_WRITE_CHECK_FAILURE: u32 = 11;
pub const FAST_FAIL_HEAP_METADATA_CORRUPTION: u32 = 50;
pub const FAST_FAIL_HOST_VISIBILITY_CHANGE: u32 = 66;
pub const FAST_FAIL_INCORRECT_STACK: u32 = 4;
pub const FAST_FAIL_INVALID_ARG: u32 = 5;
pub const FAST_FAIL_INVALID_BALANCED_TREE: u32 = 29;
pub const FAST_FAIL_INVALID_BUFFER_ACCESS: u32 = 28;
pub const FAST_FAIL_INVALID_CALL_IN_DLL_CALLOUT: u32 = 23;
pub const FAST_FAIL_INVALID_CONTROL_STACK: u32 = 47;
pub const FAST_FAIL_INVALID_DISPATCH_CONTEXT: u32 = 39;
pub const FAST_FAIL_INVALID_EXCEPTION_CHAIN: u32 = 21;
pub const FAST_FAIL_INVALID_EXTENDED_STATE: u32 = 76;
pub const FAST_FAIL_INVALID_FAST_FAIL_CODE: u32 = 4294967295;
pub const FAST_FAIL_INVALID_FIBER_SWITCH: u32 = 12;
pub const FAST_FAIL_INVALID_FILE_OPERATION: u32 = 42;
pub const FAST_FAIL_INVALID_FLS_DATA: u32 = 70;
pub const FAST_FAIL_INVALID_IAT: u32 = 49;
pub const FAST_FAIL_INVALID_IDLE_STATE: u32 = 33;
pub const FAST_FAIL_INVALID_IMAGE_BASE: u32 = 24;
pub const FAST_FAIL_INVALID_JUMP_BUFFER: u32 = 18;
pub const FAST_FAIL_INVALID_LOCK_STATE: u32 = 36;
pub const FAST_FAIL_INVALID_LONGJUMP_TARGET: u32 = 38;
pub const FAST_FAIL_INVALID_NEXT_THREAD: u32 = 30;
pub const FAST_FAIL_INVALID_PFN: u32 = 63;
pub const FAST_FAIL_INVALID_REFERENCE_COUNT: u32 = 14;
pub const FAST_FAIL_INVALID_SET_OF_CONTEXT: u32 = 13;
pub const FAST_FAIL_INVALID_SME_STATE: u32 = 78;
pub const FAST_FAIL_INVALID_SYSCALL_NUMBER: u32 = 41;
pub const FAST_FAIL_INVALID_THREAD: u32 = 40;
pub const FAST_FAIL_INVALID_THREAD_STATE: u32 = 74;
pub const FAST_FAIL_KERNEL_CET_SHADOW_STACK_ASSIST: u32 = 67;
pub const FAST_FAIL_KERNEL_POINTER_EXPECTED: u32 = 77;
pub const FAST_FAIL_LEGACY_GS_VIOLATION: u32 = 0;
pub const FAST_FAIL_LOADER_CONTINUITY_FAILURE: u32 = 45;
pub const FAST_FAIL_LOW_LABEL_ACCESS_DENIED: u32 = 52;
pub const FAST_FAIL_LPAC_ACCESS_DENIED: u32 = 43;
pub const FAST_FAIL_MRDATA_MODIFIED: u32 = 19;
pub const FAST_FAIL_MRDATA_PROTECTION_FAILURE: u32 = 34;
pub const FAST_FAIL_NTDLL_PATCH_FAILED: u32 = 69;
pub const FAST_FAIL_PATCH_CALLBACK_FAILED: u32 = 68;
pub const FAST_FAIL_PAYLOAD_RESTRICTION_VIOLATION: u32 = 51;
pub const FAST_FAIL_POINTER_AUTH_INVALID_RETURN_ADDRESS: u32 = 73;
pub const FAST_FAIL_RANGE_CHECK_FAILURE: u32 = 8;
pub const FAST_FAIL_RIO_ABORT: u32 = 62;
pub const FAST_FAIL_SET_CONTEXT_DENIED: u32 = 48;
pub const FAST_FAIL_STACK_COOKIE_CHECK_FAILURE: u32 = 2;
pub const FAST_FAIL_UNEXPECTED_CALL: u32 = 56;
pub const FAST_FAIL_UNEXPECTED_HEAP_EXCEPTION: u32 = 35;
pub const FAST_FAIL_UNEXPECTED_HOST_BEHAVIOR: u32 = 58;
pub const FAST_FAIL_UNHANDLED_LSS_EXCEPTON: u32 = 54;
pub const FAST_FAIL_UNSAFE_EXTENSION_CALL: u32 = 26;
pub const FAST_FAIL_UNSAFE_REGISTRY_ACCESS: u32 = 9;
pub const FAST_FAIL_VEH_CORRUPTION: u32 = 60;
pub const FAST_FAIL_VTGUARD_CHECK_FAILURE: u32 = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct FCHAR(pub u8);
pub const FILE_ACTION_ADDED: u32 = 1;
pub const FILE_ACTION_MODIFIED: u32 = 3;
pub const FILE_ACTION_REMOVED: u32 = 2;
pub const FILE_ACTION_RENAMED_NEW_NAME: u32 = 5;
pub const FILE_ACTION_RENAMED_OLD_NAME: u32 = 4;
pub const FILE_ADD_FILE: u32 = 2;
pub const FILE_ADD_SUBDIRECTORY: u32 = 4;
pub const FILE_ALL_ACCESS: u32 = 2032127;
pub const FILE_APPEND_DATA: u32 = 4;
pub const FILE_ATTRIBUTE_ARCHIVE: u32 = 32;
pub const FILE_ATTRIBUTE_COMPRESSED: u32 = 2048;
pub const FILE_ATTRIBUTE_DEVICE: u32 = 64;
pub const FILE_ATTRIBUTE_DIRECTORY: u32 = 16;
pub const FILE_ATTRIBUTE_EA: u32 = 262144;
pub const FILE_ATTRIBUTE_ENCRYPTED: u32 = 16384;
pub const FILE_ATTRIBUTE_HIDDEN: u32 = 2;
pub const FILE_ATTRIBUTE_INTEGRITY_STREAM: u32 = 32768;
pub const FILE_ATTRIBUTE_NORMAL: u32 = 128;
pub const FILE_ATTRIBUTE_NOT_CONTENT_INDEXED: u32 = 8192;
pub const FILE_ATTRIBUTE_NO_SCRUB_DATA: u32 = 131072;
pub const FILE_ATTRIBUTE_OFFLINE: u32 = 4096;
pub const FILE_ATTRIBUTE_PINNED: u32 = 524288;
pub const FILE_ATTRIBUTE_READONLY: u32 = 1;
pub const FILE_ATTRIBUTE_RECALL_ON_DATA_ACCESS: u32 = 4194304;
pub const FILE_ATTRIBUTE_RECALL_ON_OPEN: u32 = 262144;
pub const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 1024;
pub const FILE_ATTRIBUTE_SPARSE_FILE: u32 = 512;
pub const FILE_ATTRIBUTE_STRICTLY_SEQUENTIAL: u32 = 536870912;
pub const FILE_ATTRIBUTE_SYSTEM: u32 = 4;
pub const FILE_ATTRIBUTE_TEMPORARY: u32 = 256;
pub const FILE_ATTRIBUTE_UNPINNED: u32 = 1048576;
pub const FILE_ATTRIBUTE_VIRTUAL: u32 = 65536;
pub const FILE_CASE_PRESERVED_NAMES: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_CASE_SENSITIVE_INFORMATION {
    pub Flags: u32,
}
pub const FILE_CASE_SENSITIVE_SEARCH: u32 = 1;
pub const FILE_CREATE_PIPE_INSTANCE: u32 = 4;
pub const FILE_CS_FLAG_CASE_SENSITIVE_DIR: u32 = 1;
pub const FILE_DAX_VOLUME: u32 = 536870912;
pub const FILE_DELETE_CHILD: u32 = 64;
pub const FILE_EXECUTE: u32 = 32;
pub const FILE_FILE_COMPRESSION: u32 = 16;
pub const FILE_GENERIC_EXECUTE: u32 = 1179808;
pub const FILE_GENERIC_READ: u32 = 1179785;
pub const FILE_GENERIC_WRITE: u32 = 1179926;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILE_ID_128 {
    pub Identifier: [u8; 16],
}
impl Default for FILE_ID_128 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILE_INVALID_FILE_ID: i64 = -1;
pub const FILE_LIST_DIRECTORY: u32 = 1;
pub const FILE_NAMED_STREAMS: u32 = 262144;
pub const FILE_NAME_FLAGS_UNSPECIFIED: u32 = 128;
pub const FILE_NAME_FLAG_BOTH: u32 = 3;
pub const FILE_NAME_FLAG_DOS: u32 = 2;
pub const FILE_NAME_FLAG_HARDLINK: u32 = 0;
pub const FILE_NAME_FLAG_NTFS: u32 = 1;
pub const FILE_NOTIFY_CHANGE_ATTRIBUTES: u32 = 4;
pub const FILE_NOTIFY_CHANGE_CREATION: u32 = 64;
pub const FILE_NOTIFY_CHANGE_DIR_NAME: u32 = 2;
pub const FILE_NOTIFY_CHANGE_FILE_NAME: u32 = 1;
pub const FILE_NOTIFY_CHANGE_LAST_ACCESS: u32 = 32;
pub const FILE_NOTIFY_CHANGE_LAST_WRITE: u32 = 16;
pub const FILE_NOTIFY_CHANGE_SECURITY: u32 = 256;
pub const FILE_NOTIFY_CHANGE_SIZE: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_NOTIFY_EXTENDED_INFORMATION {
    pub NextEntryOffset: u32,
    pub Action: u32,
    pub CreationTime: i64,
    pub LastModificationTime: i64,
    pub LastChangeTime: i64,
    pub LastAccessTime: i64,
    pub AllocatedLength: i64,
    pub FileSize: i64,
    pub FileAttributes: u32,
    pub Anonymous: FILE_NOTIFY_EXTENDED_INFORMATION_0,
    pub FileId: i64,
    pub ParentFileId: i64,
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
impl Default for FILE_NOTIFY_EXTENDED_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union FILE_NOTIFY_EXTENDED_INFORMATION_0 {
    pub ReparsePointTag: u32,
    pub EaSize: u32,
}
impl Default for FILE_NOTIFY_EXTENDED_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_NOTIFY_FULL_INFORMATION {
    pub NextEntryOffset: u32,
    pub Action: u32,
    pub CreationTime: i64,
    pub LastModificationTime: i64,
    pub LastChangeTime: i64,
    pub LastAccessTime: i64,
    pub AllocatedLength: i64,
    pub FileSize: i64,
    pub FileAttributes: u32,
    pub Anonymous: FILE_NOTIFY_FULL_INFORMATION_0,
    pub FileId: i64,
    pub ParentFileId: i64,
    pub FileNameLength: u16,
    pub FileNameFlags: u8,
    pub Reserved: u8,
    pub FileName: [u16; 1],
}
impl Default for FILE_NOTIFY_FULL_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union FILE_NOTIFY_FULL_INFORMATION_0 {
    pub ReparsePointTag: u32,
    pub EaSize: u32,
}
impl Default for FILE_NOTIFY_FULL_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILE_NOTIFY_INFORMATION {
    pub NextEntryOffset: u32,
    pub Action: u32,
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
impl Default for FILE_NOTIFY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILE_PERSISTENT_ACLS: u32 = 8;
pub const FILE_READ_ATTRIBUTES: u32 = 128;
pub const FILE_READ_DATA: u32 = 1;
pub const FILE_READ_EA: u32 = 8;
pub const FILE_READ_ONLY_VOLUME: u32 = 524288;
pub const FILE_RETURNS_CLEANUP_RESULT_INFO: u32 = 512;
#[repr(C)]
#[derive(Clone, Copy)]
pub union FILE_SEGMENT_ELEMENT {
    pub Buffer: *mut core::ffi::c_void,
    pub Alignment: u64,
}
impl Default for FILE_SEGMENT_ELEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILE_SEQUENTIAL_WRITE_ONCE: u32 = 1048576;
pub const FILE_SHARE_DELETE: u32 = 4;
pub const FILE_SHARE_READ: u32 = 1;
pub const FILE_SHARE_WRITE: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_STAT_BASIC_INFORMATION {
    pub FileId: i64,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub AllocationSize: i64,
    pub EndOfFile: i64,
    pub FileAttributes: u32,
    pub ReparseTag: u32,
    pub NumberOfLinks: u32,
    pub DeviceType: u32,
    pub DeviceCharacteristics: u32,
    pub Reserved: u32,
    pub VolumeSerialNumber: i64,
    pub FileId128: FILE_ID_128,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_STAT_INFORMATION {
    pub FileId: i64,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub AllocationSize: i64,
    pub EndOfFile: i64,
    pub FileAttributes: u32,
    pub ReparseTag: u32,
    pub NumberOfLinks: u32,
    pub EffectiveAccess: ACCESS_MASK,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_STAT_LX_INFORMATION {
    pub FileId: i64,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub AllocationSize: i64,
    pub EndOfFile: i64,
    pub FileAttributes: u32,
    pub ReparseTag: u32,
    pub NumberOfLinks: u32,
    pub EffectiveAccess: ACCESS_MASK,
    pub LxFlags: u32,
    pub LxUid: u32,
    pub LxGid: u32,
    pub LxMode: u32,
    pub LxDeviceIdMajor: u32,
    pub LxDeviceIdMinor: u32,
}
pub const FILE_SUPPORTS_BLOCK_REFCOUNTING: u32 = 134217728;
pub const FILE_SUPPORTS_BYPASS_IO: u32 = 2048;
pub const FILE_SUPPORTS_CASE_SENSITIVE_DIRS: u32 = 8192;
pub const FILE_SUPPORTS_ENCRYPTION: u32 = 131072;
pub const FILE_SUPPORTS_EXTENDED_ATTRIBUTES: u32 = 8388608;
pub const FILE_SUPPORTS_GHOSTING: u32 = 1073741824;
pub const FILE_SUPPORTS_HARD_LINKS: u32 = 4194304;
pub const FILE_SUPPORTS_INTEGRITY_STREAMS: u32 = 67108864;
pub const FILE_SUPPORTS_OBJECT_IDS: u32 = 65536;
pub const FILE_SUPPORTS_OPEN_BY_FILE_ID: u32 = 16777216;
pub const FILE_SUPPORTS_POSIX_UNLINK_RENAME: u32 = 1024;
pub const FILE_SUPPORTS_REMOTE_STORAGE: u32 = 256;
pub const FILE_SUPPORTS_REPARSE_POINTS: u32 = 128;
pub const FILE_SUPPORTS_SPARSE_FILES: u32 = 64;
pub const FILE_SUPPORTS_SPARSE_VDL: u32 = 268435456;
pub const FILE_SUPPORTS_STREAM_SNAPSHOTS: u32 = 4096;
pub const FILE_SUPPORTS_TRANSACTIONS: u32 = 2097152;
pub const FILE_SUPPORTS_USN_JOURNAL: u32 = 33554432;
pub const FILE_TRAVERSE: u32 = 32;
pub const FILE_UNICODE_ON_DISK: u32 = 4;
pub const FILE_VOLUME_IS_COMPRESSED: u32 = 32768;
pub const FILE_VOLUME_QUOTAS: u32 = 32;
pub const FILE_WRITE_ATTRIBUTES: u32 = 256;
pub const FILE_WRITE_DATA: u32 = 2;
pub const FILE_WRITE_EA: u32 = 16;
pub const FILL_NV_MEMORY_FLAG_FLUSH: u32 = 1;
pub const FILL_NV_MEMORY_FLAG_NON_TEMPORAL: u32 = 2;
pub const FILL_NV_MEMORY_FLAG_NO_DRAIN: u32 = 256;
pub const FILL_NV_MEMORY_FLAG_PERSIST: u32 = 3;
pub type FIRMWARE_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FLOAT128 {
    pub LowPart: i64,
    pub HighPart: i64,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLOATING_SAVE_AREA {
    pub ControlWord: u32,
    pub StatusWord: u32,
    pub TagWord: u32,
    pub ErrorOffset: u32,
    pub ErrorSelector: u32,
    pub DataOffset: u32,
    pub DataSelector: u32,
    pub RegisterArea: [u8; 80],
    pub Spare0: u32,
}
#[cfg(target_arch = "x86")]
impl Default for FLOATING_SAVE_AREA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct FLONG(pub u32);
pub const FLS_MAXIMUM_AVAILABLE: u32 = 4080;
pub const FLUSH_FLAGS_FILE_DATA_ONLY: u32 = 1;
pub const FLUSH_FLAGS_FILE_DATA_SYNC_ONLY: u32 = 4;
pub const FLUSH_FLAGS_FLUSH_AND_PURGE: u32 = 8;
pub const FLUSH_FLAGS_NO_SYNC: u32 = 2;
pub const FLUSH_NV_MEMORY_DEFAULT_TOKEN: i32 = -1;
pub const FLUSH_NV_MEMORY_IN_FLAG_NO_DRAIN: u32 = 1;
pub const FOREST_USER_RID_MAX: u32 = 499;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FPO_DATA {
    pub ulOffStart: u32,
    pub cbProcSize: u32,
    pub cdwLocals: u32,
    pub cdwParams: u16,
    pub _bitfield: u16,
}
pub const FRAME_FPO: u32 = 0;
pub const FRAME_NONFPO: u32 = 3;
pub const FRAME_TRAP: u32 = 1;
pub const FRAME_TSS: u32 = 2;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct FSHORT(pub u16);
pub const FileInformationInAssemblyOfAssemblyInActivationContext: ACTIVATION_CONTEXT_INFO_CLASS = 4;
pub const FileInformationInAssemblyOfAssemblyInActivationContxt: ACTIVATION_CONTEXT_INFO_CLASS = 4;
pub const FileSystemType: SERVICE_NODE_TYPE = 2;
pub const FirmwareTableInformationRegistered: POWER_INFORMATION_LEVEL = 69;
pub const FirmwareTypeBios: FIRMWARE_TYPE = 1;
pub const FirmwareTypeMax: FIRMWARE_TYPE = 3;
pub const FirmwareTypeUefi: FIRMWARE_TYPE = 2;
pub const FirmwareTypeUnknown: FIRMWARE_TYPE = 0;
pub const GENERIC_ALL: u32 = 268435456;
pub const GENERIC_EXECUTE: u32 = 536870912;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GENERIC_MAPPING {
    pub GenericRead: ACCESS_MASK,
    pub GenericWrite: ACCESS_MASK,
    pub GenericExecute: ACCESS_MASK,
    pub GenericAll: ACCESS_MASK,
}
pub const GENERIC_READ: u32 = 2147483648;
pub const GENERIC_WRITE: u32 = 1073741824;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub type GET_RUNTIME_FUNCTION_CALLBACK = Option<unsafe extern "system" fn(controlpc: u64, context: *const core::ffi::c_void) -> PRUNTIME_FUNCTION>;
#[cfg(target_arch = "aarch64")]
pub type GET_RUNTIME_FUNCTION_CALLBACK = Option<unsafe extern "system" fn(controlpc: u64, context: *const core::ffi::c_void) -> PARM64_RUNTIME_FUNCTION>;
#[repr(C)]
#[cfg(feature = "basetsd")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GROUP_AFFINITY {
    pub Mask: super::basetsd::KAFFINITY,
    pub Group: u16,
    pub Reserved: [u16; 3],
}
#[cfg(feature = "basetsd")]
impl Default for GROUP_AFFINITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GROUP_AFFINITY32 {
    pub Mask: u32,
    pub Group: u16,
    pub Reserved: [u16; 3],
}
impl Default for GROUP_AFFINITY32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GROUP_AFFINITY64 {
    pub Mask: u64,
    pub Group: u16,
    pub Reserved: [u16; 3],
}
impl Default for GROUP_AFFINITY64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "basetsd")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GROUP_RELATIONSHIP {
    pub MaximumGroupCount: u16,
    pub ActiveGroupCount: u16,
    pub Reserved: [u8; 20],
    pub GroupInfo: [PROCESSOR_GROUP_INFO; 1],
}
#[cfg(feature = "basetsd")]
impl Default for GROUP_RELATIONSHIP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const GROUP_SECURITY_INFORMATION: u32 = 2;
pub const GUID_ACDC_POWER_SOURCE: windows_core::GUID = windows_core::GUID::from_u128(0x5d3e9a59_e9d5_4b00_a6bd_ff34ff516548);
pub const GUID_ACTIVE_POWERSCHEME: windows_core::GUID = windows_core::GUID::from_u128(0x31f9f286_5084_42fe_b720_2b0264993763);
pub const GUID_ADAPTIVE_INPUT_CONTROLLER_STATE: windows_core::GUID = windows_core::GUID::from_u128(0x0e98fae9_f45a_4de1_a757_6031f197f6ea);
pub const GUID_ADAPTIVE_POWER_BEHAVIOR_SUBGROUP: windows_core::GUID = windows_core::GUID::from_u128(0x8619b916_e004_4dd8_9b66_dae86f806698);
pub const GUID_ADVANCED_COLOR_QUALITY_BIAS: windows_core::GUID = windows_core::GUID::from_u128(0x684c3e69_a4f7_4014_8754_d45179a56167);
pub const GUID_ALLOW_AWAYMODE: windows_core::GUID = windows_core::GUID::from_u128(0x25dfa149_5dd1_4736_b5ab_e8a37b5b8187);
pub const GUID_ALLOW_DISPLAY_REQUIRED: windows_core::GUID = windows_core::GUID::from_u128(0xa9ceb8da_cd46_44fb_a98b_02af69de4623);
pub const GUID_ALLOW_RTC_WAKE: windows_core::GUID = windows_core::GUID::from_u128(0xbd3b718a_0680_4d9d_8ab2_e1d2b4ac806d);
pub const GUID_ALLOW_STANDBY_STATES: windows_core::GUID = windows_core::GUID::from_u128(0xabfc2519_3608_4c2a_94ea_171b0ed546ab);
pub const GUID_ALLOW_SYSTEM_REQUIRED: windows_core::GUID = windows_core::GUID::from_u128(0xa4b195f5_8225_47d8_8012_9d41369786e2);
pub const GUID_APPLAUNCH_BUTTON: windows_core::GUID = windows_core::GUID::from_u128(0x1a689231_7399_4e9a_8f99_b71f999db3fa);
pub const GUID_BACKGROUND_TASK_NOTIFICATION: windows_core::GUID = windows_core::GUID::from_u128(0xcf23f240_2a54_48d8_b114_de1518ff052e);
pub const GUID_BATTERY_COUNT: windows_core::GUID = windows_core::GUID::from_u128(0x7d263f15_fca4_49e5_854b_a9f2bfbd5c24);
pub const GUID_BATTERY_DISCHARGE_ACTION_0: windows_core::GUID = windows_core::GUID::from_u128(0x637ea02f_bbcb_4015_8e2c_a1c7b9c0b546);
pub const GUID_BATTERY_DISCHARGE_ACTION_1: windows_core::GUID = windows_core::GUID::from_u128(0xd8742dcb_3e6a_4b3c_b3fe_374623cdcf06);
pub const GUID_BATTERY_DISCHARGE_ACTION_2: windows_core::GUID = windows_core::GUID::from_u128(0x421cba38_1a8e_4881_ac89_e33a8b04ece4);
pub const GUID_BATTERY_DISCHARGE_ACTION_3: windows_core::GUID = windows_core::GUID::from_u128(0x80472613_9780_455e_b308_72d3003cf2f8);
pub const GUID_BATTERY_DISCHARGE_FLAGS_0: windows_core::GUID = windows_core::GUID::from_u128(0x5dbb7c9f_38e9_40d2_9749_4f8a0e9f640f);
pub const GUID_BATTERY_DISCHARGE_FLAGS_1: windows_core::GUID = windows_core::GUID::from_u128(0xbcded951_187b_4d05_bccc_f7e51960c258);
pub const GUID_BATTERY_DISCHARGE_FLAGS_2: windows_core::GUID = windows_core::GUID::from_u128(0x7fd2f0c4_feb7_4da3_8117_e3fbedc46582);
pub const GUID_BATTERY_DISCHARGE_FLAGS_3: windows_core::GUID = windows_core::GUID::from_u128(0x73613ccf_dbfa_4279_8356_4935f6bf62f3);
pub const GUID_BATTERY_DISCHARGE_LEVEL_0: windows_core::GUID = windows_core::GUID::from_u128(0x9a66d8d7_4ff7_4ef9_b5a2_5a326ca2a469);
pub const GUID_BATTERY_DISCHARGE_LEVEL_1: windows_core::GUID = windows_core::GUID::from_u128(0x8183ba9a_e910_48da_8769_14ae6dc1170a);
pub const GUID_BATTERY_DISCHARGE_LEVEL_2: windows_core::GUID = windows_core::GUID::from_u128(0x07a07ca2_adaf_40d7_b077_533aaded1bfa);
pub const GUID_BATTERY_DISCHARGE_LEVEL_3: windows_core::GUID = windows_core::GUID::from_u128(0x58afd5a6_c2dd_47d2_9fbf_ef70cc5c5965);
pub const GUID_BATTERY_PERCENTAGE_REMAINING: windows_core::GUID = windows_core::GUID::from_u128(0xa7ad8041_b45a_4cae_87a3_eecbb468a9e1);
pub const GUID_BATTERY_SUBGROUP: windows_core::GUID = windows_core::GUID::from_u128(0xe73a048d_bf27_4f12_9731_8b2076e8891f);
pub const GUID_CONNECTIVITY_IN_STANDBY: windows_core::GUID = windows_core::GUID::from_u128(0xf15576e8_98b7_4186_b944_eafa664402d9);
pub const GUID_CONSOLE_DISPLAY_STATE: windows_core::GUID = windows_core::GUID::from_u128(0x6fe69556_704a_47a0_8f24_c28d936fda47);
pub const GUID_CRITICAL_POWER_TRANSITION: windows_core::GUID = windows_core::GUID::from_u128(0xb7a27025_e569_46c2_a504_2b96cad225a1);
pub const GUID_DEEP_SLEEP_ENABLED: windows_core::GUID = windows_core::GUID::from_u128(0xd502f7ee_1dc7_4efd_a55d_f04b6f5c0545);
pub const GUID_DEEP_SLEEP_PLATFORM_STATE: windows_core::GUID = windows_core::GUID::from_u128(0xd23f2fb8_9536_4038_9c94_1ce02e5c2152);
pub const GUID_DEVICE_IDLE_POLICY: windows_core::GUID = windows_core::GUID::from_u128(0x4faab71a_92e5_4726_b531_224559672d19);
pub const GUID_DEVICE_POWER_POLICY_VIDEO_BRIGHTNESS: windows_core::GUID = windows_core::GUID::from_u128(0xaded5e82_b909_4619_9949_f5d71dac0bcb);
pub const GUID_DEVICE_POWER_POLICY_VIDEO_DIM_BRIGHTNESS: windows_core::GUID = windows_core::GUID::from_u128(0xf1fbfde2_a960_4165_9f88_50667911ce96);
pub const GUID_DISCONNECTED_STANDBY_MODE: windows_core::GUID = windows_core::GUID::from_u128(0x68afb2d9_ee95_47a8_8f50_4115088073b1);
pub const GUID_DISK_ADAPTIVE_POWERDOWN: windows_core::GUID = windows_core::GUID::from_u128(0x396a32e1_499a_40b2_9124_a96afe707667);
pub const GUID_DISK_BURST_IGNORE_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0x80e3c60e_bb94_4ad8_bbe0_0d3195efc663);
pub const GUID_DISK_COALESCING_POWERDOWN_TIMEOUT: windows_core::GUID = windows_core::GUID::from_u128(0xc36f0eb4_2988_4a70_8eee_0884fc2c2433);
pub const GUID_DISK_IDLE_TIMEOUT: windows_core::GUID = windows_core::GUID::from_u128(0x58e39ba8_b8e6_4ef6_90d0_89ae32b258d6);
pub const GUID_DISK_MAX_POWER: windows_core::GUID = windows_core::GUID::from_u128(0x51dea550_bb38_4bc4_991b_eacf37be5ec8);
pub const GUID_DISK_NVME_NOPPME: windows_core::GUID = windows_core::GUID::from_u128(0xfc7372b6_ab2d_43ee_8797_15e9841f2cca);
pub const GUID_DISK_POWERDOWN_TIMEOUT: windows_core::GUID = windows_core::GUID::from_u128(0x6738e2c4_e8a5_4a42_b16a_e040e769756e);
pub const GUID_DISK_SUBGROUP: windows_core::GUID = windows_core::GUID::from_u128(0x0012ee47_9041_4b5d_9b77_535fba8b1442);
pub const GUID_ENABLE_SWITCH_FORCED_SHUTDOWN: windows_core::GUID = windows_core::GUID::from_u128(0x833a6b62_dfa4_46d1_82f8_e09e34d029d6);
pub const GUID_ENERGY_SAVER_BATTERY_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0xe69653ca_cf7f_4f05_aa73_cb833fa90ad4);
pub const GUID_ENERGY_SAVER_BRIGHTNESS: windows_core::GUID = windows_core::GUID::from_u128(0x13d09884_f74e_474a_a852_b6bde8ad03a8);
pub const GUID_ENERGY_SAVER_POLICY: windows_core::GUID = windows_core::GUID::from_u128(0x5c5bb349_ad29_4ee2_9d0b_2b25270f7a81);
pub const GUID_ENERGY_SAVER_STATUS: windows_core::GUID = windows_core::GUID::from_u128(0x550e8400_e29b_41d4_a716_446655440000);
pub const GUID_ENERGY_SAVER_SUBGROUP: windows_core::GUID = windows_core::GUID::from_u128(0xde830923_a562_41af_a086_e3a2c6bad2da);
pub const GUID_EXECUTION_REQUIRED_REQUEST_TIMEOUT: windows_core::GUID = windows_core::GUID::from_u128(0x3166bc41_7e98_4e03_b34e_ec0f5f2b218e);
pub const GUID_GLOBAL_USER_PRESENCE: windows_core::GUID = windows_core::GUID::from_u128(0x786e8a1d_b427_4344_9207_09e70bdcbea9);
pub const GUID_GPU_PREFERENCE_POLICY: windows_core::GUID = windows_core::GUID::from_u128(0xdd848b2a_8a5d_4451_9ae2_39cd41658f6c);
pub const GUID_GRAPHICS_SUBGROUP: windows_core::GUID = windows_core::GUID::from_u128(0x5fb4938d_1ee8_4b0f_9a3c_5036b0ab995c);
pub const GUID_HIBERNATE_FASTS4_POLICY: windows_core::GUID = windows_core::GUID::from_u128(0x94ac6d29_73ce_41a6_809f_6363ba21b47e);
pub const GUID_HIBERNATE_TIMEOUT: windows_core::GUID = windows_core::GUID::from_u128(0x9d7815a6_7ee4_497e_8888_515a05f02364);
pub const GUID_HUPR_ADAPTIVE_AWAY_DIM_TIMEOUT: windows_core::GUID = windows_core::GUID::from_u128(0xa79c8e0e_f271_482d_8f8a_5db9a18312de);
pub const GUID_HUPR_ADAPTIVE_AWAY_DISPLAY_TIMEOUT: windows_core::GUID = windows_core::GUID::from_u128(0x0a7d6ab6_ac83_4ad1_8282_eca5b58308f3);
pub const GUID_HUPR_ADAPTIVE_INATTENTIVE_DIM_TIMEOUT: windows_core::GUID = windows_core::GUID::from_u128(0xcf8c6097_12b8_4279_bbdd_44601ee5209d);
pub const GUID_HUPR_ADAPTIVE_INATTENTIVE_DISPLAY_TIMEOUT: windows_core::GUID = windows_core::GUID::from_u128(0xee16691e_6ab3_4619_bb48_1c77c9357e5a);
pub const GUID_IDLE_BACKGROUND_TASK: windows_core::GUID = windows_core::GUID::from_u128(0x515c31d8_f734_163d_a0fd_11a08c91e8f1);
pub const GUID_IDLE_RESILIENCY_PERIOD: windows_core::GUID = windows_core::GUID::from_u128(0xc42b79aa_aa3a_484b_a98f_2cf32aa90a28);
pub const GUID_IDLE_RESILIENCY_SUBGROUP: windows_core::GUID = windows_core::GUID::from_u128(0x2e601130_5351_4d9d_8e04_252966bad054);
pub const GUID_INTSTEER_LOAD_PER_PROC_TRIGGER: windows_core::GUID = windows_core::GUID::from_u128(0x73cde64d_d720_4bb2_a860_c755afe77ef2);
pub const GUID_INTSTEER_MODE: windows_core::GUID = windows_core::GUID::from_u128(0x2bfc24f9_5ea2_4801_8213_3dbae01aa39d);
pub const GUID_INTSTEER_SUBGROUP: windows_core::GUID = windows_core::GUID::from_u128(0x48672f38_7a9a_4bb2_8bf8_3d85be19de4e);
pub const GUID_INTSTEER_TIME_UNPARK_TRIGGER: windows_core::GUID = windows_core::GUID::from_u128(0xd6ba4903_386f_4c2c_8adb_5c21b3328d25);
pub const GUID_LEGACY_RTC_MITIGATION: windows_core::GUID = windows_core::GUID::from_u128(0x1a34bdc3_7e6b_442e_a9d0_64b6ef378e84);
pub const GUID_LIDCLOSE_ACTION: windows_core::GUID = windows_core::GUID::from_u128(0x5ca83367_6e45_459f_a27b_476b1d01c936);
pub const GUID_LIDOPEN_POWERSTATE: windows_core::GUID = windows_core::GUID::from_u128(0x99ff10e7_23b1_4c07_a9d1_5c3206d741b4);
pub const GUID_LIDSWITCH_STATE_CHANGE: windows_core::GUID = windows_core::GUID::from_u128(0xba3e0f4d_b817_4094_a2d1_d56379e6a0f3);
pub const GUID_LIDSWITCH_STATE_RELIABILITY: windows_core::GUID = windows_core::GUID::from_u128(0xae4c4ff1_d361_43f4_80aa_bbb6eb03de94);
pub const GUID_LOCK_CONSOLE_ON_WAKE: windows_core::GUID = windows_core::GUID::from_u128(0x0e796bdb_100d_47d6_a2d5_f7d2daa51f51);
pub const GUID_MAX_POWER_SAVINGS: windows_core::GUID = windows_core::GUID::from_u128(0xa1841308_3541_4fab_bc81_f71556f20b4a);
pub const GUID_MIN_POWER_SAVINGS: windows_core::GUID = windows_core::GUID::from_u128(0x8c5e7fda_e8bf_4a96_9a85_a6e23a8c635c);
pub const GUID_MIXED_REALITY_MODE: windows_core::GUID = windows_core::GUID::from_u128(0x1e626b4e_cf04_4f8d_9cc7_c97c5b0f2391);
pub const GUID_MONITOR_POWER_ON: windows_core::GUID = windows_core::GUID::from_u128(0x02731015_4510_4526_99e6_e5a17ebd1aea);
pub const GUID_NON_ADAPTIVE_INPUT_TIMEOUT: windows_core::GUID = windows_core::GUID::from_u128(0x5adbbfbc_074e_4da1_ba38_db8b36b2c8f3);
pub const GUID_PCIEXPRESS_ASPM_POLICY: windows_core::GUID = windows_core::GUID::from_u128(0xee12f906_d277_404b_b6da_e5fa1a576df5);
pub const GUID_PCIEXPRESS_SETTINGS_SUBGROUP: windows_core::GUID = windows_core::GUID::from_u128(0x501a4d13_42af_4429_9fd1_a8218c268e20);
pub const GUID_POWERBUTTON_ACTION: windows_core::GUID = windows_core::GUID::from_u128(0x7648efa3_dd9c_4e3e_b566_50f929386280);
pub const GUID_POWERSCHEME_PERSONALITY: windows_core::GUID = windows_core::GUID::from_u128(0x245d8541_3943_4422_b025_13a784f679b7);
pub const GUID_POWER_MODE_BEST_EFFICIENCY: windows_core::GUID = windows_core::GUID::from_u128(0x961cc777_2547_4f9d_8174_7d86181b8a7a);
pub const GUID_POWER_MODE_BEST_PERFORMANCE: windows_core::GUID = windows_core::GUID::from_u128(0xded574b5_45a0_4f42_8737_46345c09c238);
pub const GUID_POWER_MODE_NONE: windows_core::GUID = windows_core::GUID::from_u128(0x00000000_0000_0000_0000_000000000000);
pub const GUID_POWER_MODE_PERFORMANCE: windows_core::GUID = windows_core::GUID::from_u128(0x3af9b8d9_7c97_431d_ad78_34a8bfea439f);
pub const GUID_POWER_SAVING_STATUS: windows_core::GUID = windows_core::GUID::from_u128(0xe00958c0_c213_4ace_ac77_fecced2eeea5);
pub const GUID_PROCESSOR_ALLOW_THROTTLING: windows_core::GUID = windows_core::GUID::from_u128(0x3b04d4fd_1cc7_4f23_ab1c_d1337819c4bb);
pub const GUID_PROCESSOR_CLASS0_FLOOR_PERF: windows_core::GUID = windows_core::GUID::from_u128(0xfddc842b_8364_4edc_94cf_c17f60de1c80);
pub const GUID_PROCESSOR_CLASS1_INITIAL_PERF: windows_core::GUID = windows_core::GUID::from_u128(0x1facfc65_a930_4bc5_9f38_504ec097bbc0);
pub const GUID_PROCESSOR_COMPLEX_PARKING_POLICY: windows_core::GUID = windows_core::GUID::from_u128(0xb669a5e9_7b1d_4132_baaa_49190abcfeb6);
pub const GUID_PROCESSOR_CORE_PARKING_AFFINITY_HISTORY_DECREASE_FACTOR: windows_core::GUID = windows_core::GUID::from_u128(0x8f7b45e3_c393_480a_878c_f67ac3d07082);
pub const GUID_PROCESSOR_CORE_PARKING_AFFINITY_HISTORY_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0x5b33697b_e89d_4d38_aa46_9e7dfb7cd2f9);
pub const GUID_PROCESSOR_CORE_PARKING_AFFINITY_WEIGHTING: windows_core::GUID = windows_core::GUID::from_u128(0xe70867f1_fa2f_4f4e_aea1_4d8a0ba23b20);
pub const GUID_PROCESSOR_CORE_PARKING_DECREASE_POLICY: windows_core::GUID = windows_core::GUID::from_u128(0x71021b41_c749_4d21_be74_a00f335d582b);
pub const GUID_PROCESSOR_CORE_PARKING_DECREASE_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0x68dd2f27_a4ce_4e11_8487_3794e4135dfa);
pub const GUID_PROCESSOR_CORE_PARKING_DECREASE_TIME: windows_core::GUID = windows_core::GUID::from_u128(0xdfd10d17_d5eb_45dd_877a_9a34ddd15c82);
pub const GUID_PROCESSOR_CORE_PARKING_INCREASE_POLICY: windows_core::GUID = windows_core::GUID::from_u128(0xc7be0679_2817_4d69_9d02_519a537ed0c6);
pub const GUID_PROCESSOR_CORE_PARKING_INCREASE_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0xdf142941_20f3_4edf_9a4a_9c83d3d717d1);
pub const GUID_PROCESSOR_CORE_PARKING_INCREASE_TIME: windows_core::GUID = windows_core::GUID::from_u128(0x2ddd5a84_5a71_437e_912a_db0b8c788732);
pub const GUID_PROCESSOR_CORE_PARKING_MAX_CORES: windows_core::GUID = windows_core::GUID::from_u128(0xea062031_0e34_4ff1_9b6d_eb1059334028);
pub const GUID_PROCESSOR_CORE_PARKING_MAX_CORES_1: windows_core::GUID = windows_core::GUID::from_u128(0xea062031_0e34_4ff1_9b6d_eb1059334029);
pub const GUID_PROCESSOR_CORE_PARKING_MIN_CORES: windows_core::GUID = windows_core::GUID::from_u128(0x0cc5b647_c1df_4637_891a_dec35c318583);
pub const GUID_PROCESSOR_CORE_PARKING_MIN_CORES_1: windows_core::GUID = windows_core::GUID::from_u128(0x0cc5b647_c1df_4637_891a_dec35c318584);
pub const GUID_PROCESSOR_CORE_PARKING_OVER_UTILIZATION_HISTORY_DECREASE_FACTOR: windows_core::GUID = windows_core::GUID::from_u128(0x1299023c_bc28_4f0a_81ec_d3295a8d815d);
pub const GUID_PROCESSOR_CORE_PARKING_OVER_UTILIZATION_HISTORY_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0x9ac18e92_aa3c_4e27_b307_01ae37307129);
pub const GUID_PROCESSOR_CORE_PARKING_OVER_UTILIZATION_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0x943c8cb6_6f93_4227_ad87_e9a3feec08d1);
pub const GUID_PROCESSOR_CORE_PARKING_OVER_UTILIZATION_WEIGHTING: windows_core::GUID = windows_core::GUID::from_u128(0x8809c2d8_b155_42d4_bcda_0d345651b1db);
pub const GUID_PROCESSOR_DISTRIBUTE_UTILITY: windows_core::GUID = windows_core::GUID::from_u128(0xe0007330_f589_42ed_a401_5ddb10e785d3);
pub const GUID_PROCESSOR_DUTY_CYCLING: windows_core::GUID = windows_core::GUID::from_u128(0x4e4450b3_6179_4e91_b8f1_5bb9938f81a1);
pub const GUID_PROCESSOR_FREQUENCY_LIMIT: windows_core::GUID = windows_core::GUID::from_u128(0x75b0ae3f_bce0_45a7_8c89_c9611c25e100);
pub const GUID_PROCESSOR_FREQUENCY_LIMIT_1: windows_core::GUID = windows_core::GUID::from_u128(0x75b0ae3f_bce0_45a7_8c89_c9611c25e101);
pub const GUID_PROCESSOR_FREQUENCY_LIMIT_2: windows_core::GUID = windows_core::GUID::from_u128(0x75b0ae3f_bce0_45a7_8c89_c9611c25e102);
pub const GUID_PROCESSOR_FREQUENCY_MINIMUM: windows_core::GUID = windows_core::GUID::from_u128(0x2ac92cea_5efa_4a1b_bed5_1a2bd9aa0b94);
pub const GUID_PROCESSOR_FREQUENCY_MINIMUM_1: windows_core::GUID = windows_core::GUID::from_u128(0x2ac92cea_5efa_4a1b_bed5_1a2bd9aa0b95);
pub const GUID_PROCESSOR_FREQUENCY_MINIMUM_2: windows_core::GUID = windows_core::GUID::from_u128(0x2ac92cea_5efa_4a1b_bed5_1a2bd9aa0b96);
pub const GUID_PROCESSOR_HETEROGENEOUS_POLICY: windows_core::GUID = windows_core::GUID::from_u128(0x7f2f5cfa_f10c_4823_b5e1_e93ae85f46b5);
pub const GUID_PROCESSOR_HETERO_CONTAINMENT_DECREASE_TIME: windows_core::GUID = windows_core::GUID::from_u128(0x6ff13aeb_7897_4356_9999_dd9930af065f);
pub const GUID_PROCESSOR_HETERO_CONTAINMENT_EFFICIENCY_IMP_UTIL_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0x6ece9e1f_b6dd_42bf_b1b7_5a512b10c092);
pub const GUID_PROCESSOR_HETERO_CONTAINMENT_EFFICIENCY_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0x69439b22_221b_4830_bd34_f7bcece24583);
pub const GUID_PROCESSOR_HETERO_CONTAINMENT_HYBRID_IMP_UTIL_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0x12fd031f_53d2_4bf4_ac6d_c699fc9538c7);
pub const GUID_PROCESSOR_HETERO_CONTAINMENT_HYBRID_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0x6788488b_1b90_4d11_8fa7_973e470dff47);
pub const GUID_PROCESSOR_HETERO_CONTAINMENT_INCREASE_TIME: windows_core::GUID = windows_core::GUID::from_u128(0x64fcee6b_5b1f_45a4_a76a_19b2c36ee290);
pub const GUID_PROCESSOR_HETERO_CONTAINMENT_POLICY: windows_core::GUID = windows_core::GUID::from_u128(0x60fbe21b_efd9_49f2_b066_8674d8e9f423);
pub const GUID_PROCESSOR_HETERO_DECREASE_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0xf8861c27_95e7_475c_865b_13c0cb3f9d6b);
pub const GUID_PROCESSOR_HETERO_DECREASE_THRESHOLD_1: windows_core::GUID = windows_core::GUID::from_u128(0xf8861c27_95e7_475c_865b_13c0cb3f9d6c);
pub const GUID_PROCESSOR_HETERO_DECREASE_TIME: windows_core::GUID = windows_core::GUID::from_u128(0x7f2492b6_60b1_45e5_ae55_773f8cd5caec);
pub const GUID_PROCESSOR_HETERO_INCREASE_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0xb000397d_9b0b_483d_98c9_692a6060cfbf);
pub const GUID_PROCESSOR_HETERO_INCREASE_THRESHOLD_1: windows_core::GUID = windows_core::GUID::from_u128(0xb000397d_9b0b_483d_98c9_692a6060cfc0);
pub const GUID_PROCESSOR_HETERO_INCREASE_TIME: windows_core::GUID = windows_core::GUID::from_u128(0x4009efa7_e72d_4cba_9edf_91084ea8cbc3);
pub const GUID_PROCESSOR_IDLESTATE_POLICY: windows_core::GUID = windows_core::GUID::from_u128(0x68f262a7_f621_4069_b9a5_4874169be23c);
pub const GUID_PROCESSOR_IDLE_ALLOW_SCALING: windows_core::GUID = windows_core::GUID::from_u128(0x6c2993b0_8f48_481f_bcc6_00dd2742aa06);
pub const GUID_PROCESSOR_IDLE_DEMOTE_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0x4b92d758_5a24_4851_a470_815d78aee119);
pub const GUID_PROCESSOR_IDLE_DISABLE: windows_core::GUID = windows_core::GUID::from_u128(0x5d76a2ca_e8c0_402f_a133_2158492d58ad);
pub const GUID_PROCESSOR_IDLE_PROMOTE_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0x7b224883_b3cc_4d79_819f_8374152cbe7c);
pub const GUID_PROCESSOR_IDLE_STATE_MAXIMUM: windows_core::GUID = windows_core::GUID::from_u128(0x9943e905_9a30_4ec1_9b99_44dd3b76f7a2);
pub const GUID_PROCESSOR_IDLE_TIME_CHECK: windows_core::GUID = windows_core::GUID::from_u128(0xc4581c31_89ab_4597_8e2b_9c9cab440e6b);
pub const GUID_PROCESSOR_LATENCY_HINT_MIN_UNPARK: windows_core::GUID = windows_core::GUID::from_u128(0x616cdaa5_695e_4545_97ad_97dc2d1bdd88);
pub const GUID_PROCESSOR_LATENCY_HINT_MIN_UNPARK_1: windows_core::GUID = windows_core::GUID::from_u128(0x616cdaa5_695e_4545_97ad_97dc2d1bdd89);
pub const GUID_PROCESSOR_LONG_THREAD_ARCH_CLASS_LOWER_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0x43f278bc_0f8a_46d0_8b31_9a23e615d713);
pub const GUID_PROCESSOR_LONG_THREAD_ARCH_CLASS_UPPER_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0xbf903d33_9d24_49d3_a468_e65e0325046a);
pub const GUID_PROCESSOR_MODULE_PARKING_POLICY: windows_core::GUID = windows_core::GUID::from_u128(0xb0deaf6b_59c0_4523_8a45_ca7f40244114);
pub const GUID_PROCESSOR_PACKAGE_C6_POLICY: windows_core::GUID = windows_core::GUID::from_u128(0xfc1b015c_eb75_496a_ab47_028b0459c8f8);
pub const GUID_PROCESSOR_PARKING_CONCURRENCY_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0x2430ab6f_a520_44a2_9601_f7f23b5134b1);
pub const GUID_PROCESSOR_PARKING_CORE_OVERRIDE: windows_core::GUID = windows_core::GUID::from_u128(0xa55612aa_f624_42c6_a443_7397d064c04f);
pub const GUID_PROCESSOR_PARKING_DISTRIBUTION_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0x4bdaf4e9_d103_46d7_a5f0_6280121616ef);
pub const GUID_PROCESSOR_PARKING_HEADROOM_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0xf735a673_2066_4f80_a0c5_ddee0cf1bf5d);
pub const GUID_PROCESSOR_PARKING_PERF_STATE: windows_core::GUID = windows_core::GUID::from_u128(0x447235c7_6a8d_4cc0_8e24_9eaf70b96e2b);
pub const GUID_PROCESSOR_PARKING_PERF_STATE_1: windows_core::GUID = windows_core::GUID::from_u128(0x447235c7_6a8d_4cc0_8e24_9eaf70b96e2c);
pub const GUID_PROCESSOR_PERFSTATE_POLICY: windows_core::GUID = windows_core::GUID::from_u128(0xbbdc3814_18e9_4463_8a55_d197327c45c0);
pub const GUID_PROCESSOR_PERF_AUTONOMOUS_ACTIVITY_WINDOW: windows_core::GUID = windows_core::GUID::from_u128(0xcfeda3d0_7697_4566_a922_a9086cd49dfa);
pub const GUID_PROCESSOR_PERF_AUTONOMOUS_MODE: windows_core::GUID = windows_core::GUID::from_u128(0x8baa4a8a_14c6_4451_8e8b_14bdbd197537);
pub const GUID_PROCESSOR_PERF_BOOST_MODE: windows_core::GUID = windows_core::GUID::from_u128(0xbe337238_0d82_4146_a960_4f3749d470c7);
pub const GUID_PROCESSOR_PERF_BOOST_POLICY: windows_core::GUID = windows_core::GUID::from_u128(0x45bcc044_d885_43e2_8605_ee0ec6e96b59);
pub const GUID_PROCESSOR_PERF_CORE_PARKING_HISTORY: windows_core::GUID = windows_core::GUID::from_u128(0x77d7f282_8f1a_42cd_8537_45450a839be8);
pub const GUID_PROCESSOR_PERF_DECREASE_HISTORY: windows_core::GUID = windows_core::GUID::from_u128(0x0300f6f8_abd6_45a9_b74f_4908691a40b5);
pub const GUID_PROCESSOR_PERF_DECREASE_POLICY: windows_core::GUID = windows_core::GUID::from_u128(0x40fbefc7_2e9d_4d25_a185_0cfd8574bac6);
pub const GUID_PROCESSOR_PERF_DECREASE_POLICY_1: windows_core::GUID = windows_core::GUID::from_u128(0x40fbefc7_2e9d_4d25_a185_0cfd8574bac7);
pub const GUID_PROCESSOR_PERF_DECREASE_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0x12a0ab44_fe28_4fa9_b3bd_4b64f44960a6);
pub const GUID_PROCESSOR_PERF_DECREASE_THRESHOLD_1: windows_core::GUID = windows_core::GUID::from_u128(0x12a0ab44_fe28_4fa9_b3bd_4b64f44960a7);
pub const GUID_PROCESSOR_PERF_DECREASE_TIME: windows_core::GUID = windows_core::GUID::from_u128(0xd8edeb9b_95cf_4f95_a73c_b061973693c8);
pub const GUID_PROCESSOR_PERF_DECREASE_TIME_1: windows_core::GUID = windows_core::GUID::from_u128(0xd8edeb9b_95cf_4f95_a73c_b061973693c9);
pub const GUID_PROCESSOR_PERF_ENERGY_PERFORMANCE_PREFERENCE: windows_core::GUID = windows_core::GUID::from_u128(0x36687f9e_e3a5_4dbf_b1dc_15eb381c6863);
pub const GUID_PROCESSOR_PERF_ENERGY_PERFORMANCE_PREFERENCE_1: windows_core::GUID = windows_core::GUID::from_u128(0x36687f9e_e3a5_4dbf_b1dc_15eb381c6864);
pub const GUID_PROCESSOR_PERF_ENERGY_PERFORMANCE_PREFERENCE_2: windows_core::GUID = windows_core::GUID::from_u128(0x36687f9e_e3a5_4dbf_b1dc_15eb381c6865);
pub const GUID_PROCESSOR_PERF_HISTORY: windows_core::GUID = windows_core::GUID::from_u128(0x7d24baa7_0b84_480f_840c_1b0743c00f5f);
pub const GUID_PROCESSOR_PERF_HISTORY_1: windows_core::GUID = windows_core::GUID::from_u128(0x7d24baa7_0b84_480f_840c_1b0743c00f60);
pub const GUID_PROCESSOR_PERF_INCREASE_HISTORY: windows_core::GUID = windows_core::GUID::from_u128(0x99b3ef01_752f_46a1_80fb_7730011f2354);
pub const GUID_PROCESSOR_PERF_INCREASE_POLICY: windows_core::GUID = windows_core::GUID::from_u128(0x465e1f50_b610_473a_ab58_00d1077dc418);
pub const GUID_PROCESSOR_PERF_INCREASE_POLICY_1: windows_core::GUID = windows_core::GUID::from_u128(0x465e1f50_b610_473a_ab58_00d1077dc419);
pub const GUID_PROCESSOR_PERF_INCREASE_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0x06cadf0e_64ed_448a_8927_ce7bf90eb35d);
pub const GUID_PROCESSOR_PERF_INCREASE_THRESHOLD_1: windows_core::GUID = windows_core::GUID::from_u128(0x06cadf0e_64ed_448a_8927_ce7bf90eb35e);
pub const GUID_PROCESSOR_PERF_INCREASE_TIME: windows_core::GUID = windows_core::GUID::from_u128(0x984cf492_3bed_4488_a8f9_4286c97bf5aa);
pub const GUID_PROCESSOR_PERF_INCREASE_TIME_1: windows_core::GUID = windows_core::GUID::from_u128(0x984cf492_3bed_4488_a8f9_4286c97bf5ab);
pub const GUID_PROCESSOR_PERF_LATENCY_HINT: windows_core::GUID = windows_core::GUID::from_u128(0x0822df31_9c83_441c_a079_0de4cf009c7b);
pub const GUID_PROCESSOR_PERF_LATENCY_HINT_EPP: windows_core::GUID = windows_core::GUID::from_u128(0x4b70f900_cdd9_4e66_aa26_ae8417f98173);
pub const GUID_PROCESSOR_PERF_LATENCY_HINT_EPP_1: windows_core::GUID = windows_core::GUID::from_u128(0x4b70f900_cdd9_4e66_aa26_ae8417f98174);
pub const GUID_PROCESSOR_PERF_LATENCY_HINT_EPP_2: windows_core::GUID = windows_core::GUID::from_u128(0x4b70f900_cdd9_4e66_aa26_ae8417f98175);
pub const GUID_PROCESSOR_PERF_LATENCY_HINT_FREQ: windows_core::GUID = windows_core::GUID::from_u128(0x81202931_acbb_405c_a7ee_3e2ba4866f6f);
pub const GUID_PROCESSOR_PERF_LATENCY_HINT_FREQ_1: windows_core::GUID = windows_core::GUID::from_u128(0x81202931_acbb_405c_a7ee_3e2ba4866f70);
pub const GUID_PROCESSOR_PERF_LATENCY_HINT_FREQ_2: windows_core::GUID = windows_core::GUID::from_u128(0x81202931_acbb_405c_a7ee_3e2ba4866f71);
pub const GUID_PROCESSOR_PERF_LATENCY_HINT_PERF: windows_core::GUID = windows_core::GUID::from_u128(0x619b7505_003b_4e82_b7a6_4dd29c300971);
pub const GUID_PROCESSOR_PERF_LATENCY_HINT_PERF_1: windows_core::GUID = windows_core::GUID::from_u128(0x619b7505_003b_4e82_b7a6_4dd29c300972);
pub const GUID_PROCESSOR_PERF_LATENCY_HINT_PERF_2: windows_core::GUID = windows_core::GUID::from_u128(0x619b7505_003b_4e82_b7a6_4dd29c300973);
pub const GUID_PROCESSOR_PERF_TIME_CHECK: windows_core::GUID = windows_core::GUID::from_u128(0x4d2b0152_7d5c_498b_88e2_34345392a2c5);
pub const GUID_PROCESSOR_RESOURCE_PRIORITY: windows_core::GUID = windows_core::GUID::from_u128(0x603fe9ce_8d01_4b48_a968_1d706c28fd5c);
pub const GUID_PROCESSOR_RESOURCE_PRIORITY_1: windows_core::GUID = windows_core::GUID::from_u128(0x603fe9ce_8d01_4b48_a968_1d706c28fd5d);
pub const GUID_PROCESSOR_RESOURCE_PRIORITY_2: windows_core::GUID = windows_core::GUID::from_u128(0x603fe9ce_8d01_4b48_a968_1d706c28fd5e);
pub const GUID_PROCESSOR_RESPONSIVENESS_DISABLE_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0x38b8383d_cce0_4c79_9e3e_56a4f17cc480);
pub const GUID_PROCESSOR_RESPONSIVENESS_DISABLE_THRESHOLD_1: windows_core::GUID = windows_core::GUID::from_u128(0x38b8383d_cce0_4c79_9e3e_56a4f17cc481);
pub const GUID_PROCESSOR_RESPONSIVENESS_DISABLE_TIME: windows_core::GUID = windows_core::GUID::from_u128(0xf565999f_3fb0_411a_a226_3f0198dec130);
pub const GUID_PROCESSOR_RESPONSIVENESS_DISABLE_TIME_1: windows_core::GUID = windows_core::GUID::from_u128(0xf565999f_3fb0_411a_a226_3f0198dec131);
pub const GUID_PROCESSOR_RESPONSIVENESS_ENABLE_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0x3d44e256_7222_4415_a9ed_9c45fa3dd830);
pub const GUID_PROCESSOR_RESPONSIVENESS_ENABLE_THRESHOLD_1: windows_core::GUID = windows_core::GUID::from_u128(0x3d44e256_7222_4415_a9ed_9c45fa3dd831);
pub const GUID_PROCESSOR_RESPONSIVENESS_ENABLE_TIME: windows_core::GUID = windows_core::GUID::from_u128(0x3d915188_7830_49ae_a79a_0fb0a1e5a200);
pub const GUID_PROCESSOR_RESPONSIVENESS_ENABLE_TIME_1: windows_core::GUID = windows_core::GUID::from_u128(0x3d915188_7830_49ae_a79a_0fb0a1e5a201);
pub const GUID_PROCESSOR_RESPONSIVENESS_EPP_CEILING: windows_core::GUID = windows_core::GUID::from_u128(0x4427c73b_9756_4a5c_b84b_c7bda79c7320);
pub const GUID_PROCESSOR_RESPONSIVENESS_EPP_CEILING_1: windows_core::GUID = windows_core::GUID::from_u128(0x4427c73b_9756_4a5c_b84b_c7bda79c7321);
pub const GUID_PROCESSOR_RESPONSIVENESS_PERF_FLOOR: windows_core::GUID = windows_core::GUID::from_u128(0xce8e92ee_6a86_4572_bfe0_20c21d03cd40);
pub const GUID_PROCESSOR_RESPONSIVENESS_PERF_FLOOR_1: windows_core::GUID = windows_core::GUID::from_u128(0xce8e92ee_6a86_4572_bfe0_20c21d03cd41);
pub const GUID_PROCESSOR_RESTRICTION_COUNT: windows_core::GUID = windows_core::GUID::from_u128(0x1a98ad09_af22_42ca_8e61_f0a5802c270a);
pub const GUID_PROCESSOR_SETTINGS_SUBGROUP: windows_core::GUID = windows_core::GUID::from_u128(0x54533251_82be_4824_96c1_47b60b740d00);
pub const GUID_PROCESSOR_SHORT_THREAD_ARCH_CLASS_LOWER_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0x53824d46_87bd_4739_aa1b_aa793fac36d6);
pub const GUID_PROCESSOR_SHORT_THREAD_ARCH_CLASS_UPPER_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0x828423eb_8662_4344_90f7_52bf15870f5a);
pub const GUID_PROCESSOR_SHORT_THREAD_RUNTIME_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0xd92998c2_6a48_49ca_85d4_8cceec294570);
pub const GUID_PROCESSOR_SHORT_THREAD_SCHEDULING_POLICY: windows_core::GUID = windows_core::GUID::from_u128(0xbae08b81_2d5e_4688_ad6a_13243356654b);
pub const GUID_PROCESSOR_SMT_UNPARKING_POLICY: windows_core::GUID = windows_core::GUID::from_u128(0xb28a6829_c5f7_444e_8f61_10e24e85c532);
pub const GUID_PROCESSOR_SOFT_PARKING_LATENCY: windows_core::GUID = windows_core::GUID::from_u128(0x97cfac41_2217_47eb_992d_618b1977c907);
pub const GUID_PROCESSOR_THREAD_SCHEDULING_POLICY: windows_core::GUID = windows_core::GUID::from_u128(0x93b8b6dc_0698_4d1c_9ee4_0644e900c85d);
pub const GUID_PROCESSOR_THROTTLE_MAXIMUM: windows_core::GUID = windows_core::GUID::from_u128(0xbc5038f7_23e0_4960_96da_33abaf5935ec);
pub const GUID_PROCESSOR_THROTTLE_MAXIMUM_1: windows_core::GUID = windows_core::GUID::from_u128(0xbc5038f7_23e0_4960_96da_33abaf5935ed);
pub const GUID_PROCESSOR_THROTTLE_MAXIMUM_2: windows_core::GUID = windows_core::GUID::from_u128(0xbc5038f7_23e0_4960_96da_33abaf5935ee);
pub const GUID_PROCESSOR_THROTTLE_MINIMUM: windows_core::GUID = windows_core::GUID::from_u128(0x893dee8e_2bef_41e0_89c6_b55d0929964c);
pub const GUID_PROCESSOR_THROTTLE_MINIMUM_1: windows_core::GUID = windows_core::GUID::from_u128(0x893dee8e_2bef_41e0_89c6_b55d0929964d);
pub const GUID_PROCESSOR_THROTTLE_MINIMUM_2: windows_core::GUID = windows_core::GUID::from_u128(0x893dee8e_2bef_41e0_89c6_b55d0929964e);
pub const GUID_PROCESSOR_THROTTLE_POLICY: windows_core::GUID = windows_core::GUID::from_u128(0x57027304_4af6_4104_9260_e3d95248fc36);
pub const GUID_PROCESSOR_WPS_MIN_EFFICIENCY_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0x5ba7419a_295c_4b02_841b_66799388d6da);
pub const GUID_SESSION_DISPLAY_STATUS: windows_core::GUID = windows_core::GUID::from_u128(0x2b84c20e_ad23_4ddf_93db_05ffbd7efca5);
pub const GUID_SESSION_USER_PRESENCE: windows_core::GUID = windows_core::GUID::from_u128(0x3c0f4548_c03f_4c4d_b9f2_237ede686376);
pub const GUID_SLEEPBUTTON_ACTION: windows_core::GUID = windows_core::GUID::from_u128(0x96996bc0_ad50_47ec_923b_6f41874dd9eb);
pub const GUID_SLEEP_IDLE_THRESHOLD: windows_core::GUID = windows_core::GUID::from_u128(0x81cd32e0_7833_44f3_8737_7081f38d1f70);
pub const GUID_SLEEP_SUBGROUP: windows_core::GUID = windows_core::GUID::from_u128(0x238c9fa8_0aad_41ed_83f4_97be242c8f20);
pub const GUID_SPR_ACTIVE_SESSION_CHANGE: windows_core::GUID = windows_core::GUID::from_u128(0x0e24ce38_c393_4742_bdb1_744f4b9ee08e);
pub const GUID_STANDBY_BUDGET_GRACE_PERIOD: windows_core::GUID = windows_core::GUID::from_u128(0x60c07fe1_0556_45cf_9903_d56e32210242);
pub const GUID_STANDBY_BUDGET_PERCENT: windows_core::GUID = windows_core::GUID::from_u128(0x9fe527be_1b70_48da_930d_7bcf17b44990);
pub const GUID_STANDBY_BUDGET_REFRESH_COUNT: windows_core::GUID = windows_core::GUID::from_u128(0xaca8648e_c4b1_4baa_8cce_9390ad647f8c);
pub const GUID_STANDBY_BUDGET_REFRESH_INTERVAL: windows_core::GUID = windows_core::GUID::from_u128(0x61f45dfe_1919_4180_bb46_8cc70e0b38f1);
pub const GUID_STANDBY_RESERVE_GRACE_PERIOD: windows_core::GUID = windows_core::GUID::from_u128(0xc763ee92_71e8_4127_84eb_f6ed043a3e3d);
pub const GUID_STANDBY_RESERVE_TIME: windows_core::GUID = windows_core::GUID::from_u128(0x468fe7e5_1158_46ec_88bc_5b96c9e44fd0);
pub const GUID_STANDBY_RESET_PERCENT: windows_core::GUID = windows_core::GUID::from_u128(0x49cb11a5_56e2_4afb_9d38_3df47872e21b);
pub const GUID_STANDBY_TIMEOUT: windows_core::GUID = windows_core::GUID::from_u128(0x29f6c1db_86da_48c5_9fdb_f2b67b1f44da);
pub const GUID_SYSTEM_AWAYMODE: windows_core::GUID = windows_core::GUID::from_u128(0x98a7f580_01f7_48aa_9c0f_44352c29e5c0);
pub const GUID_SYSTEM_BUTTON_SUBGROUP: windows_core::GUID = windows_core::GUID::from_u128(0x4f971e89_eebd_4455_a8de_9e59040e7347);
pub const GUID_SYSTEM_COOLING_POLICY: windows_core::GUID = windows_core::GUID::from_u128(0x94d3a615_a899_4ac5_ae2b_e4d8f634367f);
pub const GUID_TYPICAL_POWER_SAVINGS: windows_core::GUID = windows_core::GUID::from_u128(0x381b4222_f694_41f0_9685_ff5bb260df2e);
pub const GUID_UNATTEND_SLEEP_TIMEOUT: windows_core::GUID = windows_core::GUID::from_u128(0x7bc4a2f9_d8fc_4469_b07b_33eb785aaca0);
pub const GUID_USERINTERFACEBUTTON_ACTION: windows_core::GUID = windows_core::GUID::from_u128(0xa7066653_8d6c_40a8_910e_a1f54b84c7e5);
pub const GUID_USER_PRESENCE_PREDICTION: windows_core::GUID = windows_core::GUID::from_u128(0x82011705_fb95_4d46_8d35_4042b1d20def);
pub const GUID_VIDEO_ADAPTIVE_DISPLAY_BRIGHTNESS: windows_core::GUID = windows_core::GUID::from_u128(0xfbd9aa66_9553_4097_ba44_ed6e9d65eab8);
pub const GUID_VIDEO_ADAPTIVE_PERCENT_INCREASE: windows_core::GUID = windows_core::GUID::from_u128(0xeed904df_b142_4183_b10b_5a1197a37864);
pub const GUID_VIDEO_ADAPTIVE_POWERDOWN: windows_core::GUID = windows_core::GUID::from_u128(0x90959d22_d6a1_49b9_af93_bce885ad335b);
pub const GUID_VIDEO_ANNOYANCE_TIMEOUT: windows_core::GUID = windows_core::GUID::from_u128(0x82dbcf2d_cd67_40c5_bfdc_9f1a5ccd4663);
pub const GUID_VIDEO_CONSOLE_LOCK_TIMEOUT: windows_core::GUID = windows_core::GUID::from_u128(0x8ec4b3a5_6868_48c2_be75_4f3044be88a7);
pub const GUID_VIDEO_CURRENT_MONITOR_BRIGHTNESS: windows_core::GUID = windows_core::GUID::from_u128(0x8ffee2c6_2d01_46be_adb9_398addc5b4ff);
pub const GUID_VIDEO_DIM_TIMEOUT: windows_core::GUID = windows_core::GUID::from_u128(0x17aaa29b_8b43_4b94_aafe_35f64daaf1ee);
pub const GUID_VIDEO_POWERDOWN_TIMEOUT: windows_core::GUID = windows_core::GUID::from_u128(0x3c0bc021_c8a8_4e07_a973_6b14cbcb2b7e);
pub const GUID_VIDEO_SUBGROUP: windows_core::GUID = windows_core::GUID::from_u128(0x7516b95f_f776_4464_8c53_06167f40cc99);
pub const GetPowerRequestList: POWER_INFORMATION_LEVEL = 45;
pub const GetPowerSettingValue: POWER_INFORMATION_LEVEL = 59;
pub const GlobalDataIdConsoleSharedDataFlags: RTL_SYSTEM_GLOBAL_DATA_ID = 14;
pub const GlobalDataIdCyclesPerYield: RTL_SYSTEM_GLOBAL_DATA_ID = 11;
pub const GlobalDataIdImageNumberHigh: RTL_SYSTEM_GLOBAL_DATA_ID = 5;
pub const GlobalDataIdImageNumberLow: RTL_SYSTEM_GLOBAL_DATA_ID = 4;
pub const GlobalDataIdInterruptTime: RTL_SYSTEM_GLOBAL_DATA_ID = 2;
pub const GlobalDataIdKdDebuggerEnabled: RTL_SYSTEM_GLOBAL_DATA_ID = 10;
pub const GlobalDataIdLastSystemRITEventTickCount: RTL_SYSTEM_GLOBAL_DATA_ID = 13;
pub const GlobalDataIdNtMajorVersion: RTL_SYSTEM_GLOBAL_DATA_ID = 7;
pub const GlobalDataIdNtMinorVersion: RTL_SYSTEM_GLOBAL_DATA_ID = 8;
pub const GlobalDataIdNtSystemRootDrive: RTL_SYSTEM_GLOBAL_DATA_ID = 15;
pub const GlobalDataIdQpcBias: RTL_SYSTEM_GLOBAL_DATA_ID = 18;
pub const GlobalDataIdQpcBypassEnabled: RTL_SYSTEM_GLOBAL_DATA_ID = 16;
pub const GlobalDataIdQpcData: RTL_SYSTEM_GLOBAL_DATA_ID = 17;
pub const GlobalDataIdRngSeedVersion: RTL_SYSTEM_GLOBAL_DATA_ID = 1;
pub const GlobalDataIdSafeBootMode: RTL_SYSTEM_GLOBAL_DATA_ID = 12;
pub const GlobalDataIdSystemExpirationDate: RTL_SYSTEM_GLOBAL_DATA_ID = 9;
pub const GlobalDataIdTimeZoneBias: RTL_SYSTEM_GLOBAL_DATA_ID = 3;
pub const GlobalDataIdTimeZoneId: RTL_SYSTEM_GLOBAL_DATA_ID = 6;
pub const GlobalDataIdUnknown: RTL_SYSTEM_GLOBAL_DATA_ID = 0;
pub const GroupPark: POWER_INFORMATION_LEVEL = 48;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HANDLE(pub *mut core::ffi::c_void);
impl HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HARDWARE_COUNTER_DATA {
    pub Type: HARDWARE_COUNTER_TYPE,
    pub Reserved: u32,
    pub Value: u64,
}
pub type HARDWARE_COUNTER_TYPE = i32;
pub const HEAP_CREATE_ALIGN_16: u32 = 65536;
pub const HEAP_CREATE_ENABLE_EXECUTE: u32 = 262144;
pub const HEAP_CREATE_ENABLE_TRACING: u32 = 131072;
pub const HEAP_CREATE_HARDENED: u32 = 512;
pub const HEAP_CREATE_SEGMENT_HEAP: u32 = 256;
pub const HEAP_DISABLE_COALESCE_ON_FREE: u32 = 128;
pub const HEAP_FREE_CHECKING_ENABLED: u32 = 64;
pub const HEAP_GENERATE_EXCEPTIONS: u32 = 4;
pub const HEAP_GROWABLE: u32 = 2;
pub type HEAP_INFORMATION_CLASS = i32;
pub const HEAP_MAXIMUM_TAG: u32 = 4095;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HEAP_MEMORY_USAGE_ENTRY {
    pub HeapHandle: *mut core::ffi::c_void,
    pub TotalCommittedBytes: usize,
    pub TotalReservedBytes: usize,
}
impl Default for HEAP_MEMORY_USAGE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HEAP_MEMORY_USAGE_INFORMATION {
    pub Version: u16,
    pub EntryCount: usize,
    pub Entries: [HEAP_MEMORY_USAGE_ENTRY; 1],
}
impl Default for HEAP_MEMORY_USAGE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HEAP_MEMORY_USAGE_INFO_CURRENT_VERSION: u32 = 1;
pub const HEAP_NO_SERIALIZE: u32 = 1;
pub const HEAP_OPTIMIZE_RESOURCES_CURRENT_VERSION: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HEAP_OPTIMIZE_RESOURCES_INFORMATION {
    pub Version: u32,
    pub Flags: u32,
}
pub const HEAP_PSEUDO_TAG_FLAG: u32 = 32768;
pub const HEAP_REALLOC_IN_PLACE_ONLY: u32 = 16;
pub const HEAP_TAG_SHIFT: u32 = 18;
pub const HEAP_TAIL_CHECKING_ENABLED: u32 = 32;
pub const HEAP_ZERO_MEMORY: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HIBERFILE_BUCKET {
    pub MaxPhysicalMemory: u64,
    pub PhysicalMemoryPercent: [u32; 3],
}
impl Default for HIBERFILE_BUCKET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type HIBERFILE_BUCKET_SIZE = i32;
pub const HIBERFILE_TYPE_FULL: u32 = 2;
pub const HIBERFILE_TYPE_MAX: u32 = 3;
pub const HIBERFILE_TYPE_NONE: u32 = 0;
pub const HIBERFILE_TYPE_REDUCED: u32 = 1;
pub const HeapCompatibilityInformation: HEAP_INFORMATION_CLASS = 0;
pub const HeapEnableTerminationOnCorruption: HEAP_INFORMATION_CLASS = 1;
pub const HeapMemoryUsageInformation: HEAP_INFORMATION_CLASS = 8;
pub const HeapOptimizeResources: HEAP_INFORMATION_CLASS = 3;
pub const HeapTag: HEAP_INFORMATION_CLASS = 7;
pub const HiberFileBucket16GB: HIBERFILE_BUCKET_SIZE = 4;
pub const HiberFileBucket1GB: HIBERFILE_BUCKET_SIZE = 0;
pub const HiberFileBucket2GB: HIBERFILE_BUCKET_SIZE = 1;
pub const HiberFileBucket32GB: HIBERFILE_BUCKET_SIZE = 5;
pub const HiberFileBucket4GB: HIBERFILE_BUCKET_SIZE = 2;
pub const HiberFileBucket8GB: HIBERFILE_BUCKET_SIZE = 3;
pub const HiberFileBucketMax: HIBERFILE_BUCKET_SIZE = 7;
pub const HiberFileBucketUnlimited: HIBERFILE_BUCKET_SIZE = 6;
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct IMAGE_ALPHA64_RUNTIME_FUNCTION_ENTRY {
    pub BeginAddress: u64,
    pub EndAddress: u64,
    pub ExceptionHandler: u64,
    pub HandlerData: u64,
    pub PrologEndAddress: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_ALPHA_RUNTIME_FUNCTION_ENTRY {
    pub BeginAddress: u32,
    pub EndAddress: u32,
    pub ExceptionHandler: u32,
    pub HandlerData: u32,
    pub PrologEndAddress: u32,
}
pub type IMAGE_AMD64_RUNTIME_FUNCTION_ENTRY = _IMAGE_RUNTIME_FUNCTION_ENTRY;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_ARCHITECTURE_ENTRY {
    pub FixupInstRVA: u32,
    pub NewInst: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_ARCHITECTURE_HEADER {
    pub _bitfield: u32,
    pub FirstEntryRVA: u32,
}
pub const IMAGE_ARCHIVE_END: windows_core::PCSTR = windows_core::s!("`\n");
pub const IMAGE_ARCHIVE_HYBRIDMAP_MEMBER: windows_core::PCSTR = windows_core::s!("/<HYBRIDMAP>/   ");
pub const IMAGE_ARCHIVE_LINKER_MEMBER: windows_core::PCSTR = windows_core::s!("/               ");
pub const IMAGE_ARCHIVE_LONGNAMES_MEMBER: windows_core::PCSTR = windows_core::s!("//              ");
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGE_ARCHIVE_MEMBER_HEADER {
    pub Name: [u8; 16],
    pub Date: [u8; 12],
    pub UserID: [u8; 6],
    pub GroupID: [u8; 6],
    pub Mode: [u8; 8],
    pub Size: [u8; 10],
    pub EndHeader: [u8; 2],
}
impl Default for IMAGE_ARCHIVE_MEMBER_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IMAGE_ARCHIVE_PAD: windows_core::PCSTR = windows_core::s!("\n");
pub const IMAGE_ARCHIVE_START: windows_core::PCSTR = windows_core::s!("!<arch>\n");
pub const IMAGE_ARCHIVE_START_SIZE: u32 = 8;
pub type IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY = ARM64_RUNTIME_FUNCTION;
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_XDATA {
    pub HeaderData: u32,
    pub Anonymous: IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_XDATA_0,
}
impl Default for IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_XDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_XDATA_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_XDATA_EPILOG_SCOPE {
    pub EpilogScopeData: u32,
    pub Anonymous: IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_XDATA_EPILOG_SCOPE_0,
}
impl Default for IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_XDATA_EPILOG_SCOPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_XDATA_EPILOG_SCOPE_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_XDATA_EXTENDED {
    pub ExtendedHeaderData: u32,
    pub Anonymous: IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_XDATA_EXTENDED_0,
}
impl Default for IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_XDATA_EXTENDED {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY_XDATA_EXTENDED_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMAGE_ARM_RUNTIME_FUNCTION_ENTRY {
    pub BeginAddress: u32,
    pub Anonymous: IMAGE_ARM_RUNTIME_FUNCTION_ENTRY_0,
}
impl Default for IMAGE_ARM_RUNTIME_FUNCTION_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_ARM_RUNTIME_FUNCTION_ENTRY_0 {
    pub UnwindData: u32,
    pub Anonymous: IMAGE_ARM_RUNTIME_FUNCTION_ENTRY_0_0,
}
impl Default for IMAGE_ARM_RUNTIME_FUNCTION_ENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_ARM_RUNTIME_FUNCTION_ENTRY_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_AUX_SYMBOL {
    pub Sym: IMAGE_AUX_SYMBOL_0,
    pub File: IMAGE_AUX_SYMBOL_1,
    pub Section: IMAGE_AUX_SYMBOL_2,
    pub TokenDef: IMAGE_AUX_SYMBOL_TOKEN_DEF,
    pub CRC: IMAGE_AUX_SYMBOL_3,
}
impl Default for IMAGE_AUX_SYMBOL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct IMAGE_AUX_SYMBOL_0 {
    pub TagIndex: u32,
    pub Misc: IMAGE_AUX_SYMBOL_0_0,
    pub FcnAry: IMAGE_AUX_SYMBOL_0_1,
    pub TvIndex: u16,
}
impl Default for IMAGE_AUX_SYMBOL_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub union IMAGE_AUX_SYMBOL_0_0 {
    pub LnSz: IMAGE_AUX_SYMBOL_0_0_0,
    pub TotalSize: u32,
}
impl Default for IMAGE_AUX_SYMBOL_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_AUX_SYMBOL_0_0_0 {
    pub Linenumber: u16,
    pub Size: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_AUX_SYMBOL_0_1 {
    pub Function: IMAGE_AUX_SYMBOL_0_1_0,
    pub Array: IMAGE_AUX_SYMBOL_0_1_1,
}
impl Default for IMAGE_AUX_SYMBOL_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[derive(Clone, Copy, Default)]
pub struct IMAGE_AUX_SYMBOL_0_1_0 {
    pub PointerToLinenumber: u32,
    pub PointerToNextFunction: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGE_AUX_SYMBOL_0_1_1 {
    pub Dimension: [u16; 4],
}
impl Default for IMAGE_AUX_SYMBOL_0_1_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGE_AUX_SYMBOL_1 {
    pub Name: [u8; 18],
}
impl Default for IMAGE_AUX_SYMBOL_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[derive(Clone, Copy, Default)]
pub struct IMAGE_AUX_SYMBOL_2 {
    pub Length: u32,
    pub NumberOfRelocations: u16,
    pub NumberOfLinenumbers: u16,
    pub CheckSum: u32,
    pub Number: i16,
    pub Selection: u8,
    pub bReserved: u8,
    pub HighNumber: i16,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct IMAGE_AUX_SYMBOL_3 {
    pub crc: u32,
    pub rgbReserved: [u8; 14],
}
impl Default for IMAGE_AUX_SYMBOL_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_AUX_SYMBOL_EX {
    pub Sym: IMAGE_AUX_SYMBOL_EX_0,
    pub File: IMAGE_AUX_SYMBOL_EX_1,
    pub Section: IMAGE_AUX_SYMBOL_EX_2,
    pub Anonymous: IMAGE_AUX_SYMBOL_EX_3,
    pub CRC: IMAGE_AUX_SYMBOL_EX_4,
}
impl Default for IMAGE_AUX_SYMBOL_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct IMAGE_AUX_SYMBOL_EX_0 {
    pub WeakDefaultSymIndex: u32,
    pub WeakSearchType: u32,
    pub rgbReserved: [u8; 12],
}
impl Default for IMAGE_AUX_SYMBOL_EX_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGE_AUX_SYMBOL_EX_1 {
    pub Name: [u8; 20],
}
impl Default for IMAGE_AUX_SYMBOL_EX_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct IMAGE_AUX_SYMBOL_EX_2 {
    pub Length: u32,
    pub NumberOfRelocations: u16,
    pub NumberOfLinenumbers: u16,
    pub CheckSum: u32,
    pub Number: i16,
    pub Selection: u8,
    pub bReserved: u8,
    pub HighNumber: i16,
    pub rgbReserved: [u8; 2],
}
impl Default for IMAGE_AUX_SYMBOL_EX_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMAGE_AUX_SYMBOL_EX_3 {
    pub TokenDef: IMAGE_AUX_SYMBOL_TOKEN_DEF,
    pub rgbReserved: [u8; 2],
}
impl Default for IMAGE_AUX_SYMBOL_EX_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct IMAGE_AUX_SYMBOL_EX_4 {
    pub crc: u32,
    pub rgbReserved: [u8; 16],
}
impl Default for IMAGE_AUX_SYMBOL_EX_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct IMAGE_AUX_SYMBOL_TOKEN_DEF {
    pub bAuxType: u8,
    pub bReserved: u8,
    pub SymbolTableIndex: u32,
    pub rgbReserved: [u8; 12],
}
impl Default for IMAGE_AUX_SYMBOL_TOKEN_DEF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type IMAGE_AUX_SYMBOL_TYPE = i32;
pub const IMAGE_AUX_SYMBOL_TYPE_TOKEN_DEF: IMAGE_AUX_SYMBOL_TYPE = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_BASE_RELOCATION {
    pub VirtualAddress: u32,
    pub SizeOfBlock: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct IMAGE_BDD_DYNAMIC_RELOCATION {
    pub Left: u16,
    pub Right: u16,
    pub Value: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct IMAGE_BDD_INFO {
    pub Version: u32,
    pub BDDSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_BOUND_FORWARDER_REF {
    pub TimeDateStamp: u32,
    pub OffsetModuleName: u16,
    pub Reserved: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_BOUND_IMPORT_DESCRIPTOR {
    pub TimeDateStamp: u32,
    pub OffsetModuleName: u16,
    pub NumberOfModuleForwarderRefs: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_CE_RUNTIME_FUNCTION_ENTRY {
    pub FuncStart: u32,
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_COFF_SYMBOLS_HEADER {
    pub NumberOfSymbols: u32,
    pub LvaToFirstSymbol: u32,
    pub NumberOfLinenumbers: u32,
    pub LvaToFirstLinenumber: u32,
    pub RvaToFirstByteOfCode: u32,
    pub RvaToLastByteOfCode: u32,
    pub RvaToFirstByteOfData: u32,
    pub RvaToLastByteOfData: u32,
}
pub const IMAGE_COMDAT_SELECT_ANY: u32 = 2;
pub const IMAGE_COMDAT_SELECT_ASSOCIATIVE: u32 = 5;
pub const IMAGE_COMDAT_SELECT_EXACT_MATCH: u32 = 4;
pub const IMAGE_COMDAT_SELECT_LARGEST: u32 = 6;
pub const IMAGE_COMDAT_SELECT_NEWEST: u32 = 7;
pub const IMAGE_COMDAT_SELECT_NODUPLICATES: u32 = 1;
pub const IMAGE_COMDAT_SELECT_SAME_SIZE: u32 = 3;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMAGE_COR20_HEADER {
    pub cb: u32,
    pub MajorRuntimeVersion: u16,
    pub MinorRuntimeVersion: u16,
    pub MetaData: IMAGE_DATA_DIRECTORY,
    pub Flags: u32,
    pub Anonymous: IMAGE_COR20_HEADER_0,
    pub Resources: IMAGE_DATA_DIRECTORY,
    pub StrongNameSignature: IMAGE_DATA_DIRECTORY,
    pub CodeManagerTable: IMAGE_DATA_DIRECTORY,
    pub VTableFixups: IMAGE_DATA_DIRECTORY,
    pub ExportAddressTableJumps: IMAGE_DATA_DIRECTORY,
    pub ManagedNativeHeader: IMAGE_DATA_DIRECTORY,
}
impl Default for IMAGE_COR20_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_COR20_HEADER_0 {
    pub EntryPointToken: u32,
    pub EntryPointRVA: u32,
}
impl Default for IMAGE_COR20_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IMAGE_COR_EATJ_THUNK_SIZE: ReplacesCorHdrNumericDefines = 32;
pub const IMAGE_COR_MIH_BASICBLOCK: ReplacesCorHdrNumericDefines = 8;
pub const IMAGE_COR_MIH_EHRVA: ReplacesCorHdrNumericDefines = 2;
pub const IMAGE_COR_MIH_METHODRVA: ReplacesCorHdrNumericDefines = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_DATA_DIRECTORY {
    pub VirtualAddress: u32,
    pub Size: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_DEBUG_DIRECTORY {
    pub Characteristics: u32,
    pub TimeDateStamp: u32,
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub Type: u32,
    pub SizeOfData: u32,
    pub AddressOfRawData: u32,
    pub PointerToRawData: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGE_DEBUG_MISC {
    pub DataType: u32,
    pub Length: u32,
    pub Unicode: bool,
    pub Reserved: [u8; 3],
    pub Data: [u8; 1],
}
impl Default for IMAGE_DEBUG_MISC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IMAGE_DEBUG_MISC_EXENAME: u32 = 1;
pub const IMAGE_DEBUG_TYPE_BBT: u32 = 10;
pub const IMAGE_DEBUG_TYPE_BORLAND: u32 = 9;
pub const IMAGE_DEBUG_TYPE_CLSID: u32 = 11;
pub const IMAGE_DEBUG_TYPE_CODEVIEW: u32 = 2;
pub const IMAGE_DEBUG_TYPE_COFF: u32 = 1;
pub const IMAGE_DEBUG_TYPE_EXCEPTION: u32 = 5;
pub const IMAGE_DEBUG_TYPE_EX_DLLCHARACTERISTICS: u32 = 20;
pub const IMAGE_DEBUG_TYPE_FIXUP: u32 = 6;
pub const IMAGE_DEBUG_TYPE_FPO: u32 = 3;
pub const IMAGE_DEBUG_TYPE_ILTCG: u32 = 14;
pub const IMAGE_DEBUG_TYPE_MISC: u32 = 4;
pub const IMAGE_DEBUG_TYPE_MPX: u32 = 15;
pub const IMAGE_DEBUG_TYPE_OMAP_FROM_SRC: u32 = 8;
pub const IMAGE_DEBUG_TYPE_OMAP_TO_SRC: u32 = 7;
pub const IMAGE_DEBUG_TYPE_POGO: u32 = 13;
pub const IMAGE_DEBUG_TYPE_REPRO: u32 = 16;
pub const IMAGE_DEBUG_TYPE_RESERVED10: u32 = 10;
pub const IMAGE_DEBUG_TYPE_SPGO: u32 = 18;
pub const IMAGE_DEBUG_TYPE_UNKNOWN: u32 = 0;
pub const IMAGE_DEBUG_TYPE_VC_FEATURE: u32 = 12;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMAGE_DELAYLOAD_DESCRIPTOR {
    pub Attributes: IMAGE_DELAYLOAD_DESCRIPTOR_0,
    pub DllNameRVA: u32,
    pub ModuleHandleRVA: u32,
    pub ImportAddressTableRVA: u32,
    pub ImportNameTableRVA: u32,
    pub BoundImportAddressTableRVA: u32,
    pub UnloadInformationTableRVA: u32,
    pub TimeDateStamp: u32,
}
impl Default for IMAGE_DELAYLOAD_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_DELAYLOAD_DESCRIPTOR_0 {
    pub AllAttributes: u32,
    pub Anonymous: IMAGE_DELAYLOAD_DESCRIPTOR_0_0,
}
impl Default for IMAGE_DELAYLOAD_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_DELAYLOAD_DESCRIPTOR_0_0 {
    pub _bitfield: u32,
}
pub const IMAGE_DIRECTORY_ENTRY_ARCHITECTURE: u32 = 7;
pub const IMAGE_DIRECTORY_ENTRY_BASERELOC: u32 = 5;
pub const IMAGE_DIRECTORY_ENTRY_BOUND_IMPORT: u32 = 11;
pub const IMAGE_DIRECTORY_ENTRY_COM_DESCRIPTOR: u32 = 14;
pub const IMAGE_DIRECTORY_ENTRY_DEBUG: u32 = 6;
pub const IMAGE_DIRECTORY_ENTRY_DELAY_IMPORT: u32 = 13;
pub const IMAGE_DIRECTORY_ENTRY_EXCEPTION: u32 = 3;
pub const IMAGE_DIRECTORY_ENTRY_EXPORT: u32 = 0;
pub const IMAGE_DIRECTORY_ENTRY_GLOBALPTR: u32 = 8;
pub const IMAGE_DIRECTORY_ENTRY_IAT: u32 = 12;
pub const IMAGE_DIRECTORY_ENTRY_IMPORT: u32 = 1;
pub const IMAGE_DIRECTORY_ENTRY_LOAD_CONFIG: u32 = 10;
pub const IMAGE_DIRECTORY_ENTRY_RESOURCE: u32 = 2;
pub const IMAGE_DIRECTORY_ENTRY_SECURITY: u32 = 4;
pub const IMAGE_DIRECTORY_ENTRY_TLS: u32 = 9;
pub const IMAGE_DLLCHARACTERISTICS_APPCONTAINER: u32 = 4096;
pub const IMAGE_DLLCHARACTERISTICS_DYNAMIC_BASE: u32 = 64;
pub const IMAGE_DLLCHARACTERISTICS_EX_CET_COMPAT: u32 = 1;
pub const IMAGE_DLLCHARACTERISTICS_EX_CET_COMPAT_STRICT_MODE: u32 = 2;
pub const IMAGE_DLLCHARACTERISTICS_EX_CET_DYNAMIC_APIS_ALLOW_IN_PROC: u32 = 8;
pub const IMAGE_DLLCHARACTERISTICS_EX_CET_RESERVED_1: u32 = 16;
pub const IMAGE_DLLCHARACTERISTICS_EX_CET_RESERVED_2: u32 = 32;
pub const IMAGE_DLLCHARACTERISTICS_EX_CET_SET_CONTEXT_IP_VALIDATION_RELAXED_MODE: u32 = 4;
pub const IMAGE_DLLCHARACTERISTICS_EX_FORWARD_CFI_COMPAT: u32 = 64;
pub const IMAGE_DLLCHARACTERISTICS_EX_HOTPATCH_COMPATIBLE: u32 = 128;
pub const IMAGE_DLLCHARACTERISTICS_FORCE_INTEGRITY: u32 = 128;
pub const IMAGE_DLLCHARACTERISTICS_GUARD_CF: u32 = 16384;
pub const IMAGE_DLLCHARACTERISTICS_HIGH_ENTROPY_VA: u32 = 32;
pub const IMAGE_DLLCHARACTERISTICS_NO_BIND: u32 = 2048;
pub const IMAGE_DLLCHARACTERISTICS_NO_ISOLATION: u32 = 512;
pub const IMAGE_DLLCHARACTERISTICS_NO_SEH: u32 = 1024;
pub const IMAGE_DLLCHARACTERISTICS_NX_COMPAT: u32 = 256;
pub const IMAGE_DLLCHARACTERISTICS_TERMINAL_SERVER_AWARE: u32 = 32768;
pub const IMAGE_DLLCHARACTERISTICS_WDM_DRIVER: u32 = 8192;
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct IMAGE_DOS_HEADER {
    pub e_magic: u16,
    pub e_cblp: u16,
    pub e_cp: u16,
    pub e_crlc: u16,
    pub e_cparhdr: u16,
    pub e_minalloc: u16,
    pub e_maxalloc: u16,
    pub e_ss: u16,
    pub e_sp: u16,
    pub e_csum: u16,
    pub e_ip: u16,
    pub e_cs: u16,
    pub e_lfarlc: u16,
    pub e_ovno: u16,
    pub e_res: [u16; 4],
    pub e_oemid: u16,
    pub e_oeminfo: u16,
    pub e_res2: [u16; 10],
    pub e_lfanew: i32,
}
impl Default for IMAGE_DOS_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IMAGE_DOS_SIGNATURE: u32 = 23117;
#[cfg(target_arch = "x86")]
pub type IMAGE_DYNAMIC_RELOCATION = IMAGE_DYNAMIC_RELOCATION32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type IMAGE_DYNAMIC_RELOCATION = IMAGE_DYNAMIC_RELOCATION64;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct IMAGE_DYNAMIC_RELOCATION32 {
    pub Symbol: u32,
    pub BaseRelocSize: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct IMAGE_DYNAMIC_RELOCATION32_V2 {
    pub HeaderSize: u32,
    pub FixupInfoSize: u32,
    pub Symbol: u32,
    pub SymbolGroup: u32,
    pub Flags: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct IMAGE_DYNAMIC_RELOCATION64 {
    pub Symbol: u64,
    pub BaseRelocSize: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct IMAGE_DYNAMIC_RELOCATION64_V2 {
    pub HeaderSize: u32,
    pub FixupInfoSize: u32,
    pub Symbol: u64,
    pub SymbolGroup: u32,
    pub Flags: u32,
}
pub const IMAGE_DYNAMIC_RELOCATION_ARM64_KERNEL_IMPORT_CALL_TRANSFER: u32 = 8;
pub const IMAGE_DYNAMIC_RELOCATION_FUNCTION_OVERRIDE: u32 = 7;
pub const IMAGE_DYNAMIC_RELOCATION_GUARD_IMPORT_CONTROL_TRANSFER: u32 = 3;
pub const IMAGE_DYNAMIC_RELOCATION_GUARD_INDIR_CONTROL_TRANSFER: u32 = 4;
pub const IMAGE_DYNAMIC_RELOCATION_GUARD_RF_EPILOGUE: u32 = 2;
pub const IMAGE_DYNAMIC_RELOCATION_GUARD_RF_PROLOGUE: u32 = 1;
pub const IMAGE_DYNAMIC_RELOCATION_GUARD_SWITCHTABLE_BRANCH: u32 = 5;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const IMAGE_DYNAMIC_RELOCATION_IMPORT_CONTROL_TRANSFER: u32 = 3;
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
pub const IMAGE_DYNAMIC_RELOCATION_IMPORT_CONTROL_TRANSFER: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_DYNAMIC_RELOCATION_TABLE {
    pub Version: u32,
    pub Size: u32,
}
#[cfg(target_arch = "x86")]
pub type IMAGE_DYNAMIC_RELOCATION_V2 = IMAGE_DYNAMIC_RELOCATION32_V2;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type IMAGE_DYNAMIC_RELOCATION_V2 = IMAGE_DYNAMIC_RELOCATION64_V2;
#[cfg(target_arch = "x86")]
pub type IMAGE_ENCLAVE_CONFIG = IMAGE_ENCLAVE_CONFIG32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type IMAGE_ENCLAVE_CONFIG = IMAGE_ENCLAVE_CONFIG64;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGE_ENCLAVE_CONFIG32 {
    pub Size: u32,
    pub MinimumRequiredConfigSize: u32,
    pub PolicyFlags: u32,
    pub NumberOfImports: u32,
    pub ImportList: u32,
    pub ImportEntrySize: u32,
    pub FamilyID: [u8; 16],
    pub ImageID: [u8; 16],
    pub ImageVersion: u32,
    pub SecurityVersion: u32,
    pub EnclaveSize: u32,
    pub NumberOfThreads: u32,
    pub EnclaveFlags: u32,
}
impl Default for IMAGE_ENCLAVE_CONFIG32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct IMAGE_ENCLAVE_CONFIG64 {
    pub Size: u32,
    pub MinimumRequiredConfigSize: u32,
    pub PolicyFlags: u32,
    pub NumberOfImports: u32,
    pub ImportList: u32,
    pub ImportEntrySize: u32,
    pub FamilyID: [u8; 16],
    pub ImageID: [u8; 16],
    pub ImageVersion: u32,
    pub SecurityVersion: u32,
    pub EnclaveSize: u64,
    pub NumberOfThreads: u32,
    pub EnclaveFlags: u32,
}
impl Default for IMAGE_ENCLAVE_CONFIG64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IMAGE_ENCLAVE_FLAG_PRIMARY_IMAGE: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGE_ENCLAVE_IMPORT {
    pub MatchType: u32,
    pub MinimumSecurityVersion: u32,
    pub UniqueOrAuthorID: [u8; 32],
    pub FamilyID: [u8; 16],
    pub ImageID: [u8; 16],
    pub ImportName: u32,
    pub Reserved: u32,
}
impl Default for IMAGE_ENCLAVE_IMPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IMAGE_ENCLAVE_IMPORT_MATCH_AUTHOR_ID: u32 = 2;
pub const IMAGE_ENCLAVE_IMPORT_MATCH_FAMILY_ID: u32 = 3;
pub const IMAGE_ENCLAVE_IMPORT_MATCH_IMAGE_ID: u32 = 4;
pub const IMAGE_ENCLAVE_IMPORT_MATCH_NONE: u32 = 0;
pub const IMAGE_ENCLAVE_IMPORT_MATCH_UNIQUE_ID: u32 = 1;
pub const IMAGE_ENCLAVE_LONG_ID_LENGTH: u32 = 32;
#[cfg(target_arch = "x86")]
pub const IMAGE_ENCLAVE_MINIMUM_CONFIG_SIZE: u32 = 72;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const IMAGE_ENCLAVE_MINIMUM_CONFIG_SIZE: u32 = 76;
pub const IMAGE_ENCLAVE_POLICY_DEBUGGABLE: u32 = 1;
pub const IMAGE_ENCLAVE_POLICY_STRICT_MEMORY: u32 = 2;
pub const IMAGE_ENCLAVE_SHORT_ID_LENGTH: u32 = 16;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct IMAGE_EPILOGUE_DYNAMIC_RELOCATION_HEADER {
    pub EpilogueCount: u32,
    pub EpilogueByteCount: u8,
    pub BranchDescriptorElementSize: u8,
    pub BranchDescriptorCount: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_EXPORT_DIRECTORY {
    pub Characteristics: u32,
    pub TimeDateStamp: u32,
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub Name: u32,
    pub Base: u32,
    pub NumberOfFunctions: u32,
    pub NumberOfNames: u32,
    pub AddressOfFunctions: u32,
    pub AddressOfNames: u32,
    pub AddressOfNameOrdinals: u32,
}
pub const IMAGE_FILE_32BIT_MACHINE: u32 = 256;
pub const IMAGE_FILE_AGGRESIVE_WS_TRIM: u32 = 16;
pub const IMAGE_FILE_BYTES_REVERSED_HI: u32 = 32768;
pub const IMAGE_FILE_BYTES_REVERSED_LO: u32 = 128;
pub const IMAGE_FILE_DEBUG_STRIPPED: u32 = 512;
pub const IMAGE_FILE_DLL: u32 = 8192;
pub const IMAGE_FILE_EXECUTABLE_IMAGE: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_FILE_HEADER {
    pub Machine: u16,
    pub NumberOfSections: u16,
    pub TimeDateStamp: u32,
    pub PointerToSymbolTable: u32,
    pub NumberOfSymbols: u32,
    pub SizeOfOptionalHeader: u16,
    pub Characteristics: u16,
}
pub const IMAGE_FILE_LARGE_ADDRESS_AWARE: u32 = 32;
pub const IMAGE_FILE_LINE_NUMS_STRIPPED: u32 = 4;
pub const IMAGE_FILE_LOCAL_SYMS_STRIPPED: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMAGE_FILE_MACHINES {
    pub Anonymous: IMAGE_FILE_MACHINES_0,
}
impl Default for IMAGE_FILE_MACHINES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_FILE_MACHINES_0 {
    pub Value: u32,
    pub Anonymous: IMAGE_FILE_MACHINES_0_0,
}
impl Default for IMAGE_FILE_MACHINES_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_FILE_MACHINES_0_0 {
    pub _bitfield: u32,
}
pub const IMAGE_FILE_MACHINE_ALPHA: u32 = 388;
pub const IMAGE_FILE_MACHINE_ALPHA64: u32 = 644;
pub const IMAGE_FILE_MACHINE_AM33: u32 = 467;
pub const IMAGE_FILE_MACHINE_AMD64: u32 = 34404;
pub const IMAGE_FILE_MACHINE_ARM: u32 = 448;
pub const IMAGE_FILE_MACHINE_ARM64: u32 = 43620;
pub const IMAGE_FILE_MACHINE_ARMNT: u32 = 452;
pub const IMAGE_FILE_MACHINE_AXP64: u32 = 644;
pub const IMAGE_FILE_MACHINE_CEE: u32 = 49390;
pub const IMAGE_FILE_MACHINE_CEF: u32 = 3311;
pub const IMAGE_FILE_MACHINE_EBC: u32 = 3772;
pub const IMAGE_FILE_MACHINE_I386: u32 = 332;
pub const IMAGE_FILE_MACHINE_IA64: u32 = 512;
pub const IMAGE_FILE_MACHINE_M32R: u32 = 36929;
pub const IMAGE_FILE_MACHINE_MIPS16: u32 = 614;
pub const IMAGE_FILE_MACHINE_MIPSFPU: u32 = 870;
pub const IMAGE_FILE_MACHINE_MIPSFPU16: u32 = 1126;
pub const IMAGE_FILE_MACHINE_POWERPC: u32 = 496;
pub const IMAGE_FILE_MACHINE_POWERPCFP: u32 = 497;
pub const IMAGE_FILE_MACHINE_R10000: u32 = 360;
pub const IMAGE_FILE_MACHINE_R3000: u32 = 354;
pub const IMAGE_FILE_MACHINE_R4000: u32 = 358;
pub const IMAGE_FILE_MACHINE_SH3: u32 = 418;
pub const IMAGE_FILE_MACHINE_SH3DSP: u32 = 419;
pub const IMAGE_FILE_MACHINE_SH3E: u32 = 420;
pub const IMAGE_FILE_MACHINE_SH4: u32 = 422;
pub const IMAGE_FILE_MACHINE_SH5: u32 = 424;
pub const IMAGE_FILE_MACHINE_TARGET_HOST: u32 = 1;
pub const IMAGE_FILE_MACHINE_THUMB: u32 = 450;
pub const IMAGE_FILE_MACHINE_TRICORE: u32 = 1312;
pub const IMAGE_FILE_MACHINE_UNKNOWN: u32 = 0;
pub const IMAGE_FILE_MACHINE_WCEMIPSV2: u32 = 361;
pub const IMAGE_FILE_NET_RUN_FROM_SWAP: u32 = 2048;
pub const IMAGE_FILE_RELOCS_STRIPPED: u32 = 1;
pub const IMAGE_FILE_REMOVABLE_RUN_FROM_SWAP: u32 = 1024;
pub const IMAGE_FILE_SYSTEM: u32 = 4096;
pub const IMAGE_FILE_UP_SYSTEM_ONLY: u32 = 16384;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_FUNCTION_ENTRY {
    pub StartingAddress: u32,
    pub EndingAddress: u32,
    pub EndOfPrologue: u32,
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct IMAGE_FUNCTION_ENTRY64 {
    pub StartingAddress: u64,
    pub EndingAddress: u64,
    pub Anonymous: IMAGE_FUNCTION_ENTRY64_0,
}
impl Default for IMAGE_FUNCTION_ENTRY64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub union IMAGE_FUNCTION_ENTRY64_0 {
    pub EndOfPrologue: u64,
    pub UnwindInfoAddress: u64,
}
impl Default for IMAGE_FUNCTION_ENTRY64_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IMAGE_FUNCTION_OVERRIDE_ARM64_BRANCH26: u32 = 2;
pub const IMAGE_FUNCTION_OVERRIDE_ARM64_THUNK: u32 = 3;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct IMAGE_FUNCTION_OVERRIDE_DYNAMIC_RELOCATION {
    pub OriginalRva: u32,
    pub BDDOffset: u32,
    pub RvaSize: u32,
    pub BaseRelocSize: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct IMAGE_FUNCTION_OVERRIDE_HEADER {
    pub FuncOverrideSize: u32,
}
pub const IMAGE_FUNCTION_OVERRIDE_INVALID: u32 = 0;
pub const IMAGE_FUNCTION_OVERRIDE_X64_REL32: u32 = 1;
pub const IMAGE_GUARD_CASTGUARD_PRESENT: u32 = 16777216;
pub const IMAGE_GUARD_CFW_INSTRUMENTED: u32 = 512;
pub const IMAGE_GUARD_CF_ENABLE_EXPORT_SUPPRESSION: u32 = 32768;
pub const IMAGE_GUARD_CF_EXPORT_SUPPRESSION_INFO_PRESENT: u32 = 16384;
pub const IMAGE_GUARD_CF_FUNCTION_TABLE_PRESENT: u32 = 1024;
pub const IMAGE_GUARD_CF_FUNCTION_TABLE_SIZE_MASK: u32 = 4026531840;
pub const IMAGE_GUARD_CF_FUNCTION_TABLE_SIZE_SHIFT: u32 = 28;
pub const IMAGE_GUARD_CF_INSTRUMENTED: u32 = 256;
pub const IMAGE_GUARD_CF_LONGJUMP_TABLE_PRESENT: u32 = 65536;
pub const IMAGE_GUARD_DELAYLOAD_IAT_IN_ITS_OWN_SECTION: u32 = 8192;
pub const IMAGE_GUARD_EH_CONTINUATION_TABLE_PRESENT: u32 = 4194304;
pub const IMAGE_GUARD_FLAG_EXPORT_SUPPRESSED: u32 = 2;
pub const IMAGE_GUARD_FLAG_FID_LANGEXCPTHANDLER: u32 = 4;
pub const IMAGE_GUARD_FLAG_FID_SUPPRESSED: u32 = 1;
pub const IMAGE_GUARD_FLAG_FID_XFG: u32 = 8;
pub const IMAGE_GUARD_MEMCPY_PRESENT: u32 = 33554432;
pub const IMAGE_GUARD_PROTECT_DELAYLOAD_IAT: u32 = 4096;
pub const IMAGE_GUARD_RETPOLINE_PRESENT: u32 = 1048576;
pub const IMAGE_GUARD_RF_ENABLE: u32 = 262144;
pub const IMAGE_GUARD_RF_INSTRUMENTED: u32 = 131072;
pub const IMAGE_GUARD_RF_STRICT: u32 = 524288;
pub const IMAGE_GUARD_SECURITY_COOKIE_UNUSED: u32 = 2048;
pub const IMAGE_GUARD_XFG_ENABLED: u32 = 8388608;
pub type IMAGE_HOTSWAP_ARM64_ENDPOINT_INFO_CC_RETURN = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGE_HOTSWAP_ARM64_ENDPOINT_INFO_ENTRY_V2 {
    pub Common: IMAGE_HOTSWAP_ENDPOINT_INFO_ENTRY_COMMON,
    pub IntArgs: u32,
    pub FloatArgs: u32,
    pub ArgStackSize: u32,
    pub ReturnType: IMAGE_HOTSWAP_ARM64_ENDPOINT_INFO_CC_RETURN,
    pub Name: [u8; 1],
}
impl Default for IMAGE_HOTSWAP_ARM64_ENDPOINT_INFO_ENTRY_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_HOTSWAP_ENDPOINT_INFO_ENTRY_COMMON {
    pub Size: u32,
    pub Rva: u32,
    pub NameSize: u32,
    pub NameOffset: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_HOTSWAP_ENDPOINT_INFO_HEADER_COMMON {
    pub Version: u32,
    pub Size: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_HOTSWAP_ENDPOINT_INFO_HEADER_V2 {
    pub Common: IMAGE_HOTSWAP_ENDPOINT_INFO_HEADER_COMMON,
    pub Count: u32,
}
pub const IMAGE_HOTSWAP_ENDPOINT_INFO_V2: u32 = 2;
pub const IMAGE_HOTSWAP_ENDPOINT_TABLE_SECTION: windows_core::PCSTR = windows_core::s!(".shsept");
pub type IMAGE_HOTSWAP_X64_ENDPOINT_INFO_CC_REG = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGE_HOTSWAP_X64_ENDPOINT_INFO_ENTRY_V2 {
    pub Common: IMAGE_HOTSWAP_ENDPOINT_INFO_ENTRY_COMMON,
    pub ArgRegs: [IMAGE_HOTSWAP_X64_ENDPOINT_INFO_CC_REG; 4],
    pub ArgStackSize: u32,
    pub RetReg: IMAGE_HOTSWAP_X64_ENDPOINT_INFO_CC_REG,
    pub Name: [u8; 1],
}
impl Default for IMAGE_HOTSWAP_X64_ENDPOINT_INFO_ENTRY_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IMAGE_HOT_PATCH_ABSOLUTE: u32 = 180224;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_HOT_PATCH_BASE {
    pub SequenceNumber: u32,
    pub Flags: u32,
    pub OriginalTimeDateStamp: u32,
    pub OriginalCheckSum: u32,
    pub CodeIntegrityInfo: u32,
    pub CodeIntegritySize: u32,
    pub PatchTable: u32,
    pub BufferOffset: u32,
}
pub const IMAGE_HOT_PATCH_BASE_CAN_ROLL_BACK: u32 = 2;
pub const IMAGE_HOT_PATCH_BASE_MACHINE_AMD64: u32 = 16;
pub const IMAGE_HOT_PATCH_BASE_MACHINE_ARM64: u32 = 8;
pub const IMAGE_HOT_PATCH_BASE_MACHINE_I386: u32 = 4;
pub const IMAGE_HOT_PATCH_BASE_OBLIGATORY: u32 = 1;
pub const IMAGE_HOT_PATCH_CALL_TARGET: u32 = 278528;
pub const IMAGE_HOT_PATCH_CHUNK_INVERSE: u32 = 2147483648;
pub const IMAGE_HOT_PATCH_CHUNK_OBLIGATORY: u32 = 1073741824;
pub const IMAGE_HOT_PATCH_CHUNK_RESERVED: u32 = 1072705536;
pub const IMAGE_HOT_PATCH_CHUNK_SIZE: u32 = 4095;
pub const IMAGE_HOT_PATCH_CHUNK_SOURCE_RVA: u32 = 32768;
pub const IMAGE_HOT_PATCH_CHUNK_TARGET_RVA: u32 = 16384;
pub const IMAGE_HOT_PATCH_CHUNK_TYPE: u32 = 1032192;
pub const IMAGE_HOT_PATCH_DYNAMIC_VALUE: u32 = 491520;
pub const IMAGE_HOT_PATCH_FUNCTION: u32 = 114688;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGE_HOT_PATCH_HASHES {
    pub SHA256: [u8; 32],
    pub SHA1: [u8; 20],
}
impl Default for IMAGE_HOT_PATCH_HASHES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IMAGE_HOT_PATCH_INDIRECT: u32 = 376832;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_HOT_PATCH_INFO {
    pub Version: u32,
    pub Size: u32,
    pub SequenceNumber: u32,
    pub BaseImageList: u32,
    pub BaseImageCount: u32,
    pub BufferOffset: u32,
    pub ExtraPatchSize: u32,
    pub MinSequenceNumber: u32,
    pub Flags: u32,
}
pub const IMAGE_HOT_PATCH_INFO_FLAG_HOTSWAP: u32 = 2;
pub const IMAGE_HOT_PATCH_INFO_FLAG_PATCHORDERCRITICAL: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_HOT_PATCH_MACHINE {
    pub Anonymous: IMAGE_HOT_PATCH_MACHINE_0,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_HOT_PATCH_MACHINE_0 {
    pub _bitfield: u32,
}
pub const IMAGE_HOT_PATCH_NONE: u32 = 0;
pub const IMAGE_HOT_PATCH_NO_CALL_TARGET: u32 = 409600;
pub const IMAGE_HOT_PATCH_REL32: u32 = 245760;
pub type IMAGE_IA64_RUNTIME_FUNCTION_ENTRY = _IMAGE_RUNTIME_FUNCTION_ENTRY;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGE_IMPORT_BY_NAME {
    pub Hint: u16,
    pub Name: [i8; 1],
}
impl Default for IMAGE_IMPORT_BY_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct IMAGE_IMPORT_CONTROL_TRANSFER_ARM64_RELOCATION {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct IMAGE_IMPORT_CONTROL_TRANSFER_DYNAMIC_RELOCATION {
    pub _bitfield: u32,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub type IMAGE_IMPORT_CONTROL_TRANSFER_RELOCATION = IMAGE_IMPORT_CONTROL_TRANSFER_DYNAMIC_RELOCATION;
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
pub type IMAGE_IMPORT_CONTROL_TRANSFER_RELOCATION = IMAGE_IMPORT_CONTROL_TRANSFER_ARM64_RELOCATION;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMAGE_IMPORT_DESCRIPTOR {
    pub Anonymous: IMAGE_IMPORT_DESCRIPTOR_0,
    pub TimeDateStamp: u32,
    pub ForwarderChain: u32,
    pub Name: u32,
    pub FirstThunk: u32,
}
impl Default for IMAGE_IMPORT_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_IMPORT_DESCRIPTOR_0 {
    pub Characteristics: u32,
    pub OriginalFirstThunk: u32,
}
impl Default for IMAGE_IMPORT_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct IMAGE_INDIR_CONTROL_TRANSFER_DYNAMIC_RELOCATION {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMAGE_LINENUMBER {
    pub Type: IMAGE_LINENUMBER_0,
    pub Linenumber: u16,
}
impl Default for IMAGE_LINENUMBER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub union IMAGE_LINENUMBER_0 {
    pub SymbolTableIndex: u32,
    pub VirtualAddress: u32,
}
impl Default for IMAGE_LINENUMBER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_LOAD_CONFIG_CODE_INTEGRITY {
    pub Flags: u16,
    pub Catalog: u16,
    pub CatalogOffset: u32,
    pub Reserved: u32,
}
#[cfg(target_arch = "x86")]
pub type IMAGE_LOAD_CONFIG_DIRECTORY = IMAGE_LOAD_CONFIG_DIRECTORY32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type IMAGE_LOAD_CONFIG_DIRECTORY = IMAGE_LOAD_CONFIG_DIRECTORY64;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_LOAD_CONFIG_DIRECTORY32 {
    pub Size: u32,
    pub TimeDateStamp: u32,
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub GlobalFlagsClear: u32,
    pub GlobalFlagsSet: u32,
    pub CriticalSectionDefaultTimeout: u32,
    pub DeCommitFreeBlockThreshold: u32,
    pub DeCommitTotalFreeThreshold: u32,
    pub LockPrefixTable: u32,
    pub MaximumAllocationSize: u32,
    pub VirtualMemoryThreshold: u32,
    pub ProcessHeapFlags: u32,
    pub ProcessAffinityMask: u32,
    pub CSDVersion: u16,
    pub DependentLoadFlags: u16,
    pub EditList: u32,
    pub SecurityCookie: u32,
    pub SEHandlerTable: u32,
    pub SEHandlerCount: u32,
    pub GuardCFCheckFunctionPointer: u32,
    pub GuardCFDispatchFunctionPointer: u32,
    pub GuardCFFunctionTable: u32,
    pub GuardCFFunctionCount: u32,
    pub GuardFlags: u32,
    pub CodeIntegrity: IMAGE_LOAD_CONFIG_CODE_INTEGRITY,
    pub GuardAddressTakenIatEntryTable: u32,
    pub GuardAddressTakenIatEntryCount: u32,
    pub GuardLongJumpTargetTable: u32,
    pub GuardLongJumpTargetCount: u32,
    pub DynamicValueRelocTable: u32,
    pub CHPEMetadataPointer: u32,
    pub GuardRFFailureRoutine: u32,
    pub GuardRFFailureRoutineFunctionPointer: u32,
    pub DynamicValueRelocTableOffset: u32,
    pub DynamicValueRelocTableSection: u16,
    pub Reserved2: u16,
    pub GuardRFVerifyStackPointerFunctionPointer: u32,
    pub HotPatchTableOffset: u32,
    pub Reserved3: u32,
    pub EnclaveConfigurationPointer: u32,
    pub VolatileMetadataPointer: u32,
    pub GuardEHContinuationTable: u32,
    pub GuardEHContinuationCount: u32,
    pub GuardXFGCheckFunctionPointer: u32,
    pub GuardXFGDispatchFunctionPointer: u32,
    pub GuardXFGTableDispatchFunctionPointer: u32,
    pub CastGuardOsDeterminedFailureMode: u32,
    pub GuardMemcpyFunctionPointer: u32,
    pub UmaFunctionPointers: u32,
}
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct IMAGE_LOAD_CONFIG_DIRECTORY64 {
    pub Size: u32,
    pub TimeDateStamp: u32,
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub GlobalFlagsClear: u32,
    pub GlobalFlagsSet: u32,
    pub CriticalSectionDefaultTimeout: u32,
    pub DeCommitFreeBlockThreshold: u64,
    pub DeCommitTotalFreeThreshold: u64,
    pub LockPrefixTable: u64,
    pub MaximumAllocationSize: u64,
    pub VirtualMemoryThreshold: u64,
    pub ProcessAffinityMask: u64,
    pub ProcessHeapFlags: u32,
    pub CSDVersion: u16,
    pub DependentLoadFlags: u16,
    pub EditList: u64,
    pub SecurityCookie: u64,
    pub SEHandlerTable: u64,
    pub SEHandlerCount: u64,
    pub GuardCFCheckFunctionPointer: u64,
    pub GuardCFDispatchFunctionPointer: u64,
    pub GuardCFFunctionTable: u64,
    pub GuardCFFunctionCount: u64,
    pub GuardFlags: u32,
    pub CodeIntegrity: IMAGE_LOAD_CONFIG_CODE_INTEGRITY,
    pub GuardAddressTakenIatEntryTable: u64,
    pub GuardAddressTakenIatEntryCount: u64,
    pub GuardLongJumpTargetTable: u64,
    pub GuardLongJumpTargetCount: u64,
    pub DynamicValueRelocTable: u64,
    pub CHPEMetadataPointer: u64,
    pub GuardRFFailureRoutine: u64,
    pub GuardRFFailureRoutineFunctionPointer: u64,
    pub DynamicValueRelocTableOffset: u32,
    pub DynamicValueRelocTableSection: u16,
    pub Reserved2: u16,
    pub GuardRFVerifyStackPointerFunctionPointer: u64,
    pub HotPatchTableOffset: u32,
    pub Reserved3: u32,
    pub EnclaveConfigurationPointer: u64,
    pub VolatileMetadataPointer: u64,
    pub GuardEHContinuationTable: u64,
    pub GuardEHContinuationCount: u64,
    pub GuardXFGCheckFunctionPointer: u64,
    pub GuardXFGDispatchFunctionPointer: u64,
    pub GuardXFGTableDispatchFunctionPointer: u64,
    pub CastGuardOsDeterminedFailureMode: u64,
    pub GuardMemcpyFunctionPointer: u64,
    pub UmaFunctionPointers: u64,
}
#[cfg(target_arch = "x86")]
pub type IMAGE_NT_HEADERS = IMAGE_NT_HEADERS32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type IMAGE_NT_HEADERS = IMAGE_NT_HEADERS64;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_NT_HEADERS32 {
    pub Signature: u32,
    pub FileHeader: IMAGE_FILE_HEADER,
    pub OptionalHeader: IMAGE_OPTIONAL_HEADER32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct IMAGE_NT_HEADERS64 {
    pub Signature: u32,
    pub FileHeader: IMAGE_FILE_HEADER,
    pub OptionalHeader: IMAGE_OPTIONAL_HEADER64,
}
pub const IMAGE_NT_OPTIONAL_HDR32_MAGIC: u32 = 267;
pub const IMAGE_NT_OPTIONAL_HDR64_MAGIC: u32 = 523;
#[cfg(target_arch = "x86")]
pub const IMAGE_NT_OPTIONAL_HDR_MAGIC: u32 = 267;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const IMAGE_NT_OPTIONAL_HDR_MAGIC: u32 = 523;
pub const IMAGE_NT_SIGNATURE: u32 = 17744;
pub const IMAGE_NUMBEROF_DIRECTORY_ENTRIES: u32 = 16;
#[cfg(target_arch = "x86")]
pub type IMAGE_OPTIONAL_HEADER = IMAGE_OPTIONAL_HEADER32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type IMAGE_OPTIONAL_HEADER = IMAGE_OPTIONAL_HEADER64;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGE_OPTIONAL_HEADER32 {
    pub Magic: u16,
    pub MajorLinkerVersion: u8,
    pub MinorLinkerVersion: u8,
    pub SizeOfCode: u32,
    pub SizeOfInitializedData: u32,
    pub SizeOfUninitializedData: u32,
    pub AddressOfEntryPoint: u32,
    pub BaseOfCode: u32,
    pub BaseOfData: u32,
    pub ImageBase: u32,
    pub SectionAlignment: u32,
    pub FileAlignment: u32,
    pub MajorOperatingSystemVersion: u16,
    pub MinorOperatingSystemVersion: u16,
    pub MajorImageVersion: u16,
    pub MinorImageVersion: u16,
    pub MajorSubsystemVersion: u16,
    pub MinorSubsystemVersion: u16,
    pub Win32VersionValue: u32,
    pub SizeOfImage: u32,
    pub SizeOfHeaders: u32,
    pub CheckSum: u32,
    pub Subsystem: u16,
    pub DllCharacteristics: u16,
    pub SizeOfStackReserve: u32,
    pub SizeOfStackCommit: u32,
    pub SizeOfHeapReserve: u32,
    pub SizeOfHeapCommit: u32,
    pub LoaderFlags: u32,
    pub NumberOfRvaAndSizes: u32,
    pub DataDirectory: [IMAGE_DATA_DIRECTORY; 16],
}
impl Default for IMAGE_OPTIONAL_HEADER32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct IMAGE_OPTIONAL_HEADER64 {
    pub Magic: u16,
    pub MajorLinkerVersion: u8,
    pub MinorLinkerVersion: u8,
    pub SizeOfCode: u32,
    pub SizeOfInitializedData: u32,
    pub SizeOfUninitializedData: u32,
    pub AddressOfEntryPoint: u32,
    pub BaseOfCode: u32,
    pub ImageBase: u64,
    pub SectionAlignment: u32,
    pub FileAlignment: u32,
    pub MajorOperatingSystemVersion: u16,
    pub MinorOperatingSystemVersion: u16,
    pub MajorImageVersion: u16,
    pub MinorImageVersion: u16,
    pub MajorSubsystemVersion: u16,
    pub MinorSubsystemVersion: u16,
    pub Win32VersionValue: u32,
    pub SizeOfImage: u32,
    pub SizeOfHeaders: u32,
    pub CheckSum: u32,
    pub Subsystem: u16,
    pub DllCharacteristics: u16,
    pub SizeOfStackReserve: u64,
    pub SizeOfStackCommit: u64,
    pub SizeOfHeapReserve: u64,
    pub SizeOfHeapCommit: u64,
    pub LoaderFlags: u32,
    pub NumberOfRvaAndSizes: u32,
    pub DataDirectory: [IMAGE_DATA_DIRECTORY; 16],
}
impl Default for IMAGE_OPTIONAL_HEADER64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
pub const IMAGE_ORDINAL_FLAG: i32 = -2147483648;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const IMAGE_ORDINAL_FLAG: u32 = 0;
pub const IMAGE_ORDINAL_FLAG32: u32 = 2147483648;
pub const IMAGE_ORDINAL_FLAG64: u64 = 9223372036854775808;
#[repr(C, packed(2))]
#[derive(Clone, Copy, Default)]
pub struct IMAGE_OS2_HEADER {
    pub ne_magic: u16,
    pub ne_ver: i8,
    pub ne_rev: i8,
    pub ne_enttab: u16,
    pub ne_cbenttab: u16,
    pub ne_crc: i32,
    pub ne_flags: u16,
    pub ne_autodata: u16,
    pub ne_heap: u16,
    pub ne_stack: u16,
    pub ne_csip: i32,
    pub ne_sssp: i32,
    pub ne_cseg: u16,
    pub ne_cmod: u16,
    pub ne_cbnrestab: u16,
    pub ne_segtab: u16,
    pub ne_rsrctab: u16,
    pub ne_restab: u16,
    pub ne_modtab: u16,
    pub ne_imptab: u16,
    pub ne_nrestab: i32,
    pub ne_cmovent: u16,
    pub ne_align: u16,
    pub ne_cres: u16,
    pub ne_exetyp: u8,
    pub ne_flagsothers: u8,
    pub ne_pretthunks: u16,
    pub ne_psegrefbytes: u16,
    pub ne_swaparea: u16,
    pub ne_expver: u16,
}
pub const IMAGE_OS2_SIGNATURE: u32 = 17742;
pub const IMAGE_OS2_SIGNATURE_LE: u32 = 17740;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMAGE_POLICY_ENTRY {
    pub Type: IMAGE_POLICY_ENTRY_TYPE,
    pub PolicyId: IMAGE_POLICY_ID,
    pub u: IMAGE_POLICY_ENTRY_0,
}
impl Default for IMAGE_POLICY_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_POLICY_ENTRY_0 {
    pub None: *const core::ffi::c_void,
    pub BoolValue: bool,
    pub Int8Value: i8,
    pub UInt8Value: u8,
    pub Int16Value: i16,
    pub UInt16Value: u16,
    pub Int32Value: i32,
    pub UInt32Value: u32,
    pub Int64Value: i64,
    pub UInt64Value: u64,
    pub AnsiStringValue: windows_core::PCSTR,
    pub UnicodeStringValue: windows_core::PCWSTR,
}
impl Default for IMAGE_POLICY_ENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type IMAGE_POLICY_ENTRY_TYPE = i32;
pub type IMAGE_POLICY_ID = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMAGE_POLICY_METADATA {
    pub Version: u8,
    pub Reserved0: [u8; 7],
    pub ApplicationId: u64,
    pub Policies: [IMAGE_POLICY_ENTRY; 0],
}
impl Default for IMAGE_POLICY_METADATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IMAGE_POLICY_METADATA_VERSION: u32 = 1;
pub const IMAGE_POLICY_SECTION_NAME: windows_core::PCSTR = windows_core::s!(".tPolicy");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_PROLOGUE_DYNAMIC_RELOCATION_HEADER {
    pub PrologueByteCount: u8,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct IMAGE_RELOCATION {
    pub Anonymous: IMAGE_RELOCATION_0,
    pub SymbolTableIndex: u32,
    pub Type: u16,
}
impl Default for IMAGE_RELOCATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub union IMAGE_RELOCATION_0 {
    pub VirtualAddress: u32,
    pub RelocCount: u32,
}
impl Default for IMAGE_RELOCATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IMAGE_REL_ALPHA_ABSOLUTE: u32 = 0;
pub const IMAGE_REL_ALPHA_BRADDR: u32 = 7;
pub const IMAGE_REL_ALPHA_GPDISP: u32 = 6;
pub const IMAGE_REL_ALPHA_GPREL32: u32 = 3;
pub const IMAGE_REL_ALPHA_GPRELHI: u32 = 23;
pub const IMAGE_REL_ALPHA_GPRELLO: u32 = 22;
pub const IMAGE_REL_ALPHA_HINT: u32 = 8;
pub const IMAGE_REL_ALPHA_INLINE_REFLONG: u32 = 9;
pub const IMAGE_REL_ALPHA_LITERAL: u32 = 4;
pub const IMAGE_REL_ALPHA_LITUSE: u32 = 5;
pub const IMAGE_REL_ALPHA_MATCH: u32 = 13;
pub const IMAGE_REL_ALPHA_PAIR: u32 = 12;
pub const IMAGE_REL_ALPHA_REFHI: u32 = 10;
pub const IMAGE_REL_ALPHA_REFLO: u32 = 11;
pub const IMAGE_REL_ALPHA_REFLONG: u32 = 1;
pub const IMAGE_REL_ALPHA_REFLONGNB: u32 = 16;
pub const IMAGE_REL_ALPHA_REFQ1: u32 = 21;
pub const IMAGE_REL_ALPHA_REFQ2: u32 = 20;
pub const IMAGE_REL_ALPHA_REFQ3: u32 = 19;
pub const IMAGE_REL_ALPHA_REFQUAD: u32 = 2;
pub const IMAGE_REL_ALPHA_SECREL: u32 = 15;
pub const IMAGE_REL_ALPHA_SECRELHI: u32 = 18;
pub const IMAGE_REL_ALPHA_SECRELLO: u32 = 17;
pub const IMAGE_REL_ALPHA_SECTION: u32 = 14;
pub const IMAGE_REL_AMD64_ABSOLUTE: u32 = 0;
pub const IMAGE_REL_AMD64_ADDR32: u32 = 2;
pub const IMAGE_REL_AMD64_ADDR32NB: u32 = 3;
pub const IMAGE_REL_AMD64_ADDR64: u32 = 1;
pub const IMAGE_REL_AMD64_CFG_BR: u32 = 20;
pub const IMAGE_REL_AMD64_CFG_BR_REX: u32 = 21;
pub const IMAGE_REL_AMD64_CFG_CALL: u32 = 22;
pub const IMAGE_REL_AMD64_EHANDLER: u32 = 17;
pub const IMAGE_REL_AMD64_IMPORT_BR: u32 = 18;
pub const IMAGE_REL_AMD64_IMPORT_CALL: u32 = 19;
pub const IMAGE_REL_AMD64_INDIR_BR: u32 = 23;
pub const IMAGE_REL_AMD64_INDIR_BR_REX: u32 = 24;
pub const IMAGE_REL_AMD64_INDIR_BR_SWITCHTABLE_FIRST: u32 = 32;
pub const IMAGE_REL_AMD64_INDIR_BR_SWITCHTABLE_LAST: u32 = 47;
pub const IMAGE_REL_AMD64_INDIR_CALL: u32 = 25;
pub const IMAGE_REL_AMD64_PAIR: u32 = 15;
pub const IMAGE_REL_AMD64_REL32: u32 = 4;
pub const IMAGE_REL_AMD64_REL32_1: u32 = 5;
pub const IMAGE_REL_AMD64_REL32_2: u32 = 6;
pub const IMAGE_REL_AMD64_REL32_3: u32 = 7;
pub const IMAGE_REL_AMD64_REL32_4: u32 = 8;
pub const IMAGE_REL_AMD64_REL32_5: u32 = 9;
pub const IMAGE_REL_AMD64_SECREL: u32 = 11;
pub const IMAGE_REL_AMD64_SECREL7: u32 = 12;
pub const IMAGE_REL_AMD64_SECTION: u32 = 10;
pub const IMAGE_REL_AMD64_SREL32: u32 = 14;
pub const IMAGE_REL_AMD64_SSPAN32: u32 = 16;
pub const IMAGE_REL_AMD64_TOKEN: u32 = 13;
pub const IMAGE_REL_AM_ABSOLUTE: u32 = 0;
pub const IMAGE_REL_AM_ADDR32: u32 = 1;
pub const IMAGE_REL_AM_ADDR32NB: u32 = 2;
pub const IMAGE_REL_AM_CALL32: u32 = 3;
pub const IMAGE_REL_AM_FUNCINFO: u32 = 4;
pub const IMAGE_REL_AM_REL32_1: u32 = 5;
pub const IMAGE_REL_AM_REL32_2: u32 = 6;
pub const IMAGE_REL_AM_SECREL: u32 = 7;
pub const IMAGE_REL_AM_SECTION: u32 = 8;
pub const IMAGE_REL_AM_TOKEN: u32 = 9;
pub const IMAGE_REL_ARM64_ABSOLUTE: u32 = 0;
pub const IMAGE_REL_ARM64_ADDR32: u32 = 1;
pub const IMAGE_REL_ARM64_ADDR32NB: u32 = 2;
pub const IMAGE_REL_ARM64_ADDR64: u32 = 14;
pub const IMAGE_REL_ARM64_BRANCH19: u32 = 15;
pub const IMAGE_REL_ARM64_BRANCH26: u32 = 3;
pub const IMAGE_REL_ARM64_PAGEBASE_REL21: u32 = 4;
pub const IMAGE_REL_ARM64_PAGEOFFSET_12A: u32 = 6;
pub const IMAGE_REL_ARM64_PAGEOFFSET_12L: u32 = 7;
pub const IMAGE_REL_ARM64_REL21: u32 = 5;
pub const IMAGE_REL_ARM64_SECREL: u32 = 8;
pub const IMAGE_REL_ARM64_SECREL_HIGH12A: u32 = 10;
pub const IMAGE_REL_ARM64_SECREL_LOW12A: u32 = 9;
pub const IMAGE_REL_ARM64_SECREL_LOW12L: u32 = 11;
pub const IMAGE_REL_ARM64_SECTION: u32 = 13;
pub const IMAGE_REL_ARM64_TOKEN: u32 = 12;
pub const IMAGE_REL_ARM_ABSOLUTE: u32 = 0;
pub const IMAGE_REL_ARM_ADDR32: u32 = 1;
pub const IMAGE_REL_ARM_ADDR32NB: u32 = 2;
pub const IMAGE_REL_ARM_BLX11: u32 = 9;
pub const IMAGE_REL_ARM_BLX23T: u32 = 21;
pub const IMAGE_REL_ARM_BLX24: u32 = 8;
pub const IMAGE_REL_ARM_BRANCH11: u32 = 4;
pub const IMAGE_REL_ARM_BRANCH20T: u32 = 18;
pub const IMAGE_REL_ARM_BRANCH24: u32 = 3;
pub const IMAGE_REL_ARM_BRANCH24T: u32 = 20;
pub const IMAGE_REL_ARM_GPREL12: u32 = 6;
pub const IMAGE_REL_ARM_GPREL7: u32 = 7;
pub const IMAGE_REL_ARM_MOV32: u32 = 16;
pub const IMAGE_REL_ARM_MOV32A: u32 = 16;
pub const IMAGE_REL_ARM_MOV32T: u32 = 17;
pub const IMAGE_REL_ARM_SECREL: u32 = 15;
pub const IMAGE_REL_ARM_SECTION: u32 = 14;
pub const IMAGE_REL_ARM_TOKEN: u32 = 5;
pub const IMAGE_REL_BASED_ABSOLUTE: u32 = 0;
pub const IMAGE_REL_BASED_ARM_MOV32: u32 = 5;
pub const IMAGE_REL_BASED_DIR64: u32 = 10;
pub const IMAGE_REL_BASED_HIGH: u32 = 1;
pub const IMAGE_REL_BASED_HIGHADJ: u32 = 4;
pub const IMAGE_REL_BASED_HIGHLOW: u32 = 3;
pub const IMAGE_REL_BASED_IA64_IMM64: u32 = 9;
pub const IMAGE_REL_BASED_LOW: u32 = 2;
pub const IMAGE_REL_BASED_MACHINE_SPECIFIC_5: u32 = 5;
pub const IMAGE_REL_BASED_MACHINE_SPECIFIC_7: u32 = 7;
pub const IMAGE_REL_BASED_MACHINE_SPECIFIC_8: u32 = 8;
pub const IMAGE_REL_BASED_MACHINE_SPECIFIC_9: u32 = 9;
pub const IMAGE_REL_BASED_MIPS_JMPADDR: u32 = 5;
pub const IMAGE_REL_BASED_MIPS_JMPADDR16: u32 = 9;
pub const IMAGE_REL_BASED_RESERVED: u32 = 6;
pub const IMAGE_REL_BASED_THUMB_MOV32: u32 = 7;
pub const IMAGE_REL_CEE_ABSOLUTE: u32 = 0;
pub const IMAGE_REL_CEE_ADDR32: u32 = 1;
pub const IMAGE_REL_CEE_ADDR32NB: u32 = 3;
pub const IMAGE_REL_CEE_ADDR64: u32 = 2;
pub const IMAGE_REL_CEE_SECREL: u32 = 5;
pub const IMAGE_REL_CEE_SECTION: u32 = 4;
pub const IMAGE_REL_CEE_TOKEN: u32 = 6;
pub const IMAGE_REL_CEF_ABSOLUTE: u32 = 0;
pub const IMAGE_REL_CEF_ADDR32: u32 = 1;
pub const IMAGE_REL_CEF_ADDR32NB: u32 = 3;
pub const IMAGE_REL_CEF_ADDR64: u32 = 2;
pub const IMAGE_REL_CEF_SECREL: u32 = 5;
pub const IMAGE_REL_CEF_SECTION: u32 = 4;
pub const IMAGE_REL_CEF_TOKEN: u32 = 6;
pub const IMAGE_REL_EBC_ABSOLUTE: u32 = 0;
pub const IMAGE_REL_EBC_ADDR32NB: u32 = 1;
pub const IMAGE_REL_EBC_REL32: u32 = 2;
pub const IMAGE_REL_EBC_SECREL: u32 = 4;
pub const IMAGE_REL_EBC_SECTION: u32 = 3;
pub const IMAGE_REL_I386_ABSOLUTE: u32 = 0;
pub const IMAGE_REL_I386_DIR16: u32 = 1;
pub const IMAGE_REL_I386_DIR32: u32 = 6;
pub const IMAGE_REL_I386_DIR32NB: u32 = 7;
pub const IMAGE_REL_I386_REL16: u32 = 2;
pub const IMAGE_REL_I386_REL32: u32 = 20;
pub const IMAGE_REL_I386_SECREL: u32 = 11;
pub const IMAGE_REL_I386_SECREL7: u32 = 13;
pub const IMAGE_REL_I386_SECTION: u32 = 10;
pub const IMAGE_REL_I386_SEG12: u32 = 9;
pub const IMAGE_REL_I386_TOKEN: u32 = 12;
pub const IMAGE_REL_IA64_ABSOLUTE: u32 = 0;
pub const IMAGE_REL_IA64_ADDEND: u32 = 31;
pub const IMAGE_REL_IA64_DIR32: u32 = 4;
pub const IMAGE_REL_IA64_DIR32NB: u32 = 16;
pub const IMAGE_REL_IA64_DIR64: u32 = 5;
pub const IMAGE_REL_IA64_GPREL22: u32 = 9;
pub const IMAGE_REL_IA64_GPREL32: u32 = 28;
pub const IMAGE_REL_IA64_IMM14: u32 = 1;
pub const IMAGE_REL_IA64_IMM22: u32 = 2;
pub const IMAGE_REL_IA64_IMM64: u32 = 3;
pub const IMAGE_REL_IA64_IMMGPREL64: u32 = 26;
pub const IMAGE_REL_IA64_LTOFF22: u32 = 10;
pub const IMAGE_REL_IA64_PCREL21B: u32 = 6;
pub const IMAGE_REL_IA64_PCREL21F: u32 = 8;
pub const IMAGE_REL_IA64_PCREL21M: u32 = 7;
pub const IMAGE_REL_IA64_PCREL60B: u32 = 22;
pub const IMAGE_REL_IA64_PCREL60F: u32 = 23;
pub const IMAGE_REL_IA64_PCREL60I: u32 = 24;
pub const IMAGE_REL_IA64_PCREL60M: u32 = 25;
pub const IMAGE_REL_IA64_PCREL60X: u32 = 21;
pub const IMAGE_REL_IA64_SECREL22: u32 = 12;
pub const IMAGE_REL_IA64_SECREL32: u32 = 14;
pub const IMAGE_REL_IA64_SECREL64I: u32 = 13;
pub const IMAGE_REL_IA64_SECTION: u32 = 11;
pub const IMAGE_REL_IA64_SREL14: u32 = 17;
pub const IMAGE_REL_IA64_SREL22: u32 = 18;
pub const IMAGE_REL_IA64_SREL32: u32 = 19;
pub const IMAGE_REL_IA64_TOKEN: u32 = 27;
pub const IMAGE_REL_IA64_UREL32: u32 = 20;
pub const IMAGE_REL_M32R_ABSOLUTE: u32 = 0;
pub const IMAGE_REL_M32R_ADDR24: u32 = 3;
pub const IMAGE_REL_M32R_ADDR32: u32 = 1;
pub const IMAGE_REL_M32R_ADDR32NB: u32 = 2;
pub const IMAGE_REL_M32R_GPREL16: u32 = 4;
pub const IMAGE_REL_M32R_PAIR: u32 = 11;
pub const IMAGE_REL_M32R_PCREL16: u32 = 6;
pub const IMAGE_REL_M32R_PCREL24: u32 = 5;
pub const IMAGE_REL_M32R_PCREL8: u32 = 7;
pub const IMAGE_REL_M32R_REFHALF: u32 = 8;
pub const IMAGE_REL_M32R_REFHI: u32 = 9;
pub const IMAGE_REL_M32R_REFLO: u32 = 10;
pub const IMAGE_REL_M32R_SECREL32: u32 = 13;
pub const IMAGE_REL_M32R_SECTION: u32 = 12;
pub const IMAGE_REL_M32R_TOKEN: u32 = 14;
pub const IMAGE_REL_MIPS_ABSOLUTE: u32 = 0;
pub const IMAGE_REL_MIPS_GPREL: u32 = 6;
pub const IMAGE_REL_MIPS_JMPADDR: u32 = 3;
pub const IMAGE_REL_MIPS_JMPADDR16: u32 = 16;
pub const IMAGE_REL_MIPS_LITERAL: u32 = 7;
pub const IMAGE_REL_MIPS_PAIR: u32 = 37;
pub const IMAGE_REL_MIPS_REFHALF: u32 = 1;
pub const IMAGE_REL_MIPS_REFHI: u32 = 4;
pub const IMAGE_REL_MIPS_REFLO: u32 = 5;
pub const IMAGE_REL_MIPS_REFWORD: u32 = 2;
pub const IMAGE_REL_MIPS_REFWORDNB: u32 = 34;
pub const IMAGE_REL_MIPS_SECREL: u32 = 11;
pub const IMAGE_REL_MIPS_SECRELHI: u32 = 13;
pub const IMAGE_REL_MIPS_SECRELLO: u32 = 12;
pub const IMAGE_REL_MIPS_SECTION: u32 = 10;
pub const IMAGE_REL_MIPS_TOKEN: u32 = 14;
pub const IMAGE_REL_PPC_ABSOLUTE: u32 = 0;
pub const IMAGE_REL_PPC_ADDR14: u32 = 5;
pub const IMAGE_REL_PPC_ADDR16: u32 = 4;
pub const IMAGE_REL_PPC_ADDR24: u32 = 3;
pub const IMAGE_REL_PPC_ADDR32: u32 = 2;
pub const IMAGE_REL_PPC_ADDR32NB: u32 = 10;
pub const IMAGE_REL_PPC_ADDR64: u32 = 1;
pub const IMAGE_REL_PPC_BRNTAKEN: u32 = 1024;
pub const IMAGE_REL_PPC_BRTAKEN: u32 = 512;
pub const IMAGE_REL_PPC_GPREL: u32 = 21;
pub const IMAGE_REL_PPC_IFGLUE: u32 = 13;
pub const IMAGE_REL_PPC_IMGLUE: u32 = 14;
pub const IMAGE_REL_PPC_NEG: u32 = 256;
pub const IMAGE_REL_PPC_PAIR: u32 = 18;
pub const IMAGE_REL_PPC_REFHI: u32 = 16;
pub const IMAGE_REL_PPC_REFLO: u32 = 17;
pub const IMAGE_REL_PPC_REL14: u32 = 7;
pub const IMAGE_REL_PPC_REL24: u32 = 6;
pub const IMAGE_REL_PPC_SECREL: u32 = 11;
pub const IMAGE_REL_PPC_SECREL16: u32 = 15;
pub const IMAGE_REL_PPC_SECRELHI: u32 = 20;
pub const IMAGE_REL_PPC_SECRELLO: u32 = 19;
pub const IMAGE_REL_PPC_SECTION: u32 = 12;
pub const IMAGE_REL_PPC_TOCDEFN: u32 = 2048;
pub const IMAGE_REL_PPC_TOCREL14: u32 = 9;
pub const IMAGE_REL_PPC_TOCREL16: u32 = 8;
pub const IMAGE_REL_PPC_TOKEN: u32 = 22;
pub const IMAGE_REL_PPC_TYPEMASK: u32 = 255;
pub const IMAGE_REL_SH3_ABSOLUTE: u32 = 0;
pub const IMAGE_REL_SH3_DIRECT16: u32 = 1;
pub const IMAGE_REL_SH3_DIRECT32: u32 = 2;
pub const IMAGE_REL_SH3_DIRECT32_NB: u32 = 16;
pub const IMAGE_REL_SH3_DIRECT4: u32 = 6;
pub const IMAGE_REL_SH3_DIRECT4_LONG: u32 = 8;
pub const IMAGE_REL_SH3_DIRECT4_WORD: u32 = 7;
pub const IMAGE_REL_SH3_DIRECT8: u32 = 3;
pub const IMAGE_REL_SH3_DIRECT8_LONG: u32 = 5;
pub const IMAGE_REL_SH3_DIRECT8_WORD: u32 = 4;
pub const IMAGE_REL_SH3_GPREL4_LONG: u32 = 17;
pub const IMAGE_REL_SH3_PCREL12_WORD: u32 = 11;
pub const IMAGE_REL_SH3_PCREL8_LONG: u32 = 10;
pub const IMAGE_REL_SH3_PCREL8_WORD: u32 = 9;
pub const IMAGE_REL_SH3_SECREL: u32 = 15;
pub const IMAGE_REL_SH3_SECTION: u32 = 14;
pub const IMAGE_REL_SH3_SIZEOF_SECTION: u32 = 13;
pub const IMAGE_REL_SH3_STARTOF_SECTION: u32 = 12;
pub const IMAGE_REL_SH3_TOKEN: u32 = 18;
pub const IMAGE_REL_SHM_PAIR: u32 = 24;
pub const IMAGE_REL_SHM_PCRELPT: u32 = 19;
pub const IMAGE_REL_SHM_REFHALF: u32 = 21;
pub const IMAGE_REL_SHM_REFLO: u32 = 20;
pub const IMAGE_REL_SHM_RELHALF: u32 = 23;
pub const IMAGE_REL_SHM_RELLO: u32 = 22;
pub const IMAGE_REL_SH_NOMODE: u32 = 32768;
pub const IMAGE_REL_THUMB_BLX23: u32 = 21;
pub const IMAGE_REL_THUMB_BRANCH20: u32 = 18;
pub const IMAGE_REL_THUMB_BRANCH24: u32 = 20;
pub const IMAGE_REL_THUMB_MOV32: u32 = 17;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_RESOURCE_DATA_ENTRY {
    pub OffsetToData: u32,
    pub Size: u32,
    pub CodePage: u32,
    pub Reserved: u32,
}
pub const IMAGE_RESOURCE_DATA_IS_DIRECTORY: u32 = 2147483648;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_RESOURCE_DIRECTORY {
    pub Characteristics: u32,
    pub TimeDateStamp: u32,
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub NumberOfNamedEntries: u16,
    pub NumberOfIdEntries: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMAGE_RESOURCE_DIRECTORY_ENTRY {
    pub Anonymous: IMAGE_RESOURCE_DIRECTORY_ENTRY_0,
    pub Anonymous2: IMAGE_RESOURCE_DIRECTORY_ENTRY_1,
}
impl Default for IMAGE_RESOURCE_DIRECTORY_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_RESOURCE_DIRECTORY_ENTRY_0 {
    pub Anonymous: IMAGE_RESOURCE_DIRECTORY_ENTRY_0_0,
    pub Name: u32,
    pub Id: u16,
}
impl Default for IMAGE_RESOURCE_DIRECTORY_ENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_RESOURCE_DIRECTORY_ENTRY_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_RESOURCE_DIRECTORY_ENTRY_1 {
    pub OffsetToData: u32,
    pub Anonymous: IMAGE_RESOURCE_DIRECTORY_ENTRY_1_0,
}
impl Default for IMAGE_RESOURCE_DIRECTORY_ENTRY_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_RESOURCE_DIRECTORY_ENTRY_1_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGE_RESOURCE_DIRECTORY_STRING {
    pub Length: u16,
    pub NameString: [i8; 1],
}
impl Default for IMAGE_RESOURCE_DIRECTORY_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGE_RESOURCE_DIR_STRING_U {
    pub Length: u16,
    pub NameString: [u16; 1],
}
impl Default for IMAGE_RESOURCE_DIR_STRING_U {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IMAGE_RESOURCE_NAME_IS_STRING: u32 = 2147483648;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_ROM_HEADERS {
    pub FileHeader: IMAGE_FILE_HEADER,
    pub OptionalHeader: IMAGE_ROM_OPTIONAL_HEADER,
}
pub const IMAGE_ROM_OPTIONAL_HDR_MAGIC: u32 = 263;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGE_ROM_OPTIONAL_HEADER {
    pub Magic: u16,
    pub MajorLinkerVersion: u8,
    pub MinorLinkerVersion: u8,
    pub SizeOfCode: u32,
    pub SizeOfInitializedData: u32,
    pub SizeOfUninitializedData: u32,
    pub AddressOfEntryPoint: u32,
    pub BaseOfCode: u32,
    pub BaseOfData: u32,
    pub BaseOfBss: u32,
    pub GprMask: u32,
    pub CprMask: [u32; 4],
    pub GpValue: u32,
}
impl Default for IMAGE_ROM_OPTIONAL_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
pub type IMAGE_RUNTIME_FUNCTION_ENTRY = _IMAGE_RUNTIME_FUNCTION_ENTRY;
#[cfg(target_arch = "aarch64")]
pub type IMAGE_RUNTIME_FUNCTION_ENTRY = IMAGE_ARM64_RUNTIME_FUNCTION_ENTRY;
pub const IMAGE_SCN_ALIGN_1024BYTES: u32 = 11534336;
pub const IMAGE_SCN_ALIGN_128BYTES: u32 = 8388608;
pub const IMAGE_SCN_ALIGN_16BYTES: u32 = 5242880;
pub const IMAGE_SCN_ALIGN_1BYTES: u32 = 1048576;
pub const IMAGE_SCN_ALIGN_2048BYTES: u32 = 12582912;
pub const IMAGE_SCN_ALIGN_256BYTES: u32 = 9437184;
pub const IMAGE_SCN_ALIGN_2BYTES: u32 = 2097152;
pub const IMAGE_SCN_ALIGN_32BYTES: u32 = 6291456;
pub const IMAGE_SCN_ALIGN_4096BYTES: u32 = 13631488;
pub const IMAGE_SCN_ALIGN_4BYTES: u32 = 3145728;
pub const IMAGE_SCN_ALIGN_512BYTES: u32 = 10485760;
pub const IMAGE_SCN_ALIGN_64BYTES: u32 = 7340032;
pub const IMAGE_SCN_ALIGN_8192BYTES: u32 = 14680064;
pub const IMAGE_SCN_ALIGN_8BYTES: u32 = 4194304;
pub const IMAGE_SCN_ALIGN_MASK: u32 = 15728640;
pub const IMAGE_SCN_CNT_CODE: u32 = 32;
pub const IMAGE_SCN_CNT_INITIALIZED_DATA: u32 = 64;
pub const IMAGE_SCN_CNT_UNINITIALIZED_DATA: u32 = 128;
pub const IMAGE_SCN_GPREL: u32 = 32768;
pub const IMAGE_SCN_LNK_COMDAT: u32 = 4096;
pub const IMAGE_SCN_LNK_INFO: u32 = 512;
pub const IMAGE_SCN_LNK_NRELOC_OVFL: u32 = 16777216;
pub const IMAGE_SCN_LNK_OTHER: u32 = 256;
pub const IMAGE_SCN_LNK_REMOVE: u32 = 2048;
pub const IMAGE_SCN_MEM_16BIT: u32 = 131072;
pub const IMAGE_SCN_MEM_DISCARDABLE: u32 = 33554432;
pub const IMAGE_SCN_MEM_EXECUTE: u32 = 536870912;
pub const IMAGE_SCN_MEM_FARDATA: u32 = 32768;
pub const IMAGE_SCN_MEM_LOCKED: u32 = 262144;
pub const IMAGE_SCN_MEM_NOT_CACHED: u32 = 67108864;
pub const IMAGE_SCN_MEM_NOT_PAGED: u32 = 134217728;
pub const IMAGE_SCN_MEM_PRELOAD: u32 = 524288;
pub const IMAGE_SCN_MEM_PURGEABLE: u32 = 131072;
pub const IMAGE_SCN_MEM_READ: u32 = 1073741824;
pub const IMAGE_SCN_MEM_SHARED: u32 = 268435456;
pub const IMAGE_SCN_MEM_WRITE: u32 = 2147483648;
pub const IMAGE_SCN_NO_DEFER_SPEC_EXC: u32 = 16384;
pub const IMAGE_SCN_SCALE_INDEX: u32 = 1;
pub const IMAGE_SCN_TYPE_NO_PAD: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMAGE_SECTION_HEADER {
    pub Name: [u8; 8],
    pub Misc: IMAGE_SECTION_HEADER_0,
    pub VirtualAddress: u32,
    pub SizeOfRawData: u32,
    pub PointerToRawData: u32,
    pub PointerToRelocations: u32,
    pub PointerToLinenumbers: u32,
    pub NumberOfRelocations: u16,
    pub NumberOfLinenumbers: u16,
    pub Characteristics: u32,
}
impl Default for IMAGE_SECTION_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_SECTION_HEADER_0 {
    pub PhysicalAddress: u32,
    pub VirtualSize: u32,
}
impl Default for IMAGE_SECTION_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IMAGE_SEPARATE_DEBUG_FLAGS_MASK: u32 = 32768;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IMAGE_SEPARATE_DEBUG_HEADER {
    pub Signature: u16,
    pub Flags: u16,
    pub Machine: u16,
    pub Characteristics: u16,
    pub TimeDateStamp: u32,
    pub CheckSum: u32,
    pub ImageBase: u32,
    pub SizeOfImage: u32,
    pub NumberOfSections: u32,
    pub ExportedNamesSize: u32,
    pub DebugDirectorySize: u32,
    pub SectionAlignment: u32,
    pub Reserved: [u32; 2],
}
impl Default for IMAGE_SEPARATE_DEBUG_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IMAGE_SEPARATE_DEBUG_MISMATCH: u32 = 32768;
pub const IMAGE_SEPARATE_DEBUG_SIGNATURE: u32 = 18756;
pub const IMAGE_SIZEOF_ARCHIVE_MEMBER_HDR: u32 = 60;
pub const IMAGE_SIZEOF_FILE_HEADER: u32 = 20;
pub const IMAGE_SIZEOF_SECTION_HEADER: u32 = 40;
pub const IMAGE_SIZEOF_SHORT_NAME: u32 = 8;
pub const IMAGE_SIZEOF_SYMBOL: u32 = 18;
pub const IMAGE_SUBSYSTEM_EFI_APPLICATION: u32 = 10;
pub const IMAGE_SUBSYSTEM_EFI_BOOT_SERVICE_DRIVER: u32 = 11;
pub const IMAGE_SUBSYSTEM_EFI_ROM: u32 = 13;
pub const IMAGE_SUBSYSTEM_EFI_RUNTIME_DRIVER: u32 = 12;
pub const IMAGE_SUBSYSTEM_NATIVE: u32 = 1;
pub const IMAGE_SUBSYSTEM_NATIVE_WINDOWS: u32 = 8;
pub const IMAGE_SUBSYSTEM_OS2_CUI: u32 = 5;
pub const IMAGE_SUBSYSTEM_POSIX_CUI: u32 = 7;
pub const IMAGE_SUBSYSTEM_UNKNOWN: u32 = 0;
pub const IMAGE_SUBSYSTEM_WINDOWS_BOOT_APPLICATION: u32 = 16;
pub const IMAGE_SUBSYSTEM_WINDOWS_CE_GUI: u32 = 9;
pub const IMAGE_SUBSYSTEM_WINDOWS_CUI: u32 = 3;
pub const IMAGE_SUBSYSTEM_WINDOWS_GUI: u32 = 2;
pub const IMAGE_SUBSYSTEM_XBOX: u32 = 14;
pub const IMAGE_SUBSYSTEM_XBOX_CODE_CATALOG: u32 = 17;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct IMAGE_SWITCHTABLE_BRANCH_DYNAMIC_RELOCATION {
    pub _bitfield: u16,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct IMAGE_SYMBOL {
    pub N: IMAGE_SYMBOL_0,
    pub Value: u32,
    pub SectionNumber: i16,
    pub Type: u16,
    pub StorageClass: u8,
    pub NumberOfAuxSymbols: u8,
}
impl Default for IMAGE_SYMBOL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub union IMAGE_SYMBOL_0 {
    pub ShortName: [u8; 8],
    pub Name: IMAGE_SYMBOL_0_0,
    pub LongName: [u32; 2],
}
impl Default for IMAGE_SYMBOL_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[derive(Clone, Copy, Default)]
pub struct IMAGE_SYMBOL_0_0 {
    pub Short: u32,
    pub Long: u32,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct IMAGE_SYMBOL_EX {
    pub N: IMAGE_SYMBOL_EX_0,
    pub Value: u32,
    pub SectionNumber: i32,
    pub Type: u16,
    pub StorageClass: u8,
    pub NumberOfAuxSymbols: u8,
}
impl Default for IMAGE_SYMBOL_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub union IMAGE_SYMBOL_EX_0 {
    pub ShortName: [u8; 8],
    pub Name: IMAGE_SYMBOL_EX_0_0,
    pub LongName: [u32; 2],
}
impl Default for IMAGE_SYMBOL_EX_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(2))]
#[derive(Clone, Copy, Default)]
pub struct IMAGE_SYMBOL_EX_0_0 {
    pub Short: u32,
    pub Long: u32,
}
pub const IMAGE_SYM_ABSOLUTE: i16 = -1;
pub const IMAGE_SYM_CLASS_ARGUMENT: u32 = 9;
pub const IMAGE_SYM_CLASS_AUTOMATIC: u32 = 1;
pub const IMAGE_SYM_CLASS_BIT_FIELD: u32 = 18;
pub const IMAGE_SYM_CLASS_BLOCK: u32 = 100;
pub const IMAGE_SYM_CLASS_CLR_TOKEN: u32 = 107;
pub const IMAGE_SYM_CLASS_END_OF_FUNCTION: u8 = 255;
pub const IMAGE_SYM_CLASS_END_OF_STRUCT: u32 = 102;
pub const IMAGE_SYM_CLASS_ENUM_TAG: u32 = 15;
pub const IMAGE_SYM_CLASS_EXTERNAL: u32 = 2;
pub const IMAGE_SYM_CLASS_EXTERNAL_DEF: u32 = 5;
pub const IMAGE_SYM_CLASS_FAR_EXTERNAL: u32 = 68;
pub const IMAGE_SYM_CLASS_FILE: u32 = 103;
pub const IMAGE_SYM_CLASS_FUNCTION: u32 = 101;
pub const IMAGE_SYM_CLASS_LABEL: u32 = 6;
pub const IMAGE_SYM_CLASS_MEMBER_OF_ENUM: u32 = 16;
pub const IMAGE_SYM_CLASS_MEMBER_OF_STRUCT: u32 = 8;
pub const IMAGE_SYM_CLASS_MEMBER_OF_UNION: u32 = 11;
pub const IMAGE_SYM_CLASS_NULL: u32 = 0;
pub const IMAGE_SYM_CLASS_REGISTER: u32 = 4;
pub const IMAGE_SYM_CLASS_REGISTER_PARAM: u32 = 17;
pub const IMAGE_SYM_CLASS_SECTION: u32 = 104;
pub const IMAGE_SYM_CLASS_STATIC: u32 = 3;
pub const IMAGE_SYM_CLASS_STRUCT_TAG: u32 = 10;
pub const IMAGE_SYM_CLASS_TYPE_DEFINITION: u32 = 13;
pub const IMAGE_SYM_CLASS_UNDEFINED_LABEL: u32 = 7;
pub const IMAGE_SYM_CLASS_UNDEFINED_STATIC: u32 = 14;
pub const IMAGE_SYM_CLASS_UNION_TAG: u32 = 12;
pub const IMAGE_SYM_CLASS_WEAK_EXTERNAL: u32 = 105;
pub const IMAGE_SYM_DEBUG: i16 = -2;
pub const IMAGE_SYM_DTYPE_ARRAY: u32 = 3;
pub const IMAGE_SYM_DTYPE_FUNCTION: u32 = 2;
pub const IMAGE_SYM_DTYPE_NULL: u32 = 0;
pub const IMAGE_SYM_DTYPE_POINTER: u32 = 1;
pub const IMAGE_SYM_SECTION_MAX: u32 = 65279;
pub const IMAGE_SYM_SECTION_MAX_EX: u32 = 2147483647;
pub const IMAGE_SYM_TYPE_BYTE: u32 = 12;
pub const IMAGE_SYM_TYPE_CHAR: u32 = 2;
pub const IMAGE_SYM_TYPE_DOUBLE: u32 = 7;
pub const IMAGE_SYM_TYPE_DWORD: u32 = 15;
pub const IMAGE_SYM_TYPE_ENUM: u32 = 10;
pub const IMAGE_SYM_TYPE_FLOAT: u32 = 6;
pub const IMAGE_SYM_TYPE_INT: u32 = 4;
pub const IMAGE_SYM_TYPE_LONG: u32 = 5;
pub const IMAGE_SYM_TYPE_MOE: u32 = 11;
pub const IMAGE_SYM_TYPE_NULL: u32 = 0;
pub const IMAGE_SYM_TYPE_PCODE: u32 = 32768;
pub const IMAGE_SYM_TYPE_SHORT: u32 = 3;
pub const IMAGE_SYM_TYPE_STRUCT: u32 = 8;
pub const IMAGE_SYM_TYPE_UINT: u32 = 14;
pub const IMAGE_SYM_TYPE_UNION: u32 = 9;
pub const IMAGE_SYM_TYPE_VOID: u32 = 1;
pub const IMAGE_SYM_TYPE_WORD: u32 = 13;
pub const IMAGE_SYM_UNDEFINED: i16 = 0;
#[cfg(target_arch = "x86")]
pub type IMAGE_THUNK_DATA = IMAGE_THUNK_DATA32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type IMAGE_THUNK_DATA = IMAGE_THUNK_DATA64;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMAGE_THUNK_DATA32 {
    pub u1: IMAGE_THUNK_DATA32_0,
}
impl Default for IMAGE_THUNK_DATA32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_THUNK_DATA32_0 {
    pub ForwarderString: u32,
    pub Function: u32,
    pub Ordinal: u32,
    pub AddressOfData: u32,
}
impl Default for IMAGE_THUNK_DATA32_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMAGE_THUNK_DATA64 {
    pub u1: IMAGE_THUNK_DATA64_0,
}
impl Default for IMAGE_THUNK_DATA64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_THUNK_DATA64_0 {
    pub ForwarderString: u64,
    pub Function: u64,
    pub Ordinal: u64,
    pub AddressOfData: u64,
}
impl Default for IMAGE_THUNK_DATA64_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
pub type IMAGE_TLS_DIRECTORY = IMAGE_TLS_DIRECTORY32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type IMAGE_TLS_DIRECTORY = IMAGE_TLS_DIRECTORY64;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMAGE_TLS_DIRECTORY32 {
    pub StartAddressOfRawData: u32,
    pub EndAddressOfRawData: u32,
    pub AddressOfIndex: u32,
    pub AddressOfCallBacks: u32,
    pub SizeOfZeroFill: u32,
    pub Anonymous: IMAGE_TLS_DIRECTORY32_0,
}
impl Default for IMAGE_TLS_DIRECTORY32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_TLS_DIRECTORY32_0 {
    pub Characteristics: u32,
    pub Anonymous: IMAGE_TLS_DIRECTORY32_0_0,
}
impl Default for IMAGE_TLS_DIRECTORY32_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_TLS_DIRECTORY32_0_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct IMAGE_TLS_DIRECTORY64 {
    pub StartAddressOfRawData: u64,
    pub EndAddressOfRawData: u64,
    pub AddressOfIndex: u64,
    pub AddressOfCallBacks: u64,
    pub SizeOfZeroFill: u32,
    pub Anonymous: IMAGE_TLS_DIRECTORY64_0,
}
impl Default for IMAGE_TLS_DIRECTORY64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_TLS_DIRECTORY64_0 {
    pub Characteristics: u32,
    pub Anonymous: IMAGE_TLS_DIRECTORY64_0_0,
}
impl Default for IMAGE_TLS_DIRECTORY64_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IMAGE_TLS_DIRECTORY64_0_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(2))]
#[derive(Clone, Copy)]
pub struct IMAGE_VXD_HEADER {
    pub e32_magic: u16,
    pub e32_border: u8,
    pub e32_worder: u8,
    pub e32_level: u32,
    pub e32_cpu: u16,
    pub e32_os: u16,
    pub e32_ver: u32,
    pub e32_mflags: u32,
    pub e32_mpages: u32,
    pub e32_startobj: u32,
    pub e32_eip: u32,
    pub e32_stackobj: u32,
    pub e32_esp: u32,
    pub e32_pagesize: u32,
    pub e32_lastpagesize: u32,
    pub e32_fixupsize: u32,
    pub e32_fixupsum: u32,
    pub e32_ldrsize: u32,
    pub e32_ldrsum: u32,
    pub e32_objtab: u32,
    pub e32_objcnt: u32,
    pub e32_objmap: u32,
    pub e32_itermap: u32,
    pub e32_rsrctab: u32,
    pub e32_rsrccnt: u32,
    pub e32_restab: u32,
    pub e32_enttab: u32,
    pub e32_dirtab: u32,
    pub e32_dircnt: u32,
    pub e32_fpagetab: u32,
    pub e32_frectab: u32,
    pub e32_impmod: u32,
    pub e32_impmodcnt: u32,
    pub e32_impproc: u32,
    pub e32_pagesum: u32,
    pub e32_datapage: u32,
    pub e32_preload: u32,
    pub e32_nrestab: u32,
    pub e32_cbnrestab: u32,
    pub e32_nressum: u32,
    pub e32_autodata: u32,
    pub e32_debuginfo: u32,
    pub e32_debuglen: u32,
    pub e32_instpreload: u32,
    pub e32_instdemand: u32,
    pub e32_heapsize: u32,
    pub e32_res3: [u8; 12],
    pub e32_winresoff: u32,
    pub e32_winreslen: u32,
    pub e32_devid: u16,
    pub e32_ddkver: u16,
}
impl Default for IMAGE_VXD_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IMAGE_VXD_SIGNATURE: u32 = 17740;
pub const IMAGE_WEAK_EXTERN_ANTI_DEPENDENCY: u32 = 4;
pub const IMAGE_WEAK_EXTERN_SEARCH_ALIAS: u32 = 3;
pub const IMAGE_WEAK_EXTERN_SEARCH_LIBRARY: u32 = 2;
pub const IMAGE_WEAK_EXTERN_SEARCH_NOLIBRARY: u32 = 1;
pub const IMPORT_OBJECT_CODE: IMPORT_OBJECT_TYPE = 0;
pub const IMPORT_OBJECT_CONST: IMPORT_OBJECT_TYPE = 2;
pub const IMPORT_OBJECT_DATA: IMPORT_OBJECT_TYPE = 1;
pub const IMPORT_OBJECT_HDR_SIG2: u32 = 65535;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMPORT_OBJECT_HEADER {
    pub Sig1: u16,
    pub Sig2: u16,
    pub Version: u16,
    pub Machine: u16,
    pub TimeDateStamp: u32,
    pub SizeOfData: u32,
    pub Anonymous: IMPORT_OBJECT_HEADER_0,
    pub _bitfield: u16,
}
impl Default for IMPORT_OBJECT_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMPORT_OBJECT_HEADER_0 {
    pub Ordinal: u16,
    pub Hint: u16,
}
impl Default for IMPORT_OBJECT_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IMPORT_OBJECT_NAME: IMPORT_OBJECT_NAME_TYPE = 1;
pub const IMPORT_OBJECT_NAME_EXPORTAS: IMPORT_OBJECT_NAME_TYPE = 4;
pub const IMPORT_OBJECT_NAME_NO_PREFIX: IMPORT_OBJECT_NAME_TYPE = 2;
pub type IMPORT_OBJECT_NAME_TYPE = i32;
pub const IMPORT_OBJECT_NAME_UNDECORATE: IMPORT_OBJECT_NAME_TYPE = 3;
pub const IMPORT_OBJECT_ORDINAL: IMPORT_OBJECT_NAME_TYPE = 0;
pub type IMPORT_OBJECT_TYPE = i32;
pub const INHERITED_ACE: u32 = 16;
pub const INHERIT_ONLY_ACE: u32 = 8;
#[cfg(target_arch = "aarch64")]
pub const INITIAL_CPSR: u32 = 16;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const INITIAL_FPCSR: u32 = 639;
#[cfg(target_arch = "aarch64")]
pub const INITIAL_FPSCR: u32 = 0;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const INITIAL_MXCSR: u32 = 8064;
pub const IO_COMPLETION_ALL_ACCESS: u32 = 2031619;
pub const IO_COMPLETION_MODIFY_STATE: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_COUNTERS {
    pub ReadOperationCount: u64,
    pub WriteOperationCount: u64,
    pub OtherOperationCount: u64,
    pub ReadTransferCount: u64,
    pub WriteTransferCount: u64,
    pub OtherTransferCount: u64,
}
pub const IO_QOS_MAX_RESERVATION: u64 = 1000000000;
pub const IO_REPARSE_TAG_AF_UNIX: u32 = 2147483683;
pub const IO_REPARSE_TAG_APPEXECLINK: u32 = 2147483675;
pub const IO_REPARSE_TAG_CLOUD: u32 = 2415919130;
pub const IO_REPARSE_TAG_CLOUD_1: u32 = 2415923226;
pub const IO_REPARSE_TAG_CLOUD_2: u32 = 2415927322;
pub const IO_REPARSE_TAG_CLOUD_3: u32 = 2415931418;
pub const IO_REPARSE_TAG_CLOUD_4: u32 = 2415935514;
pub const IO_REPARSE_TAG_CLOUD_5: u32 = 2415939610;
pub const IO_REPARSE_TAG_CLOUD_6: u32 = 2415943706;
pub const IO_REPARSE_TAG_CLOUD_7: u32 = 2415947802;
pub const IO_REPARSE_TAG_CLOUD_8: u32 = 2415951898;
pub const IO_REPARSE_TAG_CLOUD_9: u32 = 2415955994;
pub const IO_REPARSE_TAG_CLOUD_A: u32 = 2415960090;
pub const IO_REPARSE_TAG_CLOUD_B: u32 = 2415964186;
pub const IO_REPARSE_TAG_CLOUD_C: u32 = 2415968282;
pub const IO_REPARSE_TAG_CLOUD_D: u32 = 2415972378;
pub const IO_REPARSE_TAG_CLOUD_E: u32 = 2415976474;
pub const IO_REPARSE_TAG_CLOUD_F: u32 = 2415980570;
pub const IO_REPARSE_TAG_CLOUD_MASK: u32 = 61440;
pub const IO_REPARSE_TAG_CSV: u32 = 2147483657;
pub const IO_REPARSE_TAG_DATALESS_CIM: u32 = 2684354600;
pub const IO_REPARSE_TAG_DEDUP: u32 = 2147483667;
pub const IO_REPARSE_TAG_DFS: u32 = 2147483658;
pub const IO_REPARSE_TAG_DFSR: u32 = 2147483666;
pub const IO_REPARSE_TAG_FILE_PLACEHOLDER: u32 = 2147483669;
pub const IO_REPARSE_TAG_GLOBAL_REPARSE: u32 = 2684354585;
pub const IO_REPARSE_TAG_HSM: u32 = 3221225476;
pub const IO_REPARSE_TAG_HSM2: u32 = 2147483654;
pub const IO_REPARSE_TAG_MOUNT_POINT: u32 = 2684354563;
pub const IO_REPARSE_TAG_NFS: u32 = 2147483668;
pub const IO_REPARSE_TAG_ONEDRIVE: u32 = 2147483681;
pub const IO_REPARSE_TAG_PROJFS: u32 = 2415919132;
pub const IO_REPARSE_TAG_PROJFS_TOMBSTONE: u32 = 2684354594;
pub const IO_REPARSE_TAG_RESERVED_INVALID: u32 = 3221258240;
pub const IO_REPARSE_TAG_RESERVED_ONE: u32 = 1;
pub const IO_REPARSE_TAG_RESERVED_RANGE: u32 = 2;
pub const IO_REPARSE_TAG_RESERVED_TWO: u32 = 2;
pub const IO_REPARSE_TAG_RESERVED_ZERO: u32 = 0;
pub const IO_REPARSE_TAG_SIS: u32 = 2147483655;
pub const IO_REPARSE_TAG_STORAGE_SYNC: u32 = 2147483678;
pub const IO_REPARSE_TAG_STORAGE_SYNC_FOLDER: u32 = 2415919143;
pub const IO_REPARSE_TAG_SYMLINK: u32 = 2684354572;
pub const IO_REPARSE_TAG_UNHANDLED: u32 = 2147483680;
pub const IO_REPARSE_TAG_WCI: u32 = 2147483672;
pub const IO_REPARSE_TAG_WCI_1: u32 = 2415923224;
pub const IO_REPARSE_TAG_WCI_LINK: u32 = 2684354599;
pub const IO_REPARSE_TAG_WCI_LINK_1: u32 = 2684358695;
pub const IO_REPARSE_TAG_WCI_TOMBSTONE: u32 = 2684354591;
pub const IO_REPARSE_TAG_WIM: u32 = 2147483656;
pub const IO_REPARSE_TAG_WOF: u32 = 2147483671;
pub const IS_TEXT_UNICODE_ASCII16: u32 = 1;
pub const IS_TEXT_UNICODE_CONTROLS: u32 = 4;
pub const IS_TEXT_UNICODE_DBCS_LEADBYTE: u32 = 1024;
pub const IS_TEXT_UNICODE_ILLEGAL_CHARS: u32 = 256;
pub const IS_TEXT_UNICODE_NOT_ASCII_MASK: u32 = 61440;
pub const IS_TEXT_UNICODE_NOT_UNICODE_MASK: u32 = 3840;
pub const IS_TEXT_UNICODE_NULL_BYTES: u32 = 4096;
pub const IS_TEXT_UNICODE_ODD_LENGTH: u32 = 512;
pub const IS_TEXT_UNICODE_REVERSE_ASCII16: u32 = 16;
pub const IS_TEXT_UNICODE_REVERSE_CONTROLS: u32 = 64;
pub const IS_TEXT_UNICODE_REVERSE_MASK: u32 = 240;
pub const IS_TEXT_UNICODE_REVERSE_SIGNATURE: u32 = 128;
pub const IS_TEXT_UNICODE_REVERSE_STATISTICS: u32 = 32;
pub const IS_TEXT_UNICODE_SIGNATURE: u32 = 8;
pub const IS_TEXT_UNICODE_STATISTICS: u32 = 2;
pub const IS_TEXT_UNICODE_UNICODE_MASK: u32 = 15;
pub const IS_TEXT_UNICODE_UTF8: u32 = 2048;
pub const IdleResiliency: POWER_INFORMATION_LEVEL = 60;
pub const IgnoreError: SERVICE_ERROR_TYPE = 0;
pub const ImagePolicyEntryTypeAnsiString: IMAGE_POLICY_ENTRY_TYPE = 10;
pub const ImagePolicyEntryTypeBool: IMAGE_POLICY_ENTRY_TYPE = 1;
pub const ImagePolicyEntryTypeInt16: IMAGE_POLICY_ENTRY_TYPE = 4;
pub const ImagePolicyEntryTypeInt32: IMAGE_POLICY_ENTRY_TYPE = 6;
pub const ImagePolicyEntryTypeInt64: IMAGE_POLICY_ENTRY_TYPE = 8;
pub const ImagePolicyEntryTypeInt8: IMAGE_POLICY_ENTRY_TYPE = 2;
pub const ImagePolicyEntryTypeMaximum: IMAGE_POLICY_ENTRY_TYPE = 13;
pub const ImagePolicyEntryTypeNone: IMAGE_POLICY_ENTRY_TYPE = 0;
pub const ImagePolicyEntryTypeOverride: IMAGE_POLICY_ENTRY_TYPE = 12;
pub const ImagePolicyEntryTypeUInt16: IMAGE_POLICY_ENTRY_TYPE = 5;
pub const ImagePolicyEntryTypeUInt32: IMAGE_POLICY_ENTRY_TYPE = 7;
pub const ImagePolicyEntryTypeUInt64: IMAGE_POLICY_ENTRY_TYPE = 9;
pub const ImagePolicyEntryTypeUInt8: IMAGE_POLICY_ENTRY_TYPE = 3;
pub const ImagePolicyEntryTypeUnicodeString: IMAGE_POLICY_ENTRY_TYPE = 11;
pub const ImagePolicyIdCapability: IMAGE_POLICY_ID = 10;
pub const ImagePolicyIdCapabilityOverridable: IMAGE_POLICY_ID = 12;
pub const ImagePolicyIdCrashDump: IMAGE_POLICY_ID = 3;
pub const ImagePolicyIdCrashDumpKey: IMAGE_POLICY_ID = 4;
pub const ImagePolicyIdCrashDumpKeyGuid: IMAGE_POLICY_ID = 5;
pub const ImagePolicyIdDebug: IMAGE_POLICY_ID = 2;
pub const ImagePolicyIdDeviceId: IMAGE_POLICY_ID = 9;
pub const ImagePolicyIdEtw: IMAGE_POLICY_ID = 1;
pub const ImagePolicyIdMaximum: IMAGE_POLICY_ID = 14;
pub const ImagePolicyIdNone: IMAGE_POLICY_ID = 0;
pub const ImagePolicyIdParentSd: IMAGE_POLICY_ID = 6;
pub const ImagePolicyIdParentSdRev: IMAGE_POLICY_ID = 7;
pub const ImagePolicyIdScenarioId: IMAGE_POLICY_ID = 11;
pub const ImagePolicyIdSvn: IMAGE_POLICY_ID = 8;
pub const ImagePolicyIdTrustletIdOverridable: IMAGE_POLICY_ID = 13;
pub type JOBOBJECTINFOCLASS = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct JOBOBJECT_ASSOCIATE_COMPLETION_PORT {
    pub CompletionKey: *mut core::ffi::c_void,
    pub CompletionPort: HANDLE,
}
impl Default for JOBOBJECT_ASSOCIATE_COMPLETION_PORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct JOBOBJECT_BASIC_ACCOUNTING_INFORMATION {
    pub TotalUserTime: i64,
    pub TotalKernelTime: i64,
    pub ThisPeriodTotalUserTime: i64,
    pub ThisPeriodTotalKernelTime: i64,
    pub TotalPageFaultCount: u32,
    pub TotalProcesses: u32,
    pub ActiveProcesses: u32,
    pub TotalTerminatedProcesses: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct JOBOBJECT_BASIC_AND_IO_ACCOUNTING_INFORMATION {
    pub BasicInfo: JOBOBJECT_BASIC_ACCOUNTING_INFORMATION,
    pub IoInfo: IO_COUNTERS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct JOBOBJECT_BASIC_LIMIT_INFORMATION {
    pub PerProcessUserTimeLimit: i64,
    pub PerJobUserTimeLimit: i64,
    pub LimitFlags: u32,
    pub MinimumWorkingSetSize: usize,
    pub MaximumWorkingSetSize: usize,
    pub ActiveProcessLimit: u32,
    pub Affinity: usize,
    pub PriorityClass: u32,
    pub SchedulingClass: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct JOBOBJECT_BASIC_PROCESS_ID_LIST {
    pub NumberOfAssignedProcesses: u32,
    pub NumberOfProcessIdsInList: u32,
    pub ProcessIdList: [usize; 1],
}
impl Default for JOBOBJECT_BASIC_PROCESS_ID_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct JOBOBJECT_BASIC_UI_RESTRICTIONS {
    pub UIRestrictionsClass: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JOBOBJECT_CPU_RATE_CONTROL_INFORMATION {
    pub ControlFlags: u32,
    pub Anonymous: JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0,
}
impl Default for JOBOBJECT_CPU_RATE_CONTROL_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0 {
    pub CpuRate: u32,
    pub Weight: u32,
    pub Anonymous: JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0_0,
}
impl Default for JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct JOBOBJECT_CPU_RATE_CONTROL_INFORMATION_0_0 {
    pub MinRate: u16,
    pub MaxRate: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct JOBOBJECT_END_OF_JOB_TIME_INFORMATION {
    pub EndOfJobTimeAction: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct JOBOBJECT_EXTENDED_LIMIT_INFORMATION {
    pub BasicLimitInformation: JOBOBJECT_BASIC_LIMIT_INFORMATION,
    pub IoInfo: IO_COUNTERS,
    pub ProcessMemoryLimit: usize,
    pub JobMemoryLimit: usize,
    pub PeakProcessMemoryUsed: usize,
    pub PeakJobMemoryUsed: usize,
}
pub const JOBOBJECT_IO_ATTRIBUTION_CONTROL_DISABLE: JOBOBJECT_IO_ATTRIBUTION_CONTROL_FLAGS = 2;
pub const JOBOBJECT_IO_ATTRIBUTION_CONTROL_ENABLE: JOBOBJECT_IO_ATTRIBUTION_CONTROL_FLAGS = 1;
pub type JOBOBJECT_IO_ATTRIBUTION_CONTROL_FLAGS = i32;
pub const JOBOBJECT_IO_ATTRIBUTION_CONTROL_VALID_FLAGS: JOBOBJECT_IO_ATTRIBUTION_CONTROL_FLAGS = 3;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct JOBOBJECT_IO_ATTRIBUTION_INFORMATION {
    pub ControlFlags: u32,
    pub ReadStats: JOBOBJECT_IO_ATTRIBUTION_STATS,
    pub WriteStats: JOBOBJECT_IO_ATTRIBUTION_STATS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct JOBOBJECT_IO_ATTRIBUTION_STATS {
    pub IoCount: usize,
    pub TotalNonOverlappedQueueTime: u64,
    pub TotalNonOverlappedServiceTime: u64,
    pub TotalSize: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE {
    pub MaxIops: i64,
    pub MaxBandwidth: i64,
    pub ReservationIops: i64,
    pub VolumeName: windows_core::PWSTR,
    pub BaseIoSize: u32,
    pub ControlFlags: JOB_OBJECT_IO_RATE_CONTROL_FLAGS,
    pub VolumeNameLength: u16,
}
pub type JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V1 = JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V2 {
    pub MaxIops: i64,
    pub MaxBandwidth: i64,
    pub ReservationIops: i64,
    pub VolumeName: windows_core::PWSTR,
    pub BaseIoSize: u32,
    pub ControlFlags: JOB_OBJECT_IO_RATE_CONTROL_FLAGS,
    pub VolumeNameLength: u16,
    pub CriticalReservationIops: i64,
    pub ReservationBandwidth: i64,
    pub CriticalReservationBandwidth: i64,
    pub MaxTimePercent: i64,
    pub ReservationTimePercent: i64,
    pub CriticalReservationTimePercent: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct JOBOBJECT_IO_RATE_CONTROL_INFORMATION_NATIVE_V3 {
    pub MaxIops: i64,
    pub MaxBandwidth: i64,
    pub ReservationIops: i64,
    pub VolumeName: windows_core::PWSTR,
    pub BaseIoSize: u32,
    pub ControlFlags: JOB_OBJECT_IO_RATE_CONTROL_FLAGS,
    pub VolumeNameLength: u16,
    pub CriticalReservationIops: i64,
    pub ReservationBandwidth: i64,
    pub CriticalReservationBandwidth: i64,
    pub MaxTimePercent: i64,
    pub ReservationTimePercent: i64,
    pub CriticalReservationTimePercent: i64,
    pub SoftMaxIops: i64,
    pub SoftMaxBandwidth: i64,
    pub SoftMaxTimePercent: i64,
    pub LimitExcessNotifyIops: i64,
    pub LimitExcessNotifyBandwidth: i64,
    pub LimitExcessNotifyTimePercent: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct JOBOBJECT_JOBSET_INFORMATION {
    pub MemberLevel: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct JOBOBJECT_LIMIT_VIOLATION_INFORMATION {
    pub LimitFlags: u32,
    pub ViolationLimitFlags: u32,
    pub IoReadBytes: u64,
    pub IoReadBytesLimit: u64,
    pub IoWriteBytes: u64,
    pub IoWriteBytesLimit: u64,
    pub PerJobUserTime: i64,
    pub PerJobUserTimeLimit: i64,
    pub JobMemory: u64,
    pub JobMemoryLimit: u64,
    pub RateControlTolerance: JOBOBJECT_RATE_CONTROL_TOLERANCE,
    pub RateControlToleranceLimit: JOBOBJECT_RATE_CONTROL_TOLERANCE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2 {
    pub LimitFlags: u32,
    pub ViolationLimitFlags: u32,
    pub IoReadBytes: u64,
    pub IoReadBytesLimit: u64,
    pub IoWriteBytes: u64,
    pub IoWriteBytesLimit: u64,
    pub PerJobUserTime: i64,
    pub PerJobUserTimeLimit: i64,
    pub JobMemory: u64,
    pub Anonymous: JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_0,
    pub Anonymous2: JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_1,
    pub Anonymous3: JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_2,
    pub JobLowMemoryLimit: u64,
    pub IoRateControlTolerance: JOBOBJECT_RATE_CONTROL_TOLERANCE,
    pub IoRateControlToleranceLimit: JOBOBJECT_RATE_CONTROL_TOLERANCE,
    pub NetRateControlTolerance: JOBOBJECT_RATE_CONTROL_TOLERANCE,
    pub NetRateControlToleranceLimit: JOBOBJECT_RATE_CONTROL_TOLERANCE,
}
impl Default for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_0 {
    pub JobHighMemoryLimit: u64,
    pub JobMemoryLimit: u64,
}
impl Default for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_1 {
    pub RateControlTolerance: JOBOBJECT_RATE_CONTROL_TOLERANCE,
    pub CpuRateControlTolerance: JOBOBJECT_RATE_CONTROL_TOLERANCE,
}
impl Default for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_2 {
    pub RateControlToleranceLimit: JOBOBJECT_RATE_CONTROL_TOLERANCE,
    pub CpuRateControlToleranceLimit: JOBOBJECT_RATE_CONTROL_TOLERANCE,
}
impl Default for JOBOBJECT_LIMIT_VIOLATION_INFORMATION_2_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct JOBOBJECT_NETWORK_ACCOUNTING_INFORMATION {
    pub DataBytesIn: u64,
    pub DataBytesOut: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct JOBOBJECT_NET_RATE_CONTROL_INFORMATION {
    pub MaxBandwidth: u64,
    pub ControlFlags: JOB_OBJECT_NET_RATE_CONTROL_FLAGS,
    pub DscpTag: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION {
    pub IoReadBytesLimit: u64,
    pub IoWriteBytesLimit: u64,
    pub PerJobUserTimeLimit: i64,
    pub JobMemoryLimit: u64,
    pub RateControlTolerance: JOBOBJECT_RATE_CONTROL_TOLERANCE,
    pub RateControlToleranceInterval: JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL,
    pub LimitFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2 {
    pub IoReadBytesLimit: u64,
    pub IoWriteBytesLimit: u64,
    pub PerJobUserTimeLimit: i64,
    pub Anonymous: JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_0,
    pub Anonymous2: JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_1,
    pub Anonymous3: JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_2,
    pub LimitFlags: u32,
    pub IoRateControlTolerance: JOBOBJECT_RATE_CONTROL_TOLERANCE,
    pub JobLowMemoryLimit: u64,
    pub IoRateControlToleranceInterval: JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL,
    pub NetRateControlTolerance: JOBOBJECT_RATE_CONTROL_TOLERANCE,
    pub NetRateControlToleranceInterval: JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL,
}
impl Default for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_0 {
    pub JobHighMemoryLimit: u64,
    pub JobMemoryLimit: u64,
}
impl Default for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_1 {
    pub RateControlTolerance: JOBOBJECT_RATE_CONTROL_TOLERANCE,
    pub CpuRateControlTolerance: JOBOBJECT_RATE_CONTROL_TOLERANCE,
}
impl Default for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_2 {
    pub RateControlToleranceInterval: JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL,
    pub CpuRateControlToleranceInterval: JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL,
}
impl Default for JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION_2_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type JOBOBJECT_RATE_CONTROL_TOLERANCE = i32;
pub type JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct JOBOBJECT_SECURITY_LIMIT_INFORMATION {
    pub SecurityLimitFlags: u32,
    pub JobToken: HANDLE,
    pub SidsToDisable: PTOKEN_GROUPS,
    pub PrivilegesToDelete: PTOKEN_PRIVILEGES,
    pub RestrictedSids: PTOKEN_GROUPS,
}
pub const JOB_OBJECT_ALL_ACCESS: u32 = 2031679;
pub const JOB_OBJECT_ASSIGN_PROCESS: u32 = 1;
pub const JOB_OBJECT_BASIC_LIMIT_VALID_FLAGS: u32 = 255;
pub const JOB_OBJECT_CPU_RATE_CONTROL_ENABLE: u32 = 1;
pub const JOB_OBJECT_CPU_RATE_CONTROL_HARD_CAP: u32 = 4;
pub const JOB_OBJECT_CPU_RATE_CONTROL_MIN_MAX_RATE: u32 = 16;
pub const JOB_OBJECT_CPU_RATE_CONTROL_NOTIFY: u32 = 8;
pub const JOB_OBJECT_CPU_RATE_CONTROL_PER_PROCESSOR_CAPS: u32 = 32;
pub const JOB_OBJECT_CPU_RATE_CONTROL_VALID_FLAGS: u32 = 63;
pub const JOB_OBJECT_CPU_RATE_CONTROL_WEIGHT_BASED: u32 = 2;
pub const JOB_OBJECT_EXTENDED_LIMIT_VALID_FLAGS: u32 = 32767;
pub const JOB_OBJECT_IMPERSONATE: u32 = 32;
pub const JOB_OBJECT_IO_RATE_CONTROL_ENABLE: JOB_OBJECT_IO_RATE_CONTROL_FLAGS = 1;
pub type JOB_OBJECT_IO_RATE_CONTROL_FLAGS = u32;
pub const JOB_OBJECT_IO_RATE_CONTROL_FORCE_UNIT_ACCESS_ALL: JOB_OBJECT_IO_RATE_CONTROL_FLAGS = 4;
pub const JOB_OBJECT_IO_RATE_CONTROL_FORCE_UNIT_ACCESS_ON_SOFT_CAP: JOB_OBJECT_IO_RATE_CONTROL_FLAGS = 8;
pub const JOB_OBJECT_IO_RATE_CONTROL_STANDALONE_VOLUME: JOB_OBJECT_IO_RATE_CONTROL_FLAGS = 2;
pub const JOB_OBJECT_IO_RATE_CONTROL_VALID_FLAGS: JOB_OBJECT_IO_RATE_CONTROL_FLAGS = 15;
pub const JOB_OBJECT_LIMIT_ACTIVE_PROCESS: u32 = 8;
pub const JOB_OBJECT_LIMIT_AFFINITY: u32 = 16;
pub const JOB_OBJECT_LIMIT_BREAKAWAY_OK: u32 = 2048;
pub const JOB_OBJECT_LIMIT_CPU_RATE_CONTROL: u32 = 262144;
pub const JOB_OBJECT_LIMIT_DIE_ON_UNHANDLED_EXCEPTION: u32 = 1024;
pub const JOB_OBJECT_LIMIT_IO_RATE_CONTROL: u32 = 524288;
pub const JOB_OBJECT_LIMIT_JOB_MEMORY: u32 = 512;
pub const JOB_OBJECT_LIMIT_JOB_MEMORY_HIGH: u32 = 512;
pub const JOB_OBJECT_LIMIT_JOB_MEMORY_LOW: u32 = 32768;
pub const JOB_OBJECT_LIMIT_JOB_READ_BYTES: u32 = 65536;
pub const JOB_OBJECT_LIMIT_JOB_TIME: u32 = 4;
pub const JOB_OBJECT_LIMIT_JOB_WRITE_BYTES: u32 = 131072;
pub const JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE: u32 = 8192;
pub const JOB_OBJECT_LIMIT_NET_RATE_CONTROL: u32 = 1048576;
pub const JOB_OBJECT_LIMIT_PRESERVE_JOB_TIME: u32 = 64;
pub const JOB_OBJECT_LIMIT_PRIORITY_CLASS: u32 = 32;
pub const JOB_OBJECT_LIMIT_PROCESS_MEMORY: u32 = 256;
pub const JOB_OBJECT_LIMIT_PROCESS_TIME: u32 = 2;
pub const JOB_OBJECT_LIMIT_RATE_CONTROL: u32 = 262144;
pub const JOB_OBJECT_LIMIT_SCHEDULING_CLASS: u32 = 128;
pub const JOB_OBJECT_LIMIT_SILENT_BREAKAWAY_OK: u32 = 4096;
pub const JOB_OBJECT_LIMIT_SUBSET_AFFINITY: u32 = 16384;
pub const JOB_OBJECT_LIMIT_VALID_FLAGS: u32 = 524287;
pub const JOB_OBJECT_LIMIT_WORKINGSET: u32 = 1;
pub const JOB_OBJECT_MSG_ABNORMAL_EXIT_PROCESS: u32 = 8;
pub const JOB_OBJECT_MSG_ACTIVE_PROCESS_LIMIT: u32 = 3;
pub const JOB_OBJECT_MSG_ACTIVE_PROCESS_ZERO: u32 = 4;
pub const JOB_OBJECT_MSG_END_OF_JOB_TIME: u32 = 1;
pub const JOB_OBJECT_MSG_END_OF_PROCESS_TIME: u32 = 2;
pub const JOB_OBJECT_MSG_EXIT_PROCESS: u32 = 7;
pub const JOB_OBJECT_MSG_JOB_CYCLE_TIME_LIMIT: u32 = 12;
pub const JOB_OBJECT_MSG_JOB_MEMORY_LIMIT: u32 = 10;
pub const JOB_OBJECT_MSG_MAXIMUM: u32 = 13;
pub const JOB_OBJECT_MSG_MINIMUM: u32 = 1;
pub const JOB_OBJECT_MSG_NEW_PROCESS: u32 = 6;
pub const JOB_OBJECT_MSG_NOTIFICATION_LIMIT: u32 = 11;
pub const JOB_OBJECT_MSG_PROCESS_MEMORY_LIMIT: u32 = 9;
pub const JOB_OBJECT_MSG_SILO_TERMINATED: u32 = 13;
pub const JOB_OBJECT_NET_RATE_CONTROL_DSCP_TAG: JOB_OBJECT_NET_RATE_CONTROL_FLAGS = 4;
pub const JOB_OBJECT_NET_RATE_CONTROL_ENABLE: JOB_OBJECT_NET_RATE_CONTROL_FLAGS = 1;
pub type JOB_OBJECT_NET_RATE_CONTROL_FLAGS = u32;
pub const JOB_OBJECT_NET_RATE_CONTROL_MAX_BANDWIDTH: JOB_OBJECT_NET_RATE_CONTROL_FLAGS = 2;
pub const JOB_OBJECT_NET_RATE_CONTROL_MAX_DSCP_TAG: u32 = 64;
pub const JOB_OBJECT_NET_RATE_CONTROL_VALID_FLAGS: JOB_OBJECT_NET_RATE_CONTROL_FLAGS = 7;
pub const JOB_OBJECT_NOTIFICATION_LIMIT_VALID_FLAGS: u32 = 2064900;
pub const JOB_OBJECT_POST_AT_END_OF_JOB: u32 = 1;
pub const JOB_OBJECT_QUERY: u32 = 4;
pub const JOB_OBJECT_SECURITY_FILTER_TOKENS: u32 = 8;
pub const JOB_OBJECT_SECURITY_NO_ADMIN: u32 = 1;
pub const JOB_OBJECT_SECURITY_ONLY_TOKEN: u32 = 4;
pub const JOB_OBJECT_SECURITY_RESTRICTED_TOKEN: u32 = 2;
pub const JOB_OBJECT_SECURITY_VALID_FLAGS: u32 = 15;
pub const JOB_OBJECT_SET_ATTRIBUTES: u32 = 2;
pub const JOB_OBJECT_SET_SECURITY_ATTRIBUTES: u32 = 16;
pub const JOB_OBJECT_TERMINATE: u32 = 8;
pub const JOB_OBJECT_TERMINATE_AT_END_OF_JOB: u32 = 0;
pub const JOB_OBJECT_UILIMIT_ALL: u32 = 1023;
pub const JOB_OBJECT_UILIMIT_DESKTOP: u32 = 64;
pub const JOB_OBJECT_UILIMIT_DISPLAYSETTINGS: u32 = 16;
pub const JOB_OBJECT_UILIMIT_EXITWINDOWS: u32 = 128;
pub const JOB_OBJECT_UILIMIT_GLOBALATOMS: u32 = 32;
pub const JOB_OBJECT_UILIMIT_HANDLES: u32 = 1;
pub const JOB_OBJECT_UILIMIT_IME: u32 = 256;
pub const JOB_OBJECT_UILIMIT_INJECTION: u32 = 512;
pub const JOB_OBJECT_UILIMIT_NONE: u32 = 0;
pub const JOB_OBJECT_UILIMIT_READCLIPBOARD: u32 = 2;
pub const JOB_OBJECT_UILIMIT_SYSTEMPARAMETERS: u32 = 8;
pub const JOB_OBJECT_UILIMIT_WRITECLIPBOARD: u32 = 4;
pub const JOB_OBJECT_UI_VALID_FLAGS: u32 = 1023;
pub const JOB_OBJECT_VALID_COMPLETION_FILTER: u32 = 16382;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct JOB_SET_ARRAY {
    pub JobHandle: HANDLE,
    pub MemberLevel: u32,
    pub Flags: u32,
}
pub const JobObjectAssociateCompletionPortInformation: JOBOBJECTINFOCLASS = 7;
pub const JobObjectBasicAccountingInformation: JOBOBJECTINFOCLASS = 1;
pub const JobObjectBasicAndIoAccountingInformation: JOBOBJECTINFOCLASS = 8;
pub const JobObjectBasicLimitInformation: JOBOBJECTINFOCLASS = 2;
pub const JobObjectBasicProcessIdList: JOBOBJECTINFOCLASS = 3;
pub const JobObjectBasicUIRestrictions: JOBOBJECTINFOCLASS = 4;
pub const JobObjectCompletionCounter: JOBOBJECTINFOCLASS = 17;
pub const JobObjectCompletionFilter: JOBOBJECTINFOCLASS = 16;
pub const JobObjectCpuPartition: JOBOBJECTINFOCLASS = 52;
pub const JobObjectCpuRateControlInformation: JOBOBJECTINFOCLASS = 15;
pub const JobObjectCreateSilo: JOBOBJECTINFOCLASS = 35;
pub const JobObjectEndOfJobTimeInformation: JOBOBJECTINFOCLASS = 6;
pub const JobObjectExtendedLimitInformation: JOBOBJECTINFOCLASS = 9;
pub const JobObjectGroupInformation: JOBOBJECTINFOCLASS = 11;
pub const JobObjectGroupInformationEx: JOBOBJECTINFOCLASS = 14;
pub const JobObjectJobSetInformation: JOBOBJECTINFOCLASS = 10;
pub const JobObjectLimitViolationInformation: JOBOBJECTINFOCLASS = 13;
pub const JobObjectLimitViolationInformation2: JOBOBJECTINFOCLASS = 34;
pub const JobObjectNetRateControlInformation: JOBOBJECTINFOCLASS = 32;
pub const JobObjectNetworkAccountingInformation: JOBOBJECTINFOCLASS = 51;
pub const JobObjectNotificationLimitInformation: JOBOBJECTINFOCLASS = 12;
pub const JobObjectNotificationLimitInformation2: JOBOBJECTINFOCLASS = 33;
pub const JobObjectReserved10Information: JOBOBJECTINFOCLASS = 27;
pub const JobObjectReserved11Information: JOBOBJECTINFOCLASS = 28;
pub const JobObjectReserved12Information: JOBOBJECTINFOCLASS = 29;
pub const JobObjectReserved13Information: JOBOBJECTINFOCLASS = 30;
pub const JobObjectReserved14Information: JOBOBJECTINFOCLASS = 31;
pub const JobObjectReserved15Information: JOBOBJECTINFOCLASS = 37;
pub const JobObjectReserved16Information: JOBOBJECTINFOCLASS = 38;
pub const JobObjectReserved17Information: JOBOBJECTINFOCLASS = 39;
pub const JobObjectReserved18Information: JOBOBJECTINFOCLASS = 40;
pub const JobObjectReserved19Information: JOBOBJECTINFOCLASS = 41;
pub const JobObjectReserved1Information: JOBOBJECTINFOCLASS = 18;
pub const JobObjectReserved20Information: JOBOBJECTINFOCLASS = 42;
pub const JobObjectReserved21Information: JOBOBJECTINFOCLASS = 43;
pub const JobObjectReserved22Information: JOBOBJECTINFOCLASS = 44;
pub const JobObjectReserved23Information: JOBOBJECTINFOCLASS = 45;
pub const JobObjectReserved24Information: JOBOBJECTINFOCLASS = 46;
pub const JobObjectReserved25Information: JOBOBJECTINFOCLASS = 47;
pub const JobObjectReserved26Information: JOBOBJECTINFOCLASS = 48;
pub const JobObjectReserved27Information: JOBOBJECTINFOCLASS = 49;
pub const JobObjectReserved28Information: JOBOBJECTINFOCLASS = 50;
pub const JobObjectReserved2Information: JOBOBJECTINFOCLASS = 19;
pub const JobObjectReserved3Information: JOBOBJECTINFOCLASS = 20;
pub const JobObjectReserved4Information: JOBOBJECTINFOCLASS = 21;
pub const JobObjectReserved5Information: JOBOBJECTINFOCLASS = 22;
pub const JobObjectReserved6Information: JOBOBJECTINFOCLASS = 23;
pub const JobObjectReserved7Information: JOBOBJECTINFOCLASS = 24;
pub const JobObjectReserved8Information: JOBOBJECTINFOCLASS = 25;
pub const JobObjectReserved9Information: JOBOBJECTINFOCLASS = 26;
pub const JobObjectSecurityLimitInformation: JOBOBJECTINFOCLASS = 5;
pub const JobObjectSiloBasicInformation: JOBOBJECTINFOCLASS = 36;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KERNEL_CET_CONTEXT {
    pub Ssp: u64,
    pub Rip: u64,
    pub SegCs: u16,
    pub Anonymous: KERNEL_CET_CONTEXT_0,
    pub Fill: [u16; 2],
}
impl Default for KERNEL_CET_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union KERNEL_CET_CONTEXT_0 {
    pub AllFlags: u16,
    pub Anonymous: KERNEL_CET_CONTEXT_0_0,
}
impl Default for KERNEL_CET_CONTEXT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERNEL_CET_CONTEXT_0_0 {
    pub _bitfield: u16,
}
pub const KEY_ALL_ACCESS: u32 = 983103;
pub const KEY_CREATE_LINK: u32 = 32;
pub const KEY_CREATE_SUB_KEY: u32 = 4;
pub const KEY_ENUMERATE_SUB_KEYS: u32 = 8;
pub const KEY_EXECUTE: u32 = 131097;
pub const KEY_NOTIFY: u32 = 16;
pub const KEY_QUERY_VALUE: u32 = 1;
pub const KEY_READ: u32 = 131097;
pub const KEY_SET_VALUE: u32 = 2;
pub const KEY_WOW64_32KEY: u32 = 512;
pub const KEY_WOW64_64KEY: u32 = 256;
pub const KEY_WOW64_RES: u32 = 768;
pub const KEY_WRITE: u32 = 131078;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KNONVOLATILE_CONTEXT_POINTERS {
    pub Dummy: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "basetsd")]
#[derive(Clone, Copy)]
pub struct KNONVOLATILE_CONTEXT_POINTERS {
    pub Anonymous: KNONVOLATILE_CONTEXT_POINTERS_0,
    pub Anonymous2: KNONVOLATILE_CONTEXT_POINTERS_1,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "basetsd")]
impl Default for KNONVOLATILE_CONTEXT_POINTERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "basetsd")]
#[derive(Clone, Copy)]
pub union KNONVOLATILE_CONTEXT_POINTERS_0 {
    pub FloatingContext: [PM128A; 16],
    pub Anonymous: KNONVOLATILE_CONTEXT_POINTERS_0_0,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "basetsd")]
impl Default for KNONVOLATILE_CONTEXT_POINTERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "basetsd")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KNONVOLATILE_CONTEXT_POINTERS_0_0 {
    pub Xmm0: PM128A,
    pub Xmm1: PM128A,
    pub Xmm2: PM128A,
    pub Xmm3: PM128A,
    pub Xmm4: PM128A,
    pub Xmm5: PM128A,
    pub Xmm6: PM128A,
    pub Xmm7: PM128A,
    pub Xmm8: PM128A,
    pub Xmm9: PM128A,
    pub Xmm10: PM128A,
    pub Xmm11: PM128A,
    pub Xmm12: PM128A,
    pub Xmm13: PM128A,
    pub Xmm14: PM128A,
    pub Xmm15: PM128A,
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "basetsd")]
#[derive(Clone, Copy)]
pub union KNONVOLATILE_CONTEXT_POINTERS_1 {
    pub IntegerContext: [super::basetsd::PDWORD64; 16],
    pub Anonymous: KNONVOLATILE_CONTEXT_POINTERS_1_0,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "basetsd")]
impl Default for KNONVOLATILE_CONTEXT_POINTERS_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "basetsd")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KNONVOLATILE_CONTEXT_POINTERS_1_0 {
    pub Rax: super::basetsd::PDWORD64,
    pub Rcx: super::basetsd::PDWORD64,
    pub Rdx: super::basetsd::PDWORD64,
    pub Rbx: super::basetsd::PDWORD64,
    pub Rsp: super::basetsd::PDWORD64,
    pub Rbp: super::basetsd::PDWORD64,
    pub Rsi: super::basetsd::PDWORD64,
    pub Rdi: super::basetsd::PDWORD64,
    pub R8: super::basetsd::PDWORD64,
    pub R9: super::basetsd::PDWORD64,
    pub R10: super::basetsd::PDWORD64,
    pub R11: super::basetsd::PDWORD64,
    pub R12: super::basetsd::PDWORD64,
    pub R13: super::basetsd::PDWORD64,
    pub R14: super::basetsd::PDWORD64,
    pub R15: super::basetsd::PDWORD64,
}
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "basetsd")]
pub type KNONVOLATILE_CONTEXT_POINTERS = KNONVOLATILE_CONTEXT_POINTERS_ARM64;
#[repr(C)]
#[cfg(feature = "basetsd")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KNONVOLATILE_CONTEXT_POINTERS_ARM64 {
    pub X19: super::basetsd::PDWORD64,
    pub X20: super::basetsd::PDWORD64,
    pub X21: super::basetsd::PDWORD64,
    pub X22: super::basetsd::PDWORD64,
    pub X23: super::basetsd::PDWORD64,
    pub X24: super::basetsd::PDWORD64,
    pub X25: super::basetsd::PDWORD64,
    pub X26: super::basetsd::PDWORD64,
    pub X27: super::basetsd::PDWORD64,
    pub X28: super::basetsd::PDWORD64,
    pub Fp: super::basetsd::PDWORD64,
    pub Lr: super::basetsd::PDWORD64,
    pub D8: super::basetsd::PDWORD64,
    pub D9: super::basetsd::PDWORD64,
    pub D10: super::basetsd::PDWORD64,
    pub D11: super::basetsd::PDWORD64,
    pub D12: super::basetsd::PDWORD64,
    pub D13: super::basetsd::PDWORD64,
    pub D14: super::basetsd::PDWORD64,
    pub D15: super::basetsd::PDWORD64,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct KSPIN_LOCK(pub usize);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KTMOBJECT_CURSOR {
    pub LastQuery: windows_core::GUID,
    pub ObjectIdCount: u32,
    pub ObjectIds: [windows_core::GUID; 1],
}
impl Default for KTMOBJECT_CURSOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const KTMOBJECT_ENLISTMENT: KTMOBJECT_TYPE = 3;
pub const KTMOBJECT_INVALID: KTMOBJECT_TYPE = 4;
pub const KTMOBJECT_RESOURCE_MANAGER: KTMOBJECT_TYPE = 2;
pub const KTMOBJECT_TRANSACTION: KTMOBJECT_TYPE = 0;
pub const KTMOBJECT_TRANSACTION_MANAGER: KTMOBJECT_TYPE = 1;
pub type KTMOBJECT_TYPE = i32;
pub const LABEL_SECURITY_INFORMATION: u32 = 16;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LANGID(pub u16);
pub const LANG_AFRIKAANS: u32 = 54;
pub const LANG_ALBANIAN: u32 = 28;
pub const LANG_ALSATIAN: u32 = 132;
pub const LANG_AMHARIC: u32 = 94;
pub const LANG_ARABIC: u32 = 1;
pub const LANG_ARMENIAN: u32 = 43;
pub const LANG_ASSAMESE: u32 = 77;
pub const LANG_AZERBAIJANI: u32 = 44;
pub const LANG_AZERI: u32 = 44;
pub const LANG_BANGLA: u32 = 69;
pub const LANG_BASHKIR: u32 = 109;
pub const LANG_BASQUE: u32 = 45;
pub const LANG_BELARUSIAN: u32 = 35;
pub const LANG_BENGALI: u32 = 69;
pub const LANG_BOSNIAN: u32 = 26;
pub const LANG_BOSNIAN_NEUTRAL: u32 = 30746;
pub const LANG_BRETON: u32 = 126;
pub const LANG_BULGARIAN: u32 = 2;
pub const LANG_CATALAN: u32 = 3;
pub const LANG_CENTRAL_KURDISH: u32 = 146;
pub const LANG_CHEROKEE: u32 = 92;
pub const LANG_CHINESE: u32 = 4;
pub const LANG_CHINESE_SIMPLIFIED: u32 = 4;
pub const LANG_CHINESE_TRADITIONAL: u32 = 31748;
pub const LANG_CORSICAN: u32 = 131;
pub const LANG_CROATIAN: u32 = 26;
pub const LANG_CZECH: u32 = 5;
pub const LANG_DANISH: u32 = 6;
pub const LANG_DARI: u32 = 140;
pub const LANG_DIVEHI: u32 = 101;
pub const LANG_DUTCH: u32 = 19;
pub const LANG_ENGLISH: u32 = 9;
pub const LANG_ESTONIAN: u32 = 37;
pub const LANG_FAEROESE: u32 = 56;
pub const LANG_FARSI: u32 = 41;
pub const LANG_FILIPINO: u32 = 100;
pub const LANG_FINNISH: u32 = 11;
pub const LANG_FRENCH: u32 = 12;
pub const LANG_FRISIAN: u32 = 98;
pub const LANG_FULAH: u32 = 103;
pub const LANG_GALICIAN: u32 = 86;
pub const LANG_GEORGIAN: u32 = 55;
pub const LANG_GERMAN: u32 = 7;
pub const LANG_GREEK: u32 = 8;
pub const LANG_GREENLANDIC: u32 = 111;
pub const LANG_GUJARATI: u32 = 71;
pub const LANG_HAUSA: u32 = 104;
pub const LANG_HAWAIIAN: u32 = 117;
pub const LANG_HEBREW: u32 = 13;
pub const LANG_HINDI: u32 = 57;
pub const LANG_HUNGARIAN: u32 = 14;
pub const LANG_ICELANDIC: u32 = 15;
pub const LANG_IGBO: u32 = 112;
pub const LANG_INDONESIAN: u32 = 33;
pub const LANG_INUKTITUT: u32 = 93;
pub const LANG_INVARIANT: u32 = 127;
pub const LANG_IRISH: u32 = 60;
pub const LANG_ITALIAN: u32 = 16;
pub const LANG_JAPANESE: u32 = 17;
pub const LANG_KANNADA: u32 = 75;
pub const LANG_KASHMIRI: u32 = 96;
pub const LANG_KAZAK: u32 = 63;
pub const LANG_KHMER: u32 = 83;
pub const LANG_KICHE: u32 = 134;
pub const LANG_KINYARWANDA: u32 = 135;
pub const LANG_KONKANI: u32 = 87;
pub const LANG_KOREAN: u32 = 18;
pub const LANG_KYRGYZ: u32 = 64;
pub const LANG_LAO: u32 = 84;
pub const LANG_LATVIAN: u32 = 38;
pub const LANG_LITHUANIAN: u32 = 39;
pub const LANG_LOWER_SORBIAN: u32 = 46;
pub const LANG_LUXEMBOURGISH: u32 = 110;
pub const LANG_MACEDONIAN: u32 = 47;
pub const LANG_MALAY: u32 = 62;
pub const LANG_MALAYALAM: u32 = 76;
pub const LANG_MALTESE: u32 = 58;
pub const LANG_MANIPURI: u32 = 88;
pub const LANG_MAORI: u32 = 129;
pub const LANG_MAPUDUNGUN: u32 = 122;
pub const LANG_MARATHI: u32 = 78;
pub const LANG_MOHAWK: u32 = 124;
pub const LANG_MONGOLIAN: u32 = 80;
pub const LANG_NEPALI: u32 = 97;
pub const LANG_NEUTRAL: u32 = 0;
pub const LANG_NORWEGIAN: u32 = 20;
pub const LANG_OCCITAN: u32 = 130;
pub const LANG_ODIA: u32 = 72;
pub const LANG_ORIYA: u32 = 72;
pub const LANG_PASHTO: u32 = 99;
pub const LANG_PERSIAN: u32 = 41;
pub const LANG_POLISH: u32 = 21;
pub const LANG_PORTUGUESE: u32 = 22;
pub const LANG_PULAR: u32 = 103;
pub const LANG_PUNJABI: u32 = 70;
pub const LANG_QUECHUA: u32 = 107;
pub const LANG_ROMANIAN: u32 = 24;
pub const LANG_ROMANSH: u32 = 23;
pub const LANG_RUSSIAN: u32 = 25;
pub const LANG_SAKHA: u32 = 133;
pub const LANG_SAMI: u32 = 59;
pub const LANG_SANSKRIT: u32 = 79;
pub const LANG_SCOTTISH_GAELIC: u32 = 145;
pub const LANG_SERBIAN: u32 = 26;
pub const LANG_SERBIAN_NEUTRAL: u32 = 31770;
pub const LANG_SINDHI: u32 = 89;
pub const LANG_SINHALESE: u32 = 91;
pub const LANG_SLOVAK: u32 = 27;
pub const LANG_SLOVENIAN: u32 = 36;
pub const LANG_SOTHO: u32 = 108;
pub const LANG_SPANISH: u32 = 10;
pub const LANG_SWAHILI: u32 = 65;
pub const LANG_SWEDISH: u32 = 29;
pub const LANG_SYRIAC: u32 = 90;
pub const LANG_SYSTEM_DEFAULT: u32 = 2048;
pub const LANG_TAJIK: u32 = 40;
pub const LANG_TAMAZIGHT: u32 = 95;
pub const LANG_TAMIL: u32 = 73;
pub const LANG_TATAR: u32 = 68;
pub const LANG_TELUGU: u32 = 74;
pub const LANG_THAI: u32 = 30;
pub const LANG_TIBETAN: u32 = 81;
pub const LANG_TIGRIGNA: u32 = 115;
pub const LANG_TIGRINYA: u32 = 115;
pub const LANG_TSWANA: u32 = 50;
pub const LANG_TURKISH: u32 = 31;
pub const LANG_TURKMEN: u32 = 66;
pub const LANG_UIGHUR: u32 = 128;
pub const LANG_UKRAINIAN: u32 = 34;
pub const LANG_UPPER_SORBIAN: u32 = 46;
pub const LANG_URDU: u32 = 32;
pub const LANG_USER_DEFAULT: u32 = 1024;
pub const LANG_UZBEK: u32 = 67;
pub const LANG_VALENCIAN: u32 = 3;
pub const LANG_VIETNAMESE: u32 = 42;
pub const LANG_WELSH: u32 = 82;
pub const LANG_WOLOF: u32 = 136;
pub const LANG_XHOSA: u32 = 52;
pub const LANG_YAKUT: u32 = 133;
pub const LANG_YI: u32 = 120;
pub const LANG_YORUBA: u32 = 106;
pub const LANG_ZULU: u32 = 53;
pub type LATENCY_TIME = i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LCID(pub u32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LDT_ENTRY {
    pub LimitLow: u16,
    pub BaseLow: u16,
    pub HighWord: LDT_ENTRY_0,
}
impl Default for LDT_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union LDT_ENTRY_0 {
    pub Bytes: LDT_ENTRY_0_0,
    pub Bits: LDT_ENTRY_0_1,
}
impl Default for LDT_ENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LDT_ENTRY_0_0 {
    pub BaseMid: u8,
    pub Flags1: u8,
    pub Flags2: u8,
    pub BaseHi: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LDT_ENTRY_0_1 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LIST_ENTRY {
    pub Flink: *mut Self,
    pub Blink: *mut Self,
}
impl Default for LIST_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LIST_ENTRY32 {
    pub Flink: u32,
    pub Blink: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LIST_ENTRY64 {
    pub Flink: u64,
    pub Blink: u64,
}
pub const LOCALE_CUSTOM_DEFAULT: u32 = 3072;
pub const LOCALE_CUSTOM_UI_DEFAULT: u32 = 5120;
pub const LOCALE_CUSTOM_UNSPECIFIED: u32 = 4096;
pub const LOCALE_INVARIANT: u32 = 127;
pub const LOCALE_NAME_MAX_LENGTH: u32 = 85;
pub const LOCALE_NEUTRAL: u32 = 0;
pub const LOCALE_SYSTEM_DEFAULT: u32 = 2048;
pub const LOCALE_TRANSIENT_KEYBOARD1: u32 = 8192;
pub const LOCALE_TRANSIENT_KEYBOARD2: u32 = 9216;
pub const LOCALE_TRANSIENT_KEYBOARD3: u32 = 10240;
pub const LOCALE_TRANSIENT_KEYBOARD4: u32 = 11264;
pub const LOCALE_UNASSIGNED_LCID: u32 = 4096;
pub const LOCALE_USER_DEFAULT: u32 = 1024;
pub type LOGICAL_PROCESSOR_RELATIONSHIP = i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCCH(pub *const i8);
impl LPCCH {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPCCH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCH(pub *mut i8);
impl LPCH {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPCH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPCTCH(pub LPCCH);
pub type LPCTSTR = windows_core::PCSTR;
pub type LPCUTSTR = windows_core::PCSTR;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCUWCHAR(pub *const u16);
impl LPCUWCHAR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPCUWCHAR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCUWSTR(pub *const u16);
impl LPCUWSTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPCUWSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCWCH(pub *const u16);
impl LPCWCH {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPCWCH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPCWCHAR(pub *const u16);
impl LPCWCHAR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPCWCHAR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPENCLAVE_TARGET_FUNCTION(pub PENCLAVE_TARGET_FUNCTION);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPOSVERSIONINFO(pub LPOSVERSIONINFOA);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPOSVERSIONINFOA(pub *mut OSVERSIONINFOA);
impl LPOSVERSIONINFOA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPOSVERSIONINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPOSVERSIONINFOEX(pub LPOSVERSIONINFOEXA);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPOSVERSIONINFOEXA(pub *mut OSVERSIONINFOEXA);
impl LPOSVERSIONINFOEXA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPOSVERSIONINFOEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPOSVERSIONINFOEXW(pub *mut OSVERSIONINFOEXW);
impl LPOSVERSIONINFOEXW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPOSVERSIONINFOEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPOSVERSIONINFOW(pub *mut OSVERSIONINFOW);
impl LPOSVERSIONINFOW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPOSVERSIONINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPSECURITY_CAPABILITIES(pub *mut SECURITY_CAPABILITIES);
impl LPSECURITY_CAPABILITIES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPSECURITY_CAPABILITIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LPTCH(pub LPCH);
pub type LPTSTR = windows_core::PSTR;
pub type LPUTSTR = windows_core::PSTR;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPUWSTR(pub *mut u16);
impl LPUWSTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPUWSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LPWCH(pub *mut u16);
impl LPWCH {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for LPWCH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const LTP_PC_SMT: u32 = 1;
pub const LT_DONT_CARE: LATENCY_TIME = 0;
pub const LT_LOWEST_LATENCY: LATENCY_TIME = 1;
pub const LUA_TOKEN: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LUID {
    pub LowPart: u32,
    pub HighPart: i32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LUID_AND_ATTRIBUTES {
    pub Luid: LUID,
    pub Attributes: u32,
}
pub type LUID_AND_ATTRIBUTES_ARRAY = [LUID_AND_ATTRIBUTES; 1];
pub const LX_FILE_CASE_SENSITIVE_DIR: u32 = 16;
pub const LX_FILE_METADATA_HAS_DEVICE_ID: u32 = 8;
pub const LX_FILE_METADATA_HAS_GID: u32 = 2;
pub const LX_FILE_METADATA_HAS_MODE: u32 = 4;
pub const LX_FILE_METADATA_HAS_UID: u32 = 1;
pub const LastResumePerformance: POWER_INFORMATION_LEVEL = 76;
pub const LastSleepTime: POWER_INFORMATION_LEVEL = 15;
pub const LastWakeTime: POWER_INFORMATION_LEVEL = 14;
pub const LogicalProcessorIdling: POWER_INFORMATION_LEVEL = 56;
#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct M128A {
    pub Low: u64,
    pub High: i64,
}
pub const MAILSLOT_NO_MESSAGE: u32 = 4294967295;
pub const MAILSLOT_WAIT_FOREVER: u32 = 4294967295;
pub type MANDATORY_LEVEL = i32;
pub const MAXBYTE: u32 = 255;
pub const MAXCHAR: u32 = 127;
pub const MAXDWORD: u32 = 4294967295;
pub const MAXIMUM_ALLOWED: u32 = 33554432;
#[cfg(target_arch = "x86")]
pub const MAXIMUM_PROCESSORS: u32 = 32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const MAXIMUM_PROCESSORS: u32 = 64;
#[cfg(target_arch = "x86")]
pub const MAXIMUM_PROC_PER_GROUP: u32 = 32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const MAXIMUM_PROC_PER_GROUP: u32 = 64;
pub const MAXIMUM_REPARSE_DATA_BUFFER_SIZE: u32 = 16384;
#[cfg(target_arch = "x86")]
pub const MAXIMUM_SUPPORTED_EXTENSION: u32 = 512;
pub const MAXIMUM_SUSPEND_COUNT: u32 = 127;
pub const MAXIMUM_WAIT_OBJECTS: u32 = 64;
pub const MAXIMUM_XSTATE_FEATURES: u32 = 64;
pub const MAXLOGICALLOGNAMESIZE: u32 = 256;
pub const MAXLONG: u32 = 2147483647;
pub const MAXLONGLONG: u64 = 9223372036854775807;
pub const MAXSHORT: u32 = 32767;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MAXVERSIONTESTED_INFO {
    pub MaxVersionTested: u64,
}
pub const MAXWORD: u32 = 65535;
pub const MAX_ACL_REVISION: u32 = 4;
pub const MAX_CLASS_NAME: ReplacesCorHdrNumericDefines = 1024;
pub const MAX_HW_COUNTERS: u32 = 16;
pub const MAX_PACKAGE_NAME: ReplacesCorHdrNumericDefines = 1024;
pub const MAX_UCSCHAR: u32 = 1114111;
#[cfg(target_arch = "x86")]
pub const MEMORY_ALLOCATION_ALIGNMENT: u32 = 8;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const MEMORY_ALLOCATION_ALIGNMENT: u32 = 16;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MEMORY_BASIC_INFORMATION {
    pub BaseAddress: *mut core::ffi::c_void,
    pub AllocationBase: *mut core::ffi::c_void,
    pub AllocationProtect: u32,
    pub RegionSize: usize,
    pub State: u32,
    pub Protect: u32,
    pub Type: u32,
}
#[cfg(target_arch = "x86")]
impl Default for MEMORY_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MEMORY_BASIC_INFORMATION {
    pub BaseAddress: *mut core::ffi::c_void,
    pub AllocationBase: *mut core::ffi::c_void,
    pub AllocationProtect: u32,
    pub PartitionId: u16,
    pub RegionSize: usize,
    pub State: u32,
    pub Protect: u32,
    pub Type: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for MEMORY_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MEMORY_BASIC_INFORMATION32 {
    pub BaseAddress: u32,
    pub AllocationBase: u32,
    pub AllocationProtect: u32,
    pub RegionSize: u32,
    pub State: u32,
    pub Protect: u32,
    pub Type: u32,
}
#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MEMORY_BASIC_INFORMATION64 {
    pub BaseAddress: u64,
    pub AllocationBase: u64,
    pub AllocationProtect: u32,
    pub __alignment1: u32,
    pub RegionSize: u64,
    pub State: u32,
    pub Protect: u32,
    pub Type: u32,
    pub __alignment2: u32,
}
pub const MEMORY_CURRENT_PARTITION_HANDLE: HANDLE = HANDLE(-1 as _);
pub const MEMORY_EXISTING_VAD_PARTITION_HANDLE: HANDLE = HANDLE(-3 as _);
pub const MEMORY_PARTITION_ALL_ACCESS: u32 = 2031619;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MEMORY_PARTITION_DEDICATED_MEMORY_ATTRIBUTE {
    pub Type: MEM_DEDICATED_ATTRIBUTE_TYPE,
    pub Reserved: u32,
    pub Value: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MEMORY_PARTITION_DEDICATED_MEMORY_INFORMATION {
    pub NextEntryOffset: u32,
    pub SizeOfInformation: u32,
    pub Flags: u32,
    pub AttributesOffset: u32,
    pub AttributeCount: u32,
    pub Reserved: u32,
    pub TypeId: u64,
}
pub const MEMORY_PARTITION_MODIFY_ACCESS: u32 = 2;
pub const MEMORY_PARTITION_QUERY_ACCESS: u32 = 1;
pub const MEMORY_PRIORITY_BELOW_NORMAL: u32 = 4;
pub const MEMORY_PRIORITY_LOW: u32 = 2;
pub const MEMORY_PRIORITY_LOWEST: u32 = 0;
pub const MEMORY_PRIORITY_MEDIUM: u32 = 3;
pub const MEMORY_PRIORITY_NORMAL: u32 = 5;
pub const MEMORY_PRIORITY_VERY_LOW: u32 = 1;
pub const MEMORY_SYSTEM_PARTITION_HANDLE: HANDLE = HANDLE(-2 as _);
pub const MEM_4MB_PAGES: u32 = 2147483648;
pub const MEM_64K_PAGES: u32 = 541065216;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MEM_ADDRESS_REQUIREMENTS {
    pub LowestStartingAddress: *mut core::ffi::c_void,
    pub HighestEndingAddress: *mut core::ffi::c_void,
    pub Alignment: usize,
}
impl Default for MEM_ADDRESS_REQUIREMENTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MEM_COALESCE_PLACEHOLDERS: u32 = 1;
pub const MEM_COMMIT: u32 = 4096;
pub const MEM_DECOMMIT: u32 = 16384;
pub const MEM_DEDICATED_ATTRIBUTE_NOT_SPECIFIED: u64 = 18446744073709551615;
pub type MEM_DEDICATED_ATTRIBUTE_TYPE = i32;
pub const MEM_DIFFERENT_IMAGE_BASE_OK: u32 = 8388608;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MEM_EXTENDED_PARAMETER {
    pub Anonymous: MEM_EXTENDED_PARAMETER_0,
    pub Anonymous2: MEM_EXTENDED_PARAMETER_1,
}
impl Default for MEM_EXTENDED_PARAMETER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MEM_EXTENDED_PARAMETER_0 {
    pub _bitfield: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MEM_EXTENDED_PARAMETER_1 {
    pub ULong64: u64,
    pub Pointer: *mut core::ffi::c_void,
    pub Size: usize,
    pub Handle: HANDLE,
    pub ULong: u32,
}
impl Default for MEM_EXTENDED_PARAMETER_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MEM_EXTENDED_PARAMETER_EC_CODE: u32 = 64;
pub const MEM_EXTENDED_PARAMETER_GRAPHICS: u32 = 1;
pub const MEM_EXTENDED_PARAMETER_NONPAGED: u32 = 2;
pub const MEM_EXTENDED_PARAMETER_NONPAGED_HUGE: u32 = 16;
pub const MEM_EXTENDED_PARAMETER_NONPAGED_LARGE: u32 = 8;
pub const MEM_EXTENDED_PARAMETER_NUMA_NODE_MANDATORY: u32 = 0;
pub const MEM_EXTENDED_PARAMETER_SECURE_PAGES: u32 = 128;
pub const MEM_EXTENDED_PARAMETER_SOFT_FAULT_PAGES: u32 = 32;
pub const MEM_EXTENDED_PARAMETER_TAGGED: u32 = 256;
pub type MEM_EXTENDED_PARAMETER_TYPE = i32;
pub const MEM_EXTENDED_PARAMETER_TYPE_BITS: u32 = 8;
pub const MEM_EXTENDED_PARAMETER_ZERO_PAGES_OPTIONAL: u32 = 4;
pub const MEM_FREE: u32 = 65536;
pub const MEM_IMAGE: u32 = 16777216;
pub const MEM_LARGE_PAGES: u32 = 536870912;
pub const MEM_MAPPED: u32 = 262144;
pub const MEM_PHYSICAL: u32 = 4194304;
pub const MEM_PRESERVE_PLACEHOLDER: u32 = 2;
pub const MEM_PRIVATE: u32 = 131072;
pub const MEM_RELEASE: u32 = 32768;
pub const MEM_REPLACE_PLACEHOLDER: u32 = 16384;
pub const MEM_RESERVE: u32 = 8192;
pub const MEM_RESERVE_PLACEHOLDER: u32 = 262144;
pub const MEM_RESET: u32 = 524288;
pub const MEM_RESET_UNDO: u32 = 16777216;
pub const MEM_ROTATE: u32 = 8388608;
pub const MEM_SECTION_ATTRIBUTE_SECURE_PAGES: u32 = 1;
pub type MEM_SECTION_EXTENDED_PARAMETER_TYPE = i32;
pub const MEM_TOP_DOWN: u32 = 1048576;
pub const MEM_UNMAP_WITH_TRANSIENT_BOOST: u32 = 1;
pub const MEM_WRITE_WATCH: u32 = 2097152;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MESSAGE_RESOURCE_BLOCK {
    pub LowId: u32,
    pub HighId: u32,
    pub OffsetToEntries: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MESSAGE_RESOURCE_DATA {
    pub NumberOfBlocks: u32,
    pub Blocks: [MESSAGE_RESOURCE_BLOCK; 1],
}
impl Default for MESSAGE_RESOURCE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MESSAGE_RESOURCE_ENTRY {
    pub Length: u16,
    pub Flags: u16,
    pub Text: [u8; 1],
}
impl Default for MESSAGE_RESOURCE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MESSAGE_RESOURCE_UNICODE: u32 = 1;
pub const MESSAGE_RESOURCE_UTF8: u32 = 2;
pub const MINCHAR: u32 = 128;
pub const MINLONG: u32 = 2147483648;
pub const MINSHORT: u32 = 32768;
pub const MIN_ACL_REVISION: u32 = 2;
pub const MIN_UCSCHAR: u32 = 0;
pub type MONITOR_DISPLAY_STATE = i32;
pub const MS_PPM_SOFTWARE_ALL: u32 = 1;
pub const MUTANT_ALL_ACCESS: u32 = 2031617;
pub const MUTANT_QUERY_STATE: u32 = 1;
pub const MandatoryLevelCount: MANDATORY_LEVEL = 6;
pub const MandatoryLevelHigh: MANDATORY_LEVEL = 3;
pub const MandatoryLevelLow: MANDATORY_LEVEL = 1;
pub const MandatoryLevelMedium: MANDATORY_LEVEL = 2;
pub const MandatoryLevelSecureProcess: MANDATORY_LEVEL = 5;
pub const MandatoryLevelSystem: MANDATORY_LEVEL = 4;
pub const MandatoryLevelUntrusted: MANDATORY_LEVEL = 0;
pub const MaxActivationContextInfoClass: ACTIVATION_CONTEXT_INFO_CLASS = 8;
pub const MaxHardwareCounterType: HARDWARE_COUNTER_TYPE = 1;
pub const MaxJobObjectInfoClass: JOBOBJECTINFOCLASS = 53;
pub const MaxProcessMitigationPolicy: PROCESS_MITIGATION_POLICY = 20;
pub const MaxTokenInfoClass: TOKEN_INFORMATION_CLASS = 53;
pub const MemDedicatedAttributeMax: MEM_DEDICATED_ATTRIBUTE_TYPE = 4;
pub const MemDedicatedAttributeReadBandwidth: MEM_DEDICATED_ATTRIBUTE_TYPE = 0;
pub const MemDedicatedAttributeReadLatency: MEM_DEDICATED_ATTRIBUTE_TYPE = 1;
pub const MemDedicatedAttributeWriteBandwidth: MEM_DEDICATED_ATTRIBUTE_TYPE = 2;
pub const MemDedicatedAttributeWriteLatency: MEM_DEDICATED_ATTRIBUTE_TYPE = 3;
pub const MemExtendedParameterAddressRequirements: MEM_EXTENDED_PARAMETER_TYPE = 1;
pub const MemExtendedParameterAttributeFlags: MEM_EXTENDED_PARAMETER_TYPE = 5;
pub const MemExtendedParameterImageMachine: MEM_EXTENDED_PARAMETER_TYPE = 6;
pub const MemExtendedParameterInvalidType: MEM_EXTENDED_PARAMETER_TYPE = 0;
pub const MemExtendedParameterMax: MEM_EXTENDED_PARAMETER_TYPE = 7;
pub const MemExtendedParameterNumaNode: MEM_EXTENDED_PARAMETER_TYPE = 2;
pub const MemExtendedParameterPartitionHandle: MEM_EXTENDED_PARAMETER_TYPE = 3;
pub const MemExtendedParameterUserPhysicalHandle: MEM_EXTENDED_PARAMETER_TYPE = 4;
pub const MemSectionExtendedParameterAttributeFlags: MEM_SECTION_EXTENDED_PARAMETER_TYPE = 4;
pub const MemSectionExtendedParameterInvalidType: MEM_SECTION_EXTENDED_PARAMETER_TYPE = 0;
pub const MemSectionExtendedParameterMax: MEM_SECTION_EXTENDED_PARAMETER_TYPE = 5;
pub const MemSectionExtendedParameterNumaNode: MEM_SECTION_EXTENDED_PARAMETER_TYPE = 2;
pub const MemSectionExtendedParameterSigningLevel: MEM_SECTION_EXTENDED_PARAMETER_TYPE = 3;
pub const MemSectionExtendedParameterUserPhysicalFlags: MEM_SECTION_EXTENDED_PARAMETER_TYPE = 1;
pub const MonitorCapabilities: POWER_INFORMATION_LEVEL = 40;
pub const MonitorInvocation: POWER_INFORMATION_LEVEL = 68;
pub const MonitorRequestReasonAcDcDisplayBurst: POWER_MONITOR_REQUEST_REASON = 5;
pub const MonitorRequestReasonAcDcDisplayBurstSuppressed: POWER_MONITOR_REQUEST_REASON = 28;
pub const MonitorRequestReasonBatteryCountChange: POWER_MONITOR_REQUEST_REASON = 16;
pub const MonitorRequestReasonBatteryCountChangeSuppressed: POWER_MONITOR_REQUEST_REASON = 49;
pub const MonitorRequestReasonBatteryPreCritical: POWER_MONITOR_REQUEST_REASON = 53;
pub const MonitorRequestReasonBuiltinPanel: POWER_MONITOR_REQUEST_REASON = 47;
pub const MonitorRequestReasonDP: POWER_MONITOR_REQUEST_REASON = 19;
pub const MonitorRequestReasonDim: POWER_MONITOR_REQUEST_REASON = 46;
pub const MonitorRequestReasonDirectedDrips: POWER_MONITOR_REQUEST_REASON = 45;
pub const MonitorRequestReasonDisplayRequiredUnDim: POWER_MONITOR_REQUEST_REASON = 48;
pub const MonitorRequestReasonDozeRestrictedStandby: POWER_MONITOR_REQUEST_REASON = 56;
pub const MonitorRequestReasonFullWake: POWER_MONITOR_REQUEST_REASON = 9;
pub const MonitorRequestReasonGracePeriod: POWER_MONITOR_REQUEST_REASON = 17;
pub const MonitorRequestReasonIdleTimeout: POWER_MONITOR_REQUEST_REASON = 12;
pub const MonitorRequestReasonLid: POWER_MONITOR_REQUEST_REASON = 15;
pub const MonitorRequestReasonMax: POWER_MONITOR_REQUEST_REASON = 58;
pub const MonitorRequestReasonNearProximity: POWER_MONITOR_REQUEST_REASON = 22;
pub const MonitorRequestReasonPdcSignal: POWER_MONITOR_REQUEST_REASON = 27;
pub const MonitorRequestReasonPdcSignalFingerprint: POWER_MONITOR_REQUEST_REASON = 44;
pub const MonitorRequestReasonPdcSignalHeyCortana: POWER_MONITOR_REQUEST_REASON = 42;
pub const MonitorRequestReasonPdcSignalHolographicShell: POWER_MONITOR_REQUEST_REASON = 43;
pub const MonitorRequestReasonPdcSignalSensorsHumanPresence: POWER_MONITOR_REQUEST_REASON = 52;
pub const MonitorRequestReasonPdcSignalWindowsMobilePwrNotif: POWER_MONITOR_REQUEST_REASON = 40;
pub const MonitorRequestReasonPdcSignalWindowsMobileShell: POWER_MONITOR_REQUEST_REASON = 41;
pub const MonitorRequestReasonPnP: POWER_MONITOR_REQUEST_REASON = 18;
pub const MonitorRequestReasonPoSetSystemState: POWER_MONITOR_REQUEST_REASON = 7;
pub const MonitorRequestReasonPolicyChange: POWER_MONITOR_REQUEST_REASON = 13;
pub const MonitorRequestReasonPowerButton: POWER_MONITOR_REQUEST_REASON = 1;
pub const MonitorRequestReasonRemoteConnection: POWER_MONITOR_REQUEST_REASON = 2;
pub const MonitorRequestReasonRestrictedStandbyBatteryDrain: POWER_MONITOR_REQUEST_REASON = 55;
pub const MonitorRequestReasonResumeModernStandby: POWER_MONITOR_REQUEST_REASON = 50;
pub const MonitorRequestReasonResumePdc: POWER_MONITOR_REQUEST_REASON = 24;
pub const MonitorRequestReasonResumeS4: POWER_MONITOR_REQUEST_REASON = 25;
pub const MonitorRequestReasonScMonitorpower: POWER_MONITOR_REQUEST_REASON = 3;
pub const MonitorRequestReasonScreenOffRequest: POWER_MONITOR_REQUEST_REASON = 11;
pub const MonitorRequestReasonSessionUnlock: POWER_MONITOR_REQUEST_REASON = 10;
pub const MonitorRequestReasonSetThreadExecutionState: POWER_MONITOR_REQUEST_REASON = 8;
pub const MonitorRequestReasonSleepButton: POWER_MONITOR_REQUEST_REASON = 14;
pub const MonitorRequestReasonSmartRestrictedStandby: POWER_MONITOR_REQUEST_REASON = 57;
pub const MonitorRequestReasonSxTransition: POWER_MONITOR_REQUEST_REASON = 20;
pub const MonitorRequestReasonSystemIdle: POWER_MONITOR_REQUEST_REASON = 21;
pub const MonitorRequestReasonSystemStateEntered: POWER_MONITOR_REQUEST_REASON = 29;
pub const MonitorRequestReasonTerminal: POWER_MONITOR_REQUEST_REASON = 26;
pub const MonitorRequestReasonTerminalInit: POWER_MONITOR_REQUEST_REASON = 51;
pub const MonitorRequestReasonThermalStandby: POWER_MONITOR_REQUEST_REASON = 23;
pub const MonitorRequestReasonUnknown: POWER_MONITOR_REQUEST_REASON = 0;
pub const MonitorRequestReasonUserDisplayBurst: POWER_MONITOR_REQUEST_REASON = 6;
pub const MonitorRequestReasonUserInput: POWER_MONITOR_REQUEST_REASON = 4;
pub const MonitorRequestReasonUserInputAccelerometer: POWER_MONITOR_REQUEST_REASON = 35;
pub const MonitorRequestReasonUserInputHid: POWER_MONITOR_REQUEST_REASON = 36;
pub const MonitorRequestReasonUserInputInitialization: POWER_MONITOR_REQUEST_REASON = 39;
pub const MonitorRequestReasonUserInputKeyboard: POWER_MONITOR_REQUEST_REASON = 31;
pub const MonitorRequestReasonUserInputMouse: POWER_MONITOR_REQUEST_REASON = 32;
pub const MonitorRequestReasonUserInputPen: POWER_MONITOR_REQUEST_REASON = 34;
pub const MonitorRequestReasonUserInputPoUserPresent: POWER_MONITOR_REQUEST_REASON = 37;
pub const MonitorRequestReasonUserInputSessionSwitch: POWER_MONITOR_REQUEST_REASON = 38;
pub const MonitorRequestReasonUserInputTouch: POWER_MONITOR_REQUEST_REASON = 54;
pub const MonitorRequestReasonUserInputTouchpad: POWER_MONITOR_REQUEST_REASON = 33;
pub const MonitorRequestReasonWinrt: POWER_MONITOR_REQUEST_REASON = 30;
pub const MonitorRequestTypeOff: POWER_MONITOR_REQUEST_TYPE = 0;
pub const MonitorRequestTypeOnAndPresent: POWER_MONITOR_REQUEST_TYPE = 1;
pub const MonitorRequestTypeToggleOn: POWER_MONITOR_REQUEST_TYPE = 2;
pub const NATIVE_TYPE_MAX_CB: ReplacesCorHdrNumericDefines = 1;
#[cfg(target_arch = "aarch64")]
pub type NEON128 = ARM64_NT_NEON128;
pub const NETWORK_APP_INSTANCE_CSV_FLAGS_VALID_ONLY_IF_CSV_COORDINATOR: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NETWORK_APP_INSTANCE_EA {
    pub AppInstanceID: windows_core::GUID,
    pub CsvFlags: u32,
}
pub const NLS_VALID_LOCALE_MASK: u32 = 1048575;
pub const NONVOL_FP_NUMREG_ARM64: u32 = 8;
pub const NONVOL_INT_NUMREG_ARM64: u32 = 11;
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct NON_PAGED_DEBUG_INFO {
    pub Signature: u16,
    pub Flags: u16,
    pub Size: u32,
    pub Machine: u16,
    pub Characteristics: u16,
    pub TimeDateStamp: u32,
    pub CheckSum: u32,
    pub SizeOfImage: u32,
    pub ImageBase: u64,
}
pub const NON_PAGED_DEBUG_SIGNATURE: u32 = 18766;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NOTIFY_USER_POWER_SETTING {
    pub Guid: windows_core::GUID,
}
pub const NO_PROPAGATE_INHERIT_ACE: u32 = 4;
pub const NO_SUBGROUP_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xfea3413e_7e05_4911_9a71_700331f1c294);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NPSTR(pub *mut i8);
impl NPSTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for NPSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "excpt")]
#[derive(Clone, Copy)]
pub struct NT_TIB {
    pub ExceptionList: *mut EXCEPTION_REGISTRATION_RECORD,
    pub StackBase: *mut core::ffi::c_void,
    pub StackLimit: *mut core::ffi::c_void,
    pub SubSystemTib: *mut core::ffi::c_void,
    pub Anonymous: NT_TIB_0,
    pub ArbitraryUserPointer: *mut core::ffi::c_void,
    pub Self_: *mut Self,
}
#[cfg(feature = "excpt")]
impl Default for NT_TIB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "excpt")]
#[derive(Clone, Copy)]
pub union NT_TIB_0 {
    pub FiberData: *mut core::ffi::c_void,
    pub Version: u32,
}
#[cfg(feature = "excpt")]
impl Default for NT_TIB_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NT_TIB32 {
    pub ExceptionList: u32,
    pub StackBase: u32,
    pub StackLimit: u32,
    pub SubSystemTib: u32,
    pub Anonymous: NT_TIB32_0,
    pub ArbitraryUserPointer: u32,
    pub Self_: u32,
}
impl Default for NT_TIB32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NT_TIB32_0 {
    pub FiberData: u32,
    pub Version: u32,
}
impl Default for NT_TIB32_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NT_TIB64 {
    pub ExceptionList: u64,
    pub StackBase: u64,
    pub StackLimit: u64,
    pub SubSystemTib: u64,
    pub Anonymous: NT_TIB64_0,
    pub ArbitraryUserPointer: u64,
    pub Self_: u64,
}
impl Default for NT_TIB64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NT_TIB64_0 {
    pub FiberData: u64,
    pub Version: u32,
}
impl Default for NT_TIB64_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "basetsd")]
#[derive(Clone, Copy)]
pub struct NUMA_NODE_RELATIONSHIP {
    pub NodeNumber: u32,
    pub Reserved: [u8; 18],
    pub GroupCount: u16,
    pub Anonymous: NUMA_NODE_RELATIONSHIP_0,
}
#[cfg(feature = "basetsd")]
impl Default for NUMA_NODE_RELATIONSHIP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "basetsd")]
#[derive(Clone, Copy)]
pub union NUMA_NODE_RELATIONSHIP_0 {
    pub GroupMask: GROUP_AFFINITY,
    pub GroupMasks: [GROUP_AFFINITY; 1],
}
#[cfg(feature = "basetsd")]
impl Default for NUMA_NODE_RELATIONSHIP_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NUM_DISCHARGE_POLICIES: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NV_MEMORY_RANGE {
    pub BaseAddress: *mut core::ffi::c_void,
    pub Length: usize,
}
impl Default for NV_MEMORY_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NWPSTR(pub *mut u16);
impl NWPSTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for NWPSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const N_BTMASK: u32 = 15;
pub const N_BTSHFT: u32 = 4;
pub const N_TMASK: u32 = 48;
pub const N_TMASK1: u32 = 192;
pub const N_TMASK2: u32 = 240;
pub const N_TSHIFT: u32 = 2;
pub const NormalError: SERVICE_ERROR_TYPE = 1;
pub const NotifyUserModeLegacyPowerEvent: POWER_INFORMATION_LEVEL = 47;
pub const NotifyUserPowerSetting: POWER_INFORMATION_LEVEL = 26;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OBJECTID {
    pub Lineage: windows_core::GUID,
    pub Uniquifier: u32,
}
pub const OBJECT_INHERIT_ACE: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OBJECT_TYPE_LIST {
    pub Level: u16,
    pub Sbz: u16,
    pub ObjectType: *mut windows_core::GUID,
}
impl Default for OBJECT_TYPE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type OSVERSIONINFO = OSVERSIONINFOA;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OSVERSIONINFOA {
    pub dwOSVersionInfoSize: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub dwBuildNumber: u32,
    pub dwPlatformId: u32,
    pub szCSDVersion: [i8; 128],
}
impl Default for OSVERSIONINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type OSVERSIONINFOEX = OSVERSIONINFOEXA;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OSVERSIONINFOEXA {
    pub dwOSVersionInfoSize: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub dwBuildNumber: u32,
    pub dwPlatformId: u32,
    pub szCSDVersion: [i8; 128],
    pub wServicePackMajor: u16,
    pub wServicePackMinor: u16,
    pub wSuiteMask: u16,
    pub wProductType: u8,
    pub wReserved: u8,
}
impl Default for OSVERSIONINFOEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OSVERSIONINFOEXW {
    pub dwOSVersionInfoSize: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub dwBuildNumber: u32,
    pub dwPlatformId: u32,
    pub szCSDVersion: [u16; 128],
    pub wServicePackMajor: u16,
    pub wServicePackMinor: u16,
    pub wSuiteMask: u16,
    pub wProductType: u8,
    pub wReserved: u8,
}
impl Default for OSVERSIONINFOEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OSVERSIONINFOW {
    pub dwOSVersionInfoSize: u32,
    pub dwMajorVersion: u32,
    pub dwMinorVersion: u32,
    pub dwBuildNumber: u32,
    pub dwPlatformId: u32,
    pub szCSDVersion: [u16; 128],
}
impl Default for OSVERSIONINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type OS_DEPLOYEMENT_STATE_VALUES = i32;
pub const OS_DEPLOYMENT_COMPACT: OS_DEPLOYEMENT_STATE_VALUES = 2;
pub const OS_DEPLOYMENT_STANDARD: OS_DEPLOYEMENT_STATE_VALUES = 1;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub type OUT_OF_PROCESS_FUNCTION_TABLE_CALLBACK = Option<unsafe extern "system" fn(process: HANDLE, tableaddress: *const core::ffi::c_void, entries: *mut u32, functions: *mut PRUNTIME_FUNCTION) -> u32>;
#[cfg(target_arch = "aarch64")]
pub type OUT_OF_PROCESS_FUNCTION_TABLE_CALLBACK = Option<unsafe extern "system" fn(process: HANDLE, tableaddress: *const core::ffi::c_void, entries: *mut u32, functions: *mut PARM64_RUNTIME_FUNCTION) -> u32>;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const OUT_OF_PROCESS_FUNCTION_TABLE_CALLBACK_EXPORT_NAME: windows_core::PCSTR = windows_core::s!("OutOfProcessFunctionTableCallback");
pub const OWNER_SECURITY_INFORMATION: u32 = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PACCESS_ALLOWED_ACE(pub *mut ACCESS_ALLOWED_ACE);
impl PACCESS_ALLOWED_ACE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PACCESS_ALLOWED_ACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PACCESS_ALLOWED_CALLBACK_ACE(pub *mut ACCESS_ALLOWED_CALLBACK_ACE);
impl PACCESS_ALLOWED_CALLBACK_ACE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PACCESS_ALLOWED_CALLBACK_ACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PACCESS_ALLOWED_CALLBACK_OBJECT_ACE(pub *mut ACCESS_ALLOWED_CALLBACK_OBJECT_ACE);
impl PACCESS_ALLOWED_CALLBACK_OBJECT_ACE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PACCESS_ALLOWED_CALLBACK_OBJECT_ACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PACCESS_ALLOWED_OBJECT_ACE(pub *mut ACCESS_ALLOWED_OBJECT_ACE);
impl PACCESS_ALLOWED_OBJECT_ACE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PACCESS_ALLOWED_OBJECT_ACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PACCESS_DENIED_ACE(pub *mut ACCESS_DENIED_ACE);
impl PACCESS_DENIED_ACE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PACCESS_DENIED_ACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PACCESS_DENIED_CALLBACK_ACE(pub *mut ACCESS_DENIED_CALLBACK_ACE);
impl PACCESS_DENIED_CALLBACK_ACE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PACCESS_DENIED_CALLBACK_ACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PACCESS_DENIED_CALLBACK_OBJECT_ACE(pub *mut ACCESS_DENIED_CALLBACK_OBJECT_ACE);
impl PACCESS_DENIED_CALLBACK_OBJECT_ACE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PACCESS_DENIED_CALLBACK_OBJECT_ACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PACCESS_DENIED_OBJECT_ACE(pub *mut ACCESS_DENIED_OBJECT_ACE);
impl PACCESS_DENIED_OBJECT_ACE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PACCESS_DENIED_OBJECT_ACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PACCESS_MASK(pub *mut ACCESS_MASK);
impl PACCESS_MASK {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PACCESS_MASK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PACCESS_REASONS(pub *mut ACCESS_REASONS);
impl PACCESS_REASONS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PACCESS_REASONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PACCESS_TOKEN(pub *mut core::ffi::c_void);
impl PACCESS_TOKEN {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PACCESS_TOKEN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PACE_HEADER(pub *mut ACE_HEADER);
impl PACE_HEADER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PACE_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PACKEDEVENTINFO {
    pub ulSize: u32,
    pub ulNumEventsForLogFile: u32,
    pub ulOffsets: [u32; 0],
}
impl Default for PACKEDEVENTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PACL(pub *mut ACL);
impl PACL {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PACL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PACL_REVISION_INFORMATION(pub *mut ACL_REVISION_INFORMATION);
impl PACL_REVISION_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PACL_REVISION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PACL_SIZE_INFORMATION(pub *mut ACL_SIZE_INFORMATION);
impl PACL_SIZE_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PACL_SIZE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION(pub *mut ACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION);
impl PACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION(pub *mut ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION);
impl PACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PACTIVATION_CONTEXT_DETAILED_INFORMATION(pub *mut ACTIVATION_CONTEXT_DETAILED_INFORMATION);
impl PACTIVATION_CONTEXT_DETAILED_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PACTIVATION_CONTEXT_DETAILED_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PACTIVATION_CONTEXT_QUERY_INDEX(pub *mut ACTIVATION_CONTEXT_QUERY_INDEX);
impl PACTIVATION_CONTEXT_QUERY_INDEX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PACTIVATION_CONTEXT_QUERY_INDEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION(pub *mut ACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION);
impl PACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PADMINISTRATOR_POWER_POLICY(pub *mut ADMINISTRATOR_POWER_POLICY);
impl PADMINISTRATOR_POWER_POLICY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PADMINISTRATOR_POWER_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PAGE_ENCLAVE_DECOMMIT: u32 = 268435456;
pub const PAGE_ENCLAVE_MASK: u32 = 268435456;
pub const PAGE_ENCLAVE_SS_FIRST: u32 = 268435457;
pub const PAGE_ENCLAVE_SS_REST: u32 = 268435458;
pub const PAGE_ENCLAVE_THREAD_CONTROL: u32 = 2147483648;
pub const PAGE_ENCLAVE_UNVALIDATED: u32 = 536870912;
pub const PAGE_EXECUTE: u32 = 16;
pub const PAGE_EXECUTE_READ: u32 = 32;
pub const PAGE_EXECUTE_READWRITE: u32 = 64;
pub const PAGE_EXECUTE_WRITECOPY: u32 = 128;
pub const PAGE_GRAPHICS_COHERENT: u32 = 131072;
pub const PAGE_GRAPHICS_EXECUTE: u32 = 16384;
pub const PAGE_GRAPHICS_EXECUTE_READ: u32 = 32768;
pub const PAGE_GRAPHICS_EXECUTE_READWRITE: u32 = 65536;
pub const PAGE_GRAPHICS_NOACCESS: u32 = 2048;
pub const PAGE_GRAPHICS_NOCACHE: u32 = 262144;
pub const PAGE_GRAPHICS_READONLY: u32 = 4096;
pub const PAGE_GRAPHICS_READWRITE: u32 = 8192;
pub const PAGE_GUARD: u32 = 256;
pub const PAGE_NOACCESS: u32 = 1;
pub const PAGE_NOCACHE: u32 = 512;
pub const PAGE_READONLY: u32 = 2;
pub const PAGE_READWRITE: u32 = 4;
pub const PAGE_REVERT_TO_FILE_MAP: u32 = 2147483648;
pub const PAGE_TARGETS_INVALID: u32 = 1073741824;
pub const PAGE_TARGETS_NO_UPDATE: u32 = 1073741824;
pub const PAGE_WRITECOMBINE: u32 = 1024;
pub const PAGE_WRITECOPY: u32 = 8;
pub type PAPCFUNC = Option<unsafe extern "system" fn(parameter: usize)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PAPPLICATIONLAUNCH_SETTING_VALUE(pub *mut APPLICATIONLAUNCH_SETTING_VALUE);
impl PAPPLICATIONLAUNCH_SETTING_VALUE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PAPPLICATIONLAUNCH_SETTING_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PARKING_TOPOLOGY_POLICY_DISABLED: u32 = 0;
pub const PARKING_TOPOLOGY_POLICY_ROUNDROBIN: u32 = 1;
pub const PARKING_TOPOLOGY_POLICY_ROUNDROBIN_P_ROUNDROBIN_E: u32 = 3;
pub const PARKING_TOPOLOGY_POLICY_ROUNDROBIN_P_SEQUENTIAL_E: u32 = 5;
pub const PARKING_TOPOLOGY_POLICY_SEQUENTIAL: u32 = 2;
pub const PARKING_TOPOLOGY_POLICY_SEQUENTIAL_P_ROUNDROBIN_E: u32 = 6;
pub const PARKING_TOPOLOGY_POLICY_SEQUENTIAL_P_SEQUENTIAL_E: u32 = 4;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PARM64EC_NT_CONTEXT(pub *mut ARM64EC_NT_CONTEXT);
impl PARM64EC_NT_CONTEXT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PARM64EC_NT_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PARM64_NT_CONTEXT(pub *mut ARM64_NT_CONTEXT);
impl PARM64_NT_CONTEXT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PARM64_NT_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PARM64_NT_NEON128(pub *mut ARM64_NT_NEON128);
impl PARM64_NT_NEON128 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PARM64_NT_NEON128 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PARM64_RUNTIME_FUNCTION(pub *mut ARM64_RUNTIME_FUNCTION);
impl PARM64_RUNTIME_FUNCTION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PARM64_RUNTIME_FUNCTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PARM64_TPIDR2_BLOCK(pub *mut ARM64_TPIDR2_BLOCK);
impl PARM64_TPIDR2_BLOCK {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PARM64_TPIDR2_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PASSEMBLY_FILE_DETAILED_INFORMATION(pub *mut ASSEMBLY_FILE_DETAILED_INFORMATION);
impl PASSEMBLY_FILE_DETAILED_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PASSEMBLY_FILE_DETAILED_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PATTRIBUTES_AND_SID(pub *mut ATTRIBUTES_AND_SID);
impl PATTRIBUTES_AND_SID {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PATTRIBUTES_AND_SID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PAUDIT_EVENT_TYPE(pub *mut AUDIT_EVENT_TYPE);
impl PAUDIT_EVENT_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PAUDIT_EVENT_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PBATTERY_REPORTING_SCALE(pub *mut BATTERY_REPORTING_SCALE);
impl PBATTERY_REPORTING_SCALE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PBATTERY_REPORTING_SCALE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PBOOLEAN(pub *mut bool);
impl PBOOLEAN {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PBOOLEAN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCACHE_DESCRIPTOR(pub *mut CACHE_DESCRIPTOR);
impl PCACHE_DESCRIPTOR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCACHE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "basetsd")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCACHE_RELATIONSHIP(pub *mut CACHE_RELATIONSHIP);
#[cfg(feature = "basetsd")]
impl PCACHE_RELATIONSHIP {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "basetsd")]
impl Default for PCACHE_RELATIONSHIP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION(pub *const ACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION);
impl PCACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCACTIVATION_CONTEXT_ASSEMBLY_DETAILED_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION(pub *const ACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION);
impl PCACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCACTIVATION_CONTEXT_COMPATIBILITY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCACTIVATION_CONTEXT_DETAILED_INFORMATION(pub *const ACTIVATION_CONTEXT_DETAILED_INFORMATION);
impl PCACTIVATION_CONTEXT_DETAILED_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCACTIVATION_CONTEXT_DETAILED_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCACTIVATION_CONTEXT_QUERY_INDEX(pub *const ACTIVATION_CONTEXT_QUERY_INDEX);
impl PCACTIVATION_CONTEXT_QUERY_INDEX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCACTIVATION_CONTEXT_QUERY_INDEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION(pub *const ACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION);
impl PCACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCACTIVATION_CONTEXT_RUN_LEVEL_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCASSEMBLY_FILE_DETAILED_INFORMATION(pub *const ASSEMBLY_FILE_DETAILED_INFORMATION);
impl PCASSEMBLY_FILE_DETAILED_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCASSEMBLY_FILE_DETAILED_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCCH(pub *const i8);
impl PCCH {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCCH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCCOMPATIBILITY_CONTEXT_ELEMENT(pub *const COMPATIBILITY_CONTEXT_ELEMENT);
impl PCCOMPATIBILITY_CONTEXT_ELEMENT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCCOMPATIBILITY_CONTEXT_ELEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCFG_CALL_TARGET_INFO(pub *mut CFG_CALL_TARGET_INFO);
impl PCFG_CALL_TARGET_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCFG_CALL_TARGET_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCH(pub *mut i8);
impl PCH {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCHAR(pub *mut i8);
impl PCHAR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCHAR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCIMAGE_DELAYLOAD_DESCRIPTOR(pub *const IMAGE_DELAYLOAD_DESCRIPTOR);
impl PCIMAGE_DELAYLOAD_DESCRIPTOR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCIMAGE_DELAYLOAD_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCIMAGE_POLICY_ENTRY(pub *const IMAGE_POLICY_ENTRY);
impl PCIMAGE_POLICY_ENTRY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCIMAGE_POLICY_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCIMAGE_POLICY_METADATA(pub *const IMAGE_POLICY_METADATA);
impl PCIMAGE_POLICY_METADATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCIMAGE_POLICY_METADATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLAIMS_BLOB(pub *mut core::ffi::c_void);
impl PCLAIMS_BLOB {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLAIMS_BLOB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "basetsd")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLAIM_SECURITY_ATTRIBUTES_INFORMATION(pub *mut CLAIM_SECURITY_ATTRIBUTES_INFORMATION);
#[cfg(feature = "basetsd")]
impl PCLAIM_SECURITY_ATTRIBUTES_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "basetsd")]
impl Default for PCLAIM_SECURITY_ATTRIBUTES_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE(pub *mut CLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE);
impl PCLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLAIM_SECURITY_ATTRIBUTE_FQBN_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE(pub *mut CLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE);
impl PCLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLAIM_SECURITY_ATTRIBUTE_OCTET_STRING_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1(pub *mut CLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1);
impl PCLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLAIM_SECURITY_ATTRIBUTE_RELATIVE_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "basetsd")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLAIM_SECURITY_ATTRIBUTE_V1(pub *mut CLAIM_SECURITY_ATTRIBUTE_V1);
#[cfg(feature = "basetsd")]
impl PCLAIM_SECURITY_ATTRIBUTE_V1 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "basetsd")]
impl Default for PCLAIM_SECURITY_ATTRIBUTE_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCM_POWER_DATA(pub *mut CM_POWER_DATA);
impl PCM_POWER_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCM_POWER_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCNZCH(pub *const i8);
impl PCNZCH {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCNZCH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PCNZTCH(pub PCNZCH);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCNZWCH(pub *const u16);
impl PCNZWCH {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCNZWCH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCOMPARTMENT_ID(pub *mut COMPARTMENT_ID);
impl PCOMPARTMENT_ID {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCOMPARTMENT_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCOMPATIBILITY_CONTEXT_ELEMENT(pub *mut COMPATIBILITY_CONTEXT_ELEMENT);
impl PCOMPATIBILITY_CONTEXT_ELEMENT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCOMPATIBILITY_CONTEXT_ELEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCOMPONENT_FILTER(pub *mut COMPONENT_FILTER);
impl PCOMPONENT_FILTER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCOMPONENT_FILTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCONTEXT(pub *mut CONTEXT);
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
impl PCONTEXT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
impl Default for PCONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "aarch64")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCONTEXT(pub *mut ARM64_NT_CONTEXT);
#[cfg(target_arch = "aarch64")]
impl PCONTEXT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(target_arch = "aarch64")]
impl Default for PCONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCORRELATION_VECTOR(pub *mut CORRELATION_VECTOR);
impl PCORRELATION_VECTOR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCORRELATION_VECTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCPU_SET_INFORMATION_TYPE(pub *mut CPU_SET_INFORMATION_TYPE);
impl PCPU_SET_INFORMATION_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCPU_SET_INFORMATION_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PCTCH(pub LPCCH);
pub type PCTSTR = windows_core::PCSTR;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCUCSCHAR(pub *const UCSCHAR);
impl PCUCSCHAR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCUCSCHAR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCUCSSTR(pub *const UCSCHAR);
impl PCUCSSTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCUCSSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PCUNZTCH(pub PCNZCH);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCUNZWCH(pub *const u16);
impl PCUNZWCH {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCUNZWCH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG(pub *mut CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG);
impl PCUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PCUTSTR = windows_core::PCSTR;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCUUCSCHAR(pub *const UCSCHAR);
impl PCUUCSCHAR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCUUCSCHAR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCUUCSSTR(pub *const UCSCHAR);
impl PCUUCSSTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCUUCSSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCUWCHAR(pub *const u16);
impl PCUWCHAR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCUWCHAR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCUWSTR(pub *const u16);
impl PCUWSTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCUWSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PCUZZTSTR(pub PCZZSTR);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCUZZWSTR(pub *const u16);
impl PCUZZWSTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCUZZWSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCWCH(pub *const u16);
impl PCWCH {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCWCH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCWCHAR(pub *const u16);
impl PCWCHAR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCWCHAR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCZPCSTR(pub *const windows_core::PCSTR);
impl PCZPCSTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCZPCSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCZPCWSTR(pub *const windows_core::PCWSTR);
impl PCZPCWSTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCZPCWSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCZPSTR(pub *const windows_core::PSTR);
impl PCZPSTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCZPSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCZPWSTR(pub *const windows_core::PWSTR);
impl PCZPWSTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCZPWSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCZZSTR(pub *const i8);
impl PCZZSTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCZZSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PCZZTSTR(pub PCZZSTR);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCZZWSTR(pub *const u16);
impl PCZZWSTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCZZWSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PDCAP_D0_SUPPORTED: u32 = 1;
pub const PDCAP_D1_SUPPORTED: u32 = 2;
pub const PDCAP_D2_SUPPORTED: u32 = 4;
pub const PDCAP_D3_SUPPORTED: u32 = 8;
pub const PDCAP_WAKE_FROM_D0_SUPPORTED: u32 = 16;
pub const PDCAP_WAKE_FROM_D1_SUPPORTED: u32 = 32;
pub const PDCAP_WAKE_FROM_D2_SUPPORTED: u32 = 64;
pub const PDCAP_WAKE_FROM_D3_SUPPORTED: u32 = 128;
pub const PDCAP_WARM_EJECT_SUPPORTED: u32 = 256;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDEVICE_POWER_STATE(pub *mut DEVICE_POWER_STATE);
impl PDEVICE_POWER_STATE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDEVICE_POWER_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "excpt", feature = "minwindef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDISPATCHER_CONTEXT(pub *mut DISPATCHER_CONTEXT);
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "excpt", feature = "minwindef"))]
impl PDISPATCHER_CONTEXT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "excpt", feature = "minwindef"))]
impl Default for PDISPATCHER_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "aarch64")]
#[cfg(all(feature = "excpt", feature = "minwindef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDISPATCHER_CONTEXT(pub *mut DISPATCHER_CONTEXT_ARM64);
#[cfg(target_arch = "aarch64")]
#[cfg(all(feature = "excpt", feature = "minwindef"))]
impl PDISPATCHER_CONTEXT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(target_arch = "aarch64")]
#[cfg(all(feature = "excpt", feature = "minwindef"))]
impl Default for PDISPATCHER_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "excpt", feature = "minwindef"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDISPATCHER_CONTEXT_ARM64(pub *mut DISPATCHER_CONTEXT_ARM64);
#[cfg(all(feature = "excpt", feature = "minwindef"))]
impl PDISPATCHER_CONTEXT_ARM64 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "excpt", feature = "minwindef"))]
impl Default for PDISPATCHER_CONTEXT_ARM64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDRIVER_INFO_ENTRY(pub *mut DRIVER_INFO_ENTRY);
impl PDRIVER_INFO_ENTRY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDRIVER_INFO_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDRIVER_RUNTIME_REPORT(pub *mut DRIVER_RUNTIME_REPORT);
impl PDRIVER_RUNTIME_REPORT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDRIVER_RUNTIME_REPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PDWORDLONG(pub *mut DWORDLONG);
impl PDWORDLONG {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PDWORDLONG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PENCLAVE_CREATE_INFO_SGX(pub *mut ENCLAVE_CREATE_INFO_SGX);
impl PENCLAVE_CREATE_INFO_SGX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PENCLAVE_CREATE_INFO_SGX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PENCLAVE_CREATE_INFO_VBS(pub *mut ENCLAVE_CREATE_INFO_VBS);
impl PENCLAVE_CREATE_INFO_VBS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PENCLAVE_CREATE_INFO_VBS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PENCLAVE_CREATE_INFO_VBS_BASIC(pub *mut ENCLAVE_CREATE_INFO_VBS_BASIC);
impl PENCLAVE_CREATE_INFO_VBS_BASIC {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PENCLAVE_CREATE_INFO_VBS_BASIC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PENCLAVE_INIT_INFO_SGX(pub *mut ENCLAVE_INIT_INFO_SGX);
impl PENCLAVE_INIT_INFO_SGX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PENCLAVE_INIT_INFO_SGX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PENCLAVE_INIT_INFO_VBS(pub *mut ENCLAVE_INIT_INFO_VBS);
impl PENCLAVE_INIT_INFO_VBS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PENCLAVE_INIT_INFO_VBS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PENCLAVE_INIT_INFO_VBS_BASIC(pub *mut ENCLAVE_INIT_INFO_VBS_BASIC);
impl PENCLAVE_INIT_INFO_VBS_BASIC {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PENCLAVE_INIT_INFO_VBS_BASIC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PENCLAVE_LOAD_DATA_VBS_BASIC(pub *mut ENCLAVE_LOAD_DATA_VBS_BASIC);
impl PENCLAVE_LOAD_DATA_VBS_BASIC {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PENCLAVE_LOAD_DATA_VBS_BASIC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PENCLAVE_TARGET_FUNCTION(pub *mut ENCLAVE_TARGET_FUNCTION);
impl PENCLAVE_TARGET_FUNCTION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PENCLAVE_TARGET_FUNCTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PENERGY_SAVER_STATUS(pub *mut ENERGY_SAVER_STATUS);
impl PENERGY_SAVER_STATUS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PENERGY_SAVER_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PENLISTMENT_BASIC_INFORMATION(pub *mut ENLISTMENT_BASIC_INFORMATION);
impl PENLISTMENT_BASIC_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PENLISTMENT_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PENLISTMENT_CRM_INFORMATION(pub *mut ENLISTMENT_CRM_INFORMATION);
impl PENLISTMENT_CRM_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PENLISTMENT_CRM_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PERFORMANCE_DATA {
    pub Size: u16,
    pub Version: u8,
    pub HwCountersCount: u8,
    pub ContextSwitchCount: u32,
    pub WaitReasonBitMap: u64,
    pub CycleTime: u64,
    pub RetryCount: u32,
    pub Reserved: u32,
    pub HwCounters: [HARDWARE_COUNTER_DATA; 16],
}
impl Default for PERFORMANCE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PERFORMANCE_DATA_VERSION: u32 = 1;
pub const PERFSTATE_POLICY_CHANGE_DECREASE_MAX: u32 = 2;
pub const PERFSTATE_POLICY_CHANGE_IDEAL: u32 = 0;
pub const PERFSTATE_POLICY_CHANGE_IDEAL_AGGRESSIVE: u32 = 3;
pub const PERFSTATE_POLICY_CHANGE_INCREASE_MAX: u32 = 3;
pub const PERFSTATE_POLICY_CHANGE_ROCKET: u32 = 2;
pub const PERFSTATE_POLICY_CHANGE_SINGLE: u32 = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PEVENTLOGRECORD(pub *mut EVENTLOGRECORD);
impl PEVENTLOGRECORD {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PEVENTLOGRECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PEVENTSFORLOGFILE(pub *mut EVENTSFORLOGFILE);
impl PEVENTSFORLOGFILE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PEVENTSFORLOGFILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub type PEXCEPTION_FILTER = Option<unsafe extern "system" fn(exceptionpointers: *mut EXCEPTION_POINTERS, establisherframe: *mut core::ffi::c_void) -> i32>;
#[cfg(target_arch = "aarch64")]
pub type PEXCEPTION_FILTER = Option<unsafe extern "system" fn(exceptionpointers: *mut EXCEPTION_POINTERS, establisherframe: u64) -> i32>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PEXCEPTION_POINTERS(pub *mut EXCEPTION_POINTERS);
impl PEXCEPTION_POINTERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PEXCEPTION_POINTERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PEXCEPTION_RECORD(pub *mut EXCEPTION_RECORD);
impl PEXCEPTION_RECORD {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PEXCEPTION_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PEXCEPTION_RECORD32(pub *mut EXCEPTION_RECORD32);
impl PEXCEPTION_RECORD32 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PEXCEPTION_RECORD32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PEXCEPTION_RECORD64(pub *mut EXCEPTION_RECORD64);
impl PEXCEPTION_RECORD64 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PEXCEPTION_RECORD64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "excpt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PEXCEPTION_REGISTRATION_RECORD(pub *mut EXCEPTION_REGISTRATION_RECORD);
#[cfg(feature = "excpt")]
impl PEXCEPTION_REGISTRATION_RECORD {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "excpt")]
impl Default for PEXCEPTION_REGISTRATION_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[cfg(feature = "excpt")]
pub type PEXCEPTION_ROUTINE = Option<unsafe extern "system" fn(exceptionrecord: *mut EXCEPTION_RECORD, establisherframe: *const core::ffi::c_void, contextrecord: *mut CONTEXT, dispatchercontext: *const core::ffi::c_void) -> super::excpt::EXCEPTION_DISPOSITION>;
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "excpt")]
pub type PEXCEPTION_ROUTINE = Option<unsafe extern "system" fn(exceptionrecord: *mut EXCEPTION_RECORD, establisherframe: *const core::ffi::c_void, contextrecord: *mut ARM64_NT_CONTEXT, dispatchercontext: *const core::ffi::c_void) -> super::excpt::EXCEPTION_DISPOSITION>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PEXECUTION_STATE(pub *mut u32);
impl PEXECUTION_STATE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PEXECUTION_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFILE_CASE_SENSITIVE_INFORMATION(pub *mut FILE_CASE_SENSITIVE_INFORMATION);
impl PFILE_CASE_SENSITIVE_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFILE_CASE_SENSITIVE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFILE_ID_128(pub *mut FILE_ID_128);
impl PFILE_ID_128 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFILE_ID_128 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFILE_NOTIFY_EXTENDED_INFORMATION(pub *mut FILE_NOTIFY_EXTENDED_INFORMATION);
impl PFILE_NOTIFY_EXTENDED_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFILE_NOTIFY_EXTENDED_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFILE_NOTIFY_FULL_INFORMATION(pub *mut FILE_NOTIFY_FULL_INFORMATION);
impl PFILE_NOTIFY_FULL_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFILE_NOTIFY_FULL_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFILE_NOTIFY_INFORMATION(pub *mut FILE_NOTIFY_INFORMATION);
impl PFILE_NOTIFY_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFILE_NOTIFY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFILE_SEGMENT_ELEMENT(pub *mut FILE_SEGMENT_ELEMENT);
impl PFILE_SEGMENT_ELEMENT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFILE_SEGMENT_ELEMENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFILE_STAT_BASIC_INFORMATION(pub *mut FILE_STAT_BASIC_INFORMATION);
impl PFILE_STAT_BASIC_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFILE_STAT_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFILE_STAT_INFORMATION(pub *mut FILE_STAT_INFORMATION);
impl PFILE_STAT_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFILE_STAT_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFILE_STAT_LX_INFORMATION(pub *mut FILE_STAT_LX_INFORMATION);
impl PFILE_STAT_LX_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFILE_STAT_LX_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFIRMWARE_TYPE(pub *mut FIRMWARE_TYPE);
impl PFIRMWARE_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFIRMWARE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFLOAT128(pub *mut FLOAT128);
impl PFLOAT128 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFLOAT128 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFLOATING_SAVE_AREA(pub *mut FLOATING_SAVE_AREA);
#[cfg(target_arch = "x86")]
impl PFLOATING_SAVE_AREA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(target_arch = "x86")]
impl Default for PFLOATING_SAVE_AREA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PFLS_CALLBACK_FUNCTION = Option<unsafe extern "system" fn(lpflsdata: *const core::ffi::c_void)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFPO_DATA(pub *mut FPO_DATA);
impl PFPO_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFPO_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PF_3DNOW_INSTRUCTIONS_AVAILABLE: u32 = 7;
pub const PF_ALPHA_BYTE_INSTRUCTIONS: u32 = 5;
pub const PF_ARM_64BIT_LOADSTORE_ATOMIC: u32 = 25;
pub const PF_ARM_DIVIDE_INSTRUCTION_AVAILABLE: u32 = 24;
pub const PF_ARM_EXTERNAL_CACHE_AVAILABLE: u32 = 26;
pub const PF_ARM_FMAC_INSTRUCTIONS_AVAILABLE: u32 = 27;
pub const PF_ARM_LSE2_AVAILABLE: u32 = 62;
pub const PF_ARM_NEON_INSTRUCTIONS_AVAILABLE: u32 = 19;
pub const PF_ARM_SHA3_INSTRUCTIONS_AVAILABLE: u32 = 64;
pub const PF_ARM_SHA512_INSTRUCTIONS_AVAILABLE: u32 = 65;
pub const PF_ARM_SME2_1_INSTRUCTIONS_AVAILABLE: u32 = 72;
pub const PF_ARM_SME2_2_INSTRUCTIONS_AVAILABLE: u32 = 73;
pub const PF_ARM_SME2_INSTRUCTIONS_AVAILABLE: u32 = 71;
pub const PF_ARM_SME_AES_INSTRUCTIONS_AVAILABLE: u32 = 74;
pub const PF_ARM_SME_B16B16_INSTRUCTIONS_AVAILABLE: u32 = 84;
pub const PF_ARM_SME_F16F16_INSTRUCTIONS_AVAILABLE: u32 = 83;
pub const PF_ARM_SME_F64F64_INSTRUCTIONS_AVAILABLE: u32 = 85;
pub const PF_ARM_SME_F8F16_INSTRUCTIONS_AVAILABLE: u32 = 82;
pub const PF_ARM_SME_F8F32_INSTRUCTIONS_AVAILABLE: u32 = 81;
pub const PF_ARM_SME_FA64_INSTRUCTIONS_AVAILABLE: u32 = 88;
pub const PF_ARM_SME_I16I64_INSTRUCTIONS_AVAILABLE: u32 = 86;
pub const PF_ARM_SME_INSTRUCTIONS_AVAILABLE: u32 = 70;
pub const PF_ARM_SME_LUTv2_INSTRUCTIONS_AVAILABLE: u32 = 87;
pub const PF_ARM_SME_SBITPERM_INSTRUCTIONS_AVAILABLE: u32 = 75;
pub const PF_ARM_SME_SF8DP2_INSTRUCTIONS_AVAILABLE: u32 = 78;
pub const PF_ARM_SME_SF8DP4_INSTRUCTIONS_AVAILABLE: u32 = 79;
pub const PF_ARM_SME_SF8FMA_INSTRUCTIONS_AVAILABLE: u32 = 80;
pub const PF_ARM_SME_SF8MM4_INSTRUCTIONS_AVAILABLE: u32 = 76;
pub const PF_ARM_SME_SF8MM8_INSTRUCTIONS_AVAILABLE: u32 = 77;
pub const PF_ARM_SVE2_1_INSTRUCTIONS_AVAILABLE: u32 = 48;
pub const PF_ARM_SVE2_INSTRUCTIONS_AVAILABLE: u32 = 47;
pub const PF_ARM_SVE_AES_INSTRUCTIONS_AVAILABLE: u32 = 49;
pub const PF_ARM_SVE_B16B16_INSTRUCTIONS_AVAILABLE: u32 = 54;
pub const PF_ARM_SVE_BF16_INSTRUCTIONS_AVAILABLE: u32 = 52;
pub const PF_ARM_SVE_BITPERM_INSTRUCTIONS_AVAILABLE: u32 = 51;
pub const PF_ARM_SVE_EBF16_INSTRUCTIONS_AVAILABLE: u32 = 53;
pub const PF_ARM_SVE_F32MM_INSTRUCTIONS_AVAILABLE: u32 = 58;
pub const PF_ARM_SVE_F64MM_INSTRUCTIONS_AVAILABLE: u32 = 59;
pub const PF_ARM_SVE_I8MM_INSTRUCTIONS_AVAILABLE: u32 = 57;
pub const PF_ARM_SVE_INSTRUCTIONS_AVAILABLE: u32 = 46;
pub const PF_ARM_SVE_PMULL128_INSTRUCTIONS_AVAILABLE: u32 = 50;
pub const PF_ARM_SVE_SHA3_INSTRUCTIONS_AVAILABLE: u32 = 55;
pub const PF_ARM_SVE_SM4_INSTRUCTIONS_AVAILABLE: u32 = 56;
pub const PF_ARM_V81_ATOMIC_INSTRUCTIONS_AVAILABLE: u32 = 34;
pub const PF_ARM_V82_DP_INSTRUCTIONS_AVAILABLE: u32 = 43;
pub const PF_ARM_V82_FP16_INSTRUCTIONS_AVAILABLE: u32 = 67;
pub const PF_ARM_V82_I8MM_INSTRUCTIONS_AVAILABLE: u32 = 66;
pub const PF_ARM_V83_JSCVT_INSTRUCTIONS_AVAILABLE: u32 = 44;
pub const PF_ARM_V83_LRCPC_INSTRUCTIONS_AVAILABLE: u32 = 45;
pub const PF_ARM_V86_BF16_INSTRUCTIONS_AVAILABLE: u32 = 68;
pub const PF_ARM_V86_EBF16_INSTRUCTIONS_AVAILABLE: u32 = 69;
pub const PF_ARM_V8_CRC32_INSTRUCTIONS_AVAILABLE: u32 = 31;
pub const PF_ARM_V8_CRYPTO_INSTRUCTIONS_AVAILABLE: u32 = 30;
pub const PF_ARM_V8_INSTRUCTIONS_AVAILABLE: u32 = 29;
pub const PF_ARM_VFP_32_REGISTERS_AVAILABLE: u32 = 18;
pub const PF_AVX2_INSTRUCTIONS_AVAILABLE: u32 = 40;
pub const PF_AVX512F_INSTRUCTIONS_AVAILABLE: u32 = 41;
pub const PF_AVX_INSTRUCTIONS_AVAILABLE: u32 = 39;
pub const PF_BMI2_INSTRUCTIONS_AVAILABLE: u32 = 60;
pub const PF_CHANNELS_ENABLED: u32 = 16;
pub const PF_COMPARE64_EXCHANGE128: u32 = 15;
pub const PF_COMPARE_EXCHANGE128: u32 = 14;
pub const PF_COMPARE_EXCHANGE_DOUBLE: u32 = 2;
pub const PF_ERMS_AVAILABLE: u32 = 42;
pub const PF_FASTFAIL_AVAILABLE: u32 = 23;
pub const PF_FLOATING_POINT_EMULATED: u32 = 1;
pub const PF_FLOATING_POINT_PRECISION_ERRATA: u32 = 0;
pub const PF_MMX_INSTRUCTIONS_AVAILABLE: u32 = 3;
pub const PF_MONITORX_INSTRUCTION_AVAILABLE: u32 = 35;
pub const PF_MOVDIR64B_INSTRUCTION_AVAILABLE: u32 = 61;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
pub const PF_NON_TEMPORAL_LEVEL_ALL: u32 = 0;
#[cfg(target_arch = "aarch64")]
pub const PF_NON_TEMPORAL_LEVEL_ALL: u32 = 3;
pub const PF_NX_ENABLED: u32 = 12;
pub const PF_PAE_ENABLED: u32 = 9;
pub const PF_PPC_MOVEMEM_64BIT_OK: u32 = 4;
pub const PF_RDPID_INSTRUCTION_AVAILABLE: u32 = 33;
pub const PF_RDRAND_INSTRUCTION_AVAILABLE: u32 = 28;
pub const PF_RDTSCP_INSTRUCTION_AVAILABLE: u32 = 32;
pub const PF_RDTSC_INSTRUCTION_AVAILABLE: u32 = 8;
pub const PF_RDWRFSGSBASE_AVAILABLE: u32 = 22;
pub const PF_RESERVED_FEATURE: u32 = 63;
pub const PF_SECOND_LEVEL_ADDRESS_TRANSLATION: u32 = 20;
pub const PF_SSE3_INSTRUCTIONS_AVAILABLE: u32 = 13;
pub const PF_SSE4_1_INSTRUCTIONS_AVAILABLE: u32 = 37;
pub const PF_SSE4_2_INSTRUCTIONS_AVAILABLE: u32 = 38;
pub const PF_SSE_DAZ_MODE_AVAILABLE: u32 = 11;
pub const PF_SSSE3_INSTRUCTIONS_AVAILABLE: u32 = 36;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
pub const PF_TEMPORAL_LEVEL_1: u32 = 1;
#[cfg(target_arch = "aarch64")]
pub const PF_TEMPORAL_LEVEL_1: u32 = 0;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
pub const PF_TEMPORAL_LEVEL_2: u32 = 2;
#[cfg(target_arch = "aarch64")]
pub const PF_TEMPORAL_LEVEL_2: u32 = 1;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
pub const PF_TEMPORAL_LEVEL_3: u32 = 3;
#[cfg(target_arch = "aarch64")]
pub const PF_TEMPORAL_LEVEL_3: u32 = 2;
pub const PF_UMONITOR_INSTRUCTION_AVAILABLE: u32 = 89;
pub const PF_VIRT_FIRMWARE_ENABLED: u32 = 21;
pub const PF_XMMI64_INSTRUCTIONS_AVAILABLE: u32 = 10;
pub const PF_XMMI_INSTRUCTIONS_AVAILABLE: u32 = 6;
pub const PF_XSAVE_ENABLED: u32 = 17;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PGENERIC_MAPPING(pub *mut GENERIC_MAPPING);
impl PGENERIC_MAPPING {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PGENERIC_MAPPING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PGET_RUNTIME_FUNCTION_CALLBACK(pub *mut GET_RUNTIME_FUNCTION_CALLBACK);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl PGET_RUNTIME_FUNCTION_CALLBACK {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for PGET_RUNTIME_FUNCTION_CALLBACK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "basetsd")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PGROUP_AFFINITY(pub *mut GROUP_AFFINITY);
#[cfg(feature = "basetsd")]
impl PGROUP_AFFINITY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "basetsd")]
impl Default for PGROUP_AFFINITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PGROUP_AFFINITY32(pub *mut GROUP_AFFINITY32);
impl PGROUP_AFFINITY32 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PGROUP_AFFINITY32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PGROUP_AFFINITY64(pub *mut GROUP_AFFINITY64);
impl PGROUP_AFFINITY64 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PGROUP_AFFINITY64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "basetsd")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PGROUP_RELATIONSHIP(pub *mut GROUP_RELATIONSHIP);
#[cfg(feature = "basetsd")]
impl PGROUP_RELATIONSHIP {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "basetsd")]
impl Default for PGROUP_RELATIONSHIP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PHANDLE(pub *mut HANDLE);
impl PHANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PHANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PHARDWARE_COUNTER_DATA(pub *mut HARDWARE_COUNTER_DATA);
impl PHARDWARE_COUNTER_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PHARDWARE_COUNTER_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PHARDWARE_COUNTER_TYPE(pub *mut HARDWARE_COUNTER_TYPE);
impl PHARDWARE_COUNTER_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PHARDWARE_COUNTER_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PHEAP_MEMORY_USAGE_ENTRY(pub *mut HEAP_MEMORY_USAGE_ENTRY);
impl PHEAP_MEMORY_USAGE_ENTRY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PHEAP_MEMORY_USAGE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PHEAP_MEMORY_USAGE_INFORMATION(pub *mut HEAP_MEMORY_USAGE_INFORMATION);
impl PHEAP_MEMORY_USAGE_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PHEAP_MEMORY_USAGE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PHEAP_OPTIMIZE_RESOURCES_INFORMATION(pub *mut HEAP_OPTIMIZE_RESOURCES_INFORMATION);
impl PHEAP_OPTIMIZE_RESOURCES_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PHEAP_OPTIMIZE_RESOURCES_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PHIBERFILE_BUCKET(pub *mut HIBERFILE_BUCKET);
impl PHIBERFILE_BUCKET {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PHIBERFILE_BUCKET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PHIBERFILE_BUCKET_SIZE(pub *mut HIBERFILE_BUCKET_SIZE);
impl PHIBERFILE_BUCKET_SIZE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PHIBERFILE_BUCKET_SIZE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_ALPHA64_RUNTIME_FUNCTION_ENTRY(pub *mut IMAGE_ALPHA64_RUNTIME_FUNCTION_ENTRY);
impl PIMAGE_ALPHA64_RUNTIME_FUNCTION_ENTRY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_ALPHA64_RUNTIME_FUNCTION_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_ALPHA_RUNTIME_FUNCTION_ENTRY(pub *mut IMAGE_ALPHA_RUNTIME_FUNCTION_ENTRY);
impl PIMAGE_ALPHA_RUNTIME_FUNCTION_ENTRY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_ALPHA_RUNTIME_FUNCTION_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PIMAGE_AMD64_RUNTIME_FUNCTION_ENTRY(pub _PIMAGE_RUNTIME_FUNCTION_ENTRY);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_ARCHITECTURE_ENTRY(pub *mut IMAGE_ARCHITECTURE_ENTRY);
impl PIMAGE_ARCHITECTURE_ENTRY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_ARCHITECTURE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_ARCHITECTURE_HEADER(pub *mut IMAGE_ARCHITECTURE_HEADER);
impl PIMAGE_ARCHITECTURE_HEADER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_ARCHITECTURE_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_ARCHIVE_MEMBER_HEADER(pub *mut IMAGE_ARCHIVE_MEMBER_HEADER);
impl PIMAGE_ARCHIVE_MEMBER_HEADER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_ARCHIVE_MEMBER_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_ARM64_RUNTIME_FUNCTION_ENTRY(pub *mut ARM64_RUNTIME_FUNCTION);
impl PIMAGE_ARM64_RUNTIME_FUNCTION_ENTRY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_ARM64_RUNTIME_FUNCTION_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_ARM_RUNTIME_FUNCTION_ENTRY(pub *mut IMAGE_ARM_RUNTIME_FUNCTION_ENTRY);
impl PIMAGE_ARM_RUNTIME_FUNCTION_ENTRY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_ARM_RUNTIME_FUNCTION_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_AUX_SYMBOL(pub *mut IMAGE_AUX_SYMBOL);
impl PIMAGE_AUX_SYMBOL {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_AUX_SYMBOL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_AUX_SYMBOL_EX(pub *mut IMAGE_AUX_SYMBOL_EX);
impl PIMAGE_AUX_SYMBOL_EX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_AUX_SYMBOL_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_AUX_SYMBOL_TOKEN_DEF(pub *mut IMAGE_AUX_SYMBOL_TOKEN_DEF);
impl PIMAGE_AUX_SYMBOL_TOKEN_DEF {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_AUX_SYMBOL_TOKEN_DEF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_BASE_RELOCATION(pub *mut IMAGE_BASE_RELOCATION);
impl PIMAGE_BASE_RELOCATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_BASE_RELOCATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_BDD_DYNAMIC_RELOCATION(pub *mut IMAGE_BDD_DYNAMIC_RELOCATION);
impl PIMAGE_BDD_DYNAMIC_RELOCATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_BDD_DYNAMIC_RELOCATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_BDD_INFO(pub *mut IMAGE_BDD_INFO);
impl PIMAGE_BDD_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_BDD_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_BOUND_FORWARDER_REF(pub *mut IMAGE_BOUND_FORWARDER_REF);
impl PIMAGE_BOUND_FORWARDER_REF {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_BOUND_FORWARDER_REF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_BOUND_IMPORT_DESCRIPTOR(pub *mut IMAGE_BOUND_IMPORT_DESCRIPTOR);
impl PIMAGE_BOUND_IMPORT_DESCRIPTOR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_BOUND_IMPORT_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_CE_RUNTIME_FUNCTION_ENTRY(pub *mut IMAGE_CE_RUNTIME_FUNCTION_ENTRY);
impl PIMAGE_CE_RUNTIME_FUNCTION_ENTRY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_CE_RUNTIME_FUNCTION_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_COFF_SYMBOLS_HEADER(pub *mut IMAGE_COFF_SYMBOLS_HEADER);
impl PIMAGE_COFF_SYMBOLS_HEADER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_COFF_SYMBOLS_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_COR20_HEADER(pub *mut IMAGE_COR20_HEADER);
impl PIMAGE_COR20_HEADER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_COR20_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_DATA_DIRECTORY(pub *mut IMAGE_DATA_DIRECTORY);
impl PIMAGE_DATA_DIRECTORY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_DATA_DIRECTORY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_DEBUG_DIRECTORY(pub *mut IMAGE_DEBUG_DIRECTORY);
impl PIMAGE_DEBUG_DIRECTORY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_DEBUG_DIRECTORY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_DEBUG_MISC(pub *mut IMAGE_DEBUG_MISC);
impl PIMAGE_DEBUG_MISC {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_DEBUG_MISC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_DELAYLOAD_DESCRIPTOR(pub *mut IMAGE_DELAYLOAD_DESCRIPTOR);
impl PIMAGE_DELAYLOAD_DESCRIPTOR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_DELAYLOAD_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_DOS_HEADER(pub *mut IMAGE_DOS_HEADER);
impl PIMAGE_DOS_HEADER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_DOS_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PIMAGE_DYNAMIC_RELOCATION(pub PIMAGE_DYNAMIC_RELOCATION32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PIMAGE_DYNAMIC_RELOCATION(pub PIMAGE_DYNAMIC_RELOCATION64);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_DYNAMIC_RELOCATION32(pub *mut IMAGE_DYNAMIC_RELOCATION32);
impl PIMAGE_DYNAMIC_RELOCATION32 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_DYNAMIC_RELOCATION32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_DYNAMIC_RELOCATION32_V2(pub *mut IMAGE_DYNAMIC_RELOCATION32_V2);
impl PIMAGE_DYNAMIC_RELOCATION32_V2 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_DYNAMIC_RELOCATION32_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_DYNAMIC_RELOCATION64(pub *mut IMAGE_DYNAMIC_RELOCATION64);
impl PIMAGE_DYNAMIC_RELOCATION64 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_DYNAMIC_RELOCATION64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_DYNAMIC_RELOCATION64_V2(pub *mut IMAGE_DYNAMIC_RELOCATION64_V2);
impl PIMAGE_DYNAMIC_RELOCATION64_V2 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_DYNAMIC_RELOCATION64_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_DYNAMIC_RELOCATION_TABLE(pub *mut IMAGE_DYNAMIC_RELOCATION_TABLE);
impl PIMAGE_DYNAMIC_RELOCATION_TABLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_DYNAMIC_RELOCATION_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PIMAGE_DYNAMIC_RELOCATION_V2(pub PIMAGE_DYNAMIC_RELOCATION32_V2);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PIMAGE_DYNAMIC_RELOCATION_V2(pub PIMAGE_DYNAMIC_RELOCATION64_V2);
#[cfg(target_arch = "x86")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PIMAGE_ENCLAVE_CONFIG(pub PIMAGE_ENCLAVE_CONFIG32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PIMAGE_ENCLAVE_CONFIG(pub PIMAGE_ENCLAVE_CONFIG64);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_ENCLAVE_CONFIG32(pub *mut IMAGE_ENCLAVE_CONFIG32);
impl PIMAGE_ENCLAVE_CONFIG32 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_ENCLAVE_CONFIG32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_ENCLAVE_CONFIG64(pub *mut IMAGE_ENCLAVE_CONFIG64);
impl PIMAGE_ENCLAVE_CONFIG64 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_ENCLAVE_CONFIG64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_ENCLAVE_IMPORT(pub *mut IMAGE_ENCLAVE_IMPORT);
impl PIMAGE_ENCLAVE_IMPORT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_ENCLAVE_IMPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_EPILOGUE_DYNAMIC_RELOCATION_HEADER(pub *mut IMAGE_EPILOGUE_DYNAMIC_RELOCATION_HEADER);
impl PIMAGE_EPILOGUE_DYNAMIC_RELOCATION_HEADER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_EPILOGUE_DYNAMIC_RELOCATION_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_EXPORT_DIRECTORY(pub *mut IMAGE_EXPORT_DIRECTORY);
impl PIMAGE_EXPORT_DIRECTORY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_EXPORT_DIRECTORY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_FILE_HEADER(pub *mut IMAGE_FILE_HEADER);
impl PIMAGE_FILE_HEADER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_FILE_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_FUNCTION_ENTRY(pub *mut IMAGE_FUNCTION_ENTRY);
impl PIMAGE_FUNCTION_ENTRY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_FUNCTION_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_FUNCTION_ENTRY64(pub *mut IMAGE_FUNCTION_ENTRY64);
impl PIMAGE_FUNCTION_ENTRY64 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_FUNCTION_ENTRY64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_FUNCTION_OVERRIDE_DYNAMIC_RELOCATION(pub *mut IMAGE_FUNCTION_OVERRIDE_DYNAMIC_RELOCATION);
impl PIMAGE_FUNCTION_OVERRIDE_DYNAMIC_RELOCATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_FUNCTION_OVERRIDE_DYNAMIC_RELOCATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_FUNCTION_OVERRIDE_HEADER(pub *mut IMAGE_FUNCTION_OVERRIDE_HEADER);
impl PIMAGE_FUNCTION_OVERRIDE_HEADER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_FUNCTION_OVERRIDE_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_HOTSWAP_ARM64_ENDPOINT_INFO_CC_RETURN(pub *mut IMAGE_HOTSWAP_ARM64_ENDPOINT_INFO_CC_RETURN);
impl PIMAGE_HOTSWAP_ARM64_ENDPOINT_INFO_CC_RETURN {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_HOTSWAP_ARM64_ENDPOINT_INFO_CC_RETURN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_HOTSWAP_ARM64_ENDPOINT_INFO_ENTRY_V2(pub *mut IMAGE_HOTSWAP_ARM64_ENDPOINT_INFO_ENTRY_V2);
impl PIMAGE_HOTSWAP_ARM64_ENDPOINT_INFO_ENTRY_V2 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_HOTSWAP_ARM64_ENDPOINT_INFO_ENTRY_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_HOTSWAP_ENDPOINT_INFO_ENTRY_COMMON(pub *mut IMAGE_HOTSWAP_ENDPOINT_INFO_ENTRY_COMMON);
impl PIMAGE_HOTSWAP_ENDPOINT_INFO_ENTRY_COMMON {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_HOTSWAP_ENDPOINT_INFO_ENTRY_COMMON {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_HOTSWAP_ENDPOINT_INFO_HEADER_COMMON(pub *mut IMAGE_HOTSWAP_ENDPOINT_INFO_HEADER_COMMON);
impl PIMAGE_HOTSWAP_ENDPOINT_INFO_HEADER_COMMON {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_HOTSWAP_ENDPOINT_INFO_HEADER_COMMON {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_HOTSWAP_ENDPOINT_INFO_HEADER_V2(pub *mut IMAGE_HOTSWAP_ENDPOINT_INFO_HEADER_V2);
impl PIMAGE_HOTSWAP_ENDPOINT_INFO_HEADER_V2 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_HOTSWAP_ENDPOINT_INFO_HEADER_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_HOTSWAP_X64_ENDPOINT_INFO_CC_REG(pub *mut IMAGE_HOTSWAP_X64_ENDPOINT_INFO_CC_REG);
impl PIMAGE_HOTSWAP_X64_ENDPOINT_INFO_CC_REG {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_HOTSWAP_X64_ENDPOINT_INFO_CC_REG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_HOTSWAP_X64_ENDPOINT_INFO_ENTRY_V2(pub *mut IMAGE_HOTSWAP_X64_ENDPOINT_INFO_ENTRY_V2);
impl PIMAGE_HOTSWAP_X64_ENDPOINT_INFO_ENTRY_V2 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_HOTSWAP_X64_ENDPOINT_INFO_ENTRY_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_HOT_PATCH_BASE(pub *mut IMAGE_HOT_PATCH_BASE);
impl PIMAGE_HOT_PATCH_BASE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_HOT_PATCH_BASE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_HOT_PATCH_HASHES(pub *mut IMAGE_HOT_PATCH_HASHES);
impl PIMAGE_HOT_PATCH_HASHES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_HOT_PATCH_HASHES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_HOT_PATCH_INFO(pub *mut IMAGE_HOT_PATCH_INFO);
impl PIMAGE_HOT_PATCH_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_HOT_PATCH_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_HOT_PATCH_MACHINE(pub *mut IMAGE_HOT_PATCH_MACHINE);
impl PIMAGE_HOT_PATCH_MACHINE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_HOT_PATCH_MACHINE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PIMAGE_IA64_RUNTIME_FUNCTION_ENTRY(pub _PIMAGE_RUNTIME_FUNCTION_ENTRY);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_IMPORT_BY_NAME(pub *mut IMAGE_IMPORT_BY_NAME);
impl PIMAGE_IMPORT_BY_NAME {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_IMPORT_BY_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_IMPORT_CONTROL_TRANSFER_ARM64_RELOCATION(pub *mut IMAGE_IMPORT_CONTROL_TRANSFER_ARM64_RELOCATION);
impl PIMAGE_IMPORT_CONTROL_TRANSFER_ARM64_RELOCATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_IMPORT_CONTROL_TRANSFER_ARM64_RELOCATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_IMPORT_CONTROL_TRANSFER_DYNAMIC_RELOCATION(pub *mut IMAGE_IMPORT_CONTROL_TRANSFER_DYNAMIC_RELOCATION);
impl PIMAGE_IMPORT_CONTROL_TRANSFER_DYNAMIC_RELOCATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_IMPORT_CONTROL_TRANSFER_DYNAMIC_RELOCATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_IMPORT_CONTROL_TRANSFER_RELOCATION(pub *mut IMAGE_IMPORT_CONTROL_TRANSFER_DYNAMIC_RELOCATION);
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl PIMAGE_IMPORT_CONTROL_TRANSFER_RELOCATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for PIMAGE_IMPORT_CONTROL_TRANSFER_RELOCATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_IMPORT_CONTROL_TRANSFER_RELOCATION(pub *mut IMAGE_IMPORT_CONTROL_TRANSFER_ARM64_RELOCATION);
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
impl PIMAGE_IMPORT_CONTROL_TRANSFER_RELOCATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
impl Default for PIMAGE_IMPORT_CONTROL_TRANSFER_RELOCATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_IMPORT_DESCRIPTOR(pub *mut IMAGE_IMPORT_DESCRIPTOR);
impl PIMAGE_IMPORT_DESCRIPTOR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_IMPORT_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_INDIR_CONTROL_TRANSFER_DYNAMIC_RELOCATION(pub *mut IMAGE_INDIR_CONTROL_TRANSFER_DYNAMIC_RELOCATION);
impl PIMAGE_INDIR_CONTROL_TRANSFER_DYNAMIC_RELOCATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_INDIR_CONTROL_TRANSFER_DYNAMIC_RELOCATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_LINENUMBER(pub *mut IMAGE_LINENUMBER);
impl PIMAGE_LINENUMBER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_LINENUMBER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_LOAD_CONFIG_CODE_INTEGRITY(pub *mut IMAGE_LOAD_CONFIG_CODE_INTEGRITY);
impl PIMAGE_LOAD_CONFIG_CODE_INTEGRITY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_LOAD_CONFIG_CODE_INTEGRITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PIMAGE_LOAD_CONFIG_DIRECTORY(pub PIMAGE_LOAD_CONFIG_DIRECTORY32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PIMAGE_LOAD_CONFIG_DIRECTORY(pub PIMAGE_LOAD_CONFIG_DIRECTORY64);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_LOAD_CONFIG_DIRECTORY32(pub *mut IMAGE_LOAD_CONFIG_DIRECTORY32);
impl PIMAGE_LOAD_CONFIG_DIRECTORY32 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_LOAD_CONFIG_DIRECTORY32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_LOAD_CONFIG_DIRECTORY64(pub *mut IMAGE_LOAD_CONFIG_DIRECTORY64);
impl PIMAGE_LOAD_CONFIG_DIRECTORY64 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_LOAD_CONFIG_DIRECTORY64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PIMAGE_NT_HEADERS(pub PIMAGE_NT_HEADERS32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PIMAGE_NT_HEADERS(pub PIMAGE_NT_HEADERS64);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_NT_HEADERS32(pub *mut IMAGE_NT_HEADERS32);
impl PIMAGE_NT_HEADERS32 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_NT_HEADERS32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_NT_HEADERS64(pub *mut IMAGE_NT_HEADERS64);
impl PIMAGE_NT_HEADERS64 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_NT_HEADERS64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PIMAGE_OPTIONAL_HEADER(pub PIMAGE_OPTIONAL_HEADER32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PIMAGE_OPTIONAL_HEADER(pub PIMAGE_OPTIONAL_HEADER64);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_OPTIONAL_HEADER32(pub *mut IMAGE_OPTIONAL_HEADER32);
impl PIMAGE_OPTIONAL_HEADER32 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_OPTIONAL_HEADER32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_OPTIONAL_HEADER64(pub *mut IMAGE_OPTIONAL_HEADER64);
impl PIMAGE_OPTIONAL_HEADER64 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_OPTIONAL_HEADER64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_OS2_HEADER(pub *mut IMAGE_OS2_HEADER);
impl PIMAGE_OS2_HEADER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_OS2_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_PROLOGUE_DYNAMIC_RELOCATION_HEADER(pub *mut IMAGE_PROLOGUE_DYNAMIC_RELOCATION_HEADER);
impl PIMAGE_PROLOGUE_DYNAMIC_RELOCATION_HEADER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_PROLOGUE_DYNAMIC_RELOCATION_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_RELOCATION(pub *mut IMAGE_RELOCATION);
impl PIMAGE_RELOCATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_RELOCATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_RESOURCE_DATA_ENTRY(pub *mut IMAGE_RESOURCE_DATA_ENTRY);
impl PIMAGE_RESOURCE_DATA_ENTRY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_RESOURCE_DATA_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_RESOURCE_DIRECTORY(pub *mut IMAGE_RESOURCE_DIRECTORY);
impl PIMAGE_RESOURCE_DIRECTORY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_RESOURCE_DIRECTORY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_RESOURCE_DIRECTORY_ENTRY(pub *mut IMAGE_RESOURCE_DIRECTORY_ENTRY);
impl PIMAGE_RESOURCE_DIRECTORY_ENTRY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_RESOURCE_DIRECTORY_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_RESOURCE_DIRECTORY_STRING(pub *mut IMAGE_RESOURCE_DIRECTORY_STRING);
impl PIMAGE_RESOURCE_DIRECTORY_STRING {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_RESOURCE_DIRECTORY_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_RESOURCE_DIR_STRING_U(pub *mut IMAGE_RESOURCE_DIR_STRING_U);
impl PIMAGE_RESOURCE_DIR_STRING_U {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_RESOURCE_DIR_STRING_U {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_ROM_HEADERS(pub *mut IMAGE_ROM_HEADERS);
impl PIMAGE_ROM_HEADERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_ROM_HEADERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_ROM_OPTIONAL_HEADER(pub *mut IMAGE_ROM_OPTIONAL_HEADER);
impl PIMAGE_ROM_OPTIONAL_HEADER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_ROM_OPTIONAL_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PIMAGE_RUNTIME_FUNCTION_ENTRY(pub _PIMAGE_RUNTIME_FUNCTION_ENTRY);
#[cfg(target_arch = "aarch64")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PIMAGE_RUNTIME_FUNCTION_ENTRY(pub PIMAGE_ARM64_RUNTIME_FUNCTION_ENTRY);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_SECTION_HEADER(pub *mut IMAGE_SECTION_HEADER);
impl PIMAGE_SECTION_HEADER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_SECTION_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_SEPARATE_DEBUG_HEADER(pub *mut IMAGE_SEPARATE_DEBUG_HEADER);
impl PIMAGE_SEPARATE_DEBUG_HEADER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_SEPARATE_DEBUG_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_SWITCHTABLE_BRANCH_DYNAMIC_RELOCATION(pub *mut IMAGE_SWITCHTABLE_BRANCH_DYNAMIC_RELOCATION);
impl PIMAGE_SWITCHTABLE_BRANCH_DYNAMIC_RELOCATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_SWITCHTABLE_BRANCH_DYNAMIC_RELOCATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_SYMBOL(pub *mut IMAGE_SYMBOL);
impl PIMAGE_SYMBOL {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_SYMBOL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_SYMBOL_EX(pub *mut IMAGE_SYMBOL_EX);
impl PIMAGE_SYMBOL_EX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_SYMBOL_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PIMAGE_THUNK_DATA(pub PIMAGE_THUNK_DATA32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PIMAGE_THUNK_DATA(pub PIMAGE_THUNK_DATA64);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_THUNK_DATA32(pub *mut IMAGE_THUNK_DATA32);
impl PIMAGE_THUNK_DATA32 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_THUNK_DATA32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_THUNK_DATA64(pub *mut IMAGE_THUNK_DATA64);
impl PIMAGE_THUNK_DATA64 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_THUNK_DATA64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PIMAGE_TLS_CALLBACK = Option<unsafe extern "system" fn(dllhandle: *mut core::ffi::c_void, reason: u32, reserved: *mut core::ffi::c_void)>;
#[cfg(target_arch = "x86")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PIMAGE_TLS_DIRECTORY(pub PIMAGE_TLS_DIRECTORY32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PIMAGE_TLS_DIRECTORY(pub PIMAGE_TLS_DIRECTORY64);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_TLS_DIRECTORY32(pub *mut IMAGE_TLS_DIRECTORY32);
impl PIMAGE_TLS_DIRECTORY32 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_TLS_DIRECTORY32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_TLS_DIRECTORY64(pub *mut IMAGE_TLS_DIRECTORY64);
impl PIMAGE_TLS_DIRECTORY64 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_TLS_DIRECTORY64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIMAGE_VXD_HEADER(pub *mut IMAGE_VXD_HEADER);
impl PIMAGE_VXD_HEADER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIMAGE_VXD_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PIO_COUNTERS(pub *mut IO_COUNTERS);
impl PIO_COUNTERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PIO_COUNTERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PISECURITY_DESCRIPTOR(pub *mut SECURITY_DESCRIPTOR);
impl PISECURITY_DESCRIPTOR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PISECURITY_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PISECURITY_DESCRIPTOR_RELATIVE(pub *mut SECURITY_DESCRIPTOR_RELATIVE);
impl PISECURITY_DESCRIPTOR_RELATIVE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PISECURITY_DESCRIPTOR_RELATIVE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PISID(pub *mut SID);
impl PISID {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PISID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PJOBOBJECT_ASSOCIATE_COMPLETION_PORT(pub *mut JOBOBJECT_ASSOCIATE_COMPLETION_PORT);
impl PJOBOBJECT_ASSOCIATE_COMPLETION_PORT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PJOBOBJECT_ASSOCIATE_COMPLETION_PORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PJOBOBJECT_BASIC_ACCOUNTING_INFORMATION(pub *mut JOBOBJECT_BASIC_ACCOUNTING_INFORMATION);
impl PJOBOBJECT_BASIC_ACCOUNTING_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PJOBOBJECT_BASIC_ACCOUNTING_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PJOBOBJECT_BASIC_AND_IO_ACCOUNTING_INFORMATION(pub *mut JOBOBJECT_BASIC_AND_IO_ACCOUNTING_INFORMATION);
impl PJOBOBJECT_BASIC_AND_IO_ACCOUNTING_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PJOBOBJECT_BASIC_AND_IO_ACCOUNTING_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PJOBOBJECT_BASIC_LIMIT_INFORMATION(pub *mut JOBOBJECT_BASIC_LIMIT_INFORMATION);
impl PJOBOBJECT_BASIC_LIMIT_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PJOBOBJECT_BASIC_LIMIT_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PJOBOBJECT_BASIC_PROCESS_ID_LIST(pub *mut JOBOBJECT_BASIC_PROCESS_ID_LIST);
impl PJOBOBJECT_BASIC_PROCESS_ID_LIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PJOBOBJECT_BASIC_PROCESS_ID_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PJOBOBJECT_BASIC_UI_RESTRICTIONS(pub *mut JOBOBJECT_BASIC_UI_RESTRICTIONS);
impl PJOBOBJECT_BASIC_UI_RESTRICTIONS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PJOBOBJECT_BASIC_UI_RESTRICTIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PJOBOBJECT_CPU_RATE_CONTROL_INFORMATION(pub *mut JOBOBJECT_CPU_RATE_CONTROL_INFORMATION);
impl PJOBOBJECT_CPU_RATE_CONTROL_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PJOBOBJECT_CPU_RATE_CONTROL_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PJOBOBJECT_END_OF_JOB_TIME_INFORMATION(pub *mut JOBOBJECT_END_OF_JOB_TIME_INFORMATION);
impl PJOBOBJECT_END_OF_JOB_TIME_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PJOBOBJECT_END_OF_JOB_TIME_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PJOBOBJECT_EXTENDED_LIMIT_INFORMATION(pub *mut JOBOBJECT_EXTENDED_LIMIT_INFORMATION);
impl PJOBOBJECT_EXTENDED_LIMIT_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PJOBOBJECT_EXTENDED_LIMIT_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PJOBOBJECT_IO_ATTRIBUTION_INFORMATION(pub *mut JOBOBJECT_IO_ATTRIBUTION_INFORMATION);
impl PJOBOBJECT_IO_ATTRIBUTION_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PJOBOBJECT_IO_ATTRIBUTION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PJOBOBJECT_IO_ATTRIBUTION_STATS(pub *mut JOBOBJECT_IO_ATTRIBUTION_STATS);
impl PJOBOBJECT_IO_ATTRIBUTION_STATS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PJOBOBJECT_IO_ATTRIBUTION_STATS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PJOBOBJECT_JOBSET_INFORMATION(pub *mut JOBOBJECT_JOBSET_INFORMATION);
impl PJOBOBJECT_JOBSET_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PJOBOBJECT_JOBSET_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PJOBOBJECT_LIMIT_VIOLATION_INFORMATION(pub *mut JOBOBJECT_LIMIT_VIOLATION_INFORMATION);
impl PJOBOBJECT_LIMIT_VIOLATION_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PJOBOBJECT_LIMIT_VIOLATION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PJOBOBJECT_NOTIFICATION_LIMIT_INFORMATION(pub *mut JOBOBJECT_NOTIFICATION_LIMIT_INFORMATION);
impl PJOBOBJECT_NOTIFICATION_LIMIT_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PJOBOBJECT_NOTIFICATION_LIMIT_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PJOBOBJECT_RATE_CONTROL_TOLERANCE(pub *mut JOBOBJECT_RATE_CONTROL_TOLERANCE);
impl PJOBOBJECT_RATE_CONTROL_TOLERANCE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PJOBOBJECT_RATE_CONTROL_TOLERANCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PJOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL(pub *mut JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL);
impl PJOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PJOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PJOBOBJECT_SECURITY_LIMIT_INFORMATION(pub *mut JOBOBJECT_SECURITY_LIMIT_INFORMATION);
impl PJOBOBJECT_SECURITY_LIMIT_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PJOBOBJECT_SECURITY_LIMIT_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PJOB_SET_ARRAY(pub *mut JOB_SET_ARRAY);
impl PJOB_SET_ARRAY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PJOB_SET_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PKERNEL_CET_CONTEXT(pub *mut KERNEL_CET_CONTEXT);
impl PKERNEL_CET_CONTEXT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PKERNEL_CET_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[cfg(feature = "basetsd")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PKNONVOLATILE_CONTEXT_POINTERS(pub *mut KNONVOLATILE_CONTEXT_POINTERS);
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[cfg(feature = "basetsd")]
impl PKNONVOLATILE_CONTEXT_POINTERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[cfg(feature = "basetsd")]
impl Default for PKNONVOLATILE_CONTEXT_POINTERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "basetsd")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PKNONVOLATILE_CONTEXT_POINTERS(pub *mut KNONVOLATILE_CONTEXT_POINTERS_ARM64);
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "basetsd")]
impl PKNONVOLATILE_CONTEXT_POINTERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "basetsd")]
impl Default for PKNONVOLATILE_CONTEXT_POINTERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "basetsd")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PKNONVOLATILE_CONTEXT_POINTERS_ARM64(pub *mut KNONVOLATILE_CONTEXT_POINTERS_ARM64);
#[cfg(feature = "basetsd")]
impl PKNONVOLATILE_CONTEXT_POINTERS_ARM64 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "basetsd")]
impl Default for PKNONVOLATILE_CONTEXT_POINTERS_ARM64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PKSPIN_LOCK(pub *mut KSPIN_LOCK);
impl PKSPIN_LOCK {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PKSPIN_LOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PKTMOBJECT_CURSOR(pub *mut KTMOBJECT_CURSOR);
impl PKTMOBJECT_CURSOR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PKTMOBJECT_CURSOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PKTMOBJECT_TYPE(pub *mut KTMOBJECT_TYPE);
impl PKTMOBJECT_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PKTMOBJECT_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PLARGE_INTEGER(pub *mut i64);
impl PLARGE_INTEGER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PLARGE_INTEGER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PLCID(pub super::minwindef::PDWORD);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PLDT_ENTRY(pub *mut LDT_ENTRY);
impl PLDT_ENTRY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PLDT_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PLIST_ENTRY(pub *mut LIST_ENTRY);
impl PLIST_ENTRY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PLIST_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PLIST_ENTRY32(pub *mut LIST_ENTRY32);
impl PLIST_ENTRY32 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PLIST_ENTRY32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PLIST_ENTRY64(pub *mut LIST_ENTRY64);
impl PLIST_ENTRY64 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PLIST_ENTRY64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PLONG(pub *mut i32);
impl PLONG {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PLONG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PLONGLONG(pub *mut i64);
impl PLONGLONG {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PLONGLONG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PLUID(pub *mut LUID);
impl PLUID {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PLUID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PLUID_AND_ATTRIBUTES(pub *mut LUID_AND_ATTRIBUTES);
impl PLUID_AND_ATTRIBUTES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PLUID_AND_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PLUID_AND_ATTRIBUTES_ARRAY(pub *mut LUID_AND_ATTRIBUTES_ARRAY);
impl PLUID_AND_ATTRIBUTES_ARRAY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PLUID_AND_ATTRIBUTES_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PM128A(pub *mut M128A);
impl PM128A {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PM128A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMANDATORY_LEVEL(pub *mut MANDATORY_LEVEL);
impl PMANDATORY_LEVEL {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PMANDATORY_LEVEL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMAXVERSIONTESTED_INFO(pub *mut MAXVERSIONTESTED_INFO);
impl PMAXVERSIONTESTED_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PMAXVERSIONTESTED_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PMCCounter: HARDWARE_COUNTER_TYPE = 0;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMEMORY_BASIC_INFORMATION(pub *mut MEMORY_BASIC_INFORMATION);
impl PMEMORY_BASIC_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PMEMORY_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMEMORY_BASIC_INFORMATION32(pub *mut MEMORY_BASIC_INFORMATION32);
impl PMEMORY_BASIC_INFORMATION32 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PMEMORY_BASIC_INFORMATION32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMEMORY_BASIC_INFORMATION64(pub *mut MEMORY_BASIC_INFORMATION64);
impl PMEMORY_BASIC_INFORMATION64 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PMEMORY_BASIC_INFORMATION64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMEMORY_PARTITION_DEDICATED_MEMORY_ATTRIBUTE(pub *mut MEMORY_PARTITION_DEDICATED_MEMORY_ATTRIBUTE);
impl PMEMORY_PARTITION_DEDICATED_MEMORY_ATTRIBUTE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PMEMORY_PARTITION_DEDICATED_MEMORY_ATTRIBUTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMEMORY_PARTITION_DEDICATED_MEMORY_INFORMATION(pub *mut MEMORY_PARTITION_DEDICATED_MEMORY_INFORMATION);
impl PMEMORY_PARTITION_DEDICATED_MEMORY_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PMEMORY_PARTITION_DEDICATED_MEMORY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMEM_ADDRESS_REQUIREMENTS(pub *mut MEM_ADDRESS_REQUIREMENTS);
impl PMEM_ADDRESS_REQUIREMENTS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PMEM_ADDRESS_REQUIREMENTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMEM_DEDICATED_ATTRIBUTE_TYPE(pub *mut MEM_DEDICATED_ATTRIBUTE_TYPE);
impl PMEM_DEDICATED_ATTRIBUTE_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PMEM_DEDICATED_ATTRIBUTE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMEM_EXTENDED_PARAMETER(pub *mut MEM_EXTENDED_PARAMETER);
impl PMEM_EXTENDED_PARAMETER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PMEM_EXTENDED_PARAMETER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMEM_EXTENDED_PARAMETER_TYPE(pub *mut MEM_EXTENDED_PARAMETER_TYPE);
impl PMEM_EXTENDED_PARAMETER_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PMEM_EXTENDED_PARAMETER_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMEM_SECTION_EXTENDED_PARAMETER_TYPE(pub *mut MEM_SECTION_EXTENDED_PARAMETER_TYPE);
impl PMEM_SECTION_EXTENDED_PARAMETER_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PMEM_SECTION_EXTENDED_PARAMETER_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMESSAGE_RESOURCE_BLOCK(pub *mut MESSAGE_RESOURCE_BLOCK);
impl PMESSAGE_RESOURCE_BLOCK {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PMESSAGE_RESOURCE_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMESSAGE_RESOURCE_DATA(pub *mut MESSAGE_RESOURCE_DATA);
impl PMESSAGE_RESOURCE_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PMESSAGE_RESOURCE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMESSAGE_RESOURCE_ENTRY(pub *mut MESSAGE_RESOURCE_ENTRY);
impl PMESSAGE_RESOURCE_ENTRY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PMESSAGE_RESOURCE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMONITOR_DISPLAY_STATE(pub *mut MONITOR_DISPLAY_STATE);
impl PMONITOR_DISPLAY_STATE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PMONITOR_DISPLAY_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "aarch64")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PNEON128(pub *mut ARM64_NT_NEON128);
#[cfg(target_arch = "aarch64")]
impl PNEON128 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(target_arch = "aarch64")]
impl Default for PNEON128 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PNETWORK_APP_INSTANCE_EA(pub *mut NETWORK_APP_INSTANCE_EA);
impl PNETWORK_APP_INSTANCE_EA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PNETWORK_APP_INSTANCE_EA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PNON_PAGED_DEBUG_INFO(pub *mut NON_PAGED_DEBUG_INFO);
impl PNON_PAGED_DEBUG_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PNON_PAGED_DEBUG_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PNOTIFY_USER_POWER_SETTING(pub *mut NOTIFY_USER_POWER_SETTING);
impl PNOTIFY_USER_POWER_SETTING {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PNOTIFY_USER_POWER_SETTING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "excpt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PNT_TIB(pub *mut NT_TIB);
#[cfg(feature = "excpt")]
impl PNT_TIB {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "excpt")]
impl Default for PNT_TIB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PNT_TIB32(pub *mut NT_TIB32);
impl PNT_TIB32 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PNT_TIB32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PNT_TIB64(pub *mut NT_TIB64);
impl PNT_TIB64 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PNT_TIB64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "basetsd")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PNUMA_NODE_RELATIONSHIP(pub *mut NUMA_NODE_RELATIONSHIP);
#[cfg(feature = "basetsd")]
impl PNUMA_NODE_RELATIONSHIP {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "basetsd")]
impl Default for PNUMA_NODE_RELATIONSHIP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PNV_MEMORY_RANGE(pub *mut NV_MEMORY_RANGE);
impl PNV_MEMORY_RANGE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PNV_MEMORY_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PNZCH(pub *mut i8);
impl PNZCH {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PNZCH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PNZTCH(pub PNZCH);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PNZWCH(pub *mut u16);
impl PNZWCH {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PNZWCH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct POBJECT_TYPE_LIST(pub *mut OBJECT_TYPE_LIST);
impl POBJECT_TYPE_LIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for POBJECT_TYPE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const POLICY_AUDIT_SUBCATEGORY_COUNT: u32 = 60;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct POSVERSIONINFO(pub POSVERSIONINFOA);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct POSVERSIONINFOA(pub *mut OSVERSIONINFOA);
impl POSVERSIONINFOA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for POSVERSIONINFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct POSVERSIONINFOEX(pub POSVERSIONINFOEXA);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct POSVERSIONINFOEXA(pub *mut OSVERSIONINFOEXA);
impl POSVERSIONINFOEXA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for POSVERSIONINFOEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct POSVERSIONINFOEXW(pub *mut OSVERSIONINFOEXW);
impl POSVERSIONINFOEXW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for POSVERSIONINFOEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct POSVERSIONINFOW(pub *mut OSVERSIONINFOW);
impl POSVERSIONINFOW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for POSVERSIONINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct POUT_OF_PROCESS_FUNCTION_TABLE_CALLBACK(pub *mut OUT_OF_PROCESS_FUNCTION_TABLE_CALLBACK);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl POUT_OF_PROCESS_FUNCTION_TABLE_CALLBACK {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for POUT_OF_PROCESS_FUNCTION_TABLE_CALLBACK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const POWERBUTTON_ACTION_INDEX_HIBERNATE: u32 = 2;
pub const POWERBUTTON_ACTION_INDEX_NOTHING: u32 = 0;
pub const POWERBUTTON_ACTION_INDEX_SHUTDOWN: u32 = 3;
pub const POWERBUTTON_ACTION_INDEX_SLEEP: u32 = 1;
pub const POWERBUTTON_ACTION_INDEX_TURN_OFF_THE_DISPLAY: u32 = 4;
pub const POWERBUTTON_ACTION_VALUE_HIBERNATE: u32 = 3;
pub const POWERBUTTON_ACTION_VALUE_NOTHING: u32 = 0;
pub const POWERBUTTON_ACTION_VALUE_SHUTDOWN: u32 = 6;
pub const POWERBUTTON_ACTION_VALUE_SLEEP: u32 = 2;
pub const POWERBUTTON_ACTION_VALUE_TURN_OFF_THE_DISPLAY: u32 = 8;
pub type POWER_ACTION = i32;
pub const POWER_ACTION_ACPI_CRITICAL: u32 = 16777216;
pub const POWER_ACTION_ACPI_USER_NOTIFY: u32 = 33554432;
pub const POWER_ACTION_CRITICAL: u32 = 2147483648;
pub const POWER_ACTION_DIRECTED_DRIPS: u32 = 67108864;
pub const POWER_ACTION_DISABLE_WAKES: u32 = 1073741824;
pub const POWER_ACTION_DOZE_TO_HIBERNATE: u32 = 32;
pub const POWER_ACTION_HIBERBOOT: u32 = 8;
pub const POWER_ACTION_LIGHTEST_FIRST: u32 = 268435456;
pub const POWER_ACTION_LOCK_CONSOLE: u32 = 536870912;
pub const POWER_ACTION_OVERRIDE_APPS: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POWER_ACTION_POLICY {
    pub Action: POWER_ACTION,
    pub Flags: u32,
    pub EventCode: u32,
}
pub const POWER_ACTION_PSEUDO_TRANSITION: u32 = 134217728;
pub const POWER_ACTION_QUERY_ALLOWED: u32 = 1;
pub const POWER_ACTION_UI_ALLOWED: u32 = 2;
pub const POWER_ACTION_USER_NOTIFY: u32 = 16;
pub const POWER_CONNECTIVITY_IN_STANDBY_DISABLED: u32 = 0;
pub const POWER_CONNECTIVITY_IN_STANDBY_ENABLED: u32 = 1;
pub const POWER_CONNECTIVITY_IN_STANDBY_SYSTEM_MANAGED: u32 = 2;
pub const POWER_DEVICE_IDLE_POLICY_CONSERVATIVE: u32 = 1;
pub const POWER_DEVICE_IDLE_POLICY_PERFORMANCE: u32 = 0;
pub const POWER_DISCONNECTED_STANDBY_MODE_AGGRESSIVE: u32 = 1;
pub const POWER_DISCONNECTED_STANDBY_MODE_NORMAL: u32 = 0;
pub const POWER_FORCE_TRIGGER_RESET: u32 = 2147483648;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POWER_IDLE_RESILIENCY {
    pub CoalescingTimeout: u32,
    pub IdleResiliencyPeriod: u32,
}
pub type POWER_INFORMATION_LEVEL = i32;
pub const POWER_LEVEL_USER_NOTIFY_EXEC: u32 = 4;
pub const POWER_LEVEL_USER_NOTIFY_SOUND: u32 = 2;
pub const POWER_LEVEL_USER_NOTIFY_TEXT: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct POWER_LIMIT_ATTRIBUTES {
    pub Type: POWER_LIMIT_TYPES,
    pub DomainId: u32,
    pub MaxValue: u32,
    pub MinValue: u32,
    pub MinTimeParameter: u32,
    pub MaxTimeParameter: u32,
    pub DefaultACValue: u32,
    pub DefaultDCValue: u32,
    pub Flags: POWER_LIMIT_ATTRIBUTES_0,
}
impl Default for POWER_LIMIT_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union POWER_LIMIT_ATTRIBUTES_0 {
    pub Anonymous: POWER_LIMIT_ATTRIBUTES_0_0,
    pub AsUlong: u32,
}
impl Default for POWER_LIMIT_ATTRIBUTES_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POWER_LIMIT_ATTRIBUTES_0_0 {
    pub _bitfield: u32,
}
pub type POWER_LIMIT_TYPES = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POWER_LIMIT_VALUE {
    pub Type: POWER_LIMIT_TYPES,
    pub DomainId: u32,
    pub TargetValue: u32,
    pub TimeParameter: u32,
}
pub const POWER_LIMIT_VALUE_NO_CONTROL: i32 = -1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POWER_MONITOR_INVOCATION {
    pub Console: bool,
    pub RequestReason: POWER_MONITOR_REQUEST_REASON,
}
pub type POWER_MONITOR_REQUEST_REASON = i32;
pub type POWER_MONITOR_REQUEST_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POWER_PLATFORM_INFORMATION {
    pub AoAc: bool,
}
pub type POWER_PLATFORM_ROLE = i32;
pub const POWER_PLATFORM_ROLE_V1: u32 = 1;
pub const POWER_PLATFORM_ROLE_V1_MAX: u32 = 8;
pub const POWER_PLATFORM_ROLE_V2: u32 = 2;
pub const POWER_PLATFORM_ROLE_V2_MAX: u32 = 9;
pub const POWER_PLATFORM_ROLE_VERSION: u32 = 2;
pub const POWER_PLATFORM_ROLE_VERSION_MAX: u32 = 9;
pub const POWER_REQUEST_CONTEXT_DETAILED_STRING: u32 = 2;
pub const POWER_REQUEST_CONTEXT_SIMPLE_STRING: u32 = 1;
pub const POWER_REQUEST_CONTEXT_VERSION: u32 = 0;
pub type POWER_REQUEST_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POWER_SESSION_ALLOW_EXTERNAL_DMA_DEVICES {
    pub IsAllowed: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POWER_SESSION_CONNECT {
    pub Connected: bool,
    pub Console: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POWER_SESSION_RIT_STATE {
    pub Active: bool,
    pub LastInputTime: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POWER_SESSION_TIMEOUTS {
    pub InputTimeout: u32,
    pub DisplayTimeout: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POWER_SESSION_WINLOGON {
    pub SessionId: u32,
    pub Console: bool,
    pub Locked: bool,
}
pub type POWER_SETTING_ALTITUDE = i32;
pub const POWER_SETTING_VALUE_VERSION: u32 = 1;
pub const POWER_SYSTEM_MAXIMUM: u32 = 7;
pub const POWER_USER_NOTIFY_BUTTON: u32 = 8;
pub const POWER_USER_NOTIFY_FORCED_SHUTDOWN: u32 = 32;
pub const POWER_USER_NOTIFY_SHUTDOWN: u32 = 16;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POWER_USER_PRESENCE {
    pub UserPresence: POWER_USER_PRESENCE_TYPE,
}
pub type POWER_USER_PRESENCE_TYPE = i32;
pub const PO_THROTTLE_ADAPTIVE: u32 = 3;
pub const PO_THROTTLE_CONSTANT: u32 = 1;
pub const PO_THROTTLE_DEGRADE: u32 = 2;
pub const PO_THROTTLE_MAXIMUM: u32 = 4;
pub const PO_THROTTLE_NONE: u32 = 0;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPACKEDEVENTINFO(pub *mut PACKEDEVENTINFO);
impl PPACKEDEVENTINFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPACKEDEVENTINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPERFORMANCE_DATA(pub *mut PERFORMANCE_DATA);
impl PPERFORMANCE_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPERFORMANCE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PPM_FIRMWARE_ACPI1C2: u32 = 1;
pub const PPM_FIRMWARE_ACPI1C3: u32 = 2;
pub const PPM_FIRMWARE_ACPI1TSTATES: u32 = 4;
pub const PPM_FIRMWARE_CPC: u32 = 262144;
pub const PPM_FIRMWARE_CSD: u32 = 16;
pub const PPM_FIRMWARE_CST: u32 = 8;
pub const PPM_FIRMWARE_LPI: u32 = 524288;
pub const PPM_FIRMWARE_OSC: u32 = 65536;
pub const PPM_FIRMWARE_PCCH: u32 = 16384;
pub const PPM_FIRMWARE_PCCP: u32 = 32768;
pub const PPM_FIRMWARE_PCT: u32 = 32;
pub const PPM_FIRMWARE_PDC: u32 = 131072;
pub const PPM_FIRMWARE_PPC: u32 = 256;
pub const PPM_FIRMWARE_PSD: u32 = 512;
pub const PPM_FIRMWARE_PSS: u32 = 64;
pub const PPM_FIRMWARE_PTC: u32 = 1024;
pub const PPM_FIRMWARE_TPC: u32 = 4096;
pub const PPM_FIRMWARE_TSD: u32 = 8192;
pub const PPM_FIRMWARE_TSS: u32 = 2048;
pub const PPM_FIRMWARE_XPSS: u32 = 128;
pub const PPM_IDLESTATES_DATA_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xba138e10_e250_4ad7_8616_cf1a7ad410e7);
pub const PPM_IDLESTATE_CHANGE_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x4838fe4f_f71c_4e51_9ecc_8430a7ac4c6c);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PPM_IDLESTATE_EVENT {
    pub NewState: u32,
    pub OldState: u32,
    pub Processors: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PPM_IDLE_ACCOUNTING {
    pub StateCount: u32,
    pub TotalTransitions: u32,
    pub ResetCount: u32,
    pub StartTime: u64,
    pub State: [PPM_IDLE_STATE_ACCOUNTING; 1],
}
impl Default for PPM_IDLE_ACCOUNTING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PPM_IDLE_ACCOUNTING_EX {
    pub StateCount: u32,
    pub TotalTransitions: u32,
    pub ResetCount: u32,
    pub AbortCount: u32,
    pub StartTime: u64,
    pub State: [PPM_IDLE_STATE_ACCOUNTING_EX; 1],
}
impl Default for PPM_IDLE_ACCOUNTING_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PPM_IDLE_ACCOUNTING_EX_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xd67abd39_81f8_4a5e_8152_72e31ec912ee);
pub const PPM_IDLE_ACCOUNTING_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xe2a26f78_ae07_4ee0_a30f_ce54f55a94cd);
pub const PPM_IDLE_IMPLEMENTATION_CSTATES: u32 = 1;
pub const PPM_IDLE_IMPLEMENTATION_LPISTATES: u32 = 4;
pub const PPM_IDLE_IMPLEMENTATION_MICROPEP: u32 = 3;
pub const PPM_IDLE_IMPLEMENTATION_NONE: u32 = 0;
pub const PPM_IDLE_IMPLEMENTATION_PEP: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PPM_IDLE_STATE_ACCOUNTING {
    pub IdleTransitions: u32,
    pub FailedTransitions: u32,
    pub InvalidBucketIndex: u32,
    pub TotalTime: u64,
    pub IdleTimeBuckets: [u32; 6],
}
impl Default for PPM_IDLE_STATE_ACCOUNTING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PPM_IDLE_STATE_ACCOUNTING_EX {
    pub TotalTime: u64,
    pub IdleTransitions: u32,
    pub FailedTransitions: u32,
    pub InvalidBucketIndex: u32,
    pub MinTimeUs: u32,
    pub MaxTimeUs: u32,
    pub CancelledTransitions: u32,
    pub IdleTimeBuckets: [PPM_IDLE_STATE_BUCKET_EX; 16],
}
impl Default for PPM_IDLE_STATE_ACCOUNTING_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PPM_IDLE_STATE_BUCKET_EX {
    pub TotalTimeUs: u64,
    pub MinTimeUs: u32,
    pub MaxTimeUs: u32,
    pub Count: u32,
}
pub const PPM_PERFMON_PERFSTATE_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x7fd18652_0cfe_40d2_b0a1_0b066a87759e);
pub const PPM_PERFORMANCE_IMPLEMENTATION_CPPC: u32 = 3;
pub const PPM_PERFORMANCE_IMPLEMENTATION_NONE: u32 = 0;
pub const PPM_PERFORMANCE_IMPLEMENTATION_PCCV1: u32 = 2;
pub const PPM_PERFORMANCE_IMPLEMENTATION_PEP: u32 = 4;
pub const PPM_PERFORMANCE_IMPLEMENTATION_PSTATES: u32 = 1;
pub const PPM_PERFSTATES_DATA_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x5708cc20_7d40_4bf4_b4aa_2b01338d0126);
pub const PPM_PERFSTATE_CHANGE_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xa5b32ddd_7f39_4abc_b892_900e43b59ebb);
pub const PPM_PERFSTATE_DOMAIN_CHANGE_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x995e6b7f_d653_497a_b978_36a30c29bf01);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PPM_PERFSTATE_DOMAIN_EVENT {
    pub State: u32,
    pub Latency: u32,
    pub Speed: u32,
    pub Processors: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PPM_PERFSTATE_EVENT {
    pub State: u32,
    pub Status: u32,
    pub Latency: u32,
    pub Speed: u32,
    pub Processor: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PPM_THERMALCHANGE_EVENT {
    pub ThermalConstraint: u32,
    pub Processors: u64,
}
pub const PPM_THERMALCONSTRAINT_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xa852c2c8_1a4c_423b_8c2c_f30d82931a88);
pub const PPM_THERMAL_POLICY_CHANGE_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x48f377b8_6880_4c7b_8bdc_380176c6654d);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PPM_THERMAL_POLICY_EVENT {
    pub Mode: u8,
    pub Processors: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PPM_WMI_IDLE_STATE {
    pub Latency: u32,
    pub Power: u32,
    pub TimeCheck: u32,
    pub PromotePercent: u8,
    pub DemotePercent: u8,
    pub StateType: u8,
    pub Reserved: u8,
    pub StateFlags: u32,
    pub Context: u32,
    pub IdleHandler: u32,
    pub Reserved1: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PPM_WMI_IDLE_STATES {
    pub Type: u32,
    pub Count: u32,
    pub TargetState: u32,
    pub OldState: u32,
    pub TargetProcessors: u64,
    pub State: [PPM_WMI_IDLE_STATE; 1],
}
impl Default for PPM_WMI_IDLE_STATES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PPM_WMI_IDLE_STATES_EX {
    pub Type: u32,
    pub Count: u32,
    pub TargetState: u32,
    pub OldState: u32,
    pub TargetProcessors: *mut core::ffi::c_void,
    pub State: [PPM_WMI_IDLE_STATE; 1],
}
impl Default for PPM_WMI_IDLE_STATES_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PPM_WMI_LEGACY_PERFSTATE {
    pub Frequency: u32,
    pub Flags: u32,
    pub PercentFrequency: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PPM_WMI_PERF_STATE {
    pub Frequency: u32,
    pub Power: u32,
    pub PercentFrequency: u8,
    pub IncreaseLevel: u8,
    pub DecreaseLevel: u8,
    pub Type: u8,
    pub IncreaseTime: u32,
    pub DecreaseTime: u32,
    pub Control: u64,
    pub Status: u64,
    pub HitCount: u32,
    pub Reserved1: u32,
    pub Reserved2: u64,
    pub Reserved3: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PPM_WMI_PERF_STATES {
    pub Count: u32,
    pub MaxFrequency: u32,
    pub CurrentState: u32,
    pub MaxPerfState: u32,
    pub MinPerfState: u32,
    pub LowestPerfState: u32,
    pub ThermalConstraint: u32,
    pub BusyAdjThreshold: u8,
    pub PolicyType: u8,
    pub Type: u8,
    pub Reserved: u8,
    pub TimerInterval: u32,
    pub TargetProcessors: u64,
    pub PStateHandler: u32,
    pub PStateContext: u32,
    pub TStateHandler: u32,
    pub TStateContext: u32,
    pub FeedbackHandler: u32,
    pub Reserved1: u32,
    pub Reserved2: u64,
    pub State: [PPM_WMI_PERF_STATE; 1],
}
impl Default for PPM_WMI_PERF_STATES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PPM_WMI_PERF_STATES_EX {
    pub Count: u32,
    pub MaxFrequency: u32,
    pub CurrentState: u32,
    pub MaxPerfState: u32,
    pub MinPerfState: u32,
    pub LowestPerfState: u32,
    pub ThermalConstraint: u32,
    pub BusyAdjThreshold: u8,
    pub PolicyType: u8,
    pub Type: u8,
    pub Reserved: u8,
    pub TimerInterval: u32,
    pub TargetProcessors: *mut core::ffi::c_void,
    pub PStateHandler: u32,
    pub PStateContext: u32,
    pub TStateHandler: u32,
    pub TStateContext: u32,
    pub FeedbackHandler: u32,
    pub Reserved1: u32,
    pub Reserved2: u64,
    pub State: [PPM_WMI_PERF_STATE; 1],
}
impl Default for PPM_WMI_PERF_STATES_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPOWER_ACTION(pub *mut POWER_ACTION);
impl PPOWER_ACTION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPOWER_ACTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPOWER_ACTION_POLICY(pub *mut POWER_ACTION_POLICY);
impl PPOWER_ACTION_POLICY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPOWER_ACTION_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPOWER_IDLE_RESILIENCY(pub *mut POWER_IDLE_RESILIENCY);
impl PPOWER_IDLE_RESILIENCY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPOWER_IDLE_RESILIENCY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPOWER_LIMIT_ATTRIBUTES(pub *mut POWER_LIMIT_ATTRIBUTES);
impl PPOWER_LIMIT_ATTRIBUTES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPOWER_LIMIT_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPOWER_LIMIT_TYPES(pub *mut POWER_LIMIT_TYPES);
impl PPOWER_LIMIT_TYPES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPOWER_LIMIT_TYPES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPOWER_LIMIT_VALUE(pub *mut POWER_LIMIT_VALUE);
impl PPOWER_LIMIT_VALUE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPOWER_LIMIT_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPOWER_MONITOR_INVOCATION(pub *mut POWER_MONITOR_INVOCATION);
impl PPOWER_MONITOR_INVOCATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPOWER_MONITOR_INVOCATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPOWER_PLATFORM_INFORMATION(pub *mut POWER_PLATFORM_INFORMATION);
impl PPOWER_PLATFORM_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPOWER_PLATFORM_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPOWER_PLATFORM_ROLE(pub *mut POWER_PLATFORM_ROLE);
impl PPOWER_PLATFORM_ROLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPOWER_PLATFORM_ROLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPOWER_REQUEST_TYPE(pub *mut POWER_REQUEST_TYPE);
impl PPOWER_REQUEST_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPOWER_REQUEST_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPOWER_SESSION_ALLOW_EXTERNAL_DMA_DEVICES(pub *mut POWER_SESSION_ALLOW_EXTERNAL_DMA_DEVICES);
impl PPOWER_SESSION_ALLOW_EXTERNAL_DMA_DEVICES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPOWER_SESSION_ALLOW_EXTERNAL_DMA_DEVICES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPOWER_SESSION_CONNECT(pub *mut POWER_SESSION_CONNECT);
impl PPOWER_SESSION_CONNECT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPOWER_SESSION_CONNECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPOWER_SESSION_RIT_STATE(pub *mut POWER_SESSION_RIT_STATE);
impl PPOWER_SESSION_RIT_STATE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPOWER_SESSION_RIT_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPOWER_SESSION_TIMEOUTS(pub *mut POWER_SESSION_TIMEOUTS);
impl PPOWER_SESSION_TIMEOUTS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPOWER_SESSION_TIMEOUTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPOWER_SESSION_WINLOGON(pub *mut POWER_SESSION_WINLOGON);
impl PPOWER_SESSION_WINLOGON {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPOWER_SESSION_WINLOGON {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPOWER_SETTING_ALTITUDE(pub *mut POWER_SETTING_ALTITUDE);
impl PPOWER_SETTING_ALTITUDE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPOWER_SETTING_ALTITUDE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPOWER_USER_PRESENCE(pub *mut POWER_USER_PRESENCE);
impl PPOWER_USER_PRESENCE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPOWER_USER_PRESENCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPOWER_USER_PRESENCE_TYPE(pub *mut POWER_USER_PRESENCE_TYPE);
impl PPOWER_USER_PRESENCE_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPOWER_USER_PRESENCE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPPM_IDLESTATE_EVENT(pub *mut PPM_IDLESTATE_EVENT);
impl PPPM_IDLESTATE_EVENT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPPM_IDLESTATE_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPPM_IDLE_ACCOUNTING(pub *mut PPM_IDLE_ACCOUNTING);
impl PPPM_IDLE_ACCOUNTING {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPPM_IDLE_ACCOUNTING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPPM_IDLE_ACCOUNTING_EX(pub *mut PPM_IDLE_ACCOUNTING_EX);
impl PPPM_IDLE_ACCOUNTING_EX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPPM_IDLE_ACCOUNTING_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPPM_IDLE_STATE_ACCOUNTING(pub *mut PPM_IDLE_STATE_ACCOUNTING);
impl PPPM_IDLE_STATE_ACCOUNTING {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPPM_IDLE_STATE_ACCOUNTING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPPM_IDLE_STATE_ACCOUNTING_EX(pub *mut PPM_IDLE_STATE_ACCOUNTING_EX);
impl PPPM_IDLE_STATE_ACCOUNTING_EX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPPM_IDLE_STATE_ACCOUNTING_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPPM_IDLE_STATE_BUCKET_EX(pub *mut PPM_IDLE_STATE_BUCKET_EX);
impl PPPM_IDLE_STATE_BUCKET_EX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPPM_IDLE_STATE_BUCKET_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPPM_PERFSTATE_DOMAIN_EVENT(pub *mut PPM_PERFSTATE_DOMAIN_EVENT);
impl PPPM_PERFSTATE_DOMAIN_EVENT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPPM_PERFSTATE_DOMAIN_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPPM_PERFSTATE_EVENT(pub *mut PPM_PERFSTATE_EVENT);
impl PPPM_PERFSTATE_EVENT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPPM_PERFSTATE_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPPM_THERMALCHANGE_EVENT(pub *mut PPM_THERMALCHANGE_EVENT);
impl PPPM_THERMALCHANGE_EVENT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPPM_THERMALCHANGE_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPPM_THERMAL_POLICY_EVENT(pub *mut PPM_THERMAL_POLICY_EVENT);
impl PPPM_THERMAL_POLICY_EVENT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPPM_THERMAL_POLICY_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPPM_WMI_IDLE_STATE(pub *mut PPM_WMI_IDLE_STATE);
impl PPPM_WMI_IDLE_STATE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPPM_WMI_IDLE_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPPM_WMI_IDLE_STATES(pub *mut PPM_WMI_IDLE_STATES);
impl PPPM_WMI_IDLE_STATES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPPM_WMI_IDLE_STATES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPPM_WMI_IDLE_STATES_EX(pub *mut PPM_WMI_IDLE_STATES_EX);
impl PPPM_WMI_IDLE_STATES_EX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPPM_WMI_IDLE_STATES_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPPM_WMI_LEGACY_PERFSTATE(pub *mut PPM_WMI_LEGACY_PERFSTATE);
impl PPPM_WMI_LEGACY_PERFSTATE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPPM_WMI_LEGACY_PERFSTATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPPM_WMI_PERF_STATE(pub *mut PPM_WMI_PERF_STATE);
impl PPPM_WMI_PERF_STATE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPPM_WMI_PERF_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPPM_WMI_PERF_STATES(pub *mut PPM_WMI_PERF_STATES);
impl PPPM_WMI_PERF_STATES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPPM_WMI_PERF_STATES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPPM_WMI_PERF_STATES_EX(pub *mut PPM_WMI_PERF_STATES_EX);
impl PPPM_WMI_PERF_STATES_EX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPPM_WMI_PERF_STATES_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPRIVILEGE_SET(pub *mut PRIVILEGE_SET);
impl PPRIVILEGE_SET {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPRIVILEGE_SET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROCESSOR_CACHE_TYPE(pub *mut PROCESSOR_CACHE_TYPE);
impl PPROCESSOR_CACHE_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPROCESSOR_CACHE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "basetsd")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROCESSOR_GROUP_INFO(pub *mut PROCESSOR_GROUP_INFO);
#[cfg(feature = "basetsd")]
impl PPROCESSOR_GROUP_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "basetsd")]
impl Default for PPROCESSOR_GROUP_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROCESSOR_IDLESTATE_INFO(pub *mut PROCESSOR_IDLESTATE_INFO);
impl PPROCESSOR_IDLESTATE_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPROCESSOR_IDLESTATE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROCESSOR_IDLESTATE_POLICY(pub *mut PROCESSOR_IDLESTATE_POLICY);
impl PPROCESSOR_IDLESTATE_POLICY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPROCESSOR_IDLESTATE_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROCESSOR_NUMBER(pub *mut PROCESSOR_NUMBER);
impl PPROCESSOR_NUMBER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPROCESSOR_NUMBER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROCESSOR_PERFSTATE_POLICY(pub *mut PROCESSOR_PERFSTATE_POLICY);
impl PPROCESSOR_PERFSTATE_POLICY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPROCESSOR_PERFSTATE_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROCESSOR_POWER_POLICY(pub *mut PROCESSOR_POWER_POLICY);
impl PPROCESSOR_POWER_POLICY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPROCESSOR_POWER_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROCESSOR_POWER_POLICY_INFO(pub *mut PROCESSOR_POWER_POLICY_INFO);
impl PPROCESSOR_POWER_POLICY_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPROCESSOR_POWER_POLICY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "basetsd")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROCESSOR_RELATIONSHIP(pub *mut PROCESSOR_RELATIONSHIP);
#[cfg(feature = "basetsd")]
impl PPROCESSOR_RELATIONSHIP {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "basetsd")]
impl Default for PPROCESSOR_RELATIONSHIP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROCESSOR_SHARED_COMPUTE_UNIT_TYPE(pub *mut PROCESSOR_SHARED_COMPUTE_UNIT_TYPE);
impl PPROCESSOR_SHARED_COMPUTE_UNIT_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPROCESSOR_SHARED_COMPUTE_UNIT_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROCESS_DYNAMIC_EH_CONTINUATION_TARGET(pub *mut PROCESS_DYNAMIC_EH_CONTINUATION_TARGET);
impl PPROCESS_DYNAMIC_EH_CONTINUATION_TARGET {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPROCESS_DYNAMIC_EH_CONTINUATION_TARGET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION(pub *mut PROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION);
impl PPROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE(pub *mut PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE);
impl PPROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION(pub *mut PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION);
impl PPROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROCESS_MITIGATION_ASLR_POLICY(pub *mut PROCESS_MITIGATION_ASLR_POLICY);
impl PPROCESS_MITIGATION_ASLR_POLICY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPROCESS_MITIGATION_ASLR_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROCESS_MITIGATION_BINARY_SIGNATURE_POLICY(pub *mut PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY);
impl PPROCESS_MITIGATION_BINARY_SIGNATURE_POLICY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPROCESS_MITIGATION_BINARY_SIGNATURE_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROCESS_MITIGATION_CHILD_PROCESS_POLICY(pub *mut PROCESS_MITIGATION_CHILD_PROCESS_POLICY);
impl PPROCESS_MITIGATION_CHILD_PROCESS_POLICY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPROCESS_MITIGATION_CHILD_PROCESS_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY(pub *mut PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY);
impl PPROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROCESS_MITIGATION_DEP_POLICY(pub *mut PROCESS_MITIGATION_DEP_POLICY);
impl PPROCESS_MITIGATION_DEP_POLICY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPROCESS_MITIGATION_DEP_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROCESS_MITIGATION_DYNAMIC_CODE_POLICY(pub *mut PROCESS_MITIGATION_DYNAMIC_CODE_POLICY);
impl PPROCESS_MITIGATION_DYNAMIC_CODE_POLICY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPROCESS_MITIGATION_DYNAMIC_CODE_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY(pub *mut PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY);
impl PPROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROCESS_MITIGATION_FONT_DISABLE_POLICY(pub *mut PROCESS_MITIGATION_FONT_DISABLE_POLICY);
impl PPROCESS_MITIGATION_FONT_DISABLE_POLICY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPROCESS_MITIGATION_FONT_DISABLE_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROCESS_MITIGATION_IMAGE_LOAD_POLICY(pub *mut PROCESS_MITIGATION_IMAGE_LOAD_POLICY);
impl PPROCESS_MITIGATION_IMAGE_LOAD_POLICY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPROCESS_MITIGATION_IMAGE_LOAD_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY(pub *mut PROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY);
impl PPROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROCESS_MITIGATION_POLICY(pub *mut PROCESS_MITIGATION_POLICY);
impl PPROCESS_MITIGATION_POLICY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPROCESS_MITIGATION_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROCESS_MITIGATION_REDIRECTION_TRUST_POLICY(pub *mut PROCESS_MITIGATION_REDIRECTION_TRUST_POLICY);
impl PPROCESS_MITIGATION_REDIRECTION_TRUST_POLICY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPROCESS_MITIGATION_REDIRECTION_TRUST_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROCESS_MITIGATION_SEHOP_POLICY(pub *mut PROCESS_MITIGATION_SEHOP_POLICY);
impl PPROCESS_MITIGATION_SEHOP_POLICY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPROCESS_MITIGATION_SEHOP_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY(pub *mut PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY);
impl PPROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY(pub *mut PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY);
impl PPROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY(pub *mut PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY);
impl PPROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY(pub *mut PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY);
impl PPROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROCESS_MITIGATION_USER_POINTER_AUTH_POLICY(pub *mut PROCESS_MITIGATION_USER_POINTER_AUTH_POLICY);
impl PPROCESS_MITIGATION_USER_POINTER_AUTH_POLICY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPROCESS_MITIGATION_USER_POINTER_AUTH_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROCESS_MITIGATION_USER_SHADOW_STACK_POLICY(pub *mut PROCESS_MITIGATION_USER_SHADOW_STACK_POLICY);
impl PPROCESS_MITIGATION_USER_SHADOW_STACK_POLICY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPROCESS_MITIGATION_USER_SHADOW_STACK_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPROCESS_NETWORK_COUNTERS(pub *mut PROCESS_NETWORK_COUNTERS);
impl PPROCESS_NETWORK_COUNTERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPROCESS_NETWORK_COUNTERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PQUOTA_LIMITS(pub *mut QUOTA_LIMITS);
impl PQUOTA_LIMITS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PQUOTA_LIMITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PQUOTA_LIMITS_EX(pub *mut QUOTA_LIMITS_EX);
impl PQUOTA_LIMITS_EX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PQUOTA_LIMITS_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PRAGMA_DEPRECATED_DDK: u32 = 0;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRATE_QUOTA_LIMIT(pub *mut RATE_QUOTA_LIMIT);
impl PRATE_QUOTA_LIMIT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRATE_QUOTA_LIMIT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PREARRANGE_FILE_DATA(pub *mut REARRANGE_FILE_DATA);
impl PREARRANGE_FILE_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PREARRANGE_FILE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PREARRANGE_FILE_DATA32(pub *mut REARRANGE_FILE_DATA32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl PREARRANGE_FILE_DATA32 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for PREARRANGE_FILE_DATA32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PREPARSE_GUID_DATA_BUFFER(pub *mut REPARSE_GUID_DATA_BUFFER);
impl PREPARSE_GUID_DATA_BUFFER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PREPARSE_GUID_DATA_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRESOURCEMANAGER_BASIC_INFORMATION(pub *mut RESOURCEMANAGER_BASIC_INFORMATION);
impl PRESOURCEMANAGER_BASIC_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRESOURCEMANAGER_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRESOURCEMANAGER_COMPLETION_INFORMATION(pub *mut RESOURCEMANAGER_COMPLETION_INFORMATION);
impl PRESOURCEMANAGER_COMPLETION_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRESOURCEMANAGER_COMPLETION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRESUME_PERFORMANCE(pub *mut RESUME_PERFORMANCE);
impl PRESUME_PERFORMANCE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRESUME_PERFORMANCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PRIVILEGE_SET {
    pub PrivilegeCount: u32,
    pub Control: u32,
    pub Privilege: [LUID_AND_ATTRIBUTES; 1],
}
impl Default for PRIVILEGE_SET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PRIVILEGE_SET_ALL_NECESSARY: u32 = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRLIST_ENTRY(pub *mut LIST_ENTRY);
impl PRLIST_ENTRY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRLIST_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PROCESSOR_ALPHA_21064: u32 = 21064;
pub const PROCESSOR_AMD_X8664: u32 = 8664;
pub const PROCESSOR_ARCHITECTURE_ALPHA: u32 = 2;
pub const PROCESSOR_ARCHITECTURE_ALPHA64: u32 = 7;
pub const PROCESSOR_ARCHITECTURE_AMD64: u32 = 9;
pub const PROCESSOR_ARCHITECTURE_ARM: u32 = 5;
pub const PROCESSOR_ARCHITECTURE_ARM32_ON_WIN64: u32 = 13;
pub const PROCESSOR_ARCHITECTURE_ARM64: u32 = 12;
pub const PROCESSOR_ARCHITECTURE_IA32_ON_ARM64: u32 = 14;
pub const PROCESSOR_ARCHITECTURE_IA32_ON_WIN64: u32 = 10;
pub const PROCESSOR_ARCHITECTURE_IA64: u32 = 6;
pub const PROCESSOR_ARCHITECTURE_INTEL: u32 = 0;
pub const PROCESSOR_ARCHITECTURE_MIPS: u32 = 1;
pub const PROCESSOR_ARCHITECTURE_MSIL: u32 = 8;
pub const PROCESSOR_ARCHITECTURE_NEUTRAL: u32 = 11;
pub const PROCESSOR_ARCHITECTURE_PPC: u32 = 3;
pub const PROCESSOR_ARCHITECTURE_SHX: u32 = 4;
pub const PROCESSOR_ARCHITECTURE_UNKNOWN: u32 = 65535;
pub const PROCESSOR_ARM720: u32 = 1824;
pub const PROCESSOR_ARM820: u32 = 2080;
pub const PROCESSOR_ARM920: u32 = 2336;
pub const PROCESSOR_ARM_7TDMI: u32 = 70001;
pub type PROCESSOR_CACHE_TYPE = i32;
pub const PROCESSOR_DUTY_CYCLING_DISABLED: u32 = 0;
pub const PROCESSOR_DUTY_CYCLING_ENABLED: u32 = 1;
#[repr(C)]
#[cfg(feature = "basetsd")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROCESSOR_GROUP_INFO {
    pub MaximumProcessorCount: u8,
    pub ActiveProcessorCount: u8,
    pub Reserved: [u8; 38],
    pub ActiveProcessorMask: super::basetsd::KAFFINITY,
}
#[cfg(feature = "basetsd")]
impl Default for PROCESSOR_GROUP_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PROCESSOR_HITACHI_SH3: u32 = 10003;
pub const PROCESSOR_HITACHI_SH3E: u32 = 10004;
pub const PROCESSOR_HITACHI_SH4: u32 = 10005;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROCESSOR_IDLESTATE_INFO {
    pub TimeCheck: u32,
    pub DemotePercent: u8,
    pub PromotePercent: u8,
    pub Spare: [u8; 2],
}
impl Default for PROCESSOR_IDLESTATE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESSOR_IDLESTATE_POLICY {
    pub Revision: u16,
    pub Flags: PROCESSOR_IDLESTATE_POLICY_0,
    pub PolicyCount: u32,
    pub Policy: [PROCESSOR_IDLESTATE_INFO; 3],
}
impl Default for PROCESSOR_IDLESTATE_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROCESSOR_IDLESTATE_POLICY_0 {
    pub AsWORD: u16,
    pub Anonymous: PROCESSOR_IDLESTATE_POLICY_0_0,
}
impl Default for PROCESSOR_IDLESTATE_POLICY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESSOR_IDLESTATE_POLICY_0_0 {
    pub _bitfield: u16,
}
pub const PROCESSOR_IDLESTATE_POLICY_COUNT: u32 = 3;
pub const PROCESSOR_INTEL_386: u32 = 386;
pub const PROCESSOR_INTEL_486: u32 = 486;
pub const PROCESSOR_INTEL_IA64: u32 = 2200;
pub const PROCESSOR_INTEL_PENTIUM: u32 = 586;
pub const PROCESSOR_MIPS_R4000: u32 = 4000;
pub const PROCESSOR_MOTOROLA_821: u32 = 821;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESSOR_NUMBER {
    pub Group: u16,
    pub Number: u8,
    pub Reserved: u8,
}
pub const PROCESSOR_OPTIL: u32 = 18767;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESSOR_PERFSTATE_POLICY {
    pub Revision: u32,
    pub MaxThrottle: u8,
    pub MinThrottle: u8,
    pub BusyAdjThreshold: u8,
    pub Anonymous: PROCESSOR_PERFSTATE_POLICY_0,
    pub TimeCheck: u32,
    pub IncreaseTime: u32,
    pub DecreaseTime: u32,
    pub IncreasePercent: u32,
    pub DecreasePercent: u32,
}
impl Default for PROCESSOR_PERFSTATE_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROCESSOR_PERFSTATE_POLICY_0 {
    pub Spare: u8,
    pub Flags: PROCESSOR_PERFSTATE_POLICY_0_0,
}
impl Default for PROCESSOR_PERFSTATE_POLICY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROCESSOR_PERFSTATE_POLICY_0_0 {
    pub AsBYTE: u8,
    pub Anonymous: PROCESSOR_PERFSTATE_POLICY_0_0_0,
}
impl Default for PROCESSOR_PERFSTATE_POLICY_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESSOR_PERFSTATE_POLICY_0_0_0 {
    pub _bitfield: u8,
}
pub const PROCESSOR_PERF_AUTONOMOUS_MODE_DISABLED: u32 = 0;
pub const PROCESSOR_PERF_AUTONOMOUS_MODE_ENABLED: u32 = 1;
pub const PROCESSOR_PERF_BOOST_MODE_AGGRESSIVE: u32 = 2;
pub const PROCESSOR_PERF_BOOST_MODE_AGGRESSIVE_AT_GUARANTEED: u32 = 5;
pub const PROCESSOR_PERF_BOOST_MODE_DISABLED: u32 = 0;
pub const PROCESSOR_PERF_BOOST_MODE_EFFICIENT_AGGRESSIVE: u32 = 4;
pub const PROCESSOR_PERF_BOOST_MODE_EFFICIENT_AGGRESSIVE_AT_GUARANTEED: u32 = 6;
pub const PROCESSOR_PERF_BOOST_MODE_EFFICIENT_ENABLED: u32 = 3;
pub const PROCESSOR_PERF_BOOST_MODE_ENABLED: u32 = 1;
pub const PROCESSOR_PERF_BOOST_MODE_MAX: u32 = 6;
pub const PROCESSOR_PERF_BOOST_POLICY_DISABLED: u32 = 0;
pub const PROCESSOR_PERF_BOOST_POLICY_MAX: u32 = 100;
pub const PROCESSOR_PERF_ENERGY_PREFERENCE: u32 = 0;
pub const PROCESSOR_PERF_MAXIMUM_ACTIVITY_WINDOW: u32 = 1270000000;
pub const PROCESSOR_PERF_MINIMUM_ACTIVITY_WINDOW: u32 = 0;
pub const PROCESSOR_PERF_PERFORMANCE_PREFERENCE: u32 = 255;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROCESSOR_POWER_POLICY {
    pub Revision: u32,
    pub DynamicThrottle: u8,
    pub Spare: [u8; 3],
    pub _bitfield: u32,
    pub PolicyCount: u32,
    pub Policy: [PROCESSOR_POWER_POLICY_INFO; 3],
}
impl Default for PROCESSOR_POWER_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROCESSOR_POWER_POLICY_INFO {
    pub TimeCheck: u32,
    pub DemoteLimit: u32,
    pub PromoteLimit: u32,
    pub DemotePercent: u8,
    pub PromotePercent: u8,
    pub Spare: [u8; 2],
    pub _bitfield: u32,
}
impl Default for PROCESSOR_POWER_POLICY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PROCESSOR_PPC_601: u32 = 601;
pub const PROCESSOR_PPC_603: u32 = 603;
pub const PROCESSOR_PPC_604: u32 = 604;
pub const PROCESSOR_PPC_620: u32 = 620;
#[repr(C)]
#[cfg(feature = "basetsd")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PROCESSOR_RELATIONSHIP {
    pub Flags: u8,
    pub EfficiencyClass: u8,
    pub Reserved: [u8; 20],
    pub GroupCount: u16,
    pub GroupMask: [GROUP_AFFINITY; 1],
}
#[cfg(feature = "basetsd")]
impl Default for PROCESSOR_RELATIONSHIP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PROCESSOR_SHARED_COMPUTE_UNIT_TYPE = i32;
pub const PROCESSOR_SHx_SH3: u32 = 103;
pub const PROCESSOR_SHx_SH4: u32 = 104;
pub const PROCESSOR_STRONGARM: u32 = 2577;
pub const PROCESSOR_THROTTLE_AUTOMATIC: u32 = 2;
pub const PROCESSOR_THROTTLE_DISABLED: u32 = 0;
pub const PROCESSOR_THROTTLE_ENABLED: u32 = 1;
pub const PROCESS_ALL_ACCESS: u32 = 2097151;
pub const PROCESS_CREATE_PROCESS: u32 = 128;
pub const PROCESS_CREATE_THREAD: u32 = 2;
pub const PROCESS_DUP_HANDLE: u32 = 64;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESS_DYNAMIC_EH_CONTINUATION_TARGET {
    pub TargetAddress: usize,
    pub Flags: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESS_DYNAMIC_EH_CONTINUATION_TARGETS_INFORMATION {
    pub NumberOfTargets: u16,
    pub Reserved: u16,
    pub Reserved2: u32,
    pub Targets: PPROCESS_DYNAMIC_EH_CONTINUATION_TARGET,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE {
    pub BaseAddress: usize,
    pub Size: usize,
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGES_INFORMATION {
    pub NumberOfRanges: u16,
    pub Reserved: u16,
    pub Reserved2: u32,
    pub Ranges: PPROCESS_DYNAMIC_ENFORCED_ADDRESS_RANGE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_MITIGATION_ASLR_POLICY {
    pub Anonymous: PROCESS_MITIGATION_ASLR_POLICY_0,
}
impl Default for PROCESS_MITIGATION_ASLR_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROCESS_MITIGATION_ASLR_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_ASLR_POLICY_0_0,
}
impl Default for PROCESS_MITIGATION_ASLR_POLICY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESS_MITIGATION_ASLR_POLICY_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY {
    pub Anonymous: PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY_0,
}
impl Default for PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY_0_0,
}
impl Default for PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESS_MITIGATION_BINARY_SIGNATURE_POLICY_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_MITIGATION_CHILD_PROCESS_POLICY {
    pub Anonymous: PROCESS_MITIGATION_CHILD_PROCESS_POLICY_0,
}
impl Default for PROCESS_MITIGATION_CHILD_PROCESS_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROCESS_MITIGATION_CHILD_PROCESS_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_CHILD_PROCESS_POLICY_0_0,
}
impl Default for PROCESS_MITIGATION_CHILD_PROCESS_POLICY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESS_MITIGATION_CHILD_PROCESS_POLICY_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY {
    pub Anonymous: PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY_0,
}
impl Default for PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY_0_0,
}
impl Default for PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESS_MITIGATION_CONTROL_FLOW_GUARD_POLICY_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_MITIGATION_DEP_POLICY {
    pub Anonymous: PROCESS_MITIGATION_DEP_POLICY_0,
    pub Permanent: bool,
}
impl Default for PROCESS_MITIGATION_DEP_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROCESS_MITIGATION_DEP_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_DEP_POLICY_0_0,
}
impl Default for PROCESS_MITIGATION_DEP_POLICY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESS_MITIGATION_DEP_POLICY_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_MITIGATION_DYNAMIC_CODE_POLICY {
    pub Anonymous: PROCESS_MITIGATION_DYNAMIC_CODE_POLICY_0,
}
impl Default for PROCESS_MITIGATION_DYNAMIC_CODE_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROCESS_MITIGATION_DYNAMIC_CODE_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_DYNAMIC_CODE_POLICY_0_0,
}
impl Default for PROCESS_MITIGATION_DYNAMIC_CODE_POLICY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESS_MITIGATION_DYNAMIC_CODE_POLICY_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY {
    pub Anonymous: PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY_0,
}
impl Default for PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY_0_0,
}
impl Default for PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESS_MITIGATION_EXTENSION_POINT_DISABLE_POLICY_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_MITIGATION_FONT_DISABLE_POLICY {
    pub Anonymous: PROCESS_MITIGATION_FONT_DISABLE_POLICY_0,
}
impl Default for PROCESS_MITIGATION_FONT_DISABLE_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROCESS_MITIGATION_FONT_DISABLE_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_FONT_DISABLE_POLICY_0_0,
}
impl Default for PROCESS_MITIGATION_FONT_DISABLE_POLICY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESS_MITIGATION_FONT_DISABLE_POLICY_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_MITIGATION_IMAGE_LOAD_POLICY {
    pub Anonymous: PROCESS_MITIGATION_IMAGE_LOAD_POLICY_0,
}
impl Default for PROCESS_MITIGATION_IMAGE_LOAD_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROCESS_MITIGATION_IMAGE_LOAD_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_IMAGE_LOAD_POLICY_0_0,
}
impl Default for PROCESS_MITIGATION_IMAGE_LOAD_POLICY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESS_MITIGATION_IMAGE_LOAD_POLICY_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY {
    pub Anonymous: PROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY_0,
}
impl Default for PROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY_0_0,
}
impl Default for PROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESS_MITIGATION_PAYLOAD_RESTRICTION_POLICY_0_0 {
    pub _bitfield: u32,
}
pub type PROCESS_MITIGATION_POLICY = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_MITIGATION_REDIRECTION_TRUST_POLICY {
    pub Anonymous: PROCESS_MITIGATION_REDIRECTION_TRUST_POLICY_0,
}
impl Default for PROCESS_MITIGATION_REDIRECTION_TRUST_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROCESS_MITIGATION_REDIRECTION_TRUST_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_REDIRECTION_TRUST_POLICY_0_0,
}
impl Default for PROCESS_MITIGATION_REDIRECTION_TRUST_POLICY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESS_MITIGATION_REDIRECTION_TRUST_POLICY_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_MITIGATION_SEHOP_POLICY {
    pub Anonymous: PROCESS_MITIGATION_SEHOP_POLICY_0,
}
impl Default for PROCESS_MITIGATION_SEHOP_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROCESS_MITIGATION_SEHOP_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_SEHOP_POLICY_0_0,
}
impl Default for PROCESS_MITIGATION_SEHOP_POLICY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESS_MITIGATION_SEHOP_POLICY_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY {
    pub Anonymous: PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY_0,
}
impl Default for PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY_0_0,
}
impl Default for PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESS_MITIGATION_SIDE_CHANNEL_ISOLATION_POLICY_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY {
    pub Anonymous: PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY_0,
}
impl Default for PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY_0_0,
}
impl Default for PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESS_MITIGATION_STRICT_HANDLE_CHECK_POLICY_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY {
    pub Anonymous: PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY_0,
}
impl Default for PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY_0_0,
}
impl Default for PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESS_MITIGATION_SYSTEM_CALL_DISABLE_POLICY_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY {
    pub Anonymous: PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY_0,
}
impl Default for PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY_0_0,
}
impl Default for PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESS_MITIGATION_SYSTEM_CALL_FILTER_POLICY_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_MITIGATION_USER_POINTER_AUTH_POLICY {
    pub Anonymous: PROCESS_MITIGATION_USER_POINTER_AUTH_POLICY_0,
}
impl Default for PROCESS_MITIGATION_USER_POINTER_AUTH_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROCESS_MITIGATION_USER_POINTER_AUTH_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_USER_POINTER_AUTH_POLICY_0_0,
}
impl Default for PROCESS_MITIGATION_USER_POINTER_AUTH_POLICY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESS_MITIGATION_USER_POINTER_AUTH_POLICY_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_MITIGATION_USER_SHADOW_STACK_POLICY {
    pub Anonymous: PROCESS_MITIGATION_USER_SHADOW_STACK_POLICY_0,
}
impl Default for PROCESS_MITIGATION_USER_SHADOW_STACK_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROCESS_MITIGATION_USER_SHADOW_STACK_POLICY_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_MITIGATION_USER_SHADOW_STACK_POLICY_0_0,
}
impl Default for PROCESS_MITIGATION_USER_SHADOW_STACK_POLICY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESS_MITIGATION_USER_SHADOW_STACK_POLICY_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PROCESS_NETWORK_COUNTERS {
    pub BytesIn: u64,
    pub BytesOut: u64,
}
pub const PROCESS_QUERY_INFORMATION: u32 = 1024;
pub const PROCESS_QUERY_LIMITED_INFORMATION: u32 = 4096;
pub const PROCESS_SET_INFORMATION: u32 = 512;
pub const PROCESS_SET_LIMITED_INFORMATION: u32 = 8192;
pub const PROCESS_SET_QUOTA: u32 = 256;
pub const PROCESS_SET_SESSIONID: u32 = 4;
pub const PROCESS_SUSPEND_RESUME: u32 = 2048;
pub const PROCESS_TERMINATE: u32 = 1;
pub const PROCESS_TRUST_LABEL_SECURITY_INFORMATION: u32 = 128;
pub const PROCESS_VM_OPERATION: u32 = 8;
pub const PROCESS_VM_READ: u32 = 16;
pub const PROCESS_VM_WRITE: u32 = 32;
pub const PROC_IDLE_BUCKET_COUNT: u32 = 6;
pub const PROC_IDLE_BUCKET_COUNT_EX: u32 = 16;
pub const PRODUCT_ARM64_SERVER: u32 = 120;
pub const PRODUCT_AZURESTACKHCI_SERVER_CORE: u32 = 406;
pub const PRODUCT_AZURE_NANO_SERVER: u32 = 169;
pub const PRODUCT_AZURE_SERVER_AGENTBRIDGE: u32 = 208;
pub const PRODUCT_AZURE_SERVER_CLOUDHOST: u32 = 199;
pub const PRODUCT_AZURE_SERVER_CLOUDMOS: u32 = 200;
pub const PRODUCT_AZURE_SERVER_CORE: u32 = 168;
pub const PRODUCT_AZURE_SERVER_NANOHOST: u32 = 209;
pub const PRODUCT_BUSINESS: u32 = 6;
pub const PRODUCT_BUSINESS_N: u32 = 16;
pub const PRODUCT_CLOUD: u32 = 178;
pub const PRODUCT_CLOUDE: u32 = 183;
pub const PRODUCT_CLOUDEDITION: u32 = 203;
pub const PRODUCT_CLOUDEDITIONN: u32 = 202;
pub const PRODUCT_CLOUDEN: u32 = 186;
pub const PRODUCT_CLOUDN: u32 = 179;
pub const PRODUCT_CLOUD_HOST_INFRASTRUCTURE_SERVER: u32 = 124;
pub const PRODUCT_CLOUD_STORAGE_SERVER: u32 = 110;
pub const PRODUCT_CLUSTER_SERVER: u32 = 18;
pub const PRODUCT_CLUSTER_SERVER_V: u32 = 64;
pub const PRODUCT_CONNECTED_CAR: u32 = 117;
pub const PRODUCT_CORE: u32 = 101;
pub const PRODUCT_CORE_ARM: u32 = 97;
pub const PRODUCT_CORE_CONNECTED: u32 = 111;
pub const PRODUCT_CORE_CONNECTED_COUNTRYSPECIFIC: u32 = 116;
pub const PRODUCT_CORE_CONNECTED_N: u32 = 113;
pub const PRODUCT_CORE_CONNECTED_SINGLELANGUAGE: u32 = 115;
pub const PRODUCT_CORE_COUNTRYSPECIFIC: u32 = 99;
pub const PRODUCT_CORE_N: u32 = 98;
pub const PRODUCT_CORE_SINGLELANGUAGE: u32 = 100;
pub const PRODUCT_DATACENTER_A_SERVER_CORE: u32 = 145;
pub const PRODUCT_DATACENTER_EVALUATION_SERVER: u32 = 80;
pub const PRODUCT_DATACENTER_EVALUATION_SERVER_CORE: u32 = 159;
pub const PRODUCT_DATACENTER_NANO_SERVER: u32 = 143;
pub const PRODUCT_DATACENTER_SERVER: u32 = 8;
pub const PRODUCT_DATACENTER_SERVER_AZURE_EDITION: u32 = 407;
pub const PRODUCT_DATACENTER_SERVER_CORE: u32 = 12;
pub const PRODUCT_DATACENTER_SERVER_CORE_AZURE_EDITION: u32 = 408;
pub const PRODUCT_DATACENTER_SERVER_CORE_V: u32 = 39;
pub const PRODUCT_DATACENTER_SERVER_V: u32 = 37;
pub const PRODUCT_DATACENTER_WS_SERVER_CORE: u32 = 147;
pub const PRODUCT_DATACENTER_WS_SERVER_CORE_AZURE_EDITION: u32 = 409;
pub const PRODUCT_EDUCATION: u32 = 121;
pub const PRODUCT_EDUCATION_N: u32 = 122;
pub const PRODUCT_EMBEDDED: u32 = 65;
pub const PRODUCT_EMBEDDED_A: u32 = 88;
pub const PRODUCT_EMBEDDED_AUTOMOTIVE: u32 = 85;
pub const PRODUCT_EMBEDDED_E: u32 = 90;
pub const PRODUCT_EMBEDDED_EVAL: u32 = 107;
pub const PRODUCT_EMBEDDED_E_EVAL: u32 = 108;
pub const PRODUCT_EMBEDDED_INDUSTRY: u32 = 89;
pub const PRODUCT_EMBEDDED_INDUSTRY_A: u32 = 86;
pub const PRODUCT_EMBEDDED_INDUSTRY_A_E: u32 = 92;
pub const PRODUCT_EMBEDDED_INDUSTRY_E: u32 = 91;
pub const PRODUCT_EMBEDDED_INDUSTRY_EVAL: u32 = 105;
pub const PRODUCT_EMBEDDED_INDUSTRY_E_EVAL: u32 = 106;
pub const PRODUCT_ENTERPRISE: u32 = 4;
pub const PRODUCT_ENTERPRISEG: u32 = 171;
pub const PRODUCT_ENTERPRISEGN: u32 = 172;
pub const PRODUCT_ENTERPRISE_E: u32 = 70;
pub const PRODUCT_ENTERPRISE_EVALUATION: u32 = 72;
pub const PRODUCT_ENTERPRISE_N: u32 = 27;
pub const PRODUCT_ENTERPRISE_N_EVALUATION: u32 = 84;
pub const PRODUCT_ENTERPRISE_S: u32 = 125;
pub const PRODUCT_ENTERPRISE_SERVER: u32 = 10;
pub const PRODUCT_ENTERPRISE_SERVER_CORE: u32 = 14;
pub const PRODUCT_ENTERPRISE_SERVER_CORE_V: u32 = 41;
pub const PRODUCT_ENTERPRISE_SERVER_IA64: u32 = 15;
pub const PRODUCT_ENTERPRISE_SERVER_V: u32 = 38;
pub const PRODUCT_ENTERPRISE_SUBSCRIPTION: u32 = 140;
pub const PRODUCT_ENTERPRISE_SUBSCRIPTION_N: u32 = 141;
pub const PRODUCT_ENTERPRISE_S_EVALUATION: u32 = 129;
pub const PRODUCT_ENTERPRISE_S_N: u32 = 126;
pub const PRODUCT_ENTERPRISE_S_N_EVALUATION: u32 = 130;
pub const PRODUCT_ESSENTIALBUSINESS_SERVER_ADDL: u32 = 60;
pub const PRODUCT_ESSENTIALBUSINESS_SERVER_ADDLSVC: u32 = 62;
pub const PRODUCT_ESSENTIALBUSINESS_SERVER_MGMT: u32 = 59;
pub const PRODUCT_ESSENTIALBUSINESS_SERVER_MGMTSVC: u32 = 61;
pub const PRODUCT_HOLOGRAPHIC: u32 = 135;
pub const PRODUCT_HOLOGRAPHIC_BUSINESS: u32 = 136;
pub const PRODUCT_HOME_BASIC: u32 = 2;
pub const PRODUCT_HOME_BASIC_E: u32 = 67;
pub const PRODUCT_HOME_BASIC_N: u32 = 5;
pub const PRODUCT_HOME_PREMIUM: u32 = 3;
pub const PRODUCT_HOME_PREMIUM_E: u32 = 68;
pub const PRODUCT_HOME_PREMIUM_N: u32 = 26;
pub const PRODUCT_HOME_PREMIUM_SERVER: u32 = 34;
pub const PRODUCT_HOME_SERVER: u32 = 19;
pub const PRODUCT_HUBOS: u32 = 180;
pub const PRODUCT_HYPERV: u32 = 42;
pub const PRODUCT_INDUSTRY_HANDHELD: u32 = 118;
pub const PRODUCT_IOTEDGEOS: u32 = 187;
pub const PRODUCT_IOTENTERPRISE: u32 = 188;
pub const PRODUCT_IOTENTERPRISEK: u32 = 206;
pub const PRODUCT_IOTENTERPRISES: u32 = 191;
pub const PRODUCT_IOTENTERPRISESEVAL: u32 = 207;
pub const PRODUCT_IOTENTERPRISESK: u32 = 205;
pub const PRODUCT_IOTOS: u32 = 185;
pub const PRODUCT_IOTUAP: u32 = 123;
pub const PRODUCT_LITE: u32 = 189;
pub const PRODUCT_MEDIUMBUSINESS_SERVER_MANAGEMENT: u32 = 30;
pub const PRODUCT_MEDIUMBUSINESS_SERVER_MESSAGING: u32 = 32;
pub const PRODUCT_MEDIUMBUSINESS_SERVER_SECURITY: u32 = 31;
pub const PRODUCT_MULTIPOINT_PREMIUM_SERVER: u32 = 77;
pub const PRODUCT_MULTIPOINT_STANDARD_SERVER: u32 = 76;
pub const PRODUCT_NANO_SERVER: u32 = 109;
pub const PRODUCT_ONECOREUPDATEOS: u32 = 182;
pub const PRODUCT_PPI_PRO: u32 = 119;
pub const PRODUCT_PROFESSIONAL: u32 = 48;
pub const PRODUCT_PROFESSIONAL_E: u32 = 69;
pub const PRODUCT_PROFESSIONAL_EMBEDDED: u32 = 58;
pub const PRODUCT_PROFESSIONAL_N: u32 = 49;
pub const PRODUCT_PROFESSIONAL_S: u32 = 127;
pub const PRODUCT_PROFESSIONAL_STUDENT: u32 = 112;
pub const PRODUCT_PROFESSIONAL_STUDENT_N: u32 = 114;
pub const PRODUCT_PROFESSIONAL_S_N: u32 = 128;
pub const PRODUCT_PROFESSIONAL_WMC: u32 = 103;
pub const PRODUCT_PRO_CHINA: u32 = 139;
pub const PRODUCT_PRO_FOR_EDUCATION: u32 = 164;
pub const PRODUCT_PRO_FOR_EDUCATION_N: u32 = 165;
pub const PRODUCT_PRO_SINGLE_LANGUAGE: u32 = 138;
pub const PRODUCT_PRO_WORKSTATION: u32 = 161;
pub const PRODUCT_PRO_WORKSTATION_N: u32 = 162;
pub const PRODUCT_SB_SOLUTION_SERVER: u32 = 50;
pub const PRODUCT_SB_SOLUTION_SERVER_EM: u32 = 54;
pub const PRODUCT_SERVERRDSH: u32 = 175;
pub const PRODUCT_SERVER_FOR_SB_SOLUTIONS: u32 = 51;
pub const PRODUCT_SERVER_FOR_SB_SOLUTIONS_EM: u32 = 55;
pub const PRODUCT_SERVER_FOR_SMALLBUSINESS: u32 = 24;
pub const PRODUCT_SERVER_FOR_SMALLBUSINESS_V: u32 = 35;
pub const PRODUCT_SERVER_FOUNDATION: u32 = 33;
pub const PRODUCT_SMALLBUSINESS_SERVER: u32 = 9;
pub const PRODUCT_SMALLBUSINESS_SERVER_PREMIUM: u32 = 25;
pub const PRODUCT_SMALLBUSINESS_SERVER_PREMIUM_CORE: u32 = 63;
pub const PRODUCT_SOLUTION_EMBEDDEDSERVER: u32 = 56;
pub const PRODUCT_SOLUTION_EMBEDDEDSERVER_CORE: u32 = 57;
pub const PRODUCT_STANDARD_A_SERVER_CORE: u32 = 146;
pub const PRODUCT_STANDARD_EVALUATION_SERVER: u32 = 79;
pub const PRODUCT_STANDARD_EVALUATION_SERVER_CORE: u32 = 160;
pub const PRODUCT_STANDARD_NANO_SERVER: u32 = 144;
pub const PRODUCT_STANDARD_SERVER: u32 = 7;
pub const PRODUCT_STANDARD_SERVER_CORE: u32 = 13;
pub const PRODUCT_STANDARD_SERVER_CORE_V: u32 = 40;
pub const PRODUCT_STANDARD_SERVER_SOLUTIONS: u32 = 52;
pub const PRODUCT_STANDARD_SERVER_SOLUTIONS_CORE: u32 = 53;
pub const PRODUCT_STANDARD_SERVER_V: u32 = 36;
pub const PRODUCT_STANDARD_WS_SERVER_CORE: u32 = 148;
pub const PRODUCT_STARTER: u32 = 11;
pub const PRODUCT_STARTER_E: u32 = 66;
pub const PRODUCT_STARTER_N: u32 = 47;
pub const PRODUCT_STORAGE_ENTERPRISE_SERVER: u32 = 23;
pub const PRODUCT_STORAGE_ENTERPRISE_SERVER_CORE: u32 = 46;
pub const PRODUCT_STORAGE_EXPRESS_SERVER: u32 = 20;
pub const PRODUCT_STORAGE_EXPRESS_SERVER_CORE: u32 = 43;
pub const PRODUCT_STORAGE_STANDARD_EVALUATION_SERVER: u32 = 96;
pub const PRODUCT_STORAGE_STANDARD_SERVER: u32 = 21;
pub const PRODUCT_STORAGE_STANDARD_SERVER_CORE: u32 = 44;
pub const PRODUCT_STORAGE_WORKGROUP_EVALUATION_SERVER: u32 = 95;
pub const PRODUCT_STORAGE_WORKGROUP_SERVER: u32 = 22;
pub const PRODUCT_STORAGE_WORKGROUP_SERVER_CORE: u32 = 45;
pub const PRODUCT_THINPC: u32 = 87;
pub const PRODUCT_ULTIMATE: u32 = 1;
pub const PRODUCT_ULTIMATE_E: u32 = 71;
pub const PRODUCT_ULTIMATE_N: u32 = 28;
pub const PRODUCT_UNDEFINED: u32 = 0;
pub const PRODUCT_UNLICENSED: u32 = 2882382797;
pub const PRODUCT_UTILITY_VM: u32 = 149;
pub const PRODUCT_VALIDATION: u32 = 204;
pub const PRODUCT_WEB_SERVER: u32 = 17;
pub const PRODUCT_WEB_SERVER_CORE: u32 = 29;
pub const PRODUCT_WNC: u32 = 210;
pub const PRODUCT_XBOX_DURANGOHOSTOS: u32 = 196;
pub const PRODUCT_XBOX_ERAOS: u32 = 195;
pub const PRODUCT_XBOX_GAMEOS: u32 = 194;
pub const PRODUCT_XBOX_KEYSTONE: u32 = 198;
pub const PRODUCT_XBOX_SCARLETTHOSTOS: u32 = 197;
pub const PRODUCT_XBOX_SYSTEMOS: u32 = 192;
pub const PROTECTED_DACL_SECURITY_INFORMATION: u32 = 2147483648;
pub const PROTECTED_SACL_SECURITY_INFORMATION: u32 = 1073741824;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRTL_BARRIER(pub *mut RTL_BARRIER);
impl PRTL_BARRIER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRTL_BARRIER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRTL_CONDITION_VARIABLE(pub *mut RTL_CONDITION_VARIABLE);
impl PRTL_CONDITION_VARIABLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRTL_CONDITION_VARIABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRTL_CRITICAL_SECTION(pub *mut RTL_CRITICAL_SECTION);
impl PRTL_CRITICAL_SECTION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRTL_CRITICAL_SECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRTL_CRITICAL_SECTION_DEBUG(pub *mut RTL_CRITICAL_SECTION_DEBUG);
impl PRTL_CRITICAL_SECTION_DEBUG {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRTL_CRITICAL_SECTION_DEBUG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRTL_OSVERSIONINFOEXW(pub *mut OSVERSIONINFOEXW);
impl PRTL_OSVERSIONINFOEXW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRTL_OSVERSIONINFOEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRTL_OSVERSIONINFOW(pub *mut OSVERSIONINFOW);
impl PRTL_OSVERSIONINFOW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRTL_OSVERSIONINFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRTL_REFERENCE_COUNT(pub *mut isize);
impl PRTL_REFERENCE_COUNT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRTL_REFERENCE_COUNT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRTL_REFERENCE_COUNT32(pub *mut i32);
impl PRTL_REFERENCE_COUNT32 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRTL_REFERENCE_COUNT32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRTL_RESOURCE_DEBUG(pub *mut RTL_CRITICAL_SECTION_DEBUG);
impl PRTL_RESOURCE_DEBUG {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRTL_RESOURCE_DEBUG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRTL_RUN_ONCE(pub *mut RTL_RUN_ONCE);
impl PRTL_RUN_ONCE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRTL_RUN_ONCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRTL_SRWLOCK(pub *mut RTL_SRWLOCK);
impl PRTL_SRWLOCK {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRTL_SRWLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRTL_SYSTEM_GLOBAL_DATA_ID(pub *mut RTL_SYSTEM_GLOBAL_DATA_ID);
impl PRTL_SYSTEM_GLOBAL_DATA_ID {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRTL_SYSTEM_GLOBAL_DATA_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PRTL_UMS_SCHEDULER_ENTRY_POINT = Option<unsafe extern "system" fn(reason: RTL_UMS_SCHEDULER_REASON, activationpayload: usize, schedulerparam: *const core::ffi::c_void)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRTL_UMS_SCHEDULER_REASON(pub *mut RTL_UMS_SCHEDULER_REASON);
impl PRTL_UMS_SCHEDULER_REASON {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRTL_UMS_SCHEDULER_REASON {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRTL_UMS_THREAD_INFO_CLASS(pub *mut RTL_UMS_THREAD_INFO_CLASS);
impl PRTL_UMS_THREAD_INFO_CLASS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRTL_UMS_THREAD_INFO_CLASS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRUNTIME_FUNCTION(pub *mut RUNTIME_FUNCTION);
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl PRUNTIME_FUNCTION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for PRUNTIME_FUNCTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "aarch64")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRUNTIME_FUNCTION(pub *mut ARM64_RUNTIME_FUNCTION);
#[cfg(target_arch = "aarch64")]
impl PRUNTIME_FUNCTION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(target_arch = "aarch64")]
impl Default for PRUNTIME_FUNCTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRUNTIME_REPORT_DIGEST_HEADER(pub *mut RUNTIME_REPORT_DIGEST_HEADER);
impl PRUNTIME_REPORT_DIGEST_HEADER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRUNTIME_REPORT_DIGEST_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRUNTIME_REPORT_HEADER(pub *mut RUNTIME_REPORT_HEADER);
impl PRUNTIME_REPORT_HEADER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRUNTIME_REPORT_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRUNTIME_REPORT_PACKAGE_HEADER(pub *mut RUNTIME_REPORT_PACKAGE_HEADER);
impl PRUNTIME_REPORT_PACKAGE_HEADER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRUNTIME_REPORT_PACKAGE_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSCOPE_TABLE(pub *mut SCOPE_TABLE_AMD64);
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl PSCOPE_TABLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for PSCOPE_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "aarch64")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSCOPE_TABLE(pub *mut SCOPE_TABLE_ARM64);
#[cfg(target_arch = "aarch64")]
impl PSCOPE_TABLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(target_arch = "aarch64")]
impl Default for PSCOPE_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSCOPE_TABLE_AMD64(pub *mut SCOPE_TABLE_AMD64);
impl PSCOPE_TABLE_AMD64 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSCOPE_TABLE_AMD64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSCOPE_TABLE_ARM(pub *mut SCOPE_TABLE_ARM);
impl PSCOPE_TABLE_ARM {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSCOPE_TABLE_ARM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSCOPE_TABLE_ARM64(pub *mut SCOPE_TABLE_ARM64);
impl PSCOPE_TABLE_ARM64 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSCOPE_TABLE_ARM64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSCRUB_DATA_INPUT(pub *mut SCRUB_DATA_INPUT);
impl PSCRUB_DATA_INPUT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSCRUB_DATA_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSCRUB_DATA_OUTPUT(pub *mut SCRUB_DATA_OUTPUT);
impl PSCRUB_DATA_OUTPUT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSCRUB_DATA_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSCRUB_PARITY_EXTENT(pub *mut SCRUB_PARITY_EXTENT);
impl PSCRUB_PARITY_EXTENT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSCRUB_PARITY_EXTENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSCRUB_PARITY_EXTENT_DATA(pub *mut SCRUB_PARITY_EXTENT_DATA);
impl PSCRUB_PARITY_EXTENT_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSCRUB_PARITY_EXTENT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PSECURE_MEMORY_CACHE_CALLBACK = Option<unsafe extern "system" fn(addr: *const core::ffi::c_void, range: usize) -> bool>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSECURITY_ATTRIBUTES_OPAQUE(pub *mut core::ffi::c_void);
impl PSECURITY_ATTRIBUTES_OPAQUE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSECURITY_ATTRIBUTES_OPAQUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSECURITY_CAPABILITIES(pub *mut SECURITY_CAPABILITIES);
impl PSECURITY_CAPABILITIES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSECURITY_CAPABILITIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSECURITY_CONTEXT_TRACKING_MODE(pub *mut bool);
impl PSECURITY_CONTEXT_TRACKING_MODE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSECURITY_CONTEXT_TRACKING_MODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSECURITY_DESCRIPTOR(pub *mut core::ffi::c_void);
impl PSECURITY_DESCRIPTOR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSECURITY_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSECURITY_DESCRIPTOR_CONTROL(pub *mut u16);
impl PSECURITY_DESCRIPTOR_CONTROL {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSECURITY_DESCRIPTOR_CONTROL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSECURITY_IMPERSONATION_LEVEL(pub *mut SECURITY_IMPERSONATION_LEVEL);
impl PSECURITY_IMPERSONATION_LEVEL {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSECURITY_IMPERSONATION_LEVEL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSECURITY_INFORMATION(pub *mut u32);
impl PSECURITY_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSECURITY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSECURITY_OBJECT_AI_PARAMS(pub *mut SECURITY_OBJECT_AI_PARAMS);
impl PSECURITY_OBJECT_AI_PARAMS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSECURITY_OBJECT_AI_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSECURITY_QUALITY_OF_SERVICE(pub *mut SECURITY_QUALITY_OF_SERVICE);
impl PSECURITY_QUALITY_OF_SERVICE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSECURITY_QUALITY_OF_SERVICE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVERSILO_BASIC_INFORMATION(pub *mut SERVERSILO_BASIC_INFORMATION);
impl PSERVERSILO_BASIC_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVERSILO_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVERSILO_DIAGNOSTIC_INFORMATION(pub *mut SERVERSILO_DIAGNOSTIC_INFORMATION);
impl PSERVERSILO_DIAGNOSTIC_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVERSILO_DIAGNOSTIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSERVERSILO_STATE(pub *mut SERVERSILO_STATE);
impl PSERVERSILO_STATE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSERVERSILO_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSET_POWER_SETTING_VALUE(pub *mut SET_POWER_SETTING_VALUE);
impl PSET_POWER_SETTING_VALUE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSET_POWER_SETTING_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSE_ACCESS_REPLY(pub *mut SE_ACCESS_REPLY);
#[cfg(feature = "minwindef")]
impl PSE_ACCESS_REPLY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwindef")]
impl Default for PSE_ACCESS_REPLY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSE_ACCESS_REQUEST(pub *mut SE_ACCESS_REQUEST);
impl PSE_ACCESS_REQUEST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSE_ACCESS_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSE_IMAGE_SIGNATURE_TYPE(pub *mut SE_IMAGE_SIGNATURE_TYPE);
impl PSE_IMAGE_SIGNATURE_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSE_IMAGE_SIGNATURE_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSE_IMPERSONATION_STATE(pub *mut SE_IMPERSONATION_STATE);
impl PSE_IMPERSONATION_STATE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSE_IMPERSONATION_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSE_SECURITY_DESCRIPTOR(pub *mut SE_SECURITY_DESCRIPTOR);
impl PSE_SECURITY_DESCRIPTOR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSE_SECURITY_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSE_SID(pub *mut SE_SID);
impl PSE_SID {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSE_SID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSE_SIGNING_LEVEL(pub *mut u8);
impl PSE_SIGNING_LEVEL {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSE_SIGNING_LEVEL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PSE_TOKEN_USER = SE_TOKEN_USER;
#[cfg(feature = "basetsd")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSHARED_COMPUTE_UNIT_RELATIONSHIP(pub *mut SHARED_COMPUTE_UNIT_RELATIONSHIP);
#[cfg(feature = "basetsd")]
impl PSHARED_COMPUTE_UNIT_RELATIONSHIP {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "basetsd")]
impl Default for PSHARED_COMPUTE_UNIT_RELATIONSHIP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSHARED_VIRTUAL_DISK_SUPPORT(pub *mut SHARED_VIRTUAL_DISK_SUPPORT);
impl PSHARED_VIRTUAL_DISK_SUPPORT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSHARED_VIRTUAL_DISK_SUPPORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSHORT(pub *mut i16);
impl PSHORT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSHORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSHUFFLE_FILE_DATA(pub *mut SHUFFLE_FILE_DATA);
impl PSHUFFLE_FILE_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSHUFFLE_FILE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSID(pub *mut core::ffi::c_void);
impl PSID {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSID_AND_ATTRIBUTES(pub *mut SID_AND_ATTRIBUTES);
impl PSID_AND_ATTRIBUTES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSID_AND_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSID_AND_ATTRIBUTES_ARRAY(pub *mut SID_AND_ATTRIBUTES_ARRAY);
impl PSID_AND_ATTRIBUTES_ARRAY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSID_AND_ATTRIBUTES_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSID_AND_ATTRIBUTES_HASH(pub *mut SID_AND_ATTRIBUTES_HASH);
impl PSID_AND_ATTRIBUTES_HASH {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSID_AND_ATTRIBUTES_HASH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSID_HASH_ENTRY(pub *mut usize);
impl PSID_HASH_ENTRY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSID_HASH_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSID_IDENTIFIER_AUTHORITY(pub *mut SID_IDENTIFIER_AUTHORITY);
impl PSID_IDENTIFIER_AUTHORITY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSID_IDENTIFIER_AUTHORITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSID_NAME_USE(pub *mut SID_NAME_USE);
impl PSID_NAME_USE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSID_NAME_USE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSILOOBJECT_BASIC_INFORMATION(pub *mut SILOOBJECT_BASIC_INFORMATION);
impl PSILOOBJECT_BASIC_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSILOOBJECT_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSINGLE_LIST_ENTRY(pub *mut SINGLE_LIST_ENTRY);
impl PSINGLE_LIST_ENTRY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSINGLE_LIST_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSLIST_ENTRY(pub *mut SINGLE_LIST_ENTRY);
#[cfg(target_arch = "x86")]
impl PSLIST_ENTRY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(target_arch = "x86")]
impl Default for PSLIST_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSLIST_ENTRY(pub *mut SLIST_ENTRY);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl PSLIST_ENTRY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for PSLIST_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSLIST_HEADER(pub *mut SLIST_HEADER);
impl PSLIST_HEADER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSLIST_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSUPPORTED_OS_INFO(pub *mut SUPPORTED_OS_INFO);
impl PSUPPORTED_OS_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSUPPORTED_OS_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSYSTEM_ACCESS_FILTER_ACE(pub *mut SYSTEM_ACCESS_FILTER_ACE);
impl PSYSTEM_ACCESS_FILTER_ACE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSYSTEM_ACCESS_FILTER_ACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSYSTEM_ALARM_ACE(pub *mut SYSTEM_ALARM_ACE);
impl PSYSTEM_ALARM_ACE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSYSTEM_ALARM_ACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSYSTEM_ALARM_CALLBACK_ACE(pub *mut SYSTEM_ALARM_CALLBACK_ACE);
impl PSYSTEM_ALARM_CALLBACK_ACE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSYSTEM_ALARM_CALLBACK_ACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSYSTEM_ALARM_CALLBACK_OBJECT_ACE(pub *mut SYSTEM_ALARM_CALLBACK_OBJECT_ACE);
impl PSYSTEM_ALARM_CALLBACK_OBJECT_ACE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSYSTEM_ALARM_CALLBACK_OBJECT_ACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSYSTEM_ALARM_OBJECT_ACE(pub *mut SYSTEM_ALARM_OBJECT_ACE);
impl PSYSTEM_ALARM_OBJECT_ACE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSYSTEM_ALARM_OBJECT_ACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSYSTEM_AUDIT_ACE(pub *mut SYSTEM_AUDIT_ACE);
impl PSYSTEM_AUDIT_ACE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSYSTEM_AUDIT_ACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSYSTEM_AUDIT_CALLBACK_ACE(pub *mut SYSTEM_AUDIT_CALLBACK_ACE);
impl PSYSTEM_AUDIT_CALLBACK_ACE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSYSTEM_AUDIT_CALLBACK_ACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSYSTEM_AUDIT_CALLBACK_OBJECT_ACE(pub *mut SYSTEM_AUDIT_CALLBACK_OBJECT_ACE);
impl PSYSTEM_AUDIT_CALLBACK_OBJECT_ACE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSYSTEM_AUDIT_CALLBACK_OBJECT_ACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSYSTEM_AUDIT_OBJECT_ACE(pub *mut SYSTEM_AUDIT_OBJECT_ACE);
impl PSYSTEM_AUDIT_OBJECT_ACE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSYSTEM_AUDIT_OBJECT_ACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSYSTEM_BATTERY_STATE(pub *mut SYSTEM_BATTERY_STATE);
impl PSYSTEM_BATTERY_STATE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSYSTEM_BATTERY_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSYSTEM_CPU_SET_INFORMATION(pub *mut SYSTEM_CPU_SET_INFORMATION);
impl PSYSTEM_CPU_SET_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSYSTEM_CPU_SET_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSYSTEM_LOGICAL_PROCESSOR_INFORMATION(pub *mut SYSTEM_LOGICAL_PROCESSOR_INFORMATION);
impl PSYSTEM_LOGICAL_PROCESSOR_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSYSTEM_LOGICAL_PROCESSOR_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "basetsd")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX(pub *mut SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX);
#[cfg(feature = "basetsd")]
impl PSYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "basetsd")]
impl Default for PSYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSYSTEM_MANDATORY_LABEL_ACE(pub *mut SYSTEM_MANDATORY_LABEL_ACE);
impl PSYSTEM_MANDATORY_LABEL_ACE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSYSTEM_MANDATORY_LABEL_ACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSYSTEM_POOL_ZEROING_INFORMATION(pub *mut SYSTEM_POOL_ZEROING_INFORMATION);
impl PSYSTEM_POOL_ZEROING_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSYSTEM_POOL_ZEROING_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSYSTEM_POWER_CAPABILITIES(pub *mut SYSTEM_POWER_CAPABILITIES);
impl PSYSTEM_POWER_CAPABILITIES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSYSTEM_POWER_CAPABILITIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSYSTEM_POWER_LEVEL(pub *mut SYSTEM_POWER_LEVEL);
impl PSYSTEM_POWER_LEVEL {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSYSTEM_POWER_LEVEL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSYSTEM_POWER_POLICY(pub *mut SYSTEM_POWER_POLICY);
impl PSYSTEM_POWER_POLICY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSYSTEM_POWER_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSYSTEM_POWER_SOURCE_STATE(pub *mut SYSTEM_POWER_SOURCE_STATE);
impl PSYSTEM_POWER_SOURCE_STATE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSYSTEM_POWER_SOURCE_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSYSTEM_POWER_STATE(pub *mut SYSTEM_POWER_STATE);
impl PSYSTEM_POWER_STATE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSYSTEM_POWER_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION(pub *mut SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION);
impl PSYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSYSTEM_PROCESS_TRUST_LABEL_ACE(pub *mut SYSTEM_PROCESS_TRUST_LABEL_ACE);
impl PSYSTEM_PROCESS_TRUST_LABEL_ACE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSYSTEM_PROCESS_TRUST_LABEL_ACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSYSTEM_RESOURCE_ATTRIBUTE_ACE(pub *mut SYSTEM_RESOURCE_ATTRIBUTE_ACE);
impl PSYSTEM_RESOURCE_ATTRIBUTE_ACE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSYSTEM_RESOURCE_ATTRIBUTE_ACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSYSTEM_SCOPED_POLICY_ID_ACE(pub *mut SYSTEM_SCOPED_POLICY_ID_ACE);
impl PSYSTEM_SCOPED_POLICY_ID_ACE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSYSTEM_SCOPED_POLICY_ID_ACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTAPE_CREATE_PARTITION(pub *mut TAPE_CREATE_PARTITION);
impl PTAPE_CREATE_PARTITION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTAPE_CREATE_PARTITION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTAPE_ERASE(pub *mut TAPE_ERASE);
impl PTAPE_ERASE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTAPE_ERASE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTAPE_GET_DRIVE_PARAMETERS(pub *mut TAPE_GET_DRIVE_PARAMETERS);
impl PTAPE_GET_DRIVE_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTAPE_GET_DRIVE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTAPE_GET_MEDIA_PARAMETERS(pub *mut TAPE_GET_MEDIA_PARAMETERS);
impl PTAPE_GET_MEDIA_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTAPE_GET_MEDIA_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTAPE_GET_POSITION(pub *mut TAPE_GET_POSITION);
impl PTAPE_GET_POSITION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTAPE_GET_POSITION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTAPE_PREPARE(pub *mut TAPE_PREPARE);
impl PTAPE_PREPARE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTAPE_PREPARE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTAPE_SET_DRIVE_PARAMETERS(pub *mut TAPE_SET_DRIVE_PARAMETERS);
impl PTAPE_SET_DRIVE_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTAPE_SET_DRIVE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTAPE_SET_MEDIA_PARAMETERS(pub *mut TAPE_SET_MEDIA_PARAMETERS);
impl PTAPE_SET_MEDIA_PARAMETERS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTAPE_SET_MEDIA_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTAPE_SET_POSITION(pub *mut TAPE_SET_POSITION);
impl PTAPE_SET_POSITION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTAPE_SET_POSITION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTAPE_WMI_OPERATIONS(pub *mut TAPE_WMI_OPERATIONS);
impl PTAPE_WMI_OPERATIONS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTAPE_WMI_OPERATIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTAPE_WRITE_MARKS(pub *mut TAPE_WRITE_MARKS);
impl PTAPE_WRITE_MARKS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTAPE_WRITE_MARKS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTBYTE(pub *mut u8);
impl PTBYTE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTBYTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PTCH(pub LPCH);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTCHAR(pub *mut i8);
impl PTCHAR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTCHAR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub type PTERMINATION_HANDLER = Option<unsafe extern "system" fn(_abnormal_termination: bool, establisherframe: *mut core::ffi::c_void)>;
#[cfg(target_arch = "aarch64")]
pub type PTERMINATION_HANDLER = Option<unsafe extern "system" fn(_abnormal_termination: bool, establisherframe: u64)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTOKEN_ACCESS_INFORMATION(pub *mut TOKEN_ACCESS_INFORMATION);
impl PTOKEN_ACCESS_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTOKEN_ACCESS_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTOKEN_APPCONTAINER_INFORMATION(pub *mut TOKEN_APPCONTAINER_INFORMATION);
impl PTOKEN_APPCONTAINER_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTOKEN_APPCONTAINER_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTOKEN_AUDIT_POLICY(pub *mut TOKEN_AUDIT_POLICY);
impl PTOKEN_AUDIT_POLICY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTOKEN_AUDIT_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTOKEN_BNO_ISOLATION_INFORMATION(pub *mut TOKEN_BNO_ISOLATION_INFORMATION);
impl PTOKEN_BNO_ISOLATION_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTOKEN_BNO_ISOLATION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTOKEN_CONTROL(pub *mut TOKEN_CONTROL);
impl PTOKEN_CONTROL {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTOKEN_CONTROL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTOKEN_DEFAULT_DACL(pub *mut TOKEN_DEFAULT_DACL);
impl PTOKEN_DEFAULT_DACL {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTOKEN_DEFAULT_DACL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTOKEN_DEVICE_CLAIMS(pub *mut TOKEN_DEVICE_CLAIMS);
impl PTOKEN_DEVICE_CLAIMS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTOKEN_DEVICE_CLAIMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTOKEN_ELEVATION(pub *mut TOKEN_ELEVATION);
impl PTOKEN_ELEVATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTOKEN_ELEVATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTOKEN_ELEVATION_TYPE(pub *mut TOKEN_ELEVATION_TYPE);
impl PTOKEN_ELEVATION_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTOKEN_ELEVATION_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTOKEN_GROUPS(pub *mut TOKEN_GROUPS);
impl PTOKEN_GROUPS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTOKEN_GROUPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTOKEN_GROUPS_AND_PRIVILEGES(pub *mut TOKEN_GROUPS_AND_PRIVILEGES);
impl PTOKEN_GROUPS_AND_PRIVILEGES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTOKEN_GROUPS_AND_PRIVILEGES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTOKEN_INFORMATION_CLASS(pub *mut TOKEN_INFORMATION_CLASS);
impl PTOKEN_INFORMATION_CLASS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTOKEN_INFORMATION_CLASS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTOKEN_LINKED_TOKEN(pub *mut TOKEN_LINKED_TOKEN);
impl PTOKEN_LINKED_TOKEN {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTOKEN_LINKED_TOKEN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTOKEN_LOGGING_INFORMATION(pub *mut TOKEN_LOGGING_INFORMATION);
impl PTOKEN_LOGGING_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTOKEN_LOGGING_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTOKEN_MANDATORY_LABEL(pub *mut TOKEN_MANDATORY_LABEL);
impl PTOKEN_MANDATORY_LABEL {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTOKEN_MANDATORY_LABEL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTOKEN_MANDATORY_POLICY(pub *mut TOKEN_MANDATORY_POLICY);
impl PTOKEN_MANDATORY_POLICY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTOKEN_MANDATORY_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTOKEN_ORIGIN(pub *mut TOKEN_ORIGIN);
impl PTOKEN_ORIGIN {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTOKEN_ORIGIN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTOKEN_OWNER(pub *mut TOKEN_OWNER);
impl PTOKEN_OWNER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTOKEN_OWNER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTOKEN_PRIMARY_GROUP(pub *mut TOKEN_PRIMARY_GROUP);
impl PTOKEN_PRIMARY_GROUP {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTOKEN_PRIMARY_GROUP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTOKEN_PRIVILEGES(pub *mut TOKEN_PRIVILEGES);
impl PTOKEN_PRIVILEGES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTOKEN_PRIVILEGES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTOKEN_SID_INFORMATION(pub *mut TOKEN_SID_INFORMATION);
impl PTOKEN_SID_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTOKEN_SID_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTOKEN_SOURCE(pub *mut TOKEN_SOURCE);
impl PTOKEN_SOURCE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTOKEN_SOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTOKEN_STATISTICS(pub *mut TOKEN_STATISTICS);
impl PTOKEN_STATISTICS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTOKEN_STATISTICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTOKEN_TYPE(pub *mut TOKEN_TYPE);
impl PTOKEN_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTOKEN_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTOKEN_USER(pub *mut TOKEN_USER);
impl PTOKEN_USER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTOKEN_USER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTOKEN_USER_CLAIMS(pub *mut TOKEN_USER_CLAIMS);
impl PTOKEN_USER_CLAIMS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTOKEN_USER_CLAIMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTP_CALLBACK_ENVIRON(pub *mut TP_CALLBACK_ENVIRON_V3);
impl PTP_CALLBACK_ENVIRON {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTP_CALLBACK_ENVIRON {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTP_CALLBACK_INSTANCE(pub *mut TP_CALLBACK_INSTANCE);
impl PTP_CALLBACK_INSTANCE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTP_CALLBACK_INSTANCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTP_CLEANUP_GROUP(pub *mut TP_CLEANUP_GROUP);
impl PTP_CLEANUP_GROUP {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTP_CLEANUP_GROUP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PTP_CLEANUP_GROUP_CANCEL_CALLBACK = Option<unsafe extern "system" fn(objectcontext: *mut core::ffi::c_void, cleanupcontext: *mut core::ffi::c_void)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTP_IO(pub *mut TP_IO);
impl PTP_IO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTP_IO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTP_POOL(pub *mut TP_POOL);
impl PTP_POOL {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTP_POOL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTP_POOL_STACK_INFORMATION(pub *mut TP_POOL_STACK_INFORMATION);
impl PTP_POOL_STACK_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTP_POOL_STACK_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PTP_SIMPLE_CALLBACK = Option<unsafe extern "system" fn(instance: *mut TP_CALLBACK_INSTANCE, context: *mut core::ffi::c_void)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTP_TIMER(pub *mut TP_TIMER);
impl PTP_TIMER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTP_TIMER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PTP_TIMER_CALLBACK = Option<unsafe extern "system" fn(instance: *mut TP_CALLBACK_INSTANCE, context: *mut core::ffi::c_void, timer: *mut TP_TIMER)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTP_VERSION(pub *mut u32);
impl PTP_VERSION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTP_VERSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTP_WAIT(pub *mut TP_WAIT);
impl PTP_WAIT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTP_WAIT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PTP_WAIT_CALLBACK = Option<unsafe extern "system" fn(instance: *mut TP_CALLBACK_INSTANCE, context: *mut core::ffi::c_void, wait: *mut TP_WAIT, waitresult: TP_WAIT_RESULT)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTP_WORK(pub *mut TP_WORK);
impl PTP_WORK {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTP_WORK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PTP_WORK_CALLBACK = Option<unsafe extern "system" fn(instance: *mut TP_CALLBACK_INSTANCE, context: *mut core::ffi::c_void, work: *mut TP_WORK)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTRANSACTIONMANAGER_BASIC_INFORMATION(pub *mut TRANSACTIONMANAGER_BASIC_INFORMATION);
impl PTRANSACTIONMANAGER_BASIC_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTRANSACTIONMANAGER_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTRANSACTIONMANAGER_LOGPATH_INFORMATION(pub *mut TRANSACTIONMANAGER_LOGPATH_INFORMATION);
impl PTRANSACTIONMANAGER_LOGPATH_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTRANSACTIONMANAGER_LOGPATH_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTRANSACTIONMANAGER_LOG_INFORMATION(pub *mut TRANSACTIONMANAGER_LOG_INFORMATION);
impl PTRANSACTIONMANAGER_LOG_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTRANSACTIONMANAGER_LOG_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTRANSACTIONMANAGER_OLDEST_INFORMATION(pub *mut TRANSACTIONMANAGER_OLDEST_INFORMATION);
impl PTRANSACTIONMANAGER_OLDEST_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTRANSACTIONMANAGER_OLDEST_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTRANSACTIONMANAGER_RECOVERY_INFORMATION(pub *mut TRANSACTIONMANAGER_RECOVERY_INFORMATION);
impl PTRANSACTIONMANAGER_RECOVERY_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTRANSACTIONMANAGER_RECOVERY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTRANSACTION_BASIC_INFORMATION(pub *mut TRANSACTION_BASIC_INFORMATION);
impl PTRANSACTION_BASIC_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTRANSACTION_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTRANSACTION_BIND_INFORMATION(pub *mut TRANSACTION_BIND_INFORMATION);
impl PTRANSACTION_BIND_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTRANSACTION_BIND_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTRANSACTION_ENLISTMENTS_INFORMATION(pub *mut TRANSACTION_ENLISTMENTS_INFORMATION);
impl PTRANSACTION_ENLISTMENTS_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTRANSACTION_ENLISTMENTS_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTRANSACTION_ENLISTMENT_PAIR(pub *mut TRANSACTION_ENLISTMENT_PAIR);
impl PTRANSACTION_ENLISTMENT_PAIR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTRANSACTION_ENLISTMENT_PAIR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "ktmtypes")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTRANSACTION_LIST_ENTRY(pub *mut TRANSACTION_LIST_ENTRY);
#[cfg(feature = "ktmtypes")]
impl PTRANSACTION_LIST_ENTRY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "ktmtypes")]
impl Default for PTRANSACTION_LIST_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "ktmtypes")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTRANSACTION_LIST_INFORMATION(pub *mut TRANSACTION_LIST_INFORMATION);
#[cfg(feature = "ktmtypes")]
impl PTRANSACTION_LIST_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "ktmtypes")]
impl Default for PTRANSACTION_LIST_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTRANSACTION_PROPERTIES_INFORMATION(pub *mut TRANSACTION_PROPERTIES_INFORMATION);
impl PTRANSACTION_PROPERTIES_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTRANSACTION_PROPERTIES_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PTRANSACTION_SUPERIOR_ENLISTMENT_INFORMATION(pub *mut TRANSACTION_SUPERIOR_ENLISTMENT_INFORMATION);
impl PTRANSACTION_SUPERIOR_ENLISTMENT_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PTRANSACTION_SUPERIOR_ENLISTMENT_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PTSTR = windows_core::PSTR;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUCSCHAR(pub *mut UCSCHAR);
impl PUCSCHAR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUCSCHAR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUCSSTR(pub *mut UCSCHAR);
impl PUCSSTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUCSSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PULARGE_INTEGER(pub *mut u64);
impl PULARGE_INTEGER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PULARGE_INTEGER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PULONGLONG(pub *mut u64);
impl PULONGLONG {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PULONGLONG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUMS_CREATE_THREAD_ATTRIBUTES(pub *mut UMS_CREATE_THREAD_ATTRIBUTES);
impl PUMS_CREATE_THREAD_ATTRIBUTES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUMS_CREATE_THREAD_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUNWIND_HISTORY_TABLE(pub *mut UNWIND_HISTORY_TABLE);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl PUNWIND_HISTORY_TABLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for PUNWIND_HISTORY_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUNWIND_HISTORY_TABLE_ENTRY(pub *mut UNWIND_HISTORY_TABLE_ENTRY);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl PUNWIND_HISTORY_TABLE_ENTRY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for PUNWIND_HISTORY_TABLE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PUNZTCH(pub PNZCH);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUNZWCH(pub *mut u16);
impl PUNZWCH {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUNZWCH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUSER_ACTIVITY_PRESENCE(pub *mut USER_ACTIVITY_PRESENCE);
impl PUSER_ACTIVITY_PRESENCE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUSER_ACTIVITY_PRESENCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PUTSTR = windows_core::PSTR;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUUCSCHAR(pub *mut UCSCHAR);
impl PUUCSCHAR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUUCSCHAR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUUCSSTR(pub *mut UCSCHAR);
impl PUUCSSTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUUCSSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUWSTR(pub *mut u16);
impl PUWSTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUWSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PUZZTSTR(pub PZZSTR);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PUZZWSTR(pub *mut u16);
impl PUZZWSTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PUZZWSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PVECTORED_EXCEPTION_HANDLER = Option<unsafe extern "system" fn(exceptioninfo: *mut EXCEPTION_POINTERS) -> i32>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWCH(pub *mut u16);
impl PWCH {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PWCH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWCHAR(pub *mut u16);
impl PWCHAR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PWCHAR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWOW64_CONTEXT(pub *mut WOW64_CONTEXT);
impl PWOW64_CONTEXT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PWOW64_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWOW64_DESCRIPTOR_TABLE_ENTRY(pub *mut WOW64_DESCRIPTOR_TABLE_ENTRY);
impl PWOW64_DESCRIPTOR_TABLE_ENTRY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PWOW64_DESCRIPTOR_TABLE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWOW64_FLOATING_SAVE_AREA(pub *mut WOW64_FLOATING_SAVE_AREA);
impl PWOW64_FLOATING_SAVE_AREA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PWOW64_FLOATING_SAVE_AREA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWOW64_LDT_ENTRY(pub *mut WOW64_LDT_ENTRY);
impl PWOW64_LDT_ENTRY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PWOW64_LDT_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PXMM_SAVE_AREA32(pub *mut XSAVE_FORMAT);
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl PXMM_SAVE_AREA32 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for PXMM_SAVE_AREA32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PXSAVE_AREA(pub *mut XSAVE_AREA);
impl PXSAVE_AREA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PXSAVE_AREA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PXSAVE_AREA_HEADER(pub *mut XSAVE_AREA_HEADER);
impl PXSAVE_AREA_HEADER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PXSAVE_AREA_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PXSAVE_ARM64_SME_TPIDR2_HEADER(pub *mut XSAVE_ARM64_SME_TPIDR2_HEADER);
impl PXSAVE_ARM64_SME_TPIDR2_HEADER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PXSAVE_ARM64_SME_TPIDR2_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PXSAVE_ARM64_SME_ZA_HEADER(pub *mut XSAVE_ARM64_SME_ZA_HEADER);
impl PXSAVE_ARM64_SME_ZA_HEADER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PXSAVE_ARM64_SME_ZA_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PXSAVE_ARM64_SME_ZT_HEADER(pub *mut XSAVE_ARM64_SME_ZT_HEADER);
impl PXSAVE_ARM64_SME_ZT_HEADER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PXSAVE_ARM64_SME_ZT_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PXSAVE_ARM64_SVE_HEADER(pub *mut XSAVE_ARM64_SVE_HEADER);
impl PXSAVE_ARM64_SVE_HEADER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PXSAVE_ARM64_SVE_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PXSAVE_CET_U_FORMAT(pub *mut XSAVE_CET_U_FORMAT);
impl PXSAVE_CET_U_FORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PXSAVE_CET_U_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PXSAVE_FORMAT(pub *mut XSAVE_FORMAT);
impl PXSAVE_FORMAT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PXSAVE_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PXSTATE_CONFIGURATION(pub *mut XSTATE_CONFIGURATION);
impl PXSTATE_CONFIGURATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PXSTATE_CONFIGURATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PXSTATE_CONTEXT(pub *mut XSTATE_CONTEXT);
impl PXSTATE_CONTEXT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PXSTATE_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PXSTATE_FEATURE(pub *mut XSTATE_FEATURE);
impl PXSTATE_FEATURE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PXSTATE_FEATURE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PZPCSTR(pub *mut windows_core::PCSTR);
impl PZPCSTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PZPCSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PZPCWSTR(pub *mut windows_core::PCWSTR);
impl PZPCWSTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PZPCWSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PZPSTR(pub *mut windows_core::PSTR);
impl PZPSTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PZPSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PZPTSTR(pub PZPSTR);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PZPWSTR(pub *mut windows_core::PWSTR);
impl PZPWSTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PZPWSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PZZSTR(pub *mut i8);
impl PZZSTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PZZSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PZZTSTR(pub PZZSTR);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PZZWSTR(pub *mut u16);
impl PZZWSTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PZZWSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
pub const PcTeb: u32 = 24;
pub const PdataCrChained: ARM64_FNPDATA_CR = 3;
pub const PdataCrChainedWithPac: ARM64_FNPDATA_CR = 2;
pub const PdataCrUnchained: ARM64_FNPDATA_CR = 0;
pub const PdataCrUnchainedSavedLr: ARM64_FNPDATA_CR = 1;
pub const PdataPackedUnwindFragment: ARM64_FNPDATA_FLAGS = 2;
pub const PdataPackedUnwindFunction: ARM64_FNPDATA_FLAGS = 1;
pub const PdataRefToFullXdata: ARM64_FNPDATA_FLAGS = 0;
pub const PdcInvocation: POWER_INFORMATION_LEVEL = 67;
pub const PhysicalPowerButtonPress: POWER_INFORMATION_LEVEL = 90;
pub const PlatformIdleStates: POWER_INFORMATION_LEVEL = 80;
pub const PlatformIdleVeto: POWER_INFORMATION_LEVEL = 82;
pub const PlatformInformation: POWER_INFORMATION_LEVEL = 66;
pub const PlatformRole: POWER_INFORMATION_LEVEL = 75;
pub const PlatformRoleAppliancePC: POWER_PLATFORM_ROLE = 6;
pub const PlatformRoleDesktop: POWER_PLATFORM_ROLE = 1;
pub const PlatformRoleEnterpriseServer: POWER_PLATFORM_ROLE = 4;
pub const PlatformRoleMaximum: POWER_PLATFORM_ROLE = 9;
pub const PlatformRoleMobile: POWER_PLATFORM_ROLE = 2;
pub const PlatformRolePerformanceServer: POWER_PLATFORM_ROLE = 7;
pub const PlatformRoleSOHOServer: POWER_PLATFORM_ROLE = 5;
pub const PlatformRoleSlate: POWER_PLATFORM_ROLE = 8;
pub const PlatformRoleUnspecified: POWER_PLATFORM_ROLE = 0;
pub const PlatformRoleWorkstation: POWER_PLATFORM_ROLE = 3;
pub const PlmPowerRequestCreate: POWER_INFORMATION_LEVEL = 72;
pub const PoAc: SYSTEM_POWER_CONDITION = 0;
pub const PoConditionMaximum: SYSTEM_POWER_CONDITION = 3;
pub const PoDc: SYSTEM_POWER_CONDITION = 1;
pub const PoHot: SYSTEM_POWER_CONDITION = 2;
pub const PowerActionDisplayOff: POWER_ACTION = 8;
pub const PowerActionHibernate: POWER_ACTION = 3;
pub const PowerActionNone: POWER_ACTION = 0;
pub const PowerActionReserved: POWER_ACTION = 1;
pub const PowerActionShutdown: POWER_ACTION = 4;
pub const PowerActionShutdownOff: POWER_ACTION = 6;
pub const PowerActionShutdownReset: POWER_ACTION = 5;
pub const PowerActionSleep: POWER_ACTION = 2;
pub const PowerActionWarmEject: POWER_ACTION = 7;
pub const PowerDeviceD0: DEVICE_POWER_STATE = 1;
pub const PowerDeviceD1: DEVICE_POWER_STATE = 2;
pub const PowerDeviceD2: DEVICE_POWER_STATE = 3;
pub const PowerDeviceD3: DEVICE_POWER_STATE = 4;
pub const PowerDeviceMaximum: DEVICE_POWER_STATE = 5;
pub const PowerDeviceUnspecified: DEVICE_POWER_STATE = 0;
pub const PowerInformationInternal: POWER_INFORMATION_LEVEL = 87;
pub const PowerInformationLevelMaximum: POWER_INFORMATION_LEVEL = 99;
pub const PowerInformationLevelUnused0: POWER_INFORMATION_LEVEL = 27;
pub const PowerLimitBurst: POWER_LIMIT_TYPES = 1;
pub const PowerLimitContinuous: POWER_LIMIT_TYPES = 0;
pub const PowerLimitPreemptive: POWER_LIMIT_TYPES = 3;
pub const PowerLimitPreemptiveOffset: POWER_LIMIT_TYPES = 4;
pub const PowerLimitRapid: POWER_LIMIT_TYPES = 2;
pub const PowerLimitType1: POWER_LIMIT_TYPES = 0;
pub const PowerLimitType2: POWER_LIMIT_TYPES = 1;
pub const PowerLimitType3: POWER_LIMIT_TYPES = 2;
pub const PowerLimitType4: POWER_LIMIT_TYPES = 3;
pub const PowerLimitTypeMax: POWER_LIMIT_TYPES = 5;
pub const PowerMonitorDim: MONITOR_DISPLAY_STATE = 2;
pub const PowerMonitorOff: MONITOR_DISPLAY_STATE = 0;
pub const PowerMonitorOn: MONITOR_DISPLAY_STATE = 1;
pub const PowerRequestAction: POWER_INFORMATION_LEVEL = 44;
pub const PowerRequestActionInternal: POWER_INFORMATION_LEVEL = 85;
pub const PowerRequestAwayModeRequired: POWER_REQUEST_TYPE = 2;
pub const PowerRequestCreate: POWER_INFORMATION_LEVEL = 43;
pub const PowerRequestDisplayRequired: POWER_REQUEST_TYPE = 0;
pub const PowerRequestExecutionRequired: POWER_REQUEST_TYPE = 3;
pub const PowerRequestSystemRequired: POWER_REQUEST_TYPE = 1;
pub const PowerSettingNotificationName: POWER_INFORMATION_LEVEL = 58;
pub const PowerShutdownNotification: POWER_INFORMATION_LEVEL = 39;
pub const PowerSystemHibernate: SYSTEM_POWER_STATE = 5;
pub const PowerSystemMaximum: SYSTEM_POWER_STATE = 7;
pub const PowerSystemShutdown: SYSTEM_POWER_STATE = 6;
pub const PowerSystemSleeping1: SYSTEM_POWER_STATE = 2;
pub const PowerSystemSleeping2: SYSTEM_POWER_STATE = 3;
pub const PowerSystemSleeping3: SYSTEM_POWER_STATE = 4;
pub const PowerSystemUnspecified: SYSTEM_POWER_STATE = 0;
pub const PowerSystemWorking: SYSTEM_POWER_STATE = 1;
pub const PowerUserInactive: USER_ACTIVITY_PRESENCE = 2;
pub const PowerUserInvalid: USER_ACTIVITY_PRESENCE = 3;
pub const PowerUserMaximum: USER_ACTIVITY_PRESENCE = 3;
pub const PowerUserNotPresent: USER_ACTIVITY_PRESENCE = 1;
pub const PowerUserPresent: USER_ACTIVITY_PRESENCE = 0;
pub const ProcessASLRPolicy: PROCESS_MITIGATION_POLICY = 1;
pub const ProcessActivationContextTrustPolicy: PROCESS_MITIGATION_POLICY = 19;
pub const ProcessChildProcessPolicy: PROCESS_MITIGATION_POLICY = 13;
pub const ProcessControlFlowGuardPolicy: PROCESS_MITIGATION_POLICY = 7;
pub const ProcessDEPPolicy: PROCESS_MITIGATION_POLICY = 0;
pub const ProcessDynamicCodePolicy: PROCESS_MITIGATION_POLICY = 2;
pub const ProcessExtensionPointDisablePolicy: PROCESS_MITIGATION_POLICY = 6;
pub const ProcessFontDisablePolicy: PROCESS_MITIGATION_POLICY = 9;
pub const ProcessImageLoadPolicy: PROCESS_MITIGATION_POLICY = 10;
pub const ProcessMitigationOptionsMask: PROCESS_MITIGATION_POLICY = 5;
pub const ProcessPayloadRestrictionPolicy: PROCESS_MITIGATION_POLICY = 12;
pub const ProcessRedirectionTrustPolicy: PROCESS_MITIGATION_POLICY = 16;
pub const ProcessSEHOPPolicy: PROCESS_MITIGATION_POLICY = 18;
pub const ProcessSideChannelIsolationPolicy: PROCESS_MITIGATION_POLICY = 14;
pub const ProcessSignaturePolicy: PROCESS_MITIGATION_POLICY = 8;
pub const ProcessStrictHandleCheckPolicy: PROCESS_MITIGATION_POLICY = 3;
pub const ProcessSystemCallDisablePolicy: PROCESS_MITIGATION_POLICY = 4;
pub const ProcessSystemCallFilterPolicy: PROCESS_MITIGATION_POLICY = 11;
pub const ProcessUserPointerAuthPolicy: PROCESS_MITIGATION_POLICY = 17;
pub const ProcessUserShadowStackPolicy: PROCESS_MITIGATION_POLICY = 15;
pub const ProcessorCap: POWER_INFORMATION_LEVEL = 34;
pub const ProcessorIdleDomains: POWER_INFORMATION_LEVEL = 49;
pub const ProcessorIdleStates: POWER_INFORMATION_LEVEL = 33;
pub const ProcessorIdleStatesHv: POWER_INFORMATION_LEVEL = 52;
pub const ProcessorIdleVeto: POWER_INFORMATION_LEVEL = 81;
pub const ProcessorInformation: POWER_INFORMATION_LEVEL = 11;
pub const ProcessorInformationEx: POWER_INFORMATION_LEVEL = 46;
pub const ProcessorLoad: POWER_INFORMATION_LEVEL = 38;
pub const ProcessorPerfCapHv: POWER_INFORMATION_LEVEL = 54;
pub const ProcessorPerfStates: POWER_INFORMATION_LEVEL = 32;
pub const ProcessorPerfStatesHv: POWER_INFORMATION_LEVEL = 53;
pub const ProcessorPowerPolicyAc: POWER_INFORMATION_LEVEL = 18;
pub const ProcessorPowerPolicyCurrent: POWER_INFORMATION_LEVEL = 22;
pub const ProcessorPowerPolicyDc: POWER_INFORMATION_LEVEL = 19;
pub const ProcessorSetIdle: POWER_INFORMATION_LEVEL = 55;
pub const ProcessorStateHandler: POWER_INFORMATION_LEVEL = 7;
pub const ProcessorStateHandler2: POWER_INFORMATION_LEVEL = 13;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct QUOTA_LIMITS {
    pub PagedPoolLimit: usize,
    pub NonPagedPoolLimit: usize,
    pub MinimumWorkingSetSize: usize,
    pub MaximumWorkingSetSize: usize,
    pub PagefileLimit: usize,
    pub TimeLimit: i64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct QUOTA_LIMITS_EX {
    pub PagedPoolLimit: usize,
    pub NonPagedPoolLimit: usize,
    pub MinimumWorkingSetSize: usize,
    pub MaximumWorkingSetSize: usize,
    pub PagefileLimit: usize,
    pub TimeLimit: i64,
    pub WorkingSetLimit: usize,
    pub Reserved2: usize,
    pub Reserved3: usize,
    pub Reserved4: usize,
    pub Flags: u32,
    pub CpuRateLimit: RATE_QUOTA_LIMIT,
}
impl Default for QUOTA_LIMITS_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const QUOTA_LIMITS_HARDWS_MAX_DISABLE: u32 = 8;
pub const QUOTA_LIMITS_HARDWS_MAX_ENABLE: u32 = 4;
pub const QUOTA_LIMITS_HARDWS_MIN_DISABLE: u32 = 2;
pub const QUOTA_LIMITS_HARDWS_MIN_ENABLE: u32 = 1;
pub const QUOTA_LIMITS_USE_DEFAULT_LIMITS: u32 = 16;
pub const QueryPotentialDripsConstraint: POWER_INFORMATION_LEVEL = 91;
#[repr(C)]
#[derive(Clone, Copy)]
pub union RATE_QUOTA_LIMIT {
    pub RateData: u32,
    pub Anonymous: RATE_QUOTA_LIMIT_0,
}
impl Default for RATE_QUOTA_LIMIT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RATE_QUOTA_LIMIT_0 {
    pub _bitfield: u32,
}
pub const READ_CONTROL: u32 = 131072;
pub const READ_THREAD_PROFILING_FLAG_DISPATCHING: u32 = 1;
pub const READ_THREAD_PROFILING_FLAG_HARDWARE_COUNTERS: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct REARRANGE_FILE_DATA {
    pub SourceStartingOffset: u64,
    pub TargetOffset: u64,
    pub SourceFileHandle: HANDLE,
    pub Length: u32,
    pub Flags: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct REARRANGE_FILE_DATA32 {
    pub SourceStartingOffset: u64,
    pub TargetOffset: u64,
    pub SourceFileHandle: u32,
    pub Length: u32,
    pub Flags: u32,
}
pub const REG_APP_HIVE: u32 = 16;
pub const REG_APP_HIVE_OPEN_READ_ONLY: u32 = 8192;
pub const REG_BINARY: u32 = 3;
pub const REG_BOOT_HIVE: u32 = 1024;
pub const REG_CREATED_NEW_KEY: u32 = 1;
pub const REG_DWORD: u32 = 4;
pub const REG_DWORD_BIG_ENDIAN: u32 = 5;
pub const REG_DWORD_LITTLE_ENDIAN: u32 = 4;
pub const REG_EXPAND_SZ: u32 = 2;
pub const REG_FLUSH_HIVE_FILE_GROWTH: u32 = 4096;
pub const REG_FORCE_RESTORE: u32 = 8;
pub const REG_FORCE_UNLOAD: u32 = 1;
pub const REG_FULL_RESOURCE_DESCRIPTOR: u32 = 9;
pub const REG_HIVE_EXACT_FILE_GROWTH: u32 = 128;
pub const REG_HIVE_NO_RM: u32 = 256;
pub const REG_HIVE_SINGLE_LOG: u32 = 512;
pub const REG_IMMUTABLE: u32 = 16384;
pub const REG_LATEST_FORMAT: u32 = 2;
pub const REG_LEGAL_CHANGE_FILTER: u32 = 268435471;
pub const REG_LEGAL_OPTION: u32 = 31;
pub const REG_LINK: u32 = 6;
pub const REG_LOAD_HIVE_OPEN_HANDLE: u32 = 2048;
pub const REG_MULTI_SZ: u32 = 7;
pub const REG_NONE: u32 = 0;
pub const REG_NOTIFY_CHANGE_ATTRIBUTES: u32 = 2;
pub const REG_NOTIFY_CHANGE_LAST_SET: u32 = 4;
pub const REG_NOTIFY_CHANGE_NAME: u32 = 1;
pub const REG_NOTIFY_CHANGE_SECURITY: u32 = 8;
pub const REG_NOTIFY_THREAD_AGNOSTIC: u32 = 268435456;
pub const REG_NO_COMPRESSION: u32 = 4;
pub const REG_NO_IMPERSONATION_FALLBACK: u32 = 32768;
pub const REG_NO_LAZY_FLUSH: u32 = 4;
pub const REG_OPENED_EXISTING_KEY: u32 = 2;
pub const REG_OPEN_LEGAL_OPTION: u32 = 28;
pub const REG_OPEN_READ_ONLY: u32 = 8192;
pub const REG_OPTION_BACKUP_RESTORE: u32 = 4;
pub const REG_OPTION_CREATE_LINK: u32 = 2;
pub const REG_OPTION_DONT_VIRTUALIZE: u32 = 16;
pub const REG_OPTION_NON_VOLATILE: u32 = 0;
pub const REG_OPTION_OPEN_LINK: u32 = 8;
pub const REG_OPTION_RESERVED: u32 = 0;
pub const REG_OPTION_VOLATILE: u32 = 1;
pub const REG_PROCESS_PRIVATE: u32 = 32;
pub const REG_QWORD: u32 = 11;
pub const REG_QWORD_LITTLE_ENDIAN: u32 = 11;
pub const REG_REFRESH_HIVE: u32 = 2;
pub const REG_RESOURCE_LIST: u32 = 8;
pub const REG_RESOURCE_REQUIREMENTS_LIST: u32 = 10;
pub const REG_STANDARD_FORMAT: u32 = 1;
pub const REG_START_JOURNAL: u32 = 64;
pub const REG_SZ: u32 = 1;
pub const REG_UNLOAD_LEGAL_FLAGS: u32 = 1;
pub const REG_WHOLE_HIVE_VOLATILE: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct REPARSE_GUID_DATA_BUFFER {
    pub ReparseTag: u32,
    pub ReparseDataLength: u16,
    pub Reserved: u16,
    pub ReparseGuid: windows_core::GUID,
    pub GenericReparseBuffer: REPARSE_GUID_DATA_BUFFER_0,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REPARSE_GUID_DATA_BUFFER_0 {
    pub DataBuffer: [u8; 1],
}
impl Default for REPARSE_GUID_DATA_BUFFER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const REPARSE_GUID_DATA_BUFFER_HEADER_SIZE: u32 = 24;
pub const RESOURCEMANAGER_ALL_ACCESS: u32 = 2031743;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RESOURCEMANAGER_BASIC_INFORMATION {
    pub ResourceManagerId: windows_core::GUID,
    pub DescriptionLength: u32,
    pub Description: [u16; 1],
}
impl Default for RESOURCEMANAGER_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RESOURCEMANAGER_COMPLETE_PROPAGATION: u32 = 64;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RESOURCEMANAGER_COMPLETION_INFORMATION {
    pub IoCompletionPortHandle: HANDLE,
    pub CompletionKey: usize,
}
pub const RESOURCEMANAGER_ENLIST: u32 = 8;
pub const RESOURCEMANAGER_GENERIC_EXECUTE: u32 = 1179740;
pub const RESOURCEMANAGER_GENERIC_READ: u32 = 1179649;
pub const RESOURCEMANAGER_GENERIC_WRITE: u32 = 1179774;
pub const RESOURCEMANAGER_GET_NOTIFICATION: u32 = 16;
pub type RESOURCEMANAGER_INFORMATION_CLASS = i32;
pub const RESOURCEMANAGER_QUERY_INFORMATION: u32 = 1;
pub const RESOURCEMANAGER_RECOVER: u32 = 4;
pub const RESOURCEMANAGER_REGISTER_PROTOCOL: u32 = 32;
pub const RESOURCEMANAGER_SET_INFORMATION: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RESUME_PERFORMANCE {
    pub PostTimeMs: u32,
    pub TotalResumeTimeMs: u64,
    pub ResumeCompleteTimestamp: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RTL_BARRIER {
    pub Reserved1: u32,
    pub Reserved2: u32,
    pub Reserved3: [usize; 2],
    pub Reserved4: u32,
    pub Reserved5: u32,
}
impl Default for RTL_BARRIER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RTL_CONDITION_VARIABLE {
    pub Ptr: *mut core::ffi::c_void,
}
impl Default for RTL_CONDITION_VARIABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RTL_CONDITION_VARIABLE_LOCKMODE_SHARED: u32 = 1;
pub const RTL_CORRELATION_VECTOR_STRING_LENGTH: u32 = 129;
pub const RTL_CORRELATION_VECTOR_V1_LENGTH: u32 = 64;
pub const RTL_CORRELATION_VECTOR_V1_PREFIX_LENGTH: u32 = 16;
pub const RTL_CORRELATION_VECTOR_V2_LENGTH: u32 = 128;
pub const RTL_CORRELATION_VECTOR_V2_PREFIX_LENGTH: u32 = 22;
pub const RTL_CORRELATION_VECTOR_VERSION_1: i8 = 1;
pub const RTL_CORRELATION_VECTOR_VERSION_2: i8 = 2;
pub const RTL_CORRELATION_VECTOR_VERSION_CURRENT: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RTL_CRITICAL_SECTION {
    pub DebugInfo: PRTL_CRITICAL_SECTION_DEBUG,
    pub LockCount: i32,
    pub RecursionCount: i32,
    pub OwningThread: HANDLE,
    pub LockSemaphore: HANDLE,
    pub SpinCount: usize,
}
pub const RTL_CRITICAL_SECTION_ALL_FLAG_BITS: u32 = 4278190080;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RTL_CRITICAL_SECTION_DEBUG {
    pub Type: u16,
    pub CreatorBackTraceIndex: u16,
    pub CriticalSection: *mut RTL_CRITICAL_SECTION,
    pub ProcessLocksList: LIST_ENTRY,
    pub EntryCount: u32,
    pub ContentionCount: u32,
    pub Flags: u32,
    pub CreatorBackTraceIndexHigh: u16,
    pub Identifier: u16,
}
impl Default for RTL_CRITICAL_SECTION_DEBUG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RTL_CRITICAL_SECTION_DEBUG_FLAG_STATIC_INIT: u32 = 1;
pub const RTL_CRITICAL_SECTION_FLAG_DYNAMIC_SPIN: u32 = 33554432;
pub const RTL_CRITICAL_SECTION_FLAG_FORCE_DEBUG_INFO: u32 = 268435456;
pub const RTL_CRITICAL_SECTION_FLAG_NO_DEBUG_INFO: u32 = 16777216;
pub const RTL_CRITICAL_SECTION_FLAG_RESERVED: i32 = -536870912;
pub const RTL_CRITICAL_SECTION_FLAG_RESOURCE_TYPE: u32 = 134217728;
pub const RTL_CRITICAL_SECTION_FLAG_STATIC_INIT: u32 = 67108864;
pub type RTL_OSVERSIONINFOEXW = OSVERSIONINFOEXW;
pub type RTL_OSVERSIONINFOW = OSVERSIONINFOW;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct RTL_REFERENCE_COUNT(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct RTL_REFERENCE_COUNT32(pub i32);
pub type RTL_RESOURCE_DEBUG = RTL_CRITICAL_SECTION_DEBUG;
#[repr(C)]
#[derive(Clone, Copy)]
pub union RTL_RUN_ONCE {
    pub Ptr: *mut core::ffi::c_void,
}
impl Default for RTL_RUN_ONCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RTL_RUN_ONCE_ASYNC: u32 = 2;
pub const RTL_RUN_ONCE_CHECK_ONLY: u32 = 1;
pub const RTL_RUN_ONCE_CTX_RESERVED_BITS: u32 = 2;
pub const RTL_RUN_ONCE_INIT_FAILED: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RTL_SRWLOCK {
    pub Ptr: *mut core::ffi::c_void,
}
impl Default for RTL_SRWLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type RTL_SYSTEM_GLOBAL_DATA_ID = i32;
pub type RTL_UMS_SCHEDULER_ENTRY_POINT = Option<unsafe extern "system" fn(reason: RTL_UMS_SCHEDULER_REASON, activationpayload: usize, schedulerparam: *const core::ffi::c_void)>;
pub type RTL_UMS_SCHEDULER_REASON = i32;
pub type RTL_UMS_THREAD_INFO_CLASS = i32;
pub const RTL_UMS_VERSION: u32 = 256;
pub const RTL_VIRTUAL_UNWIND2_VALIDATE_PAC: u32 = 1;
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub struct RUNTIME_FUNCTION {
    pub BeginAddress: u32,
    pub EndAddress: u32,
    pub Anonymous: RUNTIME_FUNCTION_0,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for RUNTIME_FUNCTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union RUNTIME_FUNCTION_0 {
    pub UnwindInfoAddress: u32,
    pub UnwindData: u32,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for RUNTIME_FUNCTION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "aarch64")]
pub type RUNTIME_FUNCTION = ARM64_RUNTIME_FUNCTION;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const RUNTIME_FUNCTION_INDIRECT: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RUNTIME_REPORT_DIGEST_HEADER {
    pub ReportType: u16,
    pub Reserved: u16,
    pub ReportDigest: [u8; 64],
}
impl Default for RUNTIME_REPORT_DIGEST_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RUNTIME_REPORT_DIGEST_MAX_SIZE: u32 = 64;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RUNTIME_REPORT_HEADER {
    pub ReportType: u16,
    pub Reserved: u16,
    pub ReportSize: u32,
}
pub const RUNTIME_REPORT_NONCE_SIZE: u32 = 32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RUNTIME_REPORT_PACKAGE_HEADER {
    pub Magic: u32,
    pub PackageVersion: u16,
    pub NumberOfReports: u16,
    pub ReportTypesBitmap: u64,
    pub PackageSize: u32,
    pub ReportDigestType: u16,
    pub TotalReportDigestsSize: u16,
    pub Reserved: u16,
    pub SignatureScheme: u16,
    pub SignatureSize: u32,
    pub TotalAuthenticatedReportsSize: u32,
}
pub const RUNTIME_REPORT_PACKAGE_MAGIC: u32 = 1381257808;
pub const RUNTIME_REPORT_PACKAGE_VERSION_CURRENT: u32 = 1;
pub const RUNTIME_REPORT_SIGNATURE_SCHEME_SHA512_RSA_PSS_SHA512: u32 = 1;
pub type RUNTIME_REPORT_TYPE = i32;
pub const RUNTIME_REPORT_TYPE_MASK_ALL: u32 = 3;
pub const RecognizerType: SERVICE_NODE_TYPE = 8;
pub const RegisterSpmPowerSettings: POWER_INFORMATION_LEVEL = 79;
pub const RelationAll: LOGICAL_PROCESSOR_RELATIONSHIP = 65535;
pub const RelationCache: LOGICAL_PROCESSOR_RELATIONSHIP = 2;
pub const RelationGroup: LOGICAL_PROCESSOR_RELATIONSHIP = 4;
pub const RelationNumaNode: LOGICAL_PROCESSOR_RELATIONSHIP = 1;
pub const RelationNumaNodeEx: LOGICAL_PROCESSOR_RELATIONSHIP = 6;
pub const RelationProcessorCore: LOGICAL_PROCESSOR_RELATIONSHIP = 0;
pub const RelationProcessorDie: LOGICAL_PROCESSOR_RELATIONSHIP = 5;
pub const RelationProcessorModule: LOGICAL_PROCESSOR_RELATIONSHIP = 7;
pub const RelationProcessorPackage: LOGICAL_PROCESSOR_RELATIONSHIP = 3;
pub const RelationProcessorSharedComputeUnit: LOGICAL_PROCESSOR_RELATIONSHIP = 8;
pub type ReplacesCorHdrNumericDefines = i32;
pub const ResourceManagerBasicInformation: RESOURCEMANAGER_INFORMATION_CLASS = 0;
pub const ResourceManagerCompletionInformation: RESOURCEMANAGER_INFORMATION_CLASS = 1;
pub const RunlevelInformationInActivationContext: ACTIVATION_CONTEXT_INFO_CLASS = 5;
pub const RuntimeReportTypeCodeIntegrity: RUNTIME_REPORT_TYPE = 1;
pub const RuntimeReportTypeDriver: RUNTIME_REPORT_TYPE = 0;
pub const RuntimeReportTypeMax: RUNTIME_REPORT_TYPE = 2;
pub const SACL_SECURITY_INFORMATION: u32 = 8;
pub const SANDBOX_INERT: u32 = 2;
pub const SCOPE_SECURITY_INFORMATION: u32 = 64;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub type SCOPE_TABLE = SCOPE_TABLE_AMD64;
#[cfg(target_arch = "aarch64")]
pub type SCOPE_TABLE = SCOPE_TABLE_ARM64;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCOPE_TABLE_AMD64 {
    pub Count: u32,
    pub ScopeRecord: [SCOPE_TABLE_AMD64_0; 1],
}
impl Default for SCOPE_TABLE_AMD64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCOPE_TABLE_AMD64_0 {
    pub BeginAddress: u32,
    pub EndAddress: u32,
    pub HandlerAddress: u32,
    pub JumpTarget: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCOPE_TABLE_ARM {
    pub Count: u32,
    pub ScopeRecord: [SCOPE_TABLE_ARM_0; 1],
}
impl Default for SCOPE_TABLE_ARM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCOPE_TABLE_ARM64 {
    pub Count: u32,
    pub ScopeRecord: [SCOPE_TABLE_ARM64_0; 1],
}
impl Default for SCOPE_TABLE_ARM64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCOPE_TABLE_ARM64_0 {
    pub BeginAddress: u32,
    pub EndAddress: u32,
    pub HandlerAddress: u32,
    pub JumpTarget: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCOPE_TABLE_ARM_0 {
    pub BeginAddress: u32,
    pub EndAddress: u32,
    pub HandlerAddress: u32,
    pub JumpTarget: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCRUB_DATA_INPUT {
    pub Size: u32,
    pub Flags: u32,
    pub MaximumIos: u32,
    pub ObjectId: [u32; 4],
    pub StartingByteOffset: u64,
    pub ByteCount: u64,
    pub Reserved: [u32; 36],
    pub ResumeContext: [u8; 1040],
}
impl Default for SCRUB_DATA_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SCRUB_DATA_INPUT_FLAG_IGNORE_REDUNDANCY: u32 = 8;
pub const SCRUB_DATA_INPUT_FLAG_OPLOCK_NOT_ACQUIRED: u32 = 64;
pub const SCRUB_DATA_INPUT_FLAG_RESUME: u32 = 1;
pub const SCRUB_DATA_INPUT_FLAG_SCRUB_BY_OBJECT_ID: u32 = 32;
pub const SCRUB_DATA_INPUT_FLAG_SKIP_DATA: u32 = 16;
pub const SCRUB_DATA_INPUT_FLAG_SKIP_IN_SYNC: u32 = 2;
pub const SCRUB_DATA_INPUT_FLAG_SKIP_NON_INTEGRITY_DATA: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCRUB_DATA_OUTPUT {
    pub Size: u32,
    pub Flags: u32,
    pub Status: u32,
    pub ErrorFileOffset: u64,
    pub ErrorLength: u64,
    pub NumberOfBytesRepaired: u64,
    pub NumberOfBytesFailed: u64,
    pub InternalFileReference: u64,
    pub ResumeContextLength: u16,
    pub ParityExtentDataOffset: u16,
    pub NextStartingByteOffset: u64,
    pub ValidDataLength: u64,
    pub Reserved: [u32; 4],
    pub NumberOfMetadataBytesProcessed: u64,
    pub NumberOfDataBytesProcessed: u64,
    pub TotalNumberOfMetadataBytesInUse: u64,
    pub TotalNumberOfDataBytesInUse: u64,
    pub DataBytesSkippedDueToNoAllocation: u64,
    pub DataBytesSkippedDueToInvalidRun: u64,
    pub DataBytesSkippedDueToIntegrityStream: u64,
    pub DataBytesSkippedDueToRegionBeingClean: u64,
    pub DataBytesSkippedDueToLockConflict: u64,
    pub DataBytesSkippedDueToNoScrubDataFlag: u64,
    pub DataBytesSkippedDueToNoScrubNonIntegrityStreamFlag: u64,
    pub DataBytesScrubbed: u64,
    pub ResumeContext: [u8; 1040],
}
impl Default for SCRUB_DATA_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SCRUB_DATA_OUTPUT_FLAG_INCOMPLETE: u32 = 1;
pub const SCRUB_DATA_OUTPUT_FLAG_NON_USER_DATA_RANGE: u32 = 65536;
pub const SCRUB_DATA_OUTPUT_FLAG_PARITY_EXTENT_DATA_RETURNED: u32 = 131072;
pub const SCRUB_DATA_OUTPUT_FLAG_RESUME_CONTEXT_LENGTH_SPECIFIED: u32 = 262144;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCRUB_PARITY_EXTENT {
    pub Offset: i64,
    pub Length: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCRUB_PARITY_EXTENT_DATA {
    pub Size: u16,
    pub Flags: u16,
    pub NumberOfParityExtents: u16,
    pub MaximumNumberOfParityExtents: u16,
    pub ParityExtents: [SCRUB_PARITY_EXTENT; 1],
}
impl Default for SCRUB_PARITY_EXTENT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SECTION_ALL_ACCESS: u32 = 983071;
pub const SECTION_EXTEND_SIZE: u32 = 16;
pub const SECTION_MAP_EXECUTE: u32 = 8;
pub const SECTION_MAP_EXECUTE_EXPLICIT: u32 = 32;
pub const SECTION_MAP_READ: u32 = 4;
pub const SECTION_MAP_WRITE: u32 = 2;
pub const SECTION_QUERY: u32 = 1;
pub const SECURITY_AGENTIC_PLATFORM_BASE_RID: u32 = 101;
pub const SECURITY_ANONYMOUS_LOGON_RID: u32 = 7;
pub const SECURITY_APPPOOL_ID_BASE_RID: u32 = 82;
pub const SECURITY_APPPOOL_ID_RID_COUNT: u32 = 6;
pub const SECURITY_APP_PACKAGE_BASE_RID: u32 = 2;
pub const SECURITY_APP_PACKAGE_RID_COUNT: u32 = 8;
pub const SECURITY_AUTHENTICATED_USER_RID: u32 = 11;
pub const SECURITY_AUTHENTICATION_AUTHORITY_ASSERTED_RID: u32 = 1;
pub const SECURITY_AUTHENTICATION_AUTHORITY_RID_COUNT: u32 = 1;
pub const SECURITY_AUTHENTICATION_FRESH_KEY_AUTH_RID: u32 = 3;
pub const SECURITY_AUTHENTICATION_KEY_PROPERTY_ATTESTATION_RID: u32 = 6;
pub const SECURITY_AUTHENTICATION_KEY_PROPERTY_MFA_RID: u32 = 5;
pub const SECURITY_AUTHENTICATION_KEY_TRUST_RID: u32 = 4;
pub const SECURITY_AUTHENTICATION_SERVICE_ASSERTED_RID: u32 = 2;
pub const SECURITY_BATCH_RID: u32 = 3;
pub const SECURITY_BUILTIN_APP_PACKAGE_RID_COUNT: u32 = 2;
pub const SECURITY_BUILTIN_CAPABILITY_RID_COUNT: u32 = 2;
pub const SECURITY_BUILTIN_DOMAIN_RID: u32 = 32;
pub const SECURITY_BUILTIN_PACKAGE_ANY_PACKAGE: u32 = 1;
pub const SECURITY_BUILTIN_PACKAGE_ANY_RESTRICTED_PACKAGE: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SECURITY_CAPABILITIES {
    pub AppContainerSid: PSID,
    pub Capabilities: PSID_AND_ATTRIBUTES,
    pub CapabilityCount: u32,
    pub Reserved: u32,
}
pub const SECURITY_CAPABILITY_APPOINTMENTS: u32 = 11;
pub const SECURITY_CAPABILITY_APP_RID: u32 = 1024;
pub const SECURITY_CAPABILITY_APP_SILO_RID: u32 = 65536;
pub const SECURITY_CAPABILITY_BASE_RID: u32 = 3;
pub const SECURITY_CAPABILITY_CONTACTS: u32 = 12;
pub const SECURITY_CAPABILITY_DOCUMENTS_LIBRARY: u32 = 7;
pub const SECURITY_CAPABILITY_ENTERPRISE_AUTHENTICATION: u32 = 8;
pub const SECURITY_CAPABILITY_INTERNET_CLIENT: u32 = 1;
pub const SECURITY_CAPABILITY_INTERNET_CLIENT_SERVER: u32 = 2;
pub const SECURITY_CAPABILITY_INTERNET_EXPLORER: u32 = 4096;
pub const SECURITY_CAPABILITY_MUSIC_LIBRARY: u32 = 6;
pub const SECURITY_CAPABILITY_PICTURES_LIBRARY: u32 = 4;
pub const SECURITY_CAPABILITY_PRIVATE_NETWORK_CLIENT_SERVER: u32 = 3;
pub const SECURITY_CAPABILITY_REMOVABLE_STORAGE: u32 = 10;
pub const SECURITY_CAPABILITY_RID_COUNT: u32 = 5;
pub const SECURITY_CAPABILITY_SHARED_USER_CERTIFICATES: u32 = 9;
pub const SECURITY_CAPABILITY_VIDEOS_LIBRARY: u32 = 5;
pub const SECURITY_CCG_ID_BASE_RID: u32 = 95;
pub const SECURITY_CHILD_PACKAGE_RID_COUNT: u32 = 12;
pub const SECURITY_CLOUD_INFRASTRUCTURE_SERVICES_ID_BASE_RID: u32 = 85;
pub const SECURITY_CLOUD_INFRASTRUCTURE_SERVICES_ID_RID_COUNT: u32 = 6;
pub const SECURITY_COM_ID_BASE_RID: u32 = 89;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SECURITY_CONTEXT_TRACKING_MODE(pub bool);
pub const SECURITY_CREATOR_GROUP_RID: u32 = 1;
pub const SECURITY_CREATOR_GROUP_SERVER_RID: u32 = 3;
pub const SECURITY_CREATOR_OWNER_RID: u32 = 0;
pub const SECURITY_CREATOR_OWNER_RIGHTS_RID: u32 = 4;
pub const SECURITY_CREATOR_OWNER_SERVER_RID: u32 = 2;
pub const SECURITY_CRED_TYPE_BASE_RID: u32 = 65;
pub const SECURITY_CRED_TYPE_RID_COUNT: u32 = 2;
pub const SECURITY_CRED_TYPE_THIS_ORG_CERT_RID: u32 = 1;
pub const SECURITY_DASHOST_ID_BASE_RID: u32 = 92;
pub const SECURITY_DASHOST_ID_RID_COUNT: u32 = 6;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SECURITY_DESCRIPTOR {
    pub Revision: u8,
    pub Sbz1: u8,
    pub Control: SECURITY_DESCRIPTOR_CONTROL,
    pub Owner: PSID,
    pub Group: PSID,
    pub Sacl: PACL,
    pub Dacl: PACL,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SECURITY_DESCRIPTOR_CONTROL(pub u16);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SECURITY_DESCRIPTOR_RELATIVE {
    pub Revision: u8,
    pub Sbz1: u8,
    pub Control: SECURITY_DESCRIPTOR_CONTROL,
    pub Owner: u32,
    pub Group: u32,
    pub Sacl: u32,
    pub Dacl: u32,
}
pub const SECURITY_DESCRIPTOR_REVISION: u32 = 1;
pub const SECURITY_DESCRIPTOR_REVISION1: u32 = 1;
pub const SECURITY_DIALUP_RID: u32 = 1;
pub const SECURITY_DYNAMIC_TRACKING: u32 = 1;
pub const SECURITY_EDGE_CLOUD_INFRASTRUCTURE_SERVICE_ID_BASE_RID: u32 = 98;
pub const SECURITY_ENTERPRISE_CONTROLLERS_RID: u32 = 9;
pub const SECURITY_ENTERPRISE_READONLY_CONTROLLERS_RID: u32 = 22;
pub type SECURITY_IMPERSONATION_LEVEL = i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SECURITY_INFORMATION(pub u32);
pub const SECURITY_INSTALLER_CAPABILITY_RID_COUNT: u32 = 10;
pub const SECURITY_INSTALLER_GROUP_CAPABILITY_BASE: u32 = 32;
pub const SECURITY_INSTALLER_GROUP_CAPABILITY_RID_COUNT: u32 = 9;
pub const SECURITY_INTERACTIVE_RID: u32 = 4;
pub const SECURITY_IUSER_RID: u32 = 17;
pub const SECURITY_LOCAL_ACCOUNT_AND_ADMIN_RID: u32 = 114;
pub const SECURITY_LOCAL_ACCOUNT_RID: u32 = 113;
pub const SECURITY_LOCAL_LOGON_RID: u32 = 1;
pub const SECURITY_LOCAL_RID: u32 = 0;
pub const SECURITY_LOCAL_SERVICE_RID: u32 = 19;
pub const SECURITY_LOCAL_SYSTEM_RID: u32 = 18;
pub const SECURITY_LOGON_IDS_RID: u32 = 5;
pub const SECURITY_LOGON_IDS_RID_COUNT: u32 = 3;
pub const SECURITY_MANDATORY_HIGH_RID: u32 = 12288;
pub const SECURITY_MANDATORY_LOW_RID: u32 = 4096;
pub const SECURITY_MANDATORY_MAXIMUM_USER_RID: u32 = 16384;
pub const SECURITY_MANDATORY_MEDIUM_PLUS_CREDUI_RID: u32 = 8202;
pub const SECURITY_MANDATORY_MEDIUM_PLUS_RID: u32 = 8448;
pub const SECURITY_MANDATORY_MEDIUM_RID: u32 = 8192;
pub const SECURITY_MANDATORY_PROTECTED_PROCESS_RID: u32 = 20480;
pub const SECURITY_MANDATORY_SYSTEM_RID: u32 = 16384;
pub const SECURITY_MANDATORY_UNTRUSTED_RID: u32 = 0;
pub const SECURITY_MAX_ALWAYS_FILTERED: u32 = 999;
pub const SECURITY_MAX_BASE_RID: u32 = 111;
pub const SECURITY_MAX_IMPERSONATION_LEVEL: u32 = 3;
pub const SECURITY_MAX_SID_STRING_CHARACTERS: u32 = 187;
pub const SECURITY_MIN_BASE_RID: u32 = 80;
pub const SECURITY_MIN_IMPERSONATION_LEVEL: u32 = 0;
pub const SECURITY_MIN_NEVER_FILTERED: u32 = 1000;
pub const SECURITY_NETWORK_RID: u32 = 2;
pub const SECURITY_NETWORK_SERVICE_RID: u32 = 20;
pub const SECURITY_NFS_ID_BASE_RID: u32 = 88;
pub const SECURITY_NT_NON_UNIQUE: u32 = 21;
pub const SECURITY_NT_NON_UNIQUE_SUB_AUTH_COUNT: u32 = 3;
pub const SECURITY_NULL_RID: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SECURITY_OBJECT_AI_PARAMS {
    pub Size: u32,
    pub ConstraintMask: u32,
}
pub const SECURITY_OTHER_ORGANIZATION_RID: u32 = 1000;
pub const SECURITY_PACKAGE_BASE_RID: u32 = 64;
pub const SECURITY_PACKAGE_DIGEST_RID: u32 = 21;
pub const SECURITY_PACKAGE_NTLM_RID: u32 = 10;
pub const SECURITY_PACKAGE_RID_COUNT: u32 = 2;
pub const SECURITY_PACKAGE_SCHANNEL_RID: u32 = 14;
pub const SECURITY_PARENT_PACKAGE_RID_COUNT: u32 = 8;
pub const SECURITY_PRINCIPAL_SELF_RID: u32 = 10;
pub const SECURITY_PROCESS_PROTECTION_LEVEL_ANTIMALWARE_RID: u32 = 1536;
pub const SECURITY_PROCESS_PROTECTION_LEVEL_APP_RID: u32 = 2048;
pub const SECURITY_PROCESS_PROTECTION_LEVEL_AUTHENTICODE_RID: u32 = 1024;
pub const SECURITY_PROCESS_PROTECTION_LEVEL_NONE_RID: u32 = 0;
pub const SECURITY_PROCESS_PROTECTION_LEVEL_WINDOWS_RID: u32 = 4096;
pub const SECURITY_PROCESS_PROTECTION_LEVEL_WINTCB_RID: u32 = 8192;
pub const SECURITY_PROCESS_PROTECTION_TYPE_FULL_RID: u32 = 1024;
pub const SECURITY_PROCESS_PROTECTION_TYPE_LITE_RID: u32 = 512;
pub const SECURITY_PROCESS_PROTECTION_TYPE_NONE_RID: u32 = 0;
pub const SECURITY_PROCESS_TRUST_AUTHORITY_RID_COUNT: u32 = 2;
pub const SECURITY_PROXY_RID: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SECURITY_QUALITY_OF_SERVICE {
    pub Length: u32,
    pub ImpersonationLevel: SECURITY_IMPERSONATION_LEVEL,
    pub ContextTrackingMode: SECURITY_CONTEXT_TRACKING_MODE,
    pub EffectiveOnly: bool,
}
pub const SECURITY_RDV_GFX_BASE_RID: u32 = 91;
pub const SECURITY_REMOTE_LOGON_RID: u32 = 14;
pub const SECURITY_RESERVED_ID_BASE_RID: u32 = 81;
pub const SECURITY_RESTRICTED_CODE_RID: u32 = 12;
pub const SECURITY_RESTRICTED_SERVICES_BASE_RID: u32 = 99;
pub const SECURITY_RESTRICTED_SERVICES_RID_COUNT: u32 = 6;
pub const SECURITY_SERVER_LOGON_RID: u32 = 9;
pub const SECURITY_SERVICE_ID_BASE_RID: u32 = 80;
pub const SECURITY_SERVICE_ID_RID_COUNT: u32 = 6;
pub const SECURITY_SERVICE_RID: u32 = 6;
pub const SECURITY_SHADOWADMINACCOUNT_RID: u32 = 100;
pub const SECURITY_STATIC_TRACKING: u32 = 0;
pub const SECURITY_TASK_ID_BASE_RID: u32 = 87;
pub const SECURITY_TERMINAL_SERVER_RID: u32 = 13;
pub const SECURITY_THIS_ORGANIZATION_RID: u32 = 15;
pub const SECURITY_TRUSTED_INSTALLER_RID1: u32 = 956008885;
pub const SECURITY_TRUSTED_INSTALLER_RID2: u32 = 3418522649;
pub const SECURITY_TRUSTED_INSTALLER_RID3: u32 = 1831038044;
pub const SECURITY_TRUSTED_INSTALLER_RID4: u32 = 1853292631;
pub const SECURITY_TRUSTED_INSTALLER_RID5: u32 = 2271478464;
pub const SECURITY_UMFD_BASE_RID: u32 = 96;
pub const SECURITY_UNIQUIFIED_SERVICE_BASE_RID: u32 = 97;
pub const SECURITY_USERMANAGER_ID_BASE_RID: u32 = 93;
pub const SECURITY_USERMANAGER_ID_RID_COUNT: u32 = 6;
pub const SECURITY_USERMODEDRIVERHOST_ID_BASE_RID: u32 = 84;
pub const SECURITY_USERMODEDRIVERHOST_ID_RID_COUNT: u32 = 6;
pub const SECURITY_VIRTUALACCOUNT_ID_RID_COUNT: u32 = 6;
pub const SECURITY_VIRTUALSERVER_ID_BASE_RID: u32 = 83;
pub const SECURITY_VIRTUALSERVER_ID_RID_COUNT: u32 = 6;
pub const SECURITY_WINDOWSMOBILE_ID_BASE_RID: u32 = 112;
pub const SECURITY_WINDOW_MANAGER_BASE_RID: u32 = 90;
pub const SECURITY_WINRM_ID_BASE_RID: u32 = 94;
pub const SECURITY_WINRM_ID_RID_COUNT: u32 = 6;
pub const SECURITY_WMIHOST_ID_BASE_RID: u32 = 86;
pub const SECURITY_WMIHOST_ID_RID_COUNT: u32 = 6;
pub const SECURITY_WORLD_RID: u32 = 0;
pub const SECURITY_WRITE_RESTRICTED_CODE_RID: u32 = 33;
pub const SEC_64K_PAGES: u32 = 524288;
pub const SEC_COMMIT: u32 = 134217728;
pub const SEC_FILE: u32 = 8388608;
pub const SEC_HUGE_PAGES: u32 = 131072;
pub const SEC_IMAGE: u32 = 16777216;
pub const SEC_IMAGE_NO_EXECUTE: u32 = 285212672;
pub const SEC_LARGE_PAGES: u32 = 2147483648;
pub const SEC_NOCACHE: u32 = 268435456;
pub const SEC_PARTITION_OWNER_HANDLE: u32 = 262144;
pub const SEC_PROTECTED_IMAGE: u32 = 33554432;
pub const SEC_RESERVE: u32 = 67108864;
pub const SEC_WRITECOMBINE: u32 = 1073741824;
pub const SEF_AI_USE_EXTRA_PARAMS: u32 = 2048;
pub const SEF_AVOID_OWNER_CHECK: u32 = 16;
pub const SEF_AVOID_OWNER_RESTRICTION: u32 = 4096;
pub const SEF_AVOID_PRIVILEGE_CHECK: u32 = 8;
pub const SEF_DACL_AUTO_INHERIT: u32 = 1;
pub const SEF_DEFAULT_DESCRIPTOR_FOR_OBJECT: u32 = 4;
pub const SEF_DEFAULT_GROUP_FROM_PARENT: u32 = 64;
pub const SEF_DEFAULT_OWNER_FROM_PARENT: u32 = 32;
pub const SEF_FORCE_USER_MODE: u32 = 8192;
pub const SEF_MACL_NO_EXECUTE_UP: u32 = 1024;
pub const SEF_MACL_NO_READ_UP: u32 = 512;
pub const SEF_MACL_NO_WRITE_UP: u32 = 256;
pub const SEF_MACL_VALID_FLAGS: u32 = 1792;
pub const SEF_NORMALIZE_OUTPUT_DESCRIPTOR: u32 = 16384;
pub const SEF_SACL_AUTO_INHERIT: u32 = 2;
pub const SEMAPHORE_ALL_ACCESS: u32 = 2031619;
pub const SEMAPHORE_MODIFY_STATE: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SERVERSILO_BASIC_INFORMATION {
    pub ServiceSessionId: u32,
    pub State: SERVERSILO_STATE,
    pub ExitStatus: u32,
    pub Reserved: bool,
    pub ApiSetSchema: *mut core::ffi::c_void,
    pub HostApiSetSchema: *mut core::ffi::c_void,
    pub ContainerBuildNumber: u32,
    pub HostBuildNumber: u32,
}
impl Default for SERVERSILO_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SERVERSILO_DIAGNOSTIC_INFORMATION {
    pub ReportId: windows_core::GUID,
    pub ExitStatus: u32,
    pub CriticalProcessName: [u16; 15],
}
impl Default for SERVERSILO_DIAGNOSTIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SERVERSILO_INITING: SERVERSILO_STATE = 0;
pub const SERVERSILO_SHUTTING_DOWN: SERVERSILO_STATE = 2;
pub const SERVERSILO_STARTED: SERVERSILO_STATE = 1;
pub type SERVERSILO_STATE = i32;
pub const SERVERSILO_TERMINATED: SERVERSILO_STATE = 4;
pub const SERVERSILO_TERMINATING: SERVERSILO_STATE = 3;
pub const SERVICE_ADAPTER: u32 = 4;
pub const SERVICE_AUTO_START: u32 = 2;
pub const SERVICE_BOOT_START: u32 = 0;
pub const SERVICE_DEMAND_START: u32 = 3;
pub const SERVICE_DISABLED: u32 = 4;
pub const SERVICE_DRIVER: u32 = 11;
pub const SERVICE_ERROR_CRITICAL: u32 = 3;
pub const SERVICE_ERROR_IGNORE: u32 = 0;
pub const SERVICE_ERROR_NORMAL: u32 = 1;
pub const SERVICE_ERROR_SEVERE: u32 = 2;
pub type SERVICE_ERROR_TYPE = i32;
pub const SERVICE_FILE_SYSTEM_DRIVER: u32 = 2;
pub const SERVICE_INTERACTIVE_PROCESS: u32 = 256;
pub const SERVICE_KERNEL_DRIVER: u32 = 1;
pub type SERVICE_LOAD_TYPE = i32;
pub type SERVICE_NODE_TYPE = i32;
pub const SERVICE_PKG_SERVICE: u32 = 512;
pub const SERVICE_RECOGNIZER_DRIVER: u32 = 8;
pub const SERVICE_SYSTEM_START: u32 = 1;
pub const SERVICE_TYPE_ALL: u32 = 1023;
pub const SERVICE_USERSERVICE_INSTANCE: u32 = 128;
pub const SERVICE_USER_OWN_PROCESS: u32 = 80;
pub const SERVICE_USER_SERVICE: u32 = 64;
pub const SERVICE_USER_SHARE_PROCESS: u32 = 96;
pub const SERVICE_WIN32: u32 = 48;
pub const SERVICE_WIN32_OWN_PROCESS: u32 = 16;
pub const SERVICE_WIN32_SHARE_PROCESS: u32 = 32;
pub const SESSION_ALL_ACCESS: u32 = 983043;
pub const SESSION_MODIFY_ACCESS: u32 = 2;
pub const SESSION_QUERY_ACCESS: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SET_POWER_SETTING_VALUE {
    pub Version: u32,
    pub Guid: windows_core::GUID,
    pub PowerCondition: SYSTEM_POWER_CONDITION,
    pub DataLength: u32,
    pub Data: [u8; 1],
}
impl Default for SET_POWER_SETTING_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SE_ACCESS_CHECK_FLAG_NO_LEARNING_MODE_LOGGING: u32 = 8;
pub const SE_ACCESS_CHECK_VALID_FLAGS: u32 = 8;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SE_ACCESS_REPLY {
    pub Size: u32,
    pub ResultListCount: u32,
    pub GrantedAccess: PACCESS_MASK,
    pub AccessStatus: super::minwindef::PDWORD,
    pub AccessReason: PACCESS_REASONS,
    pub Privileges: *mut PPRIVILEGE_SET,
}
#[cfg(feature = "minwindef")]
impl Default for SE_ACCESS_REPLY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SE_ACCESS_REQUEST {
    pub Size: u32,
    pub SeSecurityDescriptor: PSE_SECURITY_DESCRIPTOR,
    pub DesiredAccess: ACCESS_MASK,
    pub PreviouslyGrantedAccess: ACCESS_MASK,
    pub PrincipalSelfSid: PSID,
    pub GenericMapping: PGENERIC_MAPPING,
    pub ObjectTypeListCount: u32,
    pub ObjectTypeList: POBJECT_TYPE_LIST,
}
pub const SE_ACTIVATE_AS_USER_CAPABILITY: windows_core::PCWSTR = windows_core::w!("activateAsUser");
pub const SE_APP_SILO_ACCESS_TO_PUBLISHER_DIRECTORY_CAPABILITY: windows_core::PCWSTR = windows_core::w!("isolatedWin32-accessToPublisherDirectory");
pub const SE_APP_SILO_PRINT_CAPABILITY: windows_core::PCWSTR = windows_core::w!("isolatedWin32-print");
pub const SE_APP_SILO_PROFILES_ROOT_MINIMAL_CAPABILITY: windows_core::PCWSTR = windows_core::w!("isolatedWin32-profilesRootMinimal");
pub const SE_APP_SILO_PROMPT_FOR_ACCESS_CAPABILITY: windows_core::PCWSTR = windows_core::w!("isolatedWin32-promptForAccess");
pub const SE_APP_SILO_SCANNER_CAPABILITY: windows_core::PCWSTR = windows_core::w!("isolatedWin32-scanner");
pub const SE_APP_SILO_USER_PROFILE_MINIMAL_CAPABILITY: windows_core::PCWSTR = windows_core::w!("isolatedWin32-userProfileMinimal");
pub const SE_APP_SILO_VOLUME_ROOT_MINIMAL_CAPABILITY: windows_core::PCWSTR = windows_core::w!("isolatedWin32-volumeRootMinimal");
pub const SE_CONSTRAINED_IMPERSONATION_CAPABILITY: windows_core::PCWSTR = windows_core::w!("constrainedImpersonation");
pub const SE_DACL_AUTO_INHERITED: u32 = 1024;
pub const SE_DACL_AUTO_INHERIT_REQ: u32 = 256;
pub const SE_DACL_DEFAULTED: u32 = 8;
pub const SE_DACL_PRESENT: u32 = 4;
pub const SE_DACL_PROTECTED: u32 = 4096;
pub const SE_DEVELOPMENT_MODE_NETWORK_CAPABILITY: windows_core::PCWSTR = windows_core::w!("developmentModeNetwork");
pub const SE_GROUP_DEFAULTED: u32 = 2;
pub const SE_GROUP_ENABLED: u32 = 4;
pub const SE_GROUP_ENABLED_BY_DEFAULT: u32 = 2;
pub const SE_GROUP_INTEGRITY: u32 = 32;
pub const SE_GROUP_INTEGRITY_ENABLED: u32 = 64;
pub const SE_GROUP_LOGON_ID: u32 = 3221225472;
pub const SE_GROUP_MANDATORY: u32 = 1;
pub const SE_GROUP_OWNER: u32 = 8;
pub const SE_GROUP_RESOURCE: u32 = 536870912;
pub const SE_GROUP_USE_FOR_DENY_ONLY: u32 = 16;
pub const SE_GROUP_VALID_ATTRIBUTES: i32 = -536870785;
pub type SE_IMAGE_SIGNATURE_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SE_IMPERSONATION_STATE {
    pub Token: PACCESS_TOKEN,
    pub CopyOnOpen: bool,
    pub EffectiveOnly: bool,
    pub Level: SECURITY_IMPERSONATION_LEVEL,
}
pub const SE_LEARNING_MODE_LOGGING_CAPABILITY: windows_core::PCWSTR = windows_core::w!("learningModeLogging");
pub const SE_MUMA_CAPABILITY: windows_core::PCWSTR = windows_core::w!("muma");
pub const SE_OWNER_DEFAULTED: u32 = 1;
pub const SE_PERMISSIVE_LEARNING_MODE_CAPABILITY: windows_core::PCWSTR = windows_core::w!("permissiveLearningMode");
pub const SE_PRIVILEGE_ENABLED: u32 = 2;
pub const SE_PRIVILEGE_ENABLED_BY_DEFAULT: u32 = 1;
pub const SE_PRIVILEGE_REMOVED: u32 = 4;
pub const SE_PRIVILEGE_USED_FOR_ACCESS: u32 = 2147483648;
pub const SE_PRIVILEGE_VALID_ATTRIBUTES: i32 = -2147483641;
pub const SE_RM_CONTROL_VALID: u32 = 16384;
pub const SE_SACL_AUTO_INHERITED: u32 = 2048;
pub const SE_SACL_AUTO_INHERIT_REQ: u32 = 512;
pub const SE_SACL_DEFAULTED: u32 = 32;
pub const SE_SACL_PRESENT: u32 = 16;
pub const SE_SACL_PROTECTED: u32 = 8192;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SE_SECURITY_DESCRIPTOR {
    pub Size: u32,
    pub Flags: u32,
    pub SecurityDescriptor: PSECURITY_DESCRIPTOR,
}
pub const SE_SECURITY_DESCRIPTOR_FLAG_NO_ACCESS_FILTER_ACE: u32 = 4;
pub const SE_SECURITY_DESCRIPTOR_FLAG_NO_LABEL_ACE: u32 = 2;
pub const SE_SECURITY_DESCRIPTOR_FLAG_NO_OWNER_ACE: u32 = 1;
pub const SE_SECURITY_DESCRIPTOR_VALID_FLAGS: u32 = 7;
pub const SE_SELF_RELATIVE: u32 = 32768;
pub const SE_SESSION_IMPERSONATION_CAPABILITY: windows_core::PCWSTR = windows_core::w!("sessionImpersonation");
#[repr(C)]
#[derive(Clone, Copy)]
pub union SE_SID {
    pub Sid: SID,
    pub Buffer: [u8; 68],
}
impl Default for SE_SID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SE_SIGNING_LEVEL(pub u8);
pub const SE_SIGNING_LEVEL_ANTIMALWARE: u32 = 7;
pub const SE_SIGNING_LEVEL_AUTHENTICODE: u32 = 4;
pub const SE_SIGNING_LEVEL_CUSTOM_1: u32 = 3;
pub const SE_SIGNING_LEVEL_CUSTOM_2: u32 = 5;
pub const SE_SIGNING_LEVEL_CUSTOM_3: u32 = 7;
pub const SE_SIGNING_LEVEL_CUSTOM_4: u32 = 9;
pub const SE_SIGNING_LEVEL_CUSTOM_5: u32 = 10;
pub const SE_SIGNING_LEVEL_CUSTOM_6: u32 = 15;
pub const SE_SIGNING_LEVEL_CUSTOM_7: u32 = 13;
pub const SE_SIGNING_LEVEL_DEVELOPER: u32 = 3;
pub const SE_SIGNING_LEVEL_DYNAMIC_CODEGEN: u32 = 11;
pub const SE_SIGNING_LEVEL_ENTERPRISE: u32 = 2;
pub const SE_SIGNING_LEVEL_MICROSOFT: u32 = 8;
pub const SE_SIGNING_LEVEL_STORE: u32 = 6;
pub const SE_SIGNING_LEVEL_UNCHECKED: u32 = 0;
pub const SE_SIGNING_LEVEL_UNSIGNED: u32 = 1;
pub const SE_SIGNING_LEVEL_WINDOWS: u32 = 12;
pub const SE_SIGNING_LEVEL_WINDOWS_TCB: u32 = 14;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SE_TOKEN_USER {
    pub Anonymous: SE_TOKEN_USER_0,
    pub Anonymous2: SE_TOKEN_USER_1,
}
impl Default for SE_TOKEN_USER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SE_TOKEN_USER_0 {
    pub TokenUser: TOKEN_USER,
    pub User: SID_AND_ATTRIBUTES,
}
impl Default for SE_TOKEN_USER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SE_TOKEN_USER_1 {
    pub Sid: SID,
    pub Buffer: [u8; 68],
}
impl Default for SE_TOKEN_USER_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "basetsd")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SHARED_COMPUTE_UNIT_RELATIONSHIP {
    pub Type: u32,
    pub ComputeUnitCount: u32,
    pub Reserved: [u8; 14],
    pub GroupCount: u16,
    pub GroupMasks: [GROUP_AFFINITY; 1],
}
#[cfg(feature = "basetsd")]
impl Default for SHARED_COMPUTE_UNIT_RELATIONSHIP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SHARED_VIRTUAL_DISK_SUPPORT {
    pub SharedVirtualDiskSupport: SharedVirtualDiskSupportType,
    pub HandleState: SharedVirtualDiskHandleState,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SHUFFLE_FILE_DATA {
    pub StartingOffset: i64,
    pub Length: i64,
    pub Flags: u32,
}
pub const SHUFFLE_FILE_FLAG_SKIP_INITIALIZING_NEW_CLUSTERS: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SID {
    pub Revision: u8,
    pub SubAuthorityCount: u8,
    pub IdentifierAuthority: SID_IDENTIFIER_AUTHORITY,
    pub SubAuthority: [u32; 1],
}
impl Default for SID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SID_AND_ATTRIBUTES {
    pub Sid: PSID,
    pub Attributes: u32,
}
pub type SID_AND_ATTRIBUTES_ARRAY = [SID_AND_ATTRIBUTES; 1];
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SID_AND_ATTRIBUTES_HASH {
    pub SidCount: u32,
    pub SidAttr: PSID_AND_ATTRIBUTES,
    pub Hash: [SID_HASH_ENTRY; 32],
}
impl Default for SID_AND_ATTRIBUTES_HASH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SID_HASH_ENTRY(pub usize);
pub const SID_HASH_SIZE: u32 = 32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SID_IDENTIFIER_AUTHORITY {
    pub Value: [u8; 6],
}
impl Default for SID_IDENTIFIER_AUTHORITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SID_MAX_SUB_AUTHORITIES: u32 = 15;
pub type SID_NAME_USE = i32;
pub const SID_RECOMMENDED_SUB_AUTHORITIES: u32 = 1;
pub const SID_REVISION: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SILOOBJECT_BASIC_INFORMATION {
    pub SiloId: u32,
    pub SiloParentId: u32,
    pub NumberOfProcesses: u32,
    pub IsInServerSilo: bool,
    pub Reserved: [u8; 3],
}
impl Default for SILOOBJECT_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SINGLE_LIST_ENTRY {
    pub Next: *mut Self,
}
impl Default for SINGLE_LIST_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SIZEOF_RFPO_DATA: u32 = 16;
#[cfg(target_arch = "x86")]
pub const SIZE_OF_80387_REGISTERS: u32 = 80;
#[cfg(target_arch = "x86")]
pub type SLIST_ENTRY = SINGLE_LIST_ENTRY;
#[repr(C, align(16))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SLIST_ENTRY {
    pub Next: *mut Self,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SLIST_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub union SLIST_HEADER {
    pub Alignment: u64,
    pub Anonymous: SLIST_HEADER_0,
}
#[cfg(target_arch = "x86")]
impl Default for SLIST_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SLIST_HEADER_0 {
    pub Next: SLIST_ENTRY,
    pub Depth: u16,
    pub CpuId: u16,
}
#[repr(C, align(16))]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy)]
pub union SLIST_HEADER {
    pub Anonymous: SLIST_HEADER_0,
    pub HeaderX64: SLIST_HEADER_1,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for SLIST_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SLIST_HEADER_0 {
    pub Alignment: u64,
    pub Region: u64,
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SLIST_HEADER_1 {
    pub _bitfield1: u64,
    pub _bitfield2: u64,
}
#[repr(C, align(16))]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy)]
pub union SLIST_HEADER {
    pub Anonymous: SLIST_HEADER_0,
    pub HeaderArm64: SLIST_HEADER_1,
}
#[cfg(target_arch = "aarch64")]
impl Default for SLIST_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SLIST_HEADER_0 {
    pub Alignment: u64,
    pub Region: u64,
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SLIST_HEADER_1 {
    pub _bitfield1: u64,
    pub _bitfield2: u64,
}
pub const SMB_CCF_APP_INSTANCE_EA_NAME: windows_core::PCSTR = windows_core::s!("ClusteredApplicationInstance");
pub const SMT_UNPARKING_POLICY_CORE: u32 = 0;
pub const SMT_UNPARKING_POLICY_CORE_PER_THREAD: u32 = 1;
pub const SMT_UNPARKING_POLICY_LP_ROUNDROBIN: u32 = 2;
pub const SMT_UNPARKING_POLICY_LP_SEQUENTIAL: u32 = 3;
pub const SORT_CHINESE_BIG5: u32 = 0;
pub const SORT_CHINESE_BOPOMOFO: u32 = 3;
pub const SORT_CHINESE_PRC: u32 = 2;
pub const SORT_CHINESE_PRCP: u32 = 0;
pub const SORT_CHINESE_RADICALSTROKE: u32 = 4;
pub const SORT_CHINESE_UNICODE: u32 = 1;
pub const SORT_DEFAULT: u32 = 0;
pub const SORT_GEORGIAN_MODERN: u32 = 1;
pub const SORT_GEORGIAN_TRADITIONAL: u32 = 0;
pub const SORT_GERMAN_PHONE_BOOK: u32 = 1;
pub const SORT_HUNGARIAN_DEFAULT: u32 = 0;
pub const SORT_HUNGARIAN_TECHNICAL: u32 = 1;
pub const SORT_INVARIANT_MATH: u32 = 1;
pub const SORT_JAPANESE_RADICALSTROKE: u32 = 4;
pub const SORT_JAPANESE_UNICODE: u32 = 1;
pub const SORT_JAPANESE_XJIS: u32 = 0;
pub const SORT_KOREAN_KSC: u32 = 0;
pub const SORT_KOREAN_UNICODE: u32 = 1;
pub const SPECIFIC_RIGHTS_ALL: u32 = 65535;
pub const STANDARD_RIGHTS_ALL: u32 = 2031616;
pub const STANDARD_RIGHTS_EXECUTE: u32 = 131072;
pub const STANDARD_RIGHTS_READ: u32 = 131072;
pub const STANDARD_RIGHTS_REQUIRED: u32 = 983040;
pub const STANDARD_RIGHTS_WRITE: u32 = 131072;
pub const STATUS_ABANDONED_WAIT_0: u32 = 128;
pub const STATUS_ACCESS_VIOLATION: u32 = 3221225477;
pub const STATUS_ALREADY_REGISTERED: u32 = 3221227288;
pub const STATUS_ARRAY_BOUNDS_EXCEEDED: u32 = 3221225612;
pub const STATUS_ASSERTION_FAILURE: u32 = 3221226528;
pub const STATUS_BREAKPOINT: u32 = 2147483651;
pub const STATUS_CONTROL_C_EXIT: u32 = 3221225786;
pub const STATUS_CONTROL_STACK_VIOLATION: u32 = 3221225906;
pub const STATUS_DATATYPE_MISALIGNMENT: u32 = 2147483650;
pub const STATUS_DLL_INIT_FAILED: u32 = 3221225794;
pub const STATUS_DLL_NOT_FOUND: u32 = 3221225781;
pub const STATUS_ENCLAVE_VIOLATION: u32 = 3221226658;
pub const STATUS_ENTRYPOINT_NOT_FOUND: u32 = 3221225785;
pub const STATUS_FATAL_APP_EXIT: u32 = 1073741845;
pub const STATUS_FLOAT_DENORMAL_OPERAND: u32 = 3221225613;
pub const STATUS_FLOAT_DIVIDE_BY_ZERO: u32 = 3221225614;
pub const STATUS_FLOAT_INEXACT_RESULT: u32 = 3221225615;
pub const STATUS_FLOAT_INVALID_OPERATION: u32 = 3221225616;
pub const STATUS_FLOAT_MULTIPLE_FAULTS: u32 = 3221226164;
pub const STATUS_FLOAT_MULTIPLE_TRAPS: u32 = 3221226165;
pub const STATUS_FLOAT_OVERFLOW: u32 = 3221225617;
pub const STATUS_FLOAT_STACK_CHECK: u32 = 3221225618;
pub const STATUS_FLOAT_UNDERFLOW: u32 = 3221225619;
pub const STATUS_GUARD_PAGE_VIOLATION: u32 = 2147483649;
pub const STATUS_HEAP_CORRUPTION: u32 = 3221226356;
pub const STATUS_ILLEGAL_INSTRUCTION: u32 = 3221225501;
pub const STATUS_INTEGER_DIVIDE_BY_ZERO: u32 = 3221225620;
pub const STATUS_INTEGER_OVERFLOW: u32 = 3221225621;
pub const STATUS_INTERRUPTED: u32 = 3221226773;
pub const STATUS_INVALID_CRUNTIME_PARAMETER: u32 = 3221226519;
pub const STATUS_INVALID_DISPOSITION: u32 = 3221225510;
pub const STATUS_INVALID_HANDLE: u32 = 3221225480;
pub const STATUS_INVALID_PARAMETER: u32 = 3221225485;
pub const STATUS_IN_PAGE_ERROR: u32 = 3221225478;
pub const STATUS_LONGJUMP: u32 = 2147483686;
pub const STATUS_NONCONTINUABLE_EXCEPTION: u32 = 3221225509;
pub const STATUS_NO_MEMORY: u32 = 3221225495;
pub const STATUS_ORDINAL_NOT_FOUND: u32 = 3221225784;
pub const STATUS_PENDING: u32 = 259;
pub const STATUS_PRIVILEGED_INSTRUCTION: u32 = 3221225622;
pub const STATUS_REG_NAT_CONSUMPTION: u32 = 3221226185;
pub const STATUS_SEGMENT_NOTIFICATION: u32 = 1073741829;
pub const STATUS_SINGLE_STEP: u32 = 2147483652;
pub const STATUS_STACK_BUFFER_OVERRUN: u32 = 3221226505;
pub const STATUS_STACK_OVERFLOW: u32 = 3221225725;
pub const STATUS_SXS_EARLY_DEACTIVATION: u32 = 3222601743;
pub const STATUS_SXS_INVALID_DEACTIVATION: u32 = 3222601744;
pub const STATUS_THREAD_NOT_RUNNING: u32 = 3221226774;
pub const STATUS_TIMEOUT: u32 = 258;
pub const STATUS_UNWIND_CONSOLIDATE: u32 = 2147483689;
pub const STATUS_USER_APC: u32 = 192;
pub const STATUS_WAIT_0: u32 = 0;
pub const SUBLANG_AFRIKAANS_SOUTH_AFRICA: u32 = 1;
pub const SUBLANG_ALBANIAN_ALBANIA: u32 = 1;
pub const SUBLANG_ALSATIAN_FRANCE: u32 = 1;
pub const SUBLANG_AMHARIC_ETHIOPIA: u32 = 1;
pub const SUBLANG_ARABIC_ALGERIA: u32 = 5;
pub const SUBLANG_ARABIC_BAHRAIN: u32 = 15;
pub const SUBLANG_ARABIC_EGYPT: u32 = 3;
pub const SUBLANG_ARABIC_IRAQ: u32 = 2;
pub const SUBLANG_ARABIC_JORDAN: u32 = 11;
pub const SUBLANG_ARABIC_KUWAIT: u32 = 13;
pub const SUBLANG_ARABIC_LEBANON: u32 = 12;
pub const SUBLANG_ARABIC_LIBYA: u32 = 4;
pub const SUBLANG_ARABIC_MOROCCO: u32 = 6;
pub const SUBLANG_ARABIC_OMAN: u32 = 8;
pub const SUBLANG_ARABIC_QATAR: u32 = 16;
pub const SUBLANG_ARABIC_SAUDI_ARABIA: u32 = 1;
pub const SUBLANG_ARABIC_SYRIA: u32 = 10;
pub const SUBLANG_ARABIC_TUNISIA: u32 = 7;
pub const SUBLANG_ARABIC_UAE: u32 = 14;
pub const SUBLANG_ARABIC_YEMEN: u32 = 9;
pub const SUBLANG_ARMENIAN_ARMENIA: u32 = 1;
pub const SUBLANG_ASSAMESE_INDIA: u32 = 1;
pub const SUBLANG_AZERBAIJANI_AZERBAIJAN_CYRILLIC: u32 = 2;
pub const SUBLANG_AZERBAIJANI_AZERBAIJAN_LATIN: u32 = 1;
pub const SUBLANG_AZERI_CYRILLIC: u32 = 2;
pub const SUBLANG_AZERI_LATIN: u32 = 1;
pub const SUBLANG_BANGLA_BANGLADESH: u32 = 2;
pub const SUBLANG_BANGLA_INDIA: u32 = 1;
pub const SUBLANG_BASHKIR_RUSSIA: u32 = 1;
pub const SUBLANG_BASQUE_BASQUE: u32 = 1;
pub const SUBLANG_BELARUSIAN_BELARUS: u32 = 1;
pub const SUBLANG_BENGALI_BANGLADESH: u32 = 2;
pub const SUBLANG_BENGALI_INDIA: u32 = 1;
pub const SUBLANG_BOSNIAN_BOSNIA_HERZEGOVINA_CYRILLIC: u32 = 8;
pub const SUBLANG_BOSNIAN_BOSNIA_HERZEGOVINA_LATIN: u32 = 5;
pub const SUBLANG_BRETON_FRANCE: u32 = 1;
pub const SUBLANG_BULGARIAN_BULGARIA: u32 = 1;
pub const SUBLANG_CATALAN_CATALAN: u32 = 1;
pub const SUBLANG_CENTRAL_KURDISH_IRAQ: u32 = 1;
pub const SUBLANG_CHEROKEE_CHEROKEE: u32 = 1;
pub const SUBLANG_CHINESE_HONGKONG: u32 = 3;
pub const SUBLANG_CHINESE_MACAU: u32 = 5;
pub const SUBLANG_CHINESE_SIMPLIFIED: u32 = 2;
pub const SUBLANG_CHINESE_SINGAPORE: u32 = 4;
pub const SUBLANG_CHINESE_TRADITIONAL: u32 = 1;
pub const SUBLANG_CORSICAN_FRANCE: u32 = 1;
pub const SUBLANG_CROATIAN_BOSNIA_HERZEGOVINA_LATIN: u32 = 4;
pub const SUBLANG_CROATIAN_CROATIA: u32 = 1;
pub const SUBLANG_CUSTOM_DEFAULT: u32 = 3;
pub const SUBLANG_CUSTOM_UNSPECIFIED: u32 = 4;
pub const SUBLANG_CZECH_CZECH_REPUBLIC: u32 = 1;
pub const SUBLANG_DANISH_DENMARK: u32 = 1;
pub const SUBLANG_DARI_AFGHANISTAN: u32 = 1;
pub const SUBLANG_DEFAULT: u32 = 1;
pub const SUBLANG_DIVEHI_MALDIVES: u32 = 1;
pub const SUBLANG_DUTCH: u32 = 1;
pub const SUBLANG_DUTCH_BELGIAN: u32 = 2;
pub const SUBLANG_ENGLISH_AUS: u32 = 3;
pub const SUBLANG_ENGLISH_BELIZE: u32 = 10;
pub const SUBLANG_ENGLISH_CAN: u32 = 4;
pub const SUBLANG_ENGLISH_CARIBBEAN: u32 = 9;
pub const SUBLANG_ENGLISH_EIRE: u32 = 6;
pub const SUBLANG_ENGLISH_INDIA: u32 = 16;
pub const SUBLANG_ENGLISH_JAMAICA: u32 = 8;
pub const SUBLANG_ENGLISH_MALAYSIA: u32 = 17;
pub const SUBLANG_ENGLISH_NZ: u32 = 5;
pub const SUBLANG_ENGLISH_PHILIPPINES: u32 = 13;
pub const SUBLANG_ENGLISH_SINGAPORE: u32 = 18;
pub const SUBLANG_ENGLISH_SOUTH_AFRICA: u32 = 7;
pub const SUBLANG_ENGLISH_TRINIDAD: u32 = 11;
pub const SUBLANG_ENGLISH_UK: u32 = 2;
pub const SUBLANG_ENGLISH_US: u32 = 1;
pub const SUBLANG_ENGLISH_ZIMBABWE: u32 = 12;
pub const SUBLANG_ESTONIAN_ESTONIA: u32 = 1;
pub const SUBLANG_FAEROESE_FAROE_ISLANDS: u32 = 1;
pub const SUBLANG_FILIPINO_PHILIPPINES: u32 = 1;
pub const SUBLANG_FINNISH_FINLAND: u32 = 1;
pub const SUBLANG_FRENCH: u32 = 1;
pub const SUBLANG_FRENCH_BELGIAN: u32 = 2;
pub const SUBLANG_FRENCH_CANADIAN: u32 = 3;
pub const SUBLANG_FRENCH_LUXEMBOURG: u32 = 5;
pub const SUBLANG_FRENCH_MONACO: u32 = 6;
pub const SUBLANG_FRENCH_SWISS: u32 = 4;
pub const SUBLANG_FRISIAN_NETHERLANDS: u32 = 1;
pub const SUBLANG_FULAH_SENEGAL: u32 = 2;
pub const SUBLANG_GALICIAN_GALICIAN: u32 = 1;
pub const SUBLANG_GEORGIAN_GEORGIA: u32 = 1;
pub const SUBLANG_GERMAN: u32 = 1;
pub const SUBLANG_GERMAN_AUSTRIAN: u32 = 3;
pub const SUBLANG_GERMAN_LIECHTENSTEIN: u32 = 5;
pub const SUBLANG_GERMAN_LUXEMBOURG: u32 = 4;
pub const SUBLANG_GERMAN_SWISS: u32 = 2;
pub const SUBLANG_GREEK_GREECE: u32 = 1;
pub const SUBLANG_GREENLANDIC_GREENLAND: u32 = 1;
pub const SUBLANG_GUJARATI_INDIA: u32 = 1;
pub const SUBLANG_HAUSA_NIGERIA_LATIN: u32 = 1;
pub const SUBLANG_HAWAIIAN_US: u32 = 1;
pub const SUBLANG_HEBREW_ISRAEL: u32 = 1;
pub const SUBLANG_HINDI_INDIA: u32 = 1;
pub const SUBLANG_HUNGARIAN_HUNGARY: u32 = 1;
pub const SUBLANG_ICELANDIC_ICELAND: u32 = 1;
pub const SUBLANG_IGBO_NIGERIA: u32 = 1;
pub const SUBLANG_INDONESIAN_INDONESIA: u32 = 1;
pub const SUBLANG_INUKTITUT_CANADA: u32 = 1;
pub const SUBLANG_INUKTITUT_CANADA_LATIN: u32 = 2;
pub const SUBLANG_IRISH_IRELAND: u32 = 2;
pub const SUBLANG_ITALIAN: u32 = 1;
pub const SUBLANG_ITALIAN_SWISS: u32 = 2;
pub const SUBLANG_JAPANESE_JAPAN: u32 = 1;
pub const SUBLANG_KANNADA_INDIA: u32 = 1;
pub const SUBLANG_KASHMIRI_INDIA: u32 = 2;
pub const SUBLANG_KASHMIRI_SASIA: u32 = 2;
pub const SUBLANG_KAZAK_KAZAKHSTAN: u32 = 1;
pub const SUBLANG_KHMER_CAMBODIA: u32 = 1;
pub const SUBLANG_KICHE_GUATEMALA: u32 = 1;
pub const SUBLANG_KINYARWANDA_RWANDA: u32 = 1;
pub const SUBLANG_KONKANI_INDIA: u32 = 1;
pub const SUBLANG_KOREAN: u32 = 1;
pub const SUBLANG_KYRGYZ_KYRGYZSTAN: u32 = 1;
pub const SUBLANG_LAO_LAO: u32 = 1;
pub const SUBLANG_LATVIAN_LATVIA: u32 = 1;
pub const SUBLANG_LITHUANIAN: u32 = 1;
pub const SUBLANG_LOWER_SORBIAN_GERMANY: u32 = 2;
pub const SUBLANG_LUXEMBOURGISH_LUXEMBOURG: u32 = 1;
pub const SUBLANG_MACEDONIAN_MACEDONIA: u32 = 1;
pub const SUBLANG_MALAYALAM_INDIA: u32 = 1;
pub const SUBLANG_MALAY_BRUNEI_DARUSSALAM: u32 = 2;
pub const SUBLANG_MALAY_MALAYSIA: u32 = 1;
pub const SUBLANG_MALTESE_MALTA: u32 = 1;
pub const SUBLANG_MAORI_NEW_ZEALAND: u32 = 1;
pub const SUBLANG_MAPUDUNGUN_CHILE: u32 = 1;
pub const SUBLANG_MARATHI_INDIA: u32 = 1;
pub const SUBLANG_MOHAWK_MOHAWK: u32 = 1;
pub const SUBLANG_MONGOLIAN_CYRILLIC_MONGOLIA: u32 = 1;
pub const SUBLANG_MONGOLIAN_PRC: u32 = 2;
pub const SUBLANG_NEPALI_INDIA: u32 = 2;
pub const SUBLANG_NEPALI_NEPAL: u32 = 1;
pub const SUBLANG_NEUTRAL: u32 = 0;
pub const SUBLANG_NORWEGIAN_BOKMAL: u32 = 1;
pub const SUBLANG_NORWEGIAN_NYNORSK: u32 = 2;
pub const SUBLANG_OCCITAN_FRANCE: u32 = 1;
pub const SUBLANG_ODIA_INDIA: u32 = 1;
pub const SUBLANG_ORIYA_INDIA: u32 = 1;
pub const SUBLANG_PASHTO_AFGHANISTAN: u32 = 1;
pub const SUBLANG_PERSIAN_IRAN: u32 = 1;
pub const SUBLANG_POLISH_POLAND: u32 = 1;
pub const SUBLANG_PORTUGUESE: u32 = 2;
pub const SUBLANG_PORTUGUESE_BRAZILIAN: u32 = 1;
pub const SUBLANG_PULAR_SENEGAL: u32 = 2;
pub const SUBLANG_PUNJABI_INDIA: u32 = 1;
pub const SUBLANG_PUNJABI_PAKISTAN: u32 = 2;
pub const SUBLANG_QUECHUA_BOLIVIA: u32 = 1;
pub const SUBLANG_QUECHUA_ECUADOR: u32 = 2;
pub const SUBLANG_QUECHUA_PERU: u32 = 3;
pub const SUBLANG_ROMANIAN_ROMANIA: u32 = 1;
pub const SUBLANG_ROMANSH_SWITZERLAND: u32 = 1;
pub const SUBLANG_RUSSIAN_RUSSIA: u32 = 1;
pub const SUBLANG_SAKHA_RUSSIA: u32 = 1;
pub const SUBLANG_SAMI_INARI_FINLAND: u32 = 9;
pub const SUBLANG_SAMI_LULE_NORWAY: u32 = 4;
pub const SUBLANG_SAMI_LULE_SWEDEN: u32 = 5;
pub const SUBLANG_SAMI_NORTHERN_FINLAND: u32 = 3;
pub const SUBLANG_SAMI_NORTHERN_NORWAY: u32 = 1;
pub const SUBLANG_SAMI_NORTHERN_SWEDEN: u32 = 2;
pub const SUBLANG_SAMI_SKOLT_FINLAND: u32 = 8;
pub const SUBLANG_SAMI_SOUTHERN_NORWAY: u32 = 6;
pub const SUBLANG_SAMI_SOUTHERN_SWEDEN: u32 = 7;
pub const SUBLANG_SANSKRIT_INDIA: u32 = 1;
pub const SUBLANG_SCOTTISH_GAELIC: u32 = 1;
pub const SUBLANG_SERBIAN_BOSNIA_HERZEGOVINA_CYRILLIC: u32 = 7;
pub const SUBLANG_SERBIAN_BOSNIA_HERZEGOVINA_LATIN: u32 = 6;
pub const SUBLANG_SERBIAN_CROATIA: u32 = 1;
pub const SUBLANG_SERBIAN_CYRILLIC: u32 = 3;
pub const SUBLANG_SERBIAN_LATIN: u32 = 2;
pub const SUBLANG_SERBIAN_MONTENEGRO_CYRILLIC: u32 = 12;
pub const SUBLANG_SERBIAN_MONTENEGRO_LATIN: u32 = 11;
pub const SUBLANG_SERBIAN_SERBIA_CYRILLIC: u32 = 10;
pub const SUBLANG_SERBIAN_SERBIA_LATIN: u32 = 9;
pub const SUBLANG_SINDHI_AFGHANISTAN: u32 = 2;
pub const SUBLANG_SINDHI_INDIA: u32 = 1;
pub const SUBLANG_SINDHI_PAKISTAN: u32 = 2;
pub const SUBLANG_SINHALESE_SRI_LANKA: u32 = 1;
pub const SUBLANG_SLOVAK_SLOVAKIA: u32 = 1;
pub const SUBLANG_SLOVENIAN_SLOVENIA: u32 = 1;
pub const SUBLANG_SOTHO_NORTHERN_SOUTH_AFRICA: u32 = 1;
pub const SUBLANG_SPANISH: u32 = 1;
pub const SUBLANG_SPANISH_ARGENTINA: u32 = 11;
pub const SUBLANG_SPANISH_BOLIVIA: u32 = 16;
pub const SUBLANG_SPANISH_CHILE: u32 = 13;
pub const SUBLANG_SPANISH_COLOMBIA: u32 = 9;
pub const SUBLANG_SPANISH_COSTA_RICA: u32 = 5;
pub const SUBLANG_SPANISH_DOMINICAN_REPUBLIC: u32 = 7;
pub const SUBLANG_SPANISH_ECUADOR: u32 = 12;
pub const SUBLANG_SPANISH_EL_SALVADOR: u32 = 17;
pub const SUBLANG_SPANISH_GUATEMALA: u32 = 4;
pub const SUBLANG_SPANISH_HONDURAS: u32 = 18;
pub const SUBLANG_SPANISH_MEXICAN: u32 = 2;
pub const SUBLANG_SPANISH_MODERN: u32 = 3;
pub const SUBLANG_SPANISH_NICARAGUA: u32 = 19;
pub const SUBLANG_SPANISH_PANAMA: u32 = 6;
pub const SUBLANG_SPANISH_PARAGUAY: u32 = 15;
pub const SUBLANG_SPANISH_PERU: u32 = 10;
pub const SUBLANG_SPANISH_PUERTO_RICO: u32 = 20;
pub const SUBLANG_SPANISH_URUGUAY: u32 = 14;
pub const SUBLANG_SPANISH_US: u32 = 21;
pub const SUBLANG_SPANISH_VENEZUELA: u32 = 8;
pub const SUBLANG_SWAHILI_KENYA: u32 = 1;
pub const SUBLANG_SWEDISH: u32 = 1;
pub const SUBLANG_SWEDISH_FINLAND: u32 = 2;
pub const SUBLANG_SYRIAC_SYRIA: u32 = 1;
pub const SUBLANG_SYS_DEFAULT: u32 = 2;
pub const SUBLANG_TAJIK_TAJIKISTAN: u32 = 1;
pub const SUBLANG_TAMAZIGHT_ALGERIA_LATIN: u32 = 2;
pub const SUBLANG_TAMAZIGHT_MOROCCO_TIFINAGH: u32 = 4;
pub const SUBLANG_TAMIL_INDIA: u32 = 1;
pub const SUBLANG_TAMIL_SRI_LANKA: u32 = 2;
pub const SUBLANG_TATAR_RUSSIA: u32 = 1;
pub const SUBLANG_TELUGU_INDIA: u32 = 1;
pub const SUBLANG_THAI_THAILAND: u32 = 1;
pub const SUBLANG_TIBETAN_PRC: u32 = 1;
pub const SUBLANG_TIGRIGNA_ERITREA: u32 = 2;
pub const SUBLANG_TIGRINYA_ERITREA: u32 = 2;
pub const SUBLANG_TIGRINYA_ETHIOPIA: u32 = 1;
pub const SUBLANG_TSWANA_BOTSWANA: u32 = 2;
pub const SUBLANG_TSWANA_SOUTH_AFRICA: u32 = 1;
pub const SUBLANG_TURKISH_TURKEY: u32 = 1;
pub const SUBLANG_TURKMEN_TURKMENISTAN: u32 = 1;
pub const SUBLANG_UIGHUR_PRC: u32 = 1;
pub const SUBLANG_UI_CUSTOM_DEFAULT: u32 = 5;
pub const SUBLANG_UKRAINIAN_UKRAINE: u32 = 1;
pub const SUBLANG_UPPER_SORBIAN_GERMANY: u32 = 1;
pub const SUBLANG_URDU_INDIA: u32 = 2;
pub const SUBLANG_URDU_PAKISTAN: u32 = 1;
pub const SUBLANG_UZBEK_CYRILLIC: u32 = 2;
pub const SUBLANG_UZBEK_LATIN: u32 = 1;
pub const SUBLANG_VALENCIAN_VALENCIA: u32 = 2;
pub const SUBLANG_VIETNAMESE_VIETNAM: u32 = 1;
pub const SUBLANG_WELSH_UNITED_KINGDOM: u32 = 1;
pub const SUBLANG_WOLOF_SENEGAL: u32 = 1;
pub const SUBLANG_XHOSA_SOUTH_AFRICA: u32 = 1;
pub const SUBLANG_YAKUT_RUSSIA: u32 = 1;
pub const SUBLANG_YI_PRC: u32 = 1;
pub const SUBLANG_YORUBA_NIGERIA: u32 = 1;
pub const SUBLANG_ZULU_SOUTH_AFRICA: u32 = 1;
pub const SUCCESSFUL_ACCESS_ACE_FLAG: u32 = 64;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SUPPORTED_OS_INFO {
    pub MajorVersion: u16,
    pub MinorVersion: u16,
}
pub const SYNCHRONIZE: u32 = 1048576;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SYSTEM_ACCESS_FILTER_ACE {
    pub Header: ACE_HEADER,
    pub Mask: ACCESS_MASK,
    pub SidStart: u32,
}
pub const SYSTEM_ACCESS_FILTER_ACE_TYPE: u32 = 21;
pub const SYSTEM_ACCESS_FILTER_NOCONSTRAINT_MASK: u32 = 4294967295;
pub const SYSTEM_ACCESS_FILTER_VALID_MASK: u32 = 16777215;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SYSTEM_ALARM_ACE {
    pub Header: ACE_HEADER,
    pub Mask: ACCESS_MASK,
    pub SidStart: u32,
}
pub const SYSTEM_ALARM_ACE_TYPE: u32 = 3;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SYSTEM_ALARM_CALLBACK_ACE {
    pub Header: ACE_HEADER,
    pub Mask: ACCESS_MASK,
    pub SidStart: u32,
}
pub const SYSTEM_ALARM_CALLBACK_ACE_TYPE: u32 = 14;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SYSTEM_ALARM_CALLBACK_OBJECT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: ACCESS_MASK,
    pub Flags: u32,
    pub ObjectType: windows_core::GUID,
    pub InheritedObjectType: windows_core::GUID,
    pub SidStart: u32,
}
pub const SYSTEM_ALARM_CALLBACK_OBJECT_ACE_TYPE: u32 = 16;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SYSTEM_ALARM_OBJECT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: ACCESS_MASK,
    pub Flags: u32,
    pub ObjectType: windows_core::GUID,
    pub InheritedObjectType: windows_core::GUID,
    pub SidStart: u32,
}
pub const SYSTEM_ALARM_OBJECT_ACE_TYPE: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SYSTEM_AUDIT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: ACCESS_MASK,
    pub SidStart: u32,
}
pub const SYSTEM_AUDIT_ACE_TYPE: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SYSTEM_AUDIT_CALLBACK_ACE {
    pub Header: ACE_HEADER,
    pub Mask: ACCESS_MASK,
    pub SidStart: u32,
}
pub const SYSTEM_AUDIT_CALLBACK_ACE_TYPE: u32 = 13;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SYSTEM_AUDIT_CALLBACK_OBJECT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: ACCESS_MASK,
    pub Flags: u32,
    pub ObjectType: windows_core::GUID,
    pub InheritedObjectType: windows_core::GUID,
    pub SidStart: u32,
}
pub const SYSTEM_AUDIT_CALLBACK_OBJECT_ACE_TYPE: u32 = 15;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SYSTEM_AUDIT_OBJECT_ACE {
    pub Header: ACE_HEADER,
    pub Mask: ACCESS_MASK,
    pub Flags: u32,
    pub ObjectType: windows_core::GUID,
    pub InheritedObjectType: windows_core::GUID,
    pub SidStart: u32,
}
pub const SYSTEM_AUDIT_OBJECT_ACE_TYPE: u32 = 7;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SYSTEM_BATTERY_STATE {
    pub AcOnLine: bool,
    pub BatteryPresent: bool,
    pub Charging: bool,
    pub Discharging: bool,
    pub Spare1: [bool; 3],
    pub Tag: u8,
    pub MaxCapacity: u32,
    pub RemainingCapacity: u32,
    pub Rate: u32,
    pub EstimatedTime: u32,
    pub DefaultAlert1: u32,
    pub DefaultAlert2: u32,
}
impl Default for SYSTEM_BATTERY_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
pub const SYSTEM_CACHE_ALIGNMENT_SIZE: u32 = 64;
#[cfg(target_arch = "aarch64")]
pub const SYSTEM_CACHE_ALIGNMENT_SIZE: u32 = 128;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SYSTEM_CPU_SET_INFORMATION {
    pub Size: u32,
    pub Type: CPU_SET_INFORMATION_TYPE,
    pub Anonymous: SYSTEM_CPU_SET_INFORMATION_0,
}
impl Default for SYSTEM_CPU_SET_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SYSTEM_CPU_SET_INFORMATION_0 {
    pub CpuSet: SYSTEM_CPU_SET_INFORMATION_0_0,
}
impl Default for SYSTEM_CPU_SET_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SYSTEM_CPU_SET_INFORMATION_0_0 {
    pub Id: u32,
    pub Group: u16,
    pub LogicalProcessorIndex: u8,
    pub CoreIndex: u8,
    pub LastLevelCacheIndex: u8,
    pub NumaNodeIndex: u8,
    pub EfficiencyClass: u8,
    pub Anonymous: SYSTEM_CPU_SET_INFORMATION_0_0_0,
    pub Anonymous2: SYSTEM_CPU_SET_INFORMATION_0_0_1,
    pub AllocationTag: u64,
}
impl Default for SYSTEM_CPU_SET_INFORMATION_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SYSTEM_CPU_SET_INFORMATION_0_0_0 {
    pub AllFlags: u8,
    pub Anonymous: SYSTEM_CPU_SET_INFORMATION_0_0_0_0,
}
impl Default for SYSTEM_CPU_SET_INFORMATION_0_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SYSTEM_CPU_SET_INFORMATION_0_0_0_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SYSTEM_CPU_SET_INFORMATION_0_0_1 {
    pub Reserved: u32,
    pub SchedulingClass: u8,
}
impl Default for SYSTEM_CPU_SET_INFORMATION_0_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SYSTEM_CPU_SET_INFORMATION_ALLOCATED: u32 = 2;
pub const SYSTEM_CPU_SET_INFORMATION_ALLOCATED_TO_TARGET_PROCESS: u32 = 4;
pub const SYSTEM_CPU_SET_INFORMATION_PARKED: u32 = 1;
pub const SYSTEM_CPU_SET_INFORMATION_REALTIME: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SYSTEM_LOGICAL_PROCESSOR_INFORMATION {
    pub ProcessorMask: usize,
    pub Relationship: LOGICAL_PROCESSOR_RELATIONSHIP,
    pub Anonymous: SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0,
}
impl Default for SYSTEM_LOGICAL_PROCESSOR_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0 {
    pub ProcessorCore: SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_0,
    pub NumaNode: SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_1,
    pub Cache: CACHE_DESCRIPTOR,
    pub Reserved: [u64; 2],
}
impl Default for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_0 {
    pub Flags: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SYSTEM_LOGICAL_PROCESSOR_INFORMATION_0_1 {
    pub NodeNumber: u32,
}
#[repr(C)]
#[cfg(feature = "basetsd")]
#[derive(Clone, Copy)]
pub struct SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX {
    pub Relationship: LOGICAL_PROCESSOR_RELATIONSHIP,
    pub Size: u32,
    pub Anonymous: SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX_0,
}
#[cfg(feature = "basetsd")]
impl Default for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "basetsd")]
#[derive(Clone, Copy)]
pub union SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX_0 {
    pub Processor: PROCESSOR_RELATIONSHIP,
    pub NumaNode: NUMA_NODE_RELATIONSHIP,
    pub Cache: CACHE_RELATIONSHIP,
    pub Group: GROUP_RELATIONSHIP,
    pub SharedComputeUnit: SHARED_COMPUTE_UNIT_RELATIONSHIP,
}
#[cfg(feature = "basetsd")]
impl Default for SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SYSTEM_MANDATORY_LABEL_ACE {
    pub Header: ACE_HEADER,
    pub Mask: ACCESS_MASK,
    pub SidStart: u32,
}
pub const SYSTEM_MANDATORY_LABEL_ACE_TYPE: u32 = 17;
pub const SYSTEM_MANDATORY_LABEL_NO_EXECUTE_UP: u32 = 4;
pub const SYSTEM_MANDATORY_LABEL_NO_READ_UP: u32 = 2;
pub const SYSTEM_MANDATORY_LABEL_NO_WRITE_UP: u32 = 1;
pub const SYSTEM_MANDATORY_LABEL_VALID_MASK: u32 = 7;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SYSTEM_POOL_ZEROING_INFORMATION {
    pub PoolZeroingSupportPresent: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SYSTEM_POWER_CAPABILITIES {
    pub PowerButtonPresent: bool,
    pub SleepButtonPresent: bool,
    pub LidPresent: bool,
    pub SystemS1: bool,
    pub SystemS2: bool,
    pub SystemS3: bool,
    pub SystemS4: bool,
    pub SystemS5: bool,
    pub HiberFilePresent: bool,
    pub FullWake: bool,
    pub VideoDimPresent: bool,
    pub ApmPresent: bool,
    pub UpsPresent: bool,
    pub ThermalControl: bool,
    pub ProcessorThrottle: bool,
    pub ProcessorMinThrottle: u8,
    pub ProcessorMaxThrottle: u8,
    pub FastSystemS4: bool,
    pub Hiberboot: bool,
    pub WakeAlarmPresent: bool,
    pub AoAc: bool,
    pub DiskSpinDown: bool,
    pub HiberFileType: u8,
    pub AoAcConnectivitySupported: bool,
    pub spare3: [u8; 6],
    pub SystemBatteriesPresent: bool,
    pub BatteriesAreShortTerm: bool,
    pub BatteryScale: [BATTERY_REPORTING_SCALE; 3],
    pub AcOnLineWake: SYSTEM_POWER_STATE,
    pub SoftLidWake: SYSTEM_POWER_STATE,
    pub RtcWake: SYSTEM_POWER_STATE,
    pub MinDeviceWakeState: SYSTEM_POWER_STATE,
    pub DefaultLowLatencyWake: SYSTEM_POWER_STATE,
}
impl Default for SYSTEM_POWER_CAPABILITIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SYSTEM_POWER_CONDITION = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SYSTEM_POWER_LEVEL {
    pub Enable: bool,
    pub Spare: [u8; 3],
    pub BatteryLevel: u32,
    pub PowerPolicy: POWER_ACTION_POLICY,
    pub MinSystemState: SYSTEM_POWER_STATE,
}
impl Default for SYSTEM_POWER_LEVEL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SYSTEM_POWER_POLICY {
    pub Revision: u32,
    pub PowerButton: POWER_ACTION_POLICY,
    pub SleepButton: POWER_ACTION_POLICY,
    pub LidClose: POWER_ACTION_POLICY,
    pub LidOpenWake: SYSTEM_POWER_STATE,
    pub Reserved: u32,
    pub Idle: POWER_ACTION_POLICY,
    pub IdleTimeout: u32,
    pub IdleSensitivity: u8,
    pub DynamicThrottle: u8,
    pub Spare2: [u8; 2],
    pub MinSleep: SYSTEM_POWER_STATE,
    pub MaxSleep: SYSTEM_POWER_STATE,
    pub ReducedLatencySleep: SYSTEM_POWER_STATE,
    pub WinLogonFlags: u32,
    pub Spare3: u32,
    pub DozeS4Timeout: u32,
    pub BroadcastCapacityResolution: u32,
    pub DischargePolicy: [SYSTEM_POWER_LEVEL; 4],
    pub VideoTimeout: u32,
    pub VideoDimDisplay: bool,
    pub VideoReserved: [u32; 3],
    pub SpindownTimeout: u32,
    pub OptimizeForPower: bool,
    pub FanThrottleTolerance: u8,
    pub ForcedThrottle: u8,
    pub MinThrottle: u8,
    pub OverThrottled: POWER_ACTION_POLICY,
}
impl Default for SYSTEM_POWER_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SYSTEM_POWER_SOURCE_STATE {
    pub BatteryState: SYSTEM_BATTERY_STATE,
    pub InstantaneousPeakPower: u32,
    pub InstantaneousPeakPeriod: u32,
    pub SustainablePeakPower: u32,
    pub SustainablePeakPeriod: u32,
    pub PeakPower: u32,
    pub MaxOutputPower: u32,
    pub MaxInputPower: u32,
    pub BatteryRateInCurrent: i32,
    pub BatteryVoltage: u32,
}
pub type SYSTEM_POWER_STATE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SYSTEM_PROCESSOR_CYCLE_TIME_INFORMATION {
    pub CycleTime: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SYSTEM_PROCESS_TRUST_LABEL_ACE {
    pub Header: ACE_HEADER,
    pub Mask: ACCESS_MASK,
    pub SidStart: u32,
}
pub const SYSTEM_PROCESS_TRUST_LABEL_ACE_TYPE: u32 = 20;
pub const SYSTEM_PROCESS_TRUST_LABEL_VALID_MASK: u32 = 16777215;
pub const SYSTEM_PROCESS_TRUST_NOCONSTRAINT_MASK: u32 = 4294967295;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SYSTEM_RESOURCE_ATTRIBUTE_ACE {
    pub Header: ACE_HEADER,
    pub Mask: ACCESS_MASK,
    pub SidStart: u32,
}
pub const SYSTEM_RESOURCE_ATTRIBUTE_ACE_TYPE: u32 = 18;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SYSTEM_SCOPED_POLICY_ID_ACE {
    pub Header: ACE_HEADER,
    pub Mask: ACCESS_MASK,
    pub SidStart: u32,
}
pub const SYSTEM_SCOPED_POLICY_ID_ACE_TYPE: u32 = 19;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SYSTEM_SUPPORTED_PROCESSOR_ARCHITECTURES_INFORMATION {
    pub _bitfield: u32,
}
pub const ScreenOff: POWER_INFORMATION_LEVEL = 73;
pub const SeImageSignatureCache: SE_IMAGE_SIGNATURE_TYPE = 2;
pub const SeImageSignatureCatalogCached: SE_IMAGE_SIGNATURE_TYPE = 3;
pub const SeImageSignatureCatalogHint: SE_IMAGE_SIGNATURE_TYPE = 5;
pub const SeImageSignatureCatalogNotCached: SE_IMAGE_SIGNATURE_TYPE = 4;
pub const SeImageSignatureEmbedded: SE_IMAGE_SIGNATURE_TYPE = 1;
pub const SeImageSignatureNone: SE_IMAGE_SIGNATURE_TYPE = 0;
pub const SeImageSignaturePackageCatalog: SE_IMAGE_SIGNATURE_TYPE = 6;
pub const SeImageSignaturePplMitigated: SE_IMAGE_SIGNATURE_TYPE = 7;
pub const SecurityAnonymous: SECURITY_IMPERSONATION_LEVEL = 0;
pub const SecurityDelegation: SECURITY_IMPERSONATION_LEVEL = 3;
pub const SecurityIdentification: SECURITY_IMPERSONATION_LEVEL = 1;
pub const SecurityImpersonation: SECURITY_IMPERSONATION_LEVEL = 2;
pub const SendSuspendResumeNotification: POWER_INFORMATION_LEVEL = 96;
pub const SessionAllowExternalDmaDevices: POWER_INFORMATION_LEVEL = 95;
pub const SessionConnectNotification: POWER_INFORMATION_LEVEL = 62;
pub const SessionDisplayState: POWER_INFORMATION_LEVEL = 42;
pub const SessionLockState: POWER_INFORMATION_LEVEL = 64;
pub const SessionPowerCleanup: POWER_INFORMATION_LEVEL = 63;
pub const SessionPowerInit: POWER_INFORMATION_LEVEL = 41;
pub const SessionRITState: POWER_INFORMATION_LEVEL = 61;
pub const SetPowerSettingValue: POWER_INFORMATION_LEVEL = 25;
pub const SetShutdownSelectedTime: POWER_INFORMATION_LEVEL = 70;
pub const SevereError: SERVICE_ERROR_TYPE = 2;
pub const SharedComputeUnitArm64SMCU: PROCESSOR_SHARED_COMPUTE_UNIT_TYPE = 0;
pub const SharedVirtualDiskCDPSnapshotsSupported: SharedVirtualDiskSupportType = 7;
pub type SharedVirtualDiskHandleState = i32;
pub const SharedVirtualDiskHandleStateFileShared: SharedVirtualDiskHandleState = 1;
pub const SharedVirtualDiskHandleStateHandleShared: SharedVirtualDiskHandleState = 3;
pub const SharedVirtualDiskHandleStateNone: SharedVirtualDiskHandleState = 0;
pub const SharedVirtualDiskSnapshotsSupported: SharedVirtualDiskSupportType = 3;
pub type SharedVirtualDiskSupportType = i32;
pub const SharedVirtualDisksSupported: SharedVirtualDiskSupportType = 1;
pub const SharedVirtualDisksUnsupported: SharedVirtualDiskSupportType = 0;
pub const SidTypeAlias: SID_NAME_USE = 4;
pub const SidTypeComputer: SID_NAME_USE = 9;
pub const SidTypeDeletedAccount: SID_NAME_USE = 6;
pub const SidTypeDomain: SID_NAME_USE = 3;
pub const SidTypeGroup: SID_NAME_USE = 2;
pub const SidTypeInvalid: SID_NAME_USE = 7;
pub const SidTypeLabel: SID_NAME_USE = 10;
pub const SidTypeLogonSession: SID_NAME_USE = 11;
pub const SidTypeUnknown: SID_NAME_USE = 8;
pub const SidTypeUser: SID_NAME_USE = 1;
pub const SidTypeWellKnownGroup: SID_NAME_USE = 5;
pub const SuspendResumeInvocation: POWER_INFORMATION_LEVEL = 71;
pub const SystemBatteryState: POWER_INFORMATION_LEVEL = 5;
pub const SystemBatteryStatePrecise: POWER_INFORMATION_LEVEL = 83;
pub const SystemExecutionState: POWER_INFORMATION_LEVEL = 16;
pub const SystemHiberFileInformation: POWER_INFORMATION_LEVEL = 36;
pub const SystemHiberFileSize: POWER_INFORMATION_LEVEL = 51;
pub const SystemHiberFileType: POWER_INFORMATION_LEVEL = 89;
pub const SystemHiberbootState: POWER_INFORMATION_LEVEL = 65;
pub const SystemLoad: SERVICE_LOAD_TYPE = 1;
pub const SystemMonitorHiberBootPowerOff: POWER_INFORMATION_LEVEL = 28;
pub const SystemPowerCapabilities: POWER_INFORMATION_LEVEL = 4;
pub const SystemPowerInformation: POWER_INFORMATION_LEVEL = 12;
pub const SystemPowerLoggingEntry: POWER_INFORMATION_LEVEL = 24;
pub const SystemPowerPolicyAc: POWER_INFORMATION_LEVEL = 0;
pub const SystemPowerPolicyCurrent: POWER_INFORMATION_LEVEL = 8;
pub const SystemPowerPolicyDc: POWER_INFORMATION_LEVEL = 1;
pub const SystemPowerSourceState: POWER_INFORMATION_LEVEL = 98;
pub const SystemPowerStateHandler: POWER_INFORMATION_LEVEL = 6;
pub const SystemPowerStateLogging: POWER_INFORMATION_LEVEL = 23;
pub const SystemPowerStateNotifyHandler: POWER_INFORMATION_LEVEL = 17;
pub const SystemReserveHiberFile: POWER_INFORMATION_LEVEL = 10;
pub const SystemVideoState: POWER_INFORMATION_LEVEL = 29;
pub const SystemWakeSource: POWER_INFORMATION_LEVEL = 35;
pub const TAPE_ABSOLUTE_BLOCK: u32 = 1;
pub const TAPE_ABSOLUTE_POSITION: u32 = 0;
pub const TAPE_CHECK_FOR_DRIVE_PROBLEM: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TAPE_CREATE_PARTITION {
    pub Method: u32,
    pub Count: u32,
    pub Size: u32,
}
pub const TAPE_DRIVE_ABSOLUTE_BLK: u32 = 2147487744;
pub const TAPE_DRIVE_ABS_BLK_IMMED: u32 = 2147491840;
pub const TAPE_DRIVE_CLEAN_REQUESTS: u32 = 33554432;
pub const TAPE_DRIVE_COMPRESSION: u32 = 131072;
pub const TAPE_DRIVE_ECC: u32 = 65536;
pub const TAPE_DRIVE_EJECT_MEDIA: u32 = 16777216;
pub const TAPE_DRIVE_END_OF_DATA: u32 = 2147549184;
pub const TAPE_DRIVE_EOT_WZ_SIZE: u32 = 8192;
pub const TAPE_DRIVE_ERASE_BOP_ONLY: u32 = 64;
pub const TAPE_DRIVE_ERASE_IMMEDIATE: u32 = 128;
pub const TAPE_DRIVE_ERASE_LONG: u32 = 32;
pub const TAPE_DRIVE_ERASE_SHORT: u32 = 16;
pub const TAPE_DRIVE_FILEMARKS: u32 = 2147745792;
pub const TAPE_DRIVE_FIXED: u32 = 1;
pub const TAPE_DRIVE_FIXED_BLOCK: u32 = 1024;
pub const TAPE_DRIVE_FORMAT: u32 = 2684354560;
pub const TAPE_DRIVE_FORMAT_IMMEDIATE: u32 = 3221225472;
pub const TAPE_DRIVE_GET_ABSOLUTE_BLK: u32 = 1048576;
pub const TAPE_DRIVE_GET_LOGICAL_BLK: u32 = 2097152;
pub const TAPE_DRIVE_HIGH_FEATURES: u32 = 2147483648;
pub const TAPE_DRIVE_INITIATOR: u32 = 4;
pub const TAPE_DRIVE_LOAD_UNLD_IMMED: u32 = 2147483680;
pub const TAPE_DRIVE_LOAD_UNLOAD: u32 = 2147483649;
pub const TAPE_DRIVE_LOCK_UNLK_IMMED: u32 = 2147483776;
pub const TAPE_DRIVE_LOCK_UNLOCK: u32 = 2147483652;
pub const TAPE_DRIVE_LOGICAL_BLK: u32 = 2147500032;
pub const TAPE_DRIVE_LOG_BLK_IMMED: u32 = 2147516416;
pub const TAPE_DRIVE_PADDING: u32 = 262144;
pub type TAPE_DRIVE_PROBLEM_TYPE = i32;
pub const TAPE_DRIVE_RELATIVE_BLKS: u32 = 2147614720;
pub const TAPE_DRIVE_REPORT_SMKS: u32 = 524288;
pub const TAPE_DRIVE_RESERVED_BIT: u32 = 2147483648;
pub const TAPE_DRIVE_REVERSE_POSITION: u32 = 2151677952;
pub const TAPE_DRIVE_REWIND_IMMEDIATE: u32 = 2147483656;
pub const TAPE_DRIVE_SELECT: u32 = 2;
pub const TAPE_DRIVE_SEQUENTIAL_FMKS: u32 = 2148007936;
pub const TAPE_DRIVE_SEQUENTIAL_SMKS: u32 = 2149580800;
pub const TAPE_DRIVE_SETMARKS: u32 = 2148532224;
pub const TAPE_DRIVE_SET_BLOCK_SIZE: u32 = 2147483664;
pub const TAPE_DRIVE_SET_CMP_BOP_ONLY: u32 = 67108864;
pub const TAPE_DRIVE_SET_COMPRESSION: u32 = 2147484160;
pub const TAPE_DRIVE_SET_ECC: u32 = 2147483904;
pub const TAPE_DRIVE_SET_EOT_WZ_SIZE: u32 = 4194304;
pub const TAPE_DRIVE_SET_PADDING: u32 = 2147484672;
pub const TAPE_DRIVE_SET_REPORT_SMKS: u32 = 2147485696;
pub const TAPE_DRIVE_SPACE_IMMEDIATE: u32 = 2155872256;
pub const TAPE_DRIVE_TAPE_CAPACITY: u32 = 256;
pub const TAPE_DRIVE_TAPE_REMAINING: u32 = 512;
pub const TAPE_DRIVE_TENSION: u32 = 2147483650;
pub const TAPE_DRIVE_TENSION_IMMED: u32 = 2147483712;
pub const TAPE_DRIVE_VARIABLE_BLOCK: u32 = 2048;
pub const TAPE_DRIVE_WRITE_FILEMARKS: u32 = 2181038080;
pub const TAPE_DRIVE_WRITE_LONG_FMKS: u32 = 2281701376;
pub const TAPE_DRIVE_WRITE_MARK_IMMED: u32 = 2415919104;
pub const TAPE_DRIVE_WRITE_PROTECT: u32 = 4096;
pub const TAPE_DRIVE_WRITE_SETMARKS: u32 = 2164260864;
pub const TAPE_DRIVE_WRITE_SHORT_FMKS: u32 = 2214592512;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TAPE_ERASE {
    pub Type: u32,
    pub Immediate: bool,
}
pub const TAPE_ERASE_LONG: u32 = 1;
pub const TAPE_ERASE_SHORT: u32 = 0;
pub const TAPE_FILEMARKS: u32 = 1;
pub const TAPE_FIXED_PARTITIONS: u32 = 0;
pub const TAPE_FORMAT: u32 = 5;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TAPE_GET_DRIVE_PARAMETERS {
    pub ECC: bool,
    pub Compression: bool,
    pub DataPadding: bool,
    pub ReportSetmarks: bool,
    pub DefaultBlockSize: u32,
    pub MaximumBlockSize: u32,
    pub MinimumBlockSize: u32,
    pub MaximumPartitionCount: u32,
    pub FeaturesLow: u32,
    pub FeaturesHigh: u32,
    pub EOTWarningZoneSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TAPE_GET_MEDIA_PARAMETERS {
    pub Capacity: i64,
    pub Remaining: i64,
    pub BlockSize: u32,
    pub PartitionCount: u32,
    pub WriteProtected: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TAPE_GET_POSITION {
    pub Type: u32,
    pub Partition: u32,
    pub Offset: i64,
}
pub const TAPE_INITIATOR_PARTITIONS: u32 = 2;
pub const TAPE_LOAD: u32 = 0;
pub const TAPE_LOCK: u32 = 3;
pub const TAPE_LOGICAL_BLOCK: u32 = 2;
pub const TAPE_LOGICAL_POSITION: u32 = 1;
pub const TAPE_LONG_FILEMARKS: u32 = 3;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TAPE_PREPARE {
    pub Operation: u32,
    pub Immediate: bool,
}
pub const TAPE_PSEUDO_LOGICAL_BLOCK: u32 = 3;
pub const TAPE_PSEUDO_LOGICAL_POSITION: u32 = 2;
pub const TAPE_QUERY_DEVICE_ERROR_DATA: u32 = 4;
pub const TAPE_QUERY_DRIVE_PARAMETERS: u32 = 0;
pub const TAPE_QUERY_IO_ERROR_DATA: u32 = 3;
pub const TAPE_QUERY_MEDIA_CAPACITY: u32 = 1;
pub const TAPE_REWIND: u32 = 0;
pub const TAPE_SELECT_PARTITIONS: u32 = 1;
pub const TAPE_SETMARKS: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TAPE_SET_DRIVE_PARAMETERS {
    pub ECC: bool,
    pub Compression: bool,
    pub DataPadding: bool,
    pub ReportSetmarks: bool,
    pub EOTWarningZoneSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TAPE_SET_MEDIA_PARAMETERS {
    pub BlockSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TAPE_SET_POSITION {
    pub Method: u32,
    pub Partition: u32,
    pub Offset: i64,
    pub Immediate: bool,
}
pub const TAPE_SHORT_FILEMARKS: u32 = 2;
pub const TAPE_SPACE_END_OF_DATA: u32 = 4;
pub const TAPE_SPACE_FILEMARKS: u32 = 6;
pub const TAPE_SPACE_RELATIVE_BLOCKS: u32 = 5;
pub const TAPE_SPACE_SEQUENTIAL_FMKS: u32 = 7;
pub const TAPE_SPACE_SEQUENTIAL_SMKS: u32 = 9;
pub const TAPE_SPACE_SETMARKS: u32 = 8;
pub const TAPE_TENSION: u32 = 2;
pub const TAPE_UNLOAD: u32 = 1;
pub const TAPE_UNLOCK: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TAPE_WMI_OPERATIONS {
    pub Method: u32,
    pub DataBufferSize: u32,
    pub DataBuffer: *mut core::ffi::c_void,
}
impl Default for TAPE_WMI_OPERATIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TAPE_WRITE_MARKS {
    pub Type: u32,
    pub Count: u32,
    pub Immediate: bool,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct TBYTE(pub u8);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct TCHAR(pub i8);
pub const THREAD_ALL_ACCESS: u32 = 2097151;
pub const THREAD_BASE_PRIORITY_IDLE: i32 = -15;
pub const THREAD_BASE_PRIORITY_LOWRT: u32 = 15;
pub const THREAD_BASE_PRIORITY_MAX: u32 = 2;
pub const THREAD_BASE_PRIORITY_MIN: i32 = -2;
pub const THREAD_DIRECT_IMPERSONATION: u32 = 512;
pub const THREAD_DYNAMIC_CODE_ALLOW: u32 = 1;
pub const THREAD_GET_CONTEXT: u32 = 8;
pub const THREAD_IMPERSONATE: u32 = 256;
pub const THREAD_PROFILING_FLAG_DISPATCH: u32 = 1;
pub const THREAD_QUERY_INFORMATION: u32 = 64;
pub const THREAD_QUERY_LIMITED_INFORMATION: u32 = 2048;
pub const THREAD_RESUME: u32 = 4096;
pub const THREAD_SET_CONTEXT: u32 = 16;
pub const THREAD_SET_INFORMATION: u32 = 32;
pub const THREAD_SET_LIMITED_INFORMATION: u32 = 1024;
pub const THREAD_SET_THREAD_TOKEN: u32 = 128;
pub const THREAD_SUSPEND_RESUME: u32 = 2;
pub const THREAD_TERMINATE: u32 = 1;
pub const TIMER_ALL_ACCESS: u32 = 2031619;
pub const TIMER_MODIFY_STATE: u32 = 2;
pub const TIMER_QUERY_STATE: u32 = 1;
pub const TIME_ZONE_ID_DAYLIGHT: u32 = 2;
pub const TIME_ZONE_ID_STANDARD: u32 = 1;
pub const TIME_ZONE_ID_UNKNOWN: u32 = 0;
pub const TLS_MINIMUM_AVAILABLE: u32 = 64;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TOKEN_ACCESS_INFORMATION {
    pub SidHash: PSID_AND_ATTRIBUTES_HASH,
    pub RestrictedSidHash: PSID_AND_ATTRIBUTES_HASH,
    pub Privileges: PTOKEN_PRIVILEGES,
    pub AuthenticationId: LUID,
    pub TokenType: TOKEN_TYPE,
    pub ImpersonationLevel: SECURITY_IMPERSONATION_LEVEL,
    pub MandatoryPolicy: TOKEN_MANDATORY_POLICY,
    pub Flags: u32,
    pub AppContainerNumber: u32,
    pub PackageSid: PSID,
    pub CapabilitiesHash: PSID_AND_ATTRIBUTES_HASH,
    pub TrustLevelSid: PSID,
    pub SecurityAttributes: PSECURITY_ATTRIBUTES_OPAQUE,
}
pub const TOKEN_ACCESS_PSEUDO_HANDLE: u32 = 24;
pub const TOKEN_ACCESS_PSEUDO_HANDLE_WIN8: u32 = 24;
pub const TOKEN_ADJUST_DEFAULT: u32 = 128;
pub const TOKEN_ADJUST_GROUPS: u32 = 64;
pub const TOKEN_ADJUST_PRIVILEGES: u32 = 32;
pub const TOKEN_ADJUST_SESSIONID: u32 = 256;
pub const TOKEN_ALL_ACCESS: u32 = 983551;
pub const TOKEN_ALL_ACCESS_P: u32 = 983295;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TOKEN_APPCONTAINER_INFORMATION {
    pub TokenAppContainer: PSID,
}
pub const TOKEN_ASSIGN_PRIMARY: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TOKEN_AUDIT_POLICY {
    pub PerUserPolicy: [u8; 31],
}
impl Default for TOKEN_AUDIT_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TOKEN_BNO_ISOLATION_INFORMATION {
    pub IsolationPrefix: windows_core::PWSTR,
    pub IsolationEnabled: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TOKEN_CONTROL {
    pub TokenId: LUID,
    pub AuthenticationId: LUID,
    pub ModifiedId: LUID,
    pub TokenSource: TOKEN_SOURCE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TOKEN_DEFAULT_DACL {
    pub DefaultDacl: PACL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TOKEN_DEVICE_CLAIMS {
    pub DeviceClaims: PCLAIMS_BLOB,
}
pub const TOKEN_DUPLICATE: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TOKEN_ELEVATION {
    pub TokenIsElevated: u32,
}
pub type TOKEN_ELEVATION_TYPE = i32;
pub const TOKEN_EXECUTE: u32 = 131072;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TOKEN_GROUPS {
    pub GroupCount: u32,
    pub Groups: [SID_AND_ATTRIBUTES; 1],
}
impl Default for TOKEN_GROUPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TOKEN_GROUPS_AND_PRIVILEGES {
    pub SidCount: u32,
    pub SidLength: u32,
    pub Sids: PSID_AND_ATTRIBUTES,
    pub RestrictedSidCount: u32,
    pub RestrictedSidLength: u32,
    pub RestrictedSids: PSID_AND_ATTRIBUTES,
    pub PrivilegeCount: u32,
    pub PrivilegeLength: u32,
    pub Privileges: PLUID_AND_ATTRIBUTES,
    pub AuthenticationId: LUID,
}
pub const TOKEN_IMPERSONATE: u32 = 4;
pub type TOKEN_INFORMATION_CLASS = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TOKEN_LINKED_TOKEN {
    pub LinkedToken: HANDLE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TOKEN_LOGGING_INFORMATION {
    pub TokenType: TOKEN_TYPE,
    pub TokenElevation: TOKEN_ELEVATION,
    pub TokenElevationType: TOKEN_ELEVATION_TYPE,
    pub ImpersonationLevel: SECURITY_IMPERSONATION_LEVEL,
    pub IntegrityLevel: u32,
    pub User: SID_AND_ATTRIBUTES,
    pub TrustLevelSid: PSID,
    pub SessionId: u32,
    pub AppContainerNumber: u32,
    pub AuthenticationId: LUID,
    pub GroupCount: u32,
    pub GroupsLength: u32,
    pub Groups: PSID_AND_ATTRIBUTES,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TOKEN_MANDATORY_LABEL {
    pub Label: SID_AND_ATTRIBUTES,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TOKEN_MANDATORY_POLICY {
    pub Policy: u32,
}
pub const TOKEN_MANDATORY_POLICY_NEW_PROCESS_MIN: u32 = 2;
pub const TOKEN_MANDATORY_POLICY_NO_WRITE_UP: u32 = 1;
pub const TOKEN_MANDATORY_POLICY_OFF: u32 = 0;
pub const TOKEN_MANDATORY_POLICY_VALID_MASK: u32 = 3;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TOKEN_ORIGIN {
    pub OriginatingLogonSession: LUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TOKEN_OWNER {
    pub Owner: PSID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TOKEN_PRIMARY_GROUP {
    pub PrimaryGroup: PSID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TOKEN_PRIVILEGES {
    pub PrivilegeCount: u32,
    pub Privileges: [LUID_AND_ATTRIBUTES; 1],
}
impl Default for TOKEN_PRIVILEGES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TOKEN_QUERY: u32 = 8;
pub const TOKEN_QUERY_SOURCE: u32 = 16;
pub const TOKEN_READ: u32 = 131080;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TOKEN_SID_INFORMATION {
    pub Sid: PSID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TOKEN_SOURCE {
    pub SourceName: [i8; 8],
    pub SourceIdentifier: LUID,
}
impl Default for TOKEN_SOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TOKEN_SOURCE_LENGTH: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TOKEN_STATISTICS {
    pub TokenId: LUID,
    pub AuthenticationId: LUID,
    pub ExpirationTime: i64,
    pub TokenType: TOKEN_TYPE,
    pub ImpersonationLevel: SECURITY_IMPERSONATION_LEVEL,
    pub DynamicCharged: u32,
    pub DynamicAvailable: u32,
    pub GroupCount: u32,
    pub PrivilegeCount: u32,
    pub ModifiedId: LUID,
}
pub const TOKEN_TRUST_ALLOWED_MASK: u32 = 131102;
pub const TOKEN_TRUST_CONSTRAINT_MASK: u32 = 131096;
pub type TOKEN_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TOKEN_USER {
    pub User: SID_AND_ATTRIBUTES,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TOKEN_USER_CLAIMS {
    pub UserClaims: PCLAIMS_BLOB,
}
pub const TOKEN_WRITE: u32 = 131296;
pub type TP_CALLBACK_ENVIRON = TP_CALLBACK_ENVIRON_V3;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TP_CALLBACK_ENVIRON_V3 {
    pub Version: TP_VERSION,
    pub Pool: PTP_POOL,
    pub CleanupGroup: PTP_CLEANUP_GROUP,
    pub CleanupGroupCancelCallback: PTP_CLEANUP_GROUP_CANCEL_CALLBACK,
    pub RaceDll: *mut core::ffi::c_void,
    pub ActivationContext: *mut _ACTIVATION_CONTEXT,
    pub FinalizationCallback: PTP_SIMPLE_CALLBACK,
    pub u: TP_CALLBACK_ENVIRON_V3_0,
    pub CallbackPriority: TP_CALLBACK_PRIORITY,
    pub Size: u32,
}
impl Default for TP_CALLBACK_ENVIRON_V3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union TP_CALLBACK_ENVIRON_V3_0 {
    pub Flags: u32,
    pub s: TP_CALLBACK_ENVIRON_V3_0_0,
}
impl Default for TP_CALLBACK_ENVIRON_V3_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TP_CALLBACK_ENVIRON_V3_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TP_CALLBACK_INSTANCE(pub u8);
pub type TP_CALLBACK_PRIORITY = i32;
pub const TP_CALLBACK_PRIORITY_COUNT: TP_CALLBACK_PRIORITY = 3;
pub const TP_CALLBACK_PRIORITY_HIGH: TP_CALLBACK_PRIORITY = 0;
pub const TP_CALLBACK_PRIORITY_INVALID: TP_CALLBACK_PRIORITY = 3;
pub const TP_CALLBACK_PRIORITY_LOW: TP_CALLBACK_PRIORITY = 2;
pub const TP_CALLBACK_PRIORITY_NORMAL: TP_CALLBACK_PRIORITY = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TP_CLEANUP_GROUP(pub u8);
pub type TP_CLEANUP_GROUP_CANCEL_CALLBACK = Option<unsafe extern "system" fn(objectcontext: *mut core::ffi::c_void, cleanupcontext: *mut core::ffi::c_void)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TP_IO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TP_POOL(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TP_POOL_STACK_INFORMATION {
    pub StackReserve: usize,
    pub StackCommit: usize,
}
pub type TP_SIMPLE_CALLBACK = Option<unsafe extern "system" fn(instance: *mut TP_CALLBACK_INSTANCE, context: *mut core::ffi::c_void)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TP_TIMER(pub u8);
pub type TP_TIMER_CALLBACK = Option<unsafe extern "system" fn(instance: *mut TP_CALLBACK_INSTANCE, context: *mut core::ffi::c_void, timer: *mut TP_TIMER)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct TP_VERSION(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TP_WAIT(pub u8);
pub type TP_WAIT_CALLBACK = Option<unsafe extern "system" fn(instance: *mut TP_CALLBACK_INSTANCE, context: *mut core::ffi::c_void, wait: *mut TP_WAIT, waitresult: TP_WAIT_RESULT)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct TP_WAIT_RESULT(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TP_WORK(pub u8);
pub type TP_WORK_CALLBACK = Option<unsafe extern "system" fn(instance: *mut TP_CALLBACK_INSTANCE, context: *mut core::ffi::c_void, work: *mut TP_WORK)>;
pub const TRANSACTIONMANAGER_ALL_ACCESS: u32 = 983103;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TRANSACTIONMANAGER_BASIC_INFORMATION {
    pub TmIdentity: windows_core::GUID,
    pub VirtualClock: i64,
}
pub const TRANSACTIONMANAGER_BIND_TRANSACTION: u32 = 32;
pub const TRANSACTIONMANAGER_CREATE_RM: u32 = 16;
pub const TRANSACTIONMANAGER_GENERIC_EXECUTE: u32 = 131072;
pub const TRANSACTIONMANAGER_GENERIC_READ: u32 = 131073;
pub const TRANSACTIONMANAGER_GENERIC_WRITE: u32 = 131102;
pub type TRANSACTIONMANAGER_INFORMATION_CLASS = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TRANSACTIONMANAGER_LOGPATH_INFORMATION {
    pub LogPathLength: u32,
    pub LogPath: [u16; 1],
}
impl Default for TRANSACTIONMANAGER_LOGPATH_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TRANSACTIONMANAGER_LOG_INFORMATION {
    pub LogIdentity: windows_core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TRANSACTIONMANAGER_OLDEST_INFORMATION {
    pub OldestTransactionGuid: windows_core::GUID,
}
pub const TRANSACTIONMANAGER_QUERY_INFORMATION: u32 = 1;
pub const TRANSACTIONMANAGER_RECOVER: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TRANSACTIONMANAGER_RECOVERY_INFORMATION {
    pub LastRecoveredLsn: u64,
}
pub const TRANSACTIONMANAGER_RENAME: u32 = 8;
pub const TRANSACTIONMANAGER_SET_INFORMATION: u32 = 2;
pub const TRANSACTION_ALL_ACCESS: u32 = 2031679;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TRANSACTION_BASIC_INFORMATION {
    pub TransactionId: windows_core::GUID,
    pub State: u32,
    pub Outcome: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TRANSACTION_BIND_INFORMATION {
    pub TmHandle: HANDLE,
}
pub const TRANSACTION_COMMIT: u32 = 8;
pub const TRANSACTION_ENLIST: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TRANSACTION_ENLISTMENTS_INFORMATION {
    pub NumberOfEnlistments: u32,
    pub EnlistmentPair: [TRANSACTION_ENLISTMENT_PAIR; 1],
}
impl Default for TRANSACTION_ENLISTMENTS_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TRANSACTION_ENLISTMENT_PAIR {
    pub EnlistmentId: windows_core::GUID,
    pub ResourceManagerId: windows_core::GUID,
}
pub const TRANSACTION_GENERIC_EXECUTE: u32 = 1179672;
pub const TRANSACTION_GENERIC_READ: u32 = 1179649;
pub const TRANSACTION_GENERIC_WRITE: u32 = 1179710;
pub type TRANSACTION_INFORMATION_CLASS = i32;
#[repr(C)]
#[cfg(feature = "ktmtypes")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TRANSACTION_LIST_ENTRY {
    pub UOW: super::ktmtypes::UOW,
}
#[repr(C)]
#[cfg(feature = "ktmtypes")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TRANSACTION_LIST_INFORMATION {
    pub NumberOfTransactions: u32,
    pub TransactionInformation: [TRANSACTION_LIST_ENTRY; 1],
}
#[cfg(feature = "ktmtypes")]
impl Default for TRANSACTION_LIST_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type TRANSACTION_OUTCOME = i32;
pub const TRANSACTION_PROPAGATE: u32 = 32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TRANSACTION_PROPERTIES_INFORMATION {
    pub IsolationLevel: u32,
    pub IsolationFlags: u32,
    pub Timeout: i64,
    pub Outcome: u32,
    pub DescriptionLength: u32,
    pub Description: [u16; 1],
}
impl Default for TRANSACTION_PROPERTIES_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TRANSACTION_QUERY_INFORMATION: u32 = 1;
pub const TRANSACTION_RESOURCE_MANAGER_RIGHTS: u32 = 1179703;
pub const TRANSACTION_RIGHT_RESERVED1: u32 = 64;
pub const TRANSACTION_ROLLBACK: u32 = 16;
pub const TRANSACTION_SET_INFORMATION: u32 = 2;
pub type TRANSACTION_STATE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TRANSACTION_SUPERIOR_ENLISTMENT_INFORMATION {
    pub SuperiorEnlistmentPair: TRANSACTION_ENLISTMENT_PAIR,
}
pub const TREE_CONNECT_ATTRIBUTE_GLOBAL: u32 = 4;
pub const TREE_CONNECT_ATTRIBUTE_INTEGRITY: u32 = 32768;
pub const TREE_CONNECT_ATTRIBUTE_PINNED: u32 = 2;
pub const TREE_CONNECT_ATTRIBUTE_PRIVACY: u32 = 16384;
pub const TRUST_PROTECTED_FILTER_ACE_FLAG: u32 = 64;
pub const TapeDriveCleanDriveNow: TAPE_DRIVE_PROBLEM_TYPE = 11;
pub const TapeDriveHardwareError: TAPE_DRIVE_PROBLEM_TYPE = 7;
pub const TapeDriveMediaLifeExpired: TAPE_DRIVE_PROBLEM_TYPE = 12;
pub const TapeDriveProblemNone: TAPE_DRIVE_PROBLEM_TYPE = 0;
pub const TapeDriveReadError: TAPE_DRIVE_PROBLEM_TYPE = 5;
pub const TapeDriveReadWarning: TAPE_DRIVE_PROBLEM_TYPE = 3;
pub const TapeDriveReadWriteError: TAPE_DRIVE_PROBLEM_TYPE = 2;
pub const TapeDriveReadWriteWarning: TAPE_DRIVE_PROBLEM_TYPE = 1;
pub const TapeDriveScsiConnectionError: TAPE_DRIVE_PROBLEM_TYPE = 9;
pub const TapeDriveSnappedTape: TAPE_DRIVE_PROBLEM_TYPE = 13;
pub const TapeDriveTimetoClean: TAPE_DRIVE_PROBLEM_TYPE = 10;
pub const TapeDriveUnsupportedMedia: TAPE_DRIVE_PROBLEM_TYPE = 8;
pub const TapeDriveWriteError: TAPE_DRIVE_PROBLEM_TYPE = 6;
pub const TapeDriveWriteWarning: TAPE_DRIVE_PROBLEM_TYPE = 4;
pub const ThermalEvent: POWER_INFORMATION_LEVEL = 84;
pub const ThermalStandby: POWER_INFORMATION_LEVEL = 88;
pub const TokenAccessInformation: TOKEN_INFORMATION_CLASS = 22;
pub const TokenAppContainerNumber: TOKEN_INFORMATION_CLASS = 32;
pub const TokenAppContainerSid: TOKEN_INFORMATION_CLASS = 31;
pub const TokenAuditPolicy: TOKEN_INFORMATION_CLASS = 16;
pub const TokenBnoIsolation: TOKEN_INFORMATION_CLASS = 44;
pub const TokenCapabilities: TOKEN_INFORMATION_CLASS = 30;
pub const TokenChildProcessFlags: TOKEN_INFORMATION_CLASS = 45;
pub const TokenDefaultDacl: TOKEN_INFORMATION_CLASS = 6;
pub const TokenDeviceClaimAttributes: TOKEN_INFORMATION_CLASS = 34;
pub const TokenDeviceGroups: TOKEN_INFORMATION_CLASS = 37;
pub const TokenElevation: TOKEN_INFORMATION_CLASS = 20;
pub const TokenElevationType: TOKEN_INFORMATION_CLASS = 18;
pub const TokenElevationTypeDefault: TOKEN_ELEVATION_TYPE = 1;
pub const TokenElevationTypeFull: TOKEN_ELEVATION_TYPE = 2;
pub const TokenElevationTypeLimited: TOKEN_ELEVATION_TYPE = 3;
pub const TokenGroups: TOKEN_INFORMATION_CLASS = 2;
pub const TokenGroupsAndPrivileges: TOKEN_INFORMATION_CLASS = 13;
pub const TokenHasRestrictions: TOKEN_INFORMATION_CLASS = 21;
pub const TokenImpersonation: TOKEN_TYPE = 2;
pub const TokenImpersonationLevel: TOKEN_INFORMATION_CLASS = 9;
pub const TokenIntegrityLevel: TOKEN_INFORMATION_CLASS = 25;
pub const TokenIsAppContainer: TOKEN_INFORMATION_CLASS = 29;
pub const TokenIsAppSilo: TOKEN_INFORMATION_CLASS = 48;
pub const TokenIsInstaller: TOKEN_INFORMATION_CLASS = 52;
pub const TokenIsLessPrivilegedAppContainer: TOKEN_INFORMATION_CLASS = 46;
pub const TokenIsRestricted: TOKEN_INFORMATION_CLASS = 40;
pub const TokenIsSandboxed: TOKEN_INFORMATION_CLASS = 47;
pub const TokenIsSystemManagedAdmin: TOKEN_INFORMATION_CLASS = 51;
pub const TokenLearningMode: TOKEN_INFORMATION_CLASS = 50;
pub const TokenLinkedToken: TOKEN_INFORMATION_CLASS = 19;
pub const TokenLoggingInformation: TOKEN_INFORMATION_CLASS = 49;
pub const TokenLogonSid: TOKEN_INFORMATION_CLASS = 28;
pub const TokenMandatoryPolicy: TOKEN_INFORMATION_CLASS = 27;
pub const TokenOrigin: TOKEN_INFORMATION_CLASS = 17;
pub const TokenOwner: TOKEN_INFORMATION_CLASS = 4;
pub const TokenPrimary: TOKEN_TYPE = 1;
pub const TokenPrimaryGroup: TOKEN_INFORMATION_CLASS = 5;
pub const TokenPrivateNameSpace: TOKEN_INFORMATION_CLASS = 42;
pub const TokenPrivileges: TOKEN_INFORMATION_CLASS = 3;
pub const TokenProcessTrustLevel: TOKEN_INFORMATION_CLASS = 41;
pub const TokenRestrictedDeviceClaimAttributes: TOKEN_INFORMATION_CLASS = 36;
pub const TokenRestrictedDeviceGroups: TOKEN_INFORMATION_CLASS = 38;
pub const TokenRestrictedSids: TOKEN_INFORMATION_CLASS = 11;
pub const TokenRestrictedUserClaimAttributes: TOKEN_INFORMATION_CLASS = 35;
pub const TokenSandBoxInert: TOKEN_INFORMATION_CLASS = 15;
pub const TokenSecurityAttributes: TOKEN_INFORMATION_CLASS = 39;
pub const TokenSessionId: TOKEN_INFORMATION_CLASS = 12;
pub const TokenSessionReference: TOKEN_INFORMATION_CLASS = 14;
pub const TokenSingletonAttributes: TOKEN_INFORMATION_CLASS = 43;
pub const TokenSource: TOKEN_INFORMATION_CLASS = 7;
pub const TokenStatistics: TOKEN_INFORMATION_CLASS = 10;
pub const TokenType: TOKEN_INFORMATION_CLASS = 8;
pub const TokenUIAccess: TOKEN_INFORMATION_CLASS = 26;
pub const TokenUser: TOKEN_INFORMATION_CLASS = 1;
pub const TokenUserClaimAttributes: TOKEN_INFORMATION_CLASS = 33;
pub const TokenVirtualizationAllowed: TOKEN_INFORMATION_CLASS = 23;
pub const TokenVirtualizationEnabled: TOKEN_INFORMATION_CLASS = 24;
pub const ToleranceHigh: JOBOBJECT_RATE_CONTROL_TOLERANCE = 3;
pub const ToleranceIntervalLong: JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL = 3;
pub const ToleranceIntervalMedium: JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL = 2;
pub const ToleranceIntervalShort: JOBOBJECT_RATE_CONTROL_TOLERANCE_INTERVAL = 1;
pub const ToleranceLow: JOBOBJECT_RATE_CONTROL_TOLERANCE = 1;
pub const ToleranceMedium: JOBOBJECT_RATE_CONTROL_TOLERANCE = 2;
pub const TraceApplicationPowerMessage: POWER_INFORMATION_LEVEL = 30;
pub const TraceApplicationPowerMessageEnd: POWER_INFORMATION_LEVEL = 31;
pub const TraceServicePowerMessage: POWER_INFORMATION_LEVEL = 37;
pub const TransactionBasicInformation: TRANSACTION_INFORMATION_CLASS = 0;
pub const TransactionBindInformation: TRANSACTION_INFORMATION_CLASS = 4;
pub const TransactionDTCPrivateInformation: TRANSACTION_INFORMATION_CLASS = 5;
pub const TransactionEnlistmentInformation: TRANSACTION_INFORMATION_CLASS = 2;
pub const TransactionManagerBasicInformation: TRANSACTIONMANAGER_INFORMATION_CLASS = 0;
pub const TransactionManagerLogInformation: TRANSACTIONMANAGER_INFORMATION_CLASS = 1;
pub const TransactionManagerLogPathInformation: TRANSACTIONMANAGER_INFORMATION_CLASS = 2;
pub const TransactionManagerOldestTransactionInformation: TRANSACTIONMANAGER_INFORMATION_CLASS = 5;
pub const TransactionManagerOnlineProbeInformation: TRANSACTIONMANAGER_INFORMATION_CLASS = 3;
pub const TransactionManagerRecoveryInformation: TRANSACTIONMANAGER_INFORMATION_CLASS = 4;
pub const TransactionOutcomeAborted: TRANSACTION_OUTCOME = 3;
pub const TransactionOutcomeCommitted: TRANSACTION_OUTCOME = 2;
pub const TransactionOutcomeUndetermined: TRANSACTION_OUTCOME = 1;
pub const TransactionPropertiesInformation: TRANSACTION_INFORMATION_CLASS = 1;
pub const TransactionStateCommittedNotify: TRANSACTION_STATE = 3;
pub const TransactionStateIndoubt: TRANSACTION_STATE = 2;
pub const TransactionStateNormal: TRANSACTION_STATE = 1;
pub const TransactionSuperiorEnlistmentInformation: TRANSACTION_INFORMATION_CLASS = 3;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct UCSCHAR(pub u32);
pub const UCSCHAR_INVALID_CHARACTER: u32 = 4294967295;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UMS_CREATE_THREAD_ATTRIBUTES {
    pub UmsVersion: u32,
    pub UmsContext: *mut core::ffi::c_void,
    pub UmsCompletionList: *mut core::ffi::c_void,
}
impl Default for UMS_CREATE_THREAD_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const UNICODE_NULL: u16 = 0;
pub const UNICODE_STRING_MAX_BYTES: u16 = 65534;
pub const UNICODE_STRING_MAX_CHARS: u32 = 32767;
pub const UNIFIEDBUILDREVISION_KEY: windows_core::PCWSTR = windows_core::w!("\\Registry\\Machine\\Software\\Microsoft\\Windows NT\\CurrentVersion");
pub const UNIFIEDBUILDREVISION_MIN: u32 = 0;
pub const UNIFIEDBUILDREVISION_VALUE: windows_core::PCWSTR = windows_core::w!("UBR");
pub const UNPROTECTED_DACL_SECURITY_INFORMATION: u32 = 536870912;
pub const UNPROTECTED_SACL_SECURITY_INFORMATION: u32 = 268435456;
pub const UNSPECIFIED_COMPARTMENT_ID: COMPARTMENT_ID = 0;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const UNWIND_CHAIN_LIMIT: u32 = 32;
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UNWIND_HISTORY_TABLE {
    pub Count: u32,
    pub LocalHint: u8,
    pub GlobalHint: u8,
    pub Search: u8,
    pub Once: u8,
    pub LowAddress: usize,
    pub HighAddress: usize,
    pub Entry: [UNWIND_HISTORY_TABLE_ENTRY; 12],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for UNWIND_HISTORY_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UNWIND_HISTORY_TABLE_ENTRY {
    pub ImageBase: usize,
    pub FunctionEntry: PRUNTIME_FUNCTION,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const UNWIND_HISTORY_TABLE_SIZE: u32 = 12;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const UNW_FLAG_CHAININFO: u32 = 4;
pub const UNW_FLAG_EHANDLER: u32 = 1;
pub const UNW_FLAG_NHANDLER: u32 = 0;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const UNW_FLAG_NO_EPILOGUE: u32 = 2147483648;
pub const UNW_FLAG_UHANDLER: u32 = 2;
pub type USER_ACTIVITY_PRESENCE = i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct USN(pub i64);
pub const UmsSchedulerStartup: RTL_UMS_SCHEDULER_REASON = 0;
pub const UmsSchedulerThreadBlocked: RTL_UMS_SCHEDULER_REASON = 1;
pub const UmsSchedulerThreadYield: RTL_UMS_SCHEDULER_REASON = 2;
pub const UmsThreadAffinity: RTL_UMS_THREAD_INFO_CLASS = 3;
pub const UmsThreadInvalidInfoClass: RTL_UMS_THREAD_INFO_CLASS = 0;
pub const UmsThreadIsSuspended: RTL_UMS_THREAD_INFO_CLASS = 5;
pub const UmsThreadIsTerminated: RTL_UMS_THREAD_INFO_CLASS = 6;
pub const UmsThreadMaxInfoClass: RTL_UMS_THREAD_INFO_CLASS = 7;
pub const UmsThreadPriority: RTL_UMS_THREAD_INFO_CLASS = 2;
pub const UmsThreadTeb: RTL_UMS_THREAD_INFO_CLASS = 4;
pub const UmsThreadUserContext: RTL_UMS_THREAD_INFO_CLASS = 1;
pub const UpdateBlackBoxRecorder: POWER_INFORMATION_LEVEL = 94;
pub const UserNotPresent: POWER_USER_PRESENCE_TYPE = 0;
pub const UserPresence: POWER_INFORMATION_LEVEL = 57;
pub const UserPresent: POWER_USER_PRESENCE_TYPE = 1;
pub const UserUnknown: POWER_USER_PRESENCE_TYPE = 255;
pub const VALID_INHERIT_FLAGS: u32 = 31;
pub const VBS_BASIC_PAGE_MEASURED_DATA: u32 = 1;
pub const VBS_BASIC_PAGE_SYSTEM_CALL: u32 = 5;
pub const VBS_BASIC_PAGE_THREAD_DESCRIPTOR: u32 = 4;
pub const VBS_BASIC_PAGE_UNMEASURED_DATA: u32 = 2;
pub const VBS_BASIC_PAGE_ZERO_FILL: u32 = 3;
pub const VER_AND: u32 = 6;
pub const VER_BUILDNUMBER: u32 = 4;
pub const VER_CONDITION_MASK: u32 = 7;
pub const VER_EQUAL: u32 = 1;
pub const VER_GREATER: u32 = 2;
pub const VER_GREATER_EQUAL: u32 = 3;
pub const VER_LESS: u32 = 4;
pub const VER_LESS_EQUAL: u32 = 5;
pub const VER_MAJORVERSION: u32 = 2;
pub const VER_MINORVERSION: u32 = 1;
pub const VER_NT_DOMAIN_CONTROLLER: u32 = 2;
pub const VER_NT_SERVER: u32 = 3;
pub const VER_NT_WORKSTATION: u32 = 1;
pub const VER_NUM_BITS_PER_CONDITION_MASK: u32 = 3;
pub const VER_OR: u32 = 7;
pub const VER_PLATFORMID: u32 = 8;
pub const VER_PLATFORM_WIN32_NT: u32 = 2;
pub const VER_PLATFORM_WIN32_WINDOWS: u32 = 1;
pub const VER_PLATFORM_WIN32s: u32 = 0;
pub const VER_PRODUCT_TYPE: u32 = 128;
pub const VER_SERVER_NT: u32 = 2147483648;
pub const VER_SERVICEPACKMAJOR: u32 = 32;
pub const VER_SERVICEPACKMINOR: u32 = 16;
pub const VER_SUITENAME: u32 = 64;
pub const VER_SUITE_BACKOFFICE: u32 = 4;
pub const VER_SUITE_BLADE: u32 = 1024;
pub const VER_SUITE_COMMUNICATIONS: u32 = 8;
pub const VER_SUITE_COMPUTE_SERVER: u32 = 16384;
pub const VER_SUITE_DATACENTER: u32 = 128;
pub const VER_SUITE_EMBEDDEDNT: u32 = 64;
pub const VER_SUITE_EMBEDDED_RESTRICTED: u32 = 2048;
pub const VER_SUITE_ENTERPRISE: u32 = 2;
pub const VER_SUITE_MULTIUSERTS: u32 = 131072;
pub const VER_SUITE_PERSONAL: u32 = 512;
pub const VER_SUITE_SECURITY_APPLIANCE: u32 = 4096;
pub const VER_SUITE_SINGLEUSERTS: u32 = 256;
pub const VER_SUITE_SMALLBUSINESS: u32 = 1;
pub const VER_SUITE_SMALLBUSINESS_RESTRICTED: u32 = 32;
pub const VER_SUITE_STORAGE_SERVER: u32 = 8192;
pub const VER_SUITE_TERMINAL: u32 = 16;
pub const VER_SUITE_WH_SERVER: u32 = 32768;
pub const VER_WORKSTATION_NT: u32 = 1073741824;
pub const VM_PREFETCH_TO_WORKING_SET: u32 = 1;
pub const VRL_CLASS_CONSISTENCY: u32 = 1;
pub const VRL_CUSTOM_CLASS_BEGIN: u32 = 256;
pub const VRL_ENABLE_KERNEL_BREAKS: i32 = -2147483648;
pub const VRL_PREDEFINED_CLASS_BEGIN: u32 = 1;
pub const VerifyProcessorPowerPolicyAc: POWER_INFORMATION_LEVEL = 20;
pub const VerifyProcessorPowerPolicyDc: POWER_INFORMATION_LEVEL = 21;
pub const VerifySystemPolicyAc: POWER_INFORMATION_LEVEL = 2;
pub const VerifySystemPolicyDc: POWER_INFORMATION_LEVEL = 3;
pub type WAITORTIMERCALLBACK = WAITORTIMERCALLBACKFUNC;
pub type WAITORTIMERCALLBACKFUNC = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void, param1: bool)>;
pub type WELL_KNOWN_SID_TYPE = i32;
pub type WORKERCALLBACKFUNC = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WOW64_CONTEXT {
    pub ContextFlags: u32,
    pub Dr0: u32,
    pub Dr1: u32,
    pub Dr2: u32,
    pub Dr3: u32,
    pub Dr6: u32,
    pub Dr7: u32,
    pub FloatSave: WOW64_FLOATING_SAVE_AREA,
    pub SegGs: u32,
    pub SegFs: u32,
    pub SegEs: u32,
    pub SegDs: u32,
    pub Edi: u32,
    pub Esi: u32,
    pub Ebx: u32,
    pub Edx: u32,
    pub Ecx: u32,
    pub Eax: u32,
    pub Ebp: u32,
    pub Eip: u32,
    pub SegCs: u32,
    pub EFlags: u32,
    pub Esp: u32,
    pub SegSs: u32,
    pub ExtendedRegisters: [u8; 512],
}
impl Default for WOW64_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WOW64_CONTEXT_ALL: u32 = 65599;
pub const WOW64_CONTEXT_CONTROL: u32 = 65537;
pub const WOW64_CONTEXT_DEBUG_REGISTERS: u32 = 65552;
pub const WOW64_CONTEXT_EXCEPTION_ACTIVE: u32 = 134217728;
pub const WOW64_CONTEXT_EXCEPTION_REPORTING: u32 = 2147483648;
pub const WOW64_CONTEXT_EXCEPTION_REQUEST: u32 = 1073741824;
pub const WOW64_CONTEXT_EXTENDED_REGISTERS: u32 = 65568;
pub const WOW64_CONTEXT_FLOATING_POINT: u32 = 65544;
pub const WOW64_CONTEXT_FULL: u32 = 65543;
pub const WOW64_CONTEXT_INTEGER: u32 = 65538;
pub const WOW64_CONTEXT_SEGMENTS: u32 = 65540;
pub const WOW64_CONTEXT_SERVICE_ACTIVE: u32 = 268435456;
pub const WOW64_CONTEXT_XSTATE: u32 = 65600;
pub const WOW64_CONTEXT_i386: u32 = 65536;
pub const WOW64_CONTEXT_i486: u32 = 65536;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WOW64_DESCRIPTOR_TABLE_ENTRY {
    pub Selector: u32,
    pub Descriptor: WOW64_LDT_ENTRY,
}
impl Default for WOW64_DESCRIPTOR_TABLE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WOW64_FLOATING_SAVE_AREA {
    pub ControlWord: u32,
    pub StatusWord: u32,
    pub TagWord: u32,
    pub ErrorOffset: u32,
    pub ErrorSelector: u32,
    pub DataOffset: u32,
    pub DataSelector: u32,
    pub RegisterArea: [u8; 80],
    pub Cr0NpxState: u32,
}
impl Default for WOW64_FLOATING_SAVE_AREA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WOW64_LDT_ENTRY {
    pub LimitLow: u16,
    pub BaseLow: u16,
    pub HighWord: WOW64_LDT_ENTRY_0,
}
impl Default for WOW64_LDT_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WOW64_LDT_ENTRY_0 {
    pub Bytes: WOW64_LDT_ENTRY_0_0,
    pub Bits: WOW64_LDT_ENTRY_0_1,
}
impl Default for WOW64_LDT_ENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WOW64_LDT_ENTRY_0_0 {
    pub BaseMid: u8,
    pub Flags1: u8,
    pub Flags2: u8,
    pub BaseHi: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WOW64_LDT_ENTRY_0_1 {
    pub _bitfield: u32,
}
pub const WOW64_MAXIMUM_SUPPORTED_EXTENSION: u32 = 512;
pub const WOW64_SIZE_OF_80387_REGISTERS: u32 = 80;
pub const WRITE_DAC: u32 = 262144;
pub const WRITE_NV_MEMORY_FLAG_FLUSH: u32 = 1;
pub const WRITE_NV_MEMORY_FLAG_NON_TEMPORAL: u32 = 2;
pub const WRITE_NV_MEMORY_FLAG_NO_DRAIN: u32 = 256;
pub const WRITE_NV_MEMORY_FLAG_PERSIST: u32 = 3;
pub const WRITE_OWNER: u32 = 524288;
pub const WRITE_RESTRICTED: u32 = 8;
pub const WRITE_WATCH_FLAG_RESET: u32 = 1;
pub const WT_EXECUTEDEFAULT: u32 = 0;
pub const WT_EXECUTEDELETEWAIT: u32 = 8;
pub const WT_EXECUTEINIOTHREAD: u32 = 1;
pub const WT_EXECUTEINLONGTHREAD: u32 = 16;
pub const WT_EXECUTEINPERSISTENTIOTHREAD: u32 = 64;
pub const WT_EXECUTEINPERSISTENTTHREAD: u32 = 128;
pub const WT_EXECUTEINTIMERTHREAD: u32 = 32;
pub const WT_EXECUTEINUITHREAD: u32 = 2;
pub const WT_EXECUTEINWAITTHREAD: u32 = 4;
pub const WT_EXECUTELONGFUNCTION: u32 = 16;
pub const WT_EXECUTEONLYONCE: u32 = 8;
pub const WT_TRANSFER_IMPERSONATION: u32 = 256;
pub const WakeTimerList: POWER_INFORMATION_LEVEL = 50;
pub const Win32ServiceOwnProcess: SERVICE_NODE_TYPE = 16;
pub const Win32ServiceShareProcess: SERVICE_NODE_TYPE = 32;
pub const WinAccountAdministratorSid: WELL_KNOWN_SID_TYPE = 38;
pub const WinAccountCertAdminsSid: WELL_KNOWN_SID_TYPE = 46;
pub const WinAccountCloneableControllersSid: WELL_KNOWN_SID_TYPE = 100;
pub const WinAccountComputersSid: WELL_KNOWN_SID_TYPE = 44;
pub const WinAccountControllersSid: WELL_KNOWN_SID_TYPE = 45;
pub const WinAccountDefaultSystemManagedSid: WELL_KNOWN_SID_TYPE = 110;
pub const WinAccountDomainAdminsSid: WELL_KNOWN_SID_TYPE = 41;
pub const WinAccountDomainGuestsSid: WELL_KNOWN_SID_TYPE = 43;
pub const WinAccountDomainUsersSid: WELL_KNOWN_SID_TYPE = 42;
pub const WinAccountEnterpriseAdminsSid: WELL_KNOWN_SID_TYPE = 48;
pub const WinAccountEnterpriseKeyAdminsSid: WELL_KNOWN_SID_TYPE = 114;
pub const WinAccountGuestSid: WELL_KNOWN_SID_TYPE = 39;
pub const WinAccountKeyAdminsSid: WELL_KNOWN_SID_TYPE = 113;
pub const WinAccountKrbtgtSid: WELL_KNOWN_SID_TYPE = 40;
pub const WinAccountPolicyAdminsSid: WELL_KNOWN_SID_TYPE = 49;
pub const WinAccountProtectedUsersSid: WELL_KNOWN_SID_TYPE = 107;
pub const WinAccountRasAndIasServersSid: WELL_KNOWN_SID_TYPE = 50;
pub const WinAccountReadonlyControllersSid: WELL_KNOWN_SID_TYPE = 75;
pub const WinAccountSchemaAdminsSid: WELL_KNOWN_SID_TYPE = 47;
pub const WinAnonymousSid: WELL_KNOWN_SID_TYPE = 13;
pub const WinApplicationPackageAuthoritySid: WELL_KNOWN_SID_TYPE = 83;
pub const WinAuthenticatedUserSid: WELL_KNOWN_SID_TYPE = 17;
pub const WinAuthenticationAuthorityAssertedSid: WELL_KNOWN_SID_TYPE = 103;
pub const WinAuthenticationFreshKeyAuthSid: WELL_KNOWN_SID_TYPE = 118;
pub const WinAuthenticationKeyPropertyAttestationSid: WELL_KNOWN_SID_TYPE = 117;
pub const WinAuthenticationKeyPropertyMFASid: WELL_KNOWN_SID_TYPE = 116;
pub const WinAuthenticationKeyTrustSid: WELL_KNOWN_SID_TYPE = 115;
pub const WinAuthenticationServiceAssertedSid: WELL_KNOWN_SID_TYPE = 104;
pub const WinBatchSid: WELL_KNOWN_SID_TYPE = 10;
pub const WinBuiltinAccessControlAssistanceOperatorsSid: WELL_KNOWN_SID_TYPE = 101;
pub const WinBuiltinAccountOperatorsSid: WELL_KNOWN_SID_TYPE = 30;
pub const WinBuiltinAdministratorsSid: WELL_KNOWN_SID_TYPE = 26;
pub const WinBuiltinAnyPackageSid: WELL_KNOWN_SID_TYPE = 84;
pub const WinBuiltinAuthorizationAccessSid: WELL_KNOWN_SID_TYPE = 59;
pub const WinBuiltinBackupOperatorsSid: WELL_KNOWN_SID_TYPE = 33;
pub const WinBuiltinCUAUsersSid: WELL_KNOWN_SID_TYPE = 122;
pub const WinBuiltinCertSvcDComAccessGroup: WELL_KNOWN_SID_TYPE = 78;
pub const WinBuiltinCryptoOperatorsSid: WELL_KNOWN_SID_TYPE = 64;
pub const WinBuiltinDCOMUsersSid: WELL_KNOWN_SID_TYPE = 61;
pub const WinBuiltinDefaultSystemManagedGroupSid: WELL_KNOWN_SID_TYPE = 111;
pub const WinBuiltinDeviceOwnersSid: WELL_KNOWN_SID_TYPE = 119;
pub const WinBuiltinDomainSid: WELL_KNOWN_SID_TYPE = 25;
pub const WinBuiltinEventLogReadersGroup: WELL_KNOWN_SID_TYPE = 76;
pub const WinBuiltinGuestsSid: WELL_KNOWN_SID_TYPE = 28;
pub const WinBuiltinHyperVAdminsSid: WELL_KNOWN_SID_TYPE = 99;
pub const WinBuiltinIUsersSid: WELL_KNOWN_SID_TYPE = 62;
pub const WinBuiltinIncomingForestTrustBuildersSid: WELL_KNOWN_SID_TYPE = 56;
pub const WinBuiltinNetworkConfigurationOperatorsSid: WELL_KNOWN_SID_TYPE = 37;
pub const WinBuiltinOpenSSHUsersSid: WELL_KNOWN_SID_TYPE = 121;
pub const WinBuiltinPerfLoggingUsersSid: WELL_KNOWN_SID_TYPE = 58;
pub const WinBuiltinPerfMonitoringUsersSid: WELL_KNOWN_SID_TYPE = 57;
pub const WinBuiltinPowerUsersSid: WELL_KNOWN_SID_TYPE = 29;
pub const WinBuiltinPreWindows2000CompatibleAccessSid: WELL_KNOWN_SID_TYPE = 35;
pub const WinBuiltinPrintOperatorsSid: WELL_KNOWN_SID_TYPE = 32;
pub const WinBuiltinRDSEndpointServersSid: WELL_KNOWN_SID_TYPE = 96;
pub const WinBuiltinRDSManagementServersSid: WELL_KNOWN_SID_TYPE = 97;
pub const WinBuiltinRDSRemoteAccessServersSid: WELL_KNOWN_SID_TYPE = 95;
pub const WinBuiltinRemoteDesktopUsersSid: WELL_KNOWN_SID_TYPE = 36;
pub const WinBuiltinRemoteManagementUsersSid: WELL_KNOWN_SID_TYPE = 102;
pub const WinBuiltinReplicatorSid: WELL_KNOWN_SID_TYPE = 34;
pub const WinBuiltinStorageReplicaAdminsSid: WELL_KNOWN_SID_TYPE = 112;
pub const WinBuiltinSystemOperatorsSid: WELL_KNOWN_SID_TYPE = 31;
pub const WinBuiltinTerminalServerLicenseServersSid: WELL_KNOWN_SID_TYPE = 60;
pub const WinBuiltinUserModeHardwareOperatorsSid: WELL_KNOWN_SID_TYPE = 120;
pub const WinBuiltinUsersSid: WELL_KNOWN_SID_TYPE = 27;
pub const WinCacheablePrincipalsGroupSid: WELL_KNOWN_SID_TYPE = 72;
pub const WinCapabilityAppointmentsSid: WELL_KNOWN_SID_TYPE = 108;
pub const WinCapabilityContactsSid: WELL_KNOWN_SID_TYPE = 109;
pub const WinCapabilityDocumentsLibrarySid: WELL_KNOWN_SID_TYPE = 91;
pub const WinCapabilityEnterpriseAuthenticationSid: WELL_KNOWN_SID_TYPE = 93;
pub const WinCapabilityInternetClientServerSid: WELL_KNOWN_SID_TYPE = 86;
pub const WinCapabilityInternetClientSid: WELL_KNOWN_SID_TYPE = 85;
pub const WinCapabilityMusicLibrarySid: WELL_KNOWN_SID_TYPE = 90;
pub const WinCapabilityPicturesLibrarySid: WELL_KNOWN_SID_TYPE = 88;
pub const WinCapabilityPrivateNetworkClientServerSid: WELL_KNOWN_SID_TYPE = 87;
pub const WinCapabilityRemovableStorageSid: WELL_KNOWN_SID_TYPE = 94;
pub const WinCapabilitySharedUserCertificatesSid: WELL_KNOWN_SID_TYPE = 92;
pub const WinCapabilityVideosLibrarySid: WELL_KNOWN_SID_TYPE = 89;
pub const WinConsoleLogonSid: WELL_KNOWN_SID_TYPE = 81;
pub const WinCreatorGroupServerSid: WELL_KNOWN_SID_TYPE = 6;
pub const WinCreatorGroupSid: WELL_KNOWN_SID_TYPE = 4;
pub const WinCreatorOwnerRightsSid: WELL_KNOWN_SID_TYPE = 71;
pub const WinCreatorOwnerServerSid: WELL_KNOWN_SID_TYPE = 5;
pub const WinCreatorOwnerSid: WELL_KNOWN_SID_TYPE = 3;
pub const WinDialupSid: WELL_KNOWN_SID_TYPE = 8;
pub const WinDigestAuthenticationSid: WELL_KNOWN_SID_TYPE = 52;
pub const WinEnterpriseControllersSid: WELL_KNOWN_SID_TYPE = 15;
pub const WinEnterpriseReadonlyControllersSid: WELL_KNOWN_SID_TYPE = 74;
pub const WinHighLabelSid: WELL_KNOWN_SID_TYPE = 68;
pub const WinIUserSid: WELL_KNOWN_SID_TYPE = 63;
pub const WinInteractiveSid: WELL_KNOWN_SID_TYPE = 11;
pub const WinLocalAccountAndAdministratorSid: WELL_KNOWN_SID_TYPE = 106;
pub const WinLocalAccountSid: WELL_KNOWN_SID_TYPE = 105;
pub const WinLocalLogonSid: WELL_KNOWN_SID_TYPE = 80;
pub const WinLocalServiceSid: WELL_KNOWN_SID_TYPE = 23;
pub const WinLocalSid: WELL_KNOWN_SID_TYPE = 2;
pub const WinLocalSystemSid: WELL_KNOWN_SID_TYPE = 22;
pub const WinLogonIdsSid: WELL_KNOWN_SID_TYPE = 21;
pub const WinLowLabelSid: WELL_KNOWN_SID_TYPE = 66;
pub const WinMediumLabelSid: WELL_KNOWN_SID_TYPE = 67;
pub const WinMediumPlusLabelSid: WELL_KNOWN_SID_TYPE = 79;
pub const WinNTLMAuthenticationSid: WELL_KNOWN_SID_TYPE = 51;
pub const WinNetworkServiceSid: WELL_KNOWN_SID_TYPE = 24;
pub const WinNetworkSid: WELL_KNOWN_SID_TYPE = 9;
pub const WinNewEnterpriseReadonlyControllersSid: WELL_KNOWN_SID_TYPE = 77;
pub const WinNonCacheablePrincipalsGroupSid: WELL_KNOWN_SID_TYPE = 73;
pub const WinNtAuthoritySid: WELL_KNOWN_SID_TYPE = 7;
pub const WinNullSid: WELL_KNOWN_SID_TYPE = 0;
pub const WinOtherOrganizationSid: WELL_KNOWN_SID_TYPE = 55;
pub const WinProxySid: WELL_KNOWN_SID_TYPE = 14;
pub const WinRemoteLogonIdSid: WELL_KNOWN_SID_TYPE = 20;
pub const WinRestrictedCodeSid: WELL_KNOWN_SID_TYPE = 18;
pub const WinSChannelAuthenticationSid: WELL_KNOWN_SID_TYPE = 53;
pub const WinSelfSid: WELL_KNOWN_SID_TYPE = 16;
pub const WinServiceSid: WELL_KNOWN_SID_TYPE = 12;
pub const WinSystemLabelSid: WELL_KNOWN_SID_TYPE = 69;
pub const WinTerminalServerSid: WELL_KNOWN_SID_TYPE = 19;
pub const WinThisOrganizationCertificateSid: WELL_KNOWN_SID_TYPE = 82;
pub const WinThisOrganizationSid: WELL_KNOWN_SID_TYPE = 54;
pub const WinUntrustedLabelSid: WELL_KNOWN_SID_TYPE = 65;
pub const WinUserModeDriversSid: WELL_KNOWN_SID_TYPE = 98;
pub const WinWorldSid: WELL_KNOWN_SID_TYPE = 1;
pub const WinWriteRestrictedCodeSid: WELL_KNOWN_SID_TYPE = 70;
pub const X3_BTYPE_QP_INST_VAL_POS_X: u32 = 0;
pub const X3_BTYPE_QP_INST_WORD_POS_X: u32 = 23;
pub const X3_BTYPE_QP_INST_WORD_X: u32 = 2;
pub const X3_BTYPE_QP_SIZE_X: u32 = 9;
pub const X3_D_WH_INST_WORD_POS_X: u32 = 24;
pub const X3_D_WH_INST_WORD_X: u32 = 3;
pub const X3_D_WH_SIGN_VAL_POS_X: u32 = 0;
pub const X3_D_WH_SIZE_X: u32 = 3;
pub const X3_EMPTY_INST_VAL_POS_X: u32 = 0;
pub const X3_EMPTY_INST_WORD_POS_X: u32 = 14;
pub const X3_EMPTY_INST_WORD_X: u32 = 1;
pub const X3_EMPTY_SIZE_X: u32 = 2;
pub const X3_IMM20_INST_WORD_POS_X: u32 = 4;
pub const X3_IMM20_INST_WORD_X: u32 = 3;
pub const X3_IMM20_SIGN_VAL_POS_X: u32 = 0;
pub const X3_IMM20_SIZE_X: u32 = 20;
pub const X3_IMM39_1_INST_WORD_POS_X: u32 = 0;
pub const X3_IMM39_1_INST_WORD_X: u32 = 2;
pub const X3_IMM39_1_SIGN_VAL_POS_X: u32 = 36;
pub const X3_IMM39_1_SIZE_X: u32 = 23;
pub const X3_IMM39_2_INST_WORD_POS_X: u32 = 16;
pub const X3_IMM39_2_INST_WORD_X: u32 = 1;
pub const X3_IMM39_2_SIGN_VAL_POS_X: u32 = 20;
pub const X3_IMM39_2_SIZE_X: u32 = 16;
pub const X3_I_INST_WORD_POS_X: u32 = 27;
pub const X3_I_INST_WORD_X: u32 = 3;
pub const X3_I_SIGN_VAL_POS_X: u32 = 59;
pub const X3_I_SIZE_X: u32 = 1;
pub const X3_OPCODE_INST_WORD_POS_X: u32 = 28;
pub const X3_OPCODE_INST_WORD_X: u32 = 3;
pub const X3_OPCODE_SIGN_VAL_POS_X: u32 = 0;
pub const X3_OPCODE_SIZE_X: u32 = 4;
pub const X3_P_INST_WORD_POS_X: u32 = 0;
pub const X3_P_INST_WORD_X: u32 = 3;
pub const X3_P_SIGN_VAL_POS_X: u32 = 0;
pub const X3_P_SIZE_X: u32 = 4;
pub const X3_TMPLT_INST_WORD_POS_X: u32 = 0;
pub const X3_TMPLT_INST_WORD_X: u32 = 0;
pub const X3_TMPLT_SIGN_VAL_POS_X: u32 = 0;
pub const X3_TMPLT_SIZE_X: u32 = 4;
pub const X86_CACHE_ALIGNMENT_SIZE: u32 = 64;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub type XMM_SAVE_AREA32 = XSAVE_FORMAT;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct XSAVE_AREA {
    pub LegacyState: XSAVE_FORMAT,
    pub Header: XSAVE_AREA_HEADER,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XSAVE_AREA_HEADER {
    pub Mask: u64,
    pub CompactionMask: u64,
    pub Reserved2: [u64; 6],
}
impl Default for XSAVE_AREA_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XSAVE_ARM64_SME_TPIDR2_HEADER {
    pub Reserved: [u32; 8],
}
impl Default for XSAVE_ARM64_SME_TPIDR2_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct XSAVE_ARM64_SME_ZA_HEADER {
    pub VectorLength: u32,
    pub Anonymous: XSAVE_ARM64_SME_ZA_HEADER_0,
    pub Reserved: [u32; 6],
}
impl Default for XSAVE_ARM64_SME_ZA_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union XSAVE_ARM64_SME_ZA_HEADER_0 {
    pub Flags: u32,
    pub Anonymous: XSAVE_ARM64_SME_ZA_HEADER_0_0,
}
impl Default for XSAVE_ARM64_SME_ZA_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct XSAVE_ARM64_SME_ZA_HEADER_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XSAVE_ARM64_SME_ZT_HEADER {
    pub RegisterCount: u32,
    pub Reserved: [u32; 7],
}
impl Default for XSAVE_ARM64_SME_ZT_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct XSAVE_ARM64_SVE_HEADER {
    pub VectorLength: u32,
    pub VectorRegisterOffset: u32,
    pub PredicateRegisterOffset: u32,
    pub Anonymous: XSAVE_ARM64_SVE_HEADER_0,
    pub Reserved: [u32; 4],
}
impl Default for XSAVE_ARM64_SVE_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union XSAVE_ARM64_SVE_HEADER_0 {
    pub Flags: u32,
    pub Anonymous: XSAVE_ARM64_SVE_HEADER_0_0,
}
impl Default for XSAVE_ARM64_SVE_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct XSAVE_ARM64_SVE_HEADER_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct XSAVE_CET_U_FORMAT {
    pub Ia32CetUMsr: u64,
    pub Ia32Pl3SspMsr: u64,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XSAVE_FORMAT {
    pub ControlWord: u16,
    pub StatusWord: u16,
    pub TagWord: u8,
    pub Reserved1: u8,
    pub ErrorOpcode: u16,
    pub ErrorOffset: u32,
    pub ErrorSelector: u16,
    pub Reserved2: u16,
    pub DataOffset: u32,
    pub DataSelector: u16,
    pub Reserved3: u16,
    pub MxCsr: u32,
    pub MxCsr_Mask: u32,
    pub FloatRegisters: [M128A; 8],
    pub XmmRegisters: [M128A; 8],
    pub Reserved4: [u8; 224],
}
#[cfg(target_arch = "x86")]
impl Default for XSAVE_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XSAVE_FORMAT {
    pub ControlWord: u16,
    pub StatusWord: u16,
    pub TagWord: u8,
    pub Reserved1: u8,
    pub ErrorOpcode: u16,
    pub ErrorOffset: u32,
    pub ErrorSelector: u16,
    pub Reserved2: u16,
    pub DataOffset: u32,
    pub DataSelector: u16,
    pub Reserved3: u16,
    pub MxCsr: u32,
    pub MxCsr_Mask: u32,
    pub FloatRegisters: [M128A; 8],
    pub XmmRegisters: [M128A; 16],
    pub Reserved4: [u8; 96],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for XSAVE_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const XSTATE_ALIGN_BIT: u32 = 1;
pub const XSTATE_ALIGN_MASK: u32 = 2;
pub const XSTATE_AMX_TILE_CONFIG: u32 = 17;
pub const XSTATE_AMX_TILE_DATA: u32 = 18;
pub const XSTATE_ARM64_SME_TPIDR2: u32 = 4;
pub const XSTATE_ARM64_SME_ZA: u32 = 3;
pub const XSTATE_ARM64_SME_ZT: u32 = 5;
pub const XSTATE_ARM64_SVE: u32 = 2;
pub const XSTATE_AVX: u32 = 2;
pub const XSTATE_AVX512_KMASK: u32 = 5;
pub const XSTATE_AVX512_ZMM: u32 = 7;
pub const XSTATE_AVX512_ZMM_H: u32 = 6;
pub const XSTATE_CET_S: u32 = 12;
pub const XSTATE_CET_U: u32 = 11;
pub const XSTATE_COMPACTION_ENABLE: u32 = 63;
pub const XSTATE_COMPACTION_ENABLE_MASK: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct XSTATE_CONFIGURATION {
    pub EnabledFeatures: u64,
    pub EnabledVolatileFeatures: u64,
    pub Size: u32,
    pub Anonymous: XSTATE_CONFIGURATION_0,
    pub Features: [XSTATE_FEATURE; 64],
    pub EnabledSupervisorFeatures: u64,
    pub AlignedFeatures: u64,
    pub AllFeatureSize: u32,
    pub AllFeatures: [u32; 64],
    pub EnabledUserVisibleSupervisorFeatures: u64,
    pub ExtendedFeatureDisableFeatures: u64,
    pub AllNonLargeFeatureSize: u32,
    pub Anonymous2: XSTATE_CONFIGURATION_1,
    pub Spare: u64,
}
impl Default for XSTATE_CONFIGURATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union XSTATE_CONFIGURATION_0 {
    pub ControlFlags: u32,
    pub Anonymous: XSTATE_CONFIGURATION_0_0,
}
impl Default for XSTATE_CONFIGURATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct XSTATE_CONFIGURATION_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union XSTATE_CONFIGURATION_1 {
    pub Anonymous: XSTATE_CONFIGURATION_1_0,
    pub Anonymous2: XSTATE_CONFIGURATION_1_1,
}
impl Default for XSTATE_CONFIGURATION_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XSTATE_CONFIGURATION_1_0 {
    pub Amd64Spare1: [u32; 3],
}
impl Default for XSTATE_CONFIGURATION_1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct XSTATE_CONFIGURATION_1_1 {
    pub MaxSveVectorLength: u16,
    pub MaxSmeVectorLength: u16,
    pub SmeZTRegisterCount: u16,
    pub Anonymous: XSTATE_CONFIGURATION_1_1_0,
    pub SupportedSmeVectorLengths: u8,
    pub Arm64Spare: [u8; 3],
}
impl Default for XSTATE_CONFIGURATION_1_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union XSTATE_CONFIGURATION_1_1_0 {
    pub Arm64Flags: u16,
    pub Anonymous: XSTATE_CONFIGURATION_1_1_0_0,
}
impl Default for XSTATE_CONFIGURATION_1_1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct XSTATE_CONFIGURATION_1_1_0_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XSTATE_CONTEXT {
    pub Mask: u64,
    pub Length: u32,
    pub Flags: u8,
    pub Reserved0: [u8; 3],
    pub Area: PXSAVE_AREA,
    pub Reserved2: u32,
    pub Buffer: *mut core::ffi::c_void,
    pub Reserved3: u32,
}
#[cfg(target_arch = "x86")]
impl Default for XSTATE_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XSTATE_CONTEXT {
    pub Mask: u64,
    pub Length: u32,
    pub Flags: u8,
    pub Reserved0: [u8; 3],
    pub Area: PXSAVE_AREA,
    pub Buffer: *mut core::ffi::c_void,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for XSTATE_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const XSTATE_CONTEXT_FLAG_LOOKASIDE: u32 = 1;
pub const XSTATE_CONTROLFLAG_VALID_MASK: u32 = 7;
pub const XSTATE_CONTROLFLAG_XFD_MASK: u32 = 4;
pub const XSTATE_CONTROLFLAG_XSAVEC_MASK: u32 = 2;
pub const XSTATE_CONTROLFLAG_XSAVEOPT_MASK: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct XSTATE_FEATURE {
    pub Offset: u32,
    pub Size: u32,
}
pub const XSTATE_FIRST_NON_LEGACY_FEATURE: u32 = 2;
pub const XSTATE_GSSE: u32 = 2;
pub const XSTATE_IPT: u32 = 8;
pub const XSTATE_LEGACY_FLOATING_POINT: u32 = 0;
pub const XSTATE_LEGACY_SSE: u32 = 1;
pub const XSTATE_LWP: u32 = 62;
#[cfg(target_arch = "x86")]
pub const XSTATE_MASK_ALLOWED: u32 = 511;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const XSTATE_MASK_ALLOWED: u32 = 396799;
#[cfg(target_arch = "aarch64")]
pub const XSTATE_MASK_ALLOWED: u32 = 60;
pub const XSTATE_MASK_AMD64_LEGACY: u32 = 3;
pub const XSTATE_MASK_AMX_TILE_CONFIG: u32 = 131072;
pub const XSTATE_MASK_AMX_TILE_DATA: u32 = 262144;
pub const XSTATE_MASK_ARM64_SME_TPIDR2: u32 = 16;
pub const XSTATE_MASK_ARM64_SME_ZA: u32 = 8;
pub const XSTATE_MASK_ARM64_SME_ZT: u32 = 32;
pub const XSTATE_MASK_ARM64_SVE: u32 = 4;
pub const XSTATE_MASK_AVX: u32 = 4;
pub const XSTATE_MASK_AVX512: u32 = 224;
pub const XSTATE_MASK_CET_S: u32 = 4096;
pub const XSTATE_MASK_CET_U: u32 = 2048;
pub const XSTATE_MASK_GSSE: u32 = 4;
pub const XSTATE_MASK_IPT: u32 = 256;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const XSTATE_MASK_LARGE_FEATURES: u32 = 262144;
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
pub const XSTATE_MASK_LARGE_FEATURES: u32 = 0;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
pub const XSTATE_MASK_LEGACY: u32 = 3;
#[cfg(target_arch = "aarch64")]
pub const XSTATE_MASK_LEGACY: u32 = 0;
pub const XSTATE_MASK_LEGACY_FLOATING_POINT: u32 = 1;
pub const XSTATE_MASK_LEGACY_SSE: u32 = 2;
pub const XSTATE_MASK_LWP: u32 = 0;
pub const XSTATE_MASK_MPX: u32 = 24;
pub const XSTATE_MASK_PASID: u32 = 1024;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
pub const XSTATE_MASK_PERSISTENT: u32 = 16;
#[cfg(target_arch = "aarch64")]
pub const XSTATE_MASK_PERSISTENT: u32 = 0;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
pub const XSTATE_MASK_USER_VISIBLE_SUPERVISOR: u32 = 2048;
#[cfg(target_arch = "aarch64")]
pub const XSTATE_MASK_USER_VISIBLE_SUPERVISOR: u32 = 0;
pub const XSTATE_MPX_BNDCSR: u32 = 4;
pub const XSTATE_MPX_BNDREGS: u32 = 3;
pub const XSTATE_PASID: u32 = 10;
pub const XSTATE_XFD_BIT: u32 = 2;
pub const XSTATE_XFD_MASK: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _ACTIVATION_CONTEXT(pub u8);
#[repr(C, align(1))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _ENUM_FLAG_INTEGER_FOR_SIZE(pub u8);
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub type _IMAGE_RUNTIME_FUNCTION_ENTRY = RUNTIME_FUNCTION;
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
#[derive(Clone, Copy)]
pub struct _IMAGE_RUNTIME_FUNCTION_ENTRY {
    pub BeginAddress: u32,
    pub EndAddress: u32,
    pub Anonymous: _IMAGE_RUNTIME_FUNCTION_ENTRY_0,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
impl Default for _IMAGE_RUNTIME_FUNCTION_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
#[derive(Clone, Copy)]
pub union _IMAGE_RUNTIME_FUNCTION_ENTRY_0 {
    pub UnwindInfoAddress: u32,
    pub UnwindData: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
impl Default for _IMAGE_RUNTIME_FUNCTION_ENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct _PIMAGE_RUNTIME_FUNCTION_ENTRY(pub *mut RUNTIME_FUNCTION);
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl _PIMAGE_RUNTIME_FUNCTION_ENTRY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for _PIMAGE_RUNTIME_FUNCTION_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct _PIMAGE_RUNTIME_FUNCTION_ENTRY(pub *mut _IMAGE_RUNTIME_FUNCTION_ENTRY);
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
impl _PIMAGE_RUNTIME_FUNCTION_ENTRY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
impl Default for _PIMAGE_RUNTIME_FUNCTION_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _TEB(pub u8);
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _UNWIND_HISTORY_TABLE(pub u8);
pub type __C_ASSERT__ = [i8; 1];
