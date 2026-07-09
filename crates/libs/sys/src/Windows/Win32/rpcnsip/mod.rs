#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
windows_link::link!("rpcns4.dll" "system" fn I_RpcNsGetBuffer(message : *mut super::rpcdcep::RPC_MESSAGE) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
windows_link::link!("rpcns4.dll" "system" fn I_RpcNsRaiseException(message : *mut super::rpcdcep::RPC_MESSAGE, status : super::rpc::RPC_STATUS));
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
windows_link::link!("rpcns4.dll" "system" fn I_RpcNsSendReceive(message : *mut super::rpcdcep::RPC_MESSAGE, handle : *mut super::rpcdce::RPC_BINDING_HANDLE) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
windows_link::link!("rpcns4.dll" "system" fn I_RpcReBindBuffer(message : *mut super::rpcdcep::RPC_MESSAGE) -> super::rpc::RPC_STATUS);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcnsi"))]
pub type PRPC_IMPORT_CONTEXT_P = *mut RPC_IMPORT_CONTEXT_P;
#[repr(C)]
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcnsi"))]
#[derive(Clone, Copy)]
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
