#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
windows_link::link!("rpcns4.dll" "system" fn RpcNsBindingExportA(entrynamesyntax : u32, entryname : *const u8, ifspec : super::rpcdce::RPC_IF_HANDLE, bindingvec : *const super::rpcdce::RPC_BINDING_VECTOR, objectuuidvec : *const super::rpcdce::UUID_VECTOR) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
windows_link::link!("rpcns4.dll" "system" fn RpcNsBindingExportPnPA(entrynamesyntax : u32, entryname : *const u8, ifspec : super::rpcdce::RPC_IF_HANDLE, objectvector : *const super::rpcdce::UUID_VECTOR) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
windows_link::link!("rpcns4.dll" "system" fn RpcNsBindingExportPnPW(entrynamesyntax : u32, entryname : *const u16, ifspec : super::rpcdce::RPC_IF_HANDLE, objectvector : *const super::rpcdce::UUID_VECTOR) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
windows_link::link!("rpcns4.dll" "system" fn RpcNsBindingExportW(entrynamesyntax : u32, entryname : *const u16, ifspec : super::rpcdce::RPC_IF_HANDLE, bindingvec : *const super::rpcdce::RPC_BINDING_VECTOR, objectuuidvec : *const super::rpcdce::UUID_VECTOR) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
windows_link::link!("rpcns4.dll" "system" fn RpcNsBindingImportBeginA(entrynamesyntax : u32, entryname : *const u8, ifspec : super::rpcdce::RPC_IF_HANDLE, objuuid : *const windows_sys::core::GUID, importcontext : *mut RPC_NS_HANDLE) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
windows_link::link!("rpcns4.dll" "system" fn RpcNsBindingImportBeginW(entrynamesyntax : u32, entryname : *const u16, ifspec : super::rpcdce::RPC_IF_HANDLE, objuuid : *const windows_sys::core::GUID, importcontext : *mut RPC_NS_HANDLE) -> super::rpc::RPC_STATUS);
#[cfg(feature = "Win32_rpc")]
windows_link::link!("rpcns4.dll" "system" fn RpcNsBindingImportDone(importcontext : *mut RPC_NS_HANDLE) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
windows_link::link!("rpcns4.dll" "system" fn RpcNsBindingImportNext(importcontext : RPC_NS_HANDLE, binding : *mut super::rpcdce::RPC_BINDING_HANDLE) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
windows_link::link!("rpcns4.dll" "system" fn RpcNsBindingLookupBeginA(entrynamesyntax : u32, entryname : *const u8, ifspec : super::rpcdce::RPC_IF_HANDLE, objuuid : *const windows_sys::core::GUID, bindingmaxcount : u32, lookupcontext : *mut RPC_NS_HANDLE) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
windows_link::link!("rpcns4.dll" "system" fn RpcNsBindingLookupBeginW(entrynamesyntax : u32, entryname : *const u16, ifspec : super::rpcdce::RPC_IF_HANDLE, objuuid : *const windows_sys::core::GUID, bindingmaxcount : u32, lookupcontext : *mut RPC_NS_HANDLE) -> super::rpc::RPC_STATUS);
#[cfg(feature = "Win32_rpc")]
windows_link::link!("rpcns4.dll" "system" fn RpcNsBindingLookupDone(lookupcontext : *mut RPC_NS_HANDLE) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
windows_link::link!("rpcns4.dll" "system" fn RpcNsBindingLookupNext(lookupcontext : RPC_NS_HANDLE, bindingvec : *mut *mut super::rpcdce::RPC_BINDING_VECTOR) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
windows_link::link!("rpcns4.dll" "system" fn RpcNsBindingSelect(bindingvec : *mut super::rpcdce::RPC_BINDING_VECTOR, binding : *mut super::rpcdce::RPC_BINDING_HANDLE) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
windows_link::link!("rpcns4.dll" "system" fn RpcNsBindingUnexportA(entrynamesyntax : u32, entryname : *const u8, ifspec : super::rpcdce::RPC_IF_HANDLE, objectuuidvec : *const super::rpcdce::UUID_VECTOR) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
windows_link::link!("rpcns4.dll" "system" fn RpcNsBindingUnexportPnPA(entrynamesyntax : u32, entryname : *const u8, ifspec : super::rpcdce::RPC_IF_HANDLE, objectvector : *const super::rpcdce::UUID_VECTOR) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
windows_link::link!("rpcns4.dll" "system" fn RpcNsBindingUnexportPnPW(entrynamesyntax : u32, entryname : *const u16, ifspec : super::rpcdce::RPC_IF_HANDLE, objectvector : *const super::rpcdce::UUID_VECTOR) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
windows_link::link!("rpcns4.dll" "system" fn RpcNsBindingUnexportW(entrynamesyntax : u32, entryname : *const u16, ifspec : super::rpcdce::RPC_IF_HANDLE, objectuuidvec : *const super::rpcdce::UUID_VECTOR) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
windows_link::link!("rpcns4.dll" "system" fn RpcNsEntryExpandNameA(entrynamesyntax : u32, entryname : *const u8, expandedname : *mut super::rpcdce::RPC_CSTR) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
windows_link::link!("rpcns4.dll" "system" fn RpcNsEntryExpandNameW(entrynamesyntax : u32, entryname : *const u16, expandedname : *mut super::rpcdce::RPC_WSTR) -> super::rpc::RPC_STATUS);
#[cfg(feature = "Win32_rpc")]
windows_link::link!("rpcns4.dll" "system" fn RpcNsEntryObjectInqBeginA(entrynamesyntax : u32, entryname : *const u8, inquirycontext : *mut RPC_NS_HANDLE) -> super::rpc::RPC_STATUS);
#[cfg(feature = "Win32_rpc")]
windows_link::link!("rpcns4.dll" "system" fn RpcNsEntryObjectInqBeginW(entrynamesyntax : u32, entryname : *const u16, inquirycontext : *mut RPC_NS_HANDLE) -> super::rpc::RPC_STATUS);
#[cfg(feature = "Win32_rpc")]
windows_link::link!("rpcns4.dll" "system" fn RpcNsEntryObjectInqDone(inquirycontext : *mut RPC_NS_HANDLE) -> super::rpc::RPC_STATUS);
#[cfg(feature = "Win32_rpc")]
windows_link::link!("rpcns4.dll" "system" fn RpcNsEntryObjectInqNext(inquirycontext : RPC_NS_HANDLE, objuuid : *mut windows_sys::core::GUID) -> super::rpc::RPC_STATUS);
#[cfg(feature = "Win32_rpc")]
windows_link::link!("rpcns4.dll" "system" fn RpcNsGroupDeleteA(groupnamesyntax : u32, groupname : *const u8) -> super::rpc::RPC_STATUS);
#[cfg(feature = "Win32_rpc")]
windows_link::link!("rpcns4.dll" "system" fn RpcNsGroupDeleteW(groupnamesyntax : u32, groupname : *const u16) -> super::rpc::RPC_STATUS);
#[cfg(feature = "Win32_rpc")]
windows_link::link!("rpcns4.dll" "system" fn RpcNsGroupMbrAddA(groupnamesyntax : u32, groupname : *const u8, membernamesyntax : u32, membername : *const u8) -> super::rpc::RPC_STATUS);
#[cfg(feature = "Win32_rpc")]
windows_link::link!("rpcns4.dll" "system" fn RpcNsGroupMbrAddW(groupnamesyntax : u32, groupname : *const u16, membernamesyntax : u32, membername : *const u16) -> super::rpc::RPC_STATUS);
#[cfg(feature = "Win32_rpc")]
windows_link::link!("rpcns4.dll" "system" fn RpcNsGroupMbrInqBeginA(groupnamesyntax : u32, groupname : *const u8, membernamesyntax : u32, inquirycontext : *mut RPC_NS_HANDLE) -> super::rpc::RPC_STATUS);
#[cfg(feature = "Win32_rpc")]
windows_link::link!("rpcns4.dll" "system" fn RpcNsGroupMbrInqBeginW(groupnamesyntax : u32, groupname : *const u16, membernamesyntax : u32, inquirycontext : *mut RPC_NS_HANDLE) -> super::rpc::RPC_STATUS);
#[cfg(feature = "Win32_rpc")]
windows_link::link!("rpcns4.dll" "system" fn RpcNsGroupMbrInqDone(inquirycontext : *mut RPC_NS_HANDLE) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
windows_link::link!("rpcns4.dll" "system" fn RpcNsGroupMbrInqNextA(inquirycontext : RPC_NS_HANDLE, membername : *mut super::rpcdce::RPC_CSTR) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
windows_link::link!("rpcns4.dll" "system" fn RpcNsGroupMbrInqNextW(inquirycontext : RPC_NS_HANDLE, membername : *mut super::rpcdce::RPC_WSTR) -> super::rpc::RPC_STATUS);
#[cfg(feature = "Win32_rpc")]
windows_link::link!("rpcns4.dll" "system" fn RpcNsGroupMbrRemoveA(groupnamesyntax : u32, groupname : *const u8, membernamesyntax : u32, membername : *const u8) -> super::rpc::RPC_STATUS);
#[cfg(feature = "Win32_rpc")]
windows_link::link!("rpcns4.dll" "system" fn RpcNsGroupMbrRemoveW(groupnamesyntax : u32, groupname : *const u16, membernamesyntax : u32, membername : *const u16) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
windows_link::link!("rpcns4.dll" "system" fn RpcNsMgmtBindingUnexportA(entrynamesyntax : u32, entryname : *const u8, ifid : *const super::rpcdce::RPC_IF_ID, versoption : u32, objectuuidvec : *const super::rpcdce::UUID_VECTOR) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
windows_link::link!("rpcns4.dll" "system" fn RpcNsMgmtBindingUnexportW(entrynamesyntax : u32, entryname : *const u16, ifid : *const super::rpcdce::RPC_IF_ID, versoption : u32, objectuuidvec : *const super::rpcdce::UUID_VECTOR) -> super::rpc::RPC_STATUS);
#[cfg(feature = "Win32_rpc")]
windows_link::link!("rpcns4.dll" "system" fn RpcNsMgmtEntryCreateA(entrynamesyntax : u32, entryname : *const u8) -> super::rpc::RPC_STATUS);
#[cfg(feature = "Win32_rpc")]
windows_link::link!("rpcns4.dll" "system" fn RpcNsMgmtEntryCreateW(entrynamesyntax : u32, entryname : *const u16) -> super::rpc::RPC_STATUS);
#[cfg(feature = "Win32_rpc")]
windows_link::link!("rpcns4.dll" "system" fn RpcNsMgmtEntryDeleteA(entrynamesyntax : u32, entryname : *const u8) -> super::rpc::RPC_STATUS);
#[cfg(feature = "Win32_rpc")]
windows_link::link!("rpcns4.dll" "system" fn RpcNsMgmtEntryDeleteW(entrynamesyntax : u32, entryname : *const u16) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
windows_link::link!("rpcns4.dll" "system" fn RpcNsMgmtEntryInqIfIdsA(entrynamesyntax : u32, entryname : *const u8, ifidvec : *mut *mut super::rpcdce::RPC_IF_ID_VECTOR) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
windows_link::link!("rpcns4.dll" "system" fn RpcNsMgmtEntryInqIfIdsW(entrynamesyntax : u32, entryname : *const u16, ifidvec : *mut *mut super::rpcdce::RPC_IF_ID_VECTOR) -> super::rpc::RPC_STATUS);
#[cfg(feature = "Win32_rpc")]
windows_link::link!("rpcns4.dll" "system" fn RpcNsMgmtHandleSetExpAge(nshandle : RPC_NS_HANDLE, expirationage : u32) -> super::rpc::RPC_STATUS);
#[cfg(feature = "Win32_rpc")]
windows_link::link!("rpcns4.dll" "system" fn RpcNsMgmtInqExpAge(expirationage : *mut u32) -> super::rpc::RPC_STATUS);
#[cfg(feature = "Win32_rpc")]
windows_link::link!("rpcns4.dll" "system" fn RpcNsMgmtSetExpAge(expirationage : u32) -> super::rpc::RPC_STATUS);
#[cfg(feature = "Win32_rpc")]
windows_link::link!("rpcns4.dll" "system" fn RpcNsProfileDeleteA(profilenamesyntax : u32, profilename : *const u8) -> super::rpc::RPC_STATUS);
#[cfg(feature = "Win32_rpc")]
windows_link::link!("rpcns4.dll" "system" fn RpcNsProfileDeleteW(profilenamesyntax : u32, profilename : *const u16) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
windows_link::link!("rpcns4.dll" "system" fn RpcNsProfileEltAddA(profilenamesyntax : u32, profilename : *const u8, ifid : *const super::rpcdce::RPC_IF_ID, membernamesyntax : u32, membername : *const u8, priority : u32, annotation : *const u8) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
windows_link::link!("rpcns4.dll" "system" fn RpcNsProfileEltAddW(profilenamesyntax : u32, profilename : *const u16, ifid : *const super::rpcdce::RPC_IF_ID, membernamesyntax : u32, membername : *const u16, priority : u32, annotation : *const u16) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
windows_link::link!("rpcns4.dll" "system" fn RpcNsProfileEltInqBeginA(profilenamesyntax : u32, profilename : *const u8, inquirytype : u32, ifid : *const super::rpcdce::RPC_IF_ID, versoption : u32, membernamesyntax : u32, membername : *const u8, inquirycontext : *mut RPC_NS_HANDLE) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
windows_link::link!("rpcns4.dll" "system" fn RpcNsProfileEltInqBeginW(profilenamesyntax : u32, profilename : *const u16, inquirytype : u32, ifid : *const super::rpcdce::RPC_IF_ID, versoption : u32, membernamesyntax : u32, membername : *const u16, inquirycontext : *mut RPC_NS_HANDLE) -> super::rpc::RPC_STATUS);
#[cfg(feature = "Win32_rpc")]
windows_link::link!("rpcns4.dll" "system" fn RpcNsProfileEltInqDone(inquirycontext : *mut RPC_NS_HANDLE) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
windows_link::link!("rpcns4.dll" "system" fn RpcNsProfileEltInqNextA(inquirycontext : RPC_NS_HANDLE, ifid : *mut super::rpcdce::RPC_IF_ID, membername : *mut super::rpcdce::RPC_CSTR, priority : *mut u32, annotation : *mut super::rpcdce::RPC_CSTR) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
windows_link::link!("rpcns4.dll" "system" fn RpcNsProfileEltInqNextW(inquirycontext : RPC_NS_HANDLE, ifid : *mut super::rpcdce::RPC_IF_ID, membername : *mut super::rpcdce::RPC_WSTR, priority : *mut u32, annotation : *mut super::rpcdce::RPC_WSTR) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
windows_link::link!("rpcns4.dll" "system" fn RpcNsProfileEltRemoveA(profilenamesyntax : u32, profilename : *const u8, ifid : *const super::rpcdce::RPC_IF_ID, membernamesyntax : u32, membername : *const u8) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
windows_link::link!("rpcns4.dll" "system" fn RpcNsProfileEltRemoveW(profilenamesyntax : u32, profilename : *const u16, ifid : *const super::rpcdce::RPC_IF_ID, membernamesyntax : u32, membername : *const u16) -> super::rpc::RPC_STATUS);
pub const RPC_C_NS_DEFAULT_EXP_AGE: i32 = -1;
pub const RPC_C_NS_SYNTAX_DCE: u32 = 3;
pub const RPC_C_NS_SYNTAX_DEFAULT: u32 = 0;
pub const RPC_C_PROFILE_ALL_ELT: u32 = 1;
pub const RPC_C_PROFILE_ALL_ELTS: u32 = 1;
pub const RPC_C_PROFILE_DEFAULT_ELT: u32 = 0;
pub const RPC_C_PROFILE_MATCH_BY_BOTH: u32 = 4;
pub const RPC_C_PROFILE_MATCH_BY_IF: u32 = 2;
pub const RPC_C_PROFILE_MATCH_BY_MBR: u32 = 3;
pub type RPC_NS_HANDLE = *mut core::ffi::c_void;
