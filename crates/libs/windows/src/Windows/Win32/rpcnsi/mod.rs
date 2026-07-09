#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn RpcNsBindingExportA(entrynamesyntax: u32, entryname: Option<*const u8>, ifspec: Option<super::rpcdce::RPC_IF_HANDLE>, bindingvec: Option<*const super::rpcdce::RPC_BINDING_VECTOR>, objectuuidvec: Option<*const super::rpcdce::UUID_VECTOR>) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingExportA(entrynamesyntax : u32, entryname : *const u8, ifspec : super::rpcdce::RPC_IF_HANDLE, bindingvec : *const super::rpcdce::RPC_BINDING_VECTOR, objectuuidvec : *const super::rpcdce::UUID_VECTOR) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsBindingExportA(entrynamesyntax, entryname.unwrap_or(core::mem::zeroed()) as _, ifspec.unwrap_or(core::mem::zeroed()) as _, bindingvec.unwrap_or(core::mem::zeroed()) as _, objectuuidvec.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn RpcNsBindingExportPnPA(entrynamesyntax: u32, entryname: Option<*const u8>, ifspec: Option<super::rpcdce::RPC_IF_HANDLE>, objectvector: Option<*const super::rpcdce::UUID_VECTOR>) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingExportPnPA(entrynamesyntax : u32, entryname : *const u8, ifspec : super::rpcdce::RPC_IF_HANDLE, objectvector : *const super::rpcdce::UUID_VECTOR) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsBindingExportPnPA(entrynamesyntax, entryname.unwrap_or(core::mem::zeroed()) as _, ifspec.unwrap_or(core::mem::zeroed()) as _, objectvector.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn RpcNsBindingExportPnPW(entrynamesyntax: u32, entryname: Option<*const u16>, ifspec: Option<super::rpcdce::RPC_IF_HANDLE>, objectvector: Option<*const super::rpcdce::UUID_VECTOR>) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingExportPnPW(entrynamesyntax : u32, entryname : *const u16, ifspec : super::rpcdce::RPC_IF_HANDLE, objectvector : *const super::rpcdce::UUID_VECTOR) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsBindingExportPnPW(entrynamesyntax, entryname.unwrap_or(core::mem::zeroed()) as _, ifspec.unwrap_or(core::mem::zeroed()) as _, objectvector.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn RpcNsBindingExportW(entrynamesyntax: u32, entryname: Option<*const u16>, ifspec: Option<super::rpcdce::RPC_IF_HANDLE>, bindingvec: Option<*const super::rpcdce::RPC_BINDING_VECTOR>, objectuuidvec: Option<*const super::rpcdce::UUID_VECTOR>) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingExportW(entrynamesyntax : u32, entryname : *const u16, ifspec : super::rpcdce::RPC_IF_HANDLE, bindingvec : *const super::rpcdce::RPC_BINDING_VECTOR, objectuuidvec : *const super::rpcdce::UUID_VECTOR) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsBindingExportW(entrynamesyntax, entryname.unwrap_or(core::mem::zeroed()) as _, ifspec.unwrap_or(core::mem::zeroed()) as _, bindingvec.unwrap_or(core::mem::zeroed()) as _, objectuuidvec.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn RpcNsBindingImportBeginA(entrynamesyntax: u32, entryname: Option<*const u8>, ifspec: Option<super::rpcdce::RPC_IF_HANDLE>, objuuid: Option<*const windows_core::GUID>, importcontext: *mut RPC_NS_HANDLE) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingImportBeginA(entrynamesyntax : u32, entryname : *const u8, ifspec : super::rpcdce::RPC_IF_HANDLE, objuuid : *const windows_core::GUID, importcontext : *mut RPC_NS_HANDLE) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsBindingImportBeginA(entrynamesyntax, entryname.unwrap_or(core::mem::zeroed()) as _, ifspec.unwrap_or(core::mem::zeroed()) as _, objuuid.unwrap_or(core::mem::zeroed()) as _, importcontext as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn RpcNsBindingImportBeginW(entrynamesyntax: u32, entryname: Option<*const u16>, ifspec: Option<super::rpcdce::RPC_IF_HANDLE>, objuuid: Option<*const windows_core::GUID>, importcontext: *mut RPC_NS_HANDLE) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingImportBeginW(entrynamesyntax : u32, entryname : *const u16, ifspec : super::rpcdce::RPC_IF_HANDLE, objuuid : *const windows_core::GUID, importcontext : *mut RPC_NS_HANDLE) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsBindingImportBeginW(entrynamesyntax, entryname.unwrap_or(core::mem::zeroed()) as _, ifspec.unwrap_or(core::mem::zeroed()) as _, objuuid.unwrap_or(core::mem::zeroed()) as _, importcontext as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcNsBindingImportDone(importcontext: *mut RPC_NS_HANDLE) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingImportDone(importcontext : *mut RPC_NS_HANDLE) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsBindingImportDone(importcontext as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn RpcNsBindingImportNext(importcontext: RPC_NS_HANDLE, binding: *mut super::rpcdce::RPC_BINDING_HANDLE) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingImportNext(importcontext : RPC_NS_HANDLE, binding : *mut super::rpcdce::RPC_BINDING_HANDLE) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsBindingImportNext(importcontext, binding as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn RpcNsBindingLookupBeginA(entrynamesyntax: u32, entryname: Option<*const u8>, ifspec: Option<super::rpcdce::RPC_IF_HANDLE>, objuuid: Option<*const windows_core::GUID>, bindingmaxcount: u32, lookupcontext: *mut RPC_NS_HANDLE) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingLookupBeginA(entrynamesyntax : u32, entryname : *const u8, ifspec : super::rpcdce::RPC_IF_HANDLE, objuuid : *const windows_core::GUID, bindingmaxcount : u32, lookupcontext : *mut RPC_NS_HANDLE) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsBindingLookupBeginA(entrynamesyntax, entryname.unwrap_or(core::mem::zeroed()) as _, ifspec.unwrap_or(core::mem::zeroed()) as _, objuuid.unwrap_or(core::mem::zeroed()) as _, bindingmaxcount, lookupcontext as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn RpcNsBindingLookupBeginW(entrynamesyntax: u32, entryname: Option<*const u16>, ifspec: Option<super::rpcdce::RPC_IF_HANDLE>, objuuid: Option<*const windows_core::GUID>, bindingmaxcount: u32, lookupcontext: *mut RPC_NS_HANDLE) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingLookupBeginW(entrynamesyntax : u32, entryname : *const u16, ifspec : super::rpcdce::RPC_IF_HANDLE, objuuid : *const windows_core::GUID, bindingmaxcount : u32, lookupcontext : *mut RPC_NS_HANDLE) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsBindingLookupBeginW(entrynamesyntax, entryname.unwrap_or(core::mem::zeroed()) as _, ifspec.unwrap_or(core::mem::zeroed()) as _, objuuid.unwrap_or(core::mem::zeroed()) as _, bindingmaxcount, lookupcontext as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcNsBindingLookupDone(lookupcontext: *mut RPC_NS_HANDLE) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingLookupDone(lookupcontext : *mut RPC_NS_HANDLE) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsBindingLookupDone(lookupcontext as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn RpcNsBindingLookupNext(lookupcontext: RPC_NS_HANDLE, bindingvec: *mut *mut super::rpcdce::RPC_BINDING_VECTOR) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingLookupNext(lookupcontext : RPC_NS_HANDLE, bindingvec : *mut *mut super::rpcdce::RPC_BINDING_VECTOR) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsBindingLookupNext(lookupcontext, bindingvec as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn RpcNsBindingSelect(bindingvec: *mut super::rpcdce::RPC_BINDING_VECTOR, binding: *mut super::rpcdce::RPC_BINDING_HANDLE) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingSelect(bindingvec : *mut super::rpcdce::RPC_BINDING_VECTOR, binding : *mut super::rpcdce::RPC_BINDING_HANDLE) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsBindingSelect(bindingvec as _, binding as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn RpcNsBindingUnexportA(entrynamesyntax: u32, entryname: Option<*const u8>, ifspec: Option<super::rpcdce::RPC_IF_HANDLE>, objectuuidvec: Option<*const super::rpcdce::UUID_VECTOR>) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingUnexportA(entrynamesyntax : u32, entryname : *const u8, ifspec : super::rpcdce::RPC_IF_HANDLE, objectuuidvec : *const super::rpcdce::UUID_VECTOR) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsBindingUnexportA(entrynamesyntax, entryname.unwrap_or(core::mem::zeroed()) as _, ifspec.unwrap_or(core::mem::zeroed()) as _, objectuuidvec.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn RpcNsBindingUnexportPnPA(entrynamesyntax: u32, entryname: Option<*const u8>, ifspec: Option<super::rpcdce::RPC_IF_HANDLE>, objectvector: Option<*const super::rpcdce::UUID_VECTOR>) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingUnexportPnPA(entrynamesyntax : u32, entryname : *const u8, ifspec : super::rpcdce::RPC_IF_HANDLE, objectvector : *const super::rpcdce::UUID_VECTOR) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsBindingUnexportPnPA(entrynamesyntax, entryname.unwrap_or(core::mem::zeroed()) as _, ifspec.unwrap_or(core::mem::zeroed()) as _, objectvector.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn RpcNsBindingUnexportPnPW(entrynamesyntax: u32, entryname: Option<*const u16>, ifspec: Option<super::rpcdce::RPC_IF_HANDLE>, objectvector: Option<*const super::rpcdce::UUID_VECTOR>) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingUnexportPnPW(entrynamesyntax : u32, entryname : *const u16, ifspec : super::rpcdce::RPC_IF_HANDLE, objectvector : *const super::rpcdce::UUID_VECTOR) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsBindingUnexportPnPW(entrynamesyntax, entryname.unwrap_or(core::mem::zeroed()) as _, ifspec.unwrap_or(core::mem::zeroed()) as _, objectvector.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn RpcNsBindingUnexportW(entrynamesyntax: u32, entryname: Option<*const u16>, ifspec: Option<super::rpcdce::RPC_IF_HANDLE>, objectuuidvec: Option<*const super::rpcdce::UUID_VECTOR>) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsBindingUnexportW(entrynamesyntax : u32, entryname : *const u16, ifspec : super::rpcdce::RPC_IF_HANDLE, objectuuidvec : *const super::rpcdce::UUID_VECTOR) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsBindingUnexportW(entrynamesyntax, entryname.unwrap_or(core::mem::zeroed()) as _, ifspec.unwrap_or(core::mem::zeroed()) as _, objectuuidvec.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn RpcNsEntryExpandNameA(entrynamesyntax: u32, entryname: *const u8, expandedname: *mut super::rpcdce::RPC_CSTR) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsEntryExpandNameA(entrynamesyntax : u32, entryname : *const u8, expandedname : *mut super::rpcdce::RPC_CSTR) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsEntryExpandNameA(entrynamesyntax, entryname, expandedname as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn RpcNsEntryExpandNameW(entrynamesyntax: u32, entryname: *const u16, expandedname: *mut super::rpcdce::RPC_WSTR) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsEntryExpandNameW(entrynamesyntax : u32, entryname : *const u16, expandedname : *mut super::rpcdce::RPC_WSTR) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsEntryExpandNameW(entrynamesyntax, entryname, expandedname as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcNsEntryObjectInqBeginA(entrynamesyntax: u32, entryname: *const u8, inquirycontext: *mut RPC_NS_HANDLE) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsEntryObjectInqBeginA(entrynamesyntax : u32, entryname : *const u8, inquirycontext : *mut RPC_NS_HANDLE) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsEntryObjectInqBeginA(entrynamesyntax, entryname, inquirycontext as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcNsEntryObjectInqBeginW(entrynamesyntax: u32, entryname: *const u16, inquirycontext: *mut RPC_NS_HANDLE) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsEntryObjectInqBeginW(entrynamesyntax : u32, entryname : *const u16, inquirycontext : *mut RPC_NS_HANDLE) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsEntryObjectInqBeginW(entrynamesyntax, entryname, inquirycontext as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcNsEntryObjectInqDone(inquirycontext: *mut RPC_NS_HANDLE) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsEntryObjectInqDone(inquirycontext : *mut RPC_NS_HANDLE) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsEntryObjectInqDone(inquirycontext as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcNsEntryObjectInqNext(inquirycontext: RPC_NS_HANDLE, objuuid: *mut windows_core::GUID) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsEntryObjectInqNext(inquirycontext : RPC_NS_HANDLE, objuuid : *mut windows_core::GUID) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsEntryObjectInqNext(inquirycontext, objuuid as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcNsGroupDeleteA(groupnamesyntax: u32, groupname: Option<*const u8>) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsGroupDeleteA(groupnamesyntax : u32, groupname : *const u8) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsGroupDeleteA(groupnamesyntax, groupname.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcNsGroupDeleteW(groupnamesyntax: u32, groupname: Option<*const u16>) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsGroupDeleteW(groupnamesyntax : u32, groupname : *const u16) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsGroupDeleteW(groupnamesyntax, groupname.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcNsGroupMbrAddA(groupnamesyntax: u32, groupname: *const u8, membernamesyntax: u32, membername: *const u8) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsGroupMbrAddA(groupnamesyntax : u32, groupname : *const u8, membernamesyntax : u32, membername : *const u8) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsGroupMbrAddA(groupnamesyntax, groupname, membernamesyntax, membername) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcNsGroupMbrAddW(groupnamesyntax: u32, groupname: *const u16, membernamesyntax: u32, membername: *const u16) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsGroupMbrAddW(groupnamesyntax : u32, groupname : *const u16, membernamesyntax : u32, membername : *const u16) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsGroupMbrAddW(groupnamesyntax, groupname, membernamesyntax, membername) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcNsGroupMbrInqBeginA(groupnamesyntax: u32, groupname: *const u8, membernamesyntax: u32, inquirycontext: *mut RPC_NS_HANDLE) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsGroupMbrInqBeginA(groupnamesyntax : u32, groupname : *const u8, membernamesyntax : u32, inquirycontext : *mut RPC_NS_HANDLE) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsGroupMbrInqBeginA(groupnamesyntax, groupname, membernamesyntax, inquirycontext as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcNsGroupMbrInqBeginW(groupnamesyntax: u32, groupname: *const u16, membernamesyntax: u32, inquirycontext: *mut RPC_NS_HANDLE) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsGroupMbrInqBeginW(groupnamesyntax : u32, groupname : *const u16, membernamesyntax : u32, inquirycontext : *mut RPC_NS_HANDLE) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsGroupMbrInqBeginW(groupnamesyntax, groupname, membernamesyntax, inquirycontext as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcNsGroupMbrInqDone(inquirycontext: *mut RPC_NS_HANDLE) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsGroupMbrInqDone(inquirycontext : *mut RPC_NS_HANDLE) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsGroupMbrInqDone(inquirycontext as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn RpcNsGroupMbrInqNextA(inquirycontext: RPC_NS_HANDLE, membername: *mut super::rpcdce::RPC_CSTR) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsGroupMbrInqNextA(inquirycontext : RPC_NS_HANDLE, membername : *mut super::rpcdce::RPC_CSTR) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsGroupMbrInqNextA(inquirycontext as _, membername as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn RpcNsGroupMbrInqNextW(inquirycontext: RPC_NS_HANDLE, membername: *mut super::rpcdce::RPC_WSTR) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsGroupMbrInqNextW(inquirycontext : RPC_NS_HANDLE, membername : *mut super::rpcdce::RPC_WSTR) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsGroupMbrInqNextW(inquirycontext as _, membername as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcNsGroupMbrRemoveA(groupnamesyntax: u32, groupname: *const u8, membernamesyntax: u32, membername: *const u8) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsGroupMbrRemoveA(groupnamesyntax : u32, groupname : *const u8, membernamesyntax : u32, membername : *const u8) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsGroupMbrRemoveA(groupnamesyntax, groupname, membernamesyntax, membername) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcNsGroupMbrRemoveW(groupnamesyntax: u32, groupname: *const u16, membernamesyntax: u32, membername: *const u16) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsGroupMbrRemoveW(groupnamesyntax : u32, groupname : *const u16, membernamesyntax : u32, membername : *const u16) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsGroupMbrRemoveW(groupnamesyntax, groupname, membernamesyntax, membername) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn RpcNsMgmtBindingUnexportA(entrynamesyntax: u32, entryname: *const u8, ifid: Option<*const super::rpcdce::RPC_IF_ID>, versoption: u32, objectuuidvec: Option<*const super::rpcdce::UUID_VECTOR>) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsMgmtBindingUnexportA(entrynamesyntax : u32, entryname : *const u8, ifid : *const super::rpcdce::RPC_IF_ID, versoption : u32, objectuuidvec : *const super::rpcdce::UUID_VECTOR) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsMgmtBindingUnexportA(entrynamesyntax, entryname, ifid.unwrap_or(core::mem::zeroed()) as _, versoption, objectuuidvec.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn RpcNsMgmtBindingUnexportW(entrynamesyntax: u32, entryname: *const u16, ifid: Option<*const super::rpcdce::RPC_IF_ID>, versoption: u32, objectuuidvec: Option<*const super::rpcdce::UUID_VECTOR>) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsMgmtBindingUnexportW(entrynamesyntax : u32, entryname : *const u16, ifid : *const super::rpcdce::RPC_IF_ID, versoption : u32, objectuuidvec : *const super::rpcdce::UUID_VECTOR) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsMgmtBindingUnexportW(entrynamesyntax, entryname, ifid.unwrap_or(core::mem::zeroed()) as _, versoption, objectuuidvec.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcNsMgmtEntryCreateA(entrynamesyntax: u32, entryname: *const u8) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsMgmtEntryCreateA(entrynamesyntax : u32, entryname : *const u8) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsMgmtEntryCreateA(entrynamesyntax, entryname) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcNsMgmtEntryCreateW(entrynamesyntax: u32, entryname: *const u16) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsMgmtEntryCreateW(entrynamesyntax : u32, entryname : *const u16) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsMgmtEntryCreateW(entrynamesyntax, entryname) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcNsMgmtEntryDeleteA(entrynamesyntax: u32, entryname: *const u8) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsMgmtEntryDeleteA(entrynamesyntax : u32, entryname : *const u8) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsMgmtEntryDeleteA(entrynamesyntax, entryname) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcNsMgmtEntryDeleteW(entrynamesyntax: u32, entryname: *const u16) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsMgmtEntryDeleteW(entrynamesyntax : u32, entryname : *const u16) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsMgmtEntryDeleteW(entrynamesyntax, entryname) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn RpcNsMgmtEntryInqIfIdsA(entrynamesyntax: u32, entryname: *const u8, ifidvec: *mut *mut super::rpcdce::RPC_IF_ID_VECTOR) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsMgmtEntryInqIfIdsA(entrynamesyntax : u32, entryname : *const u8, ifidvec : *mut *mut super::rpcdce::RPC_IF_ID_VECTOR) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsMgmtEntryInqIfIdsA(entrynamesyntax, entryname, ifidvec as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn RpcNsMgmtEntryInqIfIdsW(entrynamesyntax: u32, entryname: *const u16, ifidvec: *mut *mut super::rpcdce::RPC_IF_ID_VECTOR) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsMgmtEntryInqIfIdsW(entrynamesyntax : u32, entryname : *const u16, ifidvec : *mut *mut super::rpcdce::RPC_IF_ID_VECTOR) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsMgmtEntryInqIfIdsW(entrynamesyntax, entryname, ifidvec as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcNsMgmtHandleSetExpAge(nshandle: RPC_NS_HANDLE, expirationage: u32) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsMgmtHandleSetExpAge(nshandle : RPC_NS_HANDLE, expirationage : u32) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsMgmtHandleSetExpAge(nshandle, expirationage) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcNsMgmtInqExpAge(expirationage: *mut u32) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsMgmtInqExpAge(expirationage : *mut u32) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsMgmtInqExpAge(expirationage as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcNsMgmtSetExpAge(expirationage: u32) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsMgmtSetExpAge(expirationage : u32) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsMgmtSetExpAge(expirationage) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcNsProfileDeleteA(profilenamesyntax: u32, profilename: *const u8) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsProfileDeleteA(profilenamesyntax : u32, profilename : *const u8) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsProfileDeleteA(profilenamesyntax, profilename) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcNsProfileDeleteW(profilenamesyntax: u32, profilename: *const u16) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsProfileDeleteW(profilenamesyntax : u32, profilename : *const u16) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsProfileDeleteW(profilenamesyntax, profilename) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn RpcNsProfileEltAddA(profilenamesyntax: u32, profilename: *const u8, ifid: Option<*const super::rpcdce::RPC_IF_ID>, membernamesyntax: u32, membername: *const u8, priority: u32, annotation: Option<*const u8>) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsProfileEltAddA(profilenamesyntax : u32, profilename : *const u8, ifid : *const super::rpcdce::RPC_IF_ID, membernamesyntax : u32, membername : *const u8, priority : u32, annotation : *const u8) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsProfileEltAddA(profilenamesyntax, profilename, ifid.unwrap_or(core::mem::zeroed()) as _, membernamesyntax, membername, priority, annotation.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn RpcNsProfileEltAddW(profilenamesyntax: u32, profilename: *const u16, ifid: Option<*const super::rpcdce::RPC_IF_ID>, membernamesyntax: u32, membername: *const u16, priority: u32, annotation: Option<*const u16>) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsProfileEltAddW(profilenamesyntax : u32, profilename : *const u16, ifid : *const super::rpcdce::RPC_IF_ID, membernamesyntax : u32, membername : *const u16, priority : u32, annotation : *const u16) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsProfileEltAddW(profilenamesyntax, profilename, ifid.unwrap_or(core::mem::zeroed()) as _, membernamesyntax, membername, priority, annotation.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn RpcNsProfileEltInqBeginA(profilenamesyntax: u32, profilename: *const u8, inquirytype: u32, ifid: Option<*const super::rpcdce::RPC_IF_ID>, versoption: u32, membernamesyntax: u32, membername: Option<*const u8>, inquirycontext: *mut RPC_NS_HANDLE) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsProfileEltInqBeginA(profilenamesyntax : u32, profilename : *const u8, inquirytype : u32, ifid : *const super::rpcdce::RPC_IF_ID, versoption : u32, membernamesyntax : u32, membername : *const u8, inquirycontext : *mut RPC_NS_HANDLE) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsProfileEltInqBeginA(profilenamesyntax, profilename, inquirytype, ifid.unwrap_or(core::mem::zeroed()) as _, versoption, membernamesyntax, membername.unwrap_or(core::mem::zeroed()) as _, inquirycontext as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn RpcNsProfileEltInqBeginW(profilenamesyntax: u32, profilename: *const u16, inquirytype: u32, ifid: Option<*const super::rpcdce::RPC_IF_ID>, versoption: u32, membernamesyntax: u32, membername: Option<*const u16>, inquirycontext: *mut RPC_NS_HANDLE) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsProfileEltInqBeginW(profilenamesyntax : u32, profilename : *const u16, inquirytype : u32, ifid : *const super::rpcdce::RPC_IF_ID, versoption : u32, membernamesyntax : u32, membername : *const u16, inquirycontext : *mut RPC_NS_HANDLE) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsProfileEltInqBeginW(profilenamesyntax, profilename, inquirytype, ifid.unwrap_or(core::mem::zeroed()) as _, versoption, membernamesyntax, membername.unwrap_or(core::mem::zeroed()) as _, inquirycontext as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcNsProfileEltInqDone(inquirycontext: *mut RPC_NS_HANDLE) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsProfileEltInqDone(inquirycontext : *mut RPC_NS_HANDLE) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsProfileEltInqDone(inquirycontext as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn RpcNsProfileEltInqNextA(inquirycontext: RPC_NS_HANDLE, ifid: Option<*mut super::rpcdce::RPC_IF_ID>, membername: *mut super::rpcdce::RPC_CSTR, priority: *mut u32, annotation: *mut super::rpcdce::RPC_CSTR) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsProfileEltInqNextA(inquirycontext : RPC_NS_HANDLE, ifid : *mut super::rpcdce::RPC_IF_ID, membername : *mut super::rpcdce::RPC_CSTR, priority : *mut u32, annotation : *mut super::rpcdce::RPC_CSTR) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsProfileEltInqNextA(inquirycontext, ifid.unwrap_or(core::mem::zeroed()) as _, membername as _, priority as _, annotation as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn RpcNsProfileEltInqNextW(inquirycontext: RPC_NS_HANDLE, ifid: Option<*mut super::rpcdce::RPC_IF_ID>, membername: *mut super::rpcdce::RPC_WSTR, priority: *mut u32, annotation: *mut super::rpcdce::RPC_WSTR) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsProfileEltInqNextW(inquirycontext : RPC_NS_HANDLE, ifid : *mut super::rpcdce::RPC_IF_ID, membername : *mut super::rpcdce::RPC_WSTR, priority : *mut u32, annotation : *mut super::rpcdce::RPC_WSTR) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsProfileEltInqNextW(inquirycontext, ifid.unwrap_or(core::mem::zeroed()) as _, membername as _, priority as _, annotation as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn RpcNsProfileEltRemoveA(profilenamesyntax: u32, profilename: *const u8, ifid: Option<*const super::rpcdce::RPC_IF_ID>, membernamesyntax: u32, membername: *const u8) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsProfileEltRemoveA(profilenamesyntax : u32, profilename : *const u8, ifid : *const super::rpcdce::RPC_IF_ID, membernamesyntax : u32, membername : *const u8) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsProfileEltRemoveA(profilenamesyntax, profilename, ifid.unwrap_or(core::mem::zeroed()) as _, membernamesyntax, membername) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn RpcNsProfileEltRemoveW(profilenamesyntax: u32, profilename: *const u16, ifid: Option<*const super::rpcdce::RPC_IF_ID>, membernamesyntax: u32, membername: *const u16) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn RpcNsProfileEltRemoveW(profilenamesyntax : u32, profilename : *const u16, ifid : *const super::rpcdce::RPC_IF_ID, membernamesyntax : u32, membername : *const u16) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsProfileEltRemoveW(profilenamesyntax, profilename, ifid.unwrap_or(core::mem::zeroed()) as _, membernamesyntax, membername) }
}
pub const RPC_C_NS_DEFAULT_EXP_AGE: i32 = -1;
pub const RPC_C_NS_SYNTAX_DCE: u32 = 3;
pub const RPC_C_NS_SYNTAX_DEFAULT: u32 = 0;
pub const RPC_C_PROFILE_ALL_ELT: u32 = 1;
pub const RPC_C_PROFILE_ALL_ELTS: u32 = 1;
pub const RPC_C_PROFILE_DEFAULT_ELT: u32 = 0;
pub const RPC_C_PROFILE_MATCH_BY_BOTH: u32 = 4;
pub const RPC_C_PROFILE_MATCH_BY_IF: u32 = 2;
pub const RPC_C_PROFILE_MATCH_BY_MBR: u32 = 3;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RPC_NS_HANDLE(pub *mut core::ffi::c_void);
impl RPC_NS_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for RPC_NS_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
