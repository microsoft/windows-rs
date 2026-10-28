#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CommitComplete(enlistmenthandle: super::HANDLE, tmvirtualclock: super::PLARGE_INTEGER) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn CommitComplete(enlistmenthandle : super::HANDLE, tmvirtualclock : super::PLARGE_INTEGER) -> windows_core::BOOL);
    unsafe { CommitComplete(enlistmenthandle as _, tmvirtualclock as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CommitEnlistment(enlistmenthandle: super::HANDLE, tmvirtualclock: super::PLARGE_INTEGER) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn CommitEnlistment(enlistmenthandle : super::HANDLE, tmvirtualclock : super::PLARGE_INTEGER) -> windows_core::BOOL);
    unsafe { CommitEnlistment(enlistmenthandle as _, tmvirtualclock as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CommitTransaction(transactionhandle: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn CommitTransaction(transactionhandle : super::HANDLE) -> windows_core::BOOL);
    unsafe { CommitTransaction(transactionhandle as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CommitTransactionAsync(transactionhandle: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn CommitTransactionAsync(transactionhandle : super::HANDLE) -> windows_core::BOOL);
    unsafe { CommitTransactionAsync(transactionhandle as _) }
}
#[cfg(all(feature = "ktmtypes", feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateEnlistment(lpenlistmentattributes: super::LPSECURITY_ATTRIBUTES, resourcemanagerhandle: super::HANDLE, transactionhandle: super::HANDLE, notificationmask: super::NOTIFICATION_MASK, createoptions: super::DWORD, enlistmentkey: super::PVOID) -> super::HANDLE {
    windows_core::link!("ktmw32.dll" "system" fn CreateEnlistment(lpenlistmentattributes : super::LPSECURITY_ATTRIBUTES, resourcemanagerhandle : super::HANDLE, transactionhandle : super::HANDLE, notificationmask : super::NOTIFICATION_MASK, createoptions : super::DWORD, enlistmentkey : super::PVOID) -> super::HANDLE);
    unsafe { CreateEnlistment(lpenlistmentattributes as _, resourcemanagerhandle as _, transactionhandle as _, notificationmask, createoptions, enlistmentkey as _) }
}
#[cfg(all(feature = "guiddef", feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateResourceManager<P4>(lpresourcemanagerattributes: super::LPSECURITY_ATTRIBUTES, resourcemanagerid: super::LPGUID, createoptions: super::DWORD, tmhandle: super::HANDLE, description: P4) -> super::HANDLE
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ktmw32.dll" "system" fn CreateResourceManager(lpresourcemanagerattributes : super::LPSECURITY_ATTRIBUTES, resourcemanagerid : super::LPGUID, createoptions : super::DWORD, tmhandle : super::HANDLE, description : windows_core::PCWSTR) -> super::HANDLE);
    unsafe { CreateResourceManager(lpresourcemanagerattributes as _, resourcemanagerid as _, createoptions, tmhandle as _, description.param().abi()) }
}
#[cfg(all(feature = "guiddef", feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateTransaction<P6>(lptransactionattributes: super::LPSECURITY_ATTRIBUTES, uow: super::LPGUID, createoptions: super::DWORD, isolationlevel: super::DWORD, isolationflags: super::DWORD, timeout: super::DWORD, description: P6) -> super::HANDLE
where
    P6: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ktmw32.dll" "system" fn CreateTransaction(lptransactionattributes : super::LPSECURITY_ATTRIBUTES, uow : super::LPGUID, createoptions : super::DWORD, isolationlevel : super::DWORD, isolationflags : super::DWORD, timeout : super::DWORD, description : windows_core::PCWSTR) -> super::HANDLE);
    unsafe { CreateTransaction(lptransactionattributes as _, uow as _, createoptions, isolationlevel, isolationflags, timeout, description.param().abi()) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateTransactionManager<P1>(lptransactionattributes: super::LPSECURITY_ATTRIBUTES, logfilename: P1, createoptions: super::ULONG, commitstrength: super::ULONG) -> super::HANDLE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ktmw32.dll" "system" fn CreateTransactionManager(lptransactionattributes : super::LPSECURITY_ATTRIBUTES, logfilename : windows_core::PCWSTR, createoptions : super::ULONG, commitstrength : super::ULONG) -> super::HANDLE);
    unsafe { CreateTransactionManager(lptransactionattributes as _, logfilename.param().abi(), createoptions, commitstrength) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetCurrentClockTransactionManager(transactionmanagerhandle: super::HANDLE, tmvirtualclock: super::PLARGE_INTEGER) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "C" fn GetCurrentClockTransactionManager(transactionmanagerhandle : super::HANDLE, tmvirtualclock : super::PLARGE_INTEGER) -> windows_core::BOOL);
    unsafe { GetCurrentClockTransactionManager(transactionmanagerhandle as _, tmvirtualclock as _) }
}
#[cfg(all(feature = "guiddef", feature = "winnt"))]
#[inline]
pub unsafe fn GetEnlistmentId(enlistmenthandle: super::HANDLE, enlistmentid: super::LPGUID) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn GetEnlistmentId(enlistmenthandle : super::HANDLE, enlistmentid : super::LPGUID) -> windows_core::BOOL);
    unsafe { GetEnlistmentId(enlistmenthandle as _, enlistmentid as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetEnlistmentRecoveryInformation(enlistmenthandle: super::HANDLE, buffersize: super::ULONG, buffer: super::PVOID, bufferused: super::PULONG) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn GetEnlistmentRecoveryInformation(enlistmenthandle : super::HANDLE, buffersize : super::ULONG, buffer : super::PVOID, bufferused : super::PULONG) -> windows_core::BOOL);
    unsafe { GetEnlistmentRecoveryInformation(enlistmenthandle as _, buffersize, buffer as _, bufferused as _) }
}
#[cfg(all(feature = "ktmtypes", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetNotificationResourceManager(resourcemanagerhandle: super::HANDLE, transactionnotification: super::PTRANSACTION_NOTIFICATION, notificationlength: super::ULONG, dwmilliseconds: super::DWORD, returnlength: super::PULONG) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn GetNotificationResourceManager(resourcemanagerhandle : super::HANDLE, transactionnotification : super::PTRANSACTION_NOTIFICATION, notificationlength : super::ULONG, dwmilliseconds : super::DWORD, returnlength : super::PULONG) -> windows_core::BOOL);
    unsafe { GetNotificationResourceManager(resourcemanagerhandle as _, transactionnotification as _, notificationlength, dwmilliseconds, returnlength as _) }
}
#[cfg(all(feature = "ktmtypes", feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetNotificationResourceManagerAsync(resourcemanagerhandle: super::HANDLE, transactionnotification: super::PTRANSACTION_NOTIFICATION, transactionnotificationlength: super::ULONG, returnlength: super::PULONG, lpoverlapped: super::LPOVERLAPPED) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn GetNotificationResourceManagerAsync(resourcemanagerhandle : super::HANDLE, transactionnotification : super::PTRANSACTION_NOTIFICATION, transactionnotificationlength : super::ULONG, returnlength : super::PULONG, lpoverlapped : super::LPOVERLAPPED) -> windows_core::BOOL);
    unsafe { GetNotificationResourceManagerAsync(resourcemanagerhandle as _, transactionnotification as _, transactionnotificationlength, returnlength as _, lpoverlapped as _) }
}
#[cfg(all(feature = "guiddef", feature = "winnt"))]
#[inline]
pub unsafe fn GetTransactionId(transactionhandle: super::HANDLE, transactionid: super::LPGUID) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn GetTransactionId(transactionhandle : super::HANDLE, transactionid : super::LPGUID) -> windows_core::BOOL);
    unsafe { GetTransactionId(transactionhandle as _, transactionid as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetTransactionInformation(transactionhandle: super::HANDLE, outcome: super::PDWORD, isolationlevel: super::PDWORD, isolationflags: super::PDWORD, timeout: super::PDWORD, bufferlength: super::DWORD, description: Option<windows_core::PWSTR>) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn GetTransactionInformation(transactionhandle : super::HANDLE, outcome : super::PDWORD, isolationlevel : super::PDWORD, isolationflags : super::PDWORD, timeout : super::PDWORD, bufferlength : super::DWORD, description : windows_core::PWSTR) -> windows_core::BOOL);
    unsafe { GetTransactionInformation(transactionhandle as _, outcome as _, isolationlevel as _, isolationflags as _, timeout as _, bufferlength, description.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "guiddef", feature = "winnt"))]
