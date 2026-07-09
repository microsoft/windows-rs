#[cfg(feature = "Win32_winnt")]
windows_link::link!("ktmw32.dll" "system" fn CommitComplete(enlistmenthandle : super::winnt::HANDLE, tmvirtualclock : *mut i64) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("ktmw32.dll" "system" fn CommitEnlistment(enlistmenthandle : super::winnt::HANDLE, tmvirtualclock : *mut i64) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("ktmw32.dll" "system" fn CommitTransaction(transactionhandle : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("ktmw32.dll" "system" fn CommitTransactionAsync(transactionhandle : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_ktmtypes", feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("ktmw32.dll" "system" fn CreateEnlistment(lpenlistmentattributes : *mut super::minwinbase::SECURITY_ATTRIBUTES, resourcemanagerhandle : super::winnt::HANDLE, transactionhandle : super::winnt::HANDLE, notificationmask : super::ktmtypes::NOTIFICATION_MASK, createoptions : u32, enlistmentkey : *mut core::ffi::c_void) -> super::winnt::HANDLE);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("ktmw32.dll" "system" fn CreateResourceManager(lpresourcemanagerattributes : *mut super::minwinbase::SECURITY_ATTRIBUTES, resourcemanagerid : *mut windows_sys::core::GUID, createoptions : u32, tmhandle : super::winnt::HANDLE, description : windows_sys::core::PCWSTR) -> super::winnt::HANDLE);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("ktmw32.dll" "system" fn CreateTransaction(lptransactionattributes : *mut super::minwinbase::SECURITY_ATTRIBUTES, uow : *mut windows_sys::core::GUID, createoptions : u32, isolationlevel : u32, isolationflags : u32, timeout : u32, description : windows_sys::core::PCWSTR) -> super::winnt::HANDLE);
#[cfg(all(feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("ktmw32.dll" "system" fn CreateTransactionManager(lptransactionattributes : *mut super::minwinbase::SECURITY_ATTRIBUTES, logfilename : windows_sys::core::PCWSTR, createoptions : u32, commitstrength : u32) -> super::winnt::HANDLE);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("ktmw32.dll" "C" fn GetCurrentClockTransactionManager(transactionmanagerhandle : super::winnt::HANDLE, tmvirtualclock : *mut i64) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("ktmw32.dll" "system" fn GetEnlistmentId(enlistmenthandle : super::winnt::HANDLE, enlistmentid : *mut windows_sys::core::GUID) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("ktmw32.dll" "system" fn GetEnlistmentRecoveryInformation(enlistmenthandle : super::winnt::HANDLE, buffersize : u32, buffer : *mut core::ffi::c_void, bufferused : *mut u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_ktmtypes", feature = "Win32_winnt"))]
windows_link::link!("ktmw32.dll" "system" fn GetNotificationResourceManager(resourcemanagerhandle : super::winnt::HANDLE, transactionnotification : *mut super::ktmtypes::TRANSACTION_NOTIFICATION, notificationlength : u32, dwmilliseconds : u32, returnlength : *mut u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_ktmtypes", feature = "Win32_minwinbase", feature = "Win32_winnt"))]
windows_link::link!("ktmw32.dll" "system" fn GetNotificationResourceManagerAsync(resourcemanagerhandle : super::winnt::HANDLE, transactionnotification : *mut super::ktmtypes::TRANSACTION_NOTIFICATION, transactionnotificationlength : u32, returnlength : *mut u32, lpoverlapped : *mut super::minwinbase::OVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("ktmw32.dll" "system" fn GetTransactionId(transactionhandle : super::winnt::HANDLE, transactionid : *mut windows_sys::core::GUID) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("ktmw32.dll" "system" fn GetTransactionInformation(transactionhandle : super::winnt::HANDLE, outcome : *mut u32, isolationlevel : *mut u32, isolationflags : *mut u32, timeout : *mut u32, bufferlength : u32, description : windows_sys::core::PWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("ktmw32.dll" "C" fn GetTransactionManagerId(transactionmanagerhandle : super::winnt::HANDLE, transactionmanagerid : *mut windows_sys::core::GUID) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("ktmw32.dll" "system" fn OpenEnlistment(dwdesiredaccess : u32, resourcemanagerhandle : super::winnt::HANDLE, enlistmentid : *mut windows_sys::core::GUID) -> super::winnt::HANDLE);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("ktmw32.dll" "system" fn OpenResourceManager(dwdesiredaccess : u32, tmhandle : super::winnt::HANDLE, resourcemanagerid : *mut windows_sys::core::GUID) -> super::winnt::HANDLE);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("ktmw32.dll" "system" fn OpenTransaction(dwdesiredaccess : u32, transactionid : *mut windows_sys::core::GUID) -> super::winnt::HANDLE);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("ktmw32.dll" "system" fn OpenTransactionManager(logfilename : windows_sys::core::PCWSTR, desiredaccess : super::winnt::ACCESS_MASK, openoptions : u32) -> super::winnt::HANDLE);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("ktmw32.dll" "system" fn OpenTransactionManagerById(transactionmanagerid : *const windows_sys::core::GUID, desiredaccess : super::winnt::ACCESS_MASK, openoptions : u32) -> super::winnt::HANDLE);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("ktmw32.dll" "system" fn PrePrepareComplete(enlistmenthandle : super::winnt::HANDLE, tmvirtualclock : *mut i64) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("ktmw32.dll" "system" fn PrePrepareEnlistment(enlistmenthandle : super::winnt::HANDLE, tmvirtualclock : *mut i64) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("ktmw32.dll" "system" fn PrepareComplete(enlistmenthandle : super::winnt::HANDLE, tmvirtualclock : *mut i64) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("ktmw32.dll" "system" fn PrepareEnlistment(enlistmenthandle : super::winnt::HANDLE, tmvirtualclock : *mut i64) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("ktmw32.dll" "system" fn ReadOnlyEnlistment(enlistmenthandle : super::winnt::HANDLE, tmvirtualclock : *mut i64) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("ktmw32.dll" "system" fn RecoverEnlistment(enlistmenthandle : super::winnt::HANDLE, enlistmentkey : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("ktmw32.dll" "system" fn RecoverResourceManager(resourcemanagerhandle : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("ktmw32.dll" "system" fn RecoverTransactionManager(transactionmanagerhandle : super::winnt::HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("ktmw32.dll" "system" fn RenameTransactionManager(logfilename : windows_sys::core::PCWSTR, existingtransactionmanagerguid : *mut windows_sys::core::GUID) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("ktmw32.dll" "system" fn RollbackComplete(enlistmenthandle : super::winnt::HANDLE, tmvirtualclock : *mut i64) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("ktmw32.dll" "system" fn RollbackEnlistment(enlistmenthandle : super::winnt::HANDLE, tmvirtualclock : *mut i64) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("ktmw32.dll" "system" fn RollbackTransaction(transactionhandle : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("ktmw32.dll" "system" fn RollbackTransactionAsync(transactionhandle : super::winnt::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("ktmw32.dll" "system" fn RollforwardTransactionManager(transactionmanagerhandle : super::winnt::HANDLE, tmvirtualclock : *mut i64) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("ktmw32.dll" "system" fn SetEnlistmentRecoveryInformation(enlistmenthandle : super::winnt::HANDLE, buffersize : u32, buffer : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("ktmw32.dll" "system" fn SetResourceManagerCompletionPort(resourcemanagerhandle : super::winnt::HANDLE, iocompletionporthandle : super::winnt::HANDLE, completionkey : usize) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("ktmw32.dll" "system" fn SetTransactionInformation(transactionhandle : super::winnt::HANDLE, isolationlevel : u32, isolationflags : u32, timeout : u32, description : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("ktmw32.dll" "system" fn SinglePhaseReject(enlistmenthandle : super::winnt::HANDLE, tmvirtualclock : *mut i64) -> windows_sys::core::BOOL);
