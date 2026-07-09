#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn NDRCContextBinding(ccontext: NDR_CCONTEXT) -> super::rpcdce::RPC_BINDING_HANDLE {
    windows_core::link!("rpcrt4.dll" "system" fn NDRCContextBinding(ccontext : NDR_CCONTEXT) -> super::rpcdce::RPC_BINDING_HANDLE);
    unsafe { NDRCContextBinding(ccontext) }
}
#[inline]
pub unsafe fn NDRCContextMarshall(ccontext: Option<NDR_CCONTEXT>, pbuff: *mut core::ffi::c_void) {
    windows_core::link!("rpcrt4.dll" "system" fn NDRCContextMarshall(ccontext : NDR_CCONTEXT, pbuff : *mut core::ffi::c_void));
    unsafe { NDRCContextMarshall(ccontext.unwrap_or(core::mem::zeroed()) as _, pbuff as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn NDRCContextUnmarshall(pccontext: Option<*mut NDR_CCONTEXT>, hbinding: super::rpcdce::RPC_BINDING_HANDLE, pbuff: *const core::ffi::c_void, datarepresentation: u32) {
    windows_core::link!("rpcrt4.dll" "system" fn NDRCContextUnmarshall(pccontext : *mut NDR_CCONTEXT, hbinding : super::rpcdce::RPC_BINDING_HANDLE, pbuff : *const core::ffi::c_void, datarepresentation : u32));
    unsafe { NDRCContextUnmarshall(pccontext.unwrap_or(core::mem::zeroed()) as _, hbinding, pbuff, datarepresentation) }
}
#[inline]
pub unsafe fn NDRSContextMarshall(ccontext: *const _NDR_SCONTEXT, pbuff: *mut core::ffi::c_void, userrundownin: NDR_RUNDOWN) {
    windows_core::link!("rpcrt4.dll" "system" fn NDRSContextMarshall(ccontext : *const _NDR_SCONTEXT, pbuff : *mut core::ffi::c_void, userrundownin : NDR_RUNDOWN));
    unsafe { NDRSContextMarshall(ccontext, pbuff as _, userrundownin) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn NDRSContextMarshall2(bindinghandle: super::rpcdce::RPC_BINDING_HANDLE, ccontext: *const _NDR_SCONTEXT, pbuff: *mut core::ffi::c_void, userrundownin: NDR_RUNDOWN, ctxguard: Option<*const core::ffi::c_void>, flags: u32) {
    windows_core::link!("rpcrt4.dll" "system" fn NDRSContextMarshall2(bindinghandle : super::rpcdce::RPC_BINDING_HANDLE, ccontext : *const _NDR_SCONTEXT, pbuff : *mut core::ffi::c_void, userrundownin : NDR_RUNDOWN, ctxguard : *const core::ffi::c_void, flags : u32));
    unsafe { NDRSContextMarshall2(bindinghandle, ccontext, pbuff as _, userrundownin, ctxguard.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn NDRSContextMarshallEx(bindinghandle: super::rpcdce::RPC_BINDING_HANDLE, ccontext: *const _NDR_SCONTEXT, pbuff: *mut core::ffi::c_void, userrundownin: NDR_RUNDOWN) {
    windows_core::link!("rpcrt4.dll" "system" fn NDRSContextMarshallEx(bindinghandle : super::rpcdce::RPC_BINDING_HANDLE, ccontext : *const _NDR_SCONTEXT, pbuff : *mut core::ffi::c_void, userrundownin : NDR_RUNDOWN));
    unsafe { NDRSContextMarshallEx(bindinghandle, ccontext, pbuff as _, userrundownin) }
}
#[inline]
pub unsafe fn NDRSContextUnmarshall(pbuff: *const core::ffi::c_void, datarepresentation: u32) -> NDR_SCONTEXT {
    windows_core::link!("rpcrt4.dll" "system" fn NDRSContextUnmarshall(pbuff : *const core::ffi::c_void, datarepresentation : u32) -> NDR_SCONTEXT);
    unsafe { NDRSContextUnmarshall(pbuff, datarepresentation) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn NDRSContextUnmarshall2(bindinghandle: super::rpcdce::RPC_BINDING_HANDLE, pbuff: Option<*const core::ffi::c_void>, datarepresentation: u32, ctxguard: Option<*const core::ffi::c_void>, flags: u32) -> NDR_SCONTEXT {
    windows_core::link!("rpcrt4.dll" "system" fn NDRSContextUnmarshall2(bindinghandle : super::rpcdce::RPC_BINDING_HANDLE, pbuff : *const core::ffi::c_void, datarepresentation : u32, ctxguard : *const core::ffi::c_void, flags : u32) -> NDR_SCONTEXT);
    unsafe { NDRSContextUnmarshall2(bindinghandle, pbuff.unwrap_or(core::mem::zeroed()) as _, datarepresentation, ctxguard.unwrap_or(core::mem::zeroed()) as _, flags) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn NDRSContextUnmarshallEx(bindinghandle: super::rpcdce::RPC_BINDING_HANDLE, pbuff: *const core::ffi::c_void, datarepresentation: u32) -> NDR_SCONTEXT {
    windows_core::link!("rpcrt4.dll" "system" fn NDRSContextUnmarshallEx(bindinghandle : super::rpcdce::RPC_BINDING_HANDLE, pbuff : *const core::ffi::c_void, datarepresentation : u32) -> NDR_SCONTEXT);
    unsafe { NDRSContextUnmarshallEx(bindinghandle, pbuff, datarepresentation) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn Ndr64AsyncClientCall(pproxyinfo: *mut MIDL_STUBLESS_PROXY_INFO, nprocnum: u32, preturnvalue: *mut core::ffi::c_void) -> CLIENT_CALL_RETURN {
    windows_core::link!("rpcrt4.dll" "C" fn Ndr64AsyncClientCall(pproxyinfo : *mut MIDL_STUBLESS_PROXY_INFO, nprocnum : u32, preturnvalue : *mut core::ffi::c_void) -> CLIENT_CALL_RETURN);
    unsafe { Ndr64AsyncClientCall(pproxyinfo as _, nprocnum, preturnvalue as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn Ndr64AsyncServerCall64(prpcmsg: *mut super::rpcdcep::RPC_MESSAGE) {
    windows_core::link!("rpcrt4.dll" "system" fn Ndr64AsyncServerCall64(prpcmsg : *mut super::rpcdcep::RPC_MESSAGE));
    unsafe { Ndr64AsyncServerCall64(prpcmsg as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn Ndr64AsyncServerCallAll(prpcmsg: *mut super::rpcdcep::RPC_MESSAGE) {
    windows_core::link!("rpcrt4.dll" "system" fn Ndr64AsyncServerCallAll(prpcmsg : *mut super::rpcdcep::RPC_MESSAGE));
    unsafe { Ndr64AsyncServerCallAll(prpcmsg as _) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn Ndr64DcomAsyncClientCall(pproxyinfo: *mut MIDL_STUBLESS_PROXY_INFO, nprocnum: u32, preturnvalue: *mut core::ffi::c_void) -> CLIENT_CALL_RETURN {
    windows_core::link!("rpcrt4.dll" "C" fn Ndr64DcomAsyncClientCall(pproxyinfo : *mut MIDL_STUBLESS_PROXY_INFO, nprocnum : u32, preturnvalue : *mut core::ffi::c_void) -> CLIENT_CALL_RETURN);
    unsafe { Ndr64DcomAsyncClientCall(pproxyinfo as _, nprocnum, preturnvalue as _) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn Ndr64DcomAsyncStubCall<P0, P1>(pthis: P0, pchannel: P1, prpcmsg: *mut super::rpcdcep::RPC_MESSAGE, pdwstubphase: *mut u32) -> i32
where
    P0: windows_core::Param<super::objidlbase::IRpcStubBuffer>,
    P1: windows_core::Param<super::objidlbase::IRpcChannelBuffer>,
{
    windows_core::link!("rpcrt4.dll" "system" fn Ndr64DcomAsyncStubCall(pthis : *mut core::ffi::c_void, pchannel : *mut core::ffi::c_void, prpcmsg : *mut super::rpcdcep::RPC_MESSAGE, pdwstubphase : *mut u32) -> i32);
    unsafe { Ndr64DcomAsyncStubCall(pthis.param().abi(), pchannel.param().abi(), prpcmsg as _, pdwstubphase as _) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrAllocate(pstubmsg: *mut MIDL_STUB_MESSAGE, len: usize) -> *mut core::ffi::c_void {
    windows_core::link!("rpcrt4.dll" "system" fn NdrAllocate(pstubmsg : *mut MIDL_STUB_MESSAGE, len : usize) -> *mut core::ffi::c_void);
    unsafe { NdrAllocate(core::mem::transmute(pstubmsg), len) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrAsyncClientCall(pstubdescriptor: *const MIDL_STUB_DESC, pformat: *const u8) -> CLIENT_CALL_RETURN {
    windows_core::link!("rpcrt4.dll" "C" fn NdrAsyncClientCall(pstubdescriptor : *const MIDL_STUB_DESC, pformat : *const u8) -> CLIENT_CALL_RETURN);
    unsafe { NdrAsyncClientCall(pstubdescriptor, pformat) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrAsyncServerCall(prpcmsg: *mut super::rpcdcep::RPC_MESSAGE) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrAsyncServerCall(prpcmsg : *mut super::rpcdcep::RPC_MESSAGE));
    unsafe { NdrAsyncServerCall(prpcmsg as _) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrByteCountPointerBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrByteCountPointerBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrByteCountPointerBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrByteCountPointerFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrByteCountPointerFree(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrByteCountPointerFree(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrByteCountPointerMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrByteCountPointerMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8) -> *mut u8);
    unsafe { NdrByteCountPointerMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrByteCountPointerUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *const u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrByteCountPointerUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *const u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrByteCountPointerUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat, fmustalloc) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrClearOutParameters(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *const u8, argaddr: *mut core::ffi::c_void) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrClearOutParameters(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *const u8, argaddr : *mut core::ffi::c_void));
    unsafe { NdrClearOutParameters(core::mem::transmute(pstubmsg), pformat, argaddr as _) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrClientCall2(pstubdescriptor: *const MIDL_STUB_DESC, pformat: *const u8) -> CLIENT_CALL_RETURN {
    windows_core::link!("rpcrt4.dll" "C" fn NdrClientCall2(pstubdescriptor : *const MIDL_STUB_DESC, pformat : *const u8) -> CLIENT_CALL_RETURN);
    unsafe { NdrClientCall2(pstubdescriptor, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrClientCall3(pproxyinfo: *mut MIDL_STUBLESS_PROXY_INFO, nprocnum: u32, preturnvalue: *mut core::ffi::c_void) -> CLIENT_CALL_RETURN {
    windows_core::link!("rpcrt4.dll" "C" fn NdrClientCall3(pproxyinfo : *mut MIDL_STUBLESS_PROXY_INFO, nprocnum : u32, preturnvalue : *mut core::ffi::c_void) -> CLIENT_CALL_RETURN);
    unsafe { NdrClientCall3(pproxyinfo as _, nprocnum, preturnvalue as _) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrClientContextMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, contexthandle: NDR_CCONTEXT, fcheck: i32) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrClientContextMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, contexthandle : NDR_CCONTEXT, fcheck : i32));
    unsafe { NdrClientContextMarshall(core::mem::transmute(pstubmsg), contexthandle, fcheck) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrClientContextUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pcontexthandle: *mut NDR_CCONTEXT, bindhandle: super::rpcdce::RPC_BINDING_HANDLE) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrClientContextUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pcontexthandle : *mut NDR_CCONTEXT, bindhandle : super::rpcdce::RPC_BINDING_HANDLE));
    unsafe { NdrClientContextUnmarshall(core::mem::transmute(pstubmsg), pcontexthandle as _, bindhandle) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrClientInitialize(prpcmsg: *mut super::rpcdcep::RPC_MESSAGE, pstubmsg: *mut MIDL_STUB_MESSAGE, pstubdescriptor: *const MIDL_STUB_DESC, procnum: u32) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrClientInitialize(prpcmsg : *mut super::rpcdcep::RPC_MESSAGE, pstubmsg : *mut MIDL_STUB_MESSAGE, pstubdescriptor : *const MIDL_STUB_DESC, procnum : u32));
    unsafe { NdrClientInitialize(prpcmsg as _, core::mem::transmute(pstubmsg), pstubdescriptor, procnum) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrClientInitializeNew(prpcmsg: *mut super::rpcdcep::RPC_MESSAGE, pstubmsg: *mut MIDL_STUB_MESSAGE, pstubdescriptor: *const MIDL_STUB_DESC, procnum: u32) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrClientInitializeNew(prpcmsg : *mut super::rpcdcep::RPC_MESSAGE, pstubmsg : *mut MIDL_STUB_MESSAGE, pstubdescriptor : *const MIDL_STUB_DESC, procnum : u32));
    unsafe { NdrClientInitializeNew(prpcmsg as _, core::mem::transmute(pstubmsg), pstubdescriptor, procnum) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrComplexArrayBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrComplexArrayBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrComplexArrayBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrComplexArrayFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrComplexArrayFree(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrComplexArrayFree(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrComplexArrayMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrComplexArrayMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8) -> *mut u8);
    unsafe { NdrComplexArrayMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrComplexArrayMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *const u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrComplexArrayMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *const u8) -> u32);
    unsafe { NdrComplexArrayMemorySize(core::mem::transmute(pstubmsg), pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrComplexArrayUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *const u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrComplexArrayUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *const u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrComplexArrayUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat, fmustalloc) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrComplexStructBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrComplexStructBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrComplexStructBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrComplexStructFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrComplexStructFree(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrComplexStructFree(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrComplexStructMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrComplexStructMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8) -> *mut u8);
    unsafe { NdrComplexStructMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrComplexStructMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *const u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrComplexStructMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *const u8) -> u32);
    unsafe { NdrComplexStructMemorySize(core::mem::transmute(pstubmsg), pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrComplexStructUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *const u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrComplexStructUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *const u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrComplexStructUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat, fmustalloc) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrConformantArrayBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantArrayBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrConformantArrayBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrConformantArrayFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantArrayFree(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrConformantArrayFree(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrConformantArrayMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantArrayMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8) -> *mut u8);
    unsafe { NdrConformantArrayMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrConformantArrayMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *const u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantArrayMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *const u8) -> u32);
    unsafe { NdrConformantArrayMemorySize(core::mem::transmute(pstubmsg), pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrConformantArrayUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *const u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantArrayUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *const u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrConformantArrayUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat, fmustalloc) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrConformantStringBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantStringBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrConformantStringBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrConformantStringMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantStringMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8) -> *mut u8);
    unsafe { NdrConformantStringMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrConformantStringMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *const u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantStringMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *const u8) -> u32);
    unsafe { NdrConformantStringMemorySize(core::mem::transmute(pstubmsg), pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrConformantStringUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *const u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantStringUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *const u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrConformantStringUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat, fmustalloc) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrConformantStructBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantStructBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrConformantStructBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrConformantStructFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantStructFree(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrConformantStructFree(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrConformantStructMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantStructMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8) -> *mut u8);
    unsafe { NdrConformantStructMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrConformantStructMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *const u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantStructMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *const u8) -> u32);
    unsafe { NdrConformantStructMemorySize(core::mem::transmute(pstubmsg), pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrConformantStructUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *const u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantStructUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *const u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrConformantStructUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat, fmustalloc) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrConformantVaryingArrayBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantVaryingArrayBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrConformantVaryingArrayBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrConformantVaryingArrayFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantVaryingArrayFree(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrConformantVaryingArrayFree(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrConformantVaryingArrayMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantVaryingArrayMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8) -> *mut u8);
    unsafe { NdrConformantVaryingArrayMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrConformantVaryingArrayMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *const u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantVaryingArrayMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *const u8) -> u32);
    unsafe { NdrConformantVaryingArrayMemorySize(core::mem::transmute(pstubmsg), pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrConformantVaryingArrayUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *const u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantVaryingArrayUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *const u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrConformantVaryingArrayUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat, fmustalloc) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrConformantVaryingStructBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantVaryingStructBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrConformantVaryingStructBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrConformantVaryingStructFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantVaryingStructFree(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrConformantVaryingStructFree(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrConformantVaryingStructMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantVaryingStructMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8) -> *mut u8);
    unsafe { NdrConformantVaryingStructMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrConformantVaryingStructMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *const u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantVaryingStructMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *const u8) -> u32);
    unsafe { NdrConformantVaryingStructMemorySize(core::mem::transmute(pstubmsg), pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrConformantVaryingStructUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *const u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConformantVaryingStructUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *const u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrConformantVaryingStructUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat, fmustalloc) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrContextHandleInitialize(pstubmsg: *const MIDL_STUB_MESSAGE, pformat: *const u8) -> NDR_SCONTEXT {
    windows_core::link!("rpcrt4.dll" "system" fn NdrContextHandleInitialize(pstubmsg : *const MIDL_STUB_MESSAGE, pformat : *const u8) -> NDR_SCONTEXT);
    unsafe { NdrContextHandleInitialize(core::mem::transmute(pstubmsg), pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrContextHandleSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrContextHandleSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrContextHandleSize(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrConvert(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConvert(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *const u8));
    unsafe { NdrConvert(core::mem::transmute(pstubmsg), pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrConvert2(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *const u8, numberparams: i32) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrConvert2(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *const u8, numberparams : i32));
    unsafe { NdrConvert2(core::mem::transmute(pstubmsg), pformat, numberparams) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrCorrelationFree(pstubmsg: *mut MIDL_STUB_MESSAGE) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrCorrelationFree(pstubmsg : *mut MIDL_STUB_MESSAGE));
    unsafe { NdrCorrelationFree(core::mem::transmute(pstubmsg)) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrCorrelationInitialize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut core::ffi::c_void, cachesize: u32, flags: u32) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrCorrelationInitialize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut core::ffi::c_void, cachesize : u32, flags : u32));
    unsafe { NdrCorrelationInitialize(core::mem::transmute(pstubmsg), pmemory as _, cachesize, flags) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrCorrelationPass(pstubmsg: *mut MIDL_STUB_MESSAGE) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrCorrelationPass(pstubmsg : *mut MIDL_STUB_MESSAGE));
    unsafe { NdrCorrelationPass(core::mem::transmute(pstubmsg)) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrCreateServerInterfaceFromStub<P0>(pstub: P0, pserverif: *mut super::rpcdcep::RPC_SERVER_INTERFACE) -> super::rpc::RPC_STATUS
where
    P0: windows_core::Param<super::objidlbase::IRpcStubBuffer>,
{
    windows_core::link!("rpcrt4.dll" "system" fn NdrCreateServerInterfaceFromStub(pstub : *mut core::ffi::c_void, pserverif : *mut super::rpcdcep::RPC_SERVER_INTERFACE) -> super::rpc::RPC_STATUS);
    unsafe { NdrCreateServerInterfaceFromStub(pstub.param().abi(), pserverif as _) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrDcomAsyncClientCall(pstubdescriptor: *const MIDL_STUB_DESC, pformat: *const u8) -> CLIENT_CALL_RETURN {
    windows_core::link!("rpcrt4.dll" "C" fn NdrDcomAsyncClientCall(pstubdescriptor : *const MIDL_STUB_DESC, pformat : *const u8) -> CLIENT_CALL_RETURN);
    unsafe { NdrDcomAsyncClientCall(pstubdescriptor, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrDcomAsyncStubCall<P0, P1>(pthis: P0, pchannel: P1, prpcmsg: *mut super::rpcdcep::RPC_MESSAGE, pdwstubphase: *mut u32) -> i32
where
    P0: windows_core::Param<super::objidlbase::IRpcStubBuffer>,
    P1: windows_core::Param<super::objidlbase::IRpcChannelBuffer>,
{
    windows_core::link!("rpcrt4.dll" "system" fn NdrDcomAsyncStubCall(pthis : *mut core::ffi::c_void, pchannel : *mut core::ffi::c_void, prpcmsg : *mut super::rpcdcep::RPC_MESSAGE, pdwstubphase : *mut u32) -> i32);
    unsafe { NdrDcomAsyncStubCall(pthis.param().abi(), pchannel.param().abi(), prpcmsg as _, pdwstubphase as _) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrEncapsulatedUnionBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrEncapsulatedUnionBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrEncapsulatedUnionBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrEncapsulatedUnionFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrEncapsulatedUnionFree(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrEncapsulatedUnionFree(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrEncapsulatedUnionMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrEncapsulatedUnionMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8) -> *mut u8);
    unsafe { NdrEncapsulatedUnionMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrEncapsulatedUnionMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *const u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrEncapsulatedUnionMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *const u8) -> u32);
    unsafe { NdrEncapsulatedUnionMemorySize(core::mem::transmute(pstubmsg), pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrEncapsulatedUnionUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *const u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrEncapsulatedUnionUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *const u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrEncapsulatedUnionUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat, fmustalloc) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrFixedArrayBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrFixedArrayBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrFixedArrayBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrFixedArrayFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrFixedArrayFree(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrFixedArrayFree(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrFixedArrayMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrFixedArrayMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8) -> *mut u8);
    unsafe { NdrFixedArrayMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrFixedArrayMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *const u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrFixedArrayMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *const u8) -> u32);
    unsafe { NdrFixedArrayMemorySize(core::mem::transmute(pstubmsg), pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrFixedArrayUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *const u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrFixedArrayUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *const u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrFixedArrayUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat, fmustalloc) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrFreeBuffer(pstubmsg: *mut MIDL_STUB_MESSAGE) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrFreeBuffer(pstubmsg : *mut MIDL_STUB_MESSAGE));
    unsafe { NdrFreeBuffer(core::mem::transmute(pstubmsg)) }
}
#[inline]
pub unsafe fn NdrFullPointerXlatFree() -> FULL_PTR_XLAT_TABLES {
    windows_core::link!("rpcrt4.dll" "system" fn NdrFullPointerXlatFree(pxlattables : *mut FULL_PTR_XLAT_TABLES));
    unsafe {
        let mut result__ = core::mem::zeroed();
        NdrFullPointerXlatFree(&mut result__);
        result__
    }
}
#[inline]
pub unsafe fn NdrFullPointerXlatInit(numberofpointers: u32, xlatside: XLAT_SIDE) -> PFULL_PTR_XLAT_TABLES {
    windows_core::link!("rpcrt4.dll" "system" fn NdrFullPointerXlatInit(numberofpointers : u32, xlatside : XLAT_SIDE) -> PFULL_PTR_XLAT_TABLES);
    unsafe { NdrFullPointerXlatInit(numberofpointers, xlatside) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrGetBuffer(pstubmsg: *mut MIDL_STUB_MESSAGE, bufferlength: u32, handle: super::rpcdce::RPC_BINDING_HANDLE) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrGetBuffer(pstubmsg : *mut MIDL_STUB_MESSAGE, bufferlength : u32, handle : super::rpcdce::RPC_BINDING_HANDLE) -> *mut u8);
    unsafe { NdrGetBuffer(core::mem::transmute(pstubmsg), bufferlength, handle) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrGetDcomProtocolVersion(pstubmsg: *mut MIDL_STUB_MESSAGE, pversion: *mut super::rpcdcep::RPC_VERSION) -> windows_core::HRESULT {
    windows_core::link!("rpcrt4.dll" "system" fn NdrGetDcomProtocolVersion(pstubmsg : *mut MIDL_STUB_MESSAGE, pversion : *mut super::rpcdcep::RPC_VERSION) -> windows_core::HRESULT);
    unsafe { NdrGetDcomProtocolVersion(core::mem::transmute(pstubmsg), pversion as _) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc"))]
#[inline]
pub unsafe fn NdrGetUserMarshalInfo(pflags: *const u32, informationlevel: u32, pmarshalinfo: *mut NDR_USER_MARSHAL_INFO) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn NdrGetUserMarshalInfo(pflags : *const u32, informationlevel : u32, pmarshalinfo : *mut NDR_USER_MARSHAL_INFO) -> super::rpc::RPC_STATUS);
    unsafe { NdrGetUserMarshalInfo(pflags, informationlevel, core::mem::transmute(pmarshalinfo)) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrInterfacePointerBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrInterfacePointerBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrInterfacePointerBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrInterfacePointerFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrInterfacePointerFree(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrInterfacePointerFree(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrInterfacePointerMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrInterfacePointerMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8) -> *mut u8);
    unsafe { NdrInterfacePointerMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrInterfacePointerMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *const u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrInterfacePointerMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *const u8) -> u32);
    unsafe { NdrInterfacePointerMemorySize(core::mem::transmute(pstubmsg), pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrInterfacePointerUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *const u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrInterfacePointerUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *const u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrInterfacePointerUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat, fmustalloc) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrMapCommAndFaultStatus(pstubmsg: *mut MIDL_STUB_MESSAGE, pcommstatus: *mut u32, pfaultstatus: *mut u32, status: super::rpc::RPC_STATUS) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn NdrMapCommAndFaultStatus(pstubmsg : *mut MIDL_STUB_MESSAGE, pcommstatus : *mut u32, pfaultstatus : *mut u32, status : super::rpc::RPC_STATUS) -> super::rpc::RPC_STATUS);
    unsafe { NdrMapCommAndFaultStatus(core::mem::transmute(pstubmsg), pcommstatus as _, pfaultstatus as _, status) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrNonConformantStringBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrNonConformantStringBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrNonConformantStringBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrNonConformantStringMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrNonConformantStringMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8) -> *mut u8);
    unsafe { NdrNonConformantStringMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrNonConformantStringMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *const u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrNonConformantStringMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *const u8) -> u32);
    unsafe { NdrNonConformantStringMemorySize(core::mem::transmute(pstubmsg), pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrNonConformantStringUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *const u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrNonConformantStringUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *const u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrNonConformantStringUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat, fmustalloc) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrNonEncapsulatedUnionBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrNonEncapsulatedUnionBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrNonEncapsulatedUnionBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrNonEncapsulatedUnionFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrNonEncapsulatedUnionFree(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrNonEncapsulatedUnionFree(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrNonEncapsulatedUnionMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrNonEncapsulatedUnionMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8) -> *mut u8);
    unsafe { NdrNonEncapsulatedUnionMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrNonEncapsulatedUnionMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *const u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrNonEncapsulatedUnionMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *const u8) -> u32);
    unsafe { NdrNonEncapsulatedUnionMemorySize(core::mem::transmute(pstubmsg), pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrNonEncapsulatedUnionUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *const u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrNonEncapsulatedUnionUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *const u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrNonEncapsulatedUnionUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat, fmustalloc) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrNsGetBuffer(pstubmsg: *mut MIDL_STUB_MESSAGE, bufferlength: u32, handle: super::rpcdce::RPC_BINDING_HANDLE) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrNsGetBuffer(pstubmsg : *mut MIDL_STUB_MESSAGE, bufferlength : u32, handle : super::rpcdce::RPC_BINDING_HANDLE) -> *mut u8);
    unsafe { NdrNsGetBuffer(core::mem::transmute(pstubmsg), bufferlength, handle) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrNsSendReceive(pstubmsg: *mut MIDL_STUB_MESSAGE, pbufferend: *mut u8, pautohandle: *mut super::rpcdce::RPC_BINDING_HANDLE) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrNsSendReceive(pstubmsg : *mut MIDL_STUB_MESSAGE, pbufferend : *mut u8, pautohandle : *mut super::rpcdce::RPC_BINDING_HANDLE) -> *mut u8);
    unsafe { NdrNsSendReceive(core::mem::transmute(pstubmsg), pbufferend as _, pautohandle as _) }
}
#[inline]
pub unsafe fn NdrOleAllocate(size: usize) -> *mut core::ffi::c_void {
    windows_core::link!("rpcrt4.dll" "system" fn NdrOleAllocate(size : usize) -> *mut core::ffi::c_void);
    unsafe { NdrOleAllocate(size) }
}
#[inline]
pub unsafe fn NdrOleFree(nodetofree: *const core::ffi::c_void) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrOleFree(nodetofree : *const core::ffi::c_void));
    unsafe { NdrOleFree(nodetofree) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrPartialIgnoreClientBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut core::ffi::c_void) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrPartialIgnoreClientBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut core::ffi::c_void));
    unsafe { NdrPartialIgnoreClientBufferSize(core::mem::transmute(pstubmsg), pmemory as _) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrPartialIgnoreClientMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut core::ffi::c_void) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrPartialIgnoreClientMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut core::ffi::c_void));
    unsafe { NdrPartialIgnoreClientMarshall(core::mem::transmute(pstubmsg), pmemory as _) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrPartialIgnoreServerInitialize(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut core::ffi::c_void, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrPartialIgnoreServerInitialize(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut core::ffi::c_void, pformat : *const u8));
    unsafe { NdrPartialIgnoreServerInitialize(core::mem::transmute(pstubmsg), ppmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrPartialIgnoreServerUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut core::ffi::c_void) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrPartialIgnoreServerUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut core::ffi::c_void));
    unsafe { NdrPartialIgnoreServerUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrPointerBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrPointerBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrPointerBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrPointerFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrPointerFree(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrPointerFree(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrPointerMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrPointerMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8) -> *mut u8);
    unsafe { NdrPointerMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrPointerMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *const u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrPointerMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *const u8) -> u32);
    unsafe { NdrPointerMemorySize(core::mem::transmute(pstubmsg), pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrPointerUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *const u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrPointerUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *const u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrPointerUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat, fmustalloc) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrRangeUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *const u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrRangeUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *const u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrRangeUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat, fmustalloc) }
}
#[inline]
pub unsafe fn NdrRpcSmClientAllocate(size: usize) -> *mut core::ffi::c_void {
    windows_core::link!("rpcrt4.dll" "system" fn NdrRpcSmClientAllocate(size : usize) -> *mut core::ffi::c_void);
    unsafe { NdrRpcSmClientAllocate(size) }
}
#[inline]
pub unsafe fn NdrRpcSmClientFree(nodetofree: *const core::ffi::c_void) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrRpcSmClientFree(nodetofree : *const core::ffi::c_void));
    unsafe { NdrRpcSmClientFree(nodetofree) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrRpcSmSetClientToOsf(pmessage: *mut MIDL_STUB_MESSAGE) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrRpcSmSetClientToOsf(pmessage : *mut MIDL_STUB_MESSAGE));
    unsafe { NdrRpcSmSetClientToOsf(core::mem::transmute(pmessage)) }
}
#[inline]
pub unsafe fn NdrRpcSsDefaultAllocate(size: usize) -> *mut core::ffi::c_void {
    windows_core::link!("rpcrt4.dll" "system" fn NdrRpcSsDefaultAllocate(size : usize) -> *mut core::ffi::c_void);
    unsafe { NdrRpcSsDefaultAllocate(size) }
}
#[inline]
pub unsafe fn NdrRpcSsDefaultFree(nodetofree: *const core::ffi::c_void) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrRpcSsDefaultFree(nodetofree : *const core::ffi::c_void));
    unsafe { NdrRpcSsDefaultFree(nodetofree) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrRpcSsDisableAllocate(pmessage: *mut MIDL_STUB_MESSAGE) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrRpcSsDisableAllocate(pmessage : *mut MIDL_STUB_MESSAGE));
    unsafe { NdrRpcSsDisableAllocate(core::mem::transmute(pmessage)) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrRpcSsEnableAllocate(pmessage: *mut MIDL_STUB_MESSAGE) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrRpcSsEnableAllocate(pmessage : *mut MIDL_STUB_MESSAGE));
    unsafe { NdrRpcSsEnableAllocate(core::mem::transmute(pmessage)) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrSendReceive(pstubmsg: *mut MIDL_STUB_MESSAGE, pbufferend: *mut u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrSendReceive(pstubmsg : *mut MIDL_STUB_MESSAGE, pbufferend : *mut u8) -> *mut u8);
    unsafe { NdrSendReceive(core::mem::transmute(pstubmsg), pbufferend as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrServerCall2(prpcmsg: *mut super::rpcdcep::RPC_MESSAGE) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrServerCall2(prpcmsg : *mut super::rpcdcep::RPC_MESSAGE));
    unsafe { NdrServerCall2(prpcmsg as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrServerCallAll(prpcmsg: *mut super::rpcdcep::RPC_MESSAGE) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrServerCallAll(prpcmsg : *mut super::rpcdcep::RPC_MESSAGE));
    unsafe { NdrServerCallAll(prpcmsg as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrServerCallNdr64(prpcmsg: *mut super::rpcdcep::RPC_MESSAGE) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrServerCallNdr64(prpcmsg : *mut super::rpcdcep::RPC_MESSAGE));
    unsafe { NdrServerCallNdr64(prpcmsg as _) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrServerContextMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, contexthandle: *mut _NDR_SCONTEXT, rundownroutine: NDR_RUNDOWN) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrServerContextMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, contexthandle : *mut _NDR_SCONTEXT, rundownroutine : NDR_RUNDOWN));
    unsafe { NdrServerContextMarshall(core::mem::transmute(pstubmsg), contexthandle as _, rundownroutine) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrServerContextNewMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, contexthandle: *mut _NDR_SCONTEXT, rundownroutine: NDR_RUNDOWN, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrServerContextNewMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, contexthandle : *mut _NDR_SCONTEXT, rundownroutine : NDR_RUNDOWN, pformat : *const u8));
    unsafe { NdrServerContextNewMarshall(core::mem::transmute(pstubmsg), contexthandle as _, rundownroutine, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrServerContextNewUnmarshall(pstubmsg: *const MIDL_STUB_MESSAGE, pformat: *const u8) -> NDR_SCONTEXT {
    windows_core::link!("rpcrt4.dll" "system" fn NdrServerContextNewUnmarshall(pstubmsg : *const MIDL_STUB_MESSAGE, pformat : *const u8) -> NDR_SCONTEXT);
    unsafe { NdrServerContextNewUnmarshall(core::mem::transmute(pstubmsg), pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrServerContextUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE) -> NDR_SCONTEXT {
    windows_core::link!("rpcrt4.dll" "system" fn NdrServerContextUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE) -> NDR_SCONTEXT);
    unsafe { NdrServerContextUnmarshall(core::mem::transmute(pstubmsg)) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrServerInitialize(prpcmsg: *mut super::rpcdcep::RPC_MESSAGE, pstubmsg: *mut MIDL_STUB_MESSAGE, pstubdescriptor: *const MIDL_STUB_DESC) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrServerInitialize(prpcmsg : *mut super::rpcdcep::RPC_MESSAGE, pstubmsg : *mut MIDL_STUB_MESSAGE, pstubdescriptor : *const MIDL_STUB_DESC) -> *mut u8);
    unsafe { NdrServerInitialize(prpcmsg as _, core::mem::transmute(pstubmsg), pstubdescriptor) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrServerInitializeMarshall(prpcmsg: *mut super::rpcdcep::RPC_MESSAGE, pstubmsg: *mut MIDL_STUB_MESSAGE) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrServerInitializeMarshall(prpcmsg : *mut super::rpcdcep::RPC_MESSAGE, pstubmsg : *mut MIDL_STUB_MESSAGE));
    unsafe { NdrServerInitializeMarshall(prpcmsg as _, core::mem::transmute(pstubmsg)) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrServerInitializeNew(prpcmsg: *mut super::rpcdcep::RPC_MESSAGE, pstubmsg: *mut MIDL_STUB_MESSAGE, pstubdescriptor: *const MIDL_STUB_DESC) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrServerInitializeNew(prpcmsg : *mut super::rpcdcep::RPC_MESSAGE, pstubmsg : *mut MIDL_STUB_MESSAGE, pstubdescriptor : *const MIDL_STUB_DESC) -> *mut u8);
    unsafe { NdrServerInitializeNew(prpcmsg as _, core::mem::transmute(pstubmsg), pstubdescriptor) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrServerInitializePartial(prpcmsg: *mut super::rpcdcep::RPC_MESSAGE, pstubmsg: *mut MIDL_STUB_MESSAGE, pstubdescriptor: *const MIDL_STUB_DESC, requestedbuffersize: u32) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrServerInitializePartial(prpcmsg : *mut super::rpcdcep::RPC_MESSAGE, pstubmsg : *mut MIDL_STUB_MESSAGE, pstubdescriptor : *const MIDL_STUB_DESC, requestedbuffersize : u32));
    unsafe { NdrServerInitializePartial(prpcmsg as _, core::mem::transmute(pstubmsg), pstubdescriptor, requestedbuffersize) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrServerInitializeUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pstubdescriptor: *const MIDL_STUB_DESC, prpcmsg: *mut super::rpcdcep::RPC_MESSAGE) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrServerInitializeUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pstubdescriptor : *const MIDL_STUB_DESC, prpcmsg : *mut super::rpcdcep::RPC_MESSAGE) -> *mut u8);
    unsafe { NdrServerInitializeUnmarshall(core::mem::transmute(pstubmsg), pstubdescriptor, prpcmsg as _) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrSimpleStructBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrSimpleStructBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrSimpleStructBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrSimpleStructFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrSimpleStructFree(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrSimpleStructFree(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrSimpleStructMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrSimpleStructMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8) -> *mut u8);
    unsafe { NdrSimpleStructMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrSimpleStructMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *const u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrSimpleStructMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *const u8) -> u32);
    unsafe { NdrSimpleStructMemorySize(core::mem::transmute(pstubmsg), pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrSimpleStructUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *const u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrSimpleStructUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *const u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrSimpleStructUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat, fmustalloc) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrSimpleTypeMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, formatchar: u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrSimpleTypeMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, formatchar : u8));
    unsafe { NdrSimpleTypeMarshall(core::mem::transmute(pstubmsg), pmemory as _, formatchar) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrSimpleTypeUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, formatchar: u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrSimpleTypeUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, formatchar : u8));
    unsafe { NdrSimpleTypeUnmarshall(core::mem::transmute(pstubmsg), pmemory as _, formatchar) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrStubCall2(pthis: *mut core::ffi::c_void, pchannel: *mut core::ffi::c_void, prpcmsg: *mut super::rpcdcep::RPC_MESSAGE, pdwstubphase: *mut u32) -> i32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrStubCall2(pthis : *mut core::ffi::c_void, pchannel : *mut core::ffi::c_void, prpcmsg : *mut super::rpcdcep::RPC_MESSAGE, pdwstubphase : *mut u32) -> i32);
    unsafe { NdrStubCall2(pthis as _, pchannel as _, prpcmsg as _, pdwstubphase as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrStubCall3(pthis: *mut core::ffi::c_void, pchannel: *mut core::ffi::c_void, prpcmsg: *mut super::rpcdcep::RPC_MESSAGE, pdwstubphase: *mut u32) -> i32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrStubCall3(pthis : *mut core::ffi::c_void, pchannel : *mut core::ffi::c_void, prpcmsg : *mut super::rpcdcep::RPC_MESSAGE, pdwstubphase : *mut u32) -> i32);
    unsafe { NdrStubCall3(pthis as _, pchannel as _, prpcmsg as _, pdwstubphase as _) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrUserMarshalBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrUserMarshalBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrUserMarshalBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrUserMarshalFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrUserMarshalFree(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrUserMarshalFree(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrUserMarshalMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrUserMarshalMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8) -> *mut u8);
    unsafe { NdrUserMarshalMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrUserMarshalMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *const u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrUserMarshalMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *const u8) -> u32);
    unsafe { NdrUserMarshalMemorySize(core::mem::transmute(pstubmsg), pformat) }
}
#[inline]
pub unsafe fn NdrUserMarshalSimpleTypeConvert(pflags: *mut u32, pbuffer: *mut u8, formatchar: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrUserMarshalSimpleTypeConvert(pflags : *mut u32, pbuffer : *mut u8, formatchar : u8) -> *mut u8);
    unsafe { NdrUserMarshalSimpleTypeConvert(pflags as _, pbuffer as _, formatchar) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrUserMarshalUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *const u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrUserMarshalUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *const u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrUserMarshalUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat, fmustalloc) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrVaryingArrayBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrVaryingArrayBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrVaryingArrayBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrVaryingArrayFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrVaryingArrayFree(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrVaryingArrayFree(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrVaryingArrayMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrVaryingArrayMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8) -> *mut u8);
    unsafe { NdrVaryingArrayMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrVaryingArrayMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *const u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrVaryingArrayMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *const u8) -> u32);
    unsafe { NdrVaryingArrayMemorySize(core::mem::transmute(pstubmsg), pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrVaryingArrayUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *const u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrVaryingArrayUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *const u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrVaryingArrayUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat, fmustalloc) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrXmitOrRepAsBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrXmitOrRepAsBufferSize(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrXmitOrRepAsBufferSize(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrXmitOrRepAsFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) {
    windows_core::link!("rpcrt4.dll" "system" fn NdrXmitOrRepAsFree(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8));
    unsafe { NdrXmitOrRepAsFree(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrXmitOrRepAsMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *const u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrXmitOrRepAsMarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, pmemory : *mut u8, pformat : *const u8) -> *mut u8);
    unsafe { NdrXmitOrRepAsMarshall(core::mem::transmute(pstubmsg), pmemory as _, pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrXmitOrRepAsMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *const u8) -> u32 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrXmitOrRepAsMemorySize(pstubmsg : *mut MIDL_STUB_MESSAGE, pformat : *const u8) -> u32);
    unsafe { NdrXmitOrRepAsMemorySize(core::mem::transmute(pstubmsg), pformat) }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[inline]
pub unsafe fn NdrXmitOrRepAsUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *const u8, fmustalloc: u8) -> *mut u8 {
    windows_core::link!("rpcrt4.dll" "system" fn NdrXmitOrRepAsUnmarshall(pstubmsg : *mut MIDL_STUB_MESSAGE, ppmemory : *mut *mut u8, pformat : *const u8, fmustalloc : u8) -> *mut u8);
    unsafe { NdrXmitOrRepAsUnmarshall(core::mem::transmute(pstubmsg), ppmemory as _, pformat, fmustalloc) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcSmAllocate(size: usize, pstatus: *mut super::rpc::RPC_STATUS) -> *mut core::ffi::c_void {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSmAllocate(size : usize, pstatus : *mut super::rpc::RPC_STATUS) -> *mut core::ffi::c_void);
    unsafe { RpcSmAllocate(size, pstatus as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcSmClientFree(pnodetofree: *const core::ffi::c_void) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSmClientFree(pnodetofree : *const core::ffi::c_void) -> super::rpc::RPC_STATUS);
    unsafe { RpcSmClientFree(pnodetofree) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcSmDestroyClientContext(contexthandle: *const *const core::ffi::c_void) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSmDestroyClientContext(contexthandle : *const *const core::ffi::c_void) -> super::rpc::RPC_STATUS);
    unsafe { RpcSmDestroyClientContext(contexthandle) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcSmDisableAllocate() -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSmDisableAllocate() -> super::rpc::RPC_STATUS);
    unsafe { RpcSmDisableAllocate() }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcSmEnableAllocate() -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSmEnableAllocate() -> super::rpc::RPC_STATUS);
    unsafe { RpcSmEnableAllocate() }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcSmFree(nodetofree: *const core::ffi::c_void) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSmFree(nodetofree : *const core::ffi::c_void) -> super::rpc::RPC_STATUS);
    unsafe { RpcSmFree(nodetofree) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcSmGetThreadHandle(pstatus: *mut super::rpc::RPC_STATUS) -> RPC_SS_THREAD_HANDLE {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSmGetThreadHandle(pstatus : *mut super::rpc::RPC_STATUS) -> RPC_SS_THREAD_HANDLE);
    unsafe { RpcSmGetThreadHandle(pstatus as _) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcSmSetClientAllocFree(clientalloc: RPC_CLIENT_ALLOC, clientfree: RPC_CLIENT_FREE) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSmSetClientAllocFree(clientalloc : RPC_CLIENT_ALLOC, clientfree : RPC_CLIENT_FREE) -> super::rpc::RPC_STATUS);
    unsafe { RpcSmSetClientAllocFree(clientalloc, clientfree) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcSmSetThreadHandle(id: RPC_SS_THREAD_HANDLE) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSmSetThreadHandle(id : RPC_SS_THREAD_HANDLE) -> super::rpc::RPC_STATUS);
    unsafe { RpcSmSetThreadHandle(id) }
}
#[cfg(feature = "Win32_rpc")]
#[inline]
pub unsafe fn RpcSmSwapClientAllocFree(clientalloc: RPC_CLIENT_ALLOC, clientfree: RPC_CLIENT_FREE, oldclientalloc: *mut RPC_CLIENT_ALLOC, oldclientfree: *mut RPC_CLIENT_FREE) -> super::rpc::RPC_STATUS {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSmSwapClientAllocFree(clientalloc : RPC_CLIENT_ALLOC, clientfree : RPC_CLIENT_FREE, oldclientalloc : *mut RPC_CLIENT_ALLOC, oldclientfree : *mut RPC_CLIENT_FREE) -> super::rpc::RPC_STATUS);
    unsafe { RpcSmSwapClientAllocFree(clientalloc, clientfree, oldclientalloc as _, oldclientfree as _) }
}
#[inline]
pub unsafe fn RpcSsAllocate(size: usize) -> *mut core::ffi::c_void {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSsAllocate(size : usize) -> *mut core::ffi::c_void);
    unsafe { RpcSsAllocate(size) }
}
#[inline]
pub unsafe fn RpcSsDestroyClientContext(contexthandle: *const *const core::ffi::c_void) {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSsDestroyClientContext(contexthandle : *const *const core::ffi::c_void));
    unsafe { RpcSsDestroyClientContext(contexthandle) }
}
#[inline]
pub unsafe fn RpcSsDisableAllocate() {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSsDisableAllocate());
    unsafe { RpcSsDisableAllocate() }
}
#[inline]
pub unsafe fn RpcSsEnableAllocate() {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSsEnableAllocate());
    unsafe { RpcSsEnableAllocate() }
}
#[inline]
pub unsafe fn RpcSsFree(nodetofree: *const core::ffi::c_void) {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSsFree(nodetofree : *const core::ffi::c_void));
    unsafe { RpcSsFree(nodetofree) }
}
#[inline]
pub unsafe fn RpcSsGetThreadHandle() -> RPC_SS_THREAD_HANDLE {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSsGetThreadHandle() -> RPC_SS_THREAD_HANDLE);
    unsafe { RpcSsGetThreadHandle() }
}
#[inline]
pub unsafe fn RpcSsSetClientAllocFree(clientalloc: RPC_CLIENT_ALLOC, clientfree: RPC_CLIENT_FREE) {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSsSetClientAllocFree(clientalloc : RPC_CLIENT_ALLOC, clientfree : RPC_CLIENT_FREE));
    unsafe { RpcSsSetClientAllocFree(clientalloc, clientfree) }
}
#[inline]
pub unsafe fn RpcSsSetThreadHandle(id: RPC_SS_THREAD_HANDLE) {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSsSetThreadHandle(id : RPC_SS_THREAD_HANDLE));
    unsafe { RpcSsSetThreadHandle(id) }
}
#[inline]
pub unsafe fn RpcSsSwapClientAllocFree(clientalloc: RPC_CLIENT_ALLOC, clientfree: RPC_CLIENT_FREE, oldclientalloc: *mut RPC_CLIENT_ALLOC, oldclientfree: *mut RPC_CLIENT_FREE) {
    windows_core::link!("rpcrt4.dll" "system" fn RpcSsSwapClientAllocFree(clientalloc : RPC_CLIENT_ALLOC, clientfree : RPC_CLIENT_FREE, oldclientalloc : *mut RPC_CLIENT_ALLOC, oldclientfree : *mut RPC_CLIENT_FREE));
    unsafe { RpcSsSwapClientAllocFree(clientalloc, clientfree, oldclientalloc as _, oldclientfree as _) }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[inline]
pub unsafe fn RpcUserFree(asynchandle: super::rpcdce::handle_t, pbuffer: *mut core::ffi::c_void) {
    windows_core::link!("rpcrt4.dll" "system" fn RpcUserFree(asynchandle : super::rpcdce::handle_t, pbuffer : *mut core::ffi::c_void));
    unsafe { RpcUserFree(asynchandle, pbuffer as _) }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ARRAY_INFO {
    pub Dimension: i32,
    pub BufferConformanceMark: *mut u32,
    pub BufferVarianceMark: *mut u32,
    pub MaxCountArray: *mut u32,
    pub OffsetArray: *mut u32,
    pub ActualCountArray: *mut u32,
}
impl Default for ARRAY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CLIENT_CALL_RETURN {
    pub Pointer: *mut core::ffi::c_void,
    pub Simple: isize,
}
impl Default for CLIENT_CALL_RETURN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct COMM_FAULT_OFFSETS {
    pub CommOffset: i16,
    pub FaultOffset: i16,
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
pub type CS_TAG_GETTING_ROUTINE = Option<unsafe extern "system" fn(hbinding: super::rpcdce::RPC_BINDING_HANDLE, fserverside: i32, pulsendingtag: *mut u32, puldesiredreceivingtag: *mut u32, pulreceivingtag: *mut u32, pstatus: *mut error_status_t)>;
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
pub type CS_TYPE_FROM_NETCS_ROUTINE = Option<unsafe extern "system" fn(hbinding: super::rpcdce::RPC_BINDING_HANDLE, ulnetworkcodeset: u32, pnetworkdata: *mut byte, ulnetworkdatalength: u32, ullocalbuffersize: u32, plocaldata: *mut core::ffi::c_void, pullocaldatalength: *mut u32, pstatus: *mut error_status_t)>;
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
pub type CS_TYPE_LOCAL_SIZE_ROUTINE = Option<unsafe extern "system" fn(hbinding: super::rpcdce::RPC_BINDING_HANDLE, ulnetworkcodeset: u32, ulnetworkbuffersize: u32, conversiontype: *mut IDL_CS_CONVERT, pullocalbuffersize: *mut u32, pstatus: *mut error_status_t)>;
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
pub type CS_TYPE_NET_SIZE_ROUTINE = Option<unsafe extern "system" fn(hbinding: super::rpcdce::RPC_BINDING_HANDLE, ulnetworkcodeset: u32, ullocalbuffersize: u32, conversiontype: *mut IDL_CS_CONVERT, pulnetworkbuffersize: *mut u32, pstatus: *mut error_status_t)>;
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
pub type CS_TYPE_TO_NETCS_ROUTINE = Option<unsafe extern "system" fn(hbinding: super::rpcdce::RPC_BINDING_HANDLE, ulnetworkcodeset: u32, plocaldata: *mut core::ffi::c_void, ullocaldatalength: u32, pnetworkdata: *mut byte, pulnetworkdatalength: *mut u32, pstatus: *mut error_status_t)>;
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
pub type EXPR_EVAL = Option<unsafe extern "system" fn(param0: *mut MIDL_STUB_MESSAGE)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FULL_PTR_XLAT_TABLES {
    pub RefIdToPointer: *mut core::ffi::c_void,
    pub PointerToRefId: *mut core::ffi::c_void,
    pub NextRefId: u32,
    pub XlatSide: XLAT_SIDE,
}
impl Default for FULL_PTR_XLAT_TABLES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct GENERIC_BINDING_INFO {
    pub pObj: *mut core::ffi::c_void,
    pub Size: u32,
    pub pfnBind: GENERIC_BINDING_ROUTINE,
    pub pfnUnbind: GENERIC_UNBIND_ROUTINE,
}
impl Default for GENERIC_BINDING_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type GENERIC_BINDING_ROUTINE = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void) -> *mut core::ffi::c_void>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct GENERIC_BINDING_ROUTINE_PAIR {
    pub pfnBind: GENERIC_BINDING_ROUTINE,
    pub pfnUnbind: GENERIC_UNBIND_ROUTINE,
}
pub type GENERIC_UNBIND_ROUTINE = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void, param1: *mut u8)>;
pub type IDL_CS_CONVERT = i32;
pub const IDL_CS_IN_PLACE_CONVERT: IDL_CS_CONVERT = 1;
pub const IDL_CS_NEW_BUFFER_CONVERT: IDL_CS_CONVERT = 2;
pub const IDL_CS_NO_CONVERT: IDL_CS_CONVERT = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MALLOC_FREE_STRUCT {
    pub pfnAllocate: *mut u8,
    pub pfnFree: *mut u8,
}
impl Default for MALLOC_FREE_STRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIDL_FORMAT_STRING {
    pub Pad: i16,
    pub Format: [u8; 0],
}
impl Default for MIDL_FORMAT_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIDL_INTERCEPTION_INFO {
    pub Version: u32,
    pub ProcString: PFORMAT_STRING,
    pub ProcFormatOffsetTable: *const u16,
    pub ProcCount: u32,
    pub TypeString: PFORMAT_STRING,
}
impl Default for MIDL_INTERCEPTION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIDL_INTERFACE_METHOD_PROPERTIES {
    pub MethodCount: u16,
    pub MethodProperties: *const *const MIDL_METHOD_PROPERTY_MAP,
}
impl Default for MIDL_INTERFACE_METHOD_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MIDL_METHOD_PROPERTY {
    pub Id: u32,
    pub Value: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIDL_METHOD_PROPERTY_MAP {
    pub Count: u32,
    pub Properties: *const MIDL_METHOD_PROPERTY,
}
impl Default for MIDL_METHOD_PROPERTY_MAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIDL_SERVER_INFO {
    pub pStubDesc: PMIDL_STUB_DESC,
    pub DispatchTable: *const SERVER_ROUTINE,
    pub ProcString: PFORMAT_STRING,
    pub FmtStringOffset: *const u16,
    pub ThunkTable: *const STUB_THUNK,
    pub pTransferSyntax: super::rpcdcep::PRPC_SYNTAX_IDENTIFIER,
    pub nCount: usize,
    pub pSyntaxInfo: PMIDL_SYNTAX_INFO,
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
impl Default for MIDL_SERVER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIDL_STUBLESS_PROXY_INFO {
    pub pStubDesc: PMIDL_STUB_DESC,
    pub ProcFormatString: PFORMAT_STRING,
    pub FormatStringOffset: *const u16,
    pub pTransferSyntax: super::rpcdcep::PRPC_SYNTAX_IDENTIFIER,
    pub nCount: usize,
    pub pSyntaxInfo: PMIDL_SYNTAX_INFO,
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
impl Default for MIDL_STUBLESS_PROXY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[derive(Clone, Copy)]
pub struct MIDL_STUB_DESC {
    pub RpcInterfaceInformation: *mut core::ffi::c_void,
    pub pfnAllocate: *mut u8,
    pub pfnFree: *mut u8,
    pub IMPLICIT_HANDLE_INFO: MIDL_STUB_DESC_0,
    pub apfnNdrRundownRoutines: *const NDR_RUNDOWN,
    pub aGenericBindingRoutinePairs: *const GENERIC_BINDING_ROUTINE_PAIR,
    pub apfnExprEval: *const EXPR_EVAL,
    pub aXmitQuintuple: *const XMIT_ROUTINE_QUINTUPLE,
    pub pFormatTypes: *const u8,
    pub fCheckBounds: i32,
    pub Version: u32,
    pub pMallocFreeStruct: *mut MALLOC_FREE_STRUCT,
    pub MIDLVersion: i32,
    pub CommFaultOffsets: *const COMM_FAULT_OFFSETS,
    pub aUserMarshalQuadruple: *const USER_MARSHAL_ROUTINE_QUADRUPLE,
    pub NotifyRoutineTable: *const NDR_NOTIFY_ROUTINE,
    pub mFlags: usize,
    pub CsRoutineTables: *const NDR_CS_ROUTINES,
    pub ProxyServerInfo: *mut core::ffi::c_void,
    pub pExprInfo: *const NDR_EXPR_DESC,
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
impl Default for MIDL_STUB_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[derive(Clone, Copy)]
pub union MIDL_STUB_DESC_0 {
    pub pAutoHandle: *mut super::rpcdce::handle_t,
    pub pPrimitiveHandle: *mut super::rpcdce::handle_t,
    pub pGenericBindingInfo: PGENERIC_BINDING_INFO,
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
impl Default for MIDL_STUB_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[derive(Clone, Debug, PartialEq)]
pub struct MIDL_STUB_MESSAGE {
    pub RpcMsg: super::rpcdcep::PRPC_MESSAGE,
    pub Buffer: *mut u8,
    pub BufferStart: *mut u8,
    pub BufferEnd: *mut u8,
    pub BufferMark: *mut u8,
    pub BufferLength: u32,
    pub MemorySize: u32,
    pub Memory: *mut u8,
    pub IsClient: u8,
    pub Pad: u8,
    pub uFlags2: u16,
    pub ReuseBuffer: i32,
    pub pAllocAllNodesContext: *mut NDR_ALLOC_ALL_NODES_CONTEXT,
    pub pPointerQueueState: *mut NDR_POINTER_QUEUE_STATE,
    pub IgnoreEmbeddedPointers: i32,
    pub PointerBufferMark: *mut u8,
    pub CorrDespIncrement: u8,
    pub uFlags: u8,
    pub UniquePtrCount: u16,
    pub MaxCount: usize,
    pub Offset: u32,
    pub ActualCount: u32,
    pub pfnAllocate: *mut u8,
    pub pfnFree: *mut u8,
    pub StackTop: *mut u8,
    pub pPresentedType: *mut u8,
    pub pTransmitType: *mut u8,
    pub SavedHandle: super::rpcdce::handle_t,
    pub StubDesc: *const MIDL_STUB_DESC,
    pub FullPtrXlatTables: *mut FULL_PTR_XLAT_TABLES,
    pub FullPtrRefId: u32,
    pub PointerLength: u32,
    pub _bitfield: u32,
    pub dwDestContext: u32,
    pub pvDestContext: *mut core::ffi::c_void,
    pub SavedContextHandles: *mut NDR_SCONTEXT,
    pub ParamNumber: i32,
    pub pRpcChannelBuffer: core::mem::ManuallyDrop<Option<super::objidlbase::IRpcChannelBuffer>>,
    pub pArrayInfo: PARRAY_INFO,
    pub SizePtrCountArray: *mut u32,
    pub SizePtrOffsetArray: *mut u32,
    pub SizePtrLengthArray: *mut u32,
    pub pArgQueue: *mut core::ffi::c_void,
    pub dwStubPhase: u32,
    pub LowStackMark: *mut core::ffi::c_void,
    pub pAsyncMsg: PNDR_ASYNC_MESSAGE,
    pub pCorrInfo: PNDR_CORRELATION_INFO,
    pub pCorrMemory: *mut u8,
    pub pMemoryList: *mut core::ffi::c_void,
    pub pCSInfo: isize,
    pub ConformanceMark: *mut u8,
    pub VarianceMark: *mut u8,
    pub Unused: isize,
    pub pContext: *mut _NDR_PROC_CONTEXT,
    pub ContextHandleHash: *mut core::ffi::c_void,
    pub pUserMarshalList: *mut core::ffi::c_void,
    pub pFullPtrFormat: *mut u8,
    pub Reserved51_4: isize,
    pub Reserved51_5: isize,
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
impl Default for MIDL_STUB_MESSAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIDL_SYNTAX_INFO {
    pub TransferSyntax: super::rpcdcep::RPC_SYNTAX_IDENTIFIER,
    pub DispatchTable: *mut super::rpcdcep::RPC_DISPATCH_TABLE,
    pub ProcString: PFORMAT_STRING,
    pub FmtStringOffset: *const u16,
    pub TypeString: PFORMAT_STRING,
    pub aUserMarshalQuadruple: *const core::ffi::c_void,
    pub pMethodProperties: *const MIDL_INTERFACE_METHOD_PROPERTIES,
    pub pReserved2: usize,
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
impl Default for MIDL_SYNTAX_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MIDL_WINRT_TYPE_SERIALIZATION_INFO {
    pub Version: u32,
    pub TypeFormatString: PFORMAT_STRING,
    pub FormatStringSize: u16,
    pub TypeOffset: u16,
    pub StubDesc: PMIDL_STUB_DESC,
}
pub const MIDL_WINRT_TYPE_SERIALIZATION_INFO_CURRENT_VERSION: u32 = 1;
pub const MidlInterceptionInfoVersionOne: i32 = 1;
pub const MidlWinrtTypeSerializationInfoVersionOne: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NDR_ALLOC_ALL_NODES_CONTEXT(pub u8);
pub const NDR_ASCII_CHAR: u32 = 0;
pub const NDR_BIG_ENDIAN: u32 = 0;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NDR_CCONTEXT(pub *mut core::ffi::c_void);
impl NDR_CCONTEXT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for NDR_CCONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NDR_CHAR_REP_MASK: u32 = 15;
#[repr(C)]
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR_CS_ROUTINES {
    pub pSizeConvertRoutines: *mut NDR_CS_SIZE_CONVERT_ROUTINES,
    pub pTagGettingRoutines: *mut CS_TAG_GETTING_ROUTINE,
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
impl Default for NDR_CS_ROUTINES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct NDR_CS_SIZE_CONVERT_ROUTINES {
    pub pfnNetSize: CS_TYPE_NET_SIZE_ROUTINE,
    pub pfnToNetCs: CS_TYPE_TO_NETCS_ROUTINE,
    pub pfnLocalSize: CS_TYPE_LOCAL_SIZE_ROUTINE,
    pub pfnFromNetCs: CS_TYPE_FROM_NETCS_ROUTINE,
}
pub const NDR_EBCDIC_CHAR: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR_EXPR_DESC {
    pub pOffset: *const u16,
    pub pFormatExpr: PFORMAT_STRING,
}
impl Default for NDR_EXPR_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NDR_FLOAT_REP_MASK: u32 = 65280;
pub const NDR_IBM_FLOAT: u32 = 768;
pub const NDR_IEEE_FLOAT: u32 = 0;
pub const NDR_INT_REP_MASK: u32 = 240;
pub const NDR_LITTLE_ENDIAN: u32 = 16;
pub const NDR_LOCAL_DATA_REPRESENTATION: u32 = 16;
pub const NDR_LOCAL_ENDIAN: u32 = 16;
pub type NDR_NOTIFY2_ROUTINE = Option<unsafe extern "system" fn(flag: boolean)>;
pub type NDR_NOTIFY_ROUTINE = Option<unsafe extern "system" fn()>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NDR_POINTER_QUEUE_STATE(pub u8);
pub type NDR_RUNDOWN = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NDR_SCONTEXT(pub *mut _NDR_SCONTEXT);
impl NDR_SCONTEXT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for NDR_SCONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_objidlbase")]
pub struct NDR_USER_MARSHAL_INFO {
    pub InformationLevel: u32,
    pub Anonymous: NDR_USER_MARSHAL_INFO_0,
}
#[cfg(feature = "Win32_objidlbase")]
impl Clone for NDR_USER_MARSHAL_INFO {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_objidlbase")]
impl Default for NDR_USER_MARSHAL_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_objidlbase")]
pub union NDR_USER_MARSHAL_INFO_0 {
    pub Level1: core::mem::ManuallyDrop<NDR_USER_MARSHAL_INFO_LEVEL1>,
}
#[cfg(feature = "Win32_objidlbase")]
impl Clone for NDR_USER_MARSHAL_INFO_0 {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_objidlbase")]
impl Default for NDR_USER_MARSHAL_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_objidlbase")]
#[derive(Clone, Debug, PartialEq)]
pub struct NDR_USER_MARSHAL_INFO_LEVEL1 {
    pub Buffer: *mut core::ffi::c_void,
    pub BufferSize: u32,
    pub pfnAllocate: *mut u8,
    pub pfnFree: *mut u8,
    pub pRpcChannelBuffer: core::mem::ManuallyDrop<Option<super::objidlbase::IRpcChannelBuffer>>,
    pub Reserved: [usize; 5],
}
#[cfg(feature = "Win32_objidlbase")]
impl Default for NDR_USER_MARSHAL_INFO_LEVEL1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NDR_VAX_FLOAT: u32 = 256;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PARAM_OFFSETTABLE(pub *mut u16);
impl PARAM_OFFSETTABLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PARAM_OFFSETTABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PARRAY_INFO(pub *mut ARRAY_INFO);
impl PARRAY_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PARRAY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFORMAT_STRING(pub *const u8);
impl PFORMAT_STRING {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFORMAT_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFULL_PTR_XLAT_TABLES(pub *mut FULL_PTR_XLAT_TABLES);
impl PFULL_PTR_XLAT_TABLES {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PFULL_PTR_XLAT_TABLES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PGENERIC_BINDING_INFO(pub *mut GENERIC_BINDING_INFO);
impl PGENERIC_BINDING_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PGENERIC_BINDING_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PGENERIC_BINDING_ROUTINE_PAIR(pub *mut GENERIC_BINDING_ROUTINE_PAIR);
impl PGENERIC_BINDING_ROUTINE_PAIR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PGENERIC_BINDING_ROUTINE_PAIR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMIDL_INTERCEPTION_INFO(pub *mut MIDL_INTERCEPTION_INFO);
impl PMIDL_INTERCEPTION_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PMIDL_INTERCEPTION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMIDL_METHOD_PROPERTY(pub *mut MIDL_METHOD_PROPERTY);
impl PMIDL_METHOD_PROPERTY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PMIDL_METHOD_PROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMIDL_METHOD_PROPERTY_MAP(pub *mut MIDL_METHOD_PROPERTY_MAP);
impl PMIDL_METHOD_PROPERTY_MAP {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PMIDL_METHOD_PROPERTY_MAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMIDL_SERVER_INFO(pub *mut MIDL_SERVER_INFO);
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
impl PMIDL_SERVER_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
impl Default for PMIDL_SERVER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMIDL_STUBLESS_PROXY_INFO(pub *mut MIDL_STUBLESS_PROXY_INFO);
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
impl PMIDL_STUBLESS_PROXY_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
impl Default for PMIDL_STUBLESS_PROXY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMIDL_STUB_DESC(pub *const MIDL_STUB_DESC);
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
impl PMIDL_STUB_DESC {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
impl Default for PMIDL_STUB_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMIDL_STUB_MESSAGE(pub *mut MIDL_STUB_MESSAGE);
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
impl PMIDL_STUB_MESSAGE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
impl Default for PMIDL_STUB_MESSAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMIDL_SYNTAX_INFO(pub *mut MIDL_SYNTAX_INFO);
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
impl PMIDL_SYNTAX_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
impl Default for PMIDL_SYNTAX_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMIDL_WINRT_TYPE_SERIALIZATION_INFO(pub *mut MIDL_WINRT_TYPE_SERIALIZATION_INFO);
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
impl PMIDL_WINRT_TYPE_SERIALIZATION_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
impl Default for PMIDL_WINRT_TYPE_SERIALIZATION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PMIDL_XMIT_TYPE(pub *mut core::ffi::c_void);
impl PMIDL_XMIT_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PMIDL_XMIT_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PNDR_ASYNC_MESSAGE(pub *mut _NDR_ASYNC_MESSAGE);
impl PNDR_ASYNC_MESSAGE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PNDR_ASYNC_MESSAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PNDR_CORRELATION_INFO(pub *mut _NDR_CORRELATION_INFO);
impl PNDR_CORRELATION_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PNDR_CORRELATION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPARAM_OFFSETTABLE(pub *mut u16);
impl PPARAM_OFFSETTABLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPARAM_OFFSETTABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PROXY_CALCSIZE: PROXY_PHASE = 0;
pub const PROXY_GETBUFFER: PROXY_PHASE = 1;
pub const PROXY_MARSHAL: PROXY_PHASE = 2;
pub type PROXY_PHASE = i32;
pub const PROXY_SENDRECEIVE: PROXY_PHASE = 3;
pub const PROXY_UNMARSHAL: PROXY_PHASE = 4;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PSCONTEXT_QUEUE(pub *mut SCONTEXT_QUEUE);
impl PSCONTEXT_QUEUE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PSCONTEXT_QUEUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PXMIT_ROUTINE_QUINTUPLE(pub *mut XMIT_ROUTINE_QUINTUPLE);
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
impl PXMIT_ROUTINE_QUINTUPLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
impl Default for PXMIT_ROUTINE_QUINTUPLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RPC_BUFPTR(pub *mut u8);
impl RPC_BUFPTR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for RPC_BUFPTR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type RPC_CLIENT_ALLOC = Option<unsafe extern "system" fn(size: usize) -> *mut core::ffi::c_void>;
pub type RPC_CLIENT_FREE = Option<unsafe extern "system" fn(ptr: *const core::ffi::c_void)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct RPC_LENGTH(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RPC_SS_THREAD_HANDLE(pub *mut core::ffi::c_void);
impl RPC_SS_THREAD_HANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for RPC_SS_THREAD_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCONTEXT_QUEUE {
    pub NumberOfObjects: u32,
    pub ArrayOfObjects: *mut NDR_SCONTEXT,
}
impl Default for SCONTEXT_QUEUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SERVER_ROUTINE = Option<unsafe extern "system" fn() -> i32>;
pub const STUB_CALL_SERVER: STUB_PHASE = 1;
pub const STUB_CALL_SERVER_NO_HRESULT: STUB_PHASE = 3;
pub const STUB_MARSHAL: STUB_PHASE = 2;
pub type STUB_PHASE = i32;
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
pub type STUB_THUNK = Option<unsafe extern "system" fn(param0: *mut MIDL_STUB_MESSAGE)>;
pub const STUB_UNMARSHAL: STUB_PHASE = 0;
pub const SYSTEM_HANDLE_COMPOSITION_OBJECT: system_handle_t = 9;
pub const SYSTEM_HANDLE_EVENT: system_handle_t = 2;
pub const SYSTEM_HANDLE_FILE: system_handle_t = 0;
pub const SYSTEM_HANDLE_INVALID: system_handle_t = 255;
pub const SYSTEM_HANDLE_JOB: system_handle_t = 11;
pub const SYSTEM_HANDLE_MAX: system_handle_t = 12;
pub const SYSTEM_HANDLE_MUTEX: system_handle_t = 3;
pub const SYSTEM_HANDLE_PIPE: system_handle_t = 12;
pub const SYSTEM_HANDLE_PROCESS: system_handle_t = 4;
pub const SYSTEM_HANDLE_REG_KEY: system_handle_t = 7;
pub const SYSTEM_HANDLE_SECTION: system_handle_t = 6;
pub const SYSTEM_HANDLE_SEMAPHORE: system_handle_t = 1;
pub const SYSTEM_HANDLE_SOCKET: system_handle_t = 10;
pub const SYSTEM_HANDLE_THREAD: system_handle_t = 8;
pub const SYSTEM_HANDLE_TOKEN: system_handle_t = 5;
pub const TARGET_IS_NT100_OR_LATER: u32 = 1;
pub const TARGET_IS_NT1012_OR_LATER: u32 = 1;
pub const TARGET_IS_NT102_OR_LATER: u32 = 1;
pub const TARGET_IS_NT351_OR_WIN95_OR_LATER: u32 = 1;
pub const TARGET_IS_NT40_OR_LATER: u32 = 1;
pub const TARGET_IS_NT50_OR_LATER: u32 = 1;
pub const TARGET_IS_NT51_OR_LATER: u32 = 1;
pub const TARGET_IS_NT60_OR_LATER: u32 = 1;
pub const TARGET_IS_NT61_OR_LATER: u32 = 1;
pub const TARGET_IS_NT62_OR_LATER: u32 = 1;
pub const TARGET_IS_NT63_OR_LATER: u32 = 1;
pub const USER_CALL_IS_ASYNC: u32 = 256;
pub const USER_CALL_NEW_CORRELATION_DESC: u32 = 512;
#[repr(C)]
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct USER_MARSHAL_CB {
    pub Flags: u32,
    pub pStubMsg: PMIDL_STUB_MESSAGE,
    pub pReserve: PFORMAT_STRING,
    pub Signature: u32,
    pub CBType: USER_MARSHAL_CB_TYPE,
    pub pFormat: PFORMAT_STRING,
    pub pTypeFormat: PFORMAT_STRING,
}
pub const USER_MARSHAL_CB_BUFFER_SIZE: USER_MARSHAL_CB_TYPE = 0;
pub const USER_MARSHAL_CB_FREE: USER_MARSHAL_CB_TYPE = 3;
pub const USER_MARSHAL_CB_MARSHALL: USER_MARSHAL_CB_TYPE = 1;
pub const USER_MARSHAL_CB_SIGNATURE: u32 = 1431523907;
pub type USER_MARSHAL_CB_TYPE = i32;
pub const USER_MARSHAL_CB_UNMARSHALL: USER_MARSHAL_CB_TYPE = 2;
pub const USER_MARSHAL_FC_BYTE: u32 = 1;
pub const USER_MARSHAL_FC_CHAR: u32 = 2;
pub const USER_MARSHAL_FC_DOUBLE: u32 = 12;
pub const USER_MARSHAL_FC_FLOAT: u32 = 10;
pub const USER_MARSHAL_FC_HYPER: u32 = 11;
pub const USER_MARSHAL_FC_LONG: u32 = 8;
pub const USER_MARSHAL_FC_SHORT: u32 = 6;
pub const USER_MARSHAL_FC_SMALL: u32 = 3;
pub const USER_MARSHAL_FC_ULONG: u32 = 9;
pub const USER_MARSHAL_FC_USHORT: u32 = 7;
pub const USER_MARSHAL_FC_USMALL: u32 = 4;
pub const USER_MARSHAL_FC_WCHAR: u32 = 5;
pub type USER_MARSHAL_FREEING_ROUTINE = Option<unsafe extern "system" fn(param0: *mut u32, param1: *mut core::ffi::c_void)>;
pub type USER_MARSHAL_MARSHALLING_ROUTINE = Option<unsafe extern "system" fn(param0: *mut u32, param1: *mut u8, param2: *mut core::ffi::c_void) -> *mut u8>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct USER_MARSHAL_ROUTINE_QUADRUPLE {
    pub pfnBufferSize: USER_MARSHAL_SIZING_ROUTINE,
    pub pfnMarshall: USER_MARSHAL_MARSHALLING_ROUTINE,
    pub pfnUnmarshall: USER_MARSHAL_UNMARSHALLING_ROUTINE,
    pub pfnFree: USER_MARSHAL_FREEING_ROUTINE,
}
pub type USER_MARSHAL_SIZING_ROUTINE = Option<unsafe extern "system" fn(param0: *mut u32, param1: u32, param2: *mut core::ffi::c_void) -> u32>;
pub type USER_MARSHAL_UNMARSHALLING_ROUTINE = Option<unsafe extern "system" fn(param0: *mut u32, param1: *mut u8, param2: *mut core::ffi::c_void) -> *mut u8>;
pub const XLAT_CLIENT: XLAT_SIDE = 2;
pub const XLAT_SERVER: XLAT_SIDE = 1;
pub type XLAT_SIDE = i32;
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
pub type XMIT_HELPER_ROUTINE = Option<unsafe extern "system" fn(param0: *mut MIDL_STUB_MESSAGE)>;
#[repr(C)]
#[cfg(all(feature = "Win32_objidlbase", feature = "Win32_rpc", feature = "Win32_rpcdce", feature = "Win32_rpcdcep"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct XMIT_ROUTINE_QUINTUPLE {
    pub pfnTranslateToXmit: XMIT_HELPER_ROUTINE,
    pub pfnTranslateFromXmit: XMIT_HELPER_ROUTINE,
    pub pfnFreeXmit: XMIT_HELPER_ROUTINE,
    pub pfnFreeInst: XMIT_HELPER_ROUTINE,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _NDR_ASYNC_MESSAGE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _NDR_CORRELATION_INFO(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _NDR_PROC_CONTEXT(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct _NDR_SCONTEXT {
    pub pad: [*mut core::ffi::c_void; 2],
    pub userContext: *mut core::ffi::c_void,
}
impl Default for _NDR_SCONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct boolean(pub u8);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct byte(pub u8);
pub const cbNDRContext: u32 = 20;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct cs_byte(pub byte);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct error_status_t(pub u32);
pub type system_handle_t = i32;
