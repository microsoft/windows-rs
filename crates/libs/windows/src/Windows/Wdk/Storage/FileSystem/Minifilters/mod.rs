#[inline]
pub unsafe fn FltAcknowledgeEcp(filter: PFLT_FILTER, ecpcontext: *const core::ffi::c_void) {
    windows_core::link!("fltmgr.sys" "system" fn FltAcknowledgeEcp(filter : PFLT_FILTER, ecpcontext : *const core::ffi::c_void));
    unsafe { FltAcknowledgeEcp(filter, ecpcontext) }
}
#[inline]
pub unsafe fn FltAcquirePushLockExclusive() -> usize {
    windows_core::link!("fltmgr.sys" "system" fn FltAcquirePushLockExclusive(pushlock : *mut usize));
    unsafe {
        let mut result__ = core::mem::zeroed();
        FltAcquirePushLockExclusive(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn FltAcquirePushLockExclusiveEx(pushlock: *mut usize, flags: u32) {
    windows_core::link!("fltmgr.sys" "system" fn FltAcquirePushLockExclusiveEx(pushlock : *mut usize, flags : u32));
    unsafe { FltAcquirePushLockExclusiveEx(pushlock as _, flags) }
}
#[inline]
pub unsafe fn FltAcquirePushLockShared() -> usize {
    windows_core::link!("fltmgr.sys" "system" fn FltAcquirePushLockShared(pushlock : *mut usize));
    unsafe {
        let mut result__ = core::mem::zeroed();
        FltAcquirePushLockShared(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn FltAcquirePushLockSharedEx(pushlock: *mut usize, flags: u32) {
    windows_core::link!("fltmgr.sys" "system" fn FltAcquirePushLockSharedEx(pushlock : *mut usize, flags : u32));
    unsafe { FltAcquirePushLockSharedEx(pushlock as _, flags) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn FltAcquireResourceExclusive(resource: *mut super::super::super::Foundation::ERESOURCE) {
    windows_core::link!("fltmgr.sys" "system" fn FltAcquireResourceExclusive(resource : *mut super::super::super::Foundation:: ERESOURCE));
    unsafe { FltAcquireResourceExclusive(resource as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn FltAcquireResourceShared(resource: *mut super::super::super::Foundation::ERESOURCE) {
    windows_core::link!("fltmgr.sys" "system" fn FltAcquireResourceShared(resource : *mut super::super::super::Foundation:: ERESOURCE));
    unsafe { FltAcquireResourceShared(resource as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltAddOpenReparseEntry(filter: PFLT_FILTER, data: *mut FLT_CALLBACK_DATA, openreparseentry: *mut super::OPEN_REPARSE_LIST_ENTRY) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltAddOpenReparseEntry(filter : PFLT_FILTER, data : *mut FLT_CALLBACK_DATA, openreparseentry : *mut super:: OPEN_REPARSE_LIST_ENTRY) -> windows_core:: NTSTATUS);
    unsafe { FltAddOpenReparseEntry(filter, data as _, openreparseentry as _) }
}
#[inline]
pub unsafe fn FltAdjustDeviceStackSizeForIoRedirection(sourceinstance: PFLT_INSTANCE, targetinstance: PFLT_INSTANCE, sourcedevicestacksizemodified: *mut bool) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltAdjustDeviceStackSizeForIoRedirection(sourceinstance : PFLT_INSTANCE, targetinstance : PFLT_INSTANCE, sourcedevicestacksizemodified : *mut bool) -> windows_core:: NTSTATUS);
    unsafe { FltAdjustDeviceStackSizeForIoRedirection(sourceinstance, targetinstance, sourcedevicestacksizemodified as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltAllocateCallbackData(instance: PFLT_INSTANCE, fileobject: *mut super::super::super::Foundation::FILE_OBJECT, retnewcallbackdata: *mut *mut FLT_CALLBACK_DATA) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltAllocateCallbackData(instance : PFLT_INSTANCE, fileobject : *mut super::super::super::Foundation:: FILE_OBJECT, retnewcallbackdata : *mut *mut FLT_CALLBACK_DATA) -> windows_core:: NTSTATUS);
    unsafe { FltAllocateCallbackData(instance, fileobject as _, retnewcallbackdata as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltAllocateCallbackDataEx(instance: PFLT_INSTANCE, fileobject: *const super::super::super::Foundation::FILE_OBJECT, flags: u32, retnewcallbackdata: *mut *mut FLT_CALLBACK_DATA) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltAllocateCallbackDataEx(instance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, flags : u32, retnewcallbackdata : *mut *mut FLT_CALLBACK_DATA) -> windows_core:: NTSTATUS);
    unsafe { FltAllocateCallbackDataEx(instance, fileobject, flags, retnewcallbackdata as _) }
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FltAllocateContext(filter: PFLT_FILTER, contexttype: u16, contextsize: usize, pooltype: super::super::super::Foundation::POOL_TYPE, returnedcontext: *mut PFLT_CONTEXT) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltAllocateContext(filter : PFLT_FILTER, contexttype : u16, contextsize : usize, pooltype : super::super::super::Foundation:: POOL_TYPE, returnedcontext : *mut PFLT_CONTEXT) -> windows_core:: NTSTATUS);
    unsafe { FltAllocateContext(filter, contexttype, contextsize, pooltype, returnedcontext as _) }
}
#[inline]
pub unsafe fn FltAllocateDeferredIoWorkItem() -> PFLT_DEFERRED_IO_WORKITEM {
    windows_core::link!("fltmgr.sys" "system" fn FltAllocateDeferredIoWorkItem() -> PFLT_DEFERRED_IO_WORKITEM);
    unsafe { FltAllocateDeferredIoWorkItem() }
}
#[inline]
pub unsafe fn FltAllocateExtraCreateParameter(filter: PFLT_FILTER, ecptype: *mut windows_core::GUID, sizeofcontext: u32, flags: u32, cleanupcallback: super::PFSRTL_EXTRA_CREATE_PARAMETER_CLEANUP_CALLBACK, pooltag: u32, ecpcontext: *mut *mut core::ffi::c_void) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltAllocateExtraCreateParameter(filter : PFLT_FILTER, ecptype : *mut windows_core::GUID, sizeofcontext : u32, flags : u32, cleanupcallback : super:: PFSRTL_EXTRA_CREATE_PARAMETER_CLEANUP_CALLBACK, pooltag : u32, ecpcontext : *mut *mut core::ffi::c_void) -> windows_core:: NTSTATUS);
    unsafe { FltAllocateExtraCreateParameter(filter, ecptype as _, sizeofcontext, flags, cleanupcallback, pooltag, ecpcontext as _) }
}
#[inline]
pub unsafe fn FltAllocateExtraCreateParameterFromLookasideList(filter: PFLT_FILTER, ecptype: *mut windows_core::GUID, sizeofcontext: u32, flags: u32, cleanupcallback: super::PFSRTL_EXTRA_CREATE_PARAMETER_CLEANUP_CALLBACK, lookasidelist: *mut core::ffi::c_void, ecpcontext: *mut *mut core::ffi::c_void) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltAllocateExtraCreateParameterFromLookasideList(filter : PFLT_FILTER, ecptype : *mut windows_core::GUID, sizeofcontext : u32, flags : u32, cleanupcallback : super:: PFSRTL_EXTRA_CREATE_PARAMETER_CLEANUP_CALLBACK, lookasidelist : *mut core::ffi::c_void, ecpcontext : *mut *mut core::ffi::c_void) -> windows_core:: NTSTATUS);
    unsafe { FltAllocateExtraCreateParameterFromLookasideList(filter, ecptype as _, sizeofcontext, flags, cleanupcallback, lookasidelist as _, ecpcontext as _) }
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FltAllocateExtraCreateParameterList(filter: PFLT_FILTER, flags: u32, ecplist: *mut *mut super::super::super::Foundation::ECP_LIST) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltAllocateExtraCreateParameterList(filter : PFLT_FILTER, flags : u32, ecplist : *mut *mut super::super::super::Foundation:: ECP_LIST) -> windows_core:: NTSTATUS);
    unsafe { FltAllocateExtraCreateParameterList(filter, flags, ecplist as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltAllocateFileLock(completelockcallbackdataroutine: PFLT_COMPLETE_LOCK_CALLBACK_DATA_ROUTINE, unlockroutine: super::PUNLOCK_ROUTINE) -> *mut super::FILE_LOCK {
    windows_core::link!("fltmgr.sys" "system" fn FltAllocateFileLock(completelockcallbackdataroutine : PFLT_COMPLETE_LOCK_CALLBACK_DATA_ROUTINE, unlockroutine : super:: PUNLOCK_ROUTINE) -> *mut super:: FILE_LOCK);
    unsafe { FltAllocateFileLock(completelockcallbackdataroutine, unlockroutine) }
}
#[inline]
pub unsafe fn FltAllocateGenericWorkItem() -> PFLT_GENERIC_WORKITEM {
    windows_core::link!("fltmgr.sys" "system" fn FltAllocateGenericWorkItem() -> PFLT_GENERIC_WORKITEM);
    unsafe { FltAllocateGenericWorkItem() }
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FltAllocatePoolAlignedWithTag(instance: PFLT_INSTANCE, pooltype: super::super::super::Foundation::POOL_TYPE, numberofbytes: usize, tag: u32) -> *mut core::ffi::c_void {
    windows_core::link!("fltmgr.sys" "system" fn FltAllocatePoolAlignedWithTag(instance : PFLT_INSTANCE, pooltype : super::super::super::Foundation:: POOL_TYPE, numberofbytes : usize, tag : u32) -> *mut core::ffi::c_void);
    unsafe { FltAllocatePoolAlignedWithTag(instance, pooltype, numberofbytes, tag) }
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FltApplyPriorityInfoThread(inputpriorityinfo: *const super::IO_PRIORITY_INFO, outputpriorityinfo: *mut super::IO_PRIORITY_INFO, thread: super::super::super::Foundation::PETHREAD) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltApplyPriorityInfoThread(inputpriorityinfo : *const super:: IO_PRIORITY_INFO, outputpriorityinfo : *mut super:: IO_PRIORITY_INFO, thread : super::super::super::Foundation:: PETHREAD) -> windows_core:: NTSTATUS);
    unsafe { FltApplyPriorityInfoThread(inputpriorityinfo, outputpriorityinfo as _, thread) }
}
#[inline]
pub unsafe fn FltAttachVolume(filter: PFLT_FILTER, volume: PFLT_VOLUME, instancename: *mut super::super::super::super::Win32::Foundation::UNICODE_STRING, retinstance: *mut PFLT_INSTANCE) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltAttachVolume(filter : PFLT_FILTER, volume : PFLT_VOLUME, instancename : *mut super::super::super::super::Win32::Foundation:: UNICODE_STRING, retinstance : *mut PFLT_INSTANCE) -> windows_core:: NTSTATUS);
    unsafe { FltAttachVolume(filter, volume, instancename as _, retinstance as _) }
}
#[inline]
pub unsafe fn FltAttachVolumeAtAltitude(filter: PFLT_FILTER, volume: PFLT_VOLUME, altitude: *mut super::super::super::super::Win32::Foundation::UNICODE_STRING, instancename: *mut super::super::super::super::Win32::Foundation::UNICODE_STRING, retinstance: *mut PFLT_INSTANCE) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltAttachVolumeAtAltitude(filter : PFLT_FILTER, volume : PFLT_VOLUME, altitude : *mut super::super::super::super::Win32::Foundation:: UNICODE_STRING, instancename : *mut super::super::super::super::Win32::Foundation:: UNICODE_STRING, retinstance : *mut PFLT_INSTANCE) -> windows_core:: NTSTATUS);
    unsafe { FltAttachVolumeAtAltitude(filter, volume, altitude as _, instancename as _, retinstance as _) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FltBuildDefaultSecurityDescriptor(securitydescriptor: *mut super::super::super::super::Win32::Security::PSECURITY_DESCRIPTOR, desiredaccess: u32) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltBuildDefaultSecurityDescriptor(securitydescriptor : *mut super::super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, desiredaccess : u32) -> windows_core:: NTSTATUS);
    unsafe { FltBuildDefaultSecurityDescriptor(securitydescriptor as _, desiredaccess) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltCancelFileOpen(instance: PFLT_INSTANCE, fileobject: *const super::super::super::Foundation::FILE_OBJECT) {
    windows_core::link!("fltmgr.sys" "system" fn FltCancelFileOpen(instance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT));
    unsafe { FltCancelFileOpen(instance, fileobject) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltCancelIo(callbackdata: *mut FLT_CALLBACK_DATA) -> bool {
    windows_core::link!("fltmgr.sys" "system" fn FltCancelIo(callbackdata : *mut FLT_CALLBACK_DATA) -> bool);
    unsafe { FltCancelIo(callbackdata as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltCancellableWaitForMultipleObjects(count: u32, objectarray: *mut *mut core::ffi::c_void, waittype: super::super::super::super::Win32::System::Kernel::WAIT_TYPE, timeout: *mut i64, waitblockarray: *mut super::super::super::Foundation::KWAIT_BLOCK, callbackdata: *mut FLT_CALLBACK_DATA) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltCancellableWaitForMultipleObjects(count : u32, objectarray : *mut *mut core::ffi::c_void, waittype : super::super::super::super::Win32::System::Kernel:: WAIT_TYPE, timeout : *mut i64, waitblockarray : *mut super::super::super::Foundation:: KWAIT_BLOCK, callbackdata : *mut FLT_CALLBACK_DATA) -> windows_core:: NTSTATUS);
    unsafe { FltCancellableWaitForMultipleObjects(count, objectarray as _, waittype, timeout as _, waitblockarray as _, callbackdata as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltCancellableWaitForSingleObject(object: *mut core::ffi::c_void, timeout: *mut i64, callbackdata: *mut FLT_CALLBACK_DATA) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltCancellableWaitForSingleObject(object : *mut core::ffi::c_void, timeout : *mut i64, callbackdata : *mut FLT_CALLBACK_DATA) -> windows_core:: NTSTATUS);
    unsafe { FltCancellableWaitForSingleObject(object as _, timeout as _, callbackdata as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltCbdqDisable(cbdq: *mut FLT_CALLBACK_DATA_QUEUE) {
    windows_core::link!("fltmgr.sys" "system" fn FltCbdqDisable(cbdq : *mut FLT_CALLBACK_DATA_QUEUE));
    unsafe { FltCbdqDisable(cbdq as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltCbdqEnable(cbdq: *mut FLT_CALLBACK_DATA_QUEUE) {
    windows_core::link!("fltmgr.sys" "system" fn FltCbdqEnable(cbdq : *mut FLT_CALLBACK_DATA_QUEUE));
    unsafe { FltCbdqEnable(cbdq as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltCbdqInitialize(instance: PFLT_INSTANCE, cbdq: *mut FLT_CALLBACK_DATA_QUEUE, cbdqinsertio: PFLT_CALLBACK_DATA_QUEUE_INSERT_IO, cbdqremoveio: PFLT_CALLBACK_DATA_QUEUE_REMOVE_IO, cbdqpeeknextio: PFLT_CALLBACK_DATA_QUEUE_PEEK_NEXT_IO, cbdqacquire: PFLT_CALLBACK_DATA_QUEUE_ACQUIRE, cbdqrelease: PFLT_CALLBACK_DATA_QUEUE_RELEASE, cbdqcompletecanceledio: PFLT_CALLBACK_DATA_QUEUE_COMPLETE_CANCELED_IO) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltCbdqInitialize(instance : PFLT_INSTANCE, cbdq : *mut FLT_CALLBACK_DATA_QUEUE, cbdqinsertio : PFLT_CALLBACK_DATA_QUEUE_INSERT_IO, cbdqremoveio : PFLT_CALLBACK_DATA_QUEUE_REMOVE_IO, cbdqpeeknextio : PFLT_CALLBACK_DATA_QUEUE_PEEK_NEXT_IO, cbdqacquire : PFLT_CALLBACK_DATA_QUEUE_ACQUIRE, cbdqrelease : PFLT_CALLBACK_DATA_QUEUE_RELEASE, cbdqcompletecanceledio : PFLT_CALLBACK_DATA_QUEUE_COMPLETE_CANCELED_IO) -> windows_core:: NTSTATUS);
    unsafe { FltCbdqInitialize(instance, cbdq as _, cbdqinsertio, cbdqremoveio, cbdqpeeknextio, cbdqacquire, cbdqrelease, cbdqcompletecanceledio) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltCbdqInsertIo(cbdq: *mut FLT_CALLBACK_DATA_QUEUE, cbd: *mut FLT_CALLBACK_DATA, context: *mut super::super::super::System::SystemServices::IO_CSQ_IRP_CONTEXT, insertcontext: *mut core::ffi::c_void) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltCbdqInsertIo(cbdq : *mut FLT_CALLBACK_DATA_QUEUE, cbd : *mut FLT_CALLBACK_DATA, context : *mut super::super::super::System::SystemServices:: IO_CSQ_IRP_CONTEXT, insertcontext : *mut core::ffi::c_void) -> windows_core:: NTSTATUS);
    unsafe { FltCbdqInsertIo(cbdq as _, cbd as _, context as _, insertcontext as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltCbdqRemoveIo(cbdq: *mut FLT_CALLBACK_DATA_QUEUE, context: *mut super::super::super::System::SystemServices::IO_CSQ_IRP_CONTEXT) -> *mut FLT_CALLBACK_DATA {
    windows_core::link!("fltmgr.sys" "system" fn FltCbdqRemoveIo(cbdq : *mut FLT_CALLBACK_DATA_QUEUE, context : *mut super::super::super::System::SystemServices:: IO_CSQ_IRP_CONTEXT) -> *mut FLT_CALLBACK_DATA);
    unsafe { FltCbdqRemoveIo(cbdq as _, context as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltCbdqRemoveNextIo(cbdq: *mut FLT_CALLBACK_DATA_QUEUE, peekcontext: *mut core::ffi::c_void) -> *mut FLT_CALLBACK_DATA {
    windows_core::link!("fltmgr.sys" "system" fn FltCbdqRemoveNextIo(cbdq : *mut FLT_CALLBACK_DATA_QUEUE, peekcontext : *mut core::ffi::c_void) -> *mut FLT_CALLBACK_DATA);
    unsafe { FltCbdqRemoveNextIo(cbdq as _, peekcontext as _) }
}
#[inline]
pub unsafe fn FltCheckAndGrowNameControl(namectrl: *mut FLT_NAME_CONTROL, newsize: u16) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltCheckAndGrowNameControl(namectrl : *mut FLT_NAME_CONTROL, newsize : u16) -> windows_core:: NTSTATUS);
    unsafe { FltCheckAndGrowNameControl(namectrl as _, newsize) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltCheckLockForReadAccess(filelock: *mut super::FILE_LOCK, callbackdata: *mut FLT_CALLBACK_DATA) -> bool {
    windows_core::link!("fltmgr.sys" "system" fn FltCheckLockForReadAccess(filelock : *mut super:: FILE_LOCK, callbackdata : *mut FLT_CALLBACK_DATA) -> bool);
    unsafe { FltCheckLockForReadAccess(filelock as _, callbackdata as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltCheckLockForWriteAccess(filelock: *mut super::FILE_LOCK, callbackdata: *mut FLT_CALLBACK_DATA) -> bool {
    windows_core::link!("fltmgr.sys" "system" fn FltCheckLockForWriteAccess(filelock : *mut super:: FILE_LOCK, callbackdata : *mut FLT_CALLBACK_DATA) -> bool);
    unsafe { FltCheckLockForWriteAccess(filelock as _, callbackdata as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltCheckOplock(oplock: *mut *mut core::ffi::c_void, callbackdata: *mut FLT_CALLBACK_DATA, context: *mut core::ffi::c_void, waitcompletionroutine: PFLTOPLOCK_WAIT_COMPLETE_ROUTINE, prepostcallbackdataroutine: PFLTOPLOCK_PREPOST_CALLBACKDATA_ROUTINE) -> FLT_PREOP_CALLBACK_STATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltCheckOplock(oplock : *mut *mut core::ffi::c_void, callbackdata : *mut FLT_CALLBACK_DATA, context : *mut core::ffi::c_void, waitcompletionroutine : PFLTOPLOCK_WAIT_COMPLETE_ROUTINE, prepostcallbackdataroutine : PFLTOPLOCK_PREPOST_CALLBACKDATA_ROUTINE) -> FLT_PREOP_CALLBACK_STATUS);
    unsafe { FltCheckOplock(oplock as _, callbackdata as _, context as _, waitcompletionroutine, prepostcallbackdataroutine) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltCheckOplockEx(oplock: *const *const core::ffi::c_void, callbackdata: *const FLT_CALLBACK_DATA, flags: u32, context: *const core::ffi::c_void, waitcompletionroutine: PFLTOPLOCK_WAIT_COMPLETE_ROUTINE, prepostcallbackdataroutine: PFLTOPLOCK_PREPOST_CALLBACKDATA_ROUTINE) -> FLT_PREOP_CALLBACK_STATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltCheckOplockEx(oplock : *const *const core::ffi::c_void, callbackdata : *const FLT_CALLBACK_DATA, flags : u32, context : *const core::ffi::c_void, waitcompletionroutine : PFLTOPLOCK_WAIT_COMPLETE_ROUTINE, prepostcallbackdataroutine : PFLTOPLOCK_PREPOST_CALLBACKDATA_ROUTINE) -> FLT_PREOP_CALLBACK_STATUS);
    unsafe { FltCheckOplockEx(oplock, callbackdata, flags, context, waitcompletionroutine, prepostcallbackdataroutine) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltClearCallbackDataDirty(data: *mut FLT_CALLBACK_DATA) {
    windows_core::link!("fltmgr.sys" "system" fn FltClearCallbackDataDirty(data : *mut FLT_CALLBACK_DATA));
    unsafe { FltClearCallbackDataDirty(data as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltClearCancelCompletion(callbackdata: *const FLT_CALLBACK_DATA) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltClearCancelCompletion(callbackdata : *const FLT_CALLBACK_DATA) -> windows_core:: NTSTATUS);
    unsafe { FltClearCancelCompletion(callbackdata) }
}
#[inline]
pub unsafe fn FltClose(filehandle: super::super::super::super::Win32::Foundation::HANDLE) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltClose(filehandle : super::super::super::super::Win32::Foundation:: HANDLE) -> windows_core:: NTSTATUS);
    unsafe { FltClose(filehandle) }
}
#[inline]
pub unsafe fn FltCloseClientPort(filter: PFLT_FILTER) -> PFLT_PORT {
    windows_core::link!("fltmgr.sys" "system" fn FltCloseClientPort(filter : PFLT_FILTER, clientport : *mut PFLT_PORT));
    unsafe {
        let mut result__ = core::mem::zeroed();
        FltCloseClientPort(filter, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn FltCloseCommunicationPort(serverport: PFLT_PORT) {
    windows_core::link!("fltmgr.sys" "system" fn FltCloseCommunicationPort(serverport : PFLT_PORT));
    unsafe { FltCloseCommunicationPort(serverport) }
}
#[inline]
pub unsafe fn FltCloseSectionForDataScan(sectioncontext: PFLT_CONTEXT) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltCloseSectionForDataScan(sectioncontext : PFLT_CONTEXT) -> windows_core:: NTSTATUS);
    unsafe { FltCloseSectionForDataScan(sectioncontext) }
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FltCommitComplete(instance: PFLT_INSTANCE, transaction: *mut super::super::super::Foundation::KTRANSACTION, transactioncontext: PFLT_CONTEXT) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltCommitComplete(instance : PFLT_INSTANCE, transaction : *mut super::super::super::Foundation:: KTRANSACTION, transactioncontext : PFLT_CONTEXT) -> windows_core:: NTSTATUS);
    unsafe { FltCommitComplete(instance, transaction as _, transactioncontext) }
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FltCommitFinalizeComplete(instance: PFLT_INSTANCE, transaction: *mut super::super::super::Foundation::KTRANSACTION, transactioncontext: PFLT_CONTEXT) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltCommitFinalizeComplete(instance : PFLT_INSTANCE, transaction : *mut super::super::super::Foundation:: KTRANSACTION, transactioncontext : PFLT_CONTEXT) -> windows_core:: NTSTATUS);
    unsafe { FltCommitFinalizeComplete(instance, transaction as _, transactioncontext) }
}
#[inline]
pub unsafe fn FltCompareInstanceAltitudes(instance1: PFLT_INSTANCE, instance2: PFLT_INSTANCE) -> i32 {
    windows_core::link!("fltmgr.sys" "system" fn FltCompareInstanceAltitudes(instance1 : PFLT_INSTANCE, instance2 : PFLT_INSTANCE) -> i32);
    unsafe { FltCompareInstanceAltitudes(instance1, instance2) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltCompletePendedPostOperation(callbackdata: *mut FLT_CALLBACK_DATA) {
    windows_core::link!("fltmgr.sys" "system" fn FltCompletePendedPostOperation(callbackdata : *mut FLT_CALLBACK_DATA));
    unsafe { FltCompletePendedPostOperation(callbackdata as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltCompletePendedPreOperation(callbackdata: *mut FLT_CALLBACK_DATA, callbackstatus: FLT_PREOP_CALLBACK_STATUS, context: *mut core::ffi::c_void) {
    windows_core::link!("fltmgr.sys" "system" fn FltCompletePendedPreOperation(callbackdata : *mut FLT_CALLBACK_DATA, callbackstatus : FLT_PREOP_CALLBACK_STATUS, context : *mut core::ffi::c_void));
    unsafe { FltCompletePendedPreOperation(callbackdata as _, callbackstatus, context as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltCopyOpenReparseList(filter: PFLT_FILTER, data: *const FLT_CALLBACK_DATA, ecplist: *mut super::super::super::Foundation::ECP_LIST) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltCopyOpenReparseList(filter : PFLT_FILTER, data : *const FLT_CALLBACK_DATA, ecplist : *mut super::super::super::Foundation:: ECP_LIST) -> windows_core:: NTSTATUS);
    unsafe { FltCopyOpenReparseList(filter, data, ecplist as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn FltCreateCommunicationPort(filter: PFLT_FILTER, serverport: *mut PFLT_PORT, objectattributes: *mut super::super::super::Foundation::OBJECT_ATTRIBUTES, serverportcookie: *mut core::ffi::c_void, connectnotifycallback: PFLT_CONNECT_NOTIFY, disconnectnotifycallback: PFLT_DISCONNECT_NOTIFY, messagenotifycallback: PFLT_MESSAGE_NOTIFY, maxconnections: i32) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltCreateCommunicationPort(filter : PFLT_FILTER, serverport : *mut PFLT_PORT, objectattributes : *mut super::super::super::Foundation:: OBJECT_ATTRIBUTES, serverportcookie : *mut core::ffi::c_void, connectnotifycallback : PFLT_CONNECT_NOTIFY, disconnectnotifycallback : PFLT_DISCONNECT_NOTIFY, messagenotifycallback : PFLT_MESSAGE_NOTIFY, maxconnections : i32) -> windows_core:: NTSTATUS);
    unsafe { FltCreateCommunicationPort(filter, serverport as _, objectattributes as _, serverportcookie as _, connectnotifycallback, disconnectnotifycallback, messagenotifycallback, maxconnections) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn FltCreateFile(filter: PFLT_FILTER, instance: PFLT_INSTANCE, filehandle: *mut super::super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *mut super::super::super::Foundation::OBJECT_ATTRIBUTES, iostatusblock: *mut super::super::super::super::Win32::System::IO::IO_STATUS_BLOCK, allocationsize: *mut i64, fileattributes: u32, shareaccess: u32, createdisposition: u32, createoptions: u32, eabuffer: *mut core::ffi::c_void, ealength: u32, flags: u32) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltCreateFile(filter : PFLT_FILTER, instance : PFLT_INSTANCE, filehandle : *mut super::super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *mut super::super::super::Foundation:: OBJECT_ATTRIBUTES, iostatusblock : *mut super::super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, allocationsize : *mut i64, fileattributes : u32, shareaccess : u32, createdisposition : u32, createoptions : u32, eabuffer : *mut core::ffi::c_void, ealength : u32, flags : u32) -> windows_core:: NTSTATUS);
    unsafe { FltCreateFile(filter, instance, filehandle as _, desiredaccess, objectattributes as _, iostatusblock as _, allocationsize as _, fileattributes, shareaccess, createdisposition, createoptions, eabuffer as _, ealength, flags) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltCreateFileEx(filter: PFLT_FILTER, instance: PFLT_INSTANCE, filehandle: *mut super::super::super::super::Win32::Foundation::HANDLE, fileobject: *mut *mut super::super::super::Foundation::FILE_OBJECT, desiredaccess: u32, objectattributes: *const super::super::super::Foundation::OBJECT_ATTRIBUTES, iostatusblock: *mut super::super::super::super::Win32::System::IO::IO_STATUS_BLOCK, allocationsize: *const i64, fileattributes: u32, shareaccess: u32, createdisposition: u32, createoptions: u32, eabuffer: *const core::ffi::c_void, ealength: u32, flags: u32) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltCreateFileEx(filter : PFLT_FILTER, instance : PFLT_INSTANCE, filehandle : *mut super::super::super::super::Win32::Foundation:: HANDLE, fileobject : *mut *mut super::super::super::Foundation:: FILE_OBJECT, desiredaccess : u32, objectattributes : *const super::super::super::Foundation:: OBJECT_ATTRIBUTES, iostatusblock : *mut super::super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, allocationsize : *const i64, fileattributes : u32, shareaccess : u32, createdisposition : u32, createoptions : u32, eabuffer : *const core::ffi::c_void, ealength : u32, flags : u32) -> windows_core:: NTSTATUS);
    unsafe { FltCreateFileEx(filter, instance, filehandle as _, fileobject as _, desiredaccess, objectattributes, iostatusblock as _, allocationsize, fileattributes, shareaccess, createdisposition, createoptions, eabuffer, ealength, flags) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltCreateFileEx2(filter: PFLT_FILTER, instance: PFLT_INSTANCE, filehandle: *mut super::super::super::super::Win32::Foundation::HANDLE, fileobject: *mut *mut super::super::super::Foundation::FILE_OBJECT, desiredaccess: u32, objectattributes: *mut super::super::super::Foundation::OBJECT_ATTRIBUTES, iostatusblock: *mut super::super::super::super::Win32::System::IO::IO_STATUS_BLOCK, allocationsize: *mut i64, fileattributes: u32, shareaccess: u32, createdisposition: u32, createoptions: u32, eabuffer: *mut core::ffi::c_void, ealength: u32, flags: u32, drivercontext: *mut super::super::super::System::SystemServices::IO_DRIVER_CREATE_CONTEXT) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltCreateFileEx2(filter : PFLT_FILTER, instance : PFLT_INSTANCE, filehandle : *mut super::super::super::super::Win32::Foundation:: HANDLE, fileobject : *mut *mut super::super::super::Foundation:: FILE_OBJECT, desiredaccess : u32, objectattributes : *mut super::super::super::Foundation:: OBJECT_ATTRIBUTES, iostatusblock : *mut super::super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, allocationsize : *mut i64, fileattributes : u32, shareaccess : u32, createdisposition : u32, createoptions : u32, eabuffer : *mut core::ffi::c_void, ealength : u32, flags : u32, drivercontext : *mut super::super::super::System::SystemServices:: IO_DRIVER_CREATE_CONTEXT) -> windows_core:: NTSTATUS);
    unsafe { FltCreateFileEx2(filter, instance, filehandle as _, fileobject as _, desiredaccess, objectattributes as _, iostatusblock as _, allocationsize as _, fileattributes, shareaccess, createdisposition, createoptions, eabuffer as _, ealength, flags, drivercontext as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltCreateMailslotFile(filter: PFLT_FILTER, instance: PFLT_INSTANCE, filehandle: *mut super::super::super::super::Win32::Foundation::HANDLE, fileobject: *mut *mut super::super::super::Foundation::FILE_OBJECT, desiredaccess: u32, objectattributes: *mut super::super::super::Foundation::OBJECT_ATTRIBUTES, iostatusblock: *mut super::super::super::super::Win32::System::IO::IO_STATUS_BLOCK, createoptions: u32, mailslotquota: u32, maximummessagesize: u32, readtimeout: *mut i64, drivercontext: *mut super::super::super::System::SystemServices::IO_DRIVER_CREATE_CONTEXT) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltCreateMailslotFile(filter : PFLT_FILTER, instance : PFLT_INSTANCE, filehandle : *mut super::super::super::super::Win32::Foundation:: HANDLE, fileobject : *mut *mut super::super::super::Foundation:: FILE_OBJECT, desiredaccess : u32, objectattributes : *mut super::super::super::Foundation:: OBJECT_ATTRIBUTES, iostatusblock : *mut super::super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, createoptions : u32, mailslotquota : u32, maximummessagesize : u32, readtimeout : *mut i64, drivercontext : *mut super::super::super::System::SystemServices:: IO_DRIVER_CREATE_CONTEXT) -> windows_core:: NTSTATUS);
    unsafe { FltCreateMailslotFile(filter, instance, filehandle as _, fileobject as _, desiredaccess, objectattributes as _, iostatusblock as _, createoptions, mailslotquota, maximummessagesize, readtimeout as _, drivercontext as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltCreateNamedPipeFile(filter: PFLT_FILTER, instance: PFLT_INSTANCE, filehandle: *mut super::super::super::super::Win32::Foundation::HANDLE, fileobject: *mut *mut super::super::super::Foundation::FILE_OBJECT, desiredaccess: u32, objectattributes: *mut super::super::super::Foundation::OBJECT_ATTRIBUTES, iostatusblock: *mut super::super::super::super::Win32::System::IO::IO_STATUS_BLOCK, shareaccess: u32, createdisposition: u32, createoptions: u32, namedpipetype: u32, readmode: u32, completionmode: u32, maximuminstances: u32, inboundquota: u32, outboundquota: u32, defaulttimeout: *mut i64, drivercontext: *mut super::super::super::System::SystemServices::IO_DRIVER_CREATE_CONTEXT) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltCreateNamedPipeFile(filter : PFLT_FILTER, instance : PFLT_INSTANCE, filehandle : *mut super::super::super::super::Win32::Foundation:: HANDLE, fileobject : *mut *mut super::super::super::Foundation:: FILE_OBJECT, desiredaccess : u32, objectattributes : *mut super::super::super::Foundation:: OBJECT_ATTRIBUTES, iostatusblock : *mut super::super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, shareaccess : u32, createdisposition : u32, createoptions : u32, namedpipetype : u32, readmode : u32, completionmode : u32, maximuminstances : u32, inboundquota : u32, outboundquota : u32, defaulttimeout : *mut i64, drivercontext : *mut super::super::super::System::SystemServices:: IO_DRIVER_CREATE_CONTEXT) -> windows_core:: NTSTATUS);
    unsafe { FltCreateNamedPipeFile(filter, instance, filehandle as _, fileobject as _, desiredaccess, objectattributes as _, iostatusblock as _, shareaccess, createdisposition, createoptions, namedpipetype, readmode, completionmode, maximuminstances, inboundquota, outboundquota, defaulttimeout as _, drivercontext as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltCreateSectionForDataScan(instance: PFLT_INSTANCE, fileobject: *mut super::super::super::Foundation::FILE_OBJECT, sectioncontext: PFLT_CONTEXT, desiredaccess: u32, objectattributes: *mut super::super::super::Foundation::OBJECT_ATTRIBUTES, maximumsize: *mut i64, sectionpageprotection: u32, allocationattributes: u32, flags: u32, sectionhandle: *mut super::super::super::super::Win32::Foundation::HANDLE, sectionobject: *mut *mut core::ffi::c_void, sectionfilesize: *mut i64) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltCreateSectionForDataScan(instance : PFLT_INSTANCE, fileobject : *mut super::super::super::Foundation:: FILE_OBJECT, sectioncontext : PFLT_CONTEXT, desiredaccess : u32, objectattributes : *mut super::super::super::Foundation:: OBJECT_ATTRIBUTES, maximumsize : *mut i64, sectionpageprotection : u32, allocationattributes : u32, flags : u32, sectionhandle : *mut super::super::super::super::Win32::Foundation:: HANDLE, sectionobject : *mut *mut core::ffi::c_void, sectionfilesize : *mut i64) -> windows_core:: NTSTATUS);
    unsafe { FltCreateSectionForDataScan(instance, fileobject as _, sectioncontext, desiredaccess, objectattributes as _, maximumsize as _, sectionpageprotection, allocationattributes, flags, sectionhandle as _, sectionobject as _, sectionfilesize as _) }
}
#[inline]
pub unsafe fn FltCreateSystemVolumeInformationFolder(instance: PFLT_INSTANCE) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltCreateSystemVolumeInformationFolder(instance : PFLT_INSTANCE) -> windows_core:: NTSTATUS);
    unsafe { FltCreateSystemVolumeInformationFolder(instance) }
}
#[inline]
pub unsafe fn FltCurrentBatchOplock(oplock: *const *const core::ffi::c_void) -> bool {
    windows_core::link!("fltmgr.sys" "system" fn FltCurrentBatchOplock(oplock : *const *const core::ffi::c_void) -> bool);
    unsafe { FltCurrentBatchOplock(oplock) }
}
#[inline]
pub unsafe fn FltCurrentOplock(oplock: *mut *mut core::ffi::c_void) -> bool {
    windows_core::link!("fltmgr.sys" "system" fn FltCurrentOplock(oplock : *mut *mut core::ffi::c_void) -> bool);
    unsafe { FltCurrentOplock(oplock as _) }
}
#[inline]
pub unsafe fn FltCurrentOplockH(oplock: *mut *mut core::ffi::c_void) -> bool {
    windows_core::link!("fltmgr.sys" "system" fn FltCurrentOplockH(oplock : *mut *mut core::ffi::c_void) -> bool);
    unsafe { FltCurrentOplockH(oplock as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltDecodeParameters(callbackdata: *const FLT_CALLBACK_DATA, mdladdresspointer: *mut *mut *mut super::super::super::Foundation::MDL, buffer: *mut *mut *mut core::ffi::c_void, length: *mut *mut u32, desiredaccess: *mut super::super::super::System::SystemServices::LOCK_OPERATION) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltDecodeParameters(callbackdata : *const FLT_CALLBACK_DATA, mdladdresspointer : *mut *mut *mut super::super::super::Foundation:: MDL, buffer : *mut *mut *mut core::ffi::c_void, length : *mut *mut u32, desiredaccess : *mut super::super::super::System::SystemServices:: LOCK_OPERATION) -> windows_core:: NTSTATUS);
    unsafe { FltDecodeParameters(callbackdata, mdladdresspointer as _, buffer as _, length as _, desiredaccess as _) }
}
#[inline]
pub unsafe fn FltDeleteContext(context: PFLT_CONTEXT) {
    windows_core::link!("fltmgr.sys" "system" fn FltDeleteContext(context : PFLT_CONTEXT));
    unsafe { FltDeleteContext(context) }
}
#[inline]
pub unsafe fn FltDeleteExtraCreateParameterLookasideList(filter: PFLT_FILTER, lookaside: *mut core::ffi::c_void, flags: u32) {
    windows_core::link!("fltmgr.sys" "system" fn FltDeleteExtraCreateParameterLookasideList(filter : PFLT_FILTER, lookaside : *mut core::ffi::c_void, flags : u32));
    unsafe { FltDeleteExtraCreateParameterLookasideList(filter, lookaside as _, flags) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltDeleteFileContext(instance: PFLT_INSTANCE, fileobject: *mut super::super::super::Foundation::FILE_OBJECT, oldcontext: *mut PFLT_CONTEXT) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltDeleteFileContext(instance : PFLT_INSTANCE, fileobject : *mut super::super::super::Foundation:: FILE_OBJECT, oldcontext : *mut PFLT_CONTEXT) -> windows_core:: NTSTATUS);
    unsafe { FltDeleteFileContext(instance, fileobject as _, oldcontext as _) }
}
#[inline]
pub unsafe fn FltDeleteInstanceContext(instance: PFLT_INSTANCE, oldcontext: *mut PFLT_CONTEXT) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltDeleteInstanceContext(instance : PFLT_INSTANCE, oldcontext : *mut PFLT_CONTEXT) -> windows_core:: NTSTATUS);
    unsafe { FltDeleteInstanceContext(instance, oldcontext as _) }
}
#[inline]
pub unsafe fn FltDeletePushLock() -> usize {
    windows_core::link!("fltmgr.sys" "system" fn FltDeletePushLock(pushlock : *mut usize));
    unsafe {
        let mut result__ = core::mem::zeroed();
        FltDeletePushLock(&mut result__);
        result__
    }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltDeleteStreamContext(instance: PFLT_INSTANCE, fileobject: *mut super::super::super::Foundation::FILE_OBJECT, oldcontext: *mut PFLT_CONTEXT) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltDeleteStreamContext(instance : PFLT_INSTANCE, fileobject : *mut super::super::super::Foundation:: FILE_OBJECT, oldcontext : *mut PFLT_CONTEXT) -> windows_core:: NTSTATUS);
    unsafe { FltDeleteStreamContext(instance, fileobject as _, oldcontext as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltDeleteStreamHandleContext(instance: PFLT_INSTANCE, fileobject: *const super::super::super::Foundation::FILE_OBJECT, oldcontext: *mut PFLT_CONTEXT) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltDeleteStreamHandleContext(instance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, oldcontext : *mut PFLT_CONTEXT) -> windows_core:: NTSTATUS);
    unsafe { FltDeleteStreamHandleContext(instance, fileobject, oldcontext as _) }
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FltDeleteTransactionContext(instance: PFLT_INSTANCE, transaction: *mut super::super::super::Foundation::KTRANSACTION, oldcontext: *mut PFLT_CONTEXT) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltDeleteTransactionContext(instance : PFLT_INSTANCE, transaction : *mut super::super::super::Foundation:: KTRANSACTION, oldcontext : *mut PFLT_CONTEXT) -> windows_core:: NTSTATUS);
    unsafe { FltDeleteTransactionContext(instance, transaction as _, oldcontext as _) }
}
#[inline]
pub unsafe fn FltDeleteVolumeContext(filter: PFLT_FILTER, volume: PFLT_VOLUME, oldcontext: *mut PFLT_CONTEXT) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltDeleteVolumeContext(filter : PFLT_FILTER, volume : PFLT_VOLUME, oldcontext : *mut PFLT_CONTEXT) -> windows_core:: NTSTATUS);
    unsafe { FltDeleteVolumeContext(filter, volume, oldcontext as _) }
}
#[inline]
pub unsafe fn FltDetachVolume(filter: PFLT_FILTER, volume: PFLT_VOLUME, instancename: *mut super::super::super::super::Win32::Foundation::UNICODE_STRING) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltDetachVolume(filter : PFLT_FILTER, volume : PFLT_VOLUME, instancename : *mut super::super::super::super::Win32::Foundation:: UNICODE_STRING) -> windows_core:: NTSTATUS);
    unsafe { FltDetachVolume(filter, volume, instancename as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltDeviceIoControlFile(instance: PFLT_INSTANCE, fileobject: *const super::super::super::Foundation::FILE_OBJECT, iocontrolcode: u32, inputbuffer: *const core::ffi::c_void, inputbufferlength: u32, outputbuffer: *mut core::ffi::c_void, outputbufferlength: u32, lengthreturned: *mut u32) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltDeviceIoControlFile(instance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, iocontrolcode : u32, inputbuffer : *const core::ffi::c_void, inputbufferlength : u32, outputbuffer : *mut core::ffi::c_void, outputbufferlength : u32, lengthreturned : *mut u32) -> windows_core:: NTSTATUS);
    unsafe { FltDeviceIoControlFile(instance, fileobject, iocontrolcode, inputbuffer, inputbufferlength, outputbuffer as _, outputbufferlength, lengthreturned as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltDoCompletionProcessingWhenSafe(data: *mut FLT_CALLBACK_DATA, fltobjects: *mut FLT_RELATED_OBJECTS, completioncontext: *mut core::ffi::c_void, flags: u32, safepostcallback: PFLT_POST_OPERATION_CALLBACK, retpostoperationstatus: *mut FLT_POSTOP_CALLBACK_STATUS) -> bool {
    windows_core::link!("fltmgr.sys" "system" fn FltDoCompletionProcessingWhenSafe(data : *mut FLT_CALLBACK_DATA, fltobjects : *mut FLT_RELATED_OBJECTS, completioncontext : *mut core::ffi::c_void, flags : u32, safepostcallback : PFLT_POST_OPERATION_CALLBACK, retpostoperationstatus : *mut FLT_POSTOP_CALLBACK_STATUS) -> bool);
    unsafe { FltDoCompletionProcessingWhenSafe(data as _, fltobjects as _, completioncontext as _, flags, safepostcallback, retpostoperationstatus as _) }
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FltEnlistInTransaction(instance: PFLT_INSTANCE, transaction: *const super::super::super::Foundation::KTRANSACTION, transactioncontext: PFLT_CONTEXT, notificationmask: u32) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltEnlistInTransaction(instance : PFLT_INSTANCE, transaction : *const super::super::super::Foundation:: KTRANSACTION, transactioncontext : PFLT_CONTEXT, notificationmask : u32) -> windows_core:: NTSTATUS);
    unsafe { FltEnlistInTransaction(instance, transaction, transactioncontext, notificationmask) }
}
#[cfg(feature = "Win32_Storage_InstallableFileSystems")]
#[inline]
pub unsafe fn FltEnumerateFilterInformation(index: u32, informationclass: super::super::super::super::Win32::Storage::InstallableFileSystems::FILTER_INFORMATION_CLASS, buffer: *mut core::ffi::c_void, buffersize: u32, bytesreturned: *mut u32) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltEnumerateFilterInformation(index : u32, informationclass : super::super::super::super::Win32::Storage::InstallableFileSystems:: FILTER_INFORMATION_CLASS, buffer : *mut core::ffi::c_void, buffersize : u32, bytesreturned : *mut u32) -> windows_core:: NTSTATUS);
    unsafe { FltEnumerateFilterInformation(index, informationclass, buffer as _, buffersize, bytesreturned as _) }
}
#[inline]
pub unsafe fn FltEnumerateFilters(filterlist: *mut PFLT_FILTER, filterlistsize: u32, numberfiltersreturned: *mut u32) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltEnumerateFilters(filterlist : *mut PFLT_FILTER, filterlistsize : u32, numberfiltersreturned : *mut u32) -> windows_core:: NTSTATUS);
    unsafe { FltEnumerateFilters(filterlist as _, filterlistsize, numberfiltersreturned as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_Storage_InstallableFileSystems", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltEnumerateInstanceInformationByDeviceObject(deviceobject: *mut super::super::super::Foundation::DEVICE_OBJECT, index: u32, informationclass: super::super::super::super::Win32::Storage::InstallableFileSystems::INSTANCE_INFORMATION_CLASS, buffer: *mut core::ffi::c_void, buffersize: u32, bytesreturned: *mut u32) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltEnumerateInstanceInformationByDeviceObject(deviceobject : *mut super::super::super::Foundation:: DEVICE_OBJECT, index : u32, informationclass : super::super::super::super::Win32::Storage::InstallableFileSystems:: INSTANCE_INFORMATION_CLASS, buffer : *mut core::ffi::c_void, buffersize : u32, bytesreturned : *mut u32) -> windows_core:: NTSTATUS);
    unsafe { FltEnumerateInstanceInformationByDeviceObject(deviceobject as _, index, informationclass, buffer as _, buffersize, bytesreturned as _) }
}
#[cfg(feature = "Win32_Storage_InstallableFileSystems")]
#[inline]
pub unsafe fn FltEnumerateInstanceInformationByFilter(filter: PFLT_FILTER, index: u32, informationclass: super::super::super::super::Win32::Storage::InstallableFileSystems::INSTANCE_INFORMATION_CLASS, buffer: *mut core::ffi::c_void, buffersize: u32, bytesreturned: *mut u32) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltEnumerateInstanceInformationByFilter(filter : PFLT_FILTER, index : u32, informationclass : super::super::super::super::Win32::Storage::InstallableFileSystems:: INSTANCE_INFORMATION_CLASS, buffer : *mut core::ffi::c_void, buffersize : u32, bytesreturned : *mut u32) -> windows_core:: NTSTATUS);
    unsafe { FltEnumerateInstanceInformationByFilter(filter, index, informationclass, buffer as _, buffersize, bytesreturned as _) }
}
#[cfg(feature = "Win32_Storage_InstallableFileSystems")]
#[inline]
pub unsafe fn FltEnumerateInstanceInformationByVolume(volume: PFLT_VOLUME, index: u32, informationclass: super::super::super::super::Win32::Storage::InstallableFileSystems::INSTANCE_INFORMATION_CLASS, buffer: *mut core::ffi::c_void, buffersize: u32, bytesreturned: *mut u32) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltEnumerateInstanceInformationByVolume(volume : PFLT_VOLUME, index : u32, informationclass : super::super::super::super::Win32::Storage::InstallableFileSystems:: INSTANCE_INFORMATION_CLASS, buffer : *mut core::ffi::c_void, buffersize : u32, bytesreturned : *mut u32) -> windows_core:: NTSTATUS);
    unsafe { FltEnumerateInstanceInformationByVolume(volume, index, informationclass, buffer as _, buffersize, bytesreturned as _) }
}
#[cfg(feature = "Win32_Storage_InstallableFileSystems")]
#[inline]
pub unsafe fn FltEnumerateInstanceInformationByVolumeName(volumename: *mut super::super::super::super::Win32::Foundation::UNICODE_STRING, index: u32, informationclass: super::super::super::super::Win32::Storage::InstallableFileSystems::INSTANCE_INFORMATION_CLASS, buffer: *mut core::ffi::c_void, buffersize: u32, bytesreturned: *mut u32) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltEnumerateInstanceInformationByVolumeName(volumename : *mut super::super::super::super::Win32::Foundation:: UNICODE_STRING, index : u32, informationclass : super::super::super::super::Win32::Storage::InstallableFileSystems:: INSTANCE_INFORMATION_CLASS, buffer : *mut core::ffi::c_void, buffersize : u32, bytesreturned : *mut u32) -> windows_core:: NTSTATUS);
    unsafe { FltEnumerateInstanceInformationByVolumeName(volumename as _, index, informationclass, buffer as _, buffersize, bytesreturned as _) }
}
#[inline]
pub unsafe fn FltEnumerateInstances(volume: PFLT_VOLUME, filter: PFLT_FILTER, instancelist: *mut PFLT_INSTANCE, instancelistsize: u32, numberinstancesreturned: *mut u32) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltEnumerateInstances(volume : PFLT_VOLUME, filter : PFLT_FILTER, instancelist : *mut PFLT_INSTANCE, instancelistsize : u32, numberinstancesreturned : *mut u32) -> windows_core:: NTSTATUS);
    unsafe { FltEnumerateInstances(volume, filter, instancelist as _, instancelistsize, numberinstancesreturned as _) }
}
#[cfg(feature = "Win32_Storage_InstallableFileSystems")]
#[inline]
pub unsafe fn FltEnumerateVolumeInformation(filter: PFLT_FILTER, index: u32, informationclass: super::super::super::super::Win32::Storage::InstallableFileSystems::FILTER_VOLUME_INFORMATION_CLASS, buffer: *mut core::ffi::c_void, buffersize: u32, bytesreturned: *mut u32) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltEnumerateVolumeInformation(filter : PFLT_FILTER, index : u32, informationclass : super::super::super::super::Win32::Storage::InstallableFileSystems:: FILTER_VOLUME_INFORMATION_CLASS, buffer : *mut core::ffi::c_void, buffersize : u32, bytesreturned : *mut u32) -> windows_core:: NTSTATUS);
    unsafe { FltEnumerateVolumeInformation(filter, index, informationclass, buffer as _, buffersize, bytesreturned as _) }
}
#[inline]
pub unsafe fn FltEnumerateVolumes(filter: PFLT_FILTER, volumelist: *mut PFLT_VOLUME, volumelistsize: u32, numbervolumesreturned: *mut u32) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltEnumerateVolumes(filter : PFLT_FILTER, volumelist : *mut PFLT_VOLUME, volumelistsize : u32, numbervolumesreturned : *mut u32) -> windows_core:: NTSTATUS);
    unsafe { FltEnumerateVolumes(filter, volumelist as _, volumelistsize, numbervolumesreturned as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltFastIoMdlRead(initiatinginstance: PFLT_INSTANCE, fileobject: *const super::super::super::Foundation::FILE_OBJECT, fileoffset: *const i64, length: u32, lockkey: u32, mdlchain: *mut *mut super::super::super::Foundation::MDL, iostatus: *mut super::super::super::super::Win32::System::IO::IO_STATUS_BLOCK) -> bool {
    windows_core::link!("fltmgr.sys" "system" fn FltFastIoMdlRead(initiatinginstance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, length : u32, lockkey : u32, mdlchain : *mut *mut super::super::super::Foundation:: MDL, iostatus : *mut super::super::super::super::Win32::System::IO:: IO_STATUS_BLOCK) -> bool);
    unsafe { FltFastIoMdlRead(initiatinginstance, fileobject, fileoffset, length, lockkey, mdlchain as _, iostatus as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltFastIoMdlReadComplete(initiatinginstance: PFLT_INSTANCE, fileobject: *mut super::super::super::Foundation::FILE_OBJECT, mdlchain: *mut super::super::super::Foundation::MDL) -> bool {
    windows_core::link!("fltmgr.sys" "system" fn FltFastIoMdlReadComplete(initiatinginstance : PFLT_INSTANCE, fileobject : *mut super::super::super::Foundation:: FILE_OBJECT, mdlchain : *mut super::super::super::Foundation:: MDL) -> bool);
    unsafe { FltFastIoMdlReadComplete(initiatinginstance, fileobject as _, mdlchain as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltFastIoMdlWriteComplete(initiatinginstance: PFLT_INSTANCE, fileobject: *mut super::super::super::Foundation::FILE_OBJECT, fileoffset: *mut i64, mdlchain: *mut super::super::super::Foundation::MDL) -> bool {
    windows_core::link!("fltmgr.sys" "system" fn FltFastIoMdlWriteComplete(initiatinginstance : PFLT_INSTANCE, fileobject : *mut super::super::super::Foundation:: FILE_OBJECT, fileoffset : *mut i64, mdlchain : *mut super::super::super::Foundation:: MDL) -> bool);
    unsafe { FltFastIoMdlWriteComplete(initiatinginstance, fileobject as _, fileoffset as _, mdlchain as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltFastIoPrepareMdlWrite(initiatinginstance: PFLT_INSTANCE, fileobject: *const super::super::super::Foundation::FILE_OBJECT, fileoffset: *const i64, length: u32, lockkey: u32, mdlchain: *mut *mut super::super::super::Foundation::MDL, iostatus: *mut super::super::super::super::Win32::System::IO::IO_STATUS_BLOCK) -> bool {
    windows_core::link!("fltmgr.sys" "system" fn FltFastIoPrepareMdlWrite(initiatinginstance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, fileoffset : *const i64, length : u32, lockkey : u32, mdlchain : *mut *mut super::super::super::Foundation:: MDL, iostatus : *mut super::super::super::super::Win32::System::IO:: IO_STATUS_BLOCK) -> bool);
    unsafe { FltFastIoPrepareMdlWrite(initiatinginstance, fileobject, fileoffset, length, lockkey, mdlchain as _, iostatus as _) }
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FltFindExtraCreateParameter(filter: PFLT_FILTER, ecplist: *mut super::super::super::Foundation::ECP_LIST, ecptype: *mut windows_core::GUID, ecpcontext: *mut *mut core::ffi::c_void, ecpcontextsize: *mut u32) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltFindExtraCreateParameter(filter : PFLT_FILTER, ecplist : *mut super::super::super::Foundation:: ECP_LIST, ecptype : *mut windows_core::GUID, ecpcontext : *mut *mut core::ffi::c_void, ecpcontextsize : *mut u32) -> windows_core:: NTSTATUS);
    unsafe { FltFindExtraCreateParameter(filter, ecplist as _, ecptype as _, ecpcontext as _, ecpcontextsize as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltFlushBuffers(instance: PFLT_INSTANCE, fileobject: *mut super::super::super::Foundation::FILE_OBJECT) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltFlushBuffers(instance : PFLT_INSTANCE, fileobject : *mut super::super::super::Foundation:: FILE_OBJECT) -> windows_core:: NTSTATUS);
    unsafe { FltFlushBuffers(instance, fileobject as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltFlushBuffers2(instance: PFLT_INSTANCE, fileobject: *const super::super::super::Foundation::FILE_OBJECT, flushtype: u32, callbackdata: *const FLT_CALLBACK_DATA) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltFlushBuffers2(instance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, flushtype : u32, callbackdata : *const FLT_CALLBACK_DATA) -> windows_core:: NTSTATUS);
    unsafe { FltFlushBuffers2(instance, fileobject, flushtype, callbackdata) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltFreeCallbackData(callbackdata: *const FLT_CALLBACK_DATA) {
    windows_core::link!("fltmgr.sys" "system" fn FltFreeCallbackData(callbackdata : *const FLT_CALLBACK_DATA));
    unsafe { FltFreeCallbackData(callbackdata) }
}
#[inline]
pub unsafe fn FltFreeDeferredIoWorkItem(fltworkitem: PFLT_DEFERRED_IO_WORKITEM) {
    windows_core::link!("fltmgr.sys" "system" fn FltFreeDeferredIoWorkItem(fltworkitem : PFLT_DEFERRED_IO_WORKITEM));
    unsafe { FltFreeDeferredIoWorkItem(fltworkitem) }
}
#[inline]
pub unsafe fn FltFreeExtraCreateParameter(filter: PFLT_FILTER, ecpcontext: *const core::ffi::c_void) {
    windows_core::link!("fltmgr.sys" "system" fn FltFreeExtraCreateParameter(filter : PFLT_FILTER, ecpcontext : *const core::ffi::c_void));
    unsafe { FltFreeExtraCreateParameter(filter, ecpcontext) }
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FltFreeExtraCreateParameterList(filter: PFLT_FILTER) -> super::super::super::Foundation::ECP_LIST {
    windows_core::link!("fltmgr.sys" "system" fn FltFreeExtraCreateParameterList(filter : PFLT_FILTER, ecplist : *mut super::super::super::Foundation:: ECP_LIST));
    unsafe {
        let mut result__ = core::mem::zeroed();
        FltFreeExtraCreateParameterList(filter, &mut result__);
        result__
    }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltFreeFileLock(filelock: *const super::FILE_LOCK) {
    windows_core::link!("fltmgr.sys" "system" fn FltFreeFileLock(filelock : *const super:: FILE_LOCK));
    unsafe { FltFreeFileLock(filelock) }
}
#[inline]
pub unsafe fn FltFreeGenericWorkItem(fltworkitem: PFLT_GENERIC_WORKITEM) {
    windows_core::link!("fltmgr.sys" "system" fn FltFreeGenericWorkItem(fltworkitem : PFLT_GENERIC_WORKITEM));
    unsafe { FltFreeGenericWorkItem(fltworkitem) }
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FltFreeOpenReparseList(filter: PFLT_FILTER) -> super::super::super::Foundation::ECP_LIST {
    windows_core::link!("fltmgr.sys" "system" fn FltFreeOpenReparseList(filter : PFLT_FILTER, ecplist : *mut super::super::super::Foundation:: ECP_LIST));
    unsafe {
        let mut result__ = core::mem::zeroed();
        FltFreeOpenReparseList(filter, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn FltFreePoolAlignedWithTag(instance: PFLT_INSTANCE, buffer: *mut core::ffi::c_void, tag: u32) {
    windows_core::link!("fltmgr.sys" "system" fn FltFreePoolAlignedWithTag(instance : PFLT_INSTANCE, buffer : *mut core::ffi::c_void, tag : u32));
    unsafe { FltFreePoolAlignedWithTag(instance, buffer as _, tag) }
}
#[cfg(feature = "Win32_Security")]
#[inline]
pub unsafe fn FltFreeSecurityDescriptor(securitydescriptor: super::super::super::super::Win32::Security::PSECURITY_DESCRIPTOR) {
    windows_core::link!("fltmgr.sys" "system" fn FltFreeSecurityDescriptor(securitydescriptor : super::super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR));
    unsafe { FltFreeSecurityDescriptor(securitydescriptor) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltFsControlFile(instance: PFLT_INSTANCE, fileobject: *const super::super::super::Foundation::FILE_OBJECT, fscontrolcode: u32, inputbuffer: *const core::ffi::c_void, inputbufferlength: u32, outputbuffer: *mut core::ffi::c_void, outputbufferlength: u32, lengthreturned: *mut u32) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltFsControlFile(instance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, fscontrolcode : u32, inputbuffer : *const core::ffi::c_void, inputbufferlength : u32, outputbuffer : *mut core::ffi::c_void, outputbufferlength : u32, lengthreturned : *mut u32) -> windows_core:: NTSTATUS);
    unsafe { FltFsControlFile(instance, fileobject, fscontrolcode, inputbuffer, inputbufferlength, outputbuffer as _, outputbufferlength, lengthreturned as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetActivityIdCallbackData(callbackdata: *const FLT_CALLBACK_DATA, guid: *mut windows_core::GUID) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltGetActivityIdCallbackData(callbackdata : *const FLT_CALLBACK_DATA, guid : *mut windows_core::GUID) -> windows_core:: NTSTATUS);
    unsafe { FltGetActivityIdCallbackData(callbackdata, guid as _) }
}
#[inline]
pub unsafe fn FltGetBottomInstance(volume: PFLT_VOLUME, instance: *mut PFLT_INSTANCE) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltGetBottomInstance(volume : PFLT_VOLUME, instance : *mut PFLT_INSTANCE) -> windows_core:: NTSTATUS);
    unsafe { FltGetBottomInstance(volume, instance as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetContexts(fltobjects: *const FLT_RELATED_OBJECTS, desiredcontexts: u16, contexts: *mut FLT_RELATED_CONTEXTS) {
    windows_core::link!("fltmgr.sys" "system" fn FltGetContexts(fltobjects : *const FLT_RELATED_OBJECTS, desiredcontexts : u16, contexts : *mut FLT_RELATED_CONTEXTS));
    unsafe { FltGetContexts(fltobjects, desiredcontexts, contexts as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetContextsEx(fltobjects: *const FLT_RELATED_OBJECTS, desiredcontexts: u16, contextssize: usize, contexts: *mut FLT_RELATED_CONTEXTS_EX) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltGetContextsEx(fltobjects : *const FLT_RELATED_OBJECTS, desiredcontexts : u16, contextssize : usize, contexts : *mut FLT_RELATED_CONTEXTS_EX) -> windows_core:: NTSTATUS);
    unsafe { FltGetContextsEx(fltobjects, desiredcontexts, contextssize, contexts as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetDestinationFileNameInformation<P3>(instance: PFLT_INSTANCE, fileobject: *mut super::super::super::Foundation::FILE_OBJECT, rootdirectory: super::super::super::super::Win32::Foundation::HANDLE, filename: P3, filenamelength: u32, nameoptions: u32, retfilenameinformation: *mut *mut FLT_FILE_NAME_INFORMATION) -> windows_core::NTSTATUS
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("fltmgr.sys" "system" fn FltGetDestinationFileNameInformation(instance : PFLT_INSTANCE, fileobject : *mut super::super::super::Foundation:: FILE_OBJECT, rootdirectory : super::super::super::super::Win32::Foundation:: HANDLE, filename : windows_core::PCWSTR, filenamelength : u32, nameoptions : u32, retfilenameinformation : *mut *mut FLT_FILE_NAME_INFORMATION) -> windows_core:: NTSTATUS);
    unsafe { FltGetDestinationFileNameInformation(instance, fileobject as _, rootdirectory, filename.param().abi(), filenamelength, nameoptions, retfilenameinformation as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetDeviceObject(volume: PFLT_VOLUME, deviceobject: *mut *mut super::super::super::Foundation::DEVICE_OBJECT) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltGetDeviceObject(volume : PFLT_VOLUME, deviceobject : *mut *mut super::super::super::Foundation:: DEVICE_OBJECT) -> windows_core:: NTSTATUS);
    unsafe { FltGetDeviceObject(volume, deviceobject as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetDiskDeviceObject(volume: PFLT_VOLUME, diskdeviceobject: *mut *mut super::super::super::Foundation::DEVICE_OBJECT) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltGetDiskDeviceObject(volume : PFLT_VOLUME, diskdeviceobject : *mut *mut super::super::super::Foundation:: DEVICE_OBJECT) -> windows_core:: NTSTATUS);
    unsafe { FltGetDiskDeviceObject(volume, diskdeviceobject as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetEcpListFromCallbackData(filter: PFLT_FILTER, callbackdata: *const FLT_CALLBACK_DATA, ecplist: *mut *mut super::super::super::Foundation::ECP_LIST) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltGetEcpListFromCallbackData(filter : PFLT_FILTER, callbackdata : *const FLT_CALLBACK_DATA, ecplist : *mut *mut super::super::super::Foundation:: ECP_LIST) -> windows_core:: NTSTATUS);
    unsafe { FltGetEcpListFromCallbackData(filter, callbackdata, ecplist as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetFileContext(instance: PFLT_INSTANCE, fileobject: *mut super::super::super::Foundation::FILE_OBJECT, context: *mut PFLT_CONTEXT) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltGetFileContext(instance : PFLT_INSTANCE, fileobject : *mut super::super::super::Foundation:: FILE_OBJECT, context : *mut PFLT_CONTEXT) -> windows_core:: NTSTATUS);
    unsafe { FltGetFileContext(instance, fileobject as _, context as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetFileNameInformation(callbackdata: *const FLT_CALLBACK_DATA, nameoptions: u32, filenameinformation: *mut *mut FLT_FILE_NAME_INFORMATION) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltGetFileNameInformation(callbackdata : *const FLT_CALLBACK_DATA, nameoptions : u32, filenameinformation : *mut *mut FLT_FILE_NAME_INFORMATION) -> windows_core:: NTSTATUS);
    unsafe { FltGetFileNameInformation(callbackdata, nameoptions, filenameinformation as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetFileNameInformationUnsafe(fileobject: *const super::super::super::Foundation::FILE_OBJECT, instance: PFLT_INSTANCE, nameoptions: u32, filenameinformation: *mut *mut FLT_FILE_NAME_INFORMATION) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltGetFileNameInformationUnsafe(fileobject : *const super::super::super::Foundation:: FILE_OBJECT, instance : PFLT_INSTANCE, nameoptions : u32, filenameinformation : *mut *mut FLT_FILE_NAME_INFORMATION) -> windows_core:: NTSTATUS);
    unsafe { FltGetFileNameInformationUnsafe(fileobject, instance, nameoptions, filenameinformation as _) }
}
#[cfg(feature = "Win32_Storage_InstallableFileSystems")]
#[inline]
pub unsafe fn FltGetFileSystemType(fltobject: *mut core::ffi::c_void, filesystemtype: *mut super::super::super::super::Win32::Storage::InstallableFileSystems::FLT_FILESYSTEM_TYPE) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltGetFileSystemType(fltobject : *mut core::ffi::c_void, filesystemtype : *mut super::super::super::super::Win32::Storage::InstallableFileSystems:: FLT_FILESYSTEM_TYPE) -> windows_core:: NTSTATUS);
    unsafe { FltGetFileSystemType(fltobject as _, filesystemtype as _) }
}
#[inline]
pub unsafe fn FltGetFilterFromInstance(instance: PFLT_INSTANCE, retfilter: *mut PFLT_FILTER) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltGetFilterFromInstance(instance : PFLT_INSTANCE, retfilter : *mut PFLT_FILTER) -> windows_core:: NTSTATUS);
    unsafe { FltGetFilterFromInstance(instance, retfilter as _) }
}
#[inline]
pub unsafe fn FltGetFilterFromName(filtername: *const super::super::super::super::Win32::Foundation::UNICODE_STRING, retfilter: *mut PFLT_FILTER) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltGetFilterFromName(filtername : *const super::super::super::super::Win32::Foundation:: UNICODE_STRING, retfilter : *mut PFLT_FILTER) -> windows_core:: NTSTATUS);
    unsafe { FltGetFilterFromName(filtername, retfilter as _) }
}
#[cfg(feature = "Win32_Storage_InstallableFileSystems")]
#[inline]
pub unsafe fn FltGetFilterInformation(filter: PFLT_FILTER, informationclass: super::super::super::super::Win32::Storage::InstallableFileSystems::FILTER_INFORMATION_CLASS, buffer: *mut core::ffi::c_void, buffersize: u32, bytesreturned: *mut u32) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltGetFilterInformation(filter : PFLT_FILTER, informationclass : super::super::super::super::Win32::Storage::InstallableFileSystems:: FILTER_INFORMATION_CLASS, buffer : *mut core::ffi::c_void, buffersize : u32, bytesreturned : *mut u32) -> windows_core:: NTSTATUS);
    unsafe { FltGetFilterInformation(filter, informationclass, buffer as _, buffersize, bytesreturned as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetFsZeroingOffset(data: *mut FLT_CALLBACK_DATA, zeroingoffset: *mut u32) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltGetFsZeroingOffset(data : *mut FLT_CALLBACK_DATA, zeroingoffset : *mut u32) -> windows_core:: NTSTATUS);
    unsafe { FltGetFsZeroingOffset(data as _, zeroingoffset as _) }
}
#[inline]
pub unsafe fn FltGetInstanceContext(instance: PFLT_INSTANCE, context: *mut PFLT_CONTEXT) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltGetInstanceContext(instance : PFLT_INSTANCE, context : *mut PFLT_CONTEXT) -> windows_core:: NTSTATUS);
    unsafe { FltGetInstanceContext(instance, context as _) }
}
#[cfg(feature = "Win32_Storage_InstallableFileSystems")]
#[inline]
pub unsafe fn FltGetInstanceInformation(instance: PFLT_INSTANCE, informationclass: super::super::super::super::Win32::Storage::InstallableFileSystems::INSTANCE_INFORMATION_CLASS, buffer: *mut core::ffi::c_void, buffersize: u32, bytesreturned: *mut u32) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltGetInstanceInformation(instance : PFLT_INSTANCE, informationclass : super::super::super::super::Win32::Storage::InstallableFileSystems:: INSTANCE_INFORMATION_CLASS, buffer : *mut core::ffi::c_void, buffersize : u32, bytesreturned : *mut u32) -> windows_core:: NTSTATUS);
    unsafe { FltGetInstanceInformation(instance, informationclass, buffer as _, buffersize, bytesreturned as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetIoAttributionHandleFromCallbackData(data: *mut FLT_CALLBACK_DATA) -> *mut core::ffi::c_void {
    windows_core::link!("fltmgr.sys" "system" fn FltGetIoAttributionHandleFromCallbackData(data : *mut FLT_CALLBACK_DATA) -> *mut core::ffi::c_void);
    unsafe { FltGetIoAttributionHandleFromCallbackData(data as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetIoPriorityHint(data: *mut FLT_CALLBACK_DATA) -> super::super::super::Foundation::IO_PRIORITY_HINT {
    windows_core::link!("fltmgr.sys" "system" fn FltGetIoPriorityHint(data : *mut FLT_CALLBACK_DATA) -> super::super::super::Foundation:: IO_PRIORITY_HINT);
    unsafe { FltGetIoPriorityHint(data as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetIoPriorityHintFromCallbackData(data: *mut FLT_CALLBACK_DATA) -> super::super::super::Foundation::IO_PRIORITY_HINT {
    windows_core::link!("fltmgr.sys" "system" fn FltGetIoPriorityHintFromCallbackData(data : *mut FLT_CALLBACK_DATA) -> super::super::super::Foundation:: IO_PRIORITY_HINT);
    unsafe { FltGetIoPriorityHintFromCallbackData(data as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetIoPriorityHintFromFileObject(fileobject: *mut super::super::super::Foundation::FILE_OBJECT) -> super::super::super::Foundation::IO_PRIORITY_HINT {
    windows_core::link!("fltmgr.sys" "system" fn FltGetIoPriorityHintFromFileObject(fileobject : *mut super::super::super::Foundation:: FILE_OBJECT) -> super::super::super::Foundation:: IO_PRIORITY_HINT);
    unsafe { FltGetIoPriorityHintFromFileObject(fileobject as _) }
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FltGetIoPriorityHintFromThread(thread: super::super::super::Foundation::PETHREAD) -> super::super::super::Foundation::IO_PRIORITY_HINT {
    windows_core::link!("fltmgr.sys" "system" fn FltGetIoPriorityHintFromThread(thread : super::super::super::Foundation:: PETHREAD) -> super::super::super::Foundation:: IO_PRIORITY_HINT);
    unsafe { FltGetIoPriorityHintFromThread(thread) }
}
#[inline]
pub unsafe fn FltGetIrpName(irpmajorcode: u8) -> windows_core::PSTR {
    windows_core::link!("fltmgr.sys" "system" fn FltGetIrpName(irpmajorcode : u8) -> windows_core::PSTR);
    unsafe { FltGetIrpName(irpmajorcode) }
}
#[inline]
pub unsafe fn FltGetLowerInstance(currentinstance: PFLT_INSTANCE, lowerinstance: *mut PFLT_INSTANCE) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltGetLowerInstance(currentinstance : PFLT_INSTANCE, lowerinstance : *mut PFLT_INSTANCE) -> windows_core:: NTSTATUS);
    unsafe { FltGetLowerInstance(currentinstance, lowerinstance as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetNewSystemBufferAddress(callbackdata: *mut FLT_CALLBACK_DATA) -> *mut core::ffi::c_void {
    windows_core::link!("fltmgr.sys" "system" fn FltGetNewSystemBufferAddress(callbackdata : *mut FLT_CALLBACK_DATA) -> *mut core::ffi::c_void);
    unsafe { FltGetNewSystemBufferAddress(callbackdata as _) }
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FltGetNextExtraCreateParameter(filter: PFLT_FILTER, ecplist: *const super::super::super::Foundation::ECP_LIST, currentecpcontext: *const core::ffi::c_void, nextecptype: *mut windows_core::GUID, nextecpcontext: *mut *mut core::ffi::c_void, nextecpcontextsize: *mut u32) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltGetNextExtraCreateParameter(filter : PFLT_FILTER, ecplist : *const super::super::super::Foundation:: ECP_LIST, currentecpcontext : *const core::ffi::c_void, nextecptype : *mut windows_core::GUID, nextecpcontext : *mut *mut core::ffi::c_void, nextecpcontextsize : *mut u32) -> windows_core:: NTSTATUS);
    unsafe { FltGetNextExtraCreateParameter(filter, ecplist, currentecpcontext, nextecptype as _, nextecpcontext as _, nextecpcontextsize as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetRequestorProcess(callbackdata: *const FLT_CALLBACK_DATA) -> super::super::super::Foundation::PEPROCESS {
    windows_core::link!("fltmgr.sys" "system" fn FltGetRequestorProcess(callbackdata : *const FLT_CALLBACK_DATA) -> super::super::super::Foundation:: PEPROCESS);
    unsafe { FltGetRequestorProcess(callbackdata) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetRequestorProcessId(callbackdata: *const FLT_CALLBACK_DATA) -> u32 {
    windows_core::link!("fltmgr.sys" "system" fn FltGetRequestorProcessId(callbackdata : *const FLT_CALLBACK_DATA) -> u32);
    unsafe { FltGetRequestorProcessId(callbackdata) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetRequestorProcessIdEx(callbackdata: *mut FLT_CALLBACK_DATA) -> super::super::super::super::Win32::Foundation::HANDLE {
    windows_core::link!("fltmgr.sys" "system" fn FltGetRequestorProcessIdEx(callbackdata : *mut FLT_CALLBACK_DATA) -> super::super::super::super::Win32::Foundation:: HANDLE);
    unsafe { FltGetRequestorProcessIdEx(callbackdata as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetRequestorSessionId(callbackdata: *mut FLT_CALLBACK_DATA, sessionid: *mut u32) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltGetRequestorSessionId(callbackdata : *mut FLT_CALLBACK_DATA, sessionid : *mut u32) -> windows_core:: NTSTATUS);
    unsafe { FltGetRequestorSessionId(callbackdata as _, sessionid as _) }
}
#[inline]
pub unsafe fn FltGetRoutineAddress<P0>(fltmgrroutinename: P0) -> *mut core::ffi::c_void
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("fltmgr.sys" "system" fn FltGetRoutineAddress(fltmgrroutinename : windows_core::PCSTR) -> *mut core::ffi::c_void);
    unsafe { FltGetRoutineAddress(fltmgrroutinename.param().abi()) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetSectionContext(instance: PFLT_INSTANCE, fileobject: *mut super::super::super::Foundation::FILE_OBJECT, context: *mut PFLT_CONTEXT) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltGetSectionContext(instance : PFLT_INSTANCE, fileobject : *mut super::super::super::Foundation:: FILE_OBJECT, context : *mut PFLT_CONTEXT) -> windows_core:: NTSTATUS);
    unsafe { FltGetSectionContext(instance, fileobject as _, context as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetStreamContext(instance: PFLT_INSTANCE, fileobject: *mut super::super::super::Foundation::FILE_OBJECT, context: *mut PFLT_CONTEXT) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltGetStreamContext(instance : PFLT_INSTANCE, fileobject : *mut super::super::super::Foundation:: FILE_OBJECT, context : *mut PFLT_CONTEXT) -> windows_core:: NTSTATUS);
    unsafe { FltGetStreamContext(instance, fileobject as _, context as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetStreamHandleContext(instance: PFLT_INSTANCE, fileobject: *mut super::super::super::Foundation::FILE_OBJECT, context: *mut PFLT_CONTEXT) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltGetStreamHandleContext(instance : PFLT_INSTANCE, fileobject : *mut super::super::super::Foundation:: FILE_OBJECT, context : *mut PFLT_CONTEXT) -> windows_core:: NTSTATUS);
    unsafe { FltGetStreamHandleContext(instance, fileobject as _, context as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetSwappedBufferMdlAddress(callbackdata: *const FLT_CALLBACK_DATA) -> *mut super::super::super::Foundation::MDL {
    windows_core::link!("fltmgr.sys" "system" fn FltGetSwappedBufferMdlAddress(callbackdata : *const FLT_CALLBACK_DATA) -> *mut super::super::super::Foundation:: MDL);
    unsafe { FltGetSwappedBufferMdlAddress(callbackdata) }
}
#[inline]
pub unsafe fn FltGetTopInstance(volume: PFLT_VOLUME, instance: *mut PFLT_INSTANCE) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltGetTopInstance(volume : PFLT_VOLUME, instance : *mut PFLT_INSTANCE) -> windows_core:: NTSTATUS);
    unsafe { FltGetTopInstance(volume, instance as _) }
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FltGetTransactionContext(instance: PFLT_INSTANCE, transaction: *mut super::super::super::Foundation::KTRANSACTION, context: *mut PFLT_CONTEXT) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltGetTransactionContext(instance : PFLT_INSTANCE, transaction : *mut super::super::super::Foundation:: KTRANSACTION, context : *mut PFLT_CONTEXT) -> windows_core:: NTSTATUS);
    unsafe { FltGetTransactionContext(instance, transaction as _, context as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetTunneledName(callbackdata: *mut FLT_CALLBACK_DATA, filenameinformation: *mut FLT_FILE_NAME_INFORMATION, rettunneledfilenameinformation: *mut *mut FLT_FILE_NAME_INFORMATION) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltGetTunneledName(callbackdata : *mut FLT_CALLBACK_DATA, filenameinformation : *mut FLT_FILE_NAME_INFORMATION, rettunneledfilenameinformation : *mut *mut FLT_FILE_NAME_INFORMATION) -> windows_core:: NTSTATUS);
    unsafe { FltGetTunneledName(callbackdata as _, filenameinformation as _, rettunneledfilenameinformation as _) }
}
#[inline]
pub unsafe fn FltGetUpperInstance(currentinstance: PFLT_INSTANCE, upperinstance: *mut PFLT_INSTANCE) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltGetUpperInstance(currentinstance : PFLT_INSTANCE, upperinstance : *mut PFLT_INSTANCE) -> windows_core:: NTSTATUS);
    unsafe { FltGetUpperInstance(currentinstance, upperinstance as _) }
}
#[inline]
pub unsafe fn FltGetVolumeContext(filter: PFLT_FILTER, volume: PFLT_VOLUME, context: *mut PFLT_CONTEXT) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltGetVolumeContext(filter : PFLT_FILTER, volume : PFLT_VOLUME, context : *mut PFLT_CONTEXT) -> windows_core:: NTSTATUS);
    unsafe { FltGetVolumeContext(filter, volume, context as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetVolumeFromDeviceObject(filter: PFLT_FILTER, deviceobject: *const super::super::super::Foundation::DEVICE_OBJECT, retvolume: *mut PFLT_VOLUME) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltGetVolumeFromDeviceObject(filter : PFLT_FILTER, deviceobject : *const super::super::super::Foundation:: DEVICE_OBJECT, retvolume : *mut PFLT_VOLUME) -> windows_core:: NTSTATUS);
    unsafe { FltGetVolumeFromDeviceObject(filter, deviceobject, retvolume as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltGetVolumeFromFileObject(filter: PFLT_FILTER, fileobject: *mut super::super::super::Foundation::FILE_OBJECT, retvolume: *mut PFLT_VOLUME) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltGetVolumeFromFileObject(filter : PFLT_FILTER, fileobject : *mut super::super::super::Foundation:: FILE_OBJECT, retvolume : *mut PFLT_VOLUME) -> windows_core:: NTSTATUS);
    unsafe { FltGetVolumeFromFileObject(filter, fileobject as _, retvolume as _) }
}
#[inline]
pub unsafe fn FltGetVolumeFromInstance(instance: PFLT_INSTANCE, retvolume: *mut PFLT_VOLUME) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltGetVolumeFromInstance(instance : PFLT_INSTANCE, retvolume : *mut PFLT_VOLUME) -> windows_core:: NTSTATUS);
    unsafe { FltGetVolumeFromInstance(instance, retvolume as _) }
}
#[inline]
pub unsafe fn FltGetVolumeFromName(filter: PFLT_FILTER, volumename: *mut super::super::super::super::Win32::Foundation::UNICODE_STRING, retvolume: *mut PFLT_VOLUME) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltGetVolumeFromName(filter : PFLT_FILTER, volumename : *mut super::super::super::super::Win32::Foundation:: UNICODE_STRING, retvolume : *mut PFLT_VOLUME) -> windows_core:: NTSTATUS);
    unsafe { FltGetVolumeFromName(filter, volumename as _, retvolume as _) }
}
#[inline]
pub unsafe fn FltGetVolumeGuidName(volume: PFLT_VOLUME, volumeguidname: *mut super::super::super::super::Win32::Foundation::UNICODE_STRING, buffersizeneeded: *mut u32) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltGetVolumeGuidName(volume : PFLT_VOLUME, volumeguidname : *mut super::super::super::super::Win32::Foundation:: UNICODE_STRING, buffersizeneeded : *mut u32) -> windows_core:: NTSTATUS);
    unsafe { FltGetVolumeGuidName(volume, volumeguidname as _, buffersizeneeded as _) }
}
#[cfg(feature = "Win32_Storage_InstallableFileSystems")]
#[inline]
pub unsafe fn FltGetVolumeInformation(volume: PFLT_VOLUME, informationclass: super::super::super::super::Win32::Storage::InstallableFileSystems::FILTER_VOLUME_INFORMATION_CLASS, buffer: *mut core::ffi::c_void, buffersize: u32, bytesreturned: *mut u32) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltGetVolumeInformation(volume : PFLT_VOLUME, informationclass : super::super::super::super::Win32::Storage::InstallableFileSystems:: FILTER_VOLUME_INFORMATION_CLASS, buffer : *mut core::ffi::c_void, buffersize : u32, bytesreturned : *mut u32) -> windows_core:: NTSTATUS);
    unsafe { FltGetVolumeInformation(volume, informationclass, buffer as _, buffersize, bytesreturned as _) }
}
#[inline]
pub unsafe fn FltGetVolumeInstanceFromName(filter: PFLT_FILTER, volume: PFLT_VOLUME, instancename: *const super::super::super::super::Win32::Foundation::UNICODE_STRING, retinstance: *mut PFLT_INSTANCE) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltGetVolumeInstanceFromName(filter : PFLT_FILTER, volume : PFLT_VOLUME, instancename : *const super::super::super::super::Win32::Foundation:: UNICODE_STRING, retinstance : *mut PFLT_INSTANCE) -> windows_core:: NTSTATUS);
    unsafe { FltGetVolumeInstanceFromName(filter, volume, instancename, retinstance as _) }
}
#[inline]
pub unsafe fn FltGetVolumeName(volume: PFLT_VOLUME, volumename: *mut super::super::super::super::Win32::Foundation::UNICODE_STRING, buffersizeneeded: *mut u32) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltGetVolumeName(volume : PFLT_VOLUME, volumename : *mut super::super::super::super::Win32::Foundation:: UNICODE_STRING, buffersizeneeded : *mut u32) -> windows_core:: NTSTATUS);
    unsafe { FltGetVolumeName(volume, volumename as _, buffersizeneeded as _) }
}
#[inline]
pub unsafe fn FltGetVolumeProperties(volume: PFLT_VOLUME, volumeproperties: *mut FLT_VOLUME_PROPERTIES, volumepropertieslength: u32, lengthreturned: *mut u32) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltGetVolumeProperties(volume : PFLT_VOLUME, volumeproperties : *mut FLT_VOLUME_PROPERTIES, volumepropertieslength : u32, lengthreturned : *mut u32) -> windows_core:: NTSTATUS);
    unsafe { FltGetVolumeProperties(volume, volumeproperties as _, volumepropertieslength, lengthreturned as _) }
}
#[inline]
pub unsafe fn FltInitExtraCreateParameterLookasideList(filter: PFLT_FILTER, lookaside: *mut core::ffi::c_void, flags: u32, size: usize, tag: u32) {
    windows_core::link!("fltmgr.sys" "system" fn FltInitExtraCreateParameterLookasideList(filter : PFLT_FILTER, lookaside : *mut core::ffi::c_void, flags : u32, size : usize, tag : u32));
    unsafe { FltInitExtraCreateParameterLookasideList(filter, lookaside as _, flags, size, tag) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltInitializeFileLock(filelock: *mut super::FILE_LOCK) {
    windows_core::link!("fltmgr.sys" "system" fn FltInitializeFileLock(filelock : *mut super:: FILE_LOCK));
    unsafe { FltInitializeFileLock(filelock as _) }
}
#[inline]
pub unsafe fn FltInitializeOplock(oplock: *mut *mut core::ffi::c_void) {
    windows_core::link!("fltmgr.sys" "system" fn FltInitializeOplock(oplock : *mut *mut core::ffi::c_void));
    unsafe { FltInitializeOplock(oplock as _) }
}
#[inline]
pub unsafe fn FltInitializePushLock() -> usize {
    windows_core::link!("fltmgr.sys" "system" fn FltInitializePushLock(pushlock : *mut usize));
    unsafe {
        let mut result__ = core::mem::zeroed();
        FltInitializePushLock(&mut result__);
        result__
    }
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FltInsertExtraCreateParameter(filter: PFLT_FILTER, ecplist: *mut super::super::super::Foundation::ECP_LIST, ecpcontext: *mut core::ffi::c_void) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltInsertExtraCreateParameter(filter : PFLT_FILTER, ecplist : *mut super::super::super::Foundation:: ECP_LIST, ecpcontext : *mut core::ffi::c_void) -> windows_core:: NTSTATUS);
    unsafe { FltInsertExtraCreateParameter(filter, ecplist as _, ecpcontext as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltIs32bitProcess(callbackdata: *mut FLT_CALLBACK_DATA) -> bool {
    windows_core::link!("fltmgr.sys" "system" fn FltIs32bitProcess(callbackdata : *mut FLT_CALLBACK_DATA) -> bool);
    unsafe { FltIs32bitProcess(callbackdata as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltIsCallbackDataDirty(data: *const FLT_CALLBACK_DATA) -> bool {
    windows_core::link!("fltmgr.sys" "system" fn FltIsCallbackDataDirty(data : *const FLT_CALLBACK_DATA) -> bool);
    unsafe { FltIsCallbackDataDirty(data) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltIsDirectory(fileobject: *mut super::super::super::Foundation::FILE_OBJECT, instance: PFLT_INSTANCE, isdirectory: *mut bool) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltIsDirectory(fileobject : *mut super::super::super::Foundation:: FILE_OBJECT, instance : PFLT_INSTANCE, isdirectory : *mut bool) -> windows_core:: NTSTATUS);
    unsafe { FltIsDirectory(fileobject as _, instance, isdirectory as _) }
}
#[inline]
pub unsafe fn FltIsEcpAcknowledged(filter: PFLT_FILTER, ecpcontext: *mut core::ffi::c_void) -> bool {
    windows_core::link!("fltmgr.sys" "system" fn FltIsEcpAcknowledged(filter : PFLT_FILTER, ecpcontext : *mut core::ffi::c_void) -> bool);
    unsafe { FltIsEcpAcknowledged(filter, ecpcontext as _) }
}
#[inline]
pub unsafe fn FltIsEcpFromUserMode(filter: PFLT_FILTER, ecpcontext: *mut core::ffi::c_void) -> bool {
    windows_core::link!("fltmgr.sys" "system" fn FltIsEcpFromUserMode(filter : PFLT_FILTER, ecpcontext : *mut core::ffi::c_void) -> bool);
    unsafe { FltIsEcpFromUserMode(filter, ecpcontext as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltIsFltMgrVolumeDeviceObject(deviceobject: *mut super::super::super::Foundation::DEVICE_OBJECT) -> bool {
    windows_core::link!("fltmgr.sys" "system" fn FltIsFltMgrVolumeDeviceObject(deviceobject : *mut super::super::super::Foundation:: DEVICE_OBJECT) -> bool);
    unsafe { FltIsFltMgrVolumeDeviceObject(deviceobject as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltIsIoCanceled(callbackdata: *mut FLT_CALLBACK_DATA) -> bool {
    windows_core::link!("fltmgr.sys" "system" fn FltIsIoCanceled(callbackdata : *mut FLT_CALLBACK_DATA) -> bool);
    unsafe { FltIsIoCanceled(callbackdata as _) }
}
#[inline]
pub unsafe fn FltIsIoRedirectionAllowed(sourceinstance: PFLT_INSTANCE, targetinstance: PFLT_INSTANCE, redirectionallowed: *mut bool) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltIsIoRedirectionAllowed(sourceinstance : PFLT_INSTANCE, targetinstance : PFLT_INSTANCE, redirectionallowed : *mut bool) -> windows_core:: NTSTATUS);
    unsafe { FltIsIoRedirectionAllowed(sourceinstance, targetinstance, redirectionallowed as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltIsIoRedirectionAllowedForOperation(data: *mut FLT_CALLBACK_DATA, targetinstance: PFLT_INSTANCE, redirectionallowedthisio: *mut bool, redirectionallowedallio: *mut bool) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltIsIoRedirectionAllowedForOperation(data : *mut FLT_CALLBACK_DATA, targetinstance : PFLT_INSTANCE, redirectionallowedthisio : *mut bool, redirectionallowedallio : *mut bool) -> windows_core:: NTSTATUS);
    unsafe { FltIsIoRedirectionAllowedForOperation(data as _, targetinstance, redirectionallowedthisio as _, redirectionallowedallio as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltIsOperationSynchronous(callbackdata: *mut FLT_CALLBACK_DATA) -> bool {
    windows_core::link!("fltmgr.sys" "system" fn FltIsOperationSynchronous(callbackdata : *mut FLT_CALLBACK_DATA) -> bool);
    unsafe { FltIsOperationSynchronous(callbackdata as _) }
}
#[inline]
pub unsafe fn FltIsVolumeSnapshot(fltobject: *mut core::ffi::c_void, issnapshotvolume: *mut bool) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltIsVolumeSnapshot(fltobject : *mut core::ffi::c_void, issnapshotvolume : *mut bool) -> windows_core:: NTSTATUS);
    unsafe { FltIsVolumeSnapshot(fltobject as _, issnapshotvolume as _) }
}
#[inline]
pub unsafe fn FltIsVolumeWritable(fltobject: *mut core::ffi::c_void, iswritable: *mut bool) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltIsVolumeWritable(fltobject : *mut core::ffi::c_void, iswritable : *mut bool) -> windows_core:: NTSTATUS);
    unsafe { FltIsVolumeWritable(fltobject as _, iswritable as _) }
}
#[inline]
pub unsafe fn FltLoadFilter(filtername: *mut super::super::super::super::Win32::Foundation::UNICODE_STRING) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltLoadFilter(filtername : *mut super::super::super::super::Win32::Foundation:: UNICODE_STRING) -> windows_core:: NTSTATUS);
    unsafe { FltLoadFilter(filtername as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltLockUserBuffer(callbackdata: *const FLT_CALLBACK_DATA) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltLockUserBuffer(callbackdata : *const FLT_CALLBACK_DATA) -> windows_core:: NTSTATUS);
    unsafe { FltLockUserBuffer(callbackdata) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltNotifyFilterChangeDirectory(notifysync: super::super::super::Foundation::PNOTIFY_SYNC, notifylist: *mut super::super::super::super::Win32::System::Kernel::LIST_ENTRY, fscontext: *mut core::ffi::c_void, fulldirectoryname: *mut super::super::super::super::Win32::System::Kernel::STRING, watchtree: bool, ignorebuffer: bool, completionfilter: u32, notifycallbackdata: *mut FLT_CALLBACK_DATA, traversecallback: super::PCHECK_FOR_TRAVERSE_ACCESS, subjectcontext: *mut super::super::super::Foundation::SECURITY_SUBJECT_CONTEXT, filtercallback: super::PFILTER_REPORT_CHANGE) {
    windows_core::link!("fltmgr.sys" "system" fn FltNotifyFilterChangeDirectory(notifysync : super::super::super::Foundation:: PNOTIFY_SYNC, notifylist : *mut super::super::super::super::Win32::System::Kernel:: LIST_ENTRY, fscontext : *mut core::ffi::c_void, fulldirectoryname : *mut super::super::super::super::Win32::System::Kernel:: STRING, watchtree : bool, ignorebuffer : bool, completionfilter : u32, notifycallbackdata : *mut FLT_CALLBACK_DATA, traversecallback : super:: PCHECK_FOR_TRAVERSE_ACCESS, subjectcontext : *mut super::super::super::Foundation:: SECURITY_SUBJECT_CONTEXT, filtercallback : super:: PFILTER_REPORT_CHANGE));
    unsafe { FltNotifyFilterChangeDirectory(notifysync, notifylist as _, fscontext as _, fulldirectoryname as _, watchtree, ignorebuffer, completionfilter, notifycallbackdata as _, traversecallback, subjectcontext as _, filtercallback) }
}
#[inline]
pub unsafe fn FltObjectDereference(fltobject: *mut core::ffi::c_void) {
    windows_core::link!("fltmgr.sys" "system" fn FltObjectDereference(fltobject : *mut core::ffi::c_void));
    unsafe { FltObjectDereference(fltobject as _) }
}
#[inline]
pub unsafe fn FltObjectReference(fltobject: *mut core::ffi::c_void) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltObjectReference(fltobject : *mut core::ffi::c_void) -> windows_core:: NTSTATUS);
    unsafe { FltObjectReference(fltobject as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltOpenVolume(instance: PFLT_INSTANCE, volumehandle: *mut super::super::super::super::Win32::Foundation::HANDLE, volumefileobject: *mut *mut super::super::super::Foundation::FILE_OBJECT) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltOpenVolume(instance : PFLT_INSTANCE, volumehandle : *mut super::super::super::super::Win32::Foundation:: HANDLE, volumefileobject : *mut *mut super::super::super::Foundation:: FILE_OBJECT) -> windows_core:: NTSTATUS);
    unsafe { FltOpenVolume(instance, volumehandle as _, volumefileobject as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltOplockBreakH(oplock: *mut *mut core::ffi::c_void, callbackdata: *mut FLT_CALLBACK_DATA, flags: u32, context: *mut core::ffi::c_void, waitcompletionroutine: PFLTOPLOCK_WAIT_COMPLETE_ROUTINE, prepostcallbackdataroutine: PFLTOPLOCK_PREPOST_CALLBACKDATA_ROUTINE) -> FLT_PREOP_CALLBACK_STATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltOplockBreakH(oplock : *mut *mut core::ffi::c_void, callbackdata : *mut FLT_CALLBACK_DATA, flags : u32, context : *mut core::ffi::c_void, waitcompletionroutine : PFLTOPLOCK_WAIT_COMPLETE_ROUTINE, prepostcallbackdataroutine : PFLTOPLOCK_PREPOST_CALLBACKDATA_ROUTINE) -> FLT_PREOP_CALLBACK_STATUS);
    unsafe { FltOplockBreakH(oplock as _, callbackdata as _, flags, context as _, waitcompletionroutine, prepostcallbackdataroutine) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltOplockBreakToNone(oplock: *const *const core::ffi::c_void, callbackdata: *const FLT_CALLBACK_DATA, context: *const core::ffi::c_void, waitcompletionroutine: PFLTOPLOCK_WAIT_COMPLETE_ROUTINE, prepostcallbackdataroutine: PFLTOPLOCK_PREPOST_CALLBACKDATA_ROUTINE) -> FLT_PREOP_CALLBACK_STATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltOplockBreakToNone(oplock : *const *const core::ffi::c_void, callbackdata : *const FLT_CALLBACK_DATA, context : *const core::ffi::c_void, waitcompletionroutine : PFLTOPLOCK_WAIT_COMPLETE_ROUTINE, prepostcallbackdataroutine : PFLTOPLOCK_PREPOST_CALLBACKDATA_ROUTINE) -> FLT_PREOP_CALLBACK_STATUS);
    unsafe { FltOplockBreakToNone(oplock, callbackdata, context, waitcompletionroutine, prepostcallbackdataroutine) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltOplockBreakToNoneEx(oplock: *mut *mut core::ffi::c_void, callbackdata: *mut FLT_CALLBACK_DATA, flags: u32, context: *mut core::ffi::c_void, waitcompletionroutine: PFLTOPLOCK_WAIT_COMPLETE_ROUTINE, prepostcallbackdataroutine: PFLTOPLOCK_PREPOST_CALLBACKDATA_ROUTINE) -> FLT_PREOP_CALLBACK_STATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltOplockBreakToNoneEx(oplock : *mut *mut core::ffi::c_void, callbackdata : *mut FLT_CALLBACK_DATA, flags : u32, context : *mut core::ffi::c_void, waitcompletionroutine : PFLTOPLOCK_WAIT_COMPLETE_ROUTINE, prepostcallbackdataroutine : PFLTOPLOCK_PREPOST_CALLBACKDATA_ROUTINE) -> FLT_PREOP_CALLBACK_STATUS);
    unsafe { FltOplockBreakToNoneEx(oplock as _, callbackdata as _, flags, context as _, waitcompletionroutine, prepostcallbackdataroutine) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltOplockFsctrl(oplock: *const *const core::ffi::c_void, callbackdata: *const FLT_CALLBACK_DATA, opencount: u32) -> FLT_PREOP_CALLBACK_STATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltOplockFsctrl(oplock : *const *const core::ffi::c_void, callbackdata : *const FLT_CALLBACK_DATA, opencount : u32) -> FLT_PREOP_CALLBACK_STATUS);
    unsafe { FltOplockFsctrl(oplock, callbackdata, opencount) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltOplockFsctrlEx(oplock: *mut *mut core::ffi::c_void, callbackdata: *mut FLT_CALLBACK_DATA, opencount: u32, flags: u32) -> FLT_PREOP_CALLBACK_STATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltOplockFsctrlEx(oplock : *mut *mut core::ffi::c_void, callbackdata : *mut FLT_CALLBACK_DATA, opencount : u32, flags : u32) -> FLT_PREOP_CALLBACK_STATUS);
    unsafe { FltOplockFsctrlEx(oplock as _, callbackdata as _, opencount, flags) }
}
#[inline]
pub unsafe fn FltOplockIsFastIoPossible(oplock: *mut *mut core::ffi::c_void) -> bool {
    windows_core::link!("fltmgr.sys" "system" fn FltOplockIsFastIoPossible(oplock : *mut *mut core::ffi::c_void) -> bool);
    unsafe { FltOplockIsFastIoPossible(oplock as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltOplockIsSharedRequest(callbackdata: *mut FLT_CALLBACK_DATA) -> bool {
    windows_core::link!("fltmgr.sys" "system" fn FltOplockIsSharedRequest(callbackdata : *mut FLT_CALLBACK_DATA) -> bool);
    unsafe { FltOplockIsSharedRequest(callbackdata as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltOplockKeysEqual(fo1: *const super::super::super::Foundation::FILE_OBJECT, fo2: *const super::super::super::Foundation::FILE_OBJECT) -> bool {
    windows_core::link!("fltmgr.sys" "system" fn FltOplockKeysEqual(fo1 : *const super::super::super::Foundation:: FILE_OBJECT, fo2 : *const super::super::super::Foundation:: FILE_OBJECT) -> bool);
    unsafe { FltOplockKeysEqual(fo1, fo2) }
}
#[inline]
pub unsafe fn FltParseFileName(filename: *mut super::super::super::super::Win32::Foundation::UNICODE_STRING, extension: *mut super::super::super::super::Win32::Foundation::UNICODE_STRING, stream: *mut super::super::super::super::Win32::Foundation::UNICODE_STRING, finalcomponent: *mut super::super::super::super::Win32::Foundation::UNICODE_STRING) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltParseFileName(filename : *mut super::super::super::super::Win32::Foundation:: UNICODE_STRING, extension : *mut super::super::super::super::Win32::Foundation:: UNICODE_STRING, stream : *mut super::super::super::super::Win32::Foundation:: UNICODE_STRING, finalcomponent : *mut super::super::super::super::Win32::Foundation:: UNICODE_STRING) -> windows_core:: NTSTATUS);
    unsafe { FltParseFileName(filename as _, extension as _, stream as _, finalcomponent as _) }
}
#[inline]
pub unsafe fn FltParseFileNameInformation(filenameinformation: *mut FLT_FILE_NAME_INFORMATION) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltParseFileNameInformation(filenameinformation : *mut FLT_FILE_NAME_INFORMATION) -> windows_core:: NTSTATUS);
    unsafe { FltParseFileNameInformation(filenameinformation as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltPerformAsynchronousIo(callbackdata: *mut FLT_CALLBACK_DATA, callbackroutine: PFLT_COMPLETED_ASYNC_IO_CALLBACK, callbackcontext: *mut core::ffi::c_void) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltPerformAsynchronousIo(callbackdata : *mut FLT_CALLBACK_DATA, callbackroutine : PFLT_COMPLETED_ASYNC_IO_CALLBACK, callbackcontext : *mut core::ffi::c_void) -> windows_core:: NTSTATUS);
    unsafe { FltPerformAsynchronousIo(callbackdata as _, callbackroutine, callbackcontext as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltPerformSynchronousIo(callbackdata: *mut FLT_CALLBACK_DATA) {
    windows_core::link!("fltmgr.sys" "system" fn FltPerformSynchronousIo(callbackdata : *mut FLT_CALLBACK_DATA));
    unsafe { FltPerformSynchronousIo(callbackdata as _) }
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FltPrePrepareComplete(instance: PFLT_INSTANCE, transaction: *const super::super::super::Foundation::KTRANSACTION, transactioncontext: PFLT_CONTEXT) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltPrePrepareComplete(instance : PFLT_INSTANCE, transaction : *const super::super::super::Foundation:: KTRANSACTION, transactioncontext : PFLT_CONTEXT) -> windows_core:: NTSTATUS);
    unsafe { FltPrePrepareComplete(instance, transaction, transactioncontext) }
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FltPrepareComplete(instance: PFLT_INSTANCE, transaction: *mut super::super::super::Foundation::KTRANSACTION, transactioncontext: PFLT_CONTEXT) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltPrepareComplete(instance : PFLT_INSTANCE, transaction : *mut super::super::super::Foundation:: KTRANSACTION, transactioncontext : PFLT_CONTEXT) -> windows_core:: NTSTATUS);
    unsafe { FltPrepareComplete(instance, transaction as _, transactioncontext) }
}
#[inline]
pub unsafe fn FltPrepareToReuseEcp(filter: PFLT_FILTER, ecpcontext: *mut core::ffi::c_void) {
    windows_core::link!("fltmgr.sys" "system" fn FltPrepareToReuseEcp(filter : PFLT_FILTER, ecpcontext : *mut core::ffi::c_void));
    unsafe { FltPrepareToReuseEcp(filter, ecpcontext as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltProcessFileLock(filelock: *mut super::FILE_LOCK, callbackdata: *mut FLT_CALLBACK_DATA, context: *mut core::ffi::c_void) -> FLT_PREOP_CALLBACK_STATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltProcessFileLock(filelock : *mut super:: FILE_LOCK, callbackdata : *mut FLT_CALLBACK_DATA, context : *mut core::ffi::c_void) -> FLT_PREOP_CALLBACK_STATUS);
    unsafe { FltProcessFileLock(filelock as _, callbackdata as _, context as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltPropagateActivityIdToThread(callbackdata: *mut FLT_CALLBACK_DATA, propagateid: *mut windows_core::GUID, originalid: *mut *mut windows_core::GUID) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltPropagateActivityIdToThread(callbackdata : *mut FLT_CALLBACK_DATA, propagateid : *mut windows_core::GUID, originalid : *mut *mut windows_core::GUID) -> windows_core:: NTSTATUS);
    unsafe { FltPropagateActivityIdToThread(callbackdata as _, propagateid as _, originalid as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltPropagateIrpExtension(sourcedata: *mut FLT_CALLBACK_DATA, targetdata: *mut FLT_CALLBACK_DATA, flags: u32) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltPropagateIrpExtension(sourcedata : *mut FLT_CALLBACK_DATA, targetdata : *mut FLT_CALLBACK_DATA, flags : u32) -> windows_core:: NTSTATUS);
    unsafe { FltPropagateIrpExtension(sourcedata as _, targetdata as _, flags) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltPurgeFileNameInformationCache(instance: PFLT_INSTANCE, fileobject: *mut super::super::super::Foundation::FILE_OBJECT) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltPurgeFileNameInformationCache(instance : PFLT_INSTANCE, fileobject : *mut super::super::super::Foundation:: FILE_OBJECT) -> windows_core:: NTSTATUS);
    unsafe { FltPurgeFileNameInformationCache(instance, fileobject as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltQueryDirectoryFile(instance: PFLT_INSTANCE, fileobject: *mut super::super::super::Foundation::FILE_OBJECT, fileinformation: *mut core::ffi::c_void, length: u32, fileinformationclass: super::FILE_INFORMATION_CLASS, returnsingleentry: bool, filename: *mut super::super::super::super::Win32::Foundation::UNICODE_STRING, restartscan: bool, lengthreturned: *mut u32) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltQueryDirectoryFile(instance : PFLT_INSTANCE, fileobject : *mut super::super::super::Foundation:: FILE_OBJECT, fileinformation : *mut core::ffi::c_void, length : u32, fileinformationclass : super:: FILE_INFORMATION_CLASS, returnsingleentry : bool, filename : *mut super::super::super::super::Win32::Foundation:: UNICODE_STRING, restartscan : bool, lengthreturned : *mut u32) -> windows_core:: NTSTATUS);
    unsafe { FltQueryDirectoryFile(instance, fileobject as _, fileinformation as _, length, fileinformationclass, returnsingleentry, filename as _, restartscan, lengthreturned as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltQueryDirectoryFileEx(instance: PFLT_INSTANCE, fileobject: *const super::super::super::Foundation::FILE_OBJECT, fileinformation: *mut core::ffi::c_void, length: u32, fileinformationclass: super::FILE_INFORMATION_CLASS, queryflags: u32, filename: *const super::super::super::super::Win32::Foundation::UNICODE_STRING, lengthreturned: *mut u32) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltQueryDirectoryFileEx(instance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, fileinformation : *mut core::ffi::c_void, length : u32, fileinformationclass : super:: FILE_INFORMATION_CLASS, queryflags : u32, filename : *const super::super::super::super::Win32::Foundation:: UNICODE_STRING, lengthreturned : *mut u32) -> windows_core:: NTSTATUS);
    unsafe { FltQueryDirectoryFileEx(instance, fileobject, fileinformation as _, length, fileinformationclass, queryflags, filename, lengthreturned as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltQueryEaFile(instance: PFLT_INSTANCE, fileobject: *mut super::super::super::Foundation::FILE_OBJECT, returnedeadata: *mut core::ffi::c_void, length: u32, returnsingleentry: bool, ealist: *mut core::ffi::c_void, ealistlength: u32, eaindex: *mut u32, restartscan: bool, lengthreturned: *mut u32) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltQueryEaFile(instance : PFLT_INSTANCE, fileobject : *mut super::super::super::Foundation:: FILE_OBJECT, returnedeadata : *mut core::ffi::c_void, length : u32, returnsingleentry : bool, ealist : *mut core::ffi::c_void, ealistlength : u32, eaindex : *mut u32, restartscan : bool, lengthreturned : *mut u32) -> windows_core:: NTSTATUS);
    unsafe { FltQueryEaFile(instance, fileobject as _, returnedeadata as _, length, returnsingleentry, ealist as _, ealistlength, eaindex as _, restartscan, lengthreturned as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn FltQueryInformationByName(filter: PFLT_FILTER, instance: PFLT_INSTANCE, objectattributes: *mut super::super::super::Foundation::OBJECT_ATTRIBUTES, iostatusblock: *mut super::super::super::super::Win32::System::IO::IO_STATUS_BLOCK, fileinformation: *mut core::ffi::c_void, length: u32, fileinformationclass: super::FILE_INFORMATION_CLASS, drivercontext: *mut super::super::super::System::SystemServices::IO_DRIVER_CREATE_CONTEXT) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltQueryInformationByName(filter : PFLT_FILTER, instance : PFLT_INSTANCE, objectattributes : *mut super::super::super::Foundation:: OBJECT_ATTRIBUTES, iostatusblock : *mut super::super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, fileinformation : *mut core::ffi::c_void, length : u32, fileinformationclass : super:: FILE_INFORMATION_CLASS, drivercontext : *mut super::super::super::System::SystemServices:: IO_DRIVER_CREATE_CONTEXT) -> windows_core:: NTSTATUS);
    unsafe { FltQueryInformationByName(filter, instance, objectattributes as _, iostatusblock as _, fileinformation as _, length, fileinformationclass, drivercontext as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltQueryInformationFile(instance: PFLT_INSTANCE, fileobject: *mut super::super::super::Foundation::FILE_OBJECT, fileinformation: *mut core::ffi::c_void, length: u32, fileinformationclass: super::FILE_INFORMATION_CLASS, lengthreturned: *mut u32) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltQueryInformationFile(instance : PFLT_INSTANCE, fileobject : *mut super::super::super::Foundation:: FILE_OBJECT, fileinformation : *mut core::ffi::c_void, length : u32, fileinformationclass : super:: FILE_INFORMATION_CLASS, lengthreturned : *mut u32) -> windows_core:: NTSTATUS);
    unsafe { FltQueryInformationFile(instance, fileobject as _, fileinformation as _, length, fileinformationclass, lengthreturned as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltQueryQuotaInformationFile(instance: PFLT_INSTANCE, fileobject: *mut super::super::super::Foundation::FILE_OBJECT, iostatusblock: *mut super::super::super::super::Win32::System::IO::IO_STATUS_BLOCK, buffer: *mut core::ffi::c_void, length: u32, returnsingleentry: bool, sidlist: *mut core::ffi::c_void, sidlistlength: u32, startsid: *mut u32, restartscan: bool, lengthreturned: *mut u32) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltQueryQuotaInformationFile(instance : PFLT_INSTANCE, fileobject : *mut super::super::super::Foundation:: FILE_OBJECT, iostatusblock : *mut super::super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, buffer : *mut core::ffi::c_void, length : u32, returnsingleentry : bool, sidlist : *mut core::ffi::c_void, sidlistlength : u32, startsid : *mut u32, restartscan : bool, lengthreturned : *mut u32) -> windows_core:: NTSTATUS);
    unsafe { FltQueryQuotaInformationFile(instance, fileobject as _, iostatusblock as _, buffer as _, length, returnsingleentry, sidlist as _, sidlistlength, startsid as _, restartscan, lengthreturned as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltQuerySecurityObject(instance: PFLT_INSTANCE, fileobject: *mut super::super::super::Foundation::FILE_OBJECT, securityinformation: u32, securitydescriptor: super::super::super::super::Win32::Security::PSECURITY_DESCRIPTOR, length: u32, lengthneeded: *mut u32) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltQuerySecurityObject(instance : PFLT_INSTANCE, fileobject : *mut super::super::super::Foundation:: FILE_OBJECT, securityinformation : u32, securitydescriptor : super::super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, length : u32, lengthneeded : *mut u32) -> windows_core:: NTSTATUS);
    unsafe { FltQuerySecurityObject(instance, fileobject as _, securityinformation, securitydescriptor, length, lengthneeded as _) }
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn FltQueryVolumeInformation(instance: PFLT_INSTANCE, iosb: *mut super::super::super::super::Win32::System::IO::IO_STATUS_BLOCK, fsinformation: *mut core::ffi::c_void, length: u32, fsinformationclass: super::FS_INFORMATION_CLASS) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltQueryVolumeInformation(instance : PFLT_INSTANCE, iosb : *mut super::super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, fsinformation : *mut core::ffi::c_void, length : u32, fsinformationclass : super:: FS_INFORMATION_CLASS) -> windows_core:: NTSTATUS);
    unsafe { FltQueryVolumeInformation(instance, iosb as _, fsinformation as _, length, fsinformationclass) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltQueryVolumeInformationFile(instance: PFLT_INSTANCE, fileobject: *mut super::super::super::Foundation::FILE_OBJECT, fsinformation: *mut core::ffi::c_void, length: u32, fsinformationclass: super::FS_INFORMATION_CLASS, lengthreturned: *mut u32) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltQueryVolumeInformationFile(instance : PFLT_INSTANCE, fileobject : *mut super::super::super::Foundation:: FILE_OBJECT, fsinformation : *mut core::ffi::c_void, length : u32, fsinformationclass : super:: FS_INFORMATION_CLASS, lengthreturned : *mut u32) -> windows_core:: NTSTATUS);
    unsafe { FltQueryVolumeInformationFile(instance, fileobject as _, fsinformation as _, length, fsinformationclass, lengthreturned as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltQueueDeferredIoWorkItem(fltworkitem: PFLT_DEFERRED_IO_WORKITEM, data: *const FLT_CALLBACK_DATA, workerroutine: PFLT_DEFERRED_IO_WORKITEM_ROUTINE, queuetype: super::super::super::System::SystemServices::WORK_QUEUE_TYPE, context: *const core::ffi::c_void) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltQueueDeferredIoWorkItem(fltworkitem : PFLT_DEFERRED_IO_WORKITEM, data : *const FLT_CALLBACK_DATA, workerroutine : PFLT_DEFERRED_IO_WORKITEM_ROUTINE, queuetype : super::super::super::System::SystemServices:: WORK_QUEUE_TYPE, context : *const core::ffi::c_void) -> windows_core:: NTSTATUS);
    unsafe { FltQueueDeferredIoWorkItem(fltworkitem, data, workerroutine, queuetype, context) }
}
#[cfg(feature = "Wdk_System_SystemServices")]
#[inline]
pub unsafe fn FltQueueGenericWorkItem(fltworkitem: PFLT_GENERIC_WORKITEM, fltobject: *mut core::ffi::c_void, workerroutine: PFLT_GENERIC_WORKITEM_ROUTINE, queuetype: super::super::super::System::SystemServices::WORK_QUEUE_TYPE, context: *mut core::ffi::c_void) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltQueueGenericWorkItem(fltworkitem : PFLT_GENERIC_WORKITEM, fltobject : *mut core::ffi::c_void, workerroutine : PFLT_GENERIC_WORKITEM_ROUTINE, queuetype : super::super::super::System::SystemServices:: WORK_QUEUE_TYPE, context : *mut core::ffi::c_void) -> windows_core:: NTSTATUS);
    unsafe { FltQueueGenericWorkItem(fltworkitem, fltobject as _, workerroutine, queuetype, context as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltReadFile(initiatinginstance: PFLT_INSTANCE, fileobject: *mut super::super::super::Foundation::FILE_OBJECT, byteoffset: *mut i64, length: u32, buffer: *mut core::ffi::c_void, flags: u32, bytesread: *mut u32, callbackroutine: PFLT_COMPLETED_ASYNC_IO_CALLBACK, callbackcontext: *mut core::ffi::c_void) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltReadFile(initiatinginstance : PFLT_INSTANCE, fileobject : *mut super::super::super::Foundation:: FILE_OBJECT, byteoffset : *mut i64, length : u32, buffer : *mut core::ffi::c_void, flags : u32, bytesread : *mut u32, callbackroutine : PFLT_COMPLETED_ASYNC_IO_CALLBACK, callbackcontext : *mut core::ffi::c_void) -> windows_core:: NTSTATUS);
    unsafe { FltReadFile(initiatinginstance, fileobject as _, byteoffset as _, length, buffer as _, flags, bytesread as _, callbackroutine, callbackcontext as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltReadFileEx(initiatinginstance: PFLT_INSTANCE, fileobject: *const super::super::super::Foundation::FILE_OBJECT, byteoffset: *const i64, length: u32, buffer: *mut core::ffi::c_void, flags: u32, bytesread: *mut u32, callbackroutine: PFLT_COMPLETED_ASYNC_IO_CALLBACK, callbackcontext: *const core::ffi::c_void, key: *const u32, mdl: *const super::super::super::Foundation::MDL) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltReadFileEx(initiatinginstance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, byteoffset : *const i64, length : u32, buffer : *mut core::ffi::c_void, flags : u32, bytesread : *mut u32, callbackroutine : PFLT_COMPLETED_ASYNC_IO_CALLBACK, callbackcontext : *const core::ffi::c_void, key : *const u32, mdl : *const super::super::super::Foundation:: MDL) -> windows_core:: NTSTATUS);
    unsafe { FltReadFileEx(initiatinginstance, fileobject, byteoffset, length, buffer as _, flags, bytesread as _, callbackroutine, callbackcontext, key, mdl) }
}
#[inline]
pub unsafe fn FltReferenceContext(context: PFLT_CONTEXT) {
    windows_core::link!("fltmgr.sys" "system" fn FltReferenceContext(context : PFLT_CONTEXT));
    unsafe { FltReferenceContext(context) }
}
#[inline]
pub unsafe fn FltReferenceFileNameInformation(filenameinformation: *mut FLT_FILE_NAME_INFORMATION) {
    windows_core::link!("fltmgr.sys" "system" fn FltReferenceFileNameInformation(filenameinformation : *mut FLT_FILE_NAME_INFORMATION));
    unsafe { FltReferenceFileNameInformation(filenameinformation as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_Storage_InstallableFileSystems", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltRegisterFilter(driver: *const super::super::super::Foundation::DRIVER_OBJECT, registration: *const FLT_REGISTRATION, retfilter: *mut PFLT_FILTER) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltRegisterFilter(driver : *const super::super::super::Foundation:: DRIVER_OBJECT, registration : *const FLT_REGISTRATION, retfilter : *mut PFLT_FILTER) -> windows_core:: NTSTATUS);
    unsafe { FltRegisterFilter(driver, registration, retfilter as _) }
}
#[inline]
pub unsafe fn FltRegisterForDataScan(instance: PFLT_INSTANCE) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltRegisterForDataScan(instance : PFLT_INSTANCE) -> windows_core:: NTSTATUS);
    unsafe { FltRegisterForDataScan(instance) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltReissueSynchronousIo(initiatinginstance: PFLT_INSTANCE, callbackdata: *mut FLT_CALLBACK_DATA) {
    windows_core::link!("fltmgr.sys" "system" fn FltReissueSynchronousIo(initiatinginstance : PFLT_INSTANCE, callbackdata : *mut FLT_CALLBACK_DATA));
    unsafe { FltReissueSynchronousIo(initiatinginstance, callbackdata as _) }
}
#[inline]
pub unsafe fn FltReleaseContext(context: PFLT_CONTEXT) {
    windows_core::link!("fltmgr.sys" "system" fn FltReleaseContext(context : PFLT_CONTEXT));
    unsafe { FltReleaseContext(context) }
}
#[inline]
pub unsafe fn FltReleaseContexts(contexts: *const FLT_RELATED_CONTEXTS) {
    windows_core::link!("fltmgr.sys" "system" fn FltReleaseContexts(contexts : *const FLT_RELATED_CONTEXTS));
    unsafe { FltReleaseContexts(contexts) }
}
#[inline]
pub unsafe fn FltReleaseContextsEx(contextssize: usize, contexts: *mut FLT_RELATED_CONTEXTS_EX) {
    windows_core::link!("fltmgr.sys" "system" fn FltReleaseContextsEx(contextssize : usize, contexts : *mut FLT_RELATED_CONTEXTS_EX));
    unsafe { FltReleaseContextsEx(contextssize, contexts as _) }
}
#[inline]
pub unsafe fn FltReleaseFileNameInformation(filenameinformation: *mut FLT_FILE_NAME_INFORMATION) {
    windows_core::link!("fltmgr.sys" "system" fn FltReleaseFileNameInformation(filenameinformation : *mut FLT_FILE_NAME_INFORMATION));
    unsafe { FltReleaseFileNameInformation(filenameinformation as _) }
}
#[inline]
pub unsafe fn FltReleasePushLock() -> usize {
    windows_core::link!("fltmgr.sys" "system" fn FltReleasePushLock(pushlock : *mut usize));
    unsafe {
        let mut result__ = core::mem::zeroed();
        FltReleasePushLock(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn FltReleasePushLockEx(pushlock: *mut usize, flags: u32) {
    windows_core::link!("fltmgr.sys" "system" fn FltReleasePushLockEx(pushlock : *mut usize, flags : u32));
    unsafe { FltReleasePushLockEx(pushlock as _, flags) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[inline]
pub unsafe fn FltReleaseResource(resource: *mut super::super::super::Foundation::ERESOURCE) {
    windows_core::link!("fltmgr.sys" "system" fn FltReleaseResource(resource : *mut super::super::super::Foundation:: ERESOURCE));
    unsafe { FltReleaseResource(resource as _) }
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FltRemoveExtraCreateParameter(filter: PFLT_FILTER, ecplist: *mut super::super::super::Foundation::ECP_LIST, ecptype: *const windows_core::GUID, ecpcontext: *mut *mut core::ffi::c_void, ecpcontextsize: *mut u32) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltRemoveExtraCreateParameter(filter : PFLT_FILTER, ecplist : *mut super::super::super::Foundation:: ECP_LIST, ecptype : *const windows_core::GUID, ecpcontext : *mut *mut core::ffi::c_void, ecpcontextsize : *mut u32) -> windows_core:: NTSTATUS);
    unsafe { FltRemoveExtraCreateParameter(filter, ecplist as _, ecptype, ecpcontext as _, ecpcontextsize as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltRemoveOpenReparseEntry(filter: PFLT_FILTER, data: *mut FLT_CALLBACK_DATA, openreparseentry: *mut super::OPEN_REPARSE_LIST_ENTRY) {
    windows_core::link!("fltmgr.sys" "system" fn FltRemoveOpenReparseEntry(filter : PFLT_FILTER, data : *mut FLT_CALLBACK_DATA, openreparseentry : *mut super:: OPEN_REPARSE_LIST_ENTRY));
    unsafe { FltRemoveOpenReparseEntry(filter, data as _, openreparseentry as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltRequestFileInfoOnCreateCompletion(filter: PFLT_FILTER, data: *mut FLT_CALLBACK_DATA, infoclassflags: u32) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltRequestFileInfoOnCreateCompletion(filter : PFLT_FILTER, data : *mut FLT_CALLBACK_DATA, infoclassflags : u32) -> windows_core:: NTSTATUS);
    unsafe { FltRequestFileInfoOnCreateCompletion(filter, data as _, infoclassflags) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltRequestOperationStatusCallback(data: *mut FLT_CALLBACK_DATA, callbackroutine: PFLT_GET_OPERATION_STATUS_CALLBACK, requestercontext: *mut core::ffi::c_void) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltRequestOperationStatusCallback(data : *mut FLT_CALLBACK_DATA, callbackroutine : PFLT_GET_OPERATION_STATUS_CALLBACK, requestercontext : *mut core::ffi::c_void) -> windows_core:: NTSTATUS);
    unsafe { FltRequestOperationStatusCallback(data as _, callbackroutine, requestercontext as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltRetainSwappedBufferMdlAddress(callbackdata: *mut FLT_CALLBACK_DATA) {
    windows_core::link!("fltmgr.sys" "system" fn FltRetainSwappedBufferMdlAddress(callbackdata : *mut FLT_CALLBACK_DATA));
    unsafe { FltRetainSwappedBufferMdlAddress(callbackdata as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltRetrieveFileInfoOnCreateCompletion(filter: PFLT_FILTER, data: *mut FLT_CALLBACK_DATA, infoclass: u32, size: *mut u32) -> *mut core::ffi::c_void {
    windows_core::link!("fltmgr.sys" "system" fn FltRetrieveFileInfoOnCreateCompletion(filter : PFLT_FILTER, data : *mut FLT_CALLBACK_DATA, infoclass : u32, size : *mut u32) -> *mut core::ffi::c_void);
    unsafe { FltRetrieveFileInfoOnCreateCompletion(filter, data as _, infoclass, size as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltRetrieveFileInfoOnCreateCompletionEx(filter: PFLT_FILTER, data: *mut FLT_CALLBACK_DATA, infoclass: u32, retinfosize: *mut u32, retinfobuffer: *mut *mut core::ffi::c_void) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltRetrieveFileInfoOnCreateCompletionEx(filter : PFLT_FILTER, data : *mut FLT_CALLBACK_DATA, infoclass : u32, retinfosize : *mut u32, retinfobuffer : *mut *mut core::ffi::c_void) -> windows_core:: NTSTATUS);
    unsafe { FltRetrieveFileInfoOnCreateCompletionEx(filter, data as _, infoclass, retinfosize as _, retinfobuffer as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltRetrieveIoPriorityInfo(data: *mut FLT_CALLBACK_DATA, fileobject: *mut super::super::super::Foundation::FILE_OBJECT, thread: super::super::super::Foundation::PETHREAD, priorityinfo: *mut super::IO_PRIORITY_INFO) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltRetrieveIoPriorityInfo(data : *mut FLT_CALLBACK_DATA, fileobject : *mut super::super::super::Foundation:: FILE_OBJECT, thread : super::super::super::Foundation:: PETHREAD, priorityinfo : *mut super:: IO_PRIORITY_INFO) -> windows_core:: NTSTATUS);
    unsafe { FltRetrieveIoPriorityInfo(data as _, fileobject as _, thread, priorityinfo as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltReuseCallbackData(callbackdata: *mut FLT_CALLBACK_DATA) {
    windows_core::link!("fltmgr.sys" "system" fn FltReuseCallbackData(callbackdata : *mut FLT_CALLBACK_DATA));
    unsafe { FltReuseCallbackData(callbackdata as _) }
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FltRollbackComplete(instance: PFLT_INSTANCE, transaction: *const super::super::super::Foundation::KTRANSACTION, transactioncontext: PFLT_CONTEXT) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltRollbackComplete(instance : PFLT_INSTANCE, transaction : *const super::super::super::Foundation:: KTRANSACTION, transactioncontext : PFLT_CONTEXT) -> windows_core:: NTSTATUS);
    unsafe { FltRollbackComplete(instance, transaction, transactioncontext) }
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FltRollbackEnlistment(instance: PFLT_INSTANCE, transaction: *mut super::super::super::Foundation::KTRANSACTION, transactioncontext: PFLT_CONTEXT) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltRollbackEnlistment(instance : PFLT_INSTANCE, transaction : *mut super::super::super::Foundation:: KTRANSACTION, transactioncontext : PFLT_CONTEXT) -> windows_core:: NTSTATUS);
    unsafe { FltRollbackEnlistment(instance, transaction as _, transactioncontext) }
}
#[inline]
pub unsafe fn FltSendMessage(filter: PFLT_FILTER, clientport: *mut PFLT_PORT, senderbuffer: *mut core::ffi::c_void, senderbufferlength: u32, replybuffer: *mut core::ffi::c_void, replylength: *mut u32, timeout: *mut i64) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltSendMessage(filter : PFLT_FILTER, clientport : *mut PFLT_PORT, senderbuffer : *mut core::ffi::c_void, senderbufferlength : u32, replybuffer : *mut core::ffi::c_void, replylength : *mut u32, timeout : *mut i64) -> windows_core:: NTSTATUS);
    unsafe { FltSendMessage(filter, clientport as _, senderbuffer as _, senderbufferlength, replybuffer as _, replylength as _, timeout as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltSetActivityIdCallbackData(callbackdata: *mut FLT_CALLBACK_DATA, guid: *const windows_core::GUID) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltSetActivityIdCallbackData(callbackdata : *mut FLT_CALLBACK_DATA, guid : *const windows_core::GUID) -> windows_core:: NTSTATUS);
    unsafe { FltSetActivityIdCallbackData(callbackdata as _, guid) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltSetCallbackDataDirty(data: *mut FLT_CALLBACK_DATA) {
    windows_core::link!("fltmgr.sys" "system" fn FltSetCallbackDataDirty(data : *mut FLT_CALLBACK_DATA));
    unsafe { FltSetCallbackDataDirty(data as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltSetCancelCompletion(callbackdata: *mut FLT_CALLBACK_DATA, canceledcallback: PFLT_COMPLETE_CANCELED_CALLBACK) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltSetCancelCompletion(callbackdata : *mut FLT_CALLBACK_DATA, canceledcallback : PFLT_COMPLETE_CANCELED_CALLBACK) -> windows_core:: NTSTATUS);
    unsafe { FltSetCancelCompletion(callbackdata as _, canceledcallback) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltSetEaFile(instance: PFLT_INSTANCE, fileobject: *mut super::super::super::Foundation::FILE_OBJECT, eabuffer: *mut core::ffi::c_void, length: u32) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltSetEaFile(instance : PFLT_INSTANCE, fileobject : *mut super::super::super::Foundation:: FILE_OBJECT, eabuffer : *mut core::ffi::c_void, length : u32) -> windows_core:: NTSTATUS);
    unsafe { FltSetEaFile(instance, fileobject as _, eabuffer as _, length) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltSetEcpListIntoCallbackData(filter: PFLT_FILTER, callbackdata: *const FLT_CALLBACK_DATA, ecplist: *const super::super::super::Foundation::ECP_LIST) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltSetEcpListIntoCallbackData(filter : PFLT_FILTER, callbackdata : *const FLT_CALLBACK_DATA, ecplist : *const super::super::super::Foundation:: ECP_LIST) -> windows_core:: NTSTATUS);
    unsafe { FltSetEcpListIntoCallbackData(filter, callbackdata, ecplist) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltSetFileContext(instance: PFLT_INSTANCE, fileobject: *const super::super::super::Foundation::FILE_OBJECT, operation: FLT_SET_CONTEXT_OPERATION, newcontext: PFLT_CONTEXT, oldcontext: *mut PFLT_CONTEXT) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltSetFileContext(instance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, operation : FLT_SET_CONTEXT_OPERATION, newcontext : PFLT_CONTEXT, oldcontext : *mut PFLT_CONTEXT) -> windows_core:: NTSTATUS);
    unsafe { FltSetFileContext(instance, fileobject, operation, newcontext, oldcontext as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltSetFsZeroingOffset(data: *mut FLT_CALLBACK_DATA, zeroingoffset: u32) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltSetFsZeroingOffset(data : *mut FLT_CALLBACK_DATA, zeroingoffset : u32) -> windows_core:: NTSTATUS);
    unsafe { FltSetFsZeroingOffset(data as _, zeroingoffset) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltSetFsZeroingOffsetRequired(data: *mut FLT_CALLBACK_DATA) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltSetFsZeroingOffsetRequired(data : *mut FLT_CALLBACK_DATA) -> windows_core:: NTSTATUS);
    unsafe { FltSetFsZeroingOffsetRequired(data as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltSetInformationFile(instance: PFLT_INSTANCE, fileobject: *const super::super::super::Foundation::FILE_OBJECT, fileinformation: *const core::ffi::c_void, length: u32, fileinformationclass: super::FILE_INFORMATION_CLASS) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltSetInformationFile(instance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, fileinformation : *const core::ffi::c_void, length : u32, fileinformationclass : super:: FILE_INFORMATION_CLASS) -> windows_core:: NTSTATUS);
    unsafe { FltSetInformationFile(instance, fileobject, fileinformation, length, fileinformationclass) }
}
#[inline]
pub unsafe fn FltSetInstanceContext(instance: PFLT_INSTANCE, operation: FLT_SET_CONTEXT_OPERATION, newcontext: PFLT_CONTEXT, oldcontext: *mut PFLT_CONTEXT) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltSetInstanceContext(instance : PFLT_INSTANCE, operation : FLT_SET_CONTEXT_OPERATION, newcontext : PFLT_CONTEXT, oldcontext : *mut PFLT_CONTEXT) -> windows_core:: NTSTATUS);
    unsafe { FltSetInstanceContext(instance, operation, newcontext, oldcontext as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltSetIoPriorityHintIntoCallbackData(data: *mut FLT_CALLBACK_DATA, priorityhint: super::super::super::Foundation::IO_PRIORITY_HINT) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltSetIoPriorityHintIntoCallbackData(data : *mut FLT_CALLBACK_DATA, priorityhint : super::super::super::Foundation:: IO_PRIORITY_HINT) -> windows_core:: NTSTATUS);
    unsafe { FltSetIoPriorityHintIntoCallbackData(data as _, priorityhint) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltSetIoPriorityHintIntoFileObject(fileobject: *const super::super::super::Foundation::FILE_OBJECT, priorityhint: super::super::super::Foundation::IO_PRIORITY_HINT) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltSetIoPriorityHintIntoFileObject(fileobject : *const super::super::super::Foundation:: FILE_OBJECT, priorityhint : super::super::super::Foundation:: IO_PRIORITY_HINT) -> windows_core:: NTSTATUS);
    unsafe { FltSetIoPriorityHintIntoFileObject(fileobject, priorityhint) }
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FltSetIoPriorityHintIntoThread(thread: super::super::super::Foundation::PETHREAD, priorityhint: super::super::super::Foundation::IO_PRIORITY_HINT) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltSetIoPriorityHintIntoThread(thread : super::super::super::Foundation:: PETHREAD, priorityhint : super::super::super::Foundation:: IO_PRIORITY_HINT) -> windows_core:: NTSTATUS);
    unsafe { FltSetIoPriorityHintIntoThread(thread, priorityhint) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltSetQuotaInformationFile(instance: PFLT_INSTANCE, fileobject: *const super::super::super::Foundation::FILE_OBJECT, buffer: *const core::ffi::c_void, length: u32) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltSetQuotaInformationFile(instance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, buffer : *const core::ffi::c_void, length : u32) -> windows_core:: NTSTATUS);
    unsafe { FltSetQuotaInformationFile(instance, fileobject, buffer, length) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltSetSecurityObject(instance: PFLT_INSTANCE, fileobject: *mut super::super::super::Foundation::FILE_OBJECT, securityinformation: u32, securitydescriptor: super::super::super::super::Win32::Security::PSECURITY_DESCRIPTOR) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltSetSecurityObject(instance : PFLT_INSTANCE, fileobject : *mut super::super::super::Foundation:: FILE_OBJECT, securityinformation : u32, securitydescriptor : super::super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> windows_core:: NTSTATUS);
    unsafe { FltSetSecurityObject(instance, fileobject as _, securityinformation, securitydescriptor) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltSetStreamContext(instance: PFLT_INSTANCE, fileobject: *const super::super::super::Foundation::FILE_OBJECT, operation: FLT_SET_CONTEXT_OPERATION, newcontext: PFLT_CONTEXT, oldcontext: *mut PFLT_CONTEXT) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltSetStreamContext(instance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, operation : FLT_SET_CONTEXT_OPERATION, newcontext : PFLT_CONTEXT, oldcontext : *mut PFLT_CONTEXT) -> windows_core:: NTSTATUS);
    unsafe { FltSetStreamContext(instance, fileobject, operation, newcontext, oldcontext as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltSetStreamHandleContext(instance: PFLT_INSTANCE, fileobject: *mut super::super::super::Foundation::FILE_OBJECT, operation: FLT_SET_CONTEXT_OPERATION, newcontext: PFLT_CONTEXT, oldcontext: *mut PFLT_CONTEXT) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltSetStreamHandleContext(instance : PFLT_INSTANCE, fileobject : *mut super::super::super::Foundation:: FILE_OBJECT, operation : FLT_SET_CONTEXT_OPERATION, newcontext : PFLT_CONTEXT, oldcontext : *mut PFLT_CONTEXT) -> windows_core:: NTSTATUS);
    unsafe { FltSetStreamHandleContext(instance, fileobject as _, operation, newcontext, oldcontext as _) }
}
#[cfg(feature = "Wdk_Foundation")]
#[inline]
pub unsafe fn FltSetTransactionContext(instance: PFLT_INSTANCE, transaction: *mut super::super::super::Foundation::KTRANSACTION, operation: FLT_SET_CONTEXT_OPERATION, newcontext: PFLT_CONTEXT, oldcontext: *mut PFLT_CONTEXT) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltSetTransactionContext(instance : PFLT_INSTANCE, transaction : *mut super::super::super::Foundation:: KTRANSACTION, operation : FLT_SET_CONTEXT_OPERATION, newcontext : PFLT_CONTEXT, oldcontext : *mut PFLT_CONTEXT) -> windows_core:: NTSTATUS);
    unsafe { FltSetTransactionContext(instance, transaction as _, operation, newcontext, oldcontext as _) }
}
#[inline]
pub unsafe fn FltSetVolumeContext(volume: PFLT_VOLUME, operation: FLT_SET_CONTEXT_OPERATION, newcontext: PFLT_CONTEXT, oldcontext: *mut PFLT_CONTEXT) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltSetVolumeContext(volume : PFLT_VOLUME, operation : FLT_SET_CONTEXT_OPERATION, newcontext : PFLT_CONTEXT, oldcontext : *mut PFLT_CONTEXT) -> windows_core:: NTSTATUS);
    unsafe { FltSetVolumeContext(volume, operation, newcontext, oldcontext as _) }
}
#[cfg(feature = "Win32_System_IO")]
#[inline]
pub unsafe fn FltSetVolumeInformation(instance: PFLT_INSTANCE, iosb: *mut super::super::super::super::Win32::System::IO::IO_STATUS_BLOCK, fsinformation: *mut core::ffi::c_void, length: u32, fsinformationclass: super::FS_INFORMATION_CLASS) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltSetVolumeInformation(instance : PFLT_INSTANCE, iosb : *mut super::super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, fsinformation : *mut core::ffi::c_void, length : u32, fsinformationclass : super:: FS_INFORMATION_CLASS) -> windows_core:: NTSTATUS);
    unsafe { FltSetVolumeInformation(instance, iosb as _, fsinformation as _, length, fsinformationclass) }
}
#[inline]
pub unsafe fn FltStartFiltering(filter: PFLT_FILTER) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltStartFiltering(filter : PFLT_FILTER) -> windows_core:: NTSTATUS);
    unsafe { FltStartFiltering(filter) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltSupportsFileContexts(fileobject: *mut super::super::super::Foundation::FILE_OBJECT) -> bool {
    windows_core::link!("fltmgr.sys" "system" fn FltSupportsFileContexts(fileobject : *mut super::super::super::Foundation:: FILE_OBJECT) -> bool);
    unsafe { FltSupportsFileContexts(fileobject as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltSupportsFileContextsEx(fileobject: *mut super::super::super::Foundation::FILE_OBJECT, instance: PFLT_INSTANCE) -> bool {
    windows_core::link!("fltmgr.sys" "system" fn FltSupportsFileContextsEx(fileobject : *mut super::super::super::Foundation:: FILE_OBJECT, instance : PFLT_INSTANCE) -> bool);
    unsafe { FltSupportsFileContextsEx(fileobject as _, instance) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltSupportsStreamContexts(fileobject: *mut super::super::super::Foundation::FILE_OBJECT) -> bool {
    windows_core::link!("fltmgr.sys" "system" fn FltSupportsStreamContexts(fileobject : *mut super::super::super::Foundation:: FILE_OBJECT) -> bool);
    unsafe { FltSupportsStreamContexts(fileobject as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltSupportsStreamHandleContexts(fileobject: *mut super::super::super::Foundation::FILE_OBJECT) -> bool {
    windows_core::link!("fltmgr.sys" "system" fn FltSupportsStreamHandleContexts(fileobject : *mut super::super::super::Foundation:: FILE_OBJECT) -> bool);
    unsafe { FltSupportsStreamHandleContexts(fileobject as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltTagFile(initiatinginstance: PFLT_INSTANCE, fileobject: *const super::super::super::Foundation::FILE_OBJECT, filetag: u32, guid: *const windows_core::GUID, databuffer: *const core::ffi::c_void, databufferlength: u16) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltTagFile(initiatinginstance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, filetag : u32, guid : *const windows_core::GUID, databuffer : *const core::ffi::c_void, databufferlength : u16) -> windows_core:: NTSTATUS);
    unsafe { FltTagFile(initiatinginstance, fileobject, filetag, guid, databuffer, databufferlength) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltTagFileEx(initiatinginstance: PFLT_INSTANCE, fileobject: *mut super::super::super::Foundation::FILE_OBJECT, filetag: u32, guid: *mut windows_core::GUID, databuffer: *mut core::ffi::c_void, databufferlength: u16, existingfiletag: u32, existingguid: *mut windows_core::GUID, flags: u32) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltTagFileEx(initiatinginstance : PFLT_INSTANCE, fileobject : *mut super::super::super::Foundation:: FILE_OBJECT, filetag : u32, guid : *mut windows_core::GUID, databuffer : *mut core::ffi::c_void, databufferlength : u16, existingfiletag : u32, existingguid : *mut windows_core::GUID, flags : u32) -> windows_core:: NTSTATUS);
    unsafe { FltTagFileEx(initiatinginstance, fileobject as _, filetag, guid as _, databuffer as _, databufferlength, existingfiletag, existingguid as _, flags) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltUninitializeFileLock(filelock: *const super::FILE_LOCK) {
    windows_core::link!("fltmgr.sys" "system" fn FltUninitializeFileLock(filelock : *const super:: FILE_LOCK));
    unsafe { FltUninitializeFileLock(filelock) }
}
#[inline]
pub unsafe fn FltUninitializeOplock(oplock: *mut *mut core::ffi::c_void) {
    windows_core::link!("fltmgr.sys" "system" fn FltUninitializeOplock(oplock : *mut *mut core::ffi::c_void));
    unsafe { FltUninitializeOplock(oplock as _) }
}
#[inline]
pub unsafe fn FltUnloadFilter(filtername: *mut super::super::super::super::Win32::Foundation::UNICODE_STRING) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltUnloadFilter(filtername : *mut super::super::super::super::Win32::Foundation:: UNICODE_STRING) -> windows_core:: NTSTATUS);
    unsafe { FltUnloadFilter(filtername as _) }
}
#[inline]
pub unsafe fn FltUnregisterFilter(filter: PFLT_FILTER) {
    windows_core::link!("fltmgr.sys" "system" fn FltUnregisterFilter(filter : PFLT_FILTER));
    unsafe { FltUnregisterFilter(filter) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltUntagFile(initiatinginstance: PFLT_INSTANCE, fileobject: *const super::super::super::Foundation::FILE_OBJECT, filetag: u32, guid: *const windows_core::GUID) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltUntagFile(initiatinginstance : PFLT_INSTANCE, fileobject : *const super::super::super::Foundation:: FILE_OBJECT, filetag : u32, guid : *const windows_core::GUID) -> windows_core:: NTSTATUS);
    unsafe { FltUntagFile(initiatinginstance, fileobject, filetag, guid) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltVetoBypassIo(callbackdata: *mut FLT_CALLBACK_DATA, fltobjects: *mut FLT_RELATED_OBJECTS, operationstatus: windows_core::NTSTATUS, failurereason: *mut super::super::super::super::Win32::Foundation::UNICODE_STRING) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltVetoBypassIo(callbackdata : *mut FLT_CALLBACK_DATA, fltobjects : *mut FLT_RELATED_OBJECTS, operationstatus : windows_core:: NTSTATUS, failurereason : *mut super::super::super::super::Win32::Foundation:: UNICODE_STRING) -> windows_core:: NTSTATUS);
    unsafe { FltVetoBypassIo(callbackdata as _, fltobjects as _, operationstatus, failurereason as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltWriteFile(initiatinginstance: PFLT_INSTANCE, fileobject: *mut super::super::super::Foundation::FILE_OBJECT, byteoffset: *mut i64, length: u32, buffer: *mut core::ffi::c_void, flags: u32, byteswritten: *mut u32, callbackroutine: PFLT_COMPLETED_ASYNC_IO_CALLBACK, callbackcontext: *mut core::ffi::c_void) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltWriteFile(initiatinginstance : PFLT_INSTANCE, fileobject : *mut super::super::super::Foundation:: FILE_OBJECT, byteoffset : *mut i64, length : u32, buffer : *mut core::ffi::c_void, flags : u32, byteswritten : *mut u32, callbackroutine : PFLT_COMPLETED_ASYNC_IO_CALLBACK, callbackcontext : *mut core::ffi::c_void) -> windows_core:: NTSTATUS);
    unsafe { FltWriteFile(initiatinginstance, fileobject as _, byteoffset as _, length, buffer as _, flags, byteswritten as _, callbackroutine, callbackcontext as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltWriteFileEx(initiatinginstance: PFLT_INSTANCE, fileobject: *mut super::super::super::Foundation::FILE_OBJECT, byteoffset: *mut i64, length: u32, buffer: *mut core::ffi::c_void, flags: u32, byteswritten: *mut u32, callbackroutine: PFLT_COMPLETED_ASYNC_IO_CALLBACK, callbackcontext: *mut core::ffi::c_void, key: *mut u32, mdl: *mut super::super::super::Foundation::MDL) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltWriteFileEx(initiatinginstance : PFLT_INSTANCE, fileobject : *mut super::super::super::Foundation:: FILE_OBJECT, byteoffset : *mut i64, length : u32, buffer : *mut core::ffi::c_void, flags : u32, byteswritten : *mut u32, callbackroutine : PFLT_COMPLETED_ASYNC_IO_CALLBACK, callbackcontext : *mut core::ffi::c_void, key : *mut u32, mdl : *mut super::super::super::Foundation:: MDL) -> windows_core:: NTSTATUS);
    unsafe { FltWriteFileEx(initiatinginstance, fileobject as _, byteoffset as _, length, buffer as _, flags, byteswritten as _, callbackroutine, callbackcontext as _, key as _, mdl as _) }
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[inline]
pub unsafe fn FltpTraceRedirectedFileIo(originatingfileobject: *mut super::super::super::Foundation::FILE_OBJECT, childcallbackdata: *mut FLT_CALLBACK_DATA) -> windows_core::NTSTATUS {
    windows_core::link!("fltmgr.sys" "system" fn FltpTraceRedirectedFileIo(originatingfileobject : *mut super::super::super::Foundation:: FILE_OBJECT, childcallbackdata : *mut FLT_CALLBACK_DATA) -> windows_core:: NTSTATUS);
    unsafe { FltpTraceRedirectedFileIo(originatingfileobject as _, childcallbackdata as _) }
}
pub const FLTFL_CALLBACK_DATA_DIRTY: u32 = 2147483648u32;
pub const FLTFL_CALLBACK_DATA_DRAINING_IO: u32 = 262144u32;
pub const FLTFL_CALLBACK_DATA_FAST_IO_OPERATION: u32 = 2u32;
pub const FLTFL_CALLBACK_DATA_FS_FILTER_OPERATION: u32 = 4u32;
pub const FLTFL_CALLBACK_DATA_GENERATED_IO: u32 = 65536u32;
pub const FLTFL_CALLBACK_DATA_IRP_OPERATION: u32 = 1u32;
pub const FLTFL_CALLBACK_DATA_NEW_SYSTEM_BUFFER: u32 = 1048576u32;
pub const FLTFL_CALLBACK_DATA_POST_OPERATION: u32 = 524288u32;
pub const FLTFL_CALLBACK_DATA_REISSUED_IO: u32 = 131072u32;
pub const FLTFL_CALLBACK_DATA_REISSUE_MASK: u32 = 65535u32;
pub const FLTFL_CALLBACK_DATA_SYSTEM_BUFFER: u32 = 8u32;
pub const FLTFL_CONTEXT_REGISTRATION_NO_EXACT_SIZE_MATCH: u32 = 1u32;
pub const FLTFL_FILE_NAME_PARSED_EXTENSION: u32 = 2u32;
pub const FLTFL_FILE_NAME_PARSED_FINAL_COMPONENT: u32 = 1u32;
pub const FLTFL_FILE_NAME_PARSED_PARENT_DIR: u32 = 8u32;
pub const FLTFL_FILE_NAME_PARSED_STREAM: u32 = 4u32;
pub const FLTFL_FILTER_UNLOAD_MANDATORY: u32 = 1u32;
pub const FLTFL_INSTANCE_SETUP_AUTOMATIC_ATTACHMENT: u32 = 1u32;
pub const FLTFL_INSTANCE_SETUP_DETACHED_VOLUME: u32 = 8u32;
pub const FLTFL_INSTANCE_SETUP_MANUAL_ATTACHMENT: u32 = 2u32;
pub const FLTFL_INSTANCE_SETUP_NEWLY_MOUNTED_VOLUME: u32 = 4u32;
pub const FLTFL_INSTANCE_TEARDOWN_FILTER_UNLOAD: u32 = 2u32;
pub const FLTFL_INSTANCE_TEARDOWN_INTERNAL_ERROR: u32 = 16u32;
pub const FLTFL_INSTANCE_TEARDOWN_MANDATORY_FILTER_UNLOAD: u32 = 4u32;
pub const FLTFL_INSTANCE_TEARDOWN_MANUAL: u32 = 1u32;
pub const FLTFL_INSTANCE_TEARDOWN_VOLUME_DISMOUNT: u32 = 8u32;
pub const FLTFL_IO_OPERATION_DO_NOT_UPDATE_BYTE_OFFSET: u32 = 4u32;
pub const FLTFL_IO_OPERATION_NON_CACHED: u32 = 1u32;
pub const FLTFL_IO_OPERATION_PAGING: u32 = 2u32;
pub const FLTFL_IO_OPERATION_SYNCHRONOUS_PAGING: u32 = 8u32;
pub const FLTFL_NORMALIZE_NAME_CASE_SENSITIVE: u32 = 1u32;
pub const FLTFL_NORMALIZE_NAME_DESTINATION_FILE_NAME: u32 = 2u32;
pub const FLTFL_OPERATION_REGISTRATION_SKIP_CACHED_IO: u32 = 2u32;
pub const FLTFL_OPERATION_REGISTRATION_SKIP_NON_CACHED_NON_PAGING_IO: u32 = 8u32;
pub const FLTFL_OPERATION_REGISTRATION_SKIP_NON_DASD_IO: u32 = 4u32;
pub const FLTFL_OPERATION_REGISTRATION_SKIP_PAGING_IO: u32 = 1u32;
pub const FLTFL_POST_OPERATION_DRAINING: u32 = 1u32;
pub const FLTFL_REGISTRATION_DO_NOT_SUPPORT_SERVICE_STOP: u32 = 1u32;
pub const FLTFL_REGISTRATION_SUPPORT_DAX_VOLUME: u32 = 4u32;
pub const FLTFL_REGISTRATION_SUPPORT_NPFS_MSFS: u32 = 2u32;
pub const FLTFL_REGISTRATION_SUPPORT_WCOS: u32 = 8u32;
pub const FLTTCFL_AUTO_REPARSE: u32 = 1u32;
pub const FLT_ALLOCATE_CALLBACK_DATA_PREALLOCATE_ALL_MEMORY: u32 = 1u32;
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct FLT_CALLBACK_DATA {
    pub Flags: u32,
    pub Thread: super::super::super::Foundation::PETHREAD,
    pub Iopb: *const FLT_IO_PARAMETER_BLOCK,
    pub IoStatus: super::super::super::super::Win32::System::IO::IO_STATUS_BLOCK,
    pub TagData: *mut FLT_TAG_DATA_BUFFER,
    pub Anonymous: FLT_CALLBACK_DATA_0,
    pub RequestorMode: i8,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_CALLBACK_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub union FLT_CALLBACK_DATA_0 {
    pub Anonymous: FLT_CALLBACK_DATA_0_0,
    pub FilterContext: [*mut core::ffi::c_void; 4],
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for FLT_CALLBACK_DATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLT_CALLBACK_DATA_0_0 {
    pub QueueLinks: super::super::super::super::Win32::System::Kernel::LIST_ENTRY,
    pub QueueContext: [*mut core::ffi::c_void; 2],
}
#[cfg(feature = "Win32_System_Kernel")]
impl Default for FLT_CALLBACK_DATA_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct FLT_CALLBACK_DATA_QUEUE {
    pub Csq: super::super::super::System::SystemServices::IO_CSQ,
    pub Flags: FLT_CALLBACK_DATA_QUEUE_FLAGS,
    pub Instance: PFLT_INSTANCE,
    pub InsertIo: PFLT_CALLBACK_DATA_QUEUE_INSERT_IO,
    pub RemoveIo: PFLT_CALLBACK_DATA_QUEUE_REMOVE_IO,
    pub PeekNextIo: PFLT_CALLBACK_DATA_QUEUE_PEEK_NEXT_IO,
    pub Acquire: PFLT_CALLBACK_DATA_QUEUE_ACQUIRE,
    pub Release: PFLT_CALLBACK_DATA_QUEUE_RELEASE,
    pub CompleteCanceledIo: PFLT_CALLBACK_DATA_QUEUE_COMPLETE_CANCELED_IO,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FLT_CALLBACK_DATA_QUEUE_FLAGS(pub i32);
pub const FLT_CONTEXT_END: u32 = 65535u32;
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy, Debug)]
pub struct FLT_CONTEXT_REGISTRATION {
    pub ContextType: u16,
    pub Flags: u16,
    pub ContextCleanupCallback: PFLT_CONTEXT_CLEANUP_CALLBACK,
    pub Size: usize,
    pub PoolTag: u32,
    pub ContextAllocateCallback: PFLT_CONTEXT_ALLOCATE_CALLBACK,
    pub ContextFreeCallback: PFLT_CONTEXT_FREE_CALLBACK,
    pub Reserved1: *mut core::ffi::c_void,
}
#[cfg(feature = "Wdk_Foundation")]
impl Default for FLT_CONTEXT_REGISTRATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLT_CREATEFILE_TARGET_ECP_CONTEXT {
    pub Instance: PFLT_INSTANCE,
    pub Volume: PFLT_VOLUME,
    pub FileNameInformation: *mut FLT_FILE_NAME_INFORMATION,
    pub Flags: u16,
}
impl Default for FLT_CREATEFILE_TARGET_ECP_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FLT_FILE_CONTEXT: u32 = 4u32;
pub const FLT_FILE_NAME_ALLOW_QUERY_ON_REPARSE: u32 = 67108864u32;
pub const FLT_FILE_NAME_DO_NOT_CACHE: u32 = 33554432u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FLT_FILE_NAME_INFORMATION {
    pub Size: u16,
    pub NamesParsed: u16,
    pub Format: u32,
    pub Name: super::super::super::super::Win32::Foundation::UNICODE_STRING,
    pub Volume: super::super::super::super::Win32::Foundation::UNICODE_STRING,
    pub Share: super::super::super::super::Win32::Foundation::UNICODE_STRING,
    pub Extension: super::super::super::super::Win32::Foundation::UNICODE_STRING,
    pub Stream: super::super::super::super::Win32::Foundation::UNICODE_STRING,
    pub FinalComponent: super::super::super::super::Win32::Foundation::UNICODE_STRING,
    pub ParentDir: super::super::super::super::Win32::Foundation::UNICODE_STRING,
}
pub const FLT_FILE_NAME_NORMALIZED: u32 = 1u32;
pub const FLT_FILE_NAME_OPENED: u32 = 2u32;
pub const FLT_FILE_NAME_QUERY_ALWAYS_ALLOW_CACHE_LOOKUP: u32 = 1024u32;
pub const FLT_FILE_NAME_QUERY_CACHE_ONLY: u32 = 512u32;
pub const FLT_FILE_NAME_QUERY_DEFAULT: u32 = 256u32;
pub const FLT_FILE_NAME_QUERY_FILESYSTEM_ONLY: u32 = 768u32;
pub const FLT_FILE_NAME_REQUEST_FROM_CURRENT_PROVIDER: u32 = 16777216u32;
pub const FLT_FILE_NAME_SHORT: u32 = 3u32;
pub const FLT_FLUSH_TYPE_DATA_SYNC_ONLY: u32 = 8u32;
pub const FLT_FLUSH_TYPE_FILE_DATA_ONLY: u32 = 2u32;
pub const FLT_FLUSH_TYPE_FLUSH_AND_PURGE: u32 = 1u32;
pub const FLT_FLUSH_TYPE_NO_SYNC: u32 = 4u32;
pub const FLT_INSTANCE_CONTEXT: u32 = 2u32;
pub const FLT_INTERNAL_OPERATION_COUNT: u32 = 22u32;
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct FLT_IO_PARAMETER_BLOCK {
    pub IrpFlags: u32,
    pub MajorFunction: u8,
    pub MinorFunction: u8,
    pub OperationFlags: u8,
    pub Reserved: u8,
    pub TargetFileObject: *mut super::super::super::Foundation::FILE_OBJECT,
    pub TargetInstance: PFLT_INSTANCE,
    pub Parameters: FLT_PARAMETERS,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_IO_PARAMETER_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FLT_MAX_DEVICE_REPARSE_ATTEMPTS: u32 = 64u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FLT_NAME_CONTROL {
    pub Name: super::super::super::super::Win32::Foundation::UNICODE_STRING,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug)]
pub struct FLT_OPERATION_REGISTRATION {
    pub MajorFunction: u8,
    pub Flags: u32,
    pub PreOperation: PFLT_PRE_OPERATION_CALLBACK,
    pub PostOperation: PFLT_POST_OPERATION_CALLBACK,
    pub Reserved1: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_OPERATION_REGISTRATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub union FLT_PARAMETERS {
    pub Create: FLT_PARAMETERS_0,
    pub CreatePipe: FLT_PARAMETERS_1,
    pub CreateMailslot: FLT_PARAMETERS_2,
    pub Read: FLT_PARAMETERS_3,
    pub Write: FLT_PARAMETERS_4,
    pub QueryFileInformation: FLT_PARAMETERS_5,
    pub SetFileInformation: FLT_PARAMETERS_6,
    pub QueryEa: FLT_PARAMETERS_7,
    pub SetEa: FLT_PARAMETERS_8,
    pub QueryVolumeInformation: FLT_PARAMETERS_9,
    pub SetVolumeInformation: FLT_PARAMETERS_10,
    pub DirectoryControl: FLT_PARAMETERS_11,
    pub FileSystemControl: FLT_PARAMETERS_12,
    pub DeviceIoControl: FLT_PARAMETERS_13,
    pub LockControl: FLT_PARAMETERS_14,
    pub QuerySecurity: FLT_PARAMETERS_15,
    pub SetSecurity: FLT_PARAMETERS_16,
    pub WMI: FLT_PARAMETERS_17,
    pub QueryQuota: FLT_PARAMETERS_18,
    pub SetQuota: FLT_PARAMETERS_19,
    pub Pnp: FLT_PARAMETERS_20,
    pub AcquireForSectionSynchronization: FLT_PARAMETERS_21,
    pub AcquireForModifiedPageWriter: FLT_PARAMETERS_22,
    pub ReleaseForModifiedPageWriter: FLT_PARAMETERS_23,
    pub QueryOpen: FLT_PARAMETERS_24,
    pub FastIoCheckIfPossible: FLT_PARAMETERS_25,
    pub NetworkQueryOpen: FLT_PARAMETERS_26,
    pub MdlRead: FLT_PARAMETERS_27,
    pub MdlReadComplete: FLT_PARAMETERS_28,
    pub PrepareMdlWrite: FLT_PARAMETERS_29,
    pub MdlWriteComplete: FLT_PARAMETERS_30,
    pub MountVolume: FLT_PARAMETERS_31,
    pub Others: FLT_PARAMETERS_32,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLT_PARAMETERS_0 {
    pub SecurityContext: *mut super::super::super::Foundation::IO_SECURITY_CONTEXT,
    pub Options: u32,
    pub FileAttributes: u16,
    pub ShareAccess: u16,
    pub EaLength: u32,
    pub EaBuffer: *mut core::ffi::c_void,
    pub AllocationSize: i64,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security"))]
impl Default for FLT_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLT_PARAMETERS_1 {
    pub SecurityContext: *mut super::super::super::Foundation::IO_SECURITY_CONTEXT,
    pub Options: u32,
    pub Reserved: u16,
    pub ShareAccess: u16,
    pub Parameters: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security"))]
impl Default for FLT_PARAMETERS_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLT_PARAMETERS_10 {
    pub Length: u32,
    pub FsInformationClass: super::FS_INFORMATION_CLASS,
    pub VolumeBuffer: *mut core::ffi::c_void,
}
impl Default for FLT_PARAMETERS_10 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices"))]
#[derive(Clone, Copy)]
pub union FLT_PARAMETERS_11 {
    pub QueryDirectory: FLT_PARAMETERS_11_0,
    pub NotifyDirectory: FLT_PARAMETERS_11_1,
    pub NotifyDirectoryEx: FLT_PARAMETERS_11_2,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices"))]
impl Default for FLT_PARAMETERS_11 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLT_PARAMETERS_11_0 {
    pub Length: u32,
    pub FileName: *mut super::super::super::super::Win32::Foundation::UNICODE_STRING,
    pub FileInformationClass: super::FILE_INFORMATION_CLASS,
    pub FileIndex: u32,
    pub DirectoryBuffer: *mut core::ffi::c_void,
    pub MdlAddress: *mut super::super::super::Foundation::MDL,
}
#[cfg(feature = "Wdk_Foundation")]
impl Default for FLT_PARAMETERS_11_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLT_PARAMETERS_11_1 {
    pub Length: u32,
    pub CompletionFilter: u32,
    pub Spare1: u32,
    pub Spare2: u32,
    pub DirectoryBuffer: *mut core::ffi::c_void,
    pub MdlAddress: *mut super::super::super::Foundation::MDL,
}
#[cfg(feature = "Wdk_Foundation")]
impl Default for FLT_PARAMETERS_11_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLT_PARAMETERS_11_2 {
    pub Length: u32,
    pub CompletionFilter: u32,
    pub DirectoryNotifyInformationClass: super::super::super::System::SystemServices::DIRECTORY_NOTIFY_INFORMATION_CLASS,
    pub Spare2: u32,
    pub DirectoryBuffer: *mut core::ffi::c_void,
    pub MdlAddress: *mut super::super::super::Foundation::MDL,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices"))]
impl Default for FLT_PARAMETERS_11_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub union FLT_PARAMETERS_12 {
    pub VerifyVolume: FLT_PARAMETERS_12_0,
    pub Common: FLT_PARAMETERS_12_1,
    pub Neither: FLT_PARAMETERS_12_2,
    pub Buffered: FLT_PARAMETERS_12_3,
    pub Direct: FLT_PARAMETERS_12_4,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_12 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLT_PARAMETERS_12_0 {
    pub Vpb: *mut super::super::super::Foundation::VPB,
    pub DeviceObject: *mut super::super::super::Foundation::DEVICE_OBJECT,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_12_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FLT_PARAMETERS_12_1 {
    pub OutputBufferLength: u32,
    pub InputBufferLength: u32,
    pub FsControlCode: u32,
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLT_PARAMETERS_12_2 {
    pub OutputBufferLength: u32,
    pub InputBufferLength: u32,
    pub FsControlCode: u32,
    pub InputBuffer: *mut core::ffi::c_void,
    pub OutputBuffer: *mut core::ffi::c_void,
    pub OutputMdlAddress: *mut super::super::super::Foundation::MDL,
}
#[cfg(feature = "Wdk_Foundation")]
impl Default for FLT_PARAMETERS_12_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLT_PARAMETERS_12_3 {
    pub OutputBufferLength: u32,
    pub InputBufferLength: u32,
    pub FsControlCode: u32,
    pub SystemBuffer: *mut core::ffi::c_void,
}
impl Default for FLT_PARAMETERS_12_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLT_PARAMETERS_12_4 {
    pub OutputBufferLength: u32,
    pub InputBufferLength: u32,
    pub FsControlCode: u32,
    pub InputSystemBuffer: *mut core::ffi::c_void,
    pub OutputBuffer: *mut core::ffi::c_void,
    pub OutputMdlAddress: *mut super::super::super::Foundation::MDL,
}
#[cfg(feature = "Wdk_Foundation")]
impl Default for FLT_PARAMETERS_12_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy)]
pub union FLT_PARAMETERS_13 {
    pub Common: FLT_PARAMETERS_13_0,
    pub Neither: FLT_PARAMETERS_13_1,
    pub Buffered: FLT_PARAMETERS_13_2,
    pub Direct: FLT_PARAMETERS_13_3,
    pub FastIo: FLT_PARAMETERS_13_4,
}
#[cfg(feature = "Wdk_Foundation")]
impl Default for FLT_PARAMETERS_13 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FLT_PARAMETERS_13_0 {
    pub OutputBufferLength: u32,
    pub InputBufferLength: u32,
    pub IoControlCode: u32,
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLT_PARAMETERS_13_1 {
    pub OutputBufferLength: u32,
    pub InputBufferLength: u32,
    pub IoControlCode: u32,
    pub InputBuffer: *mut core::ffi::c_void,
    pub OutputBuffer: *mut core::ffi::c_void,
    pub OutputMdlAddress: *mut super::super::super::Foundation::MDL,
}
#[cfg(feature = "Wdk_Foundation")]
impl Default for FLT_PARAMETERS_13_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLT_PARAMETERS_13_2 {
    pub OutputBufferLength: u32,
    pub InputBufferLength: u32,
    pub IoControlCode: u32,
    pub SystemBuffer: *mut core::ffi::c_void,
}
impl Default for FLT_PARAMETERS_13_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLT_PARAMETERS_13_3 {
    pub OutputBufferLength: u32,
    pub InputBufferLength: u32,
    pub IoControlCode: u32,
    pub InputSystemBuffer: *mut core::ffi::c_void,
    pub OutputBuffer: *mut core::ffi::c_void,
    pub OutputMdlAddress: *mut super::super::super::Foundation::MDL,
}
#[cfg(feature = "Wdk_Foundation")]
impl Default for FLT_PARAMETERS_13_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLT_PARAMETERS_13_4 {
    pub OutputBufferLength: u32,
    pub InputBufferLength: u32,
    pub IoControlCode: u32,
    pub InputBuffer: *mut core::ffi::c_void,
    pub OutputBuffer: *mut core::ffi::c_void,
}
impl Default for FLT_PARAMETERS_13_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLT_PARAMETERS_14 {
    pub Length: *mut i64,
    pub Key: u32,
    pub ByteOffset: i64,
    pub ProcessId: super::super::super::Foundation::PEPROCESS,
    pub FailImmediately: bool,
    pub ExclusiveLock: bool,
}
#[cfg(feature = "Wdk_Foundation")]
impl Default for FLT_PARAMETERS_14 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLT_PARAMETERS_15 {
    pub SecurityInformation: u32,
    pub Length: u32,
    pub SecurityBuffer: *mut core::ffi::c_void,
    pub MdlAddress: *mut super::super::super::Foundation::MDL,
}
#[cfg(feature = "Wdk_Foundation")]
impl Default for FLT_PARAMETERS_15 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FLT_PARAMETERS_16 {
    pub SecurityInformation: u32,
    pub SecurityDescriptor: super::super::super::super::Win32::Security::PSECURITY_DESCRIPTOR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLT_PARAMETERS_17 {
    pub ProviderId: usize,
    pub DataPath: *mut core::ffi::c_void,
    pub BufferSize: u32,
    pub Buffer: *mut core::ffi::c_void,
}
impl Default for FLT_PARAMETERS_17 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLT_PARAMETERS_18 {
    pub Length: u32,
    pub StartSid: super::super::super::super::Win32::Security::PSID,
    pub SidList: *mut super::FILE_GET_QUOTA_INFORMATION,
    pub SidListLength: u32,
    pub QuotaBuffer: *mut core::ffi::c_void,
    pub MdlAddress: *mut super::super::super::Foundation::MDL,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
impl Default for FLT_PARAMETERS_18 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLT_PARAMETERS_19 {
    pub Length: u32,
    pub QuotaBuffer: *mut core::ffi::c_void,
    pub MdlAddress: *mut super::super::super::Foundation::MDL,
}
#[cfg(feature = "Wdk_Foundation")]
impl Default for FLT_PARAMETERS_19 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLT_PARAMETERS_2 {
    pub SecurityContext: *mut super::super::super::Foundation::IO_SECURITY_CONTEXT,
    pub Options: u32,
    pub Reserved: u16,
    pub ShareAccess: u16,
    pub Parameters: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security"))]
impl Default for FLT_PARAMETERS_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_System_SystemServices", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub union FLT_PARAMETERS_20 {
    pub StartDevice: FLT_PARAMETERS_20_0,
    pub QueryDeviceRelations: FLT_PARAMETERS_20_1,
    pub QueryInterface: FLT_PARAMETERS_20_2,
    pub DeviceCapabilities: FLT_PARAMETERS_20_3,
    pub FilterResourceRequirements: FLT_PARAMETERS_20_4,
    pub ReadWriteConfig: FLT_PARAMETERS_20_5,
    pub SetLock: FLT_PARAMETERS_20_6,
    pub QueryId: FLT_PARAMETERS_20_7,
    pub QueryDeviceText: FLT_PARAMETERS_20_8,
    pub UsageNotification: FLT_PARAMETERS_20_9,
}
#[cfg(all(feature = "Wdk_System_SystemServices", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_20 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Wdk_System_SystemServices")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLT_PARAMETERS_20_0 {
    pub AllocatedResources: *mut super::super::super::System::SystemServices::CM_RESOURCE_LIST,
    pub AllocatedResourcesTranslated: *mut super::super::super::System::SystemServices::CM_RESOURCE_LIST,
}
#[cfg(feature = "Wdk_System_SystemServices")]
impl Default for FLT_PARAMETERS_20_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Wdk_System_SystemServices")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FLT_PARAMETERS_20_1 {
    pub Type: super::super::super::System::SystemServices::DEVICE_RELATION_TYPE,
}
#[repr(C)]
#[cfg(feature = "Wdk_System_SystemServices")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLT_PARAMETERS_20_2 {
    pub InterfaceType: *const windows_core::GUID,
    pub Size: u16,
    pub Version: u16,
    pub Interface: *mut super::super::super::System::SystemServices::INTERFACE,
    pub InterfaceSpecificData: *mut core::ffi::c_void,
}
#[cfg(feature = "Wdk_System_SystemServices")]
impl Default for FLT_PARAMETERS_20_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_System_SystemServices", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLT_PARAMETERS_20_3 {
    pub Capabilities: *mut super::super::super::System::SystemServices::DEVICE_CAPABILITIES,
}
#[cfg(all(feature = "Wdk_System_SystemServices", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_20_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Wdk_System_SystemServices")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLT_PARAMETERS_20_4 {
    pub IoResourceRequirementList: *mut super::super::super::System::SystemServices::IO_RESOURCE_REQUIREMENTS_LIST,
}
#[cfg(feature = "Wdk_System_SystemServices")]
impl Default for FLT_PARAMETERS_20_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLT_PARAMETERS_20_5 {
    pub WhichSpace: u32,
    pub Buffer: *mut core::ffi::c_void,
    pub Offset: u32,
    pub Length: u32,
}
impl Default for FLT_PARAMETERS_20_5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FLT_PARAMETERS_20_6 {
    pub Lock: bool,
}
#[repr(C)]
#[cfg(feature = "Wdk_System_SystemServices")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FLT_PARAMETERS_20_7 {
    pub IdType: super::super::super::System::SystemServices::BUS_QUERY_ID_TYPE,
}
#[repr(C)]
#[cfg(feature = "Wdk_System_SystemServices")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FLT_PARAMETERS_20_8 {
    pub DeviceTextType: super::super::super::System::SystemServices::DEVICE_TEXT_TYPE,
    pub LocaleId: u32,
}
#[repr(C)]
#[cfg(feature = "Wdk_System_SystemServices")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLT_PARAMETERS_20_9 {
    pub InPath: bool,
    pub Reserved: [bool; 3],
    pub Type: super::super::super::System::SystemServices::DEVICE_USAGE_NOTIFICATION_TYPE,
}
#[cfg(feature = "Wdk_System_SystemServices")]
impl Default for FLT_PARAMETERS_20_9 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLT_PARAMETERS_21 {
    pub SyncType: super::FS_FILTER_SECTION_SYNC_TYPE,
    pub PageProtection: u32,
    pub OutputInformation: *mut super::FS_FILTER_SECTION_SYNC_OUTPUT,
    pub Flags: u32,
    pub AllocationAttributes: u32,
}
impl Default for FLT_PARAMETERS_21 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLT_PARAMETERS_22 {
    pub EndingOffset: *mut i64,
    pub ResourceToRelease: *mut *mut super::super::super::Foundation::ERESOURCE,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
impl Default for FLT_PARAMETERS_22 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLT_PARAMETERS_23 {
    pub ResourceToRelease: *mut super::super::super::Foundation::ERESOURCE,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
impl Default for FLT_PARAMETERS_23 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLT_PARAMETERS_24 {
    pub Irp: *mut super::super::super::Foundation::IRP,
    pub FileInformation: *mut core::ffi::c_void,
    pub Length: *mut u32,
    pub FileInformationClass: super::FILE_INFORMATION_CLASS,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_24 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FLT_PARAMETERS_25 {
    pub FileOffset: i64,
    pub Length: u32,
    pub LockKey: u32,
    pub CheckForReadOperation: bool,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLT_PARAMETERS_26 {
    pub Irp: *mut super::super::super::Foundation::IRP,
    pub NetworkInformation: *mut super::FILE_NETWORK_OPEN_INFORMATION,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_26 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLT_PARAMETERS_27 {
    pub FileOffset: i64,
    pub Length: u32,
    pub Key: u32,
    pub MdlChain: *mut *mut super::super::super::Foundation::MDL,
}
#[cfg(feature = "Wdk_Foundation")]
impl Default for FLT_PARAMETERS_27 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLT_PARAMETERS_28 {
    pub MdlChain: *mut super::super::super::Foundation::MDL,
}
#[cfg(feature = "Wdk_Foundation")]
impl Default for FLT_PARAMETERS_28 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLT_PARAMETERS_29 {
    pub FileOffset: i64,
    pub Length: u32,
    pub Key: u32,
    pub MdlChain: *mut *mut super::super::super::Foundation::MDL,
}
#[cfg(feature = "Wdk_Foundation")]
impl Default for FLT_PARAMETERS_29 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLT_PARAMETERS_3 {
    pub Length: u32,
    pub Key: u32,
    pub ByteOffset: i64,
    pub ReadBuffer: *mut core::ffi::c_void,
    pub MdlAddress: *mut super::super::super::Foundation::MDL,
}
#[cfg(feature = "Wdk_Foundation")]
impl Default for FLT_PARAMETERS_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLT_PARAMETERS_30 {
    pub FileOffset: i64,
    pub MdlChain: *mut super::super::super::Foundation::MDL,
}
#[cfg(feature = "Wdk_Foundation")]
impl Default for FLT_PARAMETERS_30 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FLT_PARAMETERS_31 {
    pub DeviceType: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLT_PARAMETERS_32 {
    pub Argument1: *mut core::ffi::c_void,
    pub Argument2: *mut core::ffi::c_void,
    pub Argument3: *mut core::ffi::c_void,
    pub Argument4: *mut core::ffi::c_void,
    pub Argument5: *mut core::ffi::c_void,
    pub Argument6: i64,
}
impl Default for FLT_PARAMETERS_32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLT_PARAMETERS_4 {
    pub Length: u32,
    pub Key: u32,
    pub ByteOffset: i64,
    pub WriteBuffer: *mut core::ffi::c_void,
    pub MdlAddress: *mut super::super::super::Foundation::MDL,
}
#[cfg(feature = "Wdk_Foundation")]
impl Default for FLT_PARAMETERS_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLT_PARAMETERS_5 {
    pub Length: u32,
    pub FileInformationClass: super::FILE_INFORMATION_CLASS,
    pub InfoBuffer: *mut core::ffi::c_void,
}
impl Default for FLT_PARAMETERS_5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct FLT_PARAMETERS_6 {
    pub Length: u32,
    pub FileInformationClass: super::FILE_INFORMATION_CLASS,
    pub ParentOfTarget: *mut super::super::super::Foundation::FILE_OBJECT,
    pub Anonymous: FLT_PARAMETERS_6_0,
    pub InfoBuffer: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_PARAMETERS_6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union FLT_PARAMETERS_6_0 {
    pub Anonymous: FLT_PARAMETERS_6_0_0,
    pub ClusterCount: u32,
    pub DeleteHandle: super::super::super::super::Win32::Foundation::HANDLE,
}
impl Default for FLT_PARAMETERS_6_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FLT_PARAMETERS_6_0_0 {
    pub ReplaceIfExists: bool,
    pub AdvanceOnly: bool,
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLT_PARAMETERS_7 {
    pub Length: u32,
    pub EaList: *mut core::ffi::c_void,
    pub EaListLength: u32,
    pub EaIndex: u32,
    pub EaBuffer: *mut core::ffi::c_void,
    pub MdlAddress: *mut super::super::super::Foundation::MDL,
}
#[cfg(feature = "Wdk_Foundation")]
impl Default for FLT_PARAMETERS_7 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLT_PARAMETERS_8 {
    pub Length: u32,
    pub EaBuffer: *mut core::ffi::c_void,
    pub MdlAddress: *mut super::super::super::Foundation::MDL,
}
#[cfg(feature = "Wdk_Foundation")]
impl Default for FLT_PARAMETERS_8 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLT_PARAMETERS_9 {
    pub Length: u32,
    pub FsInformationClass: super::FS_INFORMATION_CLASS,
    pub VolumeBuffer: *mut core::ffi::c_void,
}
impl Default for FLT_PARAMETERS_9 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FLT_PORT_CONNECT: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FLT_POSTOP_CALLBACK_STATUS(pub i32);
pub const FLT_POSTOP_DISALLOW_FSFILTER_IO: FLT_POSTOP_CALLBACK_STATUS = FLT_POSTOP_CALLBACK_STATUS(2i32);
pub const FLT_POSTOP_FINISHED_PROCESSING: FLT_POSTOP_CALLBACK_STATUS = FLT_POSTOP_CALLBACK_STATUS(0i32);
pub const FLT_POSTOP_MORE_PROCESSING_REQUIRED: FLT_POSTOP_CALLBACK_STATUS = FLT_POSTOP_CALLBACK_STATUS(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FLT_PREOP_CALLBACK_STATUS(pub i32);
pub const FLT_PREOP_COMPLETE: FLT_PREOP_CALLBACK_STATUS = FLT_PREOP_CALLBACK_STATUS(4i32);
pub const FLT_PREOP_DISALLOW_FASTIO: FLT_PREOP_CALLBACK_STATUS = FLT_PREOP_CALLBACK_STATUS(3i32);
pub const FLT_PREOP_DISALLOW_FSFILTER_IO: FLT_PREOP_CALLBACK_STATUS = FLT_PREOP_CALLBACK_STATUS(6i32);
pub const FLT_PREOP_PENDING: FLT_PREOP_CALLBACK_STATUS = FLT_PREOP_CALLBACK_STATUS(2i32);
pub const FLT_PREOP_SUCCESS_NO_CALLBACK: FLT_PREOP_CALLBACK_STATUS = FLT_PREOP_CALLBACK_STATUS(1i32);
pub const FLT_PREOP_SUCCESS_WITH_CALLBACK: FLT_PREOP_CALLBACK_STATUS = FLT_PREOP_CALLBACK_STATUS(0i32);
pub const FLT_PREOP_SYNCHRONIZE: FLT_PREOP_CALLBACK_STATUS = FLT_PREOP_CALLBACK_STATUS(5i32);
pub const FLT_PUSH_LOCK_DISABLE_AUTO_BOOST: u32 = 2u32;
pub const FLT_PUSH_LOCK_ENABLE_AUTO_BOOST: u32 = 1u32;
pub const FLT_PUSH_LOCK_VALID_FLAGS: u32 = 3u32;
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_Storage_InstallableFileSystems", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug)]
pub struct FLT_REGISTRATION {
    pub Size: u16,
    pub Version: u16,
    pub Flags: u32,
    pub ContextRegistration: *const FLT_CONTEXT_REGISTRATION,
    pub OperationRegistration: *const FLT_OPERATION_REGISTRATION,
    pub FilterUnloadCallback: PFLT_FILTER_UNLOAD_CALLBACK,
    pub InstanceSetupCallback: PFLT_INSTANCE_SETUP_CALLBACK,
    pub InstanceQueryTeardownCallback: PFLT_INSTANCE_QUERY_TEARDOWN_CALLBACK,
    pub InstanceTeardownStartCallback: PFLT_INSTANCE_TEARDOWN_CALLBACK,
    pub InstanceTeardownCompleteCallback: PFLT_INSTANCE_TEARDOWN_CALLBACK,
    pub GenerateFileNameCallback: PFLT_GENERATE_FILE_NAME,
    pub NormalizeNameComponentCallback: PFLT_NORMALIZE_NAME_COMPONENT,
    pub NormalizeContextCleanupCallback: PFLT_NORMALIZE_CONTEXT_CLEANUP,
    pub TransactionNotificationCallback: PFLT_TRANSACTION_NOTIFICATION_CALLBACK,
    pub NormalizeNameComponentExCallback: PFLT_NORMALIZE_NAME_COMPONENT_EX,
    pub SectionNotificationCallback: PFLT_SECTION_CONFLICT_NOTIFICATION_CALLBACK,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_Storage_InstallableFileSystems", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_REGISTRATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FLT_REGISTRATION_VERSION: u32 = 515u32;
pub const FLT_REGISTRATION_VERSION_0200: u32 = 512u32;
pub const FLT_REGISTRATION_VERSION_0201: u32 = 513u32;
pub const FLT_REGISTRATION_VERSION_0202: u32 = 514u32;
pub const FLT_REGISTRATION_VERSION_0203: u32 = 515u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FLT_RELATED_CONTEXTS {
    pub VolumeContext: PFLT_CONTEXT,
    pub InstanceContext: PFLT_CONTEXT,
    pub FileContext: PFLT_CONTEXT,
    pub StreamContext: PFLT_CONTEXT,
    pub StreamHandleContext: PFLT_CONTEXT,
    pub TransactionContext: PFLT_CONTEXT,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FLT_RELATED_CONTEXTS_EX {
    pub VolumeContext: PFLT_CONTEXT,
    pub InstanceContext: PFLT_CONTEXT,
    pub FileContext: PFLT_CONTEXT,
    pub StreamContext: PFLT_CONTEXT,
    pub StreamHandleContext: PFLT_CONTEXT,
    pub TransactionContext: PFLT_CONTEXT,
    pub SectionContext: PFLT_CONTEXT,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLT_RELATED_OBJECTS {
    pub Size: u16,
    pub TransactionContext: u16,
    pub Filter: PFLT_FILTER,
    pub Volume: PFLT_VOLUME,
    pub Instance: PFLT_INSTANCE,
    pub FileObject: *const super::super::super::Foundation::FILE_OBJECT,
    pub Transaction: *const super::super::super::Foundation::KTRANSACTION,
}
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
impl Default for FLT_RELATED_OBJECTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FLT_SECTION_CONTEXT: u32 = 64u32;
pub const FLT_SET_CONTEXT_KEEP_IF_EXISTS: FLT_SET_CONTEXT_OPERATION = FLT_SET_CONTEXT_OPERATION(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FLT_SET_CONTEXT_OPERATION(pub i32);
pub const FLT_SET_CONTEXT_REPLACE_IF_EXISTS: FLT_SET_CONTEXT_OPERATION = FLT_SET_CONTEXT_OPERATION(0i32);
pub const FLT_STREAMHANDLE_CONTEXT: u32 = 16u32;
pub const FLT_STREAM_CONTEXT: u32 = 8u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FLT_TAG_DATA_BUFFER {
    pub FileTag: u32,
    pub TagDataLength: u16,
    pub UnparsedNameLength: u16,
    pub Anonymous: FLT_TAG_DATA_BUFFER_0,
}
impl Default for FLT_TAG_DATA_BUFFER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union FLT_TAG_DATA_BUFFER_0 {
    pub SymbolicLinkReparseBuffer: FLT_TAG_DATA_BUFFER_0_0,
    pub MountPointReparseBuffer: FLT_TAG_DATA_BUFFER_0_1,
    pub GenericReparseBuffer: FLT_TAG_DATA_BUFFER_0_2,
    pub GenericGUIDReparseBuffer: FLT_TAG_DATA_BUFFER_0_3,
}
impl Default for FLT_TAG_DATA_BUFFER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLT_TAG_DATA_BUFFER_0_0 {
    pub SubstituteNameOffset: u16,
    pub SubstituteNameLength: u16,
    pub PrintNameOffset: u16,
    pub PrintNameLength: u16,
    pub Flags: u32,
    pub PathBuffer: [u16; 1],
}
impl Default for FLT_TAG_DATA_BUFFER_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLT_TAG_DATA_BUFFER_0_1 {
    pub SubstituteNameOffset: u16,
    pub SubstituteNameLength: u16,
    pub PrintNameOffset: u16,
    pub PrintNameLength: u16,
    pub PathBuffer: [u16; 1],
}
impl Default for FLT_TAG_DATA_BUFFER_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLT_TAG_DATA_BUFFER_0_2 {
    pub DataBuffer: [u8; 1],
}
impl Default for FLT_TAG_DATA_BUFFER_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FLT_TAG_DATA_BUFFER_0_3 {
    pub TagGuid: windows_core::GUID,
    pub DataBuffer: [u8; 1],
}
impl Default for FLT_TAG_DATA_BUFFER_0_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FLT_TRANSACTION_CONTEXT: u32 = 32u32;
pub const FLT_VALID_FILE_NAME_FLAGS: u32 = 4278190080u32;
pub const FLT_VALID_FILE_NAME_FORMATS: u32 = 255u32;
pub const FLT_VALID_FILE_NAME_QUERY_METHODS: u32 = 65280u32;
pub const FLT_VOLUME_CONTEXT: u32 = 1u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FLT_VOLUME_PROPERTIES {
    pub DeviceType: u32,
    pub DeviceCharacteristics: u32,
    pub DeviceObjectFlags: u32,
    pub AlignmentRequirement: u32,
    pub SectorSize: u16,
    pub Flags: u16,
    pub FileSystemDriverName: super::super::super::super::Win32::Foundation::UNICODE_STRING,
    pub FileSystemDeviceName: super::super::super::super::Win32::Foundation::UNICODE_STRING,
    pub RealDeviceName: super::super::super::super::Win32::Foundation::UNICODE_STRING,
}
pub const GUID_ECP_FLT_CREATEFILE_TARGET: windows_core::GUID = windows_core::GUID::from_u128(0xce08041d_f411_447f_b70d_ccee45c23fac);
pub const IRP_MJ_ACQUIRE_FOR_CC_FLUSH: u16 = 65531u16;
pub const IRP_MJ_ACQUIRE_FOR_MOD_WRITE: u16 = 65533u16;
pub const IRP_MJ_ACQUIRE_FOR_SECTION_SYNCHRONIZATION: u16 = 65535u16;
pub const IRP_MJ_FAST_IO_CHECK_IF_POSSIBLE: u16 = 65523u16;
pub const IRP_MJ_MDL_READ: u16 = 65521u16;
pub const IRP_MJ_MDL_READ_COMPLETE: u16 = 65520u16;
pub const IRP_MJ_MDL_WRITE_COMPLETE: u16 = 65518u16;
pub const IRP_MJ_NETWORK_QUERY_OPEN: u16 = 65522u16;
pub const IRP_MJ_OPERATION_END: u16 = 128u16;
pub const IRP_MJ_PREPARE_MDL_WRITE: u16 = 65519u16;
pub const IRP_MJ_QUERY_OPEN: u16 = 65529u16;
pub const IRP_MJ_RELEASE_FOR_CC_FLUSH: u16 = 65530u16;
pub const IRP_MJ_RELEASE_FOR_MOD_WRITE: u16 = 65532u16;
pub const IRP_MJ_RELEASE_FOR_SECTION_SYNCHRONIZATION: u16 = 65534u16;
pub const IRP_MJ_VOLUME_DISMOUNT: u16 = 65516u16;
pub const IRP_MJ_VOLUME_MOUNT: u16 = 65517u16;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFLTOPLOCK_PREPOST_CALLBACKDATA_ROUTINE = Option<unsafe extern "system" fn(callbackdata: *const FLT_CALLBACK_DATA, context: *const core::ffi::c_void)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFLTOPLOCK_WAIT_COMPLETE_ROUTINE = Option<unsafe extern "system" fn(callbackdata: *const FLT_CALLBACK_DATA, context: *const core::ffi::c_void)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFLT_CALLBACK_DATA_QUEUE_ACQUIRE = Option<unsafe extern "system" fn(cbdq: *mut FLT_CALLBACK_DATA_QUEUE, irql: *mut u8)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFLT_CALLBACK_DATA_QUEUE_COMPLETE_CANCELED_IO = Option<unsafe extern "system" fn(cbdq: *mut FLT_CALLBACK_DATA_QUEUE, cbd: *mut FLT_CALLBACK_DATA)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFLT_CALLBACK_DATA_QUEUE_INSERT_IO = Option<unsafe extern "system" fn(cbdq: *mut FLT_CALLBACK_DATA_QUEUE, cbd: *mut FLT_CALLBACK_DATA, insertcontext: *mut core::ffi::c_void) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFLT_CALLBACK_DATA_QUEUE_PEEK_NEXT_IO = Option<unsafe extern "system" fn(cbdq: *mut FLT_CALLBACK_DATA_QUEUE, cbd: *mut FLT_CALLBACK_DATA, peekcontext: *mut core::ffi::c_void) -> *mut FLT_CALLBACK_DATA>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFLT_CALLBACK_DATA_QUEUE_RELEASE = Option<unsafe extern "system" fn(cbdq: *mut FLT_CALLBACK_DATA_QUEUE, irql: u8)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFLT_CALLBACK_DATA_QUEUE_REMOVE_IO = Option<unsafe extern "system" fn(cbdq: *mut FLT_CALLBACK_DATA_QUEUE, cbd: *mut FLT_CALLBACK_DATA)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFLT_COMPLETED_ASYNC_IO_CALLBACK = Option<unsafe extern "system" fn(callbackdata: *mut FLT_CALLBACK_DATA, context: PFLT_CONTEXT)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFLT_COMPLETE_CANCELED_CALLBACK = Option<unsafe extern "system" fn(callbackdata: *mut FLT_CALLBACK_DATA)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFLT_COMPLETE_LOCK_CALLBACK_DATA_ROUTINE = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, callbackdata: *mut FLT_CALLBACK_DATA) -> windows_core::NTSTATUS>;
pub type PFLT_CONNECT_NOTIFY = Option<unsafe extern "system" fn(clientport: PFLT_PORT, serverportcookie: *mut core::ffi::c_void, connectioncontext: *mut core::ffi::c_void, sizeofcontext: u32, connectionportcookie: *mut *mut core::ffi::c_void) -> windows_core::NTSTATUS>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFLT_CONTEXT(pub *mut core::ffi::c_void);
impl PFLT_CONTEXT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFLT_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Wdk_Foundation")]
pub type PFLT_CONTEXT_ALLOCATE_CALLBACK = Option<unsafe extern "system" fn(pooltype: super::super::super::Foundation::POOL_TYPE, size: usize, contexttype: u16) -> *mut core::ffi::c_void>;
pub type PFLT_CONTEXT_CLEANUP_CALLBACK = Option<unsafe extern "system" fn(context: PFLT_CONTEXT, contexttype: u16)>;
pub type PFLT_CONTEXT_FREE_CALLBACK = Option<unsafe extern "system" fn(pool: *mut core::ffi::c_void, contexttype: u16)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PFLT_DEFERRED_IO_WORKITEM(pub isize);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFLT_DEFERRED_IO_WORKITEM_ROUTINE = Option<unsafe extern "system" fn(fltworkitem: PFLT_DEFERRED_IO_WORKITEM, callbackdata: *mut FLT_CALLBACK_DATA, context: *mut core::ffi::c_void)>;
pub type PFLT_DISCONNECT_NOTIFY = Option<unsafe extern "system" fn(connectioncookie: *mut core::ffi::c_void)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PFLT_FILTER(pub isize);
pub type PFLT_FILTER_UNLOAD_CALLBACK = Option<unsafe extern "system" fn(flags: u32) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFLT_GENERATE_FILE_NAME = Option<unsafe extern "system" fn(instance: PFLT_INSTANCE, fileobject: *mut super::super::super::Foundation::FILE_OBJECT, callbackdata: *mut FLT_CALLBACK_DATA, nameoptions: u32, cachefilenameinformation: *mut bool, filename: *mut FLT_NAME_CONTROL) -> windows_core::NTSTATUS>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PFLT_GENERIC_WORKITEM(pub isize);
pub type PFLT_GENERIC_WORKITEM_ROUTINE = Option<unsafe extern "system" fn(fltworkitem: PFLT_GENERIC_WORKITEM, fltobject: *mut core::ffi::c_void, context: *mut core::ffi::c_void)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFLT_GET_OPERATION_STATUS_CALLBACK = Option<unsafe extern "system" fn(fltobjects: *mut FLT_RELATED_OBJECTS, iopbsnapshot: *mut FLT_IO_PARAMETER_BLOCK, operationstatus: windows_core::NTSTATUS, requestercontext: *mut core::ffi::c_void)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PFLT_INSTANCE(pub isize);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFLT_INSTANCE_QUERY_TEARDOWN_CALLBACK = Option<unsafe extern "system" fn(fltobjects: *const FLT_RELATED_OBJECTS, flags: u32) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_Storage_InstallableFileSystems", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFLT_INSTANCE_SETUP_CALLBACK = Option<unsafe extern "system" fn(fltobjects: *mut FLT_RELATED_OBJECTS, flags: u32, volumedevicetype: u32, volumefilesystemtype: super::super::super::super::Win32::Storage::InstallableFileSystems::FLT_FILESYSTEM_TYPE) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFLT_INSTANCE_TEARDOWN_CALLBACK = Option<unsafe extern "system" fn(fltobjects: *const FLT_RELATED_OBJECTS, reason: u32)>;
pub type PFLT_MESSAGE_NOTIFY = Option<unsafe extern "system" fn(portcookie: *mut core::ffi::c_void, inputbuffer: *mut core::ffi::c_void, inputbufferlength: u32, outputbuffer: *mut core::ffi::c_void, outputbufferlength: u32, returnoutputbufferlength: *mut u32) -> windows_core::NTSTATUS>;
pub type PFLT_NORMALIZE_CONTEXT_CLEANUP = Option<unsafe extern "system" fn(normalizationcontext: *mut *mut core::ffi::c_void)>;
pub type PFLT_NORMALIZE_NAME_COMPONENT = Option<unsafe extern "system" fn(instance: PFLT_INSTANCE, parentdirectory: *mut super::super::super::super::Win32::Foundation::UNICODE_STRING, volumenamelength: u16, component: *mut super::super::super::super::Win32::Foundation::UNICODE_STRING, expandcomponentname: *mut super::FILE_NAMES_INFORMATION, expandcomponentnamelength: u32, flags: u32, normalizationcontext: *mut *mut core::ffi::c_void) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFLT_NORMALIZE_NAME_COMPONENT_EX = Option<unsafe extern "system" fn(instance: PFLT_INSTANCE, fileobject: *mut super::super::super::Foundation::FILE_OBJECT, parentdirectory: *mut super::super::super::super::Win32::Foundation::UNICODE_STRING, volumenamelength: u16, component: *mut super::super::super::super::Win32::Foundation::UNICODE_STRING, expandcomponentname: *mut super::FILE_NAMES_INFORMATION, expandcomponentnamelength: u32, flags: u32, normalizationcontext: *mut *mut core::ffi::c_void) -> windows_core::NTSTATUS>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PFLT_PORT(pub isize);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFLT_POST_OPERATION_CALLBACK = Option<unsafe extern "system" fn(data: *mut FLT_CALLBACK_DATA, fltobjects: *mut FLT_RELATED_OBJECTS, completioncontext: *mut core::ffi::c_void, flags: u32) -> FLT_POSTOP_CALLBACK_STATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFLT_PRE_OPERATION_CALLBACK = Option<unsafe extern "system" fn(data: *mut FLT_CALLBACK_DATA, fltobjects: *const FLT_RELATED_OBJECTS, completioncontext: *mut *mut core::ffi::c_void) -> FLT_PREOP_CALLBACK_STATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFLT_SECTION_CONFLICT_NOTIFICATION_CALLBACK = Option<unsafe extern "system" fn(instance: PFLT_INSTANCE, sectioncontext: PFLT_CONTEXT, data: *mut FLT_CALLBACK_DATA) -> windows_core::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_System_SystemServices", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFLT_TRANSACTION_NOTIFICATION_CALLBACK = Option<unsafe extern "system" fn(fltobjects: *mut FLT_RELATED_OBJECTS, transactioncontext: PFLT_CONTEXT, notificationmask: u32) -> windows_core::NTSTATUS>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PFLT_VOLUME(pub isize);
pub const VOL_PROP_FL_DAX_VOLUME: u32 = 1u32;
