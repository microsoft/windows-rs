#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CfCloseHandle(filehandle: super::HANDLE) {
    windows_core::link!("cldapi.dll" "system" fn CfCloseHandle(filehandle : super::HANDLE));
    unsafe { CfCloseHandle(filehandle) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CfConnectSyncRoot<P0>(syncrootpath: P0, callbacktable: *const CF_CALLBACK_REGISTRATION, callbackcontext: Option<*const core::ffi::c_void>, connectflags: CF_CONNECT_FLAGS) -> windows_core::Result<CF_CONNECTION_KEY>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cldapi.dll" "system" fn CfConnectSyncRoot(syncrootpath : windows_core::PCWSTR, callbacktable : *const CF_CALLBACK_REGISTRATION, callbackcontext : *const core::ffi::c_void, connectflags : CF_CONNECT_FLAGS, connectionkey : *mut CF_CONNECTION_KEY) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CfConnectSyncRoot(syncrootpath.param().abi(), callbacktable, callbackcontext.unwrap_or(core::mem::zeroed()) as _, connectflags, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn CfConvertToPlaceholder(filehandle: super::HANDLE, fileidentity: Option<*const core::ffi::c_void>, fileidentitylength: u32, convertflags: CF_CONVERT_FLAGS, convertusn: Option<*mut super::USN>, overlapped: Option<*mut super::OVERLAPPED>) -> windows_core::HRESULT {
    windows_core::link!("cldapi.dll" "system" fn CfConvertToPlaceholder(filehandle : super::HANDLE, fileidentity : *const core::ffi::c_void, fileidentitylength : u32, convertflags : CF_CONVERT_FLAGS, convertusn : *mut super::USN, overlapped : *mut super::OVERLAPPED) -> windows_core::HRESULT);
    unsafe { CfConvertToPlaceholder(filehandle, fileidentity.unwrap_or(core::mem::zeroed()) as _, fileidentitylength, convertflags, convertusn.unwrap_or(core::mem::zeroed()) as _, overlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "winbase", feature = "winnt"))]
