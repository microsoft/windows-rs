#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct ARRAY_INFO {
    pub Dimension: i32,
    pub BufferConformanceMark: *mut u32,
    pub BufferVarianceMark: *mut u32,
    pub MaxCountArray: *mut u32,
    pub OffsetArray: *mut u32,
    pub ActualCountArray: *mut u32,
}
impl ::core::marker::Copy for ARRAY_INFO {}
impl ::core::clone::Clone for ARRAY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ARRAY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ARRAY_INFO").field("Dimension", &self.Dimension).field("BufferConformanceMark", &self.BufferConformanceMark).field("BufferVarianceMark", &self.BufferVarianceMark).field("MaxCountArray", &self.MaxCountArray).field("OffsetArray", &self.OffsetArray).field("ActualCountArray", &self.ActualCountArray).finish()
    }
}
unsafe impl ::windows::core::Abi for ARRAY_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for ARRAY_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ARRAY_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for ARRAY_INFO {}
impl ::core::default::Default for ARRAY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct BinaryParam {
    pub Buffer: *mut ::core::ffi::c_void,
    pub Size: i16,
}
impl ::core::marker::Copy for BinaryParam {}
impl ::core::clone::Clone for BinaryParam {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for BinaryParam {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("BinaryParam").field("Buffer", &self.Buffer).field("Size", &self.Size).finish()
    }
}
unsafe impl ::windows::core::Abi for BinaryParam {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for BinaryParam {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<BinaryParam>()) == 0 }
    }
}
impl ::core::cmp::Eq for BinaryParam {}
impl ::core::default::Default for BinaryParam {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub union CLIENT_CALL_RETURN {
    pub Pointer: *mut ::core::ffi::c_void,
    pub Simple: isize,
}
impl ::core::marker::Copy for CLIENT_CALL_RETURN {}
impl ::core::clone::Clone for CLIENT_CALL_RETURN {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for CLIENT_CALL_RETURN {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for CLIENT_CALL_RETURN {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<CLIENT_CALL_RETURN>()) == 0 }
    }
}
impl ::core::cmp::Eq for CLIENT_CALL_RETURN {}
impl ::core::default::Default for CLIENT_CALL_RETURN {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct COMM_FAULT_OFFSETS {
    pub CommOffset: i16,
    pub FaultOffset: i16,
}
impl ::core::marker::Copy for COMM_FAULT_OFFSETS {}
impl ::core::clone::Clone for COMM_FAULT_OFFSETS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for COMM_FAULT_OFFSETS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMM_FAULT_OFFSETS").field("CommOffset", &self.CommOffset).field("FaultOffset", &self.FaultOffset).finish()
    }
}
unsafe impl ::windows::core::Abi for COMM_FAULT_OFFSETS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for COMM_FAULT_OFFSETS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<COMM_FAULT_OFFSETS>()) == 0 }
    }
}
impl ::core::cmp::Eq for COMM_FAULT_OFFSETS {}
impl ::core::default::Default for COMM_FAULT_OFFSETS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type CS_TAG_GETTING_ROUTINE = ::core::option::Option<unsafe extern "system" fn(hbinding: *mut ::core::ffi::c_void, fserverside: i32, pulsendingtag: *mut u32, puldesiredreceivingtag: *mut u32, pulreceivingtag: *mut u32, pstatus: *mut u32)>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type CS_TYPE_FROM_NETCS_ROUTINE = ::core::option::Option<unsafe extern "system" fn(hbinding: *mut ::core::ffi::c_void, ulnetworkcodeset: u32, pnetworkdata: *mut u8, ulnetworkdatalength: u32, ullocalbuffersize: u32, plocaldata: *mut ::core::ffi::c_void, pullocaldatalength: *mut u32, pstatus: *mut u32)>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type CS_TYPE_LOCAL_SIZE_ROUTINE = ::core::option::Option<unsafe extern "system" fn(hbinding: *mut ::core::ffi::c_void, ulnetworkcodeset: u32, ulnetworkbuffersize: u32, conversiontype: *mut IDL_CS_CONVERT, pullocalbuffersize: *mut u32, pstatus: *mut u32)>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type CS_TYPE_NET_SIZE_ROUTINE = ::core::option::Option<unsafe extern "system" fn(hbinding: *mut ::core::ffi::c_void, ulnetworkcodeset: u32, ullocalbuffersize: u32, conversiontype: *mut IDL_CS_CONVERT, pulnetworkbuffersize: *mut u32, pstatus: *mut u32)>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type CS_TYPE_TO_NETCS_ROUTINE = ::core::option::Option<unsafe extern "system" fn(hbinding: *mut ::core::ffi::c_void, ulnetworkcodeset: u32, plocaldata: *mut ::core::ffi::c_void, ullocaldatalength: u32, pnetworkdata: *mut u8, pulnetworkdatalength: *mut u32, pstatus: *mut u32)>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const DCE_C_ERROR_STRING_LEN: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn DceErrorInqTextA(rpcstatus: RPC_STATUS, errortext: &mut [u8; 256]) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DceErrorInqTextA(rpcstatus: RPC_STATUS, errortext: *mut u8) -> RPC_STATUS;
        }
        ::core::mem::transmute(DceErrorInqTextA(::core::mem::transmute(rpcstatus), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(errortext))))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn DceErrorInqTextW(rpcstatus: RPC_STATUS, errortext: &mut [u16; 256]) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DceErrorInqTextW(rpcstatus: RPC_STATUS, errortext: *mut u16) -> RPC_STATUS;
        }
        ::core::mem::transmute(DceErrorInqTextW(::core::mem::transmute(rpcstatus), ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(errortext))))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const EEInfoGCCOM: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const EEInfoGCFRS: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const EEInfoNextRecordsMissing: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const EEInfoPreviousRecordsMissing: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const EEInfoUseFileTime: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub type EXPR_EVAL = ::core::option::Option<unsafe extern "system" fn(param0: *mut MIDL_STUB_MESSAGE)>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EXPR_TOKEN(pub i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const FC_EXPR_START: EXPR_TOKEN = EXPR_TOKEN(0i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const FC_EXPR_ILLEGAL: EXPR_TOKEN = EXPR_TOKEN(0i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const FC_EXPR_CONST32: EXPR_TOKEN = EXPR_TOKEN(1i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const FC_EXPR_CONST64: EXPR_TOKEN = EXPR_TOKEN(2i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const FC_EXPR_VAR: EXPR_TOKEN = EXPR_TOKEN(3i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const FC_EXPR_OPER: EXPR_TOKEN = EXPR_TOKEN(4i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const FC_EXPR_NOOP: EXPR_TOKEN = EXPR_TOKEN(5i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const FC_EXPR_END: EXPR_TOKEN = EXPR_TOKEN(6i32);
impl ::core::marker::Copy for EXPR_TOKEN {}
impl ::core::clone::Clone for EXPR_TOKEN {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EXPR_TOKEN {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EXPR_TOKEN {
    type Abi = Self;
}
impl ::core::fmt::Debug for EXPR_TOKEN {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EXPR_TOKEN").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct ExtendedErrorParamTypes(pub i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const eeptAnsiString: ExtendedErrorParamTypes = ExtendedErrorParamTypes(1i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const eeptUnicodeString: ExtendedErrorParamTypes = ExtendedErrorParamTypes(2i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const eeptLongVal: ExtendedErrorParamTypes = ExtendedErrorParamTypes(3i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const eeptShortVal: ExtendedErrorParamTypes = ExtendedErrorParamTypes(4i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const eeptPointerVal: ExtendedErrorParamTypes = ExtendedErrorParamTypes(5i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const eeptNone: ExtendedErrorParamTypes = ExtendedErrorParamTypes(6i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const eeptBinary: ExtendedErrorParamTypes = ExtendedErrorParamTypes(7i32);
impl ::core::marker::Copy for ExtendedErrorParamTypes {}
impl ::core::clone::Clone for ExtendedErrorParamTypes {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ExtendedErrorParamTypes {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for ExtendedErrorParamTypes {
    type Abi = Self;
}
impl ::core::fmt::Debug for ExtendedErrorParamTypes {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ExtendedErrorParamTypes").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct FULL_PTR_XLAT_TABLES {
    pub RefIdToPointer: *mut ::core::ffi::c_void,
    pub PointerToRefId: *mut ::core::ffi::c_void,
    pub NextRefId: u32,
    pub XlatSide: XLAT_SIDE,
}
impl ::core::marker::Copy for FULL_PTR_XLAT_TABLES {}
impl ::core::clone::Clone for FULL_PTR_XLAT_TABLES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for FULL_PTR_XLAT_TABLES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FULL_PTR_XLAT_TABLES").field("RefIdToPointer", &self.RefIdToPointer).field("PointerToRefId", &self.PointerToRefId).field("NextRefId", &self.NextRefId).field("XlatSide", &self.XlatSide).finish()
    }
}
unsafe impl ::windows::core::Abi for FULL_PTR_XLAT_TABLES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for FULL_PTR_XLAT_TABLES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<FULL_PTR_XLAT_TABLES>()) == 0 }
    }
}
impl ::core::cmp::Eq for FULL_PTR_XLAT_TABLES {}
impl ::core::default::Default for FULL_PTR_XLAT_TABLES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct GENERIC_BINDING_INFO {
    pub pObj: *mut ::core::ffi::c_void,
    pub Size: u32,
    pub pfnBind: GENERIC_BINDING_ROUTINE,
    pub pfnUnbind: GENERIC_UNBIND_ROUTINE,
}
impl ::core::marker::Copy for GENERIC_BINDING_INFO {}
impl ::core::clone::Clone for GENERIC_BINDING_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GENERIC_BINDING_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GENERIC_BINDING_INFO").field("pObj", &self.pObj).field("Size", &self.Size).field("pfnBind", &self.pfnBind.map(|f| f as usize)).field("pfnUnbind", &self.pfnUnbind.map(|f| f as usize)).finish()
    }
}
unsafe impl ::windows::core::Abi for GENERIC_BINDING_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GENERIC_BINDING_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GENERIC_BINDING_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for GENERIC_BINDING_INFO {}
impl ::core::default::Default for GENERIC_BINDING_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type GENERIC_BINDING_ROUTINE = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct GENERIC_BINDING_ROUTINE_PAIR {
    pub pfnBind: GENERIC_BINDING_ROUTINE,
    pub pfnUnbind: GENERIC_UNBIND_ROUTINE,
}
impl ::core::marker::Copy for GENERIC_BINDING_ROUTINE_PAIR {}
impl ::core::clone::Clone for GENERIC_BINDING_ROUTINE_PAIR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for GENERIC_BINDING_ROUTINE_PAIR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("GENERIC_BINDING_ROUTINE_PAIR").field("pfnBind", &self.pfnBind.map(|f| f as usize)).field("pfnUnbind", &self.pfnUnbind.map(|f| f as usize)).finish()
    }
}
unsafe impl ::windows::core::Abi for GENERIC_BINDING_ROUTINE_PAIR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for GENERIC_BINDING_ROUTINE_PAIR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<GENERIC_BINDING_ROUTINE_PAIR>()) == 0 }
    }
}
impl ::core::cmp::Eq for GENERIC_BINDING_ROUTINE_PAIR {}
impl ::core::default::Default for GENERIC_BINDING_ROUTINE_PAIR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type GENERIC_UNBIND_ROUTINE = ::core::option::Option<unsafe extern "system" fn(param0: *mut ::core::ffi::c_void, param1: *mut u8)>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct GROUP_NAME_SYNTAX(pub u32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_NS_SYNTAX_DEFAULT: GROUP_NAME_SYNTAX = GROUP_NAME_SYNTAX(0u32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_NS_SYNTAX_DCE: GROUP_NAME_SYNTAX = GROUP_NAME_SYNTAX(3u32);
impl ::core::marker::Copy for GROUP_NAME_SYNTAX {}
impl ::core::clone::Clone for GROUP_NAME_SYNTAX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for GROUP_NAME_SYNTAX {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for GROUP_NAME_SYNTAX {
    type Abi = Self;
}
impl ::core::fmt::Debug for GROUP_NAME_SYNTAX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("GROUP_NAME_SYNTAX").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct IDL_CS_CONVERT(pub i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const IDL_CS_NO_CONVERT: IDL_CS_CONVERT = IDL_CS_CONVERT(0i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const IDL_CS_IN_PLACE_CONVERT: IDL_CS_CONVERT = IDL_CS_CONVERT(1i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const IDL_CS_NEW_BUFFER_CONVERT: IDL_CS_CONVERT = IDL_CS_CONVERT(2i32);
impl ::core::marker::Copy for IDL_CS_CONVERT {}
impl ::core::clone::Clone for IDL_CS_CONVERT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for IDL_CS_CONVERT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for IDL_CS_CONVERT {
    type Abi = Self;
}
impl ::core::fmt::Debug for IDL_CS_CONVERT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDL_CS_CONVERT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const INVALID_FRAGMENT_ID: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn IUnknown_AddRef_Proxy<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(this: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IUnknown_AddRef_Proxy(this: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(IUnknown_AddRef_Proxy(this.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn IUnknown_QueryInterface_Proxy<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(this: Param0, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IUnknown_QueryInterface_Proxy(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        IUnknown_QueryInterface_Proxy(this.into_param().abi(), ::core::mem::transmute(riid), ::core::mem::transmute(ppvobject)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn IUnknown_Release_Proxy<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(this: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IUnknown_Release_Proxy(this: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(IUnknown_Release_Proxy(this.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcAllocate(size: u32) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcAllocate(size: u32) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(I_RpcAllocate(::core::mem::transmute(size)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn I_RpcAsyncAbortCall(pasync: *const RPC_ASYNC_STATE, exceptioncode: u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcAsyncAbortCall(pasync: *const RPC_ASYNC_STATE, exceptioncode: u32) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcAsyncAbortCall(::core::mem::transmute(pasync), ::core::mem::transmute(exceptioncode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn I_RpcAsyncSetHandle(message: *const RPC_MESSAGE, pasync: *const RPC_ASYNC_STATE) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcAsyncSetHandle(message: *const RPC_MESSAGE, pasync: *const RPC_ASYNC_STATE) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcAsyncSetHandle(::core::mem::transmute(message), ::core::mem::transmute(pasync)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcBindingCopy(sourcebinding: *mut ::core::ffi::c_void, destinationbinding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcBindingCopy(sourcebinding: *mut ::core::ffi::c_void, destinationbinding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcBindingCopy(::core::mem::transmute(sourcebinding), ::core::mem::transmute(destinationbinding)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcBindingCreateNP(servername: *const u16, servicename: *const u16, networkoptions: *const u16, binding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcBindingCreateNP(servername: *const u16, servicename: *const u16, networkoptions: *const u16, binding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcBindingCreateNP(::core::mem::transmute(servername), ::core::mem::transmute(servicename), ::core::mem::transmute(networkoptions), ::core::mem::transmute(binding)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcBindingHandleToAsyncHandle(binding: *mut ::core::ffi::c_void, asynchandle: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcBindingHandleToAsyncHandle(binding: *mut ::core::ffi::c_void, asynchandle: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcBindingHandleToAsyncHandle(::core::mem::transmute(binding), ::core::mem::transmute(asynchandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn I_RpcBindingInqClientTokenAttributes(binding: *const ::core::ffi::c_void, tokenid: *mut super::super::Foundation::LUID, authenticationid: *mut super::super::Foundation::LUID, modifiedid: *mut super::super::Foundation::LUID) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcBindingInqClientTokenAttributes(binding: *const ::core::ffi::c_void, tokenid: *mut super::super::Foundation::LUID, authenticationid: *mut super::super::Foundation::LUID, modifiedid: *mut super::super::Foundation::LUID) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcBindingInqClientTokenAttributes(::core::mem::transmute(binding), ::core::mem::transmute(tokenid), ::core::mem::transmute(authenticationid), ::core::mem::transmute(modifiedid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcBindingInqDynamicEndpointA(binding: *const ::core::ffi::c_void, dynamicendpoint: *mut *mut u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcBindingInqDynamicEndpointA(binding: *const ::core::ffi::c_void, dynamicendpoint: *mut *mut u8) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcBindingInqDynamicEndpointA(::core::mem::transmute(binding), ::core::mem::transmute(dynamicendpoint)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcBindingInqDynamicEndpointW(binding: *const ::core::ffi::c_void, dynamicendpoint: *mut *mut u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcBindingInqDynamicEndpointW(binding: *const ::core::ffi::c_void, dynamicendpoint: *mut *mut u16) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcBindingInqDynamicEndpointW(::core::mem::transmute(binding), ::core::mem::transmute(dynamicendpoint)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcBindingInqLocalClientPID(binding: *mut ::core::ffi::c_void, pid: *mut u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcBindingInqLocalClientPID(binding: *mut ::core::ffi::c_void, pid: *mut u32) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcBindingInqLocalClientPID(::core::mem::transmute(binding), ::core::mem::transmute(pid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcBindingInqMarshalledTargetInfo(binding: *const ::core::ffi::c_void, marshalledtargetinfosize: *mut u32, marshalledtargetinfo: *mut *mut u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcBindingInqMarshalledTargetInfo(binding: *const ::core::ffi::c_void, marshalledtargetinfosize: *mut u32, marshalledtargetinfo: *mut *mut u8) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcBindingInqMarshalledTargetInfo(::core::mem::transmute(binding), ::core::mem::transmute(marshalledtargetinfosize), ::core::mem::transmute(marshalledtargetinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcBindingInqSecurityContext(binding: *mut ::core::ffi::c_void, securitycontexthandle: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcBindingInqSecurityContext(binding: *mut ::core::ffi::c_void, securitycontexthandle: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcBindingInqSecurityContext(::core::mem::transmute(binding), ::core::mem::transmute(securitycontexthandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcBindingInqSecurityContextKeyInfo(binding: *const ::core::ffi::c_void, keyinfo: *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcBindingInqSecurityContextKeyInfo(binding: *const ::core::ffi::c_void, keyinfo: *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcBindingInqSecurityContextKeyInfo(::core::mem::transmute(binding), ::core::mem::transmute(keyinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcBindingInqTransportType(binding: *mut ::core::ffi::c_void, r#type: *mut u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcBindingInqTransportType(binding: *mut ::core::ffi::c_void, r#type: *mut u32) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcBindingInqTransportType(::core::mem::transmute(binding), ::core::mem::transmute(r#type)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcBindingInqWireIdForSnego(binding: *const ::core::ffi::c_void, wireid: *mut u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcBindingInqWireIdForSnego(binding: *const ::core::ffi::c_void, wireid: *mut u8) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcBindingInqWireIdForSnego(::core::mem::transmute(binding), ::core::mem::transmute(wireid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcBindingIsClientLocal(bindinghandle: *mut ::core::ffi::c_void, clientlocalflag: *mut u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcBindingIsClientLocal(bindinghandle: *mut ::core::ffi::c_void, clientlocalflag: *mut u32) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcBindingIsClientLocal(::core::mem::transmute(bindinghandle), ::core::mem::transmute(clientlocalflag)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcBindingIsServerLocal(binding: *const ::core::ffi::c_void, serverlocalflag: *mut u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcBindingIsServerLocal(binding: *const ::core::ffi::c_void, serverlocalflag: *mut u32) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcBindingIsServerLocal(::core::mem::transmute(binding), ::core::mem::transmute(serverlocalflag)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcBindingSetPrivateOption(hbinding: *const ::core::ffi::c_void, option: u32, optionvalue: usize) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcBindingSetPrivateOption(hbinding: *const ::core::ffi::c_void, option: u32, optionvalue: usize) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcBindingSetPrivateOption(::core::mem::transmute(hbinding), ::core::mem::transmute(option), ::core::mem::transmute(optionvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcBindingToStaticStringBindingW(binding: *mut ::core::ffi::c_void, stringbinding: *mut *mut u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcBindingToStaticStringBindingW(binding: *mut ::core::ffi::c_void, stringbinding: *mut *mut u16) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcBindingToStaticStringBindingW(::core::mem::transmute(binding), ::core::mem::transmute(stringbinding)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcClearMutex(mutex: *mut ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcClearMutex(mutex: *mut ::core::ffi::c_void);
        }
        I_RpcClearMutex(::core::mem::transmute(mutex))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcDeleteMutex(mutex: *mut ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcDeleteMutex(mutex: *mut ::core::ffi::c_void);
        }
        I_RpcDeleteMutex(::core::mem::transmute(mutex))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcExceptionFilter(exceptioncode: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcExceptionFilter(exceptioncode: u32) -> i32;
        }
        ::core::mem::transmute(I_RpcExceptionFilter(::core::mem::transmute(exceptioncode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcFree(object: *mut ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcFree(object: *mut ::core::ffi::c_void);
        }
        I_RpcFree(::core::mem::transmute(object))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcFreeBuffer(message: *mut RPC_MESSAGE) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcFreeBuffer(message: *mut RPC_MESSAGE) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcFreeBuffer(::core::mem::transmute(message)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type I_RpcFreeCalloutStateFn = ::core::option::Option<unsafe extern "system" fn(calloutstate: *mut RDR_CALLOUT_STATE)>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcFreePipeBuffer(message: *mut RPC_MESSAGE) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcFreePipeBuffer(message: *mut RPC_MESSAGE) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcFreePipeBuffer(::core::mem::transmute(message)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcGetBuffer(message: *mut RPC_MESSAGE) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcGetBuffer(message: *mut RPC_MESSAGE) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcGetBuffer(::core::mem::transmute(message)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcGetBufferWithObject(message: *mut RPC_MESSAGE, objectuuid: *mut ::windows::core::GUID) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcGetBufferWithObject(message: *mut RPC_MESSAGE, objectuuid: *mut ::windows::core::GUID) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcGetBufferWithObject(::core::mem::transmute(message), ::core::mem::transmute(objectuuid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcGetCurrentCallHandle() -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcGetCurrentCallHandle() -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(I_RpcGetCurrentCallHandle())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcGetDefaultSD(ppsecuritydescriptor: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcGetDefaultSD(ppsecuritydescriptor: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcGetDefaultSD(::core::mem::transmute(ppsecuritydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcGetExtendedError() -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcGetExtendedError() -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcGetExtendedError())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcIfInqTransferSyntaxes(rpcifhandle: *mut ::core::ffi::c_void, transfersyntaxes: *mut RPC_TRANSFER_SYNTAX, transfersyntaxsize: u32, transfersyntaxcount: *mut u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcIfInqTransferSyntaxes(rpcifhandle: *mut ::core::ffi::c_void, transfersyntaxes: *mut RPC_TRANSFER_SYNTAX, transfersyntaxsize: u32, transfersyntaxcount: *mut u32) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcIfInqTransferSyntaxes(::core::mem::transmute(rpcifhandle), ::core::mem::transmute(transfersyntaxes), ::core::mem::transmute(transfersyntaxsize), ::core::mem::transmute(transfersyntaxcount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcMapWin32Status(status: RPC_STATUS) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcMapWin32Status(status: RPC_STATUS) -> i32;
        }
        ::core::mem::transmute(I_RpcMapWin32Status(::core::mem::transmute(status)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcMgmtEnableDedicatedThreadPool() -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcMgmtEnableDedicatedThreadPool() -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcMgmtEnableDedicatedThreadPool())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcNegotiateTransferSyntax(message: *mut RPC_MESSAGE) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcNegotiateTransferSyntax(message: *mut RPC_MESSAGE) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcNegotiateTransferSyntax(::core::mem::transmute(message)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcNsBindingSetEntryNameA(binding: *const ::core::ffi::c_void, entrynamesyntax: u32, entryname: *const u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcNsBindingSetEntryNameA(binding: *const ::core::ffi::c_void, entrynamesyntax: u32, entryname: *const u8) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcNsBindingSetEntryNameA(::core::mem::transmute(binding), ::core::mem::transmute(entrynamesyntax), ::core::mem::transmute(entryname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcNsBindingSetEntryNameW(binding: *const ::core::ffi::c_void, entrynamesyntax: u32, entryname: *const u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcNsBindingSetEntryNameW(binding: *const ::core::ffi::c_void, entrynamesyntax: u32, entryname: *const u16) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcNsBindingSetEntryNameW(::core::mem::transmute(binding), ::core::mem::transmute(entrynamesyntax), ::core::mem::transmute(entryname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcNsGetBuffer(message: *mut RPC_MESSAGE) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcNsGetBuffer(message: *mut RPC_MESSAGE) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcNsGetBuffer(::core::mem::transmute(message)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcNsInterfaceExported(entrynamesyntax: u32, entryname: *mut u16, rpcinterfaceinformation: *mut RPC_SERVER_INTERFACE) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcNsInterfaceExported(entrynamesyntax: u32, entryname: *mut u16, rpcinterfaceinformation: *mut RPC_SERVER_INTERFACE) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcNsInterfaceExported(::core::mem::transmute(entrynamesyntax), ::core::mem::transmute(entryname), ::core::mem::transmute(rpcinterfaceinformation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcNsInterfaceUnexported(entrynamesyntax: u32, entryname: *mut u16, rpcinterfaceinformation: *mut RPC_SERVER_INTERFACE) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcNsInterfaceUnexported(entrynamesyntax: u32, entryname: *mut u16, rpcinterfaceinformation: *mut RPC_SERVER_INTERFACE) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcNsInterfaceUnexported(::core::mem::transmute(entrynamesyntax), ::core::mem::transmute(entryname), ::core::mem::transmute(rpcinterfaceinformation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcNsRaiseException(message: *mut RPC_MESSAGE, status: RPC_STATUS) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcNsRaiseException(message: *mut RPC_MESSAGE, status: RPC_STATUS);
        }
        I_RpcNsRaiseException(::core::mem::transmute(message), ::core::mem::transmute(status))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcNsSendReceive(message: *mut RPC_MESSAGE, handle: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcNsSendReceive(message: *mut RPC_MESSAGE, handle: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcNsSendReceive(::core::mem::transmute(message), ::core::mem::transmute(handle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcOpenClientProcess(binding: *const ::core::ffi::c_void, desiredaccess: u32, clientprocess: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcOpenClientProcess(binding: *const ::core::ffi::c_void, desiredaccess: u32, clientprocess: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcOpenClientProcess(::core::mem::transmute(binding), ::core::mem::transmute(desiredaccess), ::core::mem::transmute(clientprocess)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcPauseExecution(milliseconds: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcPauseExecution(milliseconds: u32);
        }
        I_RpcPauseExecution(::core::mem::transmute(milliseconds))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type I_RpcPerformCalloutFn = ::core::option::Option<unsafe extern "system" fn(context: *mut ::core::ffi::c_void, calloutstate: *mut RDR_CALLOUT_STATE, stage: RPC_HTTP_REDIRECTOR_STAGE) -> RPC_STATUS>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct I_RpcProxyCallbackInterface {
    pub IsValidMachineFn: I_RpcProxyIsValidMachineFn,
    pub GetClientAddressFn: I_RpcProxyGetClientAddressFn,
    pub GetConnectionTimeoutFn: I_RpcProxyGetConnectionTimeoutFn,
    pub PerformCalloutFn: I_RpcPerformCalloutFn,
    pub FreeCalloutStateFn: I_RpcFreeCalloutStateFn,
    pub GetClientSessionAndResourceUUIDFn: I_RpcProxyGetClientSessionAndResourceUUID,
    pub ProxyFilterIfFn: I_RpcProxyFilterIfFn,
    pub RpcProxyUpdatePerfCounterFn: I_RpcProxyUpdatePerfCounterFn,
    pub RpcProxyUpdatePerfCounterBackendServerFn: I_RpcProxyUpdatePerfCounterBackendServerFn,
}
impl ::core::marker::Copy for I_RpcProxyCallbackInterface {}
impl ::core::clone::Clone for I_RpcProxyCallbackInterface {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for I_RpcProxyCallbackInterface {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("I_RpcProxyCallbackInterface")
            .field("IsValidMachineFn", &self.IsValidMachineFn.map(|f| f as usize))
            .field("GetClientAddressFn", &self.GetClientAddressFn.map(|f| f as usize))
            .field("GetConnectionTimeoutFn", &self.GetConnectionTimeoutFn.map(|f| f as usize))
            .field("PerformCalloutFn", &self.PerformCalloutFn.map(|f| f as usize))
            .field("FreeCalloutStateFn", &self.FreeCalloutStateFn.map(|f| f as usize))
            .field("GetClientSessionAndResourceUUIDFn", &self.GetClientSessionAndResourceUUIDFn.map(|f| f as usize))
            .field("ProxyFilterIfFn", &self.ProxyFilterIfFn.map(|f| f as usize))
            .field("RpcProxyUpdatePerfCounterFn", &self.RpcProxyUpdatePerfCounterFn.map(|f| f as usize))
            .field("RpcProxyUpdatePerfCounterBackendServerFn", &self.RpcProxyUpdatePerfCounterBackendServerFn.map(|f| f as usize))
            .finish()
    }
}
unsafe impl ::windows::core::Abi for I_RpcProxyCallbackInterface {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for I_RpcProxyCallbackInterface {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<I_RpcProxyCallbackInterface>()) == 0 }
    }
}
impl ::core::cmp::Eq for I_RpcProxyCallbackInterface {}
impl ::core::default::Default for I_RpcProxyCallbackInterface {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type I_RpcProxyFilterIfFn = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, ifuuid: *const ::windows::core::GUID, ifmajorversion: u16, fallow: *mut i32) -> RPC_STATUS>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type I_RpcProxyGetClientAddressFn = ::core::option::Option<unsafe extern "system" fn(context: *mut ::core::ffi::c_void, buffer: ::windows::core::PCSTR, bufferlength: *mut u32) -> RPC_STATUS>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type I_RpcProxyGetClientSessionAndResourceUUID = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void, sessionidpresent: *mut i32, sessionid: *mut ::windows::core::GUID, resourceidpresent: *mut i32, resourceid: *mut ::windows::core::GUID) -> RPC_STATUS>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type I_RpcProxyGetConnectionTimeoutFn = ::core::option::Option<unsafe extern "system" fn(connectiontimeout: *mut u32) -> RPC_STATUS>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type I_RpcProxyIsValidMachineFn = ::core::option::Option<unsafe extern "system" fn(machine: *const u16, dotmachine: *const u16, portnumber: u32) -> RPC_STATUS>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type I_RpcProxyUpdatePerfCounterBackendServerFn = ::core::option::Option<unsafe extern "system" fn(machinename: *const u16, isconnectevent: i32)>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type I_RpcProxyUpdatePerfCounterFn = ::core::option::Option<unsafe extern "system" fn(counter: RpcProxyPerfCounters, modifytrend: i32, size: u32)>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcReBindBuffer(message: *mut RPC_MESSAGE) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcReBindBuffer(message: *mut RPC_MESSAGE) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcReBindBuffer(::core::mem::transmute(message)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcReallocPipeBuffer(message: *const RPC_MESSAGE, newsize: u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcReallocPipeBuffer(message: *const RPC_MESSAGE, newsize: u32) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcReallocPipeBuffer(::core::mem::transmute(message), ::core::mem::transmute(newsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcReceive(message: *mut RPC_MESSAGE, size: u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcReceive(message: *mut RPC_MESSAGE, size: u32) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcReceive(::core::mem::transmute(message), ::core::mem::transmute(size)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcRecordCalloutFailure(rpcstatus: RPC_STATUS, calloutstate: *mut RDR_CALLOUT_STATE, dllname: *mut u16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcRecordCalloutFailure(rpcstatus: RPC_STATUS, calloutstate: *mut RDR_CALLOUT_STATE, dllname: *mut u16);
        }
        I_RpcRecordCalloutFailure(::core::mem::transmute(rpcstatus), ::core::mem::transmute(calloutstate), ::core::mem::transmute(dllname))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcRequestMutex(mutex: *mut *mut ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcRequestMutex(mutex: *mut *mut ::core::ffi::c_void);
        }
        I_RpcRequestMutex(::core::mem::transmute(mutex))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcSend(message: *mut RPC_MESSAGE) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcSend(message: *mut RPC_MESSAGE) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcSend(::core::mem::transmute(message)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcSendReceive(message: *mut RPC_MESSAGE) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcSendReceive(message: *mut RPC_MESSAGE) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcSendReceive(::core::mem::transmute(message)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcServerCheckClientRestriction(context: *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcServerCheckClientRestriction(context: *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcServerCheckClientRestriction(::core::mem::transmute(context)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcServerDisableExceptionFilter() -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcServerDisableExceptionFilter() -> i32;
        }
        ::core::mem::transmute(I_RpcServerDisableExceptionFilter())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcServerGetAssociationID(binding: *const ::core::ffi::c_void, associationid: *mut u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcServerGetAssociationID(binding: *const ::core::ffi::c_void, associationid: *mut u32) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcServerGetAssociationID(::core::mem::transmute(binding), ::core::mem::transmute(associationid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcServerInqAddressChangeFn() -> *mut RPC_ADDRESS_CHANGE_FN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcServerInqAddressChangeFn() -> *mut RPC_ADDRESS_CHANGE_FN;
        }
        ::core::mem::transmute(I_RpcServerInqAddressChangeFn())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcServerInqLocalConnAddress(binding: *mut ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void, buffersize: *mut u32, addressformat: *mut u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcServerInqLocalConnAddress(binding: *mut ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void, buffersize: *mut u32, addressformat: *mut u32) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcServerInqLocalConnAddress(::core::mem::transmute(binding), ::core::mem::transmute(buffer), ::core::mem::transmute(buffersize), ::core::mem::transmute(addressformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcServerInqRemoteConnAddress(binding: *mut ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void, buffersize: *mut u32, addressformat: *mut u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcServerInqRemoteConnAddress(binding: *mut ::core::ffi::c_void, buffer: *mut ::core::ffi::c_void, buffersize: *mut u32, addressformat: *mut u32) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcServerInqRemoteConnAddress(::core::mem::transmute(binding), ::core::mem::transmute(buffer), ::core::mem::transmute(buffersize), ::core::mem::transmute(addressformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcServerInqTransportType(r#type: *mut u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcServerInqTransportType(r#type: *mut u32) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcServerInqTransportType(::core::mem::transmute(r#type)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcServerRegisterForwardFunction(pforwardfunction: *mut RPC_FORWARD_FUNCTION) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcServerRegisterForwardFunction(pforwardfunction: *mut ::windows::core::RawPtr) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcServerRegisterForwardFunction(::core::mem::transmute(pforwardfunction)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcServerSetAddressChangeFn(paddresschangefn: *mut RPC_ADDRESS_CHANGE_FN) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcServerSetAddressChangeFn(paddresschangefn: *mut ::windows::core::RawPtr) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcServerSetAddressChangeFn(::core::mem::transmute(paddresschangefn)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcServerStartService(protseq: *const u16, endpoint: *const u16, ifspec: *const ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcServerStartService(protseq: *const u16, endpoint: *const u16, ifspec: *const ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcServerStartService(::core::mem::transmute(protseq), ::core::mem::transmute(endpoint), ::core::mem::transmute(ifspec)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcServerSubscribeForDisconnectNotification(binding: *const ::core::ffi::c_void, hevent: *const ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcServerSubscribeForDisconnectNotification(binding: *const ::core::ffi::c_void, hevent: *const ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcServerSubscribeForDisconnectNotification(::core::mem::transmute(binding), ::core::mem::transmute(hevent)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcServerSubscribeForDisconnectNotification2(binding: *const ::core::ffi::c_void, hevent: *const ::core::ffi::c_void, subscriptionid: *mut ::windows::core::GUID) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcServerSubscribeForDisconnectNotification2(binding: *const ::core::ffi::c_void, hevent: *const ::core::ffi::c_void, subscriptionid: *mut ::windows::core::GUID) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcServerSubscribeForDisconnectNotification2(::core::mem::transmute(binding), ::core::mem::transmute(hevent), ::core::mem::transmute(subscriptionid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcServerUnsubscribeForDisconnectNotification<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(binding: *const ::core::ffi::c_void, subscriptionid: Param1) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcServerUnsubscribeForDisconnectNotification(binding: *const ::core::ffi::c_void, subscriptionid: ::windows::core::GUID) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcServerUnsubscribeForDisconnectNotification(::core::mem::transmute(binding), subscriptionid.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcServerUseProtseq2A(networkaddress: *const u8, protseq: *const u8, maxcalls: u32, securitydescriptor: *const ::core::ffi::c_void, policy: *const ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcServerUseProtseq2A(networkaddress: *const u8, protseq: *const u8, maxcalls: u32, securitydescriptor: *const ::core::ffi::c_void, policy: *const ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcServerUseProtseq2A(::core::mem::transmute(networkaddress), ::core::mem::transmute(protseq), ::core::mem::transmute(maxcalls), ::core::mem::transmute(securitydescriptor), ::core::mem::transmute(policy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcServerUseProtseq2W(networkaddress: *const u16, protseq: *const u16, maxcalls: u32, securitydescriptor: *const ::core::ffi::c_void, policy: *const ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcServerUseProtseq2W(networkaddress: *const u16, protseq: *const u16, maxcalls: u32, securitydescriptor: *const ::core::ffi::c_void, policy: *const ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcServerUseProtseq2W(::core::mem::transmute(networkaddress), ::core::mem::transmute(protseq), ::core::mem::transmute(maxcalls), ::core::mem::transmute(securitydescriptor), ::core::mem::transmute(policy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcServerUseProtseqEp2A(networkaddress: *const u8, protseq: *const u8, maxcalls: u32, endpoint: *const u8, securitydescriptor: *const ::core::ffi::c_void, policy: *const ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcServerUseProtseqEp2A(networkaddress: *const u8, protseq: *const u8, maxcalls: u32, endpoint: *const u8, securitydescriptor: *const ::core::ffi::c_void, policy: *const ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcServerUseProtseqEp2A(::core::mem::transmute(networkaddress), ::core::mem::transmute(protseq), ::core::mem::transmute(maxcalls), ::core::mem::transmute(endpoint), ::core::mem::transmute(securitydescriptor), ::core::mem::transmute(policy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcServerUseProtseqEp2W(networkaddress: *const u16, protseq: *const u16, maxcalls: u32, endpoint: *const u16, securitydescriptor: *const ::core::ffi::c_void, policy: *const ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcServerUseProtseqEp2W(networkaddress: *const u16, protseq: *const u16, maxcalls: u32, endpoint: *const u16, securitydescriptor: *const ::core::ffi::c_void, policy: *const ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcServerUseProtseqEp2W(::core::mem::transmute(networkaddress), ::core::mem::transmute(protseq), ::core::mem::transmute(maxcalls), ::core::mem::transmute(endpoint), ::core::mem::transmute(securitydescriptor), ::core::mem::transmute(policy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcSessionStrictContextHandle() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcSessionStrictContextHandle();
        }
        I_RpcSessionStrictContextHandle()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcSsDontSerializeContext() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcSsDontSerializeContext();
        }
        I_RpcSsDontSerializeContext()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcSystemHandleTypeSpecificWork(handle: *mut ::core::ffi::c_void, actualtype: u8, idltype: u8, marshaldirection: LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcSystemHandleTypeSpecificWork(handle: *mut ::core::ffi::c_void, actualtype: u8, idltype: u8, marshaldirection: LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcSystemHandleTypeSpecificWork(::core::mem::transmute(handle), ::core::mem::transmute(actualtype), ::core::mem::transmute(idltype), ::core::mem::transmute(marshaldirection)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_RpcTurnOnEEInfoPropagation() -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcTurnOnEEInfoPropagation() -> RPC_STATUS;
        }
        ::core::mem::transmute(I_RpcTurnOnEEInfoPropagation())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn I_UuidCreate(uuid: *mut ::windows::core::GUID) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_UuidCreate(uuid: *mut ::windows::core::GUID) -> RPC_STATUS;
        }
        ::core::mem::transmute(I_UuidCreate(::core::mem::transmute(uuid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION(pub i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const MarshalDirectionMarshal: LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION = LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION(0i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const MarshalDirectionUnmarshal: LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION = LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION(1i32);
impl ::core::marker::Copy for LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION {}
impl ::core::clone::Clone for LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct MALLOC_FREE_STRUCT {
    pub pfnAllocate: isize,
    pub pfnFree: isize,
}
impl ::core::marker::Copy for MALLOC_FREE_STRUCT {}
impl ::core::clone::Clone for MALLOC_FREE_STRUCT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MALLOC_FREE_STRUCT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MALLOC_FREE_STRUCT").field("pfnAllocate", &self.pfnAllocate).field("pfnFree", &self.pfnFree).finish()
    }
}
unsafe impl ::windows::core::Abi for MALLOC_FREE_STRUCT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MALLOC_FREE_STRUCT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MALLOC_FREE_STRUCT>()) == 0 }
    }
}
impl ::core::cmp::Eq for MALLOC_FREE_STRUCT {}
impl ::core::default::Default for MALLOC_FREE_STRUCT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type MIDL_ES_ALLOC = ::core::option::Option<unsafe extern "system" fn(state: *mut ::core::ffi::c_void, pbuffer: *mut *mut i8, psize: *mut u32)>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MIDL_ES_CODE(pub i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const MES_ENCODE: MIDL_ES_CODE = MIDL_ES_CODE(0i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const MES_DECODE: MIDL_ES_CODE = MIDL_ES_CODE(1i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const MES_ENCODE_NDR64: MIDL_ES_CODE = MIDL_ES_CODE(2i32);
impl ::core::marker::Copy for MIDL_ES_CODE {}
impl ::core::clone::Clone for MIDL_ES_CODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MIDL_ES_CODE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MIDL_ES_CODE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MIDL_ES_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MIDL_ES_CODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct MIDL_ES_HANDLE_STYLE(pub i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const MES_INCREMENTAL_HANDLE: MIDL_ES_HANDLE_STYLE = MIDL_ES_HANDLE_STYLE(0i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const MES_FIXED_BUFFER_HANDLE: MIDL_ES_HANDLE_STYLE = MIDL_ES_HANDLE_STYLE(1i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const MES_DYNAMIC_BUFFER_HANDLE: MIDL_ES_HANDLE_STYLE = MIDL_ES_HANDLE_STYLE(2i32);
impl ::core::marker::Copy for MIDL_ES_HANDLE_STYLE {}
impl ::core::clone::Clone for MIDL_ES_HANDLE_STYLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for MIDL_ES_HANDLE_STYLE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for MIDL_ES_HANDLE_STYLE {
    type Abi = Self;
}
impl ::core::fmt::Debug for MIDL_ES_HANDLE_STYLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("MIDL_ES_HANDLE_STYLE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type MIDL_ES_READ = ::core::option::Option<unsafe extern "system" fn(state: *mut ::core::ffi::c_void, pbuffer: *mut *mut i8, psize: *mut u32)>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type MIDL_ES_WRITE = ::core::option::Option<unsafe extern "system" fn(state: *mut ::core::ffi::c_void, buffer: ::windows::core::PCSTR, size: u32)>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct MIDL_FORMAT_STRING {
    pub Pad: i16,
    pub Format: [u8; 1],
}
impl ::core::marker::Copy for MIDL_FORMAT_STRING {}
impl ::core::clone::Clone for MIDL_FORMAT_STRING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIDL_FORMAT_STRING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIDL_FORMAT_STRING").field("Pad", &self.Pad).field("Format", &self.Format).finish()
    }
}
unsafe impl ::windows::core::Abi for MIDL_FORMAT_STRING {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIDL_FORMAT_STRING {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIDL_FORMAT_STRING>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIDL_FORMAT_STRING {}
impl ::core::default::Default for MIDL_FORMAT_STRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct MIDL_INTERCEPTION_INFO {
    pub Version: u32,
    pub ProcString: *mut u8,
    pub ProcFormatOffsetTable: *const u16,
    pub ProcCount: u32,
    pub TypeString: *mut u8,
}
impl ::core::marker::Copy for MIDL_INTERCEPTION_INFO {}
impl ::core::clone::Clone for MIDL_INTERCEPTION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIDL_INTERCEPTION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIDL_INTERCEPTION_INFO").field("Version", &self.Version).field("ProcString", &self.ProcString).field("ProcFormatOffsetTable", &self.ProcFormatOffsetTable).field("ProcCount", &self.ProcCount).field("TypeString", &self.TypeString).finish()
    }
}
unsafe impl ::windows::core::Abi for MIDL_INTERCEPTION_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIDL_INTERCEPTION_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIDL_INTERCEPTION_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIDL_INTERCEPTION_INFO {}
impl ::core::default::Default for MIDL_INTERCEPTION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct MIDL_INTERFACE_METHOD_PROPERTIES {
    pub MethodCount: u16,
    pub MethodProperties: *const *const MIDL_METHOD_PROPERTY_MAP,
}
impl ::core::marker::Copy for MIDL_INTERFACE_METHOD_PROPERTIES {}
impl ::core::clone::Clone for MIDL_INTERFACE_METHOD_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIDL_INTERFACE_METHOD_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIDL_INTERFACE_METHOD_PROPERTIES").field("MethodCount", &self.MethodCount).field("MethodProperties", &self.MethodProperties).finish()
    }
}
unsafe impl ::windows::core::Abi for MIDL_INTERFACE_METHOD_PROPERTIES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIDL_INTERFACE_METHOD_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIDL_INTERFACE_METHOD_PROPERTIES>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIDL_INTERFACE_METHOD_PROPERTIES {}
impl ::core::default::Default for MIDL_INTERFACE_METHOD_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct MIDL_METHOD_PROPERTY {
    pub Id: u32,
    pub Value: usize,
}
impl ::core::marker::Copy for MIDL_METHOD_PROPERTY {}
impl ::core::clone::Clone for MIDL_METHOD_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIDL_METHOD_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIDL_METHOD_PROPERTY").field("Id", &self.Id).field("Value", &self.Value).finish()
    }
}
unsafe impl ::windows::core::Abi for MIDL_METHOD_PROPERTY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIDL_METHOD_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIDL_METHOD_PROPERTY>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIDL_METHOD_PROPERTY {}
impl ::core::default::Default for MIDL_METHOD_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct MIDL_METHOD_PROPERTY_MAP {
    pub Count: u32,
    pub Properties: *const MIDL_METHOD_PROPERTY,
}
impl ::core::marker::Copy for MIDL_METHOD_PROPERTY_MAP {}
impl ::core::clone::Clone for MIDL_METHOD_PROPERTY_MAP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIDL_METHOD_PROPERTY_MAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIDL_METHOD_PROPERTY_MAP").field("Count", &self.Count).field("Properties", &self.Properties).finish()
    }
}
unsafe impl ::windows::core::Abi for MIDL_METHOD_PROPERTY_MAP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIDL_METHOD_PROPERTY_MAP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIDL_METHOD_PROPERTY_MAP>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIDL_METHOD_PROPERTY_MAP {}
impl ::core::default::Default for MIDL_METHOD_PROPERTY_MAP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct MIDL_SERVER_INFO {
    pub pStubDesc: *mut MIDL_STUB_DESC,
    pub DispatchTable: *const SERVER_ROUTINE,
    pub ProcString: *mut u8,
    pub FmtStringOffset: *const u16,
    pub ThunkTable: *const STUB_THUNK,
    pub pTransferSyntax: *mut RPC_SYNTAX_IDENTIFIER,
    pub nCount: usize,
    pub pSyntaxInfo: *mut MIDL_SYNTAX_INFO,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for MIDL_SERVER_INFO {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for MIDL_SERVER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for MIDL_SERVER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIDL_SERVER_INFO").field("pStubDesc", &self.pStubDesc).field("DispatchTable", &self.DispatchTable).field("ProcString", &self.ProcString).field("FmtStringOffset", &self.FmtStringOffset).field("ThunkTable", &self.ThunkTable).field("pTransferSyntax", &self.pTransferSyntax).field("nCount", &self.nCount).field("pSyntaxInfo", &self.pSyntaxInfo).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for MIDL_SERVER_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for MIDL_SERVER_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIDL_SERVER_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for MIDL_SERVER_INFO {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for MIDL_SERVER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct MIDL_STUBLESS_PROXY_INFO {
    pub pStubDesc: *mut MIDL_STUB_DESC,
    pub ProcFormatString: *mut u8,
    pub FormatStringOffset: *const u16,
    pub pTransferSyntax: *mut RPC_SYNTAX_IDENTIFIER,
    pub nCount: usize,
    pub pSyntaxInfo: *mut MIDL_SYNTAX_INFO,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for MIDL_STUBLESS_PROXY_INFO {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for MIDL_STUBLESS_PROXY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for MIDL_STUBLESS_PROXY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIDL_STUBLESS_PROXY_INFO").field("pStubDesc", &self.pStubDesc).field("ProcFormatString", &self.ProcFormatString).field("FormatStringOffset", &self.FormatStringOffset).field("pTransferSyntax", &self.pTransferSyntax).field("nCount", &self.nCount).field("pSyntaxInfo", &self.pSyntaxInfo).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for MIDL_STUBLESS_PROXY_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for MIDL_STUBLESS_PROXY_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIDL_STUBLESS_PROXY_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for MIDL_STUBLESS_PROXY_INFO {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for MIDL_STUBLESS_PROXY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct MIDL_STUB_DESC {
    pub RpcInterfaceInformation: *mut ::core::ffi::c_void,
    pub pfnAllocate: isize,
    pub pfnFree: isize,
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
    pub ProxyServerInfo: *mut ::core::ffi::c_void,
    pub pExprInfo: *const NDR_EXPR_DESC,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for MIDL_STUB_DESC {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for MIDL_STUB_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for MIDL_STUB_DESC {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for MIDL_STUB_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIDL_STUB_DESC>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for MIDL_STUB_DESC {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for MIDL_STUB_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub union MIDL_STUB_DESC_0 {
    pub pAutoHandle: *mut *mut ::core::ffi::c_void,
    pub pPrimitiveHandle: *mut *mut ::core::ffi::c_void,
    pub pGenericBindingInfo: *mut GENERIC_BINDING_INFO,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for MIDL_STUB_DESC_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for MIDL_STUB_DESC_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for MIDL_STUB_DESC_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for MIDL_STUB_DESC_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIDL_STUB_DESC_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for MIDL_STUB_DESC_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for MIDL_STUB_DESC_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct MIDL_STUB_MESSAGE {
    pub RpcMsg: *mut RPC_MESSAGE,
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
    pub pfnAllocate: isize,
    pub pfnFree: isize,
    pub StackTop: *mut u8,
    pub pPresentedType: *mut u8,
    pub pTransmitType: *mut u8,
    pub SavedHandle: *mut ::core::ffi::c_void,
    pub StubDesc: *const MIDL_STUB_DESC,
    pub FullPtrXlatTables: *mut FULL_PTR_XLAT_TABLES,
    pub FullPtrRefId: u32,
    pub PointerLength: u32,
    pub _bitfield: i32,
    pub dwDestContext: u32,
    pub pvDestContext: *mut ::core::ffi::c_void,
    pub SavedContextHandles: *mut *mut NDR_SCONTEXT_1,
    pub ParamNumber: i32,
    pub pRpcChannelBuffer: ::core::option::Option<super::Com::IRpcChannelBuffer>,
    pub pArrayInfo: *mut ARRAY_INFO,
    pub SizePtrCountArray: *mut u32,
    pub SizePtrOffsetArray: *mut u32,
    pub SizePtrLengthArray: *mut u32,
    pub pArgQueue: *mut ::core::ffi::c_void,
    pub dwStubPhase: u32,
    pub LowStackMark: *mut ::core::ffi::c_void,
    pub pAsyncMsg: *mut _NDR_ASYNC_MESSAGE,
    pub pCorrInfo: *mut _NDR_CORRELATION_INFO,
    pub pCorrMemory: *mut u8,
    pub pMemoryList: *mut ::core::ffi::c_void,
    pub pCSInfo: isize,
    pub ConformanceMark: *mut u8,
    pub VarianceMark: *mut u8,
    pub Unused: isize,
    pub pContext: *mut _NDR_PROC_CONTEXT,
    pub ContextHandleHash: *mut ::core::ffi::c_void,
    pub pUserMarshalList: *mut ::core::ffi::c_void,
    pub Reserved51_3: isize,
    pub Reserved51_4: isize,
    pub Reserved51_5: isize,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for MIDL_STUB_MESSAGE {
    fn clone(&self) -> Self {
        Self {
            RpcMsg: self.RpcMsg,
            Buffer: self.Buffer,
            BufferStart: self.BufferStart,
            BufferEnd: self.BufferEnd,
            BufferMark: self.BufferMark,
            BufferLength: self.BufferLength,
            MemorySize: self.MemorySize,
            Memory: self.Memory,
            IsClient: self.IsClient,
            Pad: self.Pad,
            uFlags2: self.uFlags2,
            ReuseBuffer: self.ReuseBuffer,
            pAllocAllNodesContext: self.pAllocAllNodesContext,
            pPointerQueueState: self.pPointerQueueState,
            IgnoreEmbeddedPointers: self.IgnoreEmbeddedPointers,
            PointerBufferMark: self.PointerBufferMark,
            CorrDespIncrement: self.CorrDespIncrement,
            uFlags: self.uFlags,
            UniquePtrCount: self.UniquePtrCount,
            MaxCount: self.MaxCount,
            Offset: self.Offset,
            ActualCount: self.ActualCount,
            pfnAllocate: self.pfnAllocate,
            pfnFree: self.pfnFree,
            StackTop: self.StackTop,
            pPresentedType: self.pPresentedType,
            pTransmitType: self.pTransmitType,
            SavedHandle: self.SavedHandle,
            StubDesc: self.StubDesc,
            FullPtrXlatTables: self.FullPtrXlatTables,
            FullPtrRefId: self.FullPtrRefId,
            PointerLength: self.PointerLength,
            _bitfield: self._bitfield,
            dwDestContext: self.dwDestContext,
            pvDestContext: self.pvDestContext,
            SavedContextHandles: self.SavedContextHandles,
            ParamNumber: self.ParamNumber,
            pRpcChannelBuffer: self.pRpcChannelBuffer.clone(),
            pArrayInfo: self.pArrayInfo,
            SizePtrCountArray: self.SizePtrCountArray,
            SizePtrOffsetArray: self.SizePtrOffsetArray,
            SizePtrLengthArray: self.SizePtrLengthArray,
            pArgQueue: self.pArgQueue,
            dwStubPhase: self.dwStubPhase,
            LowStackMark: self.LowStackMark,
            pAsyncMsg: self.pAsyncMsg,
            pCorrInfo: self.pCorrInfo,
            pCorrMemory: self.pCorrMemory,
            pMemoryList: self.pMemoryList,
            pCSInfo: self.pCSInfo,
            ConformanceMark: self.ConformanceMark,
            VarianceMark: self.VarianceMark,
            Unused: self.Unused,
            pContext: self.pContext,
            ContextHandleHash: self.ContextHandleHash,
            pUserMarshalList: self.pUserMarshalList,
            Reserved51_3: self.Reserved51_3,
            Reserved51_4: self.Reserved51_4,
            Reserved51_5: self.Reserved51_5,
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for MIDL_STUB_MESSAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIDL_STUB_MESSAGE")
            .field("RpcMsg", &self.RpcMsg)
            .field("Buffer", &self.Buffer)
            .field("BufferStart", &self.BufferStart)
            .field("BufferEnd", &self.BufferEnd)
            .field("BufferMark", &self.BufferMark)
            .field("BufferLength", &self.BufferLength)
            .field("MemorySize", &self.MemorySize)
            .field("Memory", &self.Memory)
            .field("IsClient", &self.IsClient)
            .field("Pad", &self.Pad)
            .field("uFlags2", &self.uFlags2)
            .field("ReuseBuffer", &self.ReuseBuffer)
            .field("pAllocAllNodesContext", &self.pAllocAllNodesContext)
            .field("pPointerQueueState", &self.pPointerQueueState)
            .field("IgnoreEmbeddedPointers", &self.IgnoreEmbeddedPointers)
            .field("PointerBufferMark", &self.PointerBufferMark)
            .field("CorrDespIncrement", &self.CorrDespIncrement)
            .field("uFlags", &self.uFlags)
            .field("UniquePtrCount", &self.UniquePtrCount)
            .field("MaxCount", &self.MaxCount)
            .field("Offset", &self.Offset)
            .field("ActualCount", &self.ActualCount)
            .field("pfnAllocate", &self.pfnAllocate)
            .field("pfnFree", &self.pfnFree)
            .field("StackTop", &self.StackTop)
            .field("pPresentedType", &self.pPresentedType)
            .field("pTransmitType", &self.pTransmitType)
            .field("SavedHandle", &self.SavedHandle)
            .field("StubDesc", &self.StubDesc)
            .field("FullPtrXlatTables", &self.FullPtrXlatTables)
            .field("FullPtrRefId", &self.FullPtrRefId)
            .field("PointerLength", &self.PointerLength)
            .field("_bitfield", &self._bitfield)
            .field("dwDestContext", &self.dwDestContext)
            .field("pvDestContext", &self.pvDestContext)
            .field("SavedContextHandles", &self.SavedContextHandles)
            .field("ParamNumber", &self.ParamNumber)
            .field("pRpcChannelBuffer", &self.pRpcChannelBuffer)
            .field("pArrayInfo", &self.pArrayInfo)
            .field("SizePtrCountArray", &self.SizePtrCountArray)
            .field("SizePtrOffsetArray", &self.SizePtrOffsetArray)
            .field("SizePtrLengthArray", &self.SizePtrLengthArray)
            .field("pArgQueue", &self.pArgQueue)
            .field("dwStubPhase", &self.dwStubPhase)
            .field("LowStackMark", &self.LowStackMark)
            .field("pAsyncMsg", &self.pAsyncMsg)
            .field("pCorrInfo", &self.pCorrInfo)
            .field("pCorrMemory", &self.pCorrMemory)
            .field("pMemoryList", &self.pMemoryList)
            .field("pCSInfo", &self.pCSInfo)
            .field("ConformanceMark", &self.ConformanceMark)
            .field("VarianceMark", &self.VarianceMark)
            .field("Unused", &self.Unused)
            .field("pContext", &self.pContext)
            .field("ContextHandleHash", &self.ContextHandleHash)
            .field("pUserMarshalList", &self.pUserMarshalList)
            .field("Reserved51_3", &self.Reserved51_3)
            .field("Reserved51_4", &self.Reserved51_4)
            .field("Reserved51_5", &self.Reserved51_5)
            .finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for MIDL_STUB_MESSAGE {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for MIDL_STUB_MESSAGE {
    fn eq(&self, other: &Self) -> bool {
        self.RpcMsg == other.RpcMsg
            && self.Buffer == other.Buffer
            && self.BufferStart == other.BufferStart
            && self.BufferEnd == other.BufferEnd
            && self.BufferMark == other.BufferMark
            && self.BufferLength == other.BufferLength
            && self.MemorySize == other.MemorySize
            && self.Memory == other.Memory
            && self.IsClient == other.IsClient
            && self.Pad == other.Pad
            && self.uFlags2 == other.uFlags2
            && self.ReuseBuffer == other.ReuseBuffer
            && self.pAllocAllNodesContext == other.pAllocAllNodesContext
            && self.pPointerQueueState == other.pPointerQueueState
            && self.IgnoreEmbeddedPointers == other.IgnoreEmbeddedPointers
            && self.PointerBufferMark == other.PointerBufferMark
            && self.CorrDespIncrement == other.CorrDespIncrement
            && self.uFlags == other.uFlags
            && self.UniquePtrCount == other.UniquePtrCount
            && self.MaxCount == other.MaxCount
            && self.Offset == other.Offset
            && self.ActualCount == other.ActualCount
            && self.pfnAllocate == other.pfnAllocate
            && self.pfnFree == other.pfnFree
            && self.StackTop == other.StackTop
            && self.pPresentedType == other.pPresentedType
            && self.pTransmitType == other.pTransmitType
            && self.SavedHandle == other.SavedHandle
            && self.StubDesc == other.StubDesc
            && self.FullPtrXlatTables == other.FullPtrXlatTables
            && self.FullPtrRefId == other.FullPtrRefId
            && self.PointerLength == other.PointerLength
            && self._bitfield == other._bitfield
            && self.dwDestContext == other.dwDestContext
            && self.pvDestContext == other.pvDestContext
            && self.SavedContextHandles == other.SavedContextHandles
            && self.ParamNumber == other.ParamNumber
            && self.pRpcChannelBuffer == other.pRpcChannelBuffer
            && self.pArrayInfo == other.pArrayInfo
            && self.SizePtrCountArray == other.SizePtrCountArray
            && self.SizePtrOffsetArray == other.SizePtrOffsetArray
            && self.SizePtrLengthArray == other.SizePtrLengthArray
            && self.pArgQueue == other.pArgQueue
            && self.dwStubPhase == other.dwStubPhase
            && self.LowStackMark == other.LowStackMark
            && self.pAsyncMsg == other.pAsyncMsg
            && self.pCorrInfo == other.pCorrInfo
            && self.pCorrMemory == other.pCorrMemory
            && self.pMemoryList == other.pMemoryList
            && self.pCSInfo == other.pCSInfo
            && self.ConformanceMark == other.ConformanceMark
            && self.VarianceMark == other.VarianceMark
            && self.Unused == other.Unused
            && self.pContext == other.pContext
            && self.ContextHandleHash == other.ContextHandleHash
            && self.pUserMarshalList == other.pUserMarshalList
            && self.Reserved51_3 == other.Reserved51_3
            && self.Reserved51_4 == other.Reserved51_4
            && self.Reserved51_5 == other.Reserved51_5
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for MIDL_STUB_MESSAGE {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for MIDL_STUB_MESSAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct MIDL_SYNTAX_INFO {
    pub TransferSyntax: RPC_SYNTAX_IDENTIFIER,
    pub DispatchTable: *mut RPC_DISPATCH_TABLE,
    pub ProcString: *mut u8,
    pub FmtStringOffset: *const u16,
    pub TypeString: *mut u8,
    pub aUserMarshalQuadruple: *const ::core::ffi::c_void,
    pub pMethodProperties: *const MIDL_INTERFACE_METHOD_PROPERTIES,
    pub pReserved2: usize,
}
impl ::core::marker::Copy for MIDL_SYNTAX_INFO {}
impl ::core::clone::Clone for MIDL_SYNTAX_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIDL_SYNTAX_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIDL_SYNTAX_INFO").field("TransferSyntax", &self.TransferSyntax).field("DispatchTable", &self.DispatchTable).field("ProcString", &self.ProcString).field("FmtStringOffset", &self.FmtStringOffset).field("TypeString", &self.TypeString).field("aUserMarshalQuadruple", &self.aUserMarshalQuadruple).field("pMethodProperties", &self.pMethodProperties).field("pReserved2", &self.pReserved2).finish()
    }
}
unsafe impl ::windows::core::Abi for MIDL_SYNTAX_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIDL_SYNTAX_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIDL_SYNTAX_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIDL_SYNTAX_INFO {}
impl ::core::default::Default for MIDL_SYNTAX_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct MIDL_TYPE_PICKLING_INFO {
    pub Version: u32,
    pub Flags: u32,
    pub Reserved: [usize; 3],
}
impl ::core::marker::Copy for MIDL_TYPE_PICKLING_INFO {}
impl ::core::clone::Clone for MIDL_TYPE_PICKLING_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for MIDL_TYPE_PICKLING_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIDL_TYPE_PICKLING_INFO").field("Version", &self.Version).field("Flags", &self.Flags).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for MIDL_TYPE_PICKLING_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for MIDL_TYPE_PICKLING_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIDL_TYPE_PICKLING_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for MIDL_TYPE_PICKLING_INFO {}
impl ::core::default::Default for MIDL_TYPE_PICKLING_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct MIDL_WINRT_TYPE_SERIALIZATION_INFO {
    pub Version: u32,
    pub TypeFormatString: *mut u8,
    pub FormatStringSize: u16,
    pub TypeOffset: u16,
    pub StubDesc: *mut MIDL_STUB_DESC,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for MIDL_WINRT_TYPE_SERIALIZATION_INFO {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for MIDL_WINRT_TYPE_SERIALIZATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for MIDL_WINRT_TYPE_SERIALIZATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("MIDL_WINRT_TYPE_SERIALIZATION_INFO").field("Version", &self.Version).field("TypeFormatString", &self.TypeFormatString).field("FormatStringSize", &self.FormatStringSize).field("TypeOffset", &self.TypeOffset).field("StubDesc", &self.StubDesc).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for MIDL_WINRT_TYPE_SERIALIZATION_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for MIDL_WINRT_TYPE_SERIALIZATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<MIDL_WINRT_TYPE_SERIALIZATION_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for MIDL_WINRT_TYPE_SERIALIZATION_INFO {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for MIDL_WINRT_TYPE_SERIALIZATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const MIDL_WINRT_TYPE_SERIALIZATION_INFO_CURRENT_VERSION: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const MaxNumberOfEEInfoParams: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn MesBufferHandleReset(handle: *const ::core::ffi::c_void, handlestyle: u32, operation: MIDL_ES_CODE, pbuffer: *const *const i8, buffersize: u32, pencodedsize: *mut u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MesBufferHandleReset(handle: *const ::core::ffi::c_void, handlestyle: u32, operation: MIDL_ES_CODE, pbuffer: *const *const i8, buffersize: u32, pencodedsize: *mut u32) -> RPC_STATUS;
        }
        ::core::mem::transmute(MesBufferHandleReset(::core::mem::transmute(handle), ::core::mem::transmute(handlestyle), ::core::mem::transmute(operation), ::core::mem::transmute(pbuffer), ::core::mem::transmute(buffersize), ::core::mem::transmute(pencodedsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn MesDecodeBufferHandleCreate<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCSTR>>(buffer: Param0, buffersize: u32, phandle: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MesDecodeBufferHandleCreate(buffer: ::windows::core::PCSTR, buffersize: u32, phandle: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(MesDecodeBufferHandleCreate(buffer.into_param().abi(), ::core::mem::transmute(buffersize), ::core::mem::transmute(phandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn MesDecodeIncrementalHandleCreate(userstate: *mut ::core::ffi::c_void, readfn: MIDL_ES_READ, phandle: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MesDecodeIncrementalHandleCreate(userstate: *mut ::core::ffi::c_void, readfn: ::windows::core::RawPtr, phandle: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(MesDecodeIncrementalHandleCreate(::core::mem::transmute(userstate), ::core::mem::transmute(readfn), ::core::mem::transmute(phandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn MesEncodeDynBufferHandleCreate(pbuffer: *mut *mut i8, pencodedsize: *mut u32, phandle: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MesEncodeDynBufferHandleCreate(pbuffer: *mut *mut i8, pencodedsize: *mut u32, phandle: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(MesEncodeDynBufferHandleCreate(::core::mem::transmute(pbuffer), ::core::mem::transmute(pencodedsize), ::core::mem::transmute(phandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn MesEncodeFixedBufferHandleCreate(pbuffer: ::windows::core::PSTR, buffersize: u32, pencodedsize: *mut u32, phandle: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MesEncodeFixedBufferHandleCreate(pbuffer: ::windows::core::PSTR, buffersize: u32, pencodedsize: *mut u32, phandle: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(MesEncodeFixedBufferHandleCreate(::core::mem::transmute(pbuffer), ::core::mem::transmute(buffersize), ::core::mem::transmute(pencodedsize), ::core::mem::transmute(phandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn MesEncodeIncrementalHandleCreate(userstate: *mut ::core::ffi::c_void, allocfn: MIDL_ES_ALLOC, writefn: MIDL_ES_WRITE, phandle: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MesEncodeIncrementalHandleCreate(userstate: *mut ::core::ffi::c_void, allocfn: ::windows::core::RawPtr, writefn: ::windows::core::RawPtr, phandle: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(MesEncodeIncrementalHandleCreate(::core::mem::transmute(userstate), ::core::mem::transmute(allocfn), ::core::mem::transmute(writefn), ::core::mem::transmute(phandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn MesHandleFree(handle: *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MesHandleFree(handle: *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(MesHandleFree(::core::mem::transmute(handle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn MesIncrementalHandleReset(handle: *mut ::core::ffi::c_void, userstate: *mut ::core::ffi::c_void, allocfn: MIDL_ES_ALLOC, writefn: MIDL_ES_WRITE, readfn: MIDL_ES_READ, operation: MIDL_ES_CODE) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MesIncrementalHandleReset(handle: *mut ::core::ffi::c_void, userstate: *mut ::core::ffi::c_void, allocfn: ::windows::core::RawPtr, writefn: ::windows::core::RawPtr, readfn: ::windows::core::RawPtr, operation: MIDL_ES_CODE) -> RPC_STATUS;
        }
        ::core::mem::transmute(MesIncrementalHandleReset(::core::mem::transmute(handle), ::core::mem::transmute(userstate), ::core::mem::transmute(allocfn), ::core::mem::transmute(writefn), ::core::mem::transmute(readfn), ::core::mem::transmute(operation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn MesInqProcEncodingId(handle: *mut ::core::ffi::c_void, pinterfaceid: *mut RPC_SYNTAX_IDENTIFIER, pprocnum: *mut u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MesInqProcEncodingId(handle: *mut ::core::ffi::c_void, pinterfaceid: *mut RPC_SYNTAX_IDENTIFIER, pprocnum: *mut u32) -> RPC_STATUS;
        }
        ::core::mem::transmute(MesInqProcEncodingId(::core::mem::transmute(handle), ::core::mem::transmute(pinterfaceid), ::core::mem::transmute(pprocnum)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const MidlInterceptionInfoVersionOne: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const MidlWinrtTypeSerializationInfoVersionOne: i32 = 1i32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_ARRAY_ELEMENT_INFO {
    pub ElementMemSize: u32,
    pub Element: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for NDR64_ARRAY_ELEMENT_INFO {}
impl ::core::clone::Clone for NDR64_ARRAY_ELEMENT_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_ARRAY_ELEMENT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_ARRAY_ELEMENT_INFO").field("ElementMemSize", &self.ElementMemSize).field("Element", &self.Element).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_ARRAY_ELEMENT_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_ARRAY_ELEMENT_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_ARRAY_ELEMENT_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_ARRAY_ELEMENT_INFO {}
impl ::core::default::Default for NDR64_ARRAY_ELEMENT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_ARRAY_FLAGS {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for NDR64_ARRAY_FLAGS {}
impl ::core::clone::Clone for NDR64_ARRAY_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_ARRAY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_ARRAY_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_ARRAY_FLAGS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_ARRAY_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_ARRAY_FLAGS>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_ARRAY_FLAGS {}
impl ::core::default::Default for NDR64_ARRAY_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub union NDR64_BINDINGS {
    pub Primitive: NDR64_BIND_PRIMITIVE,
    pub Generic: NDR64_BIND_GENERIC,
    pub Context: NDR64_BIND_CONTEXT,
}
impl ::core::marker::Copy for NDR64_BINDINGS {}
impl ::core::clone::Clone for NDR64_BINDINGS {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for NDR64_BINDINGS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_BINDINGS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_BINDINGS>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_BINDINGS {}
impl ::core::default::Default for NDR64_BINDINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_BIND_AND_NOTIFY_EXTENSION {
    pub Binding: NDR64_BIND_CONTEXT,
    pub NotifyIndex: u16,
}
impl ::core::marker::Copy for NDR64_BIND_AND_NOTIFY_EXTENSION {}
impl ::core::clone::Clone for NDR64_BIND_AND_NOTIFY_EXTENSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_BIND_AND_NOTIFY_EXTENSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_BIND_AND_NOTIFY_EXTENSION").field("Binding", &self.Binding).field("NotifyIndex", &self.NotifyIndex).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_BIND_AND_NOTIFY_EXTENSION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_BIND_AND_NOTIFY_EXTENSION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_BIND_AND_NOTIFY_EXTENSION>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_BIND_AND_NOTIFY_EXTENSION {}
impl ::core::default::Default for NDR64_BIND_AND_NOTIFY_EXTENSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_BIND_CONTEXT {
    pub HandleType: u8,
    pub Flags: u8,
    pub StackOffset: u16,
    pub RoutineIndex: u8,
    pub Ordinal: u8,
}
impl ::core::marker::Copy for NDR64_BIND_CONTEXT {}
impl ::core::clone::Clone for NDR64_BIND_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_BIND_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_BIND_CONTEXT").field("HandleType", &self.HandleType).field("Flags", &self.Flags).field("StackOffset", &self.StackOffset).field("RoutineIndex", &self.RoutineIndex).field("Ordinal", &self.Ordinal).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_BIND_CONTEXT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_BIND_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_BIND_CONTEXT>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_BIND_CONTEXT {}
impl ::core::default::Default for NDR64_BIND_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_BIND_GENERIC {
    pub HandleType: u8,
    pub Flags: u8,
    pub StackOffset: u16,
    pub RoutineIndex: u8,
    pub Size: u8,
}
impl ::core::marker::Copy for NDR64_BIND_GENERIC {}
impl ::core::clone::Clone for NDR64_BIND_GENERIC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_BIND_GENERIC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_BIND_GENERIC").field("HandleType", &self.HandleType).field("Flags", &self.Flags).field("StackOffset", &self.StackOffset).field("RoutineIndex", &self.RoutineIndex).field("Size", &self.Size).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_BIND_GENERIC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_BIND_GENERIC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_BIND_GENERIC>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_BIND_GENERIC {}
impl ::core::default::Default for NDR64_BIND_GENERIC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_BIND_PRIMITIVE {
    pub HandleType: u8,
    pub Flags: u8,
    pub StackOffset: u16,
    pub Reserved: u16,
}
impl ::core::marker::Copy for NDR64_BIND_PRIMITIVE {}
impl ::core::clone::Clone for NDR64_BIND_PRIMITIVE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_BIND_PRIMITIVE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_BIND_PRIMITIVE").field("HandleType", &self.HandleType).field("Flags", &self.Flags).field("StackOffset", &self.StackOffset).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_BIND_PRIMITIVE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_BIND_PRIMITIVE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_BIND_PRIMITIVE>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_BIND_PRIMITIVE {}
impl ::core::default::Default for NDR64_BIND_PRIMITIVE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_BOGUS_ARRAY_HEADER_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: NDR64_ARRAY_FLAGS,
    pub NumberDims: u8,
    pub NumberElements: u32,
    pub Element: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for NDR64_BOGUS_ARRAY_HEADER_FORMAT {}
impl ::core::clone::Clone for NDR64_BOGUS_ARRAY_HEADER_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_BOGUS_ARRAY_HEADER_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_BOGUS_ARRAY_HEADER_FORMAT").field("FormatCode", &self.FormatCode).field("Alignment", &self.Alignment).field("Flags", &self.Flags).field("NumberDims", &self.NumberDims).field("NumberElements", &self.NumberElements).field("Element", &self.Element).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_BOGUS_ARRAY_HEADER_FORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_BOGUS_ARRAY_HEADER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_BOGUS_ARRAY_HEADER_FORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_BOGUS_ARRAY_HEADER_FORMAT {}
impl ::core::default::Default for NDR64_BOGUS_ARRAY_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_BOGUS_STRUCTURE_HEADER_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: NDR64_STRUCTURE_FLAGS,
    pub Reserve: u8,
    pub MemorySize: u32,
    pub OriginalMemberLayout: *mut ::core::ffi::c_void,
    pub OriginalPointerLayout: *mut ::core::ffi::c_void,
    pub PointerLayout: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for NDR64_BOGUS_STRUCTURE_HEADER_FORMAT {}
impl ::core::clone::Clone for NDR64_BOGUS_STRUCTURE_HEADER_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_BOGUS_STRUCTURE_HEADER_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_BOGUS_STRUCTURE_HEADER_FORMAT").field("FormatCode", &self.FormatCode).field("Alignment", &self.Alignment).field("Flags", &self.Flags).field("Reserve", &self.Reserve).field("MemorySize", &self.MemorySize).field("OriginalMemberLayout", &self.OriginalMemberLayout).field("OriginalPointerLayout", &self.OriginalPointerLayout).field("PointerLayout", &self.PointerLayout).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_BOGUS_STRUCTURE_HEADER_FORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_BOGUS_STRUCTURE_HEADER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_BOGUS_STRUCTURE_HEADER_FORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_BOGUS_STRUCTURE_HEADER_FORMAT {}
impl ::core::default::Default for NDR64_BOGUS_STRUCTURE_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_BUFFER_ALIGN_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Reserved: u16,
    pub Reserved2: u32,
}
impl ::core::marker::Copy for NDR64_BUFFER_ALIGN_FORMAT {}
impl ::core::clone::Clone for NDR64_BUFFER_ALIGN_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_BUFFER_ALIGN_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_BUFFER_ALIGN_FORMAT").field("FormatCode", &self.FormatCode).field("Alignment", &self.Alignment).field("Reserved", &self.Reserved).field("Reserved2", &self.Reserved2).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_BUFFER_ALIGN_FORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_BUFFER_ALIGN_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_BUFFER_ALIGN_FORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_BUFFER_ALIGN_FORMAT {}
impl ::core::default::Default for NDR64_BUFFER_ALIGN_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_CONFORMANT_STRING_FORMAT {
    pub Header: NDR64_STRING_HEADER_FORMAT,
}
impl ::core::marker::Copy for NDR64_CONFORMANT_STRING_FORMAT {}
impl ::core::clone::Clone for NDR64_CONFORMANT_STRING_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_CONFORMANT_STRING_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_CONFORMANT_STRING_FORMAT").field("Header", &self.Header).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_CONFORMANT_STRING_FORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_CONFORMANT_STRING_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_CONFORMANT_STRING_FORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_CONFORMANT_STRING_FORMAT {}
impl ::core::default::Default for NDR64_CONFORMANT_STRING_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_CONF_ARRAY_HEADER_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: NDR64_ARRAY_FLAGS,
    pub Reserved: u8,
    pub ElementSize: u32,
    pub ConfDescriptor: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for NDR64_CONF_ARRAY_HEADER_FORMAT {}
impl ::core::clone::Clone for NDR64_CONF_ARRAY_HEADER_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_CONF_ARRAY_HEADER_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_CONF_ARRAY_HEADER_FORMAT").field("FormatCode", &self.FormatCode).field("Alignment", &self.Alignment).field("Flags", &self.Flags).field("Reserved", &self.Reserved).field("ElementSize", &self.ElementSize).field("ConfDescriptor", &self.ConfDescriptor).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_CONF_ARRAY_HEADER_FORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_CONF_ARRAY_HEADER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_CONF_ARRAY_HEADER_FORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_CONF_ARRAY_HEADER_FORMAT {}
impl ::core::default::Default for NDR64_CONF_ARRAY_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_CONF_BOGUS_STRUCTURE_HEADER_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: NDR64_STRUCTURE_FLAGS,
    pub Dimensions: u8,
    pub MemorySize: u32,
    pub OriginalMemberLayout: *mut ::core::ffi::c_void,
    pub OriginalPointerLayout: *mut ::core::ffi::c_void,
    pub PointerLayout: *mut ::core::ffi::c_void,
    pub ConfArrayDescription: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for NDR64_CONF_BOGUS_STRUCTURE_HEADER_FORMAT {}
impl ::core::clone::Clone for NDR64_CONF_BOGUS_STRUCTURE_HEADER_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_CONF_BOGUS_STRUCTURE_HEADER_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_CONF_BOGUS_STRUCTURE_HEADER_FORMAT").field("FormatCode", &self.FormatCode).field("Alignment", &self.Alignment).field("Flags", &self.Flags).field("Dimensions", &self.Dimensions).field("MemorySize", &self.MemorySize).field("OriginalMemberLayout", &self.OriginalMemberLayout).field("OriginalPointerLayout", &self.OriginalPointerLayout).field("PointerLayout", &self.PointerLayout).field("ConfArrayDescription", &self.ConfArrayDescription).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_CONF_BOGUS_STRUCTURE_HEADER_FORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_CONF_BOGUS_STRUCTURE_HEADER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_CONF_BOGUS_STRUCTURE_HEADER_FORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_CONF_BOGUS_STRUCTURE_HEADER_FORMAT {}
impl ::core::default::Default for NDR64_CONF_BOGUS_STRUCTURE_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_CONF_STRUCTURE_HEADER_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: NDR64_STRUCTURE_FLAGS,
    pub Reserve: u8,
    pub MemorySize: u32,
    pub ArrayDescription: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for NDR64_CONF_STRUCTURE_HEADER_FORMAT {}
impl ::core::clone::Clone for NDR64_CONF_STRUCTURE_HEADER_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_CONF_STRUCTURE_HEADER_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_CONF_STRUCTURE_HEADER_FORMAT").field("FormatCode", &self.FormatCode).field("Alignment", &self.Alignment).field("Flags", &self.Flags).field("Reserve", &self.Reserve).field("MemorySize", &self.MemorySize).field("ArrayDescription", &self.ArrayDescription).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_CONF_STRUCTURE_HEADER_FORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_CONF_STRUCTURE_HEADER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_CONF_STRUCTURE_HEADER_FORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_CONF_STRUCTURE_HEADER_FORMAT {}
impl ::core::default::Default for NDR64_CONF_STRUCTURE_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_CONF_VAR_ARRAY_HEADER_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: NDR64_ARRAY_FLAGS,
    pub Reserved: u8,
    pub ElementSize: u32,
    pub ConfDescriptor: *mut ::core::ffi::c_void,
    pub VarDescriptor: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for NDR64_CONF_VAR_ARRAY_HEADER_FORMAT {}
impl ::core::clone::Clone for NDR64_CONF_VAR_ARRAY_HEADER_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_CONF_VAR_ARRAY_HEADER_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_CONF_VAR_ARRAY_HEADER_FORMAT").field("FormatCode", &self.FormatCode).field("Alignment", &self.Alignment).field("Flags", &self.Flags).field("Reserved", &self.Reserved).field("ElementSize", &self.ElementSize).field("ConfDescriptor", &self.ConfDescriptor).field("VarDescriptor", &self.VarDescriptor).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_CONF_VAR_ARRAY_HEADER_FORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_CONF_VAR_ARRAY_HEADER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_CONF_VAR_ARRAY_HEADER_FORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_CONF_VAR_ARRAY_HEADER_FORMAT {}
impl ::core::default::Default for NDR64_CONF_VAR_ARRAY_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_CONF_VAR_BOGUS_ARRAY_HEADER_FORMAT {
    pub FixedArrayFormat: NDR64_BOGUS_ARRAY_HEADER_FORMAT,
    pub ConfDescription: *mut ::core::ffi::c_void,
    pub VarDescription: *mut ::core::ffi::c_void,
    pub OffsetDescription: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for NDR64_CONF_VAR_BOGUS_ARRAY_HEADER_FORMAT {}
impl ::core::clone::Clone for NDR64_CONF_VAR_BOGUS_ARRAY_HEADER_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_CONF_VAR_BOGUS_ARRAY_HEADER_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_CONF_VAR_BOGUS_ARRAY_HEADER_FORMAT").field("FixedArrayFormat", &self.FixedArrayFormat).field("ConfDescription", &self.ConfDescription).field("VarDescription", &self.VarDescription).field("OffsetDescription", &self.OffsetDescription).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_CONF_VAR_BOGUS_ARRAY_HEADER_FORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_CONF_VAR_BOGUS_ARRAY_HEADER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_CONF_VAR_BOGUS_ARRAY_HEADER_FORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_CONF_VAR_BOGUS_ARRAY_HEADER_FORMAT {}
impl ::core::default::Default for NDR64_CONF_VAR_BOGUS_ARRAY_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_CONSTANT_IID_FORMAT {
    pub FormatCode: u8,
    pub Flags: u8,
    pub Reserved: u16,
    pub Guid: ::windows::core::GUID,
}
impl ::core::marker::Copy for NDR64_CONSTANT_IID_FORMAT {}
impl ::core::clone::Clone for NDR64_CONSTANT_IID_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_CONSTANT_IID_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_CONSTANT_IID_FORMAT").field("FormatCode", &self.FormatCode).field("Flags", &self.Flags).field("Reserved", &self.Reserved).field("Guid", &self.Guid).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_CONSTANT_IID_FORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_CONSTANT_IID_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_CONSTANT_IID_FORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_CONSTANT_IID_FORMAT {}
impl ::core::default::Default for NDR64_CONSTANT_IID_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_CONTEXT_HANDLE_FLAGS {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for NDR64_CONTEXT_HANDLE_FLAGS {}
impl ::core::clone::Clone for NDR64_CONTEXT_HANDLE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_CONTEXT_HANDLE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_CONTEXT_HANDLE_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_CONTEXT_HANDLE_FLAGS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_CONTEXT_HANDLE_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_CONTEXT_HANDLE_FLAGS>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_CONTEXT_HANDLE_FLAGS {}
impl ::core::default::Default for NDR64_CONTEXT_HANDLE_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_CONTEXT_HANDLE_FORMAT {
    pub FormatCode: u8,
    pub ContextFlags: u8,
    pub RundownRoutineIndex: u8,
    pub Ordinal: u8,
}
impl ::core::marker::Copy for NDR64_CONTEXT_HANDLE_FORMAT {}
impl ::core::clone::Clone for NDR64_CONTEXT_HANDLE_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_CONTEXT_HANDLE_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_CONTEXT_HANDLE_FORMAT").field("FormatCode", &self.FormatCode).field("ContextFlags", &self.ContextFlags).field("RundownRoutineIndex", &self.RundownRoutineIndex).field("Ordinal", &self.Ordinal).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_CONTEXT_HANDLE_FORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_CONTEXT_HANDLE_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_CONTEXT_HANDLE_FORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_CONTEXT_HANDLE_FORMAT {}
impl ::core::default::Default for NDR64_CONTEXT_HANDLE_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_EMBEDDED_COMPLEX_FORMAT {
    pub FormatCode: u8,
    pub Reserve1: u8,
    pub Reserve2: u16,
    pub Type: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for NDR64_EMBEDDED_COMPLEX_FORMAT {}
impl ::core::clone::Clone for NDR64_EMBEDDED_COMPLEX_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_EMBEDDED_COMPLEX_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_EMBEDDED_COMPLEX_FORMAT").field("FormatCode", &self.FormatCode).field("Reserve1", &self.Reserve1).field("Reserve2", &self.Reserve2).field("Type", &self.Type).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_EMBEDDED_COMPLEX_FORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_EMBEDDED_COMPLEX_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_EMBEDDED_COMPLEX_FORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_EMBEDDED_COMPLEX_FORMAT {}
impl ::core::default::Default for NDR64_EMBEDDED_COMPLEX_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_ENCAPSULATED_UNION {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: u8,
    pub SwitchType: u8,
    pub MemoryOffset: u32,
    pub MemorySize: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for NDR64_ENCAPSULATED_UNION {}
impl ::core::clone::Clone for NDR64_ENCAPSULATED_UNION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_ENCAPSULATED_UNION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_ENCAPSULATED_UNION").field("FormatCode", &self.FormatCode).field("Alignment", &self.Alignment).field("Flags", &self.Flags).field("SwitchType", &self.SwitchType).field("MemoryOffset", &self.MemoryOffset).field("MemorySize", &self.MemorySize).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_ENCAPSULATED_UNION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_ENCAPSULATED_UNION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_ENCAPSULATED_UNION>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_ENCAPSULATED_UNION {}
impl ::core::default::Default for NDR64_ENCAPSULATED_UNION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_EXPR_CONST32 {
    pub ExprType: u8,
    pub Reserved: u8,
    pub Reserved1: u16,
    pub ConstValue: u32,
}
impl ::core::marker::Copy for NDR64_EXPR_CONST32 {}
impl ::core::clone::Clone for NDR64_EXPR_CONST32 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_EXPR_CONST32 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_EXPR_CONST32").field("ExprType", &self.ExprType).field("Reserved", &self.Reserved).field("Reserved1", &self.Reserved1).field("ConstValue", &self.ConstValue).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_EXPR_CONST32 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_EXPR_CONST32 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_EXPR_CONST32>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_EXPR_CONST32 {}
impl ::core::default::Default for NDR64_EXPR_CONST32 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_EXPR_CONST64 {
    pub ExprType: u8,
    pub Reserved: u8,
    pub Reserved1: u16,
    pub ConstValue: i64,
}
impl ::core::marker::Copy for NDR64_EXPR_CONST64 {}
impl ::core::clone::Clone for NDR64_EXPR_CONST64 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_EXPR_CONST64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_EXPR_CONST64").field("ExprType", &self.ExprType).field("Reserved", &self.Reserved).field("Reserved1", &self.Reserved1).field("ConstValue", &self.ConstValue).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_EXPR_CONST64 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_EXPR_CONST64 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_EXPR_CONST64>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_EXPR_CONST64 {}
impl ::core::default::Default for NDR64_EXPR_CONST64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_EXPR_NOOP {
    pub ExprType: u8,
    pub Size: u8,
    pub Reserved: u16,
}
impl ::core::marker::Copy for NDR64_EXPR_NOOP {}
impl ::core::clone::Clone for NDR64_EXPR_NOOP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_EXPR_NOOP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_EXPR_NOOP").field("ExprType", &self.ExprType).field("Size", &self.Size).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_EXPR_NOOP {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_EXPR_NOOP {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_EXPR_NOOP>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_EXPR_NOOP {}
impl ::core::default::Default for NDR64_EXPR_NOOP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_EXPR_OPERATOR {
    pub ExprType: u8,
    pub Operator: u8,
    pub CastType: u8,
    pub Reserved: u8,
}
impl ::core::marker::Copy for NDR64_EXPR_OPERATOR {}
impl ::core::clone::Clone for NDR64_EXPR_OPERATOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_EXPR_OPERATOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_EXPR_OPERATOR").field("ExprType", &self.ExprType).field("Operator", &self.Operator).field("CastType", &self.CastType).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_EXPR_OPERATOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_EXPR_OPERATOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_EXPR_OPERATOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_EXPR_OPERATOR {}
impl ::core::default::Default for NDR64_EXPR_OPERATOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_EXPR_VAR {
    pub ExprType: u8,
    pub VarType: u8,
    pub Reserved: u16,
    pub Offset: u32,
}
impl ::core::marker::Copy for NDR64_EXPR_VAR {}
impl ::core::clone::Clone for NDR64_EXPR_VAR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_EXPR_VAR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_EXPR_VAR").field("ExprType", &self.ExprType).field("VarType", &self.VarType).field("Reserved", &self.Reserved).field("Offset", &self.Offset).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_EXPR_VAR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_EXPR_VAR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_EXPR_VAR>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_EXPR_VAR {}
impl ::core::default::Default for NDR64_EXPR_VAR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const NDR64_FC_AUTO_HANDLE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const NDR64_FC_BIND_GENERIC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const NDR64_FC_BIND_PRIMITIVE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const NDR64_FC_CALLBACK_HANDLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const NDR64_FC_EXPLICIT_HANDLE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const NDR64_FC_NO_HANDLE: u32 = 5u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_FIXED_REPEAT_FORMAT {
    pub RepeatFormat: NDR64_REPEAT_FORMAT,
    pub Iterations: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for NDR64_FIXED_REPEAT_FORMAT {}
impl ::core::clone::Clone for NDR64_FIXED_REPEAT_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_FIXED_REPEAT_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_FIXED_REPEAT_FORMAT").field("RepeatFormat", &self.RepeatFormat).field("Iterations", &self.Iterations).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_FIXED_REPEAT_FORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_FIXED_REPEAT_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_FIXED_REPEAT_FORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_FIXED_REPEAT_FORMAT {}
impl ::core::default::Default for NDR64_FIXED_REPEAT_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_FIX_ARRAY_HEADER_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: NDR64_ARRAY_FLAGS,
    pub Reserved: u8,
    pub TotalSize: u32,
}
impl ::core::marker::Copy for NDR64_FIX_ARRAY_HEADER_FORMAT {}
impl ::core::clone::Clone for NDR64_FIX_ARRAY_HEADER_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_FIX_ARRAY_HEADER_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_FIX_ARRAY_HEADER_FORMAT").field("FormatCode", &self.FormatCode).field("Alignment", &self.Alignment).field("Flags", &self.Flags).field("Reserved", &self.Reserved).field("TotalSize", &self.TotalSize).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_FIX_ARRAY_HEADER_FORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_FIX_ARRAY_HEADER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_FIX_ARRAY_HEADER_FORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_FIX_ARRAY_HEADER_FORMAT {}
impl ::core::default::Default for NDR64_FIX_ARRAY_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_IID_FLAGS {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for NDR64_IID_FLAGS {}
impl ::core::clone::Clone for NDR64_IID_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_IID_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_IID_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_IID_FLAGS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_IID_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_IID_FLAGS>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_IID_FLAGS {}
impl ::core::default::Default for NDR64_IID_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_IID_FORMAT {
    pub FormatCode: u8,
    pub Flags: u8,
    pub Reserved: u16,
    pub IIDDescriptor: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for NDR64_IID_FORMAT {}
impl ::core::clone::Clone for NDR64_IID_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_IID_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_IID_FORMAT").field("FormatCode", &self.FormatCode).field("Flags", &self.Flags).field("Reserved", &self.Reserved).field("IIDDescriptor", &self.IIDDescriptor).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_IID_FORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_IID_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_IID_FORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_IID_FORMAT {}
impl ::core::default::Default for NDR64_IID_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_MEMPAD_FORMAT {
    pub FormatCode: u8,
    pub Reserve1: u8,
    pub MemPad: u16,
    pub Reserved2: u32,
}
impl ::core::marker::Copy for NDR64_MEMPAD_FORMAT {}
impl ::core::clone::Clone for NDR64_MEMPAD_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_MEMPAD_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_MEMPAD_FORMAT").field("FormatCode", &self.FormatCode).field("Reserve1", &self.Reserve1).field("MemPad", &self.MemPad).field("Reserved2", &self.Reserved2).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_MEMPAD_FORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_MEMPAD_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_MEMPAD_FORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_MEMPAD_FORMAT {}
impl ::core::default::Default for NDR64_MEMPAD_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_NON_CONFORMANT_STRING_FORMAT {
    pub Header: NDR64_STRING_HEADER_FORMAT,
    pub TotalSize: u32,
}
impl ::core::marker::Copy for NDR64_NON_CONFORMANT_STRING_FORMAT {}
impl ::core::clone::Clone for NDR64_NON_CONFORMANT_STRING_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_NON_CONFORMANT_STRING_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_NON_CONFORMANT_STRING_FORMAT").field("Header", &self.Header).field("TotalSize", &self.TotalSize).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_NON_CONFORMANT_STRING_FORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_NON_CONFORMANT_STRING_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_NON_CONFORMANT_STRING_FORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_NON_CONFORMANT_STRING_FORMAT {}
impl ::core::default::Default for NDR64_NON_CONFORMANT_STRING_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_NON_ENCAPSULATED_UNION {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: u8,
    pub SwitchType: u8,
    pub MemorySize: u32,
    pub Switch: *mut ::core::ffi::c_void,
    pub Reserved: u32,
}
impl ::core::marker::Copy for NDR64_NON_ENCAPSULATED_UNION {}
impl ::core::clone::Clone for NDR64_NON_ENCAPSULATED_UNION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_NON_ENCAPSULATED_UNION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_NON_ENCAPSULATED_UNION").field("FormatCode", &self.FormatCode).field("Alignment", &self.Alignment).field("Flags", &self.Flags).field("SwitchType", &self.SwitchType).field("MemorySize", &self.MemorySize).field("Switch", &self.Switch).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_NON_ENCAPSULATED_UNION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_NON_ENCAPSULATED_UNION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_NON_ENCAPSULATED_UNION>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_NON_ENCAPSULATED_UNION {}
impl ::core::default::Default for NDR64_NON_ENCAPSULATED_UNION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_NO_REPEAT_FORMAT {
    pub FormatCode: u8,
    pub Flags: u8,
    pub Reserved1: u16,
    pub Reserved2: u32,
}
impl ::core::marker::Copy for NDR64_NO_REPEAT_FORMAT {}
impl ::core::clone::Clone for NDR64_NO_REPEAT_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_NO_REPEAT_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_NO_REPEAT_FORMAT").field("FormatCode", &self.FormatCode).field("Flags", &self.Flags).field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_NO_REPEAT_FORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_NO_REPEAT_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_NO_REPEAT_FORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_NO_REPEAT_FORMAT {}
impl ::core::default::Default for NDR64_NO_REPEAT_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_PARAM_FLAGS {
    pub _bitfield: u16,
}
impl ::core::marker::Copy for NDR64_PARAM_FLAGS {}
impl ::core::clone::Clone for NDR64_PARAM_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_PARAM_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_PARAM_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_PARAM_FLAGS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_PARAM_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_PARAM_FLAGS>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_PARAM_FLAGS {}
impl ::core::default::Default for NDR64_PARAM_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_PARAM_FORMAT {
    pub Type: *mut ::core::ffi::c_void,
    pub Attributes: NDR64_PARAM_FLAGS,
    pub Reserved: u16,
    pub StackOffset: u32,
}
impl ::core::marker::Copy for NDR64_PARAM_FORMAT {}
impl ::core::clone::Clone for NDR64_PARAM_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_PARAM_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_PARAM_FORMAT").field("Type", &self.Type).field("Attributes", &self.Attributes).field("Reserved", &self.Reserved).field("StackOffset", &self.StackOffset).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_PARAM_FORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_PARAM_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_PARAM_FORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_PARAM_FORMAT {}
impl ::core::default::Default for NDR64_PARAM_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_PIPE_FLAGS {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for NDR64_PIPE_FLAGS {}
impl ::core::clone::Clone for NDR64_PIPE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_PIPE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_PIPE_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_PIPE_FLAGS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_PIPE_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_PIPE_FLAGS>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_PIPE_FLAGS {}
impl ::core::default::Default for NDR64_PIPE_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_PIPE_FORMAT {
    pub FormatCode: u8,
    pub Flags: u8,
    pub Alignment: u8,
    pub Reserved: u8,
    pub Type: *mut ::core::ffi::c_void,
    pub MemorySize: u32,
    pub BufferSize: u32,
}
impl ::core::marker::Copy for NDR64_PIPE_FORMAT {}
impl ::core::clone::Clone for NDR64_PIPE_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_PIPE_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_PIPE_FORMAT").field("FormatCode", &self.FormatCode).field("Flags", &self.Flags).field("Alignment", &self.Alignment).field("Reserved", &self.Reserved).field("Type", &self.Type).field("MemorySize", &self.MemorySize).field("BufferSize", &self.BufferSize).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_PIPE_FORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_PIPE_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_PIPE_FORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_PIPE_FORMAT {}
impl ::core::default::Default for NDR64_PIPE_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_POINTER_FORMAT {
    pub FormatCode: u8,
    pub Flags: u8,
    pub Reserved: u16,
    pub Pointee: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for NDR64_POINTER_FORMAT {}
impl ::core::clone::Clone for NDR64_POINTER_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_POINTER_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_POINTER_FORMAT").field("FormatCode", &self.FormatCode).field("Flags", &self.Flags).field("Reserved", &self.Reserved).field("Pointee", &self.Pointee).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_POINTER_FORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_POINTER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_POINTER_FORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_POINTER_FORMAT {}
impl ::core::default::Default for NDR64_POINTER_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_POINTER_INSTANCE_HEADER_FORMAT {
    pub Offset: u32,
    pub Reserved: u32,
}
impl ::core::marker::Copy for NDR64_POINTER_INSTANCE_HEADER_FORMAT {}
impl ::core::clone::Clone for NDR64_POINTER_INSTANCE_HEADER_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_POINTER_INSTANCE_HEADER_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_POINTER_INSTANCE_HEADER_FORMAT").field("Offset", &self.Offset).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_POINTER_INSTANCE_HEADER_FORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_POINTER_INSTANCE_HEADER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_POINTER_INSTANCE_HEADER_FORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_POINTER_INSTANCE_HEADER_FORMAT {}
impl ::core::default::Default for NDR64_POINTER_INSTANCE_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_POINTER_REPEAT_FLAGS {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for NDR64_POINTER_REPEAT_FLAGS {}
impl ::core::clone::Clone for NDR64_POINTER_REPEAT_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_POINTER_REPEAT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_POINTER_REPEAT_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_POINTER_REPEAT_FLAGS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_POINTER_REPEAT_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_POINTER_REPEAT_FLAGS>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_POINTER_REPEAT_FLAGS {}
impl ::core::default::Default for NDR64_POINTER_REPEAT_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_PROC_FLAGS {
    pub _bitfield: u32,
}
impl ::core::marker::Copy for NDR64_PROC_FLAGS {}
impl ::core::clone::Clone for NDR64_PROC_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_PROC_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_PROC_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_PROC_FLAGS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_PROC_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_PROC_FLAGS>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_PROC_FLAGS {}
impl ::core::default::Default for NDR64_PROC_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_PROC_FORMAT {
    pub Flags: u32,
    pub StackSize: u32,
    pub ConstantClientBufferSize: u32,
    pub ConstantServerBufferSize: u32,
    pub RpcFlags: u16,
    pub FloatDoubleMask: u16,
    pub NumberOfParams: u16,
    pub ExtensionSize: u16,
}
impl ::core::marker::Copy for NDR64_PROC_FORMAT {}
impl ::core::clone::Clone for NDR64_PROC_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_PROC_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_PROC_FORMAT").field("Flags", &self.Flags).field("StackSize", &self.StackSize).field("ConstantClientBufferSize", &self.ConstantClientBufferSize).field("ConstantServerBufferSize", &self.ConstantServerBufferSize).field("RpcFlags", &self.RpcFlags).field("FloatDoubleMask", &self.FloatDoubleMask).field("NumberOfParams", &self.NumberOfParams).field("ExtensionSize", &self.ExtensionSize).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_PROC_FORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_PROC_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_PROC_FORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_PROC_FORMAT {}
impl ::core::default::Default for NDR64_PROC_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_RANGED_STRING_FORMAT {
    pub Header: NDR64_STRING_HEADER_FORMAT,
    pub Reserved: u32,
    pub Min: u64,
    pub Max: u64,
}
impl ::core::marker::Copy for NDR64_RANGED_STRING_FORMAT {}
impl ::core::clone::Clone for NDR64_RANGED_STRING_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_RANGED_STRING_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_RANGED_STRING_FORMAT").field("Header", &self.Header).field("Reserved", &self.Reserved).field("Min", &self.Min).field("Max", &self.Max).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_RANGED_STRING_FORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_RANGED_STRING_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_RANGED_STRING_FORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_RANGED_STRING_FORMAT {}
impl ::core::default::Default for NDR64_RANGED_STRING_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_RANGE_FORMAT {
    pub FormatCode: u8,
    pub RangeType: u8,
    pub Reserved: u16,
    pub MinValue: i64,
    pub MaxValue: i64,
}
impl ::core::marker::Copy for NDR64_RANGE_FORMAT {}
impl ::core::clone::Clone for NDR64_RANGE_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_RANGE_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_RANGE_FORMAT").field("FormatCode", &self.FormatCode).field("RangeType", &self.RangeType).field("Reserved", &self.Reserved).field("MinValue", &self.MinValue).field("MaxValue", &self.MaxValue).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_RANGE_FORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_RANGE_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_RANGE_FORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_RANGE_FORMAT {}
impl ::core::default::Default for NDR64_RANGE_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_RANGE_PIPE_FORMAT {
    pub FormatCode: u8,
    pub Flags: u8,
    pub Alignment: u8,
    pub Reserved: u8,
    pub Type: *mut ::core::ffi::c_void,
    pub MemorySize: u32,
    pub BufferSize: u32,
    pub MinValue: u32,
    pub MaxValue: u32,
}
impl ::core::marker::Copy for NDR64_RANGE_PIPE_FORMAT {}
impl ::core::clone::Clone for NDR64_RANGE_PIPE_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_RANGE_PIPE_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_RANGE_PIPE_FORMAT").field("FormatCode", &self.FormatCode).field("Flags", &self.Flags).field("Alignment", &self.Alignment).field("Reserved", &self.Reserved).field("Type", &self.Type).field("MemorySize", &self.MemorySize).field("BufferSize", &self.BufferSize).field("MinValue", &self.MinValue).field("MaxValue", &self.MaxValue).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_RANGE_PIPE_FORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_RANGE_PIPE_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_RANGE_PIPE_FORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_RANGE_PIPE_FORMAT {}
impl ::core::default::Default for NDR64_RANGE_PIPE_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_REPEAT_FORMAT {
    pub FormatCode: u8,
    pub Flags: NDR64_POINTER_REPEAT_FLAGS,
    pub Reserved: u16,
    pub Increment: u32,
    pub OffsetToArray: u32,
    pub NumberOfPointers: u32,
}
impl ::core::marker::Copy for NDR64_REPEAT_FORMAT {}
impl ::core::clone::Clone for NDR64_REPEAT_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_REPEAT_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_REPEAT_FORMAT").field("FormatCode", &self.FormatCode).field("Flags", &self.Flags).field("Reserved", &self.Reserved).field("Increment", &self.Increment).field("OffsetToArray", &self.OffsetToArray).field("NumberOfPointers", &self.NumberOfPointers).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_REPEAT_FORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_REPEAT_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_REPEAT_FORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_REPEAT_FORMAT {}
impl ::core::default::Default for NDR64_REPEAT_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_RPC_FLAGS {
    pub _bitfield: u16,
}
impl ::core::marker::Copy for NDR64_RPC_FLAGS {}
impl ::core::clone::Clone for NDR64_RPC_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_RPC_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_RPC_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_RPC_FLAGS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_RPC_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_RPC_FLAGS>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_RPC_FLAGS {}
impl ::core::default::Default for NDR64_RPC_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_SIMPLE_MEMBER_FORMAT {
    pub FormatCode: u8,
    pub Reserved1: u8,
    pub Reserved2: u16,
    pub Reserved3: u32,
}
impl ::core::marker::Copy for NDR64_SIMPLE_MEMBER_FORMAT {}
impl ::core::clone::Clone for NDR64_SIMPLE_MEMBER_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_SIMPLE_MEMBER_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_SIMPLE_MEMBER_FORMAT").field("FormatCode", &self.FormatCode).field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).field("Reserved3", &self.Reserved3).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_SIMPLE_MEMBER_FORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_SIMPLE_MEMBER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_SIMPLE_MEMBER_FORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_SIMPLE_MEMBER_FORMAT {}
impl ::core::default::Default for NDR64_SIMPLE_MEMBER_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_SIMPLE_REGION_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub RegionSize: u16,
    pub Reserved: u32,
}
impl ::core::marker::Copy for NDR64_SIMPLE_REGION_FORMAT {}
impl ::core::clone::Clone for NDR64_SIMPLE_REGION_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_SIMPLE_REGION_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_SIMPLE_REGION_FORMAT").field("FormatCode", &self.FormatCode).field("Alignment", &self.Alignment).field("RegionSize", &self.RegionSize).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_SIMPLE_REGION_FORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_SIMPLE_REGION_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_SIMPLE_REGION_FORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_SIMPLE_REGION_FORMAT {}
impl ::core::default::Default for NDR64_SIMPLE_REGION_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_SIZED_CONFORMANT_STRING_FORMAT {
    pub Header: NDR64_STRING_HEADER_FORMAT,
    pub SizeDescription: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for NDR64_SIZED_CONFORMANT_STRING_FORMAT {}
impl ::core::clone::Clone for NDR64_SIZED_CONFORMANT_STRING_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_SIZED_CONFORMANT_STRING_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_SIZED_CONFORMANT_STRING_FORMAT").field("Header", &self.Header).field("SizeDescription", &self.SizeDescription).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_SIZED_CONFORMANT_STRING_FORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_SIZED_CONFORMANT_STRING_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_SIZED_CONFORMANT_STRING_FORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_SIZED_CONFORMANT_STRING_FORMAT {}
impl ::core::default::Default for NDR64_SIZED_CONFORMANT_STRING_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_STRING_FLAGS {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for NDR64_STRING_FLAGS {}
impl ::core::clone::Clone for NDR64_STRING_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_STRING_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_STRING_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_STRING_FLAGS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_STRING_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_STRING_FLAGS>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_STRING_FLAGS {}
impl ::core::default::Default for NDR64_STRING_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_STRING_HEADER_FORMAT {
    pub FormatCode: u8,
    pub Flags: NDR64_STRING_FLAGS,
    pub ElementSize: u16,
}
impl ::core::marker::Copy for NDR64_STRING_HEADER_FORMAT {}
impl ::core::clone::Clone for NDR64_STRING_HEADER_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_STRING_HEADER_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_STRING_HEADER_FORMAT").field("FormatCode", &self.FormatCode).field("Flags", &self.Flags).field("ElementSize", &self.ElementSize).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_STRING_HEADER_FORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_STRING_HEADER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_STRING_HEADER_FORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_STRING_HEADER_FORMAT {}
impl ::core::default::Default for NDR64_STRING_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_STRUCTURE_FLAGS {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for NDR64_STRUCTURE_FLAGS {}
impl ::core::clone::Clone for NDR64_STRUCTURE_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_STRUCTURE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_STRUCTURE_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_STRUCTURE_FLAGS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_STRUCTURE_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_STRUCTURE_FLAGS>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_STRUCTURE_FLAGS {}
impl ::core::default::Default for NDR64_STRUCTURE_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_STRUCTURE_HEADER_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: NDR64_STRUCTURE_FLAGS,
    pub Reserve: u8,
    pub MemorySize: u32,
}
impl ::core::marker::Copy for NDR64_STRUCTURE_HEADER_FORMAT {}
impl ::core::clone::Clone for NDR64_STRUCTURE_HEADER_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_STRUCTURE_HEADER_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_STRUCTURE_HEADER_FORMAT").field("FormatCode", &self.FormatCode).field("Alignment", &self.Alignment).field("Flags", &self.Flags).field("Reserve", &self.Reserve).field("MemorySize", &self.MemorySize).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_STRUCTURE_HEADER_FORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_STRUCTURE_HEADER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_STRUCTURE_HEADER_FORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_STRUCTURE_HEADER_FORMAT {}
impl ::core::default::Default for NDR64_STRUCTURE_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_SYSTEM_HANDLE_FORMAT {
    pub FormatCode: u8,
    pub HandleType: u8,
    pub DesiredAccess: u32,
}
impl ::core::marker::Copy for NDR64_SYSTEM_HANDLE_FORMAT {}
impl ::core::clone::Clone for NDR64_SYSTEM_HANDLE_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_SYSTEM_HANDLE_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_SYSTEM_HANDLE_FORMAT").field("FormatCode", &self.FormatCode).field("HandleType", &self.HandleType).field("DesiredAccess", &self.DesiredAccess).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_SYSTEM_HANDLE_FORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_SYSTEM_HANDLE_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_SYSTEM_HANDLE_FORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_SYSTEM_HANDLE_FORMAT {}
impl ::core::default::Default for NDR64_SYSTEM_HANDLE_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_TRANSMIT_AS_FLAGS {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for NDR64_TRANSMIT_AS_FLAGS {}
impl ::core::clone::Clone for NDR64_TRANSMIT_AS_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_TRANSMIT_AS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_TRANSMIT_AS_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_TRANSMIT_AS_FLAGS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_TRANSMIT_AS_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_TRANSMIT_AS_FLAGS>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_TRANSMIT_AS_FLAGS {}
impl ::core::default::Default for NDR64_TRANSMIT_AS_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_TRANSMIT_AS_FORMAT {
    pub FormatCode: u8,
    pub Flags: u8,
    pub RoutineIndex: u16,
    pub TransmittedTypeWireAlignment: u16,
    pub MemoryAlignment: u16,
    pub PresentedTypeMemorySize: u32,
    pub TransmittedTypeBufferSize: u32,
    pub TransmittedType: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for NDR64_TRANSMIT_AS_FORMAT {}
impl ::core::clone::Clone for NDR64_TRANSMIT_AS_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_TRANSMIT_AS_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_TRANSMIT_AS_FORMAT").field("FormatCode", &self.FormatCode).field("Flags", &self.Flags).field("RoutineIndex", &self.RoutineIndex).field("TransmittedTypeWireAlignment", &self.TransmittedTypeWireAlignment).field("MemoryAlignment", &self.MemoryAlignment).field("PresentedTypeMemorySize", &self.PresentedTypeMemorySize).field("TransmittedTypeBufferSize", &self.TransmittedTypeBufferSize).field("TransmittedType", &self.TransmittedType).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_TRANSMIT_AS_FORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_TRANSMIT_AS_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_TRANSMIT_AS_FORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_TRANSMIT_AS_FORMAT {}
impl ::core::default::Default for NDR64_TRANSMIT_AS_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_TYPE_STRICT_CONTEXT_HANDLE {
    pub FormatCode: u8,
    pub RealFormatCode: u8,
    pub Reserved: u16,
    pub Type: *mut ::core::ffi::c_void,
    pub CtxtFlags: u32,
    pub CtxtID: u32,
}
impl ::core::marker::Copy for NDR64_TYPE_STRICT_CONTEXT_HANDLE {}
impl ::core::clone::Clone for NDR64_TYPE_STRICT_CONTEXT_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_TYPE_STRICT_CONTEXT_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_TYPE_STRICT_CONTEXT_HANDLE").field("FormatCode", &self.FormatCode).field("RealFormatCode", &self.RealFormatCode).field("Reserved", &self.Reserved).field("Type", &self.Type).field("CtxtFlags", &self.CtxtFlags).field("CtxtID", &self.CtxtID).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_TYPE_STRICT_CONTEXT_HANDLE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_TYPE_STRICT_CONTEXT_HANDLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_TYPE_STRICT_CONTEXT_HANDLE>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_TYPE_STRICT_CONTEXT_HANDLE {}
impl ::core::default::Default for NDR64_TYPE_STRICT_CONTEXT_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_UNION_ARM {
    pub CaseValue: i64,
    pub Type: *mut ::core::ffi::c_void,
    pub Reserved: u32,
}
impl ::core::marker::Copy for NDR64_UNION_ARM {}
impl ::core::clone::Clone for NDR64_UNION_ARM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_UNION_ARM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_UNION_ARM").field("CaseValue", &self.CaseValue).field("Type", &self.Type).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_UNION_ARM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_UNION_ARM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_UNION_ARM>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_UNION_ARM {}
impl ::core::default::Default for NDR64_UNION_ARM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_UNION_ARM_SELECTOR {
    pub Reserved1: u8,
    pub Alignment: u8,
    pub Reserved2: u16,
    pub Arms: u32,
}
impl ::core::marker::Copy for NDR64_UNION_ARM_SELECTOR {}
impl ::core::clone::Clone for NDR64_UNION_ARM_SELECTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_UNION_ARM_SELECTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_UNION_ARM_SELECTOR").field("Reserved1", &self.Reserved1).field("Alignment", &self.Alignment).field("Reserved2", &self.Reserved2).field("Arms", &self.Arms).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_UNION_ARM_SELECTOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_UNION_ARM_SELECTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_UNION_ARM_SELECTOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_UNION_ARM_SELECTOR {}
impl ::core::default::Default for NDR64_UNION_ARM_SELECTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_USER_MARSHAL_FLAGS {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for NDR64_USER_MARSHAL_FLAGS {}
impl ::core::clone::Clone for NDR64_USER_MARSHAL_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_USER_MARSHAL_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_USER_MARSHAL_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_USER_MARSHAL_FLAGS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_USER_MARSHAL_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_USER_MARSHAL_FLAGS>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_USER_MARSHAL_FLAGS {}
impl ::core::default::Default for NDR64_USER_MARSHAL_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_USER_MARSHAL_FORMAT {
    pub FormatCode: u8,
    pub Flags: u8,
    pub RoutineIndex: u16,
    pub TransmittedTypeWireAlignment: u16,
    pub MemoryAlignment: u16,
    pub UserTypeMemorySize: u32,
    pub TransmittedTypeBufferSize: u32,
    pub TransmittedType: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for NDR64_USER_MARSHAL_FORMAT {}
impl ::core::clone::Clone for NDR64_USER_MARSHAL_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_USER_MARSHAL_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_USER_MARSHAL_FORMAT").field("FormatCode", &self.FormatCode).field("Flags", &self.Flags).field("RoutineIndex", &self.RoutineIndex).field("TransmittedTypeWireAlignment", &self.TransmittedTypeWireAlignment).field("MemoryAlignment", &self.MemoryAlignment).field("UserTypeMemorySize", &self.UserTypeMemorySize).field("TransmittedTypeBufferSize", &self.TransmittedTypeBufferSize).field("TransmittedType", &self.TransmittedType).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_USER_MARSHAL_FORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_USER_MARSHAL_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_USER_MARSHAL_FORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_USER_MARSHAL_FORMAT {}
impl ::core::default::Default for NDR64_USER_MARSHAL_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR64_VAR_ARRAY_HEADER_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: NDR64_ARRAY_FLAGS,
    pub Reserved: u8,
    pub TotalSize: u32,
    pub ElementSize: u32,
    pub VarDescriptor: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for NDR64_VAR_ARRAY_HEADER_FORMAT {}
impl ::core::clone::Clone for NDR64_VAR_ARRAY_HEADER_FORMAT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR64_VAR_ARRAY_HEADER_FORMAT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR64_VAR_ARRAY_HEADER_FORMAT").field("FormatCode", &self.FormatCode).field("Alignment", &self.Alignment).field("Flags", &self.Flags).field("Reserved", &self.Reserved).field("TotalSize", &self.TotalSize).field("ElementSize", &self.ElementSize).field("VarDescriptor", &self.VarDescriptor).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR64_VAR_ARRAY_HEADER_FORMAT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR64_VAR_ARRAY_HEADER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR64_VAR_ARRAY_HEADER_FORMAT>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR64_VAR_ARRAY_HEADER_FORMAT {}
impl ::core::default::Default for NDR64_VAR_ARRAY_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NDRCContextBinding(ccontext: isize) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NDRCContextBinding(ccontext: isize) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(NDRCContextBinding(::core::mem::transmute(ccontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NDRCContextMarshall(ccontext: isize, pbuff: *mut ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NDRCContextMarshall(ccontext: isize, pbuff: *mut ::core::ffi::c_void);
        }
        NDRCContextMarshall(::core::mem::transmute(ccontext), ::core::mem::transmute(pbuff))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NDRCContextUnmarshall(pccontext: *mut isize, hbinding: *const ::core::ffi::c_void, pbuff: *const ::core::ffi::c_void, datarepresentation: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NDRCContextUnmarshall(pccontext: *mut isize, hbinding: *const ::core::ffi::c_void, pbuff: *const ::core::ffi::c_void, datarepresentation: u32);
        }
        NDRCContextUnmarshall(::core::mem::transmute(pccontext), ::core::mem::transmute(hbinding), ::core::mem::transmute(pbuff), ::core::mem::transmute(datarepresentation))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NDRSContextMarshall(ccontext: *const NDR_SCONTEXT_1, pbuff: *mut ::core::ffi::c_void, userrundownin: NDR_RUNDOWN) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NDRSContextMarshall(ccontext: *const NDR_SCONTEXT_1, pbuff: *mut ::core::ffi::c_void, userrundownin: ::windows::core::RawPtr);
        }
        NDRSContextMarshall(::core::mem::transmute(ccontext), ::core::mem::transmute(pbuff), ::core::mem::transmute(userrundownin))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NDRSContextMarshall2(bindinghandle: *const ::core::ffi::c_void, ccontext: *const NDR_SCONTEXT_1, pbuff: *mut ::core::ffi::c_void, userrundownin: NDR_RUNDOWN, ctxguard: *const ::core::ffi::c_void, flags: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NDRSContextMarshall2(bindinghandle: *const ::core::ffi::c_void, ccontext: *const NDR_SCONTEXT_1, pbuff: *mut ::core::ffi::c_void, userrundownin: ::windows::core::RawPtr, ctxguard: *const ::core::ffi::c_void, flags: u32);
        }
        NDRSContextMarshall2(::core::mem::transmute(bindinghandle), ::core::mem::transmute(ccontext), ::core::mem::transmute(pbuff), ::core::mem::transmute(userrundownin), ::core::mem::transmute(ctxguard), ::core::mem::transmute(flags))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NDRSContextMarshallEx(bindinghandle: *const ::core::ffi::c_void, ccontext: *const NDR_SCONTEXT_1, pbuff: *mut ::core::ffi::c_void, userrundownin: NDR_RUNDOWN) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NDRSContextMarshallEx(bindinghandle: *const ::core::ffi::c_void, ccontext: *const NDR_SCONTEXT_1, pbuff: *mut ::core::ffi::c_void, userrundownin: ::windows::core::RawPtr);
        }
        NDRSContextMarshallEx(::core::mem::transmute(bindinghandle), ::core::mem::transmute(ccontext), ::core::mem::transmute(pbuff), ::core::mem::transmute(userrundownin))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NDRSContextUnmarshall(pbuff: *const ::core::ffi::c_void, datarepresentation: u32) -> *mut NDR_SCONTEXT_1 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NDRSContextUnmarshall(pbuff: *const ::core::ffi::c_void, datarepresentation: u32) -> *mut NDR_SCONTEXT_1;
        }
        ::core::mem::transmute(NDRSContextUnmarshall(::core::mem::transmute(pbuff), ::core::mem::transmute(datarepresentation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NDRSContextUnmarshall2(bindinghandle: *const ::core::ffi::c_void, pbuff: *const ::core::ffi::c_void, datarepresentation: u32, ctxguard: *const ::core::ffi::c_void, flags: u32) -> *mut NDR_SCONTEXT_1 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NDRSContextUnmarshall2(bindinghandle: *const ::core::ffi::c_void, pbuff: *const ::core::ffi::c_void, datarepresentation: u32, ctxguard: *const ::core::ffi::c_void, flags: u32) -> *mut NDR_SCONTEXT_1;
        }
        ::core::mem::transmute(NDRSContextUnmarshall2(::core::mem::transmute(bindinghandle), ::core::mem::transmute(pbuff), ::core::mem::transmute(datarepresentation), ::core::mem::transmute(ctxguard), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NDRSContextUnmarshallEx(bindinghandle: *const ::core::ffi::c_void, pbuff: *const ::core::ffi::c_void, datarepresentation: u32) -> *mut NDR_SCONTEXT_1 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NDRSContextUnmarshallEx(bindinghandle: *const ::core::ffi::c_void, pbuff: *const ::core::ffi::c_void, datarepresentation: u32) -> *mut NDR_SCONTEXT_1;
        }
        ::core::mem::transmute(NDRSContextUnmarshallEx(::core::mem::transmute(bindinghandle), ::core::mem::transmute(pbuff), ::core::mem::transmute(datarepresentation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
pub struct NDR_ALLOC_ALL_NODES_CONTEXT(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR_CS_ROUTINES {
    pub pSizeConvertRoutines: *mut NDR_CS_SIZE_CONVERT_ROUTINES,
    pub pTagGettingRoutines: *mut CS_TAG_GETTING_ROUTINE,
}
impl ::core::marker::Copy for NDR_CS_ROUTINES {}
impl ::core::clone::Clone for NDR_CS_ROUTINES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR_CS_ROUTINES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR_CS_ROUTINES").field("pSizeConvertRoutines", &self.pSizeConvertRoutines).field("pTagGettingRoutines", &self.pTagGettingRoutines).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR_CS_ROUTINES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR_CS_ROUTINES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR_CS_ROUTINES>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR_CS_ROUTINES {}
impl ::core::default::Default for NDR_CS_ROUTINES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR_CS_SIZE_CONVERT_ROUTINES {
    pub pfnNetSize: CS_TYPE_NET_SIZE_ROUTINE,
    pub pfnToNetCs: CS_TYPE_TO_NETCS_ROUTINE,
    pub pfnLocalSize: CS_TYPE_LOCAL_SIZE_ROUTINE,
    pub pfnFromNetCs: CS_TYPE_FROM_NETCS_ROUTINE,
}
impl ::core::marker::Copy for NDR_CS_SIZE_CONVERT_ROUTINES {}
impl ::core::clone::Clone for NDR_CS_SIZE_CONVERT_ROUTINES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR_CS_SIZE_CONVERT_ROUTINES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR_CS_SIZE_CONVERT_ROUTINES").field("pfnNetSize", &self.pfnNetSize.map(|f| f as usize)).field("pfnToNetCs", &self.pfnToNetCs.map(|f| f as usize)).field("pfnLocalSize", &self.pfnLocalSize.map(|f| f as usize)).field("pfnFromNetCs", &self.pfnFromNetCs.map(|f| f as usize)).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR_CS_SIZE_CONVERT_ROUTINES {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR_CS_SIZE_CONVERT_ROUTINES {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR_CS_SIZE_CONVERT_ROUTINES>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR_CS_SIZE_CONVERT_ROUTINES {}
impl ::core::default::Default for NDR_CS_SIZE_CONVERT_ROUTINES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const NDR_CUSTOM_OR_DEFAULT_ALLOCATOR: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const NDR_DEFAULT_ALLOCATOR: u32 = 536870912u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR_EXPR_DESC {
    pub pOffset: *const u16,
    pub pFormatExpr: *mut u8,
}
impl ::core::marker::Copy for NDR_EXPR_DESC {}
impl ::core::clone::Clone for NDR_EXPR_DESC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR_EXPR_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR_EXPR_DESC").field("pOffset", &self.pOffset).field("pFormatExpr", &self.pFormatExpr).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR_EXPR_DESC {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR_EXPR_DESC {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR_EXPR_DESC>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR_EXPR_DESC {}
impl ::core::default::Default for NDR_EXPR_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type NDR_NOTIFY2_ROUTINE = ::core::option::Option<unsafe extern "system" fn(flag: u8)>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type NDR_NOTIFY_ROUTINE = ::core::option::Option<unsafe extern "system" fn()>;
#[repr(C)]
pub struct NDR_POINTER_QUEUE_STATE(pub u8);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type NDR_RUNDOWN = ::core::option::Option<unsafe extern "system" fn(context: *mut ::core::ffi::c_void)>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct NDR_SCONTEXT_1 {
    pub pad: [*mut ::core::ffi::c_void; 2],
    pub userContext: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for NDR_SCONTEXT_1 {}
impl ::core::clone::Clone for NDR_SCONTEXT_1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NDR_SCONTEXT_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR_SCONTEXT_1").field("pad", &self.pad).field("userContext", &self.userContext).finish()
    }
}
unsafe impl ::windows::core::Abi for NDR_SCONTEXT_1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NDR_SCONTEXT_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR_SCONTEXT_1>()) == 0 }
    }
}
impl ::core::cmp::Eq for NDR_SCONTEXT_1 {}
impl ::core::default::Default for NDR_SCONTEXT_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct NDR_USER_MARSHAL_INFO {
    pub InformationLevel: u32,
    pub Anonymous: NDR_USER_MARSHAL_INFO_0,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for NDR_USER_MARSHAL_INFO {
    fn clone(&self) -> Self {
        Self { InformationLevel: self.InformationLevel, Anonymous: self.Anonymous.clone() }
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for NDR_USER_MARSHAL_INFO {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for NDR_USER_MARSHAL_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.InformationLevel == other.InformationLevel && self.Anonymous == other.Anonymous
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for NDR_USER_MARSHAL_INFO {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for NDR_USER_MARSHAL_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub union NDR_USER_MARSHAL_INFO_0 {
    pub Level1: ::core::mem::ManuallyDrop<NDR_USER_MARSHAL_INFO_LEVEL1>,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for NDR_USER_MARSHAL_INFO_0 {
    fn clone(&self) -> Self {
        unsafe { ::core::mem::transmute_copy(self) }
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for NDR_USER_MARSHAL_INFO_0 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for NDR_USER_MARSHAL_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NDR_USER_MARSHAL_INFO_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for NDR_USER_MARSHAL_INFO_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for NDR_USER_MARSHAL_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct NDR_USER_MARSHAL_INFO_LEVEL1 {
    pub Buffer: *mut ::core::ffi::c_void,
    pub BufferSize: u32,
    pub pfnAllocate: isize,
    pub pfnFree: isize,
    pub pRpcChannelBuffer: ::core::option::Option<super::Com::IRpcChannelBuffer>,
    pub Reserved: [usize; 5],
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for NDR_USER_MARSHAL_INFO_LEVEL1 {
    fn clone(&self) -> Self {
        Self {
            Buffer: self.Buffer,
            BufferSize: self.BufferSize,
            pfnAllocate: self.pfnAllocate,
            pfnFree: self.pfnFree,
            pRpcChannelBuffer: self.pRpcChannelBuffer.clone(),
            Reserved: self.Reserved,
        }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for NDR_USER_MARSHAL_INFO_LEVEL1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NDR_USER_MARSHAL_INFO_LEVEL1").field("Buffer", &self.Buffer).field("BufferSize", &self.BufferSize).field("pfnAllocate", &self.pfnAllocate).field("pfnFree", &self.pfnFree).field("pRpcChannelBuffer", &self.pRpcChannelBuffer).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for NDR_USER_MARSHAL_INFO_LEVEL1 {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for NDR_USER_MARSHAL_INFO_LEVEL1 {
    fn eq(&self, other: &Self) -> bool {
        self.Buffer == other.Buffer && self.BufferSize == other.BufferSize && self.pfnAllocate == other.pfnAllocate && self.pfnFree == other.pfnFree && self.pRpcChannelBuffer == other.pRpcChannelBuffer && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for NDR_USER_MARSHAL_INFO_LEVEL1 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for NDR_USER_MARSHAL_INFO_LEVEL1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const NT351_INTERFACE_SIZE: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn Ndr64AsyncClientCall(pproxyinfo: *mut MIDL_STUBLESS_PROXY_INFO, nprocnum: u32, preturnvalue: *mut ::core::ffi::c_void) -> CLIENT_CALL_RETURN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Ndr64AsyncClientCall(pproxyinfo: *mut MIDL_STUBLESS_PROXY_INFO, nprocnum: u32, preturnvalue: *mut ::core::ffi::c_void) -> CLIENT_CALL_RETURN;
        }
        ::core::mem::transmute(Ndr64AsyncClientCall(::core::mem::transmute(pproxyinfo), ::core::mem::transmute(nprocnum), ::core::mem::transmute(preturnvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn Ndr64AsyncServerCall64(prpcmsg: *mut RPC_MESSAGE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Ndr64AsyncServerCall64(prpcmsg: *mut RPC_MESSAGE);
        }
        Ndr64AsyncServerCall64(::core::mem::transmute(prpcmsg))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn Ndr64AsyncServerCallAll(prpcmsg: *mut RPC_MESSAGE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Ndr64AsyncServerCallAll(prpcmsg: *mut RPC_MESSAGE);
        }
        Ndr64AsyncServerCallAll(::core::mem::transmute(prpcmsg))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn Ndr64DcomAsyncClientCall(pproxyinfo: *mut MIDL_STUBLESS_PROXY_INFO, nprocnum: u32, preturnvalue: *mut ::core::ffi::c_void) -> CLIENT_CALL_RETURN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Ndr64DcomAsyncClientCall(pproxyinfo: *mut MIDL_STUBLESS_PROXY_INFO, nprocnum: u32, preturnvalue: *mut ::core::ffi::c_void) -> CLIENT_CALL_RETURN;
        }
        ::core::mem::transmute(Ndr64DcomAsyncClientCall(::core::mem::transmute(pproxyinfo), ::core::mem::transmute(nprocnum), ::core::mem::transmute(preturnvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn Ndr64DcomAsyncStubCall<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IRpcStubBuffer>, Param1: ::windows::core::IntoParam<'a, super::Com::IRpcChannelBuffer>>(pthis: Param0, pchannel: Param1, prpcmsg: *mut RPC_MESSAGE, pdwstubphase: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Ndr64DcomAsyncStubCall(pthis: ::windows::core::RawPtr, pchannel: ::windows::core::RawPtr, prpcmsg: *mut RPC_MESSAGE, pdwstubphase: *mut u32) -> i32;
        }
        ::core::mem::transmute(Ndr64DcomAsyncStubCall(pthis.into_param().abi(), pchannel.into_param().abi(), ::core::mem::transmute(prpcmsg), ::core::mem::transmute(pdwstubphase)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrAllocate(pstubmsg: *mut MIDL_STUB_MESSAGE, len: usize) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrAllocate(pstubmsg: *mut MIDL_STUB_MESSAGE, len: usize) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(NdrAllocate(::core::mem::transmute(pstubmsg), ::core::mem::transmute(len)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrAsyncClientCall(pstubdescriptor: *mut MIDL_STUB_DESC, pformat: *mut u8) -> CLIENT_CALL_RETURN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrAsyncClientCall(pstubdescriptor: *mut MIDL_STUB_DESC, pformat: *mut u8) -> CLIENT_CALL_RETURN;
        }
        ::core::mem::transmute(NdrAsyncClientCall(::core::mem::transmute(pstubdescriptor), ::core::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NdrAsyncServerCall(prpcmsg: *mut RPC_MESSAGE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrAsyncServerCall(prpcmsg: *mut RPC_MESSAGE);
        }
        NdrAsyncServerCall(::core::mem::transmute(prpcmsg))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrByteCountPointerBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrByteCountPointerBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
        }
        NdrByteCountPointerBufferSize(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrByteCountPointerFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrByteCountPointerFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
        }
        NdrByteCountPointerFree(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrByteCountPointerMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrByteCountPointerMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
        }
        ::core::mem::transmute(NdrByteCountPointerMarshall(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrByteCountPointerUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrByteCountPointerUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
        }
        ::core::mem::transmute(NdrByteCountPointerUnmarshall(::core::mem::transmute(pstubmsg), ::core::mem::transmute(ppmemory), ::core::mem::transmute(pformat), ::core::mem::transmute(fmustalloc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrClearOutParameters(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8, argaddr: *mut ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrClearOutParameters(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8, argaddr: *mut ::core::ffi::c_void);
        }
        NdrClearOutParameters(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pformat), ::core::mem::transmute(argaddr))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrClientCall2(pstubdescriptor: *mut MIDL_STUB_DESC, pformat: *mut u8) -> CLIENT_CALL_RETURN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrClientCall2(pstubdescriptor: *mut MIDL_STUB_DESC, pformat: *mut u8) -> CLIENT_CALL_RETURN;
        }
        ::core::mem::transmute(NdrClientCall2(::core::mem::transmute(pstubdescriptor), ::core::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrClientCall3(pproxyinfo: *mut MIDL_STUBLESS_PROXY_INFO, nprocnum: u32, preturnvalue: *mut ::core::ffi::c_void) -> CLIENT_CALL_RETURN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrClientCall3(pproxyinfo: *mut MIDL_STUBLESS_PROXY_INFO, nprocnum: u32, preturnvalue: *mut ::core::ffi::c_void) -> CLIENT_CALL_RETURN;
        }
        ::core::mem::transmute(NdrClientCall3(::core::mem::transmute(pproxyinfo), ::core::mem::transmute(nprocnum), ::core::mem::transmute(preturnvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrClientContextMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, contexthandle: isize, fcheck: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrClientContextMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, contexthandle: isize, fcheck: i32);
        }
        NdrClientContextMarshall(::core::mem::transmute(pstubmsg), ::core::mem::transmute(contexthandle), ::core::mem::transmute(fcheck))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrClientContextUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pcontexthandle: *mut isize, bindhandle: *mut ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrClientContextUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pcontexthandle: *mut isize, bindhandle: *mut ::core::ffi::c_void);
        }
        NdrClientContextUnmarshall(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pcontexthandle), ::core::mem::transmute(bindhandle))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrClientInitialize(prpcmsg: *mut RPC_MESSAGE, pstubmsg: *mut MIDL_STUB_MESSAGE, pstubdescriptor: *mut MIDL_STUB_DESC, procnum: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrClientInitialize(prpcmsg: *mut RPC_MESSAGE, pstubmsg: *mut MIDL_STUB_MESSAGE, pstubdescriptor: *mut MIDL_STUB_DESC, procnum: u32);
        }
        NdrClientInitialize(::core::mem::transmute(prpcmsg), ::core::mem::transmute(pstubmsg), ::core::mem::transmute(pstubdescriptor), ::core::mem::transmute(procnum))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrClientInitializeNew(prpcmsg: *mut RPC_MESSAGE, pstubmsg: *mut MIDL_STUB_MESSAGE, pstubdescriptor: *mut MIDL_STUB_DESC, procnum: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrClientInitializeNew(prpcmsg: *mut RPC_MESSAGE, pstubmsg: *mut MIDL_STUB_MESSAGE, pstubdescriptor: *mut MIDL_STUB_DESC, procnum: u32);
        }
        NdrClientInitializeNew(::core::mem::transmute(prpcmsg), ::core::mem::transmute(pstubmsg), ::core::mem::transmute(pstubdescriptor), ::core::mem::transmute(procnum))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrComplexArrayBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrComplexArrayBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
        }
        NdrComplexArrayBufferSize(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrComplexArrayFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrComplexArrayFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
        }
        NdrComplexArrayFree(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrComplexArrayMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrComplexArrayMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
        }
        ::core::mem::transmute(NdrComplexArrayMarshall(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrComplexArrayMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrComplexArrayMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32;
        }
        ::core::mem::transmute(NdrComplexArrayMemorySize(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrComplexArrayUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrComplexArrayUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
        }
        ::core::mem::transmute(NdrComplexArrayUnmarshall(::core::mem::transmute(pstubmsg), ::core::mem::transmute(ppmemory), ::core::mem::transmute(pformat), ::core::mem::transmute(fmustalloc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrComplexStructBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrComplexStructBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
        }
        NdrComplexStructBufferSize(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrComplexStructFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrComplexStructFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
        }
        NdrComplexStructFree(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrComplexStructMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrComplexStructMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
        }
        ::core::mem::transmute(NdrComplexStructMarshall(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrComplexStructMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrComplexStructMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32;
        }
        ::core::mem::transmute(NdrComplexStructMemorySize(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrComplexStructUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrComplexStructUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
        }
        ::core::mem::transmute(NdrComplexStructUnmarshall(::core::mem::transmute(pstubmsg), ::core::mem::transmute(ppmemory), ::core::mem::transmute(pformat), ::core::mem::transmute(fmustalloc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantArrayBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConformantArrayBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
        }
        NdrConformantArrayBufferSize(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantArrayFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConformantArrayFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
        }
        NdrConformantArrayFree(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantArrayMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConformantArrayMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
        }
        ::core::mem::transmute(NdrConformantArrayMarshall(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantArrayMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConformantArrayMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32;
        }
        ::core::mem::transmute(NdrConformantArrayMemorySize(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantArrayUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConformantArrayUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
        }
        ::core::mem::transmute(NdrConformantArrayUnmarshall(::core::mem::transmute(pstubmsg), ::core::mem::transmute(ppmemory), ::core::mem::transmute(pformat), ::core::mem::transmute(fmustalloc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantStringBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConformantStringBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
        }
        NdrConformantStringBufferSize(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantStringMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConformantStringMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
        }
        ::core::mem::transmute(NdrConformantStringMarshall(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantStringMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConformantStringMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32;
        }
        ::core::mem::transmute(NdrConformantStringMemorySize(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantStringUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConformantStringUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
        }
        ::core::mem::transmute(NdrConformantStringUnmarshall(::core::mem::transmute(pstubmsg), ::core::mem::transmute(ppmemory), ::core::mem::transmute(pformat), ::core::mem::transmute(fmustalloc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantStructBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConformantStructBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
        }
        NdrConformantStructBufferSize(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantStructFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConformantStructFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
        }
        NdrConformantStructFree(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantStructMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConformantStructMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
        }
        ::core::mem::transmute(NdrConformantStructMarshall(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantStructMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConformantStructMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32;
        }
        ::core::mem::transmute(NdrConformantStructMemorySize(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantStructUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConformantStructUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
        }
        ::core::mem::transmute(NdrConformantStructUnmarshall(::core::mem::transmute(pstubmsg), ::core::mem::transmute(ppmemory), ::core::mem::transmute(pformat), ::core::mem::transmute(fmustalloc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantVaryingArrayBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConformantVaryingArrayBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
        }
        NdrConformantVaryingArrayBufferSize(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantVaryingArrayFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConformantVaryingArrayFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
        }
        NdrConformantVaryingArrayFree(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantVaryingArrayMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConformantVaryingArrayMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
        }
        ::core::mem::transmute(NdrConformantVaryingArrayMarshall(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantVaryingArrayMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConformantVaryingArrayMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32;
        }
        ::core::mem::transmute(NdrConformantVaryingArrayMemorySize(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantVaryingArrayUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConformantVaryingArrayUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
        }
        ::core::mem::transmute(NdrConformantVaryingArrayUnmarshall(::core::mem::transmute(pstubmsg), ::core::mem::transmute(ppmemory), ::core::mem::transmute(pformat), ::core::mem::transmute(fmustalloc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantVaryingStructBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConformantVaryingStructBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
        }
        NdrConformantVaryingStructBufferSize(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantVaryingStructFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConformantVaryingStructFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
        }
        NdrConformantVaryingStructFree(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantVaryingStructMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConformantVaryingStructMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
        }
        ::core::mem::transmute(NdrConformantVaryingStructMarshall(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantVaryingStructMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConformantVaryingStructMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32;
        }
        ::core::mem::transmute(NdrConformantVaryingStructMemorySize(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantVaryingStructUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConformantVaryingStructUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
        }
        ::core::mem::transmute(NdrConformantVaryingStructUnmarshall(::core::mem::transmute(pstubmsg), ::core::mem::transmute(ppmemory), ::core::mem::transmute(pformat), ::core::mem::transmute(fmustalloc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrContextHandleInitialize(pstubmsg: *const MIDL_STUB_MESSAGE, pformat: *const u8) -> *mut NDR_SCONTEXT_1 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrContextHandleInitialize(pstubmsg: *const MIDL_STUB_MESSAGE, pformat: *const u8) -> *mut NDR_SCONTEXT_1;
        }
        ::core::mem::transmute(NdrContextHandleInitialize(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrContextHandleSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrContextHandleSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
        }
        NdrContextHandleSize(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConvert(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConvert(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8);
        }
        NdrConvert(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pformat))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConvert2(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8, numberparams: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConvert2(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8, numberparams: i32);
        }
        NdrConvert2(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pformat), ::core::mem::transmute(numberparams))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrCorrelationFree(pstubmsg: *mut MIDL_STUB_MESSAGE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrCorrelationFree(pstubmsg: *mut MIDL_STUB_MESSAGE);
        }
        NdrCorrelationFree(::core::mem::transmute(pstubmsg))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrCorrelationInitialize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut ::core::ffi::c_void, cachesize: u32, flags: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrCorrelationInitialize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut ::core::ffi::c_void, cachesize: u32, flags: u32);
        }
        NdrCorrelationInitialize(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(cachesize), ::core::mem::transmute(flags))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrCorrelationPass(pstubmsg: *mut MIDL_STUB_MESSAGE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrCorrelationPass(pstubmsg: *mut MIDL_STUB_MESSAGE);
        }
        NdrCorrelationPass(::core::mem::transmute(pstubmsg))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrCreateServerInterfaceFromStub<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IRpcStubBuffer>>(pstub: Param0, pserverif: *mut RPC_SERVER_INTERFACE) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrCreateServerInterfaceFromStub(pstub: ::windows::core::RawPtr, pserverif: *mut RPC_SERVER_INTERFACE) -> RPC_STATUS;
        }
        ::core::mem::transmute(NdrCreateServerInterfaceFromStub(pstub.into_param().abi(), ::core::mem::transmute(pserverif)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrDcomAsyncClientCall(pstubdescriptor: *mut MIDL_STUB_DESC, pformat: *mut u8) -> CLIENT_CALL_RETURN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrDcomAsyncClientCall(pstubdescriptor: *mut MIDL_STUB_DESC, pformat: *mut u8) -> CLIENT_CALL_RETURN;
        }
        ::core::mem::transmute(NdrDcomAsyncClientCall(::core::mem::transmute(pstubdescriptor), ::core::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrDcomAsyncStubCall<'a, Param0: ::windows::core::IntoParam<'a, super::Com::IRpcStubBuffer>, Param1: ::windows::core::IntoParam<'a, super::Com::IRpcChannelBuffer>>(pthis: Param0, pchannel: Param1, prpcmsg: *mut RPC_MESSAGE, pdwstubphase: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrDcomAsyncStubCall(pthis: ::windows::core::RawPtr, pchannel: ::windows::core::RawPtr, prpcmsg: *mut RPC_MESSAGE, pdwstubphase: *mut u32) -> i32;
        }
        ::core::mem::transmute(NdrDcomAsyncStubCall(pthis.into_param().abi(), pchannel.into_param().abi(), ::core::mem::transmute(prpcmsg), ::core::mem::transmute(pdwstubphase)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrEncapsulatedUnionBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrEncapsulatedUnionBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
        }
        NdrEncapsulatedUnionBufferSize(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrEncapsulatedUnionFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrEncapsulatedUnionFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
        }
        NdrEncapsulatedUnionFree(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrEncapsulatedUnionMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrEncapsulatedUnionMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
        }
        ::core::mem::transmute(NdrEncapsulatedUnionMarshall(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrEncapsulatedUnionMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrEncapsulatedUnionMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32;
        }
        ::core::mem::transmute(NdrEncapsulatedUnionMemorySize(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrEncapsulatedUnionUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrEncapsulatedUnionUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
        }
        ::core::mem::transmute(NdrEncapsulatedUnionUnmarshall(::core::mem::transmute(pstubmsg), ::core::mem::transmute(ppmemory), ::core::mem::transmute(pformat), ::core::mem::transmute(fmustalloc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrFixedArrayBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrFixedArrayBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
        }
        NdrFixedArrayBufferSize(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrFixedArrayFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrFixedArrayFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
        }
        NdrFixedArrayFree(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrFixedArrayMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrFixedArrayMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
        }
        ::core::mem::transmute(NdrFixedArrayMarshall(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrFixedArrayMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrFixedArrayMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32;
        }
        ::core::mem::transmute(NdrFixedArrayMemorySize(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrFixedArrayUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrFixedArrayUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
        }
        ::core::mem::transmute(NdrFixedArrayUnmarshall(::core::mem::transmute(pstubmsg), ::core::mem::transmute(ppmemory), ::core::mem::transmute(pformat), ::core::mem::transmute(fmustalloc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrFreeBuffer(pstubmsg: *mut MIDL_STUB_MESSAGE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrFreeBuffer(pstubmsg: *mut MIDL_STUB_MESSAGE);
        }
        NdrFreeBuffer(::core::mem::transmute(pstubmsg))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NdrFullPointerXlatFree(pxlattables: *mut FULL_PTR_XLAT_TABLES) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrFullPointerXlatFree(pxlattables: *mut FULL_PTR_XLAT_TABLES);
        }
        NdrFullPointerXlatFree(::core::mem::transmute(pxlattables))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NdrFullPointerXlatInit(numberofpointers: u32, xlatside: XLAT_SIDE) -> *mut FULL_PTR_XLAT_TABLES {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrFullPointerXlatInit(numberofpointers: u32, xlatside: XLAT_SIDE) -> *mut FULL_PTR_XLAT_TABLES;
        }
        ::core::mem::transmute(NdrFullPointerXlatInit(::core::mem::transmute(numberofpointers), ::core::mem::transmute(xlatside)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrGetBuffer(pstubmsg: *mut MIDL_STUB_MESSAGE, bufferlength: u32, handle: *mut ::core::ffi::c_void) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrGetBuffer(pstubmsg: *mut MIDL_STUB_MESSAGE, bufferlength: u32, handle: *mut ::core::ffi::c_void) -> *mut u8;
        }
        ::core::mem::transmute(NdrGetBuffer(::core::mem::transmute(pstubmsg), ::core::mem::transmute(bufferlength), ::core::mem::transmute(handle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrGetDcomProtocolVersion(pstubmsg: *mut MIDL_STUB_MESSAGE, pversion: *mut RPC_VERSION) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrGetDcomProtocolVersion(pstubmsg: *mut MIDL_STUB_MESSAGE, pversion: *mut RPC_VERSION) -> ::windows::core::HRESULT;
        }
        NdrGetDcomProtocolVersion(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pversion)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrGetUserMarshalInfo(pflags: *const u32, informationlevel: u32, pmarshalinfo: *mut NDR_USER_MARSHAL_INFO) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrGetUserMarshalInfo(pflags: *const u32, informationlevel: u32, pmarshalinfo: *mut NDR_USER_MARSHAL_INFO) -> RPC_STATUS;
        }
        ::core::mem::transmute(NdrGetUserMarshalInfo(::core::mem::transmute(pflags), ::core::mem::transmute(informationlevel), ::core::mem::transmute(pmarshalinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrInterfacePointerBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrInterfacePointerBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
        }
        NdrInterfacePointerBufferSize(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrInterfacePointerFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrInterfacePointerFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
        }
        NdrInterfacePointerFree(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrInterfacePointerMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrInterfacePointerMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
        }
        ::core::mem::transmute(NdrInterfacePointerMarshall(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrInterfacePointerMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrInterfacePointerMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32;
        }
        ::core::mem::transmute(NdrInterfacePointerMemorySize(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrInterfacePointerUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrInterfacePointerUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
        }
        ::core::mem::transmute(NdrInterfacePointerUnmarshall(::core::mem::transmute(pstubmsg), ::core::mem::transmute(ppmemory), ::core::mem::transmute(pformat), ::core::mem::transmute(fmustalloc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMapCommAndFaultStatus(pstubmsg: *mut MIDL_STUB_MESSAGE, pcommstatus: *mut u32, pfaultstatus: *mut u32, status: RPC_STATUS) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrMapCommAndFaultStatus(pstubmsg: *mut MIDL_STUB_MESSAGE, pcommstatus: *mut u32, pfaultstatus: *mut u32, status: RPC_STATUS) -> RPC_STATUS;
        }
        ::core::mem::transmute(NdrMapCommAndFaultStatus(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pcommstatus), ::core::mem::transmute(pfaultstatus), ::core::mem::transmute(status)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesProcEncodeDecode(handle: *mut ::core::ffi::c_void, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrMesProcEncodeDecode(handle: *mut ::core::ffi::c_void, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8);
        }
        NdrMesProcEncodeDecode(::core::mem::transmute(handle), ::core::mem::transmute(pstubdesc), ::core::mem::transmute(pformatstring))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesProcEncodeDecode2(handle: *mut ::core::ffi::c_void, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8) -> CLIENT_CALL_RETURN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrMesProcEncodeDecode2(handle: *mut ::core::ffi::c_void, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8) -> CLIENT_CALL_RETURN;
        }
        ::core::mem::transmute(NdrMesProcEncodeDecode2(::core::mem::transmute(handle), ::core::mem::transmute(pstubdesc), ::core::mem::transmute(pformatstring)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesProcEncodeDecode3(handle: *mut ::core::ffi::c_void, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO, nprocnum: u32, preturnvalue: *mut ::core::ffi::c_void) -> CLIENT_CALL_RETURN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrMesProcEncodeDecode3(handle: *mut ::core::ffi::c_void, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO, nprocnum: u32, preturnvalue: *mut ::core::ffi::c_void) -> CLIENT_CALL_RETURN;
        }
        ::core::mem::transmute(NdrMesProcEncodeDecode3(::core::mem::transmute(handle), ::core::mem::transmute(pproxyinfo), ::core::mem::transmute(nprocnum), ::core::mem::transmute(preturnvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NdrMesSimpleTypeAlignSize(param0: *mut ::core::ffi::c_void) -> usize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrMesSimpleTypeAlignSize(param0: *mut ::core::ffi::c_void) -> usize;
        }
        ::core::mem::transmute(NdrMesSimpleTypeAlignSize(::core::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesSimpleTypeAlignSizeAll(handle: *mut ::core::ffi::c_void, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO) -> usize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrMesSimpleTypeAlignSizeAll(handle: *mut ::core::ffi::c_void, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO) -> usize;
        }
        ::core::mem::transmute(NdrMesSimpleTypeAlignSizeAll(::core::mem::transmute(handle), ::core::mem::transmute(pproxyinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NdrMesSimpleTypeDecode(handle: *mut ::core::ffi::c_void, pobject: *mut ::core::ffi::c_void, size: i16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrMesSimpleTypeDecode(handle: *mut ::core::ffi::c_void, pobject: *mut ::core::ffi::c_void, size: i16);
        }
        NdrMesSimpleTypeDecode(::core::mem::transmute(handle), ::core::mem::transmute(pobject), ::core::mem::transmute(size))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesSimpleTypeDecodeAll(handle: *mut ::core::ffi::c_void, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO, pobject: *mut ::core::ffi::c_void, size: i16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrMesSimpleTypeDecodeAll(handle: *mut ::core::ffi::c_void, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO, pobject: *mut ::core::ffi::c_void, size: i16);
        }
        NdrMesSimpleTypeDecodeAll(::core::mem::transmute(handle), ::core::mem::transmute(pproxyinfo), ::core::mem::transmute(pobject), ::core::mem::transmute(size))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesSimpleTypeEncode(handle: *mut ::core::ffi::c_void, pstubdesc: *const MIDL_STUB_DESC, pobject: *const ::core::ffi::c_void, size: i16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrMesSimpleTypeEncode(handle: *mut ::core::ffi::c_void, pstubdesc: *const MIDL_STUB_DESC, pobject: *const ::core::ffi::c_void, size: i16);
        }
        NdrMesSimpleTypeEncode(::core::mem::transmute(handle), ::core::mem::transmute(pstubdesc), ::core::mem::transmute(pobject), ::core::mem::transmute(size))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesSimpleTypeEncodeAll(handle: *mut ::core::ffi::c_void, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO, pobject: *const ::core::ffi::c_void, size: i16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrMesSimpleTypeEncodeAll(handle: *mut ::core::ffi::c_void, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO, pobject: *const ::core::ffi::c_void, size: i16);
        }
        NdrMesSimpleTypeEncodeAll(::core::mem::transmute(handle), ::core::mem::transmute(pproxyinfo), ::core::mem::transmute(pobject), ::core::mem::transmute(size))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesTypeAlignSize(handle: *mut ::core::ffi::c_void, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8, pobject: *const ::core::ffi::c_void) -> usize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrMesTypeAlignSize(handle: *mut ::core::ffi::c_void, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8, pobject: *const ::core::ffi::c_void) -> usize;
        }
        ::core::mem::transmute(NdrMesTypeAlignSize(::core::mem::transmute(handle), ::core::mem::transmute(pstubdesc), ::core::mem::transmute(pformatstring), ::core::mem::transmute(pobject)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesTypeAlignSize2(handle: *mut ::core::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8, pobject: *const ::core::ffi::c_void) -> usize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrMesTypeAlignSize2(handle: *mut ::core::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8, pobject: *const ::core::ffi::c_void) -> usize;
        }
        ::core::mem::transmute(NdrMesTypeAlignSize2(::core::mem::transmute(handle), ::core::mem::transmute(ppicklinginfo), ::core::mem::transmute(pstubdesc), ::core::mem::transmute(pformatstring), ::core::mem::transmute(pobject)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesTypeAlignSize3(handle: *mut ::core::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO, arrtypeoffset: *const *const u32, ntypeindex: u32, pobject: *const ::core::ffi::c_void) -> usize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrMesTypeAlignSize3(handle: *mut ::core::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO, arrtypeoffset: *const *const u32, ntypeindex: u32, pobject: *const ::core::ffi::c_void) -> usize;
        }
        ::core::mem::transmute(NdrMesTypeAlignSize3(::core::mem::transmute(handle), ::core::mem::transmute(ppicklinginfo), ::core::mem::transmute(pproxyinfo), ::core::mem::transmute(arrtypeoffset), ::core::mem::transmute(ntypeindex), ::core::mem::transmute(pobject)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesTypeDecode(handle: *mut ::core::ffi::c_void, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8, pobject: *mut ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrMesTypeDecode(handle: *mut ::core::ffi::c_void, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8, pobject: *mut ::core::ffi::c_void);
        }
        NdrMesTypeDecode(::core::mem::transmute(handle), ::core::mem::transmute(pstubdesc), ::core::mem::transmute(pformatstring), ::core::mem::transmute(pobject))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesTypeDecode2(handle: *mut ::core::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8, pobject: *mut ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrMesTypeDecode2(handle: *mut ::core::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8, pobject: *mut ::core::ffi::c_void);
        }
        NdrMesTypeDecode2(::core::mem::transmute(handle), ::core::mem::transmute(ppicklinginfo), ::core::mem::transmute(pstubdesc), ::core::mem::transmute(pformatstring), ::core::mem::transmute(pobject))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesTypeDecode3(handle: *mut ::core::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO, arrtypeoffset: *const *const u32, ntypeindex: u32, pobject: *mut ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrMesTypeDecode3(handle: *mut ::core::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO, arrtypeoffset: *const *const u32, ntypeindex: u32, pobject: *mut ::core::ffi::c_void);
        }
        NdrMesTypeDecode3(::core::mem::transmute(handle), ::core::mem::transmute(ppicklinginfo), ::core::mem::transmute(pproxyinfo), ::core::mem::transmute(arrtypeoffset), ::core::mem::transmute(ntypeindex), ::core::mem::transmute(pobject))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesTypeEncode(handle: *mut ::core::ffi::c_void, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8, pobject: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrMesTypeEncode(handle: *mut ::core::ffi::c_void, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8, pobject: *const ::core::ffi::c_void);
        }
        NdrMesTypeEncode(::core::mem::transmute(handle), ::core::mem::transmute(pstubdesc), ::core::mem::transmute(pformatstring), ::core::mem::transmute(pobject))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesTypeEncode2(handle: *mut ::core::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8, pobject: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrMesTypeEncode2(handle: *mut ::core::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8, pobject: *const ::core::ffi::c_void);
        }
        NdrMesTypeEncode2(::core::mem::transmute(handle), ::core::mem::transmute(ppicklinginfo), ::core::mem::transmute(pstubdesc), ::core::mem::transmute(pformatstring), ::core::mem::transmute(pobject))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesTypeEncode3(handle: *mut ::core::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO, arrtypeoffset: *const *const u32, ntypeindex: u32, pobject: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrMesTypeEncode3(handle: *mut ::core::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO, arrtypeoffset: *const *const u32, ntypeindex: u32, pobject: *const ::core::ffi::c_void);
        }
        NdrMesTypeEncode3(::core::mem::transmute(handle), ::core::mem::transmute(ppicklinginfo), ::core::mem::transmute(pproxyinfo), ::core::mem::transmute(arrtypeoffset), ::core::mem::transmute(ntypeindex), ::core::mem::transmute(pobject))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesTypeFree2(handle: *mut ::core::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8, pobject: *mut ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrMesTypeFree2(handle: *mut ::core::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8, pobject: *mut ::core::ffi::c_void);
        }
        NdrMesTypeFree2(::core::mem::transmute(handle), ::core::mem::transmute(ppicklinginfo), ::core::mem::transmute(pstubdesc), ::core::mem::transmute(pformatstring), ::core::mem::transmute(pobject))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesTypeFree3(handle: *mut ::core::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO, arrtypeoffset: *const *const u32, ntypeindex: u32, pobject: *mut ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrMesTypeFree3(handle: *mut ::core::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO, arrtypeoffset: *const *const u32, ntypeindex: u32, pobject: *mut ::core::ffi::c_void);
        }
        NdrMesTypeFree3(::core::mem::transmute(handle), ::core::mem::transmute(ppicklinginfo), ::core::mem::transmute(pproxyinfo), ::core::mem::transmute(arrtypeoffset), ::core::mem::transmute(ntypeindex), ::core::mem::transmute(pobject))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrNonConformantStringBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrNonConformantStringBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
        }
        NdrNonConformantStringBufferSize(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrNonConformantStringMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrNonConformantStringMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
        }
        ::core::mem::transmute(NdrNonConformantStringMarshall(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrNonConformantStringMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrNonConformantStringMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32;
        }
        ::core::mem::transmute(NdrNonConformantStringMemorySize(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrNonConformantStringUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrNonConformantStringUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
        }
        ::core::mem::transmute(NdrNonConformantStringUnmarshall(::core::mem::transmute(pstubmsg), ::core::mem::transmute(ppmemory), ::core::mem::transmute(pformat), ::core::mem::transmute(fmustalloc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrNonEncapsulatedUnionBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrNonEncapsulatedUnionBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
        }
        NdrNonEncapsulatedUnionBufferSize(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrNonEncapsulatedUnionFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrNonEncapsulatedUnionFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
        }
        NdrNonEncapsulatedUnionFree(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrNonEncapsulatedUnionMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrNonEncapsulatedUnionMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
        }
        ::core::mem::transmute(NdrNonEncapsulatedUnionMarshall(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrNonEncapsulatedUnionMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrNonEncapsulatedUnionMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32;
        }
        ::core::mem::transmute(NdrNonEncapsulatedUnionMemorySize(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrNonEncapsulatedUnionUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrNonEncapsulatedUnionUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
        }
        ::core::mem::transmute(NdrNonEncapsulatedUnionUnmarshall(::core::mem::transmute(pstubmsg), ::core::mem::transmute(ppmemory), ::core::mem::transmute(pformat), ::core::mem::transmute(fmustalloc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrNsGetBuffer(pstubmsg: *mut MIDL_STUB_MESSAGE, bufferlength: u32, handle: *mut ::core::ffi::c_void) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrNsGetBuffer(pstubmsg: *mut MIDL_STUB_MESSAGE, bufferlength: u32, handle: *mut ::core::ffi::c_void) -> *mut u8;
        }
        ::core::mem::transmute(NdrNsGetBuffer(::core::mem::transmute(pstubmsg), ::core::mem::transmute(bufferlength), ::core::mem::transmute(handle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrNsSendReceive(pstubmsg: *mut MIDL_STUB_MESSAGE, pbufferend: *mut u8, pautohandle: *mut *mut ::core::ffi::c_void) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrNsSendReceive(pstubmsg: *mut MIDL_STUB_MESSAGE, pbufferend: *mut u8, pautohandle: *mut *mut ::core::ffi::c_void) -> *mut u8;
        }
        ::core::mem::transmute(NdrNsSendReceive(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pbufferend), ::core::mem::transmute(pautohandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NdrOleAllocate(size: usize) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrOleAllocate(size: usize) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(NdrOleAllocate(::core::mem::transmute(size)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NdrOleFree(nodetofree: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrOleFree(nodetofree: *const ::core::ffi::c_void);
        }
        NdrOleFree(::core::mem::transmute(nodetofree))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrPartialIgnoreClientBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrPartialIgnoreClientBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut ::core::ffi::c_void);
        }
        NdrPartialIgnoreClientBufferSize(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrPartialIgnoreClientMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrPartialIgnoreClientMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut ::core::ffi::c_void);
        }
        NdrPartialIgnoreClientMarshall(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrPartialIgnoreServerInitialize(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut ::core::ffi::c_void, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrPartialIgnoreServerInitialize(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut ::core::ffi::c_void, pformat: *mut u8);
        }
        NdrPartialIgnoreServerInitialize(::core::mem::transmute(pstubmsg), ::core::mem::transmute(ppmemory), ::core::mem::transmute(pformat))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrPartialIgnoreServerUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrPartialIgnoreServerUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut ::core::ffi::c_void);
        }
        NdrPartialIgnoreServerUnmarshall(::core::mem::transmute(pstubmsg), ::core::mem::transmute(ppmemory))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrPointerBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrPointerBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
        }
        NdrPointerBufferSize(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrPointerFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrPointerFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
        }
        NdrPointerFree(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrPointerMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrPointerMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
        }
        ::core::mem::transmute(NdrPointerMarshall(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrPointerMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrPointerMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32;
        }
        ::core::mem::transmute(NdrPointerMemorySize(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrPointerUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrPointerUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
        }
        ::core::mem::transmute(NdrPointerUnmarshall(::core::mem::transmute(pstubmsg), ::core::mem::transmute(ppmemory), ::core::mem::transmute(pformat), ::core::mem::transmute(fmustalloc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrRangeUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrRangeUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
        }
        ::core::mem::transmute(NdrRangeUnmarshall(::core::mem::transmute(pstubmsg), ::core::mem::transmute(ppmemory), ::core::mem::transmute(pformat), ::core::mem::transmute(fmustalloc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NdrRpcSmClientAllocate(size: usize) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrRpcSmClientAllocate(size: usize) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(NdrRpcSmClientAllocate(::core::mem::transmute(size)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NdrRpcSmClientFree(nodetofree: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrRpcSmClientFree(nodetofree: *const ::core::ffi::c_void);
        }
        NdrRpcSmClientFree(::core::mem::transmute(nodetofree))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrRpcSmSetClientToOsf(pmessage: *mut MIDL_STUB_MESSAGE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrRpcSmSetClientToOsf(pmessage: *mut MIDL_STUB_MESSAGE);
        }
        NdrRpcSmSetClientToOsf(::core::mem::transmute(pmessage))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NdrRpcSsDefaultAllocate(size: usize) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrRpcSsDefaultAllocate(size: usize) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(NdrRpcSsDefaultAllocate(::core::mem::transmute(size)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NdrRpcSsDefaultFree(nodetofree: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrRpcSsDefaultFree(nodetofree: *const ::core::ffi::c_void);
        }
        NdrRpcSsDefaultFree(::core::mem::transmute(nodetofree))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrRpcSsDisableAllocate(pmessage: *mut MIDL_STUB_MESSAGE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrRpcSsDisableAllocate(pmessage: *mut MIDL_STUB_MESSAGE);
        }
        NdrRpcSsDisableAllocate(::core::mem::transmute(pmessage))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrRpcSsEnableAllocate(pmessage: *mut MIDL_STUB_MESSAGE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrRpcSsEnableAllocate(pmessage: *mut MIDL_STUB_MESSAGE);
        }
        NdrRpcSsEnableAllocate(::core::mem::transmute(pmessage))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrSendReceive(pstubmsg: *mut MIDL_STUB_MESSAGE, pbufferend: *mut u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrSendReceive(pstubmsg: *mut MIDL_STUB_MESSAGE, pbufferend: *mut u8) -> *mut u8;
        }
        ::core::mem::transmute(NdrSendReceive(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pbufferend)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NdrServerCall2(prpcmsg: *mut RPC_MESSAGE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrServerCall2(prpcmsg: *mut RPC_MESSAGE);
        }
        NdrServerCall2(::core::mem::transmute(prpcmsg))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NdrServerCallAll(prpcmsg: *mut RPC_MESSAGE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrServerCallAll(prpcmsg: *mut RPC_MESSAGE);
        }
        NdrServerCallAll(::core::mem::transmute(prpcmsg))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NdrServerCallNdr64(prpcmsg: *mut RPC_MESSAGE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrServerCallNdr64(prpcmsg: *mut RPC_MESSAGE);
        }
        NdrServerCallNdr64(::core::mem::transmute(prpcmsg))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrServerContextMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, contexthandle: *mut NDR_SCONTEXT_1, rundownroutine: NDR_RUNDOWN) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrServerContextMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, contexthandle: *mut NDR_SCONTEXT_1, rundownroutine: ::windows::core::RawPtr);
        }
        NdrServerContextMarshall(::core::mem::transmute(pstubmsg), ::core::mem::transmute(contexthandle), ::core::mem::transmute(rundownroutine))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrServerContextNewMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, contexthandle: *mut NDR_SCONTEXT_1, rundownroutine: NDR_RUNDOWN, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrServerContextNewMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, contexthandle: *mut NDR_SCONTEXT_1, rundownroutine: ::windows::core::RawPtr, pformat: *mut u8);
        }
        NdrServerContextNewMarshall(::core::mem::transmute(pstubmsg), ::core::mem::transmute(contexthandle), ::core::mem::transmute(rundownroutine), ::core::mem::transmute(pformat))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrServerContextNewUnmarshall(pstubmsg: *const MIDL_STUB_MESSAGE, pformat: *const u8) -> *mut NDR_SCONTEXT_1 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrServerContextNewUnmarshall(pstubmsg: *const MIDL_STUB_MESSAGE, pformat: *const u8) -> *mut NDR_SCONTEXT_1;
        }
        ::core::mem::transmute(NdrServerContextNewUnmarshall(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrServerContextUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE) -> *mut NDR_SCONTEXT_1 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrServerContextUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE) -> *mut NDR_SCONTEXT_1;
        }
        ::core::mem::transmute(NdrServerContextUnmarshall(::core::mem::transmute(pstubmsg)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrServerInitialize(prpcmsg: *mut RPC_MESSAGE, pstubmsg: *mut MIDL_STUB_MESSAGE, pstubdescriptor: *mut MIDL_STUB_DESC) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrServerInitialize(prpcmsg: *mut RPC_MESSAGE, pstubmsg: *mut MIDL_STUB_MESSAGE, pstubdescriptor: *mut MIDL_STUB_DESC) -> *mut u8;
        }
        ::core::mem::transmute(NdrServerInitialize(::core::mem::transmute(prpcmsg), ::core::mem::transmute(pstubmsg), ::core::mem::transmute(pstubdescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrServerInitializeMarshall(prpcmsg: *mut RPC_MESSAGE, pstubmsg: *mut MIDL_STUB_MESSAGE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrServerInitializeMarshall(prpcmsg: *mut RPC_MESSAGE, pstubmsg: *mut MIDL_STUB_MESSAGE);
        }
        NdrServerInitializeMarshall(::core::mem::transmute(prpcmsg), ::core::mem::transmute(pstubmsg))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrServerInitializeNew(prpcmsg: *mut RPC_MESSAGE, pstubmsg: *mut MIDL_STUB_MESSAGE, pstubdescriptor: *mut MIDL_STUB_DESC) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrServerInitializeNew(prpcmsg: *mut RPC_MESSAGE, pstubmsg: *mut MIDL_STUB_MESSAGE, pstubdescriptor: *mut MIDL_STUB_DESC) -> *mut u8;
        }
        ::core::mem::transmute(NdrServerInitializeNew(::core::mem::transmute(prpcmsg), ::core::mem::transmute(pstubmsg), ::core::mem::transmute(pstubdescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrServerInitializePartial(prpcmsg: *mut RPC_MESSAGE, pstubmsg: *mut MIDL_STUB_MESSAGE, pstubdescriptor: *mut MIDL_STUB_DESC, requestedbuffersize: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrServerInitializePartial(prpcmsg: *mut RPC_MESSAGE, pstubmsg: *mut MIDL_STUB_MESSAGE, pstubdescriptor: *mut MIDL_STUB_DESC, requestedbuffersize: u32);
        }
        NdrServerInitializePartial(::core::mem::transmute(prpcmsg), ::core::mem::transmute(pstubmsg), ::core::mem::transmute(pstubdescriptor), ::core::mem::transmute(requestedbuffersize))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrServerInitializeUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pstubdescriptor: *mut MIDL_STUB_DESC, prpcmsg: *mut RPC_MESSAGE) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrServerInitializeUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pstubdescriptor: *mut MIDL_STUB_DESC, prpcmsg: *mut RPC_MESSAGE) -> *mut u8;
        }
        ::core::mem::transmute(NdrServerInitializeUnmarshall(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pstubdescriptor), ::core::mem::transmute(prpcmsg)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrSimpleStructBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrSimpleStructBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
        }
        NdrSimpleStructBufferSize(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrSimpleStructFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrSimpleStructFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
        }
        NdrSimpleStructFree(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrSimpleStructMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrSimpleStructMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
        }
        ::core::mem::transmute(NdrSimpleStructMarshall(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrSimpleStructMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrSimpleStructMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32;
        }
        ::core::mem::transmute(NdrSimpleStructMemorySize(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrSimpleStructUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrSimpleStructUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
        }
        ::core::mem::transmute(NdrSimpleStructUnmarshall(::core::mem::transmute(pstubmsg), ::core::mem::transmute(ppmemory), ::core::mem::transmute(pformat), ::core::mem::transmute(fmustalloc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrSimpleTypeMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, formatchar: u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrSimpleTypeMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, formatchar: u8);
        }
        NdrSimpleTypeMarshall(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(formatchar))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrSimpleTypeUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, formatchar: u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrSimpleTypeUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, formatchar: u8);
        }
        NdrSimpleTypeUnmarshall(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(formatchar))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NdrStubCall2(pthis: *mut ::core::ffi::c_void, pchannel: *mut ::core::ffi::c_void, prpcmsg: *mut RPC_MESSAGE, pdwstubphase: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrStubCall2(pthis: *mut ::core::ffi::c_void, pchannel: *mut ::core::ffi::c_void, prpcmsg: *mut RPC_MESSAGE, pdwstubphase: *mut u32) -> i32;
        }
        ::core::mem::transmute(NdrStubCall2(::core::mem::transmute(pthis), ::core::mem::transmute(pchannel), ::core::mem::transmute(prpcmsg), ::core::mem::transmute(pdwstubphase)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NdrStubCall3(pthis: *mut ::core::ffi::c_void, pchannel: *mut ::core::ffi::c_void, prpcmsg: *mut RPC_MESSAGE, pdwstubphase: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrStubCall3(pthis: *mut ::core::ffi::c_void, pchannel: *mut ::core::ffi::c_void, prpcmsg: *mut RPC_MESSAGE, pdwstubphase: *mut u32) -> i32;
        }
        ::core::mem::transmute(NdrStubCall3(::core::mem::transmute(pthis), ::core::mem::transmute(pchannel), ::core::mem::transmute(prpcmsg), ::core::mem::transmute(pdwstubphase)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrUserMarshalBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrUserMarshalBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
        }
        NdrUserMarshalBufferSize(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrUserMarshalFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrUserMarshalFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
        }
        NdrUserMarshalFree(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrUserMarshalMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrUserMarshalMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
        }
        ::core::mem::transmute(NdrUserMarshalMarshall(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrUserMarshalMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrUserMarshalMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32;
        }
        ::core::mem::transmute(NdrUserMarshalMemorySize(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn NdrUserMarshalSimpleTypeConvert(pflags: *mut u32, pbuffer: *mut u8, formatchar: u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrUserMarshalSimpleTypeConvert(pflags: *mut u32, pbuffer: *mut u8, formatchar: u8) -> *mut u8;
        }
        ::core::mem::transmute(NdrUserMarshalSimpleTypeConvert(::core::mem::transmute(pflags), ::core::mem::transmute(pbuffer), ::core::mem::transmute(formatchar)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrUserMarshalUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrUserMarshalUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
        }
        ::core::mem::transmute(NdrUserMarshalUnmarshall(::core::mem::transmute(pstubmsg), ::core::mem::transmute(ppmemory), ::core::mem::transmute(pformat), ::core::mem::transmute(fmustalloc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrVaryingArrayBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrVaryingArrayBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
        }
        NdrVaryingArrayBufferSize(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrVaryingArrayFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrVaryingArrayFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
        }
        NdrVaryingArrayFree(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrVaryingArrayMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrVaryingArrayMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
        }
        ::core::mem::transmute(NdrVaryingArrayMarshall(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrVaryingArrayMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrVaryingArrayMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32;
        }
        ::core::mem::transmute(NdrVaryingArrayMemorySize(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrVaryingArrayUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrVaryingArrayUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
        }
        ::core::mem::transmute(NdrVaryingArrayUnmarshall(::core::mem::transmute(pstubmsg), ::core::mem::transmute(ppmemory), ::core::mem::transmute(pformat), ::core::mem::transmute(fmustalloc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrXmitOrRepAsBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrXmitOrRepAsBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
        }
        NdrXmitOrRepAsBufferSize(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrXmitOrRepAsFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrXmitOrRepAsFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8);
        }
        NdrXmitOrRepAsFree(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrXmitOrRepAsMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrXmitOrRepAsMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
        }
        ::core::mem::transmute(NdrXmitOrRepAsMarshall(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pmemory), ::core::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrXmitOrRepAsMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrXmitOrRepAsMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32;
        }
        ::core::mem::transmute(NdrXmitOrRepAsMemorySize(::core::mem::transmute(pstubmsg), ::core::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrXmitOrRepAsUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrXmitOrRepAsUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
        }
        ::core::mem::transmute(NdrXmitOrRepAsUnmarshall(::core::mem::transmute(pstubmsg), ::core::mem::transmute(ppmemory), ::core::mem::transmute(pformat), ::core::mem::transmute(fmustalloc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PFN_RPCNOTIFICATION_ROUTINE = ::core::option::Option<unsafe extern "system" fn(pasync: *mut RPC_ASYNC_STATE, context: *mut ::core::ffi::c_void, event: RPC_ASYNC_EVENT)>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct PROXY_PHASE(pub i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const PROXY_CALCSIZE: PROXY_PHASE = PROXY_PHASE(0i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const PROXY_GETBUFFER: PROXY_PHASE = PROXY_PHASE(1i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const PROXY_MARSHAL: PROXY_PHASE = PROXY_PHASE(2i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const PROXY_SENDRECEIVE: PROXY_PHASE = PROXY_PHASE(3i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const PROXY_UNMARSHAL: PROXY_PHASE = PROXY_PHASE(4i32);
impl ::core::marker::Copy for PROXY_PHASE {}
impl ::core::clone::Clone for PROXY_PHASE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for PROXY_PHASE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for PROXY_PHASE {
    type Abi = Self;
}
impl ::core::fmt::Debug for PROXY_PHASE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("PROXY_PHASE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type PRPC_RUNDOWN = ::core::option::Option<unsafe extern "system" fn(associationcontext: *mut ::core::ffi::c_void)>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RDR_CALLOUT_STATE {
    pub LastError: RPC_STATUS,
    pub LastEEInfo: *mut ::core::ffi::c_void,
    pub LastCalledStage: RPC_HTTP_REDIRECTOR_STAGE,
    pub ServerName: *mut u16,
    pub ServerPort: *mut u16,
    pub RemoteUser: *mut u16,
    pub AuthType: *mut u16,
    pub ResourceTypePresent: u8,
    pub SessionIdPresent: u8,
    pub InterfacePresent: u8,
    pub ResourceType: ::windows::core::GUID,
    pub SessionId: ::windows::core::GUID,
    pub Interface: RPC_SYNTAX_IDENTIFIER,
    pub CertContext: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for RDR_CALLOUT_STATE {}
impl ::core::clone::Clone for RDR_CALLOUT_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RDR_CALLOUT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RDR_CALLOUT_STATE")
            .field("LastError", &self.LastError)
            .field("LastEEInfo", &self.LastEEInfo)
            .field("LastCalledStage", &self.LastCalledStage)
            .field("ServerName", &self.ServerName)
            .field("ServerPort", &self.ServerPort)
            .field("RemoteUser", &self.RemoteUser)
            .field("AuthType", &self.AuthType)
            .field("ResourceTypePresent", &self.ResourceTypePresent)
            .field("SessionIdPresent", &self.SessionIdPresent)
            .field("InterfacePresent", &self.InterfacePresent)
            .field("ResourceType", &self.ResourceType)
            .field("SessionId", &self.SessionId)
            .field("Interface", &self.Interface)
            .field("CertContext", &self.CertContext)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for RDR_CALLOUT_STATE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RDR_CALLOUT_STATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RDR_CALLOUT_STATE>()) == 0 }
    }
}
impl ::core::cmp::Eq for RDR_CALLOUT_STATE {}
impl ::core::default::Default for RDR_CALLOUT_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPCFLG_ACCESSIBILITY_BIT1: u32 = 1048576u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPCFLG_ACCESSIBILITY_BIT2: u32 = 2097152u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPCFLG_ACCESS_LOCAL: u32 = 4194304u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPCFLG_ASYNCHRONOUS: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPCFLG_AUTO_COMPLETE: u32 = 134217728u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPCFLG_HAS_CALLBACK: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPCFLG_HAS_GUARANTEE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPCFLG_HAS_MULTI_SYNTAXES: u32 = 33554432u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPCFLG_INPUT_SYNCHRONOUS: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPCFLG_LOCAL_CALL: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPCFLG_MESSAGE: u32 = 16777216u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPCFLG_NDR64_CONTAINS_ARM_LAYOUT: u32 = 67108864u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPCFLG_NON_NDR: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPCFLG_SENDER_WAITING_FOR_REPLY: u32 = 8388608u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPCFLG_WINRT_REMOTE_ASYNC: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type RPCLT_PDU_FILTER_FUNC = ::core::option::Option<unsafe extern "system" fn(buffer: *mut ::core::ffi::c_void, bufferlength: u32, fdatagram: i32)>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type RPC_ADDRESS_CHANGE_FN = ::core::option::Option<unsafe extern "system" fn(arg: *mut ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RPC_ADDRESS_CHANGE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const PROTOCOL_NOT_LOADED: RPC_ADDRESS_CHANGE_TYPE = RPC_ADDRESS_CHANGE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const PROTOCOL_LOADED: RPC_ADDRESS_CHANGE_TYPE = RPC_ADDRESS_CHANGE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const PROTOCOL_ADDRESS_CHANGE: RPC_ADDRESS_CHANGE_TYPE = RPC_ADDRESS_CHANGE_TYPE(3i32);
impl ::core::marker::Copy for RPC_ADDRESS_CHANGE_TYPE {}
impl ::core::clone::Clone for RPC_ADDRESS_CHANGE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RPC_ADDRESS_CHANGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RPC_ADDRESS_CHANGE_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RPC_ADDRESS_CHANGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPC_ADDRESS_CHANGE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RPC_ASYNC_EVENT(pub i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcCallComplete: RPC_ASYNC_EVENT = RPC_ASYNC_EVENT(0i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcSendComplete: RPC_ASYNC_EVENT = RPC_ASYNC_EVENT(1i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcReceiveComplete: RPC_ASYNC_EVENT = RPC_ASYNC_EVENT(2i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcClientDisconnect: RPC_ASYNC_EVENT = RPC_ASYNC_EVENT(3i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcClientCancel: RPC_ASYNC_EVENT = RPC_ASYNC_EVENT(4i32);
impl ::core::marker::Copy for RPC_ASYNC_EVENT {}
impl ::core::clone::Clone for RPC_ASYNC_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RPC_ASYNC_EVENT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RPC_ASYNC_EVENT {
    type Abi = Self;
}
impl ::core::fmt::Debug for RPC_ASYNC_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPC_ASYNC_EVENT").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub union RPC_ASYNC_NOTIFICATION_INFO {
    pub APC: RPC_ASYNC_NOTIFICATION_INFO_0,
    pub IOC: RPC_ASYNC_NOTIFICATION_INFO_1,
    pub IntPtr: RPC_ASYNC_NOTIFICATION_INFO_2,
    pub hEvent: super::super::Foundation::HANDLE,
    pub NotificationRoutine: PFN_RPCNOTIFICATION_ROUTINE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::marker::Copy for RPC_ASYNC_NOTIFICATION_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::clone::Clone for RPC_ASYNC_NOTIFICATION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
unsafe impl ::windows::core::Abi for RPC_ASYNC_NOTIFICATION_INFO {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::cmp::PartialEq for RPC_ASYNC_NOTIFICATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_ASYNC_NOTIFICATION_INFO>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::cmp::Eq for RPC_ASYNC_NOTIFICATION_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::default::Default for RPC_ASYNC_NOTIFICATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub struct RPC_ASYNC_NOTIFICATION_INFO_0 {
    pub NotificationRoutine: PFN_RPCNOTIFICATION_ROUTINE,
    pub hThread: super::super::Foundation::HANDLE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::marker::Copy for RPC_ASYNC_NOTIFICATION_INFO_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::clone::Clone for RPC_ASYNC_NOTIFICATION_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::fmt::Debug for RPC_ASYNC_NOTIFICATION_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_ASYNC_NOTIFICATION_INFO_0").field("NotificationRoutine", &self.NotificationRoutine.map(|f| f as usize)).field("hThread", &self.hThread).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
unsafe impl ::windows::core::Abi for RPC_ASYNC_NOTIFICATION_INFO_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::cmp::PartialEq for RPC_ASYNC_NOTIFICATION_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_ASYNC_NOTIFICATION_INFO_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::cmp::Eq for RPC_ASYNC_NOTIFICATION_INFO_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::default::Default for RPC_ASYNC_NOTIFICATION_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub struct RPC_ASYNC_NOTIFICATION_INFO_1 {
    pub hIOPort: super::super::Foundation::HANDLE,
    pub dwNumberOfBytesTransferred: u32,
    pub dwCompletionKey: usize,
    pub lpOverlapped: *mut super::IO::OVERLAPPED,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::marker::Copy for RPC_ASYNC_NOTIFICATION_INFO_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::clone::Clone for RPC_ASYNC_NOTIFICATION_INFO_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::fmt::Debug for RPC_ASYNC_NOTIFICATION_INFO_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_ASYNC_NOTIFICATION_INFO_1").field("hIOPort", &self.hIOPort).field("dwNumberOfBytesTransferred", &self.dwNumberOfBytesTransferred).field("dwCompletionKey", &self.dwCompletionKey).field("lpOverlapped", &self.lpOverlapped).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
unsafe impl ::windows::core::Abi for RPC_ASYNC_NOTIFICATION_INFO_1 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::cmp::PartialEq for RPC_ASYNC_NOTIFICATION_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_ASYNC_NOTIFICATION_INFO_1>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::cmp::Eq for RPC_ASYNC_NOTIFICATION_INFO_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::default::Default for RPC_ASYNC_NOTIFICATION_INFO_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub struct RPC_ASYNC_NOTIFICATION_INFO_2 {
    pub hWnd: super::super::Foundation::HWND,
    pub Msg: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::marker::Copy for RPC_ASYNC_NOTIFICATION_INFO_2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::clone::Clone for RPC_ASYNC_NOTIFICATION_INFO_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::fmt::Debug for RPC_ASYNC_NOTIFICATION_INFO_2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_ASYNC_NOTIFICATION_INFO_2").field("hWnd", &self.hWnd).field("Msg", &self.Msg).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
unsafe impl ::windows::core::Abi for RPC_ASYNC_NOTIFICATION_INFO_2 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::cmp::PartialEq for RPC_ASYNC_NOTIFICATION_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_ASYNC_NOTIFICATION_INFO_2>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::cmp::Eq for RPC_ASYNC_NOTIFICATION_INFO_2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::default::Default for RPC_ASYNC_NOTIFICATION_INFO_2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub struct RPC_ASYNC_STATE {
    pub Size: u32,
    pub Signature: u32,
    pub Lock: i32,
    pub Flags: u32,
    pub StubInfo: *mut ::core::ffi::c_void,
    pub UserInfo: *mut ::core::ffi::c_void,
    pub RuntimeInfo: *mut ::core::ffi::c_void,
    pub Event: RPC_ASYNC_EVENT,
    pub NotificationType: RPC_NOTIFICATION_TYPES,
    pub u: RPC_ASYNC_NOTIFICATION_INFO,
    pub Reserved: [isize; 4],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::marker::Copy for RPC_ASYNC_STATE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::clone::Clone for RPC_ASYNC_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
unsafe impl ::windows::core::Abi for RPC_ASYNC_STATE {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::cmp::PartialEq for RPC_ASYNC_STATE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_ASYNC_STATE>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::cmp::Eq for RPC_ASYNC_STATE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::core::default::Default for RPC_ASYNC_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type RPC_AUTH_KEY_RETRIEVAL_FN = ::core::option::Option<unsafe extern "system" fn(arg: *const ::core::ffi::c_void, serverprincname: *const u16, keyver: u32, key: *mut *mut ::core::ffi::c_void, status: *mut RPC_STATUS)>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_BHO_EXCLUSIVE_AND_GUARANTEED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_BHT_OBJECT_UUID_VALID: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RPC_BINDING_HANDLE_OPTIONS_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_BHO_NONCAUSAL: RPC_BINDING_HANDLE_OPTIONS_FLAGS = RPC_BINDING_HANDLE_OPTIONS_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_BHO_DONTLINGER: RPC_BINDING_HANDLE_OPTIONS_FLAGS = RPC_BINDING_HANDLE_OPTIONS_FLAGS(2u32);
impl ::core::marker::Copy for RPC_BINDING_HANDLE_OPTIONS_FLAGS {}
impl ::core::clone::Clone for RPC_BINDING_HANDLE_OPTIONS_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RPC_BINDING_HANDLE_OPTIONS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RPC_BINDING_HANDLE_OPTIONS_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for RPC_BINDING_HANDLE_OPTIONS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPC_BINDING_HANDLE_OPTIONS_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for RPC_BINDING_HANDLE_OPTIONS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for RPC_BINDING_HANDLE_OPTIONS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for RPC_BINDING_HANDLE_OPTIONS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for RPC_BINDING_HANDLE_OPTIONS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for RPC_BINDING_HANDLE_OPTIONS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_BINDING_HANDLE_OPTIONS_V1 {
    pub Version: u32,
    pub Flags: RPC_BINDING_HANDLE_OPTIONS_FLAGS,
    pub ComTimeout: u32,
    pub CallTimeout: u32,
}
impl ::core::marker::Copy for RPC_BINDING_HANDLE_OPTIONS_V1 {}
impl ::core::clone::Clone for RPC_BINDING_HANDLE_OPTIONS_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_BINDING_HANDLE_OPTIONS_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_BINDING_HANDLE_OPTIONS_V1").field("Version", &self.Version).field("Flags", &self.Flags).field("ComTimeout", &self.ComTimeout).field("CallTimeout", &self.CallTimeout).finish()
    }
}
unsafe impl ::windows::core::Abi for RPC_BINDING_HANDLE_OPTIONS_V1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RPC_BINDING_HANDLE_OPTIONS_V1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_BINDING_HANDLE_OPTIONS_V1>()) == 0 }
    }
}
impl ::core::cmp::Eq for RPC_BINDING_HANDLE_OPTIONS_V1 {}
impl ::core::default::Default for RPC_BINDING_HANDLE_OPTIONS_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct RPC_BINDING_HANDLE_SECURITY_V1_A {
    pub Version: u32,
    pub ServerPrincName: *mut u8,
    pub AuthnLevel: u32,
    pub AuthnSvc: u32,
    pub AuthIdentity: *mut SEC_WINNT_AUTH_IDENTITY_A,
    pub SecurityQos: *mut RPC_SECURITY_QOS,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for RPC_BINDING_HANDLE_SECURITY_V1_A {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for RPC_BINDING_HANDLE_SECURITY_V1_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for RPC_BINDING_HANDLE_SECURITY_V1_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_BINDING_HANDLE_SECURITY_V1_A").field("Version", &self.Version).field("ServerPrincName", &self.ServerPrincName).field("AuthnLevel", &self.AuthnLevel).field("AuthnSvc", &self.AuthnSvc).field("AuthIdentity", &self.AuthIdentity).field("SecurityQos", &self.SecurityQos).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for RPC_BINDING_HANDLE_SECURITY_V1_A {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for RPC_BINDING_HANDLE_SECURITY_V1_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_BINDING_HANDLE_SECURITY_V1_A>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for RPC_BINDING_HANDLE_SECURITY_V1_A {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RPC_BINDING_HANDLE_SECURITY_V1_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct RPC_BINDING_HANDLE_SECURITY_V1_W {
    pub Version: u32,
    pub ServerPrincName: *mut u16,
    pub AuthnLevel: u32,
    pub AuthnSvc: u32,
    pub AuthIdentity: *mut SEC_WINNT_AUTH_IDENTITY_W,
    pub SecurityQos: *mut RPC_SECURITY_QOS,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for RPC_BINDING_HANDLE_SECURITY_V1_W {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for RPC_BINDING_HANDLE_SECURITY_V1_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for RPC_BINDING_HANDLE_SECURITY_V1_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_BINDING_HANDLE_SECURITY_V1_W").field("Version", &self.Version).field("ServerPrincName", &self.ServerPrincName).field("AuthnLevel", &self.AuthnLevel).field("AuthnSvc", &self.AuthnSvc).field("AuthIdentity", &self.AuthIdentity).field("SecurityQos", &self.SecurityQos).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for RPC_BINDING_HANDLE_SECURITY_V1_W {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for RPC_BINDING_HANDLE_SECURITY_V1_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_BINDING_HANDLE_SECURITY_V1_W>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for RPC_BINDING_HANDLE_SECURITY_V1_W {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RPC_BINDING_HANDLE_SECURITY_V1_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_BINDING_HANDLE_TEMPLATE_V1_A {
    pub Version: u32,
    pub Flags: u32,
    pub ProtocolSequence: u32,
    pub NetworkAddress: *mut u8,
    pub StringEndpoint: *mut u8,
    pub u1: RPC_BINDING_HANDLE_TEMPLATE_V1_A_0,
    pub ObjectUuid: ::windows::core::GUID,
}
impl ::core::marker::Copy for RPC_BINDING_HANDLE_TEMPLATE_V1_A {}
impl ::core::clone::Clone for RPC_BINDING_HANDLE_TEMPLATE_V1_A {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RPC_BINDING_HANDLE_TEMPLATE_V1_A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RPC_BINDING_HANDLE_TEMPLATE_V1_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_BINDING_HANDLE_TEMPLATE_V1_A>()) == 0 }
    }
}
impl ::core::cmp::Eq for RPC_BINDING_HANDLE_TEMPLATE_V1_A {}
impl ::core::default::Default for RPC_BINDING_HANDLE_TEMPLATE_V1_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub union RPC_BINDING_HANDLE_TEMPLATE_V1_A_0 {
    pub Reserved: *mut u8,
}
impl ::core::marker::Copy for RPC_BINDING_HANDLE_TEMPLATE_V1_A_0 {}
impl ::core::clone::Clone for RPC_BINDING_HANDLE_TEMPLATE_V1_A_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RPC_BINDING_HANDLE_TEMPLATE_V1_A_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RPC_BINDING_HANDLE_TEMPLATE_V1_A_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_BINDING_HANDLE_TEMPLATE_V1_A_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for RPC_BINDING_HANDLE_TEMPLATE_V1_A_0 {}
impl ::core::default::Default for RPC_BINDING_HANDLE_TEMPLATE_V1_A_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_BINDING_HANDLE_TEMPLATE_V1_W {
    pub Version: u32,
    pub Flags: u32,
    pub ProtocolSequence: u32,
    pub NetworkAddress: *mut u16,
    pub StringEndpoint: *mut u16,
    pub u1: RPC_BINDING_HANDLE_TEMPLATE_V1_W_0,
    pub ObjectUuid: ::windows::core::GUID,
}
impl ::core::marker::Copy for RPC_BINDING_HANDLE_TEMPLATE_V1_W {}
impl ::core::clone::Clone for RPC_BINDING_HANDLE_TEMPLATE_V1_W {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RPC_BINDING_HANDLE_TEMPLATE_V1_W {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RPC_BINDING_HANDLE_TEMPLATE_V1_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_BINDING_HANDLE_TEMPLATE_V1_W>()) == 0 }
    }
}
impl ::core::cmp::Eq for RPC_BINDING_HANDLE_TEMPLATE_V1_W {}
impl ::core::default::Default for RPC_BINDING_HANDLE_TEMPLATE_V1_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub union RPC_BINDING_HANDLE_TEMPLATE_V1_W_0 {
    pub Reserved: *mut u16,
}
impl ::core::marker::Copy for RPC_BINDING_HANDLE_TEMPLATE_V1_W_0 {}
impl ::core::clone::Clone for RPC_BINDING_HANDLE_TEMPLATE_V1_W_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RPC_BINDING_HANDLE_TEMPLATE_V1_W_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RPC_BINDING_HANDLE_TEMPLATE_V1_W_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_BINDING_HANDLE_TEMPLATE_V1_W_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for RPC_BINDING_HANDLE_TEMPLATE_V1_W_0 {}
impl ::core::default::Default for RPC_BINDING_HANDLE_TEMPLATE_V1_W_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_BINDING_VECTOR {
    pub Count: u32,
    pub BindingH: [*mut ::core::ffi::c_void; 1],
}
impl ::core::marker::Copy for RPC_BINDING_VECTOR {}
impl ::core::clone::Clone for RPC_BINDING_VECTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_BINDING_VECTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_BINDING_VECTOR").field("Count", &self.Count).field("BindingH", &self.BindingH).finish()
    }
}
unsafe impl ::windows::core::Abi for RPC_BINDING_VECTOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RPC_BINDING_VECTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_BINDING_VECTOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for RPC_BINDING_VECTOR {}
impl ::core::default::Default for RPC_BINDING_VECTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type RPC_BLOCKING_FN = ::core::option::Option<unsafe extern "system" fn(hwnd: *mut ::core::ffi::c_void, context: *mut ::core::ffi::c_void, hsyncevent: *mut ::core::ffi::c_void) -> RPC_STATUS>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_BUFFER_ASYNC: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_BUFFER_COMPLETE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_BUFFER_EXTRA: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_BUFFER_NONOTIFY: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_BUFFER_PARTIAL: u32 = 8192u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RPC_CALL_ATTRIBUTES_V1_A {
    pub Version: u32,
    pub Flags: u32,
    pub ServerPrincipalNameBufferLength: u32,
    pub ServerPrincipalName: *mut u8,
    pub ClientPrincipalNameBufferLength: u32,
    pub ClientPrincipalName: *mut u8,
    pub AuthenticationLevel: u32,
    pub AuthenticationService: u32,
    pub NullSession: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RPC_CALL_ATTRIBUTES_V1_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RPC_CALL_ATTRIBUTES_V1_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RPC_CALL_ATTRIBUTES_V1_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_CALL_ATTRIBUTES_V1_A")
            .field("Version", &self.Version)
            .field("Flags", &self.Flags)
            .field("ServerPrincipalNameBufferLength", &self.ServerPrincipalNameBufferLength)
            .field("ServerPrincipalName", &self.ServerPrincipalName)
            .field("ClientPrincipalNameBufferLength", &self.ClientPrincipalNameBufferLength)
            .field("ClientPrincipalName", &self.ClientPrincipalName)
            .field("AuthenticationLevel", &self.AuthenticationLevel)
            .field("AuthenticationService", &self.AuthenticationService)
            .field("NullSession", &self.NullSession)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RPC_CALL_ATTRIBUTES_V1_A {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RPC_CALL_ATTRIBUTES_V1_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_CALL_ATTRIBUTES_V1_A>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RPC_CALL_ATTRIBUTES_V1_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RPC_CALL_ATTRIBUTES_V1_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RPC_CALL_ATTRIBUTES_V1_W {
    pub Version: u32,
    pub Flags: u32,
    pub ServerPrincipalNameBufferLength: u32,
    pub ServerPrincipalName: *mut u16,
    pub ClientPrincipalNameBufferLength: u32,
    pub ClientPrincipalName: *mut u16,
    pub AuthenticationLevel: u32,
    pub AuthenticationService: u32,
    pub NullSession: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RPC_CALL_ATTRIBUTES_V1_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RPC_CALL_ATTRIBUTES_V1_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RPC_CALL_ATTRIBUTES_V1_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_CALL_ATTRIBUTES_V1_W")
            .field("Version", &self.Version)
            .field("Flags", &self.Flags)
            .field("ServerPrincipalNameBufferLength", &self.ServerPrincipalNameBufferLength)
            .field("ServerPrincipalName", &self.ServerPrincipalName)
            .field("ClientPrincipalNameBufferLength", &self.ClientPrincipalNameBufferLength)
            .field("ClientPrincipalName", &self.ClientPrincipalName)
            .field("AuthenticationLevel", &self.AuthenticationLevel)
            .field("AuthenticationService", &self.AuthenticationService)
            .field("NullSession", &self.NullSession)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RPC_CALL_ATTRIBUTES_V1_W {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RPC_CALL_ATTRIBUTES_V1_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_CALL_ATTRIBUTES_V1_W>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RPC_CALL_ATTRIBUTES_V1_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RPC_CALL_ATTRIBUTES_V1_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RPC_CALL_ATTRIBUTES_V2_A {
    pub Version: u32,
    pub Flags: u32,
    pub ServerPrincipalNameBufferLength: u32,
    pub ServerPrincipalName: *mut u8,
    pub ClientPrincipalNameBufferLength: u32,
    pub ClientPrincipalName: *mut u8,
    pub AuthenticationLevel: u32,
    pub AuthenticationService: u32,
    pub NullSession: super::super::Foundation::BOOL,
    pub KernelModeCaller: super::super::Foundation::BOOL,
    pub ProtocolSequence: u32,
    pub IsClientLocal: u32,
    pub ClientPID: super::super::Foundation::HANDLE,
    pub CallStatus: u32,
    pub CallType: RpcCallType,
    pub CallLocalAddress: *mut RPC_CALL_LOCAL_ADDRESS_V1,
    pub OpNum: u16,
    pub InterfaceUuid: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RPC_CALL_ATTRIBUTES_V2_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RPC_CALL_ATTRIBUTES_V2_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RPC_CALL_ATTRIBUTES_V2_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_CALL_ATTRIBUTES_V2_A")
            .field("Version", &self.Version)
            .field("Flags", &self.Flags)
            .field("ServerPrincipalNameBufferLength", &self.ServerPrincipalNameBufferLength)
            .field("ServerPrincipalName", &self.ServerPrincipalName)
            .field("ClientPrincipalNameBufferLength", &self.ClientPrincipalNameBufferLength)
            .field("ClientPrincipalName", &self.ClientPrincipalName)
            .field("AuthenticationLevel", &self.AuthenticationLevel)
            .field("AuthenticationService", &self.AuthenticationService)
            .field("NullSession", &self.NullSession)
            .field("KernelModeCaller", &self.KernelModeCaller)
            .field("ProtocolSequence", &self.ProtocolSequence)
            .field("IsClientLocal", &self.IsClientLocal)
            .field("ClientPID", &self.ClientPID)
            .field("CallStatus", &self.CallStatus)
            .field("CallType", &self.CallType)
            .field("CallLocalAddress", &self.CallLocalAddress)
            .field("OpNum", &self.OpNum)
            .field("InterfaceUuid", &self.InterfaceUuid)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RPC_CALL_ATTRIBUTES_V2_A {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RPC_CALL_ATTRIBUTES_V2_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_CALL_ATTRIBUTES_V2_A>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RPC_CALL_ATTRIBUTES_V2_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RPC_CALL_ATTRIBUTES_V2_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RPC_CALL_ATTRIBUTES_V2_W {
    pub Version: u32,
    pub Flags: u32,
    pub ServerPrincipalNameBufferLength: u32,
    pub ServerPrincipalName: *mut u16,
    pub ClientPrincipalNameBufferLength: u32,
    pub ClientPrincipalName: *mut u16,
    pub AuthenticationLevel: u32,
    pub AuthenticationService: u32,
    pub NullSession: super::super::Foundation::BOOL,
    pub KernelModeCaller: super::super::Foundation::BOOL,
    pub ProtocolSequence: u32,
    pub IsClientLocal: RpcCallClientLocality,
    pub ClientPID: super::super::Foundation::HANDLE,
    pub CallStatus: u32,
    pub CallType: RpcCallType,
    pub CallLocalAddress: *mut RPC_CALL_LOCAL_ADDRESS_V1,
    pub OpNum: u16,
    pub InterfaceUuid: ::windows::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RPC_CALL_ATTRIBUTES_V2_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RPC_CALL_ATTRIBUTES_V2_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RPC_CALL_ATTRIBUTES_V2_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_CALL_ATTRIBUTES_V2_W")
            .field("Version", &self.Version)
            .field("Flags", &self.Flags)
            .field("ServerPrincipalNameBufferLength", &self.ServerPrincipalNameBufferLength)
            .field("ServerPrincipalName", &self.ServerPrincipalName)
            .field("ClientPrincipalNameBufferLength", &self.ClientPrincipalNameBufferLength)
            .field("ClientPrincipalName", &self.ClientPrincipalName)
            .field("AuthenticationLevel", &self.AuthenticationLevel)
            .field("AuthenticationService", &self.AuthenticationService)
            .field("NullSession", &self.NullSession)
            .field("KernelModeCaller", &self.KernelModeCaller)
            .field("ProtocolSequence", &self.ProtocolSequence)
            .field("IsClientLocal", &self.IsClientLocal)
            .field("ClientPID", &self.ClientPID)
            .field("CallStatus", &self.CallStatus)
            .field("CallType", &self.CallType)
            .field("CallLocalAddress", &self.CallLocalAddress)
            .field("OpNum", &self.OpNum)
            .field("InterfaceUuid", &self.InterfaceUuid)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RPC_CALL_ATTRIBUTES_V2_W {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RPC_CALL_ATTRIBUTES_V2_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_CALL_ATTRIBUTES_V2_W>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RPC_CALL_ATTRIBUTES_V2_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RPC_CALL_ATTRIBUTES_V2_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RPC_CALL_ATTRIBUTES_V3_A {
    pub Version: u32,
    pub Flags: u32,
    pub ServerPrincipalNameBufferLength: u32,
    pub ServerPrincipalName: *mut u8,
    pub ClientPrincipalNameBufferLength: u32,
    pub ClientPrincipalName: *mut u8,
    pub AuthenticationLevel: u32,
    pub AuthenticationService: u32,
    pub NullSession: super::super::Foundation::BOOL,
    pub KernelModeCaller: super::super::Foundation::BOOL,
    pub ProtocolSequence: u32,
    pub IsClientLocal: u32,
    pub ClientPID: super::super::Foundation::HANDLE,
    pub CallStatus: u32,
    pub CallType: RpcCallType,
    pub CallLocalAddress: *mut RPC_CALL_LOCAL_ADDRESS_V1,
    pub OpNum: u16,
    pub InterfaceUuid: ::windows::core::GUID,
    pub ClientIdentifierBufferLength: u32,
    pub ClientIdentifier: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RPC_CALL_ATTRIBUTES_V3_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RPC_CALL_ATTRIBUTES_V3_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RPC_CALL_ATTRIBUTES_V3_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_CALL_ATTRIBUTES_V3_A")
            .field("Version", &self.Version)
            .field("Flags", &self.Flags)
            .field("ServerPrincipalNameBufferLength", &self.ServerPrincipalNameBufferLength)
            .field("ServerPrincipalName", &self.ServerPrincipalName)
            .field("ClientPrincipalNameBufferLength", &self.ClientPrincipalNameBufferLength)
            .field("ClientPrincipalName", &self.ClientPrincipalName)
            .field("AuthenticationLevel", &self.AuthenticationLevel)
            .field("AuthenticationService", &self.AuthenticationService)
            .field("NullSession", &self.NullSession)
            .field("KernelModeCaller", &self.KernelModeCaller)
            .field("ProtocolSequence", &self.ProtocolSequence)
            .field("IsClientLocal", &self.IsClientLocal)
            .field("ClientPID", &self.ClientPID)
            .field("CallStatus", &self.CallStatus)
            .field("CallType", &self.CallType)
            .field("CallLocalAddress", &self.CallLocalAddress)
            .field("OpNum", &self.OpNum)
            .field("InterfaceUuid", &self.InterfaceUuid)
            .field("ClientIdentifierBufferLength", &self.ClientIdentifierBufferLength)
            .field("ClientIdentifier", &self.ClientIdentifier)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RPC_CALL_ATTRIBUTES_V3_A {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RPC_CALL_ATTRIBUTES_V3_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_CALL_ATTRIBUTES_V3_A>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RPC_CALL_ATTRIBUTES_V3_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RPC_CALL_ATTRIBUTES_V3_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RPC_CALL_ATTRIBUTES_V3_W {
    pub Version: u32,
    pub Flags: u32,
    pub ServerPrincipalNameBufferLength: u32,
    pub ServerPrincipalName: *mut u16,
    pub ClientPrincipalNameBufferLength: u32,
    pub ClientPrincipalName: *mut u16,
    pub AuthenticationLevel: u32,
    pub AuthenticationService: u32,
    pub NullSession: super::super::Foundation::BOOL,
    pub KernelModeCaller: super::super::Foundation::BOOL,
    pub ProtocolSequence: u32,
    pub IsClientLocal: RpcCallClientLocality,
    pub ClientPID: super::super::Foundation::HANDLE,
    pub CallStatus: u32,
    pub CallType: RpcCallType,
    pub CallLocalAddress: *mut RPC_CALL_LOCAL_ADDRESS_V1,
    pub OpNum: u16,
    pub InterfaceUuid: ::windows::core::GUID,
    pub ClientIdentifierBufferLength: u32,
    pub ClientIdentifier: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RPC_CALL_ATTRIBUTES_V3_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RPC_CALL_ATTRIBUTES_V3_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RPC_CALL_ATTRIBUTES_V3_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_CALL_ATTRIBUTES_V3_W")
            .field("Version", &self.Version)
            .field("Flags", &self.Flags)
            .field("ServerPrincipalNameBufferLength", &self.ServerPrincipalNameBufferLength)
            .field("ServerPrincipalName", &self.ServerPrincipalName)
            .field("ClientPrincipalNameBufferLength", &self.ClientPrincipalNameBufferLength)
            .field("ClientPrincipalName", &self.ClientPrincipalName)
            .field("AuthenticationLevel", &self.AuthenticationLevel)
            .field("AuthenticationService", &self.AuthenticationService)
            .field("NullSession", &self.NullSession)
            .field("KernelModeCaller", &self.KernelModeCaller)
            .field("ProtocolSequence", &self.ProtocolSequence)
            .field("IsClientLocal", &self.IsClientLocal)
            .field("ClientPID", &self.ClientPID)
            .field("CallStatus", &self.CallStatus)
            .field("CallType", &self.CallType)
            .field("CallLocalAddress", &self.CallLocalAddress)
            .field("OpNum", &self.OpNum)
            .field("InterfaceUuid", &self.InterfaceUuid)
            .field("ClientIdentifierBufferLength", &self.ClientIdentifierBufferLength)
            .field("ClientIdentifier", &self.ClientIdentifier)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RPC_CALL_ATTRIBUTES_V3_W {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RPC_CALL_ATTRIBUTES_V3_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_CALL_ATTRIBUTES_V3_W>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RPC_CALL_ATTRIBUTES_V3_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RPC_CALL_ATTRIBUTES_V3_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_CALL_ATTRIBUTES_VERSION: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_CALL_LOCAL_ADDRESS_V1 {
    pub Version: u32,
    pub Buffer: *mut ::core::ffi::c_void,
    pub BufferSize: u32,
    pub AddressFormat: RpcLocalAddressFormat,
}
impl ::core::marker::Copy for RPC_CALL_LOCAL_ADDRESS_V1 {}
impl ::core::clone::Clone for RPC_CALL_LOCAL_ADDRESS_V1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_CALL_LOCAL_ADDRESS_V1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_CALL_LOCAL_ADDRESS_V1").field("Version", &self.Version).field("Buffer", &self.Buffer).field("BufferSize", &self.BufferSize).field("AddressFormat", &self.AddressFormat).finish()
    }
}
unsafe impl ::windows::core::Abi for RPC_CALL_LOCAL_ADDRESS_V1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RPC_CALL_LOCAL_ADDRESS_V1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_CALL_LOCAL_ADDRESS_V1>()) == 0 }
    }
}
impl ::core::cmp::Eq for RPC_CALL_LOCAL_ADDRESS_V1 {}
impl ::core::default::Default for RPC_CALL_LOCAL_ADDRESS_V1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_CALL_STATUS_CANCELLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_CALL_STATUS_DISCONNECTED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type RPC_CLIENT_ALLOC = ::core::option::Option<unsafe extern "system" fn(size: usize) -> *mut ::core::ffi::c_void>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type RPC_CLIENT_FREE = ::core::option::Option<unsafe extern "system" fn(ptr: *const ::core::ffi::c_void)>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_CLIENT_INFORMATION1 {
    pub UserName: *mut u8,
    pub ComputerName: *mut u8,
    pub Privilege: u16,
    pub AuthFlags: u32,
}
impl ::core::marker::Copy for RPC_CLIENT_INFORMATION1 {}
impl ::core::clone::Clone for RPC_CLIENT_INFORMATION1 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_CLIENT_INFORMATION1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_CLIENT_INFORMATION1").field("UserName", &self.UserName).field("ComputerName", &self.ComputerName).field("Privilege", &self.Privilege).field("AuthFlags", &self.AuthFlags).finish()
    }
}
unsafe impl ::windows::core::Abi for RPC_CLIENT_INFORMATION1 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RPC_CLIENT_INFORMATION1 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_CLIENT_INFORMATION1>()) == 0 }
    }
}
impl ::core::cmp::Eq for RPC_CLIENT_INFORMATION1 {}
impl ::core::default::Default for RPC_CLIENT_INFORMATION1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_CLIENT_INTERFACE {
    pub Length: u32,
    pub InterfaceId: RPC_SYNTAX_IDENTIFIER,
    pub TransferSyntax: RPC_SYNTAX_IDENTIFIER,
    pub DispatchTable: *mut RPC_DISPATCH_TABLE,
    pub RpcProtseqEndpointCount: u32,
    pub RpcProtseqEndpoint: *mut RPC_PROTSEQ_ENDPOINT,
    pub Reserved: usize,
    pub InterpreterInfo: *const ::core::ffi::c_void,
    pub Flags: u32,
}
impl ::core::marker::Copy for RPC_CLIENT_INTERFACE {}
impl ::core::clone::Clone for RPC_CLIENT_INTERFACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_CLIENT_INTERFACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_CLIENT_INTERFACE").field("Length", &self.Length).field("InterfaceId", &self.InterfaceId).field("TransferSyntax", &self.TransferSyntax).field("DispatchTable", &self.DispatchTable).field("RpcProtseqEndpointCount", &self.RpcProtseqEndpointCount).field("RpcProtseqEndpoint", &self.RpcProtseqEndpoint).field("Reserved", &self.Reserved).field("InterpreterInfo", &self.InterpreterInfo).field("Flags", &self.Flags).finish()
    }
}
unsafe impl ::windows::core::Abi for RPC_CLIENT_INTERFACE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RPC_CLIENT_INTERFACE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_CLIENT_INTERFACE>()) == 0 }
    }
}
impl ::core::cmp::Eq for RPC_CLIENT_INTERFACE {}
impl ::core::default::Default for RPC_CLIENT_INTERFACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_CONTEXT_HANDLE_DEFAULT_FLAGS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_CONTEXT_HANDLE_DONT_SERIALIZE: u32 = 536870912u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_CONTEXT_HANDLE_FLAGS: u32 = 805306368u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_CONTEXT_HANDLE_SERIALIZE: u32 = 268435456u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHN_CLOUD_AP: u32 = 36u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHN_DCE_PRIVATE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHN_DCE_PUBLIC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHN_DEC_PUBLIC: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHN_DEFAULT: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHN_DIGEST: u32 = 21u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHN_DPA: u32 = 17u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHN_GSS_KERBEROS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHN_GSS_NEGOTIATE: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHN_GSS_SCHANNEL: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RPC_C_AUTHN_INFO_TYPE(pub u32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHN_INFO_NONE: RPC_C_AUTHN_INFO_TYPE = RPC_C_AUTHN_INFO_TYPE(0u32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHN_INFO_TYPE_HTTP: RPC_C_AUTHN_INFO_TYPE = RPC_C_AUTHN_INFO_TYPE(1u32);
impl ::core::marker::Copy for RPC_C_AUTHN_INFO_TYPE {}
impl ::core::clone::Clone for RPC_C_AUTHN_INFO_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RPC_C_AUTHN_INFO_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RPC_C_AUTHN_INFO_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RPC_C_AUTHN_INFO_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPC_C_AUTHN_INFO_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHN_KERNEL: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHN_LIVEXP_SSP: u32 = 35u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHN_LIVE_SSP: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHN_MQ: u32 = 100u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHN_MSN: u32 = 18u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHN_MSONLINE: u32 = 82u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHN_NEGO_EXTENDER: u32 = 30u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHN_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHN_PKU2U: u32 = 31u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHN_WINNT: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHZ_DCE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHZ_DEFAULT: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHZ_NAME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_AUTHZ_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_BINDING_DEFAULT_TIMEOUT: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_BINDING_INFINITE_TIMEOUT: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_BINDING_MAX_TIMEOUT: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_BINDING_MIN_TIMEOUT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_BIND_TO_ALL_NICS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_CANCEL_INFINITE_TIMEOUT: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_DONT_FAIL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_EP_ALL_ELTS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_EP_MATCH_BY_BOTH: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_EP_MATCH_BY_IF: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_EP_MATCH_BY_OBJ: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_FULL_CERT_CHAIN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_HTTP_AUTHN_SCHEME_BASIC: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_HTTP_AUTHN_SCHEME_CERT: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_HTTP_AUTHN_SCHEME_DIGEST: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_HTTP_AUTHN_SCHEME_NEGOTIATE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_HTTP_AUTHN_SCHEME_NTLM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_HTTP_AUTHN_SCHEME_PASSPORT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RPC_C_HTTP_AUTHN_TARGET(pub u32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_HTTP_AUTHN_TARGET_SERVER: RPC_C_HTTP_AUTHN_TARGET = RPC_C_HTTP_AUTHN_TARGET(1u32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_HTTP_AUTHN_TARGET_PROXY: RPC_C_HTTP_AUTHN_TARGET = RPC_C_HTTP_AUTHN_TARGET(2u32);
impl ::core::marker::Copy for RPC_C_HTTP_AUTHN_TARGET {}
impl ::core::clone::Clone for RPC_C_HTTP_AUTHN_TARGET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RPC_C_HTTP_AUTHN_TARGET {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RPC_C_HTTP_AUTHN_TARGET {
    type Abi = Self;
}
impl ::core::fmt::Debug for RPC_C_HTTP_AUTHN_TARGET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPC_C_HTTP_AUTHN_TARGET").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for RPC_C_HTTP_AUTHN_TARGET {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for RPC_C_HTTP_AUTHN_TARGET {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for RPC_C_HTTP_AUTHN_TARGET {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for RPC_C_HTTP_AUTHN_TARGET {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for RPC_C_HTTP_AUTHN_TARGET {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RPC_C_HTTP_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_HTTP_FLAG_USE_SSL: RPC_C_HTTP_FLAGS = RPC_C_HTTP_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_HTTP_FLAG_USE_FIRST_AUTH_SCHEME: RPC_C_HTTP_FLAGS = RPC_C_HTTP_FLAGS(2u32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_HTTP_FLAG_IGNORE_CERT_CN_INVALID: RPC_C_HTTP_FLAGS = RPC_C_HTTP_FLAGS(8u32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_HTTP_FLAG_ENABLE_CERT_REVOCATION_CHECK: RPC_C_HTTP_FLAGS = RPC_C_HTTP_FLAGS(16u32);
impl ::core::marker::Copy for RPC_C_HTTP_FLAGS {}
impl ::core::clone::Clone for RPC_C_HTTP_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RPC_C_HTTP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RPC_C_HTTP_FLAGS {
    type Abi = Self;
}
impl ::core::fmt::Debug for RPC_C_HTTP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPC_C_HTTP_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for RPC_C_HTTP_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for RPC_C_HTTP_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for RPC_C_HTTP_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for RPC_C_HTTP_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for RPC_C_HTTP_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_LISTEN_MAX_CALLS_DEFAULT: u32 = 1234u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_MGMT_INQ_IF_IDS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_MGMT_INQ_PRINC_NAME: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_MGMT_INQ_STATS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_MGMT_IS_SERVER_LISTEN: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_MGMT_STOP_SERVER_LISTEN: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_MQ_AUTHN_LEVEL_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_MQ_AUTHN_LEVEL_PKT_INTEGRITY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_MQ_AUTHN_LEVEL_PKT_PRIVACY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_MQ_CLEAR_ON_OPEN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_MQ_EXPRESS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_MQ_JOURNAL_ALWAYS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_MQ_JOURNAL_DEADLETTER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_MQ_JOURNAL_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_MQ_PERMANENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_MQ_RECOVERABLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_MQ_TEMPORARY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_MQ_USE_EXISTING_SECURITY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_NOTIFY_ON_SEND_COMPLETE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_NS_DEFAULT_EXP_AGE: i32 = -1i32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_OPT_ASYNC_BLOCK: u32 = 15u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_OPT_BINDING_NONCAUSAL: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_OPT_CALL_TIMEOUT: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_OPT_COOKIE_AUTH: u32 = 7u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_C_OPT_COOKIE_AUTH_DESCRIPTOR {
    pub BufferSize: u32,
    pub Buffer: ::windows::core::PSTR,
}
impl ::core::marker::Copy for RPC_C_OPT_COOKIE_AUTH_DESCRIPTOR {}
impl ::core::clone::Clone for RPC_C_OPT_COOKIE_AUTH_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_C_OPT_COOKIE_AUTH_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_C_OPT_COOKIE_AUTH_DESCRIPTOR").field("BufferSize", &self.BufferSize).field("Buffer", &self.Buffer).finish()
    }
}
unsafe impl ::windows::core::Abi for RPC_C_OPT_COOKIE_AUTH_DESCRIPTOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RPC_C_OPT_COOKIE_AUTH_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_C_OPT_COOKIE_AUTH_DESCRIPTOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for RPC_C_OPT_COOKIE_AUTH_DESCRIPTOR {}
impl ::core::default::Default for RPC_C_OPT_COOKIE_AUTH_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_OPT_DONT_LINGER: u32 = 13u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_OPT_MAX_OPTIONS: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_OPT_MQ_ACKNOWLEDGE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_OPT_MQ_AUTHN_LEVEL: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_OPT_MQ_AUTHN_SERVICE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_OPT_MQ_DELIVERY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_OPT_MQ_JOURNAL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_OPT_MQ_PRIORITY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_OPT_MQ_TIME_TO_BE_RECEIVED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_OPT_MQ_TIME_TO_REACH_QUEUE: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_OPT_OPTIMIZE_TIME: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_OPT_PRIVATE_BREAK_ON_SUSPEND: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_OPT_PRIVATE_DO_NOT_DISTURB: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_OPT_PRIVATE_SUPPRESS_WAKE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_OPT_RESOURCE_TYPE_UUID: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_OPT_SECURITY_CALLBACK: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_OPT_SESSION_ID: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_OPT_TRANS_SEND_BUFFER_SIZE: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_OPT_TRUST_PEER: u32 = 14u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_OPT_UNIQUE_BINDING: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_PARM_BUFFER_LENGTH: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_PARM_MAX_PACKET_LENGTH: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_PROFILE_ALL_ELT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_PROFILE_ALL_ELTS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_PROFILE_DEFAULT_ELT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_PROFILE_MATCH_BY_BOTH: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_PROFILE_MATCH_BY_IF: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_PROFILE_MATCH_BY_MBR: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_PROTSEQ_MAX_REQS_DEFAULT: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RPC_C_QOS_CAPABILITIES(pub u32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_QOS_CAPABILITIES_DEFAULT: RPC_C_QOS_CAPABILITIES = RPC_C_QOS_CAPABILITIES(0u32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_QOS_CAPABILITIES_MUTUAL_AUTH: RPC_C_QOS_CAPABILITIES = RPC_C_QOS_CAPABILITIES(1u32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_QOS_CAPABILITIES_MAKE_FULLSIC: RPC_C_QOS_CAPABILITIES = RPC_C_QOS_CAPABILITIES(2u32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_QOS_CAPABILITIES_ANY_AUTHORITY: RPC_C_QOS_CAPABILITIES = RPC_C_QOS_CAPABILITIES(4u32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_QOS_CAPABILITIES_IGNORE_DELEGATE_FAILURE: RPC_C_QOS_CAPABILITIES = RPC_C_QOS_CAPABILITIES(8u32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_QOS_CAPABILITIES_LOCAL_MA_HINT: RPC_C_QOS_CAPABILITIES = RPC_C_QOS_CAPABILITIES(16u32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_QOS_CAPABILITIES_SCHANNEL_FULL_AUTH_IDENTITY: RPC_C_QOS_CAPABILITIES = RPC_C_QOS_CAPABILITIES(32u32);
impl ::core::marker::Copy for RPC_C_QOS_CAPABILITIES {}
impl ::core::clone::Clone for RPC_C_QOS_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RPC_C_QOS_CAPABILITIES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RPC_C_QOS_CAPABILITIES {
    type Abi = Self;
}
impl ::core::fmt::Debug for RPC_C_QOS_CAPABILITIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPC_C_QOS_CAPABILITIES").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for RPC_C_QOS_CAPABILITIES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for RPC_C_QOS_CAPABILITIES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for RPC_C_QOS_CAPABILITIES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for RPC_C_QOS_CAPABILITIES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for RPC_C_QOS_CAPABILITIES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RPC_C_QOS_IDENTITY(pub u32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_QOS_IDENTITY_STATIC: RPC_C_QOS_IDENTITY = RPC_C_QOS_IDENTITY(0u32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_QOS_IDENTITY_DYNAMIC: RPC_C_QOS_IDENTITY = RPC_C_QOS_IDENTITY(1u32);
impl ::core::marker::Copy for RPC_C_QOS_IDENTITY {}
impl ::core::clone::Clone for RPC_C_QOS_IDENTITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RPC_C_QOS_IDENTITY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RPC_C_QOS_IDENTITY {
    type Abi = Self;
}
impl ::core::fmt::Debug for RPC_C_QOS_IDENTITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPC_C_QOS_IDENTITY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_RPCHTTP_USE_LOAD_BALANCE: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_SECURITY_QOS_VERSION: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_SECURITY_QOS_VERSION_1: i32 = 1i32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_SECURITY_QOS_VERSION_2: i32 = 2i32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_SECURITY_QOS_VERSION_3: i32 = 3i32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_SECURITY_QOS_VERSION_4: i32 = 4i32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_SECURITY_QOS_VERSION_5: i32 = 5i32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_STATS_CALLS_IN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_STATS_CALLS_OUT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_STATS_PKTS_IN: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_STATS_PKTS_OUT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_TRY_ENFORCE_MAX_CALLS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_USE_INTERNET_PORT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_USE_INTRANET_PORT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_VERS_ALL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_VERS_COMPATIBLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_VERS_EXACT: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_VERS_MAJOR_ONLY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_C_VERS_UPTO: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type RPC_DISPATCH_FUNCTION = ::core::option::Option<unsafe extern "system" fn(message: *mut RPC_MESSAGE)>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_DISPATCH_TABLE {
    pub DispatchTableCount: u32,
    pub DispatchTable: RPC_DISPATCH_FUNCTION,
    pub Reserved: isize,
}
impl ::core::marker::Copy for RPC_DISPATCH_TABLE {}
impl ::core::clone::Clone for RPC_DISPATCH_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_DISPATCH_TABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_DISPATCH_TABLE").field("DispatchTableCount", &self.DispatchTableCount).field("DispatchTable", &self.DispatchTable.map(|f| f as usize)).field("Reserved", &self.Reserved).finish()
    }
}
unsafe impl ::windows::core::Abi for RPC_DISPATCH_TABLE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RPC_DISPATCH_TABLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_DISPATCH_TABLE>()) == 0 }
    }
}
impl ::core::cmp::Eq for RPC_DISPATCH_TABLE {}
impl ::core::default::Default for RPC_DISPATCH_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_EEINFO_VERSION: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_EE_INFO_PARAM {
    pub ParameterType: ExtendedErrorParamTypes,
    pub u: RPC_EE_INFO_PARAM_0,
}
impl ::core::marker::Copy for RPC_EE_INFO_PARAM {}
impl ::core::clone::Clone for RPC_EE_INFO_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RPC_EE_INFO_PARAM {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RPC_EE_INFO_PARAM {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_EE_INFO_PARAM>()) == 0 }
    }
}
impl ::core::cmp::Eq for RPC_EE_INFO_PARAM {}
impl ::core::default::Default for RPC_EE_INFO_PARAM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub union RPC_EE_INFO_PARAM_0 {
    pub AnsiString: ::windows::core::PSTR,
    pub UnicodeString: ::windows::core::PWSTR,
    pub LVal: i32,
    pub SVal: i16,
    pub PVal: u64,
    pub BVal: BinaryParam,
}
impl ::core::marker::Copy for RPC_EE_INFO_PARAM_0 {}
impl ::core::clone::Clone for RPC_EE_INFO_PARAM_0 {
    fn clone(&self) -> Self {
        *self
    }
}
unsafe impl ::windows::core::Abi for RPC_EE_INFO_PARAM_0 {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RPC_EE_INFO_PARAM_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_EE_INFO_PARAM_0>()) == 0 }
    }
}
impl ::core::cmp::Eq for RPC_EE_INFO_PARAM_0 {}
impl ::core::default::Default for RPC_EE_INFO_PARAM_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_ENDPOINT_TEMPLATEA {
    pub Version: u32,
    pub ProtSeq: *mut u8,
    pub Endpoint: *mut u8,
    pub SecurityDescriptor: *mut ::core::ffi::c_void,
    pub Backlog: u32,
}
impl ::core::marker::Copy for RPC_ENDPOINT_TEMPLATEA {}
impl ::core::clone::Clone for RPC_ENDPOINT_TEMPLATEA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_ENDPOINT_TEMPLATEA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_ENDPOINT_TEMPLATEA").field("Version", &self.Version).field("ProtSeq", &self.ProtSeq).field("Endpoint", &self.Endpoint).field("SecurityDescriptor", &self.SecurityDescriptor).field("Backlog", &self.Backlog).finish()
    }
}
unsafe impl ::windows::core::Abi for RPC_ENDPOINT_TEMPLATEA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RPC_ENDPOINT_TEMPLATEA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_ENDPOINT_TEMPLATEA>()) == 0 }
    }
}
impl ::core::cmp::Eq for RPC_ENDPOINT_TEMPLATEA {}
impl ::core::default::Default for RPC_ENDPOINT_TEMPLATEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_ENDPOINT_TEMPLATEW {
    pub Version: u32,
    pub ProtSeq: *mut u16,
    pub Endpoint: *mut u16,
    pub SecurityDescriptor: *mut ::core::ffi::c_void,
    pub Backlog: u32,
}
impl ::core::marker::Copy for RPC_ENDPOINT_TEMPLATEW {}
impl ::core::clone::Clone for RPC_ENDPOINT_TEMPLATEW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_ENDPOINT_TEMPLATEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_ENDPOINT_TEMPLATEW").field("Version", &self.Version).field("ProtSeq", &self.ProtSeq).field("Endpoint", &self.Endpoint).field("SecurityDescriptor", &self.SecurityDescriptor).field("Backlog", &self.Backlog).finish()
    }
}
unsafe impl ::windows::core::Abi for RPC_ENDPOINT_TEMPLATEW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RPC_ENDPOINT_TEMPLATEW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_ENDPOINT_TEMPLATEW>()) == 0 }
    }
}
impl ::core::cmp::Eq for RPC_ENDPOINT_TEMPLATEW {}
impl ::core::default::Default for RPC_ENDPOINT_TEMPLATEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_ERROR_ENUM_HANDLE {
    pub Signature: u32,
    pub CurrentPos: *mut ::core::ffi::c_void,
    pub Head: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for RPC_ERROR_ENUM_HANDLE {}
impl ::core::clone::Clone for RPC_ERROR_ENUM_HANDLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_ERROR_ENUM_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_ERROR_ENUM_HANDLE").field("Signature", &self.Signature).field("CurrentPos", &self.CurrentPos).field("Head", &self.Head).finish()
    }
}
unsafe impl ::windows::core::Abi for RPC_ERROR_ENUM_HANDLE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RPC_ERROR_ENUM_HANDLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_ERROR_ENUM_HANDLE>()) == 0 }
    }
}
impl ::core::cmp::Eq for RPC_ERROR_ENUM_HANDLE {}
impl ::core::default::Default for RPC_ERROR_ENUM_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct RPC_EXTENDED_ERROR_INFO {
    pub Version: u32,
    pub ComputerName: ::windows::core::PWSTR,
    pub ProcessID: u32,
    pub u: RPC_EXTENDED_ERROR_INFO_0,
    pub GeneratingComponent: u32,
    pub Status: u32,
    pub DetectionLocation: u16,
    pub Flags: u16,
    pub NumberOfParameters: i32,
    pub Parameters: [RPC_EE_INFO_PARAM; 4],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RPC_EXTENDED_ERROR_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RPC_EXTENDED_ERROR_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RPC_EXTENDED_ERROR_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RPC_EXTENDED_ERROR_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_EXTENDED_ERROR_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RPC_EXTENDED_ERROR_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RPC_EXTENDED_ERROR_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub union RPC_EXTENDED_ERROR_INFO_0 {
    pub SystemTime: super::super::Foundation::SYSTEMTIME,
    pub FileTime: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for RPC_EXTENDED_ERROR_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for RPC_EXTENDED_ERROR_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RPC_EXTENDED_ERROR_INFO_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RPC_EXTENDED_ERROR_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_EXTENDED_ERROR_INFO_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RPC_EXTENDED_ERROR_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RPC_EXTENDED_ERROR_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_FLAGS_VALID_BIT: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type RPC_FORWARD_FUNCTION = ::core::option::Option<unsafe extern "system" fn(interfaceid: *mut ::windows::core::GUID, interfaceversion: *mut RPC_VERSION, objectid: *mut ::windows::core::GUID, rpcpro: *mut u8, ppdestendpoint: *mut *mut ::core::ffi::c_void) -> RPC_STATUS>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_FW_IF_FLAG_DCOM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type RPC_HTTP_PROXY_FREE_STRING = ::core::option::Option<unsafe extern "system" fn(string: *const u16)>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RPC_HTTP_REDIRECTOR_STAGE(pub i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPCHTTP_RS_REDIRECT: RPC_HTTP_REDIRECTOR_STAGE = RPC_HTTP_REDIRECTOR_STAGE(1i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPCHTTP_RS_ACCESS_1: RPC_HTTP_REDIRECTOR_STAGE = RPC_HTTP_REDIRECTOR_STAGE(2i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPCHTTP_RS_SESSION: RPC_HTTP_REDIRECTOR_STAGE = RPC_HTTP_REDIRECTOR_STAGE(3i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPCHTTP_RS_ACCESS_2: RPC_HTTP_REDIRECTOR_STAGE = RPC_HTTP_REDIRECTOR_STAGE(4i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPCHTTP_RS_INTERFACE: RPC_HTTP_REDIRECTOR_STAGE = RPC_HTTP_REDIRECTOR_STAGE(5i32);
impl ::core::marker::Copy for RPC_HTTP_REDIRECTOR_STAGE {}
impl ::core::clone::Clone for RPC_HTTP_REDIRECTOR_STAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RPC_HTTP_REDIRECTOR_STAGE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RPC_HTTP_REDIRECTOR_STAGE {
    type Abi = Self;
}
impl ::core::fmt::Debug for RPC_HTTP_REDIRECTOR_STAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPC_HTTP_REDIRECTOR_STAGE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_A {
    pub TransportCredentials: *mut SEC_WINNT_AUTH_IDENTITY_A,
    pub Flags: RPC_C_HTTP_FLAGS,
    pub AuthenticationTarget: RPC_C_HTTP_AUTHN_TARGET,
    pub NumberOfAuthnSchemes: u32,
    pub AuthnSchemes: *mut u32,
    pub ServerCertificateSubject: *mut u8,
}
impl ::core::marker::Copy for RPC_HTTP_TRANSPORT_CREDENTIALS_A {}
impl ::core::clone::Clone for RPC_HTTP_TRANSPORT_CREDENTIALS_A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_HTTP_TRANSPORT_CREDENTIALS_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_HTTP_TRANSPORT_CREDENTIALS_A").field("TransportCredentials", &self.TransportCredentials).field("Flags", &self.Flags).field("AuthenticationTarget", &self.AuthenticationTarget).field("NumberOfAuthnSchemes", &self.NumberOfAuthnSchemes).field("AuthnSchemes", &self.AuthnSchemes).field("ServerCertificateSubject", &self.ServerCertificateSubject).finish()
    }
}
unsafe impl ::windows::core::Abi for RPC_HTTP_TRANSPORT_CREDENTIALS_A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RPC_HTTP_TRANSPORT_CREDENTIALS_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_HTTP_TRANSPORT_CREDENTIALS_A>()) == 0 }
    }
}
impl ::core::cmp::Eq for RPC_HTTP_TRANSPORT_CREDENTIALS_A {}
impl ::core::default::Default for RPC_HTTP_TRANSPORT_CREDENTIALS_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A {
    pub TransportCredentials: *mut SEC_WINNT_AUTH_IDENTITY_A,
    pub Flags: RPC_C_HTTP_FLAGS,
    pub AuthenticationTarget: RPC_C_HTTP_AUTHN_TARGET,
    pub NumberOfAuthnSchemes: u32,
    pub AuthnSchemes: *mut u32,
    pub ServerCertificateSubject: *mut u8,
    pub ProxyCredentials: *mut SEC_WINNT_AUTH_IDENTITY_A,
    pub NumberOfProxyAuthnSchemes: u32,
    pub ProxyAuthnSchemes: *mut u32,
}
impl ::core::marker::Copy for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A {}
impl ::core::clone::Clone for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A")
            .field("TransportCredentials", &self.TransportCredentials)
            .field("Flags", &self.Flags)
            .field("AuthenticationTarget", &self.AuthenticationTarget)
            .field("NumberOfAuthnSchemes", &self.NumberOfAuthnSchemes)
            .field("AuthnSchemes", &self.AuthnSchemes)
            .field("ServerCertificateSubject", &self.ServerCertificateSubject)
            .field("ProxyCredentials", &self.ProxyCredentials)
            .field("NumberOfProxyAuthnSchemes", &self.NumberOfProxyAuthnSchemes)
            .field("ProxyAuthnSchemes", &self.ProxyAuthnSchemes)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A>()) == 0 }
    }
}
impl ::core::cmp::Eq for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A {}
impl ::core::default::Default for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W {
    pub TransportCredentials: *mut SEC_WINNT_AUTH_IDENTITY_W,
    pub Flags: RPC_C_HTTP_FLAGS,
    pub AuthenticationTarget: RPC_C_HTTP_AUTHN_TARGET,
    pub NumberOfAuthnSchemes: u32,
    pub AuthnSchemes: *mut u32,
    pub ServerCertificateSubject: *mut u16,
    pub ProxyCredentials: *mut SEC_WINNT_AUTH_IDENTITY_W,
    pub NumberOfProxyAuthnSchemes: u32,
    pub ProxyAuthnSchemes: *mut u32,
}
impl ::core::marker::Copy for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W {}
impl ::core::clone::Clone for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W")
            .field("TransportCredentials", &self.TransportCredentials)
            .field("Flags", &self.Flags)
            .field("AuthenticationTarget", &self.AuthenticationTarget)
            .field("NumberOfAuthnSchemes", &self.NumberOfAuthnSchemes)
            .field("AuthnSchemes", &self.AuthnSchemes)
            .field("ServerCertificateSubject", &self.ServerCertificateSubject)
            .field("ProxyCredentials", &self.ProxyCredentials)
            .field("NumberOfProxyAuthnSchemes", &self.NumberOfProxyAuthnSchemes)
            .field("ProxyAuthnSchemes", &self.ProxyAuthnSchemes)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W>()) == 0 }
    }
}
impl ::core::cmp::Eq for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W {}
impl ::core::default::Default for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A {
    pub TransportCredentials: *mut ::core::ffi::c_void,
    pub Flags: RPC_C_HTTP_FLAGS,
    pub AuthenticationTarget: RPC_C_HTTP_AUTHN_TARGET,
    pub NumberOfAuthnSchemes: u32,
    pub AuthnSchemes: *mut u32,
    pub ServerCertificateSubject: *mut u8,
    pub ProxyCredentials: *mut ::core::ffi::c_void,
    pub NumberOfProxyAuthnSchemes: u32,
    pub ProxyAuthnSchemes: *mut u32,
}
impl ::core::marker::Copy for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A {}
impl ::core::clone::Clone for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A")
            .field("TransportCredentials", &self.TransportCredentials)
            .field("Flags", &self.Flags)
            .field("AuthenticationTarget", &self.AuthenticationTarget)
            .field("NumberOfAuthnSchemes", &self.NumberOfAuthnSchemes)
            .field("AuthnSchemes", &self.AuthnSchemes)
            .field("ServerCertificateSubject", &self.ServerCertificateSubject)
            .field("ProxyCredentials", &self.ProxyCredentials)
            .field("NumberOfProxyAuthnSchemes", &self.NumberOfProxyAuthnSchemes)
            .field("ProxyAuthnSchemes", &self.ProxyAuthnSchemes)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A>()) == 0 }
    }
}
impl ::core::cmp::Eq for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A {}
impl ::core::default::Default for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W {
    pub TransportCredentials: *mut ::core::ffi::c_void,
    pub Flags: RPC_C_HTTP_FLAGS,
    pub AuthenticationTarget: RPC_C_HTTP_AUTHN_TARGET,
    pub NumberOfAuthnSchemes: u32,
    pub AuthnSchemes: *mut u32,
    pub ServerCertificateSubject: *mut u16,
    pub ProxyCredentials: *mut ::core::ffi::c_void,
    pub NumberOfProxyAuthnSchemes: u32,
    pub ProxyAuthnSchemes: *mut u32,
}
impl ::core::marker::Copy for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W {}
impl ::core::clone::Clone for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W")
            .field("TransportCredentials", &self.TransportCredentials)
            .field("Flags", &self.Flags)
            .field("AuthenticationTarget", &self.AuthenticationTarget)
            .field("NumberOfAuthnSchemes", &self.NumberOfAuthnSchemes)
            .field("AuthnSchemes", &self.AuthnSchemes)
            .field("ServerCertificateSubject", &self.ServerCertificateSubject)
            .field("ProxyCredentials", &self.ProxyCredentials)
            .field("NumberOfProxyAuthnSchemes", &self.NumberOfProxyAuthnSchemes)
            .field("ProxyAuthnSchemes", &self.ProxyAuthnSchemes)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W>()) == 0 }
    }
}
impl ::core::cmp::Eq for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W {}
impl ::core::default::Default for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_W {
    pub TransportCredentials: *mut SEC_WINNT_AUTH_IDENTITY_W,
    pub Flags: RPC_C_HTTP_FLAGS,
    pub AuthenticationTarget: RPC_C_HTTP_AUTHN_TARGET,
    pub NumberOfAuthnSchemes: u32,
    pub AuthnSchemes: *mut u32,
    pub ServerCertificateSubject: *mut u16,
}
impl ::core::marker::Copy for RPC_HTTP_TRANSPORT_CREDENTIALS_W {}
impl ::core::clone::Clone for RPC_HTTP_TRANSPORT_CREDENTIALS_W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_HTTP_TRANSPORT_CREDENTIALS_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_HTTP_TRANSPORT_CREDENTIALS_W").field("TransportCredentials", &self.TransportCredentials).field("Flags", &self.Flags).field("AuthenticationTarget", &self.AuthenticationTarget).field("NumberOfAuthnSchemes", &self.NumberOfAuthnSchemes).field("AuthnSchemes", &self.AuthnSchemes).field("ServerCertificateSubject", &self.ServerCertificateSubject).finish()
    }
}
unsafe impl ::windows::core::Abi for RPC_HTTP_TRANSPORT_CREDENTIALS_W {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RPC_HTTP_TRANSPORT_CREDENTIALS_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_HTTP_TRANSPORT_CREDENTIALS_W>()) == 0 }
    }
}
impl ::core::cmp::Eq for RPC_HTTP_TRANSPORT_CREDENTIALS_W {}
impl ::core::default::Default for RPC_HTTP_TRANSPORT_CREDENTIALS_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_IF_ALLOW_CALLBACKS_WITH_NO_AUTH: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_IF_ALLOW_LOCAL_ONLY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_IF_ALLOW_SECURE_ONLY: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_IF_ALLOW_UNKNOWN_AUTHORITY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_IF_ASYNC_CALLBACK: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_IF_AUTOLISTEN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type RPC_IF_CALLBACK_FN = ::core::option::Option<unsafe extern "system" fn(interfaceuuid: *const ::core::ffi::c_void, context: *const ::core::ffi::c_void) -> RPC_STATUS>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_IF_ID {
    pub Uuid: ::windows::core::GUID,
    pub VersMajor: u16,
    pub VersMinor: u16,
}
impl ::core::marker::Copy for RPC_IF_ID {}
impl ::core::clone::Clone for RPC_IF_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_IF_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_IF_ID").field("Uuid", &self.Uuid).field("VersMajor", &self.VersMajor).field("VersMinor", &self.VersMinor).finish()
    }
}
unsafe impl ::windows::core::Abi for RPC_IF_ID {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RPC_IF_ID {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_IF_ID>()) == 0 }
    }
}
impl ::core::cmp::Eq for RPC_IF_ID {}
impl ::core::default::Default for RPC_IF_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_IF_ID_VECTOR {
    pub Count: u32,
    pub IfId: [*mut RPC_IF_ID; 1],
}
impl ::core::marker::Copy for RPC_IF_ID_VECTOR {}
impl ::core::clone::Clone for RPC_IF_ID_VECTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_IF_ID_VECTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_IF_ID_VECTOR").field("Count", &self.Count).field("IfId", &self.IfId).finish()
    }
}
unsafe impl ::windows::core::Abi for RPC_IF_ID_VECTOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RPC_IF_ID_VECTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_IF_ID_VECTOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for RPC_IF_ID_VECTOR {}
impl ::core::default::Default for RPC_IF_ID_VECTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_IF_OLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_IF_SEC_CACHE_PER_PROC: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_IF_SEC_NO_CACHE: u32 = 64u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_IMPORT_CONTEXT_P {
    pub LookupContext: *mut ::core::ffi::c_void,
    pub ProposedHandle: *mut ::core::ffi::c_void,
    pub Bindings: *mut RPC_BINDING_VECTOR,
}
impl ::core::marker::Copy for RPC_IMPORT_CONTEXT_P {}
impl ::core::clone::Clone for RPC_IMPORT_CONTEXT_P {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_IMPORT_CONTEXT_P {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_IMPORT_CONTEXT_P").field("LookupContext", &self.LookupContext).field("ProposedHandle", &self.ProposedHandle).field("Bindings", &self.Bindings).finish()
    }
}
unsafe impl ::windows::core::Abi for RPC_IMPORT_CONTEXT_P {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RPC_IMPORT_CONTEXT_P {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_IMPORT_CONTEXT_P>()) == 0 }
    }
}
impl ::core::cmp::Eq for RPC_IMPORT_CONTEXT_P {}
impl ::core::default::Default for RPC_IMPORT_CONTEXT_P {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type RPC_INTERFACE_GROUP_IDLE_CALLBACK_FN = ::core::option::Option<unsafe extern "system" fn(ifgroup: *const ::core::ffi::c_void, idlecallbackcontext: *const ::core::ffi::c_void, isgroupidle: u32)>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_INTERFACE_HAS_PIPES: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_INTERFACE_TEMPLATEA {
    pub Version: u32,
    pub IfSpec: *mut ::core::ffi::c_void,
    pub MgrTypeUuid: *mut ::windows::core::GUID,
    pub MgrEpv: *mut ::core::ffi::c_void,
    pub Flags: u32,
    pub MaxCalls: u32,
    pub MaxRpcSize: u32,
    pub IfCallback: RPC_IF_CALLBACK_FN,
    pub UuidVector: *mut UUID_VECTOR,
    pub Annotation: *mut u8,
    pub SecurityDescriptor: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for RPC_INTERFACE_TEMPLATEA {}
impl ::core::clone::Clone for RPC_INTERFACE_TEMPLATEA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_INTERFACE_TEMPLATEA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_INTERFACE_TEMPLATEA").field("Version", &self.Version).field("IfSpec", &self.IfSpec).field("MgrTypeUuid", &self.MgrTypeUuid).field("MgrEpv", &self.MgrEpv).field("Flags", &self.Flags).field("MaxCalls", &self.MaxCalls).field("MaxRpcSize", &self.MaxRpcSize).field("IfCallback", &self.IfCallback.map(|f| f as usize)).field("UuidVector", &self.UuidVector).field("Annotation", &self.Annotation).field("SecurityDescriptor", &self.SecurityDescriptor).finish()
    }
}
unsafe impl ::windows::core::Abi for RPC_INTERFACE_TEMPLATEA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RPC_INTERFACE_TEMPLATEA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_INTERFACE_TEMPLATEA>()) == 0 }
    }
}
impl ::core::cmp::Eq for RPC_INTERFACE_TEMPLATEA {}
impl ::core::default::Default for RPC_INTERFACE_TEMPLATEA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_INTERFACE_TEMPLATEW {
    pub Version: u32,
    pub IfSpec: *mut ::core::ffi::c_void,
    pub MgrTypeUuid: *mut ::windows::core::GUID,
    pub MgrEpv: *mut ::core::ffi::c_void,
    pub Flags: u32,
    pub MaxCalls: u32,
    pub MaxRpcSize: u32,
    pub IfCallback: RPC_IF_CALLBACK_FN,
    pub UuidVector: *mut UUID_VECTOR,
    pub Annotation: *mut u16,
    pub SecurityDescriptor: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for RPC_INTERFACE_TEMPLATEW {}
impl ::core::clone::Clone for RPC_INTERFACE_TEMPLATEW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_INTERFACE_TEMPLATEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_INTERFACE_TEMPLATEW").field("Version", &self.Version).field("IfSpec", &self.IfSpec).field("MgrTypeUuid", &self.MgrTypeUuid).field("MgrEpv", &self.MgrEpv).field("Flags", &self.Flags).field("MaxCalls", &self.MaxCalls).field("MaxRpcSize", &self.MaxRpcSize).field("IfCallback", &self.IfCallback.map(|f| f as usize)).field("UuidVector", &self.UuidVector).field("Annotation", &self.Annotation).field("SecurityDescriptor", &self.SecurityDescriptor).finish()
    }
}
unsafe impl ::windows::core::Abi for RPC_INTERFACE_TEMPLATEW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RPC_INTERFACE_TEMPLATEW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_INTERFACE_TEMPLATEW>()) == 0 }
    }
}
impl ::core::cmp::Eq for RPC_INTERFACE_TEMPLATEW {}
impl ::core::default::Default for RPC_INTERFACE_TEMPLATEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_MESSAGE {
    pub Handle: *mut ::core::ffi::c_void,
    pub DataRepresentation: u32,
    pub Buffer: *mut ::core::ffi::c_void,
    pub BufferLength: u32,
    pub ProcNum: u32,
    pub TransferSyntax: *mut RPC_SYNTAX_IDENTIFIER,
    pub RpcInterfaceInformation: *mut ::core::ffi::c_void,
    pub ReservedForRuntime: *mut ::core::ffi::c_void,
    pub ManagerEpv: *mut ::core::ffi::c_void,
    pub ImportContext: *mut ::core::ffi::c_void,
    pub RpcFlags: u32,
}
impl ::core::marker::Copy for RPC_MESSAGE {}
impl ::core::clone::Clone for RPC_MESSAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_MESSAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_MESSAGE")
            .field("Handle", &self.Handle)
            .field("DataRepresentation", &self.DataRepresentation)
            .field("Buffer", &self.Buffer)
            .field("BufferLength", &self.BufferLength)
            .field("ProcNum", &self.ProcNum)
            .field("TransferSyntax", &self.TransferSyntax)
            .field("RpcInterfaceInformation", &self.RpcInterfaceInformation)
            .field("ReservedForRuntime", &self.ReservedForRuntime)
            .field("ManagerEpv", &self.ManagerEpv)
            .field("ImportContext", &self.ImportContext)
            .field("RpcFlags", &self.RpcFlags)
            .finish()
    }
}
unsafe impl ::windows::core::Abi for RPC_MESSAGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RPC_MESSAGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_MESSAGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for RPC_MESSAGE {}
impl ::core::default::Default for RPC_MESSAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type RPC_MGMT_AUTHORIZATION_FN = ::core::option::Option<unsafe extern "system" fn(clientbinding: *const ::core::ffi::c_void, requestedmgmtoperation: u32, status: *mut RPC_STATUS) -> i32>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_NCA_FLAGS_BROADCAST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_NCA_FLAGS_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_NCA_FLAGS_IDEMPOTENT: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_NCA_FLAGS_MAYBE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type RPC_NEW_HTTP_PROXY_CHANNEL = ::core::option::Option<unsafe extern "system" fn(redirectorstage: RPC_HTTP_REDIRECTOR_STAGE, servername: *const u16, serverport: *const u16, remoteuser: *const u16, authtype: *const u16, resourceuuid: *mut ::core::ffi::c_void, sessionid: *mut ::core::ffi::c_void, interface: *const ::core::ffi::c_void, reserved: *const ::core::ffi::c_void, flags: u32, newservername: *mut *mut u16, newserverport: *mut *mut u16) -> RPC_STATUS>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RPC_NOTIFICATIONS(pub i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcNotificationCallNone: RPC_NOTIFICATIONS = RPC_NOTIFICATIONS(0i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcNotificationClientDisconnect: RPC_NOTIFICATIONS = RPC_NOTIFICATIONS(1i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcNotificationCallCancel: RPC_NOTIFICATIONS = RPC_NOTIFICATIONS(2i32);
impl ::core::marker::Copy for RPC_NOTIFICATIONS {}
impl ::core::clone::Clone for RPC_NOTIFICATIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RPC_NOTIFICATIONS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RPC_NOTIFICATIONS {
    type Abi = Self;
}
impl ::core::fmt::Debug for RPC_NOTIFICATIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPC_NOTIFICATIONS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RPC_NOTIFICATION_TYPES(pub i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcNotificationTypeNone: RPC_NOTIFICATION_TYPES = RPC_NOTIFICATION_TYPES(0i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcNotificationTypeEvent: RPC_NOTIFICATION_TYPES = RPC_NOTIFICATION_TYPES(1i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcNotificationTypeApc: RPC_NOTIFICATION_TYPES = RPC_NOTIFICATION_TYPES(2i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcNotificationTypeIoc: RPC_NOTIFICATION_TYPES = RPC_NOTIFICATION_TYPES(3i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcNotificationTypeHwnd: RPC_NOTIFICATION_TYPES = RPC_NOTIFICATION_TYPES(4i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcNotificationTypeCallback: RPC_NOTIFICATION_TYPES = RPC_NOTIFICATION_TYPES(5i32);
impl ::core::marker::Copy for RPC_NOTIFICATION_TYPES {}
impl ::core::clone::Clone for RPC_NOTIFICATION_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RPC_NOTIFICATION_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RPC_NOTIFICATION_TYPES {
    type Abi = Self;
}
impl ::core::fmt::Debug for RPC_NOTIFICATION_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPC_NOTIFICATION_TYPES").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type RPC_OBJECT_INQ_FN = ::core::option::Option<unsafe extern "system" fn(objectuuid: *const ::windows::core::GUID, typeuuid: *mut ::windows::core::GUID, status: *mut RPC_STATUS)>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_POLICY {
    pub Length: u32,
    pub EndpointFlags: u32,
    pub NICFlags: u32,
}
impl ::core::marker::Copy for RPC_POLICY {}
impl ::core::clone::Clone for RPC_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_POLICY").field("Length", &self.Length).field("EndpointFlags", &self.EndpointFlags).field("NICFlags", &self.NICFlags).finish()
    }
}
unsafe impl ::windows::core::Abi for RPC_POLICY {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RPC_POLICY {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_POLICY>()) == 0 }
    }
}
impl ::core::cmp::Eq for RPC_POLICY {}
impl ::core::default::Default for RPC_POLICY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_PROTSEQ_ENDPOINT {
    pub RpcProtocolSequence: *mut u8,
    pub Endpoint: *mut u8,
}
impl ::core::marker::Copy for RPC_PROTSEQ_ENDPOINT {}
impl ::core::clone::Clone for RPC_PROTSEQ_ENDPOINT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_PROTSEQ_ENDPOINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_PROTSEQ_ENDPOINT").field("RpcProtocolSequence", &self.RpcProtocolSequence).field("Endpoint", &self.Endpoint).finish()
    }
}
unsafe impl ::windows::core::Abi for RPC_PROTSEQ_ENDPOINT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RPC_PROTSEQ_ENDPOINT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_PROTSEQ_ENDPOINT>()) == 0 }
    }
}
impl ::core::cmp::Eq for RPC_PROTSEQ_ENDPOINT {}
impl ::core::default::Default for RPC_PROTSEQ_ENDPOINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_PROTSEQ_HTTP: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_PROTSEQ_LRPC: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_PROTSEQ_NMP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_PROTSEQ_TCP: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_PROTSEQ_VECTORA {
    pub Count: u32,
    pub Protseq: [*mut u8; 1],
}
impl ::core::marker::Copy for RPC_PROTSEQ_VECTORA {}
impl ::core::clone::Clone for RPC_PROTSEQ_VECTORA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_PROTSEQ_VECTORA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_PROTSEQ_VECTORA").field("Count", &self.Count).field("Protseq", &self.Protseq).finish()
    }
}
unsafe impl ::windows::core::Abi for RPC_PROTSEQ_VECTORA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RPC_PROTSEQ_VECTORA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_PROTSEQ_VECTORA>()) == 0 }
    }
}
impl ::core::cmp::Eq for RPC_PROTSEQ_VECTORA {}
impl ::core::default::Default for RPC_PROTSEQ_VECTORA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_PROTSEQ_VECTORW {
    pub Count: u32,
    pub Protseq: [*mut u16; 1],
}
impl ::core::marker::Copy for RPC_PROTSEQ_VECTORW {}
impl ::core::clone::Clone for RPC_PROTSEQ_VECTORW {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_PROTSEQ_VECTORW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_PROTSEQ_VECTORW").field("Count", &self.Count).field("Protseq", &self.Protseq).finish()
    }
}
unsafe impl ::windows::core::Abi for RPC_PROTSEQ_VECTORW {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RPC_PROTSEQ_VECTORW {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_PROTSEQ_VECTORW>()) == 0 }
    }
}
impl ::core::cmp::Eq for RPC_PROTSEQ_VECTORW {}
impl ::core::default::Default for RPC_PROTSEQ_VECTORW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_PROXY_CONNECTION_TYPE_IN_PROXY: u32 = 0u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_PROXY_CONNECTION_TYPE_OUT_PROXY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_P_ADDR_FORMAT_TCP_IPV4: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_P_ADDR_FORMAT_TCP_IPV6: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_QUERY_CALL_LOCAL_ADDRESS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_QUERY_CLIENT_ID: u32 = 128u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_QUERY_CLIENT_PID: u32 = 16u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_QUERY_CLIENT_PRINCIPAL_NAME: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_QUERY_IS_CLIENT_LOCAL: u32 = 32u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_QUERY_NO_AUTH_REQUIRED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_QUERY_SERVER_PRINCIPAL_NAME: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type RPC_SECURITY_CALLBACK_FN = ::core::option::Option<unsafe extern "system" fn(context: *const ::core::ffi::c_void)>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct RPC_SECURITY_QOS {
    pub Version: u32,
    pub Capabilities: RPC_C_QOS_CAPABILITIES,
    pub IdentityTracking: RPC_C_QOS_IDENTITY,
    pub ImpersonationType: super::Com::RPC_C_IMP_LEVEL,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for RPC_SECURITY_QOS {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for RPC_SECURITY_QOS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for RPC_SECURITY_QOS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_SECURITY_QOS").field("Version", &self.Version).field("Capabilities", &self.Capabilities).field("IdentityTracking", &self.IdentityTracking).field("ImpersonationType", &self.ImpersonationType).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for RPC_SECURITY_QOS {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for RPC_SECURITY_QOS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_SECURITY_QOS>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for RPC_SECURITY_QOS {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RPC_SECURITY_QOS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct RPC_SECURITY_QOS_V2_A {
    pub Version: u32,
    pub Capabilities: RPC_C_QOS_CAPABILITIES,
    pub IdentityTracking: RPC_C_QOS_IDENTITY,
    pub ImpersonationType: super::Com::RPC_C_IMP_LEVEL,
    pub AdditionalSecurityInfoType: RPC_C_AUTHN_INFO_TYPE,
    pub u: RPC_SECURITY_QOS_V2_A_0,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for RPC_SECURITY_QOS_V2_A {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for RPC_SECURITY_QOS_V2_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for RPC_SECURITY_QOS_V2_A {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for RPC_SECURITY_QOS_V2_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_SECURITY_QOS_V2_A>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for RPC_SECURITY_QOS_V2_A {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RPC_SECURITY_QOS_V2_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub union RPC_SECURITY_QOS_V2_A_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_A,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for RPC_SECURITY_QOS_V2_A_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for RPC_SECURITY_QOS_V2_A_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for RPC_SECURITY_QOS_V2_A_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for RPC_SECURITY_QOS_V2_A_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_SECURITY_QOS_V2_A_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for RPC_SECURITY_QOS_V2_A_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RPC_SECURITY_QOS_V2_A_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct RPC_SECURITY_QOS_V2_W {
    pub Version: u32,
    pub Capabilities: RPC_C_QOS_CAPABILITIES,
    pub IdentityTracking: RPC_C_QOS_IDENTITY,
    pub ImpersonationType: super::Com::RPC_C_IMP_LEVEL,
    pub AdditionalSecurityInfoType: RPC_C_AUTHN_INFO_TYPE,
    pub u: RPC_SECURITY_QOS_V2_W_0,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for RPC_SECURITY_QOS_V2_W {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for RPC_SECURITY_QOS_V2_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for RPC_SECURITY_QOS_V2_W {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for RPC_SECURITY_QOS_V2_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_SECURITY_QOS_V2_W>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for RPC_SECURITY_QOS_V2_W {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RPC_SECURITY_QOS_V2_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub union RPC_SECURITY_QOS_V2_W_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_W,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for RPC_SECURITY_QOS_V2_W_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for RPC_SECURITY_QOS_V2_W_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for RPC_SECURITY_QOS_V2_W_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for RPC_SECURITY_QOS_V2_W_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_SECURITY_QOS_V2_W_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for RPC_SECURITY_QOS_V2_W_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RPC_SECURITY_QOS_V2_W_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct RPC_SECURITY_QOS_V3_A {
    pub Version: u32,
    pub Capabilities: RPC_C_QOS_CAPABILITIES,
    pub IdentityTracking: RPC_C_QOS_IDENTITY,
    pub ImpersonationType: super::Com::RPC_C_IMP_LEVEL,
    pub AdditionalSecurityInfoType: RPC_C_AUTHN_INFO_TYPE,
    pub u: RPC_SECURITY_QOS_V3_A_0,
    pub Sid: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for RPC_SECURITY_QOS_V3_A {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for RPC_SECURITY_QOS_V3_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for RPC_SECURITY_QOS_V3_A {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for RPC_SECURITY_QOS_V3_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_SECURITY_QOS_V3_A>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for RPC_SECURITY_QOS_V3_A {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RPC_SECURITY_QOS_V3_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub union RPC_SECURITY_QOS_V3_A_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_A,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for RPC_SECURITY_QOS_V3_A_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for RPC_SECURITY_QOS_V3_A_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for RPC_SECURITY_QOS_V3_A_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for RPC_SECURITY_QOS_V3_A_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_SECURITY_QOS_V3_A_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for RPC_SECURITY_QOS_V3_A_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RPC_SECURITY_QOS_V3_A_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct RPC_SECURITY_QOS_V3_W {
    pub Version: u32,
    pub Capabilities: RPC_C_QOS_CAPABILITIES,
    pub IdentityTracking: RPC_C_QOS_IDENTITY,
    pub ImpersonationType: super::Com::RPC_C_IMP_LEVEL,
    pub AdditionalSecurityInfoType: RPC_C_AUTHN_INFO_TYPE,
    pub u: RPC_SECURITY_QOS_V3_W_0,
    pub Sid: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for RPC_SECURITY_QOS_V3_W {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for RPC_SECURITY_QOS_V3_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for RPC_SECURITY_QOS_V3_W {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for RPC_SECURITY_QOS_V3_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_SECURITY_QOS_V3_W>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for RPC_SECURITY_QOS_V3_W {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RPC_SECURITY_QOS_V3_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub union RPC_SECURITY_QOS_V3_W_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_W,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for RPC_SECURITY_QOS_V3_W_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for RPC_SECURITY_QOS_V3_W_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for RPC_SECURITY_QOS_V3_W_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for RPC_SECURITY_QOS_V3_W_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_SECURITY_QOS_V3_W_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for RPC_SECURITY_QOS_V3_W_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RPC_SECURITY_QOS_V3_W_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct RPC_SECURITY_QOS_V4_A {
    pub Version: u32,
    pub Capabilities: RPC_C_QOS_CAPABILITIES,
    pub IdentityTracking: RPC_C_QOS_IDENTITY,
    pub ImpersonationType: super::Com::RPC_C_IMP_LEVEL,
    pub AdditionalSecurityInfoType: RPC_C_AUTHN_INFO_TYPE,
    pub u: RPC_SECURITY_QOS_V4_A_0,
    pub Sid: *mut ::core::ffi::c_void,
    pub EffectiveOnly: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for RPC_SECURITY_QOS_V4_A {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for RPC_SECURITY_QOS_V4_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for RPC_SECURITY_QOS_V4_A {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for RPC_SECURITY_QOS_V4_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_SECURITY_QOS_V4_A>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for RPC_SECURITY_QOS_V4_A {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RPC_SECURITY_QOS_V4_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub union RPC_SECURITY_QOS_V4_A_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_A,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for RPC_SECURITY_QOS_V4_A_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for RPC_SECURITY_QOS_V4_A_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for RPC_SECURITY_QOS_V4_A_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for RPC_SECURITY_QOS_V4_A_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_SECURITY_QOS_V4_A_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for RPC_SECURITY_QOS_V4_A_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RPC_SECURITY_QOS_V4_A_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct RPC_SECURITY_QOS_V4_W {
    pub Version: u32,
    pub Capabilities: RPC_C_QOS_CAPABILITIES,
    pub IdentityTracking: RPC_C_QOS_IDENTITY,
    pub ImpersonationType: super::Com::RPC_C_IMP_LEVEL,
    pub AdditionalSecurityInfoType: RPC_C_AUTHN_INFO_TYPE,
    pub u: RPC_SECURITY_QOS_V4_W_0,
    pub Sid: *mut ::core::ffi::c_void,
    pub EffectiveOnly: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for RPC_SECURITY_QOS_V4_W {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for RPC_SECURITY_QOS_V4_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for RPC_SECURITY_QOS_V4_W {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for RPC_SECURITY_QOS_V4_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_SECURITY_QOS_V4_W>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for RPC_SECURITY_QOS_V4_W {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RPC_SECURITY_QOS_V4_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub union RPC_SECURITY_QOS_V4_W_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_W,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for RPC_SECURITY_QOS_V4_W_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for RPC_SECURITY_QOS_V4_W_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for RPC_SECURITY_QOS_V4_W_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for RPC_SECURITY_QOS_V4_W_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_SECURITY_QOS_V4_W_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for RPC_SECURITY_QOS_V4_W_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RPC_SECURITY_QOS_V4_W_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct RPC_SECURITY_QOS_V5_A {
    pub Version: u32,
    pub Capabilities: RPC_C_QOS_CAPABILITIES,
    pub IdentityTracking: RPC_C_QOS_IDENTITY,
    pub ImpersonationType: super::Com::RPC_C_IMP_LEVEL,
    pub AdditionalSecurityInfoType: RPC_C_AUTHN_INFO_TYPE,
    pub u: RPC_SECURITY_QOS_V5_A_0,
    pub Sid: *mut ::core::ffi::c_void,
    pub EffectiveOnly: u32,
    pub ServerSecurityDescriptor: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for RPC_SECURITY_QOS_V5_A {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for RPC_SECURITY_QOS_V5_A {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for RPC_SECURITY_QOS_V5_A {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for RPC_SECURITY_QOS_V5_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_SECURITY_QOS_V5_A>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for RPC_SECURITY_QOS_V5_A {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RPC_SECURITY_QOS_V5_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub union RPC_SECURITY_QOS_V5_A_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_A,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for RPC_SECURITY_QOS_V5_A_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for RPC_SECURITY_QOS_V5_A_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for RPC_SECURITY_QOS_V5_A_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for RPC_SECURITY_QOS_V5_A_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_SECURITY_QOS_V5_A_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for RPC_SECURITY_QOS_V5_A_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RPC_SECURITY_QOS_V5_A_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct RPC_SECURITY_QOS_V5_W {
    pub Version: u32,
    pub Capabilities: RPC_C_QOS_CAPABILITIES,
    pub IdentityTracking: RPC_C_QOS_IDENTITY,
    pub ImpersonationType: super::Com::RPC_C_IMP_LEVEL,
    pub AdditionalSecurityInfoType: RPC_C_AUTHN_INFO_TYPE,
    pub u: RPC_SECURITY_QOS_V5_W_0,
    pub Sid: *mut ::core::ffi::c_void,
    pub EffectiveOnly: u32,
    pub ServerSecurityDescriptor: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for RPC_SECURITY_QOS_V5_W {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for RPC_SECURITY_QOS_V5_W {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for RPC_SECURITY_QOS_V5_W {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for RPC_SECURITY_QOS_V5_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_SECURITY_QOS_V5_W>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for RPC_SECURITY_QOS_V5_W {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RPC_SECURITY_QOS_V5_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub union RPC_SECURITY_QOS_V5_W_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_W,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for RPC_SECURITY_QOS_V5_W_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for RPC_SECURITY_QOS_V5_W_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for RPC_SECURITY_QOS_V5_W_0 {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for RPC_SECURITY_QOS_V5_W_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_SECURITY_QOS_V5_W_0>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for RPC_SECURITY_QOS_V5_W_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for RPC_SECURITY_QOS_V5_W_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_SEC_CONTEXT_KEY_INFO {
    pub EncryptAlgorithm: u32,
    pub KeySize: u32,
    pub SignatureAlgorithm: u32,
}
impl ::core::marker::Copy for RPC_SEC_CONTEXT_KEY_INFO {}
impl ::core::clone::Clone for RPC_SEC_CONTEXT_KEY_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_SEC_CONTEXT_KEY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_SEC_CONTEXT_KEY_INFO").field("EncryptAlgorithm", &self.EncryptAlgorithm).field("KeySize", &self.KeySize).field("SignatureAlgorithm", &self.SignatureAlgorithm).finish()
    }
}
unsafe impl ::windows::core::Abi for RPC_SEC_CONTEXT_KEY_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RPC_SEC_CONTEXT_KEY_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_SEC_CONTEXT_KEY_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for RPC_SEC_CONTEXT_KEY_INFO {}
impl ::core::default::Default for RPC_SEC_CONTEXT_KEY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_SERVER_INTERFACE {
    pub Length: u32,
    pub InterfaceId: RPC_SYNTAX_IDENTIFIER,
    pub TransferSyntax: RPC_SYNTAX_IDENTIFIER,
    pub DispatchTable: *mut RPC_DISPATCH_TABLE,
    pub RpcProtseqEndpointCount: u32,
    pub RpcProtseqEndpoint: *mut RPC_PROTSEQ_ENDPOINT,
    pub DefaultManagerEpv: *mut ::core::ffi::c_void,
    pub InterpreterInfo: *const ::core::ffi::c_void,
    pub Flags: u32,
}
impl ::core::marker::Copy for RPC_SERVER_INTERFACE {}
impl ::core::clone::Clone for RPC_SERVER_INTERFACE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_SERVER_INTERFACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_SERVER_INTERFACE").field("Length", &self.Length).field("InterfaceId", &self.InterfaceId).field("TransferSyntax", &self.TransferSyntax).field("DispatchTable", &self.DispatchTable).field("RpcProtseqEndpointCount", &self.RpcProtseqEndpointCount).field("RpcProtseqEndpoint", &self.RpcProtseqEndpoint).field("DefaultManagerEpv", &self.DefaultManagerEpv).field("InterpreterInfo", &self.InterpreterInfo).field("Flags", &self.Flags).finish()
    }
}
unsafe impl ::windows::core::Abi for RPC_SERVER_INTERFACE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RPC_SERVER_INTERFACE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_SERVER_INTERFACE>()) == 0 }
    }
}
impl ::core::cmp::Eq for RPC_SERVER_INTERFACE {}
impl ::core::default::Default for RPC_SERVER_INTERFACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type RPC_SETFILTER_FUNC = ::core::option::Option<unsafe extern "system" fn(pfnfilter: RPCLT_PDU_FILTER_FUNC)>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_STATS_VECTOR {
    pub Count: u32,
    pub Stats: [u32; 1],
}
impl ::core::marker::Copy for RPC_STATS_VECTOR {}
impl ::core::clone::Clone for RPC_STATS_VECTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_STATS_VECTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_STATS_VECTOR").field("Count", &self.Count).field("Stats", &self.Stats).finish()
    }
}
unsafe impl ::windows::core::Abi for RPC_STATS_VECTOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RPC_STATS_VECTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_STATS_VECTOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for RPC_STATS_VECTOR {}
impl ::core::default::Default for RPC_STATS_VECTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RPC_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_INVALID_STRING_BINDING: RPC_STATUS = RPC_STATUS(1700i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_WRONG_KIND_OF_BINDING: RPC_STATUS = RPC_STATUS(1701i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_INVALID_BINDING: RPC_STATUS = RPC_STATUS(1702i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_PROTSEQ_NOT_SUPPORTED: RPC_STATUS = RPC_STATUS(1703i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_INVALID_RPC_PROTSEQ: RPC_STATUS = RPC_STATUS(1704i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_INVALID_STRING_UUID: RPC_STATUS = RPC_STATUS(1705i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_INVALID_ENDPOINT_FORMAT: RPC_STATUS = RPC_STATUS(1706i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_INVALID_NET_ADDR: RPC_STATUS = RPC_STATUS(1707i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_NO_ENDPOINT_FOUND: RPC_STATUS = RPC_STATUS(1708i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_INVALID_TIMEOUT: RPC_STATUS = RPC_STATUS(1709i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_OBJECT_NOT_FOUND: RPC_STATUS = RPC_STATUS(1710i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_ALREADY_REGISTERED: RPC_STATUS = RPC_STATUS(1711i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_TYPE_ALREADY_REGISTERED: RPC_STATUS = RPC_STATUS(1712i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_ALREADY_LISTENING: RPC_STATUS = RPC_STATUS(1713i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_NO_PROTSEQS_REGISTERED: RPC_STATUS = RPC_STATUS(1714i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_NOT_LISTENING: RPC_STATUS = RPC_STATUS(1715i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_UNKNOWN_MGR_TYPE: RPC_STATUS = RPC_STATUS(1716i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_UNKNOWN_IF: RPC_STATUS = RPC_STATUS(1717i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_NO_BINDINGS: RPC_STATUS = RPC_STATUS(1718i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_NO_PROTSEQS: RPC_STATUS = RPC_STATUS(1719i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_CANT_CREATE_ENDPOINT: RPC_STATUS = RPC_STATUS(1720i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_OUT_OF_RESOURCES: RPC_STATUS = RPC_STATUS(1721i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_SERVER_UNAVAILABLE: RPC_STATUS = RPC_STATUS(1722i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_SERVER_TOO_BUSY: RPC_STATUS = RPC_STATUS(1723i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_INVALID_NETWORK_OPTIONS: RPC_STATUS = RPC_STATUS(1724i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_NO_CALL_ACTIVE: RPC_STATUS = RPC_STATUS(1725i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_CALL_FAILED: RPC_STATUS = RPC_STATUS(1726i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_CALL_FAILED_DNE: RPC_STATUS = RPC_STATUS(1727i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_PROTOCOL_ERROR: RPC_STATUS = RPC_STATUS(1728i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_PROXY_ACCESS_DENIED: RPC_STATUS = RPC_STATUS(1729i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_UNSUPPORTED_TRANS_SYN: RPC_STATUS = RPC_STATUS(1730i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_UNSUPPORTED_TYPE: RPC_STATUS = RPC_STATUS(1732i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_INVALID_TAG: RPC_STATUS = RPC_STATUS(1733i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_INVALID_BOUND: RPC_STATUS = RPC_STATUS(1734i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_NO_ENTRY_NAME: RPC_STATUS = RPC_STATUS(1735i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_INVALID_NAME_SYNTAX: RPC_STATUS = RPC_STATUS(1736i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_UNSUPPORTED_NAME_SYNTAX: RPC_STATUS = RPC_STATUS(1737i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_UUID_NO_ADDRESS: RPC_STATUS = RPC_STATUS(1739i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_DUPLICATE_ENDPOINT: RPC_STATUS = RPC_STATUS(1740i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_UNKNOWN_AUTHN_TYPE: RPC_STATUS = RPC_STATUS(1741i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_MAX_CALLS_TOO_SMALL: RPC_STATUS = RPC_STATUS(1742i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_STRING_TOO_LONG: RPC_STATUS = RPC_STATUS(1743i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_PROTSEQ_NOT_FOUND: RPC_STATUS = RPC_STATUS(1744i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_PROCNUM_OUT_OF_RANGE: RPC_STATUS = RPC_STATUS(1745i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_BINDING_HAS_NO_AUTH: RPC_STATUS = RPC_STATUS(1746i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_UNKNOWN_AUTHN_SERVICE: RPC_STATUS = RPC_STATUS(1747i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_UNKNOWN_AUTHN_LEVEL: RPC_STATUS = RPC_STATUS(1748i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_INVALID_AUTH_IDENTITY: RPC_STATUS = RPC_STATUS(1749i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_UNKNOWN_AUTHZ_SERVICE: RPC_STATUS = RPC_STATUS(1750i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const EPT_S_INVALID_ENTRY: RPC_STATUS = RPC_STATUS(1751i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const EPT_S_CANT_PERFORM_OP: RPC_STATUS = RPC_STATUS(1752i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const EPT_S_NOT_REGISTERED: RPC_STATUS = RPC_STATUS(1753i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_NOTHING_TO_EXPORT: RPC_STATUS = RPC_STATUS(1754i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_INCOMPLETE_NAME: RPC_STATUS = RPC_STATUS(1755i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_INVALID_VERS_OPTION: RPC_STATUS = RPC_STATUS(1756i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_NO_MORE_MEMBERS: RPC_STATUS = RPC_STATUS(1757i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_NOT_ALL_OBJS_UNEXPORTED: RPC_STATUS = RPC_STATUS(1758i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_INTERFACE_NOT_FOUND: RPC_STATUS = RPC_STATUS(1759i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_ENTRY_ALREADY_EXISTS: RPC_STATUS = RPC_STATUS(1760i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_ENTRY_NOT_FOUND: RPC_STATUS = RPC_STATUS(1761i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_NAME_SERVICE_UNAVAILABLE: RPC_STATUS = RPC_STATUS(1762i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_INVALID_NAF_ID: RPC_STATUS = RPC_STATUS(1763i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_CANNOT_SUPPORT: RPC_STATUS = RPC_STATUS(1764i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_NO_CONTEXT_AVAILABLE: RPC_STATUS = RPC_STATUS(1765i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_INTERNAL_ERROR: RPC_STATUS = RPC_STATUS(1766i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_ZERO_DIVIDE: RPC_STATUS = RPC_STATUS(1767i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_ADDRESS_ERROR: RPC_STATUS = RPC_STATUS(1768i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_FP_DIV_ZERO: RPC_STATUS = RPC_STATUS(1769i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_FP_UNDERFLOW: RPC_STATUS = RPC_STATUS(1770i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_FP_OVERFLOW: RPC_STATUS = RPC_STATUS(1771i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_CALL_IN_PROGRESS: RPC_STATUS = RPC_STATUS(1791i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_NO_MORE_BINDINGS: RPC_STATUS = RPC_STATUS(1806i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_NO_INTERFACES: RPC_STATUS = RPC_STATUS(1817i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_CALL_CANCELLED: RPC_STATUS = RPC_STATUS(1818i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_BINDING_INCOMPLETE: RPC_STATUS = RPC_STATUS(1819i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_COMM_FAILURE: RPC_STATUS = RPC_STATUS(1820i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_UNSUPPORTED_AUTHN_LEVEL: RPC_STATUS = RPC_STATUS(1821i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_NO_PRINC_NAME: RPC_STATUS = RPC_STATUS(1822i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_NOT_RPC_ERROR: RPC_STATUS = RPC_STATUS(1823i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_UUID_LOCAL_ONLY: RPC_STATUS = RPC_STATUS(1824i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_SEC_PKG_ERROR: RPC_STATUS = RPC_STATUS(1825i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_NOT_CANCELLED: RPC_STATUS = RPC_STATUS(1826i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_COOKIE_AUTH_FAILED: RPC_STATUS = RPC_STATUS(1833i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_DO_NOT_DISTURB: RPC_STATUS = RPC_STATUS(1834i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_SYSTEM_HANDLE_COUNT_EXCEEDED: RPC_STATUS = RPC_STATUS(1835i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_SYSTEM_HANDLE_TYPE_MISMATCH: RPC_STATUS = RPC_STATUS(1836i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_GROUP_MEMBER_NOT_FOUND: RPC_STATUS = RPC_STATUS(1898i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const EPT_S_CANT_CREATE: RPC_STATUS = RPC_STATUS(1899i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_INVALID_OBJECT: RPC_STATUS = RPC_STATUS(1900i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_SEND_INCOMPLETE: RPC_STATUS = RPC_STATUS(1913i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_INVALID_ASYNC_HANDLE: RPC_STATUS = RPC_STATUS(1914i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_INVALID_ASYNC_CALL: RPC_STATUS = RPC_STATUS(1915i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_ENTRY_TYPE_MISMATCH: RPC_STATUS = RPC_STATUS(1922i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_NOT_ALL_OBJS_EXPORTED: RPC_STATUS = RPC_STATUS(1923i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_INTERFACE_NOT_EXPORTED: RPC_STATUS = RPC_STATUS(1924i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_PROFILE_NOT_ADDED: RPC_STATUS = RPC_STATUS(1925i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_PRF_ELT_NOT_ADDED: RPC_STATUS = RPC_STATUS(1926i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_PRF_ELT_NOT_REMOVED: RPC_STATUS = RPC_STATUS(1927i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_GRP_ELT_NOT_ADDED: RPC_STATUS = RPC_STATUS(1928i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_S_GRP_ELT_NOT_REMOVED: RPC_STATUS = RPC_STATUS(1929i32);
impl ::core::marker::Copy for RPC_STATUS {}
impl ::core::clone::Clone for RPC_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RPC_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RPC_STATUS {
    type Abi = Self;
}
impl ::core::fmt::Debug for RPC_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RPC_STATUS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_SYNTAX_IDENTIFIER {
    pub SyntaxGUID: ::windows::core::GUID,
    pub SyntaxVersion: RPC_VERSION,
}
impl ::core::marker::Copy for RPC_SYNTAX_IDENTIFIER {}
impl ::core::clone::Clone for RPC_SYNTAX_IDENTIFIER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_SYNTAX_IDENTIFIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_SYNTAX_IDENTIFIER").field("SyntaxGUID", &self.SyntaxGUID).field("SyntaxVersion", &self.SyntaxVersion).finish()
    }
}
unsafe impl ::windows::core::Abi for RPC_SYNTAX_IDENTIFIER {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RPC_SYNTAX_IDENTIFIER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_SYNTAX_IDENTIFIER>()) == 0 }
    }
}
impl ::core::cmp::Eq for RPC_SYNTAX_IDENTIFIER {}
impl ::core::default::Default for RPC_SYNTAX_IDENTIFIER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_SYSTEM_HANDLE_FREE_ALL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_SYSTEM_HANDLE_FREE_ERROR_ON_CLOSE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_SYSTEM_HANDLE_FREE_RETRIEVED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_SYSTEM_HANDLE_FREE_UNRETRIEVED: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_TRANSFER_SYNTAX {
    pub Uuid: ::windows::core::GUID,
    pub VersMajor: u16,
    pub VersMinor: u16,
}
impl ::core::marker::Copy for RPC_TRANSFER_SYNTAX {}
impl ::core::clone::Clone for RPC_TRANSFER_SYNTAX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_TRANSFER_SYNTAX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_TRANSFER_SYNTAX").field("Uuid", &self.Uuid).field("VersMajor", &self.VersMajor).field("VersMinor", &self.VersMinor).finish()
    }
}
unsafe impl ::windows::core::Abi for RPC_TRANSFER_SYNTAX {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RPC_TRANSFER_SYNTAX {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_TRANSFER_SYNTAX>()) == 0 }
    }
}
impl ::core::cmp::Eq for RPC_TRANSFER_SYNTAX {}
impl ::core::default::Default for RPC_TRANSFER_SYNTAX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_TYPE_DISCONNECT_EVENT_CONTEXT_HANDLE: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RPC_TYPE_STRICT_CONTEXT_HANDLE: u32 = 1073741824u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct RPC_VERSION {
    pub MajorVersion: u16,
    pub MinorVersion: u16,
}
impl ::core::marker::Copy for RPC_VERSION {}
impl ::core::clone::Clone for RPC_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for RPC_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("RPC_VERSION").field("MajorVersion", &self.MajorVersion).field("MinorVersion", &self.MinorVersion).finish()
    }
}
unsafe impl ::windows::core::Abi for RPC_VERSION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for RPC_VERSION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<RPC_VERSION>()) == 0 }
    }
}
impl ::core::cmp::Eq for RPC_VERSION {}
impl ::core::default::Default for RPC_VERSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn RpcAsyncAbortCall(pasync: *mut RPC_ASYNC_STATE, exceptioncode: u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcAsyncAbortCall(pasync: *mut RPC_ASYNC_STATE, exceptioncode: u32) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcAsyncAbortCall(::core::mem::transmute(pasync), ::core::mem::transmute(exceptioncode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn RpcAsyncCancelCall<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(pasync: *mut RPC_ASYNC_STATE, fabort: Param1) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcAsyncCancelCall(pasync: *mut RPC_ASYNC_STATE, fabort: super::super::Foundation::BOOL) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcAsyncCancelCall(::core::mem::transmute(pasync), fabort.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn RpcAsyncCompleteCall(pasync: *mut RPC_ASYNC_STATE, reply: *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcAsyncCompleteCall(pasync: *mut RPC_ASYNC_STATE, reply: *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcAsyncCompleteCall(::core::mem::transmute(pasync), ::core::mem::transmute(reply)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn RpcAsyncGetCallStatus(pasync: *const RPC_ASYNC_STATE) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcAsyncGetCallStatus(pasync: *const RPC_ASYNC_STATE) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcAsyncGetCallStatus(::core::mem::transmute(pasync)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn RpcAsyncInitializeHandle(pasync: *mut RPC_ASYNC_STATE, size: u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcAsyncInitializeHandle(pasync: *mut RPC_ASYNC_STATE, size: u32) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcAsyncInitializeHandle(::core::mem::transmute(pasync), ::core::mem::transmute(size)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn RpcAsyncRegisterInfo(pasync: *const RPC_ASYNC_STATE) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcAsyncRegisterInfo(pasync: *const RPC_ASYNC_STATE) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcAsyncRegisterInfo(::core::mem::transmute(pasync)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn RpcBindingBind(pasync: *const RPC_ASYNC_STATE, binding: *const ::core::ffi::c_void, ifspec: *const ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingBind(pasync: *const RPC_ASYNC_STATE, binding: *const ::core::ffi::c_void, ifspec: *const ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcBindingBind(::core::mem::transmute(pasync), ::core::mem::transmute(binding), ::core::mem::transmute(ifspec)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcBindingCopy(sourcebinding: *const ::core::ffi::c_void, destinationbinding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingCopy(sourcebinding: *const ::core::ffi::c_void, destinationbinding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcBindingCopy(::core::mem::transmute(sourcebinding), ::core::mem::transmute(destinationbinding)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn RpcBindingCreateA(template: *const RPC_BINDING_HANDLE_TEMPLATE_V1_A, security: *const RPC_BINDING_HANDLE_SECURITY_V1_A, options: *const RPC_BINDING_HANDLE_OPTIONS_V1, binding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingCreateA(template: *const RPC_BINDING_HANDLE_TEMPLATE_V1_A, security: *const RPC_BINDING_HANDLE_SECURITY_V1_A, options: *const RPC_BINDING_HANDLE_OPTIONS_V1, binding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcBindingCreateA(::core::mem::transmute(template), ::core::mem::transmute(security), ::core::mem::transmute(options), ::core::mem::transmute(binding)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn RpcBindingCreateW(template: *const RPC_BINDING_HANDLE_TEMPLATE_V1_W, security: *const RPC_BINDING_HANDLE_SECURITY_V1_W, options: *const RPC_BINDING_HANDLE_OPTIONS_V1, binding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingCreateW(template: *const RPC_BINDING_HANDLE_TEMPLATE_V1_W, security: *const RPC_BINDING_HANDLE_SECURITY_V1_W, options: *const RPC_BINDING_HANDLE_OPTIONS_V1, binding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcBindingCreateW(::core::mem::transmute(template), ::core::mem::transmute(security), ::core::mem::transmute(options), ::core::mem::transmute(binding)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcBindingFree(binding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingFree(binding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcBindingFree(::core::mem::transmute(binding)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcBindingFromStringBindingA(stringbinding: *const u8, binding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingFromStringBindingA(stringbinding: *const u8, binding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcBindingFromStringBindingA(::core::mem::transmute(stringbinding), ::core::mem::transmute(binding)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcBindingFromStringBindingW(stringbinding: *const u16, binding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingFromStringBindingW(stringbinding: *const u16, binding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcBindingFromStringBindingW(::core::mem::transmute(stringbinding), ::core::mem::transmute(binding)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcBindingInqAuthClientA(clientbinding: *const ::core::ffi::c_void, privs: *mut *mut ::core::ffi::c_void, serverprincname: *mut *mut u8, authnlevel: *mut u32, authnsvc: *mut u32, authzsvc: *mut u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingInqAuthClientA(clientbinding: *const ::core::ffi::c_void, privs: *mut *mut ::core::ffi::c_void, serverprincname: *mut *mut u8, authnlevel: *mut u32, authnsvc: *mut u32, authzsvc: *mut u32) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcBindingInqAuthClientA(::core::mem::transmute(clientbinding), ::core::mem::transmute(privs), ::core::mem::transmute(serverprincname), ::core::mem::transmute(authnlevel), ::core::mem::transmute(authnsvc), ::core::mem::transmute(authzsvc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcBindingInqAuthClientExA(clientbinding: *const ::core::ffi::c_void, privs: *mut *mut ::core::ffi::c_void, serverprincname: *mut *mut u8, authnlevel: *mut u32, authnsvc: *mut u32, authzsvc: *mut u32, flags: u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingInqAuthClientExA(clientbinding: *const ::core::ffi::c_void, privs: *mut *mut ::core::ffi::c_void, serverprincname: *mut *mut u8, authnlevel: *mut u32, authnsvc: *mut u32, authzsvc: *mut u32, flags: u32) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcBindingInqAuthClientExA(::core::mem::transmute(clientbinding), ::core::mem::transmute(privs), ::core::mem::transmute(serverprincname), ::core::mem::transmute(authnlevel), ::core::mem::transmute(authnsvc), ::core::mem::transmute(authzsvc), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcBindingInqAuthClientExW(clientbinding: *const ::core::ffi::c_void, privs: *mut *mut ::core::ffi::c_void, serverprincname: *mut *mut u16, authnlevel: *mut u32, authnsvc: *mut u32, authzsvc: *mut u32, flags: u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingInqAuthClientExW(clientbinding: *const ::core::ffi::c_void, privs: *mut *mut ::core::ffi::c_void, serverprincname: *mut *mut u16, authnlevel: *mut u32, authnsvc: *mut u32, authzsvc: *mut u32, flags: u32) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcBindingInqAuthClientExW(::core::mem::transmute(clientbinding), ::core::mem::transmute(privs), ::core::mem::transmute(serverprincname), ::core::mem::transmute(authnlevel), ::core::mem::transmute(authnsvc), ::core::mem::transmute(authzsvc), ::core::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcBindingInqAuthClientW(clientbinding: *const ::core::ffi::c_void, privs: *mut *mut ::core::ffi::c_void, serverprincname: *mut *mut u16, authnlevel: *mut u32, authnsvc: *mut u32, authzsvc: *mut u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingInqAuthClientW(clientbinding: *const ::core::ffi::c_void, privs: *mut *mut ::core::ffi::c_void, serverprincname: *mut *mut u16, authnlevel: *mut u32, authnsvc: *mut u32, authzsvc: *mut u32) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcBindingInqAuthClientW(::core::mem::transmute(clientbinding), ::core::mem::transmute(privs), ::core::mem::transmute(serverprincname), ::core::mem::transmute(authnlevel), ::core::mem::transmute(authnsvc), ::core::mem::transmute(authzsvc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcBindingInqAuthInfoA(binding: *const ::core::ffi::c_void, serverprincname: *mut *mut u8, authnlevel: *mut u32, authnsvc: *mut u32, authidentity: *mut *mut ::core::ffi::c_void, authzsvc: *mut u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingInqAuthInfoA(binding: *const ::core::ffi::c_void, serverprincname: *mut *mut u8, authnlevel: *mut u32, authnsvc: *mut u32, authidentity: *mut *mut ::core::ffi::c_void, authzsvc: *mut u32) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcBindingInqAuthInfoA(::core::mem::transmute(binding), ::core::mem::transmute(serverprincname), ::core::mem::transmute(authnlevel), ::core::mem::transmute(authnsvc), ::core::mem::transmute(authidentity), ::core::mem::transmute(authzsvc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn RpcBindingInqAuthInfoExA(binding: *const ::core::ffi::c_void, serverprincname: *mut *mut u8, authnlevel: *mut u32, authnsvc: *mut u32, authidentity: *mut *mut ::core::ffi::c_void, authzsvc: *mut u32, rpcqosversion: u32, securityqos: *mut RPC_SECURITY_QOS) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingInqAuthInfoExA(binding: *const ::core::ffi::c_void, serverprincname: *mut *mut u8, authnlevel: *mut u32, authnsvc: *mut u32, authidentity: *mut *mut ::core::ffi::c_void, authzsvc: *mut u32, rpcqosversion: u32, securityqos: *mut RPC_SECURITY_QOS) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcBindingInqAuthInfoExA(::core::mem::transmute(binding), ::core::mem::transmute(serverprincname), ::core::mem::transmute(authnlevel), ::core::mem::transmute(authnsvc), ::core::mem::transmute(authidentity), ::core::mem::transmute(authzsvc), ::core::mem::transmute(rpcqosversion), ::core::mem::transmute(securityqos)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn RpcBindingInqAuthInfoExW(binding: *const ::core::ffi::c_void, serverprincname: *mut *mut u16, authnlevel: *mut u32, authnsvc: *mut u32, authidentity: *mut *mut ::core::ffi::c_void, authzsvc: *mut u32, rpcqosversion: u32, securityqos: *mut RPC_SECURITY_QOS) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingInqAuthInfoExW(binding: *const ::core::ffi::c_void, serverprincname: *mut *mut u16, authnlevel: *mut u32, authnsvc: *mut u32, authidentity: *mut *mut ::core::ffi::c_void, authzsvc: *mut u32, rpcqosversion: u32, securityqos: *mut RPC_SECURITY_QOS) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcBindingInqAuthInfoExW(::core::mem::transmute(binding), ::core::mem::transmute(serverprincname), ::core::mem::transmute(authnlevel), ::core::mem::transmute(authnsvc), ::core::mem::transmute(authidentity), ::core::mem::transmute(authzsvc), ::core::mem::transmute(rpcqosversion), ::core::mem::transmute(securityqos)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcBindingInqAuthInfoW(binding: *const ::core::ffi::c_void, serverprincname: *mut *mut u16, authnlevel: *mut u32, authnsvc: *mut u32, authidentity: *mut *mut ::core::ffi::c_void, authzsvc: *mut u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingInqAuthInfoW(binding: *const ::core::ffi::c_void, serverprincname: *mut *mut u16, authnlevel: *mut u32, authnsvc: *mut u32, authidentity: *mut *mut ::core::ffi::c_void, authzsvc: *mut u32) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcBindingInqAuthInfoW(::core::mem::transmute(binding), ::core::mem::transmute(serverprincname), ::core::mem::transmute(authnlevel), ::core::mem::transmute(authnsvc), ::core::mem::transmute(authidentity), ::core::mem::transmute(authzsvc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcBindingInqMaxCalls(binding: *const ::core::ffi::c_void, maxcalls: *mut u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingInqMaxCalls(binding: *const ::core::ffi::c_void, maxcalls: *mut u32) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcBindingInqMaxCalls(::core::mem::transmute(binding), ::core::mem::transmute(maxcalls)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcBindingInqObject(binding: *const ::core::ffi::c_void, objectuuid: *mut ::windows::core::GUID) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingInqObject(binding: *const ::core::ffi::c_void, objectuuid: *mut ::windows::core::GUID) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcBindingInqObject(::core::mem::transmute(binding), ::core::mem::transmute(objectuuid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcBindingInqOption(hbinding: *const ::core::ffi::c_void, option: u32, poptionvalue: *mut usize) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingInqOption(hbinding: *const ::core::ffi::c_void, option: u32, poptionvalue: *mut usize) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcBindingInqOption(::core::mem::transmute(hbinding), ::core::mem::transmute(option), ::core::mem::transmute(poptionvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcBindingReset(binding: *const ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingReset(binding: *const ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcBindingReset(::core::mem::transmute(binding)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcBindingServerFromClient(clientbinding: *const ::core::ffi::c_void, serverbinding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingServerFromClient(clientbinding: *const ::core::ffi::c_void, serverbinding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcBindingServerFromClient(::core::mem::transmute(clientbinding), ::core::mem::transmute(serverbinding)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcBindingSetAuthInfoA(binding: *const ::core::ffi::c_void, serverprincname: *const u8, authnlevel: u32, authnsvc: u32, authidentity: *const ::core::ffi::c_void, authzsvc: u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingSetAuthInfoA(binding: *const ::core::ffi::c_void, serverprincname: *const u8, authnlevel: u32, authnsvc: u32, authidentity: *const ::core::ffi::c_void, authzsvc: u32) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcBindingSetAuthInfoA(::core::mem::transmute(binding), ::core::mem::transmute(serverprincname), ::core::mem::transmute(authnlevel), ::core::mem::transmute(authnsvc), ::core::mem::transmute(authidentity), ::core::mem::transmute(authzsvc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn RpcBindingSetAuthInfoExA(binding: *const ::core::ffi::c_void, serverprincname: *const u8, authnlevel: u32, authnsvc: u32, authidentity: *const ::core::ffi::c_void, authzsvc: u32, securityqos: *const RPC_SECURITY_QOS) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingSetAuthInfoExA(binding: *const ::core::ffi::c_void, serverprincname: *const u8, authnlevel: u32, authnsvc: u32, authidentity: *const ::core::ffi::c_void, authzsvc: u32, securityqos: *const RPC_SECURITY_QOS) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcBindingSetAuthInfoExA(::core::mem::transmute(binding), ::core::mem::transmute(serverprincname), ::core::mem::transmute(authnlevel), ::core::mem::transmute(authnsvc), ::core::mem::transmute(authidentity), ::core::mem::transmute(authzsvc), ::core::mem::transmute(securityqos)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn RpcBindingSetAuthInfoExW(binding: *const ::core::ffi::c_void, serverprincname: *const u16, authnlevel: u32, authnsvc: u32, authidentity: *const ::core::ffi::c_void, authzsvc: u32, securityqos: *const RPC_SECURITY_QOS) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingSetAuthInfoExW(binding: *const ::core::ffi::c_void, serverprincname: *const u16, authnlevel: u32, authnsvc: u32, authidentity: *const ::core::ffi::c_void, authzsvc: u32, securityqos: *const RPC_SECURITY_QOS) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcBindingSetAuthInfoExW(::core::mem::transmute(binding), ::core::mem::transmute(serverprincname), ::core::mem::transmute(authnlevel), ::core::mem::transmute(authnsvc), ::core::mem::transmute(authidentity), ::core::mem::transmute(authzsvc), ::core::mem::transmute(securityqos)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcBindingSetAuthInfoW(binding: *const ::core::ffi::c_void, serverprincname: *const u16, authnlevel: u32, authnsvc: u32, authidentity: *const ::core::ffi::c_void, authzsvc: u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingSetAuthInfoW(binding: *const ::core::ffi::c_void, serverprincname: *const u16, authnlevel: u32, authnsvc: u32, authidentity: *const ::core::ffi::c_void, authzsvc: u32) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcBindingSetAuthInfoW(::core::mem::transmute(binding), ::core::mem::transmute(serverprincname), ::core::mem::transmute(authnlevel), ::core::mem::transmute(authnsvc), ::core::mem::transmute(authidentity), ::core::mem::transmute(authzsvc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcBindingSetObject(binding: *const ::core::ffi::c_void, objectuuid: *const ::windows::core::GUID) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingSetObject(binding: *const ::core::ffi::c_void, objectuuid: *const ::windows::core::GUID) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcBindingSetObject(::core::mem::transmute(binding), ::core::mem::transmute(objectuuid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcBindingSetOption(hbinding: *const ::core::ffi::c_void, option: u32, optionvalue: usize) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingSetOption(hbinding: *const ::core::ffi::c_void, option: u32, optionvalue: usize) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcBindingSetOption(::core::mem::transmute(hbinding), ::core::mem::transmute(option), ::core::mem::transmute(optionvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcBindingToStringBindingA(binding: *const ::core::ffi::c_void, stringbinding: *mut *mut u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingToStringBindingA(binding: *const ::core::ffi::c_void, stringbinding: *mut *mut u8) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcBindingToStringBindingA(::core::mem::transmute(binding), ::core::mem::transmute(stringbinding)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcBindingToStringBindingW(binding: *const ::core::ffi::c_void, stringbinding: *mut *mut u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingToStringBindingW(binding: *const ::core::ffi::c_void, stringbinding: *mut *mut u16) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcBindingToStringBindingW(::core::mem::transmute(binding), ::core::mem::transmute(stringbinding)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcBindingUnbind(binding: *const ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingUnbind(binding: *const ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcBindingUnbind(::core::mem::transmute(binding)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcBindingVectorFree(bindingvector: *mut *mut RPC_BINDING_VECTOR) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingVectorFree(bindingvector: *mut *mut RPC_BINDING_VECTOR) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcBindingVectorFree(::core::mem::transmute(bindingvector)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RpcCallClientLocality(pub i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const rcclInvalid: RpcCallClientLocality = RpcCallClientLocality(0i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const rcclLocal: RpcCallClientLocality = RpcCallClientLocality(1i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const rcclRemote: RpcCallClientLocality = RpcCallClientLocality(2i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const rcclClientUnknownLocality: RpcCallClientLocality = RpcCallClientLocality(3i32);
impl ::core::marker::Copy for RpcCallClientLocality {}
impl ::core::clone::Clone for RpcCallClientLocality {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RpcCallClientLocality {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RpcCallClientLocality {
    type Abi = Self;
}
impl ::core::fmt::Debug for RpcCallClientLocality {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RpcCallClientLocality").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RpcCallType(pub i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const rctInvalid: RpcCallType = RpcCallType(0i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const rctNormal: RpcCallType = RpcCallType(1i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const rctTraining: RpcCallType = RpcCallType(2i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const rctGuaranteed: RpcCallType = RpcCallType(3i32);
impl ::core::marker::Copy for RpcCallType {}
impl ::core::clone::Clone for RpcCallType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RpcCallType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RpcCallType {
    type Abi = Self;
}
impl ::core::fmt::Debug for RpcCallType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RpcCallType").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcCancelThread(thread: *const ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcCancelThread(thread: *const ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcCancelThread(::core::mem::transmute(thread)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcCancelThreadEx(thread: *const ::core::ffi::c_void, timeout: i32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcCancelThreadEx(thread: *const ::core::ffi::c_void, timeout: i32) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcCancelThreadEx(::core::mem::transmute(thread), ::core::mem::transmute(timeout)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn RpcCertGeneratePrincipalNameA(context: *const super::super::Security::Cryptography::CERT_CONTEXT, flags: u32, pbuffer: *mut *mut u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcCertGeneratePrincipalNameA(context: *const super::super::Security::Cryptography::CERT_CONTEXT, flags: u32, pbuffer: *mut *mut u8) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcCertGeneratePrincipalNameA(::core::mem::transmute(context), ::core::mem::transmute(flags), ::core::mem::transmute(pbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`, `\"Win32_Security_Cryptography\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn RpcCertGeneratePrincipalNameW(context: *const super::super::Security::Cryptography::CERT_CONTEXT, flags: u32, pbuffer: *mut *mut u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcCertGeneratePrincipalNameW(context: *const super::super::Security::Cryptography::CERT_CONTEXT, flags: u32, pbuffer: *mut *mut u16) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcCertGeneratePrincipalNameW(::core::mem::transmute(context), ::core::mem::transmute(flags), ::core::mem::transmute(pbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcEpRegisterA(ifspec: *const ::core::ffi::c_void, bindingvector: *const RPC_BINDING_VECTOR, uuidvector: *const UUID_VECTOR, annotation: *const u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcEpRegisterA(ifspec: *const ::core::ffi::c_void, bindingvector: *const RPC_BINDING_VECTOR, uuidvector: *const UUID_VECTOR, annotation: *const u8) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcEpRegisterA(::core::mem::transmute(ifspec), ::core::mem::transmute(bindingvector), ::core::mem::transmute(uuidvector), ::core::mem::transmute(annotation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcEpRegisterNoReplaceA(ifspec: *const ::core::ffi::c_void, bindingvector: *const RPC_BINDING_VECTOR, uuidvector: *const UUID_VECTOR, annotation: *const u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcEpRegisterNoReplaceA(ifspec: *const ::core::ffi::c_void, bindingvector: *const RPC_BINDING_VECTOR, uuidvector: *const UUID_VECTOR, annotation: *const u8) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcEpRegisterNoReplaceA(::core::mem::transmute(ifspec), ::core::mem::transmute(bindingvector), ::core::mem::transmute(uuidvector), ::core::mem::transmute(annotation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcEpRegisterNoReplaceW(ifspec: *const ::core::ffi::c_void, bindingvector: *const RPC_BINDING_VECTOR, uuidvector: *const UUID_VECTOR, annotation: *const u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcEpRegisterNoReplaceW(ifspec: *const ::core::ffi::c_void, bindingvector: *const RPC_BINDING_VECTOR, uuidvector: *const UUID_VECTOR, annotation: *const u16) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcEpRegisterNoReplaceW(::core::mem::transmute(ifspec), ::core::mem::transmute(bindingvector), ::core::mem::transmute(uuidvector), ::core::mem::transmute(annotation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcEpRegisterW(ifspec: *const ::core::ffi::c_void, bindingvector: *const RPC_BINDING_VECTOR, uuidvector: *const UUID_VECTOR, annotation: *const u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcEpRegisterW(ifspec: *const ::core::ffi::c_void, bindingvector: *const RPC_BINDING_VECTOR, uuidvector: *const UUID_VECTOR, annotation: *const u16) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcEpRegisterW(::core::mem::transmute(ifspec), ::core::mem::transmute(bindingvector), ::core::mem::transmute(uuidvector), ::core::mem::transmute(annotation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcEpResolveBinding(binding: *const ::core::ffi::c_void, ifspec: *const ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcEpResolveBinding(binding: *const ::core::ffi::c_void, ifspec: *const ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcEpResolveBinding(::core::mem::transmute(binding), ::core::mem::transmute(ifspec)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcEpUnregister(ifspec: *const ::core::ffi::c_void, bindingvector: *const RPC_BINDING_VECTOR, uuidvector: *const UUID_VECTOR) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcEpUnregister(ifspec: *const ::core::ffi::c_void, bindingvector: *const RPC_BINDING_VECTOR, uuidvector: *const UUID_VECTOR) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcEpUnregister(::core::mem::transmute(ifspec), ::core::mem::transmute(bindingvector), ::core::mem::transmute(uuidvector)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RpcErrorAddRecord(errorinfo: *const RPC_EXTENDED_ERROR_INFO) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcErrorAddRecord(errorinfo: *const RPC_EXTENDED_ERROR_INFO) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcErrorAddRecord(::core::mem::transmute(errorinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcErrorClearInformation() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcErrorClearInformation();
        }
        RpcErrorClearInformation()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcErrorEndEnumeration(enumhandle: *mut RPC_ERROR_ENUM_HANDLE) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcErrorEndEnumeration(enumhandle: *mut RPC_ERROR_ENUM_HANDLE) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcErrorEndEnumeration(::core::mem::transmute(enumhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RpcErrorGetNextRecord<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(enumhandle: *const RPC_ERROR_ENUM_HANDLE, copystrings: Param1, errorinfo: *mut RPC_EXTENDED_ERROR_INFO) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcErrorGetNextRecord(enumhandle: *const RPC_ERROR_ENUM_HANDLE, copystrings: super::super::Foundation::BOOL, errorinfo: *mut RPC_EXTENDED_ERROR_INFO) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcErrorGetNextRecord(::core::mem::transmute(enumhandle), copystrings.into_param().abi(), ::core::mem::transmute(errorinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcErrorGetNumberOfRecords(enumhandle: *const RPC_ERROR_ENUM_HANDLE, records: *mut i32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcErrorGetNumberOfRecords(enumhandle: *const RPC_ERROR_ENUM_HANDLE, records: *mut i32) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcErrorGetNumberOfRecords(::core::mem::transmute(enumhandle), ::core::mem::transmute(records)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcErrorLoadErrorInfo(errorblob: *const ::core::ffi::c_void, blobsize: usize, enumhandle: *mut RPC_ERROR_ENUM_HANDLE) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcErrorLoadErrorInfo(errorblob: *const ::core::ffi::c_void, blobsize: usize, enumhandle: *mut RPC_ERROR_ENUM_HANDLE) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcErrorLoadErrorInfo(::core::mem::transmute(errorblob), ::core::mem::transmute(blobsize), ::core::mem::transmute(enumhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcErrorResetEnumeration(enumhandle: *mut RPC_ERROR_ENUM_HANDLE) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcErrorResetEnumeration(enumhandle: *mut RPC_ERROR_ENUM_HANDLE) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcErrorResetEnumeration(::core::mem::transmute(enumhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcErrorSaveErrorInfo(enumhandle: *const RPC_ERROR_ENUM_HANDLE, errorblob: *mut *mut ::core::ffi::c_void, blobsize: *mut usize) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcErrorSaveErrorInfo(enumhandle: *const RPC_ERROR_ENUM_HANDLE, errorblob: *mut *mut ::core::ffi::c_void, blobsize: *mut usize) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcErrorSaveErrorInfo(::core::mem::transmute(enumhandle), ::core::mem::transmute(errorblob), ::core::mem::transmute(blobsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcErrorStartEnumeration(enumhandle: *mut RPC_ERROR_ENUM_HANDLE) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcErrorStartEnumeration(enumhandle: *mut RPC_ERROR_ENUM_HANDLE) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcErrorStartEnumeration(::core::mem::transmute(enumhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcExceptionFilter(exceptioncode: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcExceptionFilter(exceptioncode: u32) -> i32;
        }
        ::core::mem::transmute(RpcExceptionFilter(::core::mem::transmute(exceptioncode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcFreeAuthorizationContext(pauthzclientcontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcFreeAuthorizationContext(pauthzclientcontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcFreeAuthorizationContext(::core::mem::transmute(pauthzclientcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RpcGetAuthorizationContextForClient<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param4: ::windows::core::IntoParam<'a, super::super::Foundation::LUID>>(clientbinding: *const ::core::ffi::c_void, impersonateonreturn: Param1, reserved1: *const ::core::ffi::c_void, pexpirationtime: *const i64, reserved2: Param4, reserved3: u32, reserved4: *const ::core::ffi::c_void, pauthzclientcontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcGetAuthorizationContextForClient(clientbinding: *const ::core::ffi::c_void, impersonateonreturn: super::super::Foundation::BOOL, reserved1: *const ::core::ffi::c_void, pexpirationtime: *const i64, reserved2: super::super::Foundation::LUID, reserved3: u32, reserved4: *const ::core::ffi::c_void, pauthzclientcontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcGetAuthorizationContextForClient(::core::mem::transmute(clientbinding), impersonateonreturn.into_param().abi(), ::core::mem::transmute(reserved1), ::core::mem::transmute(pexpirationtime), reserved2.into_param().abi(), ::core::mem::transmute(reserved3), ::core::mem::transmute(reserved4), ::core::mem::transmute(pauthzclientcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcIfIdVectorFree(ifidvector: *mut *mut RPC_IF_ID_VECTOR) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcIfIdVectorFree(ifidvector: *mut *mut RPC_IF_ID_VECTOR) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcIfIdVectorFree(::core::mem::transmute(ifidvector)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcIfInqId(rpcifhandle: *const ::core::ffi::c_void, rpcifid: *mut RPC_IF_ID) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcIfInqId(rpcifhandle: *const ::core::ffi::c_void, rpcifid: *mut RPC_IF_ID) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcIfInqId(::core::mem::transmute(rpcifhandle), ::core::mem::transmute(rpcifid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcImpersonateClient(bindinghandle: *const ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcImpersonateClient(bindinghandle: *const ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcImpersonateClient(::core::mem::transmute(bindinghandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcImpersonateClient2(bindinghandle: *const ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcImpersonateClient2(bindinghandle: *const ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcImpersonateClient2(::core::mem::transmute(bindinghandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcImpersonateClientContainer(bindinghandle: *const ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcImpersonateClientContainer(bindinghandle: *const ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcImpersonateClientContainer(::core::mem::transmute(bindinghandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RpcLocalAddressFormat(pub i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const rlafInvalid: RpcLocalAddressFormat = RpcLocalAddressFormat(0i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const rlafIPv4: RpcLocalAddressFormat = RpcLocalAddressFormat(1i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const rlafIPv6: RpcLocalAddressFormat = RpcLocalAddressFormat(2i32);
impl ::core::marker::Copy for RpcLocalAddressFormat {}
impl ::core::clone::Clone for RpcLocalAddressFormat {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RpcLocalAddressFormat {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RpcLocalAddressFormat {
    type Abi = Self;
}
impl ::core::fmt::Debug for RpcLocalAddressFormat {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RpcLocalAddressFormat").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcMgmtEnableIdleCleanup() -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcMgmtEnableIdleCleanup() -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcMgmtEnableIdleCleanup())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcMgmtEpEltInqBegin(epbinding: *const ::core::ffi::c_void, inquirytype: u32, ifid: *const RPC_IF_ID, versoption: u32, objectuuid: *const ::windows::core::GUID, inquirycontext: *mut *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcMgmtEpEltInqBegin(epbinding: *const ::core::ffi::c_void, inquirytype: u32, ifid: *const RPC_IF_ID, versoption: u32, objectuuid: *const ::windows::core::GUID, inquirycontext: *mut *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcMgmtEpEltInqBegin(::core::mem::transmute(epbinding), ::core::mem::transmute(inquirytype), ::core::mem::transmute(ifid), ::core::mem::transmute(versoption), ::core::mem::transmute(objectuuid), ::core::mem::transmute(inquirycontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcMgmtEpEltInqDone(inquirycontext: *mut *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcMgmtEpEltInqDone(inquirycontext: *mut *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcMgmtEpEltInqDone(::core::mem::transmute(inquirycontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcMgmtEpEltInqNextA(inquirycontext: *const *const ::core::ffi::c_void, ifid: *mut RPC_IF_ID, binding: *mut *mut ::core::ffi::c_void, objectuuid: *mut ::windows::core::GUID, annotation: *mut *mut u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcMgmtEpEltInqNextA(inquirycontext: *const *const ::core::ffi::c_void, ifid: *mut RPC_IF_ID, binding: *mut *mut ::core::ffi::c_void, objectuuid: *mut ::windows::core::GUID, annotation: *mut *mut u8) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcMgmtEpEltInqNextA(::core::mem::transmute(inquirycontext), ::core::mem::transmute(ifid), ::core::mem::transmute(binding), ::core::mem::transmute(objectuuid), ::core::mem::transmute(annotation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcMgmtEpEltInqNextW(inquirycontext: *const *const ::core::ffi::c_void, ifid: *mut RPC_IF_ID, binding: *mut *mut ::core::ffi::c_void, objectuuid: *mut ::windows::core::GUID, annotation: *mut *mut u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcMgmtEpEltInqNextW(inquirycontext: *const *const ::core::ffi::c_void, ifid: *mut RPC_IF_ID, binding: *mut *mut ::core::ffi::c_void, objectuuid: *mut ::windows::core::GUID, annotation: *mut *mut u16) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcMgmtEpEltInqNextW(::core::mem::transmute(inquirycontext), ::core::mem::transmute(ifid), ::core::mem::transmute(binding), ::core::mem::transmute(objectuuid), ::core::mem::transmute(annotation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcMgmtEpUnregister(epbinding: *const ::core::ffi::c_void, ifid: *const RPC_IF_ID, binding: *const ::core::ffi::c_void, objectuuid: *const ::windows::core::GUID) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcMgmtEpUnregister(epbinding: *const ::core::ffi::c_void, ifid: *const RPC_IF_ID, binding: *const ::core::ffi::c_void, objectuuid: *const ::windows::core::GUID) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcMgmtEpUnregister(::core::mem::transmute(epbinding), ::core::mem::transmute(ifid), ::core::mem::transmute(binding), ::core::mem::transmute(objectuuid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcMgmtInqComTimeout(binding: *const ::core::ffi::c_void, timeout: *mut u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcMgmtInqComTimeout(binding: *const ::core::ffi::c_void, timeout: *mut u32) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcMgmtInqComTimeout(::core::mem::transmute(binding), ::core::mem::transmute(timeout)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcMgmtInqDefaultProtectLevel(authnsvc: u32, authnlevel: *mut u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcMgmtInqDefaultProtectLevel(authnsvc: u32, authnlevel: *mut u32) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcMgmtInqDefaultProtectLevel(::core::mem::transmute(authnsvc), ::core::mem::transmute(authnlevel)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcMgmtInqIfIds(binding: *const ::core::ffi::c_void, ifidvector: *mut *mut RPC_IF_ID_VECTOR) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcMgmtInqIfIds(binding: *const ::core::ffi::c_void, ifidvector: *mut *mut RPC_IF_ID_VECTOR) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcMgmtInqIfIds(::core::mem::transmute(binding), ::core::mem::transmute(ifidvector)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcMgmtInqServerPrincNameA(binding: *const ::core::ffi::c_void, authnsvc: u32, serverprincname: *mut *mut u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcMgmtInqServerPrincNameA(binding: *const ::core::ffi::c_void, authnsvc: u32, serverprincname: *mut *mut u8) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcMgmtInqServerPrincNameA(::core::mem::transmute(binding), ::core::mem::transmute(authnsvc), ::core::mem::transmute(serverprincname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcMgmtInqServerPrincNameW(binding: *const ::core::ffi::c_void, authnsvc: u32, serverprincname: *mut *mut u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcMgmtInqServerPrincNameW(binding: *const ::core::ffi::c_void, authnsvc: u32, serverprincname: *mut *mut u16) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcMgmtInqServerPrincNameW(::core::mem::transmute(binding), ::core::mem::transmute(authnsvc), ::core::mem::transmute(serverprincname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcMgmtInqStats(binding: *const ::core::ffi::c_void, statistics: *mut *mut RPC_STATS_VECTOR) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcMgmtInqStats(binding: *const ::core::ffi::c_void, statistics: *mut *mut RPC_STATS_VECTOR) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcMgmtInqStats(::core::mem::transmute(binding), ::core::mem::transmute(statistics)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcMgmtIsServerListening(binding: *const ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcMgmtIsServerListening(binding: *const ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcMgmtIsServerListening(::core::mem::transmute(binding)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcMgmtSetAuthorizationFn(authorizationfn: RPC_MGMT_AUTHORIZATION_FN) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcMgmtSetAuthorizationFn(authorizationfn: ::windows::core::RawPtr) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcMgmtSetAuthorizationFn(::core::mem::transmute(authorizationfn)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcMgmtSetCancelTimeout(timeout: i32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcMgmtSetCancelTimeout(timeout: i32) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcMgmtSetCancelTimeout(::core::mem::transmute(timeout)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcMgmtSetComTimeout(binding: *const ::core::ffi::c_void, timeout: u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcMgmtSetComTimeout(binding: *const ::core::ffi::c_void, timeout: u32) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcMgmtSetComTimeout(::core::mem::transmute(binding), ::core::mem::transmute(timeout)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcMgmtSetServerStackSize(threadstacksize: u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcMgmtSetServerStackSize(threadstacksize: u32) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcMgmtSetServerStackSize(::core::mem::transmute(threadstacksize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcMgmtStatsVectorFree(statsvector: *mut *mut RPC_STATS_VECTOR) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcMgmtStatsVectorFree(statsvector: *mut *mut RPC_STATS_VECTOR) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcMgmtStatsVectorFree(::core::mem::transmute(statsvector)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcMgmtStopServerListening(binding: *const ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcMgmtStopServerListening(binding: *const ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcMgmtStopServerListening(::core::mem::transmute(binding)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcMgmtWaitServerListen() -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcMgmtWaitServerListen() -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcMgmtWaitServerListen())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNetworkInqProtseqsA(protseqvector: *mut *mut RPC_PROTSEQ_VECTORA) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNetworkInqProtseqsA(protseqvector: *mut *mut RPC_PROTSEQ_VECTORA) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNetworkInqProtseqsA(::core::mem::transmute(protseqvector)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNetworkInqProtseqsW(protseqvector: *mut *mut RPC_PROTSEQ_VECTORW) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNetworkInqProtseqsW(protseqvector: *mut *mut RPC_PROTSEQ_VECTORW) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNetworkInqProtseqsW(::core::mem::transmute(protseqvector)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNetworkIsProtseqValidA(protseq: *const u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNetworkIsProtseqValidA(protseq: *const u8) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNetworkIsProtseqValidA(::core::mem::transmute(protseq)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNetworkIsProtseqValidW(protseq: *const u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNetworkIsProtseqValidW(protseq: *const u16) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNetworkIsProtseqValidW(::core::mem::transmute(protseq)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsBindingExportA(entrynamesyntax: u32, entryname: *const u8, ifspec: *const ::core::ffi::c_void, bindingvec: *const RPC_BINDING_VECTOR, objectuuidvec: *const UUID_VECTOR) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsBindingExportA(entrynamesyntax: u32, entryname: *const u8, ifspec: *const ::core::ffi::c_void, bindingvec: *const RPC_BINDING_VECTOR, objectuuidvec: *const UUID_VECTOR) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsBindingExportA(::core::mem::transmute(entrynamesyntax), ::core::mem::transmute(entryname), ::core::mem::transmute(ifspec), ::core::mem::transmute(bindingvec), ::core::mem::transmute(objectuuidvec)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsBindingExportPnPA(entrynamesyntax: u32, entryname: *const u8, ifspec: *const ::core::ffi::c_void, objectvector: *const UUID_VECTOR) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsBindingExportPnPA(entrynamesyntax: u32, entryname: *const u8, ifspec: *const ::core::ffi::c_void, objectvector: *const UUID_VECTOR) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsBindingExportPnPA(::core::mem::transmute(entrynamesyntax), ::core::mem::transmute(entryname), ::core::mem::transmute(ifspec), ::core::mem::transmute(objectvector)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsBindingExportPnPW(entrynamesyntax: u32, entryname: *const u16, ifspec: *const ::core::ffi::c_void, objectvector: *const UUID_VECTOR) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsBindingExportPnPW(entrynamesyntax: u32, entryname: *const u16, ifspec: *const ::core::ffi::c_void, objectvector: *const UUID_VECTOR) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsBindingExportPnPW(::core::mem::transmute(entrynamesyntax), ::core::mem::transmute(entryname), ::core::mem::transmute(ifspec), ::core::mem::transmute(objectvector)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsBindingExportW(entrynamesyntax: u32, entryname: *const u16, ifspec: *const ::core::ffi::c_void, bindingvec: *const RPC_BINDING_VECTOR, objectuuidvec: *const UUID_VECTOR) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsBindingExportW(entrynamesyntax: u32, entryname: *const u16, ifspec: *const ::core::ffi::c_void, bindingvec: *const RPC_BINDING_VECTOR, objectuuidvec: *const UUID_VECTOR) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsBindingExportW(::core::mem::transmute(entrynamesyntax), ::core::mem::transmute(entryname), ::core::mem::transmute(ifspec), ::core::mem::transmute(bindingvec), ::core::mem::transmute(objectuuidvec)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsBindingImportBeginA(entrynamesyntax: u32, entryname: *const u8, ifspec: *const ::core::ffi::c_void, objuuid: *const ::windows::core::GUID, importcontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsBindingImportBeginA(entrynamesyntax: u32, entryname: *const u8, ifspec: *const ::core::ffi::c_void, objuuid: *const ::windows::core::GUID, importcontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsBindingImportBeginA(::core::mem::transmute(entrynamesyntax), ::core::mem::transmute(entryname), ::core::mem::transmute(ifspec), ::core::mem::transmute(objuuid), ::core::mem::transmute(importcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsBindingImportBeginW(entrynamesyntax: u32, entryname: *const u16, ifspec: *const ::core::ffi::c_void, objuuid: *const ::windows::core::GUID, importcontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsBindingImportBeginW(entrynamesyntax: u32, entryname: *const u16, ifspec: *const ::core::ffi::c_void, objuuid: *const ::windows::core::GUID, importcontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsBindingImportBeginW(::core::mem::transmute(entrynamesyntax), ::core::mem::transmute(entryname), ::core::mem::transmute(ifspec), ::core::mem::transmute(objuuid), ::core::mem::transmute(importcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsBindingImportDone(importcontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsBindingImportDone(importcontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsBindingImportDone(::core::mem::transmute(importcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsBindingImportNext(importcontext: *mut ::core::ffi::c_void, binding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsBindingImportNext(importcontext: *mut ::core::ffi::c_void, binding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsBindingImportNext(::core::mem::transmute(importcontext), ::core::mem::transmute(binding)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsBindingInqEntryNameA(binding: *const ::core::ffi::c_void, entrynamesyntax: u32, entryname: *mut *mut u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsBindingInqEntryNameA(binding: *const ::core::ffi::c_void, entrynamesyntax: u32, entryname: *mut *mut u8) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsBindingInqEntryNameA(::core::mem::transmute(binding), ::core::mem::transmute(entrynamesyntax), ::core::mem::transmute(entryname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsBindingInqEntryNameW(binding: *const ::core::ffi::c_void, entrynamesyntax: u32, entryname: *mut *mut u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsBindingInqEntryNameW(binding: *const ::core::ffi::c_void, entrynamesyntax: u32, entryname: *mut *mut u16) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsBindingInqEntryNameW(::core::mem::transmute(binding), ::core::mem::transmute(entrynamesyntax), ::core::mem::transmute(entryname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsBindingLookupBeginA(entrynamesyntax: u32, entryname: *const u8, ifspec: *const ::core::ffi::c_void, objuuid: *const ::windows::core::GUID, bindingmaxcount: u32, lookupcontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsBindingLookupBeginA(entrynamesyntax: u32, entryname: *const u8, ifspec: *const ::core::ffi::c_void, objuuid: *const ::windows::core::GUID, bindingmaxcount: u32, lookupcontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsBindingLookupBeginA(::core::mem::transmute(entrynamesyntax), ::core::mem::transmute(entryname), ::core::mem::transmute(ifspec), ::core::mem::transmute(objuuid), ::core::mem::transmute(bindingmaxcount), ::core::mem::transmute(lookupcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsBindingLookupBeginW(entrynamesyntax: u32, entryname: *const u16, ifspec: *const ::core::ffi::c_void, objuuid: *const ::windows::core::GUID, bindingmaxcount: u32, lookupcontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsBindingLookupBeginW(entrynamesyntax: u32, entryname: *const u16, ifspec: *const ::core::ffi::c_void, objuuid: *const ::windows::core::GUID, bindingmaxcount: u32, lookupcontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsBindingLookupBeginW(::core::mem::transmute(entrynamesyntax), ::core::mem::transmute(entryname), ::core::mem::transmute(ifspec), ::core::mem::transmute(objuuid), ::core::mem::transmute(bindingmaxcount), ::core::mem::transmute(lookupcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsBindingLookupDone(lookupcontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsBindingLookupDone(lookupcontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsBindingLookupDone(::core::mem::transmute(lookupcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsBindingLookupNext(lookupcontext: *mut ::core::ffi::c_void, bindingvec: *mut *mut RPC_BINDING_VECTOR) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsBindingLookupNext(lookupcontext: *mut ::core::ffi::c_void, bindingvec: *mut *mut RPC_BINDING_VECTOR) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsBindingLookupNext(::core::mem::transmute(lookupcontext), ::core::mem::transmute(bindingvec)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsBindingSelect(bindingvec: *mut RPC_BINDING_VECTOR, binding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsBindingSelect(bindingvec: *mut RPC_BINDING_VECTOR, binding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsBindingSelect(::core::mem::transmute(bindingvec), ::core::mem::transmute(binding)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsBindingUnexportA(entrynamesyntax: u32, entryname: *const u8, ifspec: *const ::core::ffi::c_void, objectuuidvec: *const UUID_VECTOR) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsBindingUnexportA(entrynamesyntax: u32, entryname: *const u8, ifspec: *const ::core::ffi::c_void, objectuuidvec: *const UUID_VECTOR) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsBindingUnexportA(::core::mem::transmute(entrynamesyntax), ::core::mem::transmute(entryname), ::core::mem::transmute(ifspec), ::core::mem::transmute(objectuuidvec)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsBindingUnexportPnPA(entrynamesyntax: u32, entryname: *const u8, ifspec: *const ::core::ffi::c_void, objectvector: *const UUID_VECTOR) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsBindingUnexportPnPA(entrynamesyntax: u32, entryname: *const u8, ifspec: *const ::core::ffi::c_void, objectvector: *const UUID_VECTOR) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsBindingUnexportPnPA(::core::mem::transmute(entrynamesyntax), ::core::mem::transmute(entryname), ::core::mem::transmute(ifspec), ::core::mem::transmute(objectvector)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsBindingUnexportPnPW(entrynamesyntax: u32, entryname: *const u16, ifspec: *const ::core::ffi::c_void, objectvector: *const UUID_VECTOR) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsBindingUnexportPnPW(entrynamesyntax: u32, entryname: *const u16, ifspec: *const ::core::ffi::c_void, objectvector: *const UUID_VECTOR) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsBindingUnexportPnPW(::core::mem::transmute(entrynamesyntax), ::core::mem::transmute(entryname), ::core::mem::transmute(ifspec), ::core::mem::transmute(objectvector)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsBindingUnexportW(entrynamesyntax: u32, entryname: *const u16, ifspec: *const ::core::ffi::c_void, objectuuidvec: *const UUID_VECTOR) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsBindingUnexportW(entrynamesyntax: u32, entryname: *const u16, ifspec: *const ::core::ffi::c_void, objectuuidvec: *const UUID_VECTOR) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsBindingUnexportW(::core::mem::transmute(entrynamesyntax), ::core::mem::transmute(entryname), ::core::mem::transmute(ifspec), ::core::mem::transmute(objectuuidvec)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsEntryExpandNameA(entrynamesyntax: u32, entryname: *const u8, expandedname: *mut *mut u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsEntryExpandNameA(entrynamesyntax: u32, entryname: *const u8, expandedname: *mut *mut u8) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsEntryExpandNameA(::core::mem::transmute(entrynamesyntax), ::core::mem::transmute(entryname), ::core::mem::transmute(expandedname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsEntryExpandNameW(entrynamesyntax: u32, entryname: *const u16, expandedname: *mut *mut u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsEntryExpandNameW(entrynamesyntax: u32, entryname: *const u16, expandedname: *mut *mut u16) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsEntryExpandNameW(::core::mem::transmute(entrynamesyntax), ::core::mem::transmute(entryname), ::core::mem::transmute(expandedname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsEntryObjectInqBeginA(entrynamesyntax: u32, entryname: *const u8, inquirycontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsEntryObjectInqBeginA(entrynamesyntax: u32, entryname: *const u8, inquirycontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsEntryObjectInqBeginA(::core::mem::transmute(entrynamesyntax), ::core::mem::transmute(entryname), ::core::mem::transmute(inquirycontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsEntryObjectInqBeginW(entrynamesyntax: u32, entryname: *const u16, inquirycontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsEntryObjectInqBeginW(entrynamesyntax: u32, entryname: *const u16, inquirycontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsEntryObjectInqBeginW(::core::mem::transmute(entrynamesyntax), ::core::mem::transmute(entryname), ::core::mem::transmute(inquirycontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsEntryObjectInqDone(inquirycontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsEntryObjectInqDone(inquirycontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsEntryObjectInqDone(::core::mem::transmute(inquirycontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsEntryObjectInqNext(inquirycontext: *mut ::core::ffi::c_void, objuuid: *mut ::windows::core::GUID) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsEntryObjectInqNext(inquirycontext: *mut ::core::ffi::c_void, objuuid: *mut ::windows::core::GUID) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsEntryObjectInqNext(::core::mem::transmute(inquirycontext), ::core::mem::transmute(objuuid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsGroupDeleteA(groupnamesyntax: GROUP_NAME_SYNTAX, groupname: *const u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsGroupDeleteA(groupnamesyntax: GROUP_NAME_SYNTAX, groupname: *const u8) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsGroupDeleteA(::core::mem::transmute(groupnamesyntax), ::core::mem::transmute(groupname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsGroupDeleteW(groupnamesyntax: GROUP_NAME_SYNTAX, groupname: *const u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsGroupDeleteW(groupnamesyntax: GROUP_NAME_SYNTAX, groupname: *const u16) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsGroupDeleteW(::core::mem::transmute(groupnamesyntax), ::core::mem::transmute(groupname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsGroupMbrAddA(groupnamesyntax: u32, groupname: *const u8, membernamesyntax: u32, membername: *const u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsGroupMbrAddA(groupnamesyntax: u32, groupname: *const u8, membernamesyntax: u32, membername: *const u8) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsGroupMbrAddA(::core::mem::transmute(groupnamesyntax), ::core::mem::transmute(groupname), ::core::mem::transmute(membernamesyntax), ::core::mem::transmute(membername)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsGroupMbrAddW(groupnamesyntax: u32, groupname: *const u16, membernamesyntax: u32, membername: *const u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsGroupMbrAddW(groupnamesyntax: u32, groupname: *const u16, membernamesyntax: u32, membername: *const u16) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsGroupMbrAddW(::core::mem::transmute(groupnamesyntax), ::core::mem::transmute(groupname), ::core::mem::transmute(membernamesyntax), ::core::mem::transmute(membername)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsGroupMbrInqBeginA(groupnamesyntax: u32, groupname: *const u8, membernamesyntax: u32, inquirycontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsGroupMbrInqBeginA(groupnamesyntax: u32, groupname: *const u8, membernamesyntax: u32, inquirycontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsGroupMbrInqBeginA(::core::mem::transmute(groupnamesyntax), ::core::mem::transmute(groupname), ::core::mem::transmute(membernamesyntax), ::core::mem::transmute(inquirycontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsGroupMbrInqBeginW(groupnamesyntax: u32, groupname: *const u16, membernamesyntax: u32, inquirycontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsGroupMbrInqBeginW(groupnamesyntax: u32, groupname: *const u16, membernamesyntax: u32, inquirycontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsGroupMbrInqBeginW(::core::mem::transmute(groupnamesyntax), ::core::mem::transmute(groupname), ::core::mem::transmute(membernamesyntax), ::core::mem::transmute(inquirycontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsGroupMbrInqDone(inquirycontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsGroupMbrInqDone(inquirycontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsGroupMbrInqDone(::core::mem::transmute(inquirycontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsGroupMbrInqNextA(inquirycontext: *mut ::core::ffi::c_void, membername: *mut *mut u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsGroupMbrInqNextA(inquirycontext: *mut ::core::ffi::c_void, membername: *mut *mut u8) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsGroupMbrInqNextA(::core::mem::transmute(inquirycontext), ::core::mem::transmute(membername)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsGroupMbrInqNextW(inquirycontext: *mut ::core::ffi::c_void, membername: *mut *mut u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsGroupMbrInqNextW(inquirycontext: *mut ::core::ffi::c_void, membername: *mut *mut u16) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsGroupMbrInqNextW(::core::mem::transmute(inquirycontext), ::core::mem::transmute(membername)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsGroupMbrRemoveA(groupnamesyntax: u32, groupname: *const u8, membernamesyntax: u32, membername: *const u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsGroupMbrRemoveA(groupnamesyntax: u32, groupname: *const u8, membernamesyntax: u32, membername: *const u8) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsGroupMbrRemoveA(::core::mem::transmute(groupnamesyntax), ::core::mem::transmute(groupname), ::core::mem::transmute(membernamesyntax), ::core::mem::transmute(membername)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsGroupMbrRemoveW(groupnamesyntax: u32, groupname: *const u16, membernamesyntax: u32, membername: *const u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsGroupMbrRemoveW(groupnamesyntax: u32, groupname: *const u16, membernamesyntax: u32, membername: *const u16) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsGroupMbrRemoveW(::core::mem::transmute(groupnamesyntax), ::core::mem::transmute(groupname), ::core::mem::transmute(membernamesyntax), ::core::mem::transmute(membername)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsMgmtBindingUnexportA(entrynamesyntax: u32, entryname: *const u8, ifid: *const RPC_IF_ID, versoption: u32, objectuuidvec: *const UUID_VECTOR) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsMgmtBindingUnexportA(entrynamesyntax: u32, entryname: *const u8, ifid: *const RPC_IF_ID, versoption: u32, objectuuidvec: *const UUID_VECTOR) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsMgmtBindingUnexportA(::core::mem::transmute(entrynamesyntax), ::core::mem::transmute(entryname), ::core::mem::transmute(ifid), ::core::mem::transmute(versoption), ::core::mem::transmute(objectuuidvec)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsMgmtBindingUnexportW(entrynamesyntax: u32, entryname: *const u16, ifid: *const RPC_IF_ID, versoption: u32, objectuuidvec: *const UUID_VECTOR) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsMgmtBindingUnexportW(entrynamesyntax: u32, entryname: *const u16, ifid: *const RPC_IF_ID, versoption: u32, objectuuidvec: *const UUID_VECTOR) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsMgmtBindingUnexportW(::core::mem::transmute(entrynamesyntax), ::core::mem::transmute(entryname), ::core::mem::transmute(ifid), ::core::mem::transmute(versoption), ::core::mem::transmute(objectuuidvec)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsMgmtEntryCreateA(entrynamesyntax: u32, entryname: *const u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsMgmtEntryCreateA(entrynamesyntax: u32, entryname: *const u8) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsMgmtEntryCreateA(::core::mem::transmute(entrynamesyntax), ::core::mem::transmute(entryname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsMgmtEntryCreateW(entrynamesyntax: u32, entryname: *const u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsMgmtEntryCreateW(entrynamesyntax: u32, entryname: *const u16) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsMgmtEntryCreateW(::core::mem::transmute(entrynamesyntax), ::core::mem::transmute(entryname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsMgmtEntryDeleteA(entrynamesyntax: u32, entryname: *const u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsMgmtEntryDeleteA(entrynamesyntax: u32, entryname: *const u8) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsMgmtEntryDeleteA(::core::mem::transmute(entrynamesyntax), ::core::mem::transmute(entryname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsMgmtEntryDeleteW(entrynamesyntax: u32, entryname: *const u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsMgmtEntryDeleteW(entrynamesyntax: u32, entryname: *const u16) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsMgmtEntryDeleteW(::core::mem::transmute(entrynamesyntax), ::core::mem::transmute(entryname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsMgmtEntryInqIfIdsA(entrynamesyntax: u32, entryname: *const u8, ifidvec: *mut *mut RPC_IF_ID_VECTOR) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsMgmtEntryInqIfIdsA(entrynamesyntax: u32, entryname: *const u8, ifidvec: *mut *mut RPC_IF_ID_VECTOR) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsMgmtEntryInqIfIdsA(::core::mem::transmute(entrynamesyntax), ::core::mem::transmute(entryname), ::core::mem::transmute(ifidvec)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsMgmtEntryInqIfIdsW(entrynamesyntax: u32, entryname: *const u16, ifidvec: *mut *mut RPC_IF_ID_VECTOR) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsMgmtEntryInqIfIdsW(entrynamesyntax: u32, entryname: *const u16, ifidvec: *mut *mut RPC_IF_ID_VECTOR) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsMgmtEntryInqIfIdsW(::core::mem::transmute(entrynamesyntax), ::core::mem::transmute(entryname), ::core::mem::transmute(ifidvec)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsMgmtHandleSetExpAge(nshandle: *mut ::core::ffi::c_void, expirationage: u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsMgmtHandleSetExpAge(nshandle: *mut ::core::ffi::c_void, expirationage: u32) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsMgmtHandleSetExpAge(::core::mem::transmute(nshandle), ::core::mem::transmute(expirationage)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsMgmtInqExpAge(expirationage: *mut u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsMgmtInqExpAge(expirationage: *mut u32) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsMgmtInqExpAge(::core::mem::transmute(expirationage)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsMgmtSetExpAge(expirationage: u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsMgmtSetExpAge(expirationage: u32) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsMgmtSetExpAge(::core::mem::transmute(expirationage)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsProfileDeleteA(profilenamesyntax: u32, profilename: *const u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsProfileDeleteA(profilenamesyntax: u32, profilename: *const u8) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsProfileDeleteA(::core::mem::transmute(profilenamesyntax), ::core::mem::transmute(profilename)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsProfileDeleteW(profilenamesyntax: u32, profilename: *const u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsProfileDeleteW(profilenamesyntax: u32, profilename: *const u16) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsProfileDeleteW(::core::mem::transmute(profilenamesyntax), ::core::mem::transmute(profilename)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsProfileEltAddA(profilenamesyntax: u32, profilename: *const u8, ifid: *const RPC_IF_ID, membernamesyntax: u32, membername: *const u8, priority: u32, annotation: *const u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsProfileEltAddA(profilenamesyntax: u32, profilename: *const u8, ifid: *const RPC_IF_ID, membernamesyntax: u32, membername: *const u8, priority: u32, annotation: *const u8) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsProfileEltAddA(::core::mem::transmute(profilenamesyntax), ::core::mem::transmute(profilename), ::core::mem::transmute(ifid), ::core::mem::transmute(membernamesyntax), ::core::mem::transmute(membername), ::core::mem::transmute(priority), ::core::mem::transmute(annotation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsProfileEltAddW(profilenamesyntax: u32, profilename: *const u16, ifid: *const RPC_IF_ID, membernamesyntax: u32, membername: *const u16, priority: u32, annotation: *const u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsProfileEltAddW(profilenamesyntax: u32, profilename: *const u16, ifid: *const RPC_IF_ID, membernamesyntax: u32, membername: *const u16, priority: u32, annotation: *const u16) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsProfileEltAddW(::core::mem::transmute(profilenamesyntax), ::core::mem::transmute(profilename), ::core::mem::transmute(ifid), ::core::mem::transmute(membernamesyntax), ::core::mem::transmute(membername), ::core::mem::transmute(priority), ::core::mem::transmute(annotation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsProfileEltInqBeginA(profilenamesyntax: u32, profilename: *const u8, inquirytype: u32, ifid: *const RPC_IF_ID, versoption: u32, membernamesyntax: u32, membername: *const u8, inquirycontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsProfileEltInqBeginA(profilenamesyntax: u32, profilename: *const u8, inquirytype: u32, ifid: *const RPC_IF_ID, versoption: u32, membernamesyntax: u32, membername: *const u8, inquirycontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsProfileEltInqBeginA(::core::mem::transmute(profilenamesyntax), ::core::mem::transmute(profilename), ::core::mem::transmute(inquirytype), ::core::mem::transmute(ifid), ::core::mem::transmute(versoption), ::core::mem::transmute(membernamesyntax), ::core::mem::transmute(membername), ::core::mem::transmute(inquirycontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsProfileEltInqBeginW(profilenamesyntax: u32, profilename: *const u16, inquirytype: u32, ifid: *const RPC_IF_ID, versoption: u32, membernamesyntax: u32, membername: *const u16, inquirycontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsProfileEltInqBeginW(profilenamesyntax: u32, profilename: *const u16, inquirytype: u32, ifid: *const RPC_IF_ID, versoption: u32, membernamesyntax: u32, membername: *const u16, inquirycontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsProfileEltInqBeginW(::core::mem::transmute(profilenamesyntax), ::core::mem::transmute(profilename), ::core::mem::transmute(inquirytype), ::core::mem::transmute(ifid), ::core::mem::transmute(versoption), ::core::mem::transmute(membernamesyntax), ::core::mem::transmute(membername), ::core::mem::transmute(inquirycontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsProfileEltInqDone(inquirycontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsProfileEltInqDone(inquirycontext: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsProfileEltInqDone(::core::mem::transmute(inquirycontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsProfileEltInqNextA(inquirycontext: *const ::core::ffi::c_void, ifid: *mut RPC_IF_ID, membername: *mut *mut u8, priority: *mut u32, annotation: *mut *mut u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsProfileEltInqNextA(inquirycontext: *const ::core::ffi::c_void, ifid: *mut RPC_IF_ID, membername: *mut *mut u8, priority: *mut u32, annotation: *mut *mut u8) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsProfileEltInqNextA(::core::mem::transmute(inquirycontext), ::core::mem::transmute(ifid), ::core::mem::transmute(membername), ::core::mem::transmute(priority), ::core::mem::transmute(annotation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsProfileEltInqNextW(inquirycontext: *const ::core::ffi::c_void, ifid: *mut RPC_IF_ID, membername: *mut *mut u16, priority: *mut u32, annotation: *mut *mut u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsProfileEltInqNextW(inquirycontext: *const ::core::ffi::c_void, ifid: *mut RPC_IF_ID, membername: *mut *mut u16, priority: *mut u32, annotation: *mut *mut u16) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsProfileEltInqNextW(::core::mem::transmute(inquirycontext), ::core::mem::transmute(ifid), ::core::mem::transmute(membername), ::core::mem::transmute(priority), ::core::mem::transmute(annotation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsProfileEltRemoveA(profilenamesyntax: u32, profilename: *const u8, ifid: *const RPC_IF_ID, membernamesyntax: u32, membername: *const u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsProfileEltRemoveA(profilenamesyntax: u32, profilename: *const u8, ifid: *const RPC_IF_ID, membernamesyntax: u32, membername: *const u8) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsProfileEltRemoveA(::core::mem::transmute(profilenamesyntax), ::core::mem::transmute(profilename), ::core::mem::transmute(ifid), ::core::mem::transmute(membernamesyntax), ::core::mem::transmute(membername)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcNsProfileEltRemoveW(profilenamesyntax: u32, profilename: *const u16, ifid: *const RPC_IF_ID, membernamesyntax: u32, membername: *const u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsProfileEltRemoveW(profilenamesyntax: u32, profilename: *const u16, ifid: *const RPC_IF_ID, membernamesyntax: u32, membername: *const u16) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcNsProfileEltRemoveW(::core::mem::transmute(profilenamesyntax), ::core::mem::transmute(profilename), ::core::mem::transmute(ifid), ::core::mem::transmute(membernamesyntax), ::core::mem::transmute(membername)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcObjectInqType(objuuid: *const ::windows::core::GUID, typeuuid: *mut ::windows::core::GUID) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcObjectInqType(objuuid: *const ::windows::core::GUID, typeuuid: *mut ::windows::core::GUID) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcObjectInqType(::core::mem::transmute(objuuid), ::core::mem::transmute(typeuuid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcObjectSetInqFn(inquiryfn: RPC_OBJECT_INQ_FN) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcObjectSetInqFn(inquiryfn: ::windows::core::RawPtr) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcObjectSetInqFn(::core::mem::transmute(inquiryfn)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcObjectSetType(objuuid: *const ::windows::core::GUID, typeuuid: *const ::windows::core::GUID) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcObjectSetType(objuuid: *const ::windows::core::GUID, typeuuid: *const ::windows::core::GUID) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcObjectSetType(::core::mem::transmute(objuuid), ::core::mem::transmute(typeuuid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcProtseqVectorFreeA(protseqvector: *mut *mut RPC_PROTSEQ_VECTORA) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcProtseqVectorFreeA(protseqvector: *mut *mut RPC_PROTSEQ_VECTORA) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcProtseqVectorFreeA(::core::mem::transmute(protseqvector)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcProtseqVectorFreeW(protseqvector: *mut *mut RPC_PROTSEQ_VECTORW) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcProtseqVectorFreeW(protseqvector: *mut *mut RPC_PROTSEQ_VECTORW) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcProtseqVectorFreeW(::core::mem::transmute(protseqvector)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct RpcProxyPerfCounters(pub i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcCurrentUniqueUser: RpcProxyPerfCounters = RpcProxyPerfCounters(1i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcBackEndConnectionAttempts: RpcProxyPerfCounters = RpcProxyPerfCounters(2i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcBackEndConnectionFailed: RpcProxyPerfCounters = RpcProxyPerfCounters(3i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcRequestsPerSecond: RpcProxyPerfCounters = RpcProxyPerfCounters(4i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcIncomingConnections: RpcProxyPerfCounters = RpcProxyPerfCounters(5i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcIncomingBandwidth: RpcProxyPerfCounters = RpcProxyPerfCounters(6i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcOutgoingBandwidth: RpcProxyPerfCounters = RpcProxyPerfCounters(7i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcAttemptedLbsDecisions: RpcProxyPerfCounters = RpcProxyPerfCounters(8i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcFailedLbsDecisions: RpcProxyPerfCounters = RpcProxyPerfCounters(9i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcAttemptedLbsMessages: RpcProxyPerfCounters = RpcProxyPerfCounters(10i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcFailedLbsMessages: RpcProxyPerfCounters = RpcProxyPerfCounters(11i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const RpcLastCounter: RpcProxyPerfCounters = RpcProxyPerfCounters(12i32);
impl ::core::marker::Copy for RpcProxyPerfCounters {}
impl ::core::clone::Clone for RpcProxyPerfCounters {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for RpcProxyPerfCounters {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for RpcProxyPerfCounters {
    type Abi = Self;
}
impl ::core::fmt::Debug for RpcProxyPerfCounters {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("RpcProxyPerfCounters").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcRaiseException(exception: RPC_STATUS) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcRaiseException(exception: RPC_STATUS);
        }
        RpcRaiseException(::core::mem::transmute(exception))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcRevertContainerImpersonation() -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcRevertContainerImpersonation() -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcRevertContainerImpersonation())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcRevertToSelf() -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcRevertToSelf() -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcRevertToSelf())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcRevertToSelfEx(bindinghandle: *const ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcRevertToSelfEx(bindinghandle: *const ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcRevertToSelfEx(::core::mem::transmute(bindinghandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerCompleteSecurityCallback(bindinghandle: *const ::core::ffi::c_void, status: RPC_STATUS) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerCompleteSecurityCallback(bindinghandle: *const ::core::ffi::c_void, status: RPC_STATUS) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcServerCompleteSecurityCallback(::core::mem::transmute(bindinghandle), ::core::mem::transmute(status)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerInqBindingHandle(binding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerInqBindingHandle(binding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcServerInqBindingHandle(::core::mem::transmute(binding)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerInqBindings(bindingvector: *mut *mut RPC_BINDING_VECTOR) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerInqBindings(bindingvector: *mut *mut RPC_BINDING_VECTOR) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcServerInqBindings(::core::mem::transmute(bindingvector)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerInqBindingsEx(securitydescriptor: *const ::core::ffi::c_void, bindingvector: *mut *mut RPC_BINDING_VECTOR) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerInqBindingsEx(securitydescriptor: *const ::core::ffi::c_void, bindingvector: *mut *mut RPC_BINDING_VECTOR) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcServerInqBindingsEx(::core::mem::transmute(securitydescriptor), ::core::mem::transmute(bindingvector)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerInqCallAttributesA(clientbinding: *const ::core::ffi::c_void, rpccallattributes: *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerInqCallAttributesA(clientbinding: *const ::core::ffi::c_void, rpccallattributes: *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcServerInqCallAttributesA(::core::mem::transmute(clientbinding), ::core::mem::transmute(rpccallattributes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerInqCallAttributesW(clientbinding: *const ::core::ffi::c_void, rpccallattributes: *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerInqCallAttributesW(clientbinding: *const ::core::ffi::c_void, rpccallattributes: *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcServerInqCallAttributesW(::core::mem::transmute(clientbinding), ::core::mem::transmute(rpccallattributes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerInqDefaultPrincNameA(authnsvc: u32, princname: *mut *mut u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerInqDefaultPrincNameA(authnsvc: u32, princname: *mut *mut u8) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcServerInqDefaultPrincNameA(::core::mem::transmute(authnsvc), ::core::mem::transmute(princname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerInqDefaultPrincNameW(authnsvc: u32, princname: *mut *mut u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerInqDefaultPrincNameW(authnsvc: u32, princname: *mut *mut u16) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcServerInqDefaultPrincNameW(::core::mem::transmute(authnsvc), ::core::mem::transmute(princname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerInqIf(ifspec: *const ::core::ffi::c_void, mgrtypeuuid: *const ::windows::core::GUID, mgrepv: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerInqIf(ifspec: *const ::core::ffi::c_void, mgrtypeuuid: *const ::windows::core::GUID, mgrepv: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcServerInqIf(::core::mem::transmute(ifspec), ::core::mem::transmute(mgrtypeuuid), ::core::mem::transmute(mgrepv)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerInterfaceGroupActivate(ifgroup: *const ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerInterfaceGroupActivate(ifgroup: *const ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcServerInterfaceGroupActivate(::core::mem::transmute(ifgroup)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerInterfaceGroupClose(ifgroup: *const ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerInterfaceGroupClose(ifgroup: *const ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcServerInterfaceGroupClose(::core::mem::transmute(ifgroup)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerInterfaceGroupCreateA(interfaces: &[RPC_INTERFACE_TEMPLATEA], endpoints: &[RPC_ENDPOINT_TEMPLATEA], idleperiod: u32, idlecallbackfn: RPC_INTERFACE_GROUP_IDLE_CALLBACK_FN, idlecallbackcontext: *const ::core::ffi::c_void, ifgroup: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerInterfaceGroupCreateA(interfaces: *const RPC_INTERFACE_TEMPLATEA, numifs: u32, endpoints: *const RPC_ENDPOINT_TEMPLATEA, numendpoints: u32, idleperiod: u32, idlecallbackfn: ::windows::core::RawPtr, idlecallbackcontext: *const ::core::ffi::c_void, ifgroup: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcServerInterfaceGroupCreateA(::core::mem::transmute(::windows::core::as_ptr_or_null(interfaces)), interfaces.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(endpoints)), endpoints.len() as _, ::core::mem::transmute(idleperiod), ::core::mem::transmute(idlecallbackfn), ::core::mem::transmute(idlecallbackcontext), ::core::mem::transmute(ifgroup)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerInterfaceGroupCreateW(interfaces: &[RPC_INTERFACE_TEMPLATEW], endpoints: &[RPC_ENDPOINT_TEMPLATEW], idleperiod: u32, idlecallbackfn: RPC_INTERFACE_GROUP_IDLE_CALLBACK_FN, idlecallbackcontext: *const ::core::ffi::c_void, ifgroup: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerInterfaceGroupCreateW(interfaces: *const RPC_INTERFACE_TEMPLATEW, numifs: u32, endpoints: *const RPC_ENDPOINT_TEMPLATEW, numendpoints: u32, idleperiod: u32, idlecallbackfn: ::windows::core::RawPtr, idlecallbackcontext: *const ::core::ffi::c_void, ifgroup: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcServerInterfaceGroupCreateW(::core::mem::transmute(::windows::core::as_ptr_or_null(interfaces)), interfaces.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(endpoints)), endpoints.len() as _, ::core::mem::transmute(idleperiod), ::core::mem::transmute(idlecallbackfn), ::core::mem::transmute(idlecallbackcontext), ::core::mem::transmute(ifgroup)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerInterfaceGroupDeactivate(ifgroup: *const ::core::ffi::c_void, forcedeactivation: u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerInterfaceGroupDeactivate(ifgroup: *const ::core::ffi::c_void, forcedeactivation: u32) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcServerInterfaceGroupDeactivate(::core::mem::transmute(ifgroup), ::core::mem::transmute(forcedeactivation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerInterfaceGroupInqBindings(ifgroup: *const ::core::ffi::c_void, bindingvector: *mut *mut RPC_BINDING_VECTOR) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerInterfaceGroupInqBindings(ifgroup: *const ::core::ffi::c_void, bindingvector: *mut *mut RPC_BINDING_VECTOR) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcServerInterfaceGroupInqBindings(::core::mem::transmute(ifgroup), ::core::mem::transmute(bindingvector)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerListen(minimumcallthreads: u32, maxcalls: u32, dontwait: u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerListen(minimumcallthreads: u32, maxcalls: u32, dontwait: u32) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcServerListen(::core::mem::transmute(minimumcallthreads), ::core::mem::transmute(maxcalls), ::core::mem::transmute(dontwait)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerRegisterAuthInfoA(serverprincname: *const u8, authnsvc: u32, getkeyfn: RPC_AUTH_KEY_RETRIEVAL_FN, arg: *const ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerRegisterAuthInfoA(serverprincname: *const u8, authnsvc: u32, getkeyfn: ::windows::core::RawPtr, arg: *const ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcServerRegisterAuthInfoA(::core::mem::transmute(serverprincname), ::core::mem::transmute(authnsvc), ::core::mem::transmute(getkeyfn), ::core::mem::transmute(arg)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerRegisterAuthInfoW(serverprincname: *const u16, authnsvc: u32, getkeyfn: RPC_AUTH_KEY_RETRIEVAL_FN, arg: *const ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerRegisterAuthInfoW(serverprincname: *const u16, authnsvc: u32, getkeyfn: ::windows::core::RawPtr, arg: *const ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcServerRegisterAuthInfoW(::core::mem::transmute(serverprincname), ::core::mem::transmute(authnsvc), ::core::mem::transmute(getkeyfn), ::core::mem::transmute(arg)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerRegisterIf(ifspec: *const ::core::ffi::c_void, mgrtypeuuid: *const ::windows::core::GUID, mgrepv: *const ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerRegisterIf(ifspec: *const ::core::ffi::c_void, mgrtypeuuid: *const ::windows::core::GUID, mgrepv: *const ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcServerRegisterIf(::core::mem::transmute(ifspec), ::core::mem::transmute(mgrtypeuuid), ::core::mem::transmute(mgrepv)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerRegisterIf2(ifspec: *const ::core::ffi::c_void, mgrtypeuuid: *const ::windows::core::GUID, mgrepv: *const ::core::ffi::c_void, flags: u32, maxcalls: u32, maxrpcsize: u32, ifcallbackfn: RPC_IF_CALLBACK_FN) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerRegisterIf2(ifspec: *const ::core::ffi::c_void, mgrtypeuuid: *const ::windows::core::GUID, mgrepv: *const ::core::ffi::c_void, flags: u32, maxcalls: u32, maxrpcsize: u32, ifcallbackfn: ::windows::core::RawPtr) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcServerRegisterIf2(::core::mem::transmute(ifspec), ::core::mem::transmute(mgrtypeuuid), ::core::mem::transmute(mgrepv), ::core::mem::transmute(flags), ::core::mem::transmute(maxcalls), ::core::mem::transmute(maxrpcsize), ::core::mem::transmute(ifcallbackfn)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerRegisterIf3(ifspec: *const ::core::ffi::c_void, mgrtypeuuid: *const ::windows::core::GUID, mgrepv: *const ::core::ffi::c_void, flags: u32, maxcalls: u32, maxrpcsize: u32, ifcallback: RPC_IF_CALLBACK_FN, securitydescriptor: *const ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerRegisterIf3(ifspec: *const ::core::ffi::c_void, mgrtypeuuid: *const ::windows::core::GUID, mgrepv: *const ::core::ffi::c_void, flags: u32, maxcalls: u32, maxrpcsize: u32, ifcallback: ::windows::core::RawPtr, securitydescriptor: *const ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcServerRegisterIf3(::core::mem::transmute(ifspec), ::core::mem::transmute(mgrtypeuuid), ::core::mem::transmute(mgrepv), ::core::mem::transmute(flags), ::core::mem::transmute(maxcalls), ::core::mem::transmute(maxrpcsize), ::core::mem::transmute(ifcallback), ::core::mem::transmute(securitydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerRegisterIfEx(ifspec: *const ::core::ffi::c_void, mgrtypeuuid: *const ::windows::core::GUID, mgrepv: *const ::core::ffi::c_void, flags: u32, maxcalls: u32, ifcallback: RPC_IF_CALLBACK_FN) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerRegisterIfEx(ifspec: *const ::core::ffi::c_void, mgrtypeuuid: *const ::windows::core::GUID, mgrepv: *const ::core::ffi::c_void, flags: u32, maxcalls: u32, ifcallback: ::windows::core::RawPtr) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcServerRegisterIfEx(::core::mem::transmute(ifspec), ::core::mem::transmute(mgrtypeuuid), ::core::mem::transmute(mgrepv), ::core::mem::transmute(flags), ::core::mem::transmute(maxcalls), ::core::mem::transmute(ifcallback)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_Foundation\"`, `\"Win32_System_IO\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn RpcServerSubscribeForNotification(binding: *const ::core::ffi::c_void, notification: RPC_NOTIFICATIONS, notificationtype: RPC_NOTIFICATION_TYPES, notificationinfo: *const RPC_ASYNC_NOTIFICATION_INFO) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerSubscribeForNotification(binding: *const ::core::ffi::c_void, notification: RPC_NOTIFICATIONS, notificationtype: RPC_NOTIFICATION_TYPES, notificationinfo: *const RPC_ASYNC_NOTIFICATION_INFO) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcServerSubscribeForNotification(::core::mem::transmute(binding), ::core::mem::transmute(notification), ::core::mem::transmute(notificationtype), ::core::mem::transmute(notificationinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerTestCancel(bindinghandle: *const ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerTestCancel(bindinghandle: *const ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcServerTestCancel(::core::mem::transmute(bindinghandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerUnregisterIf(ifspec: *const ::core::ffi::c_void, mgrtypeuuid: *const ::windows::core::GUID, waitforcallstocomplete: u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerUnregisterIf(ifspec: *const ::core::ffi::c_void, mgrtypeuuid: *const ::windows::core::GUID, waitforcallstocomplete: u32) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcServerUnregisterIf(::core::mem::transmute(ifspec), ::core::mem::transmute(mgrtypeuuid), ::core::mem::transmute(waitforcallstocomplete)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerUnregisterIfEx(ifspec: *const ::core::ffi::c_void, mgrtypeuuid: *const ::windows::core::GUID, rundowncontexthandles: i32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerUnregisterIfEx(ifspec: *const ::core::ffi::c_void, mgrtypeuuid: *const ::windows::core::GUID, rundowncontexthandles: i32) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcServerUnregisterIfEx(::core::mem::transmute(ifspec), ::core::mem::transmute(mgrtypeuuid), ::core::mem::transmute(rundowncontexthandles)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerUnsubscribeForNotification(binding: *const ::core::ffi::c_void, notification: RPC_NOTIFICATIONS, notificationsqueued: *mut u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerUnsubscribeForNotification(binding: *const ::core::ffi::c_void, notification: RPC_NOTIFICATIONS, notificationsqueued: *mut u32) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcServerUnsubscribeForNotification(::core::mem::transmute(binding), ::core::mem::transmute(notification), ::core::mem::transmute(notificationsqueued)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerUseAllProtseqs(maxcalls: u32, securitydescriptor: *const ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerUseAllProtseqs(maxcalls: u32, securitydescriptor: *const ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcServerUseAllProtseqs(::core::mem::transmute(maxcalls), ::core::mem::transmute(securitydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerUseAllProtseqsEx(maxcalls: u32, securitydescriptor: *const ::core::ffi::c_void, policy: *const RPC_POLICY) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerUseAllProtseqsEx(maxcalls: u32, securitydescriptor: *const ::core::ffi::c_void, policy: *const RPC_POLICY) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcServerUseAllProtseqsEx(::core::mem::transmute(maxcalls), ::core::mem::transmute(securitydescriptor), ::core::mem::transmute(policy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerUseAllProtseqsIf(maxcalls: u32, ifspec: *const ::core::ffi::c_void, securitydescriptor: *const ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerUseAllProtseqsIf(maxcalls: u32, ifspec: *const ::core::ffi::c_void, securitydescriptor: *const ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcServerUseAllProtseqsIf(::core::mem::transmute(maxcalls), ::core::mem::transmute(ifspec), ::core::mem::transmute(securitydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerUseAllProtseqsIfEx(maxcalls: u32, ifspec: *const ::core::ffi::c_void, securitydescriptor: *const ::core::ffi::c_void, policy: *const RPC_POLICY) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerUseAllProtseqsIfEx(maxcalls: u32, ifspec: *const ::core::ffi::c_void, securitydescriptor: *const ::core::ffi::c_void, policy: *const RPC_POLICY) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcServerUseAllProtseqsIfEx(::core::mem::transmute(maxcalls), ::core::mem::transmute(ifspec), ::core::mem::transmute(securitydescriptor), ::core::mem::transmute(policy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerUseProtseqA(protseq: *const u8, maxcalls: u32, securitydescriptor: *const ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerUseProtseqA(protseq: *const u8, maxcalls: u32, securitydescriptor: *const ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcServerUseProtseqA(::core::mem::transmute(protseq), ::core::mem::transmute(maxcalls), ::core::mem::transmute(securitydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerUseProtseqEpA(protseq: *const u8, maxcalls: u32, endpoint: *const u8, securitydescriptor: *const ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerUseProtseqEpA(protseq: *const u8, maxcalls: u32, endpoint: *const u8, securitydescriptor: *const ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcServerUseProtseqEpA(::core::mem::transmute(protseq), ::core::mem::transmute(maxcalls), ::core::mem::transmute(endpoint), ::core::mem::transmute(securitydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerUseProtseqEpExA(protseq: *const u8, maxcalls: u32, endpoint: *const u8, securitydescriptor: *const ::core::ffi::c_void, policy: *const RPC_POLICY) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerUseProtseqEpExA(protseq: *const u8, maxcalls: u32, endpoint: *const u8, securitydescriptor: *const ::core::ffi::c_void, policy: *const RPC_POLICY) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcServerUseProtseqEpExA(::core::mem::transmute(protseq), ::core::mem::transmute(maxcalls), ::core::mem::transmute(endpoint), ::core::mem::transmute(securitydescriptor), ::core::mem::transmute(policy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerUseProtseqEpExW(protseq: *const u16, maxcalls: u32, endpoint: *const u16, securitydescriptor: *const ::core::ffi::c_void, policy: *const RPC_POLICY) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerUseProtseqEpExW(protseq: *const u16, maxcalls: u32, endpoint: *const u16, securitydescriptor: *const ::core::ffi::c_void, policy: *const RPC_POLICY) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcServerUseProtseqEpExW(::core::mem::transmute(protseq), ::core::mem::transmute(maxcalls), ::core::mem::transmute(endpoint), ::core::mem::transmute(securitydescriptor), ::core::mem::transmute(policy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerUseProtseqEpW(protseq: *const u16, maxcalls: u32, endpoint: *const u16, securitydescriptor: *const ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerUseProtseqEpW(protseq: *const u16, maxcalls: u32, endpoint: *const u16, securitydescriptor: *const ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcServerUseProtseqEpW(::core::mem::transmute(protseq), ::core::mem::transmute(maxcalls), ::core::mem::transmute(endpoint), ::core::mem::transmute(securitydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerUseProtseqExA(protseq: *const u8, maxcalls: u32, securitydescriptor: *const ::core::ffi::c_void, policy: *const RPC_POLICY) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerUseProtseqExA(protseq: *const u8, maxcalls: u32, securitydescriptor: *const ::core::ffi::c_void, policy: *const RPC_POLICY) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcServerUseProtseqExA(::core::mem::transmute(protseq), ::core::mem::transmute(maxcalls), ::core::mem::transmute(securitydescriptor), ::core::mem::transmute(policy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerUseProtseqExW(protseq: *const u16, maxcalls: u32, securitydescriptor: *const ::core::ffi::c_void, policy: *const RPC_POLICY) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerUseProtseqExW(protseq: *const u16, maxcalls: u32, securitydescriptor: *const ::core::ffi::c_void, policy: *const RPC_POLICY) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcServerUseProtseqExW(::core::mem::transmute(protseq), ::core::mem::transmute(maxcalls), ::core::mem::transmute(securitydescriptor), ::core::mem::transmute(policy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerUseProtseqIfA(protseq: *const u8, maxcalls: u32, ifspec: *const ::core::ffi::c_void, securitydescriptor: *const ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerUseProtseqIfA(protseq: *const u8, maxcalls: u32, ifspec: *const ::core::ffi::c_void, securitydescriptor: *const ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcServerUseProtseqIfA(::core::mem::transmute(protseq), ::core::mem::transmute(maxcalls), ::core::mem::transmute(ifspec), ::core::mem::transmute(securitydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerUseProtseqIfExA(protseq: *const u8, maxcalls: u32, ifspec: *const ::core::ffi::c_void, securitydescriptor: *const ::core::ffi::c_void, policy: *const RPC_POLICY) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerUseProtseqIfExA(protseq: *const u8, maxcalls: u32, ifspec: *const ::core::ffi::c_void, securitydescriptor: *const ::core::ffi::c_void, policy: *const RPC_POLICY) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcServerUseProtseqIfExA(::core::mem::transmute(protseq), ::core::mem::transmute(maxcalls), ::core::mem::transmute(ifspec), ::core::mem::transmute(securitydescriptor), ::core::mem::transmute(policy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerUseProtseqIfExW(protseq: *const u16, maxcalls: u32, ifspec: *const ::core::ffi::c_void, securitydescriptor: *const ::core::ffi::c_void, policy: *const RPC_POLICY) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerUseProtseqIfExW(protseq: *const u16, maxcalls: u32, ifspec: *const ::core::ffi::c_void, securitydescriptor: *const ::core::ffi::c_void, policy: *const RPC_POLICY) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcServerUseProtseqIfExW(::core::mem::transmute(protseq), ::core::mem::transmute(maxcalls), ::core::mem::transmute(ifspec), ::core::mem::transmute(securitydescriptor), ::core::mem::transmute(policy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerUseProtseqIfW(protseq: *const u16, maxcalls: u32, ifspec: *const ::core::ffi::c_void, securitydescriptor: *const ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerUseProtseqIfW(protseq: *const u16, maxcalls: u32, ifspec: *const ::core::ffi::c_void, securitydescriptor: *const ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcServerUseProtseqIfW(::core::mem::transmute(protseq), ::core::mem::transmute(maxcalls), ::core::mem::transmute(ifspec), ::core::mem::transmute(securitydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerUseProtseqW(protseq: *const u16, maxcalls: u32, securitydescriptor: *const ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerUseProtseqW(protseq: *const u16, maxcalls: u32, securitydescriptor: *const ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcServerUseProtseqW(::core::mem::transmute(protseq), ::core::mem::transmute(maxcalls), ::core::mem::transmute(securitydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcServerYield() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerYield();
        }
        RpcServerYield()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcSmAllocate(size: usize, pstatus: *mut RPC_STATUS) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcSmAllocate(size: usize, pstatus: *mut RPC_STATUS) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(RpcSmAllocate(::core::mem::transmute(size), ::core::mem::transmute(pstatus)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcSmClientFree(pnodetofree: *const ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcSmClientFree(pnodetofree: *const ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcSmClientFree(::core::mem::transmute(pnodetofree)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcSmDestroyClientContext(contexthandle: *const *const ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcSmDestroyClientContext(contexthandle: *const *const ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcSmDestroyClientContext(::core::mem::transmute(contexthandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcSmDisableAllocate() -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcSmDisableAllocate() -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcSmDisableAllocate())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcSmEnableAllocate() -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcSmEnableAllocate() -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcSmEnableAllocate())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcSmFree(nodetofree: *const ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcSmFree(nodetofree: *const ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcSmFree(::core::mem::transmute(nodetofree)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcSmGetThreadHandle(pstatus: *mut RPC_STATUS) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcSmGetThreadHandle(pstatus: *mut RPC_STATUS) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(RpcSmGetThreadHandle(::core::mem::transmute(pstatus)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcSmSetClientAllocFree(clientalloc: RPC_CLIENT_ALLOC, clientfree: RPC_CLIENT_FREE) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcSmSetClientAllocFree(clientalloc: ::windows::core::RawPtr, clientfree: ::windows::core::RawPtr) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcSmSetClientAllocFree(::core::mem::transmute(clientalloc), ::core::mem::transmute(clientfree)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcSmSetThreadHandle(id: *const ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcSmSetThreadHandle(id: *const ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcSmSetThreadHandle(::core::mem::transmute(id)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcSmSwapClientAllocFree(clientalloc: RPC_CLIENT_ALLOC, clientfree: RPC_CLIENT_FREE, oldclientalloc: *mut RPC_CLIENT_ALLOC, oldclientfree: *mut RPC_CLIENT_FREE) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcSmSwapClientAllocFree(clientalloc: ::windows::core::RawPtr, clientfree: ::windows::core::RawPtr, oldclientalloc: *mut ::windows::core::RawPtr, oldclientfree: *mut ::windows::core::RawPtr) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcSmSwapClientAllocFree(::core::mem::transmute(clientalloc), ::core::mem::transmute(clientfree), ::core::mem::transmute(oldclientalloc), ::core::mem::transmute(oldclientfree)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcSsAllocate(size: usize) -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcSsAllocate(size: usize) -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(RpcSsAllocate(::core::mem::transmute(size)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcSsContextLockExclusive(serverbindinghandle: *const ::core::ffi::c_void, usercontext: *const ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcSsContextLockExclusive(serverbindinghandle: *const ::core::ffi::c_void, usercontext: *const ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcSsContextLockExclusive(::core::mem::transmute(serverbindinghandle), ::core::mem::transmute(usercontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcSsContextLockShared(serverbindinghandle: *const ::core::ffi::c_void, usercontext: *const ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcSsContextLockShared(serverbindinghandle: *const ::core::ffi::c_void, usercontext: *const ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcSsContextLockShared(::core::mem::transmute(serverbindinghandle), ::core::mem::transmute(usercontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcSsDestroyClientContext(contexthandle: *const *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcSsDestroyClientContext(contexthandle: *const *const ::core::ffi::c_void);
        }
        RpcSsDestroyClientContext(::core::mem::transmute(contexthandle))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcSsDisableAllocate() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcSsDisableAllocate();
        }
        RpcSsDisableAllocate()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcSsDontSerializeContext() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcSsDontSerializeContext();
        }
        RpcSsDontSerializeContext()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcSsEnableAllocate() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcSsEnableAllocate();
        }
        RpcSsEnableAllocate()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcSsFree(nodetofree: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcSsFree(nodetofree: *const ::core::ffi::c_void);
        }
        RpcSsFree(::core::mem::transmute(nodetofree))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcSsGetContextBinding(contexthandle: *const ::core::ffi::c_void, binding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcSsGetContextBinding(contexthandle: *const ::core::ffi::c_void, binding: *mut *mut ::core::ffi::c_void) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcSsGetContextBinding(::core::mem::transmute(contexthandle), ::core::mem::transmute(binding)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcSsGetThreadHandle() -> *mut ::core::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcSsGetThreadHandle() -> *mut ::core::ffi::c_void;
        }
        ::core::mem::transmute(RpcSsGetThreadHandle())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcSsSetClientAllocFree(clientalloc: RPC_CLIENT_ALLOC, clientfree: RPC_CLIENT_FREE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcSsSetClientAllocFree(clientalloc: ::windows::core::RawPtr, clientfree: ::windows::core::RawPtr);
        }
        RpcSsSetClientAllocFree(::core::mem::transmute(clientalloc), ::core::mem::transmute(clientfree))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcSsSetThreadHandle(id: *const ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcSsSetThreadHandle(id: *const ::core::ffi::c_void);
        }
        RpcSsSetThreadHandle(::core::mem::transmute(id))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcSsSwapClientAllocFree(clientalloc: RPC_CLIENT_ALLOC, clientfree: RPC_CLIENT_FREE, oldclientalloc: *mut RPC_CLIENT_ALLOC, oldclientfree: *mut RPC_CLIENT_FREE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcSsSwapClientAllocFree(clientalloc: ::windows::core::RawPtr, clientfree: ::windows::core::RawPtr, oldclientalloc: *mut ::windows::core::RawPtr, oldclientfree: *mut ::windows::core::RawPtr);
        }
        RpcSsSwapClientAllocFree(::core::mem::transmute(clientalloc), ::core::mem::transmute(clientfree), ::core::mem::transmute(oldclientalloc), ::core::mem::transmute(oldclientfree))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcStringBindingComposeA(objuuid: *const u8, protseq: *const u8, networkaddr: *const u8, endpoint: *const u8, options: *const u8, stringbinding: *mut *mut u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcStringBindingComposeA(objuuid: *const u8, protseq: *const u8, networkaddr: *const u8, endpoint: *const u8, options: *const u8, stringbinding: *mut *mut u8) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcStringBindingComposeA(::core::mem::transmute(objuuid), ::core::mem::transmute(protseq), ::core::mem::transmute(networkaddr), ::core::mem::transmute(endpoint), ::core::mem::transmute(options), ::core::mem::transmute(stringbinding)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcStringBindingComposeW(objuuid: *const u16, protseq: *const u16, networkaddr: *const u16, endpoint: *const u16, options: *const u16, stringbinding: *mut *mut u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcStringBindingComposeW(objuuid: *const u16, protseq: *const u16, networkaddr: *const u16, endpoint: *const u16, options: *const u16, stringbinding: *mut *mut u16) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcStringBindingComposeW(::core::mem::transmute(objuuid), ::core::mem::transmute(protseq), ::core::mem::transmute(networkaddr), ::core::mem::transmute(endpoint), ::core::mem::transmute(options), ::core::mem::transmute(stringbinding)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcStringBindingParseA(stringbinding: *const u8, objuuid: *mut *mut u8, protseq: *mut *mut u8, networkaddr: *mut *mut u8, endpoint: *mut *mut u8, networkoptions: *mut *mut u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcStringBindingParseA(stringbinding: *const u8, objuuid: *mut *mut u8, protseq: *mut *mut u8, networkaddr: *mut *mut u8, endpoint: *mut *mut u8, networkoptions: *mut *mut u8) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcStringBindingParseA(::core::mem::transmute(stringbinding), ::core::mem::transmute(objuuid), ::core::mem::transmute(protseq), ::core::mem::transmute(networkaddr), ::core::mem::transmute(endpoint), ::core::mem::transmute(networkoptions)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcStringBindingParseW(stringbinding: *const u16, objuuid: *mut *mut u16, protseq: *mut *mut u16, networkaddr: *mut *mut u16, endpoint: *mut *mut u16, networkoptions: *mut *mut u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcStringBindingParseW(stringbinding: *const u16, objuuid: *mut *mut u16, protseq: *mut *mut u16, networkaddr: *mut *mut u16, endpoint: *mut *mut u16, networkoptions: *mut *mut u16) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcStringBindingParseW(::core::mem::transmute(stringbinding), ::core::mem::transmute(objuuid), ::core::mem::transmute(protseq), ::core::mem::transmute(networkaddr), ::core::mem::transmute(endpoint), ::core::mem::transmute(networkoptions)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcStringFreeA(string: *mut *mut u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcStringFreeA(string: *mut *mut u8) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcStringFreeA(::core::mem::transmute(string)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcStringFreeW(string: *mut *mut u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcStringFreeW(string: *mut *mut u16) -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcStringFreeW(::core::mem::transmute(string)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcTestCancel() -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcTestCancel() -> RPC_STATUS;
        }
        ::core::mem::transmute(RpcTestCancel())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn RpcUserFree(asynchandle: *mut ::core::ffi::c_void, pbuffer: *mut ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcUserFree(asynchandle: *mut ::core::ffi::c_void, pbuffer: *mut ::core::ffi::c_void);
        }
        RpcUserFree(::core::mem::transmute(asynchandle), ::core::mem::transmute(pbuffer))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct SCONTEXT_QUEUE {
    pub NumberOfObjects: u32,
    pub ArrayOfObjects: *mut *mut NDR_SCONTEXT_1,
}
impl ::core::marker::Copy for SCONTEXT_QUEUE {}
impl ::core::clone::Clone for SCONTEXT_QUEUE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SCONTEXT_QUEUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SCONTEXT_QUEUE").field("NumberOfObjects", &self.NumberOfObjects).field("ArrayOfObjects", &self.ArrayOfObjects).finish()
    }
}
unsafe impl ::windows::core::Abi for SCONTEXT_QUEUE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SCONTEXT_QUEUE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SCONTEXT_QUEUE>()) == 0 }
    }
}
impl ::core::cmp::Eq for SCONTEXT_QUEUE {}
impl ::core::default::Default for SCONTEXT_QUEUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct SEC_WINNT_AUTH_IDENTITY(pub u32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const SEC_WINNT_AUTH_IDENTITY_ANSI: SEC_WINNT_AUTH_IDENTITY = SEC_WINNT_AUTH_IDENTITY(1u32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const SEC_WINNT_AUTH_IDENTITY_UNICODE: SEC_WINNT_AUTH_IDENTITY = SEC_WINNT_AUTH_IDENTITY(2u32);
impl ::core::marker::Copy for SEC_WINNT_AUTH_IDENTITY {}
impl ::core::clone::Clone for SEC_WINNT_AUTH_IDENTITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SEC_WINNT_AUTH_IDENTITY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SEC_WINNT_AUTH_IDENTITY {
    type Abi = Self;
}
impl ::core::fmt::Debug for SEC_WINNT_AUTH_IDENTITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SEC_WINNT_AUTH_IDENTITY").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct SEC_WINNT_AUTH_IDENTITY_A {
    pub User: *mut u8,
    pub UserLength: u32,
    pub Domain: *mut u8,
    pub DomainLength: u32,
    pub Password: *mut u8,
    pub PasswordLength: u32,
    pub Flags: SEC_WINNT_AUTH_IDENTITY,
}
impl ::core::marker::Copy for SEC_WINNT_AUTH_IDENTITY_A {}
impl ::core::clone::Clone for SEC_WINNT_AUTH_IDENTITY_A {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SEC_WINNT_AUTH_IDENTITY_A {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SEC_WINNT_AUTH_IDENTITY_A").field("User", &self.User).field("UserLength", &self.UserLength).field("Domain", &self.Domain).field("DomainLength", &self.DomainLength).field("Password", &self.Password).field("PasswordLength", &self.PasswordLength).field("Flags", &self.Flags).finish()
    }
}
unsafe impl ::windows::core::Abi for SEC_WINNT_AUTH_IDENTITY_A {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SEC_WINNT_AUTH_IDENTITY_A {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SEC_WINNT_AUTH_IDENTITY_A>()) == 0 }
    }
}
impl ::core::cmp::Eq for SEC_WINNT_AUTH_IDENTITY_A {}
impl ::core::default::Default for SEC_WINNT_AUTH_IDENTITY_A {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct SEC_WINNT_AUTH_IDENTITY_W {
    pub User: *mut u16,
    pub UserLength: u32,
    pub Domain: *mut u16,
    pub DomainLength: u32,
    pub Password: *mut u16,
    pub PasswordLength: u32,
    pub Flags: SEC_WINNT_AUTH_IDENTITY,
}
impl ::core::marker::Copy for SEC_WINNT_AUTH_IDENTITY_W {}
impl ::core::clone::Clone for SEC_WINNT_AUTH_IDENTITY_W {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SEC_WINNT_AUTH_IDENTITY_W {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SEC_WINNT_AUTH_IDENTITY_W").field("User", &self.User).field("UserLength", &self.UserLength).field("Domain", &self.Domain).field("DomainLength", &self.DomainLength).field("Password", &self.Password).field("PasswordLength", &self.PasswordLength).field("Flags", &self.Flags).finish()
    }
}
unsafe impl ::windows::core::Abi for SEC_WINNT_AUTH_IDENTITY_W {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SEC_WINNT_AUTH_IDENTITY_W {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SEC_WINNT_AUTH_IDENTITY_W>()) == 0 }
    }
}
impl ::core::cmp::Eq for SEC_WINNT_AUTH_IDENTITY_W {}
impl ::core::default::Default for SEC_WINNT_AUTH_IDENTITY_W {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type SERVER_ROUTINE = ::core::option::Option<unsafe extern "system" fn() -> i32>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct STUB_PHASE(pub i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const STUB_UNMARSHAL: STUB_PHASE = STUB_PHASE(0i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const STUB_CALL_SERVER: STUB_PHASE = STUB_PHASE(1i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const STUB_MARSHAL: STUB_PHASE = STUB_PHASE(2i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const STUB_CALL_SERVER_NO_HRESULT: STUB_PHASE = STUB_PHASE(3i32);
impl ::core::marker::Copy for STUB_PHASE {}
impl ::core::clone::Clone for STUB_PHASE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for STUB_PHASE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for STUB_PHASE {
    type Abi = Self;
}
impl ::core::fmt::Debug for STUB_PHASE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("STUB_PHASE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub type STUB_THUNK = ::core::option::Option<unsafe extern "system" fn(param0: *mut MIDL_STUB_MESSAGE)>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const TARGET_IS_NT100_OR_LATER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const TARGET_IS_NT351_OR_WIN95_OR_LATER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const TARGET_IS_NT40_OR_LATER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const TARGET_IS_NT50_OR_LATER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const TARGET_IS_NT51_OR_LATER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const TARGET_IS_NT60_OR_LATER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const TARGET_IS_NT61_OR_LATER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const TARGET_IS_NT62_OR_LATER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const TARGET_IS_NT63_OR_LATER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const TRANSPORT_TYPE_CN: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const TRANSPORT_TYPE_DG: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const TRANSPORT_TYPE_LPC: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const TRANSPORT_TYPE_WMSG: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const USER_CALL_IS_ASYNC: u32 = 256u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const USER_CALL_NEW_CORRELATION_DESC: u32 = 512u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct USER_MARSHAL_CB {
    pub Flags: u32,
    pub pStubMsg: *mut MIDL_STUB_MESSAGE,
    pub pReserve: *mut u8,
    pub Signature: u32,
    pub CBType: USER_MARSHAL_CB_TYPE,
    pub pFormat: *mut u8,
    pub pTypeFormat: *mut u8,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for USER_MARSHAL_CB {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for USER_MARSHAL_CB {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for USER_MARSHAL_CB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_MARSHAL_CB").field("Flags", &self.Flags).field("pStubMsg", &self.pStubMsg).field("pReserve", &self.pReserve).field("Signature", &self.Signature).field("CBType", &self.CBType).field("pFormat", &self.pFormat).field("pTypeFormat", &self.pTypeFormat).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for USER_MARSHAL_CB {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for USER_MARSHAL_CB {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USER_MARSHAL_CB>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for USER_MARSHAL_CB {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for USER_MARSHAL_CB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct USER_MARSHAL_CB_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const USER_MARSHAL_CB_BUFFER_SIZE: USER_MARSHAL_CB_TYPE = USER_MARSHAL_CB_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const USER_MARSHAL_CB_MARSHALL: USER_MARSHAL_CB_TYPE = USER_MARSHAL_CB_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const USER_MARSHAL_CB_UNMARSHALL: USER_MARSHAL_CB_TYPE = USER_MARSHAL_CB_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const USER_MARSHAL_CB_FREE: USER_MARSHAL_CB_TYPE = USER_MARSHAL_CB_TYPE(3i32);
impl ::core::marker::Copy for USER_MARSHAL_CB_TYPE {}
impl ::core::clone::Clone for USER_MARSHAL_CB_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for USER_MARSHAL_CB_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for USER_MARSHAL_CB_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for USER_MARSHAL_CB_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("USER_MARSHAL_CB_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const USER_MARSHAL_FC_BYTE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const USER_MARSHAL_FC_CHAR: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const USER_MARSHAL_FC_DOUBLE: u32 = 12u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const USER_MARSHAL_FC_FLOAT: u32 = 10u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const USER_MARSHAL_FC_HYPER: u32 = 11u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const USER_MARSHAL_FC_LONG: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const USER_MARSHAL_FC_SHORT: u32 = 6u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const USER_MARSHAL_FC_SMALL: u32 = 3u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const USER_MARSHAL_FC_ULONG: u32 = 9u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const USER_MARSHAL_FC_USHORT: u32 = 7u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const USER_MARSHAL_FC_USMALL: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const USER_MARSHAL_FC_WCHAR: u32 = 5u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type USER_MARSHAL_FREEING_ROUTINE = ::core::option::Option<unsafe extern "system" fn(param0: *mut u32, param1: *mut ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type USER_MARSHAL_MARSHALLING_ROUTINE = ::core::option::Option<unsafe extern "system" fn(param0: *mut u32, param1: *mut u8, param2: *mut ::core::ffi::c_void) -> *mut u8>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct USER_MARSHAL_ROUTINE_QUADRUPLE {
    pub pfnBufferSize: USER_MARSHAL_SIZING_ROUTINE,
    pub pfnMarshall: USER_MARSHAL_MARSHALLING_ROUTINE,
    pub pfnUnmarshall: USER_MARSHAL_UNMARSHALLING_ROUTINE,
    pub pfnFree: USER_MARSHAL_FREEING_ROUTINE,
}
impl ::core::marker::Copy for USER_MARSHAL_ROUTINE_QUADRUPLE {}
impl ::core::clone::Clone for USER_MARSHAL_ROUTINE_QUADRUPLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for USER_MARSHAL_ROUTINE_QUADRUPLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("USER_MARSHAL_ROUTINE_QUADRUPLE").field("pfnBufferSize", &self.pfnBufferSize.map(|f| f as usize)).field("pfnMarshall", &self.pfnMarshall.map(|f| f as usize)).field("pfnUnmarshall", &self.pfnUnmarshall.map(|f| f as usize)).field("pfnFree", &self.pfnFree.map(|f| f as usize)).finish()
    }
}
unsafe impl ::windows::core::Abi for USER_MARSHAL_ROUTINE_QUADRUPLE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for USER_MARSHAL_ROUTINE_QUADRUPLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<USER_MARSHAL_ROUTINE_QUADRUPLE>()) == 0 }
    }
}
impl ::core::cmp::Eq for USER_MARSHAL_ROUTINE_QUADRUPLE {}
impl ::core::default::Default for USER_MARSHAL_ROUTINE_QUADRUPLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type USER_MARSHAL_SIZING_ROUTINE = ::core::option::Option<unsafe extern "system" fn(param0: *mut u32, param1: u32, param2: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub type USER_MARSHAL_UNMARSHALLING_ROUTINE = ::core::option::Option<unsafe extern "system" fn(param0: *mut u32, param1: *mut u8, param2: *mut ::core::ffi::c_void) -> *mut u8>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct UUID_VECTOR {
    pub Count: u32,
    pub Uuid: [*mut ::windows::core::GUID; 1],
}
impl ::core::marker::Copy for UUID_VECTOR {}
impl ::core::clone::Clone for UUID_VECTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for UUID_VECTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("UUID_VECTOR").field("Count", &self.Count).field("Uuid", &self.Uuid).finish()
    }
}
unsafe impl ::windows::core::Abi for UUID_VECTOR {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for UUID_VECTOR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<UUID_VECTOR>()) == 0 }
    }
}
impl ::core::cmp::Eq for UUID_VECTOR {}
impl ::core::default::Default for UUID_VECTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn UuidCompare(uuid1: *const ::windows::core::GUID, uuid2: *const ::windows::core::GUID, status: *mut RPC_STATUS) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UuidCompare(uuid1: *const ::windows::core::GUID, uuid2: *const ::windows::core::GUID, status: *mut RPC_STATUS) -> i32;
        }
        ::core::mem::transmute(UuidCompare(::core::mem::transmute(uuid1), ::core::mem::transmute(uuid2), ::core::mem::transmute(status)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn UuidCreate(uuid: *mut ::windows::core::GUID) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UuidCreate(uuid: *mut ::windows::core::GUID) -> RPC_STATUS;
        }
        ::core::mem::transmute(UuidCreate(::core::mem::transmute(uuid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn UuidCreateNil(niluuid: *mut ::windows::core::GUID) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UuidCreateNil(niluuid: *mut ::windows::core::GUID) -> RPC_STATUS;
        }
        ::core::mem::transmute(UuidCreateNil(::core::mem::transmute(niluuid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn UuidCreateSequential(uuid: *mut ::windows::core::GUID) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UuidCreateSequential(uuid: *mut ::windows::core::GUID) -> RPC_STATUS;
        }
        ::core::mem::transmute(UuidCreateSequential(::core::mem::transmute(uuid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn UuidEqual(uuid1: *const ::windows::core::GUID, uuid2: *const ::windows::core::GUID, status: *mut RPC_STATUS) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UuidEqual(uuid1: *const ::windows::core::GUID, uuid2: *const ::windows::core::GUID, status: *mut RPC_STATUS) -> i32;
        }
        ::core::mem::transmute(UuidEqual(::core::mem::transmute(uuid1), ::core::mem::transmute(uuid2), ::core::mem::transmute(status)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn UuidFromStringA(stringuuid: *const u8, uuid: *mut ::windows::core::GUID) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UuidFromStringA(stringuuid: *const u8, uuid: *mut ::windows::core::GUID) -> RPC_STATUS;
        }
        ::core::mem::transmute(UuidFromStringA(::core::mem::transmute(stringuuid), ::core::mem::transmute(uuid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn UuidFromStringW(stringuuid: *const u16, uuid: *mut ::windows::core::GUID) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UuidFromStringW(stringuuid: *const u16, uuid: *mut ::windows::core::GUID) -> RPC_STATUS;
        }
        ::core::mem::transmute(UuidFromStringW(::core::mem::transmute(stringuuid), ::core::mem::transmute(uuid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn UuidHash(uuid: *const ::windows::core::GUID, status: *mut RPC_STATUS) -> u16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UuidHash(uuid: *const ::windows::core::GUID, status: *mut RPC_STATUS) -> u16;
        }
        ::core::mem::transmute(UuidHash(::core::mem::transmute(uuid), ::core::mem::transmute(status)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn UuidIsNil(uuid: *const ::windows::core::GUID, status: *mut RPC_STATUS) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UuidIsNil(uuid: *const ::windows::core::GUID, status: *mut RPC_STATUS) -> i32;
        }
        ::core::mem::transmute(UuidIsNil(::core::mem::transmute(uuid), ::core::mem::transmute(status)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn UuidToStringA(uuid: *const ::windows::core::GUID, stringuuid: *mut *mut u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UuidToStringA(uuid: *const ::windows::core::GUID, stringuuid: *mut *mut u8) -> RPC_STATUS;
        }
        ::core::mem::transmute(UuidToStringA(::core::mem::transmute(uuid), ::core::mem::transmute(stringuuid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[inline]
pub unsafe fn UuidToStringW(uuid: *const ::windows::core::GUID, stringuuid: *mut *mut u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UuidToStringW(uuid: *const ::windows::core::GUID, stringuuid: *mut *mut u16) -> RPC_STATUS;
        }
        ::core::mem::transmute(UuidToStringW(::core::mem::transmute(uuid), ::core::mem::transmute(stringuuid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct XLAT_SIDE(pub i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const XLAT_SERVER: XLAT_SIDE = XLAT_SIDE(1i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const XLAT_CLIENT: XLAT_SIDE = XLAT_SIDE(2i32);
impl ::core::marker::Copy for XLAT_SIDE {}
impl ::core::clone::Clone for XLAT_SIDE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for XLAT_SIDE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for XLAT_SIDE {
    type Abi = Self;
}
impl ::core::fmt::Debug for XLAT_SIDE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("XLAT_SIDE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub type XMIT_HELPER_ROUTINE = ::core::option::Option<unsafe extern "system" fn(param0: *mut MIDL_STUB_MESSAGE)>;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`, `\"Win32_System_Com\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub struct XMIT_ROUTINE_QUINTUPLE {
    pub pfnTranslateToXmit: XMIT_HELPER_ROUTINE,
    pub pfnTranslateFromXmit: XMIT_HELPER_ROUTINE,
    pub pfnFreeXmit: XMIT_HELPER_ROUTINE,
    pub pfnFreeInst: XMIT_HELPER_ROUTINE,
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::marker::Copy for XMIT_ROUTINE_QUINTUPLE {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::clone::Clone for XMIT_ROUTINE_QUINTUPLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::fmt::Debug for XMIT_ROUTINE_QUINTUPLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("XMIT_ROUTINE_QUINTUPLE").field("pfnTranslateToXmit", &self.pfnTranslateToXmit.map(|f| f as usize)).field("pfnTranslateFromXmit", &self.pfnTranslateFromXmit.map(|f| f as usize)).field("pfnFreeXmit", &self.pfnFreeXmit.map(|f| f as usize)).field("pfnFreeInst", &self.pfnFreeInst.map(|f| f as usize)).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::core::Abi for XMIT_ROUTINE_QUINTUPLE {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::PartialEq for XMIT_ROUTINE_QUINTUPLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<XMIT_ROUTINE_QUINTUPLE>()) == 0 }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::core::cmp::Eq for XMIT_ROUTINE_QUINTUPLE {}
#[cfg(feature = "Win32_System_Com")]
impl ::core::default::Default for XMIT_ROUTINE_QUINTUPLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
pub struct _NDR_ASYNC_MESSAGE(pub u8);
#[repr(C)]
pub struct _NDR_CORRELATION_INFO(pub u8);
#[repr(C)]
pub struct _NDR_PROC_CONTEXT(pub u8);
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub struct _NDR_SCONTEXT {
    pub pad: [*mut ::core::ffi::c_void; 2],
    pub userContext: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for _NDR_SCONTEXT {}
impl ::core::clone::Clone for _NDR_SCONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for _NDR_SCONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("_NDR_SCONTEXT").field("pad", &self.pad).field("userContext", &self.userContext).finish()
    }
}
unsafe impl ::windows::core::Abi for _NDR_SCONTEXT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for _NDR_SCONTEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<_NDR_SCONTEXT>()) == 0 }
    }
}
impl ::core::cmp::Eq for _NDR_SCONTEXT {}
impl ::core::default::Default for _NDR_SCONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const __RPCPROXY_H_VERSION__: u32 = 475u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const cbNDRContext: u32 = 20u32;
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct system_handle_t(pub i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const SYSTEM_HANDLE_FILE: system_handle_t = system_handle_t(0i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const SYSTEM_HANDLE_SEMAPHORE: system_handle_t = system_handle_t(1i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const SYSTEM_HANDLE_EVENT: system_handle_t = system_handle_t(2i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const SYSTEM_HANDLE_MUTEX: system_handle_t = system_handle_t(3i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const SYSTEM_HANDLE_PROCESS: system_handle_t = system_handle_t(4i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const SYSTEM_HANDLE_TOKEN: system_handle_t = system_handle_t(5i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const SYSTEM_HANDLE_SECTION: system_handle_t = system_handle_t(6i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const SYSTEM_HANDLE_REG_KEY: system_handle_t = system_handle_t(7i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const SYSTEM_HANDLE_THREAD: system_handle_t = system_handle_t(8i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const SYSTEM_HANDLE_COMPOSITION_OBJECT: system_handle_t = system_handle_t(9i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const SYSTEM_HANDLE_SOCKET: system_handle_t = system_handle_t(10i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const SYSTEM_HANDLE_JOB: system_handle_t = system_handle_t(11i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const SYSTEM_HANDLE_PIPE: system_handle_t = system_handle_t(12i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const SYSTEM_HANDLE_MAX: system_handle_t = system_handle_t(12i32);
#[doc = "*Required features: `\"Win32_System_Rpc\"`*"]
pub const SYSTEM_HANDLE_INVALID: system_handle_t = system_handle_t(255i32);
impl ::core::marker::Copy for system_handle_t {}
impl ::core::clone::Clone for system_handle_t {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for system_handle_t {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for system_handle_t {
    type Abi = Self;
}
impl ::core::fmt::Debug for system_handle_t {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("system_handle_t").field(&self.0).finish()
    }
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
