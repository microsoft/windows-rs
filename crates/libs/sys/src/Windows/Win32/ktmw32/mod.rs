#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn CommitComplete(enlistmenthandle : super::HANDLE, tmvirtualclock : *mut i64) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn CommitEnlistment(enlistmenthandle : super::HANDLE, tmvirtualclock : *mut i64) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn CommitTransaction(transactionhandle : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn CommitTransactionAsync(transactionhandle : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "ktmtypes", feature = "minwinbase", feature = "winnt"))]
windows_link::link!("ktmw32.dll" "system" fn CreateEnlistment(lpenlistmentattributes : *mut super::SECURITY_ATTRIBUTES, resourcemanagerhandle : super::HANDLE, transactionhandle : super::HANDLE, notificationmask : super::NOTIFICATION_MASK, createoptions : u32, enlistmentkey : *mut core::ffi::c_void) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("ktmw32.dll" "system" fn CreateResourceManager(lpresourcemanagerattributes : *mut super::SECURITY_ATTRIBUTES, resourcemanagerid : *mut windows_sys::core::GUID, createoptions : u32, tmhandle : super::HANDLE, description : windows_sys::core::PCWSTR) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("ktmw32.dll" "system" fn CreateTransaction(lptransactionattributes : *mut super::SECURITY_ATTRIBUTES, uow : *mut windows_sys::core::GUID, createoptions : u32, isolationlevel : u32, isolationflags : u32, timeout : u32, description : windows_sys::core::PCWSTR) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("ktmw32.dll" "system" fn CreateTransactionManager(lptransactionattributes : *mut super::SECURITY_ATTRIBUTES, logfilename : windows_sys::core::PCWSTR, createoptions : u32, commitstrength : u32) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "C" fn GetCurrentClockTransactionManager(transactionmanagerhandle : super::HANDLE, tmvirtualclock : *mut i64) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn GetEnlistmentId(enlistmenthandle : super::HANDLE, enlistmentid : *mut windows_sys::core::GUID) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn GetEnlistmentRecoveryInformation(enlistmenthandle : super::HANDLE, buffersize : u32, buffer : *mut core::ffi::c_void, bufferused : *mut u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "ktmtypes", feature = "winnt"))]
windows_link::link!("ktmw32.dll" "system" fn GetNotificationResourceManager(resourcemanagerhandle : super::HANDLE, transactionnotification : *mut super::TRANSACTION_NOTIFICATION, notificationlength : u32, dwmilliseconds : u32, returnlength : *mut u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "ktmtypes", feature = "minwinbase", feature = "winnt"))]
windows_link::link!("ktmw32.dll" "system" fn GetNotificationResourceManagerAsync(resourcemanagerhandle : super::HANDLE, transactionnotification : *mut super::TRANSACTION_NOTIFICATION, transactionnotificationlength : u32, returnlength : *mut u32, lpoverlapped : *mut super::OVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn GetTransactionId(transactionhandle : super::HANDLE, transactionid : *mut windows_sys::core::GUID) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn GetTransactionInformation(transactionhandle : super::HANDLE, outcome : *mut u32, isolationlevel : *mut u32, isolationflags : *mut u32, timeout : *mut u32, bufferlength : u32, description : windows_sys::core::PWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "C" fn GetTransactionManagerId(transactionmanagerhandle : super::HANDLE, transactionmanagerid : *mut windows_sys::core::GUID) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn OpenEnlistment(dwdesiredaccess : u32, resourcemanagerhandle : super::HANDLE, enlistmentid : *mut windows_sys::core::GUID) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn OpenResourceManager(dwdesiredaccess : u32, tmhandle : super::HANDLE, resourcemanagerid : *mut windows_sys::core::GUID) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn OpenTransaction(dwdesiredaccess : u32, transactionid : *mut windows_sys::core::GUID) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn OpenTransactionManager(logfilename : windows_sys::core::PCWSTR, desiredaccess : super::ACCESS_MASK, openoptions : u32) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn OpenTransactionManagerById(transactionmanagerid : *const windows_sys::core::GUID, desiredaccess : super::ACCESS_MASK, openoptions : u32) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn PrePrepareComplete(enlistmenthandle : super::HANDLE, tmvirtualclock : *mut i64) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn PrePrepareEnlistment(enlistmenthandle : super::HANDLE, tmvirtualclock : *mut i64) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn PrepareComplete(enlistmenthandle : super::HANDLE, tmvirtualclock : *mut i64) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn PrepareEnlistment(enlistmenthandle : super::HANDLE, tmvirtualclock : *mut i64) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn ReadOnlyEnlistment(enlistmenthandle : super::HANDLE, tmvirtualclock : *mut i64) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn RecoverEnlistment(enlistmenthandle : super::HANDLE, enlistmentkey : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn RecoverResourceManager(resourcemanagerhandle : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn RecoverTransactionManager(transactionmanagerhandle : super::HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("ktmw32.dll" "system" fn RenameTransactionManager(logfilename : windows_sys::core::PCWSTR, existingtransactionmanagerguid : *mut windows_sys::core::GUID) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn RollbackComplete(enlistmenthandle : super::HANDLE, tmvirtualclock : *mut i64) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn RollbackEnlistment(enlistmenthandle : super::HANDLE, tmvirtualclock : *mut i64) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn RollbackTransaction(transactionhandle : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn RollbackTransactionAsync(transactionhandle : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn RollforwardTransactionManager(transactionmanagerhandle : super::HANDLE, tmvirtualclock : *mut i64) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn SetEnlistmentRecoveryInformation(enlistmenthandle : super::HANDLE, buffersize : u32, buffer : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn SetResourceManagerCompletionPort(resourcemanagerhandle : super::HANDLE, iocompletionporthandle : super::HANDLE, completionkey : usize) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn SetTransactionInformation(transactionhandle : super::HANDLE, isolationlevel : u32, isolationflags : u32, timeout : u32, description : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn SinglePhaseReject(enlistmenthandle : super::HANDLE, tmvirtualclock : *mut i64) -> windows_sys::core::BOOL);