#[inline]
pub unsafe fn GetTransactionManagerId(transactionmanagerhandle: super::HANDLE, transactionmanagerid: super::LPGUID) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "C" fn GetTransactionManagerId(transactionmanagerhandle : super::HANDLE, transactionmanagerid : super::LPGUID) -> windows_core::BOOL);
    unsafe { GetTransactionManagerId(transactionmanagerhandle as _, transactionmanagerid as _) }
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn OpenEnlistment(dwdesiredaccess: super::DWORD, resourcemanagerhandle: super::HANDLE, enlistmentid: super::LPGUID) -> super::HANDLE {
    windows_core::link!("ktmw32.dll" "system" fn OpenEnlistment(dwdesiredaccess : super::DWORD, resourcemanagerhandle : super::HANDLE, enlistmentid : super::LPGUID) -> super::HANDLE);
    unsafe { OpenEnlistment(dwdesiredaccess, resourcemanagerhandle as _, enlistmentid as _) }
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn OpenResourceManager(dwdesiredaccess: super::DWORD, tmhandle: super::HANDLE, resourcemanagerid: super::LPGUID) -> super::HANDLE {
    windows_core::link!("ktmw32.dll" "system" fn OpenResourceManager(dwdesiredaccess : super::DWORD, tmhandle : super::HANDLE, resourcemanagerid : super::LPGUID) -> super::HANDLE);
    unsafe { OpenResourceManager(dwdesiredaccess, tmhandle as _, resourcemanagerid as _) }
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn OpenTransaction(dwdesiredaccess: super::DWORD, transactionid: super::LPGUID) -> super::HANDLE {
    windows_core::link!("ktmw32.dll" "system" fn OpenTransaction(dwdesiredaccess : super::DWORD, transactionid : super::LPGUID) -> super::HANDLE);
    unsafe { OpenTransaction(dwdesiredaccess, transactionid as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn OpenTransactionManager<P0>(logfilename: P0, desiredaccess: super::ACCESS_MASK, openoptions: super::ULONG) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ktmw32.dll" "system" fn OpenTransactionManager(logfilename : windows_core::PCWSTR, desiredaccess : super::ACCESS_MASK, openoptions : super::ULONG) -> super::HANDLE);
    unsafe { OpenTransactionManager(logfilename.param().abi(), desiredaccess, openoptions) }
}
#[cfg(all(feature = "guiddef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn OpenTransactionManagerById(transactionmanagerid: super::LPGUID, desiredaccess: super::ACCESS_MASK, openoptions: super::ULONG) -> super::HANDLE {
    windows_core::link!("ktmw32.dll" "system" fn OpenTransactionManagerById(transactionmanagerid : super::LPGUID, desiredaccess : super::ACCESS_MASK, openoptions : super::ULONG) -> super::HANDLE);
    unsafe { OpenTransactionManagerById(transactionmanagerid, desiredaccess, openoptions) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn PrePrepareComplete(enlistmenthandle: super::HANDLE, tmvirtualclock: super::PLARGE_INTEGER) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn PrePrepareComplete(enlistmenthandle : super::HANDLE, tmvirtualclock : super::PLARGE_INTEGER) -> windows_core::BOOL);
    unsafe { PrePrepareComplete(enlistmenthandle as _, tmvirtualclock as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn PrePrepareEnlistment(enlistmenthandle: super::HANDLE, tmvirtualclock: super::PLARGE_INTEGER) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn PrePrepareEnlistment(enlistmenthandle : super::HANDLE, tmvirtualclock : super::PLARGE_INTEGER) -> windows_core::BOOL);
    unsafe { PrePrepareEnlistment(enlistmenthandle as _, tmvirtualclock as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn PrepareComplete(enlistmenthandle: super::HANDLE, tmvirtualclock: super::PLARGE_INTEGER) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn PrepareComplete(enlistmenthandle : super::HANDLE, tmvirtualclock : super::PLARGE_INTEGER) -> windows_core::BOOL);
    unsafe { PrepareComplete(enlistmenthandle as _, tmvirtualclock as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn PrepareEnlistment(enlistmenthandle: super::HANDLE, tmvirtualclock: super::PLARGE_INTEGER) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn PrepareEnlistment(enlistmenthandle : super::HANDLE, tmvirtualclock : super::PLARGE_INTEGER) -> windows_core::BOOL);
    unsafe { PrepareEnlistment(enlistmenthandle as _, tmvirtualclock as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ReadOnlyEnlistment(enlistmenthandle: super::HANDLE, tmvirtualclock: super::PLARGE_INTEGER) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn ReadOnlyEnlistment(enlistmenthandle : super::HANDLE, tmvirtualclock : super::PLARGE_INTEGER) -> windows_core::BOOL);
    unsafe { ReadOnlyEnlistment(enlistmenthandle as _, tmvirtualclock as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RecoverEnlistment(enlistmenthandle: super::HANDLE, enlistmentkey: super::PVOID) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn RecoverEnlistment(enlistmenthandle : super::HANDLE, enlistmentkey : super::PVOID) -> windows_core::BOOL);
    unsafe { RecoverEnlistment(enlistmenthandle as _, enlistmentkey as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RecoverResourceManager(resourcemanagerhandle: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn RecoverResourceManager(resourcemanagerhandle : super::HANDLE) -> windows_core::BOOL);
    unsafe { RecoverResourceManager(resourcemanagerhandle as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RecoverTransactionManager(transactionmanagerhandle: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn RecoverTransactionManager(transactionmanagerhandle : super::HANDLE) -> windows_core::BOOL);
    unsafe { RecoverTransactionManager(transactionmanagerhandle as _) }
}
#[cfg(feature = "guiddef")]
#[inline]
pub unsafe fn RenameTransactionManager<P0>(logfilename: P0, existingtransactionmanagerguid: super::LPGUID) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ktmw32.dll" "system" fn RenameTransactionManager(logfilename : windows_core::PCWSTR, existingtransactionmanagerguid : super::LPGUID) -> windows_core::BOOL);
    unsafe { RenameTransactionManager(logfilename.param().abi(), existingtransactionmanagerguid as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn RollbackComplete(enlistmenthandle: super::HANDLE, tmvirtualclock: super::PLARGE_INTEGER) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn RollbackComplete(enlistmenthandle : super::HANDLE, tmvirtualclock : super::PLARGE_INTEGER) -> windows_core::BOOL);
    unsafe { RollbackComplete(enlistmenthandle as _, tmvirtualclock as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn RollbackEnlistment(enlistmenthandle: super::HANDLE, tmvirtualclock: super::PLARGE_INTEGER) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn RollbackEnlistment(enlistmenthandle : super::HANDLE, tmvirtualclock : super::PLARGE_INTEGER) -> windows_core::BOOL);
    unsafe { RollbackEnlistment(enlistmenthandle as _, tmvirtualclock as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RollbackTransaction(transactionhandle: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn RollbackTransaction(transactionhandle : super::HANDLE) -> windows_core::BOOL);
    unsafe { RollbackTransaction(transactionhandle as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RollbackTransactionAsync(transactionhandle: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn RollbackTransactionAsync(transactionhandle : super::HANDLE) -> windows_core::BOOL);
    unsafe { RollbackTransactionAsync(transactionhandle as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn RollforwardTransactionManager(transactionmanagerhandle: super::HANDLE, tmvirtualclock: super::PLARGE_INTEGER) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn RollforwardTransactionManager(transactionmanagerhandle : super::HANDLE, tmvirtualclock : super::PLARGE_INTEGER) -> windows_core::BOOL);
    unsafe { RollforwardTransactionManager(transactionmanagerhandle as _, tmvirtualclock as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetEnlistmentRecoveryInformation(enlistmenthandle: super::HANDLE, buffersize: super::ULONG, buffer: super::PVOID) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn SetEnlistmentRecoveryInformation(enlistmenthandle : super::HANDLE, buffersize : super::ULONG, buffer : super::PVOID) -> windows_core::BOOL);
    unsafe { SetEnlistmentRecoveryInformation(enlistmenthandle as _, buffersize, buffer as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetResourceManagerCompletionPort(resourcemanagerhandle: super::HANDLE, iocompletionporthandle: super::HANDLE, completionkey: super::ULONG_PTR) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn SetResourceManagerCompletionPort(resourcemanagerhandle : super::HANDLE, iocompletionporthandle : super::HANDLE, completionkey : super::ULONG_PTR) -> windows_core::BOOL);
    unsafe { SetResourceManagerCompletionPort(resourcemanagerhandle as _, iocompletionporthandle as _, completionkey) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetTransactionInformation<P4>(transactionhandle: super::HANDLE, isolationlevel: super::DWORD, isolationflags: super::DWORD, timeout: super::DWORD, description: P4) -> windows_core::BOOL
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ktmw32.dll" "system" fn SetTransactionInformation(transactionhandle : super::HANDLE, isolationlevel : super::DWORD, isolationflags : super::DWORD, timeout : super::DWORD, description : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SetTransactionInformation(transactionhandle as _, isolationlevel, isolationflags, timeout, description.param().abi()) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SinglePhaseReject(enlistmenthandle: super::HANDLE, tmvirtualclock: super::PLARGE_INTEGER) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn SinglePhaseReject(enlistmenthandle : super::HANDLE, tmvirtualclock : super::PLARGE_INTEGER) -> windows_core::BOOL);
    unsafe { SinglePhaseReject(enlistmenthandle as _, tmvirtualclock as _) }
}
