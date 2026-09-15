#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn CommitComplete(enlistmenthandle : super::HANDLE, tmvirtualclock : super::PLARGE_INTEGER) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn CommitEnlistment(enlistmenthandle : super::HANDLE, tmvirtualclock : super::PLARGE_INTEGER) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn CommitTransaction(transactionhandle : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn CommitTransactionAsync(transactionhandle : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "ktmtypes", feature = "minwinbase", feature = "winnt"))]
windows_link::link!("ktmw32.dll" "system" fn CreateEnlistment(lpenlistmentattributes : super::LPSECURITY_ATTRIBUTES, resourcemanagerhandle : super::HANDLE, transactionhandle : super::HANDLE, notificationmask : super::NOTIFICATION_MASK, createoptions : u32, enlistmentkey : *const core::ffi::c_void) -> super::HANDLE);
#[cfg(all(feature = "guiddef", feature = "minwinbase", feature = "winnt"))]
windows_link::link!("ktmw32.dll" "system" fn CreateResourceManager(lpresourcemanagerattributes : super::LPSECURITY_ATTRIBUTES, resourcemanagerid : super::LPGUID, createoptions : u32, tmhandle : super::HANDLE, description : windows_sys::core::PCWSTR) -> super::HANDLE);
#[cfg(all(feature = "guiddef", feature = "minwinbase", feature = "winnt"))]
windows_link::link!("ktmw32.dll" "system" fn CreateTransaction(lptransactionattributes : super::LPSECURITY_ATTRIBUTES, uow : super::LPGUID, createoptions : u32, isolationlevel : u32, isolationflags : u32, timeout : u32, description : windows_sys::core::PCWSTR) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("ktmw32.dll" "system" fn CreateTransactionManager(lptransactionattributes : super::LPSECURITY_ATTRIBUTES, logfilename : windows_sys::core::PCWSTR, createoptions : u32, commitstrength : u32) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "C" fn GetCurrentClockTransactionManager(transactionmanagerhandle : super::HANDLE, tmvirtualclock : super::PLARGE_INTEGER) -> windows_sys::core::BOOL);
#[cfg(all(feature = "guiddef", feature = "winnt"))]
windows_link::link!("ktmw32.dll" "system" fn GetEnlistmentId(enlistmenthandle : super::HANDLE, enlistmentid : super::LPGUID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("ktmw32.dll" "system" fn GetEnlistmentRecoveryInformation(enlistmenthandle : super::HANDLE, buffersize : u32, buffer : *mut core::ffi::c_void, bufferused : super::PULONG) -> windows_sys::core::BOOL);
#[cfg(all(feature = "ktmtypes", feature = "minwindef", feature = "winnt"))]
windows_link::link!("ktmw32.dll" "system" fn GetNotificationResourceManager(resourcemanagerhandle : super::HANDLE, transactionnotification : super::PTRANSACTION_NOTIFICATION, notificationlength : u32, dwmilliseconds : u32, returnlength : super::PULONG) -> windows_sys::core::BOOL);
#[cfg(all(feature = "ktmtypes", feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
windows_link::link!("ktmw32.dll" "system" fn GetNotificationResourceManagerAsync(resourcemanagerhandle : super::HANDLE, transactionnotification : super::PTRANSACTION_NOTIFICATION, transactionnotificationlength : u32, returnlength : super::PULONG, lpoverlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "guiddef", feature = "winnt"))]
windows_link::link!("ktmw32.dll" "system" fn GetTransactionId(transactionhandle : super::HANDLE, transactionid : super::LPGUID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("ktmw32.dll" "system" fn GetTransactionInformation(transactionhandle : super::HANDLE, outcome : super::PDWORD, isolationlevel : super::PDWORD, isolationflags : super::PDWORD, timeout : super::PDWORD, bufferlength : u32, description : windows_sys::core::PWSTR) -> windows_sys::core::BOOL);
#[cfg(all(feature = "guiddef", feature = "winnt"))]
windows_link::link!("ktmw32.dll" "C" fn GetTransactionManagerId(transactionmanagerhandle : super::HANDLE, transactionmanagerid : super::LPGUID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "guiddef", feature = "winnt"))]
windows_link::link!("ktmw32.dll" "system" fn OpenEnlistment(dwdesiredaccess : u32, resourcemanagerhandle : super::HANDLE, enlistmentid : super::LPGUID) -> super::HANDLE);
#[cfg(all(feature = "guiddef", feature = "winnt"))]
windows_link::link!("ktmw32.dll" "system" fn OpenResourceManager(dwdesiredaccess : u32, tmhandle : super::HANDLE, resourcemanagerid : super::LPGUID) -> super::HANDLE);
#[cfg(all(feature = "guiddef", feature = "winnt"))]
windows_link::link!("ktmw32.dll" "system" fn OpenTransaction(dwdesiredaccess : u32, transactionid : super::LPGUID) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn OpenTransactionManager(logfilename : windows_sys::core::PCWSTR, desiredaccess : super::ACCESS_MASK, openoptions : u32) -> super::HANDLE);
#[cfg(all(feature = "guiddef", feature = "winnt"))]
windows_link::link!("ktmw32.dll" "system" fn OpenTransactionManagerById(transactionmanagerid : super::LPGUID, desiredaccess : super::ACCESS_MASK, openoptions : u32) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn PrePrepareComplete(enlistmenthandle : super::HANDLE, tmvirtualclock : super::PLARGE_INTEGER) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn PrePrepareEnlistment(enlistmenthandle : super::HANDLE, tmvirtualclock : super::PLARGE_INTEGER) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn PrepareComplete(enlistmenthandle : super::HANDLE, tmvirtualclock : super::PLARGE_INTEGER) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn PrepareEnlistment(enlistmenthandle : super::HANDLE, tmvirtualclock : super::PLARGE_INTEGER) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn ReadOnlyEnlistment(enlistmenthandle : super::HANDLE, tmvirtualclock : super::PLARGE_INTEGER) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn RecoverEnlistment(enlistmenthandle : super::HANDLE, enlistmentkey : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn RecoverResourceManager(resourcemanagerhandle : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn RecoverTransactionManager(transactionmanagerhandle : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "guiddef")]
windows_link::link!("ktmw32.dll" "system" fn RenameTransactionManager(logfilename : windows_sys::core::PCWSTR, existingtransactionmanagerguid : super::LPGUID) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn RollbackComplete(enlistmenthandle : super::HANDLE, tmvirtualclock : super::PLARGE_INTEGER) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn RollbackEnlistment(enlistmenthandle : super::HANDLE, tmvirtualclock : super::PLARGE_INTEGER) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn RollbackTransaction(transactionhandle : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn RollbackTransactionAsync(transactionhandle : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn RollforwardTransactionManager(transactionmanagerhandle : super::HANDLE, tmvirtualclock : super::PLARGE_INTEGER) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn SetEnlistmentRecoveryInformation(enlistmenthandle : super::HANDLE, buffersize : u32, buffer : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn SetResourceManagerCompletionPort(resourcemanagerhandle : super::HANDLE, iocompletionporthandle : super::HANDLE, completionkey : usize) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn SetTransactionInformation(transactionhandle : super::HANDLE, isolationlevel : u32, isolationflags : u32, timeout : u32, description : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("ktmw32.dll" "system" fn SinglePhaseReject(enlistmenthandle : super::HANDLE, tmvirtualclock : super::PLARGE_INTEGER) -> windows_sys::core::BOOL);
