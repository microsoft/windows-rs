#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn CommitComplete(enlistmenthandle: super::winnt::HANDLE, tmvirtualclock: *mut i64) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn CommitComplete(enlistmenthandle : super::winnt::HANDLE, tmvirtualclock : *mut i64) -> windows_core::BOOL);
    unsafe { CommitComplete(enlistmenthandle, tmvirtualclock as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn CommitEnlistment(enlistmenthandle: super::winnt::HANDLE, tmvirtualclock: *mut i64) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn CommitEnlistment(enlistmenthandle : super::winnt::HANDLE, tmvirtualclock : *mut i64) -> windows_core::BOOL);
    unsafe { CommitEnlistment(enlistmenthandle, tmvirtualclock as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn CommitTransaction(transactionhandle: super::winnt::HANDLE) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn CommitTransaction(transactionhandle : super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { CommitTransaction(transactionhandle) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn CommitTransactionAsync(transactionhandle: super::winnt::HANDLE) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn CommitTransactionAsync(transactionhandle : super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { CommitTransactionAsync(transactionhandle) }
}
#[cfg(all(feature = "Win32_ktmtypes", feature = "Win32_minwinbase", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn CreateEnlistment(lpenlistmentattributes: *mut super::minwinbase::SECURITY_ATTRIBUTES, resourcemanagerhandle: super::winnt::HANDLE, transactionhandle: super::winnt::HANDLE, notificationmask: super::ktmtypes::NOTIFICATION_MASK, createoptions: u32, enlistmentkey: *mut core::ffi::c_void) -> super::winnt::HANDLE {
    windows_core::link!("ktmw32.dll" "system" fn CreateEnlistment(lpenlistmentattributes : *mut super::minwinbase::SECURITY_ATTRIBUTES, resourcemanagerhandle : super::winnt::HANDLE, transactionhandle : super::winnt::HANDLE, notificationmask : super::ktmtypes::NOTIFICATION_MASK, createoptions : u32, enlistmentkey : *mut core::ffi::c_void) -> super::winnt::HANDLE);
    unsafe { CreateEnlistment(lpenlistmentattributes as _, resourcemanagerhandle, transactionhandle, notificationmask, createoptions, enlistmentkey as _) }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn CreateResourceManager<P4>(lpresourcemanagerattributes: *mut super::minwinbase::SECURITY_ATTRIBUTES, resourcemanagerid: *mut windows_core::GUID, createoptions: u32, tmhandle: super::winnt::HANDLE, description: P4) -> super::winnt::HANDLE
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ktmw32.dll" "system" fn CreateResourceManager(lpresourcemanagerattributes : *mut super::minwinbase::SECURITY_ATTRIBUTES, resourcemanagerid : *mut windows_core::GUID, createoptions : u32, tmhandle : super::winnt::HANDLE, description : windows_core::PCWSTR) -> super::winnt::HANDLE);
    unsafe { CreateResourceManager(lpresourcemanagerattributes as _, resourcemanagerid as _, createoptions, tmhandle, description.param().abi()) }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn CreateTransaction<P6>(lptransactionattributes: *mut super::minwinbase::SECURITY_ATTRIBUTES, uow: *mut windows_core::GUID, createoptions: u32, isolationlevel: u32, isolationflags: u32, timeout: u32, description: P6) -> super::winnt::HANDLE
where
    P6: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ktmw32.dll" "system" fn CreateTransaction(lptransactionattributes : *mut super::minwinbase::SECURITY_ATTRIBUTES, uow : *mut windows_core::GUID, createoptions : u32, isolationlevel : u32, isolationflags : u32, timeout : u32, description : windows_core::PCWSTR) -> super::winnt::HANDLE);
    unsafe { CreateTransaction(lptransactionattributes as _, uow as _, createoptions, isolationlevel, isolationflags, timeout, description.param().abi()) }
}
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn CreateTransactionManager<P1>(lptransactionattributes: *mut super::minwinbase::SECURITY_ATTRIBUTES, logfilename: P1, createoptions: u32, commitstrength: u32) -> super::winnt::HANDLE
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ktmw32.dll" "system" fn CreateTransactionManager(lptransactionattributes : *mut super::minwinbase::SECURITY_ATTRIBUTES, logfilename : windows_core::PCWSTR, createoptions : u32, commitstrength : u32) -> super::winnt::HANDLE);
    unsafe { CreateTransactionManager(lptransactionattributes as _, logfilename.param().abi(), createoptions, commitstrength) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn GetCurrentClockTransactionManager(transactionmanagerhandle: super::winnt::HANDLE, tmvirtualclock: *mut i64) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "C" fn GetCurrentClockTransactionManager(transactionmanagerhandle : super::winnt::HANDLE, tmvirtualclock : *mut i64) -> windows_core::BOOL);
    unsafe { GetCurrentClockTransactionManager(transactionmanagerhandle, tmvirtualclock as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn GetEnlistmentId(enlistmenthandle: super::winnt::HANDLE, enlistmentid: *mut windows_core::GUID) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn GetEnlistmentId(enlistmenthandle : super::winnt::HANDLE, enlistmentid : *mut windows_core::GUID) -> windows_core::BOOL);
    unsafe { GetEnlistmentId(enlistmenthandle, enlistmentid as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn GetEnlistmentRecoveryInformation(enlistmenthandle: super::winnt::HANDLE, buffersize: u32, buffer: *mut core::ffi::c_void, bufferused: *mut u32) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn GetEnlistmentRecoveryInformation(enlistmenthandle : super::winnt::HANDLE, buffersize : u32, buffer : *mut core::ffi::c_void, bufferused : *mut u32) -> windows_core::BOOL);
    unsafe { GetEnlistmentRecoveryInformation(enlistmenthandle, buffersize, buffer as _, bufferused as _) }
}
#[cfg(all(feature = "Win32_ktmtypes", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn GetNotificationResourceManager(resourcemanagerhandle: super::winnt::HANDLE, transactionnotification: *mut super::ktmtypes::TRANSACTION_NOTIFICATION, notificationlength: u32, dwmilliseconds: u32, returnlength: *mut u32) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn GetNotificationResourceManager(resourcemanagerhandle : super::winnt::HANDLE, transactionnotification : *mut super::ktmtypes::TRANSACTION_NOTIFICATION, notificationlength : u32, dwmilliseconds : u32, returnlength : *mut u32) -> windows_core::BOOL);
    unsafe { GetNotificationResourceManager(resourcemanagerhandle, transactionnotification as _, notificationlength, dwmilliseconds, returnlength as _) }
}
#[cfg(all(feature = "Win32_ktmtypes", feature = "Win32_minwinbase", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn GetNotificationResourceManagerAsync(resourcemanagerhandle: super::winnt::HANDLE, transactionnotification: *mut super::ktmtypes::TRANSACTION_NOTIFICATION, transactionnotificationlength: u32, returnlength: *mut u32, lpoverlapped: *mut super::minwinbase::OVERLAPPED) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn GetNotificationResourceManagerAsync(resourcemanagerhandle : super::winnt::HANDLE, transactionnotification : *mut super::ktmtypes::TRANSACTION_NOTIFICATION, transactionnotificationlength : u32, returnlength : *mut u32, lpoverlapped : *mut super::minwinbase::OVERLAPPED) -> windows_core::BOOL);
    unsafe { GetNotificationResourceManagerAsync(resourcemanagerhandle, transactionnotification as _, transactionnotificationlength, returnlength as _, lpoverlapped as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn GetTransactionId(transactionhandle: super::winnt::HANDLE, transactionid: *mut windows_core::GUID) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn GetTransactionId(transactionhandle : super::winnt::HANDLE, transactionid : *mut windows_core::GUID) -> windows_core::BOOL);
    unsafe { GetTransactionId(transactionhandle, transactionid as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn GetTransactionInformation(transactionhandle: super::winnt::HANDLE, outcome: *mut u32, isolationlevel: *mut u32, isolationflags: *mut u32, timeout: *mut u32, description: Option<&mut [u16]>) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn GetTransactionInformation(transactionhandle : super::winnt::HANDLE, outcome : *mut u32, isolationlevel : *mut u32, isolationflags : *mut u32, timeout : *mut u32, bufferlength : u32, description : windows_core::PWSTR) -> windows_core::BOOL);
    unsafe { GetTransactionInformation(transactionhandle, outcome as _, isolationlevel as _, isolationflags as _, timeout as _, description.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(description.as_deref().map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn GetTransactionManagerId(transactionmanagerhandle: super::winnt::HANDLE, transactionmanagerid: *mut windows_core::GUID) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "C" fn GetTransactionManagerId(transactionmanagerhandle : super::winnt::HANDLE, transactionmanagerid : *mut windows_core::GUID) -> windows_core::BOOL);
    unsafe { GetTransactionManagerId(transactionmanagerhandle, transactionmanagerid as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn OpenEnlistment(dwdesiredaccess: u32, resourcemanagerhandle: super::winnt::HANDLE, enlistmentid: *mut windows_core::GUID) -> super::winnt::HANDLE {
    windows_core::link!("ktmw32.dll" "system" fn OpenEnlistment(dwdesiredaccess : u32, resourcemanagerhandle : super::winnt::HANDLE, enlistmentid : *mut windows_core::GUID) -> super::winnt::HANDLE);
    unsafe { OpenEnlistment(dwdesiredaccess, resourcemanagerhandle, enlistmentid as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn OpenResourceManager(dwdesiredaccess: u32, tmhandle: super::winnt::HANDLE, resourcemanagerid: *mut windows_core::GUID) -> super::winnt::HANDLE {
    windows_core::link!("ktmw32.dll" "system" fn OpenResourceManager(dwdesiredaccess : u32, tmhandle : super::winnt::HANDLE, resourcemanagerid : *mut windows_core::GUID) -> super::winnt::HANDLE);
    unsafe { OpenResourceManager(dwdesiredaccess, tmhandle, resourcemanagerid as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn OpenTransaction(dwdesiredaccess: u32, transactionid: *mut windows_core::GUID) -> super::winnt::HANDLE {
    windows_core::link!("ktmw32.dll" "system" fn OpenTransaction(dwdesiredaccess : u32, transactionid : *mut windows_core::GUID) -> super::winnt::HANDLE);
    unsafe { OpenTransaction(dwdesiredaccess, transactionid as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn OpenTransactionManager<P0>(logfilename: P0, desiredaccess: super::winnt::ACCESS_MASK, openoptions: u32) -> super::winnt::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ktmw32.dll" "system" fn OpenTransactionManager(logfilename : windows_core::PCWSTR, desiredaccess : super::winnt::ACCESS_MASK, openoptions : u32) -> super::winnt::HANDLE);
    unsafe { OpenTransactionManager(logfilename.param().abi(), desiredaccess, openoptions) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn OpenTransactionManagerById(transactionmanagerid: *const windows_core::GUID, desiredaccess: super::winnt::ACCESS_MASK, openoptions: u32) -> super::winnt::HANDLE {
    windows_core::link!("ktmw32.dll" "system" fn OpenTransactionManagerById(transactionmanagerid : *const windows_core::GUID, desiredaccess : super::winnt::ACCESS_MASK, openoptions : u32) -> super::winnt::HANDLE);
    unsafe { OpenTransactionManagerById(transactionmanagerid, desiredaccess, openoptions) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn PrePrepareComplete(enlistmenthandle: super::winnt::HANDLE, tmvirtualclock: *mut i64) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn PrePrepareComplete(enlistmenthandle : super::winnt::HANDLE, tmvirtualclock : *mut i64) -> windows_core::BOOL);
    unsafe { PrePrepareComplete(enlistmenthandle, tmvirtualclock as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn PrePrepareEnlistment(enlistmenthandle: super::winnt::HANDLE, tmvirtualclock: *mut i64) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn PrePrepareEnlistment(enlistmenthandle : super::winnt::HANDLE, tmvirtualclock : *mut i64) -> windows_core::BOOL);
    unsafe { PrePrepareEnlistment(enlistmenthandle, tmvirtualclock as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn PrepareComplete(enlistmenthandle: super::winnt::HANDLE, tmvirtualclock: *mut i64) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn PrepareComplete(enlistmenthandle : super::winnt::HANDLE, tmvirtualclock : *mut i64) -> windows_core::BOOL);
    unsafe { PrepareComplete(enlistmenthandle, tmvirtualclock as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn PrepareEnlistment(enlistmenthandle: super::winnt::HANDLE, tmvirtualclock: *mut i64) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn PrepareEnlistment(enlistmenthandle : super::winnt::HANDLE, tmvirtualclock : *mut i64) -> windows_core::BOOL);
    unsafe { PrepareEnlistment(enlistmenthandle, tmvirtualclock as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn ReadOnlyEnlistment(enlistmenthandle: super::winnt::HANDLE, tmvirtualclock: *mut i64) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn ReadOnlyEnlistment(enlistmenthandle : super::winnt::HANDLE, tmvirtualclock : *mut i64) -> windows_core::BOOL);
    unsafe { ReadOnlyEnlistment(enlistmenthandle, tmvirtualclock as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn RecoverEnlistment(enlistmenthandle: super::winnt::HANDLE, enlistmentkey: *mut core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn RecoverEnlistment(enlistmenthandle : super::winnt::HANDLE, enlistmentkey : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { RecoverEnlistment(enlistmenthandle, enlistmentkey as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn RecoverResourceManager(resourcemanagerhandle: super::winnt::HANDLE) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn RecoverResourceManager(resourcemanagerhandle : super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { RecoverResourceManager(resourcemanagerhandle) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn RecoverTransactionManager(transactionmanagerhandle: super::winnt::HANDLE) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn RecoverTransactionManager(transactionmanagerhandle : super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { RecoverTransactionManager(transactionmanagerhandle) }
}
#[inline]
pub unsafe fn RenameTransactionManager<P0>(logfilename: P0, existingtransactionmanagerguid: *mut windows_core::GUID) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ktmw32.dll" "system" fn RenameTransactionManager(logfilename : windows_core::PCWSTR, existingtransactionmanagerguid : *mut windows_core::GUID) -> windows_core::BOOL);
    unsafe { RenameTransactionManager(logfilename.param().abi(), existingtransactionmanagerguid as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn RollbackComplete(enlistmenthandle: super::winnt::HANDLE, tmvirtualclock: *mut i64) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn RollbackComplete(enlistmenthandle : super::winnt::HANDLE, tmvirtualclock : *mut i64) -> windows_core::BOOL);
    unsafe { RollbackComplete(enlistmenthandle, tmvirtualclock as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn RollbackEnlistment(enlistmenthandle: super::winnt::HANDLE, tmvirtualclock: *mut i64) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn RollbackEnlistment(enlistmenthandle : super::winnt::HANDLE, tmvirtualclock : *mut i64) -> windows_core::BOOL);
    unsafe { RollbackEnlistment(enlistmenthandle, tmvirtualclock as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn RollbackTransaction(transactionhandle: super::winnt::HANDLE) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn RollbackTransaction(transactionhandle : super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { RollbackTransaction(transactionhandle) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn RollbackTransactionAsync(transactionhandle: super::winnt::HANDLE) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn RollbackTransactionAsync(transactionhandle : super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { RollbackTransactionAsync(transactionhandle) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn RollforwardTransactionManager(transactionmanagerhandle: super::winnt::HANDLE, tmvirtualclock: *mut i64) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn RollforwardTransactionManager(transactionmanagerhandle : super::winnt::HANDLE, tmvirtualclock : *mut i64) -> windows_core::BOOL);
    unsafe { RollforwardTransactionManager(transactionmanagerhandle, tmvirtualclock as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn SetEnlistmentRecoveryInformation(enlistmenthandle: super::winnt::HANDLE, buffersize: u32, buffer: *mut core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn SetEnlistmentRecoveryInformation(enlistmenthandle : super::winnt::HANDLE, buffersize : u32, buffer : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetEnlistmentRecoveryInformation(enlistmenthandle, buffersize, buffer as _) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn SetResourceManagerCompletionPort(resourcemanagerhandle: super::winnt::HANDLE, iocompletionporthandle: super::winnt::HANDLE, completionkey: usize) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn SetResourceManagerCompletionPort(resourcemanagerhandle : super::winnt::HANDLE, iocompletionporthandle : super::winnt::HANDLE, completionkey : usize) -> windows_core::BOOL);
    unsafe { SetResourceManagerCompletionPort(resourcemanagerhandle, iocompletionporthandle, completionkey) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn SetTransactionInformation<P4>(transactionhandle: super::winnt::HANDLE, isolationlevel: u32, isolationflags: u32, timeout: u32, description: P4) -> windows_core::BOOL
where
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("ktmw32.dll" "system" fn SetTransactionInformation(transactionhandle : super::winnt::HANDLE, isolationlevel : u32, isolationflags : u32, timeout : u32, description : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SetTransactionInformation(transactionhandle, isolationlevel, isolationflags, timeout, description.param().abi()) }
}
#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn SinglePhaseReject(enlistmenthandle: super::winnt::HANDLE, tmvirtualclock: *mut i64) -> windows_core::BOOL {
    windows_core::link!("ktmw32.dll" "system" fn SinglePhaseReject(enlistmenthandle : super::winnt::HANDLE, tmvirtualclock : *mut i64) -> windows_core::BOOL);
    unsafe { SinglePhaseReject(enlistmenthandle, tmvirtualclock as _) }
}