#[inline]
pub unsafe fn CfCreatePlaceholders<P0>(basedirectorypath: P0, placeholderarray: &mut [CF_PLACEHOLDER_CREATE_INFO], createflags: CF_CREATE_FLAGS, entriesprocessed: Option<*mut u32>) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cldapi.dll" "system" fn CfCreatePlaceholders(basedirectorypath : windows_core::PCWSTR, placeholderarray : *mut CF_PLACEHOLDER_CREATE_INFO, placeholdercount : u32, createflags : CF_CREATE_FLAGS, entriesprocessed : *mut u32) -> windows_core::HRESULT);
    unsafe { CfCreatePlaceholders(basedirectorypath.param().abi(), placeholderarray.as_mut_ptr(), placeholderarray.len().try_into().unwrap(), createflags, entriesprocessed.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn CfDehydratePlaceholder(filehandle: super::HANDLE, startingoffset: i64, length: i64, dehydrateflags: CF_DEHYDRATE_FLAGS, overlapped: Option<*mut super::OVERLAPPED>) -> windows_core::HRESULT {
    windows_core::link!("cldapi.dll" "system" fn CfDehydratePlaceholder(filehandle : super::HANDLE, startingoffset : i64, length : i64, dehydrateflags : CF_DEHYDRATE_FLAGS, overlapped : *mut super::OVERLAPPED) -> windows_core::HRESULT);
    unsafe { CfDehydratePlaceholder(filehandle, startingoffset, length, dehydrateflags, overlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CfDisconnectSyncRoot(connectionkey: CF_CONNECTION_KEY) -> windows_core::HRESULT {
    windows_core::link!("cldapi.dll" "system" fn CfDisconnectSyncRoot(connectionkey : CF_CONNECTION_KEY) -> windows_core::HRESULT);
    unsafe { CfDisconnectSyncRoot(connectionkey) }
}
#[cfg(all(feature = "bcrypt", feature = "winbase", feature = "winnt"))]
#[inline]
pub unsafe fn CfExecute(opinfo: *const CF_OPERATION_INFO, opparams: *mut CF_OPERATION_PARAMETERS) -> windows_core::HRESULT {
    windows_core::link!("cldapi.dll" "system" fn CfExecute(opinfo : *const CF_OPERATION_INFO, opparams : *mut CF_OPERATION_PARAMETERS) -> windows_core::HRESULT);
    unsafe { CfExecute(opinfo, opparams as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CfGetCorrelationVector(filehandle: super::HANDLE, correlationvector: *mut super::CORRELATION_VECTOR) -> windows_core::HRESULT {
    windows_core::link!("cldapi.dll" "system" fn CfGetCorrelationVector(filehandle : super::HANDLE, correlationvector : *mut super::CORRELATION_VECTOR) -> windows_core::HRESULT);
    unsafe { CfGetCorrelationVector(filehandle, correlationvector as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CfGetPlaceholderInfo(filehandle: super::HANDLE, infoclass: CF_PLACEHOLDER_INFO_CLASS, infobuffer: *mut core::ffi::c_void, infobufferlength: u32, returnedlength: Option<*mut u32>) -> windows_core::HRESULT {
    windows_core::link!("cldapi.dll" "system" fn CfGetPlaceholderInfo(filehandle : super::HANDLE, infoclass : CF_PLACEHOLDER_INFO_CLASS, infobuffer : *mut core::ffi::c_void, infobufferlength : u32, returnedlength : *mut u32) -> windows_core::HRESULT);
    unsafe { CfGetPlaceholderInfo(filehandle, infoclass, infobuffer as _, infobufferlength, returnedlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CfGetPlaceholderRangeInfo(filehandle: super::HANDLE, infoclass: CF_PLACEHOLDER_RANGE_INFO_CLASS, startingoffset: i64, length: i64, infobuffer: *mut core::ffi::c_void, infobufferlength: u32, returnedlength: Option<*mut u32>) -> windows_core::HRESULT {
    windows_core::link!("cldapi.dll" "system" fn CfGetPlaceholderRangeInfo(filehandle : super::HANDLE, infoclass : CF_PLACEHOLDER_RANGE_INFO_CLASS, startingoffset : i64, length : i64, infobuffer : *mut core::ffi::c_void, infobufferlength : u32, returnedlength : *mut u32) -> windows_core::HRESULT);
    unsafe { CfGetPlaceholderRangeInfo(filehandle, infoclass, startingoffset, length, infobuffer as _, infobufferlength, returnedlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CfGetPlaceholderRangeInfoForHydration(connectionkey: CF_CONNECTION_KEY, transferkey: CF_TRANSFER_KEY, fileid: i64, infoclass: CF_PLACEHOLDER_RANGE_INFO_CLASS, startingoffset: i64, rangelength: i64, infobuffer: *mut core::ffi::c_void, infobuffersize: u32, infobufferwritten: Option<*mut u32>) -> windows_core::HRESULT {
    windows_core::link!("cldapi.dll" "system" fn CfGetPlaceholderRangeInfoForHydration(connectionkey : CF_CONNECTION_KEY, transferkey : CF_TRANSFER_KEY, fileid : i64, infoclass : CF_PLACEHOLDER_RANGE_INFO_CLASS, startingoffset : i64, rangelength : i64, infobuffer : *mut core::ffi::c_void, infobuffersize : u32, infobufferwritten : *mut u32) -> windows_core::HRESULT);
    unsafe { CfGetPlaceholderRangeInfoForHydration(connectionkey, transferkey, fileid, infoclass, startingoffset, rangelength, infobuffer as _, infobuffersize, infobufferwritten.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CfGetPlaceholderStateFromAttributeTag(fileattributes: u32, reparsetag: u32) -> CF_PLACEHOLDER_STATE {
    windows_core::link!("cldapi.dll" "system" fn CfGetPlaceholderStateFromAttributeTag(fileattributes : u32, reparsetag : u32) -> CF_PLACEHOLDER_STATE);
    unsafe { CfGetPlaceholderStateFromAttributeTag(fileattributes, reparsetag) }
}
#[cfg(feature = "minwinbase")]
#[inline]
pub unsafe fn CfGetPlaceholderStateFromFileInfo(infobuffer: *const core::ffi::c_void, infoclass: super::FILE_INFO_BY_HANDLE_CLASS) -> CF_PLACEHOLDER_STATE {
    windows_core::link!("cldapi.dll" "system" fn CfGetPlaceholderStateFromFileInfo(infobuffer : *const core::ffi::c_void, infoclass : super::FILE_INFO_BY_HANDLE_CLASS) -> CF_PLACEHOLDER_STATE);
    unsafe { CfGetPlaceholderStateFromFileInfo(infobuffer, infoclass) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef"))]
#[inline]
pub unsafe fn CfGetPlaceholderStateFromFindData(finddata: *const super::WIN32_FIND_DATA) -> CF_PLACEHOLDER_STATE {
    windows_core::link!("cldapi.dll" "system" fn CfGetPlaceholderStateFromFindData(finddata : *const super::WIN32_FIND_DATA) -> CF_PLACEHOLDER_STATE);
    unsafe { CfGetPlaceholderStateFromFindData(finddata) }
}
#[inline]
pub unsafe fn CfGetPlatformInfo() -> windows_core::Result<CF_PLATFORM_INFO> {
    windows_core::link!("cldapi.dll" "system" fn CfGetPlatformInfo(platformversion : *mut CF_PLATFORM_INFO) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CfGetPlatformInfo(&mut result__).map(|| result__)
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CfGetSyncRootInfoByHandle(filehandle: super::HANDLE, infoclass: CF_SYNC_ROOT_INFO_CLASS, infobuffer: *mut core::ffi::c_void, infobufferlength: u32, returnedlength: Option<*mut u32>) -> windows_core::HRESULT {
    windows_core::link!("cldapi.dll" "system" fn CfGetSyncRootInfoByHandle(filehandle : super::HANDLE, infoclass : CF_SYNC_ROOT_INFO_CLASS, infobuffer : *mut core::ffi::c_void, infobufferlength : u32, returnedlength : *mut u32) -> windows_core::HRESULT);
    unsafe { CfGetSyncRootInfoByHandle(filehandle, infoclass, infobuffer as _, infobufferlength, returnedlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CfGetSyncRootInfoByPath<P0>(filepath: P0, infoclass: CF_SYNC_ROOT_INFO_CLASS, infobuffer: *mut core::ffi::c_void, infobufferlength: u32, returnedlength: Option<*mut u32>) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cldapi.dll" "system" fn CfGetSyncRootInfoByPath(filepath : windows_core::PCWSTR, infoclass : CF_SYNC_ROOT_INFO_CLASS, infobuffer : *mut core::ffi::c_void, infobufferlength : u32, returnedlength : *mut u32) -> windows_core::HRESULT);
    unsafe { CfGetSyncRootInfoByPath(filepath.param().abi(), infoclass, infobuffer as _, infobufferlength, returnedlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CfGetTransferKey(filehandle: super::HANDLE) -> windows_core::Result<CF_TRANSFER_KEY> {
    windows_core::link!("cldapi.dll" "system" fn CfGetTransferKey(filehandle : super::HANDLE, transferkey : *mut CF_TRANSFER_KEY) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CfGetTransferKey(filehandle, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CfGetWin32HandleFromProtectedHandle(protectedhandle: super::HANDLE) -> super::HANDLE {
    windows_core::link!("cldapi.dll" "system" fn CfGetWin32HandleFromProtectedHandle(protectedhandle : super::HANDLE) -> super::HANDLE);
    unsafe { CfGetWin32HandleFromProtectedHandle(protectedhandle) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn CfHydratePlaceholder(filehandle: super::HANDLE, startingoffset: i64, length: i64, hydrateflags: CF_HYDRATE_FLAGS, overlapped: Option<*mut super::OVERLAPPED>) -> windows_core::HRESULT {
    windows_core::link!("cldapi.dll" "system" fn CfHydratePlaceholder(filehandle : super::HANDLE, startingoffset : i64, length : i64, hydrateflags : CF_HYDRATE_FLAGS, overlapped : *mut super::OVERLAPPED) -> windows_core::HRESULT);
    unsafe { CfHydratePlaceholder(filehandle, startingoffset, length, hydrateflags, overlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CfOpenFileWithOplock<P0>(filepath: P0, flags: CF_OPEN_FILE_FLAGS) -> windows_core::Result<super::HANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cldapi.dll" "system" fn CfOpenFileWithOplock(filepath : windows_core::PCWSTR, flags : CF_OPEN_FILE_FLAGS, protectedhandle : *mut super::HANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CfOpenFileWithOplock(filepath.param().abi(), flags, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn CfQuerySyncProviderStatus(connectionkey: CF_CONNECTION_KEY) -> windows_core::Result<CF_SYNC_PROVIDER_STATUS> {
    windows_core::link!("cldapi.dll" "system" fn CfQuerySyncProviderStatus(connectionkey : CF_CONNECTION_KEY, providerstatus : *mut CF_SYNC_PROVIDER_STATUS) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CfQuerySyncProviderStatus(connectionkey, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CfReferenceProtectedHandle(protectedhandle: super::HANDLE) -> bool {
    windows_core::link!("cldapi.dll" "system" fn CfReferenceProtectedHandle(protectedhandle : super::HANDLE) -> bool);
    unsafe { CfReferenceProtectedHandle(protectedhandle) }
}
#[inline]
pub unsafe fn CfRegisterSyncRoot<P0>(syncrootpath: P0, registration: *const CF_SYNC_REGISTRATION, policies: *const CF_SYNC_POLICIES, registerflags: CF_REGISTER_FLAGS) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cldapi.dll" "system" fn CfRegisterSyncRoot(syncrootpath : windows_core::PCWSTR, registration : *const CF_SYNC_REGISTRATION, policies : *const CF_SYNC_POLICIES, registerflags : CF_REGISTER_FLAGS) -> windows_core::HRESULT);
    unsafe { CfRegisterSyncRoot(syncrootpath.param().abi(), registration, policies, registerflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CfReleaseProtectedHandle(protectedhandle: super::HANDLE) {
    windows_core::link!("cldapi.dll" "system" fn CfReleaseProtectedHandle(protectedhandle : super::HANDLE));
    unsafe { CfReleaseProtectedHandle(protectedhandle) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CfReleaseTransferKey(filehandle: super::HANDLE) -> CF_TRANSFER_KEY {
    windows_core::link!("cldapi.dll" "system" fn CfReleaseTransferKey(filehandle : super::HANDLE, transferkey : *mut CF_TRANSFER_KEY));
    unsafe {
        let mut result__ = core::mem::zeroed();
        CfReleaseTransferKey(filehandle, &mut result__);
        result__
    }
}
#[inline]
pub unsafe fn CfReportProviderProgress(connectionkey: CF_CONNECTION_KEY, transferkey: CF_TRANSFER_KEY, providerprogresstotal: i64, providerprogresscompleted: i64) -> windows_core::HRESULT {
    windows_core::link!("cldapi.dll" "system" fn CfReportProviderProgress(connectionkey : CF_CONNECTION_KEY, transferkey : CF_TRANSFER_KEY, providerprogresstotal : i64, providerprogresscompleted : i64) -> windows_core::HRESULT);
    unsafe { CfReportProviderProgress(connectionkey, transferkey, providerprogresstotal, providerprogresscompleted) }
}
#[inline]
pub unsafe fn CfReportProviderProgress2(connectionkey: CF_CONNECTION_KEY, transferkey: CF_TRANSFER_KEY, requestkey: CF_REQUEST_KEY, providerprogresstotal: i64, providerprogresscompleted: i64, targetsessionid: u32) -> windows_core::HRESULT {
    windows_core::link!("cldapi.dll" "system" fn CfReportProviderProgress2(connectionkey : CF_CONNECTION_KEY, transferkey : CF_TRANSFER_KEY, requestkey : CF_REQUEST_KEY, providerprogresstotal : i64, providerprogresscompleted : i64, targetsessionid : u32) -> windows_core::HRESULT);
    unsafe { CfReportProviderProgress2(connectionkey, transferkey, requestkey, providerprogresstotal, providerprogresscompleted, targetsessionid) }
}
#[inline]
pub unsafe fn CfReportSyncStatus<P0>(syncrootpath: P0, syncstatus: Option<*const CF_SYNC_STATUS>) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cldapi.dll" "system" fn CfReportSyncStatus(syncrootpath : windows_core::PCWSTR, syncstatus : *const CF_SYNC_STATUS) -> windows_core::HRESULT);
    unsafe { CfReportSyncStatus(syncrootpath.param().abi(), syncstatus.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn CfRevertPlaceholder(filehandle: super::HANDLE, revertflags: CF_REVERT_FLAGS, overlapped: Option<*mut super::OVERLAPPED>) -> windows_core::HRESULT {
    windows_core::link!("cldapi.dll" "system" fn CfRevertPlaceholder(filehandle : super::HANDLE, revertflags : CF_REVERT_FLAGS, overlapped : *mut super::OVERLAPPED) -> windows_core::HRESULT);
    unsafe { CfRevertPlaceholder(filehandle, revertflags, overlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CfSetCorrelationVector(filehandle: super::HANDLE, correlationvector: *const super::CORRELATION_VECTOR) -> windows_core::HRESULT {
    windows_core::link!("cldapi.dll" "system" fn CfSetCorrelationVector(filehandle : super::HANDLE, correlationvector : *const super::CORRELATION_VECTOR) -> windows_core::HRESULT);
    unsafe { CfSetCorrelationVector(filehandle, correlationvector) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CfSetInSyncState(filehandle: super::HANDLE, insyncstate: CF_IN_SYNC_STATE, insyncflags: CF_SET_IN_SYNC_FLAGS, insyncusn: Option<*mut super::USN>) -> windows_core::HRESULT {
    windows_core::link!("cldapi.dll" "system" fn CfSetInSyncState(filehandle : super::HANDLE, insyncstate : CF_IN_SYNC_STATE, insyncflags : CF_SET_IN_SYNC_FLAGS, insyncusn : *mut super::USN) -> windows_core::HRESULT);
    unsafe { CfSetInSyncState(filehandle, insyncstate, insyncflags, insyncusn.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn CfSetPinState(filehandle: super::HANDLE, pinstate: CF_PIN_STATE, pinflags: CF_SET_PIN_FLAGS, overlapped: Option<*mut super::OVERLAPPED>) -> windows_core::HRESULT {
    windows_core::link!("cldapi.dll" "system" fn CfSetPinState(filehandle : super::HANDLE, pinstate : CF_PIN_STATE, pinflags : CF_SET_PIN_FLAGS, overlapped : *mut super::OVERLAPPED) -> windows_core::HRESULT);
    unsafe { CfSetPinState(filehandle, pinstate, pinflags, overlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CfUnregisterSyncRoot<P0>(syncrootpath: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("cldapi.dll" "system" fn CfUnregisterSyncRoot(syncrootpath : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { CfUnregisterSyncRoot(syncrootpath.param().abi()) }
}
#[cfg(all(feature = "minwinbase", feature = "winbase", feature = "winnt"))]
#[inline]
pub unsafe fn CfUpdatePlaceholder(filehandle: super::HANDLE, fsmetadata: Option<*const CF_FS_METADATA>, fileidentity: Option<*const core::ffi::c_void>, fileidentitylength: u32, dehydraterangearray: Option<&[CF_FILE_RANGE]>, updateflags: CF_UPDATE_FLAGS, updateusn: Option<*mut super::USN>, overlapped: Option<*mut super::OVERLAPPED>) -> windows_core::HRESULT {
    windows_core::link!("cldapi.dll" "system" fn CfUpdatePlaceholder(filehandle : super::HANDLE, fsmetadata : *const CF_FS_METADATA, fileidentity : *const core::ffi::c_void, fileidentitylength : u32, dehydraterangearray : *const CF_FILE_RANGE, dehydraterangecount : u32, updateflags : CF_UPDATE_FLAGS, updateusn : *mut super::USN, overlapped : *mut super::OVERLAPPED) -> windows_core::HRESULT);
    unsafe { CfUpdatePlaceholder(filehandle, fsmetadata.unwrap_or(core::mem::zeroed()) as _, fileidentity.unwrap_or(core::mem::zeroed()) as _, fileidentitylength, dehydraterangearray.map_or(core::ptr::null(), |slice| slice.as_ptr()), dehydraterangearray.map_or(0, |slice| slice.len().try_into().unwrap()), updateflags, updateusn.unwrap_or(core::mem::zeroed()) as _, overlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CfUpdateSyncProviderStatus(connectionkey: CF_CONNECTION_KEY, providerstatus: CF_SYNC_PROVIDER_STATUS) -> windows_core::HRESULT {
    windows_core::link!("cldapi.dll" "system" fn CfUpdateSyncProviderStatus(connectionkey : CF_CONNECTION_KEY, providerstatus : CF_SYNC_PROVIDER_STATUS) -> windows_core::HRESULT);
    unsafe { CfUpdateSyncProviderStatus(connectionkey, providerstatus) }
}
#[cfg(feature = "winnt")]
pub type CF_CALLBACK = Option<unsafe extern "system" fn(callbackinfo: *const CF_CALLBACK_INFO, callbackparameters: *const CF_CALLBACK_PARAMETERS)>;
pub type CF_CALLBACK_CANCEL_FLAGS = u32;
pub const CF_CALLBACK_CANCEL_FLAG_IO_ABORTED: CF_CALLBACK_CANCEL_FLAGS = 2;
pub const CF_CALLBACK_CANCEL_FLAG_IO_TIMEOUT: CF_CALLBACK_CANCEL_FLAGS = 1;
pub const CF_CALLBACK_CANCEL_FLAG_NONE: CF_CALLBACK_CANCEL_FLAGS = 0;
pub type CF_CALLBACK_CLOSE_COMPLETION_FLAGS = u32;
pub const CF_CALLBACK_CLOSE_COMPLETION_FLAG_DELETED: CF_CALLBACK_CLOSE_COMPLETION_FLAGS = 1;
pub const CF_CALLBACK_CLOSE_COMPLETION_FLAG_NONE: CF_CALLBACK_CLOSE_COMPLETION_FLAGS = 0;
pub type CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS = u32;
pub const CF_CALLBACK_DEHYDRATE_COMPLETION_FLAG_BACKGROUND: CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS = 1;
pub const CF_CALLBACK_DEHYDRATE_COMPLETION_FLAG_DEHYDRATED: CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS = 2;
pub const CF_CALLBACK_DEHYDRATE_COMPLETION_FLAG_NONE: CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS = 0;
pub type CF_CALLBACK_DEHYDRATE_FLAGS = u32;
pub const CF_CALLBACK_DEHYDRATE_FLAG_BACKGROUND: CF_CALLBACK_DEHYDRATE_FLAGS = 1;
pub const CF_CALLBACK_DEHYDRATE_FLAG_NONE: CF_CALLBACK_DEHYDRATE_FLAGS = 0;
pub type CF_CALLBACK_DEHYDRATION_REASON = i32;
pub const CF_CALLBACK_DEHYDRATION_REASON_NONE: CF_CALLBACK_DEHYDRATION_REASON = 0;
pub const CF_CALLBACK_DEHYDRATION_REASON_SYSTEM_INACTIVITY: CF_CALLBACK_DEHYDRATION_REASON = 3;
pub const CF_CALLBACK_DEHYDRATION_REASON_SYSTEM_LOW_SPACE: CF_CALLBACK_DEHYDRATION_REASON = 2;
pub const CF_CALLBACK_DEHYDRATION_REASON_SYSTEM_OS_UPGRADE: CF_CALLBACK_DEHYDRATION_REASON = 4;
pub const CF_CALLBACK_DEHYDRATION_REASON_USER_MANUAL: CF_CALLBACK_DEHYDRATION_REASON = 1;
pub type CF_CALLBACK_DELETE_COMPLETION_FLAGS = u32;
pub const CF_CALLBACK_DELETE_COMPLETION_FLAG_NONE: CF_CALLBACK_DELETE_COMPLETION_FLAGS = 0;
pub type CF_CALLBACK_DELETE_FLAGS = u32;
pub const CF_CALLBACK_DELETE_FLAG_IS_DIRECTORY: CF_CALLBACK_DELETE_FLAGS = 1;
pub const CF_CALLBACK_DELETE_FLAG_IS_UNDELETE: CF_CALLBACK_DELETE_FLAGS = 2;
pub const CF_CALLBACK_DELETE_FLAG_NONE: CF_CALLBACK_DELETE_FLAGS = 0;
pub type CF_CALLBACK_FETCH_DATA_FLAGS = u32;
pub const CF_CALLBACK_FETCH_DATA_FLAG_EXPLICIT_HYDRATION: CF_CALLBACK_FETCH_DATA_FLAGS = 2;
pub const CF_CALLBACK_FETCH_DATA_FLAG_NONE: CF_CALLBACK_FETCH_DATA_FLAGS = 0;
pub const CF_CALLBACK_FETCH_DATA_FLAG_RECOVERY: CF_CALLBACK_FETCH_DATA_FLAGS = 1;
pub type CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS = u32;
pub const CF_CALLBACK_FETCH_PLACEHOLDERS_FLAG_NONE: CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS = 0;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CF_CALLBACK_INFO {
    pub StructSize: u32,
    pub ConnectionKey: CF_CONNECTION_KEY,
    pub CallbackContext: *mut core::ffi::c_void,
    pub VolumeGuidName: windows_core::PCWSTR,
    pub VolumeDosName: windows_core::PCWSTR,
    pub VolumeSerialNumber: u32,
    pub SyncRootFileId: i64,
    pub SyncRootIdentity: *const core::ffi::c_void,
    pub SyncRootIdentityLength: u32,
    pub FileId: i64,
    pub FileSize: i64,
    pub FileIdentity: *const core::ffi::c_void,
    pub FileIdentityLength: u32,
    pub NormalizedPath: windows_core::PCWSTR,
    pub TransferKey: CF_TRANSFER_KEY,
    pub PriorityHint: u8,
    pub CorrelationVector: super::PCORRELATION_VECTOR,
    pub ProcessInfo: *mut CF_PROCESS_INFO,
    pub RequestKey: CF_REQUEST_KEY,
}
#[cfg(feature = "winnt")]
impl Default for CF_CALLBACK_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CF_CALLBACK_OPEN_COMPLETION_FLAGS = u32;
pub const CF_CALLBACK_OPEN_COMPLETION_FLAG_NONE: CF_CALLBACK_OPEN_COMPLETION_FLAGS = 0;
pub const CF_CALLBACK_OPEN_COMPLETION_FLAG_PLACEHOLDER_UNKNOWN: CF_CALLBACK_OPEN_COMPLETION_FLAGS = 1;
pub const CF_CALLBACK_OPEN_COMPLETION_FLAG_PLACEHOLDER_UNSUPPORTED: CF_CALLBACK_OPEN_COMPLETION_FLAGS = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CF_CALLBACK_PARAMETERS {
    pub ParamSize: u32,
    pub Anonymous: CF_CALLBACK_PARAMETERS_0,
}
impl Default for CF_CALLBACK_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CF_CALLBACK_PARAMETERS_0 {
    pub Cancel: CF_CALLBACK_PARAMETERS_0_0,
    pub FetchData: CF_CALLBACK_PARAMETERS_0_1,
    pub ValidateData: CF_CALLBACK_PARAMETERS_0_2,
    pub FetchPlaceholders: CF_CALLBACK_PARAMETERS_0_3,
    pub OpenCompletion: CF_CALLBACK_PARAMETERS_0_4,
    pub CloseCompletion: CF_CALLBACK_PARAMETERS_0_5,
    pub Dehydrate: CF_CALLBACK_PARAMETERS_0_6,
    pub DehydrateCompletion: CF_CALLBACK_PARAMETERS_0_7,
    pub Delete: CF_CALLBACK_PARAMETERS_0_8,
    pub DeleteCompletion: CF_CALLBACK_PARAMETERS_0_9,
    pub Rename: CF_CALLBACK_PARAMETERS_0_10,
    pub RenameCompletion: CF_CALLBACK_PARAMETERS_0_11,
}
impl Default for CF_CALLBACK_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CF_CALLBACK_PARAMETERS_0_0 {
    pub Flags: CF_CALLBACK_CANCEL_FLAGS,
    pub Anonymous: CF_CALLBACK_PARAMETERS_0_0_0,
}
impl Default for CF_CALLBACK_PARAMETERS_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CF_CALLBACK_PARAMETERS_0_0_0 {
    pub FetchData: CF_CALLBACK_PARAMETERS_0_0_0_0,
}
impl Default for CF_CALLBACK_PARAMETERS_0_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_CALLBACK_PARAMETERS_0_0_0_0 {
    pub FileOffset: i64,
    pub Length: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_CALLBACK_PARAMETERS_0_1 {
    pub Flags: CF_CALLBACK_FETCH_DATA_FLAGS,
    pub RequiredFileOffset: i64,
    pub RequiredLength: i64,
    pub OptionalFileOffset: i64,
    pub OptionalLength: i64,
    pub LastDehydrationTime: i64,
    pub LastDehydrationReason: CF_CALLBACK_DEHYDRATION_REASON,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_CALLBACK_PARAMETERS_0_10 {
    pub Flags: CF_CALLBACK_RENAME_FLAGS,
    pub TargetPath: windows_core::PCWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_CALLBACK_PARAMETERS_0_11 {
    pub Flags: CF_CALLBACK_RENAME_COMPLETION_FLAGS,
    pub SourcePath: windows_core::PCWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_CALLBACK_PARAMETERS_0_2 {
    pub Flags: CF_CALLBACK_VALIDATE_DATA_FLAGS,
    pub RequiredFileOffset: i64,
    pub RequiredLength: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_CALLBACK_PARAMETERS_0_3 {
    pub Flags: CF_CALLBACK_FETCH_PLACEHOLDERS_FLAGS,
    pub Pattern: windows_core::PCWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_CALLBACK_PARAMETERS_0_4 {
    pub Flags: CF_CALLBACK_OPEN_COMPLETION_FLAGS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_CALLBACK_PARAMETERS_0_5 {
    pub Flags: CF_CALLBACK_CLOSE_COMPLETION_FLAGS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_CALLBACK_PARAMETERS_0_6 {
    pub Flags: CF_CALLBACK_DEHYDRATE_FLAGS,
    pub Reason: CF_CALLBACK_DEHYDRATION_REASON,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_CALLBACK_PARAMETERS_0_7 {
    pub Flags: CF_CALLBACK_DEHYDRATE_COMPLETION_FLAGS,
    pub Reason: CF_CALLBACK_DEHYDRATION_REASON,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_CALLBACK_PARAMETERS_0_8 {
    pub Flags: CF_CALLBACK_DELETE_FLAGS,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_CALLBACK_PARAMETERS_0_9 {
    pub Flags: CF_CALLBACK_DELETE_COMPLETION_FLAGS,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default)]
pub struct CF_CALLBACK_REGISTRATION {
    pub Type: CF_CALLBACK_TYPE,
    pub Callback: CF_CALLBACK,
}
pub type CF_CALLBACK_RENAME_COMPLETION_FLAGS = u32;
pub const CF_CALLBACK_RENAME_COMPLETION_FLAG_NONE: CF_CALLBACK_RENAME_COMPLETION_FLAGS = 0;
pub type CF_CALLBACK_RENAME_FLAGS = u32;
pub const CF_CALLBACK_RENAME_FLAG_IS_DIRECTORY: CF_CALLBACK_RENAME_FLAGS = 1;
pub const CF_CALLBACK_RENAME_FLAG_NONE: CF_CALLBACK_RENAME_FLAGS = 0;
pub const CF_CALLBACK_RENAME_FLAG_SOURCE_IN_SCOPE: CF_CALLBACK_RENAME_FLAGS = 2;
pub const CF_CALLBACK_RENAME_FLAG_TARGET_IN_SCOPE: CF_CALLBACK_RENAME_FLAGS = 4;
pub type CF_CALLBACK_TYPE = i32;
pub const CF_CALLBACK_TYPE_CANCEL_FETCH_DATA: CF_CALLBACK_TYPE = 2;
pub const CF_CALLBACK_TYPE_CANCEL_FETCH_PLACEHOLDERS: CF_CALLBACK_TYPE = 4;
pub const CF_CALLBACK_TYPE_FETCH_DATA: CF_CALLBACK_TYPE = 0;
pub const CF_CALLBACK_TYPE_FETCH_PLACEHOLDERS: CF_CALLBACK_TYPE = 3;
pub const CF_CALLBACK_TYPE_NONE: CF_CALLBACK_TYPE = -1;
pub const CF_CALLBACK_TYPE_NOTIFY_DEHYDRATE: CF_CALLBACK_TYPE = 7;
pub const CF_CALLBACK_TYPE_NOTIFY_DEHYDRATE_COMPLETION: CF_CALLBACK_TYPE = 8;
pub const CF_CALLBACK_TYPE_NOTIFY_DELETE: CF_CALLBACK_TYPE = 9;
pub const CF_CALLBACK_TYPE_NOTIFY_DELETE_COMPLETION: CF_CALLBACK_TYPE = 10;
pub const CF_CALLBACK_TYPE_NOTIFY_FILE_CLOSE_COMPLETION: CF_CALLBACK_TYPE = 6;
pub const CF_CALLBACK_TYPE_NOTIFY_FILE_OPEN_COMPLETION: CF_CALLBACK_TYPE = 5;
pub const CF_CALLBACK_TYPE_NOTIFY_RENAME: CF_CALLBACK_TYPE = 11;
pub const CF_CALLBACK_TYPE_NOTIFY_RENAME_COMPLETION: CF_CALLBACK_TYPE = 12;
pub const CF_CALLBACK_TYPE_VALIDATE_DATA: CF_CALLBACK_TYPE = 1;
pub type CF_CALLBACK_VALIDATE_DATA_FLAGS = u32;
pub const CF_CALLBACK_VALIDATE_DATA_FLAG_EXPLICIT_HYDRATION: CF_CALLBACK_VALIDATE_DATA_FLAGS = 2;
pub const CF_CALLBACK_VALIDATE_DATA_FLAG_NONE: CF_CALLBACK_VALIDATE_DATA_FLAGS = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_CONNECTION_KEY {
    pub Internal: i64,
}
pub type CF_CONNECT_FLAGS = u32;
pub const CF_CONNECT_FLAG_BLOCK_SELF_IMPLICIT_HYDRATION: CF_CONNECT_FLAGS = 8;
pub const CF_CONNECT_FLAG_NONE: CF_CONNECT_FLAGS = 0;
pub const CF_CONNECT_FLAG_REQUIRE_FULL_FILE_PATH: CF_CONNECT_FLAGS = 4;
pub const CF_CONNECT_FLAG_REQUIRE_PROCESS_INFO: CF_CONNECT_FLAGS = 2;
pub type CF_CONVERT_FLAGS = u32;
pub const CF_CONVERT_FLAG_ALWAYS_FULL: CF_CONVERT_FLAGS = 8;
pub const CF_CONVERT_FLAG_DEHYDRATE: CF_CONVERT_FLAGS = 2;
pub const CF_CONVERT_FLAG_ENABLE_ON_DEMAND_POPULATION: CF_CONVERT_FLAGS = 4;
pub const CF_CONVERT_FLAG_FORCE_CONVERT_TO_CLOUD_FILE: CF_CONVERT_FLAGS = 16;
pub const CF_CONVERT_FLAG_MARK_IN_SYNC: CF_CONVERT_FLAGS = 1;
pub const CF_CONVERT_FLAG_NONE: CF_CONVERT_FLAGS = 0;
pub type CF_CREATE_FLAGS = u32;
pub const CF_CREATE_FLAG_NONE: CF_CREATE_FLAGS = 0;
pub const CF_CREATE_FLAG_STOP_ON_ERROR: CF_CREATE_FLAGS = 1;
pub type CF_DEHYDRATE_FLAGS = u32;
pub const CF_DEHYDRATE_FLAG_BACKGROUND: CF_DEHYDRATE_FLAGS = 1;
pub const CF_DEHYDRATE_FLAG_NONE: CF_DEHYDRATE_FLAGS = 0;
pub const CF_EOF: i32 = -1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_FILE_RANGE {
    pub StartingOffset: i64,
    pub Length: i64,
}
#[repr(C)]
#[cfg(feature = "winbase")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_FS_METADATA {
    pub BasicInfo: super::FILE_BASIC_INFO,
    pub FileSize: i64,
}
pub type CF_HARDLINK_POLICY = u32;
pub const CF_HARDLINK_POLICY_ALLOWED: CF_HARDLINK_POLICY = 1;
pub const CF_HARDLINK_POLICY_NONE: CF_HARDLINK_POLICY = 0;
pub type CF_HYDRATE_FLAGS = u32;
pub const CF_HYDRATE_FLAG_NONE: CF_HYDRATE_FLAGS = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_HYDRATION_POLICY {
    pub Primary: CF_HYDRATION_POLICY_PRIMARY_USHORT,
    pub Modifier: CF_HYDRATION_POLICY_MODIFIER_USHORT,
}
pub const CF_HYDRATION_POLICY_ALWAYS_FULL: CF_HYDRATION_POLICY_PRIMARY = 3;
pub const CF_HYDRATION_POLICY_FULL: CF_HYDRATION_POLICY_PRIMARY = 2;
pub type CF_HYDRATION_POLICY_MODIFIER = u32;
pub const CF_HYDRATION_POLICY_MODIFIER_ALLOW_FULL_RESTART_HYDRATION: CF_HYDRATION_POLICY_MODIFIER = 8;
pub const CF_HYDRATION_POLICY_MODIFIER_AUTO_DEHYDRATION_ALLOWED: CF_HYDRATION_POLICY_MODIFIER = 4;
pub const CF_HYDRATION_POLICY_MODIFIER_NONE: CF_HYDRATION_POLICY_MODIFIER = 0;
pub const CF_HYDRATION_POLICY_MODIFIER_STREAMING_ALLOWED: CF_HYDRATION_POLICY_MODIFIER = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_HYDRATION_POLICY_MODIFIER_USHORT {
    pub us: u16,
}
pub const CF_HYDRATION_POLICY_MODIFIER_VALIDATION_REQUIRED: CF_HYDRATION_POLICY_MODIFIER = 1;
pub const CF_HYDRATION_POLICY_PARTIAL: CF_HYDRATION_POLICY_PRIMARY = 0;
pub type CF_HYDRATION_POLICY_PRIMARY = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_HYDRATION_POLICY_PRIMARY_USHORT {
    pub us: u16,
}
pub const CF_HYDRATION_POLICY_PROGRESSIVE: CF_HYDRATION_POLICY_PRIMARY = 1;
pub type CF_INSYNC_POLICY = u32;
pub const CF_INSYNC_POLICY_NONE: CF_INSYNC_POLICY = 0;
pub const CF_INSYNC_POLICY_PRESERVE_INSYNC_FOR_SYNC_ENGINE: CF_INSYNC_POLICY = 2147483648;
pub const CF_INSYNC_POLICY_TRACK_ALL: CF_INSYNC_POLICY = 16777215;
pub const CF_INSYNC_POLICY_TRACK_DIRECTORY_ALL: CF_INSYNC_POLICY = 11184880;
pub const CF_INSYNC_POLICY_TRACK_DIRECTORY_CREATION_TIME: CF_INSYNC_POLICY = 16;
pub const CF_INSYNC_POLICY_TRACK_DIRECTORY_HIDDEN_ATTRIBUTE: CF_INSYNC_POLICY = 64;
pub const CF_INSYNC_POLICY_TRACK_DIRECTORY_LAST_WRITE_TIME: CF_INSYNC_POLICY = 512;
pub const CF_INSYNC_POLICY_TRACK_DIRECTORY_READONLY_ATTRIBUTE: CF_INSYNC_POLICY = 32;
pub const CF_INSYNC_POLICY_TRACK_DIRECTORY_SYSTEM_ATTRIBUTE: CF_INSYNC_POLICY = 128;
pub const CF_INSYNC_POLICY_TRACK_FILE_ALL: CF_INSYNC_POLICY = 5592335;
pub const CF_INSYNC_POLICY_TRACK_FILE_CREATION_TIME: CF_INSYNC_POLICY = 1;
pub const CF_INSYNC_POLICY_TRACK_FILE_HIDDEN_ATTRIBUTE: CF_INSYNC_POLICY = 4;
pub const CF_INSYNC_POLICY_TRACK_FILE_LAST_WRITE_TIME: CF_INSYNC_POLICY = 256;
pub const CF_INSYNC_POLICY_TRACK_FILE_READONLY_ATTRIBUTE: CF_INSYNC_POLICY = 2;
pub const CF_INSYNC_POLICY_TRACK_FILE_SYSTEM_ATTRIBUTE: CF_INSYNC_POLICY = 8;
pub type CF_IN_SYNC_STATE = i32;
pub const CF_IN_SYNC_STATE_IN_SYNC: CF_IN_SYNC_STATE = 1;
pub const CF_IN_SYNC_STATE_NOT_IN_SYNC: CF_IN_SYNC_STATE = 0;
pub const CF_MAX_PRIORITY_HINT: u32 = 15;
pub const CF_MAX_PROVIDER_NAME_LENGTH: u32 = 255;
pub const CF_MAX_PROVIDER_VERSION_LENGTH: u32 = 255;
pub type CF_OPEN_FILE_FLAGS = u32;
pub const CF_OPEN_FILE_FLAG_DELETE_ACCESS: CF_OPEN_FILE_FLAGS = 4;
pub const CF_OPEN_FILE_FLAG_EXCLUSIVE: CF_OPEN_FILE_FLAGS = 1;
pub const CF_OPEN_FILE_FLAG_FOREGROUND: CF_OPEN_FILE_FLAGS = 8;
pub const CF_OPEN_FILE_FLAG_NONE: CF_OPEN_FILE_FLAGS = 0;
pub const CF_OPEN_FILE_FLAG_WRITE_ACCESS: CF_OPEN_FILE_FLAGS = 2;
pub type CF_OPERATION_ACK_DATA_FLAGS = u32;
pub const CF_OPERATION_ACK_DATA_FLAG_NONE: CF_OPERATION_ACK_DATA_FLAGS = 0;
pub type CF_OPERATION_ACK_DEHYDRATE_FLAGS = u32;
pub const CF_OPERATION_ACK_DEHYDRATE_FLAG_NONE: CF_OPERATION_ACK_DEHYDRATE_FLAGS = 0;
pub type CF_OPERATION_ACK_DELETE_FLAGS = u32;
pub const CF_OPERATION_ACK_DELETE_FLAG_NONE: CF_OPERATION_ACK_DELETE_FLAGS = 0;
pub type CF_OPERATION_ACK_RENAME_FLAGS = u32;
pub const CF_OPERATION_ACK_RENAME_FLAG_NONE: CF_OPERATION_ACK_RENAME_FLAGS = 0;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CF_OPERATION_INFO {
    pub StructSize: u32,
    pub Type: CF_OPERATION_TYPE,
    pub ConnectionKey: CF_CONNECTION_KEY,
    pub TransferKey: CF_TRANSFER_KEY,
    pub CorrelationVector: *const super::CORRELATION_VECTOR,
    pub SyncStatus: *const CF_SYNC_STATUS,
    pub RequestKey: CF_REQUEST_KEY,
}
#[cfg(feature = "winnt")]
impl Default for CF_OPERATION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "winbase", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct CF_OPERATION_PARAMETERS {
    pub ParamSize: u32,
    pub Anonymous: CF_OPERATION_PARAMETERS_0,
}
#[cfg(all(feature = "bcrypt", feature = "winbase", feature = "winnt"))]
impl Default for CF_OPERATION_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "winbase", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union CF_OPERATION_PARAMETERS_0 {
    pub TransferData: CF_OPERATION_PARAMETERS_0_0,
    pub RetrieveData: CF_OPERATION_PARAMETERS_0_1,
    pub AckData: CF_OPERATION_PARAMETERS_0_2,
    pub RestartHydration: CF_OPERATION_PARAMETERS_0_3,
    pub TransferPlaceholders: CF_OPERATION_PARAMETERS_0_4,
    pub AckDehydrate: CF_OPERATION_PARAMETERS_0_5,
    pub AckRename: CF_OPERATION_PARAMETERS_0_6,
    pub AckDelete: CF_OPERATION_PARAMETERS_0_7,
}
#[cfg(all(feature = "bcrypt", feature = "winbase", feature = "winnt"))]
impl Default for CF_OPERATION_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "winbase", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CF_OPERATION_PARAMETERS_0_0 {
    pub Flags: CF_OPERATION_TRANSFER_DATA_FLAGS,
    pub CompletionStatus: super::NTSTATUS,
    pub Buffer: *const core::ffi::c_void,
    pub Offset: i64,
    pub Length: i64,
}
#[cfg(all(feature = "bcrypt", feature = "winbase", feature = "winnt"))]
impl Default for CF_OPERATION_PARAMETERS_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "winbase", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CF_OPERATION_PARAMETERS_0_1 {
    pub Flags: CF_OPERATION_RETRIEVE_DATA_FLAGS,
    pub Buffer: *mut core::ffi::c_void,
    pub Offset: i64,
    pub Length: i64,
    pub ReturnedLength: i64,
}
#[cfg(all(feature = "bcrypt", feature = "winbase", feature = "winnt"))]
impl Default for CF_OPERATION_PARAMETERS_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "winbase", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_OPERATION_PARAMETERS_0_2 {
    pub Flags: CF_OPERATION_ACK_DATA_FLAGS,
    pub CompletionStatus: super::NTSTATUS,
    pub Offset: i64,
    pub Length: i64,
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "winbase", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CF_OPERATION_PARAMETERS_0_3 {
    pub Flags: CF_OPERATION_RESTART_HYDRATION_FLAGS,
    pub FsMetadata: *const CF_FS_METADATA,
    pub FileIdentity: *const core::ffi::c_void,
    pub FileIdentityLength: u32,
}
#[cfg(all(feature = "bcrypt", feature = "winbase", feature = "winnt"))]
impl Default for CF_OPERATION_PARAMETERS_0_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "winbase", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CF_OPERATION_PARAMETERS_0_4 {
    pub Flags: CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS,
    pub CompletionStatus: super::NTSTATUS,
    pub PlaceholderTotalCount: i64,
    pub PlaceholderArray: *mut CF_PLACEHOLDER_CREATE_INFO,
    pub PlaceholderCount: u32,
    pub EntriesProcessed: u32,
}
#[cfg(all(feature = "bcrypt", feature = "winbase", feature = "winnt"))]
impl Default for CF_OPERATION_PARAMETERS_0_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "winbase", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CF_OPERATION_PARAMETERS_0_5 {
    pub Flags: CF_OPERATION_ACK_DEHYDRATE_FLAGS,
    pub CompletionStatus: super::NTSTATUS,
    pub FileIdentity: *const core::ffi::c_void,
    pub FileIdentityLength: u32,
}
#[cfg(all(feature = "bcrypt", feature = "winbase", feature = "winnt"))]
impl Default for CF_OPERATION_PARAMETERS_0_5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "winbase", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_OPERATION_PARAMETERS_0_6 {
    pub Flags: CF_OPERATION_ACK_RENAME_FLAGS,
    pub CompletionStatus: super::NTSTATUS,
}
#[repr(C)]
#[cfg(all(feature = "bcrypt", feature = "winbase", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_OPERATION_PARAMETERS_0_7 {
    pub Flags: CF_OPERATION_ACK_DELETE_FLAGS,
    pub CompletionStatus: super::NTSTATUS,
}
pub type CF_OPERATION_RESTART_HYDRATION_FLAGS = u32;
pub const CF_OPERATION_RESTART_HYDRATION_FLAG_MARK_IN_SYNC: CF_OPERATION_RESTART_HYDRATION_FLAGS = 1;
pub const CF_OPERATION_RESTART_HYDRATION_FLAG_NONE: CF_OPERATION_RESTART_HYDRATION_FLAGS = 0;
pub type CF_OPERATION_RETRIEVE_DATA_FLAGS = u32;
pub const CF_OPERATION_RETRIEVE_DATA_FLAG_NONE: CF_OPERATION_RETRIEVE_DATA_FLAGS = 0;
pub type CF_OPERATION_TRANSFER_DATA_FLAGS = u32;
pub const CF_OPERATION_TRANSFER_DATA_FLAG_NONE: CF_OPERATION_TRANSFER_DATA_FLAGS = 0;
pub type CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS = u32;
pub const CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAG_DISABLE_ON_DEMAND_POPULATION: CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS = 2;
pub const CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAG_NONE: CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS = 0;
pub const CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAG_STOP_ON_ERROR: CF_OPERATION_TRANSFER_PLACEHOLDERS_FLAGS = 1;
pub type CF_OPERATION_TYPE = i32;
pub const CF_OPERATION_TYPE_ACK_DATA: CF_OPERATION_TYPE = 2;
pub const CF_OPERATION_TYPE_ACK_DEHYDRATE: CF_OPERATION_TYPE = 5;
pub const CF_OPERATION_TYPE_ACK_DELETE: CF_OPERATION_TYPE = 6;
pub const CF_OPERATION_TYPE_ACK_RENAME: CF_OPERATION_TYPE = 7;
pub const CF_OPERATION_TYPE_RESTART_HYDRATION: CF_OPERATION_TYPE = 3;
pub const CF_OPERATION_TYPE_RETRIEVE_DATA: CF_OPERATION_TYPE = 1;
pub const CF_OPERATION_TYPE_TRANSFER_DATA: CF_OPERATION_TYPE = 0;
pub const CF_OPERATION_TYPE_TRANSFER_PLACEHOLDERS: CF_OPERATION_TYPE = 4;
pub type CF_PIN_STATE = i32;
pub const CF_PIN_STATE_EXCLUDED: CF_PIN_STATE = 3;
pub const CF_PIN_STATE_INHERIT: CF_PIN_STATE = 4;
pub const CF_PIN_STATE_PINNED: CF_PIN_STATE = 1;
pub const CF_PIN_STATE_UNPINNED: CF_PIN_STATE = 2;
pub const CF_PIN_STATE_UNSPECIFIED: CF_PIN_STATE = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CF_PLACEHOLDER_BASIC_INFO {
    pub PinState: CF_PIN_STATE,
    pub InSyncState: CF_IN_SYNC_STATE,
    pub FileId: i64,
    pub SyncRootFileId: i64,
    pub FileIdentityLength: u32,
    pub FileIdentity: [u8; 1],
}
impl Default for CF_PLACEHOLDER_BASIC_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CF_PLACEHOLDER_CREATE_FLAGS = u32;
pub const CF_PLACEHOLDER_CREATE_FLAG_ALWAYS_FULL: CF_PLACEHOLDER_CREATE_FLAGS = 8;
pub const CF_PLACEHOLDER_CREATE_FLAG_DISABLE_ON_DEMAND_POPULATION: CF_PLACEHOLDER_CREATE_FLAGS = 1;
pub const CF_PLACEHOLDER_CREATE_FLAG_MARK_IN_SYNC: CF_PLACEHOLDER_CREATE_FLAGS = 2;
pub const CF_PLACEHOLDER_CREATE_FLAG_NONE: CF_PLACEHOLDER_CREATE_FLAGS = 0;
pub const CF_PLACEHOLDER_CREATE_FLAG_SUPERSEDE: CF_PLACEHOLDER_CREATE_FLAGS = 4;
#[repr(C)]
#[cfg(all(feature = "winbase", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CF_PLACEHOLDER_CREATE_INFO {
    pub RelativeFileName: windows_core::PCWSTR,
    pub FsMetadata: CF_FS_METADATA,
    pub FileIdentity: *const core::ffi::c_void,
    pub FileIdentityLength: u32,
    pub Flags: CF_PLACEHOLDER_CREATE_FLAGS,
    pub Result: windows_core::HRESULT,
    pub CreateUsn: super::USN,
}
#[cfg(all(feature = "winbase", feature = "winnt"))]
impl Default for CF_PLACEHOLDER_CREATE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CF_PLACEHOLDER_INFO_BASIC: CF_PLACEHOLDER_INFO_CLASS = 0;
pub type CF_PLACEHOLDER_INFO_CLASS = i32;
pub const CF_PLACEHOLDER_INFO_STANDARD: CF_PLACEHOLDER_INFO_CLASS = 1;
pub type CF_PLACEHOLDER_MANAGEMENT_POLICY = i32;
pub const CF_PLACEHOLDER_MANAGEMENT_POLICY_CONVERT_TO_UNRESTRICTED: CF_PLACEHOLDER_MANAGEMENT_POLICY = 2;
pub const CF_PLACEHOLDER_MANAGEMENT_POLICY_CREATE_UNRESTRICTED: CF_PLACEHOLDER_MANAGEMENT_POLICY = 1;
pub const CF_PLACEHOLDER_MANAGEMENT_POLICY_DEFAULT: CF_PLACEHOLDER_MANAGEMENT_POLICY = 0;
pub const CF_PLACEHOLDER_MANAGEMENT_POLICY_UPDATE_UNRESTRICTED: CF_PLACEHOLDER_MANAGEMENT_POLICY = 4;
pub const CF_PLACEHOLDER_MAX_FILE_IDENTITY_LENGTH: u32 = 4096;
pub type CF_PLACEHOLDER_RANGE_INFO_CLASS = i32;
pub const CF_PLACEHOLDER_RANGE_INFO_MODIFIED: CF_PLACEHOLDER_RANGE_INFO_CLASS = 3;
pub const CF_PLACEHOLDER_RANGE_INFO_ONDISK: CF_PLACEHOLDER_RANGE_INFO_CLASS = 1;
pub const CF_PLACEHOLDER_RANGE_INFO_VALIDATED: CF_PLACEHOLDER_RANGE_INFO_CLASS = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CF_PLACEHOLDER_STANDARD_INFO {
    pub OnDiskDataSize: i64,
    pub ValidatedDataSize: i64,
    pub ModifiedDataSize: i64,
    pub PropertiesSize: i64,
    pub PinState: CF_PIN_STATE,
    pub InSyncState: CF_IN_SYNC_STATE,
    pub FileId: i64,
    pub SyncRootFileId: i64,
    pub FileIdentityLength: u32,
    pub FileIdentity: [u8; 1],
}
impl Default for CF_PLACEHOLDER_STANDARD_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CF_PLACEHOLDER_STATE = u32;
pub const CF_PLACEHOLDER_STATE_ESSENTIAL_PROP_PRESENT: CF_PLACEHOLDER_STATE = 4;
pub const CF_PLACEHOLDER_STATE_INVALID: CF_PLACEHOLDER_STATE = 4294967295;
pub const CF_PLACEHOLDER_STATE_IN_SYNC: CF_PLACEHOLDER_STATE = 8;
pub const CF_PLACEHOLDER_STATE_NO_STATES: CF_PLACEHOLDER_STATE = 0;
pub const CF_PLACEHOLDER_STATE_PARTIAL: CF_PLACEHOLDER_STATE = 16;
pub const CF_PLACEHOLDER_STATE_PARTIALLY_ON_DISK: CF_PLACEHOLDER_STATE = 32;
pub const CF_PLACEHOLDER_STATE_PLACEHOLDER: CF_PLACEHOLDER_STATE = 1;
pub const CF_PLACEHOLDER_STATE_SYNC_ROOT: CF_PLACEHOLDER_STATE = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_PLATFORM_INFO {
    pub BuildNumber: u32,
    pub RevisionNumber: u32,
    pub IntegrationNumber: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_POPULATION_POLICY {
    pub Primary: CF_POPULATION_POLICY_PRIMARY_USHORT,
    pub Modifier: CF_POPULATION_POLICY_MODIFIER_USHORT,
}
pub const CF_POPULATION_POLICY_ALWAYS_FULL: CF_POPULATION_POLICY_PRIMARY = 3;
pub const CF_POPULATION_POLICY_FULL: CF_POPULATION_POLICY_PRIMARY = 2;
pub type CF_POPULATION_POLICY_MODIFIER = u32;
pub const CF_POPULATION_POLICY_MODIFIER_NONE: CF_POPULATION_POLICY_MODIFIER = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_POPULATION_POLICY_MODIFIER_USHORT {
    pub us: u16,
}
pub const CF_POPULATION_POLICY_PARTIAL: CF_POPULATION_POLICY_PRIMARY = 0;
pub type CF_POPULATION_POLICY_PRIMARY = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_POPULATION_POLICY_PRIMARY_USHORT {
    pub us: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_PROCESS_INFO {
    pub StructSize: u32,
    pub ProcessId: u32,
    pub ImagePath: windows_core::PCWSTR,
    pub PackageName: windows_core::PCWSTR,
    pub ApplicationId: windows_core::PCWSTR,
    pub CommandLine: windows_core::PCWSTR,
    pub SessionId: u32,
}
pub const CF_PROVIDER_STATUS_CLEAR_FLAGS: CF_SYNC_PROVIDER_STATUS = 2147483648;
pub const CF_PROVIDER_STATUS_CONNECTIVITY_LOST: CF_SYNC_PROVIDER_STATUS = 64;
pub const CF_PROVIDER_STATUS_DISCONNECTED: CF_SYNC_PROVIDER_STATUS = 0;
pub const CF_PROVIDER_STATUS_ERROR: CF_SYNC_PROVIDER_STATUS = 3221225474;
pub const CF_PROVIDER_STATUS_IDLE: CF_SYNC_PROVIDER_STATUS = 1;
pub const CF_PROVIDER_STATUS_POPULATE_CONTENT: CF_SYNC_PROVIDER_STATUS = 8;
pub const CF_PROVIDER_STATUS_POPULATE_METADATA: CF_SYNC_PROVIDER_STATUS = 4;
pub const CF_PROVIDER_STATUS_POPULATE_NAMESPACE: CF_SYNC_PROVIDER_STATUS = 2;
pub const CF_PROVIDER_STATUS_SYNC_FULL: CF_SYNC_PROVIDER_STATUS = 32;
pub const CF_PROVIDER_STATUS_SYNC_INCREMENTAL: CF_SYNC_PROVIDER_STATUS = 16;
pub const CF_PROVIDER_STATUS_TERMINATED: CF_SYNC_PROVIDER_STATUS = 3221225473;
pub type CF_REGISTER_FLAGS = u32;
pub const CF_REGISTER_FLAG_DISABLE_ON_DEMAND_POPULATION_ON_ROOT: CF_REGISTER_FLAGS = 2;
pub const CF_REGISTER_FLAG_MARK_IN_SYNC_ON_ROOT: CF_REGISTER_FLAGS = 4;
pub const CF_REGISTER_FLAG_NONE: CF_REGISTER_FLAGS = 0;
pub const CF_REGISTER_FLAG_UPDATE: CF_REGISTER_FLAGS = 1;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct CF_REQUEST_KEY(pub i64);
pub const CF_REQUEST_KEY_DEFAULT: u32 = 0;
pub type CF_REVERT_FLAGS = u32;
pub const CF_REVERT_FLAG_NONE: CF_REVERT_FLAGS = 0;
pub type CF_SET_IN_SYNC_FLAGS = u32;
pub const CF_SET_IN_SYNC_FLAG_NONE: CF_SET_IN_SYNC_FLAGS = 0;
pub type CF_SET_PIN_FLAGS = u32;
pub const CF_SET_PIN_FLAG_NONE: CF_SET_PIN_FLAGS = 0;
pub const CF_SET_PIN_FLAG_RECURSE: CF_SET_PIN_FLAGS = 1;
pub const CF_SET_PIN_FLAG_RECURSE_ONLY: CF_SET_PIN_FLAGS = 2;
pub const CF_SET_PIN_FLAG_RECURSE_STOP_ON_ERROR: CF_SET_PIN_FLAGS = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_SYNC_POLICIES {
    pub StructSize: u32,
    pub Hydration: CF_HYDRATION_POLICY,
    pub Population: CF_POPULATION_POLICY,
    pub InSync: CF_INSYNC_POLICY,
    pub HardLink: CF_HARDLINK_POLICY,
    pub PlaceholderManagement: CF_PLACEHOLDER_MANAGEMENT_POLICY,
}
pub type CF_SYNC_PROVIDER_STATUS = u32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CF_SYNC_REGISTRATION {
    pub StructSize: u32,
    pub ProviderName: windows_core::PCWSTR,
    pub ProviderVersion: windows_core::PCWSTR,
    pub SyncRootIdentity: *const core::ffi::c_void,
    pub SyncRootIdentityLength: u32,
    pub FileIdentity: *const core::ffi::c_void,
    pub FileIdentityLength: u32,
    pub ProviderId: windows_core::GUID,
}
impl Default for CF_SYNC_REGISTRATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_SYNC_ROOT_BASIC_INFO {
    pub SyncRootFileId: i64,
}
pub const CF_SYNC_ROOT_INFO_BASIC: CF_SYNC_ROOT_INFO_CLASS = 0;
pub type CF_SYNC_ROOT_INFO_CLASS = i32;
pub const CF_SYNC_ROOT_INFO_PROVIDER: CF_SYNC_ROOT_INFO_CLASS = 2;
pub const CF_SYNC_ROOT_INFO_STANDARD: CF_SYNC_ROOT_INFO_CLASS = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CF_SYNC_ROOT_PROVIDER_INFO {
    pub ProviderStatus: CF_SYNC_PROVIDER_STATUS,
    pub ProviderName: [u16; 256],
    pub ProviderVersion: [u16; 256],
}
impl Default for CF_SYNC_ROOT_PROVIDER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CF_SYNC_ROOT_STANDARD_INFO {
    pub SyncRootFileId: i64,
    pub HydrationPolicy: CF_HYDRATION_POLICY,
    pub PopulationPolicy: CF_POPULATION_POLICY,
    pub InSyncPolicy: CF_INSYNC_POLICY,
    pub HardLinkPolicy: CF_HARDLINK_POLICY,
    pub ProviderStatus: CF_SYNC_PROVIDER_STATUS,
    pub ProviderName: [u16; 256],
    pub ProviderVersion: [u16; 256],
    pub SyncRootIdentityLength: u32,
    pub SyncRootIdentity: [u8; 1],
}
impl Default for CF_SYNC_ROOT_STANDARD_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CF_SYNC_STATUS {
    pub StructSize: u32,
    pub Code: u32,
    pub DescriptionOffset: u32,
    pub DescriptionLength: u32,
    pub DeviceIdOffset: u32,
    pub DeviceIdLength: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct CF_TRANSFER_KEY(pub i64);
pub type CF_UPDATE_FLAGS = u32;
pub const CF_UPDATE_FLAG_ALLOW_PARTIAL: CF_UPDATE_FLAGS = 1024;
pub const CF_UPDATE_FLAG_ALWAYS_FULL: CF_UPDATE_FLAGS = 512;
pub const CF_UPDATE_FLAG_CLEAR_IN_SYNC: CF_UPDATE_FLAGS = 64;
pub const CF_UPDATE_FLAG_DEHYDRATE: CF_UPDATE_FLAGS = 4;
pub const CF_UPDATE_FLAG_DISABLE_ON_DEMAND_POPULATION: CF_UPDATE_FLAGS = 16;
pub const CF_UPDATE_FLAG_ENABLE_ON_DEMAND_POPULATION: CF_UPDATE_FLAGS = 8;
pub const CF_UPDATE_FLAG_MARK_IN_SYNC: CF_UPDATE_FLAGS = 2;
pub const CF_UPDATE_FLAG_NONE: CF_UPDATE_FLAGS = 0;
pub const CF_UPDATE_FLAG_PASSTHROUGH_FS_METADATA: CF_UPDATE_FLAGS = 256;
pub const CF_UPDATE_FLAG_REMOVE_FILE_IDENTITY: CF_UPDATE_FLAGS = 32;
pub const CF_UPDATE_FLAG_REMOVE_PROPERTY: CF_UPDATE_FLAGS = 128;
pub const CF_UPDATE_FLAG_VERIFY_IN_SYNC: CF_UPDATE_FLAGS = 1;
pub type PCF_CONNECTION_KEY = *mut CF_CONNECTION_KEY;
