#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn DceErrorInqTextA(rpcstatus: super::rpc::RPC_STATUS, errortext: *mut u8) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn DceErrorInqTextA(rpcstatus : super::rpc::RPC_STATUS, errortext : *mut u8) -> super::rpc::RPC_STATUS);
    unsafe { DceErrorInqTextA(rpcstatus, errortext as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn DceErrorInqTextW(rpcstatus: super::rpc::RPC_STATUS, errortext: *mut u16) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn DceErrorInqTextW(rpcstatus : super::rpc::RPC_STATUS, errortext : *mut u16) -> super::rpc::RPC_STATUS);
    unsafe { DceErrorInqTextW(rpcstatus, errortext as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcBindingCopy(sourcebinding: RPC_BINDING_HANDLE, destinationbinding: *mut RPC_BINDING_HANDLE) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingCopy(sourcebinding : RPC_BINDING_HANDLE, destinationbinding : *mut RPC_BINDING_HANDLE) -> super::rpc::RPC_STATUS);
    unsafe { RpcBindingCopy(sourcebinding, destinationbinding as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcBindingCreateA(template: *const RPC_BINDING_HANDLE_TEMPLATE_V1_A, security: Option<*const RPC_BINDING_HANDLE_SECURITY_V1_A>, options: Option<*const RPC_BINDING_HANDLE_OPTIONS_V1>, binding: *mut RPC_BINDING_HANDLE) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingCreateA(template : *const RPC_BINDING_HANDLE_TEMPLATE_V1_A, security : *const RPC_BINDING_HANDLE_SECURITY_V1_A, options : *const RPC_BINDING_HANDLE_OPTIONS_V1, binding : *mut RPC_BINDING_HANDLE) -> super::rpc::RPC_STATUS);
    unsafe { RpcBindingCreateA(template, security.unwrap_or(core::mem::zeroed()) as _, options.unwrap_or(core::mem::zeroed()) as _, binding as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcBindingCreateW(template: *const RPC_BINDING_HANDLE_TEMPLATE_V1_W, security: Option<*const RPC_BINDING_HANDLE_SECURITY_V1_W>, options: Option<*const RPC_BINDING_HANDLE_OPTIONS_V1>, binding: *mut RPC_BINDING_HANDLE) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingCreateW(template : *const RPC_BINDING_HANDLE_TEMPLATE_V1_W, security : *const RPC_BINDING_HANDLE_SECURITY_V1_W, options : *const RPC_BINDING_HANDLE_OPTIONS_V1, binding : *mut RPC_BINDING_HANDLE) -> super::rpc::RPC_STATUS);
    unsafe { RpcBindingCreateW(template, security.unwrap_or(core::mem::zeroed()) as _, options.unwrap_or(core::mem::zeroed()) as _, binding as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcBindingFree(binding: *mut RPC_BINDING_HANDLE) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingFree(binding : *mut RPC_BINDING_HANDLE) -> super::rpc::RPC_STATUS);
    unsafe { RpcBindingFree(binding as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcBindingFromStringBindingA(stringbinding: *const u8, binding: *mut RPC_BINDING_HANDLE) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingFromStringBindingA(stringbinding : *const u8, binding : *mut RPC_BINDING_HANDLE) -> super::rpc::RPC_STATUS);
    unsafe { RpcBindingFromStringBindingA(stringbinding, binding as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcBindingFromStringBindingW(stringbinding: *const u16, binding: *mut RPC_BINDING_HANDLE) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingFromStringBindingW(stringbinding : *const u16, binding : *mut RPC_BINDING_HANDLE) -> super::rpc::RPC_STATUS);
    unsafe { RpcBindingFromStringBindingW(stringbinding, binding as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcBindingInqAuthClientA(clientbinding: Option<RPC_BINDING_HANDLE>, privs: *mut RPC_AUTHZ_HANDLE, serverprincname: *mut RPC_CSTR, authnlevel: Option<*mut u32>, authnsvc: Option<*mut u32>, authzsvc: Option<*mut u32>) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingInqAuthClientA(clientbinding : RPC_BINDING_HANDLE, privs : *mut RPC_AUTHZ_HANDLE, serverprincname : *mut RPC_CSTR, authnlevel : *mut u32, authnsvc : *mut u32, authzsvc : *mut u32) -> super::rpc::RPC_STATUS);
    unsafe { RpcBindingInqAuthClientA(clientbinding.unwrap_or(core::mem::zeroed()) as _, privs as _, serverprincname as _, authnlevel.unwrap_or(core::mem::zeroed()) as _, authnsvc.unwrap_or(core::mem::zeroed()) as _, authzsvc.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcBindingInqAuthClientExA(clientbinding: Option<RPC_BINDING_HANDLE>, privs: *mut RPC_AUTHZ_HANDLE, serverprincname: *mut RPC_CSTR, authnlevel: Option<*mut u32>, authnsvc: Option<*mut u32>, authzsvc: Option<*mut u32>, flags: u32) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingInqAuthClientExA(clientbinding : RPC_BINDING_HANDLE, privs : *mut RPC_AUTHZ_HANDLE, serverprincname : *mut RPC_CSTR, authnlevel : *mut u32, authnsvc : *mut u32, authzsvc : *mut u32, flags : u32) -> super::rpc::RPC_STATUS);
    unsafe { RpcBindingInqAuthClientExA(clientbinding.unwrap_or(core::mem::zeroed()) as _, privs as _, serverprincname as _, authnlevel.unwrap_or(core::mem::zeroed()) as _, authnsvc.unwrap_or(core::mem::zeroed()) as _, authzsvc.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcBindingInqAuthClientExW(clientbinding: Option<RPC_BINDING_HANDLE>, privs: *mut RPC_AUTHZ_HANDLE, serverprincname: *mut RPC_WSTR, authnlevel: Option<*mut u32>, authnsvc: Option<*mut u32>, authzsvc: Option<*mut u32>, flags: u32) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingInqAuthClientExW(clientbinding : RPC_BINDING_HANDLE, privs : *mut RPC_AUTHZ_HANDLE, serverprincname : *mut RPC_WSTR, authnlevel : *mut u32, authnsvc : *mut u32, authzsvc : *mut u32, flags : u32) -> super::rpc::RPC_STATUS);
    unsafe { RpcBindingInqAuthClientExW(clientbinding.unwrap_or(core::mem::zeroed()) as _, privs as _, serverprincname as _, authnlevel.unwrap_or(core::mem::zeroed()) as _, authnsvc.unwrap_or(core::mem::zeroed()) as _, authzsvc.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcBindingInqAuthClientW(clientbinding: Option<RPC_BINDING_HANDLE>, privs: *mut RPC_AUTHZ_HANDLE, serverprincname: *mut RPC_WSTR, authnlevel: Option<*mut u32>, authnsvc: Option<*mut u32>, authzsvc: Option<*mut u32>) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingInqAuthClientW(clientbinding : RPC_BINDING_HANDLE, privs : *mut RPC_AUTHZ_HANDLE, serverprincname : *mut RPC_WSTR, authnlevel : *mut u32, authnsvc : *mut u32, authzsvc : *mut u32) -> super::rpc::RPC_STATUS);
    unsafe { RpcBindingInqAuthClientW(clientbinding.unwrap_or(core::mem::zeroed()) as _, privs as _, serverprincname as _, authnlevel.unwrap_or(core::mem::zeroed()) as _, authnsvc.unwrap_or(core::mem::zeroed()) as _, authzsvc.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcBindingInqAuthInfoA(binding: RPC_BINDING_HANDLE, serverprincname: *mut RPC_CSTR, authnlevel: Option<*mut u32>, authnsvc: Option<*mut u32>, authidentity: Option<*mut RPC_AUTH_IDENTITY_HANDLE>, authzsvc: Option<*mut u32>) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingInqAuthInfoA(binding : RPC_BINDING_HANDLE, serverprincname : *mut RPC_CSTR, authnlevel : *mut u32, authnsvc : *mut u32, authidentity : *mut RPC_AUTH_IDENTITY_HANDLE, authzsvc : *mut u32) -> super::rpc::RPC_STATUS);
    unsafe { RpcBindingInqAuthInfoA(binding, serverprincname as _, authnlevel.unwrap_or(core::mem::zeroed()) as _, authnsvc.unwrap_or(core::mem::zeroed()) as _, authidentity.unwrap_or(core::mem::zeroed()) as _, authzsvc.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcBindingInqAuthInfoExA(binding: RPC_BINDING_HANDLE, serverprincname: *mut RPC_CSTR, authnlevel: Option<*mut u32>, authnsvc: Option<*mut u32>, authidentity: Option<*mut RPC_AUTH_IDENTITY_HANDLE>, authzsvc: Option<*mut u32>, rpcqosversion: u32, securityqos: Option<*mut RPC_SECURITY_QOS>) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingInqAuthInfoExA(binding : RPC_BINDING_HANDLE, serverprincname : *mut RPC_CSTR, authnlevel : *mut u32, authnsvc : *mut u32, authidentity : *mut RPC_AUTH_IDENTITY_HANDLE, authzsvc : *mut u32, rpcqosversion : u32, securityqos : *mut RPC_SECURITY_QOS) -> super::rpc::RPC_STATUS);
    unsafe { RpcBindingInqAuthInfoExA(binding, serverprincname as _, authnlevel.unwrap_or(core::mem::zeroed()) as _, authnsvc.unwrap_or(core::mem::zeroed()) as _, authidentity.unwrap_or(core::mem::zeroed()) as _, authzsvc.unwrap_or(core::mem::zeroed()) as _, rpcqosversion, securityqos.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcBindingInqAuthInfoExW(binding: RPC_BINDING_HANDLE, serverprincname: *mut RPC_WSTR, authnlevel: Option<*mut u32>, authnsvc: Option<*mut u32>, authidentity: Option<*mut RPC_AUTH_IDENTITY_HANDLE>, authzsvc: Option<*mut u32>, rpcqosversion: u32, securityqos: Option<*mut RPC_SECURITY_QOS>) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingInqAuthInfoExW(binding : RPC_BINDING_HANDLE, serverprincname : *mut RPC_WSTR, authnlevel : *mut u32, authnsvc : *mut u32, authidentity : *mut RPC_AUTH_IDENTITY_HANDLE, authzsvc : *mut u32, rpcqosversion : u32, securityqos : *mut RPC_SECURITY_QOS) -> super::rpc::RPC_STATUS);
    unsafe { RpcBindingInqAuthInfoExW(binding, serverprincname as _, authnlevel.unwrap_or(core::mem::zeroed()) as _, authnsvc.unwrap_or(core::mem::zeroed()) as _, authidentity.unwrap_or(core::mem::zeroed()) as _, authzsvc.unwrap_or(core::mem::zeroed()) as _, rpcqosversion, securityqos.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcBindingInqAuthInfoW(binding: RPC_BINDING_HANDLE, serverprincname: *mut RPC_WSTR, authnlevel: Option<*mut u32>, authnsvc: Option<*mut u32>, authidentity: Option<*mut RPC_AUTH_IDENTITY_HANDLE>, authzsvc: Option<*mut u32>) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingInqAuthInfoW(binding : RPC_BINDING_HANDLE, serverprincname : *mut RPC_WSTR, authnlevel : *mut u32, authnsvc : *mut u32, authidentity : *mut RPC_AUTH_IDENTITY_HANDLE, authzsvc : *mut u32) -> super::rpc::RPC_STATUS);
    unsafe { RpcBindingInqAuthInfoW(binding, serverprincname as _, authnlevel.unwrap_or(core::mem::zeroed()) as _, authnsvc.unwrap_or(core::mem::zeroed()) as _, authidentity.unwrap_or(core::mem::zeroed()) as _, authzsvc.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcBindingInqMaxCalls(binding: RPC_BINDING_HANDLE, maxcalls: *mut u32) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingInqMaxCalls(binding : RPC_BINDING_HANDLE, maxcalls : *mut u32) -> super::rpc::RPC_STATUS);
    unsafe { RpcBindingInqMaxCalls(binding, maxcalls as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcBindingInqObject(binding: RPC_BINDING_HANDLE, objectuuid: *mut windows_core::GUID) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingInqObject(binding : RPC_BINDING_HANDLE, objectuuid : *mut windows_core::GUID) -> super::rpc::RPC_STATUS);
    unsafe { RpcBindingInqObject(binding, objectuuid as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcBindingInqOption(hbinding: RPC_BINDING_HANDLE, option: u32, poptionvalue: *mut usize) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingInqOption(hbinding : RPC_BINDING_HANDLE, option : u32, poptionvalue : *mut usize) -> super::rpc::RPC_STATUS);
    unsafe { RpcBindingInqOption(hbinding, option, poptionvalue as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcBindingReset(binding: RPC_BINDING_HANDLE) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingReset(binding : RPC_BINDING_HANDLE) -> super::rpc::RPC_STATUS);
    unsafe { RpcBindingReset(binding) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcBindingServerFromClient(clientbinding: Option<RPC_BINDING_HANDLE>, serverbinding: *mut RPC_BINDING_HANDLE) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingServerFromClient(clientbinding : RPC_BINDING_HANDLE, serverbinding : *mut RPC_BINDING_HANDLE) -> super::rpc::RPC_STATUS);
    unsafe { RpcBindingServerFromClient(clientbinding.unwrap_or(core::mem::zeroed()) as _, serverbinding as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcBindingSetAuthInfoA(binding: RPC_BINDING_HANDLE, serverprincname: Option<*const u8>, authnlevel: u32, authnsvc: u32, authidentity: Option<RPC_AUTH_IDENTITY_HANDLE>, authzsvc: u32) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingSetAuthInfoA(binding : RPC_BINDING_HANDLE, serverprincname : *const u8, authnlevel : u32, authnsvc : u32, authidentity : RPC_AUTH_IDENTITY_HANDLE, authzsvc : u32) -> super::rpc::RPC_STATUS);
    unsafe { RpcBindingSetAuthInfoA(binding, serverprincname.unwrap_or(core::mem::zeroed()) as _, authnlevel, authnsvc, authidentity.unwrap_or(core::mem::zeroed()) as _, authzsvc) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcBindingSetAuthInfoExA(binding: RPC_BINDING_HANDLE, serverprincname: Option<*const u8>, authnlevel: u32, authnsvc: u32, authidentity: Option<RPC_AUTH_IDENTITY_HANDLE>, authzsvc: u32, securityqos: Option<*const RPC_SECURITY_QOS>) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingSetAuthInfoExA(binding : RPC_BINDING_HANDLE, serverprincname : *const u8, authnlevel : u32, authnsvc : u32, authidentity : RPC_AUTH_IDENTITY_HANDLE, authzsvc : u32, securityqos : *const RPC_SECURITY_QOS) -> super::rpc::RPC_STATUS);
    unsafe { RpcBindingSetAuthInfoExA(binding, serverprincname.unwrap_or(core::mem::zeroed()) as _, authnlevel, authnsvc, authidentity.unwrap_or(core::mem::zeroed()) as _, authzsvc, securityqos.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcBindingSetAuthInfoExW(binding: RPC_BINDING_HANDLE, serverprincname: Option<*const u16>, authnlevel: u32, authnsvc: u32, authidentity: Option<RPC_AUTH_IDENTITY_HANDLE>, authzsvc: u32, securityqos: Option<*const RPC_SECURITY_QOS>) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingSetAuthInfoExW(binding : RPC_BINDING_HANDLE, serverprincname : *const u16, authnlevel : u32, authnsvc : u32, authidentity : RPC_AUTH_IDENTITY_HANDLE, authzsvc : u32, securityqos : *const RPC_SECURITY_QOS) -> super::rpc::RPC_STATUS);
    unsafe { RpcBindingSetAuthInfoExW(binding, serverprincname.unwrap_or(core::mem::zeroed()) as _, authnlevel, authnsvc, authidentity.unwrap_or(core::mem::zeroed()) as _, authzsvc, securityqos.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcBindingSetAuthInfoW(binding: RPC_BINDING_HANDLE, serverprincname: Option<*const u16>, authnlevel: u32, authnsvc: u32, authidentity: Option<RPC_AUTH_IDENTITY_HANDLE>, authzsvc: u32) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingSetAuthInfoW(binding : RPC_BINDING_HANDLE, serverprincname : *const u16, authnlevel : u32, authnsvc : u32, authidentity : RPC_AUTH_IDENTITY_HANDLE, authzsvc : u32) -> super::rpc::RPC_STATUS);
    unsafe { RpcBindingSetAuthInfoW(binding, serverprincname.unwrap_or(core::mem::zeroed()) as _, authnlevel, authnsvc, authidentity.unwrap_or(core::mem::zeroed()) as _, authzsvc) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcBindingSetObject(binding: RPC_BINDING_HANDLE, objectuuid: *const windows_core::GUID) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingSetObject(binding : RPC_BINDING_HANDLE, objectuuid : *const windows_core::GUID) -> super::rpc::RPC_STATUS);
    unsafe { RpcBindingSetObject(binding, objectuuid) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcBindingSetOption(hbinding: RPC_BINDING_HANDLE, option: u32, optionvalue: usize) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingSetOption(hbinding : RPC_BINDING_HANDLE, option : u32, optionvalue : usize) -> super::rpc::RPC_STATUS);
    unsafe { RpcBindingSetOption(hbinding, option, optionvalue) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcBindingToStringBindingA(binding: RPC_BINDING_HANDLE, stringbinding: *mut RPC_CSTR) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingToStringBindingA(binding : RPC_BINDING_HANDLE, stringbinding : *mut RPC_CSTR) -> super::rpc::RPC_STATUS);
    unsafe { RpcBindingToStringBindingA(binding, stringbinding as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcBindingToStringBindingW(binding: RPC_BINDING_HANDLE, stringbinding: *mut RPC_WSTR) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingToStringBindingW(binding : RPC_BINDING_HANDLE, stringbinding : *mut RPC_WSTR) -> super::rpc::RPC_STATUS);
    unsafe { RpcBindingToStringBindingW(binding, stringbinding as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcBindingVectorFree(bindingvector: *mut *mut RPC_BINDING_VECTOR) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcBindingVectorFree(bindingvector : *mut *mut RPC_BINDING_VECTOR) -> super::rpc::RPC_STATUS);
    unsafe { RpcBindingVectorFree(bindingvector as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcCancelThread(thread: *const core::ffi::c_void) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcCancelThread(thread : *const core::ffi::c_void) -> super::rpc::RPC_STATUS);
    unsafe { RpcCancelThread(thread) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcCancelThreadEx(thread: *const core::ffi::c_void, timeout: i32) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcCancelThreadEx(thread : *const core::ffi::c_void, timeout : i32) -> super::rpc::RPC_STATUS);
    unsafe { RpcCancelThreadEx(thread, timeout) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcEpRegisterA(ifspec: RPC_IF_HANDLE, bindingvector: *const RPC_BINDING_VECTOR, uuidvector: Option<*const UUID_VECTOR>, annotation: Option<*const u8>) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcEpRegisterA(ifspec : RPC_IF_HANDLE, bindingvector : *const RPC_BINDING_VECTOR, uuidvector : *const UUID_VECTOR, annotation : *const u8) -> super::rpc::RPC_STATUS);
    unsafe { RpcEpRegisterA(ifspec, bindingvector, uuidvector.unwrap_or(core::mem::zeroed()) as _, annotation.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcEpRegisterNoReplaceA(ifspec: RPC_IF_HANDLE, bindingvector: *const RPC_BINDING_VECTOR, uuidvector: Option<*const UUID_VECTOR>, annotation: Option<*const u8>) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcEpRegisterNoReplaceA(ifspec : RPC_IF_HANDLE, bindingvector : *const RPC_BINDING_VECTOR, uuidvector : *const UUID_VECTOR, annotation : *const u8) -> super::rpc::RPC_STATUS);
    unsafe { RpcEpRegisterNoReplaceA(ifspec, bindingvector, uuidvector.unwrap_or(core::mem::zeroed()) as _, annotation.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcEpRegisterNoReplaceW(ifspec: RPC_IF_HANDLE, bindingvector: *const RPC_BINDING_VECTOR, uuidvector: Option<*const UUID_VECTOR>, annotation: Option<*const u16>) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcEpRegisterNoReplaceW(ifspec : RPC_IF_HANDLE, bindingvector : *const RPC_BINDING_VECTOR, uuidvector : *const UUID_VECTOR, annotation : *const u16) -> super::rpc::RPC_STATUS);
    unsafe { RpcEpRegisterNoReplaceW(ifspec, bindingvector, uuidvector.unwrap_or(core::mem::zeroed()) as _, annotation.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcEpRegisterW(ifspec: RPC_IF_HANDLE, bindingvector: *const RPC_BINDING_VECTOR, uuidvector: Option<*const UUID_VECTOR>, annotation: Option<*const u16>) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcEpRegisterW(ifspec : RPC_IF_HANDLE, bindingvector : *const RPC_BINDING_VECTOR, uuidvector : *const UUID_VECTOR, annotation : *const u16) -> super::rpc::RPC_STATUS);
    unsafe { RpcEpRegisterW(ifspec, bindingvector, uuidvector.unwrap_or(core::mem::zeroed()) as _, annotation.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcEpResolveBinding(binding: RPC_BINDING_HANDLE, ifspec: RPC_IF_HANDLE) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcEpResolveBinding(binding : RPC_BINDING_HANDLE, ifspec : RPC_IF_HANDLE) -> super::rpc::RPC_STATUS);
    unsafe { RpcEpResolveBinding(binding, ifspec) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcEpUnregister(ifspec: RPC_IF_HANDLE, bindingvector: *const RPC_BINDING_VECTOR, uuidvector: Option<*const UUID_VECTOR>) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcEpUnregister(ifspec : RPC_IF_HANDLE, bindingvector : *const RPC_BINDING_VECTOR, uuidvector : *const UUID_VECTOR) -> super::rpc::RPC_STATUS);
    unsafe { RpcEpUnregister(ifspec, bindingvector, uuidvector.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcExceptionFilter(exceptioncode: u32) -> i32 {
    windows_core::link!("rpcrt4.dll" "system" fn RpcExceptionFilter(exceptioncode : u32) -> i32);
    unsafe { RpcExceptionFilter(exceptioncode) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcIfIdVectorFree(ifidvector: *mut *mut RPC_IF_ID_VECTOR) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcIfIdVectorFree(ifidvector : *mut *mut RPC_IF_ID_VECTOR) -> super::rpc::RPC_STATUS);
    unsafe { RpcIfIdVectorFree(ifidvector as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcIfInqId(rpcifhandle: RPC_IF_HANDLE, rpcifid: *mut RPC_IF_ID) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcIfInqId(rpcifhandle : RPC_IF_HANDLE, rpcifid : *mut RPC_IF_ID) -> super::rpc::RPC_STATUS);
    unsafe { RpcIfInqId(rpcifhandle, rpcifid as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcImpersonateClient(bindinghandle: Option<RPC_BINDING_HANDLE>) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcImpersonateClient(bindinghandle : RPC_BINDING_HANDLE) -> super::rpc::RPC_STATUS);
    unsafe { RpcImpersonateClient(bindinghandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcImpersonateClient2(bindinghandle: Option<RPC_BINDING_HANDLE>) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcImpersonateClient2(bindinghandle : RPC_BINDING_HANDLE) -> super::rpc::RPC_STATUS);
    unsafe { RpcImpersonateClient2(bindinghandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcImpersonateClientContainer(bindinghandle: Option<RPC_BINDING_HANDLE>) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcImpersonateClientContainer(bindinghandle : RPC_BINDING_HANDLE) -> super::rpc::RPC_STATUS);
    unsafe { RpcImpersonateClientContainer(bindinghandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcMgmtEnableIdleCleanup() -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtEnableIdleCleanup() -> super::rpc::RPC_STATUS);
    unsafe { RpcMgmtEnableIdleCleanup() }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcMgmtEpEltInqBegin(epbinding: Option<RPC_BINDING_HANDLE>, inquirytype: u32, ifid: Option<*const RPC_IF_ID>, versoption: Option<u32>, objectuuid: Option<*const windows_core::GUID>, inquirycontext: *mut RPC_EP_INQ_HANDLE) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtEpEltInqBegin(epbinding : RPC_BINDING_HANDLE, inquirytype : u32, ifid : *const RPC_IF_ID, versoption : u32, objectuuid : *const windows_core::GUID, inquirycontext : *mut RPC_EP_INQ_HANDLE) -> super::rpc::RPC_STATUS);
    unsafe { RpcMgmtEpEltInqBegin(epbinding.unwrap_or(core::mem::zeroed()) as _, inquirytype, ifid.unwrap_or(core::mem::zeroed()) as _, versoption.unwrap_or(core::mem::zeroed()) as _, objectuuid.unwrap_or(core::mem::zeroed()) as _, inquirycontext as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcMgmtEpEltInqDone(inquirycontext: *mut RPC_EP_INQ_HANDLE) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtEpEltInqDone(inquirycontext : *mut RPC_EP_INQ_HANDLE) -> super::rpc::RPC_STATUS);
    unsafe { RpcMgmtEpEltInqDone(inquirycontext as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcMgmtEpEltInqNextA(inquirycontext: *const super::rpc::I_RPC_HANDLE, ifid: *mut RPC_IF_ID, binding: Option<*mut RPC_BINDING_HANDLE>, objectuuid: Option<*mut windows_core::GUID>, annotation: *mut RPC_CSTR) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtEpEltInqNextA(inquirycontext : *const super::rpc::I_RPC_HANDLE, ifid : *mut RPC_IF_ID, binding : *mut RPC_BINDING_HANDLE, objectuuid : *mut windows_core::GUID, annotation : *mut RPC_CSTR) -> super::rpc::RPC_STATUS);
    unsafe { RpcMgmtEpEltInqNextA(inquirycontext, ifid as _, binding.unwrap_or(core::mem::zeroed()) as _, objectuuid.unwrap_or(core::mem::zeroed()) as _, annotation as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcMgmtEpEltInqNextW(inquirycontext: *const super::rpc::I_RPC_HANDLE, ifid: *mut RPC_IF_ID, binding: Option<*mut RPC_BINDING_HANDLE>, objectuuid: Option<*mut windows_core::GUID>, annotation: *mut RPC_WSTR) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtEpEltInqNextW(inquirycontext : *const super::rpc::I_RPC_HANDLE, ifid : *mut RPC_IF_ID, binding : *mut RPC_BINDING_HANDLE, objectuuid : *mut windows_core::GUID, annotation : *mut RPC_WSTR) -> super::rpc::RPC_STATUS);
    unsafe { RpcMgmtEpEltInqNextW(inquirycontext, ifid as _, binding.unwrap_or(core::mem::zeroed()) as _, objectuuid.unwrap_or(core::mem::zeroed()) as _, annotation as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcMgmtEpUnregister(epbinding: Option<RPC_BINDING_HANDLE>, ifid: *const RPC_IF_ID, binding: RPC_BINDING_HANDLE, objectuuid: Option<*const windows_core::GUID>) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtEpUnregister(epbinding : RPC_BINDING_HANDLE, ifid : *const RPC_IF_ID, binding : RPC_BINDING_HANDLE, objectuuid : *const windows_core::GUID) -> super::rpc::RPC_STATUS);
    unsafe { RpcMgmtEpUnregister(epbinding.unwrap_or(core::mem::zeroed()) as _, ifid, binding, objectuuid.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcMgmtInqComTimeout(binding: RPC_BINDING_HANDLE, timeout: *mut u32) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtInqComTimeout(binding : RPC_BINDING_HANDLE, timeout : *mut u32) -> super::rpc::RPC_STATUS);
    unsafe { RpcMgmtInqComTimeout(binding, timeout as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcMgmtInqDefaultProtectLevel(authnsvc: u32, authnlevel: *mut u32) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtInqDefaultProtectLevel(authnsvc : u32, authnlevel : *mut u32) -> super::rpc::RPC_STATUS);
    unsafe { RpcMgmtInqDefaultProtectLevel(authnsvc, authnlevel as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcMgmtInqIfIds(binding: Option<RPC_BINDING_HANDLE>, ifidvector: *mut *mut RPC_IF_ID_VECTOR) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtInqIfIds(binding : RPC_BINDING_HANDLE, ifidvector : *mut *mut RPC_IF_ID_VECTOR) -> super::rpc::RPC_STATUS);
    unsafe { RpcMgmtInqIfIds(binding.unwrap_or(core::mem::zeroed()) as _, ifidvector as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcMgmtInqServerPrincNameA(binding: Option<RPC_BINDING_HANDLE>, authnsvc: u32, serverprincname: *mut RPC_CSTR) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtInqServerPrincNameA(binding : RPC_BINDING_HANDLE, authnsvc : u32, serverprincname : *mut RPC_CSTR) -> super::rpc::RPC_STATUS);
    unsafe { RpcMgmtInqServerPrincNameA(binding.unwrap_or(core::mem::zeroed()) as _, authnsvc, serverprincname as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcMgmtInqServerPrincNameW(binding: Option<RPC_BINDING_HANDLE>, authnsvc: u32, serverprincname: *mut RPC_WSTR) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtInqServerPrincNameW(binding : RPC_BINDING_HANDLE, authnsvc : u32, serverprincname : *mut RPC_WSTR) -> super::rpc::RPC_STATUS);
    unsafe { RpcMgmtInqServerPrincNameW(binding.unwrap_or(core::mem::zeroed()) as _, authnsvc, serverprincname as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcMgmtInqStats(binding: Option<RPC_BINDING_HANDLE>, statistics: *mut *mut RPC_STATS_VECTOR) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtInqStats(binding : RPC_BINDING_HANDLE, statistics : *mut *mut RPC_STATS_VECTOR) -> super::rpc::RPC_STATUS);
    unsafe { RpcMgmtInqStats(binding.unwrap_or(core::mem::zeroed()) as _, statistics as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcMgmtIsServerListening(binding: Option<RPC_BINDING_HANDLE>) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtIsServerListening(binding : RPC_BINDING_HANDLE) -> super::rpc::RPC_STATUS);
    unsafe { RpcMgmtIsServerListening(binding.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcMgmtSetAuthorizationFn(authorizationfn: RPC_MGMT_AUTHORIZATION_FN) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtSetAuthorizationFn(authorizationfn : RPC_MGMT_AUTHORIZATION_FN) -> super::rpc::RPC_STATUS);
    unsafe { RpcMgmtSetAuthorizationFn(authorizationfn) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcMgmtSetCancelTimeout(timeout: i32) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtSetCancelTimeout(timeout : i32) -> super::rpc::RPC_STATUS);
    unsafe { RpcMgmtSetCancelTimeout(timeout) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcMgmtSetComTimeout(binding: RPC_BINDING_HANDLE, timeout: u32) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtSetComTimeout(binding : RPC_BINDING_HANDLE, timeout : u32) -> super::rpc::RPC_STATUS);
    unsafe { RpcMgmtSetComTimeout(binding, timeout) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcMgmtSetServerStackSize(threadstacksize: u32) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtSetServerStackSize(threadstacksize : u32) -> super::rpc::RPC_STATUS);
    unsafe { RpcMgmtSetServerStackSize(threadstacksize) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcMgmtStatsVectorFree(statsvector: *mut *mut RPC_STATS_VECTOR) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtStatsVectorFree(statsvector : *mut *mut RPC_STATS_VECTOR) -> super::rpc::RPC_STATUS);
    unsafe { RpcMgmtStatsVectorFree(statsvector as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcMgmtStopServerListening(binding: Option<RPC_BINDING_HANDLE>) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtStopServerListening(binding : RPC_BINDING_HANDLE) -> super::rpc::RPC_STATUS);
    unsafe { RpcMgmtStopServerListening(binding.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcMgmtWaitServerListen() -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcMgmtWaitServerListen() -> super::rpc::RPC_STATUS);
    unsafe { RpcMgmtWaitServerListen() }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcNetworkInqProtseqsA(protseqvector: *mut *mut RPC_PROTSEQ_VECTORA) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcNetworkInqProtseqsA(protseqvector : *mut *mut RPC_PROTSEQ_VECTORA) -> super::rpc::RPC_STATUS);
    unsafe { RpcNetworkInqProtseqsA(protseqvector as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcNetworkInqProtseqsW(protseqvector: *mut *mut RPC_PROTSEQ_VECTORW) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcNetworkInqProtseqsW(protseqvector : *mut *mut RPC_PROTSEQ_VECTORW) -> super::rpc::RPC_STATUS);
    unsafe { RpcNetworkInqProtseqsW(protseqvector as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcNetworkIsProtseqValidA(protseq: *const u8) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcNetworkIsProtseqValidA(protseq : *const u8) -> super::rpc::RPC_STATUS);
    unsafe { RpcNetworkIsProtseqValidA(protseq) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcNetworkIsProtseqValidW(protseq: *const u16) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcNetworkIsProtseqValidW(protseq : *const u16) -> super::rpc::RPC_STATUS);
    unsafe { RpcNetworkIsProtseqValidW(protseq) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcNsBindingInqEntryNameA(binding: RPC_BINDING_HANDLE, entrynamesyntax: u32, entryname: *mut RPC_CSTR) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcNsBindingInqEntryNameA(binding : RPC_BINDING_HANDLE, entrynamesyntax : u32, entryname : *mut RPC_CSTR) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsBindingInqEntryNameA(binding, entrynamesyntax, entryname as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcNsBindingInqEntryNameW(binding: RPC_BINDING_HANDLE, entrynamesyntax: u32, entryname: *mut RPC_WSTR) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcNsBindingInqEntryNameW(binding : RPC_BINDING_HANDLE, entrynamesyntax : u32, entryname : *mut RPC_WSTR) -> super::rpc::RPC_STATUS);
    unsafe { RpcNsBindingInqEntryNameW(binding, entrynamesyntax, entryname as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcObjectInqType(objuuid: *const windows_core::GUID, typeuuid: Option<*mut windows_core::GUID>) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcObjectInqType(objuuid : *const windows_core::GUID, typeuuid : *mut windows_core::GUID) -> super::rpc::RPC_STATUS);
    unsafe { RpcObjectInqType(objuuid, typeuuid.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcObjectSetInqFn(inquiryfn: RPC_OBJECT_INQ_FN) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcObjectSetInqFn(inquiryfn : RPC_OBJECT_INQ_FN) -> super::rpc::RPC_STATUS);
    unsafe { RpcObjectSetInqFn(inquiryfn) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcObjectSetType(objuuid: *const windows_core::GUID, typeuuid: Option<*const windows_core::GUID>) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcObjectSetType(objuuid : *const windows_core::GUID, typeuuid : *const windows_core::GUID) -> super::rpc::RPC_STATUS);
    unsafe { RpcObjectSetType(objuuid, typeuuid.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcProtseqVectorFreeA(protseqvector: *mut *mut RPC_PROTSEQ_VECTORA) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcProtseqVectorFreeA(protseqvector : *mut *mut RPC_PROTSEQ_VECTORA) -> super::rpc::RPC_STATUS);
    unsafe { RpcProtseqVectorFreeA(protseqvector as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcProtseqVectorFreeW(protseqvector: *mut *mut RPC_PROTSEQ_VECTORW) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcProtseqVectorFreeW(protseqvector : *mut *mut RPC_PROTSEQ_VECTORW) -> super::rpc::RPC_STATUS);
    unsafe { RpcProtseqVectorFreeW(protseqvector as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcRaiseException(exception: super::rpc::RPC_STATUS) -> ! {
    windows_core::link!("rpcrt4.dll" "system" fn RpcRaiseException(exception : super::rpc::RPC_STATUS) -> !);
    unsafe { RpcRaiseException(exception) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcRevertContainerImpersonation() -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcRevertContainerImpersonation() -> super::rpc::RPC_STATUS);
    unsafe { RpcRevertContainerImpersonation() }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcRevertToSelf() -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcRevertToSelf() -> super::rpc::RPC_STATUS);
    unsafe { RpcRevertToSelf() }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcRevertToSelfEx(bindinghandle: Option<RPC_BINDING_HANDLE>) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcRevertToSelfEx(bindinghandle : RPC_BINDING_HANDLE) -> super::rpc::RPC_STATUS);
    unsafe { RpcRevertToSelfEx(bindinghandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcServerCompleteSecurityCallback(bindinghandle: RPC_BINDING_HANDLE, status: super::rpc::RPC_STATUS) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerCompleteSecurityCallback(bindinghandle : RPC_BINDING_HANDLE, status : super::rpc::RPC_STATUS) -> super::rpc::RPC_STATUS);
    unsafe { RpcServerCompleteSecurityCallback(bindinghandle, status) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcServerInqBindingHandle(binding: *mut RPC_BINDING_HANDLE) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerInqBindingHandle(binding : *mut RPC_BINDING_HANDLE) -> super::rpc::RPC_STATUS);
    unsafe { RpcServerInqBindingHandle(binding as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcServerInqBindings(bindingvector: *mut *mut RPC_BINDING_VECTOR) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerInqBindings(bindingvector : *mut *mut RPC_BINDING_VECTOR) -> super::rpc::RPC_STATUS);
    unsafe { RpcServerInqBindings(bindingvector as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcServerInqBindingsEx(securitydescriptor: Option<*const core::ffi::c_void>, bindingvector: *mut *mut RPC_BINDING_VECTOR) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerInqBindingsEx(securitydescriptor : *const core::ffi::c_void, bindingvector : *mut *mut RPC_BINDING_VECTOR) -> super::rpc::RPC_STATUS);
    unsafe { RpcServerInqBindingsEx(securitydescriptor.unwrap_or(core::mem::zeroed()) as _, bindingvector as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcServerInqDefaultPrincNameA(authnsvc: u32, princname: *mut RPC_CSTR) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerInqDefaultPrincNameA(authnsvc : u32, princname : *mut RPC_CSTR) -> super::rpc::RPC_STATUS);
    unsafe { RpcServerInqDefaultPrincNameA(authnsvc, princname as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcServerInqDefaultPrincNameW(authnsvc: u32, princname: *mut RPC_WSTR) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerInqDefaultPrincNameW(authnsvc : u32, princname : *mut RPC_WSTR) -> super::rpc::RPC_STATUS);
    unsafe { RpcServerInqDefaultPrincNameW(authnsvc, princname as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcServerInqIf(ifspec: RPC_IF_HANDLE, mgrtypeuuid: Option<*const windows_core::GUID>, mgrepv: *mut *mut core::ffi::c_void) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerInqIf(ifspec : RPC_IF_HANDLE, mgrtypeuuid : *const windows_core::GUID, mgrepv : *mut *mut core::ffi::c_void) -> super::rpc::RPC_STATUS);
    unsafe { RpcServerInqIf(ifspec, mgrtypeuuid.unwrap_or(core::mem::zeroed()) as _, mgrepv as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcServerInterfaceGroupActivate(ifgroup: RPC_INTERFACE_GROUP) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerInterfaceGroupActivate(ifgroup : RPC_INTERFACE_GROUP) -> super::rpc::RPC_STATUS);
    unsafe { RpcServerInterfaceGroupActivate(ifgroup) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcServerInterfaceGroupClose(ifgroup: RPC_INTERFACE_GROUP) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerInterfaceGroupClose(ifgroup : RPC_INTERFACE_GROUP) -> super::rpc::RPC_STATUS);
    unsafe { RpcServerInterfaceGroupClose(ifgroup) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcServerInterfaceGroupCreateA(interfaces: &[RPC_INTERFACE_TEMPLATEA], endpoints: &[RPC_ENDPOINT_TEMPLATEA], idleperiod: u32, idlecallbackfn: *const core::ffi::c_void, idlecallbackcontext: *const core::ffi::c_void, ifgroup: *mut *mut core::ffi::c_void) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerInterfaceGroupCreateA(interfaces : *const RPC_INTERFACE_TEMPLATEA, numifs : u32, endpoints : *const RPC_ENDPOINT_TEMPLATEA, numendpoints : u32, idleperiod : u32, idlecallbackfn : *const core::ffi::c_void, idlecallbackcontext : *const core::ffi::c_void, ifgroup : *mut *mut core::ffi::c_void) -> super::rpc::RPC_STATUS);
    unsafe { RpcServerInterfaceGroupCreateA(core::mem::transmute(interfaces.as_ptr()), interfaces.len().try_into().unwrap(), core::mem::transmute(endpoints.as_ptr()), endpoints.len().try_into().unwrap(), idleperiod, idlecallbackfn, idlecallbackcontext, ifgroup as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcServerInterfaceGroupCreateW(interfaces: &[RPC_INTERFACE_TEMPLATEW], endpoints: &[RPC_ENDPOINT_TEMPLATEW], idleperiod: u32, idlecallbackfn: *const core::ffi::c_void, idlecallbackcontext: *const core::ffi::c_void, ifgroup: *mut *mut core::ffi::c_void) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerInterfaceGroupCreateW(interfaces : *const RPC_INTERFACE_TEMPLATEW, numifs : u32, endpoints : *const RPC_ENDPOINT_TEMPLATEW, numendpoints : u32, idleperiod : u32, idlecallbackfn : *const core::ffi::c_void, idlecallbackcontext : *const core::ffi::c_void, ifgroup : *mut *mut core::ffi::c_void) -> super::rpc::RPC_STATUS);
    unsafe { RpcServerInterfaceGroupCreateW(core::mem::transmute(interfaces.as_ptr()), interfaces.len().try_into().unwrap(), core::mem::transmute(endpoints.as_ptr()), endpoints.len().try_into().unwrap(), idleperiod, idlecallbackfn, idlecallbackcontext, ifgroup as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcServerInterfaceGroupDeactivate(ifgroup: RPC_INTERFACE_GROUP, forcedeactivation: u32) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerInterfaceGroupDeactivate(ifgroup : RPC_INTERFACE_GROUP, forcedeactivation : u32) -> super::rpc::RPC_STATUS);
    unsafe { RpcServerInterfaceGroupDeactivate(ifgroup, forcedeactivation) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcServerInterfaceGroupInqBindings(ifgroup: RPC_INTERFACE_GROUP, bindingvector: *mut *mut RPC_BINDING_VECTOR) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerInterfaceGroupInqBindings(ifgroup : RPC_INTERFACE_GROUP, bindingvector : *mut *mut RPC_BINDING_VECTOR) -> super::rpc::RPC_STATUS);
    unsafe { RpcServerInterfaceGroupInqBindings(ifgroup, bindingvector as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcServerListen(minimumcallthreads: u32, maxcalls: u32, dontwait: u32) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerListen(minimumcallthreads : u32, maxcalls : u32, dontwait : u32) -> super::rpc::RPC_STATUS);
    unsafe { RpcServerListen(minimumcallthreads, maxcalls, dontwait) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcServerRegisterAuthInfoA(serverprincname: Option<*const u8>, authnsvc: u32, getkeyfn: RPC_AUTH_KEY_RETRIEVAL_FN, arg: Option<*const core::ffi::c_void>) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerRegisterAuthInfoA(serverprincname : *const u8, authnsvc : u32, getkeyfn : RPC_AUTH_KEY_RETRIEVAL_FN, arg : *const core::ffi::c_void) -> super::rpc::RPC_STATUS);
    unsafe { RpcServerRegisterAuthInfoA(serverprincname.unwrap_or(core::mem::zeroed()) as _, authnsvc, getkeyfn, arg.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcServerRegisterAuthInfoW(serverprincname: Option<*const u16>, authnsvc: u32, getkeyfn: RPC_AUTH_KEY_RETRIEVAL_FN, arg: Option<*const core::ffi::c_void>) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerRegisterAuthInfoW(serverprincname : *const u16, authnsvc : u32, getkeyfn : RPC_AUTH_KEY_RETRIEVAL_FN, arg : *const core::ffi::c_void) -> super::rpc::RPC_STATUS);
    unsafe { RpcServerRegisterAuthInfoW(serverprincname.unwrap_or(core::mem::zeroed()) as _, authnsvc, getkeyfn, arg.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcServerRegisterIf(ifspec: RPC_IF_HANDLE, mgrtypeuuid: Option<*const windows_core::GUID>, mgrepv: Option<*const core::ffi::c_void>) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerRegisterIf(ifspec : RPC_IF_HANDLE, mgrtypeuuid : *const windows_core::GUID, mgrepv : *const core::ffi::c_void) -> super::rpc::RPC_STATUS);
    unsafe { RpcServerRegisterIf(ifspec, mgrtypeuuid.unwrap_or(core::mem::zeroed()) as _, mgrepv.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcServerRegisterIf2(ifspec: RPC_IF_HANDLE, mgrtypeuuid: Option<*const windows_core::GUID>, mgrepv: Option<*const core::ffi::c_void>, flags: u32, maxcalls: u32, maxrpcsize: u32, ifcallbackfn: RPC_IF_CALLBACK_FN) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerRegisterIf2(ifspec : RPC_IF_HANDLE, mgrtypeuuid : *const windows_core::GUID, mgrepv : *const core::ffi::c_void, flags : u32, maxcalls : u32, maxrpcsize : u32, ifcallbackfn : RPC_IF_CALLBACK_FN) -> super::rpc::RPC_STATUS);
    unsafe { RpcServerRegisterIf2(ifspec, mgrtypeuuid.unwrap_or(core::mem::zeroed()) as _, mgrepv.unwrap_or(core::mem::zeroed()) as _, flags, maxcalls, maxrpcsize, ifcallbackfn) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcServerRegisterIf3(ifspec: RPC_IF_HANDLE, mgrtypeuuid: Option<*const windows_core::GUID>, mgrepv: Option<*const core::ffi::c_void>, flags: u32, maxcalls: u32, maxrpcsize: u32, ifcallback: RPC_IF_CALLBACK_FN, securitydescriptor: Option<*const core::ffi::c_void>) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerRegisterIf3(ifspec : RPC_IF_HANDLE, mgrtypeuuid : *const windows_core::GUID, mgrepv : *const core::ffi::c_void, flags : u32, maxcalls : u32, maxrpcsize : u32, ifcallback : RPC_IF_CALLBACK_FN, securitydescriptor : *const core::ffi::c_void) -> super::rpc::RPC_STATUS);
    unsafe { RpcServerRegisterIf3(ifspec, mgrtypeuuid.unwrap_or(core::mem::zeroed()) as _, mgrepv.unwrap_or(core::mem::zeroed()) as _, flags, maxcalls, maxrpcsize, ifcallback, securitydescriptor.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcServerRegisterIfEx(ifspec: RPC_IF_HANDLE, mgrtypeuuid: Option<*const windows_core::GUID>, mgrepv: Option<*const core::ffi::c_void>, flags: u32, maxcalls: u32, ifcallback: RPC_IF_CALLBACK_FN) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerRegisterIfEx(ifspec : RPC_IF_HANDLE, mgrtypeuuid : *const windows_core::GUID, mgrepv : *const core::ffi::c_void, flags : u32, maxcalls : u32, ifcallback : RPC_IF_CALLBACK_FN) -> super::rpc::RPC_STATUS);
    unsafe { RpcServerRegisterIfEx(ifspec, mgrtypeuuid.unwrap_or(core::mem::zeroed()) as _, mgrepv.unwrap_or(core::mem::zeroed()) as _, flags, maxcalls, ifcallback) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcServerTestCancel(bindinghandle: Option<RPC_BINDING_HANDLE>) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerTestCancel(bindinghandle : RPC_BINDING_HANDLE) -> super::rpc::RPC_STATUS);
    unsafe { RpcServerTestCancel(bindinghandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcServerUnregisterIf(ifspec: Option<RPC_IF_HANDLE>, mgrtypeuuid: Option<*const windows_core::GUID>, waitforcallstocomplete: u32) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUnregisterIf(ifspec : RPC_IF_HANDLE, mgrtypeuuid : *const windows_core::GUID, waitforcallstocomplete : u32) -> super::rpc::RPC_STATUS);
    unsafe { RpcServerUnregisterIf(ifspec.unwrap_or(core::mem::zeroed()) as _, mgrtypeuuid.unwrap_or(core::mem::zeroed()) as _, waitforcallstocomplete) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcServerUnregisterIfEx(ifspec: Option<RPC_IF_HANDLE>, mgrtypeuuid: Option<*const windows_core::GUID>, rundowncontexthandles: i32) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUnregisterIfEx(ifspec : RPC_IF_HANDLE, mgrtypeuuid : *const windows_core::GUID, rundowncontexthandles : i32) -> super::rpc::RPC_STATUS);
    unsafe { RpcServerUnregisterIfEx(ifspec.unwrap_or(core::mem::zeroed()) as _, mgrtypeuuid.unwrap_or(core::mem::zeroed()) as _, rundowncontexthandles) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcServerUseAllProtseqs(maxcalls: u32, securitydescriptor: Option<*const core::ffi::c_void>) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUseAllProtseqs(maxcalls : u32, securitydescriptor : *const core::ffi::c_void) -> super::rpc::RPC_STATUS);
    unsafe { RpcServerUseAllProtseqs(maxcalls, securitydescriptor.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcServerUseAllProtseqsEx(maxcalls: u32, securitydescriptor: Option<*const core::ffi::c_void>, policy: *const RPC_POLICY) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUseAllProtseqsEx(maxcalls : u32, securitydescriptor : *const core::ffi::c_void, policy : *const RPC_POLICY) -> super::rpc::RPC_STATUS);
    unsafe { RpcServerUseAllProtseqsEx(maxcalls, securitydescriptor.unwrap_or(core::mem::zeroed()) as _, policy) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcServerUseAllProtseqsIf(maxcalls: u32, ifspec: RPC_IF_HANDLE, securitydescriptor: Option<*const core::ffi::c_void>) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUseAllProtseqsIf(maxcalls : u32, ifspec : RPC_IF_HANDLE, securitydescriptor : *const core::ffi::c_void) -> super::rpc::RPC_STATUS);
    unsafe { RpcServerUseAllProtseqsIf(maxcalls, ifspec, securitydescriptor.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcServerUseAllProtseqsIfEx(maxcalls: u32, ifspec: RPC_IF_HANDLE, securitydescriptor: Option<*const core::ffi::c_void>, policy: *const RPC_POLICY) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUseAllProtseqsIfEx(maxcalls : u32, ifspec : RPC_IF_HANDLE, securitydescriptor : *const core::ffi::c_void, policy : *const RPC_POLICY) -> super::rpc::RPC_STATUS);
    unsafe { RpcServerUseAllProtseqsIfEx(maxcalls, ifspec, securitydescriptor.unwrap_or(core::mem::zeroed()) as _, policy) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcServerUseProtseqA(protseq: *const u8, maxcalls: u32, securitydescriptor: Option<*const core::ffi::c_void>) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUseProtseqA(protseq : *const u8, maxcalls : u32, securitydescriptor : *const core::ffi::c_void) -> super::rpc::RPC_STATUS);
    unsafe { RpcServerUseProtseqA(protseq, maxcalls, securitydescriptor.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcServerUseProtseqEpA(protseq: *const u8, maxcalls: u32, endpoint: *const u8, securitydescriptor: Option<*const core::ffi::c_void>) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUseProtseqEpA(protseq : *const u8, maxcalls : u32, endpoint : *const u8, securitydescriptor : *const core::ffi::c_void) -> super::rpc::RPC_STATUS);
    unsafe { RpcServerUseProtseqEpA(protseq, maxcalls, endpoint, securitydescriptor.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcServerUseProtseqEpExA(protseq: *const u8, maxcalls: u32, endpoint: *const u8, securitydescriptor: Option<*const core::ffi::c_void>, policy: *const RPC_POLICY) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUseProtseqEpExA(protseq : *const u8, maxcalls : u32, endpoint : *const u8, securitydescriptor : *const core::ffi::c_void, policy : *const RPC_POLICY) -> super::rpc::RPC_STATUS);
    unsafe { RpcServerUseProtseqEpExA(protseq, maxcalls, endpoint, securitydescriptor.unwrap_or(core::mem::zeroed()) as _, policy) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcServerUseProtseqEpExW(protseq: *const u16, maxcalls: u32, endpoint: *const u16, securitydescriptor: Option<*const core::ffi::c_void>, policy: *const RPC_POLICY) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUseProtseqEpExW(protseq : *const u16, maxcalls : u32, endpoint : *const u16, securitydescriptor : *const core::ffi::c_void, policy : *const RPC_POLICY) -> super::rpc::RPC_STATUS);
    unsafe { RpcServerUseProtseqEpExW(protseq, maxcalls, endpoint, securitydescriptor.unwrap_or(core::mem::zeroed()) as _, policy) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcServerUseProtseqEpW(protseq: *const u16, maxcalls: u32, endpoint: *const u16, securitydescriptor: Option<*const core::ffi::c_void>) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUseProtseqEpW(protseq : *const u16, maxcalls : u32, endpoint : *const u16, securitydescriptor : *const core::ffi::c_void) -> super::rpc::RPC_STATUS);
    unsafe { RpcServerUseProtseqEpW(protseq, maxcalls, endpoint, securitydescriptor.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcServerUseProtseqExA(protseq: *const u8, maxcalls: u32, securitydescriptor: Option<*const core::ffi::c_void>, policy: *const RPC_POLICY) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUseProtseqExA(protseq : *const u8, maxcalls : u32, securitydescriptor : *const core::ffi::c_void, policy : *const RPC_POLICY) -> super::rpc::RPC_STATUS);
    unsafe { RpcServerUseProtseqExA(protseq, maxcalls, securitydescriptor.unwrap_or(core::mem::zeroed()) as _, policy) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcServerUseProtseqExW(protseq: *const u16, maxcalls: u32, securitydescriptor: Option<*const core::ffi::c_void>, policy: *const RPC_POLICY) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUseProtseqExW(protseq : *const u16, maxcalls : u32, securitydescriptor : *const core::ffi::c_void, policy : *const RPC_POLICY) -> super::rpc::RPC_STATUS);
    unsafe { RpcServerUseProtseqExW(protseq, maxcalls, securitydescriptor.unwrap_or(core::mem::zeroed()) as _, policy) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcServerUseProtseqIfA(protseq: *const u8, maxcalls: u32, ifspec: RPC_IF_HANDLE, securitydescriptor: Option<*const core::ffi::c_void>) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUseProtseqIfA(protseq : *const u8, maxcalls : u32, ifspec : RPC_IF_HANDLE, securitydescriptor : *const core::ffi::c_void) -> super::rpc::RPC_STATUS);
    unsafe { RpcServerUseProtseqIfA(protseq, maxcalls, ifspec, securitydescriptor.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcServerUseProtseqIfExA(protseq: *const u8, maxcalls: u32, ifspec: RPC_IF_HANDLE, securitydescriptor: Option<*const core::ffi::c_void>, policy: *const RPC_POLICY) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUseProtseqIfExA(protseq : *const u8, maxcalls : u32, ifspec : RPC_IF_HANDLE, securitydescriptor : *const core::ffi::c_void, policy : *const RPC_POLICY) -> super::rpc::RPC_STATUS);
    unsafe { RpcServerUseProtseqIfExA(protseq, maxcalls, ifspec, securitydescriptor.unwrap_or(core::mem::zeroed()) as _, policy) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcServerUseProtseqIfExW(protseq: *const u16, maxcalls: u32, ifspec: RPC_IF_HANDLE, securitydescriptor: Option<*const core::ffi::c_void>, policy: *const RPC_POLICY) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUseProtseqIfExW(protseq : *const u16, maxcalls : u32, ifspec : RPC_IF_HANDLE, securitydescriptor : *const core::ffi::c_void, policy : *const RPC_POLICY) -> super::rpc::RPC_STATUS);
    unsafe { RpcServerUseProtseqIfExW(protseq, maxcalls, ifspec, securitydescriptor.unwrap_or(core::mem::zeroed()) as _, policy) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcServerUseProtseqIfW(protseq: *const u16, maxcalls: u32, ifspec: RPC_IF_HANDLE, securitydescriptor: Option<*const core::ffi::c_void>) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUseProtseqIfW(protseq : *const u16, maxcalls : u32, ifspec : RPC_IF_HANDLE, securitydescriptor : *const core::ffi::c_void) -> super::rpc::RPC_STATUS);
    unsafe { RpcServerUseProtseqIfW(protseq, maxcalls, ifspec, securitydescriptor.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcServerUseProtseqW(protseq: *const u16, maxcalls: u32, securitydescriptor: Option<*const core::ffi::c_void>) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerUseProtseqW(protseq : *const u16, maxcalls : u32, securitydescriptor : *const core::ffi::c_void) -> super::rpc::RPC_STATUS);
    unsafe { RpcServerUseProtseqW(protseq, maxcalls, securitydescriptor.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn RpcServerYield() {
    windows_core::link!("rpcrt4.dll" "system" fn RpcServerYield());
    unsafe { RpcServerYield() }
}
#[inline]
pub unsafe fn RpcSsDontSerializeContext() {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSsDontSerializeContext());
    unsafe { RpcSsDontSerializeContext() }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcSsGetContextBinding(contexthandle: *const core::ffi::c_void, binding: *mut RPC_BINDING_HANDLE) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSsGetContextBinding(contexthandle : *const core::ffi::c_void, binding : *mut RPC_BINDING_HANDLE) -> super::rpc::RPC_STATUS);
    unsafe { RpcSsGetContextBinding(contexthandle, binding as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcStringBindingComposeA(objuuid: Option<*const u8>, protseq: Option<*const u8>, networkaddr: Option<*const u8>, endpoint: Option<*const u8>, options: Option<*const u8>, stringbinding: *mut RPC_CSTR) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcStringBindingComposeA(objuuid : *const u8, protseq : *const u8, networkaddr : *const u8, endpoint : *const u8, options : *const u8, stringbinding : *mut RPC_CSTR) -> super::rpc::RPC_STATUS);
    unsafe { RpcStringBindingComposeA(objuuid.unwrap_or(core::mem::zeroed()) as _, protseq.unwrap_or(core::mem::zeroed()) as _, networkaddr.unwrap_or(core::mem::zeroed()) as _, endpoint.unwrap_or(core::mem::zeroed()) as _, options.unwrap_or(core::mem::zeroed()) as _, stringbinding as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcStringBindingComposeW(objuuid: Option<*const u16>, protseq: Option<*const u16>, networkaddr: Option<*const u16>, endpoint: Option<*const u16>, options: Option<*const u16>, stringbinding: *mut RPC_WSTR) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcStringBindingComposeW(objuuid : *const u16, protseq : *const u16, networkaddr : *const u16, endpoint : *const u16, options : *const u16, stringbinding : *mut RPC_WSTR) -> super::rpc::RPC_STATUS);
    unsafe { RpcStringBindingComposeW(objuuid.unwrap_or(core::mem::zeroed()) as _, protseq.unwrap_or(core::mem::zeroed()) as _, networkaddr.unwrap_or(core::mem::zeroed()) as _, endpoint.unwrap_or(core::mem::zeroed()) as _, options.unwrap_or(core::mem::zeroed()) as _, stringbinding as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcStringBindingParseA(stringbinding: *const u8, objuuid: *mut RPC_CSTR, protseq: *mut RPC_CSTR, networkaddr: *mut RPC_CSTR, endpoint: *mut RPC_CSTR, networkoptions: *mut RPC_CSTR) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcStringBindingParseA(stringbinding : *const u8, objuuid : *mut RPC_CSTR, protseq : *mut RPC_CSTR, networkaddr : *mut RPC_CSTR, endpoint : *mut RPC_CSTR, networkoptions : *mut RPC_CSTR) -> super::rpc::RPC_STATUS);
    unsafe { RpcStringBindingParseA(stringbinding, objuuid as _, protseq as _, networkaddr as _, endpoint as _, networkoptions as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcStringBindingParseW(stringbinding: *const u16, objuuid: *mut RPC_WSTR, protseq: *mut RPC_WSTR, networkaddr: *mut RPC_WSTR, endpoint: *mut RPC_WSTR, networkoptions: *mut RPC_WSTR) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcStringBindingParseW(stringbinding : *const u16, objuuid : *mut RPC_WSTR, protseq : *mut RPC_WSTR, networkaddr : *mut RPC_WSTR, endpoint : *mut RPC_WSTR, networkoptions : *mut RPC_WSTR) -> super::rpc::RPC_STATUS);
    unsafe { RpcStringBindingParseW(stringbinding, objuuid as _, protseq as _, networkaddr as _, endpoint as _, networkoptions as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcStringFreeA(string: *mut RPC_CSTR) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcStringFreeA(string : *mut RPC_CSTR) -> super::rpc::RPC_STATUS);
    unsafe { RpcStringFreeA(string as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcStringFreeW(string: *mut RPC_WSTR) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcStringFreeW(string : *mut RPC_WSTR) -> super::rpc::RPC_STATUS);
    unsafe { RpcStringFreeW(string as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcTestCancel() -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcTestCancel() -> super::rpc::RPC_STATUS);
    unsafe { RpcTestCancel() }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn UuidCompare(uuid1: *const windows_core::GUID, uuid2: *const windows_core::GUID, status: *mut super::rpc::RPC_STATUS) -> i32 {
    windows_core::link!("rpcrt4.dll" "system" fn UuidCompare(uuid1 : *const windows_core::GUID, uuid2 : *const windows_core::GUID, status : *mut super::rpc::RPC_STATUS) -> i32);
    unsafe { UuidCompare(uuid1, uuid2, status as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn UuidCreate(uuid: *mut windows_core::GUID) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn UuidCreate(uuid : *mut windows_core::GUID) -> super::rpc::RPC_STATUS);
    unsafe { UuidCreate(uuid as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn UuidCreateNil(niluuid: *mut windows_core::GUID) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn UuidCreateNil(niluuid : *mut windows_core::GUID) -> super::rpc::RPC_STATUS);
    unsafe { UuidCreateNil(niluuid as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn UuidCreateSequential(uuid: *mut windows_core::GUID) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn UuidCreateSequential(uuid : *mut windows_core::GUID) -> super::rpc::RPC_STATUS);
    unsafe { UuidCreateSequential(uuid as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn UuidEqual(uuid1: *const windows_core::GUID, uuid2: *const windows_core::GUID, status: *mut super::rpc::RPC_STATUS) -> i32 {
    windows_core::link!("rpcrt4.dll" "system" fn UuidEqual(uuid1 : *const windows_core::GUID, uuid2 : *const windows_core::GUID, status : *mut super::rpc::RPC_STATUS) -> i32);
    unsafe { UuidEqual(uuid1, uuid2, status as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn UuidFromStringA(stringuuid: Option<*const u8>, uuid: *mut windows_core::GUID) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn UuidFromStringA(stringuuid : *const u8, uuid : *mut windows_core::GUID) -> super::rpc::RPC_STATUS);
    unsafe { UuidFromStringA(stringuuid.unwrap_or(core::mem::zeroed()) as _, uuid as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn UuidFromStringW(stringuuid: Option<*const u16>, uuid: *mut windows_core::GUID) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn UuidFromStringW(stringuuid : *const u16, uuid : *mut windows_core::GUID) -> super::rpc::RPC_STATUS);
    unsafe { UuidFromStringW(stringuuid.unwrap_or(core::mem::zeroed()) as _, uuid as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn UuidHash(uuid: *const windows_core::GUID, status: *mut super::rpc::RPC_STATUS) -> u16 {
    windows_core::link!("rpcrt4.dll" "system" fn UuidHash(uuid : *const windows_core::GUID, status : *mut super::rpc::RPC_STATUS) -> u16);
    unsafe { UuidHash(uuid, status as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn UuidIsNil(uuid: *const windows_core::GUID, status: *mut super::rpc::RPC_STATUS) -> i32 {
    windows_core::link!("rpcrt4.dll" "system" fn UuidIsNil(uuid : *const windows_core::GUID, status : *mut super::rpc::RPC_STATUS) -> i32);
    unsafe { UuidIsNil(uuid, status as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn UuidToStringA(uuid: *const windows_core::GUID, stringuuid: *mut RPC_CSTR) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn UuidToStringA(uuid : *const windows_core::GUID, stringuuid : *mut RPC_CSTR) -> super::rpc::RPC_STATUS);
    unsafe { UuidToStringA(uuid, stringuuid as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn UuidToStringW(uuid: *const windows_core::GUID, stringuuid: *mut RPC_WSTR) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn UuidToStringW(uuid : *const windows_core::GUID, stringuuid : *mut RPC_WSTR) -> super::rpc::RPC_STATUS);
    unsafe { UuidToStringW(uuid, stringuuid as _) }
}
pub const DCE_C_ERROR_STRING_LEN: u32 = 256;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRPC_BINDING_HANDLE_OPTIONS_V1(pub *mut RPC_BINDING_HANDLE_OPTIONS_V1);
impl PRPC_BINDING_HANDLE_OPTIONS_V1 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRPC_BINDING_HANDLE_OPTIONS_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRPC_BINDING_HANDLE_SECURITY_V1_A(pub *mut RPC_BINDING_HANDLE_SECURITY_V1_A);
impl PRPC_BINDING_HANDLE_SECURITY_V1_A {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRPC_BINDING_HANDLE_SECURITY_V1_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRPC_BINDING_HANDLE_SECURITY_V1_W(pub *mut RPC_BINDING_HANDLE_SECURITY_V1_W);
impl PRPC_BINDING_HANDLE_SECURITY_V1_W {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRPC_BINDING_HANDLE_SECURITY_V1_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRPC_BINDING_HANDLE_TEMPLATE_V1_A(pub *mut RPC_BINDING_HANDLE_TEMPLATE_V1_A);
impl PRPC_BINDING_HANDLE_TEMPLATE_V1_A {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRPC_BINDING_HANDLE_TEMPLATE_V1_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRPC_BINDING_HANDLE_TEMPLATE_V1_W(pub *mut RPC_BINDING_HANDLE_TEMPLATE_V1_W);
impl PRPC_BINDING_HANDLE_TEMPLATE_V1_W {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRPC_BINDING_HANDLE_TEMPLATE_V1_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRPC_CLIENT_INFORMATION1(pub *mut RPC_CLIENT_INFORMATION1);
impl PRPC_CLIENT_INFORMATION1 {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRPC_CLIENT_INFORMATION1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRPC_ENDPOINT_TEMPLATEA(pub *mut RPC_ENDPOINT_TEMPLATEA);
impl PRPC_ENDPOINT_TEMPLATEA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRPC_ENDPOINT_TEMPLATEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRPC_ENDPOINT_TEMPLATEW(pub *mut RPC_ENDPOINT_TEMPLATEW);
impl PRPC_ENDPOINT_TEMPLATEW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRPC_ENDPOINT_TEMPLATEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRPC_HTTP_TRANSPORT_CREDENTIALS_A(pub *mut RPC_HTTP_TRANSPORT_CREDENTIALS_A);
impl PRPC_HTTP_TRANSPORT_CREDENTIALS_A {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRPC_HTTP_TRANSPORT_CREDENTIALS_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRPC_HTTP_TRANSPORT_CREDENTIALS_V2_A(pub *mut RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A);
impl PRPC_HTTP_TRANSPORT_CREDENTIALS_V2_A {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRPC_HTTP_TRANSPORT_CREDENTIALS_V2_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRPC_HTTP_TRANSPORT_CREDENTIALS_V2_W(pub *mut RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W);
impl PRPC_HTTP_TRANSPORT_CREDENTIALS_V2_W {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRPC_HTTP_TRANSPORT_CREDENTIALS_V2_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRPC_HTTP_TRANSPORT_CREDENTIALS_V3_A(pub *mut RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A);
impl PRPC_HTTP_TRANSPORT_CREDENTIALS_V3_A {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRPC_HTTP_TRANSPORT_CREDENTIALS_V3_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRPC_HTTP_TRANSPORT_CREDENTIALS_V3_W(pub *mut RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W);
impl PRPC_HTTP_TRANSPORT_CREDENTIALS_V3_W {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRPC_HTTP_TRANSPORT_CREDENTIALS_V3_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRPC_HTTP_TRANSPORT_CREDENTIALS_W(pub *mut RPC_HTTP_TRANSPORT_CREDENTIALS_W);
impl PRPC_HTTP_TRANSPORT_CREDENTIALS_W {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRPC_HTTP_TRANSPORT_CREDENTIALS_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRPC_INTERFACE_GROUP(pub *mut *mut core::ffi::c_void);
impl PRPC_INTERFACE_GROUP {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRPC_INTERFACE_GROUP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_rpc")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRPC_INTERFACE_TEMPLATEA(pub *mut RPC_INTERFACE_TEMPLATEA);
#[cfg(feature = "Win32_rpc")]
impl PRPC_INTERFACE_TEMPLATEA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_rpc")]
impl Default for PRPC_INTERFACE_TEMPLATEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_rpc")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRPC_INTERFACE_TEMPLATEW(pub *mut RPC_INTERFACE_TEMPLATEW);
#[cfg(feature = "Win32_rpc")]
impl PRPC_INTERFACE_TEMPLATEW {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_rpc")]
impl Default for PRPC_INTERFACE_TEMPLATEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRPC_POLICY(pub *mut RPC_POLICY);
impl PRPC_POLICY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRPC_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRPC_SECURITY_QOS(pub *mut RPC_SECURITY_QOS);
impl PRPC_SECURITY_QOS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRPC_SECURITY_QOS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRPC_SECURITY_QOS_V2_A(pub *mut RPC_SECURITY_QOS_V2_A);
impl PRPC_SECURITY_QOS_V2_A {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRPC_SECURITY_QOS_V2_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRPC_SECURITY_QOS_V2_W(pub *mut RPC_SECURITY_QOS_V2_W);
impl PRPC_SECURITY_QOS_V2_W {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRPC_SECURITY_QOS_V2_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRPC_SECURITY_QOS_V3_A(pub *mut RPC_SECURITY_QOS_V3_A);
impl PRPC_SECURITY_QOS_V3_A {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRPC_SECURITY_QOS_V3_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRPC_SECURITY_QOS_V3_W(pub *mut RPC_SECURITY_QOS_V3_W);
impl PRPC_SECURITY_QOS_V3_W {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRPC_SECURITY_QOS_V3_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRPC_SECURITY_QOS_V4_A(pub *mut RPC_SECURITY_QOS_V4_A);
impl PRPC_SECURITY_QOS_V4_A {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRPC_SECURITY_QOS_V4_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRPC_SECURITY_QOS_V4_W(pub *mut RPC_SECURITY_QOS_V4_W);
impl PRPC_SECURITY_QOS_V4_W {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRPC_SECURITY_QOS_V4_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRPC_SECURITY_QOS_V5_A(pub *mut RPC_SECURITY_QOS_V5_A);
impl PRPC_SECURITY_QOS_V5_A {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRPC_SECURITY_QOS_V5_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRPC_SECURITY_QOS_V5_W(pub *mut RPC_SECURITY_QOS_V5_W);
impl PRPC_SECURITY_QOS_V5_W {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PRPC_SECURITY_QOS_V5_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSEC_WINNT_AUTH_IDENTITY_A(pub *mut SEC_WINNT_AUTH_IDENTITY_A);
impl PSEC_WINNT_AUTH_IDENTITY_A {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSEC_WINNT_AUTH_IDENTITY_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSEC_WINNT_AUTH_IDENTITY_W(pub *mut SEC_WINNT_AUTH_IDENTITY_W);
impl PSEC_WINNT_AUTH_IDENTITY_W {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSEC_WINNT_AUTH_IDENTITY_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RPCHTTP_RS_ACCESS_1: RPC_HTTP_REDIRECTOR_STAGE = 2;
pub const RPCHTTP_RS_ACCESS_2: RPC_HTTP_REDIRECTOR_STAGE = 4;
pub const RPCHTTP_RS_INTERFACE: RPC_HTTP_REDIRECTOR_STAGE = 5;
pub const RPCHTTP_RS_REDIRECT: RPC_HTTP_REDIRECTOR_STAGE = 1;
pub const RPCHTTP_RS_SESSION: RPC_HTTP_REDIRECTOR_STAGE = 3;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RPC_AUTHZ_HANDLE(pub *mut core::ffi::c_void);
impl RPC_AUTHZ_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for RPC_AUTHZ_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RPC_AUTH_IDENTITY_HANDLE(pub *mut core::ffi::c_void);
impl RPC_AUTH_IDENTITY_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for RPC_AUTH_IDENTITY_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_rpc")]
pub type RPC_AUTH_KEY_RETRIEVAL_FN = Option<unsafe extern "system" fn(arg: *const core::ffi::c_void, serverprincname: *const u16, keyver: u32, key: *mut *mut core::ffi::c_void, status: *mut super::rpc::RPC_STATUS)>;
pub const RPC_BHO_DONTLINGER: u32 = 2;
pub const RPC_BHO_EXCLUSIVE_AND_GUARANTEED: u32 = 4;
pub const RPC_BHO_NONCAUSAL: u32 = 1;
pub const RPC_BHT_OBJECT_UUID_VALID: u32 = 1;
#[cfg(feature = "Win32_rpc")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct RPC_BINDING_HANDLE(pub super::rpc::I_RPC_HANDLE);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RPC_BINDING_HANDLE_OPTIONS_V1 {
    pub Version: u32,
    pub Flags: u32,
    pub ComTimeout: u32,
    pub CallTimeout: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_BINDING_HANDLE_SECURITY_V1_A {
    pub Version: u32,
    pub ServerPrincName: *mut u8,
    pub AuthnLevel: u32,
    pub AuthnSvc: u32,
    pub AuthIdentity: *mut SEC_WINNT_AUTH_IDENTITY_A,
    pub SecurityQos: *mut RPC_SECURITY_QOS,
}
impl Default for RPC_BINDING_HANDLE_SECURITY_V1_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_BINDING_HANDLE_SECURITY_V1_W {
    pub Version: u32,
    pub ServerPrincName: *mut u16,
    pub AuthnLevel: u32,
    pub AuthnSvc: u32,
    pub AuthIdentity: *mut SEC_WINNT_AUTH_IDENTITY_W,
    pub SecurityQos: *mut RPC_SECURITY_QOS,
}
impl Default for RPC_BINDING_HANDLE_SECURITY_V1_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RPC_BINDING_HANDLE_TEMPLATE_V1_A {
    pub Version: u32,
    pub Flags: u32,
    pub ProtocolSequence: u32,
    pub NetworkAddress: *mut u8,
    pub StringEndpoint: *mut u8,
    pub u1: RPC_BINDING_HANDLE_TEMPLATE_V1_A_0,
    pub ObjectUuid: windows_core::GUID,
}
impl Default for RPC_BINDING_HANDLE_TEMPLATE_V1_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union RPC_BINDING_HANDLE_TEMPLATE_V1_A_0 {
    pub Reserved: *mut u8,
}
impl Default for RPC_BINDING_HANDLE_TEMPLATE_V1_A_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RPC_BINDING_HANDLE_TEMPLATE_V1_W {
    pub Version: u32,
    pub Flags: u32,
    pub ProtocolSequence: u32,
    pub NetworkAddress: *mut u16,
    pub StringEndpoint: *mut u16,
    pub u1: RPC_BINDING_HANDLE_TEMPLATE_V1_W_0,
    pub ObjectUuid: windows_core::GUID,
}
impl Default for RPC_BINDING_HANDLE_TEMPLATE_V1_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union RPC_BINDING_HANDLE_TEMPLATE_V1_W_0 {
    pub Reserved: *mut u16,
}
impl Default for RPC_BINDING_HANDLE_TEMPLATE_V1_W_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_rpc")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_BINDING_VECTOR {
    pub Count: u32,
    pub BindingH: [RPC_BINDING_HANDLE; 1],
}
#[cfg(feature = "Win32_rpc")]
impl Default for RPC_BINDING_VECTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_CLIENT_INFORMATION1 {
    pub UserName: *mut u8,
    pub ComputerName: *mut u8,
    pub Privilege: u16,
    pub AuthFlags: u32,
}
impl Default for RPC_CLIENT_INFORMATION1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RPC_CSTR(pub *mut u8);
impl RPC_CSTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for RPC_CSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RPC_CWSTR(pub *const u16);
impl RPC_CWSTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for RPC_CWSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RPC_C_AUTHN_CLOUD_AP: u32 = 36;
pub const RPC_C_AUTHN_DCE_PRIVATE: u32 = 1;
pub const RPC_C_AUTHN_DCE_PUBLIC: u32 = 2;
pub const RPC_C_AUTHN_DEC_PUBLIC: u32 = 4;
pub const RPC_C_AUTHN_DEFAULT: u32 = 4294967295;
pub const RPC_C_AUTHN_DIGEST: u32 = 21;
pub const RPC_C_AUTHN_DPA: u32 = 17;
pub const RPC_C_AUTHN_GSS_KERBEROS: u32 = 16;
pub const RPC_C_AUTHN_GSS_NEGOTIATE: u32 = 9;
pub const RPC_C_AUTHN_GSS_SCHANNEL: u32 = 14;
pub const RPC_C_AUTHN_INFO_TYPE_HTTP: u32 = 1;
pub const RPC_C_AUTHN_KERNEL: u32 = 20;
pub const RPC_C_AUTHN_LEVEL_CALL: u32 = 3;
pub const RPC_C_AUTHN_LEVEL_CONNECT: u32 = 2;
pub const RPC_C_AUTHN_LEVEL_DEFAULT: u32 = 0;
pub const RPC_C_AUTHN_LEVEL_NONE: u32 = 1;
pub const RPC_C_AUTHN_LEVEL_PKT: u32 = 4;
pub const RPC_C_AUTHN_LEVEL_PKT_INTEGRITY: u32 = 5;
pub const RPC_C_AUTHN_LEVEL_PKT_PRIVACY: u32 = 6;
pub const RPC_C_AUTHN_LIVEXP_SSP: u32 = 35;
pub const RPC_C_AUTHN_LIVE_SSP: u32 = 32;
pub const RPC_C_AUTHN_MQ: u32 = 100;
pub const RPC_C_AUTHN_MSN: u32 = 18;
pub const RPC_C_AUTHN_MSONLINE: u32 = 82;
pub const RPC_C_AUTHN_NEGO_EXTENDER: u32 = 30;
pub const RPC_C_AUTHN_NONE: u32 = 0;
pub const RPC_C_AUTHN_PKU2U: u32 = 31;
pub const RPC_C_AUTHN_WINNT: u32 = 10;
pub const RPC_C_AUTHZ_DCE: u32 = 2;
pub const RPC_C_AUTHZ_DEFAULT: u32 = 4294967295;
pub const RPC_C_AUTHZ_NAME: u32 = 1;
pub const RPC_C_AUTHZ_NONE: u32 = 0;
pub const RPC_C_BINDING_DEFAULT_TIMEOUT: u32 = 5;
pub const RPC_C_BINDING_INFINITE_TIMEOUT: u32 = 10;
pub const RPC_C_BINDING_MAX_TIMEOUT: u32 = 9;
pub const RPC_C_BINDING_MIN_TIMEOUT: u32 = 0;
pub const RPC_C_BIND_TO_ALL_NICS: u32 = 1;
pub const RPC_C_CANCEL_INFINITE_TIMEOUT: i32 = -1;
pub const RPC_C_DONT_FAIL: u32 = 4;
pub const RPC_C_EP_ALL_ELTS: u32 = 0;
pub const RPC_C_EP_MATCH_BY_BOTH: u32 = 3;
pub const RPC_C_EP_MATCH_BY_IF: u32 = 1;
pub const RPC_C_EP_MATCH_BY_OBJ: u32 = 2;
pub const RPC_C_FULL_CERT_CHAIN: u32 = 1;
pub const RPC_C_HTTP_AUTHN_SCHEME_BASIC: u32 = 1;
pub const RPC_C_HTTP_AUTHN_SCHEME_CERT: u32 = 65536;
pub const RPC_C_HTTP_AUTHN_SCHEME_DIGEST: u32 = 8;
pub const RPC_C_HTTP_AUTHN_SCHEME_NEGOTIATE: u32 = 16;
pub const RPC_C_HTTP_AUTHN_SCHEME_NTLM: u32 = 2;
pub const RPC_C_HTTP_AUTHN_SCHEME_PASSPORT: u32 = 4;
pub const RPC_C_HTTP_AUTHN_TARGET_PROXY: u32 = 2;
pub const RPC_C_HTTP_AUTHN_TARGET_SERVER: u32 = 1;
pub const RPC_C_HTTP_FLAG_ENABLE_CERT_REVOCATION_CHECK: u32 = 16;
pub const RPC_C_HTTP_FLAG_IGNORE_CERT_CN_INVALID: u32 = 8;
pub const RPC_C_HTTP_FLAG_USE_FIRST_AUTH_SCHEME: u32 = 2;
pub const RPC_C_HTTP_FLAG_USE_SSL: u32 = 1;
pub const RPC_C_IMP_LEVEL_ANONYMOUS: u32 = 1;
pub const RPC_C_IMP_LEVEL_DEFAULT: u32 = 0;
pub const RPC_C_IMP_LEVEL_DELEGATE: u32 = 4;
pub const RPC_C_IMP_LEVEL_IDENTIFY: u32 = 2;
pub const RPC_C_IMP_LEVEL_IMPERSONATE: u32 = 3;
pub const RPC_C_LISTEN_MAX_CALLS_DEFAULT: u32 = 1234;
pub const RPC_C_MGMT_INQ_IF_IDS: u32 = 0;
pub const RPC_C_MGMT_INQ_PRINC_NAME: u32 = 1;
pub const RPC_C_MGMT_INQ_STATS: u32 = 2;
pub const RPC_C_MGMT_IS_SERVER_LISTEN: u32 = 3;
pub const RPC_C_MGMT_STOP_SERVER_LISTEN: u32 = 4;
pub const RPC_C_OPT_ASYNC_BLOCK: u32 = 15;
pub const RPC_C_OPT_BINDING_NONCAUSAL: u32 = 9;
pub const RPC_C_OPT_CALL_TIMEOUT: u32 = 12;
pub const RPC_C_OPT_DONT_LINGER: u32 = 13;
pub const RPC_C_OPT_MAX_OPTIONS: u32 = 17;
pub const RPC_C_OPT_OPTIMIZE_TIME: u32 = 16;
pub const RPC_C_OPT_SECURITY_CALLBACK: u32 = 10;
pub const RPC_C_OPT_TRANS_SEND_BUFFER_SIZE: u32 = 5;
pub const RPC_C_OPT_TRUST_PEER: u32 = 14;
pub const RPC_C_OPT_UNIQUE_BINDING: u32 = 11;
pub const RPC_C_PARM_BUFFER_LENGTH: u32 = 2;
pub const RPC_C_PARM_MAX_PACKET_LENGTH: u32 = 1;
pub const RPC_C_PROTECT_LEVEL_CALL: u32 = 3;
pub const RPC_C_PROTECT_LEVEL_CONNECT: u32 = 2;
pub const RPC_C_PROTECT_LEVEL_DEFAULT: u32 = 0;
pub const RPC_C_PROTECT_LEVEL_NONE: u32 = 1;
pub const RPC_C_PROTECT_LEVEL_PKT: u32 = 4;
pub const RPC_C_PROTECT_LEVEL_PKT_INTEGRITY: u32 = 5;
pub const RPC_C_PROTECT_LEVEL_PKT_PRIVACY: u32 = 6;
pub const RPC_C_PROTSEQ_MAX_REQS_DEFAULT: u32 = 10;
pub const RPC_C_QOS_CAPABILITIES_ANY_AUTHORITY: u32 = 4;
pub const RPC_C_QOS_CAPABILITIES_DEFAULT: u32 = 0;
pub const RPC_C_QOS_CAPABILITIES_IGNORE_DELEGATE_FAILURE: u32 = 8;
pub const RPC_C_QOS_CAPABILITIES_LOCAL_MA_HINT: u32 = 16;
pub const RPC_C_QOS_CAPABILITIES_MAKE_FULLSIC: u32 = 2;
pub const RPC_C_QOS_CAPABILITIES_MUTUAL_AUTH: u32 = 1;
pub const RPC_C_QOS_CAPABILITIES_SCHANNEL_FULL_AUTH_IDENTITY: u32 = 32;
pub const RPC_C_QOS_IDENTITY_DYNAMIC: u32 = 1;
pub const RPC_C_QOS_IDENTITY_STATIC: u32 = 0;
pub const RPC_C_RPCHTTP_USE_LOAD_BALANCE: u32 = 8;
pub const RPC_C_SECURITY_QOS_VERSION: u32 = 1;
pub const RPC_C_SECURITY_QOS_VERSION_1: u32 = 1;
pub const RPC_C_SECURITY_QOS_VERSION_2: u32 = 2;
pub const RPC_C_SECURITY_QOS_VERSION_3: u32 = 3;
pub const RPC_C_SECURITY_QOS_VERSION_4: u32 = 4;
pub const RPC_C_SECURITY_QOS_VERSION_5: u32 = 5;
pub const RPC_C_STATS_CALLS_IN: u32 = 0;
pub const RPC_C_STATS_CALLS_OUT: u32 = 1;
pub const RPC_C_STATS_PKTS_IN: u32 = 2;
pub const RPC_C_STATS_PKTS_OUT: u32 = 3;
pub const RPC_C_TRY_ENFORCE_MAX_CALLS: u32 = 16;
pub const RPC_C_USE_INTERNET_PORT: u32 = 1;
pub const RPC_C_USE_INTRANET_PORT: u32 = 2;
pub const RPC_C_VERS_ALL: u32 = 1;
pub const RPC_C_VERS_COMPATIBLE: u32 = 2;
pub const RPC_C_VERS_EXACT: u32 = 3;
pub const RPC_C_VERS_MAJOR_ONLY: u32 = 4;
pub const RPC_C_VERS_UPTO: u32 = 5;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_ENDPOINT_TEMPLATEA {
    pub Version: u32,
    pub ProtSeq: RPC_CSTR,
    pub Endpoint: RPC_CSTR,
    pub SecurityDescriptor: *mut core::ffi::c_void,
    pub Backlog: u32,
}
impl Default for RPC_ENDPOINT_TEMPLATEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_ENDPOINT_TEMPLATEW {
    pub Version: u32,
    pub ProtSeq: RPC_WSTR,
    pub Endpoint: RPC_WSTR,
    pub SecurityDescriptor: *mut core::ffi::c_void,
    pub Backlog: u32,
}
impl Default for RPC_ENDPOINT_TEMPLATEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_rpc")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RPC_EP_INQ_HANDLE(pub *mut super::rpc::I_RPC_HANDLE);
#[cfg(feature = "Win32_rpc")]
impl RPC_EP_INQ_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_rpc")]
impl Default for RPC_EP_INQ_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RPC_FW_IF_FLAG_DCOM: u32 = 1;
pub type RPC_HTTP_PROXY_FREE_STRING = Option<unsafe extern "system" fn(string: *const u16)>;
pub type RPC_HTTP_REDIRECTOR_STAGE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_A {
    pub TransportCredentials: *mut SEC_WINNT_AUTH_IDENTITY_A,
    pub Flags: u32,
    pub AuthenticationTarget: u32,
    pub NumberOfAuthnSchemes: u32,
    pub AuthnSchemes: *mut u32,
    pub ServerCertificateSubject: *mut u8,
}
impl Default for RPC_HTTP_TRANSPORT_CREDENTIALS_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A {
    pub TransportCredentials: *mut SEC_WINNT_AUTH_IDENTITY_A,
    pub Flags: u32,
    pub AuthenticationTarget: u32,
    pub NumberOfAuthnSchemes: u32,
    pub AuthnSchemes: *mut u32,
    pub ServerCertificateSubject: *mut u8,
    pub ProxyCredentials: *mut SEC_WINNT_AUTH_IDENTITY_A,
    pub NumberOfProxyAuthnSchemes: u32,
    pub ProxyAuthnSchemes: *mut u32,
}
impl Default for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W {
    pub TransportCredentials: *mut SEC_WINNT_AUTH_IDENTITY_W,
    pub Flags: u32,
    pub AuthenticationTarget: u32,
    pub NumberOfAuthnSchemes: u32,
    pub AuthnSchemes: *mut u32,
    pub ServerCertificateSubject: *mut u16,
    pub ProxyCredentials: *mut SEC_WINNT_AUTH_IDENTITY_W,
    pub NumberOfProxyAuthnSchemes: u32,
    pub ProxyAuthnSchemes: *mut u32,
}
impl Default for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A {
    pub TransportCredentials: RPC_AUTH_IDENTITY_HANDLE,
    pub Flags: u32,
    pub AuthenticationTarget: u32,
    pub NumberOfAuthnSchemes: u32,
    pub AuthnSchemes: *mut u32,
    pub ServerCertificateSubject: *mut u8,
    pub ProxyCredentials: RPC_AUTH_IDENTITY_HANDLE,
    pub NumberOfProxyAuthnSchemes: u32,
    pub ProxyAuthnSchemes: *mut u32,
}
impl Default for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W {
    pub TransportCredentials: RPC_AUTH_IDENTITY_HANDLE,
    pub Flags: u32,
    pub AuthenticationTarget: u32,
    pub NumberOfAuthnSchemes: u32,
    pub AuthnSchemes: *mut u32,
    pub ServerCertificateSubject: *mut u16,
    pub ProxyCredentials: RPC_AUTH_IDENTITY_HANDLE,
    pub NumberOfProxyAuthnSchemes: u32,
    pub ProxyAuthnSchemes: *mut u32,
}
impl Default for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_W {
    pub TransportCredentials: *mut SEC_WINNT_AUTH_IDENTITY_W,
    pub Flags: u32,
    pub AuthenticationTarget: u32,
    pub NumberOfAuthnSchemes: u32,
    pub AuthnSchemes: *mut u32,
    pub ServerCertificateSubject: *mut u16,
}
impl Default for RPC_HTTP_TRANSPORT_CREDENTIALS_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RPC_IF_ALLOW_CALLBACKS_WITH_NO_AUTH: u32 = 16;
pub const RPC_IF_ALLOW_LOCAL_ONLY: u32 = 32;
pub const RPC_IF_ALLOW_SECURE_ONLY: u32 = 8;
pub const RPC_IF_ALLOW_UNKNOWN_AUTHORITY: u32 = 4;
pub const RPC_IF_ASYNC_CALLBACK: u32 = 256;
pub const RPC_IF_AUTOLISTEN: u32 = 1;
#[cfg(feature = "Win32_rpc")]
pub type RPC_IF_CALLBACK_FN = Option<unsafe extern "system" fn(interfaceuuid: RPC_IF_HANDLE, context: *const core::ffi::c_void) -> super::rpc::RPC_STATUS>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RPC_IF_HANDLE(pub *mut core::ffi::c_void);
impl RPC_IF_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for RPC_IF_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RPC_IF_ID {
    pub Uuid: windows_core::GUID,
    pub VersMajor: u16,
    pub VersMinor: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_IF_ID_VECTOR {
    pub Count: u32,
    pub IfId: [*mut RPC_IF_ID; 1],
}
impl Default for RPC_IF_ID_VECTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RPC_IF_OLE: u32 = 2;
pub const RPC_IF_SEC_CACHE_PER_PROC: u32 = 128;
pub const RPC_IF_SEC_NO_CACHE: u32 = 64;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RPC_INTERFACE_GROUP(pub *mut core::ffi::c_void);
impl RPC_INTERFACE_GROUP {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for RPC_INTERFACE_GROUP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type RPC_INTERFACE_GROUP_IDLE_CALLBACK_FN = Option<unsafe extern "system" fn(ifgroup: RPC_INTERFACE_GROUP, idlecallbackcontext: *const core::ffi::c_void, isgroupidle: u32)>;
#[repr(C)]
#[cfg(feature = "Win32_rpc")]
#[derive(Clone, Copy, Debug)]
pub struct RPC_INTERFACE_TEMPLATEA {
    pub Version: u32,
    pub IfSpec: RPC_IF_HANDLE,
    pub MgrTypeUuid: *mut windows_core::GUID,
    pub MgrEpv: *mut core::ffi::c_void,
    pub Flags: u32,
    pub MaxCalls: u32,
    pub MaxRpcSize: u32,
    pub IfCallback: RPC_IF_CALLBACK_FN,
    pub UuidVector: *mut UUID_VECTOR,
    pub Annotation: RPC_CSTR,
    pub SecurityDescriptor: *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_rpc")]
impl Default for RPC_INTERFACE_TEMPLATEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_rpc")]
#[derive(Clone, Copy, Debug)]
pub struct RPC_INTERFACE_TEMPLATEW {
    pub Version: u32,
    pub IfSpec: RPC_IF_HANDLE,
    pub MgrTypeUuid: *mut windows_core::GUID,
    pub MgrEpv: *mut core::ffi::c_void,
    pub Flags: u32,
    pub MaxCalls: u32,
    pub MaxRpcSize: u32,
    pub IfCallback: RPC_IF_CALLBACK_FN,
    pub UuidVector: *mut UUID_VECTOR,
    pub Annotation: RPC_WSTR,
    pub SecurityDescriptor: *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_rpc")]
impl Default for RPC_INTERFACE_TEMPLATEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_rpc")]
pub type RPC_MGMT_AUTHORIZATION_FN = Option<unsafe extern "system" fn(clientbinding: RPC_BINDING_HANDLE, requestedmgmtoperation: u32, status: *mut super::rpc::RPC_STATUS) -> i32>;
#[cfg(feature = "Win32_rpc")]
pub type RPC_NEW_HTTP_PROXY_CHANNEL = Option<unsafe extern "system" fn(redirectorstage: RPC_HTTP_REDIRECTOR_STAGE, servername: *const u16, serverport: *const u16, remoteuser: *const u16, authtype: *const u16, resourceuuid: *mut core::ffi::c_void, sessionid: *mut core::ffi::c_void, interface: *const core::ffi::c_void, reserved: *const core::ffi::c_void, flags: u32, newservername: *mut RPC_WSTR, newserverport: *mut RPC_WSTR) -> super::rpc::RPC_STATUS>;
#[cfg(feature = "Win32_rpc")]
pub type RPC_OBJECT_INQ_FN = Option<unsafe extern "system" fn(objectuuid: *const windows_core::GUID, typeuuid: *mut windows_core::GUID, status: *mut super::rpc::RPC_STATUS)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RPC_POLICY {
    pub Length: u32,
    pub EndpointFlags: u32,
    pub NICFlags: u32,
}
pub const RPC_PROTSEQ_HTTP: u32 = 4;
pub const RPC_PROTSEQ_LRPC: u32 = 3;
pub const RPC_PROTSEQ_NMP: u32 = 2;
pub const RPC_PROTSEQ_TCP: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_PROTSEQ_VECTORA {
    pub Count: u32,
    pub Protseq: [*mut u8; 1],
}
impl Default for RPC_PROTSEQ_VECTORA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_PROTSEQ_VECTORW {
    pub Count: u32,
    pub Protseq: [*mut u16; 1],
}
impl Default for RPC_PROTSEQ_VECTORW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type RPC_SECURITY_CALLBACK_FN = Option<unsafe extern "system" fn(context: *const core::ffi::c_void)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RPC_SECURITY_QOS {
    pub Version: u32,
    pub Capabilities: u32,
    pub IdentityTracking: u32,
    pub ImpersonationType: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RPC_SECURITY_QOS_V2_A {
    pub Version: u32,
    pub Capabilities: u32,
    pub IdentityTracking: u32,
    pub ImpersonationType: u32,
    pub AdditionalSecurityInfoType: u32,
    pub u: RPC_SECURITY_QOS_V2_A_0,
}
impl Default for RPC_SECURITY_QOS_V2_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union RPC_SECURITY_QOS_V2_A_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_A,
}
impl Default for RPC_SECURITY_QOS_V2_A_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RPC_SECURITY_QOS_V2_W {
    pub Version: u32,
    pub Capabilities: u32,
    pub IdentityTracking: u32,
    pub ImpersonationType: u32,
    pub AdditionalSecurityInfoType: u32,
    pub u: RPC_SECURITY_QOS_V2_W_0,
}
impl Default for RPC_SECURITY_QOS_V2_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union RPC_SECURITY_QOS_V2_W_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_W,
}
impl Default for RPC_SECURITY_QOS_V2_W_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RPC_SECURITY_QOS_V3_A {
    pub Version: u32,
    pub Capabilities: u32,
    pub IdentityTracking: u32,
    pub ImpersonationType: u32,
    pub AdditionalSecurityInfoType: u32,
    pub u: RPC_SECURITY_QOS_V3_A_0,
    pub Sid: *mut core::ffi::c_void,
}
impl Default for RPC_SECURITY_QOS_V3_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union RPC_SECURITY_QOS_V3_A_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_A,
}
impl Default for RPC_SECURITY_QOS_V3_A_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RPC_SECURITY_QOS_V3_W {
    pub Version: u32,
    pub Capabilities: u32,
    pub IdentityTracking: u32,
    pub ImpersonationType: u32,
    pub AdditionalSecurityInfoType: u32,
    pub u: RPC_SECURITY_QOS_V3_W_0,
    pub Sid: *mut core::ffi::c_void,
}
impl Default for RPC_SECURITY_QOS_V3_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union RPC_SECURITY_QOS_V3_W_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_W,
}
impl Default for RPC_SECURITY_QOS_V3_W_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RPC_SECURITY_QOS_V4_A {
    pub Version: u32,
    pub Capabilities: u32,
    pub IdentityTracking: u32,
    pub ImpersonationType: u32,
    pub AdditionalSecurityInfoType: u32,
    pub u: RPC_SECURITY_QOS_V4_A_0,
    pub Sid: *mut core::ffi::c_void,
    pub EffectiveOnly: u32,
}
impl Default for RPC_SECURITY_QOS_V4_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union RPC_SECURITY_QOS_V4_A_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_A,
}
impl Default for RPC_SECURITY_QOS_V4_A_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RPC_SECURITY_QOS_V4_W {
    pub Version: u32,
    pub Capabilities: u32,
    pub IdentityTracking: u32,
    pub ImpersonationType: u32,
    pub AdditionalSecurityInfoType: u32,
    pub u: RPC_SECURITY_QOS_V4_W_0,
    pub Sid: *mut core::ffi::c_void,
    pub EffectiveOnly: u32,
}
impl Default for RPC_SECURITY_QOS_V4_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union RPC_SECURITY_QOS_V4_W_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_W,
}
impl Default for RPC_SECURITY_QOS_V4_W_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RPC_SECURITY_QOS_V5_A {
    pub Version: u32,
    pub Capabilities: u32,
    pub IdentityTracking: u32,
    pub ImpersonationType: u32,
    pub AdditionalSecurityInfoType: u32,
    pub u: RPC_SECURITY_QOS_V5_A_0,
    pub Sid: *mut core::ffi::c_void,
    pub EffectiveOnly: u32,
    pub ServerSecurityDescriptor: *mut core::ffi::c_void,
}
impl Default for RPC_SECURITY_QOS_V5_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union RPC_SECURITY_QOS_V5_A_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_A,
}
impl Default for RPC_SECURITY_QOS_V5_A_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RPC_SECURITY_QOS_V5_W {
    pub Version: u32,
    pub Capabilities: u32,
    pub IdentityTracking: u32,
    pub ImpersonationType: u32,
    pub AdditionalSecurityInfoType: u32,
    pub u: RPC_SECURITY_QOS_V5_W_0,
    pub Sid: *mut core::ffi::c_void,
    pub EffectiveOnly: u32,
    pub ServerSecurityDescriptor: *mut core::ffi::c_void,
}
impl Default for RPC_SECURITY_QOS_V5_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union RPC_SECURITY_QOS_V5_W_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_W,
}
impl Default for RPC_SECURITY_QOS_V5_W_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_STATS_VECTOR {
    pub Count: u32,
    pub Stats: [u32; 1],
}
impl Default for RPC_STATS_VECTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RPC_WSTR(pub *mut u16);
impl RPC_WSTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for RPC_WSTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SEC_WINNT_AUTH_IDENTITY_A {
    pub User: *mut u8,
    pub UserLength: u32,
    pub Domain: *mut u8,
    pub DomainLength: u32,
    pub Password: *mut u8,
    pub PasswordLength: u32,
    pub Flags: u32,
}
impl Default for SEC_WINNT_AUTH_IDENTITY_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SEC_WINNT_AUTH_IDENTITY_ANSI: u32 = 1;
pub const SEC_WINNT_AUTH_IDENTITY_UNICODE: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SEC_WINNT_AUTH_IDENTITY_W {
    pub User: *mut u16,
    pub UserLength: u32,
    pub Domain: *mut u16,
    pub DomainLength: u32,
    pub Password: *mut u16,
    pub PasswordLength: u32,
    pub Flags: u32,
}
impl Default for SEC_WINNT_AUTH_IDENTITY_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UUID_VECTOR {
    pub Count: u32,
    pub Uuid: [*mut windows_core::GUID; 1],
}
impl Default for UUID_VECTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_rpc")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct handle_t(pub RPC_BINDING_HANDLE);
