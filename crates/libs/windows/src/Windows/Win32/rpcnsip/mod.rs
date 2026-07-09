#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn I_RpcNsGetBuffer(message: *mut super::rpcdcep::RPC_MESSAGE) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn I_RpcNsGetBuffer(message : *mut super::rpcdcep::RPC_MESSAGE) -> super::rpc::RPC_STATUS);
    unsafe { I_RpcNsGetBuffer(message as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn I_RpcNsRaiseException(message: *mut super::rpcdcep::RPC_MESSAGE, status: super::rpc::RPC_STATUS) {
    windows_core::link!("rpcns4.dll" "system" fn I_RpcNsRaiseException(message : *mut super::rpcdcep::RPC_MESSAGE, status : super::rpc::RPC_STATUS));
    unsafe { I_RpcNsRaiseException(message as _, status) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn I_RpcNsSendReceive(message: *mut super::rpcdcep::RPC_MESSAGE, handle: *mut super::rpcdce::RPC_BINDING_HANDLE) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn I_RpcNsSendReceive(message : *mut super::rpcdcep::RPC_MESSAGE, handle : *mut super::rpcdce::RPC_BINDING_HANDLE) -> super::rpc::RPC_STATUS);
    unsafe { I_RpcNsSendReceive(message as _, handle as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn I_RpcReBindBuffer(message: *mut super::rpcdcep::RPC_MESSAGE) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcns4.dll" "system" fn I_RpcReBindBuffer(message : *mut super::rpcdcep::RPC_MESSAGE) -> super::rpc::RPC_STATUS);
    unsafe { I_RpcReBindBuffer(message as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcnsi"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PRPC_IMPORT_CONTEXT_P(pub *mut RPC_IMPORT_CONTEXT_P);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcnsi"))]
impl PRPC_IMPORT_CONTEXT_P {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcnsi"))]
impl Default for PRPC_IMPORT_CONTEXT_P {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcnsi"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_IMPORT_CONTEXT_P {
    pub LookupContext: super::rpcnsi::RPC_NS_HANDLE,
    pub ProposedHandle: super::rpcdce::RPC_BINDING_HANDLE,
    pub Bindings: *mut super::rpcdce::RPC_BINDING_VECTOR,
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcnsi"))]
impl Default for RPC_IMPORT_CONTEXT_P {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
