#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CommitComplete(enlistmenthandle: super::HANDLE, tmvirtualclock: super::PLARGE_INTEGER) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn CommitComplete(enlistmenthandle : super::HANDLE, tmvirtualclock : super::PLARGE_INTEGER) -> windows_core::BOOL);
    unsafe { CommitComplete(enlistmenthandle, tmvirtualclock) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CommitEnlistment(enlistmenthandle: super::HANDLE, tmvirtualclock: super::PLARGE_INTEGER) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn CommitEnlistment(enlistmenthandle : super::HANDLE, tmvirtualclock : super::PLARGE_INTEGER) -> windows_core::BOOL);
    unsafe { CommitEnlistment(enlistmenthandle, tmvirtualclock) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CommitTransaction(transactionhandle: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn CommitTransaction(transactionhandle : super::HANDLE) -> windows_core::BOOL);
    unsafe { CommitTransaction(transactionhandle) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CommitTransactionAsync(transactionhandle: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn CommitTransactionAsync(transactionhandle : super::HANDLE) -> windows_core::BOOL);
    unsafe { CommitTransactionAsync(transactionhandle) }
}
#[cfg(all(feature = "ktmtypes", feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn CreateEnlistment(lpenlistmentattributes: Option<super::LPSECURITY_ATTRIBUTES>, resourcemanagerhandle: super::HANDLE, transactionhandle: super::HANDLE, notificationmask: super::NOTIFICATION_MASK, createoptions: Option<u32>, enlistmentkey: Option<*const core::ffi::c_void>) -> super::HANDLE {
    windows_core::link!("ktmw32.dll" "system" fn CreateEnlistment(lpenlistmentattributes : super::LPSECURITY_ATTRIBUTES, resourcemanagerhandle : super::HANDLE, transactionhandle : super::HANDLE, notificationmask : super::NOTIFICATION_MASK, createoptions : u32, enlistmentkey : *const core::ffi::c_void) -> super::HANDLE);
    unsafe { CreateEnlistment(lpenlistmentattributes.unwrap_or(core::mem::zeroed()) as _, resourcemanagerhandle, transactionhandle, notificationmask, createoptions.unwrap_or(core::mem::zeroed()) as _, enlistmentkey.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn CreateResourceManager<P4>(lpresourcemanagerattributes: Option<super::LPSECURITY_ATTRIBUTES>, resourcemanagerid: super::LPGUID, createoptions: Option<u32>, tmhandle: super::HANDLE, description: P4) -> super::HANDLE
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ktmw32.dll" "system" fn CreateResourceManager(lpresourcemanagerattributes : super::LPSECURITY_ATTRIBUTES, resourcemanagerid : super::LPGUID, createoptions : u32, tmhandle : super::HANDLE, description : windows_core::PCWSTR) -> super::HANDLE);
    unsafe { CreateResourceManager(lpresourcemanagerattributes.unwrap_or(core::mem::zeroed()) as _, resourcemanagerid, createoptions.unwrap_or(core::mem::zeroed()) as _, tmhandle, description.param().abi()) }
}
#[cfg(all(feature = "guiddef", feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn CreateTransaction<P6>(lptransactionattributes: Option<super::LPSECURITY_ATTRIBUTES>, uow: Option<super::LPGUID>, createoptions: Option<u32>, isolationlevel: Option<u32>, isolationflags: Option<u32>, timeout: Option<u32>, description: P6) -> super::HANDLE
where
    P6: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ktmw32.dll" "system" fn CreateTransaction(lptransactionattributes : super::LPSECURITY_ATTRIBUTES, uow : super::LPGUID, createoptions : u32, isolationlevel : u32, isolationflags : u32, timeout : u32, description : windows_core::PCWSTR) -> super::HANDLE);
    unsafe { CreateTransaction(lptransactionattributes.unwrap_or(core::mem::zeroed()) as _, uow.unwrap_or(core::mem::zeroed()) as _, createoptions.unwrap_or(core::mem::zeroed()) as _, isolationlevel.unwrap_or(core::mem::zeroed()) as _, isolationflags.unwrap_or(core::mem::zeroed()) as _, timeout.unwrap_or(core::mem::zeroed()) as _, description.param().abi()) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn CreateTransactionManager<P1>(lptransactionattributes: Option<super::LPSECURITY_ATTRIBUTES>, logfilename: P1, createoptions: Option<u32>, commitstrength: Option<u32>) -> super::HANDLE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ktmw32.dll" "system" fn CreateTransactionManager(lptransactionattributes : super::LPSECURITY_ATTRIBUTES, logfilename : windows_core::PCWSTR, createoptions : u32, commitstrength : u32) -> super::HANDLE);
    unsafe { CreateTransactionManager(lptransactionattributes.unwrap_or(core::mem::zeroed()) as _, logfilename.param().abi(), createoptions.unwrap_or(core::mem::zeroed()) as _, commitstrength.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetCurrentClockTransactionManager(transactionmanagerhandle: super::HANDLE, tmvirtualclock: super::PLARGE_INTEGER) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "C" fn GetCurrentClockTransactionManager(transactionmanagerhandle : super::HANDLE, tmvirtualclock : super::PLARGE_INTEGER) -> windows_core::BOOL);
    unsafe { GetCurrentClockTransactionManager(transactionmanagerhandle, tmvirtualclock as _) }
}
#[cfg(all(feature = "guiddef", feature = "winnt"))]
#[inline]
pub unsafe fn GetEnlistmentId(enlistmenthandle: super::HANDLE, enlistmentid: super::LPGUID) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn GetEnlistmentId(enlistmenthandle : super::HANDLE, enlistmentid : super::LPGUID) -> windows_core::BOOL);
    unsafe { GetEnlistmentId(enlistmenthandle, enlistmentid as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetEnlistmentRecoveryInformation(enlistmenthandle: super::HANDLE, buffersize: u32, buffer: *mut core::ffi::c_void, bufferused: Option<super::PULONG>) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn GetEnlistmentRecoveryInformation(enlistmenthandle : super::HANDLE, buffersize : u32, buffer : *mut core::ffi::c_void, bufferused : super::PULONG) -> windows_core::BOOL);
    unsafe { GetEnlistmentRecoveryInformation(enlistmenthandle, buffersize, buffer as _, bufferused.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "ktmtypes", feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetNotificationResourceManager(resourcemanagerhandle: super::HANDLE, transactionnotification: super::PTRANSACTION_NOTIFICATION, notificationlength: u32, dwmilliseconds: Option<u32>, returnlength: Option<super::PULONG>) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn GetNotificationResourceManager(resourcemanagerhandle : super::HANDLE, transactionnotification : super::PTRANSACTION_NOTIFICATION, notificationlength : u32, dwmilliseconds : u32, returnlength : super::PULONG) -> windows_core::BOOL);
    unsafe { GetNotificationResourceManager(resourcemanagerhandle, transactionnotification as _, notificationlength, dwmilliseconds.unwrap_or(core::mem::zeroed()) as _, returnlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "ktmtypes", feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetNotificationResourceManagerAsync(resourcemanagerhandle: super::HANDLE, transactionnotification: super::PTRANSACTION_NOTIFICATION, transactionnotificationlength: u32, returnlength: super::PULONG, lpoverlapped: super::LPOVERLAPPED) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn GetNotificationResourceManagerAsync(resourcemanagerhandle : super::HANDLE, transactionnotification : super::PTRANSACTION_NOTIFICATION, transactionnotificationlength : u32, returnlength : super::PULONG, lpoverlapped : super::LPOVERLAPPED) -> windows_core::BOOL);
    unsafe { GetNotificationResourceManagerAsync(resourcemanagerhandle, transactionnotification as _, transactionnotificationlength, returnlength as _, lpoverlapped) }
}
#[cfg(all(feature = "guiddef", feature = "winnt"))]
#[inline]
pub unsafe fn GetTransactionId(transactionhandle: super::HANDLE, transactionid: super::LPGUID) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn GetTransactionId(transactionhandle : super::HANDLE, transactionid : super::LPGUID) -> windows_core::BOOL);
    unsafe { GetTransactionId(transactionhandle, transactionid as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetTransactionInformation(transactionhandle: super::HANDLE, outcome: Option<super::PDWORD>, isolationlevel: Option<super::PDWORD>, isolationflags: Option<super::PDWORD>, timeout: Option<super::PDWORD>, bufferlength: u32, description: Option<windows_core::PWSTR>) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn GetTransactionInformation(transactionhandle : super::HANDLE, outcome : super::PDWORD, isolationlevel : super::PDWORD, isolationflags : super::PDWORD, timeout : super::PDWORD, bufferlength : u32, description : windows_core::PWSTR) -> windows_core::BOOL);
    unsafe { GetTransactionInformation(transactionhandle, outcome.unwrap_or(core::mem::zeroed()) as _, isolationlevel.unwrap_or(core::mem::zeroed()) as _, isolationflags.unwrap_or(core::mem::zeroed()) as _, timeout.unwrap_or(core::mem::zeroed()) as _, bufferlength, description.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "guiddef", feature = "winnt"))]
#[inline]
pub unsafe fn GetTransactionManagerId(transactionmanagerhandle: super::HANDLE, transactionmanagerid: super::LPGUID) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "C" fn GetTransactionManagerId(transactionmanagerhandle : super::HANDLE, transactionmanagerid : super::LPGUID) -> windows_core::BOOL);
    unsafe { GetTransactionManagerId(transactionmanagerhandle, transactionmanagerid as _) }
}
#[cfg(all(feature = "guiddef", feature = "winnt"))]
#[inline]
pub unsafe fn OpenEnlistment(dwdesiredaccess: u32, resourcemanagerhandle: super::HANDLE, enlistmentid: super::LPGUID) -> super::HANDLE {
    windows_core::link!("ktmw32.dll" "system" fn OpenEnlistment(dwdesiredaccess : u32, resourcemanagerhandle : super::HANDLE, enlistmentid : super::LPGUID) -> super::HANDLE);
    unsafe { OpenEnlistment(dwdesiredaccess, resourcemanagerhandle, enlistmentid) }
}
#[cfg(all(feature = "guiddef", feature = "winnt"))]
#[inline]
pub unsafe fn OpenResourceManager(dwdesiredaccess: u32, tmhandle: super::HANDLE, resourcemanagerid: super::LPGUID) -> super::HANDLE {
    windows_core::link!("ktmw32.dll" "system" fn OpenResourceManager(dwdesiredaccess : u32, tmhandle : super::HANDLE, resourcemanagerid : super::LPGUID) -> super::HANDLE);
    unsafe { OpenResourceManager(dwdesiredaccess, tmhandle, resourcemanagerid) }
}
#[cfg(all(feature = "guiddef", feature = "winnt"))]
#[inline]
pub unsafe fn OpenTransaction(dwdesiredaccess: u32, transactionid: super::LPGUID) -> super::HANDLE {
    windows_core::link!("ktmw32.dll" "system" fn OpenTransaction(dwdesiredaccess : u32, transactionid : super::LPGUID) -> super::HANDLE);
    unsafe { OpenTransaction(dwdesiredaccess, transactionid) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn OpenTransactionManager<P0>(logfilename: P0, desiredaccess: super::ACCESS_MASK, openoptions: Option<u32>) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ktmw32.dll" "system" fn OpenTransactionManager(logfilename : windows_core::PCWSTR, desiredaccess : super::ACCESS_MASK, openoptions : u32) -> super::HANDLE);
    unsafe { OpenTransactionManager(logfilename.param().abi(), desiredaccess, openoptions.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "guiddef", feature = "winnt"))]
#[inline]
pub unsafe fn OpenTransactionManagerById(transactionmanagerid: super::LPGUID, desiredaccess: super::ACCESS_MASK, openoptions: Option<u32>) -> super::HANDLE {
    windows_core::link!("ktmw32.dll" "system" fn OpenTransactionManagerById(transactionmanagerid : super::LPGUID, desiredaccess : super::ACCESS_MASK, openoptions : u32) -> super::HANDLE);
    unsafe { OpenTransactionManagerById(transactionmanagerid, desiredaccess, openoptions.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PrePrepareComplete(enlistmenthandle: super::HANDLE, tmvirtualclock: super::PLARGE_INTEGER) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn PrePrepareComplete(enlistmenthandle : super::HANDLE, tmvirtualclock : super::PLARGE_INTEGER) -> windows_core::BOOL);
    unsafe { PrePrepareComplete(enlistmenthandle, tmvirtualclock) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PrePrepareEnlistment(enlistmenthandle: super::HANDLE, tmvirtualclock: super::PLARGE_INTEGER) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn PrePrepareEnlistment(enlistmenthandle : super::HANDLE, tmvirtualclock : super::PLARGE_INTEGER) -> windows_core::BOOL);
    unsafe { PrePrepareEnlistment(enlistmenthandle, tmvirtualclock) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PrepareComplete(enlistmenthandle: super::HANDLE, tmvirtualclock: super::PLARGE_INTEGER) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn PrepareComplete(enlistmenthandle : super::HANDLE, tmvirtualclock : super::PLARGE_INTEGER) -> windows_core::BOOL);
    unsafe { PrepareComplete(enlistmenthandle, tmvirtualclock) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PrepareEnlistment(enlistmenthandle: super::HANDLE, tmvirtualclock: super::PLARGE_INTEGER) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn PrepareEnlistment(enlistmenthandle : super::HANDLE, tmvirtualclock : super::PLARGE_INTEGER) -> windows_core::BOOL);
    unsafe { PrepareEnlistment(enlistmenthandle, tmvirtualclock) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ReadOnlyEnlistment(enlistmenthandle: super::HANDLE, tmvirtualclock: super::PLARGE_INTEGER) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn ReadOnlyEnlistment(enlistmenthandle : super::HANDLE, tmvirtualclock : super::PLARGE_INTEGER) -> windows_core::BOOL);
    unsafe { ReadOnlyEnlistment(enlistmenthandle, tmvirtualclock) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RecoverEnlistment(enlistmenthandle: super::HANDLE, enlistmentkey: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn RecoverEnlistment(enlistmenthandle : super::HANDLE, enlistmentkey : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { RecoverEnlistment(enlistmenthandle, enlistmentkey.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RecoverResourceManager(resourcemanagerhandle: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn RecoverResourceManager(resourcemanagerhandle : super::HANDLE) -> windows_core::BOOL);
    unsafe { RecoverResourceManager(resourcemanagerhandle) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RecoverTransactionManager(transactionmanagerhandle: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn RecoverTransactionManager(transactionmanagerhandle : super::HANDLE) -> windows_core::BOOL);
    unsafe { RecoverTransactionManager(transactionmanagerhandle) }
}
#[cfg(feature = "guiddef")]
#[inline]
pub unsafe fn RenameTransactionManager<P0>(logfilename: P0, existingtransactionmanagerguid: super::LPGUID) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ktmw32.dll" "system" fn RenameTransactionManager(logfilename : windows_core::PCWSTR, existingtransactionmanagerguid : super::LPGUID) -> windows_core::BOOL);
    unsafe { RenameTransactionManager(logfilename.param().abi(), existingtransactionmanagerguid) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RollbackComplete(enlistmenthandle: super::HANDLE, tmvirtualclock: super::PLARGE_INTEGER) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn RollbackComplete(enlistmenthandle : super::HANDLE, tmvirtualclock : super::PLARGE_INTEGER) -> windows_core::BOOL);
    unsafe { RollbackComplete(enlistmenthandle, tmvirtualclock) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RollbackEnlistment(enlistmenthandle: super::HANDLE, tmvirtualclock: super::PLARGE_INTEGER) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn RollbackEnlistment(enlistmenthandle : super::HANDLE, tmvirtualclock : super::PLARGE_INTEGER) -> windows_core::BOOL);
    unsafe { RollbackEnlistment(enlistmenthandle, tmvirtualclock) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RollbackTransaction(transactionhandle: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn RollbackTransaction(transactionhandle : super::HANDLE) -> windows_core::BOOL);
    unsafe { RollbackTransaction(transactionhandle) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RollbackTransactionAsync(transactionhandle: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn RollbackTransactionAsync(transactionhandle : super::HANDLE) -> windows_core::BOOL);
    unsafe { RollbackTransactionAsync(transactionhandle) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RollforwardTransactionManager(transactionmanagerhandle: super::HANDLE, tmvirtualclock: super::PLARGE_INTEGER) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn RollforwardTransactionManager(transactionmanagerhandle : super::HANDLE, tmvirtualclock : super::PLARGE_INTEGER) -> windows_core::BOOL);
    unsafe { RollforwardTransactionManager(transactionmanagerhandle, tmvirtualclock) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetEnlistmentRecoveryInformation(enlistmenthandle: super::HANDLE, buffersize: u32, buffer: *const core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn SetEnlistmentRecoveryInformation(enlistmenthandle : super::HANDLE, buffersize : u32, buffer : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetEnlistmentRecoveryInformation(enlistmenthandle, buffersize, buffer) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetResourceManagerCompletionPort(resourcemanagerhandle: super::HANDLE, iocompletionporthandle: super::HANDLE, completionkey: usize) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn SetResourceManagerCompletionPort(resourcemanagerhandle : super::HANDLE, iocompletionporthandle : super::HANDLE, completionkey : usize) -> windows_core::BOOL);
    unsafe { SetResourceManagerCompletionPort(resourcemanagerhandle, iocompletionporthandle, completionkey) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetTransactionInformation<P4>(transactionhandle: super::HANDLE, isolationlevel: Option<u32>, isolationflags: Option<u32>, timeout: Option<u32>, description: P4) -> windows_core::BOOL
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ktmw32.dll" "system" fn SetTransactionInformation(transactionhandle : super::HANDLE, isolationlevel : u32, isolationflags : u32, timeout : u32, description : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SetTransactionInformation(transactionhandle, isolationlevel.unwrap_or(core::mem::zeroed()) as _, isolationflags.unwrap_or(core::mem::zeroed()) as _, timeout.unwrap_or(core::mem::zeroed()) as _, description.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SinglePhaseReject(enlistmenthandle: super::HANDLE, tmvirtualclock: super::PLARGE_INTEGER) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn SinglePhaseReject(enlistmenthandle : super::HANDLE, tmvirtualclock : super::PLARGE_INTEGER) -> windows_core::BOOL);
    unsafe { SinglePhaseReject(enlistmenthandle, tmvirtualclock) }
}
