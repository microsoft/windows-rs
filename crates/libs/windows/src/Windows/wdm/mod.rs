#[inline]
pub unsafe fn DbgPrint<P0>(format: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ntdll.dll" "C" fn DbgPrint(format : windows_core::PCSTR) -> u32);
    unsafe { DbgPrint(format.param().abi()) }
}
#[inline]
pub unsafe fn DbgPrintEx<P2>(componentid: u32, level: u32, format: P2) -> u32
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ntdll.dll" "C" fn DbgPrintEx(componentid : u32, level : u32, format : windows_core::PCSTR) -> u32);
    unsafe { DbgPrintEx(componentid, level, format.param().abi()) }
}
#[inline]
pub unsafe fn DbgPrintReturnControlC<P0>(format: P0) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ntdll.dll" "C" fn DbgPrintReturnControlC(format : windows_core::PCSTR) -> u32);
    unsafe { DbgPrintReturnControlC(format.param().abi()) }
}
#[cfg(feature = "bcrypt")]
#[inline]
pub unsafe fn DbgQueryDebugFilterState(componentid: u32, level: u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn DbgQueryDebugFilterState(componentid : u32, level : u32) -> super::bcrypt::NTSTATUS);
    unsafe { DbgQueryDebugFilterState(componentid, level) }
}
#[cfg(feature = "bcrypt")]
#[inline]
pub unsafe fn DbgSetDebugFilterState(componentid: u32, level: u32, state: bool) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn DbgSetDebugFilterState(componentid : u32, level : u32, state : bool) -> super::bcrypt::NTSTATUS);
    unsafe { DbgSetDebugFilterState(componentid, level, state) }
}
#[cfg(feature = "evntprov")]
#[inline]
pub unsafe fn EtwEventEnabled(reghandle: super::evntprov::REGHANDLE, eventdescriptor: *const super::evntprov::EVENT_DESCRIPTOR) -> bool {
    windows_core::link!("ntdll.dll" "C" fn EtwEventEnabled(reghandle : super::evntprov::REGHANDLE, eventdescriptor : *const super::evntprov::EVENT_DESCRIPTOR) -> bool);
    unsafe { EtwEventEnabled(reghandle, eventdescriptor) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn NtCommitComplete(enlistmenthandle: super::winnt::HANDLE, tmvirtualclock: Option<*const i64>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtCommitComplete(enlistmenthandle : super::winnt::HANDLE, tmvirtualclock : *const i64) -> super::bcrypt::NTSTATUS);
    unsafe { NtCommitComplete(enlistmenthandle, tmvirtualclock.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn NtCommitEnlistment(enlistmenthandle: super::winnt::HANDLE, tmvirtualclock: Option<*const i64>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtCommitEnlistment(enlistmenthandle : super::winnt::HANDLE, tmvirtualclock : *const i64) -> super::bcrypt::NTSTATUS);
    unsafe { NtCommitEnlistment(enlistmenthandle, tmvirtualclock.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn NtCommitTransaction(transactionhandle: super::winnt::HANDLE, wait: bool) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtCommitTransaction(transactionhandle : super::winnt::HANDLE, wait : bool) -> super::bcrypt::NTSTATUS);
    unsafe { NtCommitTransaction(transactionhandle, wait) }
}
#[cfg(all(feature = "bcrypt", feature = "d3dkmthk", feature = "ktmtypes", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn NtCreateEnlistment(enlistmenthandle: *mut super::winnt::HANDLE, desiredaccess: super::winnt::ACCESS_MASK, resourcemanagerhandle: super::winnt::HANDLE, transactionhandle: super::winnt::HANDLE, objectattributes: Option<*const super::d3dkmthk::OBJECT_ATTRIBUTES>, createoptions: Option<u32>, notificationmask: super::ktmtypes::NOTIFICATION_MASK, enlistmentkey: Option<*const core::ffi::c_void>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtCreateEnlistment(enlistmenthandle : *mut super::winnt::HANDLE, desiredaccess : super::winnt::ACCESS_MASK, resourcemanagerhandle : super::winnt::HANDLE, transactionhandle : super::winnt::HANDLE, objectattributes : *const super::d3dkmthk::OBJECT_ATTRIBUTES, createoptions : u32, notificationmask : super::ktmtypes::NOTIFICATION_MASK, enlistmentkey : *const core::ffi::c_void) -> super::bcrypt::NTSTATUS);
    unsafe { NtCreateEnlistment(enlistmenthandle as _, desiredaccess, resourcemanagerhandle, transactionhandle, objectattributes.unwrap_or(core::mem::zeroed()) as _, createoptions.unwrap_or(core::mem::zeroed()) as _, notificationmask, enlistmentkey.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn NtCreateResourceManager(resourcemanagerhandle: *mut super::winnt::HANDLE, desiredaccess: super::winnt::ACCESS_MASK, tmhandle: super::winnt::HANDLE, rmguid: *const windows_core::GUID, objectattributes: Option<*const super::d3dkmthk::OBJECT_ATTRIBUTES>, createoptions: Option<u32>, description: Option<*const super::ntsecapi::UNICODE_STRING>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtCreateResourceManager(resourcemanagerhandle : *mut super::winnt::HANDLE, desiredaccess : super::winnt::ACCESS_MASK, tmhandle : super::winnt::HANDLE, rmguid : *const windows_core::GUID, objectattributes : *const super::d3dkmthk::OBJECT_ATTRIBUTES, createoptions : u32, description : *const super::ntsecapi::UNICODE_STRING) -> super::bcrypt::NTSTATUS);
    unsafe { NtCreateResourceManager(resourcemanagerhandle as _, desiredaccess, tmhandle, rmguid, objectattributes.unwrap_or(core::mem::zeroed()) as _, createoptions.unwrap_or(core::mem::zeroed()) as _, description.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn NtCreateTransaction(transactionhandle: *mut super::winnt::HANDLE, desiredaccess: super::winnt::ACCESS_MASK, objectattributes: Option<*const super::d3dkmthk::OBJECT_ATTRIBUTES>, uow: Option<*const windows_core::GUID>, tmhandle: Option<super::winnt::HANDLE>, createoptions: Option<u32>, isolationlevel: Option<u32>, isolationflags: Option<u32>, timeout: Option<*const i64>, description: Option<*const super::ntsecapi::UNICODE_STRING>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtCreateTransaction(transactionhandle : *mut super::winnt::HANDLE, desiredaccess : super::winnt::ACCESS_MASK, objectattributes : *const super::d3dkmthk::OBJECT_ATTRIBUTES, uow : *const windows_core::GUID, tmhandle : super::winnt::HANDLE, createoptions : u32, isolationlevel : u32, isolationflags : u32, timeout : *const i64, description : *const super::ntsecapi::UNICODE_STRING) -> super::bcrypt::NTSTATUS);
    unsafe { NtCreateTransaction(transactionhandle as _, desiredaccess, objectattributes.unwrap_or(core::mem::zeroed()) as _, uow.unwrap_or(core::mem::zeroed()) as _, tmhandle.unwrap_or(core::mem::zeroed()) as _, createoptions.unwrap_or(core::mem::zeroed()) as _, isolationlevel.unwrap_or(core::mem::zeroed()) as _, isolationflags.unwrap_or(core::mem::zeroed()) as _, timeout.unwrap_or(core::mem::zeroed()) as _, description.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn NtCreateTransactionManager(tmhandle: *mut super::winnt::HANDLE, desiredaccess: super::winnt::ACCESS_MASK, objectattributes: Option<*const super::d3dkmthk::OBJECT_ATTRIBUTES>, logfilename: Option<*const super::ntsecapi::UNICODE_STRING>, createoptions: Option<u32>, commitstrength: Option<u32>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtCreateTransactionManager(tmhandle : *mut super::winnt::HANDLE, desiredaccess : super::winnt::ACCESS_MASK, objectattributes : *const super::d3dkmthk::OBJECT_ATTRIBUTES, logfilename : *const super::ntsecapi::UNICODE_STRING, createoptions : u32, commitstrength : u32) -> super::bcrypt::NTSTATUS);
    unsafe { NtCreateTransactionManager(tmhandle as _, desiredaccess, objectattributes.unwrap_or(core::mem::zeroed()) as _, logfilename.unwrap_or(core::mem::zeroed()) as _, createoptions.unwrap_or(core::mem::zeroed()) as _, commitstrength.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn NtEnumerateTransactionObject(rootobjecthandle: Option<super::winnt::HANDLE>, querytype: super::winnt::KTMOBJECT_TYPE, objectcursor: *mut super::winnt::KTMOBJECT_CURSOR, objectcursorlength: u32, returnlength: *mut u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtEnumerateTransactionObject(rootobjecthandle : super::winnt::HANDLE, querytype : super::winnt::KTMOBJECT_TYPE, objectcursor : *mut super::winnt::KTMOBJECT_CURSOR, objectcursorlength : u32, returnlength : *mut u32) -> super::bcrypt::NTSTATUS);
    unsafe { NtEnumerateTransactionObject(rootobjecthandle.unwrap_or(core::mem::zeroed()) as _, querytype, objectcursor as _, objectcursorlength, returnlength as _) }
}
#[cfg(all(feature = "bcrypt", feature = "ktmtypes", feature = "winnt"))]
#[inline]
pub unsafe fn NtGetNotificationResourceManager(resourcemanagerhandle: super::winnt::HANDLE, transactionnotification: *mut super::ktmtypes::TRANSACTION_NOTIFICATION, notificationlength: u32, timeout: Option<*const i64>, returnlength: Option<*mut u32>, asynchronous: u32, asynchronouscontext: Option<usize>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtGetNotificationResourceManager(resourcemanagerhandle : super::winnt::HANDLE, transactionnotification : *mut super::ktmtypes::TRANSACTION_NOTIFICATION, notificationlength : u32, timeout : *const i64, returnlength : *mut u32, asynchronous : u32, asynchronouscontext : usize) -> super::bcrypt::NTSTATUS);
    unsafe { NtGetNotificationResourceManager(resourcemanagerhandle, transactionnotification as _, notificationlength, timeout.unwrap_or(core::mem::zeroed()) as _, returnlength.unwrap_or(core::mem::zeroed()) as _, asynchronous, asynchronouscontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn NtManagePartition(targethandle: super::winnt::HANDLE, sourcehandle: Option<super::winnt::HANDLE>, partitioninformationclass: PARTITION_INFORMATION_CLASS, partitioninformation: *mut core::ffi::c_void, partitioninformationlength: u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtManagePartition(targethandle : super::winnt::HANDLE, sourcehandle : super::winnt::HANDLE, partitioninformationclass : PARTITION_INFORMATION_CLASS, partitioninformation : *mut core::ffi::c_void, partitioninformationlength : u32) -> super::bcrypt::NTSTATUS);
    unsafe { NtManagePartition(targethandle, sourcehandle.unwrap_or(core::mem::zeroed()) as _, partitioninformationclass, partitioninformation as _, partitioninformationlength) }
}
#[cfg(all(feature = "bcrypt", feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn NtOpenEnlistment(enlistmenthandle: *mut super::winnt::HANDLE, desiredaccess: super::winnt::ACCESS_MASK, resourcemanagerhandle: super::winnt::HANDLE, enlistmentguid: *const windows_core::GUID, objectattributes: Option<*const super::d3dkmthk::OBJECT_ATTRIBUTES>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtOpenEnlistment(enlistmenthandle : *mut super::winnt::HANDLE, desiredaccess : super::winnt::ACCESS_MASK, resourcemanagerhandle : super::winnt::HANDLE, enlistmentguid : *const windows_core::GUID, objectattributes : *const super::d3dkmthk::OBJECT_ATTRIBUTES) -> super::bcrypt::NTSTATUS);
    unsafe { NtOpenEnlistment(enlistmenthandle as _, desiredaccess, resourcemanagerhandle, enlistmentguid, objectattributes.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn NtOpenRegistryTransaction(transactionhandle: *mut super::winnt::HANDLE, desiredaccess: super::winnt::ACCESS_MASK, objectattributes: *const super::d3dkmthk::OBJECT_ATTRIBUTES) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtOpenRegistryTransaction(transactionhandle : *mut super::winnt::HANDLE, desiredaccess : super::winnt::ACCESS_MASK, objectattributes : *const super::d3dkmthk::OBJECT_ATTRIBUTES) -> super::bcrypt::NTSTATUS);
    unsafe { NtOpenRegistryTransaction(transactionhandle as _, desiredaccess, objectattributes) }
}
#[cfg(all(feature = "bcrypt", feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn NtOpenResourceManager(resourcemanagerhandle: *mut super::winnt::HANDLE, desiredaccess: super::winnt::ACCESS_MASK, tmhandle: super::winnt::HANDLE, resourcemanagerguid: Option<*const windows_core::GUID>, objectattributes: Option<*const super::d3dkmthk::OBJECT_ATTRIBUTES>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtOpenResourceManager(resourcemanagerhandle : *mut super::winnt::HANDLE, desiredaccess : super::winnt::ACCESS_MASK, tmhandle : super::winnt::HANDLE, resourcemanagerguid : *const windows_core::GUID, objectattributes : *const super::d3dkmthk::OBJECT_ATTRIBUTES) -> super::bcrypt::NTSTATUS);
    unsafe { NtOpenResourceManager(resourcemanagerhandle as _, desiredaccess, tmhandle, resourcemanagerguid.unwrap_or(core::mem::zeroed()) as _, objectattributes.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn NtOpenTransaction(transactionhandle: *mut super::winnt::HANDLE, desiredaccess: super::winnt::ACCESS_MASK, objectattributes: *const super::d3dkmthk::OBJECT_ATTRIBUTES, uow: *const windows_core::GUID, tmhandle: Option<super::winnt::HANDLE>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtOpenTransaction(transactionhandle : *mut super::winnt::HANDLE, desiredaccess : super::winnt::ACCESS_MASK, objectattributes : *const super::d3dkmthk::OBJECT_ATTRIBUTES, uow : *const windows_core::GUID, tmhandle : super::winnt::HANDLE) -> super::bcrypt::NTSTATUS);
    unsafe { NtOpenTransaction(transactionhandle as _, desiredaccess, objectattributes, uow, tmhandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn NtOpenTransactionManager(tmhandle: *mut super::winnt::HANDLE, desiredaccess: super::winnt::ACCESS_MASK, objectattributes: Option<*const super::d3dkmthk::OBJECT_ATTRIBUTES>, logfilename: Option<*const super::ntsecapi::UNICODE_STRING>, tmidentity: Option<*const windows_core::GUID>, openoptions: Option<u32>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtOpenTransactionManager(tmhandle : *mut super::winnt::HANDLE, desiredaccess : super::winnt::ACCESS_MASK, objectattributes : *const super::d3dkmthk::OBJECT_ATTRIBUTES, logfilename : *const super::ntsecapi::UNICODE_STRING, tmidentity : *const windows_core::GUID, openoptions : u32) -> super::bcrypt::NTSTATUS);
    unsafe { NtOpenTransactionManager(tmhandle as _, desiredaccess, objectattributes.unwrap_or(core::mem::zeroed()) as _, logfilename.unwrap_or(core::mem::zeroed()) as _, tmidentity.unwrap_or(core::mem::zeroed()) as _, openoptions.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn NtPowerInformation(informationlevel: super::winnt::POWER_INFORMATION_LEVEL, inputbuffer: Option<*const core::ffi::c_void>, inputbufferlength: u32, outputbuffer: Option<*mut core::ffi::c_void>, outputbufferlength: u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtPowerInformation(informationlevel : super::winnt::POWER_INFORMATION_LEVEL, inputbuffer : *const core::ffi::c_void, inputbufferlength : u32, outputbuffer : *mut core::ffi::c_void, outputbufferlength : u32) -> super::bcrypt::NTSTATUS);
    unsafe { NtPowerInformation(informationlevel, inputbuffer.unwrap_or(core::mem::zeroed()) as _, inputbufferlength, outputbuffer.unwrap_or(core::mem::zeroed()) as _, outputbufferlength) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn NtPrePrepareComplete(enlistmenthandle: super::winnt::HANDLE, tmvirtualclock: Option<*const i64>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtPrePrepareComplete(enlistmenthandle : super::winnt::HANDLE, tmvirtualclock : *const i64) -> super::bcrypt::NTSTATUS);
    unsafe { NtPrePrepareComplete(enlistmenthandle, tmvirtualclock.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn NtPrePrepareEnlistment(enlistmenthandle: super::winnt::HANDLE, tmvirtualclock: Option<*const i64>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtPrePrepareEnlistment(enlistmenthandle : super::winnt::HANDLE, tmvirtualclock : *const i64) -> super::bcrypt::NTSTATUS);
    unsafe { NtPrePrepareEnlistment(enlistmenthandle, tmvirtualclock.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn NtPrepareComplete(enlistmenthandle: super::winnt::HANDLE, tmvirtualclock: Option<*const i64>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtPrepareComplete(enlistmenthandle : super::winnt::HANDLE, tmvirtualclock : *const i64) -> super::bcrypt::NTSTATUS);
    unsafe { NtPrepareComplete(enlistmenthandle, tmvirtualclock.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn NtPrepareEnlistment(enlistmenthandle: super::winnt::HANDLE, tmvirtualclock: Option<*const i64>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtPrepareEnlistment(enlistmenthandle : super::winnt::HANDLE, tmvirtualclock : *const i64) -> super::bcrypt::NTSTATUS);
    unsafe { NtPrepareEnlistment(enlistmenthandle, tmvirtualclock.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn NtPropagationComplete(resourcemanagerhandle: super::winnt::HANDLE, requestcookie: u32, bufferlength: u32, buffer: *const core::ffi::c_void) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtPropagationComplete(resourcemanagerhandle : super::winnt::HANDLE, requestcookie : u32, bufferlength : u32, buffer : *const core::ffi::c_void) -> super::bcrypt::NTSTATUS);
    unsafe { NtPropagationComplete(resourcemanagerhandle, requestcookie, bufferlength, buffer) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn NtPropagationFailed(resourcemanagerhandle: super::winnt::HANDLE, requestcookie: u32, propstatus: super::bcrypt::NTSTATUS) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtPropagationFailed(resourcemanagerhandle : super::winnt::HANDLE, requestcookie : u32, propstatus : super::bcrypt::NTSTATUS) -> super::bcrypt::NTSTATUS);
    unsafe { NtPropagationFailed(resourcemanagerhandle, requestcookie, propstatus) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn NtQueryInformationEnlistment(enlistmenthandle: super::winnt::HANDLE, enlistmentinformationclass: super::winnt::ENLISTMENT_INFORMATION_CLASS, enlistmentinformation: *mut core::ffi::c_void, enlistmentinformationlength: u32, returnlength: *mut u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtQueryInformationEnlistment(enlistmenthandle : super::winnt::HANDLE, enlistmentinformationclass : super::winnt::ENLISTMENT_INFORMATION_CLASS, enlistmentinformation : *mut core::ffi::c_void, enlistmentinformationlength : u32, returnlength : *mut u32) -> super::bcrypt::NTSTATUS);
    unsafe { NtQueryInformationEnlistment(enlistmenthandle, enlistmentinformationclass, enlistmentinformation as _, enlistmentinformationlength, returnlength as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn NtQueryInformationResourceManager(resourcemanagerhandle: super::winnt::HANDLE, resourcemanagerinformationclass: super::winnt::RESOURCEMANAGER_INFORMATION_CLASS, resourcemanagerinformation: *mut core::ffi::c_void, resourcemanagerinformationlength: u32, returnlength: Option<*mut u32>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtQueryInformationResourceManager(resourcemanagerhandle : super::winnt::HANDLE, resourcemanagerinformationclass : super::winnt::RESOURCEMANAGER_INFORMATION_CLASS, resourcemanagerinformation : *mut core::ffi::c_void, resourcemanagerinformationlength : u32, returnlength : *mut u32) -> super::bcrypt::NTSTATUS);
    unsafe { NtQueryInformationResourceManager(resourcemanagerhandle, resourcemanagerinformationclass, resourcemanagerinformation as _, resourcemanagerinformationlength, returnlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn NtQueryInformationTransaction(transactionhandle: super::winnt::HANDLE, transactioninformationclass: super::winnt::TRANSACTION_INFORMATION_CLASS, transactioninformation: *mut core::ffi::c_void, transactioninformationlength: u32, returnlength: Option<*mut u32>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtQueryInformationTransaction(transactionhandle : super::winnt::HANDLE, transactioninformationclass : super::winnt::TRANSACTION_INFORMATION_CLASS, transactioninformation : *mut core::ffi::c_void, transactioninformationlength : u32, returnlength : *mut u32) -> super::bcrypt::NTSTATUS);
    unsafe { NtQueryInformationTransaction(transactionhandle, transactioninformationclass, transactioninformation as _, transactioninformationlength, returnlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn NtQueryInformationTransactionManager(transactionmanagerhandle: super::winnt::HANDLE, transactionmanagerinformationclass: super::winnt::TRANSACTIONMANAGER_INFORMATION_CLASS, transactionmanagerinformation: *mut core::ffi::c_void, transactionmanagerinformationlength: u32, returnlength: *mut u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtQueryInformationTransactionManager(transactionmanagerhandle : super::winnt::HANDLE, transactionmanagerinformationclass : super::winnt::TRANSACTIONMANAGER_INFORMATION_CLASS, transactionmanagerinformation : *mut core::ffi::c_void, transactionmanagerinformationlength : u32, returnlength : *mut u32) -> super::bcrypt::NTSTATUS);
    unsafe { NtQueryInformationTransactionManager(transactionmanagerhandle, transactionmanagerinformationclass, transactionmanagerinformation as _, transactionmanagerinformationlength, returnlength as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn NtReadOnlyEnlistment(enlistmenthandle: super::winnt::HANDLE, tmvirtualclock: Option<*const i64>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtReadOnlyEnlistment(enlistmenthandle : super::winnt::HANDLE, tmvirtualclock : *const i64) -> super::bcrypt::NTSTATUS);
    unsafe { NtReadOnlyEnlistment(enlistmenthandle, tmvirtualclock.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn NtRecoverEnlistment(enlistmenthandle: super::winnt::HANDLE, enlistmentkey: Option<*const core::ffi::c_void>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtRecoverEnlistment(enlistmenthandle : super::winnt::HANDLE, enlistmentkey : *const core::ffi::c_void) -> super::bcrypt::NTSTATUS);
    unsafe { NtRecoverEnlistment(enlistmenthandle, enlistmentkey.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn NtRecoverResourceManager(resourcemanagerhandle: super::winnt::HANDLE) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtRecoverResourceManager(resourcemanagerhandle : super::winnt::HANDLE) -> super::bcrypt::NTSTATUS);
    unsafe { NtRecoverResourceManager(resourcemanagerhandle) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn NtRecoverTransactionManager(transactionmanagerhandle: super::winnt::HANDLE) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtRecoverTransactionManager(transactionmanagerhandle : super::winnt::HANDLE) -> super::bcrypt::NTSTATUS);
    unsafe { NtRecoverTransactionManager(transactionmanagerhandle) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn NtRegisterProtocolAddressInformation(resourcemanager: super::winnt::HANDLE, protocolid: *const windows_core::GUID, protocolinformationsize: u32, protocolinformation: *const core::ffi::c_void, createoptions: Option<u32>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtRegisterProtocolAddressInformation(resourcemanager : super::winnt::HANDLE, protocolid : *const windows_core::GUID, protocolinformationsize : u32, protocolinformation : *const core::ffi::c_void, createoptions : u32) -> super::bcrypt::NTSTATUS);
    unsafe { NtRegisterProtocolAddressInformation(resourcemanager, protocolid, protocolinformationsize, protocolinformation, createoptions.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi"))]
#[inline]
pub unsafe fn NtRenameTransactionManager(logfilename: *const super::ntsecapi::UNICODE_STRING, existingtransactionmanagerguid: *const windows_core::GUID) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtRenameTransactionManager(logfilename : *const super::ntsecapi::UNICODE_STRING, existingtransactionmanagerguid : *const windows_core::GUID) -> super::bcrypt::NTSTATUS);
    unsafe { NtRenameTransactionManager(logfilename, existingtransactionmanagerguid) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn NtRollbackComplete(enlistmenthandle: super::winnt::HANDLE, tmvirtualclock: Option<*const i64>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtRollbackComplete(enlistmenthandle : super::winnt::HANDLE, tmvirtualclock : *const i64) -> super::bcrypt::NTSTATUS);
    unsafe { NtRollbackComplete(enlistmenthandle, tmvirtualclock.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn NtRollbackEnlistment(enlistmenthandle: super::winnt::HANDLE, tmvirtualclock: Option<*const i64>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtRollbackEnlistment(enlistmenthandle : super::winnt::HANDLE, tmvirtualclock : *const i64) -> super::bcrypt::NTSTATUS);
    unsafe { NtRollbackEnlistment(enlistmenthandle, tmvirtualclock.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn NtRollbackRegistryTransaction(transactionhandle: super::winnt::HANDLE, flags: u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtRollbackRegistryTransaction(transactionhandle : super::winnt::HANDLE, flags : u32) -> super::bcrypt::NTSTATUS);
    unsafe { NtRollbackRegistryTransaction(transactionhandle, flags) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn NtRollbackTransaction(transactionhandle: super::winnt::HANDLE, wait: bool) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtRollbackTransaction(transactionhandle : super::winnt::HANDLE, wait : bool) -> super::bcrypt::NTSTATUS);
    unsafe { NtRollbackTransaction(transactionhandle, wait) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn NtRollforwardTransactionManager(transactionmanagerhandle: super::winnt::HANDLE, tmvirtualclock: Option<*const i64>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtRollforwardTransactionManager(transactionmanagerhandle : super::winnt::HANDLE, tmvirtualclock : *const i64) -> super::bcrypt::NTSTATUS);
    unsafe { NtRollforwardTransactionManager(transactionmanagerhandle, tmvirtualclock.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn NtSetInformationEnlistment(enlistmenthandle: Option<super::winnt::HANDLE>, enlistmentinformationclass: super::winnt::ENLISTMENT_INFORMATION_CLASS, enlistmentinformation: *const core::ffi::c_void, enlistmentinformationlength: u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtSetInformationEnlistment(enlistmenthandle : super::winnt::HANDLE, enlistmentinformationclass : super::winnt::ENLISTMENT_INFORMATION_CLASS, enlistmentinformation : *const core::ffi::c_void, enlistmentinformationlength : u32) -> super::bcrypt::NTSTATUS);
    unsafe { NtSetInformationEnlistment(enlistmenthandle.unwrap_or(core::mem::zeroed()) as _, enlistmentinformationclass, enlistmentinformation, enlistmentinformationlength) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn NtSetInformationResourceManager(resourcemanagerhandle: super::winnt::HANDLE, resourcemanagerinformationclass: super::winnt::RESOURCEMANAGER_INFORMATION_CLASS, resourcemanagerinformation: *const core::ffi::c_void, resourcemanagerinformationlength: u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtSetInformationResourceManager(resourcemanagerhandle : super::winnt::HANDLE, resourcemanagerinformationclass : super::winnt::RESOURCEMANAGER_INFORMATION_CLASS, resourcemanagerinformation : *const core::ffi::c_void, resourcemanagerinformationlength : u32) -> super::bcrypt::NTSTATUS);
    unsafe { NtSetInformationResourceManager(resourcemanagerhandle, resourcemanagerinformationclass, resourcemanagerinformation, resourcemanagerinformationlength) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn NtSetInformationTransaction(transactionhandle: super::winnt::HANDLE, transactioninformationclass: super::winnt::TRANSACTION_INFORMATION_CLASS, transactioninformation: *const core::ffi::c_void, transactioninformationlength: u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtSetInformationTransaction(transactionhandle : super::winnt::HANDLE, transactioninformationclass : super::winnt::TRANSACTION_INFORMATION_CLASS, transactioninformation : *const core::ffi::c_void, transactioninformationlength : u32) -> super::bcrypt::NTSTATUS);
    unsafe { NtSetInformationTransaction(transactionhandle, transactioninformationclass, transactioninformation, transactioninformationlength) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn NtSetInformationTransactionManager(tmhandle: Option<super::winnt::HANDLE>, transactionmanagerinformationclass: super::winnt::TRANSACTIONMANAGER_INFORMATION_CLASS, transactionmanagerinformation: *const core::ffi::c_void, transactionmanagerinformationlength: u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtSetInformationTransactionManager(tmhandle : super::winnt::HANDLE, transactionmanagerinformationclass : super::winnt::TRANSACTIONMANAGER_INFORMATION_CLASS, transactionmanagerinformation : *const core::ffi::c_void, transactionmanagerinformationlength : u32) -> super::bcrypt::NTSTATUS);
    unsafe { NtSetInformationTransactionManager(tmhandle.unwrap_or(core::mem::zeroed()) as _, transactionmanagerinformationclass, transactionmanagerinformation, transactionmanagerinformationlength) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn NtSinglePhaseReject(enlistmenthandle: super::winnt::HANDLE, tmvirtualclock: Option<*const i64>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtSinglePhaseReject(enlistmenthandle : super::winnt::HANDLE, tmvirtualclock : *const i64) -> super::bcrypt::NTSTATUS);
    unsafe { NtSinglePhaseReject(enlistmenthandle, tmvirtualclock.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi"))]
#[inline]
pub unsafe fn RtlAppendUnicodeStringToString(destination: *mut super::ntsecapi::UNICODE_STRING, source: *const super::ntsecapi::UNICODE_STRING) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlAppendUnicodeStringToString(destination : *mut super::ntsecapi::UNICODE_STRING, source : *const super::ntsecapi::UNICODE_STRING) -> super::bcrypt::NTSTATUS);
    unsafe { RtlAppendUnicodeStringToString(destination as _, source) }
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi"))]
#[inline]
pub unsafe fn RtlAppendUnicodeToString<P1>(destination: *mut super::ntsecapi::UNICODE_STRING, source: P1) -> super::bcrypt::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdll.dll" "system" fn RtlAppendUnicodeToString(destination : *mut super::ntsecapi::UNICODE_STRING, source : windows_core::PCWSTR) -> super::bcrypt::NTSTATUS);
    unsafe { RtlAppendUnicodeToString(destination as _, source.param().abi()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn RtlAreBitsClear(bitmapheader: *const RTL_BITMAP, startingindex: u32, length: u32) -> bool {
    windows_core::link!("ntdll.dll" "system" fn RtlAreBitsClear(bitmapheader : *const RTL_BITMAP, startingindex : u32, length : u32) -> bool);
    unsafe { RtlAreBitsClear(bitmapheader, startingindex, length) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn RtlAreBitsSet(bitmapheader: *const RTL_BITMAP, startingindex: u32, length: u32) -> bool {
    windows_core::link!("ntdll.dll" "system" fn RtlAreBitsSet(bitmapheader : *const RTL_BITMAP, startingindex : u32, length : u32) -> bool);
    unsafe { RtlAreBitsSet(bitmapheader, startingindex, length) }
}
#[inline]
pub unsafe fn RtlAssert<P3>(voidfailedassertion: *const core::ffi::c_void, voidfilename: *const core::ffi::c_void, linenumber: u32, mutablemessage: P3)
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ntdll.dll" "system" fn RtlAssert(voidfailedassertion : *const core::ffi::c_void, voidfilename : *const core::ffi::c_void, linenumber : u32, mutablemessage : windows_core::PCSTR));
    unsafe { RtlAssert(voidfailedassertion, voidfilename, linenumber, mutablemessage.param().abi()) }
}
#[cfg(feature = "bcrypt")]
#[inline]
pub unsafe fn RtlCheckRegistryKey<P1>(relativeto: u32, path: P1) -> super::bcrypt::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdll.dll" "system" fn RtlCheckRegistryKey(relativeto : u32, path : windows_core::PCWSTR) -> super::bcrypt::NTSTATUS);
    unsafe { RtlCheckRegistryKey(relativeto, path.param().abi()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn RtlClearAllBits(bitmapheader: *const RTL_BITMAP) {
    windows_core::link!("ntdll.dll" "system" fn RtlClearAllBits(bitmapheader : *const RTL_BITMAP));
    unsafe { RtlClearAllBits(bitmapheader) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn RtlClearBit(bitmapheader: *const RTL_BITMAP, bitnumber: u32) {
    windows_core::link!("ntdll.dll" "system" fn RtlClearBit(bitmapheader : *const RTL_BITMAP, bitnumber : u32));
    unsafe { RtlClearBit(bitmapheader, bitnumber) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn RtlClearBits(bitmapheader: *const RTL_BITMAP, startingindex: u32, numbertoclear: u32) {
    windows_core::link!("ntdll.dll" "system" fn RtlClearBits(bitmapheader : *const RTL_BITMAP, startingindex : u32, numbertoclear : u32));
    unsafe { RtlClearBits(bitmapheader, startingindex, numbertoclear) }
}
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[inline]
pub unsafe fn RtlCmDecodeMemIoResource(descriptor: *const CM_PARTIAL_RESOURCE_DESCRIPTOR, start: Option<*mut u64>) -> u64 {
    windows_core::link!("ntdll.dll" "system" fn RtlCmDecodeMemIoResource(descriptor : *const CM_PARTIAL_RESOURCE_DESCRIPTOR, start : *mut u64) -> u64);
    unsafe { RtlCmDecodeMemIoResource(descriptor, start.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "usb"))]
#[inline]
pub unsafe fn RtlCmEncodeMemIoResource(descriptor: *const CM_PARTIAL_RESOURCE_DESCRIPTOR, r#type: u8, length: u64, start: u64) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlCmEncodeMemIoResource(descriptor : *const CM_PARTIAL_RESOURCE_DESCRIPTOR, r#type : u8, length : u64, start : u64) -> super::bcrypt::NTSTATUS);
    unsafe { RtlCmEncodeMemIoResource(descriptor, r#type, length, start) }
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
#[inline]
pub unsafe fn RtlCompareUnicodeString(string1: *const super::ntsecapi::UNICODE_STRING, string2: *const super::ntsecapi::UNICODE_STRING, caseinsensitive: bool) -> i32 {
    windows_core::link!("ntdll.dll" "system" fn RtlCompareUnicodeString(string1 : *const super::ntsecapi::UNICODE_STRING, string2 : *const super::ntsecapi::UNICODE_STRING, caseinsensitive : bool) -> i32);
    unsafe { RtlCompareUnicodeString(string1, string2, caseinsensitive) }
}
#[inline]
pub unsafe fn RtlCompareUnicodeStrings(string1: &[u16], string2: &[u16], caseinsensitive: bool) -> i32 {
    windows_core::link!("ntdll.dll" "system" fn RtlCompareUnicodeStrings(string1 : *const u16, string1length : usize, string2 : *const u16, string2length : usize, caseinsensitive : bool) -> i32);
    unsafe { RtlCompareUnicodeStrings(string1.as_ptr(), string1.len().try_into().unwrap(), string2.as_ptr(), string2.len().try_into().unwrap(), caseinsensitive) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn RtlCopyBitMap(source: *const RTL_BITMAP, destination: *const RTL_BITMAP, targetbit: u32) {
    windows_core::link!("ntdll.dll" "system" fn RtlCopyBitMap(source : *const RTL_BITMAP, destination : *const RTL_BITMAP, targetbit : u32));
    unsafe { RtlCopyBitMap(source, destination, targetbit) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn RtlCopyMemoryNonTemporal(destination: *mut core::ffi::c_void, source: *const core::ffi::c_void, length: usize) {
    windows_core::link!("ntdll.dll" "system" fn RtlCopyMemoryNonTemporal(destination : *mut core::ffi::c_void, source : *const core::ffi::c_void, length : usize));
    unsafe { RtlCopyMemoryNonTemporal(destination as _, source, length) }
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
#[inline]
pub unsafe fn RtlCopyUnicodeString(destinationstring: *mut super::ntsecapi::UNICODE_STRING, sourcestring: Option<*const super::ntsecapi::UNICODE_STRING>) {
    windows_core::link!("ntdll.dll" "system" fn RtlCopyUnicodeString(destinationstring : *mut super::ntsecapi::UNICODE_STRING, sourcestring : *const super::ntsecapi::UNICODE_STRING));
    unsafe { RtlCopyUnicodeString(destinationstring as _, sourcestring.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "bcrypt")]
#[inline]
pub unsafe fn RtlCreateRegistryKey<P1>(relativeto: u32, path: P1) -> super::bcrypt::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdll.dll" "system" fn RtlCreateRegistryKey(relativeto : u32, path : windows_core::PCWSTR) -> super::bcrypt::NTSTATUS);
    unsafe { RtlCreateRegistryKey(relativeto, path.param().abi()) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn RtlCreateSecurityDescriptor(securitydescriptor: super::winnt::PSECURITY_DESCRIPTOR, revision: u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlCreateSecurityDescriptor(securitydescriptor : super::winnt::PSECURITY_DESCRIPTOR, revision : u32) -> super::bcrypt::NTSTATUS);
    unsafe { RtlCreateSecurityDescriptor(securitydescriptor as _, revision) }
}
#[cfg(feature = "bcrypt")]
#[inline]
pub unsafe fn RtlDeleteRegistryValue<P1, P2>(relativeto: u32, path: P1, valuename: P2) -> super::bcrypt::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdll.dll" "system" fn RtlDeleteRegistryValue(relativeto : u32, path : windows_core::PCWSTR, valuename : windows_core::PCWSTR) -> super::bcrypt::NTSTATUS);
    unsafe { RtlDeleteRegistryValue(relativeto, path.param().abi(), valuename.param().abi()) }
}
#[inline]
pub unsafe fn RtlDowncaseUnicodeChar(sourcecharacter: u16) -> u16 {
    windows_core::link!("ntdll.dll" "system" fn RtlDowncaseUnicodeChar(sourcecharacter : u16) -> u16);
    unsafe { RtlDowncaseUnicodeChar(sourcecharacter) }
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
#[inline]
pub unsafe fn RtlEqualUnicodeString(string1: *const super::ntsecapi::UNICODE_STRING, string2: *const super::ntsecapi::UNICODE_STRING, caseinsensitive: bool) -> bool {
    windows_core::link!("ntdll.dll" "system" fn RtlEqualUnicodeString(string1 : *const super::ntsecapi::UNICODE_STRING, string2 : *const super::ntsecapi::UNICODE_STRING, caseinsensitive : bool) -> bool);
    unsafe { RtlEqualUnicodeString(string1, string2, caseinsensitive) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn RtlExtractBitMap(source: *const RTL_BITMAP, destination: *const RTL_BITMAP, targetbit: u32, numberofbits: u32) {
    windows_core::link!("ntdll.dll" "system" fn RtlExtractBitMap(source : *const RTL_BITMAP, destination : *const RTL_BITMAP, targetbit : u32, numberofbits : u32));
    unsafe { RtlExtractBitMap(source, destination, targetbit, numberofbits) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn RtlFillMemoryNonTemporal(destination: *mut core::ffi::c_void, length: usize, value: u8) {
    windows_core::link!("ntdll.dll" "system" fn RtlFillMemoryNonTemporal(destination : *mut core::ffi::c_void, length : usize, value : u8));
    unsafe { RtlFillMemoryNonTemporal(destination as _, length, value) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn RtlFindClearBits(bitmapheader: *const RTL_BITMAP, numbertofind: u32, hintindex: u32) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlFindClearBits(bitmapheader : *const RTL_BITMAP, numbertofind : u32, hintindex : u32) -> u32);
    unsafe { RtlFindClearBits(bitmapheader, numbertofind, hintindex) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn RtlFindClearBitsAndSet(bitmapheader: *const RTL_BITMAP, numbertofind: u32, hintindex: u32) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlFindClearBitsAndSet(bitmapheader : *const RTL_BITMAP, numbertofind : u32, hintindex : u32) -> u32);
    unsafe { RtlFindClearBitsAndSet(bitmapheader, numbertofind, hintindex) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn RtlFindClearRuns(bitmapheader: *const RTL_BITMAP, runarray: &mut [RTL_BITMAP_RUN], locatelongestruns: bool) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlFindClearRuns(bitmapheader : *const RTL_BITMAP, runarray : *mut RTL_BITMAP_RUN, sizeofrunarray : u32, locatelongestruns : bool) -> u32);
    unsafe { RtlFindClearRuns(bitmapheader, runarray.as_mut_ptr(), runarray.len().try_into().unwrap(), locatelongestruns) }
}
#[cfg(feature = "bcrypt")]
#[inline]
pub unsafe fn RtlFindClosestEncodableLength(sourcelength: u64, targetlength: *mut u64) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlFindClosestEncodableLength(sourcelength : u64, targetlength : *mut u64) -> super::bcrypt::NTSTATUS);
    unsafe { RtlFindClosestEncodableLength(sourcelength, targetlength as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn RtlFindLastBackwardRunClear(bitmapheader: *const RTL_BITMAP, fromindex: u32, startingrunindex: *mut u32) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlFindLastBackwardRunClear(bitmapheader : *const RTL_BITMAP, fromindex : u32, startingrunindex : *mut u32) -> u32);
    unsafe { RtlFindLastBackwardRunClear(bitmapheader, fromindex, startingrunindex as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RtlFindLeastSignificantBit(set: u64) -> super::winnt::CCHAR {
    windows_core::link!("ntdll.dll" "system" fn RtlFindLeastSignificantBit(set : u64) -> super::winnt::CCHAR);
    unsafe { RtlFindLeastSignificantBit(set) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn RtlFindLongestRunClear(bitmapheader: *const RTL_BITMAP, startingindex: *mut u32) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlFindLongestRunClear(bitmapheader : *const RTL_BITMAP, startingindex : *mut u32) -> u32);
    unsafe { RtlFindLongestRunClear(bitmapheader, startingindex as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RtlFindMostSignificantBit(set: u64) -> super::winnt::CCHAR {
    windows_core::link!("ntdll.dll" "system" fn RtlFindMostSignificantBit(set : u64) -> super::winnt::CCHAR);
    unsafe { RtlFindMostSignificantBit(set) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn RtlFindNextForwardRunClear(bitmapheader: *const RTL_BITMAP, fromindex: u32, startingrunindex: *mut u32) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlFindNextForwardRunClear(bitmapheader : *const RTL_BITMAP, fromindex : u32, startingrunindex : *mut u32) -> u32);
    unsafe { RtlFindNextForwardRunClear(bitmapheader, fromindex, startingrunindex as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn RtlFindSetBits(bitmapheader: *const RTL_BITMAP, numbertofind: u32, hintindex: u32) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlFindSetBits(bitmapheader : *const RTL_BITMAP, numbertofind : u32, hintindex : u32) -> u32);
    unsafe { RtlFindSetBits(bitmapheader, numbertofind, hintindex) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn RtlFindSetBitsAndClear(bitmapheader: *const RTL_BITMAP, numbertofind: u32, hintindex: u32) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlFindSetBitsAndClear(bitmapheader : *const RTL_BITMAP, numbertofind : u32, hintindex : u32) -> u32);
    unsafe { RtlFindSetBitsAndClear(bitmapheader, numbertofind, hintindex) }
}
#[cfg(all(feature = "lsalookup", feature = "ntdef", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn RtlFreeUTF8String(utf8string: super::ntdef::PUTF8_STRING) {
    windows_core::link!("ntdll.dll" "system" fn RtlFreeUTF8String(utf8string : super::ntdef::PUTF8_STRING));
    unsafe { RtlFreeUTF8String(utf8string as _) }
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi"))]
#[inline]
pub unsafe fn RtlGUIDFromString(guidstring: *const super::ntsecapi::UNICODE_STRING, guid: *mut windows_core::GUID) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlGUIDFromString(guidstring : *const super::ntsecapi::UNICODE_STRING, guid : *mut windows_core::GUID) -> super::bcrypt::NTSTATUS);
    unsafe { RtlGUIDFromString(guidstring, guid as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn RtlGetVersion(lpversioninformation: *mut super::winnt::OSVERSIONINFOW) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlGetVersion(lpversioninformation : *mut super::winnt::OSVERSIONINFOW) -> super::bcrypt::NTSTATUS);
    unsafe { RtlGetVersion(lpversioninformation as _) }
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi"))]
#[inline]
pub unsafe fn RtlHashUnicodeString(string: *const super::ntsecapi::UNICODE_STRING, caseinsensitive: bool, hashalgorithm: u32, hashvalue: *mut u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlHashUnicodeString(string : *const super::ntsecapi::UNICODE_STRING, caseinsensitive : bool, hashalgorithm : u32, hashvalue : *mut u32) -> super::bcrypt::NTSTATUS);
    unsafe { RtlHashUnicodeString(string, caseinsensitive, hashalgorithm, hashvalue as _) }
}
#[cfg(all(feature = "lsalookup", feature = "ntdef", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn RtlInitUTF8String<P1>(destinationstring: super::ntdef::PUTF8_STRING, sourcestring: P1)
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ntdll.dll" "system" fn RtlInitUTF8String(destinationstring : super::ntdef::PUTF8_STRING, sourcestring : windows_core::PCSTR));
    unsafe { RtlInitUTF8String(destinationstring as _, sourcestring.param().abi()) }
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn RtlInitUTF8StringEx<P1>(destinationstring: super::ntdef::PUTF8_STRING, sourcestring: P1) -> super::bcrypt::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ntdll.dll" "system" fn RtlInitUTF8StringEx(destinationstring : super::ntdef::PUTF8_STRING, sourcestring : windows_core::PCSTR) -> super::bcrypt::NTSTATUS);
    unsafe { RtlInitUTF8StringEx(destinationstring as _, sourcestring.param().abi()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn RtlInitializeBitMap(bitmapheader: *mut RTL_BITMAP, bitmapbuffer: Option<*const u32>, sizeofbitmap: Option<u32>) {
    windows_core::link!("ntdll.dll" "system" fn RtlInitializeBitMap(bitmapheader : *mut RTL_BITMAP, bitmapbuffer : *const u32, sizeofbitmap : u32));
    unsafe { RtlInitializeBitMap(bitmapheader as _, bitmapbuffer.unwrap_or(core::mem::zeroed()) as _, sizeofbitmap.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi"))]
#[inline]
pub unsafe fn RtlInt64ToUnicodeString(value: u64, base: Option<u32>, string: *mut super::ntsecapi::UNICODE_STRING) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlInt64ToUnicodeString(value : u64, base : u32, string : *mut super::ntsecapi::UNICODE_STRING) -> super::bcrypt::NTSTATUS);
    unsafe { RtlInt64ToUnicodeString(value, base.unwrap_or(core::mem::zeroed()) as _, string as _) }
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi"))]
#[inline]
pub unsafe fn RtlIntegerToUnicodeString(value: u32, base: Option<u32>, string: *mut super::ntsecapi::UNICODE_STRING) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlIntegerToUnicodeString(value : u32, base : u32, string : *mut super::ntsecapi::UNICODE_STRING) -> super::bcrypt::NTSTATUS);
    unsafe { RtlIntegerToUnicodeString(value, base.unwrap_or(core::mem::zeroed()) as _, string as _) }
}
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[inline]
pub unsafe fn RtlIoDecodeMemIoResource(descriptor: *const IO_RESOURCE_DESCRIPTOR, alignment: Option<*mut u64>, minimumaddress: Option<*mut u64>, maximumaddress: Option<*mut u64>) -> u64 {
    windows_core::link!("ntdll.dll" "system" fn RtlIoDecodeMemIoResource(descriptor : *const IO_RESOURCE_DESCRIPTOR, alignment : *mut u64, minimumaddress : *mut u64, maximumaddress : *mut u64) -> u64);
    unsafe { RtlIoDecodeMemIoResource(descriptor, alignment.unwrap_or(core::mem::zeroed()) as _, minimumaddress.unwrap_or(core::mem::zeroed()) as _, maximumaddress.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "usb"))]
#[inline]
pub unsafe fn RtlIoEncodeMemIoResource(descriptor: *const IO_RESOURCE_DESCRIPTOR, r#type: u8, length: u64, alignment: u64, minimumaddress: u64, maximumaddress: u64) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlIoEncodeMemIoResource(descriptor : *const IO_RESOURCE_DESCRIPTOR, r#type : u8, length : u64, alignment : u64, minimumaddress : u64, maximumaddress : u64) -> super::bcrypt::NTSTATUS);
    unsafe { RtlIoEncodeMemIoResource(descriptor, r#type, length, alignment, minimumaddress, maximumaddress) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn RtlIsUntrustedObject(handle: Option<super::winnt::HANDLE>, object: Option<*const core::ffi::c_void>, untrustedobject: *mut bool) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlIsUntrustedObject(handle : super::winnt::HANDLE, object : *const core::ffi::c_void, untrustedobject : *mut bool) -> super::bcrypt::NTSTATUS);
    unsafe { RtlIsUntrustedObject(handle.unwrap_or(core::mem::zeroed()) as _, object.unwrap_or(core::mem::zeroed()) as _, untrustedobject as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RtlLengthSecurityDescriptor(securitydescriptor: super::winnt::PSECURITY_DESCRIPTOR) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlLengthSecurityDescriptor(securitydescriptor : super::winnt::PSECURITY_DESCRIPTOR) -> u32);
    unsafe { RtlLengthSecurityDescriptor(securitydescriptor) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn RtlNumberOfClearBits(bitmapheader: *const RTL_BITMAP) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlNumberOfClearBits(bitmapheader : *const RTL_BITMAP) -> u32);
    unsafe { RtlNumberOfClearBits(bitmapheader) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn RtlNumberOfClearBitsInRange(bitmapheader: *const RTL_BITMAP, startingindex: u32, length: u32) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlNumberOfClearBitsInRange(bitmapheader : *const RTL_BITMAP, startingindex : u32, length : u32) -> u32);
    unsafe { RtlNumberOfClearBitsInRange(bitmapheader, startingindex, length) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn RtlNumberOfSetBits(bitmapheader: *const RTL_BITMAP) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlNumberOfSetBits(bitmapheader : *const RTL_BITMAP) -> u32);
    unsafe { RtlNumberOfSetBits(bitmapheader) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn RtlNumberOfSetBitsInRange(bitmapheader: *const RTL_BITMAP, startingindex: u32, length: u32) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlNumberOfSetBitsInRange(bitmapheader : *const RTL_BITMAP, startingindex : u32, length : u32) -> u32);
    unsafe { RtlNumberOfSetBitsInRange(bitmapheader, startingindex, length) }
}
#[inline]
pub unsafe fn RtlNumberOfSetBitsUlongPtr(target: usize) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlNumberOfSetBitsUlongPtr(target : usize) -> u32);
    unsafe { RtlNumberOfSetBitsUlongPtr(target) }
}
#[cfg(feature = "bcrypt")]
#[inline]
pub unsafe fn RtlQueryRegistryValues<P1>(relativeto: u32, path: P1, querytable: *mut RTL_QUERY_REGISTRY_TABLE, context: Option<*const core::ffi::c_void>, environment: Option<*const core::ffi::c_void>) -> super::bcrypt::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdll.dll" "system" fn RtlQueryRegistryValues(relativeto : u32, path : windows_core::PCWSTR, querytable : *mut RTL_QUERY_REGISTRY_TABLE, context : *const core::ffi::c_void, environment : *const core::ffi::c_void) -> super::bcrypt::NTSTATUS);
    unsafe { RtlQueryRegistryValues(relativeto, path.param().abi(), querytable as _, context.unwrap_or(core::mem::zeroed()) as _, environment.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
#[inline]
pub unsafe fn RtlQueryValidationRunlevel(componentname: Option<*const super::ntsecapi::UNICODE_STRING>) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlQueryValidationRunlevel(componentname : *const super::ntsecapi::UNICODE_STRING) -> u32);
    unsafe { RtlQueryValidationRunlevel(componentname.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn RtlSetAllBits(bitmapheader: *const RTL_BITMAP) {
    windows_core::link!("ntdll.dll" "system" fn RtlSetAllBits(bitmapheader : *const RTL_BITMAP));
    unsafe { RtlSetAllBits(bitmapheader) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn RtlSetBit(bitmapheader: *const RTL_BITMAP, bitnumber: u32) {
    windows_core::link!("ntdll.dll" "system" fn RtlSetBit(bitmapheader : *const RTL_BITMAP, bitnumber : u32));
    unsafe { RtlSetBit(bitmapheader, bitnumber) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn RtlSetBits(bitmapheader: *const RTL_BITMAP, startingindex: u32, numbertoset: u32) {
    windows_core::link!("ntdll.dll" "system" fn RtlSetBits(bitmapheader : *const RTL_BITMAP, startingindex : u32, numbertoset : u32));
    unsafe { RtlSetBits(bitmapheader, startingindex, numbertoset) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn RtlSetDaclSecurityDescriptor(securitydescriptor: super::winnt::PSECURITY_DESCRIPTOR, daclpresent: bool, dacl: Option<*const super::winnt::ACL>, dacldefaulted: bool) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlSetDaclSecurityDescriptor(securitydescriptor : super::winnt::PSECURITY_DESCRIPTOR, daclpresent : bool, dacl : *const super::winnt::ACL, dacldefaulted : bool) -> super::bcrypt::NTSTATUS);
    unsafe { RtlSetDaclSecurityDescriptor(securitydescriptor as _, daclpresent, dacl.unwrap_or(core::mem::zeroed()) as _, dacldefaulted) }
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi"))]
#[inline]
pub unsafe fn RtlStringFromGUID(guid: *const windows_core::GUID, guidstring: *mut super::ntsecapi::UNICODE_STRING) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlStringFromGUID(guid : *const windows_core::GUID, guidstring : *mut super::ntsecapi::UNICODE_STRING) -> super::bcrypt::NTSTATUS);
    unsafe { RtlStringFromGUID(guid, guidstring as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn RtlTestBit(bitmapheader: *const RTL_BITMAP, bitnumber: u32) -> bool {
    windows_core::link!("ntdll.dll" "system" fn RtlTestBit(bitmapheader : *const RTL_BITMAP, bitnumber : u32) -> bool);
    unsafe { RtlTestBit(bitmapheader, bitnumber) }
}
#[cfg(feature = "ntdef")]
#[inline]
pub unsafe fn RtlTimeFieldsToTime(timefields: *const TIME_FIELDS, time: *mut i64) -> bool {
    windows_core::link!("ntdll.dll" "system" fn RtlTimeFieldsToTime(timefields : *const TIME_FIELDS, time : *mut i64) -> bool);
    unsafe { RtlTimeFieldsToTime(timefields, time as _) }
}
#[cfg(feature = "ntdef")]
#[inline]
pub unsafe fn RtlTimeToTimeFields(time: *const i64) -> TIME_FIELDS {
    windows_core::link!("ntdll.dll" "system" fn RtlTimeToTimeFields(time : *const i64, timefields : *mut TIME_FIELDS));
    unsafe {
        let mut result__ = core::mem::zeroed();
        RtlTimeToTimeFields(time, &mut result__);
        result__
    }
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn RtlUTF8StringToUnicodeString(destinationstring: *mut super::ntsecapi::UNICODE_STRING, sourcestring: super::ntdef::PUTF8_STRING, allocatedestinationstring: bool) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlUTF8StringToUnicodeString(destinationstring : *mut super::ntsecapi::UNICODE_STRING, sourcestring : super::ntdef::PUTF8_STRING, allocatedestinationstring : bool) -> super::bcrypt::NTSTATUS);
    unsafe { RtlUTF8StringToUnicodeString(destinationstring as _, sourcestring, allocatedestinationstring) }
}
#[cfg(feature = "bcrypt")]
#[inline]
pub unsafe fn RtlUTF8ToUnicodeN(unicodestringdestination: windows_core::PWSTR, unicodestringmaxbytecount: u32, unicodestringactualbytecount: *mut u32, utf8stringsource: &[u8]) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlUTF8ToUnicodeN(unicodestringdestination : windows_core::PWSTR, unicodestringmaxbytecount : u32, unicodestringactualbytecount : *mut u32, utf8stringsource : *const i8, utf8stringbytecount : u32) -> super::bcrypt::NTSTATUS);
    unsafe { RtlUTF8ToUnicodeN(core::mem::transmute(unicodestringdestination), unicodestringmaxbytecount, unicodestringactualbytecount as _, core::mem::transmute(utf8stringsource.as_ptr()), utf8stringsource.len().try_into().unwrap()) }
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi"))]
#[inline]
pub unsafe fn RtlUnicodeStringToInteger(string: *const super::ntsecapi::UNICODE_STRING, base: Option<u32>, value: *mut u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlUnicodeStringToInteger(string : *const super::ntsecapi::UNICODE_STRING, base : u32, value : *mut u32) -> super::bcrypt::NTSTATUS);
    unsafe { RtlUnicodeStringToInteger(string, base.unwrap_or(core::mem::zeroed()) as _, value as _) }
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn RtlUnicodeStringToUTF8String(destinationstring: super::ntdef::PUTF8_STRING, sourcestring: *const super::ntsecapi::UNICODE_STRING, allocatedestinationstring: bool) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlUnicodeStringToUTF8String(destinationstring : super::ntdef::PUTF8_STRING, sourcestring : *const super::ntsecapi::UNICODE_STRING, allocatedestinationstring : bool) -> super::bcrypt::NTSTATUS);
    unsafe { RtlUnicodeStringToUTF8String(destinationstring, sourcestring, allocatedestinationstring) }
}
#[cfg(feature = "bcrypt")]
#[inline]
pub unsafe fn RtlUnicodeToUTF8N(utf8stringdestination: &mut [u8], utf8stringactualbytecount: *mut u32, unicodestringsource: *const u16, unicodestringbytecount: u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlUnicodeToUTF8N(utf8stringdestination : *mut i8, utf8stringmaxbytecount : u32, utf8stringactualbytecount : *mut u32, unicodestringsource : *const u16, unicodestringbytecount : u32) -> super::bcrypt::NTSTATUS);
    unsafe { RtlUnicodeToUTF8N(core::mem::transmute(utf8stringdestination.as_mut_ptr()), utf8stringdestination.len().try_into().unwrap(), utf8stringactualbytecount as _, unicodestringsource, unicodestringbytecount) }
}
#[inline]
pub unsafe fn RtlUpcaseUnicodeChar(sourcecharacter: u16) -> u16 {
    windows_core::link!("ntdll.dll" "system" fn RtlUpcaseUnicodeChar(sourcecharacter : u16) -> u16);
    unsafe { RtlUpcaseUnicodeChar(sourcecharacter) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RtlValidRelativeSecurityDescriptor(securitydescriptorinput: super::winnt::PSECURITY_DESCRIPTOR, securitydescriptorlength: u32, requiredinformation: super::winnt::SECURITY_INFORMATION) -> bool {
    windows_core::link!("ntdll.dll" "system" fn RtlValidRelativeSecurityDescriptor(securitydescriptorinput : super::winnt::PSECURITY_DESCRIPTOR, securitydescriptorlength : u32, requiredinformation : super::winnt::SECURITY_INFORMATION) -> bool);
    unsafe { RtlValidRelativeSecurityDescriptor(securitydescriptorinput, securitydescriptorlength, requiredinformation) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RtlValidSecurityDescriptor(securitydescriptor: super::winnt::PSECURITY_DESCRIPTOR) -> bool {
    windows_core::link!("ntdll.dll" "system" fn RtlValidSecurityDescriptor(securitydescriptor : super::winnt::PSECURITY_DESCRIPTOR) -> bool);
    unsafe { RtlValidSecurityDescriptor(securitydescriptor) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn RtlVerifyVersionInfo(versioninfo: *const super::winnt::OSVERSIONINFOEXW, typemask: u32, conditionmask: u64) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlVerifyVersionInfo(versioninfo : *const super::winnt::OSVERSIONINFOEXW, typemask : u32, conditionmask : u64) -> super::bcrypt::NTSTATUS);
    unsafe { RtlVerifyVersionInfo(versioninfo, typemask, conditionmask) }
}
#[cfg(feature = "bcrypt")]
#[inline]
pub unsafe fn RtlWriteRegistryValue<P1, P2>(relativeto: u32, path: P1, valuename: P2, valuetype: u32, valuedata: Option<*const core::ffi::c_void>, valuelength: u32) -> super::bcrypt::NTSTATUS
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ntdll.dll" "system" fn RtlWriteRegistryValue(relativeto : u32, path : windows_core::PCWSTR, valuename : windows_core::PCWSTR, valuetype : u32, valuedata : *const core::ffi::c_void, valuelength : u32) -> super::bcrypt::NTSTATUS);
    unsafe { RtlWriteRegistryValue(relativeto, path.param().abi(), valuename.param().abi(), valuetype, valuedata.unwrap_or(core::mem::zeroed()) as _, valuelength) }
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn RtlxAnsiStringToUnicodeSize(ansistring: super::winternl::PCANSI_STRING) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlxAnsiStringToUnicodeSize(ansistring : super::winternl::PCANSI_STRING) -> u32);
    unsafe { RtlxAnsiStringToUnicodeSize(ansistring) }
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
#[inline]
pub unsafe fn RtlxUnicodeStringToAnsiSize(unicodestring: *const super::ntsecapi::UNICODE_STRING) -> u32 {
    windows_core::link!("ntdll.dll" "system" fn RtlxUnicodeStringToAnsiSize(unicodestring : *const super::ntsecapi::UNICODE_STRING) -> u32);
    unsafe { RtlxUnicodeStringToAnsiSize(unicodestring) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwClose(handle: super::winnt::HANDLE) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwClose(handle : super::winnt::HANDLE) -> super::bcrypt::NTSTATUS);
    unsafe { ZwClose(handle) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwCommitComplete(enlistmenthandle: super::winnt::HANDLE, tmvirtualclock: Option<*const i64>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwCommitComplete(enlistmenthandle : super::winnt::HANDLE, tmvirtualclock : *const i64) -> super::bcrypt::NTSTATUS);
    unsafe { ZwCommitComplete(enlistmenthandle, tmvirtualclock.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwCommitEnlistment(enlistmenthandle: super::winnt::HANDLE, tmvirtualclock: Option<*const i64>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwCommitEnlistment(enlistmenthandle : super::winnt::HANDLE, tmvirtualclock : *const i64) -> super::bcrypt::NTSTATUS);
    unsafe { ZwCommitEnlistment(enlistmenthandle, tmvirtualclock.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwCommitRegistryTransaction(transactionhandle: super::winnt::HANDLE, flags: u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwCommitRegistryTransaction(transactionhandle : super::winnt::HANDLE, flags : u32) -> super::bcrypt::NTSTATUS);
    unsafe { ZwCommitRegistryTransaction(transactionhandle, flags) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwCommitTransaction(transactionhandle: super::winnt::HANDLE, wait: bool) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwCommitTransaction(transactionhandle : super::winnt::HANDLE, wait : bool) -> super::bcrypt::NTSTATUS);
    unsafe { ZwCommitTransaction(transactionhandle, wait) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn ZwCopyFileChunk(sourcehandle: super::winnt::HANDLE, desthandle: super::winnt::HANDLE, event: Option<super::winnt::HANDLE>, iostatusblock: *mut super::winternl::IO_STATUS_BLOCK, length: u32, sourceoffset: *const i64, destoffset: *const i64, sourcekey: Option<*const u32>, destkey: Option<*const u32>, flags: u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwCopyFileChunk(sourcehandle : super::winnt::HANDLE, desthandle : super::winnt::HANDLE, event : super::winnt::HANDLE, iostatusblock : *mut super::winternl::IO_STATUS_BLOCK, length : u32, sourceoffset : *const i64, destoffset : *const i64, sourcekey : *const u32, destkey : *const u32, flags : u32) -> super::bcrypt::NTSTATUS);
    unsafe { ZwCopyFileChunk(sourcehandle, desthandle, event.unwrap_or(core::mem::zeroed()) as _, iostatusblock as _, length, sourceoffset, destoffset, sourcekey.unwrap_or(core::mem::zeroed()) as _, destkey.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[cfg(all(feature = "bcrypt", feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn ZwCreateDirectoryObject(directoryhandle: *mut super::winnt::HANDLE, desiredaccess: super::winnt::ACCESS_MASK, objectattributes: *const super::d3dkmthk::OBJECT_ATTRIBUTES) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwCreateDirectoryObject(directoryhandle : *mut super::winnt::HANDLE, desiredaccess : super::winnt::ACCESS_MASK, objectattributes : *const super::d3dkmthk::OBJECT_ATTRIBUTES) -> super::bcrypt::NTSTATUS);
    unsafe { ZwCreateDirectoryObject(directoryhandle as _, desiredaccess, objectattributes) }
}
#[cfg(all(feature = "bcrypt", feature = "d3dkmthk", feature = "ktmtypes", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn ZwCreateEnlistment(enlistmenthandle: *mut super::winnt::HANDLE, desiredaccess: super::winnt::ACCESS_MASK, resourcemanagerhandle: super::winnt::HANDLE, transactionhandle: super::winnt::HANDLE, objectattributes: Option<*const super::d3dkmthk::OBJECT_ATTRIBUTES>, createoptions: Option<u32>, notificationmask: super::ktmtypes::NOTIFICATION_MASK, enlistmentkey: Option<*const core::ffi::c_void>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwCreateEnlistment(enlistmenthandle : *mut super::winnt::HANDLE, desiredaccess : super::winnt::ACCESS_MASK, resourcemanagerhandle : super::winnt::HANDLE, transactionhandle : super::winnt::HANDLE, objectattributes : *const super::d3dkmthk::OBJECT_ATTRIBUTES, createoptions : u32, notificationmask : super::ktmtypes::NOTIFICATION_MASK, enlistmentkey : *const core::ffi::c_void) -> super::bcrypt::NTSTATUS);
    unsafe { ZwCreateEnlistment(enlistmenthandle as _, desiredaccess, resourcemanagerhandle, transactionhandle, objectattributes.unwrap_or(core::mem::zeroed()) as _, createoptions.unwrap_or(core::mem::zeroed()) as _, notificationmask, enlistmentkey.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn ZwCreateFile(filehandle: *mut super::winnt::HANDLE, desiredaccess: super::winnt::ACCESS_MASK, objectattributes: *const super::d3dkmthk::OBJECT_ATTRIBUTES, iostatusblock: *mut super::winternl::IO_STATUS_BLOCK, allocationsize: Option<*const i64>, fileattributes: u32, shareaccess: u32, createdisposition: u32, createoptions: u32, eabuffer: Option<*const core::ffi::c_void>, ealength: u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwCreateFile(filehandle : *mut super::winnt::HANDLE, desiredaccess : super::winnt::ACCESS_MASK, objectattributes : *const super::d3dkmthk::OBJECT_ATTRIBUTES, iostatusblock : *mut super::winternl::IO_STATUS_BLOCK, allocationsize : *const i64, fileattributes : u32, shareaccess : u32, createdisposition : u32, createoptions : u32, eabuffer : *const core::ffi::c_void, ealength : u32) -> super::bcrypt::NTSTATUS);
    unsafe { ZwCreateFile(filehandle as _, desiredaccess, objectattributes, iostatusblock as _, allocationsize.unwrap_or(core::mem::zeroed()) as _, fileattributes, shareaccess, createdisposition, createoptions, eabuffer.unwrap_or(core::mem::zeroed()) as _, ealength) }
}
#[cfg(all(feature = "bcrypt", feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn ZwCreateIoCompletion(iocompletionhandle: *mut super::winnt::HANDLE, desiredaccess: super::winnt::ACCESS_MASK, objectattributes: Option<*const super::d3dkmthk::OBJECT_ATTRIBUTES>, count: Option<u32>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwCreateIoCompletion(iocompletionhandle : *mut super::winnt::HANDLE, desiredaccess : super::winnt::ACCESS_MASK, objectattributes : *const super::d3dkmthk::OBJECT_ATTRIBUTES, count : u32) -> super::bcrypt::NTSTATUS);
    unsafe { ZwCreateIoCompletion(iocompletionhandle as _, desiredaccess, objectattributes.unwrap_or(core::mem::zeroed()) as _, count.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn ZwCreateKey(keyhandle: *mut super::winnt::HANDLE, desiredaccess: super::winnt::ACCESS_MASK, objectattributes: *const super::d3dkmthk::OBJECT_ATTRIBUTES, titleindex: Option<u32>, class: Option<*const super::ntsecapi::UNICODE_STRING>, createoptions: u32, disposition: Option<*mut u32>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwCreateKey(keyhandle : *mut super::winnt::HANDLE, desiredaccess : super::winnt::ACCESS_MASK, objectattributes : *const super::d3dkmthk::OBJECT_ATTRIBUTES, titleindex : u32, class : *const super::ntsecapi::UNICODE_STRING, createoptions : u32, disposition : *mut u32) -> super::bcrypt::NTSTATUS);
    unsafe { ZwCreateKey(keyhandle as _, desiredaccess, objectattributes, titleindex.unwrap_or(core::mem::zeroed()) as _, class.unwrap_or(core::mem::zeroed()) as _, createoptions, disposition.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn ZwCreateKeyTransacted(keyhandle: *mut super::winnt::HANDLE, desiredaccess: super::winnt::ACCESS_MASK, objectattributes: *const super::d3dkmthk::OBJECT_ATTRIBUTES, titleindex: Option<u32>, class: Option<*const super::ntsecapi::UNICODE_STRING>, createoptions: u32, transactionhandle: super::winnt::HANDLE, disposition: Option<*mut u32>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "C" fn ZwCreateKeyTransacted(keyhandle : *mut super::winnt::HANDLE, desiredaccess : super::winnt::ACCESS_MASK, objectattributes : *const super::d3dkmthk::OBJECT_ATTRIBUTES, titleindex : u32, class : *const super::ntsecapi::UNICODE_STRING, createoptions : u32, transactionhandle : super::winnt::HANDLE, disposition : *mut u32) -> super::bcrypt::NTSTATUS);
    unsafe { ZwCreateKeyTransacted(keyhandle as _, desiredaccess, objectattributes, titleindex.unwrap_or(core::mem::zeroed()) as _, class.unwrap_or(core::mem::zeroed()) as _, createoptions, transactionhandle, disposition.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn ZwCreateRegistryTransaction(transactionhandle: *mut super::winnt::HANDLE, desiredaccess: super::winnt::ACCESS_MASK, objectattributes: Option<*const super::d3dkmthk::OBJECT_ATTRIBUTES>, createoptions: Option<u32>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwCreateRegistryTransaction(transactionhandle : *mut super::winnt::HANDLE, desiredaccess : super::winnt::ACCESS_MASK, objectattributes : *const super::d3dkmthk::OBJECT_ATTRIBUTES, createoptions : u32) -> super::bcrypt::NTSTATUS);
    unsafe { ZwCreateRegistryTransaction(transactionhandle as _, desiredaccess, objectattributes.unwrap_or(core::mem::zeroed()) as _, createoptions.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn ZwCreateResourceManager(resourcemanagerhandle: *mut super::winnt::HANDLE, desiredaccess: super::winnt::ACCESS_MASK, tmhandle: super::winnt::HANDLE, resourcemanagerguid: Option<*const windows_core::GUID>, objectattributes: Option<*const super::d3dkmthk::OBJECT_ATTRIBUTES>, createoptions: Option<u32>, description: Option<*const super::ntsecapi::UNICODE_STRING>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwCreateResourceManager(resourcemanagerhandle : *mut super::winnt::HANDLE, desiredaccess : super::winnt::ACCESS_MASK, tmhandle : super::winnt::HANDLE, resourcemanagerguid : *const windows_core::GUID, objectattributes : *const super::d3dkmthk::OBJECT_ATTRIBUTES, createoptions : u32, description : *const super::ntsecapi::UNICODE_STRING) -> super::bcrypt::NTSTATUS);
    unsafe { ZwCreateResourceManager(resourcemanagerhandle as _, desiredaccess, tmhandle, resourcemanagerguid.unwrap_or(core::mem::zeroed()) as _, objectattributes.unwrap_or(core::mem::zeroed()) as _, createoptions.unwrap_or(core::mem::zeroed()) as _, description.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn ZwCreateSection(sectionhandle: *mut super::winnt::HANDLE, desiredaccess: super::winnt::ACCESS_MASK, objectattributes: Option<*const super::d3dkmthk::OBJECT_ATTRIBUTES>, maximumsize: Option<*const i64>, sectionpageprotection: u32, allocationattributes: u32, filehandle: Option<super::winnt::HANDLE>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwCreateSection(sectionhandle : *mut super::winnt::HANDLE, desiredaccess : super::winnt::ACCESS_MASK, objectattributes : *const super::d3dkmthk::OBJECT_ATTRIBUTES, maximumsize : *const i64, sectionpageprotection : u32, allocationattributes : u32, filehandle : super::winnt::HANDLE) -> super::bcrypt::NTSTATUS);
    unsafe { ZwCreateSection(sectionhandle as _, desiredaccess, objectattributes.unwrap_or(core::mem::zeroed()) as _, maximumsize.unwrap_or(core::mem::zeroed()) as _, sectionpageprotection, allocationattributes, filehandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn ZwCreateTransaction(transactionhandle: *mut super::winnt::HANDLE, desiredaccess: super::winnt::ACCESS_MASK, objectattributes: Option<*const super::d3dkmthk::OBJECT_ATTRIBUTES>, uow: Option<*const windows_core::GUID>, tmhandle: Option<super::winnt::HANDLE>, createoptions: Option<u32>, isolationlevel: Option<u32>, isolationflags: Option<u32>, timeout: Option<*const i64>, description: Option<*const super::ntsecapi::UNICODE_STRING>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwCreateTransaction(transactionhandle : *mut super::winnt::HANDLE, desiredaccess : super::winnt::ACCESS_MASK, objectattributes : *const super::d3dkmthk::OBJECT_ATTRIBUTES, uow : *const windows_core::GUID, tmhandle : super::winnt::HANDLE, createoptions : u32, isolationlevel : u32, isolationflags : u32, timeout : *const i64, description : *const super::ntsecapi::UNICODE_STRING) -> super::bcrypt::NTSTATUS);
    unsafe { ZwCreateTransaction(transactionhandle as _, desiredaccess, objectattributes.unwrap_or(core::mem::zeroed()) as _, uow.unwrap_or(core::mem::zeroed()) as _, tmhandle.unwrap_or(core::mem::zeroed()) as _, createoptions.unwrap_or(core::mem::zeroed()) as _, isolationlevel.unwrap_or(core::mem::zeroed()) as _, isolationflags.unwrap_or(core::mem::zeroed()) as _, timeout.unwrap_or(core::mem::zeroed()) as _, description.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn ZwCreateTransactionManager(tmhandle: *mut super::winnt::HANDLE, desiredaccess: super::winnt::ACCESS_MASK, objectattributes: Option<*const super::d3dkmthk::OBJECT_ATTRIBUTES>, logfilename: Option<*const super::ntsecapi::UNICODE_STRING>, createoptions: Option<u32>, commitstrength: Option<u32>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwCreateTransactionManager(tmhandle : *mut super::winnt::HANDLE, desiredaccess : super::winnt::ACCESS_MASK, objectattributes : *const super::d3dkmthk::OBJECT_ATTRIBUTES, logfilename : *const super::ntsecapi::UNICODE_STRING, createoptions : u32, commitstrength : u32) -> super::bcrypt::NTSTATUS);
    unsafe { ZwCreateTransactionManager(tmhandle as _, desiredaccess, objectattributes.unwrap_or(core::mem::zeroed()) as _, logfilename.unwrap_or(core::mem::zeroed()) as _, createoptions.unwrap_or(core::mem::zeroed()) as _, commitstrength.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwDeleteKey(keyhandle: super::winnt::HANDLE) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwDeleteKey(keyhandle : super::winnt::HANDLE) -> super::bcrypt::NTSTATUS);
    unsafe { ZwDeleteKey(keyhandle) }
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn ZwDeleteValueKey(keyhandle: super::winnt::HANDLE, valuename: *const super::ntsecapi::UNICODE_STRING) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwDeleteValueKey(keyhandle : super::winnt::HANDLE, valuename : *const super::ntsecapi::UNICODE_STRING) -> super::bcrypt::NTSTATUS);
    unsafe { ZwDeleteValueKey(keyhandle, valuename) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwEnumerateKey(keyhandle: super::winnt::HANDLE, index: u32, keyinformationclass: KEY_INFORMATION_CLASS, keyinformation: Option<*mut core::ffi::c_void>, length: u32, resultlength: *mut u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwEnumerateKey(keyhandle : super::winnt::HANDLE, index : u32, keyinformationclass : KEY_INFORMATION_CLASS, keyinformation : *mut core::ffi::c_void, length : u32, resultlength : *mut u32) -> super::bcrypt::NTSTATUS);
    unsafe { ZwEnumerateKey(keyhandle, index, keyinformationclass, keyinformation.unwrap_or(core::mem::zeroed()) as _, length, resultlength as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwEnumerateTransactionObject(rootobjecthandle: Option<super::winnt::HANDLE>, querytype: super::winnt::KTMOBJECT_TYPE, objectcursor: *mut super::winnt::KTMOBJECT_CURSOR, objectcursorlength: u32, returnlength: *mut u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwEnumerateTransactionObject(rootobjecthandle : super::winnt::HANDLE, querytype : super::winnt::KTMOBJECT_TYPE, objectcursor : *mut super::winnt::KTMOBJECT_CURSOR, objectcursorlength : u32, returnlength : *mut u32) -> super::bcrypt::NTSTATUS);
    unsafe { ZwEnumerateTransactionObject(rootobjecthandle.unwrap_or(core::mem::zeroed()) as _, querytype, objectcursor as _, objectcursorlength, returnlength as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwEnumerateValueKey(keyhandle: super::winnt::HANDLE, index: u32, keyvalueinformationclass: KEY_VALUE_INFORMATION_CLASS, keyvalueinformation: Option<*mut core::ffi::c_void>, length: u32, resultlength: *mut u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwEnumerateValueKey(keyhandle : super::winnt::HANDLE, index : u32, keyvalueinformationclass : KEY_VALUE_INFORMATION_CLASS, keyvalueinformation : *mut core::ffi::c_void, length : u32, resultlength : *mut u32) -> super::bcrypt::NTSTATUS);
    unsafe { ZwEnumerateValueKey(keyhandle, index, keyvalueinformationclass, keyvalueinformation.unwrap_or(core::mem::zeroed()) as _, length, resultlength as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwFlushKey(keyhandle: super::winnt::HANDLE) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwFlushKey(keyhandle : super::winnt::HANDLE) -> super::bcrypt::NTSTATUS);
    unsafe { ZwFlushKey(keyhandle) }
}
#[cfg(all(feature = "bcrypt", feature = "ktmtypes", feature = "winnt"))]
#[inline]
pub unsafe fn ZwGetNotificationResourceManager(resourcemanagerhandle: super::winnt::HANDLE, transactionnotification: *mut super::ktmtypes::TRANSACTION_NOTIFICATION, notificationlength: u32, timeout: *const i64, returnlength: Option<*mut u32>, asynchronous: u32, asynchronouscontext: Option<usize>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwGetNotificationResourceManager(resourcemanagerhandle : super::winnt::HANDLE, transactionnotification : *mut super::ktmtypes::TRANSACTION_NOTIFICATION, notificationlength : u32, timeout : *const i64, returnlength : *mut u32, asynchronous : u32, asynchronouscontext : usize) -> super::bcrypt::NTSTATUS);
    unsafe { ZwGetNotificationResourceManager(resourcemanagerhandle, transactionnotification as _, notificationlength, timeout, returnlength.unwrap_or(core::mem::zeroed()) as _, asynchronous, asynchronouscontext.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi"))]
#[inline]
pub unsafe fn ZwLoadDriver(driverservicename: *const super::ntsecapi::UNICODE_STRING) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwLoadDriver(driverservicename : *const super::ntsecapi::UNICODE_STRING) -> super::bcrypt::NTSTATUS);
    unsafe { ZwLoadDriver(driverservicename) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwMakeTemporaryObject(handle: super::winnt::HANDLE) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwMakeTemporaryObject(handle : super::winnt::HANDLE) -> super::bcrypt::NTSTATUS);
    unsafe { ZwMakeTemporaryObject(handle) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwMapViewOfSection(sectionhandle: super::winnt::HANDLE, processhandle: super::winnt::HANDLE, baseaddress: *mut *mut core::ffi::c_void, zerobits: usize, commitsize: usize, sectionoffset: Option<*mut i64>, viewsize: *mut usize, inheritdisposition: SECTION_INHERIT, allocationtype: u32, win32protect: u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwMapViewOfSection(sectionhandle : super::winnt::HANDLE, processhandle : super::winnt::HANDLE, baseaddress : *mut *mut core::ffi::c_void, zerobits : usize, commitsize : usize, sectionoffset : *mut i64, viewsize : *mut usize, inheritdisposition : SECTION_INHERIT, allocationtype : u32, win32protect : u32) -> super::bcrypt::NTSTATUS);
    unsafe { ZwMapViewOfSection(sectionhandle, processhandle, baseaddress as _, zerobits, commitsize, sectionoffset.unwrap_or(core::mem::zeroed()) as _, viewsize as _, inheritdisposition, allocationtype, win32protect) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwMapViewOfSectionEx(sectionhandle: super::winnt::HANDLE, processhandle: super::winnt::HANDLE, baseaddress: *mut *mut core::ffi::c_void, sectionoffset: Option<*mut i64>, viewsize: *mut usize, allocationtype: u32, pageprotection: u32, extendedparameters: Option<&mut [super::winnt::MEM_EXTENDED_PARAMETER]>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwMapViewOfSectionEx(sectionhandle : super::winnt::HANDLE, processhandle : super::winnt::HANDLE, baseaddress : *mut *mut core::ffi::c_void, sectionoffset : *mut i64, viewsize : *mut usize, allocationtype : u32, pageprotection : u32, extendedparameters : *mut super::winnt::MEM_EXTENDED_PARAMETER, extendedparametercount : u32) -> super::bcrypt::NTSTATUS);
    unsafe { ZwMapViewOfSectionEx(sectionhandle, processhandle, baseaddress as _, sectionoffset.unwrap_or(core::mem::zeroed()) as _, viewsize as _, allocationtype, pageprotection, extendedparameters.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()), extendedparameters.as_deref().map_or(0, |slice| slice.len().try_into().unwrap())) }
}
#[cfg(all(feature = "bcrypt", feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn ZwOpenEnlistment(enlistmenthandle: *mut super::winnt::HANDLE, desiredaccess: super::winnt::ACCESS_MASK, rmhandle: super::winnt::HANDLE, enlistmentguid: *const windows_core::GUID, objectattributes: Option<*const super::d3dkmthk::OBJECT_ATTRIBUTES>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwOpenEnlistment(enlistmenthandle : *mut super::winnt::HANDLE, desiredaccess : super::winnt::ACCESS_MASK, rmhandle : super::winnt::HANDLE, enlistmentguid : *const windows_core::GUID, objectattributes : *const super::d3dkmthk::OBJECT_ATTRIBUTES) -> super::bcrypt::NTSTATUS);
    unsafe { ZwOpenEnlistment(enlistmenthandle as _, desiredaccess, rmhandle, enlistmentguid, objectattributes.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn ZwOpenEvent(eventhandle: *mut super::winnt::HANDLE, desiredaccess: super::winnt::ACCESS_MASK, objectattributes: *const super::d3dkmthk::OBJECT_ATTRIBUTES) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwOpenEvent(eventhandle : *mut super::winnt::HANDLE, desiredaccess : super::winnt::ACCESS_MASK, objectattributes : *const super::d3dkmthk::OBJECT_ATTRIBUTES) -> super::bcrypt::NTSTATUS);
    unsafe { ZwOpenEvent(eventhandle as _, desiredaccess, objectattributes) }
}
#[cfg(all(feature = "bcrypt", feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn ZwOpenFile(filehandle: *mut super::winnt::HANDLE, desiredaccess: super::winnt::ACCESS_MASK, objectattributes: *const super::d3dkmthk::OBJECT_ATTRIBUTES, iostatusblock: *mut super::winternl::IO_STATUS_BLOCK, shareaccess: u32, openoptions: u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwOpenFile(filehandle : *mut super::winnt::HANDLE, desiredaccess : super::winnt::ACCESS_MASK, objectattributes : *const super::d3dkmthk::OBJECT_ATTRIBUTES, iostatusblock : *mut super::winternl::IO_STATUS_BLOCK, shareaccess : u32, openoptions : u32) -> super::bcrypt::NTSTATUS);
    unsafe { ZwOpenFile(filehandle as _, desiredaccess, objectattributes, iostatusblock as _, shareaccess, openoptions) }
}
#[cfg(all(feature = "bcrypt", feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn ZwOpenKey(keyhandle: *mut super::winnt::HANDLE, desiredaccess: super::winnt::ACCESS_MASK, objectattributes: *const super::d3dkmthk::OBJECT_ATTRIBUTES) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwOpenKey(keyhandle : *mut super::winnt::HANDLE, desiredaccess : super::winnt::ACCESS_MASK, objectattributes : *const super::d3dkmthk::OBJECT_ATTRIBUTES) -> super::bcrypt::NTSTATUS);
    unsafe { ZwOpenKey(keyhandle as _, desiredaccess, objectattributes) }
}
#[cfg(all(feature = "bcrypt", feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn ZwOpenKeyEx(keyhandle: *mut super::winnt::HANDLE, desiredaccess: super::winnt::ACCESS_MASK, objectattributes: *const super::d3dkmthk::OBJECT_ATTRIBUTES, openoptions: u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwOpenKeyEx(keyhandle : *mut super::winnt::HANDLE, desiredaccess : super::winnt::ACCESS_MASK, objectattributes : *const super::d3dkmthk::OBJECT_ATTRIBUTES, openoptions : u32) -> super::bcrypt::NTSTATUS);
    unsafe { ZwOpenKeyEx(keyhandle as _, desiredaccess, objectattributes, openoptions) }
}
#[cfg(all(feature = "bcrypt", feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn ZwOpenKeyTransacted(keyhandle: *mut super::winnt::HANDLE, desiredaccess: super::winnt::ACCESS_MASK, objectattributes: *const super::d3dkmthk::OBJECT_ATTRIBUTES, transactionhandle: super::winnt::HANDLE) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwOpenKeyTransacted(keyhandle : *mut super::winnt::HANDLE, desiredaccess : super::winnt::ACCESS_MASK, objectattributes : *const super::d3dkmthk::OBJECT_ATTRIBUTES, transactionhandle : super::winnt::HANDLE) -> super::bcrypt::NTSTATUS);
    unsafe { ZwOpenKeyTransacted(keyhandle as _, desiredaccess, objectattributes, transactionhandle) }
}
#[cfg(all(feature = "bcrypt", feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn ZwOpenKeyTransactedEx(keyhandle: *mut super::winnt::HANDLE, desiredaccess: super::winnt::ACCESS_MASK, objectattributes: *const super::d3dkmthk::OBJECT_ATTRIBUTES, openoptions: u32, transactionhandle: super::winnt::HANDLE) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwOpenKeyTransactedEx(keyhandle : *mut super::winnt::HANDLE, desiredaccess : super::winnt::ACCESS_MASK, objectattributes : *const super::d3dkmthk::OBJECT_ATTRIBUTES, openoptions : u32, transactionhandle : super::winnt::HANDLE) -> super::bcrypt::NTSTATUS);
    unsafe { ZwOpenKeyTransactedEx(keyhandle as _, desiredaccess, objectattributes, openoptions, transactionhandle) }
}
#[cfg(all(feature = "bcrypt", feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn ZwOpenResourceManager(resourcemanagerhandle: *mut super::winnt::HANDLE, desiredaccess: super::winnt::ACCESS_MASK, tmhandle: super::winnt::HANDLE, resourcemanagerguid: *const windows_core::GUID, objectattributes: Option<*const super::d3dkmthk::OBJECT_ATTRIBUTES>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwOpenResourceManager(resourcemanagerhandle : *mut super::winnt::HANDLE, desiredaccess : super::winnt::ACCESS_MASK, tmhandle : super::winnt::HANDLE, resourcemanagerguid : *const windows_core::GUID, objectattributes : *const super::d3dkmthk::OBJECT_ATTRIBUTES) -> super::bcrypt::NTSTATUS);
    unsafe { ZwOpenResourceManager(resourcemanagerhandle as _, desiredaccess, tmhandle, resourcemanagerguid, objectattributes.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn ZwOpenSection(sectionhandle: *mut super::winnt::HANDLE, desiredaccess: super::winnt::ACCESS_MASK, objectattributes: *const super::d3dkmthk::OBJECT_ATTRIBUTES) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwOpenSection(sectionhandle : *mut super::winnt::HANDLE, desiredaccess : super::winnt::ACCESS_MASK, objectattributes : *const super::d3dkmthk::OBJECT_ATTRIBUTES) -> super::bcrypt::NTSTATUS);
    unsafe { ZwOpenSection(sectionhandle as _, desiredaccess, objectattributes) }
}
#[cfg(all(feature = "bcrypt", feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn ZwOpenSymbolicLinkObject(linkhandle: *mut super::winnt::HANDLE, desiredaccess: super::winnt::ACCESS_MASK, objectattributes: *const super::d3dkmthk::OBJECT_ATTRIBUTES) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwOpenSymbolicLinkObject(linkhandle : *mut super::winnt::HANDLE, desiredaccess : super::winnt::ACCESS_MASK, objectattributes : *const super::d3dkmthk::OBJECT_ATTRIBUTES) -> super::bcrypt::NTSTATUS);
    unsafe { ZwOpenSymbolicLinkObject(linkhandle as _, desiredaccess, objectattributes) }
}
#[cfg(all(feature = "bcrypt", feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn ZwOpenTransaction(transactionhandle: *mut super::winnt::HANDLE, desiredaccess: super::winnt::ACCESS_MASK, objectattributes: Option<*const super::d3dkmthk::OBJECT_ATTRIBUTES>, uow: *const windows_core::GUID, tmhandle: Option<super::winnt::HANDLE>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwOpenTransaction(transactionhandle : *mut super::winnt::HANDLE, desiredaccess : super::winnt::ACCESS_MASK, objectattributes : *const super::d3dkmthk::OBJECT_ATTRIBUTES, uow : *const windows_core::GUID, tmhandle : super::winnt::HANDLE) -> super::bcrypt::NTSTATUS);
    unsafe { ZwOpenTransaction(transactionhandle as _, desiredaccess, objectattributes.unwrap_or(core::mem::zeroed()) as _, uow, tmhandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn ZwOpenTransactionManager(tmhandle: *mut super::winnt::HANDLE, desiredaccess: super::winnt::ACCESS_MASK, objectattributes: Option<*const super::d3dkmthk::OBJECT_ATTRIBUTES>, logfilename: Option<*const super::ntsecapi::UNICODE_STRING>, tmidentity: Option<*const windows_core::GUID>, openoptions: Option<u32>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwOpenTransactionManager(tmhandle : *mut super::winnt::HANDLE, desiredaccess : super::winnt::ACCESS_MASK, objectattributes : *const super::d3dkmthk::OBJECT_ATTRIBUTES, logfilename : *const super::ntsecapi::UNICODE_STRING, tmidentity : *const windows_core::GUID, openoptions : u32) -> super::bcrypt::NTSTATUS);
    unsafe { ZwOpenTransactionManager(tmhandle as _, desiredaccess, objectattributes.unwrap_or(core::mem::zeroed()) as _, logfilename.unwrap_or(core::mem::zeroed()) as _, tmidentity.unwrap_or(core::mem::zeroed()) as _, openoptions.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwPrePrepareComplete(enlistmenthandle: super::winnt::HANDLE, tmvirtualclock: Option<*const i64>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwPrePrepareComplete(enlistmenthandle : super::winnt::HANDLE, tmvirtualclock : *const i64) -> super::bcrypt::NTSTATUS);
    unsafe { ZwPrePrepareComplete(enlistmenthandle, tmvirtualclock.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwPrePrepareEnlistment(enlistmenthandle: super::winnt::HANDLE, tmvirtualclock: Option<*const i64>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwPrePrepareEnlistment(enlistmenthandle : super::winnt::HANDLE, tmvirtualclock : *const i64) -> super::bcrypt::NTSTATUS);
    unsafe { ZwPrePrepareEnlistment(enlistmenthandle, tmvirtualclock.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwPrepareComplete(enlistmenthandle: super::winnt::HANDLE, tmvirtualclock: Option<*const i64>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwPrepareComplete(enlistmenthandle : super::winnt::HANDLE, tmvirtualclock : *const i64) -> super::bcrypt::NTSTATUS);
    unsafe { ZwPrepareComplete(enlistmenthandle, tmvirtualclock.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwPrepareEnlistment(enlistmenthandle: super::winnt::HANDLE, tmvirtualclock: Option<*const i64>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwPrepareEnlistment(enlistmenthandle : super::winnt::HANDLE, tmvirtualclock : *const i64) -> super::bcrypt::NTSTATUS);
    unsafe { ZwPrepareEnlistment(enlistmenthandle, tmvirtualclock.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn ZwQueryFullAttributesFile(objectattributes: *const super::d3dkmthk::OBJECT_ATTRIBUTES, fileinformation: *mut FILE_NETWORK_OPEN_INFORMATION) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwQueryFullAttributesFile(objectattributes : *const super::d3dkmthk::OBJECT_ATTRIBUTES, fileinformation : *mut FILE_NETWORK_OPEN_INFORMATION) -> super::bcrypt::NTSTATUS);
    unsafe { ZwQueryFullAttributesFile(objectattributes, fileinformation as _) }
}
#[cfg(all(feature = "bcrypt", feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn ZwQueryInformationByName(objectattributes: *const super::d3dkmthk::OBJECT_ATTRIBUTES, iostatusblock: *mut super::winternl::IO_STATUS_BLOCK, fileinformation: *mut core::ffi::c_void, length: u32, fileinformationclass: super::winternl::FILE_INFORMATION_CLASS) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwQueryInformationByName(objectattributes : *const super::d3dkmthk::OBJECT_ATTRIBUTES, iostatusblock : *mut super::winternl::IO_STATUS_BLOCK, fileinformation : *mut core::ffi::c_void, length : u32, fileinformationclass : super::winternl::FILE_INFORMATION_CLASS) -> super::bcrypt::NTSTATUS);
    unsafe { ZwQueryInformationByName(objectattributes, iostatusblock as _, fileinformation as _, length, fileinformationclass) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwQueryInformationEnlistment(enlistmenthandle: super::winnt::HANDLE, enlistmentinformationclass: super::winnt::ENLISTMENT_INFORMATION_CLASS, enlistmentinformation: *mut core::ffi::c_void, enlistmentinformationlength: u32, returnlength: Option<*mut u32>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwQueryInformationEnlistment(enlistmenthandle : super::winnt::HANDLE, enlistmentinformationclass : super::winnt::ENLISTMENT_INFORMATION_CLASS, enlistmentinformation : *mut core::ffi::c_void, enlistmentinformationlength : u32, returnlength : *mut u32) -> super::bcrypt::NTSTATUS);
    unsafe { ZwQueryInformationEnlistment(enlistmenthandle, enlistmentinformationclass, enlistmentinformation as _, enlistmentinformationlength, returnlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn ZwQueryInformationFile(filehandle: super::winnt::HANDLE, iostatusblock: *mut super::winternl::IO_STATUS_BLOCK, fileinformation: *mut core::ffi::c_void, length: u32, fileinformationclass: super::winternl::FILE_INFORMATION_CLASS) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwQueryInformationFile(filehandle : super::winnt::HANDLE, iostatusblock : *mut super::winternl::IO_STATUS_BLOCK, fileinformation : *mut core::ffi::c_void, length : u32, fileinformationclass : super::winternl::FILE_INFORMATION_CLASS) -> super::bcrypt::NTSTATUS);
    unsafe { ZwQueryInformationFile(filehandle, iostatusblock as _, fileinformation as _, length, fileinformationclass) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwQueryInformationResourceManager(resourcemanagerhandle: super::winnt::HANDLE, resourcemanagerinformationclass: super::winnt::RESOURCEMANAGER_INFORMATION_CLASS, resourcemanagerinformation: *mut core::ffi::c_void, resourcemanagerinformationlength: u32, returnlength: Option<*mut u32>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwQueryInformationResourceManager(resourcemanagerhandle : super::winnt::HANDLE, resourcemanagerinformationclass : super::winnt::RESOURCEMANAGER_INFORMATION_CLASS, resourcemanagerinformation : *mut core::ffi::c_void, resourcemanagerinformationlength : u32, returnlength : *mut u32) -> super::bcrypt::NTSTATUS);
    unsafe { ZwQueryInformationResourceManager(resourcemanagerhandle, resourcemanagerinformationclass, resourcemanagerinformation as _, resourcemanagerinformationlength, returnlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwQueryInformationTransaction(transactionhandle: super::winnt::HANDLE, transactioninformationclass: super::winnt::TRANSACTION_INFORMATION_CLASS, transactioninformation: *mut core::ffi::c_void, transactioninformationlength: u32, returnlength: Option<*mut u32>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwQueryInformationTransaction(transactionhandle : super::winnt::HANDLE, transactioninformationclass : super::winnt::TRANSACTION_INFORMATION_CLASS, transactioninformation : *mut core::ffi::c_void, transactioninformationlength : u32, returnlength : *mut u32) -> super::bcrypt::NTSTATUS);
    unsafe { ZwQueryInformationTransaction(transactionhandle, transactioninformationclass, transactioninformation as _, transactioninformationlength, returnlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwQueryInformationTransactionManager(transactionmanagerhandle: super::winnt::HANDLE, transactionmanagerinformationclass: super::winnt::TRANSACTIONMANAGER_INFORMATION_CLASS, transactionmanagerinformation: *mut core::ffi::c_void, transactionmanagerinformationlength: u32, returnlength: Option<*mut u32>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwQueryInformationTransactionManager(transactionmanagerhandle : super::winnt::HANDLE, transactionmanagerinformationclass : super::winnt::TRANSACTIONMANAGER_INFORMATION_CLASS, transactionmanagerinformation : *mut core::ffi::c_void, transactionmanagerinformationlength : u32, returnlength : *mut u32) -> super::bcrypt::NTSTATUS);
    unsafe { ZwQueryInformationTransactionManager(transactionmanagerhandle, transactionmanagerinformationclass, transactionmanagerinformation as _, transactionmanagerinformationlength, returnlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwQueryKey(keyhandle: super::winnt::HANDLE, keyinformationclass: KEY_INFORMATION_CLASS, keyinformation: Option<*mut core::ffi::c_void>, length: u32, resultlength: *mut u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwQueryKey(keyhandle : super::winnt::HANDLE, keyinformationclass : KEY_INFORMATION_CLASS, keyinformation : *mut core::ffi::c_void, length : u32, resultlength : *mut u32) -> super::bcrypt::NTSTATUS);
    unsafe { ZwQueryKey(keyhandle, keyinformationclass, keyinformation.unwrap_or(core::mem::zeroed()) as _, length, resultlength as _) }
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn ZwQuerySymbolicLinkObject(linkhandle: super::winnt::HANDLE, linktarget: *mut super::ntsecapi::UNICODE_STRING, returnedlength: Option<*mut u32>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwQuerySymbolicLinkObject(linkhandle : super::winnt::HANDLE, linktarget : *mut super::ntsecapi::UNICODE_STRING, returnedlength : *mut u32) -> super::bcrypt::NTSTATUS);
    unsafe { ZwQuerySymbolicLinkObject(linkhandle, linktarget as _, returnedlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn ZwQueryValueKey(keyhandle: super::winnt::HANDLE, valuename: *const super::ntsecapi::UNICODE_STRING, keyvalueinformationclass: KEY_VALUE_INFORMATION_CLASS, keyvalueinformation: Option<*mut core::ffi::c_void>, length: u32, resultlength: *mut u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwQueryValueKey(keyhandle : super::winnt::HANDLE, valuename : *const super::ntsecapi::UNICODE_STRING, keyvalueinformationclass : KEY_VALUE_INFORMATION_CLASS, keyvalueinformation : *mut core::ffi::c_void, length : u32, resultlength : *mut u32) -> super::bcrypt::NTSTATUS);
    unsafe { ZwQueryValueKey(keyhandle, valuename, keyvalueinformationclass, keyvalueinformation.unwrap_or(core::mem::zeroed()) as _, length, resultlength as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn ZwReadFile(filehandle: super::winnt::HANDLE, event: Option<super::winnt::HANDLE>, apcroutine: super::winternl::PIO_APC_ROUTINE, apccontext: Option<*const core::ffi::c_void>, iostatusblock: *mut super::winternl::IO_STATUS_BLOCK, buffer: *mut core::ffi::c_void, length: u32, byteoffset: Option<*const i64>, key: Option<*const u32>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwReadFile(filehandle : super::winnt::HANDLE, event : super::winnt::HANDLE, apcroutine : super::winternl::PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, iostatusblock : *mut super::winternl::IO_STATUS_BLOCK, buffer : *mut core::ffi::c_void, length : u32, byteoffset : *const i64, key : *const u32) -> super::bcrypt::NTSTATUS);
    unsafe { ZwReadFile(filehandle, event.unwrap_or(core::mem::zeroed()) as _, apcroutine, apccontext.unwrap_or(core::mem::zeroed()) as _, iostatusblock as _, buffer as _, length, byteoffset.unwrap_or(core::mem::zeroed()) as _, key.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwReadOnlyEnlistment(enlistmenthandle: super::winnt::HANDLE, tmvirtualclock: Option<*const i64>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwReadOnlyEnlistment(enlistmenthandle : super::winnt::HANDLE, tmvirtualclock : *const i64) -> super::bcrypt::NTSTATUS);
    unsafe { ZwReadOnlyEnlistment(enlistmenthandle, tmvirtualclock.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwRecoverEnlistment(enlistmenthandle: super::winnt::HANDLE, enlistmentkey: Option<*const core::ffi::c_void>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwRecoverEnlistment(enlistmenthandle : super::winnt::HANDLE, enlistmentkey : *const core::ffi::c_void) -> super::bcrypt::NTSTATUS);
    unsafe { ZwRecoverEnlistment(enlistmenthandle, enlistmentkey.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwRecoverResourceManager(resourcemanagerhandle: super::winnt::HANDLE) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwRecoverResourceManager(resourcemanagerhandle : super::winnt::HANDLE) -> super::bcrypt::NTSTATUS);
    unsafe { ZwRecoverResourceManager(resourcemanagerhandle) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwRecoverTransactionManager(transactionmanagerhandle: super::winnt::HANDLE) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwRecoverTransactionManager(transactionmanagerhandle : super::winnt::HANDLE) -> super::bcrypt::NTSTATUS);
    unsafe { ZwRecoverTransactionManager(transactionmanagerhandle) }
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn ZwRenameKey(keyhandle: super::winnt::HANDLE, newname: *const super::ntsecapi::UNICODE_STRING) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwRenameKey(keyhandle : super::winnt::HANDLE, newname : *const super::ntsecapi::UNICODE_STRING) -> super::bcrypt::NTSTATUS);
    unsafe { ZwRenameKey(keyhandle, newname) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwRestoreKey(keyhandle: super::winnt::HANDLE, filehandle: Option<super::winnt::HANDLE>, flags: u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwRestoreKey(keyhandle : super::winnt::HANDLE, filehandle : super::winnt::HANDLE, flags : u32) -> super::bcrypt::NTSTATUS);
    unsafe { ZwRestoreKey(keyhandle, filehandle.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwRollbackComplete(enlistmenthandle: super::winnt::HANDLE, tmvirtualclock: Option<*const i64>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwRollbackComplete(enlistmenthandle : super::winnt::HANDLE, tmvirtualclock : *const i64) -> super::bcrypt::NTSTATUS);
    unsafe { ZwRollbackComplete(enlistmenthandle, tmvirtualclock.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwRollbackEnlistment(enlistmenthandle: super::winnt::HANDLE, tmvirtualclock: Option<*const i64>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwRollbackEnlistment(enlistmenthandle : super::winnt::HANDLE, tmvirtualclock : *const i64) -> super::bcrypt::NTSTATUS);
    unsafe { ZwRollbackEnlistment(enlistmenthandle, tmvirtualclock.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwRollbackTransaction(transactionhandle: super::winnt::HANDLE, wait: bool) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwRollbackTransaction(transactionhandle : super::winnt::HANDLE, wait : bool) -> super::bcrypt::NTSTATUS);
    unsafe { ZwRollbackTransaction(transactionhandle, wait) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwRollforwardTransactionManager(transactionmanagerhandle: super::winnt::HANDLE, tmvirtualclock: Option<*const i64>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwRollforwardTransactionManager(transactionmanagerhandle : super::winnt::HANDLE, tmvirtualclock : *const i64) -> super::bcrypt::NTSTATUS);
    unsafe { ZwRollforwardTransactionManager(transactionmanagerhandle, tmvirtualclock.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwSaveKey(keyhandle: super::winnt::HANDLE, filehandle: super::winnt::HANDLE) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwSaveKey(keyhandle : super::winnt::HANDLE, filehandle : super::winnt::HANDLE) -> super::bcrypt::NTSTATUS);
    unsafe { ZwSaveKey(keyhandle, filehandle) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwSaveKeyEx(keyhandle: super::winnt::HANDLE, filehandle: super::winnt::HANDLE, format: u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwSaveKeyEx(keyhandle : super::winnt::HANDLE, filehandle : super::winnt::HANDLE, format : u32) -> super::bcrypt::NTSTATUS);
    unsafe { ZwSaveKeyEx(keyhandle, filehandle, format) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwSetInformationEnlistment(enlistmenthandle: super::winnt::HANDLE, enlistmentinformationclass: super::winnt::ENLISTMENT_INFORMATION_CLASS, enlistmentinformation: *const core::ffi::c_void, enlistmentinformationlength: u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwSetInformationEnlistment(enlistmenthandle : super::winnt::HANDLE, enlistmentinformationclass : super::winnt::ENLISTMENT_INFORMATION_CLASS, enlistmentinformation : *const core::ffi::c_void, enlistmentinformationlength : u32) -> super::bcrypt::NTSTATUS);
    unsafe { ZwSetInformationEnlistment(enlistmenthandle, enlistmentinformationclass, enlistmentinformation, enlistmentinformationlength) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn ZwSetInformationFile(filehandle: super::winnt::HANDLE, iostatusblock: *mut super::winternl::IO_STATUS_BLOCK, fileinformation: *const core::ffi::c_void, length: u32, fileinformationclass: super::winternl::FILE_INFORMATION_CLASS) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwSetInformationFile(filehandle : super::winnt::HANDLE, iostatusblock : *mut super::winternl::IO_STATUS_BLOCK, fileinformation : *const core::ffi::c_void, length : u32, fileinformationclass : super::winternl::FILE_INFORMATION_CLASS) -> super::bcrypt::NTSTATUS);
    unsafe { ZwSetInformationFile(filehandle, iostatusblock as _, fileinformation, length, fileinformationclass) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn ZwSetInformationKey(keyhandle: super::winnt::HANDLE, keysetinformationclass: super::winternl::KEY_SET_INFORMATION_CLASS, keysetinformation: *const core::ffi::c_void, keysetinformationlength: u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwSetInformationKey(keyhandle : super::winnt::HANDLE, keysetinformationclass : super::winternl::KEY_SET_INFORMATION_CLASS, keysetinformation : *const core::ffi::c_void, keysetinformationlength : u32) -> super::bcrypt::NTSTATUS);
    unsafe { ZwSetInformationKey(keyhandle, keysetinformationclass, keysetinformation, keysetinformationlength) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwSetInformationResourceManager(resourcemanagerhandle: super::winnt::HANDLE, resourcemanagerinformationclass: super::winnt::RESOURCEMANAGER_INFORMATION_CLASS, resourcemanagerinformation: *const core::ffi::c_void, resourcemanagerinformationlength: u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwSetInformationResourceManager(resourcemanagerhandle : super::winnt::HANDLE, resourcemanagerinformationclass : super::winnt::RESOURCEMANAGER_INFORMATION_CLASS, resourcemanagerinformation : *const core::ffi::c_void, resourcemanagerinformationlength : u32) -> super::bcrypt::NTSTATUS);
    unsafe { ZwSetInformationResourceManager(resourcemanagerhandle, resourcemanagerinformationclass, resourcemanagerinformation, resourcemanagerinformationlength) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwSetInformationTransaction(transactionhandle: super::winnt::HANDLE, transactioninformationclass: super::winnt::TRANSACTION_INFORMATION_CLASS, transactioninformation: *const core::ffi::c_void, transactioninformationlength: u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwSetInformationTransaction(transactionhandle : super::winnt::HANDLE, transactioninformationclass : super::winnt::TRANSACTION_INFORMATION_CLASS, transactioninformation : *const core::ffi::c_void, transactioninformationlength : u32) -> super::bcrypt::NTSTATUS);
    unsafe { ZwSetInformationTransaction(transactionhandle, transactioninformationclass, transactioninformation, transactioninformationlength) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwSetInformationTransactionManager(tmhandle: super::winnt::HANDLE, transactionmanagerinformationclass: super::winnt::TRANSACTIONMANAGER_INFORMATION_CLASS, transactionmanagerinformation: *const core::ffi::c_void, transactionmanagerinformationlength: u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwSetInformationTransactionManager(tmhandle : super::winnt::HANDLE, transactionmanagerinformationclass : super::winnt::TRANSACTIONMANAGER_INFORMATION_CLASS, transactionmanagerinformation : *const core::ffi::c_void, transactionmanagerinformationlength : u32) -> super::bcrypt::NTSTATUS);
    unsafe { ZwSetInformationTransactionManager(tmhandle, transactionmanagerinformationclass, transactionmanagerinformation, transactionmanagerinformationlength) }
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[inline]
pub unsafe fn ZwSetValueKey(keyhandle: super::winnt::HANDLE, valuename: *const super::ntsecapi::UNICODE_STRING, titleindex: Option<u32>, r#type: u32, data: Option<*const core::ffi::c_void>, datasize: u32) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwSetValueKey(keyhandle : super::winnt::HANDLE, valuename : *const super::ntsecapi::UNICODE_STRING, titleindex : u32, r#type : u32, data : *const core::ffi::c_void, datasize : u32) -> super::bcrypt::NTSTATUS);
    unsafe { ZwSetValueKey(keyhandle, valuename, titleindex.unwrap_or(core::mem::zeroed()) as _, r#type, data.unwrap_or(core::mem::zeroed()) as _, datasize) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwSinglePhaseReject(enlistmenthandle: super::winnt::HANDLE, tmvirtualclock: Option<*const i64>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwSinglePhaseReject(enlistmenthandle : super::winnt::HANDLE, tmvirtualclock : *const i64) -> super::bcrypt::NTSTATUS);
    unsafe { ZwSinglePhaseReject(enlistmenthandle, tmvirtualclock.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi"))]
#[inline]
pub unsafe fn ZwUnloadDriver(driverservicename: *const super::ntsecapi::UNICODE_STRING) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwUnloadDriver(driverservicename : *const super::ntsecapi::UNICODE_STRING) -> super::bcrypt::NTSTATUS);
    unsafe { ZwUnloadDriver(driverservicename) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[inline]
pub unsafe fn ZwUnmapViewOfSection(processhandle: super::winnt::HANDLE, baseaddress: Option<*const core::ffi::c_void>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwUnmapViewOfSection(processhandle : super::winnt::HANDLE, baseaddress : *const core::ffi::c_void) -> super::bcrypt::NTSTATUS);
    unsafe { ZwUnmapViewOfSection(processhandle, baseaddress.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "bcrypt", feature = "winnt", feature = "winternl"))]
#[inline]
pub unsafe fn ZwWriteFile(filehandle: super::winnt::HANDLE, event: Option<super::winnt::HANDLE>, apcroutine: super::winternl::PIO_APC_ROUTINE, apccontext: Option<*const core::ffi::c_void>, iostatusblock: *mut super::winternl::IO_STATUS_BLOCK, buffer: *const core::ffi::c_void, length: u32, byteoffset: Option<*const i64>, key: Option<*const u32>) -> super::bcrypt::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn ZwWriteFile(filehandle : super::winnt::HANDLE, event : super::winnt::HANDLE, apcroutine : super::winternl::PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, iostatusblock : *mut super::winternl::IO_STATUS_BLOCK, buffer : *const core::ffi::c_void, length : u32, byteoffset : *const i64, key : *const u32) -> super::bcrypt::NTSTATUS);
    unsafe { ZwWriteFile(filehandle, event.unwrap_or(core::mem::zeroed()) as _, apcroutine, apccontext.unwrap_or(core::mem::zeroed()) as _, iostatusblock as _, buffer, length, byteoffset.unwrap_or(core::mem::zeroed()) as _, key.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn vDbgPrintEx<P2>(componentid: u32, level: u32, format: P2, arglist: *const i8) -> u32
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ntdll.dll" "system" fn vDbgPrintEx(componentid : u32, level : u32, format : windows_core::PCSTR, arglist : *const i8) -> u32);
    unsafe { vDbgPrintEx(componentid, level, format.param().abi(), arglist) }
}
#[inline]
pub unsafe fn vDbgPrintExWithPrefix<P0, P3>(prefix: P0, componentid: u32, level: u32, format: P3, arglist: *const i8) -> u32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("ntdll.dll" "system" fn vDbgPrintExWithPrefix(prefix : windows_core::PCSTR, componentid : u32, level : u32, format : windows_core::PCSTR, arglist : *const i8) -> u32);
    unsafe { vDbgPrintExWithPrefix(prefix.param().abi(), componentid, level, format.param().abi(), arglist) }
}
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct ACCESS_STATE {
    pub OperationID: super::winnt::LUID,
    pub SecurityEvaluated: bool,
    pub GenerateAudit: bool,
    pub GenerateOnClose: bool,
    pub PrivilegesAllocated: bool,
    pub Flags: u32,
    pub RemainingDesiredAccess: super::winnt::ACCESS_MASK,
    pub PreviouslyGrantedAccess: super::winnt::ACCESS_MASK,
    pub OriginalDesiredAccess: super::winnt::ACCESS_MASK,
    pub SubjectSecurityContext: SECURITY_SUBJECT_CONTEXT,
    pub SecurityDescriptor: super::winnt::PSECURITY_DESCRIPTOR,
    pub AuxData: *mut core::ffi::c_void,
    pub Privileges: ACCESS_STATE_0,
    pub AuditPrivileges: bool,
    pub ObjectName: super::ntsecapi::UNICODE_STRING,
    pub ObjectTypeName: super::ntsecapi::UNICODE_STRING,
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
impl Default for ACCESS_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union ACCESS_STATE_0 {
    pub InitialPrivilegeSet: INITIAL_PRIVILEGE_SET,
    pub PrivilegeSet: super::winnt::PRIVILEGE_SET,
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
impl Default for ACCESS_STATE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ACPIBus: INTERFACE_TYPE = 17;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug)]
pub struct ACPI_INTERFACE_STANDARD {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
    pub GpeConnectVector: PGPE_CONNECT_VECTOR,
    pub GpeDisconnectVector: PGPE_DISCONNECT_VECTOR,
    pub GpeEnableEvent: PGPE_ENABLE_EVENT,
    pub GpeDisableEvent: PGPE_DISABLE_EVENT,
    pub GpeClearStatus: PGPE_CLEAR_STATUS,
    pub RegisterForDeviceNotifications: PREGISTER_FOR_DEVICE_NOTIFICATIONS,
    pub UnregisterForDeviceNotifications: PUNREGISTER_FOR_DEVICE_NOTIFICATIONS,
}
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for ACPI_INTERFACE_STANDARD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "bcrypt")]
#[derive(Clone, Copy, Debug)]
pub struct ACPI_INTERFACE_STANDARD2 {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
    pub GpeConnectVector: PGPE_CONNECT_VECTOR2,
    pub GpeDisconnectVector: PGPE_DISCONNECT_VECTOR2,
    pub GpeEnableEvent: PGPE_ENABLE_EVENT2,
    pub GpeDisableEvent: PGPE_DISABLE_EVENT2,
    pub GpeClearStatus: PGPE_CLEAR_STATUS2,
    pub RegisterForDeviceNotifications: PREGISTER_FOR_DEVICE_NOTIFICATIONS2,
    pub UnregisterForDeviceNotifications: PUNREGISTER_FOR_DEVICE_NOTIFICATIONS2,
}
#[cfg(feature = "bcrypt")]
impl Default for ACPI_INTERFACE_STANDARD2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ADAPTER_INFO_API_BYPASS: u32 = 2;
pub const ADAPTER_INFO_HYBRID_PASSTHROUGH: u32 = 4;
pub const ADAPTER_INFO_SYNCHRONOUS_CALLBACK: u32 = 1;
pub type ALLOCATE_FUNCTION = Option<unsafe extern "system" fn(pooltype: POOL_TYPE, numberofbytes: usize, tag: u32) -> *mut core::ffi::c_void>;
#[cfg(feature = "winnt")]
pub type ALLOCATE_FUNCTION_EX = Option<unsafe extern "system" fn(pooltype: POOL_TYPE, numberofbytes: usize, tag: u32, lookaside: *mut LOOKASIDE_LIST_EX) -> *mut core::ffi::c_void>;
pub const ALLOC_DATA_PRAGMA: u32 = 1;
pub const ALLOC_PRAGMA: u32 = 1;
pub type ALTERNATIVE_ARCHITECTURE_TYPE = i32;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub union AMD_L1_CACHE_INFO {
    pub Ulong: u32,
    pub Anonymous: AMD_L1_CACHE_INFO_0,
}
#[cfg(target_arch = "x86")]
impl Default for AMD_L1_CACHE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AMD_L1_CACHE_INFO_0 {
    pub LineSize: u8,
    pub LinesPerTag: u8,
    pub Associativity: u8,
    pub Size: u8,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub union AMD_L2_CACHE_INFO {
    pub Ulong: u32,
    pub Anonymous: AMD_L2_CACHE_INFO_0,
}
#[cfg(target_arch = "x86")]
impl Default for AMD_L2_CACHE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AMD_L2_CACHE_INFO_0 {
    pub LineSize: u8,
    pub _bitfield: u8,
    pub Size: u16,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub union AMD_L3_CACHE_INFO {
    pub Ulong: u32,
    pub Anonymous: AMD_L3_CACHE_INFO_0,
}
#[cfg(target_arch = "x86")]
impl Default for AMD_L3_CACHE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AMD_L3_CACHE_INFO_0 {
    pub LineSize: u8,
    pub _bitfield1: u8,
    pub _bitfield2: u16,
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy)]
pub union ARM64_IDCODE {
    pub Ulong: u64,
    pub Anonymous: ARM64_IDCODE_0,
    pub Arm: ARM64_IDCODE_1,
}
#[cfg(target_arch = "aarch64")]
impl Default for ARM64_IDCODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ARM64_IDCODE_0 {
    pub _bitfield: u64,
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ARM64_IDCODE_1 {
    pub _bitfield: u64,
}
#[cfg(target_arch = "aarch64")]
pub const ARM64_PCR_RESERVED_MASK: u32 = 4095;
pub const ATS_DEVICE_SVM_OPTOUT: u32 = 1;
pub const AccessFlagFault: FAULT_INFORMATION_ARM64_TYPE = 5;
pub const AddressSizeFault: FAULT_INFORMATION_ARM64_TYPE = 1;
pub const AllLoggerHandlesClass: TRACE_INFORMATION_CLASS = 6;
pub const AssignSecurityDescriptor: SECURITY_OPERATION_CODE = 3;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BOOTDISK_INFORMATION {
    pub BootPartitionOffset: i64,
    pub SystemPartitionOffset: i64,
    pub BootDeviceSignature: u32,
    pub SystemDeviceSignature: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BOOTDISK_INFORMATION_EX {
    pub BootPartitionOffset: i64,
    pub SystemPartitionOffset: i64,
    pub BootDeviceSignature: u32,
    pub SystemDeviceSignature: u32,
    pub BootDeviceGuid: windows_core::GUID,
    pub SystemDeviceGuid: windows_core::GUID,
    pub BootDeviceIsGpt: bool,
    pub SystemDeviceIsGpt: bool,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BOOTDISK_INFORMATION_LITE {
    pub NumberEntries: u32,
    pub Entries: [LOADER_PARTITION_INFORMATION_EX; 1],
}
impl Default for BOOTDISK_INFORMATION_LITE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
pub type BOUND_CALLBACK = Option<unsafe extern "system" fn() -> BOUND_CALLBACK_STATUS>;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
pub type BOUND_CALLBACK_STATUS = i32;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug)]
pub struct BUS_INTERFACE_STANDARD {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
    pub TranslateBusAddress: PTRANSLATE_BUS_ADDRESS,
    pub GetDmaAdapter: PGET_DMA_ADAPTER,
    pub SetBusData: PGET_SET_DEVICE_DATA,
    pub GetBusData: PGET_SET_DEVICE_DATA,
}
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for BUS_INTERFACE_STANDARD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type BUS_QUERY_ID_TYPE = i32;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "usb"))]
#[derive(Clone, Copy, Debug)]
pub struct BUS_RESOURCE_UPDATE_INTERFACE {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
    pub GetUpdatedBusResource: PGET_UPDATED_BUS_RESOURCE,
}
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "usb"))]
impl Default for BUS_RESOURCE_UPDATE_INTERFACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union BUS_SPECIFIC_RESET_FLAGS {
    pub u: BUS_SPECIFIC_RESET_FLAGS_0,
    pub AsUlonglong: u64,
}
impl Default for BUS_SPECIFIC_RESET_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BUS_SPECIFIC_RESET_FLAGS_0 {
    pub _bitfield: u64,
}
pub const BackgroundWorkQueue: WORK_QUEUE_TYPE = 4;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
pub const BoundExceptionContinueSearch: BOUND_CALLBACK_STATUS = 0;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
pub const BoundExceptionError: BOUND_CALLBACK_STATUS = 2;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
pub const BoundExceptionHandled: BOUND_CALLBACK_STATUS = 1;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
pub const BoundExceptionMaximum: BOUND_CALLBACK_STATUS = 3;
pub const BufferEmpty: KBUGCHECK_BUFFER_DUMP_STATE = 0;
pub const BufferFinished: KBUGCHECK_BUFFER_DUMP_STATE = 3;
pub const BufferIncomplete: KBUGCHECK_BUFFER_DUMP_STATE = 4;
pub const BufferInserted: KBUGCHECK_BUFFER_DUMP_STATE = 1;
pub const BufferStarted: KBUGCHECK_BUFFER_DUMP_STATE = 2;
pub const BusQueryCompatibleIDs: BUS_QUERY_ID_TYPE = 2;
pub const BusQueryContainerID: BUS_QUERY_ID_TYPE = 5;
pub const BusQueryDeviceID: BUS_QUERY_ID_TYPE = 0;
pub const BusQueryDeviceSerialNumber: BUS_QUERY_ID_TYPE = 4;
pub const BusQueryHardwareIDs: BUS_QUERY_ID_TYPE = 1;
pub const BusQueryInstanceID: BUS_QUERY_ID_TYPE = 3;
pub const BusRelations: DEVICE_RELATION_TYPE = 0;
pub type CALLBACK_FUNCTION = Option<unsafe extern "system" fn(callbackcontext: *const core::ffi::c_void, argument1: *const core::ffi::c_void, argument2: *const core::ffi::c_void)>;
pub const CALLBACK_FUNCTION: u32 = 196608;
pub const CBus: INTERFACE_TYPE = 9;
pub const CLFS_LOG_SIZE_MAXIMUM: i32 = -1;
pub const CLFS_LOG_SIZE_MINIMUM: u32 = 0;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CLFS_MGMT_CLIENT(pub *mut core::ffi::c_void);
impl Default for CLFS_MGMT_CLIENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "clfs", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug)]
pub struct CLFS_MGMT_CLIENT_REGISTRATION {
    pub Version: u32,
    pub AdvanceTailCallback: PCLFS_CLIENT_ADVANCE_TAIL_CALLBACK,
    pub AdvanceTailCallbackData: *mut core::ffi::c_void,
    pub LogGrowthCompleteCallback: PCLFS_CLIENT_LFF_HANDLER_COMPLETE_CALLBACK,
    pub LogGrowthCompleteCallbackData: *mut core::ffi::c_void,
    pub LogUnpinnedCallback: PCLFS_CLIENT_LOG_UNPINNED_CALLBACK,
    pub LogUnpinnedCallbackData: *mut core::ffi::c_void,
}
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "clfs", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for CLFS_MGMT_CLIENT_REGISTRATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CLFS_MGMT_CLIENT_REGISTRATION_VERSION: u32 = 1;
#[repr(C)]
#[cfg(feature = "clfs")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLFS_MGMT_NOTIFICATION {
    pub Notification: CLFS_MGMT_NOTIFICATION_TYPE,
    pub Lsn: super::clfs::CLFS_LSN,
    pub LogIsPinned: u16,
}
pub type CLFS_MGMT_NOTIFICATION_TYPE = i32;
pub const CLFS_MGMT_NUM_POLICIES: u32 = 10;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLFS_MGMT_POLICY {
    pub Version: u32,
    pub LengthInBytes: u32,
    pub PolicyFlags: u32,
    pub PolicyType: CLFS_MGMT_POLICY_TYPE,
    pub PolicyParameters: CLFS_MGMT_POLICY_0,
}
impl Default for CLFS_MGMT_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CLFS_MGMT_POLICY_0 {
    pub MaximumSize: CLFS_MGMT_POLICY_0_0,
    pub MinimumSize: CLFS_MGMT_POLICY_0_1,
    pub NewContainerSize: CLFS_MGMT_POLICY_0_2,
    pub GrowthRate: CLFS_MGMT_POLICY_0_3,
    pub LogTail: CLFS_MGMT_POLICY_0_4,
    pub AutoShrink: CLFS_MGMT_POLICY_0_5,
    pub AutoGrow: CLFS_MGMT_POLICY_0_6,
    pub NewContainerPrefix: CLFS_MGMT_POLICY_0_7,
    pub NewContainerSuffix: CLFS_MGMT_POLICY_0_8,
    pub NewContainerExtension: CLFS_MGMT_POLICY_0_9,
}
impl Default for CLFS_MGMT_POLICY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLFS_MGMT_POLICY_0_0 {
    pub Containers: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLFS_MGMT_POLICY_0_1 {
    pub Containers: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLFS_MGMT_POLICY_0_2 {
    pub SizeInBytes: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLFS_MGMT_POLICY_0_3 {
    pub AbsoluteGrowthInContainers: u32,
    pub RelativeGrowthPercentage: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLFS_MGMT_POLICY_0_4 {
    pub MinimumAvailablePercentage: u32,
    pub MinimumAvailableContainers: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLFS_MGMT_POLICY_0_5 {
    pub Percentage: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLFS_MGMT_POLICY_0_6 {
    pub Enabled: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLFS_MGMT_POLICY_0_7 {
    pub PrefixLengthInBytes: u16,
    pub PrefixString: [u16; 1],
}
impl Default for CLFS_MGMT_POLICY_0_7 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLFS_MGMT_POLICY_0_8 {
    pub NextContainerSuffix: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CLFS_MGMT_POLICY_0_9 {
    pub ExtensionLengthInBytes: u16,
    pub ExtensionString: [u16; 1],
}
impl Default for CLFS_MGMT_POLICY_0_9 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CLFS_MGMT_POLICY_TYPE = i32;
pub const CLFS_MGMT_POLICY_VERSION: u32 = 1;
#[cfg(target_arch = "x86")]
pub const CLOCK1_LEVEL: u32 = 28;
#[cfg(target_arch = "x86")]
pub const CLOCK2_LEVEL: u32 = 28;
#[cfg(target_arch = "x86")]
pub const CLOCK_LEVEL: u32 = 28;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const CLOCK_LEVEL: u32 = 13;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
pub const CMCI_LEVEL: u32 = 5;
#[repr(C)]
#[cfg(feature = "basetsd")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CM_COMPONENT_INFORMATION {
    pub Flags: DEVICE_FLAGS,
    pub Version: u32,
    pub Key: u32,
    pub AffinityMask: super::basetsd::KAFFINITY,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CM_DISK_GEOMETRY_DEVICE_DATA {
    pub BytesPerSector: u32,
    pub NumberOfCylinders: u32,
    pub SectorsPerTrack: u32,
    pub NumberOfHeads: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct CM_EISA_FUNCTION_INFORMATION {
    pub CompressedId: u32,
    pub IdSlotFlags1: u8,
    pub IdSlotFlags2: u8,
    pub MinorRevision: u8,
    pub MajorRevision: u8,
    pub Selections: [u8; 26],
    pub FunctionFlags: u8,
    pub TypeString: [u8; 80],
    pub EisaMemory: [EISA_MEMORY_CONFIGURATION; 9],
    pub EisaIrq: [EISA_IRQ_CONFIGURATION; 7],
    pub EisaDma: [EISA_DMA_CONFIGURATION; 4],
    pub EisaPort: [EISA_PORT_CONFIGURATION; 20],
    pub InitializationData: [u8; 60],
}
impl Default for CM_EISA_FUNCTION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct CM_EISA_SLOT_INFORMATION {
    pub ReturnCode: u8,
    pub ReturnFlags: u8,
    pub MajorRevision: u8,
    pub MinorRevision: u8,
    pub Checksum: u16,
    pub NumberFunctions: u8,
    pub FunctionInformation: u8,
    pub CompressedId: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CM_FLOPPY_DEVICE_DATA {
    pub Version: u16,
    pub Revision: u16,
    pub Size: [i8; 8],
    pub MaxDensity: u32,
    pub MountDensity: u32,
    pub StepRateHeadUnloadTime: u8,
    pub HeadLoadTime: u8,
    pub MotorOffTime: u8,
    pub SectorLengthCode: u8,
    pub SectorPerTrack: u8,
    pub ReadWriteGapLength: u8,
    pub DataTransferLength: u8,
    pub FormatGapLength: u8,
    pub FormatFillCharacter: u8,
    pub HeadSettleTime: u8,
    pub MotorSettleTime: u8,
    pub MaximumTrackValue: u8,
    pub DataTransferRate: u8,
}
impl Default for CM_FLOPPY_DEVICE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy)]
pub struct CM_FULL_RESOURCE_DESCRIPTOR {
    pub InterfaceType: INTERFACE_TYPE,
    pub BusNumber: u32,
    pub PartialResourceList: CM_PARTIAL_RESOURCE_LIST,
}
#[cfg(all(feature = "basetsd", feature = "usb"))]
impl Default for CM_FULL_RESOURCE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct CM_INT13_DRIVE_PARAMETER {
    pub DriveSelect: u16,
    pub MaxCylinders: u32,
    pub SectorsPerTrack: u16,
    pub MaxHeads: u16,
    pub NumberDrives: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CM_KEYBOARD_DEVICE_DATA {
    pub Version: u16,
    pub Revision: u16,
    pub Type: u8,
    pub Subtype: u8,
    pub KeyboardFlags: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct CM_MCA_POS_DATA {
    pub AdapterId: u16,
    pub PosData1: u8,
    pub PosData2: u8,
    pub PosData3: u8,
    pub PosData4: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CM_MONITOR_DEVICE_DATA {
    pub Version: u16,
    pub Revision: u16,
    pub HorizontalScreenSize: u16,
    pub VerticalScreenSize: u16,
    pub HorizontalResolution: u16,
    pub VerticalResolution: u16,
    pub HorizontalDisplayTimeLow: u16,
    pub HorizontalDisplayTime: u16,
    pub HorizontalDisplayTimeHigh: u16,
    pub HorizontalBackPorchLow: u16,
    pub HorizontalBackPorch: u16,
    pub HorizontalBackPorchHigh: u16,
    pub HorizontalFrontPorchLow: u16,
    pub HorizontalFrontPorch: u16,
    pub HorizontalFrontPorchHigh: u16,
    pub HorizontalSyncLow: u16,
    pub HorizontalSync: u16,
    pub HorizontalSyncHigh: u16,
    pub VerticalBackPorchLow: u16,
    pub VerticalBackPorch: u16,
    pub VerticalBackPorchHigh: u16,
    pub VerticalFrontPorchLow: u16,
    pub VerticalFrontPorch: u16,
    pub VerticalFrontPorchHigh: u16,
    pub VerticalSyncLow: u16,
    pub VerticalSync: u16,
    pub VerticalSyncHigh: u16,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR {
    pub Type: u8,
    pub ShareDisposition: u8,
    pub Flags: u16,
    pub u: CM_PARTIAL_RESOURCE_DESCRIPTOR_0,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "usb"))]
impl Default for CM_PARTIAL_RESOURCE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy)]
pub union CM_PARTIAL_RESOURCE_DESCRIPTOR_0 {
    pub Generic: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_0,
    pub Port: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_1,
    pub Interrupt: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_2,
    pub MessageInterrupt: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_3,
    pub Memory: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_4,
    pub Dma: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_5,
    pub DmaV3: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_6,
    pub DevicePrivate: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_7,
    pub BusNumber: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_8,
    pub DeviceSpecificData: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_9,
    pub Memory40: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_10,
    pub Memory48: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_11,
    pub Memory64: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_12,
    pub Connection: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_13,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "usb"))]
impl Default for CM_PARTIAL_RESOURCE_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy, Default)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR_0_0 {
    pub Start: super::usb::PHYSICAL_ADDRESS,
    pub Length: u32,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy, Default)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR_0_1 {
    pub Start: super::usb::PHYSICAL_ADDRESS,
    pub Length: u32,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy, Default)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR_0_10 {
    pub Start: super::usb::PHYSICAL_ADDRESS,
    pub Length40: u32,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy, Default)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR_0_11 {
    pub Start: super::usb::PHYSICAL_ADDRESS,
    pub Length48: u32,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy, Default)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR_0_12 {
    pub Start: super::usb::PHYSICAL_ADDRESS,
    pub Length64: u32,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR_0_13 {
    pub Class: u8,
    pub Type: u8,
    pub Reserved1: u8,
    pub Reserved2: u8,
    pub IdLowPart: u32,
    pub IdHighPart: u32,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR_0_2 {
    pub Level: u32,
    pub Vector: u32,
    pub Affinity: super::basetsd::KAFFINITY,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR_0_3 {
    pub Anonymous: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_3_0,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "usb"))]
impl Default for CM_PARTIAL_RESOURCE_DESCRIPTOR_0_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy)]
pub union CM_PARTIAL_RESOURCE_DESCRIPTOR_0_3_0 {
    pub Raw: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_3_0_0,
    pub Translated: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_3_0_1,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "usb"))]
impl Default for CM_PARTIAL_RESOURCE_DESCRIPTOR_0_3_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR_0_3_0_0 {
    pub Reserved: u16,
    pub MessageCount: u16,
    pub Vector: u32,
    pub Affinity: super::basetsd::KAFFINITY,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR_0_3_0_1 {
    pub Level: u32,
    pub Vector: u32,
    pub Affinity: super::basetsd::KAFFINITY,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy, Default)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR_0_4 {
    pub Start: super::usb::PHYSICAL_ADDRESS,
    pub Length: u32,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR_0_5 {
    pub Channel: u32,
    pub Port: u32,
    pub Reserved1: u32,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR_0_6 {
    pub Channel: u32,
    pub RequestLine: u32,
    pub TransferWidth: u8,
    pub Reserved1: u8,
    pub Reserved2: u8,
    pub Reserved3: u8,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR_0_7 {
    pub Data: [u32; 3],
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "usb"))]
impl Default for CM_PARTIAL_RESOURCE_DESCRIPTOR_0_7 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR_0_8 {
    pub Start: u32,
    pub Length: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR_0_9 {
    pub DataSize: u32,
    pub Reserved1: u32,
    pub Reserved2: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR {
    pub Type: u8,
    pub ShareDisposition: u8,
    pub Flags: u16,
    pub u: CM_PARTIAL_RESOURCE_DESCRIPTOR_0,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "usb"))]
impl Default for CM_PARTIAL_RESOURCE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy)]
pub union CM_PARTIAL_RESOURCE_DESCRIPTOR_0 {
    pub Generic: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_0,
    pub Port: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_1,
    pub Interrupt: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_2,
    pub MessageInterrupt: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_3,
    pub Memory: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_4,
    pub Dma: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_5,
    pub DmaV3: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_6,
    pub DevicePrivate: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_7,
    pub BusNumber: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_8,
    pub DeviceSpecificData: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_9,
    pub Memory40: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_10,
    pub Memory48: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_11,
    pub Memory64: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_12,
    pub Connection: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_13,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "usb"))]
impl Default for CM_PARTIAL_RESOURCE_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy, Default)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR_0_0 {
    pub Start: super::usb::PHYSICAL_ADDRESS,
    pub Length: u32,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy, Default)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR_0_1 {
    pub Start: super::usb::PHYSICAL_ADDRESS,
    pub Length: u32,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy, Default)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR_0_10 {
    pub Start: super::usb::PHYSICAL_ADDRESS,
    pub Length40: u32,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy, Default)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR_0_11 {
    pub Start: super::usb::PHYSICAL_ADDRESS,
    pub Length48: u32,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy, Default)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR_0_12 {
    pub Start: super::usb::PHYSICAL_ADDRESS,
    pub Length64: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR_0_13 {
    pub Class: u8,
    pub Type: u8,
    pub Reserved1: u8,
    pub Reserved2: u8,
    pub IdLowPart: u32,
    pub IdHighPart: u32,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy, Default)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR_0_2 {
    pub Level: u32,
    pub Vector: u32,
    pub Affinity: super::basetsd::KAFFINITY,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR_0_3 {
    pub Anonymous: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_3_0,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "usb"))]
impl Default for CM_PARTIAL_RESOURCE_DESCRIPTOR_0_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy)]
pub union CM_PARTIAL_RESOURCE_DESCRIPTOR_0_3_0 {
    pub Raw: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_3_0_0,
    pub Translated: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_3_0_1,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "usb"))]
impl Default for CM_PARTIAL_RESOURCE_DESCRIPTOR_0_3_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy, Default)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR_0_3_0_0 {
    pub Reserved: u16,
    pub MessageCount: u16,
    pub Vector: u32,
    pub Affinity: super::basetsd::KAFFINITY,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy, Default)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR_0_3_0_1 {
    pub Level: u32,
    pub Vector: u32,
    pub Affinity: super::basetsd::KAFFINITY,
}
#[repr(C, packed(4))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy, Default)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR_0_4 {
    pub Start: super::usb::PHYSICAL_ADDRESS,
    pub Length: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR_0_5 {
    pub Channel: u32,
    pub Port: u32,
    pub Reserved1: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR_0_6 {
    pub Channel: u32,
    pub RequestLine: u32,
    pub TransferWidth: u8,
    pub Reserved1: u8,
    pub Reserved2: u8,
    pub Reserved3: u8,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR_0_7 {
    pub Data: [u32; 3],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "usb"))]
impl Default for CM_PARTIAL_RESOURCE_DESCRIPTOR_0_7 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR_0_8 {
    pub Start: u32,
    pub Length: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR_0_9 {
    pub DataSize: u32,
    pub Reserved1: u32,
    pub Reserved2: u32,
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy)]
pub struct CM_PARTIAL_RESOURCE_LIST {
    pub Version: u16,
    pub Revision: u16,
    pub Count: u32,
    pub PartialDescriptors: [CM_PARTIAL_RESOURCE_DESCRIPTOR; 1],
}
#[cfg(all(feature = "basetsd", feature = "usb"))]
impl Default for CM_PARTIAL_RESOURCE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct CM_PNP_BIOS_DEVICE_NODE {
    pub Size: u16,
    pub Node: u8,
    pub ProductId: u32,
    pub DeviceType: [u8; 3],
    pub DeviceAttributes: u16,
}
impl Default for CM_PNP_BIOS_DEVICE_NODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct CM_PNP_BIOS_INSTALLATION_CHECK {
    pub Signature: [u8; 4],
    pub Revision: u8,
    pub Length: u8,
    pub ControlField: u16,
    pub Checksum: u8,
    pub EventFlagAddress: u32,
    pub RealModeEntryOffset: u16,
    pub RealModeEntrySegment: u16,
    pub ProtectedModeEntryOffset: u16,
    pub ProtectedModeCodeBaseAddress: u32,
    pub OemDeviceId: u32,
    pub RealModeDataBaseAddress: u16,
    pub ProtectedModeDataBaseAddress: u32,
}
impl Default for CM_PNP_BIOS_INSTALLATION_CHECK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CM_RESOURCE_CONNECTION_CLASS_FUNCTION_CONFIG: u32 = 3;
pub const CM_RESOURCE_CONNECTION_CLASS_GPIO: u32 = 1;
pub const CM_RESOURCE_CONNECTION_CLASS_SERIAL: u32 = 2;
pub const CM_RESOURCE_CONNECTION_TYPE_FUNCTION_CONFIG: u32 = 1;
pub const CM_RESOURCE_CONNECTION_TYPE_GPIO_IO: u32 = 2;
pub const CM_RESOURCE_CONNECTION_TYPE_SERIAL_I2C: u32 = 1;
pub const CM_RESOURCE_CONNECTION_TYPE_SERIAL_SPI: u32 = 2;
pub const CM_RESOURCE_CONNECTION_TYPE_SERIAL_UART: u32 = 3;
pub const CM_RESOURCE_DMA_16: u32 = 1;
pub const CM_RESOURCE_DMA_32: u32 = 2;
pub const CM_RESOURCE_DMA_8: u32 = 0;
pub const CM_RESOURCE_DMA_8_AND_16: u32 = 4;
pub const CM_RESOURCE_DMA_BUS_MASTER: u32 = 8;
pub const CM_RESOURCE_DMA_TYPE_A: u32 = 16;
pub const CM_RESOURCE_DMA_TYPE_B: u32 = 32;
pub const CM_RESOURCE_DMA_TYPE_F: u32 = 64;
pub const CM_RESOURCE_DMA_V3: u32 = 128;
pub const CM_RESOURCE_INTERRUPT_LATCHED: u32 = 1;
pub const CM_RESOURCE_INTERRUPT_LEVEL_LATCHED_BITS: u32 = 1;
pub const CM_RESOURCE_INTERRUPT_LEVEL_SENSITIVE: u32 = 0;
pub const CM_RESOURCE_INTERRUPT_MESSAGE: u32 = 2;
pub const CM_RESOURCE_INTERRUPT_MESSAGE_TOKEN: u32 = 4294967294;
pub const CM_RESOURCE_INTERRUPT_POLICY_INCLUDED: u32 = 4;
pub const CM_RESOURCE_INTERRUPT_SECONDARY_INTERRUPT: u32 = 16;
pub const CM_RESOURCE_INTERRUPT_WAKE_HINT: u32 = 32;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy)]
pub struct CM_RESOURCE_LIST {
    pub Count: u32,
    pub List: [CM_FULL_RESOURCE_DESCRIPTOR; 1],
}
#[cfg(all(feature = "basetsd", feature = "usb"))]
impl Default for CM_RESOURCE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CM_RESOURCE_MEMORY_24: u32 = 16;
pub const CM_RESOURCE_MEMORY_BAR: u32 = 128;
pub const CM_RESOURCE_MEMORY_CACHEABLE: u32 = 32;
pub const CM_RESOURCE_MEMORY_COMBINEDWRITE: u32 = 8;
pub const CM_RESOURCE_MEMORY_COMPAT_FOR_INACCESSIBLE_RANGE: u32 = 256;
pub const CM_RESOURCE_MEMORY_LARGE: u32 = 3584;
pub const CM_RESOURCE_MEMORY_LARGE_40: u32 = 512;
pub const CM_RESOURCE_MEMORY_LARGE_40_MAXLEN: u64 = 1099511627520;
pub const CM_RESOURCE_MEMORY_LARGE_48: u32 = 1024;
pub const CM_RESOURCE_MEMORY_LARGE_48_MAXLEN: u64 = 281474976645120;
pub const CM_RESOURCE_MEMORY_LARGE_64: u32 = 2048;
pub const CM_RESOURCE_MEMORY_LARGE_64_MAXLEN: u64 = 18446744069414584320;
pub const CM_RESOURCE_MEMORY_PREFETCHABLE: u32 = 4;
pub const CM_RESOURCE_MEMORY_READ_ONLY: u32 = 1;
pub const CM_RESOURCE_MEMORY_READ_WRITE: u32 = 0;
pub const CM_RESOURCE_MEMORY_WINDOW_DECODE: u32 = 64;
pub const CM_RESOURCE_MEMORY_WRITEABILITY_MASK: u32 = 3;
pub const CM_RESOURCE_MEMORY_WRITE_ONLY: u32 = 2;
pub const CM_RESOURCE_PORT_10_BIT_DECODE: u32 = 4;
pub const CM_RESOURCE_PORT_12_BIT_DECODE: u32 = 8;
pub const CM_RESOURCE_PORT_16_BIT_DECODE: u32 = 16;
pub const CM_RESOURCE_PORT_BAR: u32 = 256;
pub const CM_RESOURCE_PORT_IO: u32 = 1;
pub const CM_RESOURCE_PORT_MEMORY: u32 = 0;
pub const CM_RESOURCE_PORT_PASSIVE_DECODE: u32 = 64;
pub const CM_RESOURCE_PORT_POSITIVE_DECODE: u32 = 32;
pub const CM_RESOURCE_PORT_WINDOW_DECODE: u32 = 128;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct CM_RESOURCE_TYPE(pub i32);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CM_ROM_BLOCK {
    pub Address: u32,
    pub Size: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CM_SCSI_DEVICE_DATA {
    pub Version: u16,
    pub Revision: u16,
    pub HostIdentifier: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CM_SERIAL_DEVICE_DATA {
    pub Version: u16,
    pub Revision: u16,
    pub BaudClock: u32,
}
pub type CM_SHARE_DISPOSITION = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CM_SONIC_DEVICE_DATA {
    pub Version: u16,
    pub Revision: u16,
    pub DataConfigurationRegister: u16,
    pub EthernetAddress: [u8; 8],
}
impl Default for CM_SONIC_DEVICE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CM_VIDEO_DEVICE_DATA {
    pub Version: u16,
    pub Revision: u16,
    pub VideoClock: u32,
}
pub const CONNECT_CURRENT_VERSION: u32 = 5;
pub const CONNECT_FULLY_SPECIFIED: u32 = 1;
pub const CONNECT_FULLY_SPECIFIED_GROUP: u32 = 4;
pub const CONNECT_LINE_BASED: u32 = 2;
pub const CONNECT_MESSAGE_BASED: u32 = 3;
pub const CONNECT_MESSAGE_BASED_PASSIVE: u32 = 5;
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
#[derive(Clone, Copy)]
pub struct COUNTED_REASON_CONTEXT {
    pub Version: u32,
    pub Flags: u32,
    pub Anonymous: COUNTED_REASON_CONTEXT_0,
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
impl Default for COUNTED_REASON_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
#[derive(Clone, Copy)]
pub union COUNTED_REASON_CONTEXT_0 {
    pub Anonymous: COUNTED_REASON_CONTEXT_0_0,
    pub SimpleString: super::ntsecapi::UNICODE_STRING,
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
impl Default for COUNTED_REASON_CONTEXT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COUNTED_REASON_CONTEXT_0_0 {
    pub ResourceFileName: super::ntsecapi::UNICODE_STRING,
    pub ResourceReasonId: u16,
    pub StringCount: u32,
    pub ReasonStrings: super::ntsecapi::PUNICODE_STRING,
}
#[repr(C)]
#[cfg(feature = "bcrypt")]
#[derive(Clone, Copy, Debug)]
pub struct CRASHDUMP_FUNCTIONS_INTERFACE {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
    pub PowerOn: PCRASHDUMP_POWER_ON,
}
#[cfg(feature = "bcrypt")]
impl Default for CRASHDUMP_FUNCTIONS_INTERFACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CREATE_FILE_TYPE = i32;
pub const ClfsMgmtAdvanceTailNotification: CLFS_MGMT_NOTIFICATION_TYPE = 0;
pub const ClfsMgmtLogFullHandlerNotification: CLFS_MGMT_NOTIFICATION_TYPE = 1;
pub const ClfsMgmtLogUnpinnedNotification: CLFS_MGMT_NOTIFICATION_TYPE = 2;
pub const ClfsMgmtLogWriteNotification: CLFS_MGMT_NOTIFICATION_TYPE = 3;
pub const ClfsMgmtPolicyAutoGrow: CLFS_MGMT_POLICY_TYPE = 6;
pub const ClfsMgmtPolicyAutoShrink: CLFS_MGMT_POLICY_TYPE = 5;
pub const ClfsMgmtPolicyGrowthRate: CLFS_MGMT_POLICY_TYPE = 3;
pub const ClfsMgmtPolicyInvalid: CLFS_MGMT_POLICY_TYPE = 10;
pub const ClfsMgmtPolicyLogTail: CLFS_MGMT_POLICY_TYPE = 4;
pub const ClfsMgmtPolicyMaximumSize: CLFS_MGMT_POLICY_TYPE = 0;
pub const ClfsMgmtPolicyMinimumSize: CLFS_MGMT_POLICY_TYPE = 1;
pub const ClfsMgmtPolicyNewContainerExtension: CLFS_MGMT_POLICY_TYPE = 9;
pub const ClfsMgmtPolicyNewContainerPrefix: CLFS_MGMT_POLICY_TYPE = 7;
pub const ClfsMgmtPolicyNewContainerSize: CLFS_MGMT_POLICY_TYPE = 2;
pub const ClfsMgmtPolicyNewContainerSuffix: CLFS_MGMT_POLICY_TYPE = 8;
pub const CmResourceShareDeviceExclusive: CM_SHARE_DISPOSITION = 1;
pub const CmResourceShareDriverExclusive: CM_SHARE_DISPOSITION = 2;
pub const CmResourceShareShared: CM_SHARE_DISPOSITION = 3;
pub const CmResourceShareUndetermined: CM_SHARE_DISPOSITION = 0;
pub const CmResourceTypeBusNumber: u32 = 6;
pub const CmResourceTypeConfigData: u32 = 128;
pub const CmResourceTypeConnection: u32 = 132;
pub const CmResourceTypeDevicePrivate: u32 = 129;
pub const CmResourceTypeDeviceSpecific: u32 = 5;
pub const CmResourceTypeDma: u32 = 4;
pub const CmResourceTypeInterrupt: u32 = 2;
pub const CmResourceTypeMemory: u32 = 3;
pub const CmResourceTypeMemoryLarge: u32 = 7;
pub const CmResourceTypeMfCardConfig: u32 = 131;
pub const CmResourceTypeNonArbitrated: u32 = 128;
pub const CmResourceTypeNull: u32 = 0;
pub const CmResourceTypePcCardConfig: u32 = 130;
pub const CmResourceTypePort: u32 = 1;
pub const CommonBufferConfigTypeHardwareAccessPermissions: DMA_COMMON_BUFFER_EXTENDED_CONFIGURATION_TYPE = 2;
pub const CommonBufferConfigTypeLogicalAddressLimits: DMA_COMMON_BUFFER_EXTENDED_CONFIGURATION_TYPE = 0;
pub const CommonBufferConfigTypeMax: DMA_COMMON_BUFFER_EXTENDED_CONFIGURATION_TYPE = 3;
pub const CommonBufferConfigTypeSubSection: DMA_COMMON_BUFFER_EXTENDED_CONFIGURATION_TYPE = 1;
pub const CommonBufferHardwareAccessMax: DMA_COMMON_BUFFER_EXTENDED_CONFIGURATION_ACCESS_TYPE = 3;
pub const CommonBufferHardwareAccessReadOnly: DMA_COMMON_BUFFER_EXTENDED_CONFIGURATION_ACCESS_TYPE = 0;
pub const CommonBufferHardwareAccessReadWrite: DMA_COMMON_BUFFER_EXTENDED_CONFIGURATION_ACCESS_TYPE = 2;
pub const CommonBufferHardwareAccessWriteOnly: DMA_COMMON_BUFFER_EXTENDED_CONFIGURATION_ACCESS_TYPE = 1;
pub const Compatible: DMA_SPEED = 0;
pub const ContinueCompletion: IO_COMPLETION_ROUTINE_RESULT = 0;
pub const CreateFileTypeMailslot: CREATE_FILE_TYPE = 2;
pub const CreateFileTypeNamedPipe: CREATE_FILE_TYPE = 1;
pub const CreateFileTypeNone: CREATE_FILE_TYPE = 0;
pub const CriticalWorkQueue: WORK_QUEUE_TYPE = 0;
pub const CustomPriorityWorkQueue: WORK_QUEUE_TYPE = 32;
#[repr(C)]
#[cfg(feature = "bcrypt")]
#[derive(Clone, Copy, Debug)]
pub struct D3COLD_AUX_POWER_AND_TIMING_INTERFACE {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
    pub RequestCorePowerRail: PD3COLD_REQUEST_CORE_POWER_RAIL,
    pub RequestAuxPower: PD3COLD_REQUEST_AUX_POWER,
    pub RequestPerstDelay: PD3COLD_REQUEST_PERST_DELAY,
}
#[cfg(feature = "bcrypt")]
impl Default for D3COLD_AUX_POWER_AND_TIMING_INTERFACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type D3COLD_LAST_TRANSITION_STATUS = i32;
#[cfg(feature = "bcrypt")]
pub type D3COLD_REQUEST_AUX_POWER = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, auxpowerinmilliwatts: u32, retryinseconds: *mut u32) -> super::bcrypt::NTSTATUS>;
pub type D3COLD_REQUEST_CORE_POWER_RAIL = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, corepowerrailneeded: bool)>;
#[cfg(feature = "bcrypt")]
pub type D3COLD_REQUEST_PERST_DELAY = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, delayinmicroseconds: u32) -> super::bcrypt::NTSTATUS>;
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[derive(Clone, Copy, Debug)]
pub struct D3COLD_SUPPORT_INTERFACE {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
    pub SetD3ColdSupport: PSET_D3COLD_SUPPORT,
    pub GetIdleWakeInfo: PGET_IDLE_WAKE_INFO,
    pub GetD3ColdCapability: PGET_D3COLD_CAPABILITY,
    pub GetBusDriverD3ColdSupport: PGET_D3COLD_CAPABILITY,
    pub GetLastTransitionStatus: PGET_D3COLD_LAST_TRANSITION_STATUS,
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
impl Default for D3COLD_SUPPORT_INTERFACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const D3COLD_SUPPORT_INTERFACE_VERSION: u32 = 1;
pub const DBG_STATUS_BUGCHECK_FIRST: u32 = 3;
pub const DBG_STATUS_BUGCHECK_SECOND: u32 = 4;
pub const DBG_STATUS_CONTROL_C: u32 = 1;
pub const DBG_STATUS_DEBUG_CONTROL: u32 = 6;
pub const DBG_STATUS_FATAL: u32 = 5;
pub const DBG_STATUS_SYSRQ: u32 = 2;
pub const DBG_STATUS_WORKER: u32 = 7;
#[cfg(feature = "bcrypt")]
pub type DEVICE_BUS_SPECIFIC_RESET_HANDLER = Option<unsafe extern "system" fn(interfacecontext: *const core::ffi::c_void, bustype: *const windows_core::GUID, resettypeselected: DEVICE_BUS_SPECIFIC_RESET_TYPE, flags: *const BUS_SPECIFIC_RESET_FLAGS, resetparameters: *const core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DEVICE_BUS_SPECIFIC_RESET_INFO {
    pub BusTypeGuid: windows_core::GUID,
    pub ResetTypeSupported: DEVICE_BUS_SPECIFIC_RESET_TYPE,
}
impl Default for DEVICE_BUS_SPECIFIC_RESET_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DEVICE_BUS_SPECIFIC_RESET_TYPE {
    pub Pci: DEVICE_BUS_SPECIFIC_RESET_TYPE_0,
    pub Acpi: DEVICE_BUS_SPECIFIC_RESET_TYPE_1,
    pub AsULONGLONG: u64,
}
impl Default for DEVICE_BUS_SPECIFIC_RESET_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DEVICE_BUS_SPECIFIC_RESET_TYPE_0 {
    pub _bitfield: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DEVICE_BUS_SPECIFIC_RESET_TYPE_1 {
    pub _bitfield: u64,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DEVICE_CAPABILITIES {
    pub Size: u16,
    pub Version: u16,
    pub _bitfield: u32,
    pub Address: u32,
    pub UINumber: u32,
    pub DeviceState: [super::winnt::DEVICE_POWER_STATE; 7],
    pub SystemWake: super::winnt::SYSTEM_POWER_STATE,
    pub DeviceWake: super::winnt::DEVICE_POWER_STATE,
    pub D1Latency: u32,
    pub D2Latency: u32,
    pub D3Latency: u32,
}
#[cfg(feature = "winnt")]
impl Default for DEVICE_CAPABILITIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DEVICE_CHANGE_COMPLETE_CALLBACK = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void)>;
#[repr(C)]
#[cfg(feature = "usb")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DEVICE_DESCRIPTION {
    pub Version: u32,
    pub Master: bool,
    pub ScatterGather: bool,
    pub DemandMode: bool,
    pub AutoInitialize: bool,
    pub Dma32BitAddresses: bool,
    pub IgnoreCount: bool,
    pub Reserved1: bool,
    pub Dma64BitAddresses: bool,
    pub BusNumber: u32,
    pub DmaChannel: u32,
    pub InterfaceType: INTERFACE_TYPE,
    pub DmaWidth: DMA_WIDTH,
    pub DmaSpeed: DMA_SPEED,
    pub MaximumLength: u32,
    pub DmaPort: u32,
    pub DmaAddressWidth: u32,
    pub DmaControllerInstance: u32,
    pub DmaRequestLine: u32,
    pub DeviceAddress: super::usb::PHYSICAL_ADDRESS,
}
pub const DEVICE_DESCRIPTION_VERSION: u32 = 0;
pub const DEVICE_DESCRIPTION_VERSION1: u32 = 1;
pub const DEVICE_DESCRIPTION_VERSION2: u32 = 2;
pub const DEVICE_DESCRIPTION_VERSION3: u32 = 3;
pub type DEVICE_DIRECTORY_TYPE = i32;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DEVICE_FAULT_CONFIGURATION {
    pub FaultHandler: PIOMMU_DEVICE_FAULT_HANDLER,
    pub FaultContext: *mut core::ffi::c_void,
}
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for DEVICE_FAULT_CONFIGURATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DEVICE_FLAGS {
    pub _bitfield: u32,
}
pub type DEVICE_INSTALL_STATE = i32;
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DEVICE_INTERFACE_CHANGE_NOTIFICATION {
    pub Version: u16,
    pub Size: u16,
    pub Event: windows_core::GUID,
    pub InterfaceClassGuid: windows_core::GUID,
    pub SymbolicLinkName: super::ntsecapi::PUNICODE_STRING,
}
pub const DEVICE_INTERFACE_INCLUDE_NONACTIVE: u32 = 1;
#[repr(C, align(8))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub struct DEVICE_OBJECT {
    pub Type: super::ntdef::CSHORT,
    pub Size: u16,
    pub ReferenceCount: i32,
    pub DriverObject: *mut DRIVER_OBJECT,
    pub NextDevice: *mut Self,
    pub AttachedDevice: *mut Self,
    pub CurrentIrp: *mut IRP,
    pub Timer: PIO_TIMER,
    pub Flags: u32,
    pub Characteristics: u32,
    pub Vpb: PVPB,
    pub DeviceExtension: *mut core::ffi::c_void,
    pub DeviceType: u32,
    pub StackSize: super::winnt::CCHAR,
    pub Queue: DEVICE_OBJECT_0,
    pub AlignmentRequirement: u32,
    pub DeviceQueue: KDEVICE_QUEUE,
    pub Dpc: KDPC,
    pub ActiveThreadCount: u32,
    pub SecurityDescriptor: super::winnt::PSECURITY_DESCRIPTOR,
    pub DeviceLock: KEVENT,
    pub SectorSize: u16,
    pub Spare1: u16,
    pub DeviceObjectExtension: *mut DEVOBJ_EXTENSION,
    pub Reserved: *mut core::ffi::c_void,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for DEVICE_OBJECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub union DEVICE_OBJECT_0 {
    pub ListEntry: super::winnt::LIST_ENTRY,
    pub Wcb: WAIT_CONTEXT_BLOCK,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for DEVICE_OBJECT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, align(16))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub struct DEVICE_OBJECT {
    pub Type: super::ntdef::CSHORT,
    pub Size: u16,
    pub ReferenceCount: i32,
    pub DriverObject: *mut DRIVER_OBJECT,
    pub NextDevice: *mut Self,
    pub AttachedDevice: *mut Self,
    pub CurrentIrp: *mut IRP,
    pub Timer: PIO_TIMER,
    pub Flags: u32,
    pub Characteristics: u32,
    pub Vpb: PVPB,
    pub DeviceExtension: *mut core::ffi::c_void,
    pub DeviceType: u32,
    pub StackSize: super::winnt::CCHAR,
    pub Queue: DEVICE_OBJECT_0,
    pub AlignmentRequirement: u32,
    pub DeviceQueue: KDEVICE_QUEUE,
    pub Dpc: KDPC,
    pub ActiveThreadCount: u32,
    pub SecurityDescriptor: super::winnt::PSECURITY_DESCRIPTOR,
    pub DeviceLock: KEVENT,
    pub SectorSize: u16,
    pub Spare1: u16,
    pub DeviceObjectExtension: *mut DEVOBJ_EXTENSION,
    pub Reserved: *mut core::ffi::c_void,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for DEVICE_OBJECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub union DEVICE_OBJECT_0 {
    pub ListEntry: super::winnt::LIST_ENTRY,
    pub Wcb: WAIT_CONTEXT_BLOCK,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for DEVICE_OBJECT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "bcrypt")]
pub type DEVICE_QUERY_BUS_SPECIFIC_RESET_HANDLER = Option<unsafe extern "system" fn(interfacecontext: *const core::ffi::c_void, resetinfocount: *mut u32, resetinfosupported: *mut DEVICE_BUS_SPECIFIC_RESET_INFO) -> super::bcrypt::NTSTATUS>;
pub type DEVICE_REGISTRY_PROPERTY = i32;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DEVICE_RELATIONS {
    pub Count: u32,
    pub Objects: [PDEVICE_OBJECT; 1],
}
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for DEVICE_RELATIONS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DEVICE_RELATION_TYPE = i32;
pub type DEVICE_REMOVAL_POLICY = i32;
#[cfg(feature = "bcrypt")]
pub type DEVICE_RESET_COMPLETION = Option<unsafe extern "system" fn(status: super::bcrypt::NTSTATUS, context: *mut core::ffi::c_void)>;
#[cfg(feature = "bcrypt")]
pub type DEVICE_RESET_HANDLER = Option<unsafe extern "system" fn(interfacecontext: *const core::ffi::c_void, resettype: DEVICE_RESET_TYPE, flags: u32, resetparameters: *const core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
#[repr(C)]
#[cfg(feature = "bcrypt")]
#[derive(Clone, Copy, Debug)]
pub struct DEVICE_RESET_INTERFACE_STANDARD {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
    pub DeviceReset: PDEVICE_RESET_HANDLER,
    pub SupportedResetTypes: u32,
    pub Reserved: *mut core::ffi::c_void,
    pub QueryBusSpecificResetInfo: PDEVICE_QUERY_BUS_SPECIFIC_RESET_HANDLER,
    pub DeviceBusSpecificReset: PDEVICE_BUS_SPECIFIC_RESET_HANDLER,
    pub GetDeviceResetStatus: PGET_DEVICE_RESET_STATUS,
}
#[cfg(feature = "bcrypt")]
impl Default for DEVICE_RESET_INTERFACE_STANDARD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DEVICE_RESET_INTERFACE_VERSION: u32 = 1;
pub const DEVICE_RESET_INTERFACE_VERSION_1: u32 = 1;
pub const DEVICE_RESET_INTERFACE_VERSION_2: u32 = 2;
pub const DEVICE_RESET_INTERFACE_VERSION_3: u32 = 3;
#[repr(C)]
#[derive(Clone, Copy)]
pub union DEVICE_RESET_STATUS_FLAGS {
    pub u: DEVICE_RESET_STATUS_FLAGS_0,
    pub AsUlonglong: u64,
}
impl Default for DEVICE_RESET_STATUS_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DEVICE_RESET_STATUS_FLAGS_0 {
    pub _bitfield: u64,
}
pub type DEVICE_RESET_TYPE = i32;
pub type DEVICE_TEXT_TYPE = i32;
pub type DEVICE_USAGE_NOTIFICATION_TYPE = i32;
pub type DEVICE_WAKE_DEPTH = i32;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub const DEVICE_WITH_IRP_EXTENSION: PDEVICE_OBJECT = -1 as _;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DEVOBJ_EXTENSION {
    pub Type: super::ntdef::CSHORT,
    pub Size: u16,
    pub DeviceObject: PDEVICE_OBJECT,
    pub PowerFlags: u32,
    pub Dope: *mut _DEVICE_OBJECT_POWER_EXTENSION,
    pub ExtensionFlags: u32,
    pub DeviceNode: *mut core::ffi::c_void,
    pub AttachedTo: PDEVICE_OBJECT,
    pub StartIoCount: i32,
    pub StartIoKey: i32,
    pub StartIoFlags: u32,
    pub Vpb: PVPB,
    pub DependencyNode: *mut core::ffi::c_void,
    pub InterruptContext: *mut core::ffi::c_void,
    pub InterruptCount: i32,
    pub VerifierContext: *mut core::ffi::c_void,
}
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for DEVOBJ_EXTENSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DIRECTORY_ALL_ACCESS: u32 = 983055;
pub const DIRECTORY_CREATE_OBJECT: u32 = 4;
pub const DIRECTORY_CREATE_SUBDIRECTORY: u32 = 8;
pub type DIRECTORY_NOTIFY_INFORMATION_CLASS = i32;
pub const DIRECTORY_QUERY: u32 = 1;
pub const DIRECTORY_TRAVERSE: u32 = 2;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct DISPATCHER_HEADER {
    pub Anonymous: DISPATCHER_HEADER_0,
    pub SignalState: i32,
    pub WaitListHead: super::winnt::LIST_ENTRY,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
impl Default for DISPATCHER_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union DISPATCHER_HEADER_0 {
    pub Anonymous: DISPATCHER_HEADER_0_0,
    pub Anonymous2: DISPATCHER_HEADER_0_1,
    pub Anonymous3: DISPATCHER_HEADER_0_2,
    pub Anonymous4: DISPATCHER_HEADER_0_3,
    pub Anonymous5: DISPATCHER_HEADER_0_4,
    pub Anonymous6: DISPATCHER_HEADER_0_5,
    pub Anonymous7: DISPATCHER_HEADER_0_6,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
impl Default for DISPATCHER_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union DISPATCHER_HEADER_0_0 {
    pub Lock: i32,
    pub LockNV: i32,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
impl Default for DISPATCHER_HEADER_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DISPATCHER_HEADER_0_1 {
    pub Type: u8,
    pub Signalling: u8,
    pub Size: u8,
    pub Reserved1: u8,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct DISPATCHER_HEADER_0_2 {
    pub TimerType: u8,
    pub Anonymous: DISPATCHER_HEADER_0_2_0,
    pub Hand: u8,
    pub Anonymous2: DISPATCHER_HEADER_0_2_1,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
impl Default for DISPATCHER_HEADER_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union DISPATCHER_HEADER_0_2_0 {
    pub TimerControlFlags: u8,
    pub Anonymous: DISPATCHER_HEADER_0_2_0_0,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
impl Default for DISPATCHER_HEADER_0_2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DISPATCHER_HEADER_0_2_0_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union DISPATCHER_HEADER_0_2_1 {
    pub TimerMiscFlags: u8,
    pub Anonymous: DISPATCHER_HEADER_0_2_1_0,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
impl Default for DISPATCHER_HEADER_0_2_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DISPATCHER_HEADER_0_2_1_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct DISPATCHER_HEADER_0_3 {
    pub Timer2Type: u8,
    pub Anonymous: DISPATCHER_HEADER_0_3_0,
    pub Timer2ComponentId: u8,
    pub Timer2RelativeId: u8,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
impl Default for DISPATCHER_HEADER_0_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union DISPATCHER_HEADER_0_3_0 {
    pub Timer2Flags: u8,
    pub Anonymous: DISPATCHER_HEADER_0_3_0_0,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
impl Default for DISPATCHER_HEADER_0_3_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DISPATCHER_HEADER_0_3_0_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct DISPATCHER_HEADER_0_4 {
    pub QueueType: u8,
    pub Anonymous: DISPATCHER_HEADER_0_4_0,
    pub QueueSize: u8,
    pub QueueReserved: u8,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
impl Default for DISPATCHER_HEADER_0_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union DISPATCHER_HEADER_0_4_0 {
    pub QueueControlFlags: u8,
    pub Anonymous: DISPATCHER_HEADER_0_4_0_0,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
impl Default for DISPATCHER_HEADER_0_4_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DISPATCHER_HEADER_0_4_0_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct DISPATCHER_HEADER_0_5 {
    pub ThreadType: u8,
    pub ThreadReserved: u8,
    pub Anonymous: DISPATCHER_HEADER_0_5_0,
    pub Anonymous2: DISPATCHER_HEADER_0_5_1,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
impl Default for DISPATCHER_HEADER_0_5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union DISPATCHER_HEADER_0_5_0 {
    pub ThreadControlFlags: u8,
    pub Anonymous: DISPATCHER_HEADER_0_5_0_0,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
impl Default for DISPATCHER_HEADER_0_5_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DISPATCHER_HEADER_0_5_0_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union DISPATCHER_HEADER_0_5_1 {
    pub DebugActive: u8,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
impl Default for DISPATCHER_HEADER_0_5_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DISPATCHER_HEADER_0_6 {
    pub MutantType: u8,
    pub MutantSize: u8,
    pub DpcActive: bool,
    pub MutantReserved: u8,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct DISPATCHER_HEADER {
    pub Anonymous: DISPATCHER_HEADER_0,
    pub SignalState: i32,
    pub WaitListHead: super::winnt::LIST_ENTRY,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
impl Default for DISPATCHER_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union DISPATCHER_HEADER_0 {
    pub Anonymous: DISPATCHER_HEADER_0_0,
    pub Anonymous2: DISPATCHER_HEADER_0_1,
    pub Anonymous3: DISPATCHER_HEADER_0_2,
    pub Anonymous4: DISPATCHER_HEADER_0_3,
    pub Anonymous5: DISPATCHER_HEADER_0_4,
    pub Anonymous6: DISPATCHER_HEADER_0_5,
    pub Anonymous7: DISPATCHER_HEADER_0_6,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
impl Default for DISPATCHER_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union DISPATCHER_HEADER_0_0 {
    pub Lock: i32,
    pub LockNV: i32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
impl Default for DISPATCHER_HEADER_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DISPATCHER_HEADER_0_1 {
    pub Type: u8,
    pub Signalling: u8,
    pub Size: u8,
    pub Reserved1: u8,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct DISPATCHER_HEADER_0_2 {
    pub TimerType: u8,
    pub Anonymous: DISPATCHER_HEADER_0_2_0,
    pub Hand: u8,
    pub Anonymous2: DISPATCHER_HEADER_0_2_1,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
impl Default for DISPATCHER_HEADER_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union DISPATCHER_HEADER_0_2_0 {
    pub TimerControlFlags: u8,
    pub Anonymous: DISPATCHER_HEADER_0_2_0_0,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
impl Default for DISPATCHER_HEADER_0_2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DISPATCHER_HEADER_0_2_0_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union DISPATCHER_HEADER_0_2_1 {
    pub TimerMiscFlags: u8,
    pub Anonymous: DISPATCHER_HEADER_0_2_1_0,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
impl Default for DISPATCHER_HEADER_0_2_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DISPATCHER_HEADER_0_2_1_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct DISPATCHER_HEADER_0_3 {
    pub Timer2Type: u8,
    pub Anonymous: DISPATCHER_HEADER_0_3_0,
    pub Timer2ComponentId: u8,
    pub Timer2RelativeId: u8,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
impl Default for DISPATCHER_HEADER_0_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union DISPATCHER_HEADER_0_3_0 {
    pub Timer2Flags: u8,
    pub Anonymous: DISPATCHER_HEADER_0_3_0_0,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
impl Default for DISPATCHER_HEADER_0_3_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DISPATCHER_HEADER_0_3_0_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct DISPATCHER_HEADER_0_4 {
    pub QueueType: u8,
    pub Anonymous: DISPATCHER_HEADER_0_4_0,
    pub QueueSize: u8,
    pub QueueReserved: u8,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
impl Default for DISPATCHER_HEADER_0_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union DISPATCHER_HEADER_0_4_0 {
    pub QueueControlFlags: u8,
    pub Anonymous: DISPATCHER_HEADER_0_4_0_0,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
impl Default for DISPATCHER_HEADER_0_4_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DISPATCHER_HEADER_0_4_0_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct DISPATCHER_HEADER_0_5 {
    pub ThreadType: u8,
    pub ThreadReserved: u8,
    pub Anonymous: DISPATCHER_HEADER_0_5_0,
    pub Anonymous2: DISPATCHER_HEADER_0_5_1,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
impl Default for DISPATCHER_HEADER_0_5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union DISPATCHER_HEADER_0_5_0 {
    pub ThreadControlFlags: u8,
    pub Anonymous: DISPATCHER_HEADER_0_5_0_0,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
impl Default for DISPATCHER_HEADER_0_5_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DISPATCHER_HEADER_0_5_0_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union DISPATCHER_HEADER_0_5_1 {
    pub DebugActive: u8,
    pub Anonymous: DISPATCHER_HEADER_0_5_1_0,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
impl Default for DISPATCHER_HEADER_0_5_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DISPATCHER_HEADER_0_5_1_0 {
    pub _bitfield: bool,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DISPATCHER_HEADER_0_6 {
    pub MutantType: u8,
    pub MutantSize: u8,
    pub DpcActive: bool,
    pub MutantReserved: u8,
}
pub const DMAV3_TRANFER_WIDTH_128: u32 = 4;
pub const DMAV3_TRANFER_WIDTH_16: u32 = 1;
pub const DMAV3_TRANFER_WIDTH_256: u32 = 5;
pub const DMAV3_TRANFER_WIDTH_32: u32 = 2;
pub const DMAV3_TRANFER_WIDTH_64: u32 = 3;
pub const DMAV3_TRANFER_WIDTH_8: u32 = 0;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DMA_ADAPTER {
    pub Version: u16,
    pub Size: u16,
    pub DmaOperations: PDMA_OPERATIONS,
}
#[repr(C)]
#[cfg(feature = "usb")]
#[derive(Clone, Copy)]
pub union DMA_ADAPTER_INFO {
    pub Anonymous: DMA_ADAPTER_INFO_0,
    pub Anonymous2: DMA_ADAPTER_INFO_1,
}
#[cfg(feature = "usb")]
impl Default for DMA_ADAPTER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "usb")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DMA_ADAPTER_INFO_0 {
    pub Version: u32,
    pub V1: DMA_ADAPTER_INFO_V1,
}
#[repr(C)]
#[cfg(feature = "usb")]
#[derive(Clone, Copy)]
pub struct DMA_ADAPTER_INFO_1 {
    pub Reserved: u64,
    pub Anonymous: DMA_ADAPTER_INFO_1_0,
}
#[cfg(feature = "usb")]
impl Default for DMA_ADAPTER_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "usb")]
#[derive(Clone, Copy)]
pub union DMA_ADAPTER_INFO_1_0 {
    pub Crashdump: DMA_ADAPTER_INFO_CRASHDUMP,
}
#[cfg(feature = "usb")]
impl Default for DMA_ADAPTER_INFO_1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "usb")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DMA_ADAPTER_INFO_CRASHDUMP {
    pub DeviceDescription: DEVICE_DESCRIPTION,
    pub DeviceIdSize: usize,
    pub DeviceId: *mut core::ffi::c_void,
}
#[cfg(feature = "usb")]
impl Default for DMA_ADAPTER_INFO_CRASHDUMP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DMA_ADAPTER_INFO_V1 {
    pub ReadDmaCounterAvailable: u32,
    pub ScatterGatherLimit: u32,
    pub DmaAddressWidth: u32,
    pub Flags: u32,
    pub MinimumTransferUnit: u32,
}
pub const DMA_ADAPTER_INFO_VERSION1: u32 = 1;
pub const DMA_ADAPTER_INFO_VERSION_CRASHDUMP: i32 = -1;
#[repr(C)]
#[cfg(feature = "usb")]
#[derive(Clone, Copy)]
pub struct DMA_COMMON_BUFFER_EXTENDED_CONFIGURATION {
    pub ConfigType: DMA_COMMON_BUFFER_EXTENDED_CONFIGURATION_TYPE,
    pub Anonymous: DMA_COMMON_BUFFER_EXTENDED_CONFIGURATION_0,
}
#[cfg(feature = "usb")]
impl Default for DMA_COMMON_BUFFER_EXTENDED_CONFIGURATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "usb")]
#[derive(Clone, Copy)]
pub union DMA_COMMON_BUFFER_EXTENDED_CONFIGURATION_0 {
    pub LogicalAddressLimits: DMA_COMMON_BUFFER_EXTENDED_CONFIGURATION_0_0,
    pub SubSection: DMA_COMMON_BUFFER_EXTENDED_CONFIGURATION_0_1,
    pub HardwareAccessType: DMA_COMMON_BUFFER_EXTENDED_CONFIGURATION_ACCESS_TYPE,
    pub Reserved: [u64; 4],
}
#[cfg(feature = "usb")]
impl Default for DMA_COMMON_BUFFER_EXTENDED_CONFIGURATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "usb")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DMA_COMMON_BUFFER_EXTENDED_CONFIGURATION_0_0 {
    pub MinimumAddress: super::usb::PHYSICAL_ADDRESS,
    pub MaximumAddress: super::usb::PHYSICAL_ADDRESS,
}
#[repr(C)]
#[cfg(feature = "usb")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DMA_COMMON_BUFFER_EXTENDED_CONFIGURATION_0_1 {
    pub Offset: u64,
    pub Length: u32,
}
pub type DMA_COMMON_BUFFER_EXTENDED_CONFIGURATION_ACCESS_TYPE = i32;
pub type DMA_COMMON_BUFFER_EXTENDED_CONFIGURATION_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DMA_COMMON_BUFFER_VECTOR(pub u8);
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type DMA_COMPLETION_ROUTINE = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, deviceobject: *const DEVICE_OBJECT, completioncontext: *const core::ffi::c_void, status: DMA_COMPLETION_STATUS)>;
pub type DMA_COMPLETION_STATUS = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DMA_CONFIGURATION_BYTE0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DMA_CONFIGURATION_BYTE1 {
    pub _bitfield: u8,
}
pub const DMA_FAIL_ON_BOUNCE: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy)]
pub union DMA_FEATURE_QUERY_STATUS {
    pub AsUINT64: u64,
}
impl Default for DMA_FEATURE_QUERY_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DMA_FEATURE_TYPE = i32;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DMA_IOMMU_INTERFACE {
    pub Version: u32,
    pub CreateDomain: PIOMMU_DOMAIN_CREATE,
    pub DeleteDomain: PIOMMU_DOMAIN_DELETE,
    pub AttachDevice: PIOMMU_DOMAIN_ATTACH_DEVICE,
    pub DetachDevice: PIOMMU_DOMAIN_DETACH_DEVICE,
    pub FlushDomain: PIOMMU_FLUSH_DOMAIN,
    pub FlushDomainByVaList: PIOMMU_FLUSH_DOMAIN_VA_LIST,
    pub QueryInputMappings: PIOMMU_QUERY_INPUT_MAPPINGS,
    pub MapLogicalRange: PIOMMU_MAP_LOGICAL_RANGE,
    pub UnmapLogicalRange: PIOMMU_UNMAP_LOGICAL_RANGE,
    pub MapIdentityRange: PIOMMU_MAP_IDENTITY_RANGE,
    pub UnmapIdentityRange: PIOMMU_UNMAP_IDENTITY_RANGE,
    pub SetDeviceFaultReporting: PIOMMU_SET_DEVICE_FAULT_REPORTING,
    pub ConfigureDomain: PIOMMU_DOMAIN_CONFIGURE,
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub struct DMA_IOMMU_INTERFACE_EX {
    pub Size: usize,
    pub Version: u32,
    pub Anonymous: DMA_IOMMU_INTERFACE_EX_0,
}
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for DMA_IOMMU_INTERFACE_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub union DMA_IOMMU_INTERFACE_EX_0 {
    pub V1: DMA_IOMMU_INTERFACE_V1,
    pub V2: DMA_IOMMU_INTERFACE_V2,
    pub V3: DMA_IOMMU_INTERFACE_V3,
}
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for DMA_IOMMU_INTERFACE_EX_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DMA_IOMMU_INTERFACE_EX_VERSION: u32 = 3;
pub const DMA_IOMMU_INTERFACE_EX_VERSION_1: u32 = 1;
pub const DMA_IOMMU_INTERFACE_EX_VERSION_2: u32 = 2;
pub const DMA_IOMMU_INTERFACE_EX_VERSION_3: u32 = 3;
pub const DMA_IOMMU_INTERFACE_EX_VERSION_MAX: u32 = 3;
pub const DMA_IOMMU_INTERFACE_EX_VERSION_MIN: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DMA_IOMMU_INTERFACE_V1 {
    pub CreateDomain: PIOMMU_DOMAIN_CREATE,
    pub DeleteDomain: PIOMMU_DOMAIN_DELETE,
    pub AttachDevice: PIOMMU_DOMAIN_ATTACH_DEVICE,
    pub DetachDevice: PIOMMU_DOMAIN_DETACH_DEVICE,
    pub FlushDomain: PIOMMU_FLUSH_DOMAIN,
    pub FlushDomainByVaList: PIOMMU_FLUSH_DOMAIN_VA_LIST,
    pub QueryInputMappings: PIOMMU_QUERY_INPUT_MAPPINGS,
    pub MapLogicalRange: PIOMMU_MAP_LOGICAL_RANGE,
    pub UnmapLogicalRange: PIOMMU_UNMAP_LOGICAL_RANGE,
    pub MapIdentityRange: PIOMMU_MAP_IDENTITY_RANGE,
    pub UnmapIdentityRange: PIOMMU_UNMAP_IDENTITY_RANGE,
    pub SetDeviceFaultReporting: PIOMMU_SET_DEVICE_FAULT_REPORTING,
    pub ConfigureDomain: PIOMMU_DOMAIN_CONFIGURE,
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DMA_IOMMU_INTERFACE_V2 {
    pub CreateDomainEx: PIOMMU_DOMAIN_CREATE_EX,
    pub DeleteDomain: PIOMMU_DOMAIN_DELETE,
    pub AttachDeviceEx: PIOMMU_DOMAIN_ATTACH_DEVICE_EX,
    pub DetachDeviceEx: PIOMMU_DOMAIN_DETACH_DEVICE_EX,
    pub FlushDomain: PIOMMU_FLUSH_DOMAIN,
    pub FlushDomainByVaList: PIOMMU_FLUSH_DOMAIN_VA_LIST,
    pub QueryInputMappings: PIOMMU_QUERY_INPUT_MAPPINGS,
    pub MapLogicalRangeEx: PIOMMU_MAP_LOGICAL_RANGE_EX,
    pub UnmapLogicalRange: PIOMMU_UNMAP_LOGICAL_RANGE,
    pub MapIdentityRangeEx: PIOMMU_MAP_IDENTITY_RANGE_EX,
    pub UnmapIdentityRangeEx: PIOMMU_UNMAP_IDENTITY_RANGE_EX,
    pub SetDeviceFaultReportingEx: PIOMMU_SET_DEVICE_FAULT_REPORTING_EX,
    pub ConfigureDomain: PIOMMU_DOMAIN_CONFIGURE,
    pub QueryAvailableDomainTypes: PIOMMU_DEVICE_QUERY_DOMAIN_TYPES,
    pub RegisterInterfaceStateChangeCallback: PIOMMU_REGISTER_INTERFACE_STATE_CHANGE_CALLBACK,
    pub UnregisterInterfaceStateChangeCallback: PIOMMU_UNREGISTER_INTERFACE_STATE_CHANGE_CALLBACK,
    pub ReserveLogicalAddressRange: PIOMMU_RESERVE_LOGICAL_ADDRESS_RANGE,
    pub FreeReservedLogicalAddressRange: PIOMMU_FREE_RESERVED_LOGICAL_ADDRESS_RANGE,
    pub MapReservedLogicalRange: PIOMMU_MAP_RESERVED_LOGICAL_RANGE,
    pub UnmapReservedLogicalRange: PIOMMU_UNMAP_RESERVED_LOGICAL_RANGE,
    pub CreateDevice: PIOMMU_DEVICE_CREATE,
    pub DeleteDevice: PIOMMU_DEVICE_DELETE,
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DMA_IOMMU_INTERFACE_V3 {
    pub CreateDomainEx: PIOMMU_DOMAIN_CREATE_EX,
    pub DeleteDomain: PIOMMU_DOMAIN_DELETE,
    pub AttachDeviceEx: PIOMMU_DOMAIN_ATTACH_DEVICE_EX,
    pub DetachDeviceEx: PIOMMU_DOMAIN_DETACH_DEVICE_EX,
    pub FlushDomain: PIOMMU_FLUSH_DOMAIN,
    pub FlushDomainByVaList: PIOMMU_FLUSH_DOMAIN_VA_LIST,
    pub QueryInputMappings: PIOMMU_QUERY_INPUT_MAPPINGS,
    pub MapLogicalRangeEx: PIOMMU_MAP_LOGICAL_RANGE_EX,
    pub UnmapLogicalRange: PIOMMU_UNMAP_LOGICAL_RANGE,
    pub MapIdentityRangeEx: PIOMMU_MAP_IDENTITY_RANGE_EX,
    pub UnmapIdentityRangeEx: PIOMMU_UNMAP_IDENTITY_RANGE_EX,
    pub SetDeviceFaultReportingEx: PIOMMU_SET_DEVICE_FAULT_REPORTING_EX,
    pub ConfigureDomain: PIOMMU_DOMAIN_CONFIGURE,
    pub QueryAvailableDomainTypes: PIOMMU_DEVICE_QUERY_DOMAIN_TYPES,
    pub RegisterInterfaceStateChangeCallback: PIOMMU_REGISTER_INTERFACE_STATE_CHANGE_CALLBACK,
    pub UnregisterInterfaceStateChangeCallback: PIOMMU_UNREGISTER_INTERFACE_STATE_CHANGE_CALLBACK,
    pub ReserveLogicalAddressRange: PIOMMU_RESERVE_LOGICAL_ADDRESS_RANGE,
    pub FreeReservedLogicalAddressRange: PIOMMU_FREE_RESERVED_LOGICAL_ADDRESS_RANGE,
    pub MapReservedLogicalRange: PIOMMU_MAP_RESERVED_LOGICAL_RANGE,
    pub UnmapReservedLogicalRange: PIOMMU_UNMAP_RESERVED_LOGICAL_RANGE,
    pub CreateDevice: PIOMMU_DEVICE_CREATE,
    pub DeleteDevice: PIOMMU_DEVICE_DELETE,
    pub CreatePasidDevice: PIOMMU_PASID_DEVICE_CREATE,
    pub DeletePasidDevice: PIOMMU_PASID_DEVICE_DELETE,
    pub AttachPasidDevice: PIOMMU_DOMAIN_ATTACH_PASID_DEVICE,
    pub DetachPasidDevice: PIOMMU_DOMAIN_DETACH_PASID_DEVICE,
    pub QueryDeviceInfo: PIOMMU_DEVICE_QUERY_INFORMATION,
}
pub const DMA_IOMMU_INTERFACE_VERSION: u32 = 1;
pub const DMA_IOMMU_INTERFACE_VERSION_1: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct DMA_OPERATIONS {
    pub Size: u32,
    pub PutDmaAdapter: PPUT_DMA_ADAPTER,
    pub AllocateCommonBuffer: PALLOCATE_COMMON_BUFFER,
    pub FreeCommonBuffer: PFREE_COMMON_BUFFER,
    pub AllocateAdapterChannel: PALLOCATE_ADAPTER_CHANNEL,
    pub FlushAdapterBuffers: PFLUSH_ADAPTER_BUFFERS,
    pub FreeAdapterChannel: PFREE_ADAPTER_CHANNEL,
    pub FreeMapRegisters: PFREE_MAP_REGISTERS,
    pub MapTransfer: PMAP_TRANSFER,
    pub GetDmaAlignment: PGET_DMA_ALIGNMENT,
    pub ReadDmaCounter: PREAD_DMA_COUNTER,
    pub GetScatterGatherList: PGET_SCATTER_GATHER_LIST,
    pub PutScatterGatherList: PPUT_SCATTER_GATHER_LIST,
    pub CalculateScatterGatherList: PCALCULATE_SCATTER_GATHER_LIST_SIZE,
    pub BuildScatterGatherList: PBUILD_SCATTER_GATHER_LIST,
    pub BuildMdlFromScatterGatherList: PBUILD_MDL_FROM_SCATTER_GATHER_LIST,
    pub GetDmaAdapterInfo: PGET_DMA_ADAPTER_INFO,
    pub GetDmaTransferInfo: PGET_DMA_TRANSFER_INFO,
    pub InitializeDmaTransferContext: PINITIALIZE_DMA_TRANSFER_CONTEXT,
    pub AllocateCommonBufferEx: PALLOCATE_COMMON_BUFFER_EX,
    pub AllocateAdapterChannelEx: PALLOCATE_ADAPTER_CHANNEL_EX,
    pub ConfigureAdapterChannel: PCONFIGURE_ADAPTER_CHANNEL,
    pub CancelAdapterChannel: PCANCEL_ADAPTER_CHANNEL,
    pub MapTransferEx: PMAP_TRANSFER_EX,
    pub GetScatterGatherListEx: PGET_SCATTER_GATHER_LIST_EX,
    pub BuildScatterGatherListEx: PBUILD_SCATTER_GATHER_LIST_EX,
    pub FlushAdapterBuffersEx: PFLUSH_ADAPTER_BUFFERS_EX,
    pub FreeAdapterObject: PFREE_ADAPTER_OBJECT,
    pub CancelMappedTransfer: PCANCEL_MAPPED_TRANSFER,
    pub AllocateDomainCommonBuffer: PALLOCATE_DOMAIN_COMMON_BUFFER,
    pub FlushDmaBuffer: PFLUSH_DMA_BUFFER,
    pub JoinDmaDomain: PJOIN_DMA_DOMAIN,
    pub LeaveDmaDomain: PLEAVE_DMA_DOMAIN,
    pub GetDmaDomain: PGET_DMA_DOMAIN,
    pub AllocateCommonBufferWithBounds: PALLOCATE_COMMON_BUFFER_WITH_BOUNDS,
    pub AllocateCommonBufferVector: PALLOCATE_COMMON_BUFFER_VECTOR,
    pub GetCommonBufferFromVectorByIndex: PGET_COMMON_BUFFER_FROM_VECTOR_BY_INDEX,
    pub FreeCommonBufferFromVector: PFREE_COMMON_BUFFER_FROM_VECTOR,
    pub FreeCommonBufferVector: PFREE_COMMON_BUFFER_VECTOR,
    pub CreateCommonBufferFromMdl: PCREATE_COMMON_BUFFER_FROM_MDL,
}
pub type DMA_SPEED = i32;
pub const DMA_SYNCHRONOUS_CALLBACK: u32 = 1;
#[cfg(target_arch = "x86")]
pub const DMA_TRANSFER_CONTEXT_SIZE_V1: u32 = 64;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DMA_TRANSFER_CONTEXT_SIZE_V1: u32 = 128;
pub const DMA_TRANSFER_CONTEXT_VERSION1: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DMA_TRANSFER_INFO {
    pub Version: u32,
    pub Anonymous: DMA_TRANSFER_INFO_0,
}
impl Default for DMA_TRANSFER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DMA_TRANSFER_INFO_0 {
    pub V1: DMA_TRANSFER_INFO_V1,
    pub V2: DMA_TRANSFER_INFO_V2,
}
impl Default for DMA_TRANSFER_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DMA_TRANSFER_INFO_V1 {
    pub MapRegisterCount: u32,
    pub ScatterGatherElementCount: u32,
    pub ScatterGatherListSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DMA_TRANSFER_INFO_V2 {
    pub MapRegisterCount: u32,
    pub ScatterGatherElementCount: u32,
    pub ScatterGatherListSize: u32,
    pub LogicalPageCount: u32,
}
pub const DMA_TRANSFER_INFO_VERSION1: u32 = 1;
pub const DMA_TRANSFER_INFO_VERSION2: u32 = 2;
pub type DMA_WIDTH = i32;
pub const DMA_ZERO_BUFFERS: u32 = 2;
pub const DOMAIN_COMMON_BUFFER_LARGE_PAGE: u32 = 1;
#[repr(C)]
#[cfg(feature = "usb")]
#[derive(Clone, Copy)]
pub struct DOMAIN_CONFIGURATION {
    pub Type: DOMAIN_CONFIGURATION_ARCH,
    pub Anonymous: DOMAIN_CONFIGURATION_0,
}
#[cfg(feature = "usb")]
impl Default for DOMAIN_CONFIGURATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "usb")]
#[derive(Clone, Copy)]
pub union DOMAIN_CONFIGURATION_0 {
    pub Arm64: DOMAIN_CONFIGURATION_ARM64,
    pub X64: DOMAIN_CONFIGURATION_X64,
}
#[cfg(feature = "usb")]
impl Default for DOMAIN_CONFIGURATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DOMAIN_CONFIGURATION_ARCH = i32;
#[repr(C)]
#[cfg(feature = "usb")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOMAIN_CONFIGURATION_ARM64 {
    pub Ttbr0: super::usb::PHYSICAL_ADDRESS,
    pub Ttbr1: super::usb::PHYSICAL_ADDRESS,
    pub Mair0: u32,
    pub Mair1: u32,
    pub InputSize0: u8,
    pub InputSize1: u8,
    pub CoherentTableWalks: bool,
    pub TranslationEnabled: bool,
}
#[repr(C)]
#[cfg(feature = "usb")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DOMAIN_CONFIGURATION_X64 {
    pub FirstLevelPageTableRoot: super::usb::PHYSICAL_ADDRESS,
    pub TranslationEnabled: bool,
}
pub const DO_BUFFERED_IO: u32 = 4;
pub const DO_BUS_ENUMERATED_DEVICE: u32 = 4096;
pub const DO_DAX_VOLUME: u32 = 268435456;
pub const DO_DEVICE_INITIALIZING: u32 = 128;
pub const DO_DEVICE_TO_BE_RESET: u32 = 67108864;
pub const DO_DIRECT_IO: u32 = 16;
pub const DO_EXCLUSIVE: u32 = 8;
pub const DO_MAP_IO_BUFFER: u32 = 32;
pub const DO_POWER_INRUSH: u32 = 16384;
pub const DO_POWER_PAGABLE: u32 = 8192;
pub const DO_SHUTDOWN_REGISTERED: u32 = 2048;
pub const DO_VERIFY_VOLUME: u32 = 2;
pub const DPC_NORMAL: u32 = 0;
pub const DPC_THREADED: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DPC_WATCHDOG_GLOBAL_TRIAGE_BLOCK {
    pub Signature: u32,
    pub Revision: u16,
    pub Size: u16,
    pub DpcWatchdogProfileOffset: u16,
    pub DpcWatchdogProfileLength: u32,
}
pub const DPC_WATCHDOG_GLOBAL_TRIAGE_BLOCK_REVISION_1: u32 = 1;
pub const DPC_WATCHDOG_GLOBAL_TRIAGE_BLOCK_SIGNATURE: u32 = 2931740382;
pub const DPC_WATCHDOG_GLOBAL_TRIAGE_BLOCK_VER_1_SIZE: u32 = 16;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type DRIVER_ADD_DEVICE = Option<unsafe extern "system" fn(driverobject: *const DRIVER_OBJECT, physicaldeviceobject: *const DEVICE_OBJECT) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type DRIVER_CANCEL = Option<unsafe extern "system" fn(deviceobject: *mut DEVICE_OBJECT, irp: *mut IRP)>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type DRIVER_CONTROL = Option<unsafe extern "system" fn(deviceobject: *const DEVICE_OBJECT, irp: *mut IRP, mapregisterbase: *const core::ffi::c_void, context: *const core::ffi::c_void) -> IO_ALLOCATION_ACTION>;
pub type DRIVER_DIRECTORY_TYPE = i32;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type DRIVER_DISPATCH = Option<unsafe extern "system" fn(deviceobject: *const DEVICE_OBJECT, irp: *mut IRP) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type DRIVER_DISPATCH_PAGED = Option<unsafe extern "system" fn(deviceobject: *const DEVICE_OBJECT, irp: *mut IRP) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type DRIVER_DISPATCH_RAISED = DRIVER_DISPATCH;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DRIVER_EXTENSION {
    pub DriverObject: *mut DRIVER_OBJECT,
    pub AddDevice: PDRIVER_ADD_DEVICE,
    pub Count: u32,
    pub ServiceKeyName: super::ntsecapi::UNICODE_STRING,
}
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for DRIVER_EXTENSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type DRIVER_INITIALIZE = Option<unsafe extern "system" fn(driverobject: *const DRIVER_OBJECT, registrypath: *const super::ntsecapi::UNICODE_STRING) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type DRIVER_LIST_CONTROL = Option<unsafe extern "system" fn(deviceobject: *const DEVICE_OBJECT, irp: *const IRP, scattergather: *const SCATTER_GATHER_LIST, context: *const core::ffi::c_void)>;
#[cfg(feature = "bcrypt")]
pub type DRIVER_NOTIFICATION_CALLBACK_ROUTINE = Option<unsafe extern "system" fn(notificationstructure: *const core::ffi::c_void, context: *mut core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DRIVER_OBJECT {
    pub Type: super::ntdef::CSHORT,
    pub Size: super::ntdef::CSHORT,
    pub DeviceObject: PDEVICE_OBJECT,
    pub Flags: u32,
    pub DriverStart: *mut core::ffi::c_void,
    pub DriverSize: u32,
    pub DriverSection: *mut core::ffi::c_void,
    pub DriverExtension: PDRIVER_EXTENSION,
    pub DriverName: super::ntsecapi::UNICODE_STRING,
    pub HardwareDatabase: super::ntsecapi::PUNICODE_STRING,
    pub FastIoDispatch: PFAST_IO_DISPATCH,
    pub DriverInit: PDRIVER_INITIALIZE,
    pub DriverStartIo: PDRIVER_STARTIO,
    pub DriverUnload: PDRIVER_UNLOAD,
    pub MajorFunction: [PDRIVER_DISPATCH; 28],
}
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for DRIVER_OBJECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type DRIVER_PROXY_ENDPOINT_FUNCTION = Option<unsafe extern "system" fn()>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DRIVER_PROXY_ENDPOINT_FUNCTION_ID(pub u32);
pub const DRIVER_PROXY_ENDPOINT_FUNCTION_ID_INVALID: i32 = -1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DRIVER_PROXY_ENDPOINT_INFORMATION {
    pub Id: DRIVER_PROXY_ENDPOINT_FUNCTION_ID,
    pub EndpointFunction: PDRIVER_PROXY_ENDPOINT_FUNCTION,
    pub ParameterCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DRIVER_PROXY_EXTENSION_CREATION_FLAGS {
    pub AsUlong: u32,
    pub Anonymous: DRIVER_PROXY_EXTENSION_CREATION_FLAGS_0,
}
impl Default for DRIVER_PROXY_EXTENSION_CREATION_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DRIVER_PROXY_EXTENSION_CREATION_FLAGS_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DRIVER_PROXY_FEATURE_FLAGS {
    pub AsUlong: u32,
    pub Anonymous: DRIVER_PROXY_FEATURE_FLAGS_0,
}
impl Default for DRIVER_PROXY_FEATURE_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DRIVER_PROXY_FEATURE_FLAGS_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "bcrypt")]
pub type DRIVER_PROXY_HOTSWAP_CALLBACK = Option<unsafe extern "system" fn(phase: DRIVER_PROXY_HOTSWAP_CALLBACK_PHASE, context: *const core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
pub type DRIVER_PROXY_HOTSWAP_CALLBACK_PHASE = i32;
#[cfg(feature = "bcrypt")]
pub type DRIVER_PROXY_HOTSWAP_WORKER_ROUTINE = Option<unsafe extern "system" fn(workercontext: *const core::ffi::c_void, waitstatus: super::bcrypt::NTSTATUS) -> bool>;
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "ntdef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DRIVER_PROXY_HOTSWAP_WORKER_ROUTINE_START_CONTEXT {
    pub WorkerRoutine: PDRIVER_PROXY_HOTSWAP_WORKER_ROUTINE,
    pub Context: *mut core::ffi::c_void,
    pub WaitType: super::ntdef::WAIT_TYPE,
    pub WaitReason: KWAIT_REASON,
    pub WaitMode: KPROCESSOR_MODE,
    pub Altertable: bool,
    pub HasTimeout: bool,
    pub Timeout: i64,
    pub EventCount: u32,
    pub Events: [PKEVENT; 1],
}
#[cfg(all(feature = "bcrypt", feature = "ntdef", feature = "winnt"))]
impl Default for DRIVER_PROXY_HOTSWAP_WORKER_ROUTINE_START_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "bcrypt")]
pub type DRIVER_PROXY_REGISTER_CALLBACK = DRIVER_PROXY_HOTSWAP_CALLBACK;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct DRIVER_PROXY_REGISTER_CALLBACK_PHASE(pub DRIVER_PROXY_HOTSWAP_CALLBACK_PHASE);
pub type DRIVER_PROXY_VERSION = i32;
pub type DRIVER_REGKEY_TYPE = i32;
pub type DRIVER_RUNTIME_INIT_FLAGS = i32;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type DRIVER_STARTIO = Option<unsafe extern "system" fn(deviceobject: *mut DEVICE_OBJECT, irp: *mut IRP)>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type DRIVER_UNLOAD = Option<unsafe extern "system" fn(driverobject: *const DRIVER_OBJECT)>;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const DRS_LEVEL: u32 = 14;
pub const DRVO_BUILTIN_DRIVER: u32 = 4;
pub const DRVO_LEGACY_DRIVER: u32 = 2;
pub const DRVO_UNLOAD_INVOKED: u32 = 1;
pub const DUPLICATE_SAME_ATTRIBUTES: u32 = 4;
pub const DeallocateObject: IO_ALLOCATION_ACTION = 2;
pub const DeallocateObjectKeepRegisters: IO_ALLOCATION_ACTION = 3;
pub const DelayExecution: KWAIT_REASON = 4;
pub const DelayedWorkQueue: WORK_QUEUE_TYPE = 1;
pub const DeleteSecurityDescriptor: SECURITY_OPERATION_CODE = 2;
pub const DeviceDirectoryData: DEVICE_DIRECTORY_TYPE = 0;
pub const DevicePowerState: POWER_STATE_TYPE = 1;
pub const DevicePropertyAddress: DEVICE_REGISTRY_PROPERTY = 16;
pub const DevicePropertyAllocatedResources: DEVICE_REGISTRY_PROPERTY = 21;
pub const DevicePropertyBootConfiguration: DEVICE_REGISTRY_PROPERTY = 3;
pub const DevicePropertyBootConfigurationTranslated: DEVICE_REGISTRY_PROPERTY = 4;
pub const DevicePropertyBusNumber: DEVICE_REGISTRY_PROPERTY = 14;
pub const DevicePropertyBusTypeGuid: DEVICE_REGISTRY_PROPERTY = 12;
pub const DevicePropertyClassGuid: DEVICE_REGISTRY_PROPERTY = 6;
pub const DevicePropertyClassName: DEVICE_REGISTRY_PROPERTY = 5;
pub const DevicePropertyCompatibleIDs: DEVICE_REGISTRY_PROPERTY = 2;
pub const DevicePropertyContainerID: DEVICE_REGISTRY_PROPERTY = 22;
pub const DevicePropertyDeviceDescription: DEVICE_REGISTRY_PROPERTY = 0;
pub const DevicePropertyDriverKeyName: DEVICE_REGISTRY_PROPERTY = 7;
pub const DevicePropertyEnumeratorName: DEVICE_REGISTRY_PROPERTY = 15;
pub const DevicePropertyFriendlyName: DEVICE_REGISTRY_PROPERTY = 9;
pub const DevicePropertyHardwareID: DEVICE_REGISTRY_PROPERTY = 1;
pub const DevicePropertyInstallState: DEVICE_REGISTRY_PROPERTY = 18;
pub const DevicePropertyLegacyBusType: DEVICE_REGISTRY_PROPERTY = 13;
pub const DevicePropertyLocationInformation: DEVICE_REGISTRY_PROPERTY = 10;
pub const DevicePropertyManufacturer: DEVICE_REGISTRY_PROPERTY = 8;
pub const DevicePropertyPhysicalDeviceObjectName: DEVICE_REGISTRY_PROPERTY = 11;
pub const DevicePropertyRemovalPolicy: DEVICE_REGISTRY_PROPERTY = 19;
pub const DevicePropertyResourceRequirements: DEVICE_REGISTRY_PROPERTY = 20;
pub const DevicePropertyUINumber: DEVICE_REGISTRY_PROPERTY = 17;
pub const DeviceTextDescription: DEVICE_TEXT_TYPE = 0;
pub const DeviceTextLocationInformation: DEVICE_TEXT_TYPE = 1;
pub const DeviceUsageTypeBoot: DEVICE_USAGE_NOTIFICATION_TYPE = 4;
pub const DeviceUsageTypeDumpFile: DEVICE_USAGE_NOTIFICATION_TYPE = 3;
pub const DeviceUsageTypeGuestAssigned: DEVICE_USAGE_NOTIFICATION_TYPE = 6;
pub const DeviceUsageTypeHibernation: DEVICE_USAGE_NOTIFICATION_TYPE = 2;
pub const DeviceUsageTypeInlineCryptoEngine: DEVICE_USAGE_NOTIFICATION_TYPE = 7;
pub const DeviceUsageTypePaging: DEVICE_USAGE_NOTIFICATION_TYPE = 1;
pub const DeviceUsageTypePostDisplay: DEVICE_USAGE_NOTIFICATION_TYPE = 5;
pub const DeviceUsageTypeUndefined: DEVICE_USAGE_NOTIFICATION_TYPE = 0;
pub const DeviceWakeDepthD0: DEVICE_WAKE_DEPTH = 1;
pub const DeviceWakeDepthD1: DEVICE_WAKE_DEPTH = 2;
pub const DeviceWakeDepthD2: DEVICE_WAKE_DEPTH = 3;
pub const DeviceWakeDepthD3cold: DEVICE_WAKE_DEPTH = 5;
pub const DeviceWakeDepthD3hot: DEVICE_WAKE_DEPTH = 4;
pub const DeviceWakeDepthMaximum: DEVICE_WAKE_DEPTH = 6;
pub const DeviceWakeDepthNotWakeable: DEVICE_WAKE_DEPTH = 0;
pub const DirectoryNotifyExtendedInformation: DIRECTORY_NOTIFY_INFORMATION_CLASS = 2;
pub const DirectoryNotifyFullInformation: DIRECTORY_NOTIFY_INFORMATION_CLASS = 3;
pub const DirectoryNotifyInformation: DIRECTORY_NOTIFY_INFORMATION_CLASS = 1;
pub const DirectoryNotifyMaximumInformation: DIRECTORY_NOTIFY_INFORMATION_CLASS = 4;
pub const DisabledControl: NPEM_CONTROL_STANDARD_CONTROL_BIT = 11;
pub const DiskIoNotifyRoutinesClass: TRACE_INFORMATION_CLASS = 11;
pub const DmaAborted: DMA_COMPLETION_STATUS = 1;
pub const DmaCancelled: DMA_COMPLETION_STATUS = 3;
pub const DmaComplete: DMA_COMPLETION_STATUS = 0;
pub const DmaError: DMA_COMPLETION_STATUS = 2;
pub const DmaFeatureTypeBlockDeviceOnRelease: DMA_FEATURE_TYPE = 2;
pub const DmaFeatureTypeFaultCallbackRegistration: DMA_FEATURE_TYPE = 3;
pub const DmaFeatureTypeMax: DMA_FEATURE_TYPE = 5;
pub const DmaFeatureTypeQueryDmarAvailability: DMA_FEATURE_TYPE = 1;
pub const DmaFeatureTypeReserved: DMA_FEATURE_TYPE = 0;
pub const DmaFeatureTypeUserModeDriverDescriptorFlag: DMA_FEATURE_TYPE = 4;
pub const DomainConfigurationArm64: DOMAIN_CONFIGURATION_ARCH = 0;
pub const DomainConfigurationInvalid: DOMAIN_CONFIGURATION_ARCH = 2;
pub const DomainConfigurationX64: DOMAIN_CONFIGURATION_ARCH = 1;
pub const DomainTypeMax: IOMMU_DMA_DOMAIN_TYPE = 4;
pub const DomainTypePassThrough: IOMMU_DMA_DOMAIN_TYPE = 1;
pub const DomainTypeTranslate: IOMMU_DMA_DOMAIN_TYPE = 0;
pub const DomainTypeTranslateS1: IOMMU_DMA_DOMAIN_TYPE = 3;
pub const DomainTypeUnmanaged: IOMMU_DMA_DOMAIN_TYPE = 2;
pub const DontUseThisType: POOL_TYPE = 3;
pub const DontUseThisTypeSession: POOL_TYPE = 35;
pub const DriverDirectoryData: DRIVER_DIRECTORY_TYPE = 1;
pub const DriverDirectoryImage: DRIVER_DIRECTORY_TYPE = 0;
pub const DriverDirectorySharedData: DRIVER_DIRECTORY_TYPE = 2;
pub const DriverProxyHotSwapCallbackMax: DRIVER_PROXY_HOTSWAP_CALLBACK_PHASE = 3;
pub const DriverProxyHotSwapCallbackPostProcess: DRIVER_PROXY_HOTSWAP_CALLBACK_PHASE = 2;
pub const DriverProxyHotSwapCallbackPreProcess: DRIVER_PROXY_HOTSWAP_CALLBACK_PHASE = 0;
pub const DriverProxyHotSwapCallbackProxyStalled: DRIVER_PROXY_HOTSWAP_CALLBACK_PHASE = 1;
pub const DriverProxyVersion1: DRIVER_PROXY_VERSION = 1;
pub const DriverProxyVersion2: DRIVER_PROXY_VERSION = 2;
pub const DriverProxyVersionMax: DRIVER_PROXY_VERSION = 3;
pub const DriverProxyVersionNone: DRIVER_PROXY_VERSION = 0;
pub const DriverRegKeyParameters: DRIVER_REGKEY_TYPE = 0;
pub const DriverRegKeyPersistentState: DRIVER_REGKEY_TYPE = 1;
pub const DriverRegKeySharedPersistentState: DRIVER_REGKEY_TYPE = 2;
pub const DrvRtPoolNxOptIn: DRIVER_RUNTIME_INIT_FLAGS = 1;
#[cfg(target_arch = "x86")]
pub const EFLAG_SELECT: u32 = 49152;
#[cfg(target_arch = "x86")]
pub const EFLAG_SIGN: u32 = 32768;
#[cfg(target_arch = "x86")]
pub const EFLAG_ZERO: u32 = 16384;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EISA_DMA_CONFIGURATION {
    pub ConfigurationByte0: DMA_CONFIGURATION_BYTE0,
    pub ConfigurationByte1: DMA_CONFIGURATION_BYTE1,
}
pub const EISA_EMPTY_SLOT: u32 = 131;
pub const EISA_FREE_FORM_DATA: u32 = 64;
pub const EISA_FUNCTION_ENABLED: u32 = 128;
pub const EISA_HAS_DMA_ENTRY: u32 = 8;
pub const EISA_HAS_INFORMATION: u32 = 31;
pub const EISA_HAS_IRQ_ENTRY: u32 = 4;
pub const EISA_HAS_MEMORY_ENTRY: u32 = 2;
pub const EISA_HAS_PORT_INIT_ENTRY: u32 = 32;
pub const EISA_HAS_PORT_RANGE: u32 = 16;
pub const EISA_HAS_TYPE_ENTRY: u32 = 1;
pub const EISA_INVALID_BIOS_CALL: u32 = 134;
pub const EISA_INVALID_CONFIGURATION: u32 = 130;
pub const EISA_INVALID_FUNCTION: u32 = 129;
pub const EISA_INVALID_SLOT: u32 = 128;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EISA_IRQ_CONFIGURATION {
    pub ConfigurationByte: EISA_IRQ_DESCRIPTOR,
    pub Reserved: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EISA_IRQ_DESCRIPTOR {
    pub _bitfield: u8,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct EISA_MEMORY_CONFIGURATION {
    pub ConfigurationByte: EISA_MEMORY_TYPE,
    pub DataSize: u8,
    pub AddressLowWord: u16,
    pub AddressHighByte: u8,
    pub MemorySize: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EISA_MEMORY_TYPE {
    pub _bitfield: u8,
}
pub const EISA_MEMORY_TYPE_RAM: u32 = 1;
pub const EISA_MORE_ENTRIES: u32 = 128;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct EISA_PORT_CONFIGURATION {
    pub Configuration: EISA_PORT_DESCRIPTOR,
    pub PortAddress: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EISA_PORT_DESCRIPTOR {
    pub _bitfield: u8,
}
pub const EISA_SYSTEM_MEMORY: u32 = 0;
#[cfg(feature = "bcrypt")]
pub type ENABLE_VIRTUALIZATION = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, numvfs: u16, enablevfmigration: bool, enablemigrationinterrupt: bool, enablevirtualization: bool) -> super::bcrypt::NTSTATUS>;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct ERESOURCE {
    pub SystemResourcesList: super::winnt::LIST_ENTRY,
    pub OwnerTable: POWNER_ENTRY,
    pub ActiveCount: i16,
    pub Flag: u8,
    pub WaiterPriority: u8,
    pub SharedWaiters: *mut core::ffi::c_void,
    pub ExclusiveWaiters: *mut core::ffi::c_void,
    pub OwnerEntry: OWNER_ENTRY,
    pub ActiveEntries: u32,
    pub ContentionCount: u32,
    pub NumberOfSharedWaiters: u32,
    pub NumberOfExclusiveWaiters: u32,
    pub Anonymous: ERESOURCE_0,
    pub SpinLock: super::winnt::KSPIN_LOCK,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
impl Default for ERESOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union ERESOURCE_0 {
    pub Address: *mut core::ffi::c_void,
    pub CreatorBackTraceIndex: usize,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
impl Default for ERESOURCE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct ERESOURCE {
    pub SystemResourcesList: super::winnt::LIST_ENTRY,
    pub OwnerTable: POWNER_ENTRY,
    pub ActiveCount: i16,
    pub Flag: u8,
    pub WaiterPriority: u8,
    pub SharedWaiters: *mut core::ffi::c_void,
    pub ExclusiveWaiters: *mut core::ffi::c_void,
    pub OwnerEntry: OWNER_ENTRY,
    pub ActiveEntries: u32,
    pub ContentionCount: u32,
    pub NumberOfSharedWaiters: u32,
    pub NumberOfExclusiveWaiters: u32,
    pub MiscFlags: i8,
    pub Reserved1: [u8; 3],
    pub ResourceTimeoutCount: u32,
    pub Anonymous: ERESOURCE_0,
    pub SpinLock: super::winnt::KSPIN_LOCK,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
impl Default for ERESOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union ERESOURCE_0 {
    pub Address: *mut core::ffi::c_void,
    pub CreatorBackTraceIndex: usize,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
impl Default for ERESOURCE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ERESOURCE_THREAD(pub usize);
pub const ERROR_LOG_LIMIT_SIZE: u32 = 240;
#[cfg(target_arch = "x86")]
pub const ERROR_LOG_MAXIMUM_SIZE: u32 = 152;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const ERROR_LOG_MAXIMUM_SIZE: u32 = 240;
pub const ERROR_LOG_MESSAGE_LIMIT_SIZE: u32 = 344;
#[cfg(feature = "evntprov")]
pub type ETWENABLECALLBACK = Option<unsafe extern "system" fn(sourceid: *const windows_core::GUID, controlcode: u32, level: u8, matchanykeyword: u64, matchallkeyword: u64, filterdata: *const super::evntprov::EVENT_FILTER_DESCRIPTOR, callbackcontext: *mut core::ffi::c_void)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ETW_TRACE_SESSION_SETTINGS {
    pub Version: u32,
    pub BufferSize: u32,
    pub MinimumBuffers: u32,
    pub MaximumBuffers: u32,
    pub LoggerMode: u32,
    pub FlushTimer: u32,
    pub FlushThreshold: u32,
    pub ClockType: u32,
}
pub const EVENT_INCREMENT: u32 = 1;
pub const EVENT_QUERY_STATE: u32 = 1;
pub const EXCEPTION_ALIGNMENT_CHECK: u32 = 17;
pub const EXCEPTION_BOUND_CHECK: u32 = 5;
pub const EXCEPTION_CP_FAULT: u32 = 21;
pub const EXCEPTION_DEBUG: u32 = 1;
pub const EXCEPTION_DIVIDED_BY_ZERO: u32 = 0;
pub const EXCEPTION_DOUBLE_FAULT: u32 = 8;
pub const EXCEPTION_GP_FAULT: u32 = 13;
pub const EXCEPTION_INT3: u32 = 3;
pub const EXCEPTION_INVALID_OPCODE: u32 = 6;
pub const EXCEPTION_INVALID_TSS: u32 = 10;
pub const EXCEPTION_NMI: u32 = 2;
pub const EXCEPTION_NPX_ERROR: u32 = 16;
pub const EXCEPTION_NPX_NOT_AVAILABLE: u32 = 7;
pub const EXCEPTION_NPX_OVERRUN: u32 = 9;
pub const EXCEPTION_RESERVED_TRAP: u32 = 15;
pub const EXCEPTION_SEGMENT_NOT_PRESENT: u32 = 11;
pub const EXCEPTION_SE_FAULT: u32 = 23;
pub const EXCEPTION_STACK_FAULT: u32 = 12;
pub const EXCEPTION_VIRTUALIZATION_FAULT: u32 = 32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EXTENDED_CREATE_DUAL_OPLOCK_KEYS {
    pub ParentOplockKey: windows_core::GUID,
    pub TargetOplockKey: windows_core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EXTENDED_CREATE_INFORMATION {
    pub ExtendedCreateFlags: i64,
    pub EaBuffer: *mut core::ffi::c_void,
    pub EaLength: u32,
    pub DualOplockKeys: PEXTENDED_CREATE_DUAL_OPLOCK_KEYS,
}
impl Default for EXTENDED_CREATE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EXTENDED_CREATE_INFORMATION_32 {
    pub ExtendedCreateFlags: i64,
    pub EaBuffer: *mut core::ffi::c_void,
    pub EaLength: u32,
    pub DualOplockKeys: PEXTENDED_CREATE_DUAL_OPLOCK_KEYS,
}
#[cfg(target_arch = "x86")]
impl Default for EXTENDED_CREATE_INFORMATION_32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EXTENDED_CREATE_INFORMATION_32 {
    pub ExtendedCreateFlags: i64,
    pub EaBuffer: *mut core::ffi::c_void,
    pub EaLength: u32,
    pub DualOplockKeys: *mut EXTENDED_CREATE_DUAL_OPLOCK_KEYS,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
impl Default for EXTENDED_CREATE_INFORMATION_32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type EXT_CALLBACK = Option<unsafe extern "system" fn(timer: *const _EX_TIMER, context: *const core::ffi::c_void)>;
pub type EXT_DELETE_CALLBACK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EXT_DELETE_PARAMETERS {
    pub Version: u32,
    pub Reserved: u32,
    pub DeleteCallback: PEXT_DELETE_CALLBACK,
    pub DeleteContext: *mut core::ffi::c_void,
}
impl Default for EXT_DELETE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EXT_SET_PARAMETERS {
    pub Version: u32,
    pub Reserved: u32,
    pub NoWakeTolerance: i64,
}
#[cfg(feature = "bcrypt")]
pub type EX_CALLBACK_FUNCTION = Option<unsafe extern "system" fn(callbackcontext: *const core::ffi::c_void, argument1: *const core::ffi::c_void, argument2: *const core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
pub const EX_CARR_ALLOCATE_NONPAGED_POOL: u32 = 1;
pub const EX_CARR_ALLOCATE_PAGED_POOL: u32 = 0;
pub const EX_CARR_DISABLE_EXPANSION: u32 = 2;
pub const EX_CARR_VALID_FLAGS: u32 = 3;
pub const EX_CREATE_FLAG_FILE_DEST_OPEN_FOR_COPY: u32 = 2;
pub const EX_CREATE_FLAG_FILE_SOURCE_OPEN_FOR_COPY: u32 = 1;
pub const EX_DEFAULT_PUSH_LOCK_FLAGS: u32 = 0;
pub const EX_LOOKASIDE_LIST_EX_FLAGS_FAIL_NO_RAISE: u32 = 2;
pub const EX_LOOKASIDE_LIST_EX_FLAGS_RAISE_ON_FAIL: u32 = 1;
pub const EX_MAXIMUM_LOOKASIDE_DEPTH_BASE: u32 = 256;
pub const EX_MAXIMUM_LOOKASIDE_DEPTH_LIMIT: u32 = 1024;
pub type EX_POOL_PRIORITY = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EX_RCU_FREE_POOL_CONTEXT {
    pub Placeholder: [usize; 6],
}
impl Default for EX_RCU_FREE_POOL_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EX_RUNDOWN_ACTIVE: u32 = 1;
pub const EX_RUNDOWN_COUNT_INC: u32 = 2;
pub const EX_RUNDOWN_COUNT_SHIFT: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EX_RUNDOWN_REF {
    pub Anonymous: EX_RUNDOWN_REF_0,
}
impl Default for EX_RUNDOWN_REF {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union EX_RUNDOWN_REF_0 {
    pub Count: usize,
    pub Ptr: *mut core::ffi::c_void,
}
impl Default for EX_RUNDOWN_REF_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct EX_SPIN_LOCK(pub i32);
pub const EX_TIMER_HIGH_RESOLUTION: u32 = 4;
pub const EX_TIMER_NOTIFICATION: i32 = -2147483648;
pub const EX_TIMER_NO_WAKE: u32 = 8;
pub const EX_TIMER_UNLIMITED_TOLERANCE: i64 = -1;
pub const Eisa: INTERFACE_TYPE = 2;
pub const EjectionRelations: DEVICE_RELATION_TYPE = 1;
pub const EndAlternatives: ALTERNATIVE_ARCHITECTURE_TYPE = 2;
pub const EventCategoryDeviceInterfaceChange: IO_NOTIFICATION_EVENT_CATEGORY = 2;
pub const EventCategoryHardwareProfileChange: IO_NOTIFICATION_EVENT_CATEGORY = 1;
pub const EventCategoryKernelSoftRestart: IO_NOTIFICATION_EVENT_CATEGORY = 4;
pub const EventCategoryReserved: IO_NOTIFICATION_EVENT_CATEGORY = 0;
pub const EventCategoryTargetDeviceChange: IO_NOTIFICATION_EVENT_CATEGORY = 3;
pub const EventLoggerHandleClass: TRACE_INFORMATION_CLASS = 5;
pub const Executive: KWAIT_REASON = 0;
pub const ExternalFault: FAULT_INFORMATION_ARM64_TYPE = 3;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type FAST_IO_ACQUIRE_FILE = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT)>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type FAST_IO_ACQUIRE_FOR_CCFLUSH = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, deviceobject: *const DEVICE_OBJECT) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type FAST_IO_ACQUIRE_FOR_MOD_WRITE = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, endingoffset: *const i64, resourcetorelease: *mut *mut ERESOURCE, deviceobject: *const DEVICE_OBJECT) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type FAST_IO_CHECK_IF_POSSIBLE = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, fileoffset: *const i64, length: u32, wait: bool, lockkey: u32, checkforreadoperation: bool, iostatus: *mut super::winternl::IO_STATUS_BLOCK, deviceobject: *const DEVICE_OBJECT) -> bool>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type FAST_IO_DETACH_DEVICE = Option<unsafe extern "system" fn(sourcedevice: *const DEVICE_OBJECT, targetdevice: *const DEVICE_OBJECT)>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type FAST_IO_DEVICE_CONTROL = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, wait: bool, inputbuffer: *const core::ffi::c_void, inputbufferlength: u32, outputbuffer: *mut core::ffi::c_void, outputbufferlength: u32, iocontrolcode: u32, iostatus: *mut super::winternl::IO_STATUS_BLOCK, deviceobject: *const DEVICE_OBJECT) -> bool>;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FAST_IO_DISPATCH {
    pub SizeOfFastIoDispatch: u32,
    pub FastIoCheckIfPossible: PFAST_IO_CHECK_IF_POSSIBLE,
    pub FastIoRead: PFAST_IO_READ,
    pub FastIoWrite: PFAST_IO_WRITE,
    pub FastIoQueryBasicInfo: PFAST_IO_QUERY_BASIC_INFO,
    pub FastIoQueryStandardInfo: PFAST_IO_QUERY_STANDARD_INFO,
    pub FastIoLock: PFAST_IO_LOCK,
    pub FastIoUnlockSingle: PFAST_IO_UNLOCK_SINGLE,
    pub FastIoUnlockAll: PFAST_IO_UNLOCK_ALL,
    pub FastIoUnlockAllByKey: PFAST_IO_UNLOCK_ALL_BY_KEY,
    pub FastIoDeviceControl: PFAST_IO_DEVICE_CONTROL,
    pub AcquireFileForNtCreateSection: PFAST_IO_ACQUIRE_FILE,
    pub ReleaseFileForNtCreateSection: PFAST_IO_RELEASE_FILE,
    pub FastIoDetachDevice: PFAST_IO_DETACH_DEVICE,
    pub FastIoQueryNetworkOpenInfo: PFAST_IO_QUERY_NETWORK_OPEN_INFO,
    pub AcquireForModWrite: PFAST_IO_ACQUIRE_FOR_MOD_WRITE,
    pub MdlRead: PFAST_IO_MDL_READ,
    pub MdlReadComplete: PFAST_IO_MDL_READ_COMPLETE,
    pub PrepareMdlWrite: PFAST_IO_PREPARE_MDL_WRITE,
    pub MdlWriteComplete: PFAST_IO_MDL_WRITE_COMPLETE,
    pub FastIoReadCompressed: PFAST_IO_READ_COMPRESSED,
    pub FastIoWriteCompressed: PFAST_IO_WRITE_COMPRESSED,
    pub MdlReadCompleteCompressed: PFAST_IO_MDL_READ_COMPLETE_COMPRESSED,
    pub MdlWriteCompleteCompressed: PFAST_IO_MDL_WRITE_COMPLETE_COMPRESSED,
    pub FastIoQueryOpen: PFAST_IO_QUERY_OPEN,
    pub ReleaseForModWrite: PFAST_IO_RELEASE_FOR_MOD_WRITE,
    pub AcquireForCcFlush: PFAST_IO_ACQUIRE_FOR_CCFLUSH,
    pub ReleaseForCcFlush: PFAST_IO_RELEASE_FOR_CCFLUSH,
}
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type FAST_IO_LOCK = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, fileoffset: *const i64, length: *const i64, processid: *const _KPROCESS, key: u32, failimmediately: bool, exclusivelock: bool, iostatus: *mut super::winternl::IO_STATUS_BLOCK, deviceobject: *const DEVICE_OBJECT) -> bool>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type FAST_IO_MDL_READ = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, fileoffset: *const i64, length: u32, lockkey: u32, mdlchain: *mut super::usb::PMDL, iostatus: *mut super::winternl::IO_STATUS_BLOCK, deviceobject: *const DEVICE_OBJECT) -> bool>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type FAST_IO_MDL_READ_COMPLETE = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, mdlchain: *const MDL, deviceobject: *const DEVICE_OBJECT) -> bool>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type FAST_IO_MDL_READ_COMPLETE_COMPRESSED = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, mdlchain: *const MDL, deviceobject: *const DEVICE_OBJECT) -> bool>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type FAST_IO_MDL_WRITE_COMPLETE = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, fileoffset: *const i64, mdlchain: *const MDL, deviceobject: *const DEVICE_OBJECT) -> bool>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type FAST_IO_MDL_WRITE_COMPLETE_COMPRESSED = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, fileoffset: *const i64, mdlchain: *const MDL, deviceobject: *const DEVICE_OBJECT) -> bool>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type FAST_IO_PREPARE_MDL_WRITE = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, fileoffset: *const i64, length: u32, lockkey: u32, mdlchain: *mut super::usb::PMDL, iostatus: *mut super::winternl::IO_STATUS_BLOCK, deviceobject: *const DEVICE_OBJECT) -> bool>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type FAST_IO_QUERY_BASIC_INFO = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, wait: bool, buffer: *mut FILE_BASIC_INFORMATION, iostatus: *mut super::winternl::IO_STATUS_BLOCK, deviceobject: *const DEVICE_OBJECT) -> bool>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type FAST_IO_QUERY_NETWORK_OPEN_INFO = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, wait: bool, buffer: *mut FILE_NETWORK_OPEN_INFORMATION, iostatus: *mut super::winternl::IO_STATUS_BLOCK, deviceobject: *const DEVICE_OBJECT) -> bool>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type FAST_IO_QUERY_OPEN = Option<unsafe extern "system" fn(irp: *mut IRP, networkinformation: *mut FILE_NETWORK_OPEN_INFORMATION, deviceobject: *const DEVICE_OBJECT) -> bool>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type FAST_IO_QUERY_STANDARD_INFO = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, wait: bool, buffer: *mut FILE_STANDARD_INFORMATION, iostatus: *mut super::winternl::IO_STATUS_BLOCK, deviceobject: *const DEVICE_OBJECT) -> bool>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type FAST_IO_READ = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, fileoffset: *const i64, length: u32, wait: bool, lockkey: u32, buffer: *mut core::ffi::c_void, iostatus: *mut super::winternl::IO_STATUS_BLOCK, deviceobject: *const DEVICE_OBJECT) -> bool>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type FAST_IO_READ_COMPRESSED = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, fileoffset: *const i64, length: u32, lockkey: u32, buffer: *mut core::ffi::c_void, mdlchain: *mut super::usb::PMDL, iostatus: *mut super::winternl::IO_STATUS_BLOCK, compresseddatainfo: *mut super::ntifs::COMPRESSED_DATA_INFO, compresseddatainfolength: u32, deviceobject: *const DEVICE_OBJECT) -> bool>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type FAST_IO_RELEASE_FILE = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT)>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type FAST_IO_RELEASE_FOR_CCFLUSH = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, deviceobject: *const DEVICE_OBJECT) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type FAST_IO_RELEASE_FOR_MOD_WRITE = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, resourcetorelease: *const ERESOURCE, deviceobject: *const DEVICE_OBJECT) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type FAST_IO_UNLOCK_ALL = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, processid: *const _KPROCESS, iostatus: *mut super::winternl::IO_STATUS_BLOCK, deviceobject: *const DEVICE_OBJECT) -> bool>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type FAST_IO_UNLOCK_ALL_BY_KEY = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, processid: *const core::ffi::c_void, key: u32, iostatus: *mut super::winternl::IO_STATUS_BLOCK, deviceobject: *const DEVICE_OBJECT) -> bool>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type FAST_IO_UNLOCK_SINGLE = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, fileoffset: *const i64, length: *const i64, processid: *const _KPROCESS, key: u32, iostatus: *mut super::winternl::IO_STATUS_BLOCK, deviceobject: *const DEVICE_OBJECT) -> bool>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type FAST_IO_WRITE = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, fileoffset: *const i64, length: u32, wait: bool, lockkey: u32, buffer: *const core::ffi::c_void, iostatus: *mut super::winternl::IO_STATUS_BLOCK, deviceobject: *const DEVICE_OBJECT) -> bool>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type FAST_IO_WRITE_COMPRESSED = Option<unsafe extern "system" fn(fileobject: *const FILE_OBJECT, fileoffset: *const i64, length: u32, lockkey: u32, buffer: *const core::ffi::c_void, mdlchain: *mut super::usb::PMDL, iostatus: *mut super::winternl::IO_STATUS_BLOCK, compresseddatainfo: *const super::ntifs::COMPRESSED_DATA_INFO, compresseddatainfolength: u32, deviceobject: *const DEVICE_OBJECT) -> bool>;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct FAST_MUTEX {
    pub Count: i32,
    pub Owner: *mut core::ffi::c_void,
    pub Contention: u32,
    pub Event: KEVENT,
    pub OldIrql: u32,
}
#[cfg(feature = "winnt")]
impl Default for FAST_MUTEX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub struct FAULT_INFORMATION {
    pub Type: FAULT_INFORMATION_ARCH,
    pub IsStage1: bool,
    pub Anonymous: FAULT_INFORMATION_0,
}
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for FAULT_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub union FAULT_INFORMATION_0 {
    pub Arm64: FAULT_INFORMATION_ARM64,
    pub X64: FAULT_INFORMATION_X64,
}
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for FAULT_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type FAULT_INFORMATION_ARCH = i32;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FAULT_INFORMATION_ARM64 {
    pub DomainHandle: *mut core::ffi::c_void,
    pub FaultAddress: *mut core::ffi::c_void,
    pub PhysicalDeviceObject: PDEVICE_OBJECT,
    pub InputMappingId: u32,
    pub Flags: FAULT_INFORMATION_ARM64_FLAGS,
    pub Type: FAULT_INFORMATION_ARM64_TYPE,
    pub IommuBaseAddress: u64,
}
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for FAULT_INFORMATION_ARM64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FAULT_INFORMATION_ARM64_FLAGS {
    pub _bitfield: u32,
}
pub type FAULT_INFORMATION_ARM64_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FAULT_INFORMATION_X64 {
    pub DomainHandle: *mut core::ffi::c_void,
    pub FaultAddress: *mut core::ffi::c_void,
    pub Flags: FAULT_INFORMATION_X64_FLAGS,
    pub Type: FAULT_INFORMATION_X64_TYPE,
    pub IommuBaseAddress: u64,
    pub PciSegment: u32,
}
impl Default for FAULT_INFORMATION_X64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FAULT_INFORMATION_X64_FLAGS {
    pub _bitfield: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct FAULT_INFORMATION_X64_TYPE(pub FAULT_INFORMATION_ARM64_TYPE);
pub const FILE_128_BYTE_ALIGNMENT: u32 = 127;
pub const FILE_256_BYTE_ALIGNMENT: u32 = 255;
pub const FILE_32_BYTE_ALIGNMENT: u32 = 31;
pub const FILE_512_BYTE_ALIGNMENT: u32 = 511;
pub const FILE_64_BYTE_ALIGNMENT: u32 = 63;
pub const FILE_ATTRIBUTE_VALID_FLAGS: u32 = 5963703;
pub const FILE_ATTRIBUTE_VALID_KERNEL_SET_FLAGS: u32 = 5910951;
pub const FILE_ATTRIBUTE_VALID_SET_FLAGS: u32 = 1716647;
pub const FILE_AUTOGENERATED_DEVICE_NAME: u32 = 128;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_BASIC_INFORMATION {
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub FileAttributes: u32,
}
pub const FILE_BYTE_ALIGNMENT: u32 = 0;
pub const FILE_CHARACTERISTIC_CSV: u32 = 65536;
pub const FILE_CHARACTERISTIC_PNP_DEVICE: u32 = 2048;
pub const FILE_CHARACTERISTIC_TS_DEVICE: u32 = 4096;
pub const FILE_CHARACTERISTIC_WEBDAV_DEVICE: u32 = 8192;
pub const FILE_CONTAINS_EXTENDED_CREATE_INFORMATION: u32 = 268435456;
pub const FILE_DEVICE_ALLOW_APPCONTAINER_TRAVERSAL: u32 = 131072;
pub const FILE_DEVICE_IS_MOUNTED: u32 = 32;
pub const FILE_DEVICE_REQUIRE_SECURITY_CHECK: u32 = 1048576;
pub const FILE_DEVICE_SECURE_OPEN: u32 = 256;
pub const FILE_DISALLOW_EXCLUSIVE: u32 = 131072;
pub const FILE_FLOPPY_DISKETTE: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_FS_DEVICE_INFORMATION {
    pub DeviceType: u32,
    pub Characteristics: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILE_FULL_EA_INFORMATION {
    pub NextEntryOffset: u32,
    pub Flags: u8,
    pub EaNameLength: u8,
    pub EaValueLength: u16,
    pub EaName: [i8; 1],
}
impl Default for FILE_FULL_EA_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_IOSTATUSBLOCK_RANGE_INFORMATION {
    pub IoStatusBlockRange: super::minwindef::PUCHAR,
    pub Length: u32,
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub struct FILE_IO_COMPLETION_INFORMATION {
    pub KeyContext: *mut core::ffi::c_void,
    pub ApcContext: *mut core::ffi::c_void,
    pub IoStatusBlock: super::winternl::IO_STATUS_BLOCK,
}
#[cfg(all(feature = "bcrypt", feature = "winternl"))]
impl Default for FILE_IO_COMPLETION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_IO_COMPLETION_NOTIFICATION_INFORMATION {
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_IO_PRIORITY_HINT_INFORMATION {
    pub PriorityHint: IO_PRIORITY_HINT,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_IO_PRIORITY_HINT_INFORMATION_EX {
    pub PriorityHint: IO_PRIORITY_HINT,
    pub BoostOutstanding: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_IS_REMOTE_DEVICE_INFORMATION {
    pub IsRemote: bool,
}
pub const FILE_LONG_ALIGNMENT: u32 = 3;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_MEMORY_PARTITION_INFORMATION {
    pub OwnerPartitionHandle: usize,
    pub Flags: FILE_MEMORY_PARTITION_INFORMATION_0,
}
impl Default for FILE_MEMORY_PARTITION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union FILE_MEMORY_PARTITION_INFORMATION_0 {
    pub Anonymous: FILE_MEMORY_PARTITION_INFORMATION_0_0,
    pub AllFlags: u32,
}
impl Default for FILE_MEMORY_PARTITION_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILE_MEMORY_PARTITION_INFORMATION_0_0 {
    pub NoCrossPartitionAccess: u8,
    pub Spare: [u8; 3],
}
impl Default for FILE_MEMORY_PARTITION_INFORMATION_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_NETWORK_OPEN_INFORMATION {
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub AllocationSize: i64,
    pub EndOfFile: i64,
    pub FileAttributes: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_NUMA_NODE_INFORMATION {
    pub NodeNumber: u16,
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub struct FILE_OBJECT {
    pub Type: super::ntdef::CSHORT,
    pub Size: super::ntdef::CSHORT,
    pub DeviceObject: PDEVICE_OBJECT,
    pub Vpb: PVPB,
    pub FsContext: *mut core::ffi::c_void,
    pub FsContext2: *mut core::ffi::c_void,
    pub SectionObjectPointer: PSECTION_OBJECT_POINTERS,
    pub PrivateCacheMap: *mut core::ffi::c_void,
    pub FinalStatus: super::bcrypt::NTSTATUS,
    pub RelatedFileObject: *mut Self,
    pub LockOperation: bool,
    pub DeletePending: bool,
    pub ReadAccess: bool,
    pub WriteAccess: bool,
    pub DeleteAccess: bool,
    pub SharedRead: bool,
    pub SharedWrite: bool,
    pub SharedDelete: bool,
    pub Flags: u32,
    pub FileName: super::ntsecapi::UNICODE_STRING,
    pub CurrentByteOffset: i64,
    pub Waiters: u32,
    pub Busy: u32,
    pub LastLock: *mut core::ffi::c_void,
    pub Lock: KEVENT,
    pub Event: KEVENT,
    pub CompletionContext: PIO_COMPLETION_CONTEXT,
    pub IrpListLock: super::winnt::KSPIN_LOCK,
    pub IrpList: super::winnt::LIST_ENTRY,
    pub FileObjectExtension: *mut _IOP_FILE_OBJECT_EXTENSION,
}
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for FILE_OBJECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILE_OCTA_ALIGNMENT: u32 = 15;
pub const FILE_PORTABLE_DEVICE: u32 = 262144;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_POSITION_INFORMATION {
    pub CurrentByteOffset: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FILE_PROCESS_IDS_USING_FILE_INFORMATION {
    pub NumberOfProcessIdsInList: u32,
    pub ProcessIdList: [usize; 1],
}
impl Default for FILE_PROCESS_IDS_USING_FILE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILE_QUAD_ALIGNMENT: u32 = 7;
pub const FILE_QUERY_INDEX_SPECIFIED: u32 = 4;
pub const FILE_QUERY_NO_CURSOR_UPDATE: u32 = 16;
pub const FILE_QUERY_RESTART_SCAN: u32 = 1;
pub const FILE_QUERY_RETURN_ON_DISK_ENTRIES_ONLY: u32 = 8;
pub const FILE_QUERY_RETURN_SINGLE_ENTRY: u32 = 2;
pub const FILE_READ_ONLY_DEVICE: u32 = 2;
pub const FILE_REMOTE_DEVICE: u32 = 16;
pub const FILE_REMOTE_DEVICE_VSMB: u32 = 524288;
pub const FILE_REMOVABLE_MEDIA: u32 = 1;
pub const FILE_SESSION_AWARE: u32 = 262144;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_SFIO_RESERVE_INFORMATION {
    pub RequestsPerPeriod: u32,
    pub Period: u32,
    pub RetryFailures: bool,
    pub Discardable: bool,
    pub RequestSize: u32,
    pub NumOutstandingRequests: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_SFIO_VOLUME_INFORMATION {
    pub MaximumRequestsPerPeriod: u32,
    pub MinimumPeriod: u32,
    pub MinimumTransferSize: u32,
}
pub const FILE_SHARE_VALID_FLAGS: u32 = 7;
pub const FILE_SKIP_SET_USER_EVENT_ON_FAST_IO: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_STANDARD_INFORMATION {
    pub AllocationSize: i64,
    pub EndOfFile: i64,
    pub NumberOfLinks: u32,
    pub DeletePending: bool,
    pub Directory: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FILE_STANDARD_INFORMATION_EX {
    pub AllocationSize: i64,
    pub EndOfFile: i64,
    pub NumberOfLinks: u32,
    pub DeletePending: bool,
    pub Directory: bool,
    pub AlternateStream: bool,
    pub MetadataAttribute: bool,
}
pub const FILE_USE_FILE_POINTER_POSITION: u32 = 4294967294;
pub const FILE_VALID_EXTENDED_OPTION_FLAGS: u32 = 268435456;
pub const FILE_VIRTUAL_VOLUME: u32 = 64;
pub const FILE_WORD_ALIGNMENT: u32 = 1;
pub const FILE_WRITE_ONCE_MEDIA: u32 = 8;
pub const FILE_WRITE_TO_END_OF_FILE: u32 = 4294967295;
pub const FLAG_OWNER_POINTER_IS_THREAD: u32 = 1;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const FLUSH_MULTIPLE_MAXIMUM: u32 = 19;
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
pub const FLUSH_MULTIPLE_MAXIMUM: u32 = 32;
pub const FM_LOCK_BIT: u32 = 1;
pub const FM_LOCK_BIT_V: u32 = 0;
pub const FO_ALERTABLE_IO: u32 = 4;
pub const FO_BYPASS_IO_ENABLED: u32 = 8388608;
pub const FO_CACHE_SUPPORTED: u32 = 64;
pub const FO_CLEANUP_COMPLETE: u32 = 16384;
pub const FO_DELETE_ON_CLOSE: u32 = 65536;
pub const FO_DIRECT_DEVICE_OPEN: u32 = 2048;
pub const FO_DISALLOW_EXCLUSIVE: u32 = 33554432;
pub const FO_FILE_FAST_IO_READ: u32 = 524288;
pub const FO_FILE_MODIFIED: u32 = 4096;
pub const FO_FILE_OPEN: u32 = 1;
pub const FO_FILE_OPEN_CANCELLED: u32 = 2097152;
pub const FO_FILE_SIZE_CHANGED: u32 = 8192;
pub const FO_FLAGS_VALID_ONLY_DURING_CREATE: u32 = 33554432;
pub const FO_GENERATE_AUDIT_ON_CLOSE: u32 = 1024;
pub const FO_HANDLE_CREATED: u32 = 262144;
pub const FO_INDIRECT_WAIT_OBJECT: u32 = 268435456;
pub const FO_MAILSLOT: u32 = 512;
pub const FO_NAMED_PIPE: u32 = 128;
pub const FO_NO_INTERMEDIATE_BUFFERING: u32 = 8;
pub const FO_OPENED_CASE_SENSITIVE: u32 = 131072;
pub const FO_QUEUE_IRP_TO_THREAD: u32 = 1024;
pub const FO_RANDOM_ACCESS: u32 = 1048576;
pub const FO_REMOTE_ORIGIN: u32 = 16777216;
pub const FO_SECTION_MINSTORE_TREATMENT: u32 = 536870912;
pub const FO_SEQUENTIAL_ONLY: u32 = 32;
pub const FO_SKIP_COMPLETION_PORT: u32 = 33554432;
pub const FO_SKIP_SET_EVENT: u32 = 67108864;
pub const FO_SKIP_SET_FAST_IO: u32 = 134217728;
pub const FO_STREAM_FILE: u32 = 256;
pub const FO_SYNCHRONOUS_IO: u32 = 2;
pub const FO_TEMPORARY_FILE: u32 = 32768;
pub const FO_VOLUME_OPEN: u32 = 4194304;
pub const FO_WRITE_THROUGH: u32 = 16;
pub type FPGA_BUS_SCAN = Option<unsafe extern "system" fn(context: *const core::ffi::c_void)>;
#[cfg(feature = "bcrypt")]
pub type FPGA_CONTROL_CONFIG_SPACE = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, enable: bool) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "bcrypt")]
pub type FPGA_CONTROL_ERROR_REPORTING = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, uncorrectablemask: u32, correctablemask: u32, disableerrorreporting: bool) -> super::bcrypt::NTSTATUS>;
#[repr(C)]
#[cfg(feature = "bcrypt")]
#[derive(Clone, Copy, Debug)]
pub struct FPGA_CONTROL_INTERFACE {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
    pub BusScan: PFPGA_BUS_SCAN,
    pub ControlLink: PFPGA_CONTROL_LINK,
    pub ControlConfigSpace: PFPGA_CONTROL_CONFIG_SPACE,
    pub ControlErrorReporting: PFPGA_CONTROL_ERROR_REPORTING,
}
#[cfg(feature = "bcrypt")]
impl Default for FPGA_CONTROL_INTERFACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "bcrypt")]
pub type FPGA_CONTROL_LINK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, enable: bool) -> super::bcrypt::NTSTATUS>;
pub type FREE_FUNCTION = Option<unsafe extern "system" fn(buffer: *const core::ffi::c_void)>;
#[cfg(feature = "winnt")]
pub type FREE_FUNCTION_EX = Option<unsafe extern "system" fn(buffer: *const core::ffi::c_void, lookaside: *mut LOOKASIDE_LIST_EX)>;
pub type FS_INFORMATION_CLASS = i32;
#[repr(C)]
#[cfg(feature = "bcrypt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FUNCTION_LEVEL_DEVICE_RESET_PARAMETERS {
    pub Size: u32,
    pub DeviceResetCompletion: PDEVICE_RESET_COMPLETION,
    pub CompletionContext: *mut core::ffi::c_void,
}
#[cfg(feature = "bcrypt")]
impl Default for FUNCTION_LEVEL_DEVICE_RESET_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type FWMI_NOTIFICATION_CALLBACK = Option<unsafe extern "system" fn(wnode: *mut core::ffi::c_void, context: *mut core::ffi::c_void)>;
pub const FailControl: NPEM_CONTROL_STANDARD_CONTROL_BIT = 4;
pub const FaultInformationArm64: FAULT_INFORMATION_ARCH = 1;
pub const FaultInformationInvalid: FAULT_INFORMATION_ARCH = 0;
pub const FaultInformationX64: FAULT_INFORMATION_ARCH = 2;
pub const FileFsAttributeInformation: FS_INFORMATION_CLASS = 5;
pub const FileFsControlInformation: FS_INFORMATION_CLASS = 6;
pub const FileFsDataCopyInformation: FS_INFORMATION_CLASS = 12;
pub const FileFsDeviceInformation: FS_INFORMATION_CLASS = 4;
pub const FileFsDriverPathInformation: FS_INFORMATION_CLASS = 9;
pub const FileFsFullSizeInformation: FS_INFORMATION_CLASS = 7;
pub const FileFsFullSizeInformationEx: FS_INFORMATION_CLASS = 14;
pub const FileFsGuidInformation: FS_INFORMATION_CLASS = 15;
pub const FileFsLabelInformation: FS_INFORMATION_CLASS = 2;
pub const FileFsMaximumInformation: FS_INFORMATION_CLASS = 16;
pub const FileFsMetadataSizeInformation: FS_INFORMATION_CLASS = 13;
pub const FileFsObjectIdInformation: FS_INFORMATION_CLASS = 8;
pub const FileFsSectorSizeInformation: FS_INFORMATION_CLASS = 11;
pub const FileFsSizeInformation: FS_INFORMATION_CLASS = 3;
pub const FileFsVolumeFlagsInformation: FS_INFORMATION_CLASS = 10;
pub const FileFsVolumeInformation: FS_INFORMATION_CLASS = 1;
pub const FltIoNotifyRoutinesClass: TRACE_INFORMATION_CLASS = 13;
pub const FreePage: KWAIT_REASON = 1;
pub const FunctionLevelDeviceReset: DEVICE_RESET_TYPE = 0;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct GENERAL_LOOKASIDE {
    pub Anonymous: GENERAL_LOOKASIDE_0,
    pub Depth: u16,
    pub MaximumDepth: u16,
    pub TotalAllocates: u32,
    pub Anonymous2: GENERAL_LOOKASIDE_1,
    pub TotalFrees: u32,
    pub Anonymous3: GENERAL_LOOKASIDE_2,
    pub Type: POOL_TYPE,
    pub Tag: u32,
    pub Size: u32,
    pub Anonymous4: GENERAL_LOOKASIDE_3,
    pub Anonymous5: GENERAL_LOOKASIDE_4,
    pub ListEntry: super::winnt::LIST_ENTRY,
    pub LastTotalAllocates: u32,
    pub Anonymous6: GENERAL_LOOKASIDE_5,
    pub Future: [u32; 2],
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
impl Default for GENERAL_LOOKASIDE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union GENERAL_LOOKASIDE_0 {
    pub ListHead: super::winnt::SLIST_HEADER,
    pub SingleListHead: super::winnt::SINGLE_LIST_ENTRY,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
impl Default for GENERAL_LOOKASIDE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union GENERAL_LOOKASIDE_1 {
    pub AllocateMisses: u32,
    pub AllocateHits: u32,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
impl Default for GENERAL_LOOKASIDE_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union GENERAL_LOOKASIDE_2 {
    pub FreeMisses: u32,
    pub FreeHits: u32,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
impl Default for GENERAL_LOOKASIDE_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union GENERAL_LOOKASIDE_3 {
    pub AllocateEx: PALLOCATE_FUNCTION_EX,
    pub Allocate: PALLOCATE_FUNCTION,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
impl Default for GENERAL_LOOKASIDE_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union GENERAL_LOOKASIDE_4 {
    pub FreeEx: PFREE_FUNCTION_EX,
    pub Free: PFREE_FUNCTION,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
impl Default for GENERAL_LOOKASIDE_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union GENERAL_LOOKASIDE_5 {
    pub LastAllocateMisses: u32,
    pub LastAllocateHits: u32,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
impl Default for GENERAL_LOOKASIDE_5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, align(64))]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct GENERAL_LOOKASIDE {
    pub Anonymous: GENERAL_LOOKASIDE_0,
    pub Depth: u16,
    pub MaximumDepth: u16,
    pub TotalAllocates: u32,
    pub Anonymous2: GENERAL_LOOKASIDE_1,
    pub TotalFrees: u32,
    pub Anonymous3: GENERAL_LOOKASIDE_2,
    pub Type: POOL_TYPE,
    pub Tag: u32,
    pub Size: u32,
    pub Anonymous4: GENERAL_LOOKASIDE_3,
    pub Anonymous5: GENERAL_LOOKASIDE_4,
    pub ListEntry: super::winnt::LIST_ENTRY,
    pub LastTotalAllocates: u32,
    pub Anonymous6: GENERAL_LOOKASIDE_5,
    pub Future: [u32; 2],
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
impl Default for GENERAL_LOOKASIDE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union GENERAL_LOOKASIDE_0 {
    pub ListHead: super::winnt::SLIST_HEADER,
    pub SingleListHead: super::winnt::SINGLE_LIST_ENTRY,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
impl Default for GENERAL_LOOKASIDE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union GENERAL_LOOKASIDE_1 {
    pub AllocateMisses: u32,
    pub AllocateHits: u32,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
impl Default for GENERAL_LOOKASIDE_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union GENERAL_LOOKASIDE_2 {
    pub FreeMisses: u32,
    pub FreeHits: u32,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
impl Default for GENERAL_LOOKASIDE_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union GENERAL_LOOKASIDE_3 {
    pub AllocateEx: PALLOCATE_FUNCTION_EX,
    pub Allocate: PALLOCATE_FUNCTION,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
impl Default for GENERAL_LOOKASIDE_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union GENERAL_LOOKASIDE_4 {
    pub FreeEx: PFREE_FUNCTION_EX,
    pub Free: PFREE_FUNCTION,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
impl Default for GENERAL_LOOKASIDE_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union GENERAL_LOOKASIDE_5 {
    pub LastAllocateMisses: u32,
    pub LastAllocateHits: u32,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
impl Default for GENERAL_LOOKASIDE_5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, align(128))]
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct GENERAL_LOOKASIDE {
    pub Anonymous: GENERAL_LOOKASIDE_0,
    pub Depth: u16,
    pub MaximumDepth: u16,
    pub TotalAllocates: u32,
    pub Anonymous2: GENERAL_LOOKASIDE_1,
    pub TotalFrees: u32,
    pub Anonymous3: GENERAL_LOOKASIDE_2,
    pub Type: POOL_TYPE,
    pub Tag: u32,
    pub Size: u32,
    pub Anonymous4: GENERAL_LOOKASIDE_3,
    pub Anonymous5: GENERAL_LOOKASIDE_4,
    pub ListEntry: super::winnt::LIST_ENTRY,
    pub LastTotalAllocates: u32,
    pub Anonymous6: GENERAL_LOOKASIDE_5,
    pub Future: [u32; 2],
}
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "winnt")]
impl Default for GENERAL_LOOKASIDE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union GENERAL_LOOKASIDE_0 {
    pub ListHead: super::winnt::SLIST_HEADER,
    pub SingleListHead: super::winnt::SINGLE_LIST_ENTRY,
}
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "winnt")]
impl Default for GENERAL_LOOKASIDE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union GENERAL_LOOKASIDE_1 {
    pub AllocateMisses: u32,
    pub AllocateHits: u32,
}
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "winnt")]
impl Default for GENERAL_LOOKASIDE_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union GENERAL_LOOKASIDE_2 {
    pub FreeMisses: u32,
    pub FreeHits: u32,
}
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "winnt")]
impl Default for GENERAL_LOOKASIDE_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union GENERAL_LOOKASIDE_3 {
    pub AllocateEx: PALLOCATE_FUNCTION_EX,
    pub Allocate: PALLOCATE_FUNCTION,
}
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "winnt")]
impl Default for GENERAL_LOOKASIDE_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union GENERAL_LOOKASIDE_4 {
    pub FreeEx: PFREE_FUNCTION_EX,
    pub Free: PFREE_FUNCTION,
}
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "winnt")]
impl Default for GENERAL_LOOKASIDE_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union GENERAL_LOOKASIDE_5 {
    pub LastAllocateMisses: u32,
    pub LastAllocateHits: u32,
}
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "winnt")]
impl Default for GENERAL_LOOKASIDE_5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct GENERAL_LOOKASIDE_POOL {
    pub Anonymous: GENERAL_LOOKASIDE_POOL_0,
    pub Depth: u16,
    pub MaximumDepth: u16,
    pub TotalAllocates: u32,
    pub Anonymous2: GENERAL_LOOKASIDE_POOL_1,
    pub TotalFrees: u32,
    pub Anonymous3: GENERAL_LOOKASIDE_POOL_2,
    pub Type: POOL_TYPE,
    pub Tag: u32,
    pub Size: u32,
    pub Anonymous4: GENERAL_LOOKASIDE_POOL_3,
    pub Anonymous5: GENERAL_LOOKASIDE_POOL_4,
    pub ListEntry: super::winnt::LIST_ENTRY,
    pub LastTotalAllocates: u32,
    pub Anonymous6: GENERAL_LOOKASIDE_POOL_5,
    pub Future: [u32; 2],
}
#[cfg(feature = "winnt")]
impl Default for GENERAL_LOOKASIDE_POOL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union GENERAL_LOOKASIDE_POOL_0 {
    pub ListHead: super::winnt::SLIST_HEADER,
    pub SingleListHead: super::winnt::SINGLE_LIST_ENTRY,
}
#[cfg(feature = "winnt")]
impl Default for GENERAL_LOOKASIDE_POOL_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union GENERAL_LOOKASIDE_POOL_1 {
    pub AllocateMisses: u32,
    pub AllocateHits: u32,
}
#[cfg(feature = "winnt")]
impl Default for GENERAL_LOOKASIDE_POOL_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union GENERAL_LOOKASIDE_POOL_2 {
    pub FreeMisses: u32,
    pub FreeHits: u32,
}
#[cfg(feature = "winnt")]
impl Default for GENERAL_LOOKASIDE_POOL_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union GENERAL_LOOKASIDE_POOL_3 {
    pub AllocateEx: PALLOCATE_FUNCTION_EX,
    pub Allocate: PALLOCATE_FUNCTION,
}
#[cfg(feature = "winnt")]
impl Default for GENERAL_LOOKASIDE_POOL_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union GENERAL_LOOKASIDE_POOL_4 {
    pub FreeEx: PFREE_FUNCTION_EX,
    pub Free: PFREE_FUNCTION,
}
#[cfg(feature = "winnt")]
impl Default for GENERAL_LOOKASIDE_POOL_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union GENERAL_LOOKASIDE_POOL_5 {
    pub LastAllocateMisses: u32,
    pub LastAllocateHits: u32,
}
#[cfg(feature = "winnt")]
impl Default for GENERAL_LOOKASIDE_POOL_5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "bcrypt")]
pub type GET_D3COLD_CAPABILITY = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, d3coldsupported: *mut bool) -> super::bcrypt::NTSTATUS>;
pub type GET_D3COLD_LAST_TRANSITION_STATUS = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, lasttransitionstatus: *mut D3COLD_LAST_TRANSITION_STATUS)>;
#[cfg(feature = "bcrypt")]
pub type GET_DEVICE_RESET_STATUS = Option<unsafe extern "system" fn(interfacecontext: *const core::ffi::c_void, isresetting: *mut bool, resettypeselected: *mut DEVICE_BUS_SPECIFIC_RESET_TYPE, flags: *mut DEVICE_RESET_STATUS_FLAGS) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type GET_DMA_ADAPTER = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, devicedescriptor: *const DEVICE_DESCRIPTION, numberofmapregisters: *mut u32) -> *mut DMA_ADAPTER>;
#[cfg(feature = "bcrypt")]
pub type GET_DOE_PREVIOUS_RESPONSE = Option<unsafe extern "system" fn(interfacecontext: *const core::ffi::c_void, requestid: i64, outputarraycount: u32, outputwritten: *mut u32, outputarray: *mut u32, doestatus: *mut super::bcrypt::NTSTATUS) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
pub type GET_IDLE_WAKE_INFO = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, systempowerstate: super::winnt::SYSTEM_POWER_STATE, deepestwakeabledstate: *mut DEVICE_WAKE_DEPTH) -> super::bcrypt::NTSTATUS>;
pub type GET_SDEV_IDENTIFIER = Option<unsafe extern "system" fn(interfacecontext: *const core::ffi::c_void) -> u64>;
pub type GET_SET_DEVICE_DATA = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, datatype: u32, buffer: *mut core::ffi::c_void, offset: u32, length: u32) -> u32>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "usb"))]
pub type GET_UPDATED_BUS_RESOURCE = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, updatedresourcelist: *mut PCM_RESOURCE_LIST, updatedtranslatedresourcelist: *mut PCM_RESOURCE_LIST) -> super::bcrypt::NTSTATUS>;
pub type GET_VIRTUAL_DEVICE_DATA = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, virtualfunction: u16, buffer: *mut core::ffi::c_void, offset: u32, length: u32) -> u32>;
#[cfg(feature = "bcrypt")]
pub type GET_VIRTUAL_DEVICE_LOCATION = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, virtualfunction: u16, segmentnumber: *mut u16, busnumber: *mut u8, functionnumber: *mut u8) -> super::bcrypt::NTSTATUS>;
pub type GET_VIRTUAL_DEVICE_RESOURCES = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, capturedbusnumbers: *mut u8)>;
#[cfg(feature = "bcrypt")]
pub type GET_VIRTUAL_FUNCTION_PROBED_BARS = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, baseregistervalues: *mut u32) -> super::bcrypt::NTSTATUS>;
pub const GlobalLoggerHandleClass: TRACE_INFORMATION_CLASS = 4;
pub const GroupAffinityAllGroupZero: IRQ_GROUP_POLICY = 0;
pub const GroupAffinityDontCare: IRQ_GROUP_POLICY = 1;
pub const HAL_DMA_ADAPTER_VERSION_1: u32 = 1;
pub const HAL_MASK_UNMASK_FLAGS_NONE: u32 = 0;
pub const HAL_MASK_UNMASK_FLAGS_SERVICING_COMPLETE: u32 = 2;
pub const HAL_MASK_UNMASK_FLAGS_SERVICING_DEFERRED: u32 = 1;
pub const HASH_STRING_ALGORITHM_DEFAULT: u32 = 0;
pub const HASH_STRING_ALGORITHM_INVALID: u32 = 4294967295;
pub const HASH_STRING_ALGORITHM_X65599: u32 = 1;
pub const HIGH_PRIORITY: u32 = 31;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct HWPROFILE_CHANGE_NOTIFICATION {
    pub Version: u16,
    pub Size: u16,
    pub Event: windows_core::GUID,
}
pub const HighImportance: KDPC_IMPORTANCE = 2;
pub const HighPagePriority: MM_PAGE_PRIORITY = 32;
pub const HighPoolPriority: EX_POOL_PRIORITY = 32;
pub const HighPoolPrioritySpecialPoolOverrun: EX_POOL_PRIORITY = 40;
pub const HighPoolPrioritySpecialPoolUnderrun: EX_POOL_PRIORITY = 41;
pub const HotSpareControl: NPEM_CONTROL_STANDARD_CONTROL_BIT = 7;
pub const HyperCriticalWorkQueue: WORK_QUEUE_TYPE = 2;
pub const INITIAL_PRIVILEGE_COUNT: u32 = 3;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct INITIAL_PRIVILEGE_SET {
    pub PrivilegeCount: u32,
    pub Control: u32,
    pub Privilege: [super::winnt::LUID_AND_ATTRIBUTES; 3],
}
#[cfg(feature = "winnt")]
impl Default for INITIAL_PRIVILEGE_SET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INPUT_MAPPING_ELEMENT {
    pub InputMappingId: u32,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub union INTEL_CACHE_INFO_EAX {
    pub Ulong: u32,
    pub Anonymous: INTEL_CACHE_INFO_EAX_0,
}
#[cfg(target_arch = "x86")]
impl Default for INTEL_CACHE_INFO_EAX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INTEL_CACHE_INFO_EAX_0 {
    pub _bitfield: INTEL_CACHE_TYPE,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy)]
pub union INTEL_CACHE_INFO_EBX {
    pub Ulong: u32,
    pub Anonymous: INTEL_CACHE_INFO_EBX_0,
}
#[cfg(target_arch = "x86")]
impl Default for INTEL_CACHE_INFO_EBX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct INTEL_CACHE_INFO_EBX_0 {
    pub _bitfield: u32,
}
#[cfg(target_arch = "x86")]
pub type INTEL_CACHE_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct INTERFACE {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
}
impl Default for INTERFACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type INTERFACE_TYPE = i32;
pub const INVALID_PROCESSOR_INDEX: u32 = 4294967295;
pub const IOCTL_CANCEL_DEVICE_WAKE: u32 = 2719752;
pub const IOCTL_QUERY_DEVICE_POWER_STATE: u32 = 2703360;
pub const IOCTL_SET_DEVICE_WAKE: u32 = 2719748;
pub const IOMMU_ACCESS_EXECUTE: u32 = 12;
pub const IOMMU_ACCESS_NONE: u32 = 0;
pub const IOMMU_ACCESS_READ: u32 = 1;
pub const IOMMU_ACCESS_WRITE: u32 = 2;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type IOMMU_DEVICE_CREATE = Option<unsafe extern "system" fn(deviceobject: *const DEVICE_OBJECT, deviceconfig: *const IOMMU_DEVICE_CREATION_CONFIGURATION, dmadeviceout: *mut PIOMMU_DMA_DEVICE) -> super::bcrypt::NTSTATUS>;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct IOMMU_DEVICE_CREATION_CONFIGURATION {
    pub NextConfiguration: super::winnt::LIST_ENTRY,
    pub ConfigType: IOMMU_DEVICE_CREATION_CONFIGURATION_TYPE,
    pub Anonymous: IOMMU_DEVICE_CREATION_CONFIGURATION_0,
}
#[cfg(feature = "winnt")]
impl Default for IOMMU_DEVICE_CREATION_CONFIGURATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union IOMMU_DEVICE_CREATION_CONFIGURATION_0 {
    pub Acpi: IOMMU_DEVICE_CREATION_CONFIGURATION_ACPI,
    pub DeviceId: *mut core::ffi::c_void,
    pub Pasid: IOMMU_DEVICE_CREATION_CONFIGURATION_PASID,
}
#[cfg(feature = "winnt")]
impl Default for IOMMU_DEVICE_CREATION_CONFIGURATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IOMMU_DEVICE_CREATION_CONFIGURATION_ACPI {
    pub InputMappingBase: u32,
    pub MappingsCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IOMMU_DEVICE_CREATION_CONFIGURATION_PASID {
    pub ConfigType: IOMMU_PASID_CONFIGURATION_TYPE,
    pub SuppressPasidFaults: bool,
}
pub type IOMMU_DEVICE_CREATION_CONFIGURATION_TYPE = i32;
#[cfg(feature = "bcrypt")]
pub type IOMMU_DEVICE_DELETE = Option<unsafe extern "system" fn(dmadevice: *const IOMMU_DMA_DEVICE) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type IOMMU_DEVICE_FAULT_HANDLER = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, faultinformation: *const FAULT_INFORMATION)>;
pub type IOMMU_DEVICE_QUERY_DOMAIN_TYPES = Option<unsafe extern "system" fn(dmadevice: *const IOMMU_DMA_DEVICE, availabledomains: *mut u32)>;
#[cfg(feature = "bcrypt")]
pub type IOMMU_DEVICE_QUERY_INFORMATION = Option<unsafe extern "system" fn(dmadevice: *const IOMMU_DMA_DEVICE, size: u32, byteswritten: *mut u32, buffer: *mut IOMMU_DMA_DEVICE_INFORMATION) -> super::bcrypt::NTSTATUS>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IOMMU_DMA_DEVICE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IOMMU_DMA_DEVICE_INFORMATION {
    pub DefaultPasidEnabled: bool,
    pub PasidTaggedDmaEnabled: bool,
    pub PasidFaultsSuppressed: bool,
}
pub const IOMMU_DMA_DEVICE_INFORMATION_SIZE_MIN: u32 = 3;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IOMMU_DMA_DOMAIN(pub u8);
#[repr(C)]
#[derive(Clone, Copy)]
pub union IOMMU_DMA_DOMAIN_CREATION_FLAGS {
    pub Anonymous: IOMMU_DMA_DOMAIN_CREATION_FLAGS_0,
    pub AsUlonglong: u64,
}
impl Default for IOMMU_DMA_DOMAIN_CREATION_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IOMMU_DMA_DOMAIN_CREATION_FLAGS_0 {
    pub _bitfield: u64,
}
pub type IOMMU_DMA_DOMAIN_TYPE = i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct IOMMU_DMA_LOGICAL_ADDRESS(pub u64);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IOMMU_DMA_LOGICAL_ADDRESS_TOKEN {
    pub LogicalAddressBase: IOMMU_DMA_LOGICAL_ADDRESS,
    pub Size: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IOMMU_DMA_LOGICAL_ADDRESS_TOKEN_MAPPED_SEGMENT {
    pub OwningToken: PIOMMU_DMA_LOGICAL_ADDRESS_TOKEN,
    pub Offset: usize,
    pub Size: usize,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IOMMU_DMA_LOGICAL_ALLOCATOR_CONFIG {
    pub LogicalAllocatorType: IOMMU_DMA_LOGICAL_ALLOCATOR_TYPE,
    pub Anonymous: IOMMU_DMA_LOGICAL_ALLOCATOR_CONFIG_0,
}
impl Default for IOMMU_DMA_LOGICAL_ALLOCATOR_CONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IOMMU_DMA_LOGICAL_ALLOCATOR_CONFIG_0 {
    pub BuddyAllocatorConfig: IOMMU_DMA_LOGICAL_ALLOCATOR_CONFIG_0_0,
}
impl Default for IOMMU_DMA_LOGICAL_ALLOCATOR_CONFIG_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IOMMU_DMA_LOGICAL_ALLOCATOR_CONFIG_0_0 {
    pub AddressWidth: u32,
}
pub type IOMMU_DMA_LOGICAL_ALLOCATOR_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IOMMU_DMA_PASID_DEVICE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IOMMU_DMA_RESERVED_REGION {
    pub RegionNext: *mut Self,
    pub Base: IOMMU_DMA_LOGICAL_ADDRESS,
    pub NumberOfPages: usize,
    pub ShouldMap: bool,
}
impl Default for IOMMU_DMA_RESERVED_REGION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type IOMMU_DOMAIN_ATTACH_DEVICE = Option<unsafe extern "system" fn(domain: *const IOMMU_DMA_DOMAIN, physicaldeviceobject: *const DEVICE_OBJECT, inputmappingidbase: u32, mappingcount: u32) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "bcrypt")]
pub type IOMMU_DOMAIN_ATTACH_DEVICE_EX = Option<unsafe extern "system" fn(domain: *const IOMMU_DMA_DOMAIN, dmadevice: *const IOMMU_DMA_DEVICE) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "bcrypt")]
pub type IOMMU_DOMAIN_ATTACH_PASID_DEVICE = Option<unsafe extern "system" fn(domain: *const IOMMU_DMA_DOMAIN, pasiddevice: *const IOMMU_DMA_PASID_DEVICE) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "usb"))]
pub type IOMMU_DOMAIN_CONFIGURE = Option<unsafe extern "system" fn(domain: *const IOMMU_DMA_DOMAIN, configuration: *const DOMAIN_CONFIGURATION) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "bcrypt")]
pub type IOMMU_DOMAIN_CREATE = Option<unsafe extern "system" fn(osmanagedpagetable: bool, domainout: *mut PIOMMU_DMA_DOMAIN) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "bcrypt")]
pub type IOMMU_DOMAIN_CREATE_EX = Option<unsafe extern "system" fn(domaintype: IOMMU_DMA_DOMAIN_TYPE, flags: IOMMU_DMA_DOMAIN_CREATION_FLAGS, logicalallocatorconfig: *const IOMMU_DMA_LOGICAL_ALLOCATOR_CONFIG, reservedregions: *const IOMMU_DMA_RESERVED_REGION, domainout: *mut PIOMMU_DMA_DOMAIN) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "bcrypt")]
pub type IOMMU_DOMAIN_DELETE = Option<unsafe extern "system" fn(domain: *const IOMMU_DMA_DOMAIN) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type IOMMU_DOMAIN_DETACH_DEVICE = Option<unsafe extern "system" fn(domain: *const IOMMU_DMA_DOMAIN, physicaldeviceobject: *const DEVICE_OBJECT, inputmappingid: u32) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "bcrypt")]
pub type IOMMU_DOMAIN_DETACH_DEVICE_EX = Option<unsafe extern "system" fn(dmadevice: *const IOMMU_DMA_DEVICE) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "bcrypt")]
pub type IOMMU_DOMAIN_DETACH_PASID_DEVICE = Option<unsafe extern "system" fn(pasiddevice: *const IOMMU_DMA_PASID_DEVICE) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "bcrypt")]
pub type IOMMU_FLUSH_DOMAIN = Option<unsafe extern "system" fn(domain: *const IOMMU_DMA_DOMAIN) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "bcrypt")]
pub type IOMMU_FLUSH_DOMAIN_VA_LIST = Option<unsafe extern "system" fn(domain: *const IOMMU_DMA_DOMAIN, lastlevel: bool, number: u32, valist: *const core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "bcrypt")]
pub type IOMMU_FREE_RESERVED_LOGICAL_ADDRESS_RANGE = Option<unsafe extern "system" fn(logicaladdresstoken: *const IOMMU_DMA_LOGICAL_ADDRESS_TOKEN) -> super::bcrypt::NTSTATUS>;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IOMMU_INTERFACE_STATE_CHANGE {
    pub PresentFields: IOMMU_INTERFACE_STATE_CHANGE_FIELDS,
    pub AvailableDomainTypes: u32,
}
impl Default for IOMMU_INTERFACE_STATE_CHANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type IOMMU_INTERFACE_STATE_CHANGE_CALLBACK = Option<unsafe extern "system" fn(statechange: *const IOMMU_INTERFACE_STATE_CHANGE, context: *const core::ffi::c_void)>;
#[repr(C)]
#[derive(Clone, Copy)]
pub union IOMMU_INTERFACE_STATE_CHANGE_FIELDS {
    pub Anonymous: IOMMU_INTERFACE_STATE_CHANGE_FIELDS_0,
    pub AsULONG: u32,
}
impl Default for IOMMU_INTERFACE_STATE_CHANGE_FIELDS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IOMMU_INTERFACE_STATE_CHANGE_FIELDS_0 {
    pub _bitfield: u32,
}
#[cfg(all(feature = "bcrypt", feature = "ntdef"))]
pub type IOMMU_MAP_IDENTITY_RANGE = Option<unsafe extern "system" fn(domain: *const IOMMU_DMA_DOMAIN, permissions: u32, mdl: *const MDL) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "usb"))]
pub type IOMMU_MAP_IDENTITY_RANGE_EX = Option<unsafe extern "system" fn(domain: *const IOMMU_DMA_DOMAIN, permissions: u32, physicaladdresstomap: *const IOMMU_MAP_PHYSICAL_ADDRESS) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "ntdef"))]
pub type IOMMU_MAP_LOGICAL_RANGE = Option<unsafe extern "system" fn(domain: *const IOMMU_DMA_DOMAIN, permissions: u32, mdl: *const MDL, logicaladdress: u64) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "usb"))]
pub type IOMMU_MAP_LOGICAL_RANGE_EX = Option<unsafe extern "system" fn(domain: *const IOMMU_DMA_DOMAIN, permissions: u32, physicaladdresstomap: *const IOMMU_MAP_PHYSICAL_ADDRESS, explicitlogicaladdress: *const u64, minlogicaladdress: *const u64, maxlogicaladdress: *const u64, logicaladdressout: *mut u64) -> super::bcrypt::NTSTATUS>;
#[repr(C)]
#[cfg(feature = "usb")]
#[derive(Clone, Copy)]
pub struct IOMMU_MAP_PHYSICAL_ADDRESS {
    pub MapType: IOMMU_MAP_PHYSICAL_ADDRESS_TYPE,
    pub Anonymous: IOMMU_MAP_PHYSICAL_ADDRESS_0,
}
#[cfg(feature = "usb")]
impl Default for IOMMU_MAP_PHYSICAL_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "usb")]
#[derive(Clone, Copy)]
pub union IOMMU_MAP_PHYSICAL_ADDRESS_0 {
    pub Mdl: IOMMU_MAP_PHYSICAL_ADDRESS_0_0,
    pub ContiguousRange: IOMMU_MAP_PHYSICAL_ADDRESS_0_1,
    pub PfnArray: IOMMU_MAP_PHYSICAL_ADDRESS_0_2,
}
#[cfg(feature = "usb")]
impl Default for IOMMU_MAP_PHYSICAL_ADDRESS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "usb")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IOMMU_MAP_PHYSICAL_ADDRESS_0_0 {
    pub Mdl: super::usb::PMDL,
}
#[repr(C)]
#[cfg(feature = "usb")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IOMMU_MAP_PHYSICAL_ADDRESS_0_1 {
    pub Base: super::usb::PHYSICAL_ADDRESS,
    pub Size: usize,
}
#[repr(C)]
#[cfg(feature = "usb")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IOMMU_MAP_PHYSICAL_ADDRESS_0_2 {
    pub PageFrame: PPFN_NUMBER,
    pub NumberOfPages: usize,
}
pub type IOMMU_MAP_PHYSICAL_ADDRESS_TYPE = i32;
#[cfg(all(feature = "bcrypt", feature = "usb"))]
pub type IOMMU_MAP_RESERVED_LOGICAL_RANGE = Option<unsafe extern "system" fn(logicaladdresstoken: *mut IOMMU_DMA_LOGICAL_ADDRESS_TOKEN, offset: usize, permissions: u32, physicaladdresstomap: *const IOMMU_MAP_PHYSICAL_ADDRESS, mappedsegment: *mut IOMMU_DMA_LOGICAL_ADDRESS_TOKEN_MAPPED_SEGMENT) -> super::bcrypt::NTSTATUS>;
pub type IOMMU_PASID_CONFIGURATION_TYPE = i32;
#[cfg(feature = "bcrypt")]
pub type IOMMU_PASID_DEVICE_CREATE = Option<unsafe extern "system" fn(dmadevice: *const IOMMU_DMA_DEVICE, pasiddeviceout: *mut PIOMMU_DMA_PASID_DEVICE, asidout: *mut u32) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "bcrypt")]
pub type IOMMU_PASID_DEVICE_DELETE = Option<unsafe extern "system" fn(pasiddevice: *const IOMMU_DMA_PASID_DEVICE) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type IOMMU_QUERY_INPUT_MAPPINGS = Option<unsafe extern "system" fn(physicaldeviceobject: *const DEVICE_OBJECT, buffer: *mut INPUT_MAPPING_ELEMENT, bufferlength: u32, returnlength: *mut u32) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "bcrypt")]
pub type IOMMU_REGISTER_INTERFACE_STATE_CHANGE_CALLBACK = Option<unsafe extern "system" fn(statechangecallback: PIOMMU_INTERFACE_STATE_CHANGE_CALLBACK, context: *const core::ffi::c_void, dmadevice: *const IOMMU_DMA_DEVICE, statefields: *const IOMMU_INTERFACE_STATE_CHANGE_FIELDS) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "bcrypt")]
pub type IOMMU_RESERVE_LOGICAL_ADDRESS_RANGE = Option<unsafe extern "system" fn(domain: *const IOMMU_DMA_DOMAIN, size: usize, explicitlogicaladdress: *const u64, minlogicaladdress: *const u64, maxlogicaladdress: *const u64, logicaladdresstoken: *mut PIOMMU_DMA_LOGICAL_ADDRESS_TOKEN) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type IOMMU_SET_DEVICE_FAULT_REPORTING = Option<unsafe extern "system" fn(physicaldeviceobject: *const DEVICE_OBJECT, inputmappingidbase: u32, enable: bool, faultconfig: *const DEVICE_FAULT_CONFIGURATION) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type IOMMU_SET_DEVICE_FAULT_REPORTING_EX = Option<unsafe extern "system" fn(dmadevice: *const IOMMU_DMA_DEVICE, inputmappingidbase: u32, enable: bool, faultconfig: *const DEVICE_FAULT_CONFIGURATION) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "ntdef"))]
pub type IOMMU_UNMAP_IDENTITY_RANGE = Option<unsafe extern "system" fn(domain: *const IOMMU_DMA_DOMAIN, mdl: *const MDL) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "usb"))]
pub type IOMMU_UNMAP_IDENTITY_RANGE_EX = Option<unsafe extern "system" fn(domain: *const IOMMU_DMA_DOMAIN, mappedphysicaladdress: *const IOMMU_MAP_PHYSICAL_ADDRESS) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "bcrypt")]
pub type IOMMU_UNMAP_LOGICAL_RANGE = Option<unsafe extern "system" fn(domain: *const IOMMU_DMA_DOMAIN, logicaladdress: u64, numberofpages: u64) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "bcrypt")]
pub type IOMMU_UNMAP_RESERVED_LOGICAL_RANGE = Option<unsafe extern "system" fn(mappedsegment: *mut IOMMU_DMA_LOGICAL_ADDRESS_TOKEN_MAPPED_SEGMENT) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "bcrypt")]
pub type IOMMU_UNREGISTER_INTERFACE_STATE_CHANGE_CALLBACK = Option<unsafe extern "system" fn(statechangecallback: PIOMMU_INTERFACE_STATE_CHANGE_CALLBACK, dmadevice: *const IOMMU_DMA_DEVICE) -> super::bcrypt::NTSTATUS>;
pub type IO_ACCESS_MODE = i32;
pub type IO_ACCESS_TYPE = i32;
pub type IO_ALLOCATION_ACTION = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IO_ATTRIBUTION_INFORMATION {
    pub Version: u32,
    pub Flags: IO_ATTRIBUTION_INFORMATION_0,
    pub Length: u32,
    pub ServiceStartTime: u64,
    pub CurrentTime: u64,
}
impl Default for IO_ATTRIBUTION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IO_ATTRIBUTION_INFORMATION_0 {
    pub Anonymous: IO_ATTRIBUTION_INFORMATION_0_0,
    pub AllFlags: u32,
}
impl Default for IO_ATTRIBUTION_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_ATTRIBUTION_INFORMATION_0_0 {
    pub _bitfield: u32,
}
pub const IO_ATTRIBUTION_INFO_V1: u32 = 1;
pub const IO_CD_ROM_INCREMENT: u32 = 1;
pub const IO_CHECK_SHARE_ACCESS_DONT_CHECK_DELETE: u32 = 16;
pub const IO_CHECK_SHARE_ACCESS_DONT_CHECK_READ: u32 = 4;
pub const IO_CHECK_SHARE_ACCESS_DONT_CHECK_WRITE: u32 = 8;
pub const IO_CHECK_SHARE_ACCESS_DONT_UPDATE_FILE_OBJECT: u32 = 2;
pub const IO_CHECK_SHARE_ACCESS_FORCE_CHECK: u32 = 32;
pub const IO_CHECK_SHARE_ACCESS_FORCE_USING_SCB: u32 = 64;
pub const IO_CHECK_SHARE_ACCESS_UPDATE_SHARE_ACCESS: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IO_COMPLETION_CONTEXT {
    pub Port: *mut core::ffi::c_void,
    pub Key: *mut core::ffi::c_void,
    pub UsageCount: isize,
}
impl Default for IO_COMPLETION_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type IO_COMPLETION_ROUTINE = Option<unsafe extern "system" fn(deviceobject: *const DEVICE_OBJECT, irp: *const IRP, context: *const core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
pub type IO_COMPLETION_ROUTINE_RESULT = i32;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IO_CONNECT_INTERRUPT_FULLY_SPECIFIED_PARAMETERS {
    pub PhysicalDeviceObject: PDEVICE_OBJECT,
    pub InterruptObject: *mut PKINTERRUPT,
    pub ServiceRoutine: PKSERVICE_ROUTINE,
    pub ServiceContext: *mut core::ffi::c_void,
    pub SpinLock: super::winnt::PKSPIN_LOCK,
    pub SynchronizeIrql: super::ntdef::KIRQL,
    pub FloatingSave: bool,
    pub ShareVector: bool,
    pub Vector: u32,
    pub Irql: super::ntdef::KIRQL,
    pub InterruptMode: KINTERRUPT_MODE,
    pub ProcessorEnableMask: super::basetsd::KAFFINITY,
    pub Group: u16,
}
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IO_CONNECT_INTERRUPT_FULLY_SPECIFIED_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IO_CONNECT_INTERRUPT_LINE_BASED_PARAMETERS {
    pub PhysicalDeviceObject: PDEVICE_OBJECT,
    pub InterruptObject: *mut PKINTERRUPT,
    pub ServiceRoutine: PKSERVICE_ROUTINE,
    pub ServiceContext: *mut core::ffi::c_void,
    pub SpinLock: super::winnt::PKSPIN_LOCK,
    pub SynchronizeIrql: super::ntdef::KIRQL,
    pub FloatingSave: bool,
}
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IO_CONNECT_INTERRUPT_LINE_BASED_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub struct IO_CONNECT_INTERRUPT_MESSAGE_BASED_PARAMETERS {
    pub PhysicalDeviceObject: PDEVICE_OBJECT,
    pub ConnectionContext: IO_CONNECT_INTERRUPT_MESSAGE_BASED_PARAMETERS_0,
    pub MessageServiceRoutine: PKMESSAGE_SERVICE_ROUTINE,
    pub ServiceContext: *mut core::ffi::c_void,
    pub SpinLock: super::winnt::PKSPIN_LOCK,
    pub SynchronizeIrql: super::ntdef::KIRQL,
    pub FloatingSave: bool,
    pub FallBackServiceRoutine: PKSERVICE_ROUTINE,
}
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IO_CONNECT_INTERRUPT_MESSAGE_BASED_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub union IO_CONNECT_INTERRUPT_MESSAGE_BASED_PARAMETERS_0 {
    pub Generic: *mut *mut core::ffi::c_void,
    pub InterruptMessageTable: *mut PIO_INTERRUPT_MESSAGE_INFO,
    pub InterruptObject: *mut PKINTERRUPT,
}
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IO_CONNECT_INTERRUPT_MESSAGE_BASED_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub struct IO_CONNECT_INTERRUPT_PARAMETERS {
    pub Version: u32,
    pub Anonymous: IO_CONNECT_INTERRUPT_PARAMETERS_0,
}
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IO_CONNECT_INTERRUPT_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub union IO_CONNECT_INTERRUPT_PARAMETERS_0 {
    pub FullySpecified: IO_CONNECT_INTERRUPT_FULLY_SPECIFIED_PARAMETERS,
    pub LineBased: IO_CONNECT_INTERRUPT_LINE_BASED_PARAMETERS,
    pub MessageBased: IO_CONNECT_INTERRUPT_MESSAGE_BASED_PARAMETERS,
}
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IO_CONNECT_INTERRUPT_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type IO_CONTAINER_INFORMATION_CLASS = i32;
pub type IO_CONTAINER_NOTIFICATION_CLASS = i32;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IO_CSQ {
    pub Type: u32,
    pub CsqInsertIrp: PIO_CSQ_INSERT_IRP,
    pub CsqRemoveIrp: PIO_CSQ_REMOVE_IRP,
    pub CsqPeekNextIrp: PIO_CSQ_PEEK_NEXT_IRP,
    pub CsqAcquireLock: PIO_CSQ_ACQUIRE_LOCK,
    pub CsqReleaseLock: PIO_CSQ_RELEASE_LOCK,
    pub CsqCompleteCanceledIrp: PIO_CSQ_COMPLETE_CANCELED_IRP,
    pub ReservePointer: *mut core::ffi::c_void,
}
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IO_CSQ {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type IO_CSQ_ACQUIRE_LOCK = Option<unsafe extern "system" fn(csq: *const IO_CSQ, irql: *mut super::ntdef::KIRQL)>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type IO_CSQ_COMPLETE_CANCELED_IRP = Option<unsafe extern "system" fn(csq: *const IO_CSQ, irp: *const IRP)>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type IO_CSQ_INSERT_IRP = Option<unsafe extern "system" fn(csq: *const IO_CSQ, irp: *const IRP)>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type IO_CSQ_INSERT_IRP_EX = Option<unsafe extern "system" fn(csq: *const IO_CSQ, irp: *const IRP, insertcontext: *const core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_CSQ_IRP_CONTEXT {
    pub Type: u32,
    pub Irp: super::usb::PIRP,
    pub Csq: PIO_CSQ,
}
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type IO_CSQ_PEEK_NEXT_IRP = Option<unsafe extern "system" fn(csq: *const IO_CSQ, irp: *const IRP, peekcontext: *const core::ffi::c_void) -> super::usb::PIRP>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type IO_CSQ_RELEASE_LOCK = Option<unsafe extern "system" fn(csq: *const IO_CSQ, irql: super::ntdef::KIRQL)>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type IO_CSQ_REMOVE_IRP = Option<unsafe extern "system" fn(csq: *const IO_CSQ, irp: *const IRP)>;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "ntdef", feature = "usb"))]
#[derive(Clone, Copy)]
pub struct IO_DISCONNECT_INTERRUPT_PARAMETERS {
    pub Version: u32,
    pub ConnectionContext: IO_DISCONNECT_INTERRUPT_PARAMETERS_0,
}
#[cfg(all(feature = "basetsd", feature = "ntdef", feature = "usb"))]
impl Default for IO_DISCONNECT_INTERRUPT_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "ntdef", feature = "usb"))]
#[derive(Clone, Copy)]
pub union IO_DISCONNECT_INTERRUPT_PARAMETERS_0 {
    pub Generic: *mut core::ffi::c_void,
    pub InterruptObject: PKINTERRUPT,
    pub InterruptMessageTable: PIO_INTERRUPT_MESSAGE_INFO,
}
#[cfg(all(feature = "basetsd", feature = "ntdef", feature = "usb"))]
impl Default for IO_DISCONNECT_INTERRUPT_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IO_DISK_INCREMENT: u32 = 1;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type IO_DPC_ROUTINE = Option<unsafe extern "system" fn(dpc: *const KDPC, deviceobject: *const DEVICE_OBJECT, irp: *mut IRP, context: *const core::ffi::c_void)>;
#[repr(C)]
#[cfg(feature = "bcrypt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_ERROR_LOG_MESSAGE {
    pub Type: u16,
    pub Size: u16,
    pub DriverNameLength: u16,
    pub TimeStamp: i64,
    pub DriverNameOffset: u32,
    pub EntryData: IO_ERROR_LOG_PACKET,
}
#[cfg(target_arch = "x86")]
pub const IO_ERROR_LOG_MESSAGE_LENGTH: u32 = 256;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const IO_ERROR_LOG_MESSAGE_LENGTH: u32 = 344;
#[repr(C)]
#[cfg(feature = "bcrypt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IO_ERROR_LOG_PACKET {
    pub MajorFunctionCode: u8,
    pub RetryCount: u8,
    pub DumpDataSize: u16,
    pub NumberOfStrings: u16,
    pub StringOffset: u16,
    pub EventCategory: u16,
    pub ErrorCode: super::bcrypt::NTSTATUS,
    pub UniqueErrorValue: u32,
    pub FinalStatus: super::bcrypt::NTSTATUS,
    pub SequenceNumber: u32,
    pub IoControlCode: u32,
    pub DeviceOffset: i64,
    pub DumpData: [u32; 1],
}
#[cfg(feature = "bcrypt")]
impl Default for IO_ERROR_LOG_PACKET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IO_FORCE_ACCESS_CHECK: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "ntdef", feature = "usb"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IO_INTERRUPT_MESSAGE_INFO {
    pub UnifiedIrql: super::ntdef::KIRQL,
    pub MessageCount: u32,
    pub MessageInfo: [IO_INTERRUPT_MESSAGE_INFO_ENTRY; 1],
}
#[cfg(all(feature = "basetsd", feature = "ntdef", feature = "usb"))]
impl Default for IO_INTERRUPT_MESSAGE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "ntdef", feature = "usb"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_INTERRUPT_MESSAGE_INFO_ENTRY {
    pub MessageAddress: super::usb::PHYSICAL_ADDRESS,
    pub TargetProcessorSet: super::basetsd::KAFFINITY,
    pub InterruptObject: PKINTERRUPT,
    pub MessageData: u32,
    pub Vector: u32,
    pub Irql: super::ntdef::KIRQL,
    pub Mode: KINTERRUPT_MODE,
    pub Polarity: KINTERRUPT_POLARITY,
}
pub const IO_KEYBOARD_INCREMENT: u32 = 6;
pub const IO_MAILSLOT_INCREMENT: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_MINI_COMPLETION_PACKET_USER(pub u8);
pub type IO_MINI_PACKET_CALLBACK_ROUTINE = Option<unsafe extern "system" fn(minipacket: *const IO_MINI_COMPLETION_PACKET_USER, context: *const core::ffi::c_void)>;
pub const IO_MOUSE_INCREMENT: u32 = 6;
pub const IO_NAMED_PIPE_INCREMENT: u32 = 2;
pub const IO_NETWORK_INCREMENT: u32 = 2;
pub type IO_NOTIFICATION_EVENT_CATEGORY = i32;
pub const IO_NO_INCREMENT: u32 = 0;
pub const IO_NO_PARAMETER_CHECKING: u32 = 256;
pub type IO_PAGING_PRIORITY = i32;
pub const IO_PARALLEL_INCREMENT: u32 = 1;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type IO_PERSISTED_MEMORY_ENUMERATION_CALLBACK = Option<unsafe extern "system" fn(driverobject: *const DRIVER_OBJECT, physicaldeviceobject: *const DEVICE_OBJECT, physicaldeviceid: *const super::ntsecapi::UNICODE_STRING, datatag: *const u16, dataversion: *const u32, context: *const core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
pub type IO_PRIORITY_HINT = i32;
pub const IO_REMOUNT: u32 = 1;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct IO_REMOVE_LOCK {
    pub Common: IO_REMOVE_LOCK_COMMON_BLOCK,
}
#[cfg(feature = "winnt")]
impl Default for IO_REMOVE_LOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct IO_REMOVE_LOCK_COMMON_BLOCK {
    pub Removed: bool,
    pub Reserved: [bool; 3],
    pub IoCount: i32,
    pub RemoveEvent: KEVENT,
}
#[cfg(feature = "winnt")]
impl Default for IO_REMOVE_LOCK_COMMON_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IO_REMOVE_LOCK_DBG_BLOCK {
    pub Signature: i32,
    pub HighWatermark: u32,
    pub MaxLockedTicks: i64,
    pub AllocateTag: i32,
    pub LockList: super::winnt::LIST_ENTRY,
    pub Spin: super::winnt::KSPIN_LOCK,
    pub LowMemoryCount: i32,
    pub Reserved1: [u32; 4],
    pub Reserved2: *mut core::ffi::c_void,
    pub Blocks: PIO_REMOVE_LOCK_TRACKING_BLOCK,
}
#[cfg(feature = "winnt")]
impl Default for IO_REMOVE_LOCK_DBG_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IO_REPARSE: u32 = 0;
pub const IO_REPARSE_GLOBAL: u32 = 2;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "ntdef", feature = "usb"))]
#[derive(Clone, Copy)]
pub struct IO_REPORT_INTERRUPT_ACTIVE_STATE_PARAMETERS {
    pub Version: u32,
    pub ConnectionContext: IO_REPORT_INTERRUPT_ACTIVE_STATE_PARAMETERS_0,
}
#[cfg(all(feature = "basetsd", feature = "ntdef", feature = "usb"))]
impl Default for IO_REPORT_INTERRUPT_ACTIVE_STATE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "ntdef", feature = "usb"))]
#[derive(Clone, Copy)]
pub union IO_REPORT_INTERRUPT_ACTIVE_STATE_PARAMETERS_0 {
    pub Generic: *mut core::ffi::c_void,
    pub InterruptObject: PKINTERRUPT,
    pub InterruptMessageTable: PIO_INTERRUPT_MESSAGE_INFO,
}
#[cfg(all(feature = "basetsd", feature = "ntdef", feature = "usb"))]
impl Default for IO_REPORT_INTERRUPT_ACTIVE_STATE_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IO_RESOURCE_ALTERNATIVE: u32 = 8;
pub const IO_RESOURCE_DEFAULT: u32 = 2;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy)]
pub struct IO_RESOURCE_DESCRIPTOR {
    pub Option: u8,
    pub Type: u8,
    pub ShareDisposition: u8,
    pub Spare1: u8,
    pub Flags: u16,
    pub Spare2: u16,
    pub u: IO_RESOURCE_DESCRIPTOR_0,
}
#[cfg(all(feature = "basetsd", feature = "usb"))]
impl Default for IO_RESOURCE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy)]
pub union IO_RESOURCE_DESCRIPTOR_0 {
    pub Port: IO_RESOURCE_DESCRIPTOR_0_0,
    pub Memory: IO_RESOURCE_DESCRIPTOR_0_1,
    pub Interrupt: IO_RESOURCE_DESCRIPTOR_0_2,
    pub Dma: IO_RESOURCE_DESCRIPTOR_0_3,
    pub DmaV3: IO_RESOURCE_DESCRIPTOR_0_4,
    pub Generic: IO_RESOURCE_DESCRIPTOR_0_5,
    pub DevicePrivate: IO_RESOURCE_DESCRIPTOR_0_6,
    pub BusNumber: IO_RESOURCE_DESCRIPTOR_0_7,
    pub ConfigData: IO_RESOURCE_DESCRIPTOR_0_8,
    pub Memory40: IO_RESOURCE_DESCRIPTOR_0_9,
    pub Memory48: IO_RESOURCE_DESCRIPTOR_0_10,
    pub Memory64: IO_RESOURCE_DESCRIPTOR_0_11,
    pub Connection: IO_RESOURCE_DESCRIPTOR_0_12,
}
#[cfg(all(feature = "basetsd", feature = "usb"))]
impl Default for IO_RESOURCE_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_RESOURCE_DESCRIPTOR_0_0 {
    pub Length: u32,
    pub Alignment: u32,
    pub MinimumAddress: super::usb::PHYSICAL_ADDRESS,
    pub MaximumAddress: super::usb::PHYSICAL_ADDRESS,
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_RESOURCE_DESCRIPTOR_0_1 {
    pub Length: u32,
    pub Alignment: u32,
    pub MinimumAddress: super::usb::PHYSICAL_ADDRESS,
    pub MaximumAddress: super::usb::PHYSICAL_ADDRESS,
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_RESOURCE_DESCRIPTOR_0_10 {
    pub Length48: u32,
    pub Alignment48: u32,
    pub MinimumAddress: super::usb::PHYSICAL_ADDRESS,
    pub MaximumAddress: super::usb::PHYSICAL_ADDRESS,
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_RESOURCE_DESCRIPTOR_0_11 {
    pub Length64: u32,
    pub Alignment64: u32,
    pub MinimumAddress: super::usb::PHYSICAL_ADDRESS,
    pub MaximumAddress: super::usb::PHYSICAL_ADDRESS,
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_RESOURCE_DESCRIPTOR_0_12 {
    pub Class: u8,
    pub Type: u8,
    pub Reserved1: u8,
    pub Reserved2: u8,
    pub IdLowPart: u32,
    pub IdHighPart: u32,
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_RESOURCE_DESCRIPTOR_0_2 {
    pub MinimumVector: u32,
    pub MaximumVector: u32,
    pub AffinityPolicy: IRQ_DEVICE_POLICY,
    pub PriorityPolicy: IRQ_PRIORITY,
    pub TargetedProcessors: super::basetsd::KAFFINITY,
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_RESOURCE_DESCRIPTOR_0_3 {
    pub MinimumChannel: u32,
    pub MaximumChannel: u32,
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_RESOURCE_DESCRIPTOR_0_4 {
    pub RequestLine: u32,
    pub Reserved: u32,
    pub Channel: u32,
    pub TransferWidth: u32,
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_RESOURCE_DESCRIPTOR_0_5 {
    pub Length: u32,
    pub Alignment: u32,
    pub MinimumAddress: super::usb::PHYSICAL_ADDRESS,
    pub MaximumAddress: super::usb::PHYSICAL_ADDRESS,
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IO_RESOURCE_DESCRIPTOR_0_6 {
    pub Data: [u32; 3],
}
#[cfg(all(feature = "basetsd", feature = "usb"))]
impl Default for IO_RESOURCE_DESCRIPTOR_0_6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_RESOURCE_DESCRIPTOR_0_7 {
    pub Length: u32,
    pub MinBusNumber: u32,
    pub MaxBusNumber: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_RESOURCE_DESCRIPTOR_0_8 {
    pub Priority: u32,
    pub Reserved1: u32,
    pub Reserved2: u32,
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_RESOURCE_DESCRIPTOR_0_9 {
    pub Length40: u32,
    pub Alignment40: u32,
    pub MinimumAddress: super::usb::PHYSICAL_ADDRESS,
    pub MaximumAddress: super::usb::PHYSICAL_ADDRESS,
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy)]
pub struct IO_RESOURCE_LIST {
    pub Version: u16,
    pub Revision: u16,
    pub Count: u32,
    pub Descriptors: [IO_RESOURCE_DESCRIPTOR; 1],
}
#[cfg(all(feature = "basetsd", feature = "usb"))]
impl Default for IO_RESOURCE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IO_RESOURCE_PREFERRED: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "usb"))]
#[derive(Clone, Copy)]
pub struct IO_RESOURCE_REQUIREMENTS_LIST {
    pub ListSize: u32,
    pub InterfaceType: INTERFACE_TYPE,
    pub BusNumber: u32,
    pub SlotNumber: u32,
    pub Reserved: [u32; 3],
    pub AlternativeLists: u32,
    pub List: [IO_RESOURCE_LIST; 1],
}
#[cfg(all(feature = "basetsd", feature = "usb"))]
impl Default for IO_RESOURCE_REQUIREMENTS_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_SECURITY_CONTEXT {
    pub SecurityQos: super::winnt::PSECURITY_QUALITY_OF_SERVICE,
    pub AccessState: PACCESS_STATE,
    pub DesiredAccess: super::winnt::ACCESS_MASK,
    pub FullCreateOptions: u32,
}
pub const IO_SERIAL_INCREMENT: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_SESSION_CONNECT_INFO {
    pub SessionId: u32,
    pub LocalSession: bool,
}
pub type IO_SESSION_EVENT = i32;
pub const IO_SESSION_MAX_PAYLOAD_SIZE: u32 = 256;
#[cfg(feature = "bcrypt")]
pub type IO_SESSION_NOTIFICATION_FUNCTION = Option<unsafe extern "system" fn(sessionobject: *const core::ffi::c_void, ioobject: *const core::ffi::c_void, event: u32, context: *const core::ffi::c_void, notificationpayload: *const core::ffi::c_void, payloadlength: u32) -> super::bcrypt::NTSTATUS>;
pub type IO_SESSION_STATE = i32;
pub const IO_SESSION_STATE_ALL_EVENTS: u32 = 4294967295;
pub const IO_SESSION_STATE_CONNECT_EVENT: u32 = 4;
pub const IO_SESSION_STATE_CREATION_EVENT: u32 = 1;
pub const IO_SESSION_STATE_DISCONNECT_EVENT: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_SESSION_STATE_INFORMATION {
    pub SessionId: u32,
    pub SessionState: IO_SESSION_STATE,
    pub LocalSession: bool,
}
pub const IO_SESSION_STATE_LOGOFF_EVENT: u32 = 32;
pub const IO_SESSION_STATE_LOGON_EVENT: u32 = 16;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IO_SESSION_STATE_NOTIFICATION {
    pub Size: u32,
    pub Flags: u32,
    pub IoObject: *mut core::ffi::c_void,
    pub EventMask: u32,
    pub Context: *mut core::ffi::c_void,
}
impl Default for IO_SESSION_STATE_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IO_SESSION_STATE_TERMINATION_EVENT: u32 = 2;
pub const IO_SESSION_STATE_VALID_EVENT_MASK: u32 = 63;
pub const IO_SET_IRP_IO_ATTRIBUTION_FLAGS_MASK: u32 = 3;
pub const IO_SET_IRP_IO_ATTRIBUTION_FROM_PROCESS: u32 = 2;
pub const IO_SET_IRP_IO_ATTRIBUTION_FROM_THREAD: u32 = 1;
pub const IO_SHARE_ACCESS_NON_PRIMARY_STREAM: u32 = 128;
pub const IO_SHARE_ACCESS_NO_WRITE_PERMISSION: u32 = 2147483648;
pub const IO_SOUND_INCREMENT: u32 = 8;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub struct IO_STACK_LOCATION {
    pub MajorFunction: u8,
    pub MinorFunction: u8,
    pub Flags: u8,
    pub Control: u8,
    pub Parameters: IO_STACK_LOCATION_0,
    pub DeviceObject: PDEVICE_OBJECT,
    pub FileObject: PFILE_OBJECT,
    pub CompletionRoutine: PIO_COMPLETION_ROUTINE,
    pub Context: *mut core::ffi::c_void,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IO_STACK_LOCATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub union IO_STACK_LOCATION_0 {
    pub Create: IO_STACK_LOCATION_0_0,
    pub CreatePipe: IO_STACK_LOCATION_0_1,
    pub CreateMailslot: IO_STACK_LOCATION_0_2,
    pub Read: IO_STACK_LOCATION_0_3,
    pub Write: IO_STACK_LOCATION_0_4,
    pub QueryDirectory: IO_STACK_LOCATION_0_5,
    pub NotifyDirectory: IO_STACK_LOCATION_0_6,
    pub NotifyDirectoryEx: IO_STACK_LOCATION_0_7,
    pub QueryFile: IO_STACK_LOCATION_0_8,
    pub SetFile: IO_STACK_LOCATION_0_9,
    pub QueryEa: IO_STACK_LOCATION_0_10,
    pub SetEa: IO_STACK_LOCATION_0_11,
    pub QueryVolume: IO_STACK_LOCATION_0_12,
    pub SetVolume: IO_STACK_LOCATION_0_13,
    pub FileSystemControl: IO_STACK_LOCATION_0_14,
    pub LockControl: IO_STACK_LOCATION_0_15,
    pub DeviceIoControl: IO_STACK_LOCATION_0_16,
    pub QuerySecurity: IO_STACK_LOCATION_0_17,
    pub SetSecurity: IO_STACK_LOCATION_0_18,
    pub MountVolume: IO_STACK_LOCATION_0_19,
    pub VerifyVolume: IO_STACK_LOCATION_0_20,
    pub Scsi: IO_STACK_LOCATION_0_21,
    pub QueryQuota: IO_STACK_LOCATION_0_22,
    pub SetQuota: IO_STACK_LOCATION_0_23,
    pub QueryDeviceRelations: IO_STACK_LOCATION_0_24,
    pub QueryInterface: IO_STACK_LOCATION_0_25,
    pub DeviceCapabilities: IO_STACK_LOCATION_0_26,
    pub FilterResourceRequirements: IO_STACK_LOCATION_0_27,
    pub ReadWriteConfig: IO_STACK_LOCATION_0_28,
    pub SetLock: IO_STACK_LOCATION_0_29,
    pub QueryId: IO_STACK_LOCATION_0_30,
    pub QueryDeviceText: IO_STACK_LOCATION_0_31,
    pub UsageNotification: IO_STACK_LOCATION_0_32,
    pub WaitWake: IO_STACK_LOCATION_0_33,
    pub PowerSequence: IO_STACK_LOCATION_0_34,
    pub Power: IO_STACK_LOCATION_0_35,
    pub StartDevice: IO_STACK_LOCATION_0_36,
    pub WMI: IO_STACK_LOCATION_0_37,
    pub Others: IO_STACK_LOCATION_0_38,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IO_STACK_LOCATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_0 {
    pub SecurityContext: PIO_SECURITY_CONTEXT,
    pub Options: u32,
    pub FileAttributes: u16,
    pub ShareAccess: u16,
    pub EaLength: u32,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_1 {
    pub SecurityContext: PIO_SECURITY_CONTEXT,
    pub Options: u32,
    pub Reserved: u16,
    pub ShareAccess: u16,
    pub Parameters: PNAMED_PIPE_CREATE_PARAMETERS,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IO_STACK_LOCATION_0_10 {
    pub Length: u32,
    pub EaList: *mut core::ffi::c_void,
    pub EaListLength: u32,
    pub EaIndex: u32,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IO_STACK_LOCATION_0_10 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_11 {
    pub Length: u32,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_12 {
    pub Length: u32,
    pub FsInformationClass: FS_INFORMATION_CLASS,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_13 {
    pub Length: u32,
    pub FsInformationClass: FS_INFORMATION_CLASS,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IO_STACK_LOCATION_0_14 {
    pub OutputBufferLength: u32,
    pub InputBufferLength: u32,
    pub FsControlCode: u32,
    pub Type3InputBuffer: *mut core::ffi::c_void,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IO_STACK_LOCATION_0_14 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Default)]
pub struct IO_STACK_LOCATION_0_15 {
    pub Length: super::winnt::PLARGE_INTEGER,
    pub Key: u32,
    pub ByteOffset: i64,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IO_STACK_LOCATION_0_16 {
    pub OutputBufferLength: u32,
    pub InputBufferLength: u32,
    pub IoControlCode: u32,
    pub Type3InputBuffer: *mut core::ffi::c_void,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IO_STACK_LOCATION_0_16 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_17 {
    pub SecurityInformation: super::winnt::SECURITY_INFORMATION,
    pub Length: u32,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_18 {
    pub SecurityInformation: super::winnt::SECURITY_INFORMATION,
    pub SecurityDescriptor: super::winnt::PSECURITY_DESCRIPTOR,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_19 {
    pub Vpb: PVPB,
    pub DeviceObject: PDEVICE_OBJECT,
    pub OutputBufferLength: u32,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_2 {
    pub SecurityContext: PIO_SECURITY_CONTEXT,
    pub Options: u32,
    pub Reserved: u16,
    pub ShareAccess: u16,
    pub Parameters: PMAILSLOT_CREATE_PARAMETERS,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_20 {
    pub Vpb: PVPB,
    pub DeviceObject: PDEVICE_OBJECT,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IO_STACK_LOCATION_0_21 {
    pub Srb: *mut _SCSI_REQUEST_BLOCK,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IO_STACK_LOCATION_0_21 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_22 {
    pub Length: u32,
    pub StartSid: super::winnt::PSID,
    pub SidList: PFILE_GET_QUOTA_INFORMATION,
    pub SidListLength: u32,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_23 {
    pub Length: u32,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_24 {
    pub Type: DEVICE_RELATION_TYPE,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IO_STACK_LOCATION_0_25 {
    pub InterfaceType: *const windows_core::GUID,
    pub Size: u16,
    pub Version: u16,
    pub Interface: PINTERFACE,
    pub InterfaceSpecificData: *mut core::ffi::c_void,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IO_STACK_LOCATION_0_25 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_26 {
    pub Capabilities: PDEVICE_CAPABILITIES,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_27 {
    pub IoResourceRequirementList: PIO_RESOURCE_REQUIREMENTS_LIST,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IO_STACK_LOCATION_0_28 {
    pub WhichSpace: u32,
    pub Buffer: *mut core::ffi::c_void,
    pub Offset: u32,
    pub Length: u32,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IO_STACK_LOCATION_0_28 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_29 {
    pub Lock: bool,
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Default)]
pub struct IO_STACK_LOCATION_0_3 {
    pub Length: u32,
    pub Key: u32,
    pub ByteOffset: i64,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_30 {
    pub IdType: BUS_QUERY_ID_TYPE,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_31 {
    pub DeviceTextType: DEVICE_TEXT_TYPE,
    pub LocaleId: super::winnt::LCID,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IO_STACK_LOCATION_0_32 {
    pub InPath: bool,
    pub Reserved: [bool; 3],
    pub Type: DEVICE_USAGE_NOTIFICATION_TYPE,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IO_STACK_LOCATION_0_32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_33 {
    pub PowerState: super::winnt::SYSTEM_POWER_STATE,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_34 {
    pub PowerSequence: PPOWER_SEQUENCE,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub struct IO_STACK_LOCATION_0_35 {
    pub Anonymous: IO_STACK_LOCATION_0_35_0,
    pub Type: POWER_STATE_TYPE,
    pub State: POWER_STATE,
    pub ShutdownType: super::winnt::POWER_ACTION,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IO_STACK_LOCATION_0_35 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub union IO_STACK_LOCATION_0_35_0 {
    pub SystemContext: u32,
    pub SystemPowerStateContext: SYSTEM_POWER_STATE_CONTEXT,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IO_STACK_LOCATION_0_35_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_36 {
    pub AllocatedResources: PCM_RESOURCE_LIST,
    pub AllocatedResourcesTranslated: PCM_RESOURCE_LIST,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IO_STACK_LOCATION_0_37 {
    pub ProviderId: usize,
    pub DataPath: *mut core::ffi::c_void,
    pub BufferSize: u32,
    pub Buffer: *mut core::ffi::c_void,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IO_STACK_LOCATION_0_37 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IO_STACK_LOCATION_0_38 {
    pub Argument1: *mut core::ffi::c_void,
    pub Argument2: *mut core::ffi::c_void,
    pub Argument3: *mut core::ffi::c_void,
    pub Argument4: *mut core::ffi::c_void,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IO_STACK_LOCATION_0_38 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(4))]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Default)]
pub struct IO_STACK_LOCATION_0_4 {
    pub Length: u32,
    pub Key: u32,
    pub ByteOffset: i64,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_5 {
    pub Length: u32,
    pub FileName: super::ntsecapi::PUNICODE_STRING,
    pub FileInformationClass: super::winternl::FILE_INFORMATION_CLASS,
    pub FileIndex: u32,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_6 {
    pub Length: u32,
    pub CompletionFilter: u32,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_7 {
    pub Length: u32,
    pub CompletionFilter: u32,
    pub DirectoryNotifyInformationClass: DIRECTORY_NOTIFY_INFORMATION_CLASS,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_8 {
    pub Length: u32,
    pub FileInformationClass: super::winternl::FILE_INFORMATION_CLASS,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub struct IO_STACK_LOCATION_0_9 {
    pub Length: u32,
    pub FileInformationClass: super::winternl::FILE_INFORMATION_CLASS,
    pub FileObject: PFILE_OBJECT,
    pub Anonymous: IO_STACK_LOCATION_0_9_0,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IO_STACK_LOCATION_0_9 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub union IO_STACK_LOCATION_0_9_0 {
    pub Anonymous: IO_STACK_LOCATION_0_9_0_0,
    pub ClusterCount: u32,
    pub DeleteHandle: super::winnt::HANDLE,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IO_STACK_LOCATION_0_9_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_9_0_0 {
    pub ReplaceIfExists: bool,
    pub AdvanceOnly: bool,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub struct IO_STACK_LOCATION {
    pub MajorFunction: u8,
    pub MinorFunction: u8,
    pub Flags: u8,
    pub Control: u8,
    pub Parameters: IO_STACK_LOCATION_0,
    pub DeviceObject: PDEVICE_OBJECT,
    pub FileObject: PFILE_OBJECT,
    pub CompletionRoutine: PIO_COMPLETION_ROUTINE,
    pub Context: *mut core::ffi::c_void,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IO_STACK_LOCATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub union IO_STACK_LOCATION_0 {
    pub Create: IO_STACK_LOCATION_0_0,
    pub CreatePipe: IO_STACK_LOCATION_0_1,
    pub CreateMailslot: IO_STACK_LOCATION_0_2,
    pub Read: IO_STACK_LOCATION_0_3,
    pub Write: IO_STACK_LOCATION_0_4,
    pub QueryDirectory: IO_STACK_LOCATION_0_5,
    pub NotifyDirectory: IO_STACK_LOCATION_0_6,
    pub NotifyDirectoryEx: IO_STACK_LOCATION_0_7,
    pub QueryFile: IO_STACK_LOCATION_0_8,
    pub SetFile: IO_STACK_LOCATION_0_9,
    pub QueryEa: IO_STACK_LOCATION_0_10,
    pub SetEa: IO_STACK_LOCATION_0_11,
    pub QueryVolume: IO_STACK_LOCATION_0_12,
    pub SetVolume: IO_STACK_LOCATION_0_13,
    pub FileSystemControl: IO_STACK_LOCATION_0_14,
    pub LockControl: IO_STACK_LOCATION_0_15,
    pub DeviceIoControl: IO_STACK_LOCATION_0_16,
    pub QuerySecurity: IO_STACK_LOCATION_0_17,
    pub SetSecurity: IO_STACK_LOCATION_0_18,
    pub MountVolume: IO_STACK_LOCATION_0_19,
    pub VerifyVolume: IO_STACK_LOCATION_0_20,
    pub Scsi: IO_STACK_LOCATION_0_21,
    pub QueryQuota: IO_STACK_LOCATION_0_22,
    pub SetQuota: IO_STACK_LOCATION_0_23,
    pub QueryDeviceRelations: IO_STACK_LOCATION_0_24,
    pub QueryInterface: IO_STACK_LOCATION_0_25,
    pub DeviceCapabilities: IO_STACK_LOCATION_0_26,
    pub FilterResourceRequirements: IO_STACK_LOCATION_0_27,
    pub ReadWriteConfig: IO_STACK_LOCATION_0_28,
    pub SetLock: IO_STACK_LOCATION_0_29,
    pub QueryId: IO_STACK_LOCATION_0_30,
    pub QueryDeviceText: IO_STACK_LOCATION_0_31,
    pub UsageNotification: IO_STACK_LOCATION_0_32,
    pub WaitWake: IO_STACK_LOCATION_0_33,
    pub PowerSequence: IO_STACK_LOCATION_0_34,
    pub Power: IO_STACK_LOCATION_0_35,
    pub StartDevice: IO_STACK_LOCATION_0_36,
    pub WMI: IO_STACK_LOCATION_0_37,
    pub Others: IO_STACK_LOCATION_0_38,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IO_STACK_LOCATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_0 {
    pub SecurityContext: PIO_SECURITY_CONTEXT,
    pub Options: u32,
    pub FileAttributes: u16,
    pub ShareAccess: u16,
    pub EaLength: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_1 {
    pub SecurityContext: PIO_SECURITY_CONTEXT,
    pub Options: u32,
    pub Reserved: u16,
    pub ShareAccess: u16,
    pub Parameters: PNAMED_PIPE_CREATE_PARAMETERS,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IO_STACK_LOCATION_0_10 {
    pub Length: u32,
    pub EaList: *mut core::ffi::c_void,
    pub EaListLength: u32,
    pub EaIndex: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IO_STACK_LOCATION_0_10 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_11 {
    pub Length: u32,
}
#[repr(C, align(8))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_12 {
    pub Length: u32,
    pub FsInformationClass: FS_INFORMATION_CLASS,
}
#[repr(C, align(8))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_13 {
    pub Length: u32,
    pub FsInformationClass: FS_INFORMATION_CLASS,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IO_STACK_LOCATION_0_14 {
    pub OutputBufferLength: u32,
    pub InputBufferLength: u32,
    pub FsControlCode: u32,
    pub Type3InputBuffer: *mut core::ffi::c_void,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IO_STACK_LOCATION_0_14 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_15 {
    pub Length: super::winnt::PLARGE_INTEGER,
    pub Key: u32,
    pub ByteOffset: i64,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IO_STACK_LOCATION_0_16 {
    pub OutputBufferLength: u32,
    pub InputBufferLength: u32,
    pub IoControlCode: u32,
    pub Type3InputBuffer: *mut core::ffi::c_void,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IO_STACK_LOCATION_0_16 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, align(8))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_17 {
    pub SecurityInformation: super::winnt::SECURITY_INFORMATION,
    pub Length: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_18 {
    pub SecurityInformation: super::winnt::SECURITY_INFORMATION,
    pub SecurityDescriptor: super::winnt::PSECURITY_DESCRIPTOR,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_19 {
    pub Vpb: PVPB,
    pub DeviceObject: PDEVICE_OBJECT,
    pub OutputBufferLength: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_2 {
    pub SecurityContext: PIO_SECURITY_CONTEXT,
    pub Options: u32,
    pub Reserved: u16,
    pub ShareAccess: u16,
    pub Parameters: PMAILSLOT_CREATE_PARAMETERS,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_20 {
    pub Vpb: PVPB,
    pub DeviceObject: PDEVICE_OBJECT,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IO_STACK_LOCATION_0_21 {
    pub Srb: *mut _SCSI_REQUEST_BLOCK,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IO_STACK_LOCATION_0_21 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_22 {
    pub Length: u32,
    pub StartSid: super::winnt::PSID,
    pub SidList: PFILE_GET_QUOTA_INFORMATION,
    pub SidListLength: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_23 {
    pub Length: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_24 {
    pub Type: DEVICE_RELATION_TYPE,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IO_STACK_LOCATION_0_25 {
    pub InterfaceType: *const windows_core::GUID,
    pub Size: u16,
    pub Version: u16,
    pub Interface: PINTERFACE,
    pub InterfaceSpecificData: *mut core::ffi::c_void,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IO_STACK_LOCATION_0_25 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_26 {
    pub Capabilities: PDEVICE_CAPABILITIES,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_27 {
    pub IoResourceRequirementList: PIO_RESOURCE_REQUIREMENTS_LIST,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IO_STACK_LOCATION_0_28 {
    pub WhichSpace: u32,
    pub Buffer: *mut core::ffi::c_void,
    pub Offset: u32,
    pub Length: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IO_STACK_LOCATION_0_28 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_29 {
    pub Lock: bool,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_3 {
    pub Length: u32,
    pub Key: u32,
    pub Flags: u32,
    pub ByteOffset: i64,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_30 {
    pub IdType: BUS_QUERY_ID_TYPE,
}
#[repr(C, align(8))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_31 {
    pub DeviceTextType: DEVICE_TEXT_TYPE,
    pub LocaleId: super::winnt::LCID,
}
#[repr(C, align(8))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IO_STACK_LOCATION_0_32 {
    pub InPath: bool,
    pub Reserved: [bool; 3],
    pub Type: DEVICE_USAGE_NOTIFICATION_TYPE,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IO_STACK_LOCATION_0_32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_33 {
    pub PowerState: super::winnt::SYSTEM_POWER_STATE,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_34 {
    pub PowerSequence: PPOWER_SEQUENCE,
}
#[repr(C, align(8))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub struct IO_STACK_LOCATION_0_35 {
    pub Anonymous: IO_STACK_LOCATION_0_35_0,
    pub Type: POWER_STATE_TYPE,
    pub State: POWER_STATE,
    pub ShutdownType: super::winnt::POWER_ACTION,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IO_STACK_LOCATION_0_35 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub union IO_STACK_LOCATION_0_35_0 {
    pub SystemContext: u32,
    pub SystemPowerStateContext: SYSTEM_POWER_STATE_CONTEXT,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IO_STACK_LOCATION_0_35_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_36 {
    pub AllocatedResources: PCM_RESOURCE_LIST,
    pub AllocatedResourcesTranslated: PCM_RESOURCE_LIST,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IO_STACK_LOCATION_0_37 {
    pub ProviderId: usize,
    pub DataPath: *mut core::ffi::c_void,
    pub BufferSize: u32,
    pub Buffer: *mut core::ffi::c_void,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IO_STACK_LOCATION_0_37 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IO_STACK_LOCATION_0_38 {
    pub Argument1: *mut core::ffi::c_void,
    pub Argument2: *mut core::ffi::c_void,
    pub Argument3: *mut core::ffi::c_void,
    pub Argument4: *mut core::ffi::c_void,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IO_STACK_LOCATION_0_38 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_4 {
    pub Length: u32,
    pub Key: u32,
    pub Flags: u32,
    pub ByteOffset: i64,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_5 {
    pub Length: u32,
    pub FileName: super::ntsecapi::PUNICODE_STRING,
    pub FileInformationClass: super::winternl::FILE_INFORMATION_CLASS,
    pub FileIndex: u32,
}
#[repr(C, align(8))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_6 {
    pub Length: u32,
    pub CompletionFilter: u32,
}
#[repr(C, align(8))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_7 {
    pub Length: u32,
    pub CompletionFilter: u32,
    pub DirectoryNotifyInformationClass: DIRECTORY_NOTIFY_INFORMATION_CLASS,
}
#[repr(C, align(8))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_8 {
    pub Length: u32,
    pub FileInformationClass: super::winternl::FILE_INFORMATION_CLASS,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub struct IO_STACK_LOCATION_0_9 {
    pub Length: u32,
    pub FileInformationClass: super::winternl::FILE_INFORMATION_CLASS,
    pub FileObject: PFILE_OBJECT,
    pub Anonymous: IO_STACK_LOCATION_0_9_0,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IO_STACK_LOCATION_0_9 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub union IO_STACK_LOCATION_0_9_0 {
    pub Anonymous: IO_STACK_LOCATION_0_9_0_0,
    pub ClusterCount: u32,
    pub DeleteHandle: super::winnt::HANDLE,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IO_STACK_LOCATION_0_9_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STACK_LOCATION_0_9_0_0 {
    pub ReplaceIfExists: bool,
    pub AdvanceOnly: bool,
}
#[repr(C)]
#[cfg(feature = "bcrypt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IO_STATUS_BLOCK32 {
    pub Status: super::bcrypt::NTSTATUS,
    pub Information: u32,
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "bcrypt")]
#[derive(Clone, Copy)]
pub struct IO_STATUS_BLOCK64 {
    pub Anonymous: IO_STATUS_BLOCK64_0,
    pub Information: u64,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "bcrypt")]
impl Default for IO_STATUS_BLOCK64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "bcrypt")]
#[derive(Clone, Copy)]
pub union IO_STATUS_BLOCK64_0 {
    pub Status: super::bcrypt::NTSTATUS,
    pub Pointer: *mut core::ffi::c_void,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "bcrypt")]
impl Default for IO_STATUS_BLOCK64_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "bcrypt", feature = "winternl"))]
pub type IO_STATUS_BLOCK64 = super::winternl::IO_STATUS_BLOCK;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type IO_TIMER_ROUTINE = Option<unsafe extern "system" fn(deviceobject: *const DEVICE_OBJECT, context: *const core::ffi::c_void)>;
pub const IO_TYPE_ADAPTER: u32 = 1;
pub const IO_TYPE_CONTROLLER: u32 = 2;
pub const IO_TYPE_CSQ: u32 = 2;
pub const IO_TYPE_CSQ_EX: u32 = 3;
pub const IO_TYPE_CSQ_IRP_CONTEXT: u32 = 1;
pub const IO_TYPE_DEVICE: u32 = 3;
pub const IO_TYPE_DEVICE_OBJECT_EXTENSION: u32 = 13;
pub const IO_TYPE_DRIVER: u32 = 4;
pub const IO_TYPE_ERROR_LOG: u32 = 11;
pub const IO_TYPE_ERROR_MESSAGE: u32 = 12;
pub const IO_TYPE_FILE: u32 = 5;
pub const IO_TYPE_IORING: u32 = 14;
pub const IO_TYPE_IRP: u32 = 6;
pub const IO_TYPE_MASTER_ADAPTER: u32 = 7;
pub const IO_TYPE_OPEN_PACKET: u32 = 8;
pub const IO_TYPE_TIMER: u32 = 9;
pub const IO_TYPE_VPB: u32 = 10;
pub const IO_VIDEO_INCREMENT: u32 = 1;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type IO_WORKITEM_ROUTINE = Option<unsafe extern "system" fn(deviceobject: *const DEVICE_OBJECT, context: *const core::ffi::c_void)>;
pub type IO_WORKITEM_ROUTINE_EX = Option<unsafe extern "system" fn(ioobject: *const core::ffi::c_void, context: *const core::ffi::c_void, ioworkitem: *const _IO_WORKITEM)>;
#[cfg(target_arch = "x86")]
pub const IPI_LEVEL: u32 = 29;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const IPI_LEVEL: u32 = 14;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub struct IRP {
    pub Type: super::ntdef::CSHORT,
    pub Size: u16,
    pub MdlAddress: super::usb::PMDL,
    pub Flags: u32,
    pub AssociatedIrp: IRP_0,
    pub ThreadListEntry: super::winnt::LIST_ENTRY,
    pub IoStatus: super::winternl::IO_STATUS_BLOCK,
    pub RequestorMode: KPROCESSOR_MODE,
    pub PendingReturned: bool,
    pub StackCount: i8,
    pub CurrentLocation: i8,
    pub Cancel: bool,
    pub CancelIrql: super::ntdef::KIRQL,
    pub ApcEnvironment: super::winnt::CCHAR,
    pub AllocationFlags: u8,
    pub Anonymous: IRP_1,
    pub UserEvent: PKEVENT,
    pub Overlay: IRP_2,
    pub CancelRoutine: PDRIVER_CANCEL,
    pub UserBuffer: *mut core::ffi::c_void,
    pub Tail: IRP_3,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IRP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub union IRP_0 {
    pub MasterIrp: *mut IRP,
    pub IrpCount: i32,
    pub SystemBuffer: *mut core::ffi::c_void,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IRP_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub union IRP_1 {
    pub UserIosb: super::winternl::PIO_STATUS_BLOCK,
    pub IoRingContext: *mut core::ffi::c_void,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IRP_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub union IRP_2 {
    pub AsynchronousParameters: IRP_2_0,
    pub AllocationSize: i64,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IRP_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub struct IRP_2_0 {
    pub Anonymous: IRP_2_0_0,
    pub Anonymous2: IRP_2_0_1,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IRP_2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub union IRP_2_0_0 {
    pub UserApcRoutine: super::winternl::PIO_APC_ROUTINE,
    pub IssuingProcess: *mut core::ffi::c_void,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IRP_2_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub union IRP_2_0_1 {
    pub UserApcContext: *mut core::ffi::c_void,
    pub IoRing: *mut _IORING_OBJECT,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IRP_2_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub union IRP_3 {
    pub Overlay: IRP_3_0,
    pub Apc: KAPC,
    pub CompletionKey: *mut core::ffi::c_void,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IRP_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub struct IRP_3_0 {
    pub Anonymous: IRP_3_0_0,
    pub Thread: PETHREAD,
    pub AuxiliaryBuffer: super::winnt::PCHAR,
    pub Anonymous2: IRP_3_0_1,
    pub OriginalFileObject: PFILE_OBJECT,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IRP_3_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub union IRP_3_0_0 {
    pub DeviceQueueEntry: KDEVICE_QUEUE_ENTRY,
    pub Anonymous: IRP_3_0_0_0,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IRP_3_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IRP_3_0_0_0 {
    pub DriverContext: [*mut core::ffi::c_void; 4],
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IRP_3_0_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub struct IRP_3_0_1 {
    pub ListEntry: super::winnt::LIST_ENTRY,
    pub Anonymous: IRP_3_0_1_0,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IRP_3_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub union IRP_3_0_1_0 {
    pub CurrentStackLocation: *mut IO_STACK_LOCATION,
    pub PacketType: u32,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IRP_3_0_1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, align(16))]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub struct IRP {
    pub Type: super::ntdef::CSHORT,
    pub Size: u16,
    pub MdlAddress: super::usb::PMDL,
    pub Flags: u32,
    pub AssociatedIrp: IRP_0,
    pub ThreadListEntry: super::winnt::LIST_ENTRY,
    pub IoStatus: super::winternl::IO_STATUS_BLOCK,
    pub RequestorMode: KPROCESSOR_MODE,
    pub PendingReturned: bool,
    pub StackCount: i8,
    pub CurrentLocation: i8,
    pub Cancel: bool,
    pub CancelIrql: super::ntdef::KIRQL,
    pub ApcEnvironment: super::winnt::CCHAR,
    pub AllocationFlags: u8,
    pub Anonymous: IRP_1,
    pub UserEvent: PKEVENT,
    pub Overlay: IRP_2,
    pub CancelRoutine: PDRIVER_CANCEL,
    pub UserBuffer: *mut core::ffi::c_void,
    pub Tail: IRP_3,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IRP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub union IRP_0 {
    pub MasterIrp: *mut IRP,
    pub IrpCount: i32,
    pub SystemBuffer: *mut core::ffi::c_void,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IRP_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub union IRP_1 {
    pub UserIosb: super::winternl::PIO_STATUS_BLOCK,
    pub IoRingContext: *mut core::ffi::c_void,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IRP_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub union IRP_2 {
    pub AsynchronousParameters: IRP_2_0,
    pub AllocationSize: i64,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IRP_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub struct IRP_2_0 {
    pub Anonymous: IRP_2_0_0,
    pub Anonymous2: IRP_2_0_1,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IRP_2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub union IRP_2_0_0 {
    pub UserApcRoutine: super::winternl::PIO_APC_ROUTINE,
    pub IssuingProcess: *mut core::ffi::c_void,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IRP_2_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub union IRP_2_0_1 {
    pub UserApcContext: *mut core::ffi::c_void,
    pub IoRing: *mut _IORING_OBJECT,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IRP_2_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub union IRP_3 {
    pub Overlay: IRP_3_0,
    pub Apc: KAPC,
    pub CompletionKey: *mut core::ffi::c_void,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IRP_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub struct IRP_3_0 {
    pub Anonymous: IRP_3_0_0,
    pub Thread: PETHREAD,
    pub AuxiliaryBuffer: super::winnt::PCHAR,
    pub Anonymous2: IRP_3_0_1,
    pub OriginalFileObject: PFILE_OBJECT,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IRP_3_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub union IRP_3_0_0 {
    pub DeviceQueueEntry: KDEVICE_QUEUE_ENTRY,
    pub Anonymous: IRP_3_0_0_0,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IRP_3_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct IRP_3_0_0_0 {
    pub DriverContext: [*mut core::ffi::c_void; 4],
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IRP_3_0_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub struct IRP_3_0_1 {
    pub ListEntry: super::winnt::LIST_ENTRY,
    pub Anonymous: IRP_3_0_1_0,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IRP_3_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub union IRP_3_0_1_0 {
    pub CurrentStackLocation: *mut IO_STACK_LOCATION,
    pub PacketType: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for IRP_3_0_1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IRP_ALLOCATED_FIXED_SIZE: u32 = 4;
pub const IRP_ALLOCATED_MUST_SUCCEED: u32 = 2;
pub const IRP_ASSOCIATED_IRP: u32 = 8;
pub const IRP_BUFFERED_IO: u32 = 16;
pub const IRP_CLOSE_OPERATION: u32 = 1024;
pub const IRP_CREATE_OPERATION: u32 = 128;
pub const IRP_DEALLOCATE_BUFFER: u32 = 32;
pub const IRP_DEFER_IO_COMPLETION: u32 = 2048;
pub const IRP_HOLD_DEVICE_QUEUE: u32 = 8192;
pub const IRP_INPUT_OPERATION: u32 = 64;
pub const IRP_LOOKASIDE_ALLOCATION: u32 = 8;
pub const IRP_MJ_CLEANUP: u32 = 18;
pub const IRP_MJ_CLOSE: u32 = 2;
pub const IRP_MJ_CREATE: u32 = 0;
pub const IRP_MJ_CREATE_MAILSLOT: u32 = 19;
pub const IRP_MJ_CREATE_NAMED_PIPE: u32 = 1;
pub const IRP_MJ_DEVICE_CHANGE: u32 = 24;
pub const IRP_MJ_DEVICE_CONTROL: u32 = 14;
pub const IRP_MJ_DIRECTORY_CONTROL: u32 = 12;
pub const IRP_MJ_FILE_SYSTEM_CONTROL: u32 = 13;
pub const IRP_MJ_FLUSH_BUFFERS: u32 = 9;
pub const IRP_MJ_INTERNAL_DEVICE_CONTROL: u32 = 15;
pub const IRP_MJ_LOCK_CONTROL: u32 = 17;
pub const IRP_MJ_MAXIMUM_FUNCTION: u32 = 27;
pub const IRP_MJ_PNP: u32 = 27;
pub const IRP_MJ_PNP_POWER: u32 = 27;
pub const IRP_MJ_POWER: u32 = 22;
pub const IRP_MJ_QUERY_EA: u32 = 7;
pub const IRP_MJ_QUERY_INFORMATION: u32 = 5;
pub const IRP_MJ_QUERY_QUOTA: u32 = 25;
pub const IRP_MJ_QUERY_SECURITY: u32 = 20;
pub const IRP_MJ_QUERY_VOLUME_INFORMATION: u32 = 10;
pub const IRP_MJ_READ: u32 = 3;
pub const IRP_MJ_SCSI: u32 = 15;
pub const IRP_MJ_SET_EA: u32 = 8;
pub const IRP_MJ_SET_INFORMATION: u32 = 6;
pub const IRP_MJ_SET_QUOTA: u32 = 26;
pub const IRP_MJ_SET_SECURITY: u32 = 21;
pub const IRP_MJ_SET_VOLUME_INFORMATION: u32 = 11;
pub const IRP_MJ_SHUTDOWN: u32 = 16;
pub const IRP_MJ_SYSTEM_CONTROL: u32 = 23;
pub const IRP_MJ_WRITE: u32 = 4;
pub const IRP_MN_CANCEL_REMOVE_DEVICE: u32 = 3;
pub const IRP_MN_CANCEL_STOP_DEVICE: u32 = 6;
pub const IRP_MN_CHANGE_SINGLE_INSTANCE: u32 = 2;
pub const IRP_MN_CHANGE_SINGLE_ITEM: u32 = 3;
pub const IRP_MN_DEVICE_ENUMERATED: u32 = 25;
pub const IRP_MN_DEVICE_USAGE_NOTIFICATION: u32 = 22;
pub const IRP_MN_DISABLE_COLLECTION: u32 = 7;
pub const IRP_MN_DISABLE_EVENTS: u32 = 5;
pub const IRP_MN_EJECT: u32 = 17;
pub const IRP_MN_ENABLE_COLLECTION: u32 = 6;
pub const IRP_MN_ENABLE_EVENTS: u32 = 4;
pub const IRP_MN_EXECUTE_METHOD: u32 = 9;
pub const IRP_MN_FILTER_RESOURCE_REQUIREMENTS: u32 = 13;
pub const IRP_MN_POWER_SEQUENCE: u32 = 1;
pub const IRP_MN_QUERY_ALL_DATA: u32 = 0;
pub const IRP_MN_QUERY_BUS_INFORMATION: u32 = 21;
pub const IRP_MN_QUERY_CAPABILITIES: u32 = 9;
pub const IRP_MN_QUERY_DEVICE_RELATIONS: u32 = 7;
pub const IRP_MN_QUERY_DEVICE_TEXT: u32 = 12;
pub const IRP_MN_QUERY_ID: u32 = 19;
pub const IRP_MN_QUERY_INTERFACE: u32 = 8;
pub const IRP_MN_QUERY_PNP_DEVICE_STATE: u32 = 20;
pub const IRP_MN_QUERY_POWER: u32 = 3;
pub const IRP_MN_QUERY_REMOVE_DEVICE: u32 = 1;
pub const IRP_MN_QUERY_RESOURCES: u32 = 10;
pub const IRP_MN_QUERY_RESOURCE_REQUIREMENTS: u32 = 11;
pub const IRP_MN_QUERY_SINGLE_INSTANCE: u32 = 1;
pub const IRP_MN_QUERY_STOP_DEVICE: u32 = 5;
pub const IRP_MN_READ_CONFIG: u32 = 15;
pub const IRP_MN_REGINFO: u32 = 8;
pub const IRP_MN_REGINFO_EX: u32 = 11;
pub const IRP_MN_REMOVE_DEVICE: u32 = 2;
pub const IRP_MN_SCSI_CLASS: u32 = 1;
pub const IRP_MN_SET_LOCK: u32 = 18;
pub const IRP_MN_SET_POWER: u32 = 2;
pub const IRP_MN_START_DEVICE: u32 = 0;
pub const IRP_MN_STOP_DEVICE: u32 = 4;
pub const IRP_MN_SURPRISE_REMOVAL: u32 = 23;
pub const IRP_MN_WAIT_WAKE: u32 = 0;
pub const IRP_MN_WRITE_CONFIG: u32 = 16;
pub const IRP_MOUNT_COMPLETION: u32 = 2;
pub const IRP_NOCACHE: u32 = 1;
pub const IRP_OB_QUERY_NAME: u32 = 4096;
pub const IRP_PAGING_IO: u32 = 2;
pub const IRP_QUOTA_CHARGED: u32 = 1;
pub const IRP_READ_OPERATION: u32 = 256;
pub const IRP_SYNCHRONOUS_API: u32 = 4;
pub const IRP_SYNCHRONOUS_PAGING_IO: u32 = 64;
pub const IRP_UM_DRIVER_INITIATED_IO: u32 = 4194304;
pub const IRP_WRITE_OPERATION: u32 = 512;
pub type IRQ_DEVICE_POLICY = i32;
pub type IRQ_GROUP_POLICY = i32;
pub type IRQ_PRIORITY = i32;
pub const InACriticalArrayControl: NPEM_CONTROL_STANDARD_CONTROL_BIT = 8;
pub const InAFailedArrayControl: NPEM_CONTROL_STANDARD_CONTROL_BIT = 9;
pub const InitiateReset: NPEM_CONTROL_STANDARD_CONTROL_BIT = 1;
pub const InstallStateFailedInstall: DEVICE_INSTALL_STATE = 2;
pub const InstallStateFinishInstall: DEVICE_INSTALL_STATE = 3;
pub const InstallStateInstalled: DEVICE_INSTALL_STATE = 0;
pub const InstallStateNeedsReinstall: DEVICE_INSTALL_STATE = 1;
#[cfg(target_arch = "x86")]
pub const IntelCacheData: INTEL_CACHE_TYPE = 1;
#[cfg(target_arch = "x86")]
pub const IntelCacheInstruction: INTEL_CACHE_TYPE = 2;
#[cfg(target_arch = "x86")]
pub const IntelCacheNull: INTEL_CACHE_TYPE = 0;
#[cfg(target_arch = "x86")]
pub const IntelCacheRam: INTEL_CACHE_TYPE = 4;
#[cfg(target_arch = "x86")]
pub const IntelCacheTrace: INTEL_CACHE_TYPE = 5;
#[cfg(target_arch = "x86")]
pub const IntelCacheUnified: INTEL_CACHE_TYPE = 3;
pub const InterfaceTypeUndefined: INTERFACE_TYPE = -1;
pub const Internal: INTERFACE_TYPE = 0;
pub const InternalPowerBus: INTERFACE_TYPE = 13;
pub const InterruptActiveBoth: KINTERRUPT_POLARITY = 3;
pub const InterruptActiveBothTriggerHigh: KINTERRUPT_POLARITY = 4;
pub const InterruptActiveBothTriggerLow: KINTERRUPT_POLARITY = 3;
pub const InterruptActiveHigh: KINTERRUPT_POLARITY = 1;
pub const InterruptActiveLow: KINTERRUPT_POLARITY = 2;
pub const InterruptFallingEdge: KINTERRUPT_POLARITY = 2;
pub const InterruptPolarityUnknown: KINTERRUPT_POLARITY = 0;
pub const InterruptRisingEdge: KINTERRUPT_POLARITY = 1;
pub const InvalidDeviceTypeControl: NPEM_CONTROL_STANDARD_CONTROL_BIT = 10;
pub const IoMaxContainerInformationClass: IO_CONTAINER_INFORMATION_CLASS = 1;
pub const IoMaxContainerNotificationClass: IO_CONTAINER_NOTIFICATION_CLASS = 1;
pub const IoModifyAccess: LOCK_OPERATION = 2;
pub const IoPagingPriorityHigh: IO_PAGING_PRIORITY = 2;
pub const IoPagingPriorityInvalid: IO_PAGING_PRIORITY = 0;
pub const IoPagingPriorityNormal: IO_PAGING_PRIORITY = 1;
pub const IoPagingPriorityReserved1: IO_PAGING_PRIORITY = 3;
pub const IoPagingPriorityReserved2: IO_PAGING_PRIORITY = 4;
pub const IoPriorityCritical: IO_PRIORITY_HINT = 4;
pub const IoPriorityHigh: IO_PRIORITY_HINT = 3;
pub const IoPriorityLow: IO_PRIORITY_HINT = 1;
pub const IoPriorityNormal: IO_PRIORITY_HINT = 2;
pub const IoPriorityVeryLow: IO_PRIORITY_HINT = 0;
pub const IoReadAccess: LOCK_OPERATION = 0;
pub const IoSessionEventConnected: IO_SESSION_EVENT = 3;
pub const IoSessionEventCreated: IO_SESSION_EVENT = 1;
pub const IoSessionEventDisconnected: IO_SESSION_EVENT = 4;
pub const IoSessionEventIgnore: IO_SESSION_EVENT = 0;
pub const IoSessionEventLogoff: IO_SESSION_EVENT = 6;
pub const IoSessionEventLogon: IO_SESSION_EVENT = 5;
pub const IoSessionEventMax: IO_SESSION_EVENT = 7;
pub const IoSessionEventTerminated: IO_SESSION_EVENT = 2;
pub const IoSessionStateConnected: IO_SESSION_STATE = 3;
pub const IoSessionStateCreated: IO_SESSION_STATE = 1;
pub const IoSessionStateDisconnected: IO_SESSION_STATE = 4;
pub const IoSessionStateDisconnectedLoggedOn: IO_SESSION_STATE = 5;
pub const IoSessionStateInformation: IO_CONTAINER_INFORMATION_CLASS = 0;
pub const IoSessionStateInitialized: IO_SESSION_STATE = 2;
pub const IoSessionStateLoggedOff: IO_SESSION_STATE = 7;
pub const IoSessionStateLoggedOn: IO_SESSION_STATE = 6;
pub const IoSessionStateMax: IO_SESSION_STATE = 9;
pub const IoSessionStateNotification: IO_CONTAINER_NOTIFICATION_CLASS = 0;
pub const IoSessionStateTerminated: IO_SESSION_STATE = 8;
pub const IoWriteAccess: LOCK_OPERATION = 1;
pub const IommuDeviceCreationConfigTypeAcpi: IOMMU_DEVICE_CREATION_CONFIGURATION_TYPE = 1;
pub const IommuDeviceCreationConfigTypeDeviceId: IOMMU_DEVICE_CREATION_CONFIGURATION_TYPE = 2;
pub const IommuDeviceCreationConfigTypeMax: IOMMU_DEVICE_CREATION_CONFIGURATION_TYPE = 4;
pub const IommuDeviceCreationConfigTypeNone: IOMMU_DEVICE_CREATION_CONFIGURATION_TYPE = 0;
pub const IommuDeviceCreationConfigTypePasid: IOMMU_DEVICE_CREATION_CONFIGURATION_TYPE = 3;
pub const IommuDmaLogicalAllocatorBuddy: IOMMU_DMA_LOGICAL_ALLOCATOR_TYPE = 1;
pub const IommuDmaLogicalAllocatorMax: IOMMU_DMA_LOGICAL_ALLOCATOR_TYPE = 2;
pub const IommuDmaLogicalAllocatorNone: IOMMU_DMA_LOGICAL_ALLOCATOR_TYPE = 0;
pub const IrqPolicyAllCloseProcessors: IRQ_DEVICE_POLICY = 1;
pub const IrqPolicyAllProcessorsInMachine: IRQ_DEVICE_POLICY = 3;
pub const IrqPolicyAllProcessorsInMachineWhenSteered: IRQ_DEVICE_POLICY = 6;
pub const IrqPolicyMachineDefault: IRQ_DEVICE_POLICY = 0;
pub const IrqPolicyOneCloseProcessor: IRQ_DEVICE_POLICY = 2;
pub const IrqPolicySpecifiedProcessors: IRQ_DEVICE_POLICY = 4;
pub const IrqPolicySpreadMessagesAcrossAllProcessors: IRQ_DEVICE_POLICY = 5;
pub const IrqPriorityHigh: IRQ_PRIORITY = 3;
pub const IrqPriorityLow: IRQ_PRIORITY = 1;
pub const IrqPriorityNormal: IRQ_PRIORITY = 2;
pub const IrqPriorityUndefined: IRQ_PRIORITY = 0;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const IsNEC_98: u32 = 0;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const IsNotNEC_98: u32 = 1;
pub const Isa: INTERFACE_TYPE = 1;
#[cfg(target_arch = "x86")]
pub const KADDRESS_BASE: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KADDRESS_RANGE {
    pub Address: *mut core::ffi::c_void,
    pub Size: usize,
}
impl Default for KADDRESS_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KADDRESS_RANGE_DESCRIPTOR {
    pub AddressRanges: *const KADDRESS_RANGE,
    pub AddressRangeCount: usize,
}
impl Default for KADDRESS_RANGE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KAPC {
    pub Type: u8,
    pub AllFlags: u8,
    pub Size: u8,
    pub SpareByte1: u8,
    pub SpareLong0: u32,
    pub Thread: *mut _KTHREAD,
    pub ApcListEntry: super::winnt::LIST_ENTRY,
    pub Reserved: [*mut core::ffi::c_void; 3],
    pub NormalContext: *mut core::ffi::c_void,
    pub SystemArgument1: *mut core::ffi::c_void,
    pub SystemArgument2: *mut core::ffi::c_void,
    pub ApcStateIndex: super::winnt::CCHAR,
    pub ApcMode: KPROCESSOR_MODE,
    pub Inserted: bool,
}
#[cfg(feature = "winnt")]
impl Default for KAPC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
pub const KAPC_OFFSET_TO_APCSTATEINDEX: u32 = 44;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const KAPC_OFFSET_TO_APCSTATEINDEX: u32 = 80;
pub const KAPC_OFFSET_TO_SPARE_BYTE1: u32 = 3;
pub const KAPC_OFFSET_TO_SPARE_LONG: u32 = 4;
#[cfg(target_arch = "x86")]
pub const KAPC_OFFSET_TO_SYSTEMARGUMENT1: u32 = 36;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const KAPC_OFFSET_TO_SYSTEMARGUMENT1: u32 = 64;
#[cfg(target_arch = "x86")]
pub const KAPC_OFFSET_TO_SYSTEMARGUMENT2: u32 = 40;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const KAPC_OFFSET_TO_SYSTEMARGUMENT2: u32 = 72;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KBUGCHECK_ADD_PAGES {
    pub Context: *mut core::ffi::c_void,
    pub Flags: u32,
    pub BugCheckCode: u32,
    pub Address: usize,
    pub Count: usize,
}
impl Default for KBUGCHECK_ADD_PAGES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type KBUGCHECK_BUFFER_DUMP_STATE = i32;
pub type KBUGCHECK_CALLBACK_REASON = i32;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KBUGCHECK_CALLBACK_RECORD {
    pub Entry: super::winnt::LIST_ENTRY,
    pub CallbackRoutine: PKBUGCHECK_CALLBACK_ROUTINE,
    pub Buffer: *mut core::ffi::c_void,
    pub Length: u32,
    pub Component: super::minwindef::PUCHAR,
    pub Checksum: usize,
    pub State: u8,
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for KBUGCHECK_CALLBACK_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type KBUGCHECK_CALLBACK_ROUTINE = Option<unsafe extern "system" fn(buffer: *mut core::ffi::c_void, length: u32)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KBUGCHECK_DUMP_IO {
    pub Offset: u64,
    pub Buffer: *mut core::ffi::c_void,
    pub BufferLength: u32,
    pub Type: KBUGCHECK_DUMP_IO_TYPE,
}
impl Default for KBUGCHECK_DUMP_IO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type KBUGCHECK_DUMP_IO_TYPE = i32;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KBUGCHECK_REASON_CALLBACK_RECORD {
    pub Entry: super::winnt::LIST_ENTRY,
    pub CallbackRoutine: PKBUGCHECK_REASON_CALLBACK_ROUTINE,
    pub Component: super::minwindef::PUCHAR,
    pub Checksum: usize,
    pub Reason: KBUGCHECK_CALLBACK_REASON,
    pub State: u8,
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type KBUGCHECK_REASON_CALLBACK_ROUTINE = Option<unsafe extern "system" fn(reason: KBUGCHECK_CALLBACK_REASON, record: *const KBUGCHECK_REASON_CALLBACK_RECORD, reasonspecificdata: *mut core::ffi::c_void, reasonspecificdatalength: u32)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KBUGCHECK_REMOVE_PAGES {
    pub Context: *mut core::ffi::c_void,
    pub Flags: u32,
    pub BugCheckCode: u32,
    pub Address: usize,
    pub Count: usize,
}
impl Default for KBUGCHECK_REMOVE_PAGES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KBUGCHECK_SECONDARY_DUMP_DATA {
    pub InBuffer: *mut core::ffi::c_void,
    pub InBufferLength: u32,
    pub MaximumAllowed: u32,
    pub Guid: windows_core::GUID,
    pub OutBuffer: *mut core::ffi::c_void,
    pub OutBufferLength: u32,
}
impl Default for KBUGCHECK_SECONDARY_DUMP_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KBUGCHECK_SECONDARY_DUMP_DATA_EX {
    pub InBuffer: *mut core::ffi::c_void,
    pub InBufferLength: u32,
    pub MaximumAllowed: u32,
    pub Guid: windows_core::GUID,
    pub OutBuffer: *mut core::ffi::c_void,
    pub OutBufferLength: u32,
    pub Context: *mut core::ffi::c_void,
    pub Flags: u32,
    pub DumpType: u32,
    pub BugCheckCode: u32,
    pub BugCheckParameter1: usize,
    pub BugCheckParameter2: usize,
    pub BugCheckParameter3: usize,
    pub BugCheckParameter4: usize,
}
impl Default for KBUGCHECK_SECONDARY_DUMP_DATA_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KBUGCHECK_TRIAGE_DUMP_DATA {
    pub DataArray: PKTRIAGE_DUMP_DATA_ARRAY,
    pub Flags: u32,
    pub MaxVirtMemSize: u32,
    pub BugCheckCode: u32,
    pub BugCheckParameter1: usize,
    pub BugCheckParameter2: usize,
    pub BugCheckParameter3: usize,
    pub BugCheckParameter4: usize,
}
pub const KB_ADD_PAGES_FLAG_ADDITIONAL_RANGES_EXIST: u32 = 2147483648;
pub const KB_ADD_PAGES_FLAG_PHYSICAL_ADDRESS: u32 = 2;
pub const KB_ADD_PAGES_FLAG_VIRTUAL_ADDRESS: u32 = 1;
pub const KB_REMOVE_PAGES_FLAG_ADDITIONAL_RANGES_EXIST: u32 = 2147483648;
pub const KB_REMOVE_PAGES_FLAG_PHYSICAL_ADDRESS: u32 = 2;
pub const KB_REMOVE_PAGES_FLAG_VIRTUAL_ADDRESS: u32 = 1;
pub const KB_SECONDARY_DATA_FLAG_ADDITIONAL_DATA: u32 = 1;
pub const KB_SECONDARY_DATA_FLAG_NO_DEVICE_ACCESS: u32 = 2;
pub const KB_TRIAGE_DUMP_DATA_FLAG_BUGCHECK_ACTIVE: u32 = 1;
pub type KCRM_PROTOCOL_ID = windows_core::GUID;
#[cfg(all(feature = "basetsd", feature = "winnt"))]
pub type KDEFERRED_ROUTINE = Option<unsafe extern "system" fn(dpc: *const KDPC, deferredcontext: *const core::ffi::c_void, systemargument1: *const core::ffi::c_void, systemargument2: *const core::ffi::c_void)>;
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "ntdef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct KDEVICE_QUEUE {
    pub Type: super::ntdef::CSHORT,
    pub Size: super::ntdef::CSHORT,
    pub DeviceListHead: super::winnt::LIST_ENTRY,
    pub Lock: super::winnt::KSPIN_LOCK,
    pub Anonymous: KDEVICE_QUEUE_0,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "ntdef", feature = "winnt"))]
impl Default for KDEVICE_QUEUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "ntdef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union KDEVICE_QUEUE_0 {
    pub Busy: bool,
    pub Anonymous: KDEVICE_QUEUE_0_0,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "ntdef", feature = "winnt"))]
impl Default for KDEVICE_QUEUE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "ntdef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KDEVICE_QUEUE_0_0 {
    pub _bitfield: i64,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
#[cfg(all(feature = "ntdef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KDEVICE_QUEUE {
    pub Type: super::ntdef::CSHORT,
    pub Size: super::ntdef::CSHORT,
    pub DeviceListHead: super::winnt::LIST_ENTRY,
    pub Lock: super::winnt::KSPIN_LOCK,
    pub Busy: bool,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KDEVICE_QUEUE_ENTRY {
    pub DeviceListEntry: super::winnt::LIST_ENTRY,
    pub SortKey: u32,
    pub Inserted: bool,
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct KDPC {
    pub Anonymous: KDPC_0,
    pub DpcListEntry: super::winnt::SINGLE_LIST_ENTRY,
    pub ProcessorHistory: super::basetsd::KAFFINITY,
    pub DeferredRoutine: PKDEFERRED_ROUTINE,
    pub DeferredContext: *mut core::ffi::c_void,
    pub SystemArgument1: *mut core::ffi::c_void,
    pub SystemArgument2: *mut core::ffi::c_void,
    pub DpcData: *mut core::ffi::c_void,
}
#[cfg(all(feature = "basetsd", feature = "winnt"))]
impl Default for KDPC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union KDPC_0 {
    pub TargetInfoAsUlong: u32,
    pub Anonymous: KDPC_0_0,
}
#[cfg(all(feature = "basetsd", feature = "winnt"))]
impl Default for KDPC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KDPC_0_0 {
    pub Type: u8,
    pub Importance: u8,
    pub Number: u16,
}
pub type KDPC_IMPORTANCE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KDPC_WATCHDOG_INFORMATION {
    pub DpcTimeLimit: u32,
    pub DpcTimeCount: u32,
    pub DpcWatchdogLimit: u32,
    pub DpcWatchdogCount: u32,
    pub Reserved: u32,
}
pub type KD_OPTION = i32;
pub const KD_OPTION_SET_BLOCK_ENABLE: KD_OPTION = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KENLISTMENT(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KERNEL_SOFT_RESTART_NOTIFICATION {
    pub Version: u16,
    pub Size: u16,
    pub Event: windows_core::GUID,
}
pub const KERNEL_SOFT_RESTART_NOTIFICATION_VERSION: u32 = 1;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct KEVENT {
    pub Header: DISPATCHER_HEADER,
}
#[cfg(feature = "winnt")]
impl Default for KEVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KEY_BASIC_INFORMATION {
    pub LastWriteTime: i64,
    pub TitleIndex: u32,
    pub NameLength: u32,
    pub Name: [u16; 1],
}
impl Default for KEY_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KEY_CONTROL_FLAGS_INFORMATION {
    pub ControlFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KEY_FULL_INFORMATION {
    pub LastWriteTime: i64,
    pub TitleIndex: u32,
    pub ClassOffset: u32,
    pub ClassLength: u32,
    pub SubKeys: u32,
    pub MaxNameLen: u32,
    pub MaxClassLen: u32,
    pub Values: u32,
    pub MaxValueNameLen: u32,
    pub MaxValueDataLen: u32,
    pub Class: [u16; 1],
}
impl Default for KEY_FULL_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type KEY_INFORMATION_CLASS = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KEY_NODE_INFORMATION {
    pub LastWriteTime: i64,
    pub TitleIndex: u32,
    pub ClassOffset: u32,
    pub ClassLength: u32,
    pub NameLength: u32,
    pub Name: [u16; 1],
}
impl Default for KEY_NODE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KEY_SET_VIRTUALIZATION_INFORMATION {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KEY_TRUST_INFORMATION {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KEY_VALUE_BASIC_INFORMATION {
    pub TitleIndex: u32,
    pub Type: u32,
    pub NameLength: u32,
    pub Name: [u16; 1],
}
impl Default for KEY_VALUE_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KEY_VALUE_FULL_INFORMATION {
    pub TitleIndex: u32,
    pub Type: u32,
    pub DataOffset: u32,
    pub DataLength: u32,
    pub NameLength: u32,
    pub Name: [u16; 1],
}
impl Default for KEY_VALUE_FULL_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type KEY_VALUE_INFORMATION_CLASS = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KEY_VALUE_LAYER_INFORMATION {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KEY_VALUE_PARTIAL_INFORMATION {
    pub TitleIndex: u32,
    pub Type: u32,
    pub DataLength: u32,
    pub Data: [u8; 1],
}
impl Default for KEY_VALUE_PARTIAL_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KEY_VALUE_PARTIAL_INFORMATION_ALIGN64 {
    pub Type: u32,
    pub DataLength: u32,
    pub Data: [u8; 1],
}
impl Default for KEY_VALUE_PARTIAL_INFORMATION_ALIGN64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KEY_WOW64_FLAGS_INFORMATION {
    pub UserFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KEY_WRITE_TIME_INFORMATION {
    pub LastWriteTime: i64,
}
pub const KE_MAX_TRIAGE_DUMP_DATA_MEMORY_SIZE: u32 = 33554432;
pub const KE_PROCESSOR_CHANGE_ADD_EXISTING: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KE_PROCESSOR_CHANGE_NOTIFY_CONTEXT {
    pub State: KE_PROCESSOR_CHANGE_NOTIFY_STATE,
    pub NtNumber: u32,
    pub Status: super::bcrypt::NTSTATUS,
    pub ProcNumber: super::winnt::PROCESSOR_NUMBER,
}
pub type KE_PROCESSOR_CHANGE_NOTIFY_STATE = i32;
pub const KE_SRCU_FLAG_READ_AT_DISPATCH: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KE_SRCU_LOCK {
    pub Placeholder: [usize; 2],
}
impl Default for KE_SRCU_LOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KFLOATING_SAVE {
    pub ControlWord: u32,
    pub StatusWord: u32,
    pub ErrorOffset: u32,
    pub ErrorSelector: u32,
    pub DataOffset: u32,
    pub DataSelector: u32,
    pub Spare0: u32,
    pub Spare1: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KFLOATING_SAVE {
    pub Dummy: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct KGATE {
    pub Header: DISPATCHER_HEADER,
}
#[cfg(feature = "winnt")]
impl Default for KGATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
pub type KGUARDED_MUTEX = FAST_MUTEX;
pub type KINTERRUPT_MODE = i32;
pub type KINTERRUPT_POLARITY = i32;
pub type KIPI_BROADCAST_WORKER = Option<unsafe extern "system" fn(argument: usize) -> usize>;
#[repr(C)]
#[cfg(all(feature = "ntdef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KLOCK_QUEUE_HANDLE {
    pub LockQueue: KSPIN_LOCK_QUEUE,
    pub OldIrql: super::ntdef::KIRQL,
}
pub type KMESSAGE_SERVICE_ROUTINE = Option<unsafe extern "system" fn(interrupt: *const _KINTERRUPT, servicecontext: *const core::ffi::c_void, messageid: u32) -> bool>;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct KMUTANT {
    pub Header: DISPATCHER_HEADER,
    pub MutantListEntry: super::winnt::LIST_ENTRY,
    pub OwnerThread: *mut _KTHREAD,
    pub Anonymous: KMUTANT_0,
    pub ApcDisable: u8,
}
#[cfg(feature = "winnt")]
impl Default for KMUTANT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union KMUTANT_0 {
    pub MutantFlags: u8,
    pub Anonymous: KMUTANT_0_0,
}
#[cfg(feature = "winnt")]
impl Default for KMUTANT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KMUTANT_0_0 {
    pub _bitfield: u8,
}
#[cfg(feature = "winnt")]
pub type KMUTEX = KMUTANT;
#[cfg(feature = "winnt")]
pub type KPROCESSOR_MODE = super::winnt::CCHAR;
pub type KPROFILE_SOURCE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KRESOURCEMANAGER(pub u8);
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct KSEMAPHORE {
    pub Header: DISPATCHER_HEADER,
    pub Limit: i32,
}
#[cfg(feature = "winnt")]
impl Default for KSEMAPHORE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type KSERVICE_ROUTINE = Option<unsafe extern "system" fn(interrupt: *const _KINTERRUPT, servicecontext: *const core::ffi::c_void) -> bool>;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KSPIN_LOCK_QUEUE {
    pub Next: *mut Self,
    pub Lock: super::winnt::PKSPIN_LOCK,
}
#[cfg(feature = "winnt")]
impl Default for KSPIN_LOCK_QUEUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct KSPIN_LOCK_QUEUE_NUMBER(pub u64);
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
pub type KSPIN_LOCK_QUEUE_NUMBER = i32;
pub type KSTART_ROUTINE = Option<unsafe extern "system" fn(startcontext: *const core::ffi::c_void)>;
pub type KSYNCHRONIZE_ROUTINE = Option<unsafe extern "system" fn(synchronizecontext: *const core::ffi::c_void) -> bool>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KSYSTEM_TIME {
    pub LowPart: u32,
    pub High1Time: i32,
    pub High2Time: i32,
}
pub type KT2_SET_PARAMETERS = EXT_SET_PARAMETERS;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct KTIMER {
    pub Header: DISPATCHER_HEADER,
    pub DueTime: u64,
    pub TimerListEntry: super::winnt::LIST_ENTRY,
    pub Dpc: *mut KDPC,
    pub Period: u32,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "winnt"))]
impl Default for KTIMER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct KTIMER {
    pub Header: DISPATCHER_HEADER,
    pub DueTime: u64,
    pub TimerListEntry: super::winnt::LIST_ENTRY,
    pub Dpc: *mut KDPC,
    pub Processor: u16,
    pub TimerType: u8,
    pub TimerDifObjTracking: i8,
    pub Period: u32,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "winnt"))]
impl Default for KTIMER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KTM(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct KTRANSACTION(pub u8);
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KTRIAGE_DUMP_DATA_ARRAY {
    pub List: super::winnt::LIST_ENTRY,
    pub NumBlocksUsed: u32,
    pub NumBlocksTotal: u32,
    pub DataSize: u32,
    pub MaxDataSize: u32,
    pub ComponentNameBufferLength: u32,
    pub ComponentName: super::minwindef::PUCHAR,
    pub Blocks: [KADDRESS_RANGE; 1],
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for KTRIAGE_DUMP_DATA_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "ntifs", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct KWAIT_BLOCK {
    pub WaitListEntry: super::winnt::LIST_ENTRY,
    pub WaitType: u8,
    pub BlockState: u8,
    pub WaitKey: u16,
    pub Anonymous: KWAIT_BLOCK_0,
    pub Object: *mut core::ffi::c_void,
    pub SparePtr: *mut core::ffi::c_void,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "ntifs", feature = "winnt"))]
impl Default for KWAIT_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "ntifs", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union KWAIT_BLOCK_0 {
    pub Thread: *mut _KTHREAD,
    pub NotificationQueue: *mut super::ntifs::KQUEUE,
    pub Dpc: *mut KDPC,
}
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "basetsd", feature = "ntifs", feature = "winnt"))]
impl Default for KWAIT_BLOCK_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "ntifs", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct KWAIT_BLOCK {
    pub WaitListEntry: super::winnt::LIST_ENTRY,
    pub WaitType: u8,
    pub BlockState: u8,
    pub WaitKey: u16,
    pub SpareLong: i32,
    pub Anonymous: KWAIT_BLOCK_0,
    pub Object: *mut core::ffi::c_void,
    pub SparePtr: *mut core::ffi::c_void,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "ntifs", feature = "winnt"))]
impl Default for KWAIT_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "ntifs", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union KWAIT_BLOCK_0 {
    pub Thread: *mut _KTHREAD,
    pub NotificationQueue: *mut super::ntifs::KQUEUE,
    pub Dpc: *mut KDPC,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "ntifs", feature = "winnt"))]
impl Default for KWAIT_BLOCK_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KWAIT_CHAIN {
    pub Head: *mut core::ffi::c_void,
}
impl Default for KWAIT_CHAIN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type KWAIT_REASON = i32;
pub const KbCallbackAddPages: KBUGCHECK_CALLBACK_REASON = 4;
pub const KbCallbackDumpIo: KBUGCHECK_CALLBACK_REASON = 3;
pub const KbCallbackInvalid: KBUGCHECK_CALLBACK_REASON = 0;
pub const KbCallbackRemovePages: KBUGCHECK_CALLBACK_REASON = 6;
pub const KbCallbackReserved1: KBUGCHECK_CALLBACK_REASON = 1;
pub const KbCallbackReserved2: KBUGCHECK_CALLBACK_REASON = 8;
pub const KbCallbackReserved3: KBUGCHECK_CALLBACK_REASON = 9;
pub const KbCallbackSecondaryDumpData: KBUGCHECK_CALLBACK_REASON = 2;
pub const KbCallbackSecondaryMultiPartDumpData: KBUGCHECK_CALLBACK_REASON = 5;
pub const KbCallbackTriageDumpData: KBUGCHECK_CALLBACK_REASON = 7;
pub const KbDumpIoBody: KBUGCHECK_DUMP_IO_TYPE = 2;
pub const KbDumpIoComplete: KBUGCHECK_DUMP_IO_TYPE = 4;
pub const KbDumpIoHeader: KBUGCHECK_DUMP_IO_TYPE = 1;
pub const KbDumpIoInvalid: KBUGCHECK_DUMP_IO_TYPE = 0;
pub const KbDumpIoSecondaryData: KBUGCHECK_DUMP_IO_TYPE = 3;
pub const KeProcessorAddCompleteNotify: KE_PROCESSOR_CHANGE_NOTIFY_STATE = 1;
pub const KeProcessorAddFailureNotify: KE_PROCESSOR_CHANGE_NOTIFY_STATE = 2;
pub const KeProcessorAddStartNotify: KE_PROCESSOR_CHANGE_NOTIFY_STATE = 0;
pub const KeepObject: IO_ALLOCATION_ACTION = 1;
pub const KeyBasicInformation: KEY_INFORMATION_CLASS = 0;
pub const KeyCachedInformation: KEY_INFORMATION_CLASS = 4;
pub const KeyFlagsInformation: KEY_INFORMATION_CLASS = 5;
pub const KeyFullInformation: KEY_INFORMATION_CLASS = 2;
pub const KeyHandleTagsInformation: KEY_INFORMATION_CLASS = 7;
pub const KeyLayerInformation: KEY_INFORMATION_CLASS = 9;
pub const KeyNameInformation: KEY_INFORMATION_CLASS = 3;
pub const KeyNodeInformation: KEY_INFORMATION_CLASS = 1;
pub const KeyTrustInformation: KEY_INFORMATION_CLASS = 8;
pub const KeyValueBasicInformation: KEY_VALUE_INFORMATION_CLASS = 0;
pub const KeyValueFullInformation: KEY_VALUE_INFORMATION_CLASS = 1;
pub const KeyValueFullInformationAlign64: KEY_VALUE_INFORMATION_CLASS = 3;
pub const KeyValueLayerInformation: KEY_VALUE_INFORMATION_CLASS = 5;
pub const KeyValuePartialInformation: KEY_VALUE_INFORMATION_CLASS = 2;
pub const KeyValuePartialInformationAlign64: KEY_VALUE_INFORMATION_CLASS = 4;
pub const KeyVirtualizationInformation: KEY_INFORMATION_CLASS = 6;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LEGACY_BUS_INFORMATION {
    pub BusTypeGuid: windows_core::GUID,
    pub LegacyBusType: INTERFACE_TYPE,
    pub BusNumber: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LINK_SHARE_ACCESS {
    pub OpenCount: u32,
    pub Deleters: u32,
    pub SharedDelete: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LOADER_PARTITION_INFORMATION_EX {
    pub PartitionStyle: u32,
    pub PartitionNumber: u32,
    pub Anonymous: LOADER_PARTITION_INFORMATION_EX_0,
    pub Flags: u32,
}
impl Default for LOADER_PARTITION_INFORMATION_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union LOADER_PARTITION_INFORMATION_EX_0 {
    pub Signature: u32,
    pub DeviceId: windows_core::GUID,
}
impl Default for LOADER_PARTITION_INFORMATION_EX_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type LOCK_OPERATION = i32;
pub const LOCK_QUEUE_HALTED: u32 = 4;
pub const LOCK_QUEUE_HALTED_BIT: u32 = 2;
pub const LOCK_QUEUE_OWNER: u32 = 2;
pub const LOCK_QUEUE_OWNER_BIT: u32 = 1;
#[cfg(target_arch = "x86")]
pub const LOCK_QUEUE_VALID_FLAGS: u32 = 3;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const LOCK_QUEUE_VALID_FLAGS: u32 = 7;
pub const LOCK_QUEUE_WAIT: u32 = 1;
pub const LOCK_QUEUE_WAIT_BIT: u32 = 0;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type LOG_FILE_OBJECT = FILE_OBJECT;
pub const LOG_POLICY_OVERWRITE: u32 = 1;
pub const LOG_POLICY_PERSIST: u32 = 2;
pub const LONGLONG_MASK: u32 = 7;
pub const LONG_2ND_MOST_SIGNIFICANT_BIT: u32 = 2;
pub const LONG_3RD_MOST_SIGNIFICANT_BIT: u32 = 1;
pub const LONG_LEAST_SIGNIFICANT_BIT: u32 = 0;
pub const LONG_MASK: u32 = 3;
pub const LONG_MOST_SIGNIFICANT_BIT: u32 = 3;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct LOOKASIDE_LIST_EX {
    pub L: GENERAL_LOOKASIDE_POOL,
}
#[cfg(feature = "winnt")]
impl Default for LOOKASIDE_LIST_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(target_arch = "x86")]
pub const LOOKASIDE_MINIMUM_BLOCK_SIZE: u32 = 4;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const LOOKASIDE_MINIMUM_BLOCK_SIZE: u32 = 8;
pub const LOWBYTE_MASK: u32 = 255;
pub const LOW_LEVEL: u32 = 0;
pub const LOW_PRIORITY: u32 = 0;
pub const LOW_REALTIME_PRIORITY: u32 = 16;
pub const LastDStateTransitionD3cold: D3COLD_LAST_TRANSITION_STATUS = 2;
pub const LastDStateTransitionD3hot: D3COLD_LAST_TRANSITION_STATUS = 1;
pub const LastDStateTransitionStatusUnknown: D3COLD_LAST_TRANSITION_STATUS = 0;
pub const LastDrvRtFlag: DRIVER_RUNTIME_INIT_FLAGS = 2;
pub const Latched: KINTERRUPT_MODE = 1;
pub const LevelSensitive: KINTERRUPT_MODE = 0;
pub const LocateControl: NPEM_CONTROL_STANDARD_CONTROL_BIT = 3;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const LockQueueAfdWorkQueueLock: u32 = 13;
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
pub const LockQueueAfdWorkQueueLock: KSPIN_LOCK_QUEUE_NUMBER = 13 as _;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const LockQueueBcbLock: u32 = 14;
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
pub const LockQueueBcbLock: KSPIN_LOCK_QUEUE_NUMBER = 14 as _;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const LockQueueIoCancelLock: u32 = 7;
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
pub const LockQueueIoCancelLock: KSPIN_LOCK_QUEUE_NUMBER = 7 as _;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const LockQueueIoCompletionLock: u32 = 11;
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
pub const LockQueueIoCompletionLock: KSPIN_LOCK_QUEUE_NUMBER = 11 as _;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const LockQueueIoDatabaseLock: u32 = 10;
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
pub const LockQueueIoDatabaseLock: KSPIN_LOCK_QUEUE_NUMBER = 10 as _;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const LockQueueIoVpbLock: u32 = 9;
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
pub const LockQueueIoVpbLock: KSPIN_LOCK_QUEUE_NUMBER = 9 as _;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const LockQueueMasterLock: u32 = 5;
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
pub const LockQueueMasterLock: KSPIN_LOCK_QUEUE_NUMBER = 5 as _;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const LockQueueMaximumLock: u32 = 17;
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
pub const LockQueueMaximumLock: KSPIN_LOCK_QUEUE_NUMBER = 17 as _;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const LockQueueNonPagedPoolLock: u32 = 6;
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
pub const LockQueueNonPagedPoolLock: KSPIN_LOCK_QUEUE_NUMBER = 6 as _;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const LockQueueNtfsStructLock: u32 = 12;
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
pub const LockQueueNtfsStructLock: KSPIN_LOCK_QUEUE_NUMBER = 12 as _;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const LockQueueUnusedSpare0: u32 = 0;
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
pub const LockQueueUnusedSpare0: KSPIN_LOCK_QUEUE_NUMBER = 0 as _;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const LockQueueUnusedSpare1: u32 = 1;
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
pub const LockQueueUnusedSpare1: KSPIN_LOCK_QUEUE_NUMBER = 1 as _;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const LockQueueUnusedSpare15: u32 = 15;
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
pub const LockQueueUnusedSpare15: KSPIN_LOCK_QUEUE_NUMBER = 15 as _;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const LockQueueUnusedSpare16: u32 = 16;
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
pub const LockQueueUnusedSpare16: KSPIN_LOCK_QUEUE_NUMBER = 16 as _;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const LockQueueUnusedSpare2: u32 = 2;
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
pub const LockQueueUnusedSpare2: KSPIN_LOCK_QUEUE_NUMBER = 2 as _;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const LockQueueUnusedSpare3: u32 = 3;
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
pub const LockQueueUnusedSpare3: KSPIN_LOCK_QUEUE_NUMBER = 3 as _;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const LockQueueUnusedSpare8: u32 = 8;
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
pub const LockQueueUnusedSpare8: KSPIN_LOCK_QUEUE_NUMBER = 8 as _;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const LockQueueVacbLock: u32 = 4;
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
pub const LockQueueVacbLock: KSPIN_LOCK_QUEUE_NUMBER = 4 as _;
pub const LoggerEventsLoggedClass: TRACE_INFORMATION_CLASS = 10;
pub const LoggerEventsLostClass: TRACE_INFORMATION_CLASS = 8;
pub const LowImportance: KDPC_IMPORTANCE = 0;
pub const LowPagePriority: MM_PAGE_PRIORITY = 0;
pub const LowPoolPriority: EX_POOL_PRIORITY = 0;
pub const LowPoolPrioritySpecialPoolOverrun: EX_POOL_PRIORITY = 8;
pub const LowPoolPrioritySpecialPoolUnderrun: EX_POOL_PRIORITY = 9;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MAILSLOT_CREATE_PARAMETERS {
    pub MailslotQuota: u32,
    pub MaximumMessageSize: u32,
    pub ReadTimeout: i64,
    pub TimeoutSpecified: bool,
}
pub const MAXIMUM_FILENAME_LENGTH: u32 = 256;
pub const MAXIMUM_PRIORITY: u32 = 32;
#[cfg(target_arch = "aarch64")]
pub const MAX_EVENT_COUNTERS: u32 = 31;
#[repr(C)]
#[cfg(feature = "ntdef")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MDL {
    pub Next: *mut Self,
    pub Size: super::ntdef::CSHORT,
    pub MdlFlags: super::ntdef::CSHORT,
    pub Process: *mut _EPROCESS,
    pub MappedSystemVa: *mut core::ffi::c_void,
    pub StartVa: *mut core::ffi::c_void,
    pub ByteCount: u32,
    pub ByteOffset: u32,
}
#[cfg(feature = "ntdef")]
impl Default for MDL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MDL_ALLOCATED_FIXED_SIZE: u32 = 8;
pub const MDL_ALLOCATED_MUST_SUCCEED: u32 = 16384;
pub const MDL_DESCRIBES_AWE: u32 = 1024;
pub const MDL_FREE_EXTRA_PTES: u32 = 512;
pub const MDL_INTERNAL: u32 = 32768;
pub const MDL_IO_PAGE_READ: u32 = 64;
pub const MDL_IO_SPACE: u32 = 2048;
pub const MDL_LOCKED_PAGE_TABLES: u32 = 256;
pub const MDL_MAPPED_TO_SYSTEM_VA: u32 = 1;
pub const MDL_MAPPING_CAN_FAIL: u32 = 8192;
pub const MDL_NETWORK_HEADER: u32 = 4096;
pub const MDL_PAGES_LOCKED: u32 = 2;
pub const MDL_PAGE_CONTENTS_INVARIANT: u32 = 16384;
pub const MDL_PARENT_MAPPED_SYSTEM_VA: u32 = 256;
pub const MDL_PARTIAL: u32 = 16;
pub const MDL_PARTIAL_HAS_BEEN_MAPPED: u32 = 32;
pub const MDL_SOURCE_IS_NONPAGED_POOL: u32 = 4;
pub const MDL_WRITE_OPERATION: u32 = 128;
pub type MEMORY_CACHING_TYPE = i32;
pub type MEMORY_CACHING_TYPE_ORIG = i32;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MEMORY_PARTITION_DEDICATED_MEMORY_OPEN_INFORMATION {
    pub DedicatedMemoryTypeId: u64,
    pub HandleAttributes: u32,
    pub DesiredAccess: super::winnt::ACCESS_MASK,
    pub DedicatedMemoryPartitionHandle: super::winnt::HANDLE,
}
pub const MM_ALLOCATE_AND_HOT_REMOVE: u32 = 256;
pub const MM_ALLOCATE_CONTIGUOUS_MEMORY_FAST_ONLY: u32 = 1;
pub const MM_ALLOCATE_FAST_LARGE_PAGES: u32 = 64;
pub const MM_ALLOCATE_FROM_LOCAL_NODE_ONLY: u32 = 2;
pub const MM_ALLOCATE_FULLY_REQUIRED: u32 = 4;
pub const MM_ALLOCATE_NO_WAIT: u32 = 8;
pub const MM_ALLOCATE_PREFER_CONTIGUOUS: u32 = 16;
pub const MM_ALLOCATE_REQUIRE_CONTIGUOUS_CHUNKS: u32 = 32;
pub const MM_ALLOCATE_TRIM_IF_NECESSARY: u32 = 128;
pub const MM_ANY_NODE_OK: u32 = 2147483648;
pub const MM_DONT_ZERO_ALLOCATION: u32 = 1;
pub const MM_DUMP_MAP_CACHED: u32 = 1;
pub const MM_DUMP_MAP_INVALIDATE: u32 = 2;
pub const MM_FREE_MDL_PAGES_ZERO: u32 = 1;
pub const MM_MAPPING_ADDRESS_DIVISIBLE: u32 = 1;
pub const MM_MAXIMUM_DISK_IO_SIZE: u32 = 65536;
pub type MM_MDL_PAGE_CONTENTS_STATE = i32;
pub type MM_MDL_ROUTINE = Option<unsafe extern "system" fn(drivercontext: *const core::ffi::c_void, mappedva: *const core::ffi::c_void)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct MM_NODE_NUMBER_ONE_BASED(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct MM_NODE_NUMBER_ZERO_BASED(pub u32);
pub type MM_PAGE_PRIORITY = i32;
pub const MM_PERMANENT_ADDRESS_IS_IO_SPACE: u32 = 1;
#[repr(C)]
#[cfg(feature = "usb")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MM_PHYSICAL_ADDRESS_LIST {
    pub PhysicalAddress: super::usb::PHYSICAL_ADDRESS,
    pub NumberOfBytes: usize,
}
pub const MM_PROTECT_DRIVER_SECTION_ALLOW_UNLOAD: u32 = 1;
pub const MM_PROTECT_DRIVER_SECTION_VALID_FLAGS: u32 = 1;
pub type MM_SYSTEMSIZE = i32;
pub const MPIBus: INTERFACE_TYPE = 10;
pub const MPSABus: INTERFACE_TYPE = 11;
pub const MapPhysicalAddressTypeContiguousRange: IOMMU_MAP_PHYSICAL_ADDRESS_TYPE = 1;
pub const MapPhysicalAddressTypeMax: IOMMU_MAP_PHYSICAL_ADDRESS_TYPE = 3;
pub const MapPhysicalAddressTypeMdl: IOMMU_MAP_PHYSICAL_ADDRESS_TYPE = 0;
pub const MapPhysicalAddressTypePfn: IOMMU_MAP_PHYSICAL_ADDRESS_TYPE = 2;
pub const MaxFaultType: FAULT_INFORMATION_ARM64_TYPE = 7;
pub const MaxIoPriorityTypes: IO_PRIORITY_HINT = 5;
pub const MaxKeyInfoClass: KEY_INFORMATION_CLASS = 10;
pub const MaxKeyValueInfoClass: KEY_VALUE_INFORMATION_CLASS = 6;
pub const MaxPoolType: POOL_TYPE = 7;
pub const MaxRegNtNotifyClass: REG_NOTIFY_CLASS = 51;
pub const MaxTraceInformationClass: TRACE_INFORMATION_CLASS = 16;
pub const MaximumDmaSpeed: DMA_SPEED = 5;
pub const MaximumDmaWidth: DMA_WIDTH = 5;
pub const MaximumInterfaceType: INTERFACE_TYPE = 18;
pub const MaximumWaitReason: KWAIT_REASON = 43;
pub const MaximumWorkQueue: WORK_QUEUE_TYPE = 7;
pub const MdlMappingNoExecute: u32 = 1073741824;
pub const MdlMappingNoWrite: u32 = 2147483648;
pub const MdlMappingWithGuardPtes: u32 = 536870912;
pub const MediumHighImportance: KDPC_IMPORTANCE = 3;
pub const MediumImportance: KDPC_IMPORTANCE = 1;
pub const MicroChannel: INTERFACE_TYPE = 3;
pub const MmCached: MEMORY_CACHING_TYPE = 1;
pub const MmFrameBufferCached: MEMORY_CACHING_TYPE_ORIG = 2;
pub const MmHardwareCoherentCached: MEMORY_CACHING_TYPE = 3;
pub const MmLargeSystem: MM_SYSTEMSIZE = 2;
pub const MmMaximumCacheType: MEMORY_CACHING_TYPE = 6;
pub const MmMdlPageContentsDynamic: MM_MDL_PAGE_CONTENTS_STATE = 0;
pub const MmMdlPageContentsInvariant: MM_MDL_PAGE_CONTENTS_STATE = 1;
pub const MmMdlPageContentsQuery: MM_MDL_PAGE_CONTENTS_STATE = 2;
pub const MmMediumSystem: MM_SYSTEMSIZE = 1;
pub const MmNonCached: MEMORY_CACHING_TYPE = 0;
pub const MmNonCachedUnordered: MEMORY_CACHING_TYPE = 4;
pub const MmNotMapped: MEMORY_CACHING_TYPE = -1;
pub const MmSmallSystem: MM_SYSTEMSIZE = 0;
pub const MmUSWCCached: MEMORY_CACHING_TYPE = 5;
pub const MmWriteCombined: MEMORY_CACHING_TYPE = 2;
pub const ModifyAccess: IO_ACCESS_TYPE = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NAMED_PIPE_CREATE_PARAMETERS {
    pub NamedPipeType: u32,
    pub ReadMode: u32,
    pub CompletionMode: u32,
    pub MaximumInstances: u32,
    pub InboundQuota: u32,
    pub OutboundQuota: u32,
    pub DefaultTimeout: i64,
    pub TimeoutSpecified: bool,
}
pub const NEC98x86: ALTERNATIVE_ARCHITECTURE_TYPE = 1;
pub type NMI_CALLBACK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, handled: bool) -> bool>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct NODE_REQUIREMENT(pub u32);
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct NPAGED_LOOKASIDE_LIST {
    pub L: GENERAL_LOOKASIDE,
    pub Lock__ObsoleteButDoNotDelete: super::winnt::KSPIN_LOCK,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
impl Default for NPAGED_LOOKASIDE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct NPAGED_LOOKASIDE_LIST {
    pub L: GENERAL_LOOKASIDE,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
impl Default for NPAGED_LOOKASIDE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NPEM_CAPABILITY_STANDARD {
    pub Anonymous: NPEM_CAPABILITY_STANDARD_0,
    pub AsULONG: u32,
}
impl Default for NPEM_CAPABILITY_STANDARD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NPEM_CAPABILITY_STANDARD_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "bcrypt")]
pub type NPEM_CONTROL_ENABLE_DISABLE = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, enablenpem: bool) -> super::bcrypt::NTSTATUS>;
#[repr(C)]
#[cfg(feature = "bcrypt")]
#[derive(Clone, Copy, Debug)]
pub struct NPEM_CONTROL_INTERFACE {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
    pub SetNpemSupportState: PNPEM_CONTROL_ENABLE_DISABLE,
    pub QueryStandardCapabilities: PNPEM_CONTROL_QUERY_STANDARD_CAPABILITIES,
    pub SetStandardControl: PNPEM_CONTROL_SET_STANDARD_CONTROL,
    pub QueryNpemControl: PNPEM_CONTROL_QUERY_CONTROL,
}
#[cfg(feature = "bcrypt")]
impl Default for NPEM_CONTROL_INTERFACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NPEM_CONTROL_INTERFACE_CURRENT_VERSION: u32 = 2;
pub const NPEM_CONTROL_INTERFACE_VERSION1: u32 = 1;
pub const NPEM_CONTROL_INTERFACE_VERSION2: u32 = 2;
pub type NPEM_CONTROL_QUERY_CONTROL = Option<unsafe extern "system" fn(context: *const core::ffi::c_void) -> u32>;
#[cfg(feature = "bcrypt")]
pub type NPEM_CONTROL_QUERY_STANDARD_CAPABILITIES = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, standardcapabilities: *mut NPEM_CAPABILITY_STANDARD) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "bcrypt")]
pub type NPEM_CONTROL_SET_STANDARD_CONTROL = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, standardcontrol: NPEM_CONTROL_STANDARD_CONTROL_BIT, set: bool) -> super::bcrypt::NTSTATUS>;
pub type NPEM_CONTROL_STANDARD_CONTROL_BIT = i32;
#[cfg(feature = "winnt")]
pub type NTFS_DEREF_EXPORTED_SECURITY_DESCRIPTOR = Option<unsafe extern "system" fn(vcb: *const core::ffi::c_void, securitydescriptor: super::winnt::PSECURITY_DESCRIPTOR)>;
pub const NonPagedPool: POOL_TYPE = 0;
pub const NonPagedPoolBase: POOL_TYPE = 0;
pub const NonPagedPoolBaseCacheAligned: POOL_TYPE = 4;
pub const NonPagedPoolBaseCacheAlignedMustS: POOL_TYPE = 6;
pub const NonPagedPoolBaseMustSucceed: POOL_TYPE = 2;
pub const NonPagedPoolCacheAligned: POOL_TYPE = 4;
pub const NonPagedPoolCacheAlignedMustS: POOL_TYPE = 6;
pub const NonPagedPoolCacheAlignedMustSSession: POOL_TYPE = 38;
pub const NonPagedPoolCacheAlignedSession: POOL_TYPE = 36;
pub const NonPagedPoolExecute: POOL_TYPE = 0;
pub const NonPagedPoolMustSucceed: POOL_TYPE = 2;
pub const NonPagedPoolMustSucceedSession: POOL_TYPE = 34;
pub const NonPagedPoolNx: POOL_TYPE = 512;
pub const NonPagedPoolNxCacheAligned: POOL_TYPE = 516;
pub const NonPagedPoolSession: POOL_TYPE = 32;
pub const NonPagedPoolSessionNx: POOL_TYPE = 544;
pub const NormalPagePriority: MM_PAGE_PRIORITY = 16;
pub const NormalPoolPriority: EX_POOL_PRIORITY = 16;
pub const NormalPoolPrioritySpecialPoolOverrun: EX_POOL_PRIORITY = 24;
pub const NormalPoolPrioritySpecialPoolUnderrun: EX_POOL_PRIORITY = 25;
pub const NormalWorkQueue: WORK_QUEUE_TYPE = 3;
pub const NuBus: INTERFACE_TYPE = 7;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OBJECT_HANDLE_INFORMATION {
    pub HandleAttributes: u32,
    pub GrantedAccess: super::winnt::ACCESS_MASK,
}
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OBJECT_NAME_INFORMATION {
    pub Name: super::ntsecapi::UNICODE_STRING,
}
pub const OBJECT_TYPE_ALL_ACCESS: u32 = 983041;
pub const OBJECT_TYPE_CREATE: u32 = 1;
pub const OBJ_NAME_PATH_SEPARATOR: u32 = 92;
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OB_CALLBACK_REGISTRATION {
    pub Version: u16,
    pub OperationRegistrationCount: u16,
    pub Altitude: super::ntsecapi::UNICODE_STRING,
    pub RegistrationContext: *mut core::ffi::c_void,
    pub OperationRegistration: *mut OB_OPERATION_REGISTRATION,
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
impl Default for OB_CALLBACK_REGISTRATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OB_FLT_REGISTRATION_VERSION: u32 = 256;
pub const OB_FLT_REGISTRATION_VERSION_0100: u32 = 256;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct OB_OPERATION(pub u32);
pub const OB_OPERATION_HANDLE_CREATE: u32 = 1;
pub const OB_OPERATION_HANDLE_DUPLICATE: u32 = 2;
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[derive(Clone, Copy, Debug)]
pub struct OB_OPERATION_REGISTRATION {
    pub ObjectType: *mut POBJECT_TYPE,
    pub Operations: OB_OPERATION,
    pub PreOperation: POB_PRE_OPERATION_CALLBACK,
    pub PostOperation: POB_POST_OPERATION_CALLBACK,
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
impl Default for OB_OPERATION_REGISTRATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OB_POST_CREATE_HANDLE_INFORMATION {
    pub GrantedAccess: super::winnt::ACCESS_MASK,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OB_POST_DUPLICATE_HANDLE_INFORMATION {
    pub GrantedAccess: super::winnt::ACCESS_MASK,
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct OB_POST_OPERATION_INFORMATION {
    pub Operation: OB_OPERATION,
    pub Anonymous: OB_POST_OPERATION_INFORMATION_0,
    pub Object: *mut core::ffi::c_void,
    pub ObjectType: POBJECT_TYPE,
    pub CallContext: *mut core::ffi::c_void,
    pub ReturnStatus: super::bcrypt::NTSTATUS,
    pub Parameters: POB_POST_OPERATION_PARAMETERS,
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
impl Default for OB_POST_OPERATION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union OB_POST_OPERATION_INFORMATION_0 {
    pub Flags: u32,
    pub Anonymous: OB_POST_OPERATION_INFORMATION_0_0,
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
impl Default for OB_POST_OPERATION_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OB_POST_OPERATION_INFORMATION_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union OB_POST_OPERATION_PARAMETERS {
    pub CreateHandleInformation: OB_POST_CREATE_HANDLE_INFORMATION,
    pub DuplicateHandleInformation: OB_POST_DUPLICATE_HANDLE_INFORMATION,
}
#[cfg(feature = "winnt")]
impl Default for OB_POST_OPERATION_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type OB_PREOP_CALLBACK_STATUS = i32;
pub const OB_PREOP_SUCCESS: OB_PREOP_CALLBACK_STATUS = 0;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OB_PRE_CREATE_HANDLE_INFORMATION {
    pub DesiredAccess: super::winnt::ACCESS_MASK,
    pub OriginalDesiredAccess: super::winnt::ACCESS_MASK,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OB_PRE_DUPLICATE_HANDLE_INFORMATION {
    pub DesiredAccess: super::winnt::ACCESS_MASK,
    pub OriginalDesiredAccess: super::winnt::ACCESS_MASK,
    pub SourceProcess: *mut core::ffi::c_void,
    pub TargetProcess: *mut core::ffi::c_void,
}
#[cfg(feature = "winnt")]
impl Default for OB_PRE_DUPLICATE_HANDLE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct OB_PRE_OPERATION_INFORMATION {
    pub Operation: OB_OPERATION,
    pub Anonymous: OB_PRE_OPERATION_INFORMATION_0,
    pub Object: *mut core::ffi::c_void,
    pub ObjectType: POBJECT_TYPE,
    pub CallContext: *mut core::ffi::c_void,
    pub Parameters: POB_PRE_OPERATION_PARAMETERS,
}
#[cfg(feature = "winnt")]
impl Default for OB_PRE_OPERATION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union OB_PRE_OPERATION_INFORMATION_0 {
    pub Flags: u32,
    pub Anonymous: OB_PRE_OPERATION_INFORMATION_0_0,
}
#[cfg(feature = "winnt")]
impl Default for OB_PRE_OPERATION_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OB_PRE_OPERATION_INFORMATION_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union OB_PRE_OPERATION_PARAMETERS {
    pub CreateHandleInformation: OB_PRE_CREATE_HANDLE_INFORMATION,
    pub DuplicateHandleInformation: OB_PRE_DUPLICATE_HANDLE_INFORMATION,
}
#[cfg(feature = "winnt")]
impl Default for OB_PRE_OPERATION_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OWNER_ENTRY {
    pub OwnerThread: ERESOURCE_THREAD,
    pub Anonymous: OWNER_ENTRY_0,
}
impl Default for OWNER_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union OWNER_ENTRY_0 {
    pub EntryCounts: OWNER_ENTRY_COUNTS,
    pub TableSize: u32,
}
impl Default for OWNER_ENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union OWNER_ENTRY_COUNTS {
    pub Value: u32,
    pub Anonymous: OWNER_ENTRY_COUNTS_0,
}
impl Default for OWNER_ENTRY_COUNTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct OWNER_ENTRY_COUNTS_0 {
    pub _bitfield: u32,
}
pub const OkControl: NPEM_CONTROL_STANDARD_CONTROL_BIT = 2;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
pub type PACCESS_STATE = *mut ACCESS_STATE;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PACPI_INTERFACE_STANDARD = *mut ACPI_INTERFACE_STANDARD;
#[cfg(feature = "bcrypt")]
pub type PACPI_INTERFACE_STANDARD2 = *mut ACPI_INTERFACE_STANDARD2;
#[cfg(target_arch = "x86")]
#[cfg(feature = "ntddscsi")]
pub type PADAPTER_OBJECT = *mut super::ntddscsi::_ADAPTER_OBJECT;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PADAPTER_OBJECT = *mut DMA_ADAPTER;
pub type PAFFINITY_TOKEN = *mut _AFFINITY_TOKEN;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct PAGED_LOOKASIDE_LIST {
    pub L: GENERAL_LOOKASIDE,
    pub Lock__ObsoleteButDoNotDelete: FAST_MUTEX,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
impl Default for PAGED_LOOKASIDE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct PAGED_LOOKASIDE_LIST {
    pub L: GENERAL_LOOKASIDE,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
impl Default for PAGED_LOOKASIDE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PAGE_ENCLAVE_NO_CHANGE: u32 = 536870912;
pub const PAGE_SHIFT: u32 = 12;
pub const PAGE_SIZE: u32 = 4096;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PALLOCATE_ADAPTER_CHANNEL = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, deviceobject: *const DEVICE_OBJECT, numberofmapregisters: u32, executionroutine: PDRIVER_CONTROL, context: *const core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PALLOCATE_ADAPTER_CHANNEL_EX = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, deviceobject: *const DEVICE_OBJECT, dmatransfercontext: *const core::ffi::c_void, numberofmapregisters: u32, flags: u32, executionroutine: PDRIVER_CONTROL, executioncontext: *const core::ffi::c_void, mapregisterbase: *mut *mut core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PALLOCATE_COMMON_BUFFER = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, length: u32, logicaladdress: *mut i64, cacheenabled: bool) -> *mut core::ffi::c_void>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PALLOCATE_COMMON_BUFFER_EX = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, maximumaddress: *const i64, length: u32, logicaladdress: *mut i64, cacheenabled: bool, preferrednode: NODE_REQUIREMENT) -> *mut core::ffi::c_void>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PALLOCATE_COMMON_BUFFER_VECTOR = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, lowaddress: super::usb::PHYSICAL_ADDRESS, highaddress: super::usb::PHYSICAL_ADDRESS, cachetype: MEMORY_CACHING_TYPE, idealnode: NODE_REQUIREMENT, flags: u32, numberofelements: u32, sizeofelements: u64, vectorout: *mut PDMA_COMMON_BUFFER_VECTOR) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PALLOCATE_COMMON_BUFFER_WITH_BOUNDS = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, minimumaddress: *const i64, maximumaddress: *const i64, length: u32, flags: u32, cachetype: *const MEMORY_CACHING_TYPE, preferrednode: NODE_REQUIREMENT, logicaladdress: *mut i64) -> *mut core::ffi::c_void>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PALLOCATE_DOMAIN_COMMON_BUFFER = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, domainhandle: super::winnt::HANDLE, maximumaddress: *const i64, length: u32, flags: u32, cachetype: *const MEMORY_CACHING_TYPE, preferrednode: NODE_REQUIREMENT, logicaladdress: *mut i64, virtualaddress: *mut *mut core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
pub type PALLOCATE_FUNCTION = *mut ALLOCATE_FUNCTION;
#[cfg(feature = "winnt")]
pub type PALLOCATE_FUNCTION_EX = *mut ALLOCATE_FUNCTION_EX;
#[cfg(target_arch = "x86")]
pub type PAMD_L1_CACHE_INFO = *mut AMD_L1_CACHE_INFO;
#[cfg(target_arch = "x86")]
pub type PAMD_L2_CACHE_INFO = *mut AMD_L2_CACHE_INFO;
#[cfg(target_arch = "x86")]
pub type PAMD_L3_CACHE_INFO = *mut AMD_L3_CACHE_INFO;
#[cfg(target_arch = "aarch64")]
pub type PARM64_IDCODE = *mut ARM64_IDCODE;
pub type PARTITION_INFORMATION_CLASS = i32;
#[cfg(all(feature = "lsalookup", feature = "ntddk", feature = "ntsecapi"))]
pub type PBDCB_IMAGE_INFORMATION = *mut super::ntddk::BDCB_IMAGE_INFORMATION;
pub type PBOOTDISK_INFORMATION = *mut BOOTDISK_INFORMATION;
pub type PBOOTDISK_INFORMATION_EX = *mut BOOTDISK_INFORMATION_EX;
pub type PBOOTDISK_INFORMATION_LITE = *mut BOOTDISK_INFORMATION_LITE;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
pub type PBOUND_CALLBACK = *mut BOUND_CALLBACK;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
pub type PBOUND_CALLBACK_STATUS = *mut BOUND_CALLBACK_STATUS;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PBUILD_MDL_FROM_SCATTER_GATHER_LIST = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, scattergather: *const SCATTER_GATHER_LIST, originalmdl: *const MDL, targetmdl: *mut super::usb::PMDL) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PBUILD_SCATTER_GATHER_LIST = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, deviceobject: *const DEVICE_OBJECT, mdl: *const MDL, currentva: *const core::ffi::c_void, length: u32, executionroutine: PDRIVER_LIST_CONTROL, context: *const core::ffi::c_void, writetodevice: bool, scattergatherbuffer: *const core::ffi::c_void, scattergatherlength: u32) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PBUILD_SCATTER_GATHER_LIST_EX = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, deviceobject: *const DEVICE_OBJECT, dmatransfercontext: *const core::ffi::c_void, mdl: *const MDL, offset: u64, length: u32, flags: u32, executionroutine: PDRIVER_LIST_CONTROL, context: *const core::ffi::c_void, writetodevice: bool, scattergatherbuffer: *const core::ffi::c_void, scattergatherlength: u32, dmacompletionroutine: PDMA_COMPLETION_ROUTINE, completioncontext: *const core::ffi::c_void, scattergatherlist: *mut core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PBUS_INTERFACE_STANDARD = *mut BUS_INTERFACE_STANDARD;
pub type PBUS_QUERY_ID_TYPE = *mut BUS_QUERY_ID_TYPE;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "usb"))]
pub type PBUS_RESOURCE_UPDATE_INTERFACE = *mut BUS_RESOURCE_UPDATE_INTERFACE;
pub type PBUS_SPECIFIC_RESET_FLAGS = *mut BUS_SPECIFIC_RESET_FLAGS;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PCALCULATE_SCATTER_GATHER_LIST_SIZE = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, mdl: *const MDL, currentva: *const core::ffi::c_void, length: u32, scattergatherlistsize: *mut u32, pnumberofmapregisters: *mut u32) -> super::bcrypt::NTSTATUS>;
pub type PCALLBACK_FUNCTION = *mut CALLBACK_FUNCTION;
pub type PCALLBACK_OBJECT = *mut _CALLBACK_OBJECT;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PCANCEL_ADAPTER_CHANNEL = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, deviceobject: *const DEVICE_OBJECT, dmatransfercontext: *const core::ffi::c_void) -> bool>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PCANCEL_MAPPED_TRANSFER = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, dmatransfercontext: *const core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
pub type PCDRIVER_RUNTIME_INIT_FLAGS = *const DRIVER_RUNTIME_INIT_FLAGS;
pub const PCIBus: INTERFACE_TYPE = 5;
pub const PCI_ACS_ALLOWED: u32 = 0;
pub type PCI_ACS_BIT = i32;
pub const PCI_ACS_BLOCKED: u32 = 1;
pub const PCI_ACS_REDIRECTED: u32 = 2;
pub const PCI_ADDRESS_IO_ADDRESS_MASK: u32 = 4294967292;
pub const PCI_ADDRESS_IO_SPACE: u32 = 1;
pub const PCI_ADDRESS_MEMORY_ADDRESS_MASK: u32 = 4294967280;
pub const PCI_ADDRESS_MEMORY_PREFETCHABLE: u32 = 8;
pub const PCI_ADDRESS_MEMORY_TYPE_MASK: u32 = 6;
pub const PCI_ADDRESS_ROM_ADDRESS_MASK: u32 = 4294965248;
#[repr(C)]
#[cfg(feature = "bcrypt")]
#[derive(Clone, Copy, Debug)]
pub struct PCI_ATS_INTERFACE {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
    pub SetAddressTranslationServices: PPCI_SET_ATS,
    pub InvalidateQueueDepth: u8,
}
#[cfg(feature = "bcrypt")]
impl Default for PCI_ATS_INTERFACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PCI_ATS_INTERFACE_VERSION: u32 = 1;
pub const PCI_BRIDGE_TYPE: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PCI_CAPABILITIES_HEADER {
    pub CapabilityID: u8,
    pub Next: u8,
}
pub const PCI_CAPABILITY_ID_ADVANCED_FEATURES: u32 = 19;
pub const PCI_CAPABILITY_ID_AGP: u32 = 2;
pub const PCI_CAPABILITY_ID_AGP_TARGET: u32 = 14;
pub const PCI_CAPABILITY_ID_CPCI_HOTSWAP: u32 = 6;
pub const PCI_CAPABILITY_ID_CPCI_RES_CTRL: u32 = 11;
pub const PCI_CAPABILITY_ID_DEBUG_PORT: u32 = 10;
pub const PCI_CAPABILITY_ID_FPB: u32 = 21;
pub const PCI_CAPABILITY_ID_HYPERTRANSPORT: u32 = 8;
pub const PCI_CAPABILITY_ID_MSI: u32 = 5;
pub const PCI_CAPABILITY_ID_MSIX: u32 = 17;
pub const PCI_CAPABILITY_ID_P2P_SSID: u32 = 13;
pub const PCI_CAPABILITY_ID_PCIX: u32 = 7;
pub const PCI_CAPABILITY_ID_PCI_EXPRESS: u32 = 16;
pub const PCI_CAPABILITY_ID_POWER_MANAGEMENT: u32 = 1;
pub const PCI_CAPABILITY_ID_SATA_CONFIG: u32 = 18;
pub const PCI_CAPABILITY_ID_SECURE: u32 = 15;
pub const PCI_CAPABILITY_ID_SHPC: u32 = 12;
pub const PCI_CAPABILITY_ID_SLOT_ID: u32 = 4;
pub const PCI_CAPABILITY_ID_VENDOR_SPECIFIC: u32 = 9;
pub const PCI_CAPABILITY_ID_VPD: u32 = 3;
pub const PCI_CARDBUS_BRIDGE_TYPE: u32 = 2;
pub const PCI_CLASS_BASE_SYSTEM_DEV: u32 = 8;
pub const PCI_CLASS_BRIDGE_DEV: u32 = 6;
pub const PCI_CLASS_DATA_ACQ_SIGNAL_PROC: u32 = 17;
pub const PCI_CLASS_DISPLAY_CTLR: u32 = 3;
pub const PCI_CLASS_DOCKING_STATION: u32 = 10;
pub const PCI_CLASS_ENCRYPTION_DECRYPTION: u32 = 16;
pub const PCI_CLASS_INPUT_DEV: u32 = 9;
pub const PCI_CLASS_INTELLIGENT_IO_CTLR: u32 = 14;
pub const PCI_CLASS_MASS_STORAGE_CTLR: u32 = 1;
pub const PCI_CLASS_MEMORY_CTLR: u32 = 5;
pub const PCI_CLASS_MULTIMEDIA_DEV: u32 = 4;
pub const PCI_CLASS_NETWORK_CTLR: u32 = 2;
pub const PCI_CLASS_NOT_DEFINED: u32 = 255;
pub const PCI_CLASS_PRE_20: u32 = 0;
pub const PCI_CLASS_PROCESSOR: u32 = 11;
pub const PCI_CLASS_SATELLITE_COMMS_CTLR: u32 = 15;
pub const PCI_CLASS_SERIAL_BUS_CTLR: u32 = 12;
pub const PCI_CLASS_SIMPLE_COMMS_CTLR: u32 = 7;
pub const PCI_CLASS_WIRELESS_CTLR: u32 = 13;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_COMMON_CONFIG {
    pub Base: PCI_COMMON_HEADER,
    pub DeviceSpecific: [u8; 192],
}
impl Default for PCI_COMMON_CONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PCI_COMMON_HDR_LENGTH: u32 = 64;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_COMMON_HEADER {
    pub VendorID: u16,
    pub DeviceID: u16,
    pub Command: u16,
    pub Status: u16,
    pub RevisionID: u8,
    pub ProgIf: u8,
    pub SubClass: u8,
    pub BaseClass: u8,
    pub CacheLineSize: u8,
    pub LatencyTimer: u8,
    pub HeaderType: u8,
    pub BIST: u8,
    pub u: PCI_COMMON_HEADER_0,
}
impl Default for PCI_COMMON_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_COMMON_HEADER_0 {
    pub type0: PCI_COMMON_HEADER_0_0,
    pub type1: PCI_COMMON_HEADER_0_1,
    pub type2: PCI_COMMON_HEADER_0_2,
}
impl Default for PCI_COMMON_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PCI_COMMON_HEADER_0_0 {
    pub BaseAddresses: [u32; 6],
    pub CIS: u32,
    pub SubVendorID: u16,
    pub SubSystemID: u16,
    pub ROMBaseAddress: u32,
    pub CapabilitiesPtr: u8,
    pub Reserved1: [u8; 3],
    pub Reserved2: u32,
    pub InterruptLine: u8,
    pub InterruptPin: u8,
    pub MinimumGrant: u8,
    pub MaximumLatency: u8,
}
impl Default for PCI_COMMON_HEADER_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PCI_COMMON_HEADER_0_1 {
    pub BaseAddresses: [u32; 2],
    pub PrimaryBus: u8,
    pub SecondaryBus: u8,
    pub SubordinateBus: u8,
    pub SecondaryLatency: u8,
    pub IOBase: u8,
    pub IOLimit: u8,
    pub SecondaryStatus: u16,
    pub MemoryBase: u16,
    pub MemoryLimit: u16,
    pub PrefetchBase: u16,
    pub PrefetchLimit: u16,
    pub PrefetchBaseUpper32: u32,
    pub PrefetchLimitUpper32: u32,
    pub IOBaseUpper16: u16,
    pub IOLimitUpper16: u16,
    pub CapabilitiesPtr: u8,
    pub Reserved1: [u8; 3],
    pub ROMBaseAddress: u32,
    pub InterruptLine: u8,
    pub InterruptPin: u8,
    pub BridgeControl: u16,
}
impl Default for PCI_COMMON_HEADER_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PCI_COMMON_HEADER_0_2 {
    pub SocketRegistersBaseAddress: u32,
    pub CapabilitiesPtr: u8,
    pub Reserved: u8,
    pub SecondaryStatus: u16,
    pub PrimaryBus: u8,
    pub SecondaryBus: u8,
    pub SubordinateBus: u8,
    pub SecondaryLatency: u8,
    pub Range: [PCI_COMMON_HEADER_0_2_0; 4],
    pub InterruptLine: u8,
    pub InterruptPin: u8,
    pub BridgeControl: u16,
}
impl Default for PCI_COMMON_HEADER_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PCI_COMMON_HEADER_0_2_0 {
    pub Base: u32,
    pub Limit: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PCI_DEVICE_PRESENCE_PARAMETERS {
    pub Size: u32,
    pub Flags: u32,
    pub VendorID: u16,
    pub DeviceID: u16,
    pub RevisionID: u8,
    pub SubVendorID: u16,
    pub SubSystemID: u16,
    pub BaseClass: u8,
    pub SubClass: u8,
    pub ProgIf: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PCI_DEVICE_PRESENT_INTERFACE {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
    pub IsDevicePresent: PPCI_IS_DEVICE_PRESENT,
    pub IsDevicePresentEx: PPCI_IS_DEVICE_PRESENT_EX,
}
impl Default for PCI_DEVICE_PRESENT_INTERFACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PCI_DEVICE_PRESENT_INTERFACE_VERSION: u32 = 1;
pub const PCI_DEVICE_TYPE: u32 = 0;
pub const PCI_DISABLE_LEVEL_INTERRUPT: u32 = 1024;
#[repr(C)]
#[cfg(feature = "bcrypt")]
#[derive(Clone, Copy, Debug)]
pub struct PCI_DOE_INTERFACE {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
    pub QuerySupportedDoeProtocols: PQUERY_SUPPORTED_DOE_PROTOCOLS,
    pub ResetDoeInstances: PRESET_DOE_INSTANCES,
    pub SendDoeRequest: PSEND_DOE_REQUEST,
}
#[cfg(feature = "bcrypt")]
impl Default for PCI_DOE_INTERFACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
#[derive(Clone, Copy, Debug)]
pub struct PCI_DOE_INTERFACE2 {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
    pub QuerySupportedDoeProtocols: PQUERY_SUPPORTED_DOE_PROTOCOLS,
    pub ResetDoeInstances: PRESET_DOE_INSTANCES,
    pub SendDoeRequest: PSEND_DOE_REQUEST,
    pub SendDoeRequestAsync: PSEND_DOE_REQUEST_ASYNC,
    pub GetDoePreviousResponse: PGET_DOE_PREVIOUS_RESPONSE,
}
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
impl Default for PCI_DOE_INTERFACE2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PCI_ENABLE_BUS_MASTER: u32 = 4;
pub const PCI_ENABLE_FAST_BACK_TO_BACK: u32 = 512;
pub const PCI_ENABLE_IO_SPACE: u32 = 1;
pub const PCI_ENABLE_MEMORY_SPACE: u32 = 2;
pub const PCI_ENABLE_PARITY: u32 = 64;
pub const PCI_ENABLE_SERR: u32 = 256;
pub const PCI_ENABLE_SPECIAL_CYCLES: u32 = 8;
pub const PCI_ENABLE_VGA_COMPATIBLE_PALETTE: u32 = 32;
pub const PCI_ENABLE_WAIT_CYCLE: u32 = 128;
pub const PCI_ENABLE_WRITE_AND_INVALIDATE: u32 = 16;
pub const PCI_EXPRESS_ACCESS_CONTROL_SERVICES_CAP_ID: u32 = 13;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_ACS_CAPABILITY {
    pub Header: PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub Capability: PCI_EXPRESS_ACS_CAPABILITY_REGISTER,
    pub Control: PCI_EXPRESS_ACS_CONTROL,
    pub EgressControl: [u32; 1],
}
impl Default for PCI_EXPRESS_ACS_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_ACS_CAPABILITY_REGISTER {
    pub Anonymous: PCI_EXPRESS_ACS_CAPABILITY_REGISTER_0,
    pub AsUSHORT: u16,
}
impl Default for PCI_EXPRESS_ACS_CAPABILITY_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PCI_EXPRESS_ACS_CAPABILITY_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_ACS_CONTROL {
    pub Anonymous: PCI_EXPRESS_ACS_CONTROL_0,
    pub AsUSHORT: u16,
}
impl Default for PCI_EXPRESS_ACS_CONTROL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PCI_EXPRESS_ACS_CONTROL_0 {
    pub _bitfield: u16,
}
pub const PCI_EXPRESS_ADVANCED_ERROR_REPORTING_CAP_ID: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_AER_CAPABILITIES {
    pub Anonymous: PCI_EXPRESS_AER_CAPABILITIES_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_AER_CAPABILITIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PCI_EXPRESS_AER_CAPABILITIES_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_AER_CAPABILITY {
    pub Header: PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub UncorrectableErrorStatus: PCI_EXPRESS_UNCORRECTABLE_ERROR_STATUS,
    pub UncorrectableErrorMask: PCI_EXPRESS_UNCORRECTABLE_ERROR_MASK,
    pub UncorrectableErrorSeverity: PCI_EXPRESS_UNCORRECTABLE_ERROR_SEVERITY,
    pub CorrectableErrorStatus: PCI_EXPRESS_CORRECTABLE_ERROR_STATUS,
    pub CorrectableErrorMask: PCI_EXPRESS_CORRECTABLE_ERROR_MASK,
    pub CapabilitiesAndControl: PCI_EXPRESS_AER_CAPABILITIES,
    pub HeaderLog: [u32; 4],
    pub SecUncorrectableErrorStatus: PCI_EXPRESS_SEC_UNCORRECTABLE_ERROR_STATUS,
    pub SecUncorrectableErrorMask: PCI_EXPRESS_SEC_UNCORRECTABLE_ERROR_MASK,
    pub SecUncorrectableErrorSeverity: PCI_EXPRESS_SEC_UNCORRECTABLE_ERROR_SEVERITY,
    pub SecCapabilitiesAndControl: PCI_EXPRESS_SEC_AER_CAPABILITIES,
    pub SecHeaderLog: [u32; 4],
}
impl Default for PCI_EXPRESS_AER_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PCI_EXPRESS_ARI_CAPABILITY {
    pub Header: PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub Capability: PCI_EXPRESS_ARI_CAPABILITY_REGISTER,
    pub Control: PCI_EXPRESS_ARI_CONTROL_REGISTER,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PCI_EXPRESS_ARI_CAPABILITY_REGISTER {
    pub _bitfield: u16,
}
pub const PCI_EXPRESS_ARI_CAP_ID: u32 = 14;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PCI_EXPRESS_ARI_CONTROL_REGISTER {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_ATS_CAPABILITY {
    pub Header: PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub Capability: PCI_EXPRESS_ATS_CAPABILITY_REGISTER,
    pub Control: PCI_EXPRESS_ATS_CONTROL_REGISTER,
}
impl Default for PCI_EXPRESS_ATS_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PCI_EXPRESS_ATS_CAPABILITY_REGISTER {
    pub _bitfield: u16,
}
pub const PCI_EXPRESS_ATS_CAP_ID: u32 = 15;
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_ATS_CONTROL_REGISTER {
    pub Anonymous: PCI_EXPRESS_ATS_CONTROL_REGISTER_0,
    pub AsUSHORT: u16,
}
impl Default for PCI_EXPRESS_ATS_CONTROL_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PCI_EXPRESS_ATS_CONTROL_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_BRIDGE_AER_CAPABILITY {
    pub Header: PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub UncorrectableErrorStatus: PCI_EXPRESS_UNCORRECTABLE_ERROR_STATUS,
    pub UncorrectableErrorMask: PCI_EXPRESS_UNCORRECTABLE_ERROR_MASK,
    pub UncorrectableErrorSeverity: PCI_EXPRESS_UNCORRECTABLE_ERROR_SEVERITY,
    pub CorrectableErrorStatus: PCI_EXPRESS_CORRECTABLE_ERROR_STATUS,
    pub CorrectableErrorMask: PCI_EXPRESS_CORRECTABLE_ERROR_MASK,
    pub CapabilitiesAndControl: PCI_EXPRESS_AER_CAPABILITIES,
    pub HeaderLog: [u32; 4],
    pub SecUncorrectableErrorStatus: PCI_EXPRESS_SEC_UNCORRECTABLE_ERROR_STATUS,
    pub SecUncorrectableErrorMask: PCI_EXPRESS_SEC_UNCORRECTABLE_ERROR_MASK,
    pub SecUncorrectableErrorSeverity: PCI_EXPRESS_SEC_UNCORRECTABLE_ERROR_SEVERITY,
    pub SecCapabilitiesAndControl: PCI_EXPRESS_SEC_AER_CAPABILITIES,
    pub SecHeaderLog: [u32; 4],
}
impl Default for PCI_EXPRESS_BRIDGE_AER_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PCI_EXPRESS_CONFIGURATION_ACCESS_CORRELATION_CAP_ID: u32 = 12;
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_CORRECTABLE_ERROR_MASK {
    pub Anonymous: PCI_EXPRESS_CORRECTABLE_ERROR_MASK_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_CORRECTABLE_ERROR_MASK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PCI_EXPRESS_CORRECTABLE_ERROR_MASK_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_CORRECTABLE_ERROR_STATUS {
    pub Anonymous: PCI_EXPRESS_CORRECTABLE_ERROR_STATUS_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_CORRECTABLE_ERROR_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PCI_EXPRESS_CORRECTABLE_ERROR_STATUS_0 {
    pub _bitfield: u32,
}
pub const PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_CAP_ID: u32 = 35;
pub const PCI_EXPRESS_DEVICE_3_CAP_ID: u32 = 47;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_DEVICE_3_EXTENDED_CAPABILITY {
    pub Header: PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub Capability: PCI_EXPRESS_DEVICE_CAPABILITIES_3_REGISTER,
    pub Control: PCI_EXPRESS_DEVICE_CONTROL_3_REGISTER,
}
impl Default for PCI_EXPRESS_DEVICE_3_EXTENDED_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DEVICE_CAPABILITIES_3_REGISTER {
    pub Anonymous: PCI_EXPRESS_DEVICE_CAPABILITIES_3_REGISTER_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_DEVICE_CAPABILITIES_3_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PCI_EXPRESS_DEVICE_CAPABILITIES_3_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DEVICE_CONTROL_3_REGISTER {
    pub Anonymous: PCI_EXPRESS_DEVICE_CONTROL_3_REGISTER_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_DEVICE_CONTROL_3_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PCI_EXPRESS_DEVICE_CONTROL_3_REGISTER_0 {
    pub _bitfield: u32,
}
pub const PCI_EXPRESS_DEVICE_SERIAL_NUMBER_CAP_ID: u32 = 3;
pub const PCI_EXPRESS_DOE_CAP_ID: u32 = 46;
pub const PCI_EXPRESS_DPA_CAP_ID: u32 = 22;
pub const PCI_EXPRESS_DPC_CAP_ID: u32 = 29;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER {
    pub CapabilityID: u16,
    pub _bitfield: u16,
}
#[cfg(feature = "bcrypt")]
pub type PCI_EXPRESS_ENTER_LINK_QUIESCENT_MODE = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_ERROR_SOURCE_ID {
    pub Anonymous: PCI_EXPRESS_ERROR_SOURCE_ID_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_ERROR_SOURCE_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PCI_EXPRESS_ERROR_SOURCE_ID_0 {
    pub _bitfield1: u16,
    pub _bitfield2: u16,
}
#[cfg(feature = "bcrypt")]
pub type PCI_EXPRESS_EXIT_LINK_QUIESCENT_MODE = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
pub const PCI_EXPRESS_FRS_QUEUEING_CAP_ID: u32 = 33;
pub const PCI_EXPRESS_IDE_CAP_ID: u32 = 48;
pub const PCI_EXPRESS_L1_PM_SS_CAP_ID: u32 = 30;
#[repr(C)]
#[cfg(feature = "bcrypt")]
#[derive(Clone, Copy, Debug)]
pub struct PCI_EXPRESS_LINK_QUIESCENT_INTERFACE {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
    pub PciExpressEnterLinkQuiescentMode: PPCI_EXPRESS_ENTER_LINK_QUIESCENT_MODE,
    pub PciExpressExitLinkQuiescentMode: PPCI_EXPRESS_EXIT_LINK_QUIESCENT_MODE,
}
#[cfg(feature = "bcrypt")]
impl Default for PCI_EXPRESS_LINK_QUIESCENT_INTERFACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PCI_EXPRESS_LINK_QUIESCENT_INTERFACE_VERSION: u32 = 1;
pub const PCI_EXPRESS_LN_REQUESTER_CAP_ID: u32 = 28;
pub const PCI_EXPRESS_LTR_CAP_ID: u32 = 24;
pub const PCI_EXPRESS_MFVC_CAP_ID: u32 = 8;
pub const PCI_EXPRESS_MPCIE_CAP_ID: u32 = 32;
pub const PCI_EXPRESS_MULTICAST_CAP_ID: u32 = 18;
pub const PCI_EXPRESS_MULTI_ROOT_IO_VIRTUALIZATION_CAP_ID: u32 = 17;
pub const PCI_EXPRESS_NPEM_CAP_ID: u32 = 41;
pub const PCI_EXPRESS_PAGE_REQUEST_CAP_ID: u32 = 19;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_PASID_CAPABILITY {
    pub Header: PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub Capability: PCI_EXPRESS_PASID_CAPABILITY_REGISTER,
    pub Control: PCI_EXPRESS_PASID_CONTROL_REGISTER,
}
impl Default for PCI_EXPRESS_PASID_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_PASID_CAPABILITY_REGISTER {
    pub Anonymous: PCI_EXPRESS_PASID_CAPABILITY_REGISTER_0,
    pub AsUSHORT: u16,
}
impl Default for PCI_EXPRESS_PASID_CAPABILITY_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PCI_EXPRESS_PASID_CAPABILITY_REGISTER_0 {
    pub _bitfield: u16,
}
pub const PCI_EXPRESS_PASID_CAP_ID: u32 = 27;
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_PASID_CONTROL_REGISTER {
    pub Anonymous: PCI_EXPRESS_PASID_CONTROL_REGISTER_0,
    pub AsUSHORT: u16,
}
impl Default for PCI_EXPRESS_PASID_CONTROL_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PCI_EXPRESS_PASID_CONTROL_REGISTER_0 {
    pub _bitfield: u16,
}
pub const PCI_EXPRESS_PMUX_CAP_ID: u32 = 26;
pub const PCI_EXPRESS_POWER_BUDGETING_CAP_ID: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_PRI_CAPABILITY {
    pub Header: PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub Control: PCI_EXPRESS_PRI_CONTROL_REGISTER,
    pub Status: PCI_EXPRESS_PRI_STATUS_REGISTER,
    pub PRCapacity: u32,
    pub PRAllocation: u32,
}
impl Default for PCI_EXPRESS_PRI_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_PRI_CONTROL_REGISTER {
    pub Anonymous: PCI_EXPRESS_PRI_CONTROL_REGISTER_0,
    pub AsUSHORT: u16,
}
impl Default for PCI_EXPRESS_PRI_CONTROL_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PCI_EXPRESS_PRI_CONTROL_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_PRI_STATUS_REGISTER {
    pub Anonymous: PCI_EXPRESS_PRI_STATUS_REGISTER_0,
    pub AsUSHORT: u16,
}
impl Default for PCI_EXPRESS_PRI_STATUS_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PCI_EXPRESS_PRI_STATUS_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_PTM_CAPABILITY {
    pub Header: PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub PtmCapability: PCI_EXPRESS_PTM_CAPABILITY_REGISTER,
    pub PtmControl: PCI_EXPRESS_PTM_CONTROL_REGISTER,
}
impl Default for PCI_EXPRESS_PTM_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_PTM_CAPABILITY_REGISTER {
    pub Anonymous: PCI_EXPRESS_PTM_CAPABILITY_REGISTER_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_PTM_CAPABILITY_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PCI_EXPRESS_PTM_CAPABILITY_REGISTER_0 {
    pub _bitfield: u32,
}
pub const PCI_EXPRESS_PTM_CAP_ID: u32 = 31;
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_PTM_CONTROL_REGISTER {
    pub Anonymous: PCI_EXPRESS_PTM_CONTROL_REGISTER_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_PTM_CONTROL_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PCI_EXPRESS_PTM_CONTROL_REGISTER_0 {
    pub _bitfield: u32,
}
pub const PCI_EXPRESS_RCRB_HEADER_CAP_ID: u32 = 10;
pub const PCI_EXPRESS_RC_EVENT_COLLECTOR_ENDPOINT_ASSOCIATION_CAP_ID: u32 = 7;
pub const PCI_EXPRESS_RC_INTERNAL_LINK_CONTROL_CAP_ID: u32 = 6;
pub const PCI_EXPRESS_RC_LINK_DECLARATION_CAP_ID: u32 = 5;
pub const PCI_EXPRESS_READINESS_TIME_REPORTING_CAP_ID: u32 = 34;
pub const PCI_EXPRESS_RESERVED_FOR_AMD_CAP_ID: u32 = 20;
pub const PCI_EXPRESS_RESIZABLE_BAR_CAP_ID: u32 = 21;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_ROOTPORT_AER_CAPABILITY {
    pub Header: PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub UncorrectableErrorStatus: PCI_EXPRESS_UNCORRECTABLE_ERROR_STATUS,
    pub UncorrectableErrorMask: PCI_EXPRESS_UNCORRECTABLE_ERROR_MASK,
    pub UncorrectableErrorSeverity: PCI_EXPRESS_UNCORRECTABLE_ERROR_SEVERITY,
    pub CorrectableErrorStatus: PCI_EXPRESS_CORRECTABLE_ERROR_STATUS,
    pub CorrectableErrorMask: PCI_EXPRESS_CORRECTABLE_ERROR_MASK,
    pub CapabilitiesAndControl: PCI_EXPRESS_AER_CAPABILITIES,
    pub HeaderLog: [u32; 4],
    pub RootErrorCommand: PCI_EXPRESS_ROOT_ERROR_COMMAND,
    pub RootErrorStatus: PCI_EXPRESS_ROOT_ERROR_STATUS,
    pub ErrorSourceId: PCI_EXPRESS_ERROR_SOURCE_ID,
}
impl Default for PCI_EXPRESS_ROOTPORT_AER_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_ROOT_ERROR_COMMAND {
    pub Anonymous: PCI_EXPRESS_ROOT_ERROR_COMMAND_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_ROOT_ERROR_COMMAND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PCI_EXPRESS_ROOT_ERROR_COMMAND_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_ROOT_ERROR_STATUS {
    pub Anonymous: PCI_EXPRESS_ROOT_ERROR_STATUS_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_ROOT_ERROR_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PCI_EXPRESS_ROOT_ERROR_STATUS_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PCI_EXPRESS_ROOT_PORT_INTERFACE {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
    pub ReadConfigSpace: PPCI_EXPRESS_ROOT_PORT_READ_CONFIG_SPACE,
    pub WriteConfigSpace: PPCI_EXPRESS_ROOT_PORT_WRITE_CONFIG_SPACE,
}
impl Default for PCI_EXPRESS_ROOT_PORT_INTERFACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PCI_EXPRESS_ROOT_PORT_INTERFACE_VERSION: u32 = 1;
pub const PCI_EXPRESS_SECONDARY_PCI_EXPRESS_CAP_ID: u32 = 25;
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_SEC_AER_CAPABILITIES {
    pub Anonymous: PCI_EXPRESS_SEC_AER_CAPABILITIES_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_SEC_AER_CAPABILITIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PCI_EXPRESS_SEC_AER_CAPABILITIES_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_SEC_UNCORRECTABLE_ERROR_MASK {
    pub Anonymous: PCI_EXPRESS_SEC_UNCORRECTABLE_ERROR_MASK_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_SEC_UNCORRECTABLE_ERROR_MASK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PCI_EXPRESS_SEC_UNCORRECTABLE_ERROR_MASK_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_SEC_UNCORRECTABLE_ERROR_SEVERITY {
    pub Anonymous: PCI_EXPRESS_SEC_UNCORRECTABLE_ERROR_SEVERITY_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_SEC_UNCORRECTABLE_ERROR_SEVERITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PCI_EXPRESS_SEC_UNCORRECTABLE_ERROR_SEVERITY_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_SEC_UNCORRECTABLE_ERROR_STATUS {
    pub Anonymous: PCI_EXPRESS_SEC_UNCORRECTABLE_ERROR_STATUS_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_SEC_UNCORRECTABLE_ERROR_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PCI_EXPRESS_SEC_UNCORRECTABLE_ERROR_STATUS_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PCI_EXPRESS_SERIAL_NUMBER_CAPABILITY {
    pub Header: PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub LowSerialNumber: u32,
    pub HighSerialNumber: u32,
}
pub const PCI_EXPRESS_SINGLE_ROOT_IO_VIRTUALIZATION_CAP_ID: u32 = 16;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_SRIOV_CAPABILITY {
    pub Header: PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub SRIOVCapabilities: PCI_EXPRESS_SRIOV_CAPS,
    pub SRIOVControl: PCI_EXPRESS_SRIOV_CONTROL,
    pub SRIOVStatus: PCI_EXPRESS_SRIOV_STATUS,
    pub InitialVFs: u16,
    pub TotalVFs: u16,
    pub NumVFs: u16,
    pub FunctionDependencyLink: u8,
    pub RsvdP1: u8,
    pub FirstVFOffset: u16,
    pub VFStride: u16,
    pub RsvdP2: u16,
    pub VFDeviceId: u16,
    pub SupportedPageSizes: u32,
    pub SystemPageSize: u32,
    pub BaseAddresses: [u32; 6],
    pub VFMigrationStateArrayOffset: PCI_EXPRESS_SRIOV_MIGRATION_STATE_ARRAY,
}
impl Default for PCI_EXPRESS_SRIOV_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_SRIOV_CAPS {
    pub Anonymous: PCI_EXPRESS_SRIOV_CAPS_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_SRIOV_CAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PCI_EXPRESS_SRIOV_CAPS_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_SRIOV_CONTROL {
    pub Anonymous: PCI_EXPRESS_SRIOV_CONTROL_0,
    pub AsUSHORT: u16,
}
impl Default for PCI_EXPRESS_SRIOV_CONTROL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PCI_EXPRESS_SRIOV_CONTROL_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_SRIOV_MIGRATION_STATE_ARRAY {
    pub Anonymous: PCI_EXPRESS_SRIOV_MIGRATION_STATE_ARRAY_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_SRIOV_MIGRATION_STATE_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PCI_EXPRESS_SRIOV_MIGRATION_STATE_ARRAY_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_SRIOV_STATUS {
    pub Anonymous: PCI_EXPRESS_SRIOV_STATUS_0,
    pub AsUSHORT: u16,
}
impl Default for PCI_EXPRESS_SRIOV_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PCI_EXPRESS_SRIOV_STATUS_0 {
    pub _bitfield: u16,
}
pub const PCI_EXPRESS_TPH_REQUESTER_CAP_ID: u32 = 23;
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_UNCORRECTABLE_ERROR_MASK {
    pub Anonymous: PCI_EXPRESS_UNCORRECTABLE_ERROR_MASK_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_UNCORRECTABLE_ERROR_MASK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PCI_EXPRESS_UNCORRECTABLE_ERROR_MASK_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_UNCORRECTABLE_ERROR_SEVERITY {
    pub Anonymous: PCI_EXPRESS_UNCORRECTABLE_ERROR_SEVERITY_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_UNCORRECTABLE_ERROR_SEVERITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PCI_EXPRESS_UNCORRECTABLE_ERROR_SEVERITY_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_UNCORRECTABLE_ERROR_STATUS {
    pub Anonymous: PCI_EXPRESS_UNCORRECTABLE_ERROR_STATUS_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_UNCORRECTABLE_ERROR_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PCI_EXPRESS_UNCORRECTABLE_ERROR_STATUS_0 {
    pub _bitfield: u32,
}
pub const PCI_EXPRESS_VC_AND_MFVC_CAP_ID: u32 = 9;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PCI_EXPRESS_VENDOR_SPECIFIC_CAPABILITY {
    pub Header: PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub VsecId: u16,
    pub _bitfield: u16,
}
pub const PCI_EXPRESS_VENDOR_SPECIFIC_CAP_ID: u32 = 11;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_VIRTUAL_CHANNEL_CAPABILITY {
    pub Header: PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub Capabilities1: VIRTUAL_CHANNEL_CAPABILITIES1,
    pub Capabilities2: VIRTUAL_CHANNEL_CAPABILITIES2,
    pub Control: VIRTUAL_CHANNEL_CONTROL,
    pub Status: VIRTUAL_CHANNEL_STATUS,
    pub Resource: [VIRTUAL_RESOURCE; 8],
}
impl Default for PCI_EXPRESS_VIRTUAL_CHANNEL_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PCI_EXPRESS_VIRTUAL_CHANNEL_CAP_ID: u32 = 2;
pub const PCI_EXTENDED_CONFIG_LENGTH: u32 = 4096;
pub const PCI_INVALID_VENDORID: u32 = 65535;
pub type PCI_IS_DEVICE_PRESENT = Option<unsafe extern "system" fn(vendorid: u16, deviceid: u16, revisionid: u8, subvendorid: u16, subsystemid: u16, flags: u32) -> bool>;
pub type PCI_IS_DEVICE_PRESENT_EX = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, parameters: *const PCI_DEVICE_PRESENCE_PARAMETERS) -> bool>;
pub const PCI_MAX_BRIDGE_NUMBER: u32 = 255;
pub const PCI_MAX_DEVICES: u32 = 32;
pub const PCI_MAX_FUNCTION: u32 = 8;
pub const PCI_MAX_SEGMENT_NUMBER: u32 = 65535;
#[cfg(feature = "bcrypt")]
pub type PCI_MSIX_GET_ENTRY = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, tableentry: u32, messagenumber: *mut u32, masked: *mut bool) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "bcrypt")]
pub type PCI_MSIX_GET_TABLE_SIZE = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, tablesize: *mut u32) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "bcrypt")]
pub type PCI_MSIX_MASKUNMASK_ENTRY = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, tableentry: u32) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "bcrypt")]
pub type PCI_MSIX_SET_ENTRY = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, tableentry: u32, messagenumber: u32) -> super::bcrypt::NTSTATUS>;
#[repr(C)]
#[cfg(feature = "bcrypt")]
#[derive(Clone, Copy, Debug)]
pub struct PCI_MSIX_TABLE_CONFIG_INTERFACE {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
    pub SetTableEntry: PPCI_MSIX_SET_ENTRY,
    pub MaskTableEntry: PPCI_MSIX_MASKUNMASK_ENTRY,
    pub UnmaskTableEntry: PPCI_MSIX_MASKUNMASK_ENTRY,
    pub GetTableEntry: PPCI_MSIX_GET_ENTRY,
    pub GetTableSize: PPCI_MSIX_GET_TABLE_SIZE,
}
#[cfg(feature = "bcrypt")]
impl Default for PCI_MSIX_TABLE_CONFIG_INTERFACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PCI_MSIX_TABLE_CONFIG_INTERFACE_VERSION: u32 = 1;
#[cfg(target_arch = "x86")]
pub const PCI_MSIX_TABLE_CONFIG_MINIMUM_SIZE: u32 = 28;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const PCI_MSIX_TABLE_CONFIG_MINIMUM_SIZE: u32 = 56;
pub const PCI_MULTIFUNCTION: u32 = 128;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PCI_PMC {
    pub _bitfield: u8,
    pub Support: PCI_PMC_0,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PCI_PMCSR {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PCI_PMCSR_BSE {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PCI_PMC_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_PM_CAPABILITY {
    pub Header: PCI_CAPABILITIES_HEADER,
    pub PMC: PCI_PM_CAPABILITY_0,
    pub PMCSR: PCI_PM_CAPABILITY_1,
    pub PMCSR_BSE: PCI_PM_CAPABILITY_2,
    pub Data: u8,
}
impl Default for PCI_PM_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_PM_CAPABILITY_0 {
    pub Capabilities: PCI_PMC,
    pub AsUSHORT: u16,
}
impl Default for PCI_PM_CAPABILITY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_PM_CAPABILITY_1 {
    pub ControlStatus: PCI_PMCSR,
    pub AsUSHORT: u16,
}
impl Default for PCI_PM_CAPABILITY_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_PM_CAPABILITY_2 {
    pub BridgeSupport: PCI_PMCSR_BSE,
    pub AsUCHAR: u8,
}
impl Default for PCI_PM_CAPABILITY_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PCI_PROGRAMMING_INTERFACE_MSC_NVM_EXPRESS: u32 = 2;
pub const PCI_PTM_TIME_SOURCE_AUX: u32 = 4294967295;
pub const PCI_ROMADDRESS_ENABLED: u32 = 1;
pub const PCI_SECURITY_DIRECT_TRANSLATED_P2P: u32 = 4;
pub const PCI_SECURITY_ENHANCED: u32 = 2;
pub const PCI_SECURITY_FULLY_SUPPORTED: u32 = 1;
pub const PCI_SECURITY_GUEST_ASSIGNED: u32 = 1;
#[repr(C)]
#[cfg(feature = "bcrypt")]
#[derive(Clone, Copy, Debug)]
pub struct PCI_SECURITY_INTERFACE {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
    pub SetAccessControlServices: PPCI_SET_ACS,
}
#[cfg(feature = "bcrypt")]
impl Default for PCI_SECURITY_INTERFACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "bcrypt")]
#[derive(Clone, Copy, Debug)]
pub struct PCI_SECURITY_INTERFACE2 {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
    pub Flags: u32,
    pub SupportedScenarios: u32,
    pub SetAccessControlServices: PPCI_SET_ACS2,
    pub SetAccessControlServices3: PPCI_SET_ACS3,
}
#[cfg(feature = "bcrypt")]
impl Default for PCI_SECURITY_INTERFACE2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PCI_SECURITY_INTERFACE_VERSION: u32 = 1;
pub const PCI_SECURITY_INTERFACE_VERSION2: u32 = 2;
pub const PCI_SECURITY_INTERFACE_VERSION3: u32 = 3;
pub const PCI_SECURITY_SRIOV_DIRECT_TRANSLATED_P2P: u32 = 262144;
pub const PCI_SECURITY_UNTRANSLATED_P2P_SOURCE: u32 = 8;
pub const PCI_SECURITY_UNTRANSLATED_P2P_TARGET: u32 = 16;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_SEGMENT_BUS_NUMBER {
    pub u: PCI_SEGMENT_BUS_NUMBER_0,
}
impl Default for PCI_SEGMENT_BUS_NUMBER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_SEGMENT_BUS_NUMBER_0 {
    pub bits: PCI_SEGMENT_BUS_NUMBER_0_0,
    pub AsULONG: u32,
}
impl Default for PCI_SEGMENT_BUS_NUMBER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PCI_SEGMENT_BUS_NUMBER_0_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "bcrypt")]
pub type PCI_SET_ACS = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, enablesourcevalidation: PCI_ACS_BIT, enabletranslationblocking: PCI_ACS_BIT, enablep2prequestredirect: PCI_ACS_BIT, enablecompletionredirect: PCI_ACS_BIT, enableupstreamforwarding: PCI_ACS_BIT, enableegresscontrol: PCI_ACS_BIT, enabledirecttranslatedp2p: PCI_ACS_BIT) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "bcrypt")]
pub type PCI_SET_ACS2 = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, scenariostomodify: u32, scenariostate: u32) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "bcrypt")]
pub type PCI_SET_ACS3 = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, scenariostomodify: u32, scenariostate: u32, partitionid: u64) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "bcrypt")]
pub type PCI_SET_ATS = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, enableats: bool) -> super::bcrypt::NTSTATUS>;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_SLOT_NUMBER {
    pub u: PCI_SLOT_NUMBER_0,
}
impl Default for PCI_SLOT_NUMBER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_SLOT_NUMBER_0 {
    pub bits: PCI_SLOT_NUMBER_0_0,
    pub AsULONG: u32,
}
impl Default for PCI_SLOT_NUMBER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PCI_SLOT_NUMBER_0_0 {
    pub _bitfield: u32,
}
pub const PCI_STATUS_66MHZ_CAPABLE: u32 = 32;
pub const PCI_STATUS_CAPABILITIES_LIST: u32 = 16;
pub const PCI_STATUS_DATA_PARITY_DETECTED: u32 = 256;
pub const PCI_STATUS_DETECTED_PARITY_ERROR: u32 = 32768;
pub const PCI_STATUS_DEVSEL: u32 = 1536;
pub const PCI_STATUS_FAST_BACK_TO_BACK: u32 = 128;
pub const PCI_STATUS_IMMEDIATE_READINESS: u32 = 1;
pub const PCI_STATUS_INTERRUPT_PENDING: u32 = 8;
pub const PCI_STATUS_RECEIVED_MASTER_ABORT: u32 = 8192;
pub const PCI_STATUS_RECEIVED_TARGET_ABORT: u32 = 4096;
pub const PCI_STATUS_SIGNALED_SYSTEM_ERROR: u32 = 16384;
pub const PCI_STATUS_SIGNALED_TARGET_ABORT: u32 = 2048;
pub const PCI_STATUS_UDF_SUPPORTED: u32 = 64;
pub const PCI_SUBCLASS_BR_CARDBUS: u32 = 7;
pub const PCI_SUBCLASS_BR_EISA: u32 = 2;
pub const PCI_SUBCLASS_BR_HOST: u32 = 0;
pub const PCI_SUBCLASS_BR_ISA: u32 = 1;
pub const PCI_SUBCLASS_BR_MCA: u32 = 3;
pub const PCI_SUBCLASS_BR_NUBUS: u32 = 6;
pub const PCI_SUBCLASS_BR_OTHER: u32 = 128;
pub const PCI_SUBCLASS_BR_PCI_TO_PCI: u32 = 4;
pub const PCI_SUBCLASS_BR_PCMCIA: u32 = 5;
pub const PCI_SUBCLASS_BR_RACEWAY: u32 = 8;
pub const PCI_SUBCLASS_COM_MODEM: u32 = 3;
pub const PCI_SUBCLASS_COM_MULTIPORT: u32 = 2;
pub const PCI_SUBCLASS_COM_OTHER: u32 = 128;
pub const PCI_SUBCLASS_COM_PARALLEL: u32 = 1;
pub const PCI_SUBCLASS_COM_SERIAL: u32 = 0;
pub const PCI_SUBCLASS_CRYPTO_ENTERTAINMENT: u32 = 16;
pub const PCI_SUBCLASS_CRYPTO_NET_COMP: u32 = 0;
pub const PCI_SUBCLASS_CRYPTO_OTHER: u32 = 128;
pub const PCI_SUBCLASS_DASP_DPIO: u32 = 0;
pub const PCI_SUBCLASS_DASP_OTHER: u32 = 128;
pub const PCI_SUBCLASS_DOC_GENERIC: u32 = 0;
pub const PCI_SUBCLASS_DOC_OTHER: u32 = 128;
pub const PCI_SUBCLASS_INP_DIGITIZER: u32 = 1;
pub const PCI_SUBCLASS_INP_GAMEPORT: u32 = 4;
pub const PCI_SUBCLASS_INP_KEYBOARD: u32 = 0;
pub const PCI_SUBCLASS_INP_MOUSE: u32 = 2;
pub const PCI_SUBCLASS_INP_OTHER: u32 = 128;
pub const PCI_SUBCLASS_INP_SCANNER: u32 = 3;
pub const PCI_SUBCLASS_INTIO_I2O: u32 = 0;
pub const PCI_SUBCLASS_MEM_FLASH: u32 = 1;
pub const PCI_SUBCLASS_MEM_OTHER: u32 = 128;
pub const PCI_SUBCLASS_MEM_RAM: u32 = 0;
pub const PCI_SUBCLASS_MM_AUDIO_DEV: u32 = 1;
pub const PCI_SUBCLASS_MM_OTHER: u32 = 128;
pub const PCI_SUBCLASS_MM_TELEPHONY_DEV: u32 = 2;
pub const PCI_SUBCLASS_MM_VIDEO_DEV: u32 = 0;
pub const PCI_SUBCLASS_MSC_AHCI_CTLR: u32 = 6;
pub const PCI_SUBCLASS_MSC_FLOPPY_CTLR: u32 = 2;
pub const PCI_SUBCLASS_MSC_IDE_CTLR: u32 = 1;
pub const PCI_SUBCLASS_MSC_IPI_CTLR: u32 = 3;
pub const PCI_SUBCLASS_MSC_NVM_CTLR: u32 = 8;
pub const PCI_SUBCLASS_MSC_OTHER: u32 = 128;
pub const PCI_SUBCLASS_MSC_RAID_CTLR: u32 = 4;
pub const PCI_SUBCLASS_MSC_SCSI_BUS_CTLR: u32 = 0;
pub const PCI_SUBCLASS_NET_ATM_CTLR: u32 = 3;
pub const PCI_SUBCLASS_NET_ETHERNET_CTLR: u32 = 0;
pub const PCI_SUBCLASS_NET_FDDI_CTLR: u32 = 2;
pub const PCI_SUBCLASS_NET_ISDN_CTLR: u32 = 4;
pub const PCI_SUBCLASS_NET_OTHER: u32 = 128;
pub const PCI_SUBCLASS_NET_TOKEN_RING_CTLR: u32 = 1;
pub const PCI_SUBCLASS_PRE_20_NON_VGA: u32 = 0;
pub const PCI_SUBCLASS_PRE_20_VGA: u32 = 1;
pub const PCI_SUBCLASS_PROC_386: u32 = 0;
pub const PCI_SUBCLASS_PROC_486: u32 = 1;
pub const PCI_SUBCLASS_PROC_ALPHA: u32 = 16;
pub const PCI_SUBCLASS_PROC_COPROCESSOR: u32 = 64;
pub const PCI_SUBCLASS_PROC_PENTIUM: u32 = 2;
pub const PCI_SUBCLASS_PROC_POWERPC: u32 = 32;
pub const PCI_SUBCLASS_SAT_AUDIO: u32 = 2;
pub const PCI_SUBCLASS_SAT_DATA: u32 = 4;
pub const PCI_SUBCLASS_SAT_TV: u32 = 1;
pub const PCI_SUBCLASS_SAT_VOICE: u32 = 3;
pub const PCI_SUBCLASS_SB_ACCESS: u32 = 1;
pub const PCI_SUBCLASS_SB_FIBRE_CHANNEL: u32 = 4;
pub const PCI_SUBCLASS_SB_IEEE1394: u32 = 0;
pub const PCI_SUBCLASS_SB_SMBUS: u32 = 5;
pub const PCI_SUBCLASS_SB_SSA: u32 = 2;
pub const PCI_SUBCLASS_SB_THUNDERBOLT: u32 = 10;
pub const PCI_SUBCLASS_SB_USB: u32 = 3;
pub const PCI_SUBCLASS_SYS_DMA_CTLR: u32 = 1;
pub const PCI_SUBCLASS_SYS_GEN_HOTPLUG_CTLR: u32 = 4;
pub const PCI_SUBCLASS_SYS_INTERRUPT_CTLR: u32 = 0;
pub const PCI_SUBCLASS_SYS_OTHER: u32 = 128;
pub const PCI_SUBCLASS_SYS_RCEC: u32 = 7;
pub const PCI_SUBCLASS_SYS_REAL_TIME_CLOCK: u32 = 3;
pub const PCI_SUBCLASS_SYS_SDIO_CTRL: u32 = 5;
pub const PCI_SUBCLASS_SYS_SYSTEM_TIMER: u32 = 2;
pub const PCI_SUBCLASS_VID_OTHER: u32 = 128;
pub const PCI_SUBCLASS_VID_VGA_CTLR: u32 = 0;
pub const PCI_SUBCLASS_VID_XGA_CTLR: u32 = 1;
pub const PCI_SUBCLASS_WIRELESS_CON_IR: u32 = 1;
pub const PCI_SUBCLASS_WIRELESS_IRDA: u32 = 0;
pub const PCI_SUBCLASS_WIRELESS_OTHER: u32 = 128;
pub const PCI_SUBCLASS_WIRELESS_RF: u32 = 16;
pub const PCI_SUBLCASS_VID_3D_CTLR: u32 = 2;
pub const PCI_TYPE0_ADDRESSES: u32 = 6;
pub const PCI_TYPE1_ADDRESSES: u32 = 2;
pub const PCI_TYPE2_ADDRESSES: u32 = 5;
pub const PCI_TYPE_20BIT: u32 = 2;
pub const PCI_TYPE_32BIT: u32 = 0;
pub const PCI_TYPE_64BIT: u32 = 4;
pub const PCI_USE_CLASS_SUBCLASS: u32 = 8;
pub const PCI_USE_LOCAL_BUS: u32 = 32;
pub const PCI_USE_LOCAL_DEVICE: u32 = 64;
pub const PCI_USE_PROGIF: u32 = 16;
pub const PCI_USE_REVISION: u32 = 2;
pub const PCI_USE_SUBSYSTEM_IDS: u32 = 1;
pub const PCI_USE_VENDEV_IDS: u32 = 4;
#[repr(C)]
#[cfg(feature = "bcrypt")]
#[derive(Clone, Copy, Debug)]
pub struct PCI_VIRTUALIZATION_INTERFACE {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
    pub SetVirtualFunctionData: PSET_VIRTUAL_DEVICE_DATA,
    pub GetVirtualFunctionData: PGET_VIRTUAL_DEVICE_DATA,
    pub GetLocation: PGET_VIRTUAL_DEVICE_LOCATION,
    pub GetResources: PGET_VIRTUAL_DEVICE_RESOURCES,
    pub EnableVirtualization: PENABLE_VIRTUALIZATION,
    pub GetVirtualFunctionProbedBars: PGET_VIRTUAL_FUNCTION_PROBED_BARS,
}
#[cfg(feature = "bcrypt")]
impl Default for PCI_VIRTUALIZATION_INTERFACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PCI_VIRTUAL_RESOURCE_COUNT: u32 = 8;
pub const PCI_WHICHSPACE_CONFIG: u32 = 0;
pub const PCI_WHICHSPACE_ROM: u32 = 1382638416;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_X_CAPABILITY {
    pub Header: PCI_CAPABILITIES_HEADER,
    pub Command: PCI_X_CAPABILITY_0,
    pub Status: PCI_X_CAPABILITY_1,
}
impl Default for PCI_X_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_X_CAPABILITY_0 {
    pub bits: PCI_X_CAPABILITY_0_0,
    pub AsUSHORT: u16,
}
impl Default for PCI_X_CAPABILITY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PCI_X_CAPABILITY_0_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_X_CAPABILITY_1 {
    pub bits: PCI_X_CAPABILITY_1_0,
    pub AsULONG: u32,
}
impl Default for PCI_X_CAPABILITY_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PCI_X_CAPABILITY_1_0 {
    pub _bitfield: u32,
}
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "clfs", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PCLFS_CLIENT_ADVANCE_TAIL_CALLBACK = Option<unsafe extern "system" fn(logfile: *const FILE_OBJECT, targetlsn: *const super::clfs::CLFS_LSN, clientdata: *const core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PCLFS_CLIENT_LFF_HANDLER_COMPLETE_CALLBACK = Option<unsafe extern "system" fn(logfile: *const FILE_OBJECT, operationstatus: super::bcrypt::NTSTATUS, logispinned: bool, clientdata: *const core::ffi::c_void)>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PCLFS_CLIENT_LOG_UNPINNED_CALLBACK = Option<unsafe extern "system" fn(logfile: *const FILE_OBJECT, clientdata: *const core::ffi::c_void)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLFS_MGMT_CLIENT(pub *mut *mut core::ffi::c_void);
impl Default for PCLFS_MGMT_CLIENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "clfs", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PCLFS_MGMT_CLIENT_REGISTRATION = *mut CLFS_MGMT_CLIENT_REGISTRATION;
#[cfg(feature = "clfs")]
pub type PCLFS_MGMT_NOTIFICATION = *mut CLFS_MGMT_NOTIFICATION;
pub type PCLFS_MGMT_NOTIFICATION_TYPE = *mut CLFS_MGMT_NOTIFICATION_TYPE;
pub type PCLFS_MGMT_POLICY = *mut CLFS_MGMT_POLICY;
pub type PCLFS_MGMT_POLICY_TYPE = *mut CLFS_MGMT_POLICY_TYPE;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PCLFS_SET_LOG_SIZE_COMPLETE_CALLBACK = Option<unsafe extern "system" fn(logfile: *const FILE_OBJECT, operationstatus: super::bcrypt::NTSTATUS, clientdata: *const core::ffi::c_void)>;
#[cfg(all(feature = "winnt", feature = "winternl"))]
pub type PCLIENT_ID = *mut super::winternl::CLIENT_ID;
pub const PCMCIABus: INTERFACE_TYPE = 8;
#[cfg(feature = "basetsd")]
pub type PCM_COMPONENT_INFORMATION = *mut CM_COMPONENT_INFORMATION;
pub type PCM_DISK_GEOMETRY_DEVICE_DATA = *mut CM_DISK_GEOMETRY_DEVICE_DATA;
pub type PCM_EISA_FUNCTION_INFORMATION = *mut CM_EISA_FUNCTION_INFORMATION;
pub type PCM_EISA_SLOT_INFORMATION = *mut CM_EISA_SLOT_INFORMATION;
pub type PCM_FLOPPY_DEVICE_DATA = *mut CM_FLOPPY_DEVICE_DATA;
#[cfg(all(feature = "basetsd", feature = "usb"))]
pub type PCM_FULL_RESOURCE_DESCRIPTOR = *mut CM_FULL_RESOURCE_DESCRIPTOR;
pub type PCM_INT13_DRIVE_PARAMETER = *mut CM_INT13_DRIVE_PARAMETER;
pub type PCM_KEYBOARD_DEVICE_DATA = *mut CM_KEYBOARD_DEVICE_DATA;
pub type PCM_MCA_POS_DATA = *mut CM_MCA_POS_DATA;
pub type PCM_MONITOR_DEVICE_DATA = *mut CM_MONITOR_DEVICE_DATA;
#[cfg(all(feature = "basetsd", feature = "usb"))]
pub type PCM_PARTIAL_RESOURCE_DESCRIPTOR = *mut CM_PARTIAL_RESOURCE_DESCRIPTOR;
#[cfg(all(feature = "basetsd", feature = "usb"))]
pub type PCM_PARTIAL_RESOURCE_LIST = *mut CM_PARTIAL_RESOURCE_LIST;
pub type PCM_PNP_BIOS_DEVICE_NODE = *mut CM_PNP_BIOS_DEVICE_NODE;
pub type PCM_PNP_BIOS_INSTALLATION_CHECK = *mut CM_PNP_BIOS_INSTALLATION_CHECK;
#[cfg(all(feature = "basetsd", feature = "usb"))]
pub type PCM_RESOURCE_LIST = *mut CM_RESOURCE_LIST;
pub type PCM_ROM_BLOCK = *mut CM_ROM_BLOCK;
pub type PCM_SCSI_DEVICE_DATA = *mut CM_SCSI_DEVICE_DATA;
pub type PCM_SERIAL_DEVICE_DATA = *mut CM_SERIAL_DEVICE_DATA;
pub type PCM_SONIC_DEVICE_DATA = *mut CM_SONIC_DEVICE_DATA;
pub type PCM_VIDEO_DEVICE_DATA = *mut CM_VIDEO_DEVICE_DATA;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PCONFIGURE_ADAPTER_CHANNEL = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, functionnumber: u32, context: *const core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
pub type PCOUNTED_REASON_CONTEXT = *mut COUNTED_REASON_CONTEXT;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
pub type PCPOOL_CREATE_EXTENDED_PARAMETER = *const POOL_CREATE_EXTENDED_PARAMETER;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
pub type PCPOOL_CREATE_EXTENDED_PARAMS = *const POOL_CREATE_EXTENDED_PARAMS;
#[cfg(feature = "winnt")]
pub type PCPOOL_EXTENDED_PARAMETER = *const POOL_EXTENDED_PARAMETER;
#[cfg(feature = "bcrypt")]
pub type PCRASHDUMP_FUNCTIONS_INTERFACE = *mut CRASHDUMP_FUNCTIONS_INTERFACE;
#[cfg(feature = "bcrypt")]
pub type PCRASHDUMP_POWER_ON = Option<unsafe extern "system" fn(context: *const core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PCREATE_COMMON_BUFFER_FROM_MDL = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, mdl: *const MDL, extendedconfigs: *const DMA_COMMON_BUFFER_EXTENDED_CONFIGURATION, extendedconfigscount: u32, logicaladdress: *mut i64) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
pub type PCW_CALLBACK = Option<unsafe extern "system" fn(r#type: PCW_CALLBACK_TYPE, info: *const PCW_CALLBACK_INFORMATION, context: *const core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub union PCW_CALLBACK_INFORMATION {
    pub AddCounter: PCW_COUNTER_INFORMATION,
    pub RemoveCounter: PCW_COUNTER_INFORMATION,
    pub EnumerateInstances: PCW_MASK_INFORMATION,
    pub CollectData: PCW_MASK_INFORMATION,
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
impl Default for PCW_CALLBACK_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PCW_CALLBACK_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PCW_COUNTER_DESCRIPTOR {
    pub Id: u16,
    pub StructIndex: u16,
    pub Offset: u16,
    pub Size: u16,
}
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PCW_COUNTER_INFORMATION {
    pub CounterMask: u64,
    pub InstanceMask: super::winternl::PCUNICODE_STRING,
}
pub const PCW_CURRENT_VERSION: u32 = 512;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PCW_DATA {
    pub Data: *const core::ffi::c_void,
    pub Size: u32,
}
impl Default for PCW_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PCW_MASK_INFORMATION {
    pub CounterMask: u64,
    pub InstanceMask: super::winternl::PCUNICODE_STRING,
    pub InstanceId: u32,
    pub CollectMultiple: bool,
    pub Buffer: PPCW_BUFFER,
    pub CancelEvent: PKEVENT,
}
pub type PCW_REGISTRATION_FLAGS = i32;
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug)]
pub struct PCW_REGISTRATION_INFORMATION {
    pub Version: u32,
    pub Name: super::winternl::PCUNICODE_STRING,
    pub CounterCount: u32,
    pub Counters: PPCW_COUNTER_DESCRIPTOR,
    pub Callback: PPCW_CALLBACK,
    pub CallbackContext: *mut core::ffi::c_void,
    pub Flags: PCW_REGISTRATION_FLAGS,
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
impl Default for PCW_REGISTRATION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PCW_VERSION_1: u32 = 256;
pub const PCW_VERSION_2: u32 = 512;
#[cfg(feature = "bcrypt")]
pub type PD3COLD_AUX_POWER_AND_TIMING_INTERFACE = *mut D3COLD_AUX_POWER_AND_TIMING_INTERFACE;
pub type PD3COLD_LAST_TRANSITION_STATUS = *mut D3COLD_LAST_TRANSITION_STATUS;
#[cfg(feature = "bcrypt")]
pub type PD3COLD_REQUEST_AUX_POWER = *mut D3COLD_REQUEST_AUX_POWER;
pub type PD3COLD_REQUEST_CORE_POWER_RAIL = *mut D3COLD_REQUEST_CORE_POWER_RAIL;
#[cfg(feature = "bcrypt")]
pub type PD3COLD_REQUEST_PERST_DELAY = *mut D3COLD_REQUEST_PERST_DELAY;
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
pub type PD3COLD_SUPPORT_INTERFACE = *mut D3COLD_SUPPORT_INTERFACE;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
pub type PDEBUG_PRINT_CALLBACK = Option<unsafe extern "system" fn(output: *const super::ntsecapi::STRING, componentid: u32, level: u32)>;
#[cfg(feature = "bcrypt")]
pub type PDEVICE_BUS_SPECIFIC_RESET_HANDLER = *mut DEVICE_BUS_SPECIFIC_RESET_HANDLER;
pub type PDEVICE_BUS_SPECIFIC_RESET_INFO = *mut DEVICE_BUS_SPECIFIC_RESET_INFO;
pub type PDEVICE_BUS_SPECIFIC_RESET_TYPE = *mut DEVICE_BUS_SPECIFIC_RESET_TYPE;
#[cfg(feature = "winnt")]
pub type PDEVICE_CAPABILITIES = *mut DEVICE_CAPABILITIES;
pub type PDEVICE_CHANGE_COMPLETE_CALLBACK = *mut DEVICE_CHANGE_COMPLETE_CALLBACK;
#[cfg(feature = "usb")]
pub type PDEVICE_DESCRIPTION = *mut DEVICE_DESCRIPTION;
pub type PDEVICE_DIRECTORY_TYPE = *mut DEVICE_DIRECTORY_TYPE;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PDEVICE_FAULT_CONFIGURATION = *mut DEVICE_FAULT_CONFIGURATION;
pub type PDEVICE_FLAGS = *mut DEVICE_FLAGS;
pub type PDEVICE_INSTALL_STATE = *mut DEVICE_INSTALL_STATE;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
pub type PDEVICE_INTERFACE_CHANGE_NOTIFICATION = *mut DEVICE_INTERFACE_CHANGE_NOTIFICATION;
pub type PDEVICE_NOTIFY_CALLBACK = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void, param1: u32)>;
pub type PDEVICE_NOTIFY_CALLBACK2 = Option<unsafe extern "system" fn(notificationcontext: *mut core::ffi::c_void, notifycode: u32)>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PDEVICE_OBJECT = *mut DEVICE_OBJECT;
#[cfg(feature = "bcrypt")]
pub type PDEVICE_QUERY_BUS_SPECIFIC_RESET_HANDLER = *mut DEVICE_QUERY_BUS_SPECIFIC_RESET_HANDLER;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PDEVICE_RELATIONS = *mut DEVICE_RELATIONS;
pub type PDEVICE_RELATION_TYPE = *mut DEVICE_RELATION_TYPE;
pub type PDEVICE_REMOVAL_POLICY = *mut DEVICE_REMOVAL_POLICY;
#[cfg(feature = "bcrypt")]
pub type PDEVICE_RESET_COMPLETION = *mut DEVICE_RESET_COMPLETION;
#[cfg(feature = "bcrypt")]
pub type PDEVICE_RESET_HANDLER = *mut DEVICE_RESET_HANDLER;
#[cfg(feature = "bcrypt")]
pub type PDEVICE_RESET_INTERFACE_STANDARD = *mut DEVICE_RESET_INTERFACE_STANDARD;
pub type PDEVICE_RESET_STATUS_FLAGS = *mut DEVICE_RESET_STATUS_FLAGS;
pub type PDEVICE_TEXT_TYPE = *mut DEVICE_TEXT_TYPE;
pub type PDEVICE_WAKE_DEPTH = *mut DEVICE_WAKE_DEPTH;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PDEVOBJ_EXTENSION = *mut DEVOBJ_EXTENSION;
pub type PDIRECTORY_NOTIFY_INFORMATION_CLASS = *mut DIRECTORY_NOTIFY_INFORMATION_CLASS;
#[cfg(feature = "winnt")]
pub type PDISPATCHER_HEADER = *mut DISPATCHER_HEADER;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PDMA_ADAPTER = *mut DMA_ADAPTER;
#[cfg(feature = "usb")]
pub type PDMA_ADAPTER_INFO = *mut DMA_ADAPTER_INFO;
#[cfg(feature = "usb")]
pub type PDMA_ADAPTER_INFO_CRASHDUMP = *mut DMA_ADAPTER_INFO_CRASHDUMP;
pub type PDMA_ADAPTER_INFO_V1 = *mut DMA_ADAPTER_INFO_V1;
#[cfg(feature = "usb")]
pub type PDMA_COMMON_BUFFER_EXTENDED_CONFIGURATION = *mut DMA_COMMON_BUFFER_EXTENDED_CONFIGURATION;
pub type PDMA_COMMON_BUFFER_EXTENDED_CONFIGURATION_ACCESS_TYPE = *mut DMA_COMMON_BUFFER_EXTENDED_CONFIGURATION_ACCESS_TYPE;
pub type PDMA_COMMON_BUFFER_EXTENDED_CONFIGURATION_TYPE = *mut DMA_COMMON_BUFFER_EXTENDED_CONFIGURATION_TYPE;
pub type PDMA_COMMON_BUFFER_VECTOR = *mut DMA_COMMON_BUFFER_VECTOR;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PDMA_COMPLETION_ROUTINE = *mut DMA_COMPLETION_ROUTINE;
pub type PDMA_FEATURE_QUERY_STATUS = *mut DMA_FEATURE_QUERY_STATUS;
pub type PDMA_FEATURE_TYPE = *mut DMA_FEATURE_TYPE;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PDMA_IOMMU_INTERFACE = *mut DMA_IOMMU_INTERFACE;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PDMA_IOMMU_INTERFACE_EX = *mut DMA_IOMMU_INTERFACE_EX;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PDMA_IOMMU_INTERFACE_V1 = *mut DMA_IOMMU_INTERFACE_V1;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PDMA_IOMMU_INTERFACE_V2 = *mut DMA_IOMMU_INTERFACE_V2;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PDMA_IOMMU_INTERFACE_V3 = *mut DMA_IOMMU_INTERFACE_V3;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PDMA_OPERATIONS = *mut DMA_OPERATIONS;
pub type PDMA_SPEED = *mut DMA_SPEED;
pub type PDMA_TRANSFER_INFO = *mut DMA_TRANSFER_INFO;
pub type PDMA_TRANSFER_INFO_V1 = *mut DMA_TRANSFER_INFO_V1;
pub type PDMA_TRANSFER_INFO_V2 = *mut DMA_TRANSFER_INFO_V2;
pub type PDMA_WIDTH = *mut DMA_WIDTH;
#[cfg(feature = "usb")]
pub type PDOMAIN_CONFIGURATION = *mut DOMAIN_CONFIGURATION;
pub type PDOMAIN_CONFIGURATION_ARCH = *mut DOMAIN_CONFIGURATION_ARCH;
#[cfg(feature = "usb")]
pub type PDOMAIN_CONFIGURATION_ARM64 = *mut DOMAIN_CONFIGURATION_ARM64;
#[cfg(feature = "usb")]
pub type PDOMAIN_CONFIGURATION_X64 = *mut DOMAIN_CONFIGURATION_X64;
pub type PDPC_WATCHDOG_GLOBAL_TRIAGE_BLOCK = *mut DPC_WATCHDOG_GLOBAL_TRIAGE_BLOCK;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PDRIVER_ADD_DEVICE = *mut DRIVER_ADD_DEVICE;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PDRIVER_CANCEL = *mut DRIVER_CANCEL;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PDRIVER_CONTROL = *mut DRIVER_CONTROL;
pub type PDRIVER_DIRECTORY_TYPE = *mut DRIVER_DIRECTORY_TYPE;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PDRIVER_DISPATCH = *mut DRIVER_DISPATCH;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PDRIVER_DISPATCH_PAGED = *mut DRIVER_DISPATCH_PAGED;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PDRIVER_EXTENSION = *mut DRIVER_EXTENSION;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PDRIVER_INITIALIZE = *mut DRIVER_INITIALIZE;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PDRIVER_LIST_CONTROL = *mut DRIVER_LIST_CONTROL;
#[cfg(feature = "bcrypt")]
pub type PDRIVER_NOTIFICATION_CALLBACK_ROUTINE = *mut DRIVER_NOTIFICATION_CALLBACK_ROUTINE;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PDRIVER_OBJECT = *mut DRIVER_OBJECT;
pub type PDRIVER_PROXY_ENDPOINT_FUNCTION = *mut DRIVER_PROXY_ENDPOINT_FUNCTION;
pub type PDRIVER_PROXY_ENDPOINT_FUNCTION_ID = *mut u32;
pub type PDRIVER_PROXY_ENDPOINT_INFORMATION = *mut DRIVER_PROXY_ENDPOINT_INFORMATION;
pub type PDRIVER_PROXY_EXTENSION = *mut _DRIVER_PROXY_EXTENSION;
pub type PDRIVER_PROXY_EXTENSION_CREATION_FLAGS = *mut DRIVER_PROXY_EXTENSION_CREATION_FLAGS;
pub type PDRIVER_PROXY_FEATURE_FLAGS = *mut DRIVER_PROXY_FEATURE_FLAGS;
#[cfg(feature = "bcrypt")]
pub type PDRIVER_PROXY_HOTSWAP_CALLBACK = *mut DRIVER_PROXY_HOTSWAP_CALLBACK;
pub type PDRIVER_PROXY_HOTSWAP_CALLBACK_PHASE = *mut DRIVER_PROXY_HOTSWAP_CALLBACK_PHASE;
#[cfg(feature = "bcrypt")]
pub type PDRIVER_PROXY_HOTSWAP_WORKER_ROUTINE = *mut DRIVER_PROXY_HOTSWAP_WORKER_ROUTINE;
#[cfg(all(feature = "bcrypt", feature = "ntdef", feature = "winnt"))]
pub type PDRIVER_PROXY_HOTSWAP_WORKER_ROUTINE_START_CONTEXT = *mut DRIVER_PROXY_HOTSWAP_WORKER_ROUTINE_START_CONTEXT;
#[cfg(feature = "bcrypt")]
pub type PDRIVER_PROXY_REGISTER_CALLBACK = *mut DRIVER_PROXY_HOTSWAP_CALLBACK;
pub type PDRIVER_PROXY_REGISTER_CALLBACK_PHASE = *mut DRIVER_PROXY_HOTSWAP_CALLBACK_PHASE;
pub type PDRIVER_PROXY_VERSION = *mut DRIVER_PROXY_VERSION;
pub type PDRIVER_PROXY_WRAPPED_ENDPOINT_FUNCTION = *mut DRIVER_PROXY_ENDPOINT_FUNCTION;
pub type PDRIVER_REGKEY_TYPE = *mut DRIVER_REGKEY_TYPE;
pub type PDRIVER_RUNTIME_INIT_FLAGS = *mut DRIVER_RUNTIME_INIT_FLAGS;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PDRIVER_STARTIO = *mut DRIVER_STARTIO;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PDRIVER_UNLOAD = *mut DRIVER_UNLOAD;
pub type PEISA_DMA_CONFIGURATION = *mut EISA_DMA_CONFIGURATION;
pub type PEISA_IRQ_CONFIGURATION = *mut EISA_IRQ_CONFIGURATION;
pub type PEISA_IRQ_DESCRIPTOR = *mut EISA_IRQ_DESCRIPTOR;
pub type PEISA_MEMORY_CONFIGURATION = *mut EISA_MEMORY_CONFIGURATION;
pub type PEISA_MEMORY_TYPE = *mut EISA_MEMORY_TYPE;
pub type PEISA_PORT_CONFIGURATION = *mut EISA_PORT_CONFIGURATION;
pub type PEISA_PORT_DESCRIPTOR = *mut EISA_PORT_DESCRIPTOR;
#[cfg(feature = "bcrypt")]
pub type PENABLE_VIRTUALIZATION = *mut ENABLE_VIRTUALIZATION;
pub type PEPROCESS = *mut _KPROCESS;
#[cfg(feature = "winnt")]
pub type PERESOURCE = *mut ERESOURCE;
pub type PERESOURCE_THREAD = *mut ERESOURCE_THREAD;
pub type PETHREAD = *mut _KTHREAD;
#[cfg(feature = "evntprov")]
pub type PETWENABLECALLBACK = Option<unsafe extern "system" fn(sourceid: *const windows_core::GUID, controlcode: u32, level: u8, matchanykeyword: u64, matchallkeyword: u64, filterdata: *const super::evntprov::EVENT_FILTER_DESCRIPTOR, callbackcontext: *mut core::ffi::c_void)>;
pub type PETW_TRACE_SESSION_SETTINGS = *mut ETW_TRACE_SESSION_SETTINGS;
pub type PEXTENDED_CREATE_DUAL_OPLOCK_KEYS = *mut EXTENDED_CREATE_DUAL_OPLOCK_KEYS;
pub type PEXTENDED_CREATE_INFORMATION = *mut EXTENDED_CREATE_INFORMATION;
pub type PEXTENDED_CREATE_INFORMATION_32 = *mut EXTENDED_CREATE_INFORMATION_32;
pub type PEXT_CALLBACK = *mut EXT_CALLBACK;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PEXT_CANCEL_PARAMETERS(pub *mut core::ffi::c_void);
impl Default for PEXT_CANCEL_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PEXT_DELETE_CALLBACK = *mut EXT_DELETE_CALLBACK;
pub type PEXT_DELETE_PARAMETERS = *mut EXT_DELETE_PARAMETERS;
pub type PEXT_SET_PARAMETERS = *mut EXT_SET_PARAMETERS;
#[cfg(feature = "bcrypt")]
pub type PEX_CALLBACK_FUNCTION = *mut EX_CALLBACK_FUNCTION;
pub type PEX_RCU_FREE_POOL_CONTEXT = *mut EX_RCU_FREE_POOL_CONTEXT;
pub type PEX_RUNDOWN_REF = *mut EX_RUNDOWN_REF;
pub type PEX_RUNDOWN_REF_CACHE_AWARE = *mut _EX_RUNDOWN_REF_CACHE_AWARE;
pub type PEX_SPIN_LOCK = *mut i32;
pub type PEX_TIMER = *mut _EX_TIMER;
pub const PFAControl: NPEM_CONTROL_STANDARD_CONTROL_BIT = 6;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PFAST_IO_ACQUIRE_FILE = *mut FAST_IO_ACQUIRE_FILE;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PFAST_IO_ACQUIRE_FOR_CCFLUSH = *mut FAST_IO_ACQUIRE_FOR_CCFLUSH;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PFAST_IO_ACQUIRE_FOR_MOD_WRITE = *mut FAST_IO_ACQUIRE_FOR_MOD_WRITE;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PFAST_IO_CHECK_IF_POSSIBLE = *mut FAST_IO_CHECK_IF_POSSIBLE;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PFAST_IO_DETACH_DEVICE = *mut FAST_IO_DETACH_DEVICE;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PFAST_IO_DEVICE_CONTROL = *mut FAST_IO_DEVICE_CONTROL;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PFAST_IO_DISPATCH = *mut FAST_IO_DISPATCH;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PFAST_IO_LOCK = *mut FAST_IO_LOCK;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PFAST_IO_MDL_READ = *mut FAST_IO_MDL_READ;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PFAST_IO_MDL_READ_COMPLETE = *mut FAST_IO_MDL_READ_COMPLETE;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PFAST_IO_MDL_READ_COMPLETE_COMPRESSED = *mut FAST_IO_MDL_READ_COMPLETE_COMPRESSED;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PFAST_IO_MDL_WRITE_COMPLETE = *mut FAST_IO_MDL_WRITE_COMPLETE;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PFAST_IO_MDL_WRITE_COMPLETE_COMPRESSED = *mut FAST_IO_MDL_WRITE_COMPLETE_COMPRESSED;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PFAST_IO_PREPARE_MDL_WRITE = *mut FAST_IO_PREPARE_MDL_WRITE;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PFAST_IO_QUERY_BASIC_INFO = *mut FAST_IO_QUERY_BASIC_INFO;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PFAST_IO_QUERY_NETWORK_OPEN_INFO = *mut FAST_IO_QUERY_NETWORK_OPEN_INFO;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PFAST_IO_QUERY_OPEN = *mut FAST_IO_QUERY_OPEN;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PFAST_IO_QUERY_STANDARD_INFO = *mut FAST_IO_QUERY_STANDARD_INFO;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PFAST_IO_READ = *mut FAST_IO_READ;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PFAST_IO_READ_COMPRESSED = *mut FAST_IO_READ_COMPRESSED;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PFAST_IO_RELEASE_FILE = *mut FAST_IO_RELEASE_FILE;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PFAST_IO_RELEASE_FOR_CCFLUSH = *mut FAST_IO_RELEASE_FOR_CCFLUSH;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PFAST_IO_RELEASE_FOR_MOD_WRITE = *mut FAST_IO_RELEASE_FOR_MOD_WRITE;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PFAST_IO_UNLOCK_ALL = *mut FAST_IO_UNLOCK_ALL;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PFAST_IO_UNLOCK_ALL_BY_KEY = *mut FAST_IO_UNLOCK_ALL_BY_KEY;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PFAST_IO_UNLOCK_SINGLE = *mut FAST_IO_UNLOCK_SINGLE;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PFAST_IO_WRITE = *mut FAST_IO_WRITE;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PFAST_IO_WRITE_COMPRESSED = *mut FAST_IO_WRITE_COMPRESSED;
#[cfg(feature = "winnt")]
pub type PFAST_MUTEX = *mut FAST_MUTEX;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PFAULT_INFORMATION = *mut FAULT_INFORMATION;
pub type PFAULT_INFORMATION_ARCH = *mut FAULT_INFORMATION_ARCH;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PFAULT_INFORMATION_ARM64 = *mut FAULT_INFORMATION_ARM64;
pub type PFAULT_INFORMATION_ARM64_FLAGS = *mut FAULT_INFORMATION_ARM64_FLAGS;
pub type PFAULT_INFORMATION_ARM64_TYPE = *mut FAULT_INFORMATION_ARM64_TYPE;
pub type PFAULT_INFORMATION_X64 = *mut FAULT_INFORMATION_X64;
pub type PFAULT_INFORMATION_X64_FLAGS = *mut FAULT_INFORMATION_X64_FLAGS;
pub type PFAULT_INFORMATION_X64_TYPE = *mut FAULT_INFORMATION_ARM64_TYPE;
pub type PFILE_BASIC_INFORMATION = *mut FILE_BASIC_INFORMATION;
pub type PFILE_FS_DEVICE_INFORMATION = *mut FILE_FS_DEVICE_INFORMATION;
pub type PFILE_FULL_EA_INFORMATION = *mut FILE_FULL_EA_INFORMATION;
#[cfg(all(feature = "ntifs", feature = "winnt"))]
pub type PFILE_GET_QUOTA_INFORMATION = *mut super::ntifs::FILE_GET_QUOTA_INFORMATION;
#[cfg(feature = "winternl")]
pub type PFILE_INFORMATION_CLASS = *mut super::winternl::FILE_INFORMATION_CLASS;
#[cfg(feature = "minwindef")]
pub type PFILE_IOSTATUSBLOCK_RANGE_INFORMATION = *mut FILE_IOSTATUSBLOCK_RANGE_INFORMATION;
#[cfg(all(feature = "bcrypt", feature = "winternl"))]
pub type PFILE_IO_COMPLETION_INFORMATION = *mut FILE_IO_COMPLETION_INFORMATION;
pub type PFILE_IO_COMPLETION_NOTIFICATION_INFORMATION = *mut FILE_IO_COMPLETION_NOTIFICATION_INFORMATION;
pub type PFILE_IO_PRIORITY_HINT_INFORMATION = *mut FILE_IO_PRIORITY_HINT_INFORMATION;
pub type PFILE_IO_PRIORITY_HINT_INFORMATION_EX = *mut FILE_IO_PRIORITY_HINT_INFORMATION_EX;
pub type PFILE_IS_REMOTE_DEVICE_INFORMATION = *mut FILE_IS_REMOTE_DEVICE_INFORMATION;
pub type PFILE_MEMORY_PARTITION_INFORMATION = *mut FILE_MEMORY_PARTITION_INFORMATION;
pub type PFILE_NETWORK_OPEN_INFORMATION = *mut FILE_NETWORK_OPEN_INFORMATION;
pub type PFILE_NUMA_NODE_INFORMATION = *mut FILE_NUMA_NODE_INFORMATION;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PFILE_OBJECT = *mut FILE_OBJECT;
pub type PFILE_POSITION_INFORMATION = *mut FILE_POSITION_INFORMATION;
pub type PFILE_PROCESS_IDS_USING_FILE_INFORMATION = *mut FILE_PROCESS_IDS_USING_FILE_INFORMATION;
pub type PFILE_SFIO_RESERVE_INFORMATION = *mut FILE_SFIO_RESERVE_INFORMATION;
pub type PFILE_SFIO_VOLUME_INFORMATION = *mut FILE_SFIO_VOLUME_INFORMATION;
pub type PFILE_STANDARD_INFORMATION = *mut FILE_STANDARD_INFORMATION;
pub type PFILE_STANDARD_INFORMATION_EX = *mut FILE_STANDARD_INFORMATION_EX;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PFLUSH_ADAPTER_BUFFERS = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, mdl: *const MDL, mapregisterbase: *const core::ffi::c_void, currentva: *const core::ffi::c_void, length: u32, writetodevice: bool) -> bool>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PFLUSH_ADAPTER_BUFFERS_EX = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, mdl: *const MDL, mapregisterbase: *const core::ffi::c_void, offset: u64, length: u32, writetodevice: bool) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PFLUSH_DMA_BUFFER = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, mdl: *const MDL, readoperation: bool) -> super::bcrypt::NTSTATUS>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PFN_COUNT(pub u32);
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
pub type PFN_NT_COMMIT_TRANSACTION = Option<unsafe extern "system" fn(transactionhandle: super::winnt::HANDLE, wait: bool) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
pub type PFN_NT_CREATE_TRANSACTION = Option<unsafe extern "system" fn(transactionhandle: *mut super::winnt::HANDLE, desiredaccess: super::winnt::ACCESS_MASK, objectattributes: *const super::d3dkmthk::OBJECT_ATTRIBUTES, uow: *const windows_core::GUID, tmhandle: super::winnt::HANDLE, createoptions: u32, isolationlevel: u32, isolationflags: u32, timeout: *const i64, description: *const super::ntsecapi::UNICODE_STRING) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
pub type PFN_NT_OPEN_TRANSACTION = Option<unsafe extern "system" fn(transactionhandle: *mut super::winnt::HANDLE, desiredaccess: super::winnt::ACCESS_MASK, objectattributes: *const super::d3dkmthk::OBJECT_ATTRIBUTES, uow: *const windows_core::GUID, tmhandle: super::winnt::HANDLE) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
pub type PFN_NT_QUERY_INFORMATION_TRANSACTION = Option<unsafe extern "system" fn(transactionhandle: super::winnt::HANDLE, transactioninformationclass: super::winnt::TRANSACTION_INFORMATION_CLASS, transactioninformation: *mut core::ffi::c_void, transactioninformationlength: u32, returnlength: *mut u32) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
pub type PFN_NT_ROLLBACK_TRANSACTION = Option<unsafe extern "system" fn(transactionhandle: super::winnt::HANDLE, wait: bool) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
pub type PFN_NT_SET_INFORMATION_TRANSACTION = Option<unsafe extern "system" fn(transactionhandle: super::winnt::HANDLE, transactioninformationclass: super::winnt::TRANSACTION_INFORMATION_CLASS, transactioninformation: *const core::ffi::c_void, transactioninformationlength: u32) -> super::bcrypt::NTSTATUS>;
#[cfg(target_arch = "x86")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PFN_NUMBER(pub u32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PFN_NUMBER(pub u64);
pub type PFN_RTL_IS_NTDDI_VERSION_AVAILABLE = Option<unsafe extern "system" fn(version: u32) -> bool>;
pub type PFN_RTL_IS_SERVICE_PACK_VERSION_INSTALLED = Option<unsafe extern "system" fn(version: u32) -> bool>;
pub type PFPGA_BUS_SCAN = *mut FPGA_BUS_SCAN;
#[cfg(feature = "bcrypt")]
pub type PFPGA_CONTROL_CONFIG_SPACE = *mut FPGA_CONTROL_CONFIG_SPACE;
#[cfg(feature = "bcrypt")]
pub type PFPGA_CONTROL_ERROR_REPORTING = *mut FPGA_CONTROL_ERROR_REPORTING;
#[cfg(feature = "bcrypt")]
pub type PFPGA_CONTROL_INTERFACE = *mut FPGA_CONTROL_INTERFACE;
#[cfg(feature = "bcrypt")]
pub type PFPGA_CONTROL_LINK = *mut FPGA_CONTROL_LINK;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PFREE_ADAPTER_CHANNEL = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER)>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PFREE_ADAPTER_OBJECT = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, allocationaction: IO_ALLOCATION_ACTION)>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PFREE_COMMON_BUFFER = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, length: u32, logicaladdress: super::usb::PHYSICAL_ADDRESS, virtualaddress: *const core::ffi::c_void, cacheenabled: bool)>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PFREE_COMMON_BUFFER_FROM_VECTOR = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, vector: *const DMA_COMMON_BUFFER_VECTOR, index: u32)>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PFREE_COMMON_BUFFER_VECTOR = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, vector: *const DMA_COMMON_BUFFER_VECTOR)>;
pub type PFREE_FUNCTION = *mut FREE_FUNCTION;
#[cfg(feature = "winnt")]
pub type PFREE_FUNCTION_EX = *mut FREE_FUNCTION_EX;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PFREE_MAP_REGISTERS = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, mapregisterbase: *mut core::ffi::c_void, numberofmapregisters: u32)>;
pub type PFS_INFORMATION_CLASS = *mut FS_INFORMATION_CLASS;
#[cfg(feature = "bcrypt")]
pub type PFUNCTION_LEVEL_DEVICE_RESET_PARAMETERS = *mut FUNCTION_LEVEL_DEVICE_RESET_PARAMETERS;
#[cfg(feature = "winnt")]
pub type PGENERAL_LOOKASIDE = *mut GENERAL_LOOKASIDE;
#[cfg(feature = "winnt")]
pub type PGENERAL_LOOKASIDE_POOL = *mut GENERAL_LOOKASIDE_POOL;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PGET_COMMON_BUFFER_FROM_VECTOR_BY_INDEX = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, vector: *const DMA_COMMON_BUFFER_VECTOR, index: u32, virtualaddressout: *mut *mut core::ffi::c_void, logicaladdressout: *mut i64)>;
#[cfg(feature = "bcrypt")]
pub type PGET_D3COLD_CAPABILITY = *mut GET_D3COLD_CAPABILITY;
pub type PGET_D3COLD_LAST_TRANSITION_STATUS = *mut GET_D3COLD_LAST_TRANSITION_STATUS;
#[cfg(feature = "bcrypt")]
pub type PGET_DEVICE_RESET_STATUS = *mut GET_DEVICE_RESET_STATUS;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PGET_DMA_ADAPTER = *mut GET_DMA_ADAPTER;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PGET_DMA_ADAPTER_INFO = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, adapterinfo: *mut DMA_ADAPTER_INFO) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PGET_DMA_ALIGNMENT = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER) -> u32>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PGET_DMA_DOMAIN = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER) -> super::winnt::HANDLE>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PGET_DMA_TRANSFER_INFO = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, mdl: *const MDL, offset: u64, length: u32, writeonly: bool, transferinfo: *mut DMA_TRANSFER_INFO) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "bcrypt")]
pub type PGET_DOE_PREVIOUS_RESPONSE = *mut GET_DOE_PREVIOUS_RESPONSE;
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
pub type PGET_IDLE_WAKE_INFO = *mut GET_IDLE_WAKE_INFO;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PGET_SCATTER_GATHER_LIST = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, deviceobject: *const DEVICE_OBJECT, mdl: *const MDL, currentva: *const core::ffi::c_void, length: u32, executionroutine: PDRIVER_LIST_CONTROL, context: *const core::ffi::c_void, writetodevice: bool) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PGET_SCATTER_GATHER_LIST_EX = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, deviceobject: *const DEVICE_OBJECT, dmatransfercontext: *const core::ffi::c_void, mdl: *const MDL, offset: u64, length: u32, flags: u32, executionroutine: PDRIVER_LIST_CONTROL, context: *const core::ffi::c_void, writetodevice: bool, dmacompletionroutine: PDMA_COMPLETION_ROUTINE, completioncontext: *const core::ffi::c_void, scattergatherlist: *mut PSCATTER_GATHER_LIST) -> super::bcrypt::NTSTATUS>;
pub type PGET_SDEV_IDENTIFIER = *mut GET_SDEV_IDENTIFIER;
pub type PGET_SET_DEVICE_DATA = *mut GET_SET_DEVICE_DATA;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "usb"))]
pub type PGET_UPDATED_BUS_RESOURCE = *mut GET_UPDATED_BUS_RESOURCE;
pub type PGET_VIRTUAL_DEVICE_DATA = *mut GET_VIRTUAL_DEVICE_DATA;
#[cfg(feature = "bcrypt")]
pub type PGET_VIRTUAL_DEVICE_LOCATION = *mut GET_VIRTUAL_DEVICE_LOCATION;
pub type PGET_VIRTUAL_DEVICE_RESOURCES = *mut GET_VIRTUAL_DEVICE_RESOURCES;
#[cfg(feature = "bcrypt")]
pub type PGET_VIRTUAL_FUNCTION_PROBED_BARS = *mut GET_VIRTUAL_FUNCTION_PROBED_BARS;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PGPE_CLEAR_STATUS = Option<unsafe extern "system" fn(param0: *mut DEVICE_OBJECT, param1: *mut core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "bcrypt")]
pub type PGPE_CLEAR_STATUS2 = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, objectcontext: *mut core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PGPE_CONNECT_VECTOR = Option<unsafe extern "system" fn(param0: *mut DEVICE_OBJECT, param1: u32, param2: KINTERRUPT_MODE, param3: bool, param4: PGPE_SERVICE_ROUTINE, param5: *mut core::ffi::c_void, param6: *mut core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "bcrypt")]
pub type PGPE_CONNECT_VECTOR2 = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, gpenumber: u32, mode: KINTERRUPT_MODE, shareable: bool, serviceroutine: PGPE_SERVICE_ROUTINE, servicecontext: *mut core::ffi::c_void, objectcontext: *mut *mut core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PGPE_DISABLE_EVENT = Option<unsafe extern "system" fn(param0: *mut DEVICE_OBJECT, param1: *mut core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "bcrypt")]
pub type PGPE_DISABLE_EVENT2 = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, objectcontext: *mut core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "bcrypt")]
pub type PGPE_DISCONNECT_VECTOR = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "bcrypt")]
pub type PGPE_DISCONNECT_VECTOR2 = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, objectcontext: *mut core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PGPE_ENABLE_EVENT = Option<unsafe extern "system" fn(param0: *mut DEVICE_OBJECT, param1: *mut core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "bcrypt")]
pub type PGPE_ENABLE_EVENT2 = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, objectcontext: *mut core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
pub type PGPE_SERVICE_ROUTINE = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void, param1: *mut core::ffi::c_void) -> bool>;
pub type PGPE_SERVICE_ROUTINE2 = Option<unsafe extern "system" fn(objectcontext: *mut core::ffi::c_void, servicecontext: *mut core::ffi::c_void) -> bool>;
pub type PHWPROFILE_CHANGE_NOTIFICATION = *mut HWPROFILE_CHANGE_NOTIFICATION;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PINITIALIZE_DMA_TRANSFER_CONTEXT = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, dmatransfercontext: *mut core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "winnt")]
pub type PINITIAL_PRIVILEGE_SET = *mut INITIAL_PRIVILEGE_SET;
pub type PINPUT_MAPPING_ELEMENT = *mut INPUT_MAPPING_ELEMENT;
#[cfg(target_arch = "x86")]
pub type PINTEL_CACHE_INFO_EAX = *mut INTEL_CACHE_INFO_EAX;
#[cfg(target_arch = "x86")]
pub type PINTEL_CACHE_INFO_EBX = *mut INTEL_CACHE_INFO_EBX;
pub type PINTERFACE = *mut INTERFACE;
pub type PINTERFACE_DEREFERENCE = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void)>;
pub type PINTERFACE_REFERENCE = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void)>;
pub type PINTERFACE_TYPE = *mut INTERFACE_TYPE;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PIOMMU_DEVICE_CREATE = *mut IOMMU_DEVICE_CREATE;
#[cfg(feature = "winnt")]
pub type PIOMMU_DEVICE_CREATION_CONFIGURATION = *mut IOMMU_DEVICE_CREATION_CONFIGURATION;
pub type PIOMMU_DEVICE_CREATION_CONFIGURATION_ACPI = *mut IOMMU_DEVICE_CREATION_CONFIGURATION_ACPI;
pub type PIOMMU_DEVICE_CREATION_CONFIGURATION_PASID = *mut IOMMU_DEVICE_CREATION_CONFIGURATION_PASID;
pub type PIOMMU_DEVICE_CREATION_CONFIGURATION_TYPE = *mut IOMMU_DEVICE_CREATION_CONFIGURATION_TYPE;
#[cfg(feature = "bcrypt")]
pub type PIOMMU_DEVICE_DELETE = *mut IOMMU_DEVICE_DELETE;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PIOMMU_DEVICE_FAULT_HANDLER = *mut IOMMU_DEVICE_FAULT_HANDLER;
pub type PIOMMU_DEVICE_QUERY_DOMAIN_TYPES = *mut IOMMU_DEVICE_QUERY_DOMAIN_TYPES;
#[cfg(feature = "bcrypt")]
pub type PIOMMU_DEVICE_QUERY_INFORMATION = *mut IOMMU_DEVICE_QUERY_INFORMATION;
pub type PIOMMU_DMA_DEVICE = *mut IOMMU_DMA_DEVICE;
pub type PIOMMU_DMA_DEVICE_INFORMATION = *mut IOMMU_DMA_DEVICE_INFORMATION;
pub type PIOMMU_DMA_DOMAIN = *mut IOMMU_DMA_DOMAIN;
pub type PIOMMU_DMA_DOMAIN_CREATION_FLAGS = *mut IOMMU_DMA_DOMAIN_CREATION_FLAGS;
pub type PIOMMU_DMA_DOMAIN_TYPE = *mut IOMMU_DMA_DOMAIN_TYPE;
pub type PIOMMU_DMA_LOGICAL_ADDRESS = *mut u64;
pub type PIOMMU_DMA_LOGICAL_ADDRESS_TOKEN = *mut IOMMU_DMA_LOGICAL_ADDRESS_TOKEN;
pub type PIOMMU_DMA_LOGICAL_ADDRESS_TOKEN_MAPPED_SEGMENT = *mut IOMMU_DMA_LOGICAL_ADDRESS_TOKEN_MAPPED_SEGMENT;
pub type PIOMMU_DMA_LOGICAL_ALLOCATOR_CONFIG = *mut IOMMU_DMA_LOGICAL_ALLOCATOR_CONFIG;
pub type PIOMMU_DMA_LOGICAL_ALLOCATOR_TYPE = *mut IOMMU_DMA_LOGICAL_ALLOCATOR_TYPE;
pub type PIOMMU_DMA_PASID_DEVICE = *mut IOMMU_DMA_PASID_DEVICE;
pub type PIOMMU_DMA_RESERVED_REGION = *mut IOMMU_DMA_RESERVED_REGION;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PIOMMU_DOMAIN_ATTACH_DEVICE = *mut IOMMU_DOMAIN_ATTACH_DEVICE;
#[cfg(feature = "bcrypt")]
pub type PIOMMU_DOMAIN_ATTACH_DEVICE_EX = *mut IOMMU_DOMAIN_ATTACH_DEVICE_EX;
#[cfg(feature = "bcrypt")]
pub type PIOMMU_DOMAIN_ATTACH_PASID_DEVICE = *mut IOMMU_DOMAIN_ATTACH_PASID_DEVICE;
#[cfg(all(feature = "bcrypt", feature = "usb"))]
pub type PIOMMU_DOMAIN_CONFIGURE = *mut IOMMU_DOMAIN_CONFIGURE;
#[cfg(feature = "bcrypt")]
pub type PIOMMU_DOMAIN_CREATE = *mut IOMMU_DOMAIN_CREATE;
#[cfg(feature = "bcrypt")]
pub type PIOMMU_DOMAIN_CREATE_EX = *mut IOMMU_DOMAIN_CREATE_EX;
#[cfg(feature = "bcrypt")]
pub type PIOMMU_DOMAIN_DELETE = *mut IOMMU_DOMAIN_DELETE;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PIOMMU_DOMAIN_DETACH_DEVICE = *mut IOMMU_DOMAIN_DETACH_DEVICE;
#[cfg(feature = "bcrypt")]
pub type PIOMMU_DOMAIN_DETACH_DEVICE_EX = *mut IOMMU_DOMAIN_DETACH_DEVICE_EX;
#[cfg(feature = "bcrypt")]
pub type PIOMMU_DOMAIN_DETACH_PASID_DEVICE = *mut IOMMU_DOMAIN_DETACH_PASID_DEVICE;
#[cfg(feature = "bcrypt")]
pub type PIOMMU_FLUSH_DOMAIN = *mut IOMMU_FLUSH_DOMAIN;
#[cfg(feature = "bcrypt")]
pub type PIOMMU_FLUSH_DOMAIN_VA_LIST = *mut IOMMU_FLUSH_DOMAIN_VA_LIST;
#[cfg(feature = "bcrypt")]
pub type PIOMMU_FREE_RESERVED_LOGICAL_ADDRESS_RANGE = *mut IOMMU_FREE_RESERVED_LOGICAL_ADDRESS_RANGE;
pub type PIOMMU_INTERFACE_STATE_CHANGE = *mut IOMMU_INTERFACE_STATE_CHANGE;
pub type PIOMMU_INTERFACE_STATE_CHANGE_CALLBACK = *mut IOMMU_INTERFACE_STATE_CHANGE_CALLBACK;
pub type PIOMMU_INTERFACE_STATE_CHANGE_FIELDS = *mut IOMMU_INTERFACE_STATE_CHANGE_FIELDS;
#[cfg(all(feature = "bcrypt", feature = "ntdef"))]
pub type PIOMMU_MAP_IDENTITY_RANGE = *mut IOMMU_MAP_IDENTITY_RANGE;
#[cfg(all(feature = "bcrypt", feature = "usb"))]
pub type PIOMMU_MAP_IDENTITY_RANGE_EX = *mut IOMMU_MAP_IDENTITY_RANGE_EX;
#[cfg(all(feature = "bcrypt", feature = "ntdef"))]
pub type PIOMMU_MAP_LOGICAL_RANGE = *mut IOMMU_MAP_LOGICAL_RANGE;
#[cfg(all(feature = "bcrypt", feature = "usb"))]
pub type PIOMMU_MAP_LOGICAL_RANGE_EX = *mut IOMMU_MAP_LOGICAL_RANGE_EX;
#[cfg(feature = "usb")]
pub type PIOMMU_MAP_PHYSICAL_ADDRESS = *mut IOMMU_MAP_PHYSICAL_ADDRESS;
pub type PIOMMU_MAP_PHYSICAL_ADDRESS_TYPE = *mut IOMMU_MAP_PHYSICAL_ADDRESS_TYPE;
#[cfg(all(feature = "bcrypt", feature = "usb"))]
pub type PIOMMU_MAP_RESERVED_LOGICAL_RANGE = *mut IOMMU_MAP_RESERVED_LOGICAL_RANGE;
pub type PIOMMU_PASID_CONFIGURATION_TYPE = *mut IOMMU_PASID_CONFIGURATION_TYPE;
#[cfg(feature = "bcrypt")]
pub type PIOMMU_PASID_DEVICE_CREATE = *mut IOMMU_PASID_DEVICE_CREATE;
#[cfg(feature = "bcrypt")]
pub type PIOMMU_PASID_DEVICE_DELETE = *mut IOMMU_PASID_DEVICE_DELETE;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PIOMMU_QUERY_INPUT_MAPPINGS = *mut IOMMU_QUERY_INPUT_MAPPINGS;
#[cfg(feature = "bcrypt")]
pub type PIOMMU_REGISTER_INTERFACE_STATE_CHANGE_CALLBACK = *mut IOMMU_REGISTER_INTERFACE_STATE_CHANGE_CALLBACK;
#[cfg(feature = "bcrypt")]
pub type PIOMMU_RESERVE_LOGICAL_ADDRESS_RANGE = *mut IOMMU_RESERVE_LOGICAL_ADDRESS_RANGE;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PIOMMU_SET_DEVICE_FAULT_REPORTING = *mut IOMMU_SET_DEVICE_FAULT_REPORTING;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PIOMMU_SET_DEVICE_FAULT_REPORTING_EX = *mut IOMMU_SET_DEVICE_FAULT_REPORTING_EX;
#[cfg(all(feature = "bcrypt", feature = "ntdef"))]
pub type PIOMMU_UNMAP_IDENTITY_RANGE = *mut IOMMU_UNMAP_IDENTITY_RANGE;
#[cfg(all(feature = "bcrypt", feature = "usb"))]
pub type PIOMMU_UNMAP_IDENTITY_RANGE_EX = *mut IOMMU_UNMAP_IDENTITY_RANGE_EX;
#[cfg(feature = "bcrypt")]
pub type PIOMMU_UNMAP_LOGICAL_RANGE = *mut IOMMU_UNMAP_LOGICAL_RANGE;
#[cfg(feature = "bcrypt")]
pub type PIOMMU_UNMAP_RESERVED_LOGICAL_RANGE = *mut IOMMU_UNMAP_RESERVED_LOGICAL_RANGE;
#[cfg(feature = "bcrypt")]
pub type PIOMMU_UNREGISTER_INTERFACE_STATE_CHANGE_CALLBACK = *mut IOMMU_UNREGISTER_INTERFACE_STATE_CHANGE_CALLBACK;
pub type PIO_ALLOCATION_ACTION = *mut IO_ALLOCATION_ACTION;
pub type PIO_ATTRIBUTION_INFORMATION = *mut IO_ATTRIBUTION_INFORMATION;
pub type PIO_COMPLETION_CONTEXT = *mut IO_COMPLETION_CONTEXT;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PIO_COMPLETION_ROUTINE = *mut IO_COMPLETION_ROUTINE;
pub type PIO_COMPLETION_ROUTINE_RESULT = *mut IO_COMPLETION_ROUTINE_RESULT;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PIO_CONNECT_INTERRUPT_FULLY_SPECIFIED_PARAMETERS = *mut IO_CONNECT_INTERRUPT_FULLY_SPECIFIED_PARAMETERS;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PIO_CONNECT_INTERRUPT_LINE_BASED_PARAMETERS = *mut IO_CONNECT_INTERRUPT_LINE_BASED_PARAMETERS;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PIO_CONNECT_INTERRUPT_MESSAGE_BASED_PARAMETERS = *mut IO_CONNECT_INTERRUPT_MESSAGE_BASED_PARAMETERS;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PIO_CONNECT_INTERRUPT_PARAMETERS = *mut IO_CONNECT_INTERRUPT_PARAMETERS;
#[cfg(feature = "bcrypt")]
pub type PIO_CONTAINER_NOTIFICATION_FUNCTION = Option<unsafe extern "system" fn() -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PIO_CSQ = *mut IO_CSQ;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PIO_CSQ_ACQUIRE_LOCK = *mut IO_CSQ_ACQUIRE_LOCK;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PIO_CSQ_COMPLETE_CANCELED_IRP = *mut IO_CSQ_COMPLETE_CANCELED_IRP;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PIO_CSQ_INSERT_IRP = *mut IO_CSQ_INSERT_IRP;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PIO_CSQ_INSERT_IRP_EX = *mut IO_CSQ_INSERT_IRP_EX;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PIO_CSQ_IRP_CONTEXT = *mut IO_CSQ_IRP_CONTEXT;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PIO_CSQ_PEEK_NEXT_IRP = *mut IO_CSQ_PEEK_NEXT_IRP;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PIO_CSQ_RELEASE_LOCK = *mut IO_CSQ_RELEASE_LOCK;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PIO_CSQ_REMOVE_IRP = *mut IO_CSQ_REMOVE_IRP;
#[cfg(feature = "bcrypt")]
pub type PIO_DEVICE_EJECT_CALLBACK = Option<unsafe extern "system" fn(status: super::bcrypt::NTSTATUS, context: *mut core::ffi::c_void)>;
#[cfg(all(feature = "basetsd", feature = "ntdef", feature = "usb"))]
pub type PIO_DISCONNECT_INTERRUPT_PARAMETERS = *mut IO_DISCONNECT_INTERRUPT_PARAMETERS;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PIO_DPC_ROUTINE = *mut IO_DPC_ROUTINE;
#[cfg(feature = "bcrypt")]
pub type PIO_ERROR_LOG_MESSAGE = *mut IO_ERROR_LOG_MESSAGE;
#[cfg(feature = "bcrypt")]
pub type PIO_ERROR_LOG_PACKET = *mut IO_ERROR_LOG_PACKET;
#[cfg(all(feature = "basetsd", feature = "ntdef", feature = "usb"))]
pub type PIO_INTERRUPT_MESSAGE_INFO = *mut IO_INTERRUPT_MESSAGE_INFO;
#[cfg(all(feature = "basetsd", feature = "ntdef", feature = "usb"))]
pub type PIO_INTERRUPT_MESSAGE_INFO_ENTRY = *mut IO_INTERRUPT_MESSAGE_INFO_ENTRY;
pub type PIO_MINI_COMPLETION_PACKET_USER = *mut IO_MINI_COMPLETION_PACKET_USER;
pub type PIO_MINI_PACKET_CALLBACK_ROUTINE = *mut IO_MINI_PACKET_CALLBACK_ROUTINE;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PIO_PERSISTED_MEMORY_ENUMERATION_CALLBACK = *mut IO_PERSISTED_MEMORY_ENUMERATION_CALLBACK;
#[cfg(feature = "winnt")]
pub type PIO_REMOVE_LOCK = *mut IO_REMOVE_LOCK;
pub type PIO_REMOVE_LOCK_TRACKING_BLOCK = *mut _IO_REMOVE_LOCK_TRACKING_BLOCK;
#[cfg(all(feature = "basetsd", feature = "ntdef", feature = "usb"))]
pub type PIO_REPORT_INTERRUPT_ACTIVE_STATE_PARAMETERS = *mut IO_REPORT_INTERRUPT_ACTIVE_STATE_PARAMETERS;
#[cfg(all(feature = "basetsd", feature = "usb"))]
pub type PIO_RESOURCE_DESCRIPTOR = *mut IO_RESOURCE_DESCRIPTOR;
#[cfg(all(feature = "basetsd", feature = "usb"))]
pub type PIO_RESOURCE_LIST = *mut IO_RESOURCE_LIST;
#[cfg(all(feature = "basetsd", feature = "usb"))]
pub type PIO_RESOURCE_REQUIREMENTS_LIST = *mut IO_RESOURCE_REQUIREMENTS_LIST;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
pub type PIO_SECURITY_CONTEXT = *mut IO_SECURITY_CONTEXT;
pub type PIO_SESSION_CONNECT_INFO = *mut IO_SESSION_CONNECT_INFO;
pub type PIO_SESSION_EVENT = *mut IO_SESSION_EVENT;
#[cfg(feature = "bcrypt")]
pub type PIO_SESSION_NOTIFICATION_FUNCTION = *mut IO_SESSION_NOTIFICATION_FUNCTION;
pub type PIO_SESSION_STATE = *mut IO_SESSION_STATE;
pub type PIO_SESSION_STATE_INFORMATION = *mut IO_SESSION_STATE_INFORMATION;
pub type PIO_SESSION_STATE_NOTIFICATION = *mut IO_SESSION_STATE_NOTIFICATION;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PIO_STACK_LOCATION = *mut IO_STACK_LOCATION;
#[cfg(feature = "bcrypt")]
pub type PIO_STATUS_BLOCK32 = *mut IO_STATUS_BLOCK32;
#[cfg(all(feature = "bcrypt", feature = "winternl"))]
pub type PIO_STATUS_BLOCK64 = *mut IO_STATUS_BLOCK64;
pub type PIO_TIMER = *mut _IO_TIMER;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PIO_TIMER_ROUTINE = *mut IO_TIMER_ROUTINE;
pub type PIO_WORKITEM = *mut _IO_WORKITEM;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PIO_WORKITEM_ROUTINE = *mut IO_WORKITEM_ROUTINE;
pub type PIO_WORKITEM_ROUTINE_EX = *mut IO_WORKITEM_ROUTINE_EX;
pub type PIRQ_DEVICE_POLICY = *mut IRQ_DEVICE_POLICY;
pub type PIRQ_GROUP_POLICY = *mut IRQ_GROUP_POLICY;
pub type PIRQ_PRIORITY = *mut IRQ_PRIORITY;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PJOIN_DMA_DOMAIN = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, domainhandle: super::winnt::HANDLE) -> super::bcrypt::NTSTATUS>;
pub type PKADDRESS_RANGE = *mut KADDRESS_RANGE;
pub type PKADDRESS_RANGE_DESCRIPTOR = *mut KADDRESS_RANGE_DESCRIPTOR;
pub type PKAFFINITY_EX = *mut _KAFFINITY_EX;
#[cfg(feature = "winnt")]
pub type PKAPC = *mut KAPC;
pub type PKBUGCHECK_ADD_PAGES = *mut KBUGCHECK_ADD_PAGES;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PKBUGCHECK_CALLBACK_RECORD = *mut KBUGCHECK_CALLBACK_RECORD;
pub type PKBUGCHECK_CALLBACK_ROUTINE = *mut KBUGCHECK_CALLBACK_ROUTINE;
pub type PKBUGCHECK_DUMP_IO = *mut KBUGCHECK_DUMP_IO;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PKBUGCHECK_REASON_CALLBACK_RECORD = *mut KBUGCHECK_REASON_CALLBACK_RECORD;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PKBUGCHECK_REASON_CALLBACK_ROUTINE = *mut KBUGCHECK_REASON_CALLBACK_ROUTINE;
pub type PKBUGCHECK_REMOVE_PAGES = *mut KBUGCHECK_REMOVE_PAGES;
pub type PKBUGCHECK_SECONDARY_DUMP_DATA = *mut KBUGCHECK_SECONDARY_DUMP_DATA;
pub type PKBUGCHECK_SECONDARY_DUMP_DATA_EX = *mut KBUGCHECK_SECONDARY_DUMP_DATA_EX;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PKBUGCHECK_TRIAGE_DUMP_DATA = *mut KBUGCHECK_TRIAGE_DUMP_DATA;
pub type PKCRM_PROTOCOL_ID = *mut windows_core::GUID;
#[cfg(all(feature = "basetsd", feature = "winnt"))]
pub type PKDEFERRED_ROUTINE = *mut KDEFERRED_ROUTINE;
#[cfg(all(feature = "ntdef", feature = "winnt"))]
pub type PKDEVICE_QUEUE = *mut KDEVICE_QUEUE;
#[cfg(feature = "winnt")]
pub type PKDEVICE_QUEUE_ENTRY = *mut KDEVICE_QUEUE_ENTRY;
#[cfg(all(feature = "basetsd", feature = "winnt"))]
pub type PKDPC = *mut KDPC;
pub type PKDPC_WATCHDOG_INFORMATION = *mut KDPC_WATCHDOG_INFORMATION;
pub type PKENLISTMENT = *mut KENLISTMENT;
pub type PKERNEL_SOFT_RESTART_NOTIFICATION = *mut KERNEL_SOFT_RESTART_NOTIFICATION;
#[cfg(feature = "winnt")]
pub type PKEVENT = *mut KEVENT;
pub type PKEY_BASIC_INFORMATION = *mut KEY_BASIC_INFORMATION;
pub type PKEY_CONTROL_FLAGS_INFORMATION = *mut KEY_CONTROL_FLAGS_INFORMATION;
pub type PKEY_FULL_INFORMATION = *mut KEY_FULL_INFORMATION;
pub type PKEY_NODE_INFORMATION = *mut KEY_NODE_INFORMATION;
pub type PKEY_SET_VIRTUALIZATION_INFORMATION = *mut KEY_SET_VIRTUALIZATION_INFORMATION;
pub type PKEY_TRUST_INFORMATION = *mut KEY_TRUST_INFORMATION;
pub type PKEY_VALUE_BASIC_INFORMATION = *mut KEY_VALUE_BASIC_INFORMATION;
pub type PKEY_VALUE_FULL_INFORMATION = *mut KEY_VALUE_FULL_INFORMATION;
pub type PKEY_VALUE_LAYER_INFORMATION = *mut KEY_VALUE_LAYER_INFORMATION;
pub type PKEY_VALUE_PARTIAL_INFORMATION = *mut KEY_VALUE_PARTIAL_INFORMATION;
pub type PKEY_VALUE_PARTIAL_INFORMATION_ALIGN64 = *mut KEY_VALUE_PARTIAL_INFORMATION_ALIGN64;
pub type PKEY_WOW64_FLAGS_INFORMATION = *mut KEY_WOW64_FLAGS_INFORMATION;
pub type PKEY_WRITE_TIME_INFORMATION = *mut KEY_WRITE_TIME_INFORMATION;
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
pub type PKE_PROCESSOR_CHANGE_NOTIFY_CONTEXT = *mut KE_PROCESSOR_CHANGE_NOTIFY_CONTEXT;
pub type PKE_SRCU = *mut _KE_SRCU;
pub type PKE_SRCU_LOCK = *mut KE_SRCU_LOCK;
pub type PKFLOATING_SAVE = *mut KFLOATING_SAVE;
#[cfg(feature = "winnt")]
pub type PKGATE = *mut KGATE;
#[cfg(feature = "winnt")]
pub type PKGUARDED_MUTEX = *mut FAST_MUTEX;
pub type PKINTERRUPT = *mut _KINTERRUPT;
pub type PKINTERRUPT_POLARITY = *mut KINTERRUPT_POLARITY;
pub type PKIPI_BROADCAST_WORKER = *mut KIPI_BROADCAST_WORKER;
#[cfg(all(feature = "ntdef", feature = "winnt"))]
pub type PKLOCK_QUEUE_HANDLE = *mut KLOCK_QUEUE_HANDLE;
pub type PKMESSAGE_SERVICE_ROUTINE = *mut KMESSAGE_SERVICE_ROUTINE;
#[cfg(feature = "winnt")]
pub type PKMUTANT = *mut KMUTANT;
#[cfg(feature = "winnt")]
pub type PKMUTEX = *mut KMUTANT;
pub type PKPROCESS = *mut _KPROCESS;
pub type PKRESOURCEMANAGER = *mut KRESOURCEMANAGER;
#[cfg(feature = "winnt")]
pub type PKSEMAPHORE = *mut KSEMAPHORE;
pub type PKSERVICE_ROUTINE = *mut KSERVICE_ROUTINE;
#[cfg(feature = "winnt")]
pub type PKSPIN_LOCK_QUEUE = *mut KSPIN_LOCK_QUEUE;
#[cfg(any(target_arch = "aarch64", target_arch = "x86"))]
pub type PKSPIN_LOCK_QUEUE_NUMBER = *mut KSPIN_LOCK_QUEUE_NUMBER;
pub type PKSTART_ROUTINE = *mut KSTART_ROUTINE;
pub type PKSYNCHRONIZE_ROUTINE = *mut KSYNCHRONIZE_ROUTINE;
pub type PKSYSTEM_TIME = *mut KSYSTEM_TIME;
pub type PKT2_SET_PARAMETERS = *mut EXT_SET_PARAMETERS;
pub type PKTHREAD = *mut _KTHREAD;
#[cfg(all(feature = "basetsd", feature = "winnt"))]
pub type PKTIMER = *mut KTIMER;
pub type PKTM = *mut KTM;
pub type PKTRANSACTION = *mut KTRANSACTION;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PKTRIAGE_DUMP_DATA_ARRAY = *mut KTRIAGE_DUMP_DATA_ARRAY;
#[cfg(all(feature = "basetsd", feature = "ntifs", feature = "winnt"))]
pub type PKWAIT_BLOCK = *mut KWAIT_BLOCK;
pub type PKWAIT_CHAIN = *mut KWAIT_CHAIN;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PLEAVE_DMA_DOMAIN = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER) -> super::bcrypt::NTSTATUS>;
pub type PLEGACY_BUS_INFORMATION = *mut LEGACY_BUS_INFORMATION;
pub type PLINK_SHARE_ACCESS = *mut LINK_SHARE_ACCESS;
pub type PLOADER_PARTITION_INFORMATION_EX = *mut LOADER_PARTITION_INFORMATION_EX;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PLOG_FILE_OBJECT = *mut FILE_OBJECT;
#[cfg(feature = "winnt")]
pub type PLOOKASIDE_LIST_EX = *mut LOOKASIDE_LIST_EX;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PLUGPLAY_NOTIFICATION_HEADER {
    pub Version: u16,
    pub Size: u16,
    pub Event: windows_core::GUID,
}
pub const PLUGPLAY_PROPERTY_PERSISTENT: u32 = 1;
pub const PLUGPLAY_REGKEY_CURRENT_HWPROFILE: u32 = 4;
pub const PLUGPLAY_REGKEY_DEVICE: u32 = 1;
pub const PLUGPLAY_REGKEY_DRIVER: u32 = 2;
pub type PMAILSLOT_CREATE_PARAMETERS = *mut MAILSLOT_CREATE_PARAMETERS;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PMAP_TRANSFER = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, mdl: *const MDL, mapregisterbase: *const core::ffi::c_void, currentva: *const core::ffi::c_void, length: *mut u32, writetodevice: bool) -> super::usb::PHYSICAL_ADDRESS>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PMAP_TRANSFER_EX = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, mdl: *const MDL, mapregisterbase: *const core::ffi::c_void, offset: u64, deviceoffset: u32, length: *mut u32, writetodevice: bool, scattergatherbuffer: *mut SCATTER_GATHER_LIST, scattergatherbufferlength: u32, dmacompletionroutine: PDMA_COMPLETION_ROUTINE, completioncontext: *const core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "ntdef")]
pub type PMDLX = *mut MDL;
#[cfg(feature = "winnt")]
pub type PMEMORY_PARTITION_DEDICATED_MEMORY_OPEN_INFORMATION = *mut MEMORY_PARTITION_DEDICATED_MEMORY_OPEN_INFORMATION;
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi"))]
pub type PMM_DLL_INITIALIZE = Option<unsafe extern "system" fn(registrypath: *const super::ntsecapi::UNICODE_STRING) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "bcrypt")]
pub type PMM_DLL_UNLOAD = Option<unsafe extern "system" fn() -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
pub type PMM_GET_SYSTEM_ROUTINE_ADDRESS_EX = Option<unsafe extern "system" fn(modulename: *const super::ntsecapi::UNICODE_STRING, functionname: windows_core::PCSTR) -> *mut core::ffi::c_void>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PMM_MDL_PAGE_CONTENTS_STATE(pub MM_MDL_PAGE_CONTENTS_STATE);
pub type PMM_MDL_ROUTINE = *mut MM_MDL_ROUTINE;
#[cfg(feature = "usb")]
pub type PMM_PHYSICAL_ADDRESS_LIST = *mut MM_PHYSICAL_ADDRESS_LIST;
pub type PNAMED_PIPE_CREATE_PARAMETERS = *mut NAMED_PIPE_CREATE_PARAMETERS;
pub type PNMI_CALLBACK = *mut NMI_CALLBACK;
#[cfg(feature = "winnt")]
pub type PNPAGED_LOOKASIDE_LIST = *mut NPAGED_LOOKASIDE_LIST;
pub const PNPBus: INTERFACE_TYPE = 15;
pub type PNPEM_CAPABILITY_STANDARD = *mut NPEM_CAPABILITY_STANDARD;
#[cfg(feature = "bcrypt")]
pub type PNPEM_CONTROL_ENABLE_DISABLE = *mut NPEM_CONTROL_ENABLE_DISABLE;
#[cfg(feature = "bcrypt")]
pub type PNPEM_CONTROL_INTERFACE = *mut NPEM_CONTROL_INTERFACE;
pub type PNPEM_CONTROL_QUERY_CONTROL = *mut NPEM_CONTROL_QUERY_CONTROL;
#[cfg(feature = "bcrypt")]
pub type PNPEM_CONTROL_QUERY_STANDARD_CAPABILITIES = *mut NPEM_CONTROL_QUERY_STANDARD_CAPABILITIES;
#[cfg(feature = "bcrypt")]
pub type PNPEM_CONTROL_SET_STANDARD_CONTROL = *mut NPEM_CONTROL_SET_STANDARD_CONTROL;
pub type PNPEM_CONTROL_STANDARD_CONTROL_BIT = *mut NPEM_CONTROL_STANDARD_CONTROL_BIT;
pub const PNPISABus: INTERFACE_TYPE = 14;
pub const PNPNOTIFY_DEVICE_INTERFACE_INCLUDE_EXISTING_INTERFACES: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PNP_BUS_INFORMATION {
    pub BusTypeGuid: windows_core::GUID,
    pub LegacyBusType: INTERFACE_TYPE,
    pub BusNumber: u32,
}
pub const PNP_DEVICE_ASSIGNED_TO_GUEST: u32 = 256;
pub const PNP_DEVICE_DISABLED: u32 = 1;
pub const PNP_DEVICE_DISCONNECTED: u32 = 64;
pub const PNP_DEVICE_DONT_DISPLAY_IN_UI: u32 = 2;
pub const PNP_DEVICE_FAILED: u32 = 4;
pub const PNP_DEVICE_NOT_DISABLEABLE: u32 = 32;
pub const PNP_DEVICE_REMOVED: u32 = 8;
pub const PNP_DEVICE_RESOURCE_REQUIREMENTS_CHANGED: u32 = 16;
pub const PNP_DEVICE_RESOURCE_UPDATED: u32 = 128;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PNP_DEVICE_STATE(pub u32);
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct PNP_EXTENDED_ADDRESS_INTERFACE {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
    pub QueryExtendedAddress: PQUERYEXTENDEDADDRESS,
}
impl Default for PNP_EXTENDED_ADDRESS_INTERFACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PNP_EXTENDED_ADDRESS_INTERFACE_VERSION: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "usb"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct PNP_REPLACE_DRIVER_INTERFACE {
    pub Size: u32,
    pub Version: u32,
    pub Flags: u32,
    pub Unload: PREPLACE_UNLOAD,
    pub BeginReplace: PREPLACE_BEGIN,
    pub EndReplace: PREPLACE_END,
    pub MirrorPhysicalMemory: PREPLACE_MIRROR_PHYSICAL_MEMORY,
    pub SetProcessorId: PREPLACE_SET_PROCESSOR_ID,
    pub Swap: PREPLACE_SWAP,
    pub InitiateHardwareMirror: PREPLACE_INITIATE_HARDWARE_MIRROR,
    pub MirrorPlatformMemory: PREPLACE_MIRROR_PLATFORM_MEMORY,
    pub GetMemoryDestination: PREPLACE_GET_MEMORY_DESTINATION,
    pub EnableDisableHardwareQuiesce: PREPLACE_ENABLE_DISABLE_HARDWARE_QUIESCE,
}
#[cfg(target_arch = "x86")]
pub const PNP_REPLACE_DRIVER_INTERFACE_MINIMUM_SIZE: u32 = 36;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const PNP_REPLACE_DRIVER_INTERFACE_MINIMUM_SIZE: u32 = 64;
pub const PNP_REPLACE_DRIVER_INTERFACE_VERSION: u32 = 1;
pub const PNP_REPLACE_HARDWARE_MEMORY_MIRRORING: u32 = 4;
pub const PNP_REPLACE_HARDWARE_PAGE_COPY: u32 = 8;
pub const PNP_REPLACE_HARDWARE_QUIESCE: u32 = 16;
#[repr(C)]
#[cfg(feature = "usb")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PNP_REPLACE_MEMORY_LIST {
    pub AllocatedCount: u32,
    pub Count: u32,
    pub TotalLength: u64,
    pub Ranges: [PNP_REPLACE_MEMORY_LIST_0; 1],
}
#[cfg(feature = "usb")]
impl Default for PNP_REPLACE_MEMORY_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "usb")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PNP_REPLACE_MEMORY_LIST_0 {
    pub Address: super::usb::PHYSICAL_ADDRESS,
    pub Length: u64,
}
pub const PNP_REPLACE_MEMORY_SUPPORTED: u32 = 1;
pub const PNP_REPLACE_NO_MAP: i32 = -1;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "usb"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct PNP_REPLACE_PARAMETERS {
    pub Size: u32,
    pub Version: u32,
    pub Target: u64,
    pub Spare: u64,
    pub TargetProcessors: PPNP_REPLACE_PROCESSOR_LIST,
    pub SpareProcessors: PPNP_REPLACE_PROCESSOR_LIST,
    pub TargetMemory: PPNP_REPLACE_MEMORY_LIST,
    pub SpareMemory: PPNP_REPLACE_MEMORY_LIST,
    pub MapMemory: PREPLACE_MAP_MEMORY,
}
pub const PNP_REPLACE_PARAMETERS_VERSION: u32 = 2;
#[repr(C)]
#[cfg(feature = "basetsd")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PNP_REPLACE_PROCESSOR_LIST {
    pub Affinity: super::basetsd::PKAFFINITY,
    pub GroupCount: u32,
    pub AllocatedCount: u32,
    pub Count: u32,
    pub ApicIds: [u32; 1],
}
#[cfg(feature = "basetsd")]
impl Default for PNP_REPLACE_PROCESSOR_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "basetsd")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PNP_REPLACE_PROCESSOR_LIST_V1 {
    pub AffinityMask: super::basetsd::KAFFINITY,
    pub AllocatedCount: u32,
    pub Count: u32,
    pub ApicIds: [u32; 1],
}
#[cfg(feature = "basetsd")]
impl Default for PNP_REPLACE_PROCESSOR_LIST_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PNP_REPLACE_PROCESSOR_SUPPORTED: u32 = 2;
#[cfg(feature = "winnt")]
pub type PNTFS_DEREF_EXPORTED_SECURITY_DESCRIPTOR = *mut NTFS_DEREF_EXPORTED_SECURITY_DESCRIPTOR;
#[cfg(feature = "winnt")]
pub type POBJECT_HANDLE_INFORMATION = *mut OBJECT_HANDLE_INFORMATION;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
pub type POBJECT_NAME_INFORMATION = *mut OBJECT_NAME_INFORMATION;
pub type POBJECT_TYPE = *mut _OBJECT_TYPE;
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
pub type POB_CALLBACK_REGISTRATION = *mut OB_CALLBACK_REGISTRATION;
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
pub type POB_OPERATION_REGISTRATION = *mut OB_OPERATION_REGISTRATION;
#[cfg(feature = "winnt")]
pub type POB_POST_CREATE_HANDLE_INFORMATION = *mut OB_POST_CREATE_HANDLE_INFORMATION;
#[cfg(feature = "winnt")]
pub type POB_POST_DUPLICATE_HANDLE_INFORMATION = *mut OB_POST_DUPLICATE_HANDLE_INFORMATION;
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
pub type POB_POST_OPERATION_CALLBACK = Option<unsafe extern "system" fn(registrationcontext: *const core::ffi::c_void, operationinformation: *const OB_POST_OPERATION_INFORMATION)>;
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
pub type POB_POST_OPERATION_INFORMATION = *mut OB_POST_OPERATION_INFORMATION;
#[cfg(feature = "winnt")]
pub type POB_POST_OPERATION_PARAMETERS = *mut OB_POST_OPERATION_PARAMETERS;
pub type POB_PREOP_CALLBACK_STATUS = *mut OB_PREOP_CALLBACK_STATUS;
#[cfg(feature = "winnt")]
pub type POB_PRE_CREATE_HANDLE_INFORMATION = *mut OB_PRE_CREATE_HANDLE_INFORMATION;
#[cfg(feature = "winnt")]
pub type POB_PRE_DUPLICATE_HANDLE_INFORMATION = *mut OB_PRE_DUPLICATE_HANDLE_INFORMATION;
#[cfg(feature = "winnt")]
pub type POB_PRE_OPERATION_CALLBACK = Option<unsafe extern "system" fn(registrationcontext: *const core::ffi::c_void, operationinformation: *mut OB_PRE_OPERATION_INFORMATION) -> OB_PREOP_CALLBACK_STATUS>;
#[cfg(feature = "winnt")]
pub type POB_PRE_OPERATION_INFORMATION = *mut OB_PRE_OPERATION_INFORMATION;
#[cfg(feature = "winnt")]
pub type POB_PRE_OPERATION_PARAMETERS = *mut OB_PRE_OPERATION_PARAMETERS;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct POHANDLE(pub *mut core::ffi::c_void);
impl Default for POHANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const POOL_COLD_ALLOCATION: u32 = 256;
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
#[derive(Clone, Copy)]
pub struct POOL_CREATE_EXTENDED_PARAMETER {
    pub Type: POOL_CREATE_EXTENDED_PARAMETER_TYPE,
    pub Anonymous: POOL_CREATE_EXTENDED_PARAMETER_0,
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
impl Default for POOL_CREATE_EXTENDED_PARAMETER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
#[derive(Clone, Copy)]
pub union POOL_CREATE_EXTENDED_PARAMETER_0 {
    pub PoolName: super::ntsecapi::UNICODE_STRING,
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
impl Default for POOL_CREATE_EXTENDED_PARAMETER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type POOL_CREATE_EXTENDED_PARAMETER_TYPE = i32;
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POOL_CREATE_EXTENDED_PARAMS {
    pub Version: u32,
    pub ParameterCount: u32,
    pub Parameters: PPOOL_CREATE_EXTENDED_PARAMETER,
}
pub const POOL_CREATE_FLG_NONPAGED_POOL: u32 = 8;
pub const POOL_CREATE_FLG_NUMA_AWARE: u32 = 16;
pub const POOL_CREATE_FLG_PAGED_POOL: u32 = 4;
pub const POOL_CREATE_FLG_POOL_TYPES: u32 = 13;
pub const POOL_CREATE_FLG_SECURE_POOL: u32 = 1;
pub const POOL_CREATE_FLG_USE_GLOBAL_POOL: u32 = 2;
pub const POOL_CREATE_PARAMS_VERSION: u32 = 1;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct POOL_EXTENDED_PARAMETER {
    pub Anonymous: POOL_EXTENDED_PARAMETER_0,
    pub Anonymous2: POOL_EXTENDED_PARAMETER_1,
}
#[cfg(feature = "winnt")]
impl Default for POOL_EXTENDED_PARAMETER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POOL_EXTENDED_PARAMETER_0 {
    pub _bitfield: u64,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union POOL_EXTENDED_PARAMETER_1 {
    pub Reserved2: u64,
    pub Reserved3: *mut core::ffi::c_void,
    pub Priority: EX_POOL_PRIORITY,
    pub SecurePoolParams: *mut POOL_EXTENDED_PARAMS_SECURE_POOL,
    pub PreferredNode: POOL_NODE_REQUIREMENT,
    pub PrivatePoolHandle: super::winnt::HANDLE,
}
#[cfg(feature = "winnt")]
impl Default for POOL_EXTENDED_PARAMETER_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const POOL_EXTENDED_PARAMETER_REQUIRED_FIELD_BITS: u32 = 1;
pub const POOL_EXTENDED_PARAMETER_RESERVED_BITS: u32 = 55;
pub type POOL_EXTENDED_PARAMETER_TYPE = i32;
pub const POOL_EXTENDED_PARAMETER_TYPE_BITS: u32 = 8;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct POOL_EXTENDED_PARAMS_SECURE_POOL {
    pub SecurePoolHandle: super::winnt::HANDLE,
    pub Buffer: *mut core::ffi::c_void,
    pub Cookie: usize,
    pub SecurePoolFlags: u32,
}
#[cfg(feature = "winnt")]
impl Default for POOL_EXTENDED_PARAMS_SECURE_POOL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct POOL_FLAGS(pub u64);
pub const POOL_FLAG_CACHE_ALIGNED: u32 = 8;
pub const POOL_FLAG_LAST_KNOWN_REQUIRED: u32 = 2048;
pub const POOL_FLAG_NON_PAGED: u32 = 64;
pub const POOL_FLAG_NON_PAGED_EXECUTE: u32 = 128;
pub const POOL_FLAG_OPTIONAL_END: u32 = 0;
pub const POOL_FLAG_OPTIONAL_START: u32 = 0;
pub const POOL_FLAG_PAGED: u32 = 256;
pub const POOL_FLAG_RAISE_ON_FAILURE: u32 = 32;
pub const POOL_FLAG_REQUIRED_END: i32 = -2147483648;
pub const POOL_FLAG_REQUIRED_MASK: i32 = -1;
pub const POOL_FLAG_REQUIRED_START: u32 = 1;
pub const POOL_FLAG_RESERVED1: u32 = 16;
pub const POOL_FLAG_RESERVED3: u32 = 1024;
pub const POOL_FLAG_RESERVED4: u32 = 2048;
pub const POOL_FLAG_SESSION: u32 = 4;
pub const POOL_FLAG_SPECIAL_POOL: u32 = 0;
pub const POOL_FLAG_TAGGED_VA: u32 = 512;
pub const POOL_FLAG_UNINITIALIZED: u32 = 2;
pub const POOL_FLAG_UNUSED_REQUIRED_BITS: i32 = -4096;
pub const POOL_FLAG_USE_QUOTA: u32 = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct POOL_NODE_REQUIREMENT(pub u32);
pub const POOL_NX_ALLOCATION: u32 = 512;
#[cfg(target_arch = "aarch64")]
pub const POOL_NX_OPTIN_AUTO: u32 = 1;
pub const POOL_QUOTA_FAIL_INSTEAD_OF_RAISE: u32 = 8;
pub const POOL_RAISE_IF_ALLOCATION_FAILURE: u32 = 16;
pub const POOL_TAGGING: u32 = 1;
pub type POOL_TYPE = i32;
pub const POOL_ZERO_ALLOCATION: u32 = 1024;
#[cfg(target_arch = "x86")]
pub const PORT_MAXIMUM_MESSAGE_LENGTH: u32 = 256;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const PORT_MAXIMUM_MESSAGE_LENGTH: u32 = 512;
#[cfg(target_arch = "x86")]
pub const POWER_LEVEL: u32 = 30;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const POWER_LEVEL: u32 = 14;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct POWER_SEQUENCE {
    pub SequenceD1: u32,
    pub SequenceD2: u32,
    pub SequenceD3: u32,
}
#[cfg(feature = "bcrypt")]
pub type POWER_SETTING_CALLBACK = Option<unsafe extern "system" fn(settingguid: *const windows_core::GUID, value: *const core::ffi::c_void, valuelength: u32, context: *mut core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union POWER_STATE {
    pub SystemState: super::winnt::SYSTEM_POWER_STATE,
    pub DeviceState: super::winnt::DEVICE_POWER_STATE,
}
#[cfg(feature = "winnt")]
impl Default for POWER_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type POWER_STATE_TYPE = i32;
pub type POWNER_ENTRY = *mut OWNER_ENTRY;
pub const PO_CB_AC_STATUS: u32 = 1;
pub const PO_CB_BUTTON_COLLISION: u32 = 2;
pub const PO_CB_LID_SWITCH_STATE: u32 = 4;
pub const PO_CB_PROCESSOR_POWER_POLICY: u32 = 5;
pub const PO_CB_SYSTEM_POWER_POLICY: u32 = 0;
pub const PO_CB_SYSTEM_STATE_LOCK: u32 = 3;
pub type PO_EFFECTIVE_POWER_MODE = i32;
pub type PO_EFFECTIVE_POWER_MODE_CALLBACK = Option<unsafe extern "system" fn(mode: PO_EFFECTIVE_POWER_MODE, context: *const core::ffi::c_void)>;
pub const PO_EFFECTIVE_POWER_MODE_V1: u32 = 1;
pub const PO_EFFECTIVE_POWER_MODE_V2: u32 = 2;
pub const PO_EFFECTIVE_POWER_MODE_VMAX: u32 = 2;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PO_EPM_HANDLE(pub *mut core::ffi::c_void);
impl Default for PO_EPM_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PO_FX_COMPONENT = PO_FX_COMPONENT_V1;
pub type PO_FX_COMPONENT_ACTIVE_CONDITION_CALLBACK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, component: u32)>;
pub type PO_FX_COMPONENT_CRITICAL_TRANSITION_CALLBACK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, component: u32, active: bool)>;
pub const PO_FX_COMPONENT_FLAG_F0_ON_CHILD_D0: u32 = 4;
pub const PO_FX_COMPONENT_FLAG_F0_ON_DX: u32 = 1;
pub const PO_FX_COMPONENT_FLAG_NO_DEBOUNCE: u32 = 2;
pub type PO_FX_COMPONENT_IDLE_CONDITION_CALLBACK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, component: u32)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PO_FX_COMPONENT_IDLE_STATE {
    pub TransitionLatency: u64,
    pub ResidencyRequirement: u64,
    pub NominalPower: u32,
}
pub type PO_FX_COMPONENT_IDLE_STATE_CALLBACK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, component: u32, state: u32)>;
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
#[derive(Clone, Copy)]
pub struct PO_FX_COMPONENT_PERF_INFO {
    pub PerfStateSetsCount: u32,
    pub PerfStateSets: [PO_FX_COMPONENT_PERF_SET; 1],
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
impl Default for PO_FX_COMPONENT_PERF_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
#[derive(Clone, Copy)]
pub struct PO_FX_COMPONENT_PERF_SET {
    pub Name: super::ntsecapi::UNICODE_STRING,
    pub Flags: u64,
    pub Unit: PO_FX_PERF_STATE_UNIT,
    pub Type: PO_FX_PERF_STATE_TYPE,
    pub Anonymous: PO_FX_COMPONENT_PERF_SET_0,
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
impl Default for PO_FX_COMPONENT_PERF_SET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
#[derive(Clone, Copy)]
pub union PO_FX_COMPONENT_PERF_SET_0 {
    pub Discrete: PO_FX_COMPONENT_PERF_SET_0_0,
    pub Range: PO_FX_COMPONENT_PERF_SET_0_1,
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
impl Default for PO_FX_COMPONENT_PERF_SET_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PO_FX_COMPONENT_PERF_SET_0_0 {
    pub Count: u32,
    pub States: PPO_FX_PERF_STATE,
}
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PO_FX_COMPONENT_PERF_SET_0_1 {
    pub Minimum: u64,
    pub Maximum: u64,
}
pub type PO_FX_COMPONENT_PERF_STATE_CALLBACK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, component: u32, succeeded: bool, requestcontext: *const core::ffi::c_void)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PO_FX_COMPONENT_V1 {
    pub Id: windows_core::GUID,
    pub IdleStateCount: u32,
    pub DeepestWakeableIdleState: u32,
    pub IdleStates: PPO_FX_COMPONENT_IDLE_STATE,
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PO_FX_COMPONENT_V2 {
    pub Id: windows_core::GUID,
    pub Flags: u64,
    pub DeepestWakeableIdleState: u32,
    pub IdleStateCount: u32,
    pub IdleStates: PPO_FX_COMPONENT_IDLE_STATE,
    pub ProviderCount: u32,
    pub Providers: super::minwindef::PULONG,
}
#[cfg(feature = "bcrypt")]
pub type PO_FX_DEVICE = PO_FX_DEVICE_V1;
pub const PO_FX_DEVICE_FLAG_DFX_CHILDREN_OPTIONAL: u32 = 6;
pub const PO_FX_DEVICE_FLAG_DFX_DIRECT_CHILDREN_OPTIONAL: u64 = 2;
pub const PO_FX_DEVICE_FLAG_DFX_POWER_CHILDREN_OPTIONAL: u64 = 4;
pub const PO_FX_DEVICE_FLAG_DISABLE_FAST_RESUME: u64 = 8;
pub const PO_FX_DEVICE_FLAG_ENABLE_FAST_RESUME: u64 = 16;
pub const PO_FX_DEVICE_FLAG_NO_FAULT_CALLBACKS: u64 = 32;
pub const PO_FX_DEVICE_FLAG_RESERVED_1: u64 = 1;
pub const PO_FX_DEVICE_FLAG_RESERVED_2: u64 = 64;
pub type PO_FX_DEVICE_POWER_NOT_REQUIRED_CALLBACK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void)>;
pub type PO_FX_DEVICE_POWER_REQUIRED_CALLBACK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void)>;
#[repr(C)]
#[cfg(feature = "bcrypt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PO_FX_DEVICE_V1 {
    pub Version: u32,
    pub ComponentCount: u32,
    pub ComponentActiveConditionCallback: PPO_FX_COMPONENT_ACTIVE_CONDITION_CALLBACK,
    pub ComponentIdleConditionCallback: PPO_FX_COMPONENT_IDLE_CONDITION_CALLBACK,
    pub ComponentIdleStateCallback: PPO_FX_COMPONENT_IDLE_STATE_CALLBACK,
    pub DevicePowerRequiredCallback: PPO_FX_DEVICE_POWER_REQUIRED_CALLBACK,
    pub DevicePowerNotRequiredCallback: PPO_FX_DEVICE_POWER_NOT_REQUIRED_CALLBACK,
    pub PowerControlCallback: PPO_FX_POWER_CONTROL_CALLBACK,
    pub DeviceContext: *mut core::ffi::c_void,
    pub Components: [PO_FX_COMPONENT_V1; 1],
}
#[cfg(feature = "bcrypt")]
impl Default for PO_FX_DEVICE_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "minwindef"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PO_FX_DEVICE_V2 {
    pub Version: u32,
    pub Flags: u64,
    pub ComponentActiveConditionCallback: PPO_FX_COMPONENT_ACTIVE_CONDITION_CALLBACK,
    pub ComponentIdleConditionCallback: PPO_FX_COMPONENT_IDLE_CONDITION_CALLBACK,
    pub ComponentIdleStateCallback: PPO_FX_COMPONENT_IDLE_STATE_CALLBACK,
    pub DevicePowerRequiredCallback: PPO_FX_DEVICE_POWER_REQUIRED_CALLBACK,
    pub DevicePowerNotRequiredCallback: PPO_FX_DEVICE_POWER_NOT_REQUIRED_CALLBACK,
    pub PowerControlCallback: PPO_FX_POWER_CONTROL_CALLBACK,
    pub DeviceContext: *mut core::ffi::c_void,
    pub ComponentCount: u32,
    pub Components: [PO_FX_COMPONENT_V2; 1],
}
#[cfg(all(feature = "bcrypt", feature = "minwindef"))]
impl Default for PO_FX_DEVICE_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "minwindef"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PO_FX_DEVICE_V3 {
    pub Version: u32,
    pub Flags: u64,
    pub ComponentActiveConditionCallback: PPO_FX_COMPONENT_ACTIVE_CONDITION_CALLBACK,
    pub ComponentIdleConditionCallback: PPO_FX_COMPONENT_IDLE_CONDITION_CALLBACK,
    pub ComponentIdleStateCallback: PPO_FX_COMPONENT_IDLE_STATE_CALLBACK,
    pub DevicePowerRequiredCallback: PPO_FX_DEVICE_POWER_REQUIRED_CALLBACK,
    pub DevicePowerNotRequiredCallback: PPO_FX_DEVICE_POWER_NOT_REQUIRED_CALLBACK,
    pub PowerControlCallback: PPO_FX_POWER_CONTROL_CALLBACK,
    pub DirectedPowerUpCallback: PPO_FX_DIRECTED_POWER_UP_CALLBACK,
    pub DirectedPowerDownCallback: PPO_FX_DIRECTED_POWER_DOWN_CALLBACK,
    pub DirectedFxTimeoutInSeconds: u32,
    pub DeviceContext: *mut core::ffi::c_void,
    pub ComponentCount: u32,
    pub Components: [PO_FX_COMPONENT_V2; 1],
}
#[cfg(all(feature = "bcrypt", feature = "minwindef"))]
impl Default for PO_FX_DEVICE_V3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PO_FX_DIRECTED_FX_DEFAULT_IDLE_TIMEOUT: u32 = 0;
pub const PO_FX_DIRECTED_FX_IMMEDIATE_IDLE_TIMEOUT: u32 = 4294967295;
pub const PO_FX_DIRECTED_FX_MAX_IDLE_TIMEOUT: u32 = 600;
pub type PO_FX_DIRECTED_POWER_DOWN_CALLBACK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, flags: u32)>;
pub type PO_FX_DIRECTED_POWER_UP_CALLBACK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, flags: u32)>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PO_FX_DRIPS_WATCHDOG_CALLBACK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, physicaldeviceobject: *const DEVICE_OBJECT, uniqueid: u32)>;
pub const PO_FX_FLAG_ASYNC_ONLY: u32 = 2;
pub const PO_FX_FLAG_BLOCKING: u32 = 1;
pub const PO_FX_FLAG_PERF_PEP_OPTIONAL: u32 = 1;
pub const PO_FX_FLAG_PERF_QUERY_ON_ALL_IDLE_STATES: u32 = 4;
pub const PO_FX_FLAG_PERF_QUERY_ON_F0: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PO_FX_PERF_STATE {
    pub Value: u64,
    pub Context: *mut core::ffi::c_void,
}
impl Default for PO_FX_PERF_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PO_FX_PERF_STATE_CHANGE {
    pub Set: u32,
    pub Anonymous: PO_FX_PERF_STATE_CHANGE_0,
}
impl Default for PO_FX_PERF_STATE_CHANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PO_FX_PERF_STATE_CHANGE_0 {
    pub StateIndex: u32,
    pub StateValue: u64,
}
impl Default for PO_FX_PERF_STATE_CHANGE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PO_FX_PERF_STATE_TYPE = i32;
pub type PO_FX_PERF_STATE_UNIT = i32;
#[cfg(feature = "bcrypt")]
pub type PO_FX_POWER_CONTROL_CALLBACK = Option<unsafe extern "system" fn(devicecontext: *const core::ffi::c_void, powercontrolcode: *const windows_core::GUID, inbuffer: *const core::ffi::c_void, inbuffersize: usize, outbuffer: *mut core::ffi::c_void, outbuffersize: usize, bytesreturned: *mut usize) -> super::bcrypt::NTSTATUS>;
pub const PO_FX_UNKNOWN_POWER: u32 = 4294967295;
pub const PO_FX_UNKNOWN_TIME: u64 = 18446744073709551615;
pub const PO_FX_VERSION: u32 = 1;
pub const PO_FX_VERSION_V1: u32 = 1;
pub const PO_FX_VERSION_V2: u32 = 2;
pub const PO_FX_VERSION_V3: u32 = 3;
pub const PO_MEM_BOOT_PHASE: u32 = 65536;
pub const PO_MEM_CLONE: u32 = 2;
pub const PO_MEM_CL_OR_NCHK: u32 = 4;
pub const PO_MEM_DISCARD: u32 = 32768;
pub const PO_MEM_PAGE_ADDRESS: u32 = 16384;
pub const PO_MEM_PRESERVE: u32 = 1;
pub type PO_THERMAL_REQUEST_TYPE = i32;
#[cfg(feature = "winnt")]
pub type PPAGED_LOOKASIDE_LIST = *mut PAGED_LOOKASIDE_LIST;
pub type PPARTITION_INFORMATION_CLASS = *mut PARTITION_INFORMATION_CLASS;
pub type PPCI_ACS_BIT = *mut PCI_ACS_BIT;
#[cfg(feature = "bcrypt")]
pub type PPCI_ATS_INTERFACE = *mut PCI_ATS_INTERFACE;
pub type PPCI_CAPABILITIES_HEADER = *mut PCI_CAPABILITIES_HEADER;
pub type PPCI_COMMON_CONFIG = *mut PCI_COMMON_CONFIG;
pub type PPCI_COMMON_HEADER = *mut PCI_COMMON_HEADER;
pub type PPCI_CORRECTABLE_ERROR_MASK = *mut PCI_EXPRESS_CORRECTABLE_ERROR_MASK;
pub type PPCI_CORRECTABLE_ERROR_STATUS = *mut PCI_EXPRESS_CORRECTABLE_ERROR_STATUS;
pub type PPCI_DEVICE_PRESENCE_PARAMETERS = *mut PCI_DEVICE_PRESENCE_PARAMETERS;
pub type PPCI_DEVICE_PRESENT_INTERFACE = *mut PCI_DEVICE_PRESENT_INTERFACE;
#[cfg(feature = "bcrypt")]
pub type PPCI_DOE_INTERFACE = *mut PCI_DOE_INTERFACE;
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
pub type PPCI_DOE_INTERFACE2 = *mut PCI_DOE_INTERFACE2;
pub type PPCI_EXPRESS_ACS_CAPABILITY = *mut PCI_EXPRESS_ACS_CAPABILITY;
pub type PPCI_EXPRESS_ACS_CAPABILITY_REGISTER = *mut PCI_EXPRESS_ACS_CAPABILITY_REGISTER;
pub type PPCI_EXPRESS_ACS_CONTROL = *mut PCI_EXPRESS_ACS_CONTROL;
pub type PPCI_EXPRESS_AER_CAPABILITIES = *mut PCI_EXPRESS_AER_CAPABILITIES;
pub type PPCI_EXPRESS_AER_CAPABILITY = *mut PCI_EXPRESS_AER_CAPABILITY;
pub type PPCI_EXPRESS_ARI_CAPABILITY = *mut PCI_EXPRESS_ARI_CAPABILITY;
pub type PPCI_EXPRESS_ARI_CAPABILITY_REGISTER = *mut PCI_EXPRESS_ARI_CAPABILITY_REGISTER;
pub type PPCI_EXPRESS_ARI_CONTROL_REGISTER = *mut PCI_EXPRESS_ARI_CONTROL_REGISTER;
pub type PPCI_EXPRESS_ATS_CAPABILITY = *mut PCI_EXPRESS_ATS_CAPABILITY;
pub type PPCI_EXPRESS_ATS_CAPABILITY_REGISTER = *mut PCI_EXPRESS_ATS_CAPABILITY_REGISTER;
pub type PPCI_EXPRESS_ATS_CONTROL_REGISTER = *mut PCI_EXPRESS_ATS_CONTROL_REGISTER;
pub type PPCI_EXPRESS_BRIDGE_AER_CAPABILITY = *mut PCI_EXPRESS_BRIDGE_AER_CAPABILITY;
pub type PPCI_EXPRESS_DEVICE_3_EXTENDED_CAPABILITY = *mut PCI_EXPRESS_DEVICE_3_EXTENDED_CAPABILITY;
pub type PPCI_EXPRESS_DEVICE_CAPABILITIES_3_REGISTER = *mut PCI_EXPRESS_DEVICE_CAPABILITIES_3_REGISTER;
pub type PPCI_EXPRESS_DEVICE_CONTROL_3_REGISTER = *mut PCI_EXPRESS_DEVICE_CONTROL_3_REGISTER;
pub type PPCI_EXPRESS_ENHANCED_CAPABILITY_HEADER = *mut PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER;
#[cfg(feature = "bcrypt")]
pub type PPCI_EXPRESS_ENTER_LINK_QUIESCENT_MODE = *mut PCI_EXPRESS_ENTER_LINK_QUIESCENT_MODE;
pub type PPCI_EXPRESS_ERROR_SOURCE_ID = *mut PCI_EXPRESS_ERROR_SOURCE_ID;
#[cfg(feature = "bcrypt")]
pub type PPCI_EXPRESS_EXIT_LINK_QUIESCENT_MODE = *mut PCI_EXPRESS_EXIT_LINK_QUIESCENT_MODE;
#[cfg(feature = "bcrypt")]
pub type PPCI_EXPRESS_LINK_QUIESCENT_INTERFACE = *mut PCI_EXPRESS_LINK_QUIESCENT_INTERFACE;
pub type PPCI_EXPRESS_PASID_CAPABILITY = *mut PCI_EXPRESS_PASID_CAPABILITY;
pub type PPCI_EXPRESS_PASID_CAPABILITY_REGISTER = *mut PCI_EXPRESS_PASID_CAPABILITY_REGISTER;
pub type PPCI_EXPRESS_PASID_CONTROL_REGISTER = *mut PCI_EXPRESS_PASID_CONTROL_REGISTER;
pub type PPCI_EXPRESS_PRI_CAPABILITY = *mut PCI_EXPRESS_PRI_CAPABILITY;
pub type PPCI_EXPRESS_PRI_CONTROL_REGISTER = *mut PCI_EXPRESS_PRI_CONTROL_REGISTER;
pub type PPCI_EXPRESS_PRI_STATUS_REGISTER = *mut PCI_EXPRESS_PRI_STATUS_REGISTER;
pub type PPCI_EXPRESS_PTM_CAPABILITY = *mut PCI_EXPRESS_PTM_CAPABILITY;
pub type PPCI_EXPRESS_PTM_CAPABILITY_REGISTER = *mut PCI_EXPRESS_PTM_CAPABILITY_REGISTER;
pub type PPCI_EXPRESS_PTM_CONTROL_REGISTER = *mut PCI_EXPRESS_PTM_CONTROL_REGISTER;
pub type PPCI_EXPRESS_ROOTPORT_AER_CAPABILITY = *mut PCI_EXPRESS_ROOTPORT_AER_CAPABILITY;
pub type PPCI_EXPRESS_ROOT_ERROR_COMMAND = *mut PCI_EXPRESS_ROOT_ERROR_COMMAND;
pub type PPCI_EXPRESS_ROOT_ERROR_STATUS = *mut PCI_EXPRESS_ROOT_ERROR_STATUS;
pub type PPCI_EXPRESS_ROOT_PORT_INTERFACE = *mut PCI_EXPRESS_ROOT_PORT_INTERFACE;
pub type PPCI_EXPRESS_ROOT_PORT_READ_CONFIG_SPACE = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, buffer: *mut core::ffi::c_void, offset: u32, length: u32) -> u32>;
pub type PPCI_EXPRESS_ROOT_PORT_WRITE_CONFIG_SPACE = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, buffer: *const core::ffi::c_void, offset: u32, length: u32) -> u32>;
pub type PPCI_EXPRESS_SEC_AER_CAPABILITIES = *mut PCI_EXPRESS_SEC_AER_CAPABILITIES;
pub type PPCI_EXPRESS_SEC_UNCORRECTABLE_ERROR_MASK = *mut PCI_EXPRESS_SEC_UNCORRECTABLE_ERROR_MASK;
pub type PPCI_EXPRESS_SEC_UNCORRECTABLE_ERROR_SEVERITY = *mut PCI_EXPRESS_SEC_UNCORRECTABLE_ERROR_SEVERITY;
pub type PPCI_EXPRESS_SEC_UNCORRECTABLE_ERROR_STATUS = *mut PCI_EXPRESS_SEC_UNCORRECTABLE_ERROR_STATUS;
pub type PPCI_EXPRESS_SERIAL_NUMBER_CAPABILITY = *mut PCI_EXPRESS_SERIAL_NUMBER_CAPABILITY;
pub type PPCI_EXPRESS_SRIOV_CAPABILITY = *mut PCI_EXPRESS_SRIOV_CAPABILITY;
pub type PPCI_EXPRESS_SRIOV_CAPS = *mut PCI_EXPRESS_SRIOV_CAPS;
pub type PPCI_EXPRESS_SRIOV_CONTROL = *mut PCI_EXPRESS_SRIOV_CONTROL;
pub type PPCI_EXPRESS_SRIOV_MIGRATION_STATE_ARRAY = *mut PCI_EXPRESS_SRIOV_MIGRATION_STATE_ARRAY;
pub type PPCI_EXPRESS_SRIOV_STATUS = *mut PCI_EXPRESS_SRIOV_STATUS;
pub type PPCI_EXPRESS_UNCORRECTABLE_ERROR_MASK = *mut PCI_EXPRESS_UNCORRECTABLE_ERROR_MASK;
pub type PPCI_EXPRESS_UNCORRECTABLE_ERROR_SEVERITY = *mut PCI_EXPRESS_UNCORRECTABLE_ERROR_SEVERITY;
pub type PPCI_EXPRESS_UNCORRECTABLE_ERROR_STATUS = *mut PCI_EXPRESS_UNCORRECTABLE_ERROR_STATUS;
pub type PPCI_EXPRESS_VENDOR_SPECIFIC_CAPABILITY = *mut PCI_EXPRESS_VENDOR_SPECIFIC_CAPABILITY;
pub type PPCI_EXPRESS_VIRTUAL_CHANNEL_CAPABILITY = *mut PCI_EXPRESS_VIRTUAL_CHANNEL_CAPABILITY;
pub type PPCI_IS_DEVICE_PRESENT = *mut PCI_IS_DEVICE_PRESENT;
pub type PPCI_IS_DEVICE_PRESENT_EX = *mut PCI_IS_DEVICE_PRESENT_EX;
#[cfg(feature = "bcrypt")]
pub type PPCI_MSIX_GET_ENTRY = *mut PCI_MSIX_GET_ENTRY;
#[cfg(feature = "bcrypt")]
pub type PPCI_MSIX_GET_TABLE_SIZE = *mut PCI_MSIX_GET_TABLE_SIZE;
#[cfg(feature = "bcrypt")]
pub type PPCI_MSIX_MASKUNMASK_ENTRY = *mut PCI_MSIX_MASKUNMASK_ENTRY;
#[cfg(feature = "bcrypt")]
pub type PPCI_MSIX_SET_ENTRY = *mut PCI_MSIX_SET_ENTRY;
#[cfg(feature = "bcrypt")]
pub type PPCI_MSIX_TABLE_CONFIG_INTERFACE = *mut PCI_MSIX_TABLE_CONFIG_INTERFACE;
pub type PPCI_PMC = *mut PCI_PMC;
pub type PPCI_PMCSR = *mut PCI_PMCSR;
pub type PPCI_PMCSR_BSE = *mut PCI_PMCSR_BSE;
pub type PPCI_PM_CAPABILITY = *mut PCI_PM_CAPABILITY;
#[cfg(feature = "bcrypt")]
pub type PPCI_SECURITY_INTERFACE = *mut PCI_SECURITY_INTERFACE;
#[cfg(feature = "bcrypt")]
pub type PPCI_SECURITY_INTERFACE2 = *mut PCI_SECURITY_INTERFACE2;
pub type PPCI_SEGMENT_BUS_NUMBER = *mut PCI_SEGMENT_BUS_NUMBER;
#[cfg(feature = "bcrypt")]
pub type PPCI_SET_ACS = *mut PCI_SET_ACS;
#[cfg(feature = "bcrypt")]
pub type PPCI_SET_ACS2 = *mut PCI_SET_ACS2;
#[cfg(feature = "bcrypt")]
pub type PPCI_SET_ACS3 = *mut PCI_SET_ACS3;
#[cfg(feature = "bcrypt")]
pub type PPCI_SET_ATS = *mut PCI_SET_ATS;
pub type PPCI_SLOT_NUMBER = *mut PCI_SLOT_NUMBER;
#[cfg(feature = "bcrypt")]
pub type PPCI_VIRTUALIZATION_INTERFACE = *mut PCI_VIRTUALIZATION_INTERFACE;
pub type PPCI_X_CAPABILITY = *mut PCI_X_CAPABILITY;
pub type PPCW_BUFFER = *mut _PCW_BUFFER;
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
pub type PPCW_CALLBACK = Option<unsafe extern "system" fn(r#type: PCW_CALLBACK_TYPE, info: *const PCW_CALLBACK_INFORMATION, context: *const core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
pub type PPCW_CALLBACK_INFORMATION = *mut PCW_CALLBACK_INFORMATION;
pub type PPCW_CALLBACK_TYPE = *mut PCW_CALLBACK_TYPE;
pub type PPCW_COUNTER_DESCRIPTOR = *mut PCW_COUNTER_DESCRIPTOR;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winternl"))]
pub type PPCW_COUNTER_INFORMATION = *mut PCW_COUNTER_INFORMATION;
pub type PPCW_DATA = *mut PCW_DATA;
pub type PPCW_INSTANCE = *mut _PCW_INSTANCE;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
pub type PPCW_MASK_INFORMATION = *mut PCW_MASK_INFORMATION;
pub type PPCW_REGISTRATION = *mut _PCW_REGISTRATION;
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt", feature = "winternl"))]
pub type PPCW_REGISTRATION_INFORMATION = *mut PCW_REGISTRATION_INFORMATION;
#[cfg(target_arch = "x86")]
pub type PPFN_NUMBER = *mut u32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type PPFN_NUMBER = *mut u64;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PPLOG_FILE_OBJECT = *mut *mut FILE_OBJECT;
pub type PPLUGPLAY_NOTIFICATION_HEADER = *mut PLUGPLAY_NOTIFICATION_HEADER;
pub type PPNP_BUS_INFORMATION = *mut PNP_BUS_INFORMATION;
pub type PPNP_DEVICE_STATE = *mut u32;
pub type PPNP_EXTENDED_ADDRESS_INTERFACE = *mut PNP_EXTENDED_ADDRESS_INTERFACE;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "usb"))]
pub type PPNP_REPLACE_DRIVER_INTERFACE = *mut PNP_REPLACE_DRIVER_INTERFACE;
#[cfg(feature = "usb")]
pub type PPNP_REPLACE_MEMORY_LIST = *mut PNP_REPLACE_MEMORY_LIST;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "usb"))]
pub type PPNP_REPLACE_PARAMETERS = *mut PNP_REPLACE_PARAMETERS;
#[cfg(feature = "basetsd")]
pub type PPNP_REPLACE_PROCESSOR_LIST = *mut PNP_REPLACE_PROCESSOR_LIST;
#[cfg(feature = "basetsd")]
pub type PPNP_REPLACE_PROCESSOR_LIST_V1 = *mut PNP_REPLACE_PROCESSOR_LIST_V1;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
pub type PPOOL_CREATE_EXTENDED_PARAMETER = *mut POOL_CREATE_EXTENDED_PARAMETER;
pub type PPOOL_CREATE_EXTENDED_PARAMETER_TYPE = *mut POOL_CREATE_EXTENDED_PARAMETER_TYPE;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
pub type PPOOL_CREATE_EXTENDED_PARAMS = *mut POOL_CREATE_EXTENDED_PARAMS;
#[cfg(feature = "winnt")]
pub type PPOOL_EXTENDED_PARAMETER = *mut POOL_EXTENDED_PARAMETER;
pub type PPOOL_EXTENDED_PARAMETER_TYPE = *mut POOL_EXTENDED_PARAMETER_TYPE;
pub type PPOWER_SEQUENCE = *mut POWER_SEQUENCE;
#[cfg(feature = "bcrypt")]
pub type PPOWER_SETTING_CALLBACK = *mut POWER_SETTING_CALLBACK;
#[cfg(feature = "winnt")]
pub type PPOWER_STATE = *mut POWER_STATE;
pub type PPOWER_STATE_TYPE = *mut POWER_STATE_TYPE;
pub type PPO_EFFECTIVE_POWER_MODE = *mut PO_EFFECTIVE_POWER_MODE;
pub type PPO_EFFECTIVE_POWER_MODE_CALLBACK = *mut PO_EFFECTIVE_POWER_MODE_CALLBACK;
pub type PPO_FX_COMPONENT = *mut PO_FX_COMPONENT_V1;
pub type PPO_FX_COMPONENT_ACTIVE_CONDITION_CALLBACK = *mut PO_FX_COMPONENT_ACTIVE_CONDITION_CALLBACK;
pub type PPO_FX_COMPONENT_CRITICAL_TRANSITION_CALLBACK = *mut PO_FX_COMPONENT_CRITICAL_TRANSITION_CALLBACK;
pub type PPO_FX_COMPONENT_IDLE_CONDITION_CALLBACK = *mut PO_FX_COMPONENT_IDLE_CONDITION_CALLBACK;
pub type PPO_FX_COMPONENT_IDLE_STATE = *mut PO_FX_COMPONENT_IDLE_STATE;
pub type PPO_FX_COMPONENT_IDLE_STATE_CALLBACK = *mut PO_FX_COMPONENT_IDLE_STATE_CALLBACK;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
pub type PPO_FX_COMPONENT_PERF_INFO = *mut PO_FX_COMPONENT_PERF_INFO;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
pub type PPO_FX_COMPONENT_PERF_SET = *mut PO_FX_COMPONENT_PERF_SET;
pub type PPO_FX_COMPONENT_PERF_STATE_CALLBACK = *mut PO_FX_COMPONENT_PERF_STATE_CALLBACK;
pub type PPO_FX_COMPONENT_V1 = *mut PO_FX_COMPONENT_V1;
#[cfg(feature = "minwindef")]
pub type PPO_FX_COMPONENT_V2 = *mut PO_FX_COMPONENT_V2;
#[cfg(feature = "bcrypt")]
pub type PPO_FX_DEVICE = *mut PO_FX_DEVICE_V1;
pub type PPO_FX_DEVICE_POWER_NOT_REQUIRED_CALLBACK = *mut PO_FX_DEVICE_POWER_NOT_REQUIRED_CALLBACK;
pub type PPO_FX_DEVICE_POWER_REQUIRED_CALLBACK = *mut PO_FX_DEVICE_POWER_REQUIRED_CALLBACK;
#[cfg(feature = "bcrypt")]
pub type PPO_FX_DEVICE_V1 = *mut PO_FX_DEVICE_V1;
#[cfg(all(feature = "bcrypt", feature = "minwindef"))]
pub type PPO_FX_DEVICE_V2 = *mut PO_FX_DEVICE_V2;
#[cfg(all(feature = "bcrypt", feature = "minwindef"))]
pub type PPO_FX_DEVICE_V3 = *mut PO_FX_DEVICE_V3;
pub type PPO_FX_DIRECTED_POWER_DOWN_CALLBACK = *mut PO_FX_DIRECTED_POWER_DOWN_CALLBACK;
pub type PPO_FX_DIRECTED_POWER_UP_CALLBACK = *mut PO_FX_DIRECTED_POWER_UP_CALLBACK;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PPO_FX_DRIPS_WATCHDOG_CALLBACK = *mut PO_FX_DRIPS_WATCHDOG_CALLBACK;
pub type PPO_FX_PERF_STATE = *mut PO_FX_PERF_STATE;
pub type PPO_FX_PERF_STATE_CHANGE = *mut PO_FX_PERF_STATE_CHANGE;
pub type PPO_FX_PERF_STATE_TYPE = *mut PO_FX_PERF_STATE_TYPE;
pub type PPO_FX_PERF_STATE_UNIT = *mut PO_FX_PERF_STATE_UNIT;
#[cfg(feature = "bcrypt")]
pub type PPO_FX_POWER_CONTROL_CALLBACK = *mut PO_FX_POWER_CONTROL_CALLBACK;
pub type PPO_THERMAL_REQUEST_TYPE = *mut PO_THERMAL_REQUEST_TYPE;
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
pub type PPROCESSOR_CALLBACK_FUNCTION = *mut PROCESSOR_CALLBACK_FUNCTION;
#[cfg(feature = "bcrypt")]
pub type PPROCESSOR_HALT_ROUTINE = *mut PROCESSOR_HALT_ROUTINE;
#[cfg(feature = "bcrypt")]
pub type PPTM_CONTROL_INTERFACE = *mut PTM_CONTROL_INTERFACE;
#[cfg(feature = "bcrypt")]
pub type PPTM_DEVICE_DISABLE = *mut PTM_DEVICE_DISABLE;
#[cfg(feature = "bcrypt")]
pub type PPTM_DEVICE_ENABLE = *mut PTM_DEVICE_ENABLE;
#[cfg(feature = "bcrypt")]
pub type PPTM_DEVICE_QUERY_GRANULARITY = *mut PTM_DEVICE_QUERY_GRANULARITY;
#[cfg(feature = "bcrypt")]
pub type PPTM_DEVICE_QUERY_TIME_SOURCE = *mut PTM_DEVICE_QUERY_TIME_SOURCE;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PPUT_DMA_ADAPTER = Option<unsafe extern "system" fn(dmaadapter: *mut DMA_ADAPTER)>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PPUT_SCATTER_GATHER_LIST = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, scattergather: *const SCATTER_GATHER_LIST, writetodevice: bool)>;
pub type PQUERYEXTENDEDADDRESS = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, extendedaddress: *mut u64)>;
#[cfg(feature = "bcrypt")]
pub type PQUERY_SUPPORTED_DOE_PROTOCOLS = *mut QUERY_SUPPORTED_DOE_PROTOCOLS;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PREAD_DMA_COUNTER = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER) -> u32>;
pub type PREENUMERATE_SELF = Option<unsafe extern "system" fn(context: *const core::ffi::c_void)>;
pub type PREENUMERATE_SELF_INTERFACE_STANDARD = *mut REENUMERATE_SELF_INTERFACE_STANDARD;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PREGISTER_FOR_DEVICE_NOTIFICATIONS = Option<unsafe extern "system" fn(param0: *mut DEVICE_OBJECT, param1: PDEVICE_NOTIFY_CALLBACK, param2: *mut core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "bcrypt")]
pub type PREGISTER_FOR_DEVICE_NOTIFICATIONS2 = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, notificationhandler: PDEVICE_NOTIFY_CALLBACK2, notificationcontext: *mut core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
pub type PREG_CALLBACK_CONTEXT_CLEANUP_INFORMATION = *mut REG_CALLBACK_CONTEXT_CLEANUP_INFORMATION;
#[cfg(all(feature = "lsalookup", feature = "minwindef", feature = "ntsecapi", feature = "winnt"))]
pub type PREG_CREATE_KEY_INFORMATION = *mut REG_CREATE_KEY_INFORMATION;
#[cfg(all(feature = "lsalookup", feature = "minwindef", feature = "ntsecapi", feature = "winnt"))]
pub type PREG_CREATE_KEY_INFORMATION_V1 = *mut REG_CREATE_KEY_INFORMATION_V1;
pub type PREG_DELETE_KEY_INFORMATION = *mut REG_DELETE_KEY_INFORMATION;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
pub type PREG_DELETE_VALUE_KEY_INFORMATION = *mut REG_DELETE_VALUE_KEY_INFORMATION;
#[cfg(feature = "minwindef")]
pub type PREG_ENUMERATE_KEY_INFORMATION = *mut REG_ENUMERATE_KEY_INFORMATION;
#[cfg(feature = "minwindef")]
pub type PREG_ENUMERATE_VALUE_KEY_INFORMATION = *mut REG_ENUMERATE_VALUE_KEY_INFORMATION;
pub type PREG_FLUSH_KEY_INFORMATION = *mut REG_DELETE_KEY_INFORMATION;
pub type PREG_KEY_HANDLE_CLOSE_INFORMATION = *mut REG_KEY_HANDLE_CLOSE_INFORMATION;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
pub type PREG_LOAD_KEY_INFORMATION = *mut REG_LOAD_KEY_INFORMATION;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
pub type PREG_LOAD_KEY_INFORMATION_V2 = *mut REG_LOAD_KEY_INFORMATION_V2;
#[cfg(all(feature = "lsalookup", feature = "minwindef", feature = "ntsecapi", feature = "winnt"))]
pub type PREG_OPEN_KEY_INFORMATION = *mut REG_CREATE_KEY_INFORMATION;
#[cfg(all(feature = "lsalookup", feature = "minwindef", feature = "ntsecapi", feature = "winnt"))]
pub type PREG_OPEN_KEY_INFORMATION_V1 = *mut REG_CREATE_KEY_INFORMATION_V1;
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi"))]
pub type PREG_POST_CREATE_KEY_INFORMATION = *mut REG_POST_CREATE_KEY_INFORMATION;
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi"))]
pub type PREG_POST_OPEN_KEY_INFORMATION = *mut REG_POST_CREATE_KEY_INFORMATION;
#[cfg(feature = "bcrypt")]
pub type PREG_POST_OPERATION_INFORMATION = *mut REG_POST_OPERATION_INFORMATION;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
pub type PREG_PRE_CREATE_KEY_INFORMATION = *mut REG_PRE_CREATE_KEY_INFORMATION;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
pub type PREG_PRE_OPEN_KEY_INFORMATION = *mut REG_PRE_CREATE_KEY_INFORMATION;
#[cfg(feature = "minwindef")]
pub type PREG_QUERY_KEY_INFORMATION = *mut REG_QUERY_KEY_INFORMATION;
#[cfg(all(feature = "lsalookup", feature = "minwindef", feature = "ntsecapi"))]
pub type PREG_QUERY_KEY_NAME = *mut REG_QUERY_KEY_NAME;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PREG_QUERY_KEY_SECURITY_INFORMATION = *mut REG_QUERY_KEY_SECURITY_INFORMATION;
#[cfg(all(feature = "lsalookup", feature = "minwindef", feature = "ntsecapi", feature = "winternl"))]
pub type PREG_QUERY_MULTIPLE_VALUE_KEY_INFORMATION = *mut REG_QUERY_MULTIPLE_VALUE_KEY_INFORMATION;
#[cfg(all(feature = "lsalookup", feature = "minwindef", feature = "ntsecapi"))]
pub type PREG_QUERY_VALUE_KEY_INFORMATION = *mut REG_QUERY_VALUE_KEY_INFORMATION;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
pub type PREG_RENAME_KEY_INFORMATION = *mut REG_RENAME_KEY_INFORMATION;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
pub type PREG_REPLACE_KEY_INFORMATION = *mut REG_REPLACE_KEY_INFORMATION;
#[cfg(feature = "winnt")]
pub type PREG_RESTORE_KEY_INFORMATION = *mut REG_RESTORE_KEY_INFORMATION;
#[cfg(feature = "winnt")]
pub type PREG_SAVE_KEY_INFORMATION = *mut REG_SAVE_KEY_INFORMATION;
#[cfg(feature = "winnt")]
pub type PREG_SAVE_MERGED_KEY_INFORMATION = *mut REG_SAVE_MERGED_KEY_INFORMATION;
#[cfg(feature = "winternl")]
pub type PREG_SET_INFORMATION_KEY_INFORMATION = *mut REG_SET_INFORMATION_KEY_INFORMATION;
#[cfg(feature = "winnt")]
pub type PREG_SET_KEY_SECURITY_INFORMATION = *mut REG_SET_KEY_SECURITY_INFORMATION;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
pub type PREG_SET_VALUE_KEY_INFORMATION = *mut REG_SET_VALUE_KEY_INFORMATION;
pub type PREG_UNLOAD_KEY_INFORMATION = *mut REG_UNLOAD_KEY_INFORMATION;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "usb"))]
pub type PREPLACE_BEGIN = Option<unsafe extern "system" fn(parameters: *const PNP_REPLACE_PARAMETERS, context: *mut *mut core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "usb"))]
pub type PREPLACE_DRIVER_INIT = Option<unsafe extern "system" fn(interface: *mut PNP_REPLACE_DRIVER_INTERFACE, unused: *const core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "bcrypt")]
pub type PREPLACE_ENABLE_DISABLE_HARDWARE_QUIESCE = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, enable: bool) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "bcrypt")]
pub type PREPLACE_END = Option<unsafe extern "system" fn(context: *const core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "usb"))]
pub type PREPLACE_GET_MEMORY_DESTINATION = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, sourceaddress: super::usb::PHYSICAL_ADDRESS, destinationaddress: *mut i64) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "bcrypt")]
pub type PREPLACE_INITIATE_HARDWARE_MIRROR = Option<unsafe extern "system" fn(context: *const core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "usb"))]
pub type PREPLACE_MAP_MEMORY = Option<unsafe extern "system" fn(targetphysicaladdress: super::usb::PHYSICAL_ADDRESS, sparephysicaladdress: super::usb::PHYSICAL_ADDRESS, numberofbytes: *mut i64, targetaddress: *mut *mut core::ffi::c_void, spareaddress: *mut *mut core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "usb"))]
pub type PREPLACE_MIRROR_PHYSICAL_MEMORY = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, physicaladdress: super::usb::PHYSICAL_ADDRESS, bytecount: i64) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "bcrypt")]
pub type PREPLACE_MIRROR_PLATFORM_MEMORY = Option<unsafe extern "system" fn(context: *const core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "bcrypt")]
pub type PREPLACE_SET_PROCESSOR_ID = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, apicid: u32, target: bool) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "bcrypt")]
pub type PREPLACE_SWAP = Option<unsafe extern "system" fn(context: *const core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
pub type PREPLACE_UNLOAD = Option<unsafe extern "system" fn()>;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PREQUEST_POWER_COMPLETE = *mut REQUEST_POWER_COMPLETE;
pub type PRESET_DOE_INSTANCES = *mut RESET_DOE_INSTANCES;
#[cfg(feature = "winnt")]
pub type PRESOURCE_HASH_ENTRY = *mut RESOURCE_HASH_ENTRY;
#[cfg(feature = "winnt")]
pub type PRESOURCE_PERFORMANCE_DATA = *mut RESOURCE_PERFORMANCE_DATA;
#[cfg(feature = "winnt")]
pub type PRKAPC = *mut KAPC;
#[cfg(all(feature = "ntdef", feature = "winnt"))]
pub type PRKDEVICE_QUEUE = *mut KDEVICE_QUEUE;
#[cfg(feature = "winnt")]
pub type PRKDEVICE_QUEUE_ENTRY = *mut KDEVICE_QUEUE_ENTRY;
#[cfg(all(feature = "basetsd", feature = "winnt"))]
pub type PRKDPC = *mut KDPC;
pub type PRKENLISTMENT = *mut KENLISTMENT;
#[cfg(feature = "winnt")]
pub type PRKEVENT = *mut KEVENT;
#[cfg(feature = "winnt")]
pub type PRKMUTANT = *mut KMUTANT;
#[cfg(feature = "winnt")]
pub type PRKMUTEX = *mut KMUTANT;
pub type PRKPROCESS = *mut _KPROCESS;
pub type PRKRESOURCEMANAGER = *mut KRESOURCEMANAGER;
#[cfg(feature = "winnt")]
pub type PRKSEMAPHORE = *mut KSEMAPHORE;
pub type PRKTHREAD = *mut _KTHREAD;
#[cfg(all(feature = "basetsd", feature = "winnt"))]
pub type PRKTIMER = *mut KTIMER;
pub type PRKTM = *mut KTM;
pub type PRKTRANSACTION = *mut KTRANSACTION;
#[cfg(all(feature = "basetsd", feature = "ntifs", feature = "winnt"))]
pub type PRKWAIT_BLOCK = *mut KWAIT_BLOCK;
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
pub type PROCESSOR_CALLBACK_FUNCTION = Option<unsafe extern "system" fn(callbackcontext: *const core::ffi::c_void, changecontext: *const KE_PROCESSOR_CHANGE_NOTIFY_CONTEXT, operationstatus: *mut super::bcrypt::NTSTATUS)>;
pub const PROCESSOR_FEATURE_MAX: u32 = 64;
#[cfg(feature = "bcrypt")]
pub type PROCESSOR_HALT_ROUTINE = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
#[cfg(target_arch = "x86")]
pub const PROFILE_LEVEL: u32 = 27;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const PROFILE_LEVEL: u32 = 15;
#[cfg(feature = "minwindef")]
pub type PRTL_BITMAP = *mut RTL_BITMAP;
pub type PRTL_BITMAP_RUN = *mut RTL_BITMAP_RUN;
#[cfg(feature = "bcrypt")]
pub type PRTL_QUERY_REGISTRY_ROUTINE = Option<unsafe extern "system" fn(valuename: windows_core::PCWSTR, valuetype: u32, valuedata: *const core::ffi::c_void, valuelength: u32, context: *const core::ffi::c_void, entrycontext: *const core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "bcrypt")]
pub type PRTL_QUERY_REGISTRY_TABLE = *mut RTL_QUERY_REGISTRY_TABLE;
#[cfg(feature = "usb")]
pub type PSCATTER_GATHER_ELEMENT = *mut SCATTER_GATHER_ELEMENT;
#[cfg(feature = "usb")]
pub type PSCATTER_GATHER_LIST = *mut SCATTER_GATHER_LIST;
pub type PSDEV_IDENTIFIER_INTERFACE = *mut SDEV_IDENTIFIER_INTERFACE;
pub type PSECTION_OBJECT_POINTERS = *mut SECTION_OBJECT_POINTERS;
pub type PSECURE_DRIVER_INTERFACE = *mut SECURE_DRIVER_INTERFACE;
pub type PSECURE_DRIVER_PROCESS_DEREFERENCE = *mut SECURE_DRIVER_PROCESS_DEREFERENCE;
pub type PSECURE_DRIVER_PROCESS_REFERENCE = *mut SECURE_DRIVER_PROCESS_REFERENCE;
pub type PSECURITY_OPERATION_CODE = *mut SECURITY_OPERATION_CODE;
#[cfg(feature = "winnt")]
pub type PSECURITY_SUBJECT_CONTEXT = *mut SECURITY_SUBJECT_CONTEXT;
#[cfg(feature = "bcrypt")]
pub type PSEND_DOE_REQUEST = *mut SEND_DOE_REQUEST;
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
pub type PSEND_DOE_REQUEST_ASYNC = *mut SEND_DOE_REQUEST_ASYNC;
pub type PSET_D3COLD_SUPPORT = *mut SET_D3COLD_SUPPORT;
pub type PSET_VIRTUAL_DEVICE_DATA = *mut SET_VIRTUAL_DEVICE_DATA;
pub type PSE_IMAGE_TYPE = *mut SE_IMAGE_TYPE;
#[cfg(all(feature = "lsalookup", feature = "ntddk", feature = "ntsecapi"))]
pub type PSE_IMAGE_VERIFICATION_CALLBACK_FUNCTION = *mut SE_IMAGE_VERIFICATION_CALLBACK_FUNCTION;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSE_IMAGE_VERIFICATION_CALLBACK_TOKEN(pub *mut *mut core::ffi::c_void);
impl Default for PSE_IMAGE_VERIFICATION_CALLBACK_TOKEN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PSE_IMAGE_VERIFICATION_CALLBACK_TYPE = *mut SE_IMAGE_VERIFICATION_CALLBACK_TYPE;
pub type PSHARE_ACCESS = *mut SHARE_ACCESS;
#[cfg(target_arch = "x86")]
pub type PSPFN_NUMBER = *mut i32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type PSPFN_NUMBER = *mut i64;
pub type PSYSTEM_POWER_STATE_CONTEXT = *mut SYSTEM_POWER_STATE_CONTEXT;
pub type PS_AVAILABLE_CPUS_CHANGE_CALLBACK = Option<unsafe extern "system" fn(parameter: *const core::ffi::c_void)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PS_AVAILABLE_CPUS_CHANGE_REGISTRATION(pub *mut core::ffi::c_void);
impl Default for PS_AVAILABLE_CPUS_CHANGE_REGISTRATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PTARGET_DEVICE_CUSTOM_NOTIFICATION = *mut TARGET_DEVICE_CUSTOM_NOTIFICATION;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PTARGET_DEVICE_REMOVAL_NOTIFICATION = *mut TARGET_DEVICE_REMOVAL_NOTIFICATION;
#[cfg(feature = "ntdef")]
pub type PTIME_FIELDS = *mut TIME_FIELDS;
#[repr(C)]
#[cfg(feature = "bcrypt")]
#[derive(Clone, Copy, Debug)]
pub struct PTM_CONTROL_INTERFACE {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
    pub QueryGranularity: PPTM_DEVICE_QUERY_GRANULARITY,
    pub QueryTimeSource: PPTM_DEVICE_QUERY_TIME_SOURCE,
    pub Enable: PPTM_DEVICE_ENABLE,
    pub Disable: PPTM_DEVICE_DISABLE,
}
#[cfg(feature = "bcrypt")]
impl Default for PTM_CONTROL_INTERFACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "bcrypt")]
pub type PTM_DEVICE_DISABLE = Option<unsafe extern "system" fn(context: *const core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "bcrypt")]
pub type PTM_DEVICE_ENABLE = Option<unsafe extern "system" fn(context: *const core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "bcrypt")]
pub type PTM_DEVICE_QUERY_GRANULARITY = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, granularity: *mut u8) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "bcrypt")]
pub type PTM_DEVICE_QUERY_TIME_SOURCE = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, timesource: *mut u32) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "bcrypt")]
pub type PTM_PROPAGATE_ROUTINE = Option<unsafe extern "system" fn(propagationcookie: *const core::ffi::c_void, callbackdata: *const core::ffi::c_void, propagationstatus: super::bcrypt::NTSTATUS, transactionguid: windows_core::GUID) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "bcrypt")]
pub type PTM_RM_NOTIFICATION = Option<unsafe extern "system" fn(enlistmentobject: *const KENLISTMENT, rmcontext: *const core::ffi::c_void, transactioncontext: *const core::ffi::c_void, transactionnotification: u32, tmvirtualclock: *mut i64, argumentlength: u32, argument: *const core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
#[cfg(feature = "usb")]
pub type PTRANSLATE_BUS_ADDRESS = *mut TRANSLATE_BUS_ADDRESS;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PUNREGISTER_FOR_DEVICE_NOTIFICATIONS = Option<unsafe extern "system" fn(param0: *mut DEVICE_OBJECT, param1: PDEVICE_NOTIFY_CALLBACK)>;
pub type PUNREGISTER_FOR_DEVICE_NOTIFICATIONS2 = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void)>;
pub type PVIRTUAL_CHANNEL_CAPABILITIES1 = *mut VIRTUAL_CHANNEL_CAPABILITIES1;
pub type PVIRTUAL_CHANNEL_CAPABILITIES2 = *mut VIRTUAL_CHANNEL_CAPABILITIES2;
pub type PVIRTUAL_CHANNEL_CONTROL = *mut VIRTUAL_CHANNEL_CONTROL;
pub type PVIRTUAL_CHANNEL_STATUS = *mut VIRTUAL_CHANNEL_STATUS;
pub type PVIRTUAL_RESOURCE = *mut VIRTUAL_RESOURCE;
pub type PVIRTUAL_RESOURCE_CAPABILITY = *mut VIRTUAL_RESOURCE_CAPABILITY;
pub type PVIRTUAL_RESOURCE_CONTROL = *mut VIRTUAL_RESOURCE_CONTROL;
pub type PVIRTUAL_RESOURCE_STATUS = *mut VIRTUAL_RESOURCE_STATUS;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PVPB = *mut VPB;
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type PWAIT_CONTEXT_BLOCK = *mut WAIT_CONTEXT_BLOCK;
pub type PWORKER_THREAD_ROUTINE = *mut WORKER_THREAD_ROUTINE;
#[cfg(feature = "winnt")]
pub type PWORK_QUEUE_ITEM = *mut WORK_QUEUE_ITEM;
#[cfg(feature = "winnt")]
pub type PXSTATE_SAVE = *mut XSTATE_SAVE;
pub const PageIn: KWAIT_REASON = 2;
pub const PagedPool: POOL_TYPE = 1;
pub const PagedPoolCacheAligned: POOL_TYPE = 5;
pub const PagedPoolCacheAlignedSession: POOL_TYPE = 37;
pub const PagedPoolSession: POOL_TYPE = 33;
pub const PasidConfigTypeDefaultPasidOnly: IOMMU_PASID_CONFIGURATION_TYPE = 0;
pub const PasidConfigTypeMax: IOMMU_PASID_CONFIGURATION_TYPE = 2;
pub const PasidConfigTypePasidTaggedDma: IOMMU_PASID_CONFIGURATION_TYPE = 1;
pub const PciAcsBitDisable: PCI_ACS_BIT = 2;
pub const PciAcsBitDontCare: PCI_ACS_BIT = 3;
pub const PciAcsBitEnable: PCI_ACS_BIT = 1;
pub const PciAcsReserved: PCI_ACS_BIT = 0;
pub const PcwCallbackAddCounter: PCW_CALLBACK_TYPE = 0;
pub const PcwCallbackCollectData: PCW_CALLBACK_TYPE = 3;
pub const PcwCallbackEnumerateInstances: PCW_CALLBACK_TYPE = 2;
pub const PcwCallbackRemoveCounter: PCW_CALLBACK_TYPE = 1;
pub const PcwRegistrationNone: PCW_REGISTRATION_FLAGS = 0;
pub const PcwRegistrationSiloNeutral: PCW_REGISTRATION_FLAGS = 1;
pub const PermissionFault: FAULT_INFORMATION_ARM64_TYPE = 4;
pub const PlatformLevelDeviceReset: DEVICE_RESET_TYPE = 1;
pub const PoEffectivePowerModeBalanced: PO_EFFECTIVE_POWER_MODE = 2;
pub const PoEffectivePowerModeBatterySaver: PO_EFFECTIVE_POWER_MODE = 0;
pub const PoEffectivePowerModeBetterBattery: PO_EFFECTIVE_POWER_MODE = 1;
pub const PoEffectivePowerModeEnergySaverHighSavings: PO_EFFECTIVE_POWER_MODE = 0;
pub const PoEffectivePowerModeEnergySaverStandard: PO_EFFECTIVE_POWER_MODE = 1;
pub const PoEffectivePowerModeGameMode: PO_EFFECTIVE_POWER_MODE = 5;
pub const PoEffectivePowerModeHighPerformance: PO_EFFECTIVE_POWER_MODE = 3;
pub const PoEffectivePowerModeMaxPerformance: PO_EFFECTIVE_POWER_MODE = 4;
pub const PoEffectivePowerModeMixedReality: PO_EFFECTIVE_POWER_MODE = 6;
pub const PoFxPerfStateTypeDiscrete: PO_FX_PERF_STATE_TYPE = 0;
pub const PoFxPerfStateTypeMaximum: PO_FX_PERF_STATE_TYPE = 2;
pub const PoFxPerfStateTypeRange: PO_FX_PERF_STATE_TYPE = 1;
pub const PoFxPerfStateUnitBandwidth: PO_FX_PERF_STATE_UNIT = 2;
pub const PoFxPerfStateUnitFrequency: PO_FX_PERF_STATE_UNIT = 1;
pub const PoFxPerfStateUnitMaximum: PO_FX_PERF_STATE_UNIT = 3;
pub const PoFxPerfStateUnitOther: PO_FX_PERF_STATE_UNIT = 0;
pub const PoThermalRequestActive: PO_THERMAL_REQUEST_TYPE = 1;
pub const PoThermalRequestPassive: PO_THERMAL_REQUEST_TYPE = 0;
pub const PoolAllocation: KWAIT_REASON = 3;
pub const PoolCreateExtendedParameterInvalidType: POOL_CREATE_EXTENDED_PARAMETER_TYPE = 0;
pub const PoolCreateExtendedParameterName: POOL_CREATE_EXTENDED_PARAMETER_TYPE = 1;
pub const PoolExtendedParameterInvalidType: POOL_EXTENDED_PARAMETER_TYPE = 0;
pub const PoolExtendedParameterMax: POOL_EXTENDED_PARAMETER_TYPE = 5;
pub const PoolExtendedParameterNumaNode: POOL_EXTENDED_PARAMETER_TYPE = 3;
pub const PoolExtendedParameterPriority: POOL_EXTENDED_PARAMETER_TYPE = 1;
pub const PoolExtendedParameterPrivatePool: POOL_EXTENDED_PARAMETER_TYPE = 4;
pub const PoolExtendedParameterSecurePool: POOL_EXTENDED_PARAMETER_TYPE = 2;
pub const PowerRelations: DEVICE_RELATION_TYPE = 2;
pub const ProcessorInternal: INTERFACE_TYPE = 12;
pub const Profile2Issue: KPROFILE_SOURCE = 15;
pub const Profile3Issue: KPROFILE_SOURCE = 16;
pub const Profile4Issue: KPROFILE_SOURCE = 17;
pub const ProfileAlignmentFixup: KPROFILE_SOURCE = 1;
pub const ProfileBranchInstructions: KPROFILE_SOURCE = 6;
pub const ProfileBranchMispredictions: KPROFILE_SOURCE = 11;
pub const ProfileCacheMisses: KPROFILE_SOURCE = 10;
pub const ProfileDcacheAccesses: KPROFILE_SOURCE = 21;
pub const ProfileDcacheMisses: KPROFILE_SOURCE = 8;
pub const ProfileFpInstructions: KPROFILE_SOURCE = 13;
pub const ProfileIcacheIssues: KPROFILE_SOURCE = 20;
pub const ProfileIcacheMisses: KPROFILE_SOURCE = 9;
pub const ProfileIntegerInstructions: KPROFILE_SOURCE = 14;
pub const ProfileLoadInstructions: KPROFILE_SOURCE = 4;
pub const ProfileLoadLinkedIssues: KPROFILE_SOURCE = 23;
pub const ProfileMaximum: KPROFILE_SOURCE = 24;
pub const ProfileMemoryBarrierCycles: KPROFILE_SOURCE = 22;
pub const ProfilePipelineDry: KPROFILE_SOURCE = 3;
pub const ProfilePipelineFrozen: KPROFILE_SOURCE = 5;
pub const ProfileSpecialInstructions: KPROFILE_SOURCE = 18;
pub const ProfileStoreInstructions: KPROFILE_SOURCE = 12;
pub const ProfileTime: KPROFILE_SOURCE = 0;
pub const ProfileTotalCycles: KPROFILE_SOURCE = 19;
pub const ProfileTotalIssues: KPROFILE_SOURCE = 2;
pub const ProfileTotalNonissues: KPROFILE_SOURCE = 7;
#[cfg(feature = "bcrypt")]
pub type QUERY_SUPPORTED_DOE_PROTOCOLS = Option<unsafe extern "system" fn(interfacecontext: *const core::ffi::c_void, dataobjectvendorid: u16, arraycount: u32, dataobjecttypes: *mut u8, dataobjectcount: *mut u32) -> super::bcrypt::NTSTATUS>;
pub const QuerySecurityDescriptor: SECURITY_OPERATION_CODE = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct REENUMERATE_SELF_INTERFACE_STANDARD {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
    pub SurpriseRemoveAndReenumerateSelf: PREENUMERATE_SELF,
}
impl Default for REENUMERATE_SELF_INTERFACE_STANDARD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const REFTAG_AFDCONN: u32 = 1130653249;
pub const REFTAG_AFDENDPOINT: u32 = 1164207681;
pub const REFTAG_AFDPOLL: u32 = 1348757057;
pub const REFTAG_ALEIO: u32 = 1231383617;
pub const REFTAG_ALEPROCTBL: u32 = 1348824129;
pub const REFTAG_ALESIDTOKEN: u32 = 1399155777;
pub const REFTAG_CFSFILTER: u32 = 1181967939;
pub const REFTAG_HTTP: u32 = 1886680136;
pub const REFTAG_MAILSLOT: u32 = 1933996877;
pub const REFTAG_NFSVOLUME: u32 = 1450403406;
pub const REFTAG_PGMFILE: u32 = 1416456016;
pub const REFTAG_PSLOOKUP: u32 = 1431073616;
pub const REFTAG_PSNOTIFICATION: u32 = 1867412304;
pub const REFTAG_PSWAKE: u32 = 1800893264;
pub const REFTAG_RAWENDPOINT: u32 = 1165451602;
pub const REFTAG_SUBJECTCONTEXT: u32 = 1968399699;
pub const REFTAG_TCPENDPOINT: u32 = 1164993364;
pub const REFTAG_TCPLISTENER: u32 = 1282433876;
pub const REFTAG_TCPTCB: u32 = 1416651604;
pub const REFTAG_UDPENDPOINT: u32 = 1164993621;
pub const REFTAG_VIDEO_PORT: u32 = 1348757846;
pub const REFTAG_VIDEO_PORT_I386: u32 = 1768188246;
pub const REFTAG_WS2IFSL: u32 = 1764914007;
pub const REFTAG_WSKNAMERES: u32 = 1315664727;
pub const REFTAG_WSKPROV: u32 = 1349219159;
pub const REFTAG_WSKTDI: u32 = 1416328023;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REG_CALLBACK_CONTEXT_CLEANUP_INFORMATION {
    pub Object: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for REG_CALLBACK_CONTEXT_CLEANUP_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "minwindef", feature = "ntsecapi", feature = "winnt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REG_CREATE_KEY_INFORMATION {
    pub CompleteName: super::ntsecapi::PUNICODE_STRING,
    pub RootObject: *mut core::ffi::c_void,
    pub ObjectType: *mut core::ffi::c_void,
    pub CreateOptions: u32,
    pub Class: super::ntsecapi::PUNICODE_STRING,
    pub SecurityDescriptor: *mut core::ffi::c_void,
    pub SecurityQualityOfService: *mut core::ffi::c_void,
    pub DesiredAccess: super::winnt::ACCESS_MASK,
    pub GrantedAccess: super::winnt::ACCESS_MASK,
    pub Disposition: super::minwindef::PULONG,
    pub ResultObject: *mut *mut core::ffi::c_void,
    pub CallContext: *mut core::ffi::c_void,
    pub RootObjectContext: *mut core::ffi::c_void,
    pub Transaction: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
#[cfg(all(feature = "lsalookup", feature = "minwindef", feature = "ntsecapi", feature = "winnt"))]
impl Default for REG_CREATE_KEY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "minwindef", feature = "ntsecapi", feature = "winnt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REG_CREATE_KEY_INFORMATION_V1 {
    pub CompleteName: super::ntsecapi::PUNICODE_STRING,
    pub RootObject: *mut core::ffi::c_void,
    pub ObjectType: *mut core::ffi::c_void,
    pub Options: u32,
    pub Class: super::ntsecapi::PUNICODE_STRING,
    pub SecurityDescriptor: *mut core::ffi::c_void,
    pub SecurityQualityOfService: *mut core::ffi::c_void,
    pub DesiredAccess: super::winnt::ACCESS_MASK,
    pub GrantedAccess: super::winnt::ACCESS_MASK,
    pub Disposition: super::minwindef::PULONG,
    pub ResultObject: *mut *mut core::ffi::c_void,
    pub CallContext: *mut core::ffi::c_void,
    pub RootObjectContext: *mut core::ffi::c_void,
    pub Transaction: *mut core::ffi::c_void,
    pub Version: usize,
    pub RemainingName: super::ntsecapi::PUNICODE_STRING,
    pub Wow64Flags: u32,
    pub Attributes: u32,
    pub CheckAccessMode: KPROCESSOR_MODE,
}
#[cfg(all(feature = "lsalookup", feature = "minwindef", feature = "ntsecapi", feature = "winnt"))]
impl Default for REG_CREATE_KEY_INFORMATION_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REG_DELETE_KEY_INFORMATION {
    pub Object: *mut core::ffi::c_void,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for REG_DELETE_KEY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REG_DELETE_VALUE_KEY_INFORMATION {
    pub Object: *mut core::ffi::c_void,
    pub ValueName: super::ntsecapi::PUNICODE_STRING,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
impl Default for REG_DELETE_VALUE_KEY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REG_ENUMERATE_KEY_INFORMATION {
    pub Object: *mut core::ffi::c_void,
    pub Index: u32,
    pub KeyInformationClass: KEY_INFORMATION_CLASS,
    pub KeyInformation: *mut core::ffi::c_void,
    pub Length: u32,
    pub ResultLength: super::minwindef::PULONG,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
#[cfg(feature = "minwindef")]
impl Default for REG_ENUMERATE_KEY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REG_ENUMERATE_VALUE_KEY_INFORMATION {
    pub Object: *mut core::ffi::c_void,
    pub Index: u32,
    pub KeyValueInformationClass: KEY_VALUE_INFORMATION_CLASS,
    pub KeyValueInformation: *mut core::ffi::c_void,
    pub Length: u32,
    pub ResultLength: super::minwindef::PULONG,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
#[cfg(feature = "minwindef")]
impl Default for REG_ENUMERATE_VALUE_KEY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type REG_FLUSH_KEY_INFORMATION = REG_DELETE_KEY_INFORMATION;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REG_KEY_HANDLE_CLOSE_INFORMATION {
    pub Object: *mut core::ffi::c_void,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for REG_KEY_HANDLE_CLOSE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REG_LOAD_KEY_INFORMATION {
    pub Object: *mut core::ffi::c_void,
    pub KeyName: super::ntsecapi::PUNICODE_STRING,
    pub SourceFile: super::ntsecapi::PUNICODE_STRING,
    pub Flags: u32,
    pub TrustClassObject: *mut core::ffi::c_void,
    pub UserEvent: *mut core::ffi::c_void,
    pub DesiredAccess: super::winnt::ACCESS_MASK,
    pub RootHandle: super::winnt::PHANDLE,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
impl Default for REG_LOAD_KEY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REG_LOAD_KEY_INFORMATION_V2 {
    pub Object: *mut core::ffi::c_void,
    pub KeyName: super::ntsecapi::PUNICODE_STRING,
    pub SourceFile: super::ntsecapi::PUNICODE_STRING,
    pub Flags: u32,
    pub TrustClassObject: *mut core::ffi::c_void,
    pub UserEvent: *mut core::ffi::c_void,
    pub DesiredAccess: super::winnt::ACCESS_MASK,
    pub RootHandle: super::winnt::PHANDLE,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Version: usize,
    pub FileAccessToken: *mut core::ffi::c_void,
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
impl Default for REG_LOAD_KEY_INFORMATION_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type REG_NOTIFY_CLASS = i32;
#[cfg(all(feature = "lsalookup", feature = "minwindef", feature = "ntsecapi", feature = "winnt"))]
pub type REG_OPEN_KEY_INFORMATION = REG_CREATE_KEY_INFORMATION;
#[cfg(all(feature = "lsalookup", feature = "minwindef", feature = "ntsecapi", feature = "winnt"))]
pub type REG_OPEN_KEY_INFORMATION_V1 = REG_CREATE_KEY_INFORMATION_V1;
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REG_POST_CREATE_KEY_INFORMATION {
    pub CompleteName: super::ntsecapi::PUNICODE_STRING,
    pub Object: *mut core::ffi::c_void,
    pub Status: super::bcrypt::NTSTATUS,
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi"))]
impl Default for REG_POST_CREATE_KEY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi"))]
pub type REG_POST_OPEN_KEY_INFORMATION = REG_POST_CREATE_KEY_INFORMATION;
#[repr(C)]
#[cfg(feature = "bcrypt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REG_POST_OPERATION_INFORMATION {
    pub Object: *mut core::ffi::c_void,
    pub Status: super::bcrypt::NTSTATUS,
    pub PreInformation: *mut core::ffi::c_void,
    pub ReturnStatus: super::bcrypt::NTSTATUS,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
#[cfg(feature = "bcrypt")]
impl Default for REG_POST_OPERATION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct REG_PRE_CREATE_KEY_INFORMATION {
    pub CompleteName: super::ntsecapi::PUNICODE_STRING,
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
pub type REG_PRE_OPEN_KEY_INFORMATION = REG_PRE_CREATE_KEY_INFORMATION;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REG_QUERY_KEY_INFORMATION {
    pub Object: *mut core::ffi::c_void,
    pub KeyInformationClass: KEY_INFORMATION_CLASS,
    pub KeyInformation: *mut core::ffi::c_void,
    pub Length: u32,
    pub ResultLength: super::minwindef::PULONG,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
#[cfg(feature = "minwindef")]
impl Default for REG_QUERY_KEY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "minwindef", feature = "ntsecapi"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REG_QUERY_KEY_NAME {
    pub Object: *mut core::ffi::c_void,
    pub ObjectNameInfo: POBJECT_NAME_INFORMATION,
    pub Length: u32,
    pub ReturnLength: super::minwindef::PULONG,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
#[cfg(all(feature = "lsalookup", feature = "minwindef", feature = "ntsecapi"))]
impl Default for REG_QUERY_KEY_NAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REG_QUERY_KEY_SECURITY_INFORMATION {
    pub Object: *mut core::ffi::c_void,
    pub SecurityInformation: super::winnt::PSECURITY_INFORMATION,
    pub SecurityDescriptor: super::winnt::PSECURITY_DESCRIPTOR,
    pub Length: super::minwindef::PULONG,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for REG_QUERY_KEY_SECURITY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "minwindef", feature = "ntsecapi", feature = "winternl"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REG_QUERY_MULTIPLE_VALUE_KEY_INFORMATION {
    pub Object: *mut core::ffi::c_void,
    pub ValueEntries: super::winternl::PKEY_VALUE_ENTRY,
    pub EntryCount: u32,
    pub ValueBuffer: *mut core::ffi::c_void,
    pub BufferLength: super::minwindef::PULONG,
    pub RequiredBufferLength: super::minwindef::PULONG,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
#[cfg(all(feature = "lsalookup", feature = "minwindef", feature = "ntsecapi", feature = "winternl"))]
impl Default for REG_QUERY_MULTIPLE_VALUE_KEY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "minwindef", feature = "ntsecapi"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REG_QUERY_VALUE_KEY_INFORMATION {
    pub Object: *mut core::ffi::c_void,
    pub ValueName: super::ntsecapi::PUNICODE_STRING,
    pub KeyValueInformationClass: KEY_VALUE_INFORMATION_CLASS,
    pub KeyValueInformation: *mut core::ffi::c_void,
    pub Length: u32,
    pub ResultLength: super::minwindef::PULONG,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
#[cfg(all(feature = "lsalookup", feature = "minwindef", feature = "ntsecapi"))]
impl Default for REG_QUERY_VALUE_KEY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REG_RENAME_KEY_INFORMATION {
    pub Object: *mut core::ffi::c_void,
    pub NewName: super::ntsecapi::PUNICODE_STRING,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
impl Default for REG_RENAME_KEY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REG_REPLACE_KEY_INFORMATION {
    pub Object: *mut core::ffi::c_void,
    pub OldFileName: super::ntsecapi::PUNICODE_STRING,
    pub NewFileName: super::ntsecapi::PUNICODE_STRING,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
impl Default for REG_REPLACE_KEY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REG_RESTORE_KEY_INFORMATION {
    pub Object: *mut core::ffi::c_void,
    pub FileHandle: super::winnt::HANDLE,
    pub Flags: u32,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
#[cfg(feature = "winnt")]
impl Default for REG_RESTORE_KEY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REG_SAVE_KEY_INFORMATION {
    pub Object: *mut core::ffi::c_void,
    pub FileHandle: super::winnt::HANDLE,
    pub Format: u32,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
#[cfg(feature = "winnt")]
impl Default for REG_SAVE_KEY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REG_SAVE_MERGED_KEY_INFORMATION {
    pub Object: *mut core::ffi::c_void,
    pub FileHandle: super::winnt::HANDLE,
    pub HighKeyObject: *mut core::ffi::c_void,
    pub LowKeyObject: *mut core::ffi::c_void,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
#[cfg(feature = "winnt")]
impl Default for REG_SAVE_MERGED_KEY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winternl")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REG_SET_INFORMATION_KEY_INFORMATION {
    pub Object: *mut core::ffi::c_void,
    pub KeySetInformationClass: super::winternl::KEY_SET_INFORMATION_CLASS,
    pub KeySetInformation: *mut core::ffi::c_void,
    pub KeySetInformationLength: u32,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
#[cfg(feature = "winternl")]
impl Default for REG_SET_INFORMATION_KEY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REG_SET_KEY_SECURITY_INFORMATION {
    pub Object: *mut core::ffi::c_void,
    pub SecurityInformation: super::winnt::PSECURITY_INFORMATION,
    pub SecurityDescriptor: super::winnt::PSECURITY_DESCRIPTOR,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
#[cfg(feature = "winnt")]
impl Default for REG_SET_KEY_SECURITY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REG_SET_VALUE_KEY_INFORMATION {
    pub Object: *mut core::ffi::c_void,
    pub ValueName: super::ntsecapi::PUNICODE_STRING,
    pub TitleIndex: u32,
    pub Type: u32,
    pub Data: *mut core::ffi::c_void,
    pub DataSize: u32,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
impl Default for REG_SET_VALUE_KEY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct REG_UNLOAD_KEY_INFORMATION {
    pub Object: *mut core::ffi::c_void,
    pub UserEvent: *mut core::ffi::c_void,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
impl Default for REG_UNLOAD_KEY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
pub type REQUEST_POWER_COMPLETE = Option<unsafe extern "system" fn(deviceobject: *const DEVICE_OBJECT, minorfunction: u8, powerstate: POWER_STATE, context: *const core::ffi::c_void, iostatus: *const super::winternl::IO_STATUS_BLOCK)>;
pub type RESET_DOE_INSTANCES = Option<unsafe extern "system" fn(interfacecontext: *const core::ffi::c_void)>;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RESOURCE_HASH_ENTRY {
    pub ListEntry: super::winnt::LIST_ENTRY,
    pub Address: *mut core::ffi::c_void,
    pub ContentionCount: u32,
    pub Number: u32,
}
#[cfg(feature = "winnt")]
impl Default for RESOURCE_HASH_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RESOURCE_HASH_TABLE_SIZE: u32 = 64;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RESOURCE_PERFORMANCE_DATA {
    pub ActiveResourceCount: u32,
    pub TotalResourceCount: u32,
    pub ExclusiveAcquire: u32,
    pub SharedFirstLevel: u32,
    pub SharedSecondLevel: u32,
    pub StarveFirstLevel: u32,
    pub StarveSecondLevel: u32,
    pub WaitForExclusive: u32,
    pub OwnerTableExpands: u32,
    pub MaximumTableExpand: u32,
    pub HashTable: [super::winnt::LIST_ENTRY; 64],
}
#[cfg(feature = "winnt")]
impl Default for RESOURCE_PERFORMANCE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ROOT_CMD_ENABLE_CORRECTABLE_ERROR_REPORTING: u32 = 1;
pub const ROOT_CMD_ENABLE_FATAL_ERROR_REPORTING: u32 = 4;
pub const ROOT_CMD_ENABLE_NONFATAL_ERROR_REPORTING: u32 = 2;
pub const ROOT_CMD_ERROR_REPORTING_ENABLE_MASK: u32 = 7;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RTL_BITMAP {
    pub SizeOfBitMap: u32,
    pub Buffer: super::minwindef::PULONG,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RTL_BITMAP_RUN {
    pub StartingIndex: u32,
    pub NumberOfBits: u32,
}
pub const RTL_GUID_STRING_SIZE: u32 = 38;
pub const RTL_QUERY_REGISTRY_DELETE: u32 = 64;
pub const RTL_QUERY_REGISTRY_DIRECT: u32 = 32;
pub const RTL_QUERY_REGISTRY_NOEXPAND: u32 = 16;
pub const RTL_QUERY_REGISTRY_NOSTRING: u32 = 128;
pub const RTL_QUERY_REGISTRY_NOVALUE: u32 = 8;
pub const RTL_QUERY_REGISTRY_REQUIRED: u32 = 4;
#[cfg(feature = "bcrypt")]
pub type RTL_QUERY_REGISTRY_ROUTINE = Option<unsafe extern "system" fn(valuename: windows_core::PCWSTR, valuetype: u32, valuedata: *const core::ffi::c_void, valuelength: u32, context: *const core::ffi::c_void, entrycontext: *const core::ffi::c_void) -> super::bcrypt::NTSTATUS>;
pub const RTL_QUERY_REGISTRY_SUBKEY: u32 = 1;
#[repr(C)]
#[cfg(feature = "bcrypt")]
#[derive(Clone, Copy, Debug)]
pub struct RTL_QUERY_REGISTRY_TABLE {
    pub QueryRoutine: PRTL_QUERY_REGISTRY_ROUTINE,
    pub Flags: u32,
    pub Name: windows_core::PWSTR,
    pub EntryContext: *mut core::ffi::c_void,
    pub DefaultType: u32,
    pub DefaultData: *mut core::ffi::c_void,
    pub DefaultLength: u32,
}
#[cfg(feature = "bcrypt")]
impl Default for RTL_QUERY_REGISTRY_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RTL_QUERY_REGISTRY_TOPKEY: u32 = 2;
pub const RTL_QUERY_REGISTRY_TYPECHECK: u32 = 256;
pub const RTL_QUERY_REGISTRY_TYPECHECK_MASK: i32 = -16777216;
pub const RTL_QUERY_REGISTRY_TYPECHECK_SHIFT: u32 = 24;
pub const RTL_REGISTRY_ABSOLUTE: u32 = 0;
pub const RTL_REGISTRY_CONTROL: u32 = 2;
pub const RTL_REGISTRY_DEVICEMAP: u32 = 4;
pub const RTL_REGISTRY_HANDLE: u32 = 1073741824;
pub const RTL_REGISTRY_MAXIMUM: u32 = 6;
pub const RTL_REGISTRY_OPTIONAL: u32 = 2147483648;
pub const RTL_REGISTRY_SERVICES: u32 = 1;
pub const RTL_REGISTRY_USER: u32 = 5;
pub const RTL_REGISTRY_WINDOWS_NT: u32 = 3;
pub const RandomAccess: IO_ACCESS_MODE = 1;
pub const ReadAccess: IO_ACCESS_TYPE = 0;
pub const RealTimeWorkQueue: WORK_QUEUE_TYPE = 5;
pub const RebuildControl: NPEM_CONTROL_STANDARD_CONTROL_BIT = 5;
pub const RegNtCallbackObjectContextCleanup: REG_NOTIFY_CLASS = 40;
pub const RegNtDeleteKey: REG_NOTIFY_CLASS = 0;
pub const RegNtDeleteValueKey: REG_NOTIFY_CLASS = 2;
pub const RegNtEnumerateKey: REG_NOTIFY_CLASS = 5;
pub const RegNtEnumerateValueKey: REG_NOTIFY_CLASS = 6;
pub const RegNtKeyHandleClose: REG_NOTIFY_CLASS = 14;
pub const RegNtPostCreateKey: REG_NOTIFY_CLASS = 11;
pub const RegNtPostCreateKeyEx: REG_NOTIFY_CLASS = 27;
pub const RegNtPostDeleteKey: REG_NOTIFY_CLASS = 15;
pub const RegNtPostDeleteValueKey: REG_NOTIFY_CLASS = 17;
pub const RegNtPostEnumerateKey: REG_NOTIFY_CLASS = 20;
pub const RegNtPostEnumerateValueKey: REG_NOTIFY_CLASS = 21;
pub const RegNtPostFlushKey: REG_NOTIFY_CLASS = 31;
pub const RegNtPostKeyHandleClose: REG_NOTIFY_CLASS = 25;
pub const RegNtPostLoadKey: REG_NOTIFY_CLASS = 33;
pub const RegNtPostOpenKey: REG_NOTIFY_CLASS = 13;
pub const RegNtPostOpenKeyEx: REG_NOTIFY_CLASS = 29;
pub const RegNtPostQueryKey: REG_NOTIFY_CLASS = 22;
pub const RegNtPostQueryKeyName: REG_NOTIFY_CLASS = 48;
pub const RegNtPostQueryKeySecurity: REG_NOTIFY_CLASS = 37;
pub const RegNtPostQueryMultipleValueKey: REG_NOTIFY_CLASS = 24;
pub const RegNtPostQueryValueKey: REG_NOTIFY_CLASS = 23;
pub const RegNtPostRenameKey: REG_NOTIFY_CLASS = 19;
pub const RegNtPostReplaceKey: REG_NOTIFY_CLASS = 46;
pub const RegNtPostRestoreKey: REG_NOTIFY_CLASS = 42;
pub const RegNtPostSaveKey: REG_NOTIFY_CLASS = 44;
pub const RegNtPostSaveMergedKey: REG_NOTIFY_CLASS = 50;
pub const RegNtPostSetInformationKey: REG_NOTIFY_CLASS = 18;
pub const RegNtPostSetKeySecurity: REG_NOTIFY_CLASS = 39;
pub const RegNtPostSetValueKey: REG_NOTIFY_CLASS = 16;
pub const RegNtPostUnLoadKey: REG_NOTIFY_CLASS = 35;
pub const RegNtPreCreateKey: REG_NOTIFY_CLASS = 10;
pub const RegNtPreCreateKeyEx: REG_NOTIFY_CLASS = 26;
pub const RegNtPreDeleteKey: REG_NOTIFY_CLASS = 0;
pub const RegNtPreDeleteValueKey: REG_NOTIFY_CLASS = 2;
pub const RegNtPreEnumerateKey: REG_NOTIFY_CLASS = 5;
pub const RegNtPreEnumerateValueKey: REG_NOTIFY_CLASS = 6;
pub const RegNtPreFlushKey: REG_NOTIFY_CLASS = 30;
pub const RegNtPreKeyHandleClose: REG_NOTIFY_CLASS = 14;
pub const RegNtPreLoadKey: REG_NOTIFY_CLASS = 32;
pub const RegNtPreOpenKey: REG_NOTIFY_CLASS = 12;
pub const RegNtPreOpenKeyEx: REG_NOTIFY_CLASS = 28;
pub const RegNtPreQueryKey: REG_NOTIFY_CLASS = 7;
pub const RegNtPreQueryKeyName: REG_NOTIFY_CLASS = 47;
pub const RegNtPreQueryKeySecurity: REG_NOTIFY_CLASS = 36;
pub const RegNtPreQueryMultipleValueKey: REG_NOTIFY_CLASS = 9;
pub const RegNtPreQueryValueKey: REG_NOTIFY_CLASS = 8;
pub const RegNtPreRenameKey: REG_NOTIFY_CLASS = 4;
pub const RegNtPreReplaceKey: REG_NOTIFY_CLASS = 45;
pub const RegNtPreRestoreKey: REG_NOTIFY_CLASS = 41;
pub const RegNtPreSaveKey: REG_NOTIFY_CLASS = 43;
pub const RegNtPreSaveMergedKey: REG_NOTIFY_CLASS = 49;
pub const RegNtPreSetInformationKey: REG_NOTIFY_CLASS = 3;
pub const RegNtPreSetKeySecurity: REG_NOTIFY_CLASS = 38;
pub const RegNtPreSetValueKey: REG_NOTIFY_CLASS = 1;
pub const RegNtPreUnLoadKey: REG_NOTIFY_CLASS = 34;
pub const RegNtQueryKey: REG_NOTIFY_CLASS = 7;
pub const RegNtQueryMultipleValueKey: REG_NOTIFY_CLASS = 9;
pub const RegNtQueryValueKey: REG_NOTIFY_CLASS = 8;
pub const RegNtRenameKey: REG_NOTIFY_CLASS = 4;
pub const RegNtSetInformationKey: REG_NOTIFY_CLASS = 3;
pub const RegNtSetValueKey: REG_NOTIFY_CLASS = 1;
pub const RemovalPolicyExpectNoRemoval: DEVICE_REMOVAL_POLICY = 1;
pub const RemovalPolicyExpectOrderlyRemoval: DEVICE_REMOVAL_POLICY = 2;
pub const RemovalPolicyExpectSurpriseRemoval: DEVICE_REMOVAL_POLICY = 3;
pub const RemovalRelations: DEVICE_RELATION_TYPE = 3;
pub const ResourceNeverExclusive: u32 = 16;
pub const ResourceOwnedExclusive: u32 = 128;
pub const ResourceReleaseByOtherThread: u32 = 32;
#[repr(C)]
#[cfg(feature = "usb")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SCATTER_GATHER_ELEMENT {
    pub Address: super::usb::PHYSICAL_ADDRESS,
    pub Length: u32,
    pub Reserved: usize,
}
#[repr(C)]
#[cfg(feature = "usb")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCATTER_GATHER_LIST {
    pub NumberOfElements: u32,
    pub Reserved: usize,
    pub Elements: [SCATTER_GATHER_ELEMENT; 0],
}
#[cfg(feature = "usb")]
impl Default for SCATTER_GATHER_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct SDEV_IDENTIFIER_INTERFACE {
    pub InterfaceHeader: INTERFACE,
    pub GetIdentifier: PGET_SDEV_IDENTIFIER,
}
pub const SDEV_IDENTIFIER_INTERFACE_VERSION: u32 = 1;
pub type SECTION_INHERIT = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SECTION_OBJECT_POINTERS {
    pub DataSectionObject: *mut core::ffi::c_void,
    pub SharedCacheMap: *mut core::ffi::c_void,
    pub ImageSectionObject: *mut core::ffi::c_void,
}
impl Default for SECTION_OBJECT_POINTERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct SECURE_DRIVER_INTERFACE {
    pub InterfaceHeader: INTERFACE,
    pub ProcessReference: PSECURE_DRIVER_PROCESS_REFERENCE,
    pub ProcessDereference: PSECURE_DRIVER_PROCESS_DEREFERENCE,
    pub Reserved: u32,
}
pub const SECURE_DRIVER_INTERFACE_VERSION: u32 = 1;
pub type SECURE_DRIVER_PROCESS_DEREFERENCE = Option<unsafe extern "system" fn(interfacecontext: *const core::ffi::c_void, process: *const _KPROCESS)>;
pub type SECURE_DRIVER_PROCESS_REFERENCE = Option<unsafe extern "system" fn(interfacecontext: *const core::ffi::c_void) -> PEPROCESS>;
pub const SECURE_POOL_FLAGS_FREEABLE: u32 = 1;
pub const SECURE_POOL_FLAGS_MODIFIABLE: u32 = 2;
pub const SECURE_POOL_FLAGS_NONE: u32 = 0;
pub const SECURE_SECTION_ALLOW_PARTIAL_MDL: u32 = 1;
pub type SECURITY_OPERATION_CODE = i32;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SECURITY_SUBJECT_CONTEXT {
    pub ClientToken: super::winnt::PACCESS_TOKEN,
    pub ImpersonationLevel: super::winnt::SECURITY_IMPERSONATION_LEVEL,
    pub PrimaryToken: super::winnt::PACCESS_TOKEN,
    pub ProcessAuditId: *mut core::ffi::c_void,
}
#[cfg(feature = "winnt")]
impl Default for SECURITY_SUBJECT_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SEGMENT_ALL_ACCESS: u32 = 983071;
pub const SEMAPHORE_INCREMENT: u32 = 1;
pub const SEMAPHORE_QUERY_STATE: u32 = 1;
#[cfg(feature = "bcrypt")]
pub type SEND_DOE_REQUEST = Option<unsafe extern "system" fn(interfacecontext: *const core::ffi::c_void, dataobjectvendorid: u16, dataobjecttype: u8, inputarraycount: u32, inputarray: *const u32, outputarraycount: u32, outputwritten: *mut u32, outputarray: *mut u32) -> super::bcrypt::NTSTATUS>;
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
pub type SEND_DOE_REQUEST_ASYNC = Option<unsafe extern "system" fn(interfacecontext: *const core::ffi::c_void, dataobjectvendorid: u16, dataobjecttype: u8, inputarraycount: u32, inputarray: *const u32, completionevent: *const KEVENT, requestid: *mut i64) -> super::bcrypt::NTSTATUS>;
pub type SET_D3COLD_SUPPORT = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, d3coldsupport: bool)>;
pub type SET_VIRTUAL_DEVICE_DATA = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, virtualfunction: u16, buffer: *const core::ffi::c_void, offset: u32, length: u32) -> u32>;
pub const SE_ASSIGNPRIMARYTOKEN_PRIVILEGE: u32 = 3;
pub const SE_AUDIT_PRIVILEGE: u32 = 21;
pub const SE_BACKUP_PRIVILEGE: u32 = 17;
pub const SE_CHANGE_NOTIFY_PRIVILEGE: u32 = 23;
pub const SE_CREATE_GLOBAL_PRIVILEGE: u32 = 30;
pub const SE_CREATE_PAGEFILE_PRIVILEGE: u32 = 15;
pub const SE_CREATE_PERMANENT_PRIVILEGE: u32 = 16;
pub const SE_CREATE_SYMBOLIC_LINK_PRIVILEGE: u32 = 35;
pub const SE_CREATE_TOKEN_PRIVILEGE: u32 = 2;
pub const SE_DEBUG_PRIVILEGE: u32 = 20;
pub const SE_DELEGATE_SESSION_USER_IMPERSONATE_PRIVILEGE: u32 = 36;
pub const SE_ENABLE_DELEGATION_PRIVILEGE: u32 = 27;
pub type SE_IMAGE_TYPE = i32;
#[cfg(all(feature = "lsalookup", feature = "ntddk", feature = "ntsecapi"))]
pub type SE_IMAGE_VERIFICATION_CALLBACK_FUNCTION = Option<unsafe extern "system" fn(callbackcontext: *const core::ffi::c_void, imagetype: SE_IMAGE_TYPE, imageinformation: *mut super::ntddk::BDCB_IMAGE_INFORMATION)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SE_IMAGE_VERIFICATION_CALLBACK_TOKEN(pub *mut core::ffi::c_void);
impl Default for SE_IMAGE_VERIFICATION_CALLBACK_TOKEN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SE_IMAGE_VERIFICATION_CALLBACK_TYPE = i32;
pub const SE_IMPERSONATE_PRIVILEGE: u32 = 29;
pub const SE_INCREASE_QUOTA_PRIVILEGE: u32 = 5;
pub const SE_INC_BASE_PRIORITY_PRIVILEGE: u32 = 14;
pub const SE_INC_WORKING_SET_PRIVILEGE: u32 = 33;
pub const SE_LOAD_DRIVER_PRIVILEGE: u32 = 10;
pub const SE_LOCK_MEMORY_PRIVILEGE: u32 = 4;
pub const SE_MACHINE_ACCOUNT_PRIVILEGE: u32 = 6;
pub const SE_MANAGE_VOLUME_PRIVILEGE: u32 = 28;
pub const SE_MAX_WELL_KNOWN_PRIVILEGE: u32 = 36;
pub const SE_MIN_WELL_KNOWN_PRIVILEGE: u32 = 2;
pub const SE_PROF_SINGLE_PROCESS_PRIVILEGE: u32 = 13;
pub const SE_RELABEL_PRIVILEGE: u32 = 32;
pub const SE_REMOTE_SHUTDOWN_PRIVILEGE: u32 = 24;
pub const SE_RESTORE_PRIVILEGE: u32 = 18;
pub const SE_SECURITY_PRIVILEGE: u32 = 8;
pub const SE_SHUTDOWN_PRIVILEGE: u32 = 19;
pub const SE_SYNC_AGENT_PRIVILEGE: u32 = 26;
pub const SE_SYSTEMTIME_PRIVILEGE: u32 = 12;
pub const SE_SYSTEM_ENVIRONMENT_PRIVILEGE: u32 = 22;
pub const SE_SYSTEM_PROFILE_PRIVILEGE: u32 = 11;
pub const SE_TAKE_OWNERSHIP_PRIVILEGE: u32 = 9;
pub const SE_TCB_PRIVILEGE: u32 = 7;
pub const SE_TIME_ZONE_PRIVILEGE: u32 = 34;
pub const SE_TRUSTED_CREDMAN_ACCESS_PRIVILEGE: u32 = 31;
pub const SE_UNDOCK_PRIVILEGE: u32 = 25;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SHARE_ACCESS {
    pub OpenCount: u32,
    pub Readers: u32,
    pub Writers: u32,
    pub Deleters: u32,
    pub SharedRead: u32,
    pub SharedWrite: u32,
    pub SharedDelete: u32,
}
pub const SHORT_LEAST_SIGNIFICANT_BIT: u32 = 0;
pub const SHORT_MASK: u32 = 1;
pub const SHORT_MOST_SIGNIFICANT_BIT: u32 = 1;
pub const SINGLE_GROUP_LEGACY_API: u32 = 1;
pub const SL_ALLOW_RAW_MOUNT: u32 = 1;
pub const SL_BYPASS_ACCESS_CHECK: u32 = 1;
pub const SL_BYPASS_IO: u32 = 64;
pub const SL_CASE_SENSITIVE: u32 = 128;
pub const SL_ERROR_RETURNED: u32 = 2;
pub const SL_EXCLUSIVE_LOCK: u32 = 2;
pub const SL_FAIL_IMMEDIATELY: u32 = 1;
pub const SL_FORCE_ACCESS_CHECK: u32 = 1;
pub const SL_FORCE_ASYNCHRONOUS: u32 = 1;
pub const SL_FORCE_DIRECT_WRITE: u32 = 16;
pub const SL_FT_SEQUENTIAL_WRITE: u32 = 8;
pub const SL_IGNORE_READONLY_ATTRIBUTE: u32 = 64;
pub const SL_INDEX_SPECIFIED: u32 = 4;
pub const SL_INFO_FORCE_ACCESS_CHECK: u32 = 1;
pub const SL_INFO_IGNORE_READONLY_ATTRIBUTE: u32 = 64;
pub const SL_INVOKE_ON_CANCEL: u32 = 32;
pub const SL_INVOKE_ON_ERROR: u32 = 128;
pub const SL_INVOKE_ON_SUCCESS: u32 = 64;
pub const SL_KEY_SPECIFIED: u32 = 1;
pub const SL_NO_CURSOR_UPDATE: u32 = 16;
pub const SL_OPEN_PAGING_FILE: u32 = 2;
pub const SL_OPEN_TARGET_DIRECTORY: u32 = 4;
pub const SL_OVERRIDE_VERIFY_VOLUME: u32 = 2;
pub const SL_PENDING_RETURNED: u32 = 1;
pub const SL_PERSISTENT_MEMORY_FIXED_MAPPING: u32 = 32;
pub const SL_QUERY_DIRECTORY_MASK: u32 = 27;
pub const SL_READ_ACCESS_GRANTED: u32 = 1;
pub const SL_REALTIME_STREAM: u32 = 32;
pub const SL_RESTART_SCAN: u32 = 1;
pub const SL_RETURN_ON_DISK_ENTRIES_ONLY: u32 = 8;
pub const SL_RETURN_SINGLE_ENTRY: u32 = 2;
pub const SL_STOP_ON_SYMLINK: u32 = 8;
pub const SL_VERIFY_COMPLETION: u32 = 4;
pub const SL_WATCH_TREE: u32 = 1;
pub const SL_WRITE_ACCESS_GRANTED: u32 = 4;
pub const SL_WRITE_THROUGH: u32 = 4;
#[cfg(target_arch = "x86")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SPFN_NUMBER(pub i32);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SPFN_NUMBER(pub i64);
pub const STATUS_CONTINUE_COMPLETION: u32 = 0;
pub const SYMBOLIC_LINK_ALL_ACCESS: u32 = 983041;
pub const SYMBOLIC_LINK_ALL_ACCESS_EX: u32 = 1048575;
pub const SYMBOLIC_LINK_QUERY: u32 = 1;
pub const SYMBOLIC_LINK_SET: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SYSTEM_POWER_STATE_CONTEXT {
    pub Anonymous: SYSTEM_POWER_STATE_CONTEXT_0,
}
impl Default for SYSTEM_POWER_STATE_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SYSTEM_POWER_STATE_CONTEXT_0 {
    pub Anonymous: SYSTEM_POWER_STATE_CONTEXT_0_0,
    pub ContextAsUlong: u32,
}
impl Default for SYSTEM_POWER_STATE_CONTEXT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct SYSTEM_POWER_STATE_CONTEXT_0_0 {
    pub _bitfield: u32,
}
pub const SeImageTypeAll: SE_IMAGE_TYPE = 4;
pub const SeImageTypeDriver: SE_IMAGE_TYPE = 1;
pub const SeImageTypeDynamicCodeFile: SE_IMAGE_TYPE = 3;
pub const SeImageTypeElamDriver: SE_IMAGE_TYPE = 0;
pub const SeImageTypeMax: SE_IMAGE_TYPE = 5;
pub const SeImageTypePlatformSecureFile: SE_IMAGE_TYPE = 2;
pub const SeImageVerificationCallbackBlock: SE_IMAGE_VERIFICATION_CALLBACK_TYPE = 1;
pub const SeImageVerificationCallbackInformational: SE_IMAGE_VERIFICATION_CALLBACK_TYPE = 0;
pub const SequentialAccess: IO_ACCESS_MODE = 0;
pub const SetSecurityDescriptor: SECURITY_OPERATION_CODE = 0;
pub const SingleBusRelations: DEVICE_RELATION_TYPE = 5;
pub const StandardDesign: ALTERNATIVE_ARCHITECTURE_TYPE = 0;
pub const StopCompletion: IO_COMPLETION_ROUTINE_RESULT = -1073741802;
pub const SuperCriticalWorkQueue: WORK_QUEUE_TYPE = 6;
pub const Suspended: KWAIT_REASON = 5;
pub const SystemMemoryPartitionDedicatedMemoryInformation: PARTITION_INFORMATION_CLASS = 9;
pub const SystemMemoryPartitionInformation: PARTITION_INFORMATION_CLASS = 0;
pub const SystemMemoryPartitionOpenDedicatedMemory: PARTITION_INFORMATION_CLASS = 10;
pub const SystemPowerState: POWER_STATE_TYPE = 0;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TARGET_DEVICE_CUSTOM_NOTIFICATION {
    pub Version: u16,
    pub Size: u16,
    pub Event: windows_core::GUID,
    pub FileObject: PFILE_OBJECT,
    pub NameBufferOffset: i32,
    pub CustomDataBuffer: [u8; 1],
}
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for TARGET_DEVICE_CUSTOM_NOTIFICATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TARGET_DEVICE_REMOVAL_NOTIFICATION {
    pub Version: u16,
    pub Size: u16,
    pub Event: windows_core::GUID,
    pub FileObject: PFILE_OBJECT,
}
pub const THREAD_ALERT: u32 = 4;
pub const THREAD_WAIT_OBJECTS: u32 = 3;
pub const TIMER_EXPIRED_INDEX_BITS: u32 = 6;
pub const TIMER_PROCESSOR_INDEX_BITS: u32 = 5;
pub const TIMER_TOLERABLE_DELAY_BITS: u32 = 6;
#[repr(C)]
#[cfg(feature = "ntdef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TIME_FIELDS {
    pub Year: super::ntdef::CSHORT,
    pub Month: super::ntdef::CSHORT,
    pub Day: super::ntdef::CSHORT,
    pub Hour: super::ntdef::CSHORT,
    pub Minute: super::ntdef::CSHORT,
    pub Second: super::ntdef::CSHORT,
    pub Milliseconds: super::ntdef::CSHORT,
    pub Weekday: super::ntdef::CSHORT,
}
pub type TRACE_INFORMATION_CLASS = i32;
#[cfg(feature = "usb")]
pub type TRANSLATE_BUS_ADDRESS = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, busaddress: super::usb::PHYSICAL_ADDRESS, length: u32, addressspace: *mut u32, translatedaddress: *mut i64) -> bool>;
pub const TREE_CONNECT_NO_CLIENT_BUFFERING: u32 = 8;
pub const TREE_CONNECT_WRITE_THROUGH: u32 = 2;
pub const TargetDeviceRelation: DEVICE_RELATION_TYPE = 4;
pub const TlbMatchConflict: FAULT_INFORMATION_ARM64_TYPE = 2;
pub const TraceEnableFlagsClass: TRACE_INFORMATION_CLASS = 2;
pub const TraceEnableLevelClass: TRACE_INFORMATION_CLASS = 3;
pub const TraceHandleByNameClass: TRACE_INFORMATION_CLASS = 7;
pub const TraceHandleClass: TRACE_INFORMATION_CLASS = 1;
pub const TraceIdClass: TRACE_INFORMATION_CLASS = 0;
pub const TraceInformationClassReserved1: TRACE_INFORMATION_CLASS = 12;
pub const TraceInformationClassReserved2: TRACE_INFORMATION_CLASS = 14;
pub const TraceSessionSettingsClass: TRACE_INFORMATION_CLASS = 9;
pub const TranslationFault: FAULT_INFORMATION_ARM64_TYPE = 6;
pub const TransportRelations: DEVICE_RELATION_TYPE = 6;
pub const TurboChannel: INTERFACE_TYPE = 4;
pub const TypeA: DMA_SPEED = 1;
pub const TypeB: DMA_SPEED = 2;
pub const TypeC: DMA_SPEED = 3;
pub const TypeF: DMA_SPEED = 4;
#[cfg(target_arch = "x86")]
pub const UADDRESS_BASE: u32 = 0;
pub const UnsupportedUpstreamTransaction: FAULT_INFORMATION_ARM64_TYPE = 0;
pub const UserRequest: KWAIT_REASON = 6;
#[repr(C)]
#[derive(Clone, Copy)]
pub union VIRTUAL_CHANNEL_CAPABILITIES1 {
    pub Anonymous: VIRTUAL_CHANNEL_CAPABILITIES1_0,
    pub AsULONG: u32,
}
impl Default for VIRTUAL_CHANNEL_CAPABILITIES1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VIRTUAL_CHANNEL_CAPABILITIES1_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union VIRTUAL_CHANNEL_CAPABILITIES2 {
    pub Anonymous: VIRTUAL_CHANNEL_CAPABILITIES2_0,
    pub AsULONG: u32,
}
impl Default for VIRTUAL_CHANNEL_CAPABILITIES2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VIRTUAL_CHANNEL_CAPABILITIES2_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union VIRTUAL_CHANNEL_CONTROL {
    pub Anonymous: VIRTUAL_CHANNEL_CONTROL_0,
    pub AsUSHORT: u16,
}
impl Default for VIRTUAL_CHANNEL_CONTROL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VIRTUAL_CHANNEL_CONTROL_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union VIRTUAL_CHANNEL_STATUS {
    pub Anonymous: VIRTUAL_CHANNEL_STATUS_0,
    pub AsUSHORT: u16,
}
impl Default for VIRTUAL_CHANNEL_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VIRTUAL_CHANNEL_STATUS_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VIRTUAL_RESOURCE {
    pub Capability: VIRTUAL_RESOURCE_CAPABILITY,
    pub Control: VIRTUAL_RESOURCE_CONTROL,
    pub RsvdP: u16,
    pub Status: VIRTUAL_RESOURCE_STATUS,
}
impl Default for VIRTUAL_RESOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union VIRTUAL_RESOURCE_CAPABILITY {
    pub Anonymous: VIRTUAL_RESOURCE_CAPABILITY_0,
    pub AsULONG: u32,
}
impl Default for VIRTUAL_RESOURCE_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VIRTUAL_RESOURCE_CAPABILITY_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union VIRTUAL_RESOURCE_CONTROL {
    pub Anonymous: VIRTUAL_RESOURCE_CONTROL_0,
    pub AsULONG: u32,
}
impl Default for VIRTUAL_RESOURCE_CONTROL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VIRTUAL_RESOURCE_CONTROL_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union VIRTUAL_RESOURCE_STATUS {
    pub Anonymous: VIRTUAL_RESOURCE_STATUS_0,
    pub AsUSHORT: u16,
}
impl Default for VIRTUAL_RESOURCE_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct VIRTUAL_RESOURCE_STATUS_0 {
    pub _bitfield: u16,
}
pub const VMEBus: INTERFACE_TYPE = 6;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VPB {
    pub Type: super::ntdef::CSHORT,
    pub Size: super::ntdef::CSHORT,
    pub Flags: u16,
    pub VolumeLabelLength: u16,
    pub DeviceObject: *mut DEVICE_OBJECT,
    pub RealDevice: *mut DEVICE_OBJECT,
    pub SerialNumber: u32,
    pub ReferenceCount: u32,
    pub VolumeLabel: [u16; 32],
}
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for VPB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const VPB_DIRECT_WRITES_ALLOWED: u32 = 32;
pub const VPB_DISMOUNTING: u32 = 128;
pub const VPB_FLAGS_BYPASSIO_BLOCKED: u32 = 64;
pub const VPB_LOCKED: u32 = 2;
pub const VPB_MOUNTED: u32 = 1;
pub const VPB_PERSISTENT: u32 = 4;
pub const VPB_RAW_MOUNT: u32 = 16;
pub const VPB_REMOVE_PENDING: u32 = 8;
pub const ViewShare: SECTION_INHERIT = 1;
pub const ViewUnmap: SECTION_INHERIT = 2;
pub const Vmcs: INTERFACE_TYPE = 16;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub struct WAIT_CONTEXT_BLOCK {
    pub Anonymous: WAIT_CONTEXT_BLOCK_0,
    pub DeviceRoutine: PDRIVER_CONTROL,
    pub DeviceContext: *mut core::ffi::c_void,
    pub NumberOfMapRegisters: u32,
    pub DeviceObject: *mut core::ffi::c_void,
    pub CurrentIrp: *mut core::ffi::c_void,
    pub BufferChainingDpc: PKDPC,
}
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for WAIT_CONTEXT_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub union WAIT_CONTEXT_BLOCK_0 {
    pub WaitQueueEntry: KDEVICE_QUEUE_ENTRY,
    pub Anonymous: WAIT_CONTEXT_BLOCK_0_0,
}
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
impl Default for WAIT_CONTEXT_BLOCK_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "bcrypt", feature = "lsalookup", feature = "ntdef", feature = "ntifs", feature = "ntsecapi", feature = "usb", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WAIT_CONTEXT_BLOCK_0_0 {
    pub DmaWaitEntry: super::winnt::LIST_ENTRY,
    pub NumberOfChannels: u32,
    pub _bitfield: u32,
}
pub const WDM_MAJORVERSION: u32 = 6;
pub const WDM_MINORVERSION: u32 = 0;
pub const WMIREGISTER: u32 = 0;
pub const WMIREG_ACTION_BLOCK_IRPS: u32 = 5;
pub const WMIREG_ACTION_DEREGISTER: u32 = 2;
pub const WMIREG_ACTION_REGISTER: u32 = 1;
pub const WMIREG_ACTION_REREGISTER: u32 = 3;
pub const WMIREG_ACTION_UPDATE_GUIDS: u32 = 4;
pub const WMIUPDATE: u32 = 1;
pub type WMI_NOTIFICATION_CALLBACK = *mut FWMI_NOTIFICATION_CALLBACK;
pub type WORKER_THREAD_ROUTINE = Option<unsafe extern "system" fn(parameter: *const core::ffi::c_void)>;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WORK_QUEUE_ITEM {
    pub List: super::winnt::LIST_ENTRY,
    pub WorkerRoutine: PWORKER_THREAD_ROUTINE,
    pub Parameter: *mut core::ffi::c_void,
}
#[cfg(feature = "winnt")]
impl Default for WORK_QUEUE_ITEM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WORK_QUEUE_TYPE = i32;
pub const WdfNotifyRoutinesClass: TRACE_INFORMATION_CLASS = 15;
pub const Width16Bits: DMA_WIDTH = 1;
pub const Width32Bits: DMA_WIDTH = 2;
pub const Width64Bits: DMA_WIDTH = 3;
pub const Width8Bits: DMA_WIDTH = 0;
pub const WidthNoWrap: DMA_WIDTH = 4;
pub const WrAlertByThreadId: KWAIT_REASON = 37;
pub const WrCalloutStack: KWAIT_REASON = 25;
pub const WrCpuRateControl: KWAIT_REASON = 24;
pub const WrDeferredPreempt: KWAIT_REASON = 38;
pub const WrDelayExecution: KWAIT_REASON = 11;
pub const WrDispatchInt: KWAIT_REASON = 31;
pub const WrExecutive: KWAIT_REASON = 7;
pub const WrFastMutex: KWAIT_REASON = 34;
pub const WrFreePage: KWAIT_REASON = 8;
pub const WrGuardedMutex: KWAIT_REASON = 35;
pub const WrIoRing: KWAIT_REASON = 40;
pub const WrKernel: KWAIT_REASON = 26;
pub const WrKeyedEvent: KWAIT_REASON = 21;
pub const WrLpcReceive: KWAIT_REASON = 16;
pub const WrLpcReply: KWAIT_REASON = 17;
pub const WrMdlCache: KWAIT_REASON = 41;
pub const WrMutex: KWAIT_REASON = 29;
pub const WrPageIn: KWAIT_REASON = 9;
pub const WrPageOut: KWAIT_REASON = 19;
pub const WrPhysicalFault: KWAIT_REASON = 39;
pub const WrPoolAllocation: KWAIT_REASON = 10;
pub const WrPreempted: KWAIT_REASON = 32;
pub const WrProcessInSwap: KWAIT_REASON = 23;
pub const WrPushLock: KWAIT_REASON = 28;
pub const WrQuantumEnd: KWAIT_REASON = 30;
pub const WrQueue: KWAIT_REASON = 15;
pub const WrRcu: KWAIT_REASON = 42;
pub const WrRendezvous: KWAIT_REASON = 20;
pub const WrResource: KWAIT_REASON = 27;
pub const WrRundown: KWAIT_REASON = 36;
pub const WrSpare0: KWAIT_REASON = 14;
pub const WrSuspended: KWAIT_REASON = 12;
pub const WrTerminated: KWAIT_REASON = 22;
pub const WrUserRequest: KWAIT_REASON = 13;
pub const WrVirtualMemory: KWAIT_REASON = 18;
pub const WrYieldExecution: KWAIT_REASON = 33;
pub const WriteAccess: IO_ACCESS_TYPE = 1;
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct XSTATE_SAVE {
    pub Anonymous: XSTATE_SAVE_0,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
impl Default for XSTATE_SAVE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union XSTATE_SAVE_0 {
    pub Anonymous: XSTATE_SAVE_0_0,
    pub XStateContext: super::winnt::XSTATE_CONTEXT,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
impl Default for XSTATE_SAVE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XSTATE_SAVE_0_0 {
    pub Reserved1: i64,
    pub Reserved2: u32,
    pub Prev: *mut XSTATE_SAVE,
    pub Reserved3: super::winnt::PXSAVE_AREA,
    pub Thread: *mut _KTHREAD,
    pub Reserved4: *mut core::ffi::c_void,
    pub Level: u8,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
impl Default for XSTATE_SAVE_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XSTATE_SAVE {
    pub Prev: *mut Self,
    pub Thread: *mut _KTHREAD,
    pub Level: u8,
    pub XStateContext: super::winnt::XSTATE_CONTEXT,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
impl Default for XSTATE_SAVE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XSTATE_SAVE {
    pub Prev: *mut Self,
    pub Thread: *mut _KTHREAD,
    pub Level: u8,
    pub XStateContext: super::winnt::XSTATE_CONTEXT,
    pub Reserved: usize,
}
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "winnt")]
impl Default for XSTATE_SAVE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _AFFINITY_TOKEN(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _CALLBACK_OBJECT(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _DEVICE_OBJECT_POWER_EXTENSION(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _DISK_PARTITION(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _DRIVER_PROXY_EXTENSION(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _DRIVE_LAYOUT_INFORMATION(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _DRIVE_LAYOUT_INFORMATION_EX(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _EPROCESS(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _EX_RUNDOWN_REF_CACHE_AWARE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _EX_TIMER(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _IOP_FILE_OBJECT_EXTENSION(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _IORING_OBJECT(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _IO_REMOVE_LOCK_TRACKING_BLOCK(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _IO_TIMER(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _IO_WORKITEM(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _KAFFINITY_EX(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _KE_SRCU(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _KINTERRUPT(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _KPROCESS(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _KTHREAD(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _OBJECT_TYPE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _PCW_BUFFER(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _PCW_INSTANCE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _PCW_REGISTRATION(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _SCSI_REQUEST_BLOCK(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _SET_PARTITION_INFORMATION_EX(pub u8);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct __prefast_analysis_mode_flag0(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct __prefast_analysis_mode_flag1(pub i32);
