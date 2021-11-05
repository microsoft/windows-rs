#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct ARRAY_INFO {
    pub Dimension: i32,
    pub BufferConformanceMark: *mut u32,
    pub BufferVarianceMark: *mut u32,
    pub MaxCountArray: *mut u32,
    pub OffsetArray: *mut u32,
    pub ActualCountArray: *mut u32,
}
impl ARRAY_INFO {}
impl ::std::default::Default for ARRAY_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for ARRAY_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ARRAY_INFO")
            .field("Dimension", &self.Dimension)
            .field("BufferConformanceMark", &self.BufferConformanceMark)
            .field("BufferVarianceMark", &self.BufferVarianceMark)
            .field("MaxCountArray", &self.MaxCountArray)
            .field("OffsetArray", &self.OffsetArray)
            .field("ActualCountArray", &self.ActualCountArray)
            .finish()
    }
}
impl ::std::cmp::PartialEq for ARRAY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Dimension == other.Dimension && self.BufferConformanceMark == other.BufferConformanceMark && self.BufferVarianceMark == other.BufferVarianceMark && self.MaxCountArray == other.MaxCountArray && self.OffsetArray == other.OffsetArray && self.ActualCountArray == other.ActualCountArray
    }
}
impl ::std::cmp::Eq for ARRAY_INFO {}
unsafe impl ::windows::runtime::Abi for ARRAY_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct BinaryParam {
    pub Buffer: *mut ::std::ffi::c_void,
    pub Size: i16,
}
impl BinaryParam {}
impl ::std::default::Default for BinaryParam {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for BinaryParam {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("BinaryParam").field("Buffer", &self.Buffer).field("Size", &self.Size).finish()
    }
}
impl ::std::cmp::PartialEq for BinaryParam {
    fn eq(&self, other: &Self) -> bool {
        self.Buffer == other.Buffer && self.Size == other.Size
    }
}
impl ::std::cmp::Eq for BinaryParam {}
unsafe impl ::windows::runtime::Abi for BinaryParam {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub union CLIENT_CALL_RETURN {
    pub Pointer: *mut ::std::ffi::c_void,
    pub Simple: isize,
}
impl CLIENT_CALL_RETURN {}
impl ::std::default::Default for CLIENT_CALL_RETURN {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for CLIENT_CALL_RETURN {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for CLIENT_CALL_RETURN {}
unsafe impl ::windows::runtime::Abi for CLIENT_CALL_RETURN {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct COMM_FAULT_OFFSETS {
    pub CommOffset: i16,
    pub FaultOffset: i16,
}
impl COMM_FAULT_OFFSETS {}
impl ::std::default::Default for COMM_FAULT_OFFSETS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for COMM_FAULT_OFFSETS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("COMM_FAULT_OFFSETS").field("CommOffset", &self.CommOffset).field("FaultOffset", &self.FaultOffset).finish()
    }
}
impl ::std::cmp::PartialEq for COMM_FAULT_OFFSETS {
    fn eq(&self, other: &Self) -> bool {
        self.CommOffset == other.CommOffset && self.FaultOffset == other.FaultOffset
    }
}
impl ::std::cmp::Eq for COMM_FAULT_OFFSETS {}
unsafe impl ::windows::runtime::Abi for COMM_FAULT_OFFSETS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub type CS_TAG_GETTING_ROUTINE = unsafe extern "system" fn(hbinding: *mut ::std::ffi::c_void, fserverside: i32, pulsendingtag: *mut u32, puldesiredreceivingtag: *mut u32, pulreceivingtag: *mut u32, pstatus: *mut u32);
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub type CS_TYPE_FROM_NETCS_ROUTINE = unsafe extern "system" fn(hbinding: *mut ::std::ffi::c_void, ulnetworkcodeset: u32, pnetworkdata: *mut u8, ulnetworkdatalength: u32, ullocalbuffersize: u32, plocaldata: *mut ::std::ffi::c_void, pullocaldatalength: *mut u32, pstatus: *mut u32);
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub type CS_TYPE_LOCAL_SIZE_ROUTINE = unsafe extern "system" fn(hbinding: *mut ::std::ffi::c_void, ulnetworkcodeset: u32, ulnetworkbuffersize: u32, conversiontype: *mut IDL_CS_CONVERT, pullocalbuffersize: *mut u32, pstatus: *mut u32);
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub type CS_TYPE_NET_SIZE_ROUTINE = unsafe extern "system" fn(hbinding: *mut ::std::ffi::c_void, ulnetworkcodeset: u32, ullocalbuffersize: u32, conversiontype: *mut IDL_CS_CONVERT, pulnetworkbuffersize: *mut u32, pstatus: *mut u32);
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub type CS_TYPE_TO_NETCS_ROUTINE = unsafe extern "system" fn(hbinding: *mut ::std::ffi::c_void, ulnetworkcodeset: u32, plocaldata: *mut ::std::ffi::c_void, ullocaldatalength: u32, pnetworkdata: *mut u8, pulnetworkdatalength: *mut u32, pstatus: *mut u32);
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const DCE_C_ERROR_STRING_LEN: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn DceErrorInqTextA(rpcstatus: RPC_STATUS, errortext: *mut u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DceErrorInqTextA(rpcstatus: RPC_STATUS, errortext: *mut u8) -> RPC_STATUS;
        }
        ::std::mem::transmute(DceErrorInqTextA(::std::mem::transmute(rpcstatus), ::std::mem::transmute(errortext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn DceErrorInqTextW(rpcstatus: RPC_STATUS, errortext: *mut u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DceErrorInqTextW(rpcstatus: RPC_STATUS, errortext: *mut u16) -> RPC_STATUS;
        }
        ::std::mem::transmute(DceErrorInqTextW(::std::mem::transmute(rpcstatus), ::std::mem::transmute(errortext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const EEInfoGCCOM: u32 = 11u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const EEInfoGCFRS: u32 = 12u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const EEInfoNextRecordsMissing: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const EEInfoPreviousRecordsMissing: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const EEInfoUseFileTime: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
pub type EXPR_EVAL = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>);
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct EXPR_TOKEN(pub i32);
pub const FC_EXPR_START: EXPR_TOKEN = EXPR_TOKEN(0i32);
pub const FC_EXPR_ILLEGAL: EXPR_TOKEN = EXPR_TOKEN(0i32);
pub const FC_EXPR_CONST32: EXPR_TOKEN = EXPR_TOKEN(1i32);
pub const FC_EXPR_CONST64: EXPR_TOKEN = EXPR_TOKEN(2i32);
pub const FC_EXPR_VAR: EXPR_TOKEN = EXPR_TOKEN(3i32);
pub const FC_EXPR_OPER: EXPR_TOKEN = EXPR_TOKEN(4i32);
pub const FC_EXPR_NOOP: EXPR_TOKEN = EXPR_TOKEN(5i32);
pub const FC_EXPR_END: EXPR_TOKEN = EXPR_TOKEN(6i32);
impl ::std::convert::From<i32> for EXPR_TOKEN {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for EXPR_TOKEN {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct ExtendedErrorParamTypes(pub i32);
pub const eeptAnsiString: ExtendedErrorParamTypes = ExtendedErrorParamTypes(1i32);
pub const eeptUnicodeString: ExtendedErrorParamTypes = ExtendedErrorParamTypes(2i32);
pub const eeptLongVal: ExtendedErrorParamTypes = ExtendedErrorParamTypes(3i32);
pub const eeptShortVal: ExtendedErrorParamTypes = ExtendedErrorParamTypes(4i32);
pub const eeptPointerVal: ExtendedErrorParamTypes = ExtendedErrorParamTypes(5i32);
pub const eeptNone: ExtendedErrorParamTypes = ExtendedErrorParamTypes(6i32);
pub const eeptBinary: ExtendedErrorParamTypes = ExtendedErrorParamTypes(7i32);
impl ::std::convert::From<i32> for ExtendedErrorParamTypes {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for ExtendedErrorParamTypes {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct FULL_PTR_XLAT_TABLES {
    pub RefIdToPointer: *mut ::std::ffi::c_void,
    pub PointerToRefId: *mut ::std::ffi::c_void,
    pub NextRefId: u32,
    pub XlatSide: XLAT_SIDE,
}
impl FULL_PTR_XLAT_TABLES {}
impl ::std::default::Default for FULL_PTR_XLAT_TABLES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for FULL_PTR_XLAT_TABLES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("FULL_PTR_XLAT_TABLES").field("RefIdToPointer", &self.RefIdToPointer).field("PointerToRefId", &self.PointerToRefId).field("NextRefId", &self.NextRefId).field("XlatSide", &self.XlatSide).finish()
    }
}
impl ::std::cmp::PartialEq for FULL_PTR_XLAT_TABLES {
    fn eq(&self, other: &Self) -> bool {
        self.RefIdToPointer == other.RefIdToPointer && self.PointerToRefId == other.PointerToRefId && self.NextRefId == other.NextRefId && self.XlatSide == other.XlatSide
    }
}
impl ::std::cmp::Eq for FULL_PTR_XLAT_TABLES {}
unsafe impl ::windows::runtime::Abi for FULL_PTR_XLAT_TABLES {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct GENERIC_BINDING_INFO {
    pub pObj: *mut ::std::ffi::c_void,
    pub Size: u32,
    pub pfnBind: ::std::option::Option<GENERIC_BINDING_ROUTINE>,
    pub pfnUnbind: ::std::option::Option<GENERIC_UNBIND_ROUTINE>,
}
impl GENERIC_BINDING_INFO {}
impl ::std::default::Default for GENERIC_BINDING_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GENERIC_BINDING_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GENERIC_BINDING_INFO").field("pObj", &self.pObj).field("Size", &self.Size).finish()
    }
}
impl ::std::cmp::PartialEq for GENERIC_BINDING_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pObj == other.pObj && self.Size == other.Size && self.pfnBind.map(|f| f as usize) == other.pfnBind.map(|f| f as usize) && self.pfnUnbind.map(|f| f as usize) == other.pfnUnbind.map(|f| f as usize)
    }
}
impl ::std::cmp::Eq for GENERIC_BINDING_INFO {}
unsafe impl ::windows::runtime::Abi for GENERIC_BINDING_INFO {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub type GENERIC_BINDING_ROUTINE = unsafe extern "system" fn(param0: *mut ::std::ffi::c_void) -> *mut ::std::ffi::c_void;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct GENERIC_BINDING_ROUTINE_PAIR {
    pub pfnBind: ::std::option::Option<GENERIC_BINDING_ROUTINE>,
    pub pfnUnbind: ::std::option::Option<GENERIC_UNBIND_ROUTINE>,
}
impl GENERIC_BINDING_ROUTINE_PAIR {}
impl ::std::default::Default for GENERIC_BINDING_ROUTINE_PAIR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for GENERIC_BINDING_ROUTINE_PAIR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("GENERIC_BINDING_ROUTINE_PAIR").finish()
    }
}
impl ::std::cmp::PartialEq for GENERIC_BINDING_ROUTINE_PAIR {
    fn eq(&self, other: &Self) -> bool {
        self.pfnBind.map(|f| f as usize) == other.pfnBind.map(|f| f as usize) && self.pfnUnbind.map(|f| f as usize) == other.pfnUnbind.map(|f| f as usize)
    }
}
impl ::std::cmp::Eq for GENERIC_BINDING_ROUTINE_PAIR {}
unsafe impl ::windows::runtime::Abi for GENERIC_BINDING_ROUTINE_PAIR {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub type GENERIC_UNBIND_ROUTINE = unsafe extern "system" fn(param0: *mut ::std::ffi::c_void, param1: *mut u8);
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct GROUP_NAME_SYNTAX(pub u32);
pub const RPC_C_NS_SYNTAX_DEFAULT: GROUP_NAME_SYNTAX = GROUP_NAME_SYNTAX(0u32);
pub const RPC_C_NS_SYNTAX_DCE: GROUP_NAME_SYNTAX = GROUP_NAME_SYNTAX(3u32);
impl ::std::convert::From<u32> for GROUP_NAME_SYNTAX {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for GROUP_NAME_SYNTAX {
    type Abi = Self;
}
impl ::std::ops::BitOr for GROUP_NAME_SYNTAX {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for GROUP_NAME_SYNTAX {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for GROUP_NAME_SYNTAX {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for GROUP_NAME_SYNTAX {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for GROUP_NAME_SYNTAX {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct IDL_CS_CONVERT(pub i32);
pub const IDL_CS_NO_CONVERT: IDL_CS_CONVERT = IDL_CS_CONVERT(0i32);
pub const IDL_CS_IN_PLACE_CONVERT: IDL_CS_CONVERT = IDL_CS_CONVERT(1i32);
pub const IDL_CS_NEW_BUFFER_CONVERT: IDL_CS_CONVERT = IDL_CS_CONVERT(2i32);
impl ::std::convert::From<i32> for IDL_CS_CONVERT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for IDL_CS_CONVERT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const INVALID_FRAGMENT_ID: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn IUnknown_AddRef_Proxy<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(this: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IUnknown_AddRef_Proxy(this: ::windows::runtime::RawPtr) -> u32;
        }
        ::std::mem::transmute(IUnknown_AddRef_Proxy(this.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn IUnknown_QueryInterface_Proxy<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(this: Param0, riid: *const ::windows::runtime::GUID, ppvobject: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IUnknown_QueryInterface_Proxy(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppvobject: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT;
        }
        IUnknown_QueryInterface_Proxy(this.into_param().abi(), ::std::mem::transmute(riid), ::std::mem::transmute(ppvobject)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn IUnknown_Release_Proxy<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(this: Param0) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn IUnknown_Release_Proxy(this: ::windows::runtime::RawPtr) -> u32;
        }
        ::std::mem::transmute(IUnknown_Release_Proxy(this.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcAllocate(size: u32) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcAllocate(size: u32) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(I_RpcAllocate(::std::mem::transmute(size)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`, `Win32_System_IO`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn I_RpcAsyncAbortCall(pasync: *const RPC_ASYNC_STATE, exceptioncode: u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcAsyncAbortCall(pasync: *const ::std::mem::ManuallyDrop<RPC_ASYNC_STATE>, exceptioncode: u32) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcAsyncAbortCall(::std::mem::transmute(pasync), ::std::mem::transmute(exceptioncode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`, `Win32_System_IO`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn I_RpcAsyncSetHandle(message: *const RPC_MESSAGE, pasync: *const RPC_ASYNC_STATE) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcAsyncSetHandle(message: *const RPC_MESSAGE, pasync: *const ::std::mem::ManuallyDrop<RPC_ASYNC_STATE>) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcAsyncSetHandle(::std::mem::transmute(message), ::std::mem::transmute(pasync)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcBindingCopy(sourcebinding: *mut ::std::ffi::c_void, destinationbinding: *mut *mut ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcBindingCopy(sourcebinding: *mut ::std::ffi::c_void, destinationbinding: *mut *mut ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcBindingCopy(::std::mem::transmute(sourcebinding), ::std::mem::transmute(destinationbinding)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcBindingCreateNP(servername: *const u16, servicename: *const u16, networkoptions: *const u16, binding: *mut *mut ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcBindingCreateNP(servername: *const u16, servicename: *const u16, networkoptions: *const u16, binding: *mut *mut ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcBindingCreateNP(::std::mem::transmute(servername), ::std::mem::transmute(servicename), ::std::mem::transmute(networkoptions), ::std::mem::transmute(binding)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcBindingHandleToAsyncHandle(binding: *mut ::std::ffi::c_void, asynchandle: *mut *mut ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcBindingHandleToAsyncHandle(binding: *mut ::std::ffi::c_void, asynchandle: *mut *mut ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcBindingHandleToAsyncHandle(::std::mem::transmute(binding), ::std::mem::transmute(asynchandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn I_RpcBindingInqClientTokenAttributes(binding: *const ::std::ffi::c_void, tokenid: *mut super::super::Foundation::LUID, authenticationid: *mut super::super::Foundation::LUID, modifiedid: *mut super::super::Foundation::LUID) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcBindingInqClientTokenAttributes(binding: *const ::std::ffi::c_void, tokenid: *mut super::super::Foundation::LUID, authenticationid: *mut super::super::Foundation::LUID, modifiedid: *mut super::super::Foundation::LUID) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcBindingInqClientTokenAttributes(::std::mem::transmute(binding), ::std::mem::transmute(tokenid), ::std::mem::transmute(authenticationid), ::std::mem::transmute(modifiedid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcBindingInqDynamicEndpointA(binding: *const ::std::ffi::c_void, dynamicendpoint: *mut *mut u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcBindingInqDynamicEndpointA(binding: *const ::std::ffi::c_void, dynamicendpoint: *mut *mut u8) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcBindingInqDynamicEndpointA(::std::mem::transmute(binding), ::std::mem::transmute(dynamicendpoint)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcBindingInqDynamicEndpointW(binding: *const ::std::ffi::c_void, dynamicendpoint: *mut *mut u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcBindingInqDynamicEndpointW(binding: *const ::std::ffi::c_void, dynamicendpoint: *mut *mut u16) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcBindingInqDynamicEndpointW(::std::mem::transmute(binding), ::std::mem::transmute(dynamicendpoint)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcBindingInqLocalClientPID(binding: *mut ::std::ffi::c_void, pid: *mut u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcBindingInqLocalClientPID(binding: *mut ::std::ffi::c_void, pid: *mut u32) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcBindingInqLocalClientPID(::std::mem::transmute(binding), ::std::mem::transmute(pid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcBindingInqMarshalledTargetInfo(binding: *const ::std::ffi::c_void, marshalledtargetinfosize: *mut u32, marshalledtargetinfo: *mut *mut u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcBindingInqMarshalledTargetInfo(binding: *const ::std::ffi::c_void, marshalledtargetinfosize: *mut u32, marshalledtargetinfo: *mut *mut u8) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcBindingInqMarshalledTargetInfo(::std::mem::transmute(binding), ::std::mem::transmute(marshalledtargetinfosize), ::std::mem::transmute(marshalledtargetinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcBindingInqSecurityContext(binding: *mut ::std::ffi::c_void, securitycontexthandle: *mut *mut ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcBindingInqSecurityContext(binding: *mut ::std::ffi::c_void, securitycontexthandle: *mut *mut ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcBindingInqSecurityContext(::std::mem::transmute(binding), ::std::mem::transmute(securitycontexthandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcBindingInqSecurityContextKeyInfo(binding: *const ::std::ffi::c_void, keyinfo: *mut ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcBindingInqSecurityContextKeyInfo(binding: *const ::std::ffi::c_void, keyinfo: *mut ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcBindingInqSecurityContextKeyInfo(::std::mem::transmute(binding), ::std::mem::transmute(keyinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcBindingInqTransportType(binding: *mut ::std::ffi::c_void, r#type: *mut u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcBindingInqTransportType(binding: *mut ::std::ffi::c_void, r#type: *mut u32) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcBindingInqTransportType(::std::mem::transmute(binding), ::std::mem::transmute(r#type)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcBindingInqWireIdForSnego(binding: *const ::std::ffi::c_void, wireid: *mut u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcBindingInqWireIdForSnego(binding: *const ::std::ffi::c_void, wireid: *mut u8) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcBindingInqWireIdForSnego(::std::mem::transmute(binding), ::std::mem::transmute(wireid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcBindingIsClientLocal(bindinghandle: *mut ::std::ffi::c_void, clientlocalflag: *mut u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcBindingIsClientLocal(bindinghandle: *mut ::std::ffi::c_void, clientlocalflag: *mut u32) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcBindingIsClientLocal(::std::mem::transmute(bindinghandle), ::std::mem::transmute(clientlocalflag)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcBindingIsServerLocal(binding: *const ::std::ffi::c_void, serverlocalflag: *mut u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcBindingIsServerLocal(binding: *const ::std::ffi::c_void, serverlocalflag: *mut u32) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcBindingIsServerLocal(::std::mem::transmute(binding), ::std::mem::transmute(serverlocalflag)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcBindingSetPrivateOption(hbinding: *const ::std::ffi::c_void, option: u32, optionvalue: usize) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcBindingSetPrivateOption(hbinding: *const ::std::ffi::c_void, option: u32, optionvalue: usize) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcBindingSetPrivateOption(::std::mem::transmute(hbinding), ::std::mem::transmute(option), ::std::mem::transmute(optionvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcBindingToStaticStringBindingW(binding: *mut ::std::ffi::c_void, stringbinding: *mut *mut u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcBindingToStaticStringBindingW(binding: *mut ::std::ffi::c_void, stringbinding: *mut *mut u16) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcBindingToStaticStringBindingW(::std::mem::transmute(binding), ::std::mem::transmute(stringbinding)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcClearMutex(mutex: *mut ::std::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcClearMutex(mutex: *mut ::std::ffi::c_void);
        }
        ::std::mem::transmute(I_RpcClearMutex(::std::mem::transmute(mutex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcDeleteMutex(mutex: *mut ::std::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcDeleteMutex(mutex: *mut ::std::ffi::c_void);
        }
        ::std::mem::transmute(I_RpcDeleteMutex(::std::mem::transmute(mutex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcExceptionFilter(exceptioncode: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcExceptionFilter(exceptioncode: u32) -> i32;
        }
        ::std::mem::transmute(I_RpcExceptionFilter(::std::mem::transmute(exceptioncode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcFree(object: *mut ::std::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcFree(object: *mut ::std::ffi::c_void);
        }
        ::std::mem::transmute(I_RpcFree(::std::mem::transmute(object)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcFreeBuffer(message: *mut RPC_MESSAGE) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcFreeBuffer(message: *mut RPC_MESSAGE) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcFreeBuffer(::std::mem::transmute(message)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub type I_RpcFreeCalloutStateFn = unsafe extern "system" fn(calloutstate: *mut RDR_CALLOUT_STATE);
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcFreePipeBuffer(message: *mut RPC_MESSAGE) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcFreePipeBuffer(message: *mut RPC_MESSAGE) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcFreePipeBuffer(::std::mem::transmute(message)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcGetBuffer(message: *mut RPC_MESSAGE) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcGetBuffer(message: *mut RPC_MESSAGE) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcGetBuffer(::std::mem::transmute(message)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcGetBufferWithObject(message: *mut RPC_MESSAGE, objectuuid: *mut ::windows::runtime::GUID) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcGetBufferWithObject(message: *mut RPC_MESSAGE, objectuuid: *mut ::windows::runtime::GUID) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcGetBufferWithObject(::std::mem::transmute(message), ::std::mem::transmute(objectuuid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcGetCurrentCallHandle() -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcGetCurrentCallHandle() -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(I_RpcGetCurrentCallHandle())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcGetDefaultSD(ppsecuritydescriptor: *mut *mut ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcGetDefaultSD(ppsecuritydescriptor: *mut *mut ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcGetDefaultSD(::std::mem::transmute(ppsecuritydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcGetExtendedError() -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcGetExtendedError() -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcGetExtendedError())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcIfInqTransferSyntaxes(rpcifhandle: *mut ::std::ffi::c_void, transfersyntaxes: *mut RPC_TRANSFER_SYNTAX, transfersyntaxsize: u32, transfersyntaxcount: *mut u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcIfInqTransferSyntaxes(rpcifhandle: *mut ::std::ffi::c_void, transfersyntaxes: *mut RPC_TRANSFER_SYNTAX, transfersyntaxsize: u32, transfersyntaxcount: *mut u32) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcIfInqTransferSyntaxes(::std::mem::transmute(rpcifhandle), ::std::mem::transmute(transfersyntaxes), ::std::mem::transmute(transfersyntaxsize), ::std::mem::transmute(transfersyntaxcount)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcMapWin32Status(status: RPC_STATUS) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcMapWin32Status(status: RPC_STATUS) -> i32;
        }
        ::std::mem::transmute(I_RpcMapWin32Status(::std::mem::transmute(status)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcMgmtEnableDedicatedThreadPool() -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcMgmtEnableDedicatedThreadPool() -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcMgmtEnableDedicatedThreadPool())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcNegotiateTransferSyntax(message: *mut RPC_MESSAGE) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcNegotiateTransferSyntax(message: *mut RPC_MESSAGE) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcNegotiateTransferSyntax(::std::mem::transmute(message)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcNsBindingSetEntryNameA(binding: *const ::std::ffi::c_void, entrynamesyntax: u32, entryname: *const u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcNsBindingSetEntryNameA(binding: *const ::std::ffi::c_void, entrynamesyntax: u32, entryname: *const u8) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcNsBindingSetEntryNameA(::std::mem::transmute(binding), ::std::mem::transmute(entrynamesyntax), ::std::mem::transmute(entryname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcNsBindingSetEntryNameW(binding: *const ::std::ffi::c_void, entrynamesyntax: u32, entryname: *const u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcNsBindingSetEntryNameW(binding: *const ::std::ffi::c_void, entrynamesyntax: u32, entryname: *const u16) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcNsBindingSetEntryNameW(::std::mem::transmute(binding), ::std::mem::transmute(entrynamesyntax), ::std::mem::transmute(entryname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcNsInterfaceExported(entrynamesyntax: u32, entryname: *mut u16, rpcinterfaceinformation: *mut RPC_SERVER_INTERFACE) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcNsInterfaceExported(entrynamesyntax: u32, entryname: *mut u16, rpcinterfaceinformation: *mut RPC_SERVER_INTERFACE) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcNsInterfaceExported(::std::mem::transmute(entrynamesyntax), ::std::mem::transmute(entryname), ::std::mem::transmute(rpcinterfaceinformation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcNsInterfaceUnexported(entrynamesyntax: u32, entryname: *mut u16, rpcinterfaceinformation: *mut RPC_SERVER_INTERFACE) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcNsInterfaceUnexported(entrynamesyntax: u32, entryname: *mut u16, rpcinterfaceinformation: *mut RPC_SERVER_INTERFACE) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcNsInterfaceUnexported(::std::mem::transmute(entrynamesyntax), ::std::mem::transmute(entryname), ::std::mem::transmute(rpcinterfaceinformation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcOpenClientProcess(binding: *const ::std::ffi::c_void, desiredaccess: u32, clientprocess: *mut *mut ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcOpenClientProcess(binding: *const ::std::ffi::c_void, desiredaccess: u32, clientprocess: *mut *mut ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcOpenClientProcess(::std::mem::transmute(binding), ::std::mem::transmute(desiredaccess), ::std::mem::transmute(clientprocess)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcPauseExecution(milliseconds: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcPauseExecution(milliseconds: u32);
        }
        ::std::mem::transmute(I_RpcPauseExecution(::std::mem::transmute(milliseconds)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub type I_RpcPerformCalloutFn = unsafe extern "system" fn(context: *mut ::std::ffi::c_void, calloutstate: *mut RDR_CALLOUT_STATE, stage: RPC_HTTP_REDIRECTOR_STAGE) -> RPC_STATUS;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`*"]
pub struct I_RpcProxyCallbackInterface {
    pub IsValidMachineFn: ::std::option::Option<I_RpcProxyIsValidMachineFn>,
    pub GetClientAddressFn: ::std::option::Option<I_RpcProxyGetClientAddressFn>,
    pub GetConnectionTimeoutFn: ::std::option::Option<I_RpcProxyGetConnectionTimeoutFn>,
    pub PerformCalloutFn: ::std::option::Option<I_RpcPerformCalloutFn>,
    pub FreeCalloutStateFn: ::std::option::Option<I_RpcFreeCalloutStateFn>,
    pub GetClientSessionAndResourceUUIDFn: ::std::option::Option<I_RpcProxyGetClientSessionAndResourceUUID>,
    pub ProxyFilterIfFn: ::std::option::Option<I_RpcProxyFilterIfFn>,
    pub RpcProxyUpdatePerfCounterFn: ::std::option::Option<I_RpcProxyUpdatePerfCounterFn>,
    pub RpcProxyUpdatePerfCounterBackendServerFn: ::std::option::Option<I_RpcProxyUpdatePerfCounterBackendServerFn>,
}
#[cfg(feature = "Win32_Foundation")]
impl I_RpcProxyCallbackInterface {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for I_RpcProxyCallbackInterface {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for I_RpcProxyCallbackInterface {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("I_RpcProxyCallbackInterface").finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for I_RpcProxyCallbackInterface {
    fn eq(&self, other: &Self) -> bool {
        self.IsValidMachineFn.map(|f| f as usize) == other.IsValidMachineFn.map(|f| f as usize)
            && self.GetClientAddressFn.map(|f| f as usize) == other.GetClientAddressFn.map(|f| f as usize)
            && self.GetConnectionTimeoutFn.map(|f| f as usize) == other.GetConnectionTimeoutFn.map(|f| f as usize)
            && self.PerformCalloutFn.map(|f| f as usize) == other.PerformCalloutFn.map(|f| f as usize)
            && self.FreeCalloutStateFn.map(|f| f as usize) == other.FreeCalloutStateFn.map(|f| f as usize)
            && self.GetClientSessionAndResourceUUIDFn.map(|f| f as usize) == other.GetClientSessionAndResourceUUIDFn.map(|f| f as usize)
            && self.ProxyFilterIfFn.map(|f| f as usize) == other.ProxyFilterIfFn.map(|f| f as usize)
            && self.RpcProxyUpdatePerfCounterFn.map(|f| f as usize) == other.RpcProxyUpdatePerfCounterFn.map(|f| f as usize)
            && self.RpcProxyUpdatePerfCounterBackendServerFn.map(|f| f as usize) == other.RpcProxyUpdatePerfCounterBackendServerFn.map(|f| f as usize)
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for I_RpcProxyCallbackInterface {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for I_RpcProxyCallbackInterface {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub type I_RpcProxyFilterIfFn = unsafe extern "system" fn(context: *const ::std::ffi::c_void, ifuuid: *const ::windows::runtime::GUID, ifmajorversion: u16, fallow: *mut i32) -> RPC_STATUS;
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type I_RpcProxyGetClientAddressFn = unsafe extern "system" fn(context: *mut ::std::ffi::c_void, buffer: super::super::Foundation::PSTR, bufferlength: *mut u32) -> RPC_STATUS;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub type I_RpcProxyGetClientSessionAndResourceUUID = unsafe extern "system" fn(context: *const ::std::ffi::c_void, sessionidpresent: *mut i32, sessionid: *mut ::windows::runtime::GUID, resourceidpresent: *mut i32, resourceid: *mut ::windows::runtime::GUID) -> RPC_STATUS;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub type I_RpcProxyGetConnectionTimeoutFn = unsafe extern "system" fn(connectiontimeout: *mut u32) -> RPC_STATUS;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub type I_RpcProxyIsValidMachineFn = unsafe extern "system" fn(machine: *const u16, dotmachine: *const u16, portnumber: u32) -> RPC_STATUS;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub type I_RpcProxyUpdatePerfCounterBackendServerFn = unsafe extern "system" fn(machinename: *const u16, isconnectevent: i32);
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub type I_RpcProxyUpdatePerfCounterFn = unsafe extern "system" fn(counter: RpcProxyPerfCounters, modifytrend: i32, size: u32);
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcReallocPipeBuffer(message: *const RPC_MESSAGE, newsize: u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcReallocPipeBuffer(message: *const RPC_MESSAGE, newsize: u32) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcReallocPipeBuffer(::std::mem::transmute(message), ::std::mem::transmute(newsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcReceive(message: *mut RPC_MESSAGE, size: u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcReceive(message: *mut RPC_MESSAGE, size: u32) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcReceive(::std::mem::transmute(message), ::std::mem::transmute(size)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcRecordCalloutFailure(rpcstatus: RPC_STATUS, calloutstate: *mut RDR_CALLOUT_STATE, dllname: *mut u16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcRecordCalloutFailure(rpcstatus: RPC_STATUS, calloutstate: *mut RDR_CALLOUT_STATE, dllname: *mut u16);
        }
        ::std::mem::transmute(I_RpcRecordCalloutFailure(::std::mem::transmute(rpcstatus), ::std::mem::transmute(calloutstate), ::std::mem::transmute(dllname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcRequestMutex(mutex: *mut *mut ::std::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcRequestMutex(mutex: *mut *mut ::std::ffi::c_void);
        }
        ::std::mem::transmute(I_RpcRequestMutex(::std::mem::transmute(mutex)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcSend(message: *mut RPC_MESSAGE) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcSend(message: *mut RPC_MESSAGE) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcSend(::std::mem::transmute(message)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcSendReceive(message: *mut RPC_MESSAGE) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcSendReceive(message: *mut RPC_MESSAGE) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcSendReceive(::std::mem::transmute(message)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcServerCheckClientRestriction(context: *mut ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcServerCheckClientRestriction(context: *mut ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcServerCheckClientRestriction(::std::mem::transmute(context)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcServerDisableExceptionFilter() -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcServerDisableExceptionFilter() -> i32;
        }
        ::std::mem::transmute(I_RpcServerDisableExceptionFilter())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcServerGetAssociationID(binding: *const ::std::ffi::c_void, associationid: *mut u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcServerGetAssociationID(binding: *const ::std::ffi::c_void, associationid: *mut u32) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcServerGetAssociationID(::std::mem::transmute(binding), ::std::mem::transmute(associationid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcServerInqAddressChangeFn() -> *mut ::std::option::Option<RPC_ADDRESS_CHANGE_FN> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcServerInqAddressChangeFn() -> *mut ::std::option::Option<RPC_ADDRESS_CHANGE_FN>;
        }
        ::std::mem::transmute(I_RpcServerInqAddressChangeFn())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcServerInqLocalConnAddress(binding: *mut ::std::ffi::c_void, buffer: *mut ::std::ffi::c_void, buffersize: *mut u32, addressformat: *mut u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcServerInqLocalConnAddress(binding: *mut ::std::ffi::c_void, buffer: *mut ::std::ffi::c_void, buffersize: *mut u32, addressformat: *mut u32) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcServerInqLocalConnAddress(::std::mem::transmute(binding), ::std::mem::transmute(buffer), ::std::mem::transmute(buffersize), ::std::mem::transmute(addressformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcServerInqRemoteConnAddress(binding: *mut ::std::ffi::c_void, buffer: *mut ::std::ffi::c_void, buffersize: *mut u32, addressformat: *mut u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcServerInqRemoteConnAddress(binding: *mut ::std::ffi::c_void, buffer: *mut ::std::ffi::c_void, buffersize: *mut u32, addressformat: *mut u32) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcServerInqRemoteConnAddress(::std::mem::transmute(binding), ::std::mem::transmute(buffer), ::std::mem::transmute(buffersize), ::std::mem::transmute(addressformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcServerInqTransportType(r#type: *mut u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcServerInqTransportType(r#type: *mut u32) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcServerInqTransportType(::std::mem::transmute(r#type)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcServerRegisterForwardFunction(pforwardfunction: *mut ::std::option::Option<RPC_FORWARD_FUNCTION>) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcServerRegisterForwardFunction(pforwardfunction: *mut ::windows::runtime::RawPtr) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcServerRegisterForwardFunction(::std::mem::transmute(pforwardfunction)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcServerSetAddressChangeFn(paddresschangefn: *mut ::std::option::Option<RPC_ADDRESS_CHANGE_FN>) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcServerSetAddressChangeFn(paddresschangefn: *mut ::windows::runtime::RawPtr) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcServerSetAddressChangeFn(::std::mem::transmute(paddresschangefn)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcServerStartService(protseq: *const u16, endpoint: *const u16, ifspec: *const ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcServerStartService(protseq: *const u16, endpoint: *const u16, ifspec: *const ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcServerStartService(::std::mem::transmute(protseq), ::std::mem::transmute(endpoint), ::std::mem::transmute(ifspec)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcServerSubscribeForDisconnectNotification(binding: *const ::std::ffi::c_void, hevent: *const ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcServerSubscribeForDisconnectNotification(binding: *const ::std::ffi::c_void, hevent: *const ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcServerSubscribeForDisconnectNotification(::std::mem::transmute(binding), ::std::mem::transmute(hevent)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcServerSubscribeForDisconnectNotification2(binding: *const ::std::ffi::c_void, hevent: *const ::std::ffi::c_void, subscriptionid: *mut ::windows::runtime::GUID) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcServerSubscribeForDisconnectNotification2(binding: *const ::std::ffi::c_void, hevent: *const ::std::ffi::c_void, subscriptionid: *mut ::windows::runtime::GUID) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcServerSubscribeForDisconnectNotification2(::std::mem::transmute(binding), ::std::mem::transmute(hevent), ::std::mem::transmute(subscriptionid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcServerUnsubscribeForDisconnectNotification<'a, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::GUID>>(binding: *const ::std::ffi::c_void, subscriptionid: Param1) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcServerUnsubscribeForDisconnectNotification(binding: *const ::std::ffi::c_void, subscriptionid: ::windows::runtime::GUID) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcServerUnsubscribeForDisconnectNotification(::std::mem::transmute(binding), subscriptionid.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcServerUseProtseq2A(networkaddress: *const u8, protseq: *const u8, maxcalls: u32, securitydescriptor: *const ::std::ffi::c_void, policy: *const ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcServerUseProtseq2A(networkaddress: *const u8, protseq: *const u8, maxcalls: u32, securitydescriptor: *const ::std::ffi::c_void, policy: *const ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcServerUseProtseq2A(::std::mem::transmute(networkaddress), ::std::mem::transmute(protseq), ::std::mem::transmute(maxcalls), ::std::mem::transmute(securitydescriptor), ::std::mem::transmute(policy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcServerUseProtseq2W(networkaddress: *const u16, protseq: *const u16, maxcalls: u32, securitydescriptor: *const ::std::ffi::c_void, policy: *const ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcServerUseProtseq2W(networkaddress: *const u16, protseq: *const u16, maxcalls: u32, securitydescriptor: *const ::std::ffi::c_void, policy: *const ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcServerUseProtseq2W(::std::mem::transmute(networkaddress), ::std::mem::transmute(protseq), ::std::mem::transmute(maxcalls), ::std::mem::transmute(securitydescriptor), ::std::mem::transmute(policy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcServerUseProtseqEp2A(networkaddress: *const u8, protseq: *const u8, maxcalls: u32, endpoint: *const u8, securitydescriptor: *const ::std::ffi::c_void, policy: *const ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcServerUseProtseqEp2A(networkaddress: *const u8, protseq: *const u8, maxcalls: u32, endpoint: *const u8, securitydescriptor: *const ::std::ffi::c_void, policy: *const ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcServerUseProtseqEp2A(::std::mem::transmute(networkaddress), ::std::mem::transmute(protseq), ::std::mem::transmute(maxcalls), ::std::mem::transmute(endpoint), ::std::mem::transmute(securitydescriptor), ::std::mem::transmute(policy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcServerUseProtseqEp2W(networkaddress: *const u16, protseq: *const u16, maxcalls: u32, endpoint: *const u16, securitydescriptor: *const ::std::ffi::c_void, policy: *const ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcServerUseProtseqEp2W(networkaddress: *const u16, protseq: *const u16, maxcalls: u32, endpoint: *const u16, securitydescriptor: *const ::std::ffi::c_void, policy: *const ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcServerUseProtseqEp2W(::std::mem::transmute(networkaddress), ::std::mem::transmute(protseq), ::std::mem::transmute(maxcalls), ::std::mem::transmute(endpoint), ::std::mem::transmute(securitydescriptor), ::std::mem::transmute(policy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcSessionStrictContextHandle() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcSessionStrictContextHandle();
        }
        ::std::mem::transmute(I_RpcSessionStrictContextHandle())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcSsDontSerializeContext() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcSsDontSerializeContext();
        }
        ::std::mem::transmute(I_RpcSsDontSerializeContext())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcSystemHandleTypeSpecificWork(handle: *mut ::std::ffi::c_void, actualtype: u8, idltype: u8, marshaldirection: LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcSystemHandleTypeSpecificWork(handle: *mut ::std::ffi::c_void, actualtype: u8, idltype: u8, marshaldirection: LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcSystemHandleTypeSpecificWork(::std::mem::transmute(handle), ::std::mem::transmute(actualtype), ::std::mem::transmute(idltype), ::std::mem::transmute(marshaldirection)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_RpcTurnOnEEInfoPropagation() -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_RpcTurnOnEEInfoPropagation() -> RPC_STATUS;
        }
        ::std::mem::transmute(I_RpcTurnOnEEInfoPropagation())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn I_UuidCreate(uuid: *mut ::windows::runtime::GUID) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn I_UuidCreate(uuid: *mut ::windows::runtime::GUID) -> RPC_STATUS;
        }
        ::std::mem::transmute(I_UuidCreate(::std::mem::transmute(uuid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION(pub i32);
pub const MarshalDirectionMarshal: LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION = LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION(0i32);
pub const MarshalDirectionUnmarshal: LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION = LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION(1i32);
impl ::std::convert::From<i32> for LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct MALLOC_FREE_STRUCT {
    pub pfnAllocate: isize,
    pub pfnFree: isize,
}
impl MALLOC_FREE_STRUCT {}
impl ::std::default::Default for MALLOC_FREE_STRUCT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MALLOC_FREE_STRUCT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MALLOC_FREE_STRUCT").field("pfnAllocate", &self.pfnAllocate).field("pfnFree", &self.pfnFree).finish()
    }
}
impl ::std::cmp::PartialEq for MALLOC_FREE_STRUCT {
    fn eq(&self, other: &Self) -> bool {
        self.pfnAllocate == other.pfnAllocate && self.pfnFree == other.pfnFree
    }
}
impl ::std::cmp::Eq for MALLOC_FREE_STRUCT {}
unsafe impl ::windows::runtime::Abi for MALLOC_FREE_STRUCT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub type MIDL_ES_ALLOC = unsafe extern "system" fn(state: *mut ::std::ffi::c_void, pbuffer: *mut *mut i8, psize: *mut u32);
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MIDL_ES_CODE(pub i32);
pub const MES_ENCODE: MIDL_ES_CODE = MIDL_ES_CODE(0i32);
pub const MES_DECODE: MIDL_ES_CODE = MIDL_ES_CODE(1i32);
pub const MES_ENCODE_NDR64: MIDL_ES_CODE = MIDL_ES_CODE(2i32);
impl ::std::convert::From<i32> for MIDL_ES_CODE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MIDL_ES_CODE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct MIDL_ES_HANDLE_STYLE(pub i32);
pub const MES_INCREMENTAL_HANDLE: MIDL_ES_HANDLE_STYLE = MIDL_ES_HANDLE_STYLE(0i32);
pub const MES_FIXED_BUFFER_HANDLE: MIDL_ES_HANDLE_STYLE = MIDL_ES_HANDLE_STYLE(1i32);
pub const MES_DYNAMIC_BUFFER_HANDLE: MIDL_ES_HANDLE_STYLE = MIDL_ES_HANDLE_STYLE(2i32);
impl ::std::convert::From<i32> for MIDL_ES_HANDLE_STYLE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for MIDL_ES_HANDLE_STYLE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub type MIDL_ES_READ = unsafe extern "system" fn(state: *mut ::std::ffi::c_void, pbuffer: *mut *mut i8, psize: *mut u32);
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
pub type MIDL_ES_WRITE = unsafe extern "system" fn(state: *mut ::std::ffi::c_void, buffer: super::super::Foundation::PSTR, size: u32);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct MIDL_FORMAT_STRING {
    pub Pad: i16,
    pub Format: [u8; 1],
}
impl MIDL_FORMAT_STRING {}
impl ::std::default::Default for MIDL_FORMAT_STRING {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MIDL_FORMAT_STRING {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MIDL_FORMAT_STRING").field("Pad", &self.Pad).field("Format", &self.Format).finish()
    }
}
impl ::std::cmp::PartialEq for MIDL_FORMAT_STRING {
    fn eq(&self, other: &Self) -> bool {
        self.Pad == other.Pad && self.Format == other.Format
    }
}
impl ::std::cmp::Eq for MIDL_FORMAT_STRING {}
unsafe impl ::windows::runtime::Abi for MIDL_FORMAT_STRING {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct MIDL_INTERCEPTION_INFO {
    pub Version: u32,
    pub ProcString: *mut u8,
    pub ProcFormatOffsetTable: *mut u16,
    pub ProcCount: u32,
    pub TypeString: *mut u8,
}
impl MIDL_INTERCEPTION_INFO {}
impl ::std::default::Default for MIDL_INTERCEPTION_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MIDL_INTERCEPTION_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MIDL_INTERCEPTION_INFO").field("Version", &self.Version).field("ProcString", &self.ProcString).field("ProcFormatOffsetTable", &self.ProcFormatOffsetTable).field("ProcCount", &self.ProcCount).field("TypeString", &self.TypeString).finish()
    }
}
impl ::std::cmp::PartialEq for MIDL_INTERCEPTION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.ProcString == other.ProcString && self.ProcFormatOffsetTable == other.ProcFormatOffsetTable && self.ProcCount == other.ProcCount && self.TypeString == other.TypeString
    }
}
impl ::std::cmp::Eq for MIDL_INTERCEPTION_INFO {}
unsafe impl ::windows::runtime::Abi for MIDL_INTERCEPTION_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct MIDL_INTERFACE_METHOD_PROPERTIES {
    pub MethodCount: u16,
    pub MethodProperties: *mut *mut MIDL_METHOD_PROPERTY_MAP,
}
impl MIDL_INTERFACE_METHOD_PROPERTIES {}
impl ::std::default::Default for MIDL_INTERFACE_METHOD_PROPERTIES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MIDL_INTERFACE_METHOD_PROPERTIES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MIDL_INTERFACE_METHOD_PROPERTIES").field("MethodCount", &self.MethodCount).field("MethodProperties", &self.MethodProperties).finish()
    }
}
impl ::std::cmp::PartialEq for MIDL_INTERFACE_METHOD_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.MethodCount == other.MethodCount && self.MethodProperties == other.MethodProperties
    }
}
impl ::std::cmp::Eq for MIDL_INTERFACE_METHOD_PROPERTIES {}
unsafe impl ::windows::runtime::Abi for MIDL_INTERFACE_METHOD_PROPERTIES {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct MIDL_METHOD_PROPERTY {
    pub Id: u32,
    pub Value: usize,
}
impl MIDL_METHOD_PROPERTY {}
impl ::std::default::Default for MIDL_METHOD_PROPERTY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MIDL_METHOD_PROPERTY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MIDL_METHOD_PROPERTY").field("Id", &self.Id).field("Value", &self.Value).finish()
    }
}
impl ::std::cmp::PartialEq for MIDL_METHOD_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.Id == other.Id && self.Value == other.Value
    }
}
impl ::std::cmp::Eq for MIDL_METHOD_PROPERTY {}
unsafe impl ::windows::runtime::Abi for MIDL_METHOD_PROPERTY {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct MIDL_METHOD_PROPERTY_MAP {
    pub Count: u32,
    pub Properties: *mut MIDL_METHOD_PROPERTY,
}
impl MIDL_METHOD_PROPERTY_MAP {}
impl ::std::default::Default for MIDL_METHOD_PROPERTY_MAP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MIDL_METHOD_PROPERTY_MAP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MIDL_METHOD_PROPERTY_MAP").field("Count", &self.Count).field("Properties", &self.Properties).finish()
    }
}
impl ::std::cmp::PartialEq for MIDL_METHOD_PROPERTY_MAP {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.Properties == other.Properties
    }
}
impl ::std::cmp::Eq for MIDL_METHOD_PROPERTY_MAP {}
unsafe impl ::windows::runtime::Abi for MIDL_METHOD_PROPERTY_MAP {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
pub struct MIDL_SERVER_INFO {
    pub pStubDesc: *mut MIDL_STUB_DESC,
    pub DispatchTable: *mut ::std::option::Option<SERVER_ROUTINE>,
    pub ProcString: *mut u8,
    pub FmtStringOffset: *mut u16,
    pub ThunkTable: *mut ::std::option::Option<STUB_THUNK>,
    pub pTransferSyntax: *mut RPC_SYNTAX_IDENTIFIER,
    pub nCount: usize,
    pub pSyntaxInfo: *mut MIDL_SYNTAX_INFO,
}
#[cfg(feature = "Win32_System_Com")]
impl MIDL_SERVER_INFO {}
#[cfg(feature = "Win32_System_Com")]
impl ::std::default::Default for MIDL_SERVER_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::fmt::Debug for MIDL_SERVER_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MIDL_SERVER_INFO").field("pStubDesc", &self.pStubDesc).field("ProcString", &self.ProcString).field("FmtStringOffset", &self.FmtStringOffset).field("pTransferSyntax", &self.pTransferSyntax).field("nCount", &self.nCount).field("pSyntaxInfo", &self.pSyntaxInfo).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::PartialEq for MIDL_SERVER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pStubDesc == other.pStubDesc && self.DispatchTable == other.DispatchTable && self.ProcString == other.ProcString && self.FmtStringOffset == other.FmtStringOffset && self.ThunkTable == other.ThunkTable && self.pTransferSyntax == other.pTransferSyntax && self.nCount == other.nCount && self.pSyntaxInfo == other.pSyntaxInfo
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::Eq for MIDL_SERVER_INFO {}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::runtime::Abi for MIDL_SERVER_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
pub struct MIDL_STUBLESS_PROXY_INFO {
    pub pStubDesc: *mut MIDL_STUB_DESC,
    pub ProcFormatString: *mut u8,
    pub FormatStringOffset: *mut u16,
    pub pTransferSyntax: *mut RPC_SYNTAX_IDENTIFIER,
    pub nCount: usize,
    pub pSyntaxInfo: *mut MIDL_SYNTAX_INFO,
}
#[cfg(feature = "Win32_System_Com")]
impl MIDL_STUBLESS_PROXY_INFO {}
#[cfg(feature = "Win32_System_Com")]
impl ::std::default::Default for MIDL_STUBLESS_PROXY_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::fmt::Debug for MIDL_STUBLESS_PROXY_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MIDL_STUBLESS_PROXY_INFO")
            .field("pStubDesc", &self.pStubDesc)
            .field("ProcFormatString", &self.ProcFormatString)
            .field("FormatStringOffset", &self.FormatStringOffset)
            .field("pTransferSyntax", &self.pTransferSyntax)
            .field("nCount", &self.nCount)
            .field("pSyntaxInfo", &self.pSyntaxInfo)
            .finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::PartialEq for MIDL_STUBLESS_PROXY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.pStubDesc == other.pStubDesc && self.ProcFormatString == other.ProcFormatString && self.FormatStringOffset == other.FormatStringOffset && self.pTransferSyntax == other.pTransferSyntax && self.nCount == other.nCount && self.pSyntaxInfo == other.pSyntaxInfo
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::Eq for MIDL_STUBLESS_PROXY_INFO {}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::runtime::Abi for MIDL_STUBLESS_PROXY_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
pub struct MIDL_STUB_DESC {
    pub RpcInterfaceInformation: *mut ::std::ffi::c_void,
    pub pfnAllocate: isize,
    pub pfnFree: isize,
    pub IMPLICIT_HANDLE_INFO: MIDL_STUB_DESC_0,
    pub apfnNdrRundownRoutines: *mut ::std::option::Option<NDR_RUNDOWN>,
    pub aGenericBindingRoutinePairs: *mut GENERIC_BINDING_ROUTINE_PAIR,
    pub apfnExprEval: *mut ::std::option::Option<EXPR_EVAL>,
    pub aXmitQuintuple: *mut XMIT_ROUTINE_QUINTUPLE,
    pub pFormatTypes: *mut u8,
    pub fCheckBounds: i32,
    pub Version: u32,
    pub pMallocFreeStruct: *mut MALLOC_FREE_STRUCT,
    pub MIDLVersion: i32,
    pub CommFaultOffsets: *mut COMM_FAULT_OFFSETS,
    pub aUserMarshalQuadruple: *mut USER_MARSHAL_ROUTINE_QUADRUPLE,
    pub NotifyRoutineTable: *mut ::std::option::Option<NDR_NOTIFY_ROUTINE>,
    pub mFlags: usize,
    pub CsRoutineTables: *mut NDR_CS_ROUTINES,
    pub ProxyServerInfo: *mut ::std::ffi::c_void,
    pub pExprInfo: *mut NDR_EXPR_DESC,
}
#[cfg(feature = "Win32_System_Com")]
impl MIDL_STUB_DESC {}
#[cfg(feature = "Win32_System_Com")]
impl ::std::default::Default for MIDL_STUB_DESC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::PartialEq for MIDL_STUB_DESC {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::Eq for MIDL_STUB_DESC {}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::runtime::Abi for MIDL_STUB_DESC {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
pub union MIDL_STUB_DESC_0 {
    pub pAutoHandle: *mut *mut ::std::ffi::c_void,
    pub pPrimitiveHandle: *mut *mut ::std::ffi::c_void,
    pub pGenericBindingInfo: *mut ::std::mem::ManuallyDrop<GENERIC_BINDING_INFO>,
}
#[cfg(feature = "Win32_System_Com")]
impl MIDL_STUB_DESC_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::std::default::Default for MIDL_STUB_DESC_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::PartialEq for MIDL_STUB_DESC_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::Eq for MIDL_STUB_DESC_0 {}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::runtime::Abi for MIDL_STUB_DESC_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
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
    pub SavedHandle: *mut ::std::ffi::c_void,
    pub StubDesc: *mut MIDL_STUB_DESC,
    pub FullPtrXlatTables: *mut FULL_PTR_XLAT_TABLES,
    pub FullPtrRefId: u32,
    pub PointerLength: u32,
    pub _bitfield: i32,
    pub dwDestContext: u32,
    pub pvDestContext: *mut ::std::ffi::c_void,
    pub SavedContextHandles: *mut *mut NDR_SCONTEXT_1,
    pub ParamNumber: i32,
    pub pRpcChannelBuffer: ::std::option::Option<super::Com::IRpcChannelBuffer>,
    pub pArrayInfo: *mut ARRAY_INFO,
    pub SizePtrCountArray: *mut u32,
    pub SizePtrOffsetArray: *mut u32,
    pub SizePtrLengthArray: *mut u32,
    pub pArgQueue: *mut ::std::ffi::c_void,
    pub dwStubPhase: u32,
    pub LowStackMark: *mut ::std::ffi::c_void,
    pub pAsyncMsg: *mut _NDR_ASYNC_MESSAGE,
    pub pCorrInfo: *mut _NDR_CORRELATION_INFO,
    pub pCorrMemory: *mut u8,
    pub pMemoryList: *mut ::std::ffi::c_void,
    pub pCSInfo: isize,
    pub ConformanceMark: *mut u8,
    pub VarianceMark: *mut u8,
    pub Unused: isize,
    pub pContext: *mut _NDR_PROC_CONTEXT,
    pub ContextHandleHash: *mut ::std::ffi::c_void,
    pub pUserMarshalList: *mut ::std::ffi::c_void,
    pub Reserved51_3: isize,
    pub Reserved51_4: isize,
    pub Reserved51_5: isize,
}
#[cfg(feature = "Win32_System_Com")]
impl MIDL_STUB_MESSAGE {}
#[cfg(feature = "Win32_System_Com")]
impl ::std::default::Default for MIDL_STUB_MESSAGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::fmt::Debug for MIDL_STUB_MESSAGE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MIDL_STUB_MESSAGE")
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
impl ::std::cmp::PartialEq for MIDL_STUB_MESSAGE {
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
impl ::std::cmp::Eq for MIDL_STUB_MESSAGE {}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::runtime::Abi for MIDL_STUB_MESSAGE {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct MIDL_SYNTAX_INFO {
    pub TransferSyntax: RPC_SYNTAX_IDENTIFIER,
    pub DispatchTable: *mut RPC_DISPATCH_TABLE,
    pub ProcString: *mut u8,
    pub FmtStringOffset: *mut u16,
    pub TypeString: *mut u8,
    pub aUserMarshalQuadruple: *mut ::std::ffi::c_void,
    pub pMethodProperties: *mut MIDL_INTERFACE_METHOD_PROPERTIES,
    pub pReserved2: usize,
}
impl MIDL_SYNTAX_INFO {}
impl ::std::default::Default for MIDL_SYNTAX_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MIDL_SYNTAX_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MIDL_SYNTAX_INFO")
            .field("TransferSyntax", &self.TransferSyntax)
            .field("DispatchTable", &self.DispatchTable)
            .field("ProcString", &self.ProcString)
            .field("FmtStringOffset", &self.FmtStringOffset)
            .field("TypeString", &self.TypeString)
            .field("aUserMarshalQuadruple", &self.aUserMarshalQuadruple)
            .field("pMethodProperties", &self.pMethodProperties)
            .field("pReserved2", &self.pReserved2)
            .finish()
    }
}
impl ::std::cmp::PartialEq for MIDL_SYNTAX_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.TransferSyntax == other.TransferSyntax && self.DispatchTable == other.DispatchTable && self.ProcString == other.ProcString && self.FmtStringOffset == other.FmtStringOffset && self.TypeString == other.TypeString && self.aUserMarshalQuadruple == other.aUserMarshalQuadruple && self.pMethodProperties == other.pMethodProperties && self.pReserved2 == other.pReserved2
    }
}
impl ::std::cmp::Eq for MIDL_SYNTAX_INFO {}
unsafe impl ::windows::runtime::Abi for MIDL_SYNTAX_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct MIDL_TYPE_PICKLING_INFO {
    pub Version: u32,
    pub Flags: u32,
    pub Reserved: [usize; 3],
}
impl MIDL_TYPE_PICKLING_INFO {}
impl ::std::default::Default for MIDL_TYPE_PICKLING_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for MIDL_TYPE_PICKLING_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MIDL_TYPE_PICKLING_INFO").field("Version", &self.Version).field("Flags", &self.Flags).field("Reserved", &self.Reserved).finish()
    }
}
impl ::std::cmp::PartialEq for MIDL_TYPE_PICKLING_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Flags == other.Flags && self.Reserved == other.Reserved
    }
}
impl ::std::cmp::Eq for MIDL_TYPE_PICKLING_INFO {}
unsafe impl ::windows::runtime::Abi for MIDL_TYPE_PICKLING_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
pub struct MIDL_WINRT_TYPE_SERIALIZATION_INFO {
    pub Version: u32,
    pub TypeFormatString: *mut u8,
    pub FormatStringSize: u16,
    pub TypeOffset: u16,
    pub StubDesc: *mut MIDL_STUB_DESC,
}
#[cfg(feature = "Win32_System_Com")]
impl MIDL_WINRT_TYPE_SERIALIZATION_INFO {}
#[cfg(feature = "Win32_System_Com")]
impl ::std::default::Default for MIDL_WINRT_TYPE_SERIALIZATION_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::fmt::Debug for MIDL_WINRT_TYPE_SERIALIZATION_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("MIDL_WINRT_TYPE_SERIALIZATION_INFO").field("Version", &self.Version).field("TypeFormatString", &self.TypeFormatString).field("FormatStringSize", &self.FormatStringSize).field("TypeOffset", &self.TypeOffset).field("StubDesc", &self.StubDesc).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::PartialEq for MIDL_WINRT_TYPE_SERIALIZATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.TypeFormatString == other.TypeFormatString && self.FormatStringSize == other.FormatStringSize && self.TypeOffset == other.TypeOffset && self.StubDesc == other.StubDesc
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::Eq for MIDL_WINRT_TYPE_SERIALIZATION_INFO {}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::runtime::Abi for MIDL_WINRT_TYPE_SERIALIZATION_INFO {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const MIDL_WINRT_TYPE_SERIALIZATION_INFO_CURRENT_VERSION: i32 = 1i32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const MaxNumberOfEEInfoParams: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn MesBufferHandleReset(handle: *const ::std::ffi::c_void, handlestyle: u32, operation: MIDL_ES_CODE, pbuffer: *const *const i8, buffersize: u32, pencodedsize: *mut u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MesBufferHandleReset(handle: *const ::std::ffi::c_void, handlestyle: u32, operation: MIDL_ES_CODE, pbuffer: *const *const i8, buffersize: u32, pencodedsize: *mut u32) -> RPC_STATUS;
        }
        ::std::mem::transmute(MesBufferHandleReset(::std::mem::transmute(handle), ::std::mem::transmute(handlestyle), ::std::mem::transmute(operation), ::std::mem::transmute(pbuffer), ::std::mem::transmute(buffersize), ::std::mem::transmute(pencodedsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MesDecodeBufferHandleCreate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PSTR>>(buffer: Param0, buffersize: u32, phandle: *mut *mut ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MesDecodeBufferHandleCreate(buffer: super::super::Foundation::PSTR, buffersize: u32, phandle: *mut *mut ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(MesDecodeBufferHandleCreate(buffer.into_param().abi(), ::std::mem::transmute(buffersize), ::std::mem::transmute(phandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn MesDecodeIncrementalHandleCreate(userstate: *mut ::std::ffi::c_void, readfn: ::std::option::Option<MIDL_ES_READ>, phandle: *mut *mut ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MesDecodeIncrementalHandleCreate(userstate: *mut ::std::ffi::c_void, readfn: ::windows::runtime::RawPtr, phandle: *mut *mut ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(MesDecodeIncrementalHandleCreate(::std::mem::transmute(userstate), ::std::mem::transmute(readfn), ::std::mem::transmute(phandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn MesEncodeDynBufferHandleCreate(pbuffer: *mut *mut i8, pencodedsize: *mut u32, phandle: *mut *mut ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MesEncodeDynBufferHandleCreate(pbuffer: *mut *mut i8, pencodedsize: *mut u32, phandle: *mut *mut ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(MesEncodeDynBufferHandleCreate(::std::mem::transmute(pbuffer), ::std::mem::transmute(pencodedsize), ::std::mem::transmute(phandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MesEncodeFixedBufferHandleCreate(pbuffer: super::super::Foundation::PSTR, buffersize: u32, pencodedsize: *mut u32, phandle: *mut *mut ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MesEncodeFixedBufferHandleCreate(pbuffer: super::super::Foundation::PSTR, buffersize: u32, pencodedsize: *mut u32, phandle: *mut *mut ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(MesEncodeFixedBufferHandleCreate(::std::mem::transmute(pbuffer), ::std::mem::transmute(buffersize), ::std::mem::transmute(pencodedsize), ::std::mem::transmute(phandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MesEncodeIncrementalHandleCreate(userstate: *mut ::std::ffi::c_void, allocfn: ::std::option::Option<MIDL_ES_ALLOC>, writefn: ::std::option::Option<MIDL_ES_WRITE>, phandle: *mut *mut ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MesEncodeIncrementalHandleCreate(userstate: *mut ::std::ffi::c_void, allocfn: ::windows::runtime::RawPtr, writefn: ::windows::runtime::RawPtr, phandle: *mut *mut ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(MesEncodeIncrementalHandleCreate(::std::mem::transmute(userstate), ::std::mem::transmute(allocfn), ::std::mem::transmute(writefn), ::std::mem::transmute(phandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn MesHandleFree(handle: *mut ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MesHandleFree(handle: *mut ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(MesHandleFree(::std::mem::transmute(handle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn MesIncrementalHandleReset(handle: *mut ::std::ffi::c_void, userstate: *mut ::std::ffi::c_void, allocfn: ::std::option::Option<MIDL_ES_ALLOC>, writefn: ::std::option::Option<MIDL_ES_WRITE>, readfn: ::std::option::Option<MIDL_ES_READ>, operation: MIDL_ES_CODE) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MesIncrementalHandleReset(handle: *mut ::std::ffi::c_void, userstate: *mut ::std::ffi::c_void, allocfn: ::windows::runtime::RawPtr, writefn: ::windows::runtime::RawPtr, readfn: ::windows::runtime::RawPtr, operation: MIDL_ES_CODE) -> RPC_STATUS;
        }
        ::std::mem::transmute(MesIncrementalHandleReset(::std::mem::transmute(handle), ::std::mem::transmute(userstate), ::std::mem::transmute(allocfn), ::std::mem::transmute(writefn), ::std::mem::transmute(readfn), ::std::mem::transmute(operation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn MesInqProcEncodingId(handle: *mut ::std::ffi::c_void, pinterfaceid: *mut RPC_SYNTAX_IDENTIFIER, pprocnum: *mut u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn MesInqProcEncodingId(handle: *mut ::std::ffi::c_void, pinterfaceid: *mut RPC_SYNTAX_IDENTIFIER, pprocnum: *mut u32) -> RPC_STATUS;
        }
        ::std::mem::transmute(MesInqProcEncodingId(::std::mem::transmute(handle), ::std::mem::transmute(pinterfaceid), ::std::mem::transmute(pprocnum)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const MidlInterceptionInfoVersionOne: i32 = 1i32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const MidlWinrtTypeSerializationInfoVersionOne: i32 = 1i32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_ARRAY_ELEMENT_INFO {
    pub ElementMemSize: u32,
    pub Element: *mut ::std::ffi::c_void,
}
impl NDR64_ARRAY_ELEMENT_INFO {}
impl ::std::default::Default for NDR64_ARRAY_ELEMENT_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_ARRAY_ELEMENT_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_ARRAY_ELEMENT_INFO").field("ElementMemSize", &self.ElementMemSize).field("Element", &self.Element).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_ARRAY_ELEMENT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ElementMemSize == other.ElementMemSize && self.Element == other.Element
    }
}
impl ::std::cmp::Eq for NDR64_ARRAY_ELEMENT_INFO {}
unsafe impl ::windows::runtime::Abi for NDR64_ARRAY_ELEMENT_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_ARRAY_FLAGS {
    pub _bitfield: u8,
}
impl NDR64_ARRAY_FLAGS {}
impl ::std::default::Default for NDR64_ARRAY_FLAGS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_ARRAY_FLAGS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_ARRAY_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_ARRAY_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for NDR64_ARRAY_FLAGS {}
unsafe impl ::windows::runtime::Abi for NDR64_ARRAY_FLAGS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub union NDR64_BINDINGS {
    pub Primitive: NDR64_BIND_PRIMITIVE,
    pub Generic: NDR64_BIND_GENERIC,
    pub Context: NDR64_BIND_CONTEXT,
}
impl NDR64_BINDINGS {}
impl ::std::default::Default for NDR64_BINDINGS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for NDR64_BINDINGS {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for NDR64_BINDINGS {}
unsafe impl ::windows::runtime::Abi for NDR64_BINDINGS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_BIND_AND_NOTIFY_EXTENSION {
    pub Binding: NDR64_BIND_CONTEXT,
    pub NotifyIndex: u16,
}
impl NDR64_BIND_AND_NOTIFY_EXTENSION {}
impl ::std::default::Default for NDR64_BIND_AND_NOTIFY_EXTENSION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_BIND_AND_NOTIFY_EXTENSION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_BIND_AND_NOTIFY_EXTENSION").field("Binding", &self.Binding).field("NotifyIndex", &self.NotifyIndex).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_BIND_AND_NOTIFY_EXTENSION {
    fn eq(&self, other: &Self) -> bool {
        self.Binding == other.Binding && self.NotifyIndex == other.NotifyIndex
    }
}
impl ::std::cmp::Eq for NDR64_BIND_AND_NOTIFY_EXTENSION {}
unsafe impl ::windows::runtime::Abi for NDR64_BIND_AND_NOTIFY_EXTENSION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_BIND_CONTEXT {
    pub HandleType: u8,
    pub Flags: u8,
    pub StackOffset: u16,
    pub RoutineIndex: u8,
    pub Ordinal: u8,
}
impl NDR64_BIND_CONTEXT {}
impl ::std::default::Default for NDR64_BIND_CONTEXT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_BIND_CONTEXT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_BIND_CONTEXT").field("HandleType", &self.HandleType).field("Flags", &self.Flags).field("StackOffset", &self.StackOffset).field("RoutineIndex", &self.RoutineIndex).field("Ordinal", &self.Ordinal).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_BIND_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.HandleType == other.HandleType && self.Flags == other.Flags && self.StackOffset == other.StackOffset && self.RoutineIndex == other.RoutineIndex && self.Ordinal == other.Ordinal
    }
}
impl ::std::cmp::Eq for NDR64_BIND_CONTEXT {}
unsafe impl ::windows::runtime::Abi for NDR64_BIND_CONTEXT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_BIND_GENERIC {
    pub HandleType: u8,
    pub Flags: u8,
    pub StackOffset: u16,
    pub RoutineIndex: u8,
    pub Size: u8,
}
impl NDR64_BIND_GENERIC {}
impl ::std::default::Default for NDR64_BIND_GENERIC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_BIND_GENERIC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_BIND_GENERIC").field("HandleType", &self.HandleType).field("Flags", &self.Flags).field("StackOffset", &self.StackOffset).field("RoutineIndex", &self.RoutineIndex).field("Size", &self.Size).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_BIND_GENERIC {
    fn eq(&self, other: &Self) -> bool {
        self.HandleType == other.HandleType && self.Flags == other.Flags && self.StackOffset == other.StackOffset && self.RoutineIndex == other.RoutineIndex && self.Size == other.Size
    }
}
impl ::std::cmp::Eq for NDR64_BIND_GENERIC {}
unsafe impl ::windows::runtime::Abi for NDR64_BIND_GENERIC {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_BIND_PRIMITIVE {
    pub HandleType: u8,
    pub Flags: u8,
    pub StackOffset: u16,
    pub Reserved: u16,
}
impl NDR64_BIND_PRIMITIVE {}
impl ::std::default::Default for NDR64_BIND_PRIMITIVE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_BIND_PRIMITIVE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_BIND_PRIMITIVE").field("HandleType", &self.HandleType).field("Flags", &self.Flags).field("StackOffset", &self.StackOffset).field("Reserved", &self.Reserved).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_BIND_PRIMITIVE {
    fn eq(&self, other: &Self) -> bool {
        self.HandleType == other.HandleType && self.Flags == other.Flags && self.StackOffset == other.StackOffset && self.Reserved == other.Reserved
    }
}
impl ::std::cmp::Eq for NDR64_BIND_PRIMITIVE {}
unsafe impl ::windows::runtime::Abi for NDR64_BIND_PRIMITIVE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_BOGUS_ARRAY_HEADER_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: NDR64_ARRAY_FLAGS,
    pub NumberDims: u8,
    pub NumberElements: u32,
    pub Element: *mut ::std::ffi::c_void,
}
impl NDR64_BOGUS_ARRAY_HEADER_FORMAT {}
impl ::std::default::Default for NDR64_BOGUS_ARRAY_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_BOGUS_ARRAY_HEADER_FORMAT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_BOGUS_ARRAY_HEADER_FORMAT").field("FormatCode", &self.FormatCode).field("Alignment", &self.Alignment).field("Flags", &self.Flags).field("NumberDims", &self.NumberDims).field("NumberElements", &self.NumberElements).field("Element", &self.Element).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_BOGUS_ARRAY_HEADER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Alignment == other.Alignment && self.Flags == other.Flags && self.NumberDims == other.NumberDims && self.NumberElements == other.NumberElements && self.Element == other.Element
    }
}
impl ::std::cmp::Eq for NDR64_BOGUS_ARRAY_HEADER_FORMAT {}
unsafe impl ::windows::runtime::Abi for NDR64_BOGUS_ARRAY_HEADER_FORMAT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_BOGUS_STRUCTURE_HEADER_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: NDR64_STRUCTURE_FLAGS,
    pub Reserve: u8,
    pub MemorySize: u32,
    pub OriginalMemberLayout: *mut ::std::ffi::c_void,
    pub OriginalPointerLayout: *mut ::std::ffi::c_void,
    pub PointerLayout: *mut ::std::ffi::c_void,
}
impl NDR64_BOGUS_STRUCTURE_HEADER_FORMAT {}
impl ::std::default::Default for NDR64_BOGUS_STRUCTURE_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_BOGUS_STRUCTURE_HEADER_FORMAT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_BOGUS_STRUCTURE_HEADER_FORMAT")
            .field("FormatCode", &self.FormatCode)
            .field("Alignment", &self.Alignment)
            .field("Flags", &self.Flags)
            .field("Reserve", &self.Reserve)
            .field("MemorySize", &self.MemorySize)
            .field("OriginalMemberLayout", &self.OriginalMemberLayout)
            .field("OriginalPointerLayout", &self.OriginalPointerLayout)
            .field("PointerLayout", &self.PointerLayout)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_BOGUS_STRUCTURE_HEADER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Alignment == other.Alignment && self.Flags == other.Flags && self.Reserve == other.Reserve && self.MemorySize == other.MemorySize && self.OriginalMemberLayout == other.OriginalMemberLayout && self.OriginalPointerLayout == other.OriginalPointerLayout && self.PointerLayout == other.PointerLayout
    }
}
impl ::std::cmp::Eq for NDR64_BOGUS_STRUCTURE_HEADER_FORMAT {}
unsafe impl ::windows::runtime::Abi for NDR64_BOGUS_STRUCTURE_HEADER_FORMAT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_BUFFER_ALIGN_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Reserved: u16,
    pub Reserved2: u32,
}
impl NDR64_BUFFER_ALIGN_FORMAT {}
impl ::std::default::Default for NDR64_BUFFER_ALIGN_FORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_BUFFER_ALIGN_FORMAT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_BUFFER_ALIGN_FORMAT").field("FormatCode", &self.FormatCode).field("Alignment", &self.Alignment).field("Reserved", &self.Reserved).field("Reserved2", &self.Reserved2).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_BUFFER_ALIGN_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Alignment == other.Alignment && self.Reserved == other.Reserved && self.Reserved2 == other.Reserved2
    }
}
impl ::std::cmp::Eq for NDR64_BUFFER_ALIGN_FORMAT {}
unsafe impl ::windows::runtime::Abi for NDR64_BUFFER_ALIGN_FORMAT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_CONFORMANT_STRING_FORMAT {
    pub Header: NDR64_STRING_HEADER_FORMAT,
}
impl NDR64_CONFORMANT_STRING_FORMAT {}
impl ::std::default::Default for NDR64_CONFORMANT_STRING_FORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_CONFORMANT_STRING_FORMAT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_CONFORMANT_STRING_FORMAT").field("Header", &self.Header).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_CONFORMANT_STRING_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header
    }
}
impl ::std::cmp::Eq for NDR64_CONFORMANT_STRING_FORMAT {}
unsafe impl ::windows::runtime::Abi for NDR64_CONFORMANT_STRING_FORMAT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_CONF_ARRAY_HEADER_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: NDR64_ARRAY_FLAGS,
    pub Reserved: u8,
    pub ElementSize: u32,
    pub ConfDescriptor: *mut ::std::ffi::c_void,
}
impl NDR64_CONF_ARRAY_HEADER_FORMAT {}
impl ::std::default::Default for NDR64_CONF_ARRAY_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_CONF_ARRAY_HEADER_FORMAT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_CONF_ARRAY_HEADER_FORMAT").field("FormatCode", &self.FormatCode).field("Alignment", &self.Alignment).field("Flags", &self.Flags).field("Reserved", &self.Reserved).field("ElementSize", &self.ElementSize).field("ConfDescriptor", &self.ConfDescriptor).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_CONF_ARRAY_HEADER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Alignment == other.Alignment && self.Flags == other.Flags && self.Reserved == other.Reserved && self.ElementSize == other.ElementSize && self.ConfDescriptor == other.ConfDescriptor
    }
}
impl ::std::cmp::Eq for NDR64_CONF_ARRAY_HEADER_FORMAT {}
unsafe impl ::windows::runtime::Abi for NDR64_CONF_ARRAY_HEADER_FORMAT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_CONF_BOGUS_STRUCTURE_HEADER_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: NDR64_STRUCTURE_FLAGS,
    pub Dimensions: u8,
    pub MemorySize: u32,
    pub OriginalMemberLayout: *mut ::std::ffi::c_void,
    pub OriginalPointerLayout: *mut ::std::ffi::c_void,
    pub PointerLayout: *mut ::std::ffi::c_void,
    pub ConfArrayDescription: *mut ::std::ffi::c_void,
}
impl NDR64_CONF_BOGUS_STRUCTURE_HEADER_FORMAT {}
impl ::std::default::Default for NDR64_CONF_BOGUS_STRUCTURE_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_CONF_BOGUS_STRUCTURE_HEADER_FORMAT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_CONF_BOGUS_STRUCTURE_HEADER_FORMAT")
            .field("FormatCode", &self.FormatCode)
            .field("Alignment", &self.Alignment)
            .field("Flags", &self.Flags)
            .field("Dimensions", &self.Dimensions)
            .field("MemorySize", &self.MemorySize)
            .field("OriginalMemberLayout", &self.OriginalMemberLayout)
            .field("OriginalPointerLayout", &self.OriginalPointerLayout)
            .field("PointerLayout", &self.PointerLayout)
            .field("ConfArrayDescription", &self.ConfArrayDescription)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_CONF_BOGUS_STRUCTURE_HEADER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Alignment == other.Alignment && self.Flags == other.Flags && self.Dimensions == other.Dimensions && self.MemorySize == other.MemorySize && self.OriginalMemberLayout == other.OriginalMemberLayout && self.OriginalPointerLayout == other.OriginalPointerLayout && self.PointerLayout == other.PointerLayout && self.ConfArrayDescription == other.ConfArrayDescription
    }
}
impl ::std::cmp::Eq for NDR64_CONF_BOGUS_STRUCTURE_HEADER_FORMAT {}
unsafe impl ::windows::runtime::Abi for NDR64_CONF_BOGUS_STRUCTURE_HEADER_FORMAT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_CONF_STRUCTURE_HEADER_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: NDR64_STRUCTURE_FLAGS,
    pub Reserve: u8,
    pub MemorySize: u32,
    pub ArrayDescription: *mut ::std::ffi::c_void,
}
impl NDR64_CONF_STRUCTURE_HEADER_FORMAT {}
impl ::std::default::Default for NDR64_CONF_STRUCTURE_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_CONF_STRUCTURE_HEADER_FORMAT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_CONF_STRUCTURE_HEADER_FORMAT").field("FormatCode", &self.FormatCode).field("Alignment", &self.Alignment).field("Flags", &self.Flags).field("Reserve", &self.Reserve).field("MemorySize", &self.MemorySize).field("ArrayDescription", &self.ArrayDescription).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_CONF_STRUCTURE_HEADER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Alignment == other.Alignment && self.Flags == other.Flags && self.Reserve == other.Reserve && self.MemorySize == other.MemorySize && self.ArrayDescription == other.ArrayDescription
    }
}
impl ::std::cmp::Eq for NDR64_CONF_STRUCTURE_HEADER_FORMAT {}
unsafe impl ::windows::runtime::Abi for NDR64_CONF_STRUCTURE_HEADER_FORMAT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_CONF_VAR_ARRAY_HEADER_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: NDR64_ARRAY_FLAGS,
    pub Reserved: u8,
    pub ElementSize: u32,
    pub ConfDescriptor: *mut ::std::ffi::c_void,
    pub VarDescriptor: *mut ::std::ffi::c_void,
}
impl NDR64_CONF_VAR_ARRAY_HEADER_FORMAT {}
impl ::std::default::Default for NDR64_CONF_VAR_ARRAY_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_CONF_VAR_ARRAY_HEADER_FORMAT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_CONF_VAR_ARRAY_HEADER_FORMAT")
            .field("FormatCode", &self.FormatCode)
            .field("Alignment", &self.Alignment)
            .field("Flags", &self.Flags)
            .field("Reserved", &self.Reserved)
            .field("ElementSize", &self.ElementSize)
            .field("ConfDescriptor", &self.ConfDescriptor)
            .field("VarDescriptor", &self.VarDescriptor)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_CONF_VAR_ARRAY_HEADER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Alignment == other.Alignment && self.Flags == other.Flags && self.Reserved == other.Reserved && self.ElementSize == other.ElementSize && self.ConfDescriptor == other.ConfDescriptor && self.VarDescriptor == other.VarDescriptor
    }
}
impl ::std::cmp::Eq for NDR64_CONF_VAR_ARRAY_HEADER_FORMAT {}
unsafe impl ::windows::runtime::Abi for NDR64_CONF_VAR_ARRAY_HEADER_FORMAT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_CONF_VAR_BOGUS_ARRAY_HEADER_FORMAT {
    pub FixedArrayFormat: NDR64_BOGUS_ARRAY_HEADER_FORMAT,
    pub ConfDescription: *mut ::std::ffi::c_void,
    pub VarDescription: *mut ::std::ffi::c_void,
    pub OffsetDescription: *mut ::std::ffi::c_void,
}
impl NDR64_CONF_VAR_BOGUS_ARRAY_HEADER_FORMAT {}
impl ::std::default::Default for NDR64_CONF_VAR_BOGUS_ARRAY_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_CONF_VAR_BOGUS_ARRAY_HEADER_FORMAT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_CONF_VAR_BOGUS_ARRAY_HEADER_FORMAT").field("FixedArrayFormat", &self.FixedArrayFormat).field("ConfDescription", &self.ConfDescription).field("VarDescription", &self.VarDescription).field("OffsetDescription", &self.OffsetDescription).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_CONF_VAR_BOGUS_ARRAY_HEADER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FixedArrayFormat == other.FixedArrayFormat && self.ConfDescription == other.ConfDescription && self.VarDescription == other.VarDescription && self.OffsetDescription == other.OffsetDescription
    }
}
impl ::std::cmp::Eq for NDR64_CONF_VAR_BOGUS_ARRAY_HEADER_FORMAT {}
unsafe impl ::windows::runtime::Abi for NDR64_CONF_VAR_BOGUS_ARRAY_HEADER_FORMAT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_CONSTANT_IID_FORMAT {
    pub FormatCode: u8,
    pub Flags: u8,
    pub Reserved: u16,
    pub Guid: ::windows::runtime::GUID,
}
impl NDR64_CONSTANT_IID_FORMAT {}
impl ::std::default::Default for NDR64_CONSTANT_IID_FORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_CONSTANT_IID_FORMAT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_CONSTANT_IID_FORMAT").field("FormatCode", &self.FormatCode).field("Flags", &self.Flags).field("Reserved", &self.Reserved).field("Guid", &self.Guid).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_CONSTANT_IID_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Flags == other.Flags && self.Reserved == other.Reserved && self.Guid == other.Guid
    }
}
impl ::std::cmp::Eq for NDR64_CONSTANT_IID_FORMAT {}
unsafe impl ::windows::runtime::Abi for NDR64_CONSTANT_IID_FORMAT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_CONTEXT_HANDLE_FLAGS {
    pub _bitfield: u8,
}
impl NDR64_CONTEXT_HANDLE_FLAGS {}
impl ::std::default::Default for NDR64_CONTEXT_HANDLE_FLAGS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_CONTEXT_HANDLE_FLAGS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_CONTEXT_HANDLE_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_CONTEXT_HANDLE_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for NDR64_CONTEXT_HANDLE_FLAGS {}
unsafe impl ::windows::runtime::Abi for NDR64_CONTEXT_HANDLE_FLAGS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_CONTEXT_HANDLE_FORMAT {
    pub FormatCode: u8,
    pub ContextFlags: u8,
    pub RundownRoutineIndex: u8,
    pub Ordinal: u8,
}
impl NDR64_CONTEXT_HANDLE_FORMAT {}
impl ::std::default::Default for NDR64_CONTEXT_HANDLE_FORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_CONTEXT_HANDLE_FORMAT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_CONTEXT_HANDLE_FORMAT").field("FormatCode", &self.FormatCode).field("ContextFlags", &self.ContextFlags).field("RundownRoutineIndex", &self.RundownRoutineIndex).field("Ordinal", &self.Ordinal).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_CONTEXT_HANDLE_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.ContextFlags == other.ContextFlags && self.RundownRoutineIndex == other.RundownRoutineIndex && self.Ordinal == other.Ordinal
    }
}
impl ::std::cmp::Eq for NDR64_CONTEXT_HANDLE_FORMAT {}
unsafe impl ::windows::runtime::Abi for NDR64_CONTEXT_HANDLE_FORMAT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_EMBEDDED_COMPLEX_FORMAT {
    pub FormatCode: u8,
    pub Reserve1: u8,
    pub Reserve2: u16,
    pub Type: *mut ::std::ffi::c_void,
}
impl NDR64_EMBEDDED_COMPLEX_FORMAT {}
impl ::std::default::Default for NDR64_EMBEDDED_COMPLEX_FORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_EMBEDDED_COMPLEX_FORMAT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_EMBEDDED_COMPLEX_FORMAT").field("FormatCode", &self.FormatCode).field("Reserve1", &self.Reserve1).field("Reserve2", &self.Reserve2).field("Type", &self.Type).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_EMBEDDED_COMPLEX_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Reserve1 == other.Reserve1 && self.Reserve2 == other.Reserve2 && self.Type == other.Type
    }
}
impl ::std::cmp::Eq for NDR64_EMBEDDED_COMPLEX_FORMAT {}
unsafe impl ::windows::runtime::Abi for NDR64_EMBEDDED_COMPLEX_FORMAT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_ENCAPSULATED_UNION {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: u8,
    pub SwitchType: u8,
    pub MemoryOffset: u32,
    pub MemorySize: u32,
    pub Reserved: u32,
}
impl NDR64_ENCAPSULATED_UNION {}
impl ::std::default::Default for NDR64_ENCAPSULATED_UNION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_ENCAPSULATED_UNION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_ENCAPSULATED_UNION")
            .field("FormatCode", &self.FormatCode)
            .field("Alignment", &self.Alignment)
            .field("Flags", &self.Flags)
            .field("SwitchType", &self.SwitchType)
            .field("MemoryOffset", &self.MemoryOffset)
            .field("MemorySize", &self.MemorySize)
            .field("Reserved", &self.Reserved)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_ENCAPSULATED_UNION {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Alignment == other.Alignment && self.Flags == other.Flags && self.SwitchType == other.SwitchType && self.MemoryOffset == other.MemoryOffset && self.MemorySize == other.MemorySize && self.Reserved == other.Reserved
    }
}
impl ::std::cmp::Eq for NDR64_ENCAPSULATED_UNION {}
unsafe impl ::windows::runtime::Abi for NDR64_ENCAPSULATED_UNION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_EXPR_CONST32 {
    pub ExprType: u8,
    pub Reserved: u8,
    pub Reserved1: u16,
    pub ConstValue: u32,
}
impl NDR64_EXPR_CONST32 {}
impl ::std::default::Default for NDR64_EXPR_CONST32 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_EXPR_CONST32 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_EXPR_CONST32").field("ExprType", &self.ExprType).field("Reserved", &self.Reserved).field("Reserved1", &self.Reserved1).field("ConstValue", &self.ConstValue).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_EXPR_CONST32 {
    fn eq(&self, other: &Self) -> bool {
        self.ExprType == other.ExprType && self.Reserved == other.Reserved && self.Reserved1 == other.Reserved1 && self.ConstValue == other.ConstValue
    }
}
impl ::std::cmp::Eq for NDR64_EXPR_CONST32 {}
unsafe impl ::windows::runtime::Abi for NDR64_EXPR_CONST32 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_EXPR_CONST64 {
    pub ExprType: u8,
    pub Reserved: u8,
    pub Reserved1: u16,
    pub ConstValue: i64,
}
impl NDR64_EXPR_CONST64 {}
impl ::std::default::Default for NDR64_EXPR_CONST64 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_EXPR_CONST64 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_EXPR_CONST64").field("ExprType", &self.ExprType).field("Reserved", &self.Reserved).field("Reserved1", &self.Reserved1).field("ConstValue", &self.ConstValue).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_EXPR_CONST64 {
    fn eq(&self, other: &Self) -> bool {
        self.ExprType == other.ExprType && self.Reserved == other.Reserved && self.Reserved1 == other.Reserved1 && self.ConstValue == other.ConstValue
    }
}
impl ::std::cmp::Eq for NDR64_EXPR_CONST64 {}
unsafe impl ::windows::runtime::Abi for NDR64_EXPR_CONST64 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_EXPR_NOOP {
    pub ExprType: u8,
    pub Size: u8,
    pub Reserved: u16,
}
impl NDR64_EXPR_NOOP {}
impl ::std::default::Default for NDR64_EXPR_NOOP {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_EXPR_NOOP {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_EXPR_NOOP").field("ExprType", &self.ExprType).field("Size", &self.Size).field("Reserved", &self.Reserved).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_EXPR_NOOP {
    fn eq(&self, other: &Self) -> bool {
        self.ExprType == other.ExprType && self.Size == other.Size && self.Reserved == other.Reserved
    }
}
impl ::std::cmp::Eq for NDR64_EXPR_NOOP {}
unsafe impl ::windows::runtime::Abi for NDR64_EXPR_NOOP {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_EXPR_OPERATOR {
    pub ExprType: u8,
    pub Operator: u8,
    pub CastType: u8,
    pub Reserved: u8,
}
impl NDR64_EXPR_OPERATOR {}
impl ::std::default::Default for NDR64_EXPR_OPERATOR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_EXPR_OPERATOR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_EXPR_OPERATOR").field("ExprType", &self.ExprType).field("Operator", &self.Operator).field("CastType", &self.CastType).field("Reserved", &self.Reserved).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_EXPR_OPERATOR {
    fn eq(&self, other: &Self) -> bool {
        self.ExprType == other.ExprType && self.Operator == other.Operator && self.CastType == other.CastType && self.Reserved == other.Reserved
    }
}
impl ::std::cmp::Eq for NDR64_EXPR_OPERATOR {}
unsafe impl ::windows::runtime::Abi for NDR64_EXPR_OPERATOR {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_EXPR_VAR {
    pub ExprType: u8,
    pub VarType: u8,
    pub Reserved: u16,
    pub Offset: u32,
}
impl NDR64_EXPR_VAR {}
impl ::std::default::Default for NDR64_EXPR_VAR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_EXPR_VAR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_EXPR_VAR").field("ExprType", &self.ExprType).field("VarType", &self.VarType).field("Reserved", &self.Reserved).field("Offset", &self.Offset).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_EXPR_VAR {
    fn eq(&self, other: &Self) -> bool {
        self.ExprType == other.ExprType && self.VarType == other.VarType && self.Reserved == other.Reserved && self.Offset == other.Offset
    }
}
impl ::std::cmp::Eq for NDR64_EXPR_VAR {}
unsafe impl ::windows::runtime::Abi for NDR64_EXPR_VAR {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const NDR64_FC_AUTO_HANDLE: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const NDR64_FC_BIND_GENERIC: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const NDR64_FC_BIND_PRIMITIVE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const NDR64_FC_CALLBACK_HANDLE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const NDR64_FC_EXPLICIT_HANDLE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const NDR64_FC_NO_HANDLE: u32 = 5u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_FIXED_REPEAT_FORMAT {
    pub RepeatFormat: NDR64_REPEAT_FORMAT,
    pub Iterations: u32,
    pub Reserved: u32,
}
impl NDR64_FIXED_REPEAT_FORMAT {}
impl ::std::default::Default for NDR64_FIXED_REPEAT_FORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_FIXED_REPEAT_FORMAT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_FIXED_REPEAT_FORMAT").field("RepeatFormat", &self.RepeatFormat).field("Iterations", &self.Iterations).field("Reserved", &self.Reserved).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_FIXED_REPEAT_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.RepeatFormat == other.RepeatFormat && self.Iterations == other.Iterations && self.Reserved == other.Reserved
    }
}
impl ::std::cmp::Eq for NDR64_FIXED_REPEAT_FORMAT {}
unsafe impl ::windows::runtime::Abi for NDR64_FIXED_REPEAT_FORMAT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_FIX_ARRAY_HEADER_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: NDR64_ARRAY_FLAGS,
    pub Reserved: u8,
    pub TotalSize: u32,
}
impl NDR64_FIX_ARRAY_HEADER_FORMAT {}
impl ::std::default::Default for NDR64_FIX_ARRAY_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_FIX_ARRAY_HEADER_FORMAT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_FIX_ARRAY_HEADER_FORMAT").field("FormatCode", &self.FormatCode).field("Alignment", &self.Alignment).field("Flags", &self.Flags).field("Reserved", &self.Reserved).field("TotalSize", &self.TotalSize).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_FIX_ARRAY_HEADER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Alignment == other.Alignment && self.Flags == other.Flags && self.Reserved == other.Reserved && self.TotalSize == other.TotalSize
    }
}
impl ::std::cmp::Eq for NDR64_FIX_ARRAY_HEADER_FORMAT {}
unsafe impl ::windows::runtime::Abi for NDR64_FIX_ARRAY_HEADER_FORMAT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_IID_FLAGS {
    pub _bitfield: u8,
}
impl NDR64_IID_FLAGS {}
impl ::std::default::Default for NDR64_IID_FLAGS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_IID_FLAGS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_IID_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_IID_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for NDR64_IID_FLAGS {}
unsafe impl ::windows::runtime::Abi for NDR64_IID_FLAGS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_IID_FORMAT {
    pub FormatCode: u8,
    pub Flags: u8,
    pub Reserved: u16,
    pub IIDDescriptor: *mut ::std::ffi::c_void,
}
impl NDR64_IID_FORMAT {}
impl ::std::default::Default for NDR64_IID_FORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_IID_FORMAT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_IID_FORMAT").field("FormatCode", &self.FormatCode).field("Flags", &self.Flags).field("Reserved", &self.Reserved).field("IIDDescriptor", &self.IIDDescriptor).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_IID_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Flags == other.Flags && self.Reserved == other.Reserved && self.IIDDescriptor == other.IIDDescriptor
    }
}
impl ::std::cmp::Eq for NDR64_IID_FORMAT {}
unsafe impl ::windows::runtime::Abi for NDR64_IID_FORMAT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_MEMPAD_FORMAT {
    pub FormatCode: u8,
    pub Reserve1: u8,
    pub MemPad: u16,
    pub Reserved2: u32,
}
impl NDR64_MEMPAD_FORMAT {}
impl ::std::default::Default for NDR64_MEMPAD_FORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_MEMPAD_FORMAT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_MEMPAD_FORMAT").field("FormatCode", &self.FormatCode).field("Reserve1", &self.Reserve1).field("MemPad", &self.MemPad).field("Reserved2", &self.Reserved2).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_MEMPAD_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Reserve1 == other.Reserve1 && self.MemPad == other.MemPad && self.Reserved2 == other.Reserved2
    }
}
impl ::std::cmp::Eq for NDR64_MEMPAD_FORMAT {}
unsafe impl ::windows::runtime::Abi for NDR64_MEMPAD_FORMAT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_NON_CONFORMANT_STRING_FORMAT {
    pub Header: NDR64_STRING_HEADER_FORMAT,
    pub TotalSize: u32,
}
impl NDR64_NON_CONFORMANT_STRING_FORMAT {}
impl ::std::default::Default for NDR64_NON_CONFORMANT_STRING_FORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_NON_CONFORMANT_STRING_FORMAT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_NON_CONFORMANT_STRING_FORMAT").field("Header", &self.Header).field("TotalSize", &self.TotalSize).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_NON_CONFORMANT_STRING_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.TotalSize == other.TotalSize
    }
}
impl ::std::cmp::Eq for NDR64_NON_CONFORMANT_STRING_FORMAT {}
unsafe impl ::windows::runtime::Abi for NDR64_NON_CONFORMANT_STRING_FORMAT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_NON_ENCAPSULATED_UNION {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: u8,
    pub SwitchType: u8,
    pub MemorySize: u32,
    pub Switch: *mut ::std::ffi::c_void,
    pub Reserved: u32,
}
impl NDR64_NON_ENCAPSULATED_UNION {}
impl ::std::default::Default for NDR64_NON_ENCAPSULATED_UNION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_NON_ENCAPSULATED_UNION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_NON_ENCAPSULATED_UNION").field("FormatCode", &self.FormatCode).field("Alignment", &self.Alignment).field("Flags", &self.Flags).field("SwitchType", &self.SwitchType).field("MemorySize", &self.MemorySize).field("Switch", &self.Switch).field("Reserved", &self.Reserved).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_NON_ENCAPSULATED_UNION {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Alignment == other.Alignment && self.Flags == other.Flags && self.SwitchType == other.SwitchType && self.MemorySize == other.MemorySize && self.Switch == other.Switch && self.Reserved == other.Reserved
    }
}
impl ::std::cmp::Eq for NDR64_NON_ENCAPSULATED_UNION {}
unsafe impl ::windows::runtime::Abi for NDR64_NON_ENCAPSULATED_UNION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_NO_REPEAT_FORMAT {
    pub FormatCode: u8,
    pub Flags: u8,
    pub Reserved1: u16,
    pub Reserved2: u32,
}
impl NDR64_NO_REPEAT_FORMAT {}
impl ::std::default::Default for NDR64_NO_REPEAT_FORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_NO_REPEAT_FORMAT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_NO_REPEAT_FORMAT").field("FormatCode", &self.FormatCode).field("Flags", &self.Flags).field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_NO_REPEAT_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Flags == other.Flags && self.Reserved1 == other.Reserved1 && self.Reserved2 == other.Reserved2
    }
}
impl ::std::cmp::Eq for NDR64_NO_REPEAT_FORMAT {}
unsafe impl ::windows::runtime::Abi for NDR64_NO_REPEAT_FORMAT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_PARAM_FLAGS {
    pub _bitfield: u16,
}
impl NDR64_PARAM_FLAGS {}
impl ::std::default::Default for NDR64_PARAM_FLAGS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_PARAM_FLAGS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_PARAM_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_PARAM_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for NDR64_PARAM_FLAGS {}
unsafe impl ::windows::runtime::Abi for NDR64_PARAM_FLAGS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_PARAM_FORMAT {
    pub Type: *mut ::std::ffi::c_void,
    pub Attributes: NDR64_PARAM_FLAGS,
    pub Reserved: u16,
    pub StackOffset: u32,
}
impl NDR64_PARAM_FORMAT {}
impl ::std::default::Default for NDR64_PARAM_FORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_PARAM_FORMAT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_PARAM_FORMAT").field("Type", &self.Type).field("Attributes", &self.Attributes).field("Reserved", &self.Reserved).field("StackOffset", &self.StackOffset).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_PARAM_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Attributes == other.Attributes && self.Reserved == other.Reserved && self.StackOffset == other.StackOffset
    }
}
impl ::std::cmp::Eq for NDR64_PARAM_FORMAT {}
unsafe impl ::windows::runtime::Abi for NDR64_PARAM_FORMAT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_PIPE_FLAGS {
    pub _bitfield: u8,
}
impl NDR64_PIPE_FLAGS {}
impl ::std::default::Default for NDR64_PIPE_FLAGS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_PIPE_FLAGS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_PIPE_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_PIPE_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for NDR64_PIPE_FLAGS {}
unsafe impl ::windows::runtime::Abi for NDR64_PIPE_FLAGS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_PIPE_FORMAT {
    pub FormatCode: u8,
    pub Flags: u8,
    pub Alignment: u8,
    pub Reserved: u8,
    pub Type: *mut ::std::ffi::c_void,
    pub MemorySize: u32,
    pub BufferSize: u32,
}
impl NDR64_PIPE_FORMAT {}
impl ::std::default::Default for NDR64_PIPE_FORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_PIPE_FORMAT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_PIPE_FORMAT").field("FormatCode", &self.FormatCode).field("Flags", &self.Flags).field("Alignment", &self.Alignment).field("Reserved", &self.Reserved).field("Type", &self.Type).field("MemorySize", &self.MemorySize).field("BufferSize", &self.BufferSize).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_PIPE_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Flags == other.Flags && self.Alignment == other.Alignment && self.Reserved == other.Reserved && self.Type == other.Type && self.MemorySize == other.MemorySize && self.BufferSize == other.BufferSize
    }
}
impl ::std::cmp::Eq for NDR64_PIPE_FORMAT {}
unsafe impl ::windows::runtime::Abi for NDR64_PIPE_FORMAT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_POINTER_FORMAT {
    pub FormatCode: u8,
    pub Flags: u8,
    pub Reserved: u16,
    pub Pointee: *mut ::std::ffi::c_void,
}
impl NDR64_POINTER_FORMAT {}
impl ::std::default::Default for NDR64_POINTER_FORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_POINTER_FORMAT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_POINTER_FORMAT").field("FormatCode", &self.FormatCode).field("Flags", &self.Flags).field("Reserved", &self.Reserved).field("Pointee", &self.Pointee).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_POINTER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Flags == other.Flags && self.Reserved == other.Reserved && self.Pointee == other.Pointee
    }
}
impl ::std::cmp::Eq for NDR64_POINTER_FORMAT {}
unsafe impl ::windows::runtime::Abi for NDR64_POINTER_FORMAT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_POINTER_INSTANCE_HEADER_FORMAT {
    pub Offset: u32,
    pub Reserved: u32,
}
impl NDR64_POINTER_INSTANCE_HEADER_FORMAT {}
impl ::std::default::Default for NDR64_POINTER_INSTANCE_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_POINTER_INSTANCE_HEADER_FORMAT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_POINTER_INSTANCE_HEADER_FORMAT").field("Offset", &self.Offset).field("Reserved", &self.Reserved).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_POINTER_INSTANCE_HEADER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.Offset == other.Offset && self.Reserved == other.Reserved
    }
}
impl ::std::cmp::Eq for NDR64_POINTER_INSTANCE_HEADER_FORMAT {}
unsafe impl ::windows::runtime::Abi for NDR64_POINTER_INSTANCE_HEADER_FORMAT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_POINTER_REPEAT_FLAGS {
    pub _bitfield: u8,
}
impl NDR64_POINTER_REPEAT_FLAGS {}
impl ::std::default::Default for NDR64_POINTER_REPEAT_FLAGS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_POINTER_REPEAT_FLAGS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_POINTER_REPEAT_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_POINTER_REPEAT_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for NDR64_POINTER_REPEAT_FLAGS {}
unsafe impl ::windows::runtime::Abi for NDR64_POINTER_REPEAT_FLAGS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_PROC_FLAGS {
    pub _bitfield: u32,
}
impl NDR64_PROC_FLAGS {}
impl ::std::default::Default for NDR64_PROC_FLAGS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_PROC_FLAGS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_PROC_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_PROC_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for NDR64_PROC_FLAGS {}
unsafe impl ::windows::runtime::Abi for NDR64_PROC_FLAGS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
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
impl NDR64_PROC_FORMAT {}
impl ::std::default::Default for NDR64_PROC_FORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_PROC_FORMAT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_PROC_FORMAT")
            .field("Flags", &self.Flags)
            .field("StackSize", &self.StackSize)
            .field("ConstantClientBufferSize", &self.ConstantClientBufferSize)
            .field("ConstantServerBufferSize", &self.ConstantServerBufferSize)
            .field("RpcFlags", &self.RpcFlags)
            .field("FloatDoubleMask", &self.FloatDoubleMask)
            .field("NumberOfParams", &self.NumberOfParams)
            .field("ExtensionSize", &self.ExtensionSize)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_PROC_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.StackSize == other.StackSize && self.ConstantClientBufferSize == other.ConstantClientBufferSize && self.ConstantServerBufferSize == other.ConstantServerBufferSize && self.RpcFlags == other.RpcFlags && self.FloatDoubleMask == other.FloatDoubleMask && self.NumberOfParams == other.NumberOfParams && self.ExtensionSize == other.ExtensionSize
    }
}
impl ::std::cmp::Eq for NDR64_PROC_FORMAT {}
unsafe impl ::windows::runtime::Abi for NDR64_PROC_FORMAT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_RANGED_STRING_FORMAT {
    pub Header: NDR64_STRING_HEADER_FORMAT,
    pub Reserved: u32,
    pub Min: u64,
    pub Max: u64,
}
impl NDR64_RANGED_STRING_FORMAT {}
impl ::std::default::Default for NDR64_RANGED_STRING_FORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_RANGED_STRING_FORMAT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_RANGED_STRING_FORMAT").field("Header", &self.Header).field("Reserved", &self.Reserved).field("Min", &self.Min).field("Max", &self.Max).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_RANGED_STRING_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Reserved == other.Reserved && self.Min == other.Min && self.Max == other.Max
    }
}
impl ::std::cmp::Eq for NDR64_RANGED_STRING_FORMAT {}
unsafe impl ::windows::runtime::Abi for NDR64_RANGED_STRING_FORMAT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_RANGE_FORMAT {
    pub FormatCode: u8,
    pub RangeType: u8,
    pub Reserved: u16,
    pub MinValue: i64,
    pub MaxValue: i64,
}
impl NDR64_RANGE_FORMAT {}
impl ::std::default::Default for NDR64_RANGE_FORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_RANGE_FORMAT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_RANGE_FORMAT").field("FormatCode", &self.FormatCode).field("RangeType", &self.RangeType).field("Reserved", &self.Reserved).field("MinValue", &self.MinValue).field("MaxValue", &self.MaxValue).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_RANGE_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.RangeType == other.RangeType && self.Reserved == other.Reserved && self.MinValue == other.MinValue && self.MaxValue == other.MaxValue
    }
}
impl ::std::cmp::Eq for NDR64_RANGE_FORMAT {}
unsafe impl ::windows::runtime::Abi for NDR64_RANGE_FORMAT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_RANGE_PIPE_FORMAT {
    pub FormatCode: u8,
    pub Flags: u8,
    pub Alignment: u8,
    pub Reserved: u8,
    pub Type: *mut ::std::ffi::c_void,
    pub MemorySize: u32,
    pub BufferSize: u32,
    pub MinValue: u32,
    pub MaxValue: u32,
}
impl NDR64_RANGE_PIPE_FORMAT {}
impl ::std::default::Default for NDR64_RANGE_PIPE_FORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_RANGE_PIPE_FORMAT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_RANGE_PIPE_FORMAT")
            .field("FormatCode", &self.FormatCode)
            .field("Flags", &self.Flags)
            .field("Alignment", &self.Alignment)
            .field("Reserved", &self.Reserved)
            .field("Type", &self.Type)
            .field("MemorySize", &self.MemorySize)
            .field("BufferSize", &self.BufferSize)
            .field("MinValue", &self.MinValue)
            .field("MaxValue", &self.MaxValue)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_RANGE_PIPE_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Flags == other.Flags && self.Alignment == other.Alignment && self.Reserved == other.Reserved && self.Type == other.Type && self.MemorySize == other.MemorySize && self.BufferSize == other.BufferSize && self.MinValue == other.MinValue && self.MaxValue == other.MaxValue
    }
}
impl ::std::cmp::Eq for NDR64_RANGE_PIPE_FORMAT {}
unsafe impl ::windows::runtime::Abi for NDR64_RANGE_PIPE_FORMAT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_REPEAT_FORMAT {
    pub FormatCode: u8,
    pub Flags: NDR64_POINTER_REPEAT_FLAGS,
    pub Reserved: u16,
    pub Increment: u32,
    pub OffsetToArray: u32,
    pub NumberOfPointers: u32,
}
impl NDR64_REPEAT_FORMAT {}
impl ::std::default::Default for NDR64_REPEAT_FORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_REPEAT_FORMAT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_REPEAT_FORMAT").field("FormatCode", &self.FormatCode).field("Flags", &self.Flags).field("Reserved", &self.Reserved).field("Increment", &self.Increment).field("OffsetToArray", &self.OffsetToArray).field("NumberOfPointers", &self.NumberOfPointers).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_REPEAT_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Flags == other.Flags && self.Reserved == other.Reserved && self.Increment == other.Increment && self.OffsetToArray == other.OffsetToArray && self.NumberOfPointers == other.NumberOfPointers
    }
}
impl ::std::cmp::Eq for NDR64_REPEAT_FORMAT {}
unsafe impl ::windows::runtime::Abi for NDR64_REPEAT_FORMAT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_RPC_FLAGS {
    pub _bitfield: u16,
}
impl NDR64_RPC_FLAGS {}
impl ::std::default::Default for NDR64_RPC_FLAGS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_RPC_FLAGS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_RPC_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_RPC_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for NDR64_RPC_FLAGS {}
unsafe impl ::windows::runtime::Abi for NDR64_RPC_FLAGS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_SIMPLE_MEMBER_FORMAT {
    pub FormatCode: u8,
    pub Reserved1: u8,
    pub Reserved2: u16,
    pub Reserved3: u32,
}
impl NDR64_SIMPLE_MEMBER_FORMAT {}
impl ::std::default::Default for NDR64_SIMPLE_MEMBER_FORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_SIMPLE_MEMBER_FORMAT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_SIMPLE_MEMBER_FORMAT").field("FormatCode", &self.FormatCode).field("Reserved1", &self.Reserved1).field("Reserved2", &self.Reserved2).field("Reserved3", &self.Reserved3).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_SIMPLE_MEMBER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Reserved1 == other.Reserved1 && self.Reserved2 == other.Reserved2 && self.Reserved3 == other.Reserved3
    }
}
impl ::std::cmp::Eq for NDR64_SIMPLE_MEMBER_FORMAT {}
unsafe impl ::windows::runtime::Abi for NDR64_SIMPLE_MEMBER_FORMAT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_SIMPLE_REGION_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub RegionSize: u16,
    pub Reserved: u32,
}
impl NDR64_SIMPLE_REGION_FORMAT {}
impl ::std::default::Default for NDR64_SIMPLE_REGION_FORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_SIMPLE_REGION_FORMAT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_SIMPLE_REGION_FORMAT").field("FormatCode", &self.FormatCode).field("Alignment", &self.Alignment).field("RegionSize", &self.RegionSize).field("Reserved", &self.Reserved).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_SIMPLE_REGION_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Alignment == other.Alignment && self.RegionSize == other.RegionSize && self.Reserved == other.Reserved
    }
}
impl ::std::cmp::Eq for NDR64_SIMPLE_REGION_FORMAT {}
unsafe impl ::windows::runtime::Abi for NDR64_SIMPLE_REGION_FORMAT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_SIZED_CONFORMANT_STRING_FORMAT {
    pub Header: NDR64_STRING_HEADER_FORMAT,
    pub SizeDescription: *mut ::std::ffi::c_void,
}
impl NDR64_SIZED_CONFORMANT_STRING_FORMAT {}
impl ::std::default::Default for NDR64_SIZED_CONFORMANT_STRING_FORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_SIZED_CONFORMANT_STRING_FORMAT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_SIZED_CONFORMANT_STRING_FORMAT").field("Header", &self.Header).field("SizeDescription", &self.SizeDescription).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_SIZED_CONFORMANT_STRING_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.SizeDescription == other.SizeDescription
    }
}
impl ::std::cmp::Eq for NDR64_SIZED_CONFORMANT_STRING_FORMAT {}
unsafe impl ::windows::runtime::Abi for NDR64_SIZED_CONFORMANT_STRING_FORMAT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_STRING_FLAGS {
    pub _bitfield: u8,
}
impl NDR64_STRING_FLAGS {}
impl ::std::default::Default for NDR64_STRING_FLAGS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_STRING_FLAGS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_STRING_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_STRING_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for NDR64_STRING_FLAGS {}
unsafe impl ::windows::runtime::Abi for NDR64_STRING_FLAGS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_STRING_HEADER_FORMAT {
    pub FormatCode: u8,
    pub Flags: NDR64_STRING_FLAGS,
    pub ElementSize: u16,
}
impl NDR64_STRING_HEADER_FORMAT {}
impl ::std::default::Default for NDR64_STRING_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_STRING_HEADER_FORMAT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_STRING_HEADER_FORMAT").field("FormatCode", &self.FormatCode).field("Flags", &self.Flags).field("ElementSize", &self.ElementSize).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_STRING_HEADER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Flags == other.Flags && self.ElementSize == other.ElementSize
    }
}
impl ::std::cmp::Eq for NDR64_STRING_HEADER_FORMAT {}
unsafe impl ::windows::runtime::Abi for NDR64_STRING_HEADER_FORMAT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_STRUCTURE_FLAGS {
    pub _bitfield: u8,
}
impl NDR64_STRUCTURE_FLAGS {}
impl ::std::default::Default for NDR64_STRUCTURE_FLAGS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_STRUCTURE_FLAGS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_STRUCTURE_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_STRUCTURE_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for NDR64_STRUCTURE_FLAGS {}
unsafe impl ::windows::runtime::Abi for NDR64_STRUCTURE_FLAGS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_STRUCTURE_HEADER_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: NDR64_STRUCTURE_FLAGS,
    pub Reserve: u8,
    pub MemorySize: u32,
}
impl NDR64_STRUCTURE_HEADER_FORMAT {}
impl ::std::default::Default for NDR64_STRUCTURE_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_STRUCTURE_HEADER_FORMAT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_STRUCTURE_HEADER_FORMAT").field("FormatCode", &self.FormatCode).field("Alignment", &self.Alignment).field("Flags", &self.Flags).field("Reserve", &self.Reserve).field("MemorySize", &self.MemorySize).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_STRUCTURE_HEADER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Alignment == other.Alignment && self.Flags == other.Flags && self.Reserve == other.Reserve && self.MemorySize == other.MemorySize
    }
}
impl ::std::cmp::Eq for NDR64_STRUCTURE_HEADER_FORMAT {}
unsafe impl ::windows::runtime::Abi for NDR64_STRUCTURE_HEADER_FORMAT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_SYSTEM_HANDLE_FORMAT {
    pub FormatCode: u8,
    pub HandleType: u8,
    pub DesiredAccess: u32,
}
impl NDR64_SYSTEM_HANDLE_FORMAT {}
impl ::std::default::Default for NDR64_SYSTEM_HANDLE_FORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_SYSTEM_HANDLE_FORMAT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_SYSTEM_HANDLE_FORMAT").field("FormatCode", &self.FormatCode).field("HandleType", &self.HandleType).field("DesiredAccess", &self.DesiredAccess).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_SYSTEM_HANDLE_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.HandleType == other.HandleType && self.DesiredAccess == other.DesiredAccess
    }
}
impl ::std::cmp::Eq for NDR64_SYSTEM_HANDLE_FORMAT {}
unsafe impl ::windows::runtime::Abi for NDR64_SYSTEM_HANDLE_FORMAT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_TRANSMIT_AS_FLAGS {
    pub _bitfield: u8,
}
impl NDR64_TRANSMIT_AS_FLAGS {}
impl ::std::default::Default for NDR64_TRANSMIT_AS_FLAGS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_TRANSMIT_AS_FLAGS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_TRANSMIT_AS_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_TRANSMIT_AS_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for NDR64_TRANSMIT_AS_FLAGS {}
unsafe impl ::windows::runtime::Abi for NDR64_TRANSMIT_AS_FLAGS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_TRANSMIT_AS_FORMAT {
    pub FormatCode: u8,
    pub Flags: u8,
    pub RoutineIndex: u16,
    pub TransmittedTypeWireAlignment: u16,
    pub MemoryAlignment: u16,
    pub PresentedTypeMemorySize: u32,
    pub TransmittedTypeBufferSize: u32,
    pub TransmittedType: *mut ::std::ffi::c_void,
}
impl NDR64_TRANSMIT_AS_FORMAT {}
impl ::std::default::Default for NDR64_TRANSMIT_AS_FORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_TRANSMIT_AS_FORMAT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_TRANSMIT_AS_FORMAT")
            .field("FormatCode", &self.FormatCode)
            .field("Flags", &self.Flags)
            .field("RoutineIndex", &self.RoutineIndex)
            .field("TransmittedTypeWireAlignment", &self.TransmittedTypeWireAlignment)
            .field("MemoryAlignment", &self.MemoryAlignment)
            .field("PresentedTypeMemorySize", &self.PresentedTypeMemorySize)
            .field("TransmittedTypeBufferSize", &self.TransmittedTypeBufferSize)
            .field("TransmittedType", &self.TransmittedType)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_TRANSMIT_AS_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Flags == other.Flags && self.RoutineIndex == other.RoutineIndex && self.TransmittedTypeWireAlignment == other.TransmittedTypeWireAlignment && self.MemoryAlignment == other.MemoryAlignment && self.PresentedTypeMemorySize == other.PresentedTypeMemorySize && self.TransmittedTypeBufferSize == other.TransmittedTypeBufferSize && self.TransmittedType == other.TransmittedType
    }
}
impl ::std::cmp::Eq for NDR64_TRANSMIT_AS_FORMAT {}
unsafe impl ::windows::runtime::Abi for NDR64_TRANSMIT_AS_FORMAT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_TYPE_STRICT_CONTEXT_HANDLE {
    pub FormatCode: u8,
    pub RealFormatCode: u8,
    pub Reserved: u16,
    pub Type: *mut ::std::ffi::c_void,
    pub CtxtFlags: u32,
    pub CtxtID: u32,
}
impl NDR64_TYPE_STRICT_CONTEXT_HANDLE {}
impl ::std::default::Default for NDR64_TYPE_STRICT_CONTEXT_HANDLE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_TYPE_STRICT_CONTEXT_HANDLE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_TYPE_STRICT_CONTEXT_HANDLE").field("FormatCode", &self.FormatCode).field("RealFormatCode", &self.RealFormatCode).field("Reserved", &self.Reserved).field("Type", &self.Type).field("CtxtFlags", &self.CtxtFlags).field("CtxtID", &self.CtxtID).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_TYPE_STRICT_CONTEXT_HANDLE {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.RealFormatCode == other.RealFormatCode && self.Reserved == other.Reserved && self.Type == other.Type && self.CtxtFlags == other.CtxtFlags && self.CtxtID == other.CtxtID
    }
}
impl ::std::cmp::Eq for NDR64_TYPE_STRICT_CONTEXT_HANDLE {}
unsafe impl ::windows::runtime::Abi for NDR64_TYPE_STRICT_CONTEXT_HANDLE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_UNION_ARM {
    pub CaseValue: i64,
    pub Type: *mut ::std::ffi::c_void,
    pub Reserved: u32,
}
impl NDR64_UNION_ARM {}
impl ::std::default::Default for NDR64_UNION_ARM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_UNION_ARM {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_UNION_ARM").field("CaseValue", &self.CaseValue).field("Type", &self.Type).field("Reserved", &self.Reserved).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_UNION_ARM {
    fn eq(&self, other: &Self) -> bool {
        self.CaseValue == other.CaseValue && self.Type == other.Type && self.Reserved == other.Reserved
    }
}
impl ::std::cmp::Eq for NDR64_UNION_ARM {}
unsafe impl ::windows::runtime::Abi for NDR64_UNION_ARM {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_UNION_ARM_SELECTOR {
    pub Reserved1: u8,
    pub Alignment: u8,
    pub Reserved2: u16,
    pub Arms: u32,
}
impl NDR64_UNION_ARM_SELECTOR {}
impl ::std::default::Default for NDR64_UNION_ARM_SELECTOR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_UNION_ARM_SELECTOR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_UNION_ARM_SELECTOR").field("Reserved1", &self.Reserved1).field("Alignment", &self.Alignment).field("Reserved2", &self.Reserved2).field("Arms", &self.Arms).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_UNION_ARM_SELECTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Reserved1 == other.Reserved1 && self.Alignment == other.Alignment && self.Reserved2 == other.Reserved2 && self.Arms == other.Arms
    }
}
impl ::std::cmp::Eq for NDR64_UNION_ARM_SELECTOR {}
unsafe impl ::windows::runtime::Abi for NDR64_UNION_ARM_SELECTOR {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_USER_MARSHAL_FLAGS {
    pub _bitfield: u8,
}
impl NDR64_USER_MARSHAL_FLAGS {}
impl ::std::default::Default for NDR64_USER_MARSHAL_FLAGS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_USER_MARSHAL_FLAGS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_USER_MARSHAL_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_USER_MARSHAL_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::std::cmp::Eq for NDR64_USER_MARSHAL_FLAGS {}
unsafe impl ::windows::runtime::Abi for NDR64_USER_MARSHAL_FLAGS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_USER_MARSHAL_FORMAT {
    pub FormatCode: u8,
    pub Flags: u8,
    pub RoutineIndex: u16,
    pub TransmittedTypeWireAlignment: u16,
    pub MemoryAlignment: u16,
    pub UserTypeMemorySize: u32,
    pub TransmittedTypeBufferSize: u32,
    pub TransmittedType: *mut ::std::ffi::c_void,
}
impl NDR64_USER_MARSHAL_FORMAT {}
impl ::std::default::Default for NDR64_USER_MARSHAL_FORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_USER_MARSHAL_FORMAT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_USER_MARSHAL_FORMAT")
            .field("FormatCode", &self.FormatCode)
            .field("Flags", &self.Flags)
            .field("RoutineIndex", &self.RoutineIndex)
            .field("TransmittedTypeWireAlignment", &self.TransmittedTypeWireAlignment)
            .field("MemoryAlignment", &self.MemoryAlignment)
            .field("UserTypeMemorySize", &self.UserTypeMemorySize)
            .field("TransmittedTypeBufferSize", &self.TransmittedTypeBufferSize)
            .field("TransmittedType", &self.TransmittedType)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_USER_MARSHAL_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Flags == other.Flags && self.RoutineIndex == other.RoutineIndex && self.TransmittedTypeWireAlignment == other.TransmittedTypeWireAlignment && self.MemoryAlignment == other.MemoryAlignment && self.UserTypeMemorySize == other.UserTypeMemorySize && self.TransmittedTypeBufferSize == other.TransmittedTypeBufferSize && self.TransmittedType == other.TransmittedType
    }
}
impl ::std::cmp::Eq for NDR64_USER_MARSHAL_FORMAT {}
unsafe impl ::windows::runtime::Abi for NDR64_USER_MARSHAL_FORMAT {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR64_VAR_ARRAY_HEADER_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: NDR64_ARRAY_FLAGS,
    pub Reserved: u8,
    pub TotalSize: u32,
    pub ElementSize: u32,
    pub VarDescriptor: *mut ::std::ffi::c_void,
}
impl NDR64_VAR_ARRAY_HEADER_FORMAT {}
impl ::std::default::Default for NDR64_VAR_ARRAY_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR64_VAR_ARRAY_HEADER_FORMAT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR64_VAR_ARRAY_HEADER_FORMAT")
            .field("FormatCode", &self.FormatCode)
            .field("Alignment", &self.Alignment)
            .field("Flags", &self.Flags)
            .field("Reserved", &self.Reserved)
            .field("TotalSize", &self.TotalSize)
            .field("ElementSize", &self.ElementSize)
            .field("VarDescriptor", &self.VarDescriptor)
            .finish()
    }
}
impl ::std::cmp::PartialEq for NDR64_VAR_ARRAY_HEADER_FORMAT {
    fn eq(&self, other: &Self) -> bool {
        self.FormatCode == other.FormatCode && self.Alignment == other.Alignment && self.Flags == other.Flags && self.Reserved == other.Reserved && self.TotalSize == other.TotalSize && self.ElementSize == other.ElementSize && self.VarDescriptor == other.VarDescriptor
    }
}
impl ::std::cmp::Eq for NDR64_VAR_ARRAY_HEADER_FORMAT {}
unsafe impl ::windows::runtime::Abi for NDR64_VAR_ARRAY_HEADER_FORMAT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn NDRCContextBinding(ccontext: isize) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NDRCContextBinding(ccontext: isize) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(NDRCContextBinding(::std::mem::transmute(ccontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn NDRCContextMarshall(ccontext: isize, pbuff: *mut ::std::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NDRCContextMarshall(ccontext: isize, pbuff: *mut ::std::ffi::c_void);
        }
        ::std::mem::transmute(NDRCContextMarshall(::std::mem::transmute(ccontext), ::std::mem::transmute(pbuff)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn NDRCContextUnmarshall(pccontext: *mut isize, hbinding: *const ::std::ffi::c_void, pbuff: *const ::std::ffi::c_void, datarepresentation: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NDRCContextUnmarshall(pccontext: *mut isize, hbinding: *const ::std::ffi::c_void, pbuff: *const ::std::ffi::c_void, datarepresentation: u32);
        }
        ::std::mem::transmute(NDRCContextUnmarshall(::std::mem::transmute(pccontext), ::std::mem::transmute(hbinding), ::std::mem::transmute(pbuff), ::std::mem::transmute(datarepresentation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn NDRSContextMarshall(ccontext: *const NDR_SCONTEXT_1, pbuff: *mut ::std::ffi::c_void, userrundownin: ::std::option::Option<NDR_RUNDOWN>) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NDRSContextMarshall(ccontext: *const NDR_SCONTEXT_1, pbuff: *mut ::std::ffi::c_void, userrundownin: ::windows::runtime::RawPtr);
        }
        ::std::mem::transmute(NDRSContextMarshall(::std::mem::transmute(ccontext), ::std::mem::transmute(pbuff), ::std::mem::transmute(userrundownin)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn NDRSContextMarshall2(bindinghandle: *const ::std::ffi::c_void, ccontext: *const NDR_SCONTEXT_1, pbuff: *mut ::std::ffi::c_void, userrundownin: ::std::option::Option<NDR_RUNDOWN>, ctxguard: *const ::std::ffi::c_void, flags: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NDRSContextMarshall2(bindinghandle: *const ::std::ffi::c_void, ccontext: *const NDR_SCONTEXT_1, pbuff: *mut ::std::ffi::c_void, userrundownin: ::windows::runtime::RawPtr, ctxguard: *const ::std::ffi::c_void, flags: u32);
        }
        ::std::mem::transmute(NDRSContextMarshall2(::std::mem::transmute(bindinghandle), ::std::mem::transmute(ccontext), ::std::mem::transmute(pbuff), ::std::mem::transmute(userrundownin), ::std::mem::transmute(ctxguard), ::std::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn NDRSContextMarshallEx(bindinghandle: *const ::std::ffi::c_void, ccontext: *const NDR_SCONTEXT_1, pbuff: *mut ::std::ffi::c_void, userrundownin: ::std::option::Option<NDR_RUNDOWN>) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NDRSContextMarshallEx(bindinghandle: *const ::std::ffi::c_void, ccontext: *const NDR_SCONTEXT_1, pbuff: *mut ::std::ffi::c_void, userrundownin: ::windows::runtime::RawPtr);
        }
        ::std::mem::transmute(NDRSContextMarshallEx(::std::mem::transmute(bindinghandle), ::std::mem::transmute(ccontext), ::std::mem::transmute(pbuff), ::std::mem::transmute(userrundownin)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn NDRSContextUnmarshall(pbuff: *const ::std::ffi::c_void, datarepresentation: u32) -> *mut NDR_SCONTEXT_1 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NDRSContextUnmarshall(pbuff: *const ::std::ffi::c_void, datarepresentation: u32) -> *mut NDR_SCONTEXT_1;
        }
        ::std::mem::transmute(NDRSContextUnmarshall(::std::mem::transmute(pbuff), ::std::mem::transmute(datarepresentation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn NDRSContextUnmarshall2(bindinghandle: *const ::std::ffi::c_void, pbuff: *const ::std::ffi::c_void, datarepresentation: u32, ctxguard: *const ::std::ffi::c_void, flags: u32) -> *mut NDR_SCONTEXT_1 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NDRSContextUnmarshall2(bindinghandle: *const ::std::ffi::c_void, pbuff: *const ::std::ffi::c_void, datarepresentation: u32, ctxguard: *const ::std::ffi::c_void, flags: u32) -> *mut NDR_SCONTEXT_1;
        }
        ::std::mem::transmute(NDRSContextUnmarshall2(::std::mem::transmute(bindinghandle), ::std::mem::transmute(pbuff), ::std::mem::transmute(datarepresentation), ::std::mem::transmute(ctxguard), ::std::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn NDRSContextUnmarshallEx(bindinghandle: *const ::std::ffi::c_void, pbuff: *const ::std::ffi::c_void, datarepresentation: u32) -> *mut NDR_SCONTEXT_1 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NDRSContextUnmarshallEx(bindinghandle: *const ::std::ffi::c_void, pbuff: *const ::std::ffi::c_void, datarepresentation: u32) -> *mut NDR_SCONTEXT_1;
        }
        ::std::mem::transmute(NDRSContextUnmarshallEx(::std::mem::transmute(bindinghandle), ::std::mem::transmute(pbuff), ::std::mem::transmute(datarepresentation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct NDR_ALLOC_ALL_NODES_CONTEXT(pub u8);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR_CS_ROUTINES {
    pub pSizeConvertRoutines: *mut NDR_CS_SIZE_CONVERT_ROUTINES,
    pub pTagGettingRoutines: *mut ::std::option::Option<CS_TAG_GETTING_ROUTINE>,
}
impl NDR_CS_ROUTINES {}
impl ::std::default::Default for NDR_CS_ROUTINES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR_CS_ROUTINES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR_CS_ROUTINES").field("pSizeConvertRoutines", &self.pSizeConvertRoutines).finish()
    }
}
impl ::std::cmp::PartialEq for NDR_CS_ROUTINES {
    fn eq(&self, other: &Self) -> bool {
        self.pSizeConvertRoutines == other.pSizeConvertRoutines && self.pTagGettingRoutines == other.pTagGettingRoutines
    }
}
impl ::std::cmp::Eq for NDR_CS_ROUTINES {}
unsafe impl ::windows::runtime::Abi for NDR_CS_ROUTINES {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR_CS_SIZE_CONVERT_ROUTINES {
    pub pfnNetSize: ::std::option::Option<CS_TYPE_NET_SIZE_ROUTINE>,
    pub pfnToNetCs: ::std::option::Option<CS_TYPE_TO_NETCS_ROUTINE>,
    pub pfnLocalSize: ::std::option::Option<CS_TYPE_LOCAL_SIZE_ROUTINE>,
    pub pfnFromNetCs: ::std::option::Option<CS_TYPE_FROM_NETCS_ROUTINE>,
}
impl NDR_CS_SIZE_CONVERT_ROUTINES {}
impl ::std::default::Default for NDR_CS_SIZE_CONVERT_ROUTINES {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR_CS_SIZE_CONVERT_ROUTINES {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR_CS_SIZE_CONVERT_ROUTINES").finish()
    }
}
impl ::std::cmp::PartialEq for NDR_CS_SIZE_CONVERT_ROUTINES {
    fn eq(&self, other: &Self) -> bool {
        self.pfnNetSize.map(|f| f as usize) == other.pfnNetSize.map(|f| f as usize) && self.pfnToNetCs.map(|f| f as usize) == other.pfnToNetCs.map(|f| f as usize) && self.pfnLocalSize.map(|f| f as usize) == other.pfnLocalSize.map(|f| f as usize) && self.pfnFromNetCs.map(|f| f as usize) == other.pfnFromNetCs.map(|f| f as usize)
    }
}
impl ::std::cmp::Eq for NDR_CS_SIZE_CONVERT_ROUTINES {}
unsafe impl ::windows::runtime::Abi for NDR_CS_SIZE_CONVERT_ROUTINES {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const NDR_CUSTOM_OR_DEFAULT_ALLOCATOR: u32 = 268435456u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const NDR_DEFAULT_ALLOCATOR: u32 = 536870912u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR_EXPR_DESC {
    pub pOffset: *mut u16,
    pub pFormatExpr: *mut u8,
}
impl NDR_EXPR_DESC {}
impl ::std::default::Default for NDR_EXPR_DESC {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR_EXPR_DESC {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR_EXPR_DESC").field("pOffset", &self.pOffset).field("pFormatExpr", &self.pFormatExpr).finish()
    }
}
impl ::std::cmp::PartialEq for NDR_EXPR_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.pOffset == other.pOffset && self.pFormatExpr == other.pFormatExpr
    }
}
impl ::std::cmp::Eq for NDR_EXPR_DESC {}
unsafe impl ::windows::runtime::Abi for NDR_EXPR_DESC {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub type NDR_NOTIFY2_ROUTINE = unsafe extern "system" fn(flag: u8);
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub type NDR_NOTIFY_ROUTINE = unsafe extern "system" fn();
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct NDR_POINTER_QUEUE_STATE(pub u8);
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub type NDR_RUNDOWN = unsafe extern "system" fn(context: *mut ::std::ffi::c_void);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct NDR_SCONTEXT_1 {
    pub pad: [*mut ::std::ffi::c_void; 2],
    pub userContext: *mut ::std::ffi::c_void,
}
impl NDR_SCONTEXT_1 {}
impl ::std::default::Default for NDR_SCONTEXT_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for NDR_SCONTEXT_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR_SCONTEXT_1").field("pad", &self.pad).field("userContext", &self.userContext).finish()
    }
}
impl ::std::cmp::PartialEq for NDR_SCONTEXT_1 {
    fn eq(&self, other: &Self) -> bool {
        self.pad == other.pad && self.userContext == other.userContext
    }
}
impl ::std::cmp::Eq for NDR_SCONTEXT_1 {}
unsafe impl ::windows::runtime::Abi for NDR_SCONTEXT_1 {
    type Abi = Self;
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::clone::Clone for NDR_USER_MARSHAL_INFO {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
pub struct NDR_USER_MARSHAL_INFO {
    pub InformationLevel: u32,
    pub Anonymous: NDR_USER_MARSHAL_INFO_0,
}
#[cfg(feature = "Win32_System_Com")]
impl NDR_USER_MARSHAL_INFO {}
#[cfg(feature = "Win32_System_Com")]
impl ::std::default::Default for NDR_USER_MARSHAL_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::PartialEq for NDR_USER_MARSHAL_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::Eq for NDR_USER_MARSHAL_INFO {}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::runtime::Abi for NDR_USER_MARSHAL_INFO {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::clone::Clone for NDR_USER_MARSHAL_INFO_0 {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
pub union NDR_USER_MARSHAL_INFO_0 {
    pub Level1: ::std::mem::ManuallyDrop<NDR_USER_MARSHAL_INFO_LEVEL1>,
}
#[cfg(feature = "Win32_System_Com")]
impl NDR_USER_MARSHAL_INFO_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::std::default::Default for NDR_USER_MARSHAL_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::PartialEq for NDR_USER_MARSHAL_INFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::Eq for NDR_USER_MARSHAL_INFO_0 {}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::runtime::Abi for NDR_USER_MARSHAL_INFO_0 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
pub struct NDR_USER_MARSHAL_INFO_LEVEL1 {
    pub Buffer: *mut ::std::ffi::c_void,
    pub BufferSize: u32,
    pub pfnAllocate: isize,
    pub pfnFree: isize,
    pub pRpcChannelBuffer: ::std::option::Option<super::Com::IRpcChannelBuffer>,
    pub Reserved: [usize; 5],
}
#[cfg(feature = "Win32_System_Com")]
impl NDR_USER_MARSHAL_INFO_LEVEL1 {}
#[cfg(feature = "Win32_System_Com")]
impl ::std::default::Default for NDR_USER_MARSHAL_INFO_LEVEL1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::fmt::Debug for NDR_USER_MARSHAL_INFO_LEVEL1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("NDR_USER_MARSHAL_INFO_LEVEL1").field("Buffer", &self.Buffer).field("BufferSize", &self.BufferSize).field("pfnAllocate", &self.pfnAllocate).field("pfnFree", &self.pfnFree).field("pRpcChannelBuffer", &self.pRpcChannelBuffer).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::PartialEq for NDR_USER_MARSHAL_INFO_LEVEL1 {
    fn eq(&self, other: &Self) -> bool {
        self.Buffer == other.Buffer && self.BufferSize == other.BufferSize && self.pfnAllocate == other.pfnAllocate && self.pfnFree == other.pfnFree && self.pRpcChannelBuffer == other.pRpcChannelBuffer && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::Eq for NDR_USER_MARSHAL_INFO_LEVEL1 {}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::runtime::Abi for NDR_USER_MARSHAL_INFO_LEVEL1 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const NT351_INTERFACE_SIZE: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn Ndr64AsyncClientCall(pproxyinfo: *mut MIDL_STUBLESS_PROXY_INFO, nprocnum: u32, preturnvalue: *mut ::std::ffi::c_void) -> CLIENT_CALL_RETURN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Ndr64AsyncClientCall(pproxyinfo: *mut MIDL_STUBLESS_PROXY_INFO, nprocnum: u32, preturnvalue: *mut ::std::ffi::c_void) -> CLIENT_CALL_RETURN;
        }
        ::std::mem::transmute(Ndr64AsyncClientCall(::std::mem::transmute(pproxyinfo), ::std::mem::transmute(nprocnum), ::std::mem::transmute(preturnvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn Ndr64AsyncServerCall64(prpcmsg: *mut RPC_MESSAGE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Ndr64AsyncServerCall64(prpcmsg: *mut RPC_MESSAGE);
        }
        ::std::mem::transmute(Ndr64AsyncServerCall64(::std::mem::transmute(prpcmsg)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn Ndr64AsyncServerCallAll(prpcmsg: *mut RPC_MESSAGE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Ndr64AsyncServerCallAll(prpcmsg: *mut RPC_MESSAGE);
        }
        ::std::mem::transmute(Ndr64AsyncServerCallAll(::std::mem::transmute(prpcmsg)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn Ndr64DcomAsyncClientCall(pproxyinfo: *mut MIDL_STUBLESS_PROXY_INFO, nprocnum: u32, preturnvalue: *mut ::std::ffi::c_void) -> CLIENT_CALL_RETURN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Ndr64DcomAsyncClientCall(pproxyinfo: *mut MIDL_STUBLESS_PROXY_INFO, nprocnum: u32, preturnvalue: *mut ::std::ffi::c_void) -> CLIENT_CALL_RETURN;
        }
        ::std::mem::transmute(Ndr64DcomAsyncClientCall(::std::mem::transmute(pproxyinfo), ::std::mem::transmute(nprocnum), ::std::mem::transmute(preturnvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn Ndr64DcomAsyncStubCall<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::IRpcStubBuffer>, Param1: ::windows::runtime::IntoParam<'a, super::Com::IRpcChannelBuffer>>(pthis: Param0, pchannel: Param1, prpcmsg: *mut RPC_MESSAGE, pdwstubphase: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn Ndr64DcomAsyncStubCall(pthis: ::windows::runtime::RawPtr, pchannel: ::windows::runtime::RawPtr, prpcmsg: *mut RPC_MESSAGE, pdwstubphase: *mut u32) -> i32;
        }
        ::std::mem::transmute(Ndr64DcomAsyncStubCall(pthis.into_param().abi(), pchannel.into_param().abi(), ::std::mem::transmute(prpcmsg), ::std::mem::transmute(pdwstubphase)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrAllocate(pstubmsg: *mut MIDL_STUB_MESSAGE, len: usize) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrAllocate(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, len: usize) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(NdrAllocate(::std::mem::transmute(pstubmsg), ::std::mem::transmute(len)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrAsyncClientCall(pstubdescriptor: *mut MIDL_STUB_DESC, pformat: *mut u8) -> CLIENT_CALL_RETURN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrAsyncClientCall(pstubdescriptor: *mut MIDL_STUB_DESC, pformat: *mut u8) -> CLIENT_CALL_RETURN;
        }
        ::std::mem::transmute(NdrAsyncClientCall(::std::mem::transmute(pstubdescriptor), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn NdrAsyncServerCall(prpcmsg: *mut RPC_MESSAGE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrAsyncServerCall(prpcmsg: *mut RPC_MESSAGE);
        }
        ::std::mem::transmute(NdrAsyncServerCall(::std::mem::transmute(prpcmsg)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrByteCountPointerBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrByteCountPointerBufferSize(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8);
        }
        ::std::mem::transmute(NdrByteCountPointerBufferSize(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrByteCountPointerFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrByteCountPointerFree(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8);
        }
        ::std::mem::transmute(NdrByteCountPointerFree(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrByteCountPointerMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrByteCountPointerMarshall(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
        }
        ::std::mem::transmute(NdrByteCountPointerMarshall(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrByteCountPointerUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrByteCountPointerUnmarshall(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
        }
        ::std::mem::transmute(NdrByteCountPointerUnmarshall(::std::mem::transmute(pstubmsg), ::std::mem::transmute(ppmemory), ::std::mem::transmute(pformat), ::std::mem::transmute(fmustalloc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrClearOutParameters(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8, argaddr: *mut ::std::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrClearOutParameters(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pformat: *mut u8, argaddr: *mut ::std::ffi::c_void);
        }
        ::std::mem::transmute(NdrClearOutParameters(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pformat), ::std::mem::transmute(argaddr)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrClientCall2(pstubdescriptor: *mut MIDL_STUB_DESC, pformat: *mut u8) -> CLIENT_CALL_RETURN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrClientCall2(pstubdescriptor: *mut MIDL_STUB_DESC, pformat: *mut u8) -> CLIENT_CALL_RETURN;
        }
        ::std::mem::transmute(NdrClientCall2(::std::mem::transmute(pstubdescriptor), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrClientCall3(pproxyinfo: *mut MIDL_STUBLESS_PROXY_INFO, nprocnum: u32, preturnvalue: *mut ::std::ffi::c_void) -> CLIENT_CALL_RETURN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrClientCall3(pproxyinfo: *mut MIDL_STUBLESS_PROXY_INFO, nprocnum: u32, preturnvalue: *mut ::std::ffi::c_void) -> CLIENT_CALL_RETURN;
        }
        ::std::mem::transmute(NdrClientCall3(::std::mem::transmute(pproxyinfo), ::std::mem::transmute(nprocnum), ::std::mem::transmute(preturnvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrClientContextMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, contexthandle: isize, fcheck: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrClientContextMarshall(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, contexthandle: isize, fcheck: i32);
        }
        ::std::mem::transmute(NdrClientContextMarshall(::std::mem::transmute(pstubmsg), ::std::mem::transmute(contexthandle), ::std::mem::transmute(fcheck)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrClientContextUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pcontexthandle: *mut isize, bindhandle: *mut ::std::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrClientContextUnmarshall(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pcontexthandle: *mut isize, bindhandle: *mut ::std::ffi::c_void);
        }
        ::std::mem::transmute(NdrClientContextUnmarshall(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pcontexthandle), ::std::mem::transmute(bindhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrClientInitialize(prpcmsg: *mut RPC_MESSAGE, pstubmsg: *mut MIDL_STUB_MESSAGE, pstubdescriptor: *mut MIDL_STUB_DESC, procnum: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrClientInitialize(prpcmsg: *mut RPC_MESSAGE, pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pstubdescriptor: *mut MIDL_STUB_DESC, procnum: u32);
        }
        ::std::mem::transmute(NdrClientInitialize(::std::mem::transmute(prpcmsg), ::std::mem::transmute(pstubmsg), ::std::mem::transmute(pstubdescriptor), ::std::mem::transmute(procnum)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrClientInitializeNew(prpcmsg: *mut RPC_MESSAGE, pstubmsg: *mut MIDL_STUB_MESSAGE, pstubdescriptor: *mut MIDL_STUB_DESC, procnum: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrClientInitializeNew(prpcmsg: *mut RPC_MESSAGE, pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pstubdescriptor: *mut MIDL_STUB_DESC, procnum: u32);
        }
        ::std::mem::transmute(NdrClientInitializeNew(::std::mem::transmute(prpcmsg), ::std::mem::transmute(pstubmsg), ::std::mem::transmute(pstubdescriptor), ::std::mem::transmute(procnum)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrComplexArrayBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrComplexArrayBufferSize(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8);
        }
        ::std::mem::transmute(NdrComplexArrayBufferSize(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrComplexArrayFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrComplexArrayFree(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8);
        }
        ::std::mem::transmute(NdrComplexArrayFree(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrComplexArrayMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrComplexArrayMarshall(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
        }
        ::std::mem::transmute(NdrComplexArrayMarshall(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrComplexArrayMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrComplexArrayMemorySize(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pformat: *mut u8) -> u32;
        }
        ::std::mem::transmute(NdrComplexArrayMemorySize(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrComplexArrayUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrComplexArrayUnmarshall(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
        }
        ::std::mem::transmute(NdrComplexArrayUnmarshall(::std::mem::transmute(pstubmsg), ::std::mem::transmute(ppmemory), ::std::mem::transmute(pformat), ::std::mem::transmute(fmustalloc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrComplexStructBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrComplexStructBufferSize(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8);
        }
        ::std::mem::transmute(NdrComplexStructBufferSize(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrComplexStructFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrComplexStructFree(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8);
        }
        ::std::mem::transmute(NdrComplexStructFree(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrComplexStructMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrComplexStructMarshall(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
        }
        ::std::mem::transmute(NdrComplexStructMarshall(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrComplexStructMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrComplexStructMemorySize(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pformat: *mut u8) -> u32;
        }
        ::std::mem::transmute(NdrComplexStructMemorySize(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrComplexStructUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrComplexStructUnmarshall(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
        }
        ::std::mem::transmute(NdrComplexStructUnmarshall(::std::mem::transmute(pstubmsg), ::std::mem::transmute(ppmemory), ::std::mem::transmute(pformat), ::std::mem::transmute(fmustalloc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantArrayBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConformantArrayBufferSize(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8);
        }
        ::std::mem::transmute(NdrConformantArrayBufferSize(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantArrayFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConformantArrayFree(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8);
        }
        ::std::mem::transmute(NdrConformantArrayFree(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantArrayMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConformantArrayMarshall(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
        }
        ::std::mem::transmute(NdrConformantArrayMarshall(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantArrayMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConformantArrayMemorySize(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pformat: *mut u8) -> u32;
        }
        ::std::mem::transmute(NdrConformantArrayMemorySize(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantArrayUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConformantArrayUnmarshall(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
        }
        ::std::mem::transmute(NdrConformantArrayUnmarshall(::std::mem::transmute(pstubmsg), ::std::mem::transmute(ppmemory), ::std::mem::transmute(pformat), ::std::mem::transmute(fmustalloc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantStringBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConformantStringBufferSize(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8);
        }
        ::std::mem::transmute(NdrConformantStringBufferSize(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantStringMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConformantStringMarshall(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
        }
        ::std::mem::transmute(NdrConformantStringMarshall(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantStringMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConformantStringMemorySize(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pformat: *mut u8) -> u32;
        }
        ::std::mem::transmute(NdrConformantStringMemorySize(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantStringUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConformantStringUnmarshall(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
        }
        ::std::mem::transmute(NdrConformantStringUnmarshall(::std::mem::transmute(pstubmsg), ::std::mem::transmute(ppmemory), ::std::mem::transmute(pformat), ::std::mem::transmute(fmustalloc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantStructBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConformantStructBufferSize(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8);
        }
        ::std::mem::transmute(NdrConformantStructBufferSize(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantStructFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConformantStructFree(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8);
        }
        ::std::mem::transmute(NdrConformantStructFree(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantStructMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConformantStructMarshall(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
        }
        ::std::mem::transmute(NdrConformantStructMarshall(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantStructMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConformantStructMemorySize(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pformat: *mut u8) -> u32;
        }
        ::std::mem::transmute(NdrConformantStructMemorySize(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantStructUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConformantStructUnmarshall(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
        }
        ::std::mem::transmute(NdrConformantStructUnmarshall(::std::mem::transmute(pstubmsg), ::std::mem::transmute(ppmemory), ::std::mem::transmute(pformat), ::std::mem::transmute(fmustalloc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantVaryingArrayBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConformantVaryingArrayBufferSize(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8);
        }
        ::std::mem::transmute(NdrConformantVaryingArrayBufferSize(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantVaryingArrayFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConformantVaryingArrayFree(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8);
        }
        ::std::mem::transmute(NdrConformantVaryingArrayFree(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantVaryingArrayMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConformantVaryingArrayMarshall(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
        }
        ::std::mem::transmute(NdrConformantVaryingArrayMarshall(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantVaryingArrayMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConformantVaryingArrayMemorySize(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pformat: *mut u8) -> u32;
        }
        ::std::mem::transmute(NdrConformantVaryingArrayMemorySize(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantVaryingArrayUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConformantVaryingArrayUnmarshall(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
        }
        ::std::mem::transmute(NdrConformantVaryingArrayUnmarshall(::std::mem::transmute(pstubmsg), ::std::mem::transmute(ppmemory), ::std::mem::transmute(pformat), ::std::mem::transmute(fmustalloc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantVaryingStructBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConformantVaryingStructBufferSize(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8);
        }
        ::std::mem::transmute(NdrConformantVaryingStructBufferSize(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantVaryingStructFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConformantVaryingStructFree(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8);
        }
        ::std::mem::transmute(NdrConformantVaryingStructFree(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantVaryingStructMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConformantVaryingStructMarshall(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
        }
        ::std::mem::transmute(NdrConformantVaryingStructMarshall(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantVaryingStructMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConformantVaryingStructMemorySize(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pformat: *mut u8) -> u32;
        }
        ::std::mem::transmute(NdrConformantVaryingStructMemorySize(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConformantVaryingStructUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConformantVaryingStructUnmarshall(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
        }
        ::std::mem::transmute(NdrConformantVaryingStructUnmarshall(::std::mem::transmute(pstubmsg), ::std::mem::transmute(ppmemory), ::std::mem::transmute(pformat), ::std::mem::transmute(fmustalloc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrContextHandleInitialize(pstubmsg: *const MIDL_STUB_MESSAGE, pformat: *const u8) -> *mut NDR_SCONTEXT_1 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrContextHandleInitialize(pstubmsg: *const ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pformat: *const u8) -> *mut NDR_SCONTEXT_1;
        }
        ::std::mem::transmute(NdrContextHandleInitialize(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrContextHandleSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrContextHandleSize(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8);
        }
        ::std::mem::transmute(NdrContextHandleSize(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConvert(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConvert(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pformat: *mut u8);
        }
        ::std::mem::transmute(NdrConvert(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrConvert2(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8, numberparams: i32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrConvert2(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pformat: *mut u8, numberparams: i32);
        }
        ::std::mem::transmute(NdrConvert2(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pformat), ::std::mem::transmute(numberparams)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrCorrelationFree(pstubmsg: *mut MIDL_STUB_MESSAGE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrCorrelationFree(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>);
        }
        ::std::mem::transmute(NdrCorrelationFree(::std::mem::transmute(pstubmsg)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrCorrelationInitialize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut ::std::ffi::c_void, cachesize: u32, flags: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrCorrelationInitialize(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut ::std::ffi::c_void, cachesize: u32, flags: u32);
        }
        ::std::mem::transmute(NdrCorrelationInitialize(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(cachesize), ::std::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrCorrelationPass(pstubmsg: *mut MIDL_STUB_MESSAGE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrCorrelationPass(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>);
        }
        ::std::mem::transmute(NdrCorrelationPass(::std::mem::transmute(pstubmsg)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrCreateServerInterfaceFromStub<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::IRpcStubBuffer>>(pstub: Param0, pserverif: *mut RPC_SERVER_INTERFACE) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrCreateServerInterfaceFromStub(pstub: ::windows::runtime::RawPtr, pserverif: *mut RPC_SERVER_INTERFACE) -> RPC_STATUS;
        }
        ::std::mem::transmute(NdrCreateServerInterfaceFromStub(pstub.into_param().abi(), ::std::mem::transmute(pserverif)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrDcomAsyncClientCall(pstubdescriptor: *mut MIDL_STUB_DESC, pformat: *mut u8) -> CLIENT_CALL_RETURN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrDcomAsyncClientCall(pstubdescriptor: *mut MIDL_STUB_DESC, pformat: *mut u8) -> CLIENT_CALL_RETURN;
        }
        ::std::mem::transmute(NdrDcomAsyncClientCall(::std::mem::transmute(pstubdescriptor), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrDcomAsyncStubCall<'a, Param0: ::windows::runtime::IntoParam<'a, super::Com::IRpcStubBuffer>, Param1: ::windows::runtime::IntoParam<'a, super::Com::IRpcChannelBuffer>>(pthis: Param0, pchannel: Param1, prpcmsg: *mut RPC_MESSAGE, pdwstubphase: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrDcomAsyncStubCall(pthis: ::windows::runtime::RawPtr, pchannel: ::windows::runtime::RawPtr, prpcmsg: *mut RPC_MESSAGE, pdwstubphase: *mut u32) -> i32;
        }
        ::std::mem::transmute(NdrDcomAsyncStubCall(pthis.into_param().abi(), pchannel.into_param().abi(), ::std::mem::transmute(prpcmsg), ::std::mem::transmute(pdwstubphase)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrEncapsulatedUnionBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrEncapsulatedUnionBufferSize(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8);
        }
        ::std::mem::transmute(NdrEncapsulatedUnionBufferSize(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrEncapsulatedUnionFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrEncapsulatedUnionFree(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8);
        }
        ::std::mem::transmute(NdrEncapsulatedUnionFree(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrEncapsulatedUnionMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrEncapsulatedUnionMarshall(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
        }
        ::std::mem::transmute(NdrEncapsulatedUnionMarshall(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrEncapsulatedUnionMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrEncapsulatedUnionMemorySize(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pformat: *mut u8) -> u32;
        }
        ::std::mem::transmute(NdrEncapsulatedUnionMemorySize(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrEncapsulatedUnionUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrEncapsulatedUnionUnmarshall(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
        }
        ::std::mem::transmute(NdrEncapsulatedUnionUnmarshall(::std::mem::transmute(pstubmsg), ::std::mem::transmute(ppmemory), ::std::mem::transmute(pformat), ::std::mem::transmute(fmustalloc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrFixedArrayBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrFixedArrayBufferSize(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8);
        }
        ::std::mem::transmute(NdrFixedArrayBufferSize(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrFixedArrayFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrFixedArrayFree(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8);
        }
        ::std::mem::transmute(NdrFixedArrayFree(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrFixedArrayMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrFixedArrayMarshall(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
        }
        ::std::mem::transmute(NdrFixedArrayMarshall(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrFixedArrayMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrFixedArrayMemorySize(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pformat: *mut u8) -> u32;
        }
        ::std::mem::transmute(NdrFixedArrayMemorySize(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrFixedArrayUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrFixedArrayUnmarshall(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
        }
        ::std::mem::transmute(NdrFixedArrayUnmarshall(::std::mem::transmute(pstubmsg), ::std::mem::transmute(ppmemory), ::std::mem::transmute(pformat), ::std::mem::transmute(fmustalloc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrFreeBuffer(pstubmsg: *mut MIDL_STUB_MESSAGE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrFreeBuffer(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>);
        }
        ::std::mem::transmute(NdrFreeBuffer(::std::mem::transmute(pstubmsg)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn NdrFullPointerXlatFree(pxlattables: *mut FULL_PTR_XLAT_TABLES) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrFullPointerXlatFree(pxlattables: *mut FULL_PTR_XLAT_TABLES);
        }
        ::std::mem::transmute(NdrFullPointerXlatFree(::std::mem::transmute(pxlattables)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn NdrFullPointerXlatInit(numberofpointers: u32, xlatside: XLAT_SIDE) -> *mut FULL_PTR_XLAT_TABLES {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrFullPointerXlatInit(numberofpointers: u32, xlatside: XLAT_SIDE) -> *mut FULL_PTR_XLAT_TABLES;
        }
        ::std::mem::transmute(NdrFullPointerXlatInit(::std::mem::transmute(numberofpointers), ::std::mem::transmute(xlatside)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrGetBuffer(pstubmsg: *mut MIDL_STUB_MESSAGE, bufferlength: u32, handle: *mut ::std::ffi::c_void) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrGetBuffer(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, bufferlength: u32, handle: *mut ::std::ffi::c_void) -> *mut u8;
        }
        ::std::mem::transmute(NdrGetBuffer(::std::mem::transmute(pstubmsg), ::std::mem::transmute(bufferlength), ::std::mem::transmute(handle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrGetDcomProtocolVersion(pstubmsg: *mut MIDL_STUB_MESSAGE, pversion: *mut RPC_VERSION) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrGetDcomProtocolVersion(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pversion: *mut RPC_VERSION) -> ::windows::runtime::HRESULT;
        }
        NdrGetDcomProtocolVersion(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pversion)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrGetUserMarshalInfo(pflags: *const u32, informationlevel: u32, pmarshalinfo: *mut NDR_USER_MARSHAL_INFO) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrGetUserMarshalInfo(pflags: *const u32, informationlevel: u32, pmarshalinfo: *mut ::std::mem::ManuallyDrop<NDR_USER_MARSHAL_INFO>) -> RPC_STATUS;
        }
        ::std::mem::transmute(NdrGetUserMarshalInfo(::std::mem::transmute(pflags), ::std::mem::transmute(informationlevel), ::std::mem::transmute(pmarshalinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrInterfacePointerBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrInterfacePointerBufferSize(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8);
        }
        ::std::mem::transmute(NdrInterfacePointerBufferSize(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrInterfacePointerFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrInterfacePointerFree(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8);
        }
        ::std::mem::transmute(NdrInterfacePointerFree(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrInterfacePointerMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrInterfacePointerMarshall(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
        }
        ::std::mem::transmute(NdrInterfacePointerMarshall(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrInterfacePointerMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrInterfacePointerMemorySize(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pformat: *mut u8) -> u32;
        }
        ::std::mem::transmute(NdrInterfacePointerMemorySize(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrInterfacePointerUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrInterfacePointerUnmarshall(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
        }
        ::std::mem::transmute(NdrInterfacePointerUnmarshall(::std::mem::transmute(pstubmsg), ::std::mem::transmute(ppmemory), ::std::mem::transmute(pformat), ::std::mem::transmute(fmustalloc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMapCommAndFaultStatus(pstubmsg: *mut MIDL_STUB_MESSAGE, pcommstatus: *mut u32, pfaultstatus: *mut u32, status: RPC_STATUS) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrMapCommAndFaultStatus(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pcommstatus: *mut u32, pfaultstatus: *mut u32, status: RPC_STATUS) -> RPC_STATUS;
        }
        ::std::mem::transmute(NdrMapCommAndFaultStatus(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pcommstatus), ::std::mem::transmute(pfaultstatus), ::std::mem::transmute(status)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesProcEncodeDecode(handle: *mut ::std::ffi::c_void, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrMesProcEncodeDecode(handle: *mut ::std::ffi::c_void, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8);
        }
        ::std::mem::transmute(NdrMesProcEncodeDecode(::std::mem::transmute(handle), ::std::mem::transmute(pstubdesc), ::std::mem::transmute(pformatstring)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesProcEncodeDecode2(handle: *mut ::std::ffi::c_void, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8) -> CLIENT_CALL_RETURN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrMesProcEncodeDecode2(handle: *mut ::std::ffi::c_void, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8) -> CLIENT_CALL_RETURN;
        }
        ::std::mem::transmute(NdrMesProcEncodeDecode2(::std::mem::transmute(handle), ::std::mem::transmute(pstubdesc), ::std::mem::transmute(pformatstring)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesProcEncodeDecode3(handle: *mut ::std::ffi::c_void, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO, nprocnum: u32, preturnvalue: *mut ::std::ffi::c_void) -> CLIENT_CALL_RETURN {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrMesProcEncodeDecode3(handle: *mut ::std::ffi::c_void, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO, nprocnum: u32, preturnvalue: *mut ::std::ffi::c_void) -> CLIENT_CALL_RETURN;
        }
        ::std::mem::transmute(NdrMesProcEncodeDecode3(::std::mem::transmute(handle), ::std::mem::transmute(pproxyinfo), ::std::mem::transmute(nprocnum), ::std::mem::transmute(preturnvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn NdrMesSimpleTypeAlignSize(param0: *mut ::std::ffi::c_void) -> usize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrMesSimpleTypeAlignSize(param0: *mut ::std::ffi::c_void) -> usize;
        }
        ::std::mem::transmute(NdrMesSimpleTypeAlignSize(::std::mem::transmute(param0)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesSimpleTypeAlignSizeAll(handle: *mut ::std::ffi::c_void, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO) -> usize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrMesSimpleTypeAlignSizeAll(handle: *mut ::std::ffi::c_void, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO) -> usize;
        }
        ::std::mem::transmute(NdrMesSimpleTypeAlignSizeAll(::std::mem::transmute(handle), ::std::mem::transmute(pproxyinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn NdrMesSimpleTypeDecode(handle: *mut ::std::ffi::c_void, pobject: *mut ::std::ffi::c_void, size: i16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrMesSimpleTypeDecode(handle: *mut ::std::ffi::c_void, pobject: *mut ::std::ffi::c_void, size: i16);
        }
        ::std::mem::transmute(NdrMesSimpleTypeDecode(::std::mem::transmute(handle), ::std::mem::transmute(pobject), ::std::mem::transmute(size)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesSimpleTypeDecodeAll(handle: *mut ::std::ffi::c_void, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO, pobject: *mut ::std::ffi::c_void, size: i16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrMesSimpleTypeDecodeAll(handle: *mut ::std::ffi::c_void, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO, pobject: *mut ::std::ffi::c_void, size: i16);
        }
        ::std::mem::transmute(NdrMesSimpleTypeDecodeAll(::std::mem::transmute(handle), ::std::mem::transmute(pproxyinfo), ::std::mem::transmute(pobject), ::std::mem::transmute(size)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesSimpleTypeEncode(handle: *mut ::std::ffi::c_void, pstubdesc: *const MIDL_STUB_DESC, pobject: *const ::std::ffi::c_void, size: i16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrMesSimpleTypeEncode(handle: *mut ::std::ffi::c_void, pstubdesc: *const MIDL_STUB_DESC, pobject: *const ::std::ffi::c_void, size: i16);
        }
        ::std::mem::transmute(NdrMesSimpleTypeEncode(::std::mem::transmute(handle), ::std::mem::transmute(pstubdesc), ::std::mem::transmute(pobject), ::std::mem::transmute(size)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesSimpleTypeEncodeAll(handle: *mut ::std::ffi::c_void, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO, pobject: *const ::std::ffi::c_void, size: i16) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrMesSimpleTypeEncodeAll(handle: *mut ::std::ffi::c_void, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO, pobject: *const ::std::ffi::c_void, size: i16);
        }
        ::std::mem::transmute(NdrMesSimpleTypeEncodeAll(::std::mem::transmute(handle), ::std::mem::transmute(pproxyinfo), ::std::mem::transmute(pobject), ::std::mem::transmute(size)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesTypeAlignSize(handle: *mut ::std::ffi::c_void, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8, pobject: *const ::std::ffi::c_void) -> usize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrMesTypeAlignSize(handle: *mut ::std::ffi::c_void, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8, pobject: *const ::std::ffi::c_void) -> usize;
        }
        ::std::mem::transmute(NdrMesTypeAlignSize(::std::mem::transmute(handle), ::std::mem::transmute(pstubdesc), ::std::mem::transmute(pformatstring), ::std::mem::transmute(pobject)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesTypeAlignSize2(handle: *mut ::std::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8, pobject: *const ::std::ffi::c_void) -> usize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrMesTypeAlignSize2(handle: *mut ::std::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8, pobject: *const ::std::ffi::c_void) -> usize;
        }
        ::std::mem::transmute(NdrMesTypeAlignSize2(::std::mem::transmute(handle), ::std::mem::transmute(ppicklinginfo), ::std::mem::transmute(pstubdesc), ::std::mem::transmute(pformatstring), ::std::mem::transmute(pobject)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesTypeAlignSize3(handle: *mut ::std::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO, arrtypeoffset: *const *const u32, ntypeindex: u32, pobject: *const ::std::ffi::c_void) -> usize {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrMesTypeAlignSize3(handle: *mut ::std::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO, arrtypeoffset: *const *const u32, ntypeindex: u32, pobject: *const ::std::ffi::c_void) -> usize;
        }
        ::std::mem::transmute(NdrMesTypeAlignSize3(::std::mem::transmute(handle), ::std::mem::transmute(ppicklinginfo), ::std::mem::transmute(pproxyinfo), ::std::mem::transmute(arrtypeoffset), ::std::mem::transmute(ntypeindex), ::std::mem::transmute(pobject)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesTypeDecode(handle: *mut ::std::ffi::c_void, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8, pobject: *mut ::std::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrMesTypeDecode(handle: *mut ::std::ffi::c_void, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8, pobject: *mut ::std::ffi::c_void);
        }
        ::std::mem::transmute(NdrMesTypeDecode(::std::mem::transmute(handle), ::std::mem::transmute(pstubdesc), ::std::mem::transmute(pformatstring), ::std::mem::transmute(pobject)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesTypeDecode2(handle: *mut ::std::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8, pobject: *mut ::std::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrMesTypeDecode2(handle: *mut ::std::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8, pobject: *mut ::std::ffi::c_void);
        }
        ::std::mem::transmute(NdrMesTypeDecode2(::std::mem::transmute(handle), ::std::mem::transmute(ppicklinginfo), ::std::mem::transmute(pstubdesc), ::std::mem::transmute(pformatstring), ::std::mem::transmute(pobject)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesTypeDecode3(handle: *mut ::std::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO, arrtypeoffset: *const *const u32, ntypeindex: u32, pobject: *mut ::std::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrMesTypeDecode3(handle: *mut ::std::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO, arrtypeoffset: *const *const u32, ntypeindex: u32, pobject: *mut ::std::ffi::c_void);
        }
        ::std::mem::transmute(NdrMesTypeDecode3(::std::mem::transmute(handle), ::std::mem::transmute(ppicklinginfo), ::std::mem::transmute(pproxyinfo), ::std::mem::transmute(arrtypeoffset), ::std::mem::transmute(ntypeindex), ::std::mem::transmute(pobject)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesTypeEncode(handle: *mut ::std::ffi::c_void, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8, pobject: *const ::std::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrMesTypeEncode(handle: *mut ::std::ffi::c_void, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8, pobject: *const ::std::ffi::c_void);
        }
        ::std::mem::transmute(NdrMesTypeEncode(::std::mem::transmute(handle), ::std::mem::transmute(pstubdesc), ::std::mem::transmute(pformatstring), ::std::mem::transmute(pobject)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesTypeEncode2(handle: *mut ::std::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8, pobject: *const ::std::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrMesTypeEncode2(handle: *mut ::std::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8, pobject: *const ::std::ffi::c_void);
        }
        ::std::mem::transmute(NdrMesTypeEncode2(::std::mem::transmute(handle), ::std::mem::transmute(ppicklinginfo), ::std::mem::transmute(pstubdesc), ::std::mem::transmute(pformatstring), ::std::mem::transmute(pobject)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesTypeEncode3(handle: *mut ::std::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO, arrtypeoffset: *const *const u32, ntypeindex: u32, pobject: *const ::std::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrMesTypeEncode3(handle: *mut ::std::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO, arrtypeoffset: *const *const u32, ntypeindex: u32, pobject: *const ::std::ffi::c_void);
        }
        ::std::mem::transmute(NdrMesTypeEncode3(::std::mem::transmute(handle), ::std::mem::transmute(ppicklinginfo), ::std::mem::transmute(pproxyinfo), ::std::mem::transmute(arrtypeoffset), ::std::mem::transmute(ntypeindex), ::std::mem::transmute(pobject)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesTypeFree2(handle: *mut ::std::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8, pobject: *mut ::std::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrMesTypeFree2(handle: *mut ::std::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pstubdesc: *const MIDL_STUB_DESC, pformatstring: *mut u8, pobject: *mut ::std::ffi::c_void);
        }
        ::std::mem::transmute(NdrMesTypeFree2(::std::mem::transmute(handle), ::std::mem::transmute(ppicklinginfo), ::std::mem::transmute(pstubdesc), ::std::mem::transmute(pformatstring), ::std::mem::transmute(pobject)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrMesTypeFree3(handle: *mut ::std::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO, arrtypeoffset: *const *const u32, ntypeindex: u32, pobject: *mut ::std::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrMesTypeFree3(handle: *mut ::std::ffi::c_void, ppicklinginfo: *const MIDL_TYPE_PICKLING_INFO, pproxyinfo: *const MIDL_STUBLESS_PROXY_INFO, arrtypeoffset: *const *const u32, ntypeindex: u32, pobject: *mut ::std::ffi::c_void);
        }
        ::std::mem::transmute(NdrMesTypeFree3(::std::mem::transmute(handle), ::std::mem::transmute(ppicklinginfo), ::std::mem::transmute(pproxyinfo), ::std::mem::transmute(arrtypeoffset), ::std::mem::transmute(ntypeindex), ::std::mem::transmute(pobject)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrNonConformantStringBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrNonConformantStringBufferSize(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8);
        }
        ::std::mem::transmute(NdrNonConformantStringBufferSize(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrNonConformantStringMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrNonConformantStringMarshall(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
        }
        ::std::mem::transmute(NdrNonConformantStringMarshall(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrNonConformantStringMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrNonConformantStringMemorySize(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pformat: *mut u8) -> u32;
        }
        ::std::mem::transmute(NdrNonConformantStringMemorySize(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrNonConformantStringUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrNonConformantStringUnmarshall(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
        }
        ::std::mem::transmute(NdrNonConformantStringUnmarshall(::std::mem::transmute(pstubmsg), ::std::mem::transmute(ppmemory), ::std::mem::transmute(pformat), ::std::mem::transmute(fmustalloc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrNonEncapsulatedUnionBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrNonEncapsulatedUnionBufferSize(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8);
        }
        ::std::mem::transmute(NdrNonEncapsulatedUnionBufferSize(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrNonEncapsulatedUnionFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrNonEncapsulatedUnionFree(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8);
        }
        ::std::mem::transmute(NdrNonEncapsulatedUnionFree(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrNonEncapsulatedUnionMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrNonEncapsulatedUnionMarshall(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
        }
        ::std::mem::transmute(NdrNonEncapsulatedUnionMarshall(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrNonEncapsulatedUnionMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrNonEncapsulatedUnionMemorySize(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pformat: *mut u8) -> u32;
        }
        ::std::mem::transmute(NdrNonEncapsulatedUnionMemorySize(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrNonEncapsulatedUnionUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrNonEncapsulatedUnionUnmarshall(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
        }
        ::std::mem::transmute(NdrNonEncapsulatedUnionUnmarshall(::std::mem::transmute(pstubmsg), ::std::mem::transmute(ppmemory), ::std::mem::transmute(pformat), ::std::mem::transmute(fmustalloc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrNsGetBuffer(pstubmsg: *mut MIDL_STUB_MESSAGE, bufferlength: u32, handle: *mut ::std::ffi::c_void) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrNsGetBuffer(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, bufferlength: u32, handle: *mut ::std::ffi::c_void) -> *mut u8;
        }
        ::std::mem::transmute(NdrNsGetBuffer(::std::mem::transmute(pstubmsg), ::std::mem::transmute(bufferlength), ::std::mem::transmute(handle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrNsSendReceive(pstubmsg: *mut MIDL_STUB_MESSAGE, pbufferend: *mut u8, pautohandle: *mut *mut ::std::ffi::c_void) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrNsSendReceive(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pbufferend: *mut u8, pautohandle: *mut *mut ::std::ffi::c_void) -> *mut u8;
        }
        ::std::mem::transmute(NdrNsSendReceive(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pbufferend), ::std::mem::transmute(pautohandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn NdrOleAllocate(size: usize) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrOleAllocate(size: usize) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(NdrOleAllocate(::std::mem::transmute(size)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn NdrOleFree(nodetofree: *const ::std::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrOleFree(nodetofree: *const ::std::ffi::c_void);
        }
        ::std::mem::transmute(NdrOleFree(::std::mem::transmute(nodetofree)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrPartialIgnoreClientBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut ::std::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrPartialIgnoreClientBufferSize(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut ::std::ffi::c_void);
        }
        ::std::mem::transmute(NdrPartialIgnoreClientBufferSize(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrPartialIgnoreClientMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut ::std::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrPartialIgnoreClientMarshall(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut ::std::ffi::c_void);
        }
        ::std::mem::transmute(NdrPartialIgnoreClientMarshall(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrPartialIgnoreServerInitialize(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut ::std::ffi::c_void, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrPartialIgnoreServerInitialize(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, ppmemory: *mut *mut ::std::ffi::c_void, pformat: *mut u8);
        }
        ::std::mem::transmute(NdrPartialIgnoreServerInitialize(::std::mem::transmute(pstubmsg), ::std::mem::transmute(ppmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrPartialIgnoreServerUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut ::std::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrPartialIgnoreServerUnmarshall(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, ppmemory: *mut *mut ::std::ffi::c_void);
        }
        ::std::mem::transmute(NdrPartialIgnoreServerUnmarshall(::std::mem::transmute(pstubmsg), ::std::mem::transmute(ppmemory)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrPointerBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrPointerBufferSize(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8);
        }
        ::std::mem::transmute(NdrPointerBufferSize(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrPointerFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrPointerFree(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8);
        }
        ::std::mem::transmute(NdrPointerFree(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrPointerMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrPointerMarshall(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
        }
        ::std::mem::transmute(NdrPointerMarshall(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrPointerMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrPointerMemorySize(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pformat: *mut u8) -> u32;
        }
        ::std::mem::transmute(NdrPointerMemorySize(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrPointerUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrPointerUnmarshall(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
        }
        ::std::mem::transmute(NdrPointerUnmarshall(::std::mem::transmute(pstubmsg), ::std::mem::transmute(ppmemory), ::std::mem::transmute(pformat), ::std::mem::transmute(fmustalloc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrRangeUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrRangeUnmarshall(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
        }
        ::std::mem::transmute(NdrRangeUnmarshall(::std::mem::transmute(pstubmsg), ::std::mem::transmute(ppmemory), ::std::mem::transmute(pformat), ::std::mem::transmute(fmustalloc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn NdrRpcSmClientAllocate(size: usize) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrRpcSmClientAllocate(size: usize) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(NdrRpcSmClientAllocate(::std::mem::transmute(size)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn NdrRpcSmClientFree(nodetofree: *const ::std::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrRpcSmClientFree(nodetofree: *const ::std::ffi::c_void);
        }
        ::std::mem::transmute(NdrRpcSmClientFree(::std::mem::transmute(nodetofree)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrRpcSmSetClientToOsf(pmessage: *mut MIDL_STUB_MESSAGE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrRpcSmSetClientToOsf(pmessage: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>);
        }
        ::std::mem::transmute(NdrRpcSmSetClientToOsf(::std::mem::transmute(pmessage)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn NdrRpcSsDefaultAllocate(size: usize) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrRpcSsDefaultAllocate(size: usize) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(NdrRpcSsDefaultAllocate(::std::mem::transmute(size)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn NdrRpcSsDefaultFree(nodetofree: *const ::std::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrRpcSsDefaultFree(nodetofree: *const ::std::ffi::c_void);
        }
        ::std::mem::transmute(NdrRpcSsDefaultFree(::std::mem::transmute(nodetofree)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrRpcSsDisableAllocate(pmessage: *mut MIDL_STUB_MESSAGE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrRpcSsDisableAllocate(pmessage: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>);
        }
        ::std::mem::transmute(NdrRpcSsDisableAllocate(::std::mem::transmute(pmessage)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrRpcSsEnableAllocate(pmessage: *mut MIDL_STUB_MESSAGE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrRpcSsEnableAllocate(pmessage: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>);
        }
        ::std::mem::transmute(NdrRpcSsEnableAllocate(::std::mem::transmute(pmessage)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrSendReceive(pstubmsg: *mut MIDL_STUB_MESSAGE, pbufferend: *mut u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrSendReceive(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pbufferend: *mut u8) -> *mut u8;
        }
        ::std::mem::transmute(NdrSendReceive(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pbufferend)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn NdrServerCall2(prpcmsg: *mut RPC_MESSAGE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrServerCall2(prpcmsg: *mut RPC_MESSAGE);
        }
        ::std::mem::transmute(NdrServerCall2(::std::mem::transmute(prpcmsg)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn NdrServerCallAll(prpcmsg: *mut RPC_MESSAGE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrServerCallAll(prpcmsg: *mut RPC_MESSAGE);
        }
        ::std::mem::transmute(NdrServerCallAll(::std::mem::transmute(prpcmsg)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn NdrServerCallNdr64(prpcmsg: *mut RPC_MESSAGE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrServerCallNdr64(prpcmsg: *mut RPC_MESSAGE);
        }
        ::std::mem::transmute(NdrServerCallNdr64(::std::mem::transmute(prpcmsg)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrServerContextMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, contexthandle: *mut NDR_SCONTEXT_1, rundownroutine: ::std::option::Option<NDR_RUNDOWN>) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrServerContextMarshall(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, contexthandle: *mut NDR_SCONTEXT_1, rundownroutine: ::windows::runtime::RawPtr);
        }
        ::std::mem::transmute(NdrServerContextMarshall(::std::mem::transmute(pstubmsg), ::std::mem::transmute(contexthandle), ::std::mem::transmute(rundownroutine)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrServerContextNewMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, contexthandle: *mut NDR_SCONTEXT_1, rundownroutine: ::std::option::Option<NDR_RUNDOWN>, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrServerContextNewMarshall(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, contexthandle: *mut NDR_SCONTEXT_1, rundownroutine: ::windows::runtime::RawPtr, pformat: *mut u8);
        }
        ::std::mem::transmute(NdrServerContextNewMarshall(::std::mem::transmute(pstubmsg), ::std::mem::transmute(contexthandle), ::std::mem::transmute(rundownroutine), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrServerContextNewUnmarshall(pstubmsg: *const MIDL_STUB_MESSAGE, pformat: *const u8) -> *mut NDR_SCONTEXT_1 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrServerContextNewUnmarshall(pstubmsg: *const ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pformat: *const u8) -> *mut NDR_SCONTEXT_1;
        }
        ::std::mem::transmute(NdrServerContextNewUnmarshall(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrServerContextUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE) -> *mut NDR_SCONTEXT_1 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrServerContextUnmarshall(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>) -> *mut NDR_SCONTEXT_1;
        }
        ::std::mem::transmute(NdrServerContextUnmarshall(::std::mem::transmute(pstubmsg)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrServerInitialize(prpcmsg: *mut RPC_MESSAGE, pstubmsg: *mut MIDL_STUB_MESSAGE, pstubdescriptor: *mut MIDL_STUB_DESC) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrServerInitialize(prpcmsg: *mut RPC_MESSAGE, pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pstubdescriptor: *mut MIDL_STUB_DESC) -> *mut u8;
        }
        ::std::mem::transmute(NdrServerInitialize(::std::mem::transmute(prpcmsg), ::std::mem::transmute(pstubmsg), ::std::mem::transmute(pstubdescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrServerInitializeMarshall(prpcmsg: *mut RPC_MESSAGE, pstubmsg: *mut MIDL_STUB_MESSAGE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrServerInitializeMarshall(prpcmsg: *mut RPC_MESSAGE, pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>);
        }
        ::std::mem::transmute(NdrServerInitializeMarshall(::std::mem::transmute(prpcmsg), ::std::mem::transmute(pstubmsg)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrServerInitializeNew(prpcmsg: *mut RPC_MESSAGE, pstubmsg: *mut MIDL_STUB_MESSAGE, pstubdescriptor: *mut MIDL_STUB_DESC) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrServerInitializeNew(prpcmsg: *mut RPC_MESSAGE, pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pstubdescriptor: *mut MIDL_STUB_DESC) -> *mut u8;
        }
        ::std::mem::transmute(NdrServerInitializeNew(::std::mem::transmute(prpcmsg), ::std::mem::transmute(pstubmsg), ::std::mem::transmute(pstubdescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrServerInitializePartial(prpcmsg: *mut RPC_MESSAGE, pstubmsg: *mut MIDL_STUB_MESSAGE, pstubdescriptor: *mut MIDL_STUB_DESC, requestedbuffersize: u32) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrServerInitializePartial(prpcmsg: *mut RPC_MESSAGE, pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pstubdescriptor: *mut MIDL_STUB_DESC, requestedbuffersize: u32);
        }
        ::std::mem::transmute(NdrServerInitializePartial(::std::mem::transmute(prpcmsg), ::std::mem::transmute(pstubmsg), ::std::mem::transmute(pstubdescriptor), ::std::mem::transmute(requestedbuffersize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrServerInitializeUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pstubdescriptor: *mut MIDL_STUB_DESC, prpcmsg: *mut RPC_MESSAGE) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrServerInitializeUnmarshall(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pstubdescriptor: *mut MIDL_STUB_DESC, prpcmsg: *mut RPC_MESSAGE) -> *mut u8;
        }
        ::std::mem::transmute(NdrServerInitializeUnmarshall(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pstubdescriptor), ::std::mem::transmute(prpcmsg)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrSimpleStructBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrSimpleStructBufferSize(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8);
        }
        ::std::mem::transmute(NdrSimpleStructBufferSize(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrSimpleStructFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrSimpleStructFree(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8);
        }
        ::std::mem::transmute(NdrSimpleStructFree(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrSimpleStructMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrSimpleStructMarshall(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
        }
        ::std::mem::transmute(NdrSimpleStructMarshall(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrSimpleStructMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrSimpleStructMemorySize(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pformat: *mut u8) -> u32;
        }
        ::std::mem::transmute(NdrSimpleStructMemorySize(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrSimpleStructUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrSimpleStructUnmarshall(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
        }
        ::std::mem::transmute(NdrSimpleStructUnmarshall(::std::mem::transmute(pstubmsg), ::std::mem::transmute(ppmemory), ::std::mem::transmute(pformat), ::std::mem::transmute(fmustalloc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrSimpleTypeMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, formatchar: u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrSimpleTypeMarshall(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, formatchar: u8);
        }
        ::std::mem::transmute(NdrSimpleTypeMarshall(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(formatchar)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrSimpleTypeUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, formatchar: u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrSimpleTypeUnmarshall(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, formatchar: u8);
        }
        ::std::mem::transmute(NdrSimpleTypeUnmarshall(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(formatchar)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn NdrStubCall2(pthis: *mut ::std::ffi::c_void, pchannel: *mut ::std::ffi::c_void, prpcmsg: *mut RPC_MESSAGE, pdwstubphase: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrStubCall2(pthis: *mut ::std::ffi::c_void, pchannel: *mut ::std::ffi::c_void, prpcmsg: *mut RPC_MESSAGE, pdwstubphase: *mut u32) -> i32;
        }
        ::std::mem::transmute(NdrStubCall2(::std::mem::transmute(pthis), ::std::mem::transmute(pchannel), ::std::mem::transmute(prpcmsg), ::std::mem::transmute(pdwstubphase)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn NdrStubCall3(pthis: *mut ::std::ffi::c_void, pchannel: *mut ::std::ffi::c_void, prpcmsg: *mut RPC_MESSAGE, pdwstubphase: *mut u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrStubCall3(pthis: *mut ::std::ffi::c_void, pchannel: *mut ::std::ffi::c_void, prpcmsg: *mut RPC_MESSAGE, pdwstubphase: *mut u32) -> i32;
        }
        ::std::mem::transmute(NdrStubCall3(::std::mem::transmute(pthis), ::std::mem::transmute(pchannel), ::std::mem::transmute(prpcmsg), ::std::mem::transmute(pdwstubphase)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrUserMarshalBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrUserMarshalBufferSize(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8);
        }
        ::std::mem::transmute(NdrUserMarshalBufferSize(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrUserMarshalFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrUserMarshalFree(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8);
        }
        ::std::mem::transmute(NdrUserMarshalFree(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrUserMarshalMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrUserMarshalMarshall(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
        }
        ::std::mem::transmute(NdrUserMarshalMarshall(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrUserMarshalMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrUserMarshalMemorySize(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pformat: *mut u8) -> u32;
        }
        ::std::mem::transmute(NdrUserMarshalMemorySize(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn NdrUserMarshalSimpleTypeConvert(pflags: *mut u32, pbuffer: *mut u8, formatchar: u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrUserMarshalSimpleTypeConvert(pflags: *mut u32, pbuffer: *mut u8, formatchar: u8) -> *mut u8;
        }
        ::std::mem::transmute(NdrUserMarshalSimpleTypeConvert(::std::mem::transmute(pflags), ::std::mem::transmute(pbuffer), ::std::mem::transmute(formatchar)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrUserMarshalUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrUserMarshalUnmarshall(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
        }
        ::std::mem::transmute(NdrUserMarshalUnmarshall(::std::mem::transmute(pstubmsg), ::std::mem::transmute(ppmemory), ::std::mem::transmute(pformat), ::std::mem::transmute(fmustalloc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrVaryingArrayBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrVaryingArrayBufferSize(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8);
        }
        ::std::mem::transmute(NdrVaryingArrayBufferSize(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrVaryingArrayFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrVaryingArrayFree(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8);
        }
        ::std::mem::transmute(NdrVaryingArrayFree(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrVaryingArrayMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrVaryingArrayMarshall(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
        }
        ::std::mem::transmute(NdrVaryingArrayMarshall(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrVaryingArrayMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrVaryingArrayMemorySize(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pformat: *mut u8) -> u32;
        }
        ::std::mem::transmute(NdrVaryingArrayMemorySize(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrVaryingArrayUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrVaryingArrayUnmarshall(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
        }
        ::std::mem::transmute(NdrVaryingArrayUnmarshall(::std::mem::transmute(pstubmsg), ::std::mem::transmute(ppmemory), ::std::mem::transmute(pformat), ::std::mem::transmute(fmustalloc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrXmitOrRepAsBufferSize(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrXmitOrRepAsBufferSize(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8);
        }
        ::std::mem::transmute(NdrXmitOrRepAsBufferSize(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrXmitOrRepAsFree(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrXmitOrRepAsFree(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8);
        }
        ::std::mem::transmute(NdrXmitOrRepAsFree(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrXmitOrRepAsMarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, pmemory: *mut u8, pformat: *mut u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrXmitOrRepAsMarshall(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pmemory: *mut u8, pformat: *mut u8) -> *mut u8;
        }
        ::std::mem::transmute(NdrXmitOrRepAsMarshall(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pmemory), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrXmitOrRepAsMemorySize(pstubmsg: *mut MIDL_STUB_MESSAGE, pformat: *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrXmitOrRepAsMemorySize(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, pformat: *mut u8) -> u32;
        }
        ::std::mem::transmute(NdrXmitOrRepAsMemorySize(::std::mem::transmute(pstubmsg), ::std::mem::transmute(pformat)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn NdrXmitOrRepAsUnmarshall(pstubmsg: *mut MIDL_STUB_MESSAGE, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdrXmitOrRepAsUnmarshall(pstubmsg: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>, ppmemory: *mut *mut u8, pformat: *mut u8, fmustalloc: u8) -> *mut u8;
        }
        ::std::mem::transmute(NdrXmitOrRepAsUnmarshall(::std::mem::transmute(pstubmsg), ::std::mem::transmute(ppmemory), ::std::mem::transmute(pformat), ::std::mem::transmute(fmustalloc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`, `Win32_System_IO`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub type PFN_RPCNOTIFICATION_ROUTINE = unsafe extern "system" fn(pasync: *mut ::std::mem::ManuallyDrop<RPC_ASYNC_STATE>, context: *mut ::std::ffi::c_void, event: RPC_ASYNC_EVENT);
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct PROXY_PHASE(pub i32);
pub const PROXY_CALCSIZE: PROXY_PHASE = PROXY_PHASE(0i32);
pub const PROXY_GETBUFFER: PROXY_PHASE = PROXY_PHASE(1i32);
pub const PROXY_MARSHAL: PROXY_PHASE = PROXY_PHASE(2i32);
pub const PROXY_SENDRECEIVE: PROXY_PHASE = PROXY_PHASE(3i32);
pub const PROXY_UNMARSHAL: PROXY_PHASE = PROXY_PHASE(4i32);
impl ::std::convert::From<i32> for PROXY_PHASE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for PROXY_PHASE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub type PRPC_RUNDOWN = unsafe extern "system" fn(associationcontext: *mut ::std::ffi::c_void);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct RDR_CALLOUT_STATE {
    pub LastError: RPC_STATUS,
    pub LastEEInfo: *mut ::std::ffi::c_void,
    pub LastCalledStage: RPC_HTTP_REDIRECTOR_STAGE,
    pub ServerName: *mut u16,
    pub ServerPort: *mut u16,
    pub RemoteUser: *mut u16,
    pub AuthType: *mut u16,
    pub ResourceTypePresent: u8,
    pub SessionIdPresent: u8,
    pub InterfacePresent: u8,
    pub ResourceType: ::windows::runtime::GUID,
    pub SessionId: ::windows::runtime::GUID,
    pub Interface: RPC_SYNTAX_IDENTIFIER,
    pub CertContext: *mut ::std::ffi::c_void,
}
impl RDR_CALLOUT_STATE {}
impl ::std::default::Default for RDR_CALLOUT_STATE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RDR_CALLOUT_STATE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RDR_CALLOUT_STATE")
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
impl ::std::cmp::PartialEq for RDR_CALLOUT_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.LastError == other.LastError
            && self.LastEEInfo == other.LastEEInfo
            && self.LastCalledStage == other.LastCalledStage
            && self.ServerName == other.ServerName
            && self.ServerPort == other.ServerPort
            && self.RemoteUser == other.RemoteUser
            && self.AuthType == other.AuthType
            && self.ResourceTypePresent == other.ResourceTypePresent
            && self.SessionIdPresent == other.SessionIdPresent
            && self.InterfacePresent == other.InterfacePresent
            && self.ResourceType == other.ResourceType
            && self.SessionId == other.SessionId
            && self.Interface == other.Interface
            && self.CertContext == other.CertContext
    }
}
impl ::std::cmp::Eq for RDR_CALLOUT_STATE {}
unsafe impl ::windows::runtime::Abi for RDR_CALLOUT_STATE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPCFLG_ACCESSIBILITY_BIT1: u32 = 1048576u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPCFLG_ACCESSIBILITY_BIT2: u32 = 2097152u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPCFLG_ACCESS_LOCAL: u32 = 4194304u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPCFLG_ASYNCHRONOUS: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPCFLG_AUTO_COMPLETE: u32 = 134217728u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPCFLG_HAS_CALLBACK: u32 = 67108864u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPCFLG_HAS_GUARANTEE: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPCFLG_HAS_MULTI_SYNTAXES: u32 = 33554432u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPCFLG_INPUT_SYNCHRONOUS: u32 = 536870912u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPCFLG_LOCAL_CALL: u32 = 268435456u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPCFLG_MESSAGE: u32 = 16777216u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPCFLG_NDR64_CONTAINS_ARM_LAYOUT: u32 = 67108864u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPCFLG_NON_NDR: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPCFLG_SENDER_WAITING_FOR_REPLY: u32 = 8388608u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPCFLG_WINRT_REMOTE_ASYNC: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub type RPCLT_PDU_FILTER_FUNC = unsafe extern "system" fn(buffer: *mut ::std::ffi::c_void, bufferlength: u32, fdatagram: i32);
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub type RPC_ADDRESS_CHANGE_FN = unsafe extern "system" fn(arg: *mut ::std::ffi::c_void);
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RPC_ADDRESS_CHANGE_TYPE(pub i32);
pub const PROTOCOL_NOT_LOADED: RPC_ADDRESS_CHANGE_TYPE = RPC_ADDRESS_CHANGE_TYPE(1i32);
pub const PROTOCOL_LOADED: RPC_ADDRESS_CHANGE_TYPE = RPC_ADDRESS_CHANGE_TYPE(2i32);
pub const PROTOCOL_ADDRESS_CHANGE: RPC_ADDRESS_CHANGE_TYPE = RPC_ADDRESS_CHANGE_TYPE(3i32);
impl ::std::convert::From<i32> for RPC_ADDRESS_CHANGE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RPC_ADDRESS_CHANGE_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RPC_ASYNC_EVENT(pub i32);
pub const RpcCallComplete: RPC_ASYNC_EVENT = RPC_ASYNC_EVENT(0i32);
pub const RpcSendComplete: RPC_ASYNC_EVENT = RPC_ASYNC_EVENT(1i32);
pub const RpcReceiveComplete: RPC_ASYNC_EVENT = RPC_ASYNC_EVENT(2i32);
pub const RpcClientDisconnect: RPC_ASYNC_EVENT = RPC_ASYNC_EVENT(3i32);
pub const RpcClientCancel: RPC_ASYNC_EVENT = RPC_ASYNC_EVENT(4i32);
impl ::std::convert::From<i32> for RPC_ASYNC_EVENT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RPC_ASYNC_EVENT {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::clone::Clone for RPC_ASYNC_NOTIFICATION_INFO {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`, `Win32_System_IO`*"]
pub union RPC_ASYNC_NOTIFICATION_INFO {
    pub APC: ::std::mem::ManuallyDrop<RPC_ASYNC_NOTIFICATION_INFO_0>,
    pub IOC: RPC_ASYNC_NOTIFICATION_INFO_1,
    pub IntPtr: RPC_ASYNC_NOTIFICATION_INFO_2,
    pub hEvent: super::super::Foundation::HANDLE,
    pub NotificationRoutine: ::windows::runtime::RawPtr,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl RPC_ASYNC_NOTIFICATION_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::default::Default for RPC_ASYNC_NOTIFICATION_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::cmp::PartialEq for RPC_ASYNC_NOTIFICATION_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::cmp::Eq for RPC_ASYNC_NOTIFICATION_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
unsafe impl ::windows::runtime::Abi for RPC_ASYNC_NOTIFICATION_INFO {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub struct RPC_ASYNC_NOTIFICATION_INFO_0 {
    pub NotificationRoutine: ::std::option::Option<PFN_RPCNOTIFICATION_ROUTINE>,
    pub hThread: super::super::Foundation::HANDLE,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl RPC_ASYNC_NOTIFICATION_INFO_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::default::Default for RPC_ASYNC_NOTIFICATION_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::fmt::Debug for RPC_ASYNC_NOTIFICATION_INFO_0 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_APC_e__Struct").field("hThread", &self.hThread).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::cmp::PartialEq for RPC_ASYNC_NOTIFICATION_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.NotificationRoutine.map(|f| f as usize) == other.NotificationRoutine.map(|f| f as usize) && self.hThread == other.hThread
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::cmp::Eq for RPC_ASYNC_NOTIFICATION_INFO_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
unsafe impl ::windows::runtime::Abi for RPC_ASYNC_NOTIFICATION_INFO_0 {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub struct RPC_ASYNC_NOTIFICATION_INFO_1 {
    pub hIOPort: super::super::Foundation::HANDLE,
    pub dwNumberOfBytesTransferred: u32,
    pub dwCompletionKey: usize,
    pub lpOverlapped: *mut super::IO::OVERLAPPED,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl RPC_ASYNC_NOTIFICATION_INFO_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::default::Default for RPC_ASYNC_NOTIFICATION_INFO_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::fmt::Debug for RPC_ASYNC_NOTIFICATION_INFO_1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_IOC_e__Struct").field("hIOPort", &self.hIOPort).field("dwNumberOfBytesTransferred", &self.dwNumberOfBytesTransferred).field("dwCompletionKey", &self.dwCompletionKey).field("lpOverlapped", &self.lpOverlapped).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::cmp::PartialEq for RPC_ASYNC_NOTIFICATION_INFO_1 {
    fn eq(&self, other: &Self) -> bool {
        self.hIOPort == other.hIOPort && self.dwNumberOfBytesTransferred == other.dwNumberOfBytesTransferred && self.dwCompletionKey == other.dwCompletionKey && self.lpOverlapped == other.lpOverlapped
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::cmp::Eq for RPC_ASYNC_NOTIFICATION_INFO_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
unsafe impl ::windows::runtime::Abi for RPC_ASYNC_NOTIFICATION_INFO_1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
pub struct RPC_ASYNC_NOTIFICATION_INFO_2 {
    pub hWnd: super::super::Foundation::HWND,
    pub Msg: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl RPC_ASYNC_NOTIFICATION_INFO_2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::default::Default for RPC_ASYNC_NOTIFICATION_INFO_2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::fmt::Debug for RPC_ASYNC_NOTIFICATION_INFO_2 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_IntPtr_e__Struct").field("hWnd", &self.hWnd).field("Msg", &self.Msg).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::cmp::PartialEq for RPC_ASYNC_NOTIFICATION_INFO_2 {
    fn eq(&self, other: &Self) -> bool {
        self.hWnd == other.hWnd && self.Msg == other.Msg
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::cmp::Eq for RPC_ASYNC_NOTIFICATION_INFO_2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
unsafe impl ::windows::runtime::Abi for RPC_ASYNC_NOTIFICATION_INFO_2 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::clone::Clone for RPC_ASYNC_STATE {
    fn clone(&self) -> Self {
        unimplemented!()
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`, `Win32_System_IO`*"]
pub struct RPC_ASYNC_STATE {
    pub Size: u32,
    pub Signature: u32,
    pub Lock: i32,
    pub Flags: u32,
    pub StubInfo: *mut ::std::ffi::c_void,
    pub UserInfo: *mut ::std::ffi::c_void,
    pub RuntimeInfo: *mut ::std::ffi::c_void,
    pub Event: RPC_ASYNC_EVENT,
    pub NotificationType: RPC_NOTIFICATION_TYPES,
    pub u: RPC_ASYNC_NOTIFICATION_INFO,
    pub Reserved: [isize; 4],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl RPC_ASYNC_STATE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::default::Default for RPC_ASYNC_STATE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::cmp::PartialEq for RPC_ASYNC_STATE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
impl ::std::cmp::Eq for RPC_ASYNC_STATE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
unsafe impl ::windows::runtime::Abi for RPC_ASYNC_STATE {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub type RPC_AUTH_KEY_RETRIEVAL_FN = unsafe extern "system" fn(arg: *const ::std::ffi::c_void, serverprincname: *const u16, keyver: u32, key: *mut *mut ::std::ffi::c_void, status: *mut RPC_STATUS);
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_BHO_EXCLUSIVE_AND_GUARANTEED: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_BHT_OBJECT_UUID_VALID: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RPC_BINDING_HANDLE_OPTIONS_FLAGS(pub u32);
pub const RPC_BHO_NONCAUSAL: RPC_BINDING_HANDLE_OPTIONS_FLAGS = RPC_BINDING_HANDLE_OPTIONS_FLAGS(1u32);
pub const RPC_BHO_DONTLINGER: RPC_BINDING_HANDLE_OPTIONS_FLAGS = RPC_BINDING_HANDLE_OPTIONS_FLAGS(2u32);
impl ::std::convert::From<u32> for RPC_BINDING_HANDLE_OPTIONS_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RPC_BINDING_HANDLE_OPTIONS_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for RPC_BINDING_HANDLE_OPTIONS_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for RPC_BINDING_HANDLE_OPTIONS_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for RPC_BINDING_HANDLE_OPTIONS_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for RPC_BINDING_HANDLE_OPTIONS_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for RPC_BINDING_HANDLE_OPTIONS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct RPC_BINDING_HANDLE_OPTIONS_V1 {
    pub Version: u32,
    pub Flags: RPC_BINDING_HANDLE_OPTIONS_FLAGS,
    pub ComTimeout: u32,
    pub CallTimeout: u32,
}
impl RPC_BINDING_HANDLE_OPTIONS_V1 {}
impl ::std::default::Default for RPC_BINDING_HANDLE_OPTIONS_V1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RPC_BINDING_HANDLE_OPTIONS_V1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RPC_BINDING_HANDLE_OPTIONS_V1").field("Version", &self.Version).field("Flags", &self.Flags).field("ComTimeout", &self.ComTimeout).field("CallTimeout", &self.CallTimeout).finish()
    }
}
impl ::std::cmp::PartialEq for RPC_BINDING_HANDLE_OPTIONS_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Flags == other.Flags && self.ComTimeout == other.ComTimeout && self.CallTimeout == other.CallTimeout
    }
}
impl ::std::cmp::Eq for RPC_BINDING_HANDLE_OPTIONS_V1 {}
unsafe impl ::windows::runtime::Abi for RPC_BINDING_HANDLE_OPTIONS_V1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
pub struct RPC_BINDING_HANDLE_SECURITY_V1_A {
    pub Version: u32,
    pub ServerPrincName: *mut u8,
    pub AuthnLevel: u32,
    pub AuthnSvc: u32,
    pub AuthIdentity: *mut SEC_WINNT_AUTH_IDENTITY_A,
    pub SecurityQos: *mut RPC_SECURITY_QOS,
}
#[cfg(feature = "Win32_System_Com")]
impl RPC_BINDING_HANDLE_SECURITY_V1_A {}
#[cfg(feature = "Win32_System_Com")]
impl ::std::default::Default for RPC_BINDING_HANDLE_SECURITY_V1_A {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::fmt::Debug for RPC_BINDING_HANDLE_SECURITY_V1_A {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RPC_BINDING_HANDLE_SECURITY_V1_A").field("Version", &self.Version).field("ServerPrincName", &self.ServerPrincName).field("AuthnLevel", &self.AuthnLevel).field("AuthnSvc", &self.AuthnSvc).field("AuthIdentity", &self.AuthIdentity).field("SecurityQos", &self.SecurityQos).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::PartialEq for RPC_BINDING_HANDLE_SECURITY_V1_A {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.ServerPrincName == other.ServerPrincName && self.AuthnLevel == other.AuthnLevel && self.AuthnSvc == other.AuthnSvc && self.AuthIdentity == other.AuthIdentity && self.SecurityQos == other.SecurityQos
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::Eq for RPC_BINDING_HANDLE_SECURITY_V1_A {}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::runtime::Abi for RPC_BINDING_HANDLE_SECURITY_V1_A {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
pub struct RPC_BINDING_HANDLE_SECURITY_V1_W {
    pub Version: u32,
    pub ServerPrincName: *mut u16,
    pub AuthnLevel: u32,
    pub AuthnSvc: u32,
    pub AuthIdentity: *mut SEC_WINNT_AUTH_IDENTITY_W,
    pub SecurityQos: *mut RPC_SECURITY_QOS,
}
#[cfg(feature = "Win32_System_Com")]
impl RPC_BINDING_HANDLE_SECURITY_V1_W {}
#[cfg(feature = "Win32_System_Com")]
impl ::std::default::Default for RPC_BINDING_HANDLE_SECURITY_V1_W {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::fmt::Debug for RPC_BINDING_HANDLE_SECURITY_V1_W {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RPC_BINDING_HANDLE_SECURITY_V1_W").field("Version", &self.Version).field("ServerPrincName", &self.ServerPrincName).field("AuthnLevel", &self.AuthnLevel).field("AuthnSvc", &self.AuthnSvc).field("AuthIdentity", &self.AuthIdentity).field("SecurityQos", &self.SecurityQos).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::PartialEq for RPC_BINDING_HANDLE_SECURITY_V1_W {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.ServerPrincName == other.ServerPrincName && self.AuthnLevel == other.AuthnLevel && self.AuthnSvc == other.AuthnSvc && self.AuthIdentity == other.AuthIdentity && self.SecurityQos == other.SecurityQos
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::Eq for RPC_BINDING_HANDLE_SECURITY_V1_W {}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::runtime::Abi for RPC_BINDING_HANDLE_SECURITY_V1_W {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct RPC_BINDING_HANDLE_TEMPLATE_V1_A {
    pub Version: u32,
    pub Flags: u32,
    pub ProtocolSequence: u32,
    pub NetworkAddress: *mut u8,
    pub StringEndpoint: *mut u8,
    pub u1: RPC_BINDING_HANDLE_TEMPLATE_V1_A_0,
    pub ObjectUuid: ::windows::runtime::GUID,
}
impl RPC_BINDING_HANDLE_TEMPLATE_V1_A {}
impl ::std::default::Default for RPC_BINDING_HANDLE_TEMPLATE_V1_A {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for RPC_BINDING_HANDLE_TEMPLATE_V1_A {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for RPC_BINDING_HANDLE_TEMPLATE_V1_A {}
unsafe impl ::windows::runtime::Abi for RPC_BINDING_HANDLE_TEMPLATE_V1_A {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub union RPC_BINDING_HANDLE_TEMPLATE_V1_A_0 {
    pub Reserved: *mut u8,
}
impl RPC_BINDING_HANDLE_TEMPLATE_V1_A_0 {}
impl ::std::default::Default for RPC_BINDING_HANDLE_TEMPLATE_V1_A_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for RPC_BINDING_HANDLE_TEMPLATE_V1_A_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for RPC_BINDING_HANDLE_TEMPLATE_V1_A_0 {}
unsafe impl ::windows::runtime::Abi for RPC_BINDING_HANDLE_TEMPLATE_V1_A_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct RPC_BINDING_HANDLE_TEMPLATE_V1_W {
    pub Version: u32,
    pub Flags: u32,
    pub ProtocolSequence: u32,
    pub NetworkAddress: *mut u16,
    pub StringEndpoint: *mut u16,
    pub u1: RPC_BINDING_HANDLE_TEMPLATE_V1_W_0,
    pub ObjectUuid: ::windows::runtime::GUID,
}
impl RPC_BINDING_HANDLE_TEMPLATE_V1_W {}
impl ::std::default::Default for RPC_BINDING_HANDLE_TEMPLATE_V1_W {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for RPC_BINDING_HANDLE_TEMPLATE_V1_W {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for RPC_BINDING_HANDLE_TEMPLATE_V1_W {}
unsafe impl ::windows::runtime::Abi for RPC_BINDING_HANDLE_TEMPLATE_V1_W {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub union RPC_BINDING_HANDLE_TEMPLATE_V1_W_0 {
    pub Reserved: *mut u16,
}
impl RPC_BINDING_HANDLE_TEMPLATE_V1_W_0 {}
impl ::std::default::Default for RPC_BINDING_HANDLE_TEMPLATE_V1_W_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::cmp::PartialEq for RPC_BINDING_HANDLE_TEMPLATE_V1_W_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
impl ::std::cmp::Eq for RPC_BINDING_HANDLE_TEMPLATE_V1_W_0 {}
unsafe impl ::windows::runtime::Abi for RPC_BINDING_HANDLE_TEMPLATE_V1_W_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct RPC_BINDING_VECTOR {
    pub Count: u32,
    pub BindingH: [*mut ::std::ffi::c_void; 1],
}
impl RPC_BINDING_VECTOR {}
impl ::std::default::Default for RPC_BINDING_VECTOR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RPC_BINDING_VECTOR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RPC_BINDING_VECTOR").field("Count", &self.Count).field("BindingH", &self.BindingH).finish()
    }
}
impl ::std::cmp::PartialEq for RPC_BINDING_VECTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.BindingH == other.BindingH
    }
}
impl ::std::cmp::Eq for RPC_BINDING_VECTOR {}
unsafe impl ::windows::runtime::Abi for RPC_BINDING_VECTOR {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub type RPC_BLOCKING_FN = unsafe extern "system" fn(hwnd: *mut ::std::ffi::c_void, context: *mut ::std::ffi::c_void, hsyncevent: *mut ::std::ffi::c_void) -> RPC_STATUS;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_BUFFER_ASYNC: u32 = 32768u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_BUFFER_COMPLETE: u32 = 4096u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_BUFFER_EXTRA: u32 = 16384u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_BUFFER_NONOTIFY: u32 = 65536u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_BUFFER_PARTIAL: u32 = 8192u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`*"]
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
impl RPC_CALL_ATTRIBUTES_V1_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RPC_CALL_ATTRIBUTES_V1_A {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RPC_CALL_ATTRIBUTES_V1_A {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RPC_CALL_ATTRIBUTES_V1_A")
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
impl ::std::cmp::PartialEq for RPC_CALL_ATTRIBUTES_V1_A {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.Flags == other.Flags
            && self.ServerPrincipalNameBufferLength == other.ServerPrincipalNameBufferLength
            && self.ServerPrincipalName == other.ServerPrincipalName
            && self.ClientPrincipalNameBufferLength == other.ClientPrincipalNameBufferLength
            && self.ClientPrincipalName == other.ClientPrincipalName
            && self.AuthenticationLevel == other.AuthenticationLevel
            && self.AuthenticationService == other.AuthenticationService
            && self.NullSession == other.NullSession
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RPC_CALL_ATTRIBUTES_V1_A {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RPC_CALL_ATTRIBUTES_V1_A {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`*"]
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
impl RPC_CALL_ATTRIBUTES_V1_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RPC_CALL_ATTRIBUTES_V1_W {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RPC_CALL_ATTRIBUTES_V1_W {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RPC_CALL_ATTRIBUTES_V1_W")
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
impl ::std::cmp::PartialEq for RPC_CALL_ATTRIBUTES_V1_W {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.Flags == other.Flags
            && self.ServerPrincipalNameBufferLength == other.ServerPrincipalNameBufferLength
            && self.ServerPrincipalName == other.ServerPrincipalName
            && self.ClientPrincipalNameBufferLength == other.ClientPrincipalNameBufferLength
            && self.ClientPrincipalName == other.ClientPrincipalName
            && self.AuthenticationLevel == other.AuthenticationLevel
            && self.AuthenticationService == other.AuthenticationService
            && self.NullSession == other.NullSession
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RPC_CALL_ATTRIBUTES_V1_W {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RPC_CALL_ATTRIBUTES_V1_W {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`*"]
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
    pub InterfaceUuid: ::windows::runtime::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl RPC_CALL_ATTRIBUTES_V2_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RPC_CALL_ATTRIBUTES_V2_A {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RPC_CALL_ATTRIBUTES_V2_A {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RPC_CALL_ATTRIBUTES_V2_A")
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
impl ::std::cmp::PartialEq for RPC_CALL_ATTRIBUTES_V2_A {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.Flags == other.Flags
            && self.ServerPrincipalNameBufferLength == other.ServerPrincipalNameBufferLength
            && self.ServerPrincipalName == other.ServerPrincipalName
            && self.ClientPrincipalNameBufferLength == other.ClientPrincipalNameBufferLength
            && self.ClientPrincipalName == other.ClientPrincipalName
            && self.AuthenticationLevel == other.AuthenticationLevel
            && self.AuthenticationService == other.AuthenticationService
            && self.NullSession == other.NullSession
            && self.KernelModeCaller == other.KernelModeCaller
            && self.ProtocolSequence == other.ProtocolSequence
            && self.IsClientLocal == other.IsClientLocal
            && self.ClientPID == other.ClientPID
            && self.CallStatus == other.CallStatus
            && self.CallType == other.CallType
            && self.CallLocalAddress == other.CallLocalAddress
            && self.OpNum == other.OpNum
            && self.InterfaceUuid == other.InterfaceUuid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RPC_CALL_ATTRIBUTES_V2_A {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RPC_CALL_ATTRIBUTES_V2_A {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`*"]
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
    pub InterfaceUuid: ::windows::runtime::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl RPC_CALL_ATTRIBUTES_V2_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RPC_CALL_ATTRIBUTES_V2_W {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RPC_CALL_ATTRIBUTES_V2_W {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RPC_CALL_ATTRIBUTES_V2_W")
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
impl ::std::cmp::PartialEq for RPC_CALL_ATTRIBUTES_V2_W {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.Flags == other.Flags
            && self.ServerPrincipalNameBufferLength == other.ServerPrincipalNameBufferLength
            && self.ServerPrincipalName == other.ServerPrincipalName
            && self.ClientPrincipalNameBufferLength == other.ClientPrincipalNameBufferLength
            && self.ClientPrincipalName == other.ClientPrincipalName
            && self.AuthenticationLevel == other.AuthenticationLevel
            && self.AuthenticationService == other.AuthenticationService
            && self.NullSession == other.NullSession
            && self.KernelModeCaller == other.KernelModeCaller
            && self.ProtocolSequence == other.ProtocolSequence
            && self.IsClientLocal == other.IsClientLocal
            && self.ClientPID == other.ClientPID
            && self.CallStatus == other.CallStatus
            && self.CallType == other.CallType
            && self.CallLocalAddress == other.CallLocalAddress
            && self.OpNum == other.OpNum
            && self.InterfaceUuid == other.InterfaceUuid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RPC_CALL_ATTRIBUTES_V2_W {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RPC_CALL_ATTRIBUTES_V2_W {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`*"]
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
    pub InterfaceUuid: ::windows::runtime::GUID,
    pub ClientIdentifierBufferLength: u32,
    pub ClientIdentifier: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl RPC_CALL_ATTRIBUTES_V3_A {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RPC_CALL_ATTRIBUTES_V3_A {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RPC_CALL_ATTRIBUTES_V3_A {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RPC_CALL_ATTRIBUTES_V3_A")
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
impl ::std::cmp::PartialEq for RPC_CALL_ATTRIBUTES_V3_A {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.Flags == other.Flags
            && self.ServerPrincipalNameBufferLength == other.ServerPrincipalNameBufferLength
            && self.ServerPrincipalName == other.ServerPrincipalName
            && self.ClientPrincipalNameBufferLength == other.ClientPrincipalNameBufferLength
            && self.ClientPrincipalName == other.ClientPrincipalName
            && self.AuthenticationLevel == other.AuthenticationLevel
            && self.AuthenticationService == other.AuthenticationService
            && self.NullSession == other.NullSession
            && self.KernelModeCaller == other.KernelModeCaller
            && self.ProtocolSequence == other.ProtocolSequence
            && self.IsClientLocal == other.IsClientLocal
            && self.ClientPID == other.ClientPID
            && self.CallStatus == other.CallStatus
            && self.CallType == other.CallType
            && self.CallLocalAddress == other.CallLocalAddress
            && self.OpNum == other.OpNum
            && self.InterfaceUuid == other.InterfaceUuid
            && self.ClientIdentifierBufferLength == other.ClientIdentifierBufferLength
            && self.ClientIdentifier == other.ClientIdentifier
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RPC_CALL_ATTRIBUTES_V3_A {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RPC_CALL_ATTRIBUTES_V3_A {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`*"]
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
    pub InterfaceUuid: ::windows::runtime::GUID,
    pub ClientIdentifierBufferLength: u32,
    pub ClientIdentifier: *mut u8,
}
#[cfg(feature = "Win32_Foundation")]
impl RPC_CALL_ATTRIBUTES_V3_W {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RPC_CALL_ATTRIBUTES_V3_W {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RPC_CALL_ATTRIBUTES_V3_W {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RPC_CALL_ATTRIBUTES_V3_W")
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
impl ::std::cmp::PartialEq for RPC_CALL_ATTRIBUTES_V3_W {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.Flags == other.Flags
            && self.ServerPrincipalNameBufferLength == other.ServerPrincipalNameBufferLength
            && self.ServerPrincipalName == other.ServerPrincipalName
            && self.ClientPrincipalNameBufferLength == other.ClientPrincipalNameBufferLength
            && self.ClientPrincipalName == other.ClientPrincipalName
            && self.AuthenticationLevel == other.AuthenticationLevel
            && self.AuthenticationService == other.AuthenticationService
            && self.NullSession == other.NullSession
            && self.KernelModeCaller == other.KernelModeCaller
            && self.ProtocolSequence == other.ProtocolSequence
            && self.IsClientLocal == other.IsClientLocal
            && self.ClientPID == other.ClientPID
            && self.CallStatus == other.CallStatus
            && self.CallType == other.CallType
            && self.CallLocalAddress == other.CallLocalAddress
            && self.OpNum == other.OpNum
            && self.InterfaceUuid == other.InterfaceUuid
            && self.ClientIdentifierBufferLength == other.ClientIdentifierBufferLength
            && self.ClientIdentifier == other.ClientIdentifier
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RPC_CALL_ATTRIBUTES_V3_W {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RPC_CALL_ATTRIBUTES_V3_W {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_CALL_ATTRIBUTES_VERSION: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct RPC_CALL_LOCAL_ADDRESS_V1 {
    pub Version: u32,
    pub Buffer: *mut ::std::ffi::c_void,
    pub BufferSize: u32,
    pub AddressFormat: RpcLocalAddressFormat,
}
impl RPC_CALL_LOCAL_ADDRESS_V1 {}
impl ::std::default::Default for RPC_CALL_LOCAL_ADDRESS_V1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RPC_CALL_LOCAL_ADDRESS_V1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RPC_CALL_LOCAL_ADDRESS_V1").field("Version", &self.Version).field("Buffer", &self.Buffer).field("BufferSize", &self.BufferSize).field("AddressFormat", &self.AddressFormat).finish()
    }
}
impl ::std::cmp::PartialEq for RPC_CALL_LOCAL_ADDRESS_V1 {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Buffer == other.Buffer && self.BufferSize == other.BufferSize && self.AddressFormat == other.AddressFormat
    }
}
impl ::std::cmp::Eq for RPC_CALL_LOCAL_ADDRESS_V1 {}
unsafe impl ::windows::runtime::Abi for RPC_CALL_LOCAL_ADDRESS_V1 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_CALL_STATUS_CANCELLED: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_CALL_STATUS_DISCONNECTED: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub type RPC_CLIENT_ALLOC = unsafe extern "system" fn(size: usize) -> *mut ::std::ffi::c_void;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub type RPC_CLIENT_FREE = unsafe extern "system" fn(ptr: *const ::std::ffi::c_void);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct RPC_CLIENT_INFORMATION1 {
    pub UserName: *mut u8,
    pub ComputerName: *mut u8,
    pub Privilege: u16,
    pub AuthFlags: u32,
}
impl RPC_CLIENT_INFORMATION1 {}
impl ::std::default::Default for RPC_CLIENT_INFORMATION1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RPC_CLIENT_INFORMATION1 {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RPC_CLIENT_INFORMATION1").field("UserName", &self.UserName).field("ComputerName", &self.ComputerName).field("Privilege", &self.Privilege).field("AuthFlags", &self.AuthFlags).finish()
    }
}
impl ::std::cmp::PartialEq for RPC_CLIENT_INFORMATION1 {
    fn eq(&self, other: &Self) -> bool {
        self.UserName == other.UserName && self.ComputerName == other.ComputerName && self.Privilege == other.Privilege && self.AuthFlags == other.AuthFlags
    }
}
impl ::std::cmp::Eq for RPC_CLIENT_INFORMATION1 {}
unsafe impl ::windows::runtime::Abi for RPC_CLIENT_INFORMATION1 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct RPC_CLIENT_INTERFACE {
    pub Length: u32,
    pub InterfaceId: RPC_SYNTAX_IDENTIFIER,
    pub TransferSyntax: RPC_SYNTAX_IDENTIFIER,
    pub DispatchTable: *mut RPC_DISPATCH_TABLE,
    pub RpcProtseqEndpointCount: u32,
    pub RpcProtseqEndpoint: *mut RPC_PROTSEQ_ENDPOINT,
    pub Reserved: usize,
    pub InterpreterInfo: *mut ::std::ffi::c_void,
    pub Flags: u32,
}
impl RPC_CLIENT_INTERFACE {}
impl ::std::default::Default for RPC_CLIENT_INTERFACE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RPC_CLIENT_INTERFACE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RPC_CLIENT_INTERFACE")
            .field("Length", &self.Length)
            .field("InterfaceId", &self.InterfaceId)
            .field("TransferSyntax", &self.TransferSyntax)
            .field("DispatchTable", &self.DispatchTable)
            .field("RpcProtseqEndpointCount", &self.RpcProtseqEndpointCount)
            .field("RpcProtseqEndpoint", &self.RpcProtseqEndpoint)
            .field("Reserved", &self.Reserved)
            .field("InterpreterInfo", &self.InterpreterInfo)
            .field("Flags", &self.Flags)
            .finish()
    }
}
impl ::std::cmp::PartialEq for RPC_CLIENT_INTERFACE {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.InterfaceId == other.InterfaceId && self.TransferSyntax == other.TransferSyntax && self.DispatchTable == other.DispatchTable && self.RpcProtseqEndpointCount == other.RpcProtseqEndpointCount && self.RpcProtseqEndpoint == other.RpcProtseqEndpoint && self.Reserved == other.Reserved && self.InterpreterInfo == other.InterpreterInfo && self.Flags == other.Flags
    }
}
impl ::std::cmp::Eq for RPC_CLIENT_INTERFACE {}
unsafe impl ::windows::runtime::Abi for RPC_CLIENT_INTERFACE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_CONTEXT_HANDLE_DEFAULT_FLAGS: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_CONTEXT_HANDLE_DONT_SERIALIZE: u32 = 536870912u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_CONTEXT_HANDLE_FLAGS: u32 = 805306368u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_CONTEXT_HANDLE_SERIALIZE: u32 = 268435456u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_AUTHN_CLOUD_AP: u32 = 36u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_AUTHN_DCE_PRIVATE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_AUTHN_DCE_PUBLIC: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_AUTHN_DEC_PUBLIC: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_AUTHN_DEFAULT: i32 = -1i32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_AUTHN_DIGEST: u32 = 21u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_AUTHN_DPA: u32 = 17u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_AUTHN_GSS_KERBEROS: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_AUTHN_GSS_NEGOTIATE: u32 = 9u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_AUTHN_GSS_SCHANNEL: u32 = 14u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RPC_C_AUTHN_INFO_TYPE(pub u32);
pub const RPC_C_AUTHN_INFO_NONE: RPC_C_AUTHN_INFO_TYPE = RPC_C_AUTHN_INFO_TYPE(0u32);
pub const RPC_C_AUTHN_INFO_TYPE_HTTP: RPC_C_AUTHN_INFO_TYPE = RPC_C_AUTHN_INFO_TYPE(1u32);
impl ::std::convert::From<u32> for RPC_C_AUTHN_INFO_TYPE {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RPC_C_AUTHN_INFO_TYPE {
    type Abi = Self;
}
impl ::std::ops::BitOr for RPC_C_AUTHN_INFO_TYPE {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for RPC_C_AUTHN_INFO_TYPE {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for RPC_C_AUTHN_INFO_TYPE {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for RPC_C_AUTHN_INFO_TYPE {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for RPC_C_AUTHN_INFO_TYPE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_AUTHN_KERNEL: u32 = 20u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_AUTHN_LIVEXP_SSP: u32 = 35u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_AUTHN_LIVE_SSP: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_AUTHN_MQ: u32 = 100u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_AUTHN_MSN: u32 = 18u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_AUTHN_MSONLINE: u32 = 82u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_AUTHN_NEGO_EXTENDER: u32 = 30u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_AUTHN_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_AUTHN_PKU2U: u32 = 31u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_AUTHN_WINNT: u32 = 10u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_AUTHZ_DCE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_AUTHZ_DEFAULT: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_AUTHZ_NAME: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_AUTHZ_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_BINDING_DEFAULT_TIMEOUT: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_BINDING_INFINITE_TIMEOUT: u32 = 10u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_BINDING_MAX_TIMEOUT: u32 = 9u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_BINDING_MIN_TIMEOUT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_BIND_TO_ALL_NICS: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_CANCEL_INFINITE_TIMEOUT: i32 = -1i32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_DONT_FAIL: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_EP_ALL_ELTS: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_EP_MATCH_BY_BOTH: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_EP_MATCH_BY_IF: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_EP_MATCH_BY_OBJ: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_FULL_CERT_CHAIN: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_HTTP_AUTHN_SCHEME_BASIC: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_HTTP_AUTHN_SCHEME_CERT: u32 = 65536u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_HTTP_AUTHN_SCHEME_DIGEST: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_HTTP_AUTHN_SCHEME_NEGOTIATE: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_HTTP_AUTHN_SCHEME_NTLM: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_HTTP_AUTHN_SCHEME_PASSPORT: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RPC_C_HTTP_AUTHN_TARGET(pub u32);
pub const RPC_C_HTTP_AUTHN_TARGET_SERVER: RPC_C_HTTP_AUTHN_TARGET = RPC_C_HTTP_AUTHN_TARGET(1u32);
pub const RPC_C_HTTP_AUTHN_TARGET_PROXY: RPC_C_HTTP_AUTHN_TARGET = RPC_C_HTTP_AUTHN_TARGET(2u32);
impl ::std::convert::From<u32> for RPC_C_HTTP_AUTHN_TARGET {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RPC_C_HTTP_AUTHN_TARGET {
    type Abi = Self;
}
impl ::std::ops::BitOr for RPC_C_HTTP_AUTHN_TARGET {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for RPC_C_HTTP_AUTHN_TARGET {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for RPC_C_HTTP_AUTHN_TARGET {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for RPC_C_HTTP_AUTHN_TARGET {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for RPC_C_HTTP_AUTHN_TARGET {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RPC_C_HTTP_FLAGS(pub u32);
pub const RPC_C_HTTP_FLAG_USE_SSL: RPC_C_HTTP_FLAGS = RPC_C_HTTP_FLAGS(1u32);
pub const RPC_C_HTTP_FLAG_USE_FIRST_AUTH_SCHEME: RPC_C_HTTP_FLAGS = RPC_C_HTTP_FLAGS(2u32);
pub const RPC_C_HTTP_FLAG_IGNORE_CERT_CN_INVALID: RPC_C_HTTP_FLAGS = RPC_C_HTTP_FLAGS(8u32);
pub const RPC_C_HTTP_FLAG_ENABLE_CERT_REVOCATION_CHECK: RPC_C_HTTP_FLAGS = RPC_C_HTTP_FLAGS(16u32);
impl ::std::convert::From<u32> for RPC_C_HTTP_FLAGS {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RPC_C_HTTP_FLAGS {
    type Abi = Self;
}
impl ::std::ops::BitOr for RPC_C_HTTP_FLAGS {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for RPC_C_HTTP_FLAGS {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for RPC_C_HTTP_FLAGS {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for RPC_C_HTTP_FLAGS {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for RPC_C_HTTP_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_LISTEN_MAX_CALLS_DEFAULT: u32 = 1234u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_MGMT_INQ_IF_IDS: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_MGMT_INQ_PRINC_NAME: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_MGMT_INQ_STATS: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_MGMT_IS_SERVER_LISTEN: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_MGMT_STOP_SERVER_LISTEN: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_MQ_AUTHN_LEVEL_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_MQ_AUTHN_LEVEL_PKT_INTEGRITY: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_MQ_AUTHN_LEVEL_PKT_PRIVACY: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_MQ_CLEAR_ON_OPEN: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_MQ_EXPRESS: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_MQ_JOURNAL_ALWAYS: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_MQ_JOURNAL_DEADLETTER: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_MQ_JOURNAL_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_MQ_PERMANENT: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_MQ_RECOVERABLE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_MQ_TEMPORARY: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_MQ_USE_EXISTING_SECURITY: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_NOTIFY_ON_SEND_COMPLETE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_NS_DEFAULT_EXP_AGE: i32 = -1i32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_OPT_ASYNC_BLOCK: u32 = 15u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_OPT_BINDING_NONCAUSAL: u32 = 9u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_OPT_CALL_TIMEOUT: u32 = 12u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_OPT_COOKIE_AUTH: u32 = 7u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`*"]
pub struct RPC_C_OPT_COOKIE_AUTH_DESCRIPTOR {
    pub BufferSize: u32,
    pub Buffer: super::super::Foundation::PSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl RPC_C_OPT_COOKIE_AUTH_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RPC_C_OPT_COOKIE_AUTH_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for RPC_C_OPT_COOKIE_AUTH_DESCRIPTOR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RPC_C_OPT_COOKIE_AUTH_DESCRIPTOR").field("BufferSize", &self.BufferSize).field("Buffer", &self.Buffer).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RPC_C_OPT_COOKIE_AUTH_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.BufferSize == other.BufferSize && self.Buffer == other.Buffer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RPC_C_OPT_COOKIE_AUTH_DESCRIPTOR {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RPC_C_OPT_COOKIE_AUTH_DESCRIPTOR {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_OPT_DONT_LINGER: u32 = 13u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_OPT_MAX_OPTIONS: u32 = 12u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_OPT_MQ_ACKNOWLEDGE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_OPT_MQ_AUTHN_LEVEL: u32 = 6u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_OPT_MQ_AUTHN_SERVICE: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_OPT_MQ_DELIVERY: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_OPT_MQ_JOURNAL: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_OPT_MQ_PRIORITY: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_OPT_MQ_TIME_TO_BE_RECEIVED: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_OPT_MQ_TIME_TO_REACH_QUEUE: u32 = 7u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_OPT_OPTIMIZE_TIME: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_OPT_PRIVATE_BREAK_ON_SUSPEND: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_OPT_PRIVATE_DO_NOT_DISTURB: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_OPT_PRIVATE_SUPPRESS_WAKE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_OPT_RESOURCE_TYPE_UUID: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_OPT_SECURITY_CALLBACK: u32 = 10u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_OPT_SESSION_ID: u32 = 6u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_OPT_TRANS_SEND_BUFFER_SIZE: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_OPT_TRUST_PEER: u32 = 14u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_OPT_UNIQUE_BINDING: u32 = 11u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_PARM_BUFFER_LENGTH: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_PARM_MAX_PACKET_LENGTH: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_PROFILE_ALL_ELT: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_PROFILE_ALL_ELTS: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_PROFILE_DEFAULT_ELT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_PROFILE_MATCH_BY_BOTH: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_PROFILE_MATCH_BY_IF: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_PROFILE_MATCH_BY_MBR: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_PROTSEQ_MAX_REQS_DEFAULT: u32 = 10u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RPC_C_QOS_CAPABILITIES(pub u32);
pub const RPC_C_QOS_CAPABILITIES_DEFAULT: RPC_C_QOS_CAPABILITIES = RPC_C_QOS_CAPABILITIES(0u32);
pub const RPC_C_QOS_CAPABILITIES_MUTUAL_AUTH: RPC_C_QOS_CAPABILITIES = RPC_C_QOS_CAPABILITIES(1u32);
pub const RPC_C_QOS_CAPABILITIES_MAKE_FULLSIC: RPC_C_QOS_CAPABILITIES = RPC_C_QOS_CAPABILITIES(2u32);
pub const RPC_C_QOS_CAPABILITIES_ANY_AUTHORITY: RPC_C_QOS_CAPABILITIES = RPC_C_QOS_CAPABILITIES(4u32);
pub const RPC_C_QOS_CAPABILITIES_IGNORE_DELEGATE_FAILURE: RPC_C_QOS_CAPABILITIES = RPC_C_QOS_CAPABILITIES(8u32);
pub const RPC_C_QOS_CAPABILITIES_LOCAL_MA_HINT: RPC_C_QOS_CAPABILITIES = RPC_C_QOS_CAPABILITIES(16u32);
pub const RPC_C_QOS_CAPABILITIES_SCHANNEL_FULL_AUTH_IDENTITY: RPC_C_QOS_CAPABILITIES = RPC_C_QOS_CAPABILITIES(32u32);
impl ::std::convert::From<u32> for RPC_C_QOS_CAPABILITIES {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RPC_C_QOS_CAPABILITIES {
    type Abi = Self;
}
impl ::std::ops::BitOr for RPC_C_QOS_CAPABILITIES {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for RPC_C_QOS_CAPABILITIES {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for RPC_C_QOS_CAPABILITIES {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for RPC_C_QOS_CAPABILITIES {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for RPC_C_QOS_CAPABILITIES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RPC_C_QOS_IDENTITY(pub u32);
pub const RPC_C_QOS_IDENTITY_STATIC: RPC_C_QOS_IDENTITY = RPC_C_QOS_IDENTITY(0u32);
pub const RPC_C_QOS_IDENTITY_DYNAMIC: RPC_C_QOS_IDENTITY = RPC_C_QOS_IDENTITY(1u32);
impl ::std::convert::From<u32> for RPC_C_QOS_IDENTITY {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RPC_C_QOS_IDENTITY {
    type Abi = Self;
}
impl ::std::ops::BitOr for RPC_C_QOS_IDENTITY {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for RPC_C_QOS_IDENTITY {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for RPC_C_QOS_IDENTITY {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for RPC_C_QOS_IDENTITY {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for RPC_C_QOS_IDENTITY {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_RPCHTTP_USE_LOAD_BALANCE: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_SECURITY_QOS_VERSION: i32 = 1i32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_SECURITY_QOS_VERSION_1: i32 = 1i32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_SECURITY_QOS_VERSION_2: i32 = 2i32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_SECURITY_QOS_VERSION_3: i32 = 3i32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_SECURITY_QOS_VERSION_4: i32 = 4i32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_SECURITY_QOS_VERSION_5: i32 = 5i32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_STATS_CALLS_IN: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_STATS_CALLS_OUT: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_STATS_PKTS_IN: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_STATS_PKTS_OUT: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_TRY_ENFORCE_MAX_CALLS: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_USE_INTERNET_PORT: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_USE_INTRANET_PORT: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_VERS_ALL: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_VERS_COMPATIBLE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_VERS_EXACT: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_VERS_MAJOR_ONLY: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_C_VERS_UPTO: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub type RPC_DISPATCH_FUNCTION = unsafe extern "system" fn(message: *mut RPC_MESSAGE);
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct RPC_DISPATCH_TABLE {
    pub DispatchTableCount: u32,
    pub DispatchTable: ::std::option::Option<RPC_DISPATCH_FUNCTION>,
    pub Reserved: isize,
}
impl RPC_DISPATCH_TABLE {}
impl ::std::default::Default for RPC_DISPATCH_TABLE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RPC_DISPATCH_TABLE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RPC_DISPATCH_TABLE").field("DispatchTableCount", &self.DispatchTableCount).field("Reserved", &self.Reserved).finish()
    }
}
impl ::std::cmp::PartialEq for RPC_DISPATCH_TABLE {
    fn eq(&self, other: &Self) -> bool {
        self.DispatchTableCount == other.DispatchTableCount && self.DispatchTable.map(|f| f as usize) == other.DispatchTable.map(|f| f as usize) && self.Reserved == other.Reserved
    }
}
impl ::std::cmp::Eq for RPC_DISPATCH_TABLE {}
unsafe impl ::windows::runtime::Abi for RPC_DISPATCH_TABLE {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_EEINFO_VERSION: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`*"]
pub struct RPC_EE_INFO_PARAM {
    pub ParameterType: ExtendedErrorParamTypes,
    pub u: RPC_EE_INFO_PARAM_0,
}
#[cfg(feature = "Win32_Foundation")]
impl RPC_EE_INFO_PARAM {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RPC_EE_INFO_PARAM {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RPC_EE_INFO_PARAM {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RPC_EE_INFO_PARAM {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RPC_EE_INFO_PARAM {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union RPC_EE_INFO_PARAM_0 {
    pub AnsiString: super::super::Foundation::PSTR,
    pub UnicodeString: super::super::Foundation::PWSTR,
    pub LVal: i32,
    pub SVal: i16,
    pub PVal: u64,
    pub BVal: BinaryParam,
}
#[cfg(feature = "Win32_Foundation")]
impl RPC_EE_INFO_PARAM_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RPC_EE_INFO_PARAM_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RPC_EE_INFO_PARAM_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RPC_EE_INFO_PARAM_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RPC_EE_INFO_PARAM_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct RPC_ENDPOINT_TEMPLATEA {
    pub Version: u32,
    pub ProtSeq: *mut u8,
    pub Endpoint: *mut u8,
    pub SecurityDescriptor: *mut ::std::ffi::c_void,
    pub Backlog: u32,
}
impl RPC_ENDPOINT_TEMPLATEA {}
impl ::std::default::Default for RPC_ENDPOINT_TEMPLATEA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RPC_ENDPOINT_TEMPLATEA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RPC_ENDPOINT_TEMPLATEA").field("Version", &self.Version).field("ProtSeq", &self.ProtSeq).field("Endpoint", &self.Endpoint).field("SecurityDescriptor", &self.SecurityDescriptor).field("Backlog", &self.Backlog).finish()
    }
}
impl ::std::cmp::PartialEq for RPC_ENDPOINT_TEMPLATEA {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.ProtSeq == other.ProtSeq && self.Endpoint == other.Endpoint && self.SecurityDescriptor == other.SecurityDescriptor && self.Backlog == other.Backlog
    }
}
impl ::std::cmp::Eq for RPC_ENDPOINT_TEMPLATEA {}
unsafe impl ::windows::runtime::Abi for RPC_ENDPOINT_TEMPLATEA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct RPC_ENDPOINT_TEMPLATEW {
    pub Version: u32,
    pub ProtSeq: *mut u16,
    pub Endpoint: *mut u16,
    pub SecurityDescriptor: *mut ::std::ffi::c_void,
    pub Backlog: u32,
}
impl RPC_ENDPOINT_TEMPLATEW {}
impl ::std::default::Default for RPC_ENDPOINT_TEMPLATEW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RPC_ENDPOINT_TEMPLATEW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RPC_ENDPOINT_TEMPLATEW").field("Version", &self.Version).field("ProtSeq", &self.ProtSeq).field("Endpoint", &self.Endpoint).field("SecurityDescriptor", &self.SecurityDescriptor).field("Backlog", &self.Backlog).finish()
    }
}
impl ::std::cmp::PartialEq for RPC_ENDPOINT_TEMPLATEW {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.ProtSeq == other.ProtSeq && self.Endpoint == other.Endpoint && self.SecurityDescriptor == other.SecurityDescriptor && self.Backlog == other.Backlog
    }
}
impl ::std::cmp::Eq for RPC_ENDPOINT_TEMPLATEW {}
unsafe impl ::windows::runtime::Abi for RPC_ENDPOINT_TEMPLATEW {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct RPC_ERROR_ENUM_HANDLE {
    pub Signature: u32,
    pub CurrentPos: *mut ::std::ffi::c_void,
    pub Head: *mut ::std::ffi::c_void,
}
impl RPC_ERROR_ENUM_HANDLE {}
impl ::std::default::Default for RPC_ERROR_ENUM_HANDLE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RPC_ERROR_ENUM_HANDLE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RPC_ERROR_ENUM_HANDLE").field("Signature", &self.Signature).field("CurrentPos", &self.CurrentPos).field("Head", &self.Head).finish()
    }
}
impl ::std::cmp::PartialEq for RPC_ERROR_ENUM_HANDLE {
    fn eq(&self, other: &Self) -> bool {
        self.Signature == other.Signature && self.CurrentPos == other.CurrentPos && self.Head == other.Head
    }
}
impl ::std::cmp::Eq for RPC_ERROR_ENUM_HANDLE {}
unsafe impl ::windows::runtime::Abi for RPC_ERROR_ENUM_HANDLE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`*"]
pub struct RPC_EXTENDED_ERROR_INFO {
    pub Version: u32,
    pub ComputerName: super::super::Foundation::PWSTR,
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
impl RPC_EXTENDED_ERROR_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RPC_EXTENDED_ERROR_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RPC_EXTENDED_ERROR_INFO {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RPC_EXTENDED_ERROR_INFO {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RPC_EXTENDED_ERROR_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union RPC_EXTENDED_ERROR_INFO_0 {
    pub SystemTime: super::super::Foundation::SYSTEMTIME,
    pub FileTime: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl RPC_EXTENDED_ERROR_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for RPC_EXTENDED_ERROR_INFO_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for RPC_EXTENDED_ERROR_INFO_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for RPC_EXTENDED_ERROR_INFO_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for RPC_EXTENDED_ERROR_INFO_0 {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_FLAGS_VALID_BIT: u32 = 32768u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub type RPC_FORWARD_FUNCTION = unsafe extern "system" fn(interfaceid: *mut ::windows::runtime::GUID, interfaceversion: *mut RPC_VERSION, objectid: *mut ::windows::runtime::GUID, rpcpro: *mut u8, ppdestendpoint: *mut *mut ::std::ffi::c_void) -> RPC_STATUS;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_FW_IF_FLAG_DCOM: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub type RPC_HTTP_PROXY_FREE_STRING = unsafe extern "system" fn(string: *const u16);
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RPC_HTTP_REDIRECTOR_STAGE(pub i32);
pub const RPCHTTP_RS_REDIRECT: RPC_HTTP_REDIRECTOR_STAGE = RPC_HTTP_REDIRECTOR_STAGE(1i32);
pub const RPCHTTP_RS_ACCESS_1: RPC_HTTP_REDIRECTOR_STAGE = RPC_HTTP_REDIRECTOR_STAGE(2i32);
pub const RPCHTTP_RS_SESSION: RPC_HTTP_REDIRECTOR_STAGE = RPC_HTTP_REDIRECTOR_STAGE(3i32);
pub const RPCHTTP_RS_ACCESS_2: RPC_HTTP_REDIRECTOR_STAGE = RPC_HTTP_REDIRECTOR_STAGE(4i32);
pub const RPCHTTP_RS_INTERFACE: RPC_HTTP_REDIRECTOR_STAGE = RPC_HTTP_REDIRECTOR_STAGE(5i32);
impl ::std::convert::From<i32> for RPC_HTTP_REDIRECTOR_STAGE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RPC_HTTP_REDIRECTOR_STAGE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_A {
    pub TransportCredentials: *mut SEC_WINNT_AUTH_IDENTITY_A,
    pub Flags: RPC_C_HTTP_FLAGS,
    pub AuthenticationTarget: RPC_C_HTTP_AUTHN_TARGET,
    pub NumberOfAuthnSchemes: u32,
    pub AuthnSchemes: *mut u32,
    pub ServerCertificateSubject: *mut u8,
}
impl RPC_HTTP_TRANSPORT_CREDENTIALS_A {}
impl ::std::default::Default for RPC_HTTP_TRANSPORT_CREDENTIALS_A {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RPC_HTTP_TRANSPORT_CREDENTIALS_A {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RPC_HTTP_TRANSPORT_CREDENTIALS_A")
            .field("TransportCredentials", &self.TransportCredentials)
            .field("Flags", &self.Flags)
            .field("AuthenticationTarget", &self.AuthenticationTarget)
            .field("NumberOfAuthnSchemes", &self.NumberOfAuthnSchemes)
            .field("AuthnSchemes", &self.AuthnSchemes)
            .field("ServerCertificateSubject", &self.ServerCertificateSubject)
            .finish()
    }
}
impl ::std::cmp::PartialEq for RPC_HTTP_TRANSPORT_CREDENTIALS_A {
    fn eq(&self, other: &Self) -> bool {
        self.TransportCredentials == other.TransportCredentials && self.Flags == other.Flags && self.AuthenticationTarget == other.AuthenticationTarget && self.NumberOfAuthnSchemes == other.NumberOfAuthnSchemes && self.AuthnSchemes == other.AuthnSchemes && self.ServerCertificateSubject == other.ServerCertificateSubject
    }
}
impl ::std::cmp::Eq for RPC_HTTP_TRANSPORT_CREDENTIALS_A {}
unsafe impl ::windows::runtime::Abi for RPC_HTTP_TRANSPORT_CREDENTIALS_A {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
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
impl RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A {}
impl ::std::default::Default for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A")
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
impl ::std::cmp::PartialEq for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A {
    fn eq(&self, other: &Self) -> bool {
        self.TransportCredentials == other.TransportCredentials && self.Flags == other.Flags && self.AuthenticationTarget == other.AuthenticationTarget && self.NumberOfAuthnSchemes == other.NumberOfAuthnSchemes && self.AuthnSchemes == other.AuthnSchemes && self.ServerCertificateSubject == other.ServerCertificateSubject && self.ProxyCredentials == other.ProxyCredentials && self.NumberOfProxyAuthnSchemes == other.NumberOfProxyAuthnSchemes && self.ProxyAuthnSchemes == other.ProxyAuthnSchemes
    }
}
impl ::std::cmp::Eq for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A {}
unsafe impl ::windows::runtime::Abi for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
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
impl RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W {}
impl ::std::default::Default for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W")
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
impl ::std::cmp::PartialEq for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W {
    fn eq(&self, other: &Self) -> bool {
        self.TransportCredentials == other.TransportCredentials && self.Flags == other.Flags && self.AuthenticationTarget == other.AuthenticationTarget && self.NumberOfAuthnSchemes == other.NumberOfAuthnSchemes && self.AuthnSchemes == other.AuthnSchemes && self.ServerCertificateSubject == other.ServerCertificateSubject && self.ProxyCredentials == other.ProxyCredentials && self.NumberOfProxyAuthnSchemes == other.NumberOfProxyAuthnSchemes && self.ProxyAuthnSchemes == other.ProxyAuthnSchemes
    }
}
impl ::std::cmp::Eq for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W {}
unsafe impl ::windows::runtime::Abi for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A {
    pub TransportCredentials: *mut ::std::ffi::c_void,
    pub Flags: RPC_C_HTTP_FLAGS,
    pub AuthenticationTarget: RPC_C_HTTP_AUTHN_TARGET,
    pub NumberOfAuthnSchemes: u32,
    pub AuthnSchemes: *mut u32,
    pub ServerCertificateSubject: *mut u8,
    pub ProxyCredentials: *mut ::std::ffi::c_void,
    pub NumberOfProxyAuthnSchemes: u32,
    pub ProxyAuthnSchemes: *mut u32,
}
impl RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A {}
impl ::std::default::Default for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A")
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
impl ::std::cmp::PartialEq for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A {
    fn eq(&self, other: &Self) -> bool {
        self.TransportCredentials == other.TransportCredentials && self.Flags == other.Flags && self.AuthenticationTarget == other.AuthenticationTarget && self.NumberOfAuthnSchemes == other.NumberOfAuthnSchemes && self.AuthnSchemes == other.AuthnSchemes && self.ServerCertificateSubject == other.ServerCertificateSubject && self.ProxyCredentials == other.ProxyCredentials && self.NumberOfProxyAuthnSchemes == other.NumberOfProxyAuthnSchemes && self.ProxyAuthnSchemes == other.ProxyAuthnSchemes
    }
}
impl ::std::cmp::Eq for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A {}
unsafe impl ::windows::runtime::Abi for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W {
    pub TransportCredentials: *mut ::std::ffi::c_void,
    pub Flags: RPC_C_HTTP_FLAGS,
    pub AuthenticationTarget: RPC_C_HTTP_AUTHN_TARGET,
    pub NumberOfAuthnSchemes: u32,
    pub AuthnSchemes: *mut u32,
    pub ServerCertificateSubject: *mut u16,
    pub ProxyCredentials: *mut ::std::ffi::c_void,
    pub NumberOfProxyAuthnSchemes: u32,
    pub ProxyAuthnSchemes: *mut u32,
}
impl RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W {}
impl ::std::default::Default for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W")
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
impl ::std::cmp::PartialEq for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W {
    fn eq(&self, other: &Self) -> bool {
        self.TransportCredentials == other.TransportCredentials && self.Flags == other.Flags && self.AuthenticationTarget == other.AuthenticationTarget && self.NumberOfAuthnSchemes == other.NumberOfAuthnSchemes && self.AuthnSchemes == other.AuthnSchemes && self.ServerCertificateSubject == other.ServerCertificateSubject && self.ProxyCredentials == other.ProxyCredentials && self.NumberOfProxyAuthnSchemes == other.NumberOfProxyAuthnSchemes && self.ProxyAuthnSchemes == other.ProxyAuthnSchemes
    }
}
impl ::std::cmp::Eq for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W {}
unsafe impl ::windows::runtime::Abi for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_W {
    pub TransportCredentials: *mut SEC_WINNT_AUTH_IDENTITY_W,
    pub Flags: RPC_C_HTTP_FLAGS,
    pub AuthenticationTarget: RPC_C_HTTP_AUTHN_TARGET,
    pub NumberOfAuthnSchemes: u32,
    pub AuthnSchemes: *mut u32,
    pub ServerCertificateSubject: *mut u16,
}
impl RPC_HTTP_TRANSPORT_CREDENTIALS_W {}
impl ::std::default::Default for RPC_HTTP_TRANSPORT_CREDENTIALS_W {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RPC_HTTP_TRANSPORT_CREDENTIALS_W {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RPC_HTTP_TRANSPORT_CREDENTIALS_W")
            .field("TransportCredentials", &self.TransportCredentials)
            .field("Flags", &self.Flags)
            .field("AuthenticationTarget", &self.AuthenticationTarget)
            .field("NumberOfAuthnSchemes", &self.NumberOfAuthnSchemes)
            .field("AuthnSchemes", &self.AuthnSchemes)
            .field("ServerCertificateSubject", &self.ServerCertificateSubject)
            .finish()
    }
}
impl ::std::cmp::PartialEq for RPC_HTTP_TRANSPORT_CREDENTIALS_W {
    fn eq(&self, other: &Self) -> bool {
        self.TransportCredentials == other.TransportCredentials && self.Flags == other.Flags && self.AuthenticationTarget == other.AuthenticationTarget && self.NumberOfAuthnSchemes == other.NumberOfAuthnSchemes && self.AuthnSchemes == other.AuthnSchemes && self.ServerCertificateSubject == other.ServerCertificateSubject
    }
}
impl ::std::cmp::Eq for RPC_HTTP_TRANSPORT_CREDENTIALS_W {}
unsafe impl ::windows::runtime::Abi for RPC_HTTP_TRANSPORT_CREDENTIALS_W {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_IF_ALLOW_CALLBACKS_WITH_NO_AUTH: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_IF_ALLOW_LOCAL_ONLY: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_IF_ALLOW_SECURE_ONLY: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_IF_ALLOW_UNKNOWN_AUTHORITY: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_IF_ASYNC_CALLBACK: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_IF_AUTOLISTEN: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub type RPC_IF_CALLBACK_FN = unsafe extern "system" fn(interfaceuuid: *const ::std::ffi::c_void, context: *const ::std::ffi::c_void) -> RPC_STATUS;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct RPC_IF_ID {
    pub Uuid: ::windows::runtime::GUID,
    pub VersMajor: u16,
    pub VersMinor: u16,
}
impl RPC_IF_ID {}
impl ::std::default::Default for RPC_IF_ID {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RPC_IF_ID {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RPC_IF_ID").field("Uuid", &self.Uuid).field("VersMajor", &self.VersMajor).field("VersMinor", &self.VersMinor).finish()
    }
}
impl ::std::cmp::PartialEq for RPC_IF_ID {
    fn eq(&self, other: &Self) -> bool {
        self.Uuid == other.Uuid && self.VersMajor == other.VersMajor && self.VersMinor == other.VersMinor
    }
}
impl ::std::cmp::Eq for RPC_IF_ID {}
unsafe impl ::windows::runtime::Abi for RPC_IF_ID {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct RPC_IF_ID_VECTOR {
    pub Count: u32,
    pub IfId: [*mut RPC_IF_ID; 1],
}
impl RPC_IF_ID_VECTOR {}
impl ::std::default::Default for RPC_IF_ID_VECTOR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RPC_IF_ID_VECTOR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RPC_IF_ID_VECTOR").field("Count", &self.Count).field("IfId", &self.IfId).finish()
    }
}
impl ::std::cmp::PartialEq for RPC_IF_ID_VECTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.IfId == other.IfId
    }
}
impl ::std::cmp::Eq for RPC_IF_ID_VECTOR {}
unsafe impl ::windows::runtime::Abi for RPC_IF_ID_VECTOR {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_IF_OLE: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_IF_SEC_CACHE_PER_PROC: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_IF_SEC_NO_CACHE: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub type RPC_INTERFACE_GROUP_IDLE_CALLBACK_FN = unsafe extern "system" fn(ifgroup: *const ::std::ffi::c_void, idlecallbackcontext: *const ::std::ffi::c_void, isgroupidle: u32);
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_INTERFACE_HAS_PIPES: u32 = 1u32;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct RPC_INTERFACE_TEMPLATEA {
    pub Version: u32,
    pub IfSpec: *mut ::std::ffi::c_void,
    pub MgrTypeUuid: *mut ::windows::runtime::GUID,
    pub MgrEpv: *mut ::std::ffi::c_void,
    pub Flags: u32,
    pub MaxCalls: u32,
    pub MaxRpcSize: u32,
    pub IfCallback: ::std::option::Option<RPC_IF_CALLBACK_FN>,
    pub UuidVector: *mut UUID_VECTOR,
    pub Annotation: *mut u8,
    pub SecurityDescriptor: *mut ::std::ffi::c_void,
}
impl RPC_INTERFACE_TEMPLATEA {}
impl ::std::default::Default for RPC_INTERFACE_TEMPLATEA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RPC_INTERFACE_TEMPLATEA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RPC_INTERFACE_TEMPLATEA")
            .field("Version", &self.Version)
            .field("IfSpec", &self.IfSpec)
            .field("MgrTypeUuid", &self.MgrTypeUuid)
            .field("MgrEpv", &self.MgrEpv)
            .field("Flags", &self.Flags)
            .field("MaxCalls", &self.MaxCalls)
            .field("MaxRpcSize", &self.MaxRpcSize)
            .field("UuidVector", &self.UuidVector)
            .field("Annotation", &self.Annotation)
            .field("SecurityDescriptor", &self.SecurityDescriptor)
            .finish()
    }
}
impl ::std::cmp::PartialEq for RPC_INTERFACE_TEMPLATEA {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.IfSpec == other.IfSpec && self.MgrTypeUuid == other.MgrTypeUuid && self.MgrEpv == other.MgrEpv && self.Flags == other.Flags && self.MaxCalls == other.MaxCalls && self.MaxRpcSize == other.MaxRpcSize && self.IfCallback.map(|f| f as usize) == other.IfCallback.map(|f| f as usize) && self.UuidVector == other.UuidVector && self.Annotation == other.Annotation && self.SecurityDescriptor == other.SecurityDescriptor
    }
}
impl ::std::cmp::Eq for RPC_INTERFACE_TEMPLATEA {}
unsafe impl ::windows::runtime::Abi for RPC_INTERFACE_TEMPLATEA {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct RPC_INTERFACE_TEMPLATEW {
    pub Version: u32,
    pub IfSpec: *mut ::std::ffi::c_void,
    pub MgrTypeUuid: *mut ::windows::runtime::GUID,
    pub MgrEpv: *mut ::std::ffi::c_void,
    pub Flags: u32,
    pub MaxCalls: u32,
    pub MaxRpcSize: u32,
    pub IfCallback: ::std::option::Option<RPC_IF_CALLBACK_FN>,
    pub UuidVector: *mut UUID_VECTOR,
    pub Annotation: *mut u16,
    pub SecurityDescriptor: *mut ::std::ffi::c_void,
}
impl RPC_INTERFACE_TEMPLATEW {}
impl ::std::default::Default for RPC_INTERFACE_TEMPLATEW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RPC_INTERFACE_TEMPLATEW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RPC_INTERFACE_TEMPLATEW")
            .field("Version", &self.Version)
            .field("IfSpec", &self.IfSpec)
            .field("MgrTypeUuid", &self.MgrTypeUuid)
            .field("MgrEpv", &self.MgrEpv)
            .field("Flags", &self.Flags)
            .field("MaxCalls", &self.MaxCalls)
            .field("MaxRpcSize", &self.MaxRpcSize)
            .field("UuidVector", &self.UuidVector)
            .field("Annotation", &self.Annotation)
            .field("SecurityDescriptor", &self.SecurityDescriptor)
            .finish()
    }
}
impl ::std::cmp::PartialEq for RPC_INTERFACE_TEMPLATEW {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.IfSpec == other.IfSpec && self.MgrTypeUuid == other.MgrTypeUuid && self.MgrEpv == other.MgrEpv && self.Flags == other.Flags && self.MaxCalls == other.MaxCalls && self.MaxRpcSize == other.MaxRpcSize && self.IfCallback.map(|f| f as usize) == other.IfCallback.map(|f| f as usize) && self.UuidVector == other.UuidVector && self.Annotation == other.Annotation && self.SecurityDescriptor == other.SecurityDescriptor
    }
}
impl ::std::cmp::Eq for RPC_INTERFACE_TEMPLATEW {}
unsafe impl ::windows::runtime::Abi for RPC_INTERFACE_TEMPLATEW {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct RPC_MESSAGE {
    pub Handle: *mut ::std::ffi::c_void,
    pub DataRepresentation: u32,
    pub Buffer: *mut ::std::ffi::c_void,
    pub BufferLength: u32,
    pub ProcNum: u32,
    pub TransferSyntax: *mut RPC_SYNTAX_IDENTIFIER,
    pub RpcInterfaceInformation: *mut ::std::ffi::c_void,
    pub ReservedForRuntime: *mut ::std::ffi::c_void,
    pub ManagerEpv: *mut ::std::ffi::c_void,
    pub ImportContext: *mut ::std::ffi::c_void,
    pub RpcFlags: u32,
}
impl RPC_MESSAGE {}
impl ::std::default::Default for RPC_MESSAGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RPC_MESSAGE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RPC_MESSAGE")
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
impl ::std::cmp::PartialEq for RPC_MESSAGE {
    fn eq(&self, other: &Self) -> bool {
        self.Handle == other.Handle && self.DataRepresentation == other.DataRepresentation && self.Buffer == other.Buffer && self.BufferLength == other.BufferLength && self.ProcNum == other.ProcNum && self.TransferSyntax == other.TransferSyntax && self.RpcInterfaceInformation == other.RpcInterfaceInformation && self.ReservedForRuntime == other.ReservedForRuntime && self.ManagerEpv == other.ManagerEpv && self.ImportContext == other.ImportContext && self.RpcFlags == other.RpcFlags
    }
}
impl ::std::cmp::Eq for RPC_MESSAGE {}
unsafe impl ::windows::runtime::Abi for RPC_MESSAGE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub type RPC_MGMT_AUTHORIZATION_FN = unsafe extern "system" fn(clientbinding: *const ::std::ffi::c_void, requestedmgmtoperation: u32, status: *mut RPC_STATUS) -> i32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_NCA_FLAGS_BROADCAST: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_NCA_FLAGS_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_NCA_FLAGS_IDEMPOTENT: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_NCA_FLAGS_MAYBE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub type RPC_NEW_HTTP_PROXY_CHANNEL = unsafe extern "system" fn(redirectorstage: RPC_HTTP_REDIRECTOR_STAGE, servername: *const u16, serverport: *const u16, remoteuser: *const u16, authtype: *const u16, resourceuuid: *mut ::std::ffi::c_void, sessionid: *mut ::std::ffi::c_void, interface: *const ::std::ffi::c_void, reserved: *const ::std::ffi::c_void, flags: u32, newservername: *mut *mut u16, newserverport: *mut *mut u16) -> RPC_STATUS;
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RPC_NOTIFICATIONS(pub i32);
pub const RpcNotificationCallNone: RPC_NOTIFICATIONS = RPC_NOTIFICATIONS(0i32);
pub const RpcNotificationClientDisconnect: RPC_NOTIFICATIONS = RPC_NOTIFICATIONS(1i32);
pub const RpcNotificationCallCancel: RPC_NOTIFICATIONS = RPC_NOTIFICATIONS(2i32);
impl ::std::convert::From<i32> for RPC_NOTIFICATIONS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RPC_NOTIFICATIONS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RPC_NOTIFICATION_TYPES(pub i32);
pub const RpcNotificationTypeNone: RPC_NOTIFICATION_TYPES = RPC_NOTIFICATION_TYPES(0i32);
pub const RpcNotificationTypeEvent: RPC_NOTIFICATION_TYPES = RPC_NOTIFICATION_TYPES(1i32);
pub const RpcNotificationTypeApc: RPC_NOTIFICATION_TYPES = RPC_NOTIFICATION_TYPES(2i32);
pub const RpcNotificationTypeIoc: RPC_NOTIFICATION_TYPES = RPC_NOTIFICATION_TYPES(3i32);
pub const RpcNotificationTypeHwnd: RPC_NOTIFICATION_TYPES = RPC_NOTIFICATION_TYPES(4i32);
pub const RpcNotificationTypeCallback: RPC_NOTIFICATION_TYPES = RPC_NOTIFICATION_TYPES(5i32);
impl ::std::convert::From<i32> for RPC_NOTIFICATION_TYPES {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RPC_NOTIFICATION_TYPES {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub type RPC_OBJECT_INQ_FN = unsafe extern "system" fn(objectuuid: *const ::windows::runtime::GUID, typeuuid: *mut ::windows::runtime::GUID, status: *mut RPC_STATUS);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct RPC_POLICY {
    pub Length: u32,
    pub EndpointFlags: u32,
    pub NICFlags: u32,
}
impl RPC_POLICY {}
impl ::std::default::Default for RPC_POLICY {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RPC_POLICY {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RPC_POLICY").field("Length", &self.Length).field("EndpointFlags", &self.EndpointFlags).field("NICFlags", &self.NICFlags).finish()
    }
}
impl ::std::cmp::PartialEq for RPC_POLICY {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.EndpointFlags == other.EndpointFlags && self.NICFlags == other.NICFlags
    }
}
impl ::std::cmp::Eq for RPC_POLICY {}
unsafe impl ::windows::runtime::Abi for RPC_POLICY {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct RPC_PROTSEQ_ENDPOINT {
    pub RpcProtocolSequence: *mut u8,
    pub Endpoint: *mut u8,
}
impl RPC_PROTSEQ_ENDPOINT {}
impl ::std::default::Default for RPC_PROTSEQ_ENDPOINT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RPC_PROTSEQ_ENDPOINT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RPC_PROTSEQ_ENDPOINT").field("RpcProtocolSequence", &self.RpcProtocolSequence).field("Endpoint", &self.Endpoint).finish()
    }
}
impl ::std::cmp::PartialEq for RPC_PROTSEQ_ENDPOINT {
    fn eq(&self, other: &Self) -> bool {
        self.RpcProtocolSequence == other.RpcProtocolSequence && self.Endpoint == other.Endpoint
    }
}
impl ::std::cmp::Eq for RPC_PROTSEQ_ENDPOINT {}
unsafe impl ::windows::runtime::Abi for RPC_PROTSEQ_ENDPOINT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_PROTSEQ_HTTP: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_PROTSEQ_LRPC: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_PROTSEQ_NMP: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_PROTSEQ_TCP: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct RPC_PROTSEQ_VECTORA {
    pub Count: u32,
    pub Protseq: [*mut u8; 1],
}
impl RPC_PROTSEQ_VECTORA {}
impl ::std::default::Default for RPC_PROTSEQ_VECTORA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RPC_PROTSEQ_VECTORA {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RPC_PROTSEQ_VECTORA").field("Count", &self.Count).field("Protseq", &self.Protseq).finish()
    }
}
impl ::std::cmp::PartialEq for RPC_PROTSEQ_VECTORA {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.Protseq == other.Protseq
    }
}
impl ::std::cmp::Eq for RPC_PROTSEQ_VECTORA {}
unsafe impl ::windows::runtime::Abi for RPC_PROTSEQ_VECTORA {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct RPC_PROTSEQ_VECTORW {
    pub Count: u32,
    pub Protseq: [*mut u16; 1],
}
impl RPC_PROTSEQ_VECTORW {}
impl ::std::default::Default for RPC_PROTSEQ_VECTORW {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RPC_PROTSEQ_VECTORW {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RPC_PROTSEQ_VECTORW").field("Count", &self.Count).field("Protseq", &self.Protseq).finish()
    }
}
impl ::std::cmp::PartialEq for RPC_PROTSEQ_VECTORW {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.Protseq == other.Protseq
    }
}
impl ::std::cmp::Eq for RPC_PROTSEQ_VECTORW {}
unsafe impl ::windows::runtime::Abi for RPC_PROTSEQ_VECTORW {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_PROXY_CONNECTION_TYPE_IN_PROXY: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_PROXY_CONNECTION_TYPE_OUT_PROXY: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_P_ADDR_FORMAT_TCP_IPV4: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_P_ADDR_FORMAT_TCP_IPV6: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_QUERY_CALL_LOCAL_ADDRESS: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_QUERY_CLIENT_ID: u32 = 128u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_QUERY_CLIENT_PID: u32 = 16u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_QUERY_CLIENT_PRINCIPAL_NAME: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_QUERY_IS_CLIENT_LOCAL: u32 = 32u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_QUERY_NO_AUTH_REQUIRED: u32 = 64u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_QUERY_SERVER_PRINCIPAL_NAME: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub type RPC_SECURITY_CALLBACK_FN = unsafe extern "system" fn(context: *const ::std::ffi::c_void);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
pub struct RPC_SECURITY_QOS {
    pub Version: u32,
    pub Capabilities: RPC_C_QOS_CAPABILITIES,
    pub IdentityTracking: RPC_C_QOS_IDENTITY,
    pub ImpersonationType: super::Com::RPC_C_IMP_LEVEL,
}
#[cfg(feature = "Win32_System_Com")]
impl RPC_SECURITY_QOS {}
#[cfg(feature = "Win32_System_Com")]
impl ::std::default::Default for RPC_SECURITY_QOS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::fmt::Debug for RPC_SECURITY_QOS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RPC_SECURITY_QOS").field("Version", &self.Version).field("Capabilities", &self.Capabilities).field("IdentityTracking", &self.IdentityTracking).field("ImpersonationType", &self.ImpersonationType).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::PartialEq for RPC_SECURITY_QOS {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version && self.Capabilities == other.Capabilities && self.IdentityTracking == other.IdentityTracking && self.ImpersonationType == other.ImpersonationType
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::Eq for RPC_SECURITY_QOS {}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::runtime::Abi for RPC_SECURITY_QOS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
pub struct RPC_SECURITY_QOS_V2_A {
    pub Version: u32,
    pub Capabilities: RPC_C_QOS_CAPABILITIES,
    pub IdentityTracking: RPC_C_QOS_IDENTITY,
    pub ImpersonationType: super::Com::RPC_C_IMP_LEVEL,
    pub AdditionalSecurityInfoType: RPC_C_AUTHN_INFO_TYPE,
    pub u: RPC_SECURITY_QOS_V2_A_0,
}
#[cfg(feature = "Win32_System_Com")]
impl RPC_SECURITY_QOS_V2_A {}
#[cfg(feature = "Win32_System_Com")]
impl ::std::default::Default for RPC_SECURITY_QOS_V2_A {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::PartialEq for RPC_SECURITY_QOS_V2_A {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::Eq for RPC_SECURITY_QOS_V2_A {}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::runtime::Abi for RPC_SECURITY_QOS_V2_A {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
pub union RPC_SECURITY_QOS_V2_A_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_A,
}
#[cfg(feature = "Win32_System_Com")]
impl RPC_SECURITY_QOS_V2_A_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::std::default::Default for RPC_SECURITY_QOS_V2_A_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::PartialEq for RPC_SECURITY_QOS_V2_A_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::Eq for RPC_SECURITY_QOS_V2_A_0 {}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::runtime::Abi for RPC_SECURITY_QOS_V2_A_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
pub struct RPC_SECURITY_QOS_V2_W {
    pub Version: u32,
    pub Capabilities: RPC_C_QOS_CAPABILITIES,
    pub IdentityTracking: RPC_C_QOS_IDENTITY,
    pub ImpersonationType: super::Com::RPC_C_IMP_LEVEL,
    pub AdditionalSecurityInfoType: RPC_C_AUTHN_INFO_TYPE,
    pub u: RPC_SECURITY_QOS_V2_W_0,
}
#[cfg(feature = "Win32_System_Com")]
impl RPC_SECURITY_QOS_V2_W {}
#[cfg(feature = "Win32_System_Com")]
impl ::std::default::Default for RPC_SECURITY_QOS_V2_W {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::PartialEq for RPC_SECURITY_QOS_V2_W {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::Eq for RPC_SECURITY_QOS_V2_W {}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::runtime::Abi for RPC_SECURITY_QOS_V2_W {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
pub union RPC_SECURITY_QOS_V2_W_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_W,
}
#[cfg(feature = "Win32_System_Com")]
impl RPC_SECURITY_QOS_V2_W_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::std::default::Default for RPC_SECURITY_QOS_V2_W_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::PartialEq for RPC_SECURITY_QOS_V2_W_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::Eq for RPC_SECURITY_QOS_V2_W_0 {}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::runtime::Abi for RPC_SECURITY_QOS_V2_W_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
pub struct RPC_SECURITY_QOS_V3_A {
    pub Version: u32,
    pub Capabilities: RPC_C_QOS_CAPABILITIES,
    pub IdentityTracking: RPC_C_QOS_IDENTITY,
    pub ImpersonationType: super::Com::RPC_C_IMP_LEVEL,
    pub AdditionalSecurityInfoType: RPC_C_AUTHN_INFO_TYPE,
    pub u: RPC_SECURITY_QOS_V3_A_0,
    pub Sid: *mut ::std::ffi::c_void,
}
#[cfg(feature = "Win32_System_Com")]
impl RPC_SECURITY_QOS_V3_A {}
#[cfg(feature = "Win32_System_Com")]
impl ::std::default::Default for RPC_SECURITY_QOS_V3_A {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::PartialEq for RPC_SECURITY_QOS_V3_A {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::Eq for RPC_SECURITY_QOS_V3_A {}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::runtime::Abi for RPC_SECURITY_QOS_V3_A {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
pub union RPC_SECURITY_QOS_V3_A_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_A,
}
#[cfg(feature = "Win32_System_Com")]
impl RPC_SECURITY_QOS_V3_A_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::std::default::Default for RPC_SECURITY_QOS_V3_A_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::PartialEq for RPC_SECURITY_QOS_V3_A_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::Eq for RPC_SECURITY_QOS_V3_A_0 {}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::runtime::Abi for RPC_SECURITY_QOS_V3_A_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
pub struct RPC_SECURITY_QOS_V3_W {
    pub Version: u32,
    pub Capabilities: RPC_C_QOS_CAPABILITIES,
    pub IdentityTracking: RPC_C_QOS_IDENTITY,
    pub ImpersonationType: super::Com::RPC_C_IMP_LEVEL,
    pub AdditionalSecurityInfoType: RPC_C_AUTHN_INFO_TYPE,
    pub u: RPC_SECURITY_QOS_V3_W_0,
    pub Sid: *mut ::std::ffi::c_void,
}
#[cfg(feature = "Win32_System_Com")]
impl RPC_SECURITY_QOS_V3_W {}
#[cfg(feature = "Win32_System_Com")]
impl ::std::default::Default for RPC_SECURITY_QOS_V3_W {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::PartialEq for RPC_SECURITY_QOS_V3_W {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::Eq for RPC_SECURITY_QOS_V3_W {}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::runtime::Abi for RPC_SECURITY_QOS_V3_W {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
pub union RPC_SECURITY_QOS_V3_W_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_W,
}
#[cfg(feature = "Win32_System_Com")]
impl RPC_SECURITY_QOS_V3_W_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::std::default::Default for RPC_SECURITY_QOS_V3_W_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::PartialEq for RPC_SECURITY_QOS_V3_W_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::Eq for RPC_SECURITY_QOS_V3_W_0 {}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::runtime::Abi for RPC_SECURITY_QOS_V3_W_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
pub struct RPC_SECURITY_QOS_V4_A {
    pub Version: u32,
    pub Capabilities: RPC_C_QOS_CAPABILITIES,
    pub IdentityTracking: RPC_C_QOS_IDENTITY,
    pub ImpersonationType: super::Com::RPC_C_IMP_LEVEL,
    pub AdditionalSecurityInfoType: RPC_C_AUTHN_INFO_TYPE,
    pub u: RPC_SECURITY_QOS_V4_A_0,
    pub Sid: *mut ::std::ffi::c_void,
    pub EffectiveOnly: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl RPC_SECURITY_QOS_V4_A {}
#[cfg(feature = "Win32_System_Com")]
impl ::std::default::Default for RPC_SECURITY_QOS_V4_A {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::PartialEq for RPC_SECURITY_QOS_V4_A {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::Eq for RPC_SECURITY_QOS_V4_A {}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::runtime::Abi for RPC_SECURITY_QOS_V4_A {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
pub union RPC_SECURITY_QOS_V4_A_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_A,
}
#[cfg(feature = "Win32_System_Com")]
impl RPC_SECURITY_QOS_V4_A_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::std::default::Default for RPC_SECURITY_QOS_V4_A_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::PartialEq for RPC_SECURITY_QOS_V4_A_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::Eq for RPC_SECURITY_QOS_V4_A_0 {}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::runtime::Abi for RPC_SECURITY_QOS_V4_A_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
pub struct RPC_SECURITY_QOS_V4_W {
    pub Version: u32,
    pub Capabilities: RPC_C_QOS_CAPABILITIES,
    pub IdentityTracking: RPC_C_QOS_IDENTITY,
    pub ImpersonationType: super::Com::RPC_C_IMP_LEVEL,
    pub AdditionalSecurityInfoType: RPC_C_AUTHN_INFO_TYPE,
    pub u: RPC_SECURITY_QOS_V4_W_0,
    pub Sid: *mut ::std::ffi::c_void,
    pub EffectiveOnly: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl RPC_SECURITY_QOS_V4_W {}
#[cfg(feature = "Win32_System_Com")]
impl ::std::default::Default for RPC_SECURITY_QOS_V4_W {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::PartialEq for RPC_SECURITY_QOS_V4_W {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::Eq for RPC_SECURITY_QOS_V4_W {}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::runtime::Abi for RPC_SECURITY_QOS_V4_W {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
pub union RPC_SECURITY_QOS_V4_W_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_W,
}
#[cfg(feature = "Win32_System_Com")]
impl RPC_SECURITY_QOS_V4_W_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::std::default::Default for RPC_SECURITY_QOS_V4_W_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::PartialEq for RPC_SECURITY_QOS_V4_W_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::Eq for RPC_SECURITY_QOS_V4_W_0 {}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::runtime::Abi for RPC_SECURITY_QOS_V4_W_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
pub struct RPC_SECURITY_QOS_V5_A {
    pub Version: u32,
    pub Capabilities: RPC_C_QOS_CAPABILITIES,
    pub IdentityTracking: RPC_C_QOS_IDENTITY,
    pub ImpersonationType: super::Com::RPC_C_IMP_LEVEL,
    pub AdditionalSecurityInfoType: RPC_C_AUTHN_INFO_TYPE,
    pub u: RPC_SECURITY_QOS_V5_A_0,
    pub Sid: *mut ::std::ffi::c_void,
    pub EffectiveOnly: u32,
    pub ServerSecurityDescriptor: *mut ::std::ffi::c_void,
}
#[cfg(feature = "Win32_System_Com")]
impl RPC_SECURITY_QOS_V5_A {}
#[cfg(feature = "Win32_System_Com")]
impl ::std::default::Default for RPC_SECURITY_QOS_V5_A {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::PartialEq for RPC_SECURITY_QOS_V5_A {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::Eq for RPC_SECURITY_QOS_V5_A {}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::runtime::Abi for RPC_SECURITY_QOS_V5_A {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
pub union RPC_SECURITY_QOS_V5_A_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_A,
}
#[cfg(feature = "Win32_System_Com")]
impl RPC_SECURITY_QOS_V5_A_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::std::default::Default for RPC_SECURITY_QOS_V5_A_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::PartialEq for RPC_SECURITY_QOS_V5_A_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::Eq for RPC_SECURITY_QOS_V5_A_0 {}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::runtime::Abi for RPC_SECURITY_QOS_V5_A_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
pub struct RPC_SECURITY_QOS_V5_W {
    pub Version: u32,
    pub Capabilities: RPC_C_QOS_CAPABILITIES,
    pub IdentityTracking: RPC_C_QOS_IDENTITY,
    pub ImpersonationType: super::Com::RPC_C_IMP_LEVEL,
    pub AdditionalSecurityInfoType: RPC_C_AUTHN_INFO_TYPE,
    pub u: RPC_SECURITY_QOS_V5_W_0,
    pub Sid: *mut ::std::ffi::c_void,
    pub EffectiveOnly: u32,
    pub ServerSecurityDescriptor: *mut ::std::ffi::c_void,
}
#[cfg(feature = "Win32_System_Com")]
impl RPC_SECURITY_QOS_V5_W {}
#[cfg(feature = "Win32_System_Com")]
impl ::std::default::Default for RPC_SECURITY_QOS_V5_W {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::PartialEq for RPC_SECURITY_QOS_V5_W {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::Eq for RPC_SECURITY_QOS_V5_W {}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::runtime::Abi for RPC_SECURITY_QOS_V5_W {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
pub union RPC_SECURITY_QOS_V5_W_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_W,
}
#[cfg(feature = "Win32_System_Com")]
impl RPC_SECURITY_QOS_V5_W_0 {}
#[cfg(feature = "Win32_System_Com")]
impl ::std::default::Default for RPC_SECURITY_QOS_V5_W_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::PartialEq for RPC_SECURITY_QOS_V5_W_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::Eq for RPC_SECURITY_QOS_V5_W_0 {}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::runtime::Abi for RPC_SECURITY_QOS_V5_W_0 {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct RPC_SEC_CONTEXT_KEY_INFO {
    pub EncryptAlgorithm: u32,
    pub KeySize: u32,
    pub SignatureAlgorithm: u32,
}
impl RPC_SEC_CONTEXT_KEY_INFO {}
impl ::std::default::Default for RPC_SEC_CONTEXT_KEY_INFO {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RPC_SEC_CONTEXT_KEY_INFO {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RPC_SEC_CONTEXT_KEY_INFO").field("EncryptAlgorithm", &self.EncryptAlgorithm).field("KeySize", &self.KeySize).field("SignatureAlgorithm", &self.SignatureAlgorithm).finish()
    }
}
impl ::std::cmp::PartialEq for RPC_SEC_CONTEXT_KEY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.EncryptAlgorithm == other.EncryptAlgorithm && self.KeySize == other.KeySize && self.SignatureAlgorithm == other.SignatureAlgorithm
    }
}
impl ::std::cmp::Eq for RPC_SEC_CONTEXT_KEY_INFO {}
unsafe impl ::windows::runtime::Abi for RPC_SEC_CONTEXT_KEY_INFO {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct RPC_SERVER_INTERFACE {
    pub Length: u32,
    pub InterfaceId: RPC_SYNTAX_IDENTIFIER,
    pub TransferSyntax: RPC_SYNTAX_IDENTIFIER,
    pub DispatchTable: *mut RPC_DISPATCH_TABLE,
    pub RpcProtseqEndpointCount: u32,
    pub RpcProtseqEndpoint: *mut RPC_PROTSEQ_ENDPOINT,
    pub DefaultManagerEpv: *mut ::std::ffi::c_void,
    pub InterpreterInfo: *mut ::std::ffi::c_void,
    pub Flags: u32,
}
impl RPC_SERVER_INTERFACE {}
impl ::std::default::Default for RPC_SERVER_INTERFACE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RPC_SERVER_INTERFACE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RPC_SERVER_INTERFACE")
            .field("Length", &self.Length)
            .field("InterfaceId", &self.InterfaceId)
            .field("TransferSyntax", &self.TransferSyntax)
            .field("DispatchTable", &self.DispatchTable)
            .field("RpcProtseqEndpointCount", &self.RpcProtseqEndpointCount)
            .field("RpcProtseqEndpoint", &self.RpcProtseqEndpoint)
            .field("DefaultManagerEpv", &self.DefaultManagerEpv)
            .field("InterpreterInfo", &self.InterpreterInfo)
            .field("Flags", &self.Flags)
            .finish()
    }
}
impl ::std::cmp::PartialEq for RPC_SERVER_INTERFACE {
    fn eq(&self, other: &Self) -> bool {
        self.Length == other.Length && self.InterfaceId == other.InterfaceId && self.TransferSyntax == other.TransferSyntax && self.DispatchTable == other.DispatchTable && self.RpcProtseqEndpointCount == other.RpcProtseqEndpointCount && self.RpcProtseqEndpoint == other.RpcProtseqEndpoint && self.DefaultManagerEpv == other.DefaultManagerEpv && self.InterpreterInfo == other.InterpreterInfo && self.Flags == other.Flags
    }
}
impl ::std::cmp::Eq for RPC_SERVER_INTERFACE {}
unsafe impl ::windows::runtime::Abi for RPC_SERVER_INTERFACE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub type RPC_SETFILTER_FUNC = unsafe extern "system" fn(pfnfilter: ::windows::runtime::RawPtr);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct RPC_STATS_VECTOR {
    pub Count: u32,
    pub Stats: [u32; 1],
}
impl RPC_STATS_VECTOR {}
impl ::std::default::Default for RPC_STATS_VECTOR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RPC_STATS_VECTOR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RPC_STATS_VECTOR").field("Count", &self.Count).field("Stats", &self.Stats).finish()
    }
}
impl ::std::cmp::PartialEq for RPC_STATS_VECTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.Stats == other.Stats
    }
}
impl ::std::cmp::Eq for RPC_STATS_VECTOR {}
unsafe impl ::windows::runtime::Abi for RPC_STATS_VECTOR {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RPC_STATUS(pub i32);
pub const RPC_S_INVALID_STRING_BINDING: RPC_STATUS = RPC_STATUS(1700i32);
pub const RPC_S_WRONG_KIND_OF_BINDING: RPC_STATUS = RPC_STATUS(1701i32);
pub const RPC_S_INVALID_BINDING: RPC_STATUS = RPC_STATUS(1702i32);
pub const RPC_S_PROTSEQ_NOT_SUPPORTED: RPC_STATUS = RPC_STATUS(1703i32);
pub const RPC_S_INVALID_RPC_PROTSEQ: RPC_STATUS = RPC_STATUS(1704i32);
pub const RPC_S_INVALID_STRING_UUID: RPC_STATUS = RPC_STATUS(1705i32);
pub const RPC_S_INVALID_ENDPOINT_FORMAT: RPC_STATUS = RPC_STATUS(1706i32);
pub const RPC_S_INVALID_NET_ADDR: RPC_STATUS = RPC_STATUS(1707i32);
pub const RPC_S_NO_ENDPOINT_FOUND: RPC_STATUS = RPC_STATUS(1708i32);
pub const RPC_S_INVALID_TIMEOUT: RPC_STATUS = RPC_STATUS(1709i32);
pub const RPC_S_OBJECT_NOT_FOUND: RPC_STATUS = RPC_STATUS(1710i32);
pub const RPC_S_ALREADY_REGISTERED: RPC_STATUS = RPC_STATUS(1711i32);
pub const RPC_S_TYPE_ALREADY_REGISTERED: RPC_STATUS = RPC_STATUS(1712i32);
pub const RPC_S_ALREADY_LISTENING: RPC_STATUS = RPC_STATUS(1713i32);
pub const RPC_S_NO_PROTSEQS_REGISTERED: RPC_STATUS = RPC_STATUS(1714i32);
pub const RPC_S_NOT_LISTENING: RPC_STATUS = RPC_STATUS(1715i32);
pub const RPC_S_UNKNOWN_MGR_TYPE: RPC_STATUS = RPC_STATUS(1716i32);
pub const RPC_S_UNKNOWN_IF: RPC_STATUS = RPC_STATUS(1717i32);
pub const RPC_S_NO_BINDINGS: RPC_STATUS = RPC_STATUS(1718i32);
pub const RPC_S_NO_PROTSEQS: RPC_STATUS = RPC_STATUS(1719i32);
pub const RPC_S_CANT_CREATE_ENDPOINT: RPC_STATUS = RPC_STATUS(1720i32);
pub const RPC_S_OUT_OF_RESOURCES: RPC_STATUS = RPC_STATUS(1721i32);
pub const RPC_S_SERVER_UNAVAILABLE: RPC_STATUS = RPC_STATUS(1722i32);
pub const RPC_S_SERVER_TOO_BUSY: RPC_STATUS = RPC_STATUS(1723i32);
pub const RPC_S_INVALID_NETWORK_OPTIONS: RPC_STATUS = RPC_STATUS(1724i32);
pub const RPC_S_NO_CALL_ACTIVE: RPC_STATUS = RPC_STATUS(1725i32);
pub const RPC_S_CALL_FAILED: RPC_STATUS = RPC_STATUS(1726i32);
pub const RPC_S_CALL_FAILED_DNE: RPC_STATUS = RPC_STATUS(1727i32);
pub const RPC_S_PROTOCOL_ERROR: RPC_STATUS = RPC_STATUS(1728i32);
pub const RPC_S_PROXY_ACCESS_DENIED: RPC_STATUS = RPC_STATUS(1729i32);
pub const RPC_S_UNSUPPORTED_TRANS_SYN: RPC_STATUS = RPC_STATUS(1730i32);
pub const RPC_S_UNSUPPORTED_TYPE: RPC_STATUS = RPC_STATUS(1732i32);
pub const RPC_S_INVALID_TAG: RPC_STATUS = RPC_STATUS(1733i32);
pub const RPC_S_INVALID_BOUND: RPC_STATUS = RPC_STATUS(1734i32);
pub const RPC_S_NO_ENTRY_NAME: RPC_STATUS = RPC_STATUS(1735i32);
pub const RPC_S_INVALID_NAME_SYNTAX: RPC_STATUS = RPC_STATUS(1736i32);
pub const RPC_S_UNSUPPORTED_NAME_SYNTAX: RPC_STATUS = RPC_STATUS(1737i32);
pub const RPC_S_UUID_NO_ADDRESS: RPC_STATUS = RPC_STATUS(1739i32);
pub const RPC_S_DUPLICATE_ENDPOINT: RPC_STATUS = RPC_STATUS(1740i32);
pub const RPC_S_UNKNOWN_AUTHN_TYPE: RPC_STATUS = RPC_STATUS(1741i32);
pub const RPC_S_MAX_CALLS_TOO_SMALL: RPC_STATUS = RPC_STATUS(1742i32);
pub const RPC_S_STRING_TOO_LONG: RPC_STATUS = RPC_STATUS(1743i32);
pub const RPC_S_PROTSEQ_NOT_FOUND: RPC_STATUS = RPC_STATUS(1744i32);
pub const RPC_S_PROCNUM_OUT_OF_RANGE: RPC_STATUS = RPC_STATUS(1745i32);
pub const RPC_S_BINDING_HAS_NO_AUTH: RPC_STATUS = RPC_STATUS(1746i32);
pub const RPC_S_UNKNOWN_AUTHN_SERVICE: RPC_STATUS = RPC_STATUS(1747i32);
pub const RPC_S_UNKNOWN_AUTHN_LEVEL: RPC_STATUS = RPC_STATUS(1748i32);
pub const RPC_S_INVALID_AUTH_IDENTITY: RPC_STATUS = RPC_STATUS(1749i32);
pub const RPC_S_UNKNOWN_AUTHZ_SERVICE: RPC_STATUS = RPC_STATUS(1750i32);
pub const EPT_S_INVALID_ENTRY: RPC_STATUS = RPC_STATUS(1751i32);
pub const EPT_S_CANT_PERFORM_OP: RPC_STATUS = RPC_STATUS(1752i32);
pub const EPT_S_NOT_REGISTERED: RPC_STATUS = RPC_STATUS(1753i32);
pub const RPC_S_NOTHING_TO_EXPORT: RPC_STATUS = RPC_STATUS(1754i32);
pub const RPC_S_INCOMPLETE_NAME: RPC_STATUS = RPC_STATUS(1755i32);
pub const RPC_S_INVALID_VERS_OPTION: RPC_STATUS = RPC_STATUS(1756i32);
pub const RPC_S_NO_MORE_MEMBERS: RPC_STATUS = RPC_STATUS(1757i32);
pub const RPC_S_NOT_ALL_OBJS_UNEXPORTED: RPC_STATUS = RPC_STATUS(1758i32);
pub const RPC_S_INTERFACE_NOT_FOUND: RPC_STATUS = RPC_STATUS(1759i32);
pub const RPC_S_ENTRY_ALREADY_EXISTS: RPC_STATUS = RPC_STATUS(1760i32);
pub const RPC_S_ENTRY_NOT_FOUND: RPC_STATUS = RPC_STATUS(1761i32);
pub const RPC_S_NAME_SERVICE_UNAVAILABLE: RPC_STATUS = RPC_STATUS(1762i32);
pub const RPC_S_INVALID_NAF_ID: RPC_STATUS = RPC_STATUS(1763i32);
pub const RPC_S_CANNOT_SUPPORT: RPC_STATUS = RPC_STATUS(1764i32);
pub const RPC_S_NO_CONTEXT_AVAILABLE: RPC_STATUS = RPC_STATUS(1765i32);
pub const RPC_S_INTERNAL_ERROR: RPC_STATUS = RPC_STATUS(1766i32);
pub const RPC_S_ZERO_DIVIDE: RPC_STATUS = RPC_STATUS(1767i32);
pub const RPC_S_ADDRESS_ERROR: RPC_STATUS = RPC_STATUS(1768i32);
pub const RPC_S_FP_DIV_ZERO: RPC_STATUS = RPC_STATUS(1769i32);
pub const RPC_S_FP_UNDERFLOW: RPC_STATUS = RPC_STATUS(1770i32);
pub const RPC_S_FP_OVERFLOW: RPC_STATUS = RPC_STATUS(1771i32);
pub const RPC_S_CALL_IN_PROGRESS: RPC_STATUS = RPC_STATUS(1791i32);
pub const RPC_S_NO_MORE_BINDINGS: RPC_STATUS = RPC_STATUS(1806i32);
pub const RPC_S_NO_INTERFACES: RPC_STATUS = RPC_STATUS(1817i32);
pub const RPC_S_CALL_CANCELLED: RPC_STATUS = RPC_STATUS(1818i32);
pub const RPC_S_BINDING_INCOMPLETE: RPC_STATUS = RPC_STATUS(1819i32);
pub const RPC_S_COMM_FAILURE: RPC_STATUS = RPC_STATUS(1820i32);
pub const RPC_S_UNSUPPORTED_AUTHN_LEVEL: RPC_STATUS = RPC_STATUS(1821i32);
pub const RPC_S_NO_PRINC_NAME: RPC_STATUS = RPC_STATUS(1822i32);
pub const RPC_S_NOT_RPC_ERROR: RPC_STATUS = RPC_STATUS(1823i32);
pub const RPC_S_UUID_LOCAL_ONLY: RPC_STATUS = RPC_STATUS(1824i32);
pub const RPC_S_SEC_PKG_ERROR: RPC_STATUS = RPC_STATUS(1825i32);
pub const RPC_S_NOT_CANCELLED: RPC_STATUS = RPC_STATUS(1826i32);
pub const RPC_S_COOKIE_AUTH_FAILED: RPC_STATUS = RPC_STATUS(1833i32);
pub const RPC_S_DO_NOT_DISTURB: RPC_STATUS = RPC_STATUS(1834i32);
pub const RPC_S_SYSTEM_HANDLE_COUNT_EXCEEDED: RPC_STATUS = RPC_STATUS(1835i32);
pub const RPC_S_SYSTEM_HANDLE_TYPE_MISMATCH: RPC_STATUS = RPC_STATUS(1836i32);
pub const RPC_S_GROUP_MEMBER_NOT_FOUND: RPC_STATUS = RPC_STATUS(1898i32);
pub const EPT_S_CANT_CREATE: RPC_STATUS = RPC_STATUS(1899i32);
pub const RPC_S_INVALID_OBJECT: RPC_STATUS = RPC_STATUS(1900i32);
pub const RPC_S_SEND_INCOMPLETE: RPC_STATUS = RPC_STATUS(1913i32);
pub const RPC_S_INVALID_ASYNC_HANDLE: RPC_STATUS = RPC_STATUS(1914i32);
pub const RPC_S_INVALID_ASYNC_CALL: RPC_STATUS = RPC_STATUS(1915i32);
pub const RPC_S_ENTRY_TYPE_MISMATCH: RPC_STATUS = RPC_STATUS(1922i32);
pub const RPC_S_NOT_ALL_OBJS_EXPORTED: RPC_STATUS = RPC_STATUS(1923i32);
pub const RPC_S_INTERFACE_NOT_EXPORTED: RPC_STATUS = RPC_STATUS(1924i32);
pub const RPC_S_PROFILE_NOT_ADDED: RPC_STATUS = RPC_STATUS(1925i32);
pub const RPC_S_PRF_ELT_NOT_ADDED: RPC_STATUS = RPC_STATUS(1926i32);
pub const RPC_S_PRF_ELT_NOT_REMOVED: RPC_STATUS = RPC_STATUS(1927i32);
pub const RPC_S_GRP_ELT_NOT_ADDED: RPC_STATUS = RPC_STATUS(1928i32);
pub const RPC_S_GRP_ELT_NOT_REMOVED: RPC_STATUS = RPC_STATUS(1929i32);
impl ::std::convert::From<i32> for RPC_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RPC_STATUS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct RPC_SYNTAX_IDENTIFIER {
    pub SyntaxGUID: ::windows::runtime::GUID,
    pub SyntaxVersion: RPC_VERSION,
}
impl RPC_SYNTAX_IDENTIFIER {}
impl ::std::default::Default for RPC_SYNTAX_IDENTIFIER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RPC_SYNTAX_IDENTIFIER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RPC_SYNTAX_IDENTIFIER").field("SyntaxGUID", &self.SyntaxGUID).field("SyntaxVersion", &self.SyntaxVersion).finish()
    }
}
impl ::std::cmp::PartialEq for RPC_SYNTAX_IDENTIFIER {
    fn eq(&self, other: &Self) -> bool {
        self.SyntaxGUID == other.SyntaxGUID && self.SyntaxVersion == other.SyntaxVersion
    }
}
impl ::std::cmp::Eq for RPC_SYNTAX_IDENTIFIER {}
unsafe impl ::windows::runtime::Abi for RPC_SYNTAX_IDENTIFIER {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_SYSTEM_HANDLE_FREE_ALL: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_SYSTEM_HANDLE_FREE_ERROR_ON_CLOSE: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_SYSTEM_HANDLE_FREE_RETRIEVED: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_SYSTEM_HANDLE_FREE_UNRETRIEVED: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct RPC_TRANSFER_SYNTAX {
    pub Uuid: ::windows::runtime::GUID,
    pub VersMajor: u16,
    pub VersMinor: u16,
}
impl RPC_TRANSFER_SYNTAX {}
impl ::std::default::Default for RPC_TRANSFER_SYNTAX {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RPC_TRANSFER_SYNTAX {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RPC_TRANSFER_SYNTAX").field("Uuid", &self.Uuid).field("VersMajor", &self.VersMajor).field("VersMinor", &self.VersMinor).finish()
    }
}
impl ::std::cmp::PartialEq for RPC_TRANSFER_SYNTAX {
    fn eq(&self, other: &Self) -> bool {
        self.Uuid == other.Uuid && self.VersMajor == other.VersMajor && self.VersMinor == other.VersMinor
    }
}
impl ::std::cmp::Eq for RPC_TRANSFER_SYNTAX {}
unsafe impl ::windows::runtime::Abi for RPC_TRANSFER_SYNTAX {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_TYPE_DISCONNECT_EVENT_CONTEXT_HANDLE: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const RPC_TYPE_STRICT_CONTEXT_HANDLE: u32 = 1073741824u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct RPC_VERSION {
    pub MajorVersion: u16,
    pub MinorVersion: u16,
}
impl RPC_VERSION {}
impl ::std::default::Default for RPC_VERSION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for RPC_VERSION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("RPC_VERSION").field("MajorVersion", &self.MajorVersion).field("MinorVersion", &self.MinorVersion).finish()
    }
}
impl ::std::cmp::PartialEq for RPC_VERSION {
    fn eq(&self, other: &Self) -> bool {
        self.MajorVersion == other.MajorVersion && self.MinorVersion == other.MinorVersion
    }
}
impl ::std::cmp::Eq for RPC_VERSION {}
unsafe impl ::windows::runtime::Abi for RPC_VERSION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`, `Win32_System_IO`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn RpcAsyncAbortCall(pasync: *mut RPC_ASYNC_STATE, exceptioncode: u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcAsyncAbortCall(pasync: *mut ::std::mem::ManuallyDrop<RPC_ASYNC_STATE>, exceptioncode: u32) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcAsyncAbortCall(::std::mem::transmute(pasync), ::std::mem::transmute(exceptioncode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`, `Win32_System_IO`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn RpcAsyncCancelCall<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(pasync: *mut RPC_ASYNC_STATE, fabort: Param1) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcAsyncCancelCall(pasync: *mut ::std::mem::ManuallyDrop<RPC_ASYNC_STATE>, fabort: super::super::Foundation::BOOL) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcAsyncCancelCall(::std::mem::transmute(pasync), fabort.into_param().abi()))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`, `Win32_System_IO`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn RpcAsyncCompleteCall(pasync: *mut RPC_ASYNC_STATE, reply: *mut ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcAsyncCompleteCall(pasync: *mut ::std::mem::ManuallyDrop<RPC_ASYNC_STATE>, reply: *mut ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcAsyncCompleteCall(::std::mem::transmute(pasync), ::std::mem::transmute(reply)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`, `Win32_System_IO`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn RpcAsyncGetCallStatus(pasync: *const RPC_ASYNC_STATE) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcAsyncGetCallStatus(pasync: *const ::std::mem::ManuallyDrop<RPC_ASYNC_STATE>) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcAsyncGetCallStatus(::std::mem::transmute(pasync)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`, `Win32_System_IO`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn RpcAsyncInitializeHandle(pasync: *mut RPC_ASYNC_STATE, size: u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcAsyncInitializeHandle(pasync: *mut ::std::mem::ManuallyDrop<RPC_ASYNC_STATE>, size: u32) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcAsyncInitializeHandle(::std::mem::transmute(pasync), ::std::mem::transmute(size)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`, `Win32_System_IO`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn RpcAsyncRegisterInfo(pasync: *const RPC_ASYNC_STATE) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcAsyncRegisterInfo(pasync: *const ::std::mem::ManuallyDrop<RPC_ASYNC_STATE>) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcAsyncRegisterInfo(::std::mem::transmute(pasync)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`, `Win32_System_IO`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn RpcBindingBind(pasync: *const RPC_ASYNC_STATE, binding: *const ::std::ffi::c_void, ifspec: *const ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingBind(pasync: *const ::std::mem::ManuallyDrop<RPC_ASYNC_STATE>, binding: *const ::std::ffi::c_void, ifspec: *const ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcBindingBind(::std::mem::transmute(pasync), ::std::mem::transmute(binding), ::std::mem::transmute(ifspec)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcBindingCopy(sourcebinding: *const ::std::ffi::c_void, destinationbinding: *mut *mut ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingCopy(sourcebinding: *const ::std::ffi::c_void, destinationbinding: *mut *mut ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcBindingCopy(::std::mem::transmute(sourcebinding), ::std::mem::transmute(destinationbinding)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn RpcBindingCreateA(template: *const RPC_BINDING_HANDLE_TEMPLATE_V1_A, security: *const RPC_BINDING_HANDLE_SECURITY_V1_A, options: *const RPC_BINDING_HANDLE_OPTIONS_V1, binding: *mut *mut ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingCreateA(template: *const RPC_BINDING_HANDLE_TEMPLATE_V1_A, security: *const RPC_BINDING_HANDLE_SECURITY_V1_A, options: *const RPC_BINDING_HANDLE_OPTIONS_V1, binding: *mut *mut ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcBindingCreateA(::std::mem::transmute(template), ::std::mem::transmute(security), ::std::mem::transmute(options), ::std::mem::transmute(binding)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn RpcBindingCreateW(template: *const RPC_BINDING_HANDLE_TEMPLATE_V1_W, security: *const RPC_BINDING_HANDLE_SECURITY_V1_W, options: *const RPC_BINDING_HANDLE_OPTIONS_V1, binding: *mut *mut ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingCreateW(template: *const RPC_BINDING_HANDLE_TEMPLATE_V1_W, security: *const RPC_BINDING_HANDLE_SECURITY_V1_W, options: *const RPC_BINDING_HANDLE_OPTIONS_V1, binding: *mut *mut ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcBindingCreateW(::std::mem::transmute(template), ::std::mem::transmute(security), ::std::mem::transmute(options), ::std::mem::transmute(binding)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcBindingFree(binding: *mut *mut ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingFree(binding: *mut *mut ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcBindingFree(::std::mem::transmute(binding)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcBindingFromStringBindingA(stringbinding: *const u8, binding: *mut *mut ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingFromStringBindingA(stringbinding: *const u8, binding: *mut *mut ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcBindingFromStringBindingA(::std::mem::transmute(stringbinding), ::std::mem::transmute(binding)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcBindingFromStringBindingW(stringbinding: *const u16, binding: *mut *mut ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingFromStringBindingW(stringbinding: *const u16, binding: *mut *mut ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcBindingFromStringBindingW(::std::mem::transmute(stringbinding), ::std::mem::transmute(binding)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcBindingInqAuthClientA(clientbinding: *const ::std::ffi::c_void, privs: *mut *mut ::std::ffi::c_void, serverprincname: *mut *mut u8, authnlevel: *mut u32, authnsvc: *mut u32, authzsvc: *mut u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingInqAuthClientA(clientbinding: *const ::std::ffi::c_void, privs: *mut *mut ::std::ffi::c_void, serverprincname: *mut *mut u8, authnlevel: *mut u32, authnsvc: *mut u32, authzsvc: *mut u32) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcBindingInqAuthClientA(::std::mem::transmute(clientbinding), ::std::mem::transmute(privs), ::std::mem::transmute(serverprincname), ::std::mem::transmute(authnlevel), ::std::mem::transmute(authnsvc), ::std::mem::transmute(authzsvc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcBindingInqAuthClientExA(clientbinding: *const ::std::ffi::c_void, privs: *mut *mut ::std::ffi::c_void, serverprincname: *mut *mut u8, authnlevel: *mut u32, authnsvc: *mut u32, authzsvc: *mut u32, flags: u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingInqAuthClientExA(clientbinding: *const ::std::ffi::c_void, privs: *mut *mut ::std::ffi::c_void, serverprincname: *mut *mut u8, authnlevel: *mut u32, authnsvc: *mut u32, authzsvc: *mut u32, flags: u32) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcBindingInqAuthClientExA(::std::mem::transmute(clientbinding), ::std::mem::transmute(privs), ::std::mem::transmute(serverprincname), ::std::mem::transmute(authnlevel), ::std::mem::transmute(authnsvc), ::std::mem::transmute(authzsvc), ::std::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcBindingInqAuthClientExW(clientbinding: *const ::std::ffi::c_void, privs: *mut *mut ::std::ffi::c_void, serverprincname: *mut *mut u16, authnlevel: *mut u32, authnsvc: *mut u32, authzsvc: *mut u32, flags: u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingInqAuthClientExW(clientbinding: *const ::std::ffi::c_void, privs: *mut *mut ::std::ffi::c_void, serverprincname: *mut *mut u16, authnlevel: *mut u32, authnsvc: *mut u32, authzsvc: *mut u32, flags: u32) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcBindingInqAuthClientExW(::std::mem::transmute(clientbinding), ::std::mem::transmute(privs), ::std::mem::transmute(serverprincname), ::std::mem::transmute(authnlevel), ::std::mem::transmute(authnsvc), ::std::mem::transmute(authzsvc), ::std::mem::transmute(flags)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcBindingInqAuthClientW(clientbinding: *const ::std::ffi::c_void, privs: *mut *mut ::std::ffi::c_void, serverprincname: *mut *mut u16, authnlevel: *mut u32, authnsvc: *mut u32, authzsvc: *mut u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingInqAuthClientW(clientbinding: *const ::std::ffi::c_void, privs: *mut *mut ::std::ffi::c_void, serverprincname: *mut *mut u16, authnlevel: *mut u32, authnsvc: *mut u32, authzsvc: *mut u32) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcBindingInqAuthClientW(::std::mem::transmute(clientbinding), ::std::mem::transmute(privs), ::std::mem::transmute(serverprincname), ::std::mem::transmute(authnlevel), ::std::mem::transmute(authnsvc), ::std::mem::transmute(authzsvc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcBindingInqAuthInfoA(binding: *const ::std::ffi::c_void, serverprincname: *mut *mut u8, authnlevel: *mut u32, authnsvc: *mut u32, authidentity: *mut *mut ::std::ffi::c_void, authzsvc: *mut u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingInqAuthInfoA(binding: *const ::std::ffi::c_void, serverprincname: *mut *mut u8, authnlevel: *mut u32, authnsvc: *mut u32, authidentity: *mut *mut ::std::ffi::c_void, authzsvc: *mut u32) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcBindingInqAuthInfoA(::std::mem::transmute(binding), ::std::mem::transmute(serverprincname), ::std::mem::transmute(authnlevel), ::std::mem::transmute(authnsvc), ::std::mem::transmute(authidentity), ::std::mem::transmute(authzsvc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn RpcBindingInqAuthInfoExA(binding: *const ::std::ffi::c_void, serverprincname: *mut *mut u8, authnlevel: *mut u32, authnsvc: *mut u32, authidentity: *mut *mut ::std::ffi::c_void, authzsvc: *mut u32, rpcqosversion: u32, securityqos: *mut RPC_SECURITY_QOS) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingInqAuthInfoExA(binding: *const ::std::ffi::c_void, serverprincname: *mut *mut u8, authnlevel: *mut u32, authnsvc: *mut u32, authidentity: *mut *mut ::std::ffi::c_void, authzsvc: *mut u32, rpcqosversion: u32, securityqos: *mut RPC_SECURITY_QOS) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcBindingInqAuthInfoExA(
            ::std::mem::transmute(binding),
            ::std::mem::transmute(serverprincname),
            ::std::mem::transmute(authnlevel),
            ::std::mem::transmute(authnsvc),
            ::std::mem::transmute(authidentity),
            ::std::mem::transmute(authzsvc),
            ::std::mem::transmute(rpcqosversion),
            ::std::mem::transmute(securityqos),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn RpcBindingInqAuthInfoExW(binding: *const ::std::ffi::c_void, serverprincname: *mut *mut u16, authnlevel: *mut u32, authnsvc: *mut u32, authidentity: *mut *mut ::std::ffi::c_void, authzsvc: *mut u32, rpcqosversion: u32, securityqos: *mut RPC_SECURITY_QOS) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingInqAuthInfoExW(binding: *const ::std::ffi::c_void, serverprincname: *mut *mut u16, authnlevel: *mut u32, authnsvc: *mut u32, authidentity: *mut *mut ::std::ffi::c_void, authzsvc: *mut u32, rpcqosversion: u32, securityqos: *mut RPC_SECURITY_QOS) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcBindingInqAuthInfoExW(
            ::std::mem::transmute(binding),
            ::std::mem::transmute(serverprincname),
            ::std::mem::transmute(authnlevel),
            ::std::mem::transmute(authnsvc),
            ::std::mem::transmute(authidentity),
            ::std::mem::transmute(authzsvc),
            ::std::mem::transmute(rpcqosversion),
            ::std::mem::transmute(securityqos),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcBindingInqAuthInfoW(binding: *const ::std::ffi::c_void, serverprincname: *mut *mut u16, authnlevel: *mut u32, authnsvc: *mut u32, authidentity: *mut *mut ::std::ffi::c_void, authzsvc: *mut u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingInqAuthInfoW(binding: *const ::std::ffi::c_void, serverprincname: *mut *mut u16, authnlevel: *mut u32, authnsvc: *mut u32, authidentity: *mut *mut ::std::ffi::c_void, authzsvc: *mut u32) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcBindingInqAuthInfoW(::std::mem::transmute(binding), ::std::mem::transmute(serverprincname), ::std::mem::transmute(authnlevel), ::std::mem::transmute(authnsvc), ::std::mem::transmute(authidentity), ::std::mem::transmute(authzsvc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcBindingInqMaxCalls(binding: *const ::std::ffi::c_void, maxcalls: *mut u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingInqMaxCalls(binding: *const ::std::ffi::c_void, maxcalls: *mut u32) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcBindingInqMaxCalls(::std::mem::transmute(binding), ::std::mem::transmute(maxcalls)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcBindingInqObject(binding: *const ::std::ffi::c_void, objectuuid: *mut ::windows::runtime::GUID) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingInqObject(binding: *const ::std::ffi::c_void, objectuuid: *mut ::windows::runtime::GUID) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcBindingInqObject(::std::mem::transmute(binding), ::std::mem::transmute(objectuuid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcBindingInqOption(hbinding: *const ::std::ffi::c_void, option: u32, poptionvalue: *mut usize) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingInqOption(hbinding: *const ::std::ffi::c_void, option: u32, poptionvalue: *mut usize) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcBindingInqOption(::std::mem::transmute(hbinding), ::std::mem::transmute(option), ::std::mem::transmute(poptionvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcBindingReset(binding: *const ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingReset(binding: *const ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcBindingReset(::std::mem::transmute(binding)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcBindingServerFromClient(clientbinding: *const ::std::ffi::c_void, serverbinding: *mut *mut ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingServerFromClient(clientbinding: *const ::std::ffi::c_void, serverbinding: *mut *mut ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcBindingServerFromClient(::std::mem::transmute(clientbinding), ::std::mem::transmute(serverbinding)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcBindingSetAuthInfoA(binding: *const ::std::ffi::c_void, serverprincname: *const u8, authnlevel: u32, authnsvc: u32, authidentity: *const ::std::ffi::c_void, authzsvc: u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingSetAuthInfoA(binding: *const ::std::ffi::c_void, serverprincname: *const u8, authnlevel: u32, authnsvc: u32, authidentity: *const ::std::ffi::c_void, authzsvc: u32) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcBindingSetAuthInfoA(::std::mem::transmute(binding), ::std::mem::transmute(serverprincname), ::std::mem::transmute(authnlevel), ::std::mem::transmute(authnsvc), ::std::mem::transmute(authidentity), ::std::mem::transmute(authzsvc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn RpcBindingSetAuthInfoExA(binding: *const ::std::ffi::c_void, serverprincname: *const u8, authnlevel: u32, authnsvc: u32, authidentity: *const ::std::ffi::c_void, authzsvc: u32, securityqos: *const RPC_SECURITY_QOS) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingSetAuthInfoExA(binding: *const ::std::ffi::c_void, serverprincname: *const u8, authnlevel: u32, authnsvc: u32, authidentity: *const ::std::ffi::c_void, authzsvc: u32, securityqos: *const RPC_SECURITY_QOS) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcBindingSetAuthInfoExA(::std::mem::transmute(binding), ::std::mem::transmute(serverprincname), ::std::mem::transmute(authnlevel), ::std::mem::transmute(authnsvc), ::std::mem::transmute(authidentity), ::std::mem::transmute(authzsvc), ::std::mem::transmute(securityqos)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
#[inline]
pub unsafe fn RpcBindingSetAuthInfoExW(binding: *const ::std::ffi::c_void, serverprincname: *const u16, authnlevel: u32, authnsvc: u32, authidentity: *const ::std::ffi::c_void, authzsvc: u32, securityqos: *const RPC_SECURITY_QOS) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingSetAuthInfoExW(binding: *const ::std::ffi::c_void, serverprincname: *const u16, authnlevel: u32, authnsvc: u32, authidentity: *const ::std::ffi::c_void, authzsvc: u32, securityqos: *const RPC_SECURITY_QOS) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcBindingSetAuthInfoExW(::std::mem::transmute(binding), ::std::mem::transmute(serverprincname), ::std::mem::transmute(authnlevel), ::std::mem::transmute(authnsvc), ::std::mem::transmute(authidentity), ::std::mem::transmute(authzsvc), ::std::mem::transmute(securityqos)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcBindingSetAuthInfoW(binding: *const ::std::ffi::c_void, serverprincname: *const u16, authnlevel: u32, authnsvc: u32, authidentity: *const ::std::ffi::c_void, authzsvc: u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingSetAuthInfoW(binding: *const ::std::ffi::c_void, serverprincname: *const u16, authnlevel: u32, authnsvc: u32, authidentity: *const ::std::ffi::c_void, authzsvc: u32) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcBindingSetAuthInfoW(::std::mem::transmute(binding), ::std::mem::transmute(serverprincname), ::std::mem::transmute(authnlevel), ::std::mem::transmute(authnsvc), ::std::mem::transmute(authidentity), ::std::mem::transmute(authzsvc)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcBindingSetObject(binding: *const ::std::ffi::c_void, objectuuid: *const ::windows::runtime::GUID) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingSetObject(binding: *const ::std::ffi::c_void, objectuuid: *const ::windows::runtime::GUID) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcBindingSetObject(::std::mem::transmute(binding), ::std::mem::transmute(objectuuid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcBindingSetOption(hbinding: *const ::std::ffi::c_void, option: u32, optionvalue: usize) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingSetOption(hbinding: *const ::std::ffi::c_void, option: u32, optionvalue: usize) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcBindingSetOption(::std::mem::transmute(hbinding), ::std::mem::transmute(option), ::std::mem::transmute(optionvalue)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcBindingToStringBindingA(binding: *const ::std::ffi::c_void, stringbinding: *mut *mut u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingToStringBindingA(binding: *const ::std::ffi::c_void, stringbinding: *mut *mut u8) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcBindingToStringBindingA(::std::mem::transmute(binding), ::std::mem::transmute(stringbinding)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcBindingToStringBindingW(binding: *const ::std::ffi::c_void, stringbinding: *mut *mut u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingToStringBindingW(binding: *const ::std::ffi::c_void, stringbinding: *mut *mut u16) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcBindingToStringBindingW(::std::mem::transmute(binding), ::std::mem::transmute(stringbinding)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcBindingUnbind(binding: *const ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingUnbind(binding: *const ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcBindingUnbind(::std::mem::transmute(binding)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcBindingVectorFree(bindingvector: *mut *mut RPC_BINDING_VECTOR) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcBindingVectorFree(bindingvector: *mut *mut RPC_BINDING_VECTOR) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcBindingVectorFree(::std::mem::transmute(bindingvector)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RpcCallClientLocality(pub i32);
pub const rcclInvalid: RpcCallClientLocality = RpcCallClientLocality(0i32);
pub const rcclLocal: RpcCallClientLocality = RpcCallClientLocality(1i32);
pub const rcclRemote: RpcCallClientLocality = RpcCallClientLocality(2i32);
pub const rcclClientUnknownLocality: RpcCallClientLocality = RpcCallClientLocality(3i32);
impl ::std::convert::From<i32> for RpcCallClientLocality {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RpcCallClientLocality {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RpcCallType(pub i32);
pub const rctInvalid: RpcCallType = RpcCallType(0i32);
pub const rctNormal: RpcCallType = RpcCallType(1i32);
pub const rctTraining: RpcCallType = RpcCallType(2i32);
pub const rctGuaranteed: RpcCallType = RpcCallType(3i32);
impl ::std::convert::From<i32> for RpcCallType {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RpcCallType {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcCancelThread(thread: *const ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcCancelThread(thread: *const ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcCancelThread(::std::mem::transmute(thread)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcCancelThreadEx(thread: *const ::std::ffi::c_void, timeout: i32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcCancelThreadEx(thread: *const ::std::ffi::c_void, timeout: i32) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcCancelThreadEx(::std::mem::transmute(thread), ::std::mem::transmute(timeout)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn RpcCertGeneratePrincipalNameA(context: *const super::super::Security::Cryptography::CERT_CONTEXT, flags: u32, pbuffer: *mut *mut u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcCertGeneratePrincipalNameA(context: *const super::super::Security::Cryptography::CERT_CONTEXT, flags: u32, pbuffer: *mut *mut u8) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcCertGeneratePrincipalNameA(::std::mem::transmute(context), ::std::mem::transmute(flags), ::std::mem::transmute(pbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`, `Win32_Security_Cryptography`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_Cryptography"))]
#[inline]
pub unsafe fn RpcCertGeneratePrincipalNameW(context: *const super::super::Security::Cryptography::CERT_CONTEXT, flags: u32, pbuffer: *mut *mut u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcCertGeneratePrincipalNameW(context: *const super::super::Security::Cryptography::CERT_CONTEXT, flags: u32, pbuffer: *mut *mut u16) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcCertGeneratePrincipalNameW(::std::mem::transmute(context), ::std::mem::transmute(flags), ::std::mem::transmute(pbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcEpRegisterA(ifspec: *const ::std::ffi::c_void, bindingvector: *const RPC_BINDING_VECTOR, uuidvector: *const UUID_VECTOR, annotation: *const u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcEpRegisterA(ifspec: *const ::std::ffi::c_void, bindingvector: *const RPC_BINDING_VECTOR, uuidvector: *const UUID_VECTOR, annotation: *const u8) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcEpRegisterA(::std::mem::transmute(ifspec), ::std::mem::transmute(bindingvector), ::std::mem::transmute(uuidvector), ::std::mem::transmute(annotation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcEpRegisterNoReplaceA(ifspec: *const ::std::ffi::c_void, bindingvector: *const RPC_BINDING_VECTOR, uuidvector: *const UUID_VECTOR, annotation: *const u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcEpRegisterNoReplaceA(ifspec: *const ::std::ffi::c_void, bindingvector: *const RPC_BINDING_VECTOR, uuidvector: *const UUID_VECTOR, annotation: *const u8) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcEpRegisterNoReplaceA(::std::mem::transmute(ifspec), ::std::mem::transmute(bindingvector), ::std::mem::transmute(uuidvector), ::std::mem::transmute(annotation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcEpRegisterNoReplaceW(ifspec: *const ::std::ffi::c_void, bindingvector: *const RPC_BINDING_VECTOR, uuidvector: *const UUID_VECTOR, annotation: *const u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcEpRegisterNoReplaceW(ifspec: *const ::std::ffi::c_void, bindingvector: *const RPC_BINDING_VECTOR, uuidvector: *const UUID_VECTOR, annotation: *const u16) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcEpRegisterNoReplaceW(::std::mem::transmute(ifspec), ::std::mem::transmute(bindingvector), ::std::mem::transmute(uuidvector), ::std::mem::transmute(annotation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcEpRegisterW(ifspec: *const ::std::ffi::c_void, bindingvector: *const RPC_BINDING_VECTOR, uuidvector: *const UUID_VECTOR, annotation: *const u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcEpRegisterW(ifspec: *const ::std::ffi::c_void, bindingvector: *const RPC_BINDING_VECTOR, uuidvector: *const UUID_VECTOR, annotation: *const u16) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcEpRegisterW(::std::mem::transmute(ifspec), ::std::mem::transmute(bindingvector), ::std::mem::transmute(uuidvector), ::std::mem::transmute(annotation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcEpResolveBinding(binding: *const ::std::ffi::c_void, ifspec: *const ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcEpResolveBinding(binding: *const ::std::ffi::c_void, ifspec: *const ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcEpResolveBinding(::std::mem::transmute(binding), ::std::mem::transmute(ifspec)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcEpUnregister(ifspec: *const ::std::ffi::c_void, bindingvector: *const RPC_BINDING_VECTOR, uuidvector: *const UUID_VECTOR) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcEpUnregister(ifspec: *const ::std::ffi::c_void, bindingvector: *const RPC_BINDING_VECTOR, uuidvector: *const UUID_VECTOR) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcEpUnregister(::std::mem::transmute(ifspec), ::std::mem::transmute(bindingvector), ::std::mem::transmute(uuidvector)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RpcErrorAddRecord(errorinfo: *const RPC_EXTENDED_ERROR_INFO) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcErrorAddRecord(errorinfo: *const RPC_EXTENDED_ERROR_INFO) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcErrorAddRecord(::std::mem::transmute(errorinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcErrorClearInformation() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcErrorClearInformation();
        }
        ::std::mem::transmute(RpcErrorClearInformation())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcErrorEndEnumeration(enumhandle: *mut RPC_ERROR_ENUM_HANDLE) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcErrorEndEnumeration(enumhandle: *mut RPC_ERROR_ENUM_HANDLE) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcErrorEndEnumeration(::std::mem::transmute(enumhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RpcErrorGetNextRecord<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(enumhandle: *const RPC_ERROR_ENUM_HANDLE, copystrings: Param1, errorinfo: *mut RPC_EXTENDED_ERROR_INFO) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcErrorGetNextRecord(enumhandle: *const RPC_ERROR_ENUM_HANDLE, copystrings: super::super::Foundation::BOOL, errorinfo: *mut RPC_EXTENDED_ERROR_INFO) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcErrorGetNextRecord(::std::mem::transmute(enumhandle), copystrings.into_param().abi(), ::std::mem::transmute(errorinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcErrorGetNumberOfRecords(enumhandle: *const RPC_ERROR_ENUM_HANDLE, records: *mut i32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcErrorGetNumberOfRecords(enumhandle: *const RPC_ERROR_ENUM_HANDLE, records: *mut i32) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcErrorGetNumberOfRecords(::std::mem::transmute(enumhandle), ::std::mem::transmute(records)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcErrorLoadErrorInfo(errorblob: *const ::std::ffi::c_void, blobsize: usize, enumhandle: *mut RPC_ERROR_ENUM_HANDLE) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcErrorLoadErrorInfo(errorblob: *const ::std::ffi::c_void, blobsize: usize, enumhandle: *mut RPC_ERROR_ENUM_HANDLE) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcErrorLoadErrorInfo(::std::mem::transmute(errorblob), ::std::mem::transmute(blobsize), ::std::mem::transmute(enumhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcErrorResetEnumeration(enumhandle: *mut RPC_ERROR_ENUM_HANDLE) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcErrorResetEnumeration(enumhandle: *mut RPC_ERROR_ENUM_HANDLE) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcErrorResetEnumeration(::std::mem::transmute(enumhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcErrorSaveErrorInfo(enumhandle: *const RPC_ERROR_ENUM_HANDLE, errorblob: *mut *mut ::std::ffi::c_void, blobsize: *mut usize) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcErrorSaveErrorInfo(enumhandle: *const RPC_ERROR_ENUM_HANDLE, errorblob: *mut *mut ::std::ffi::c_void, blobsize: *mut usize) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcErrorSaveErrorInfo(::std::mem::transmute(enumhandle), ::std::mem::transmute(errorblob), ::std::mem::transmute(blobsize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcErrorStartEnumeration(enumhandle: *mut RPC_ERROR_ENUM_HANDLE) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcErrorStartEnumeration(enumhandle: *mut RPC_ERROR_ENUM_HANDLE) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcErrorStartEnumeration(::std::mem::transmute(enumhandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcExceptionFilter(exceptioncode: u32) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcExceptionFilter(exceptioncode: u32) -> i32;
        }
        ::std::mem::transmute(RpcExceptionFilter(::std::mem::transmute(exceptioncode)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcFreeAuthorizationContext(pauthzclientcontext: *mut *mut ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcFreeAuthorizationContext(pauthzclientcontext: *mut *mut ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcFreeAuthorizationContext(::std::mem::transmute(pauthzclientcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RpcGetAuthorizationContextForClient<'a, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param4: ::windows::runtime::IntoParam<'a, super::super::Foundation::LUID>>(clientbinding: *const ::std::ffi::c_void, impersonateonreturn: Param1, reserved1: *const ::std::ffi::c_void, pexpirationtime: *const i64, reserved2: Param4, reserved3: u32, reserved4: *const ::std::ffi::c_void, pauthzclientcontext: *mut *mut ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcGetAuthorizationContextForClient(clientbinding: *const ::std::ffi::c_void, impersonateonreturn: super::super::Foundation::BOOL, reserved1: *const ::std::ffi::c_void, pexpirationtime: *const i64, reserved2: super::super::Foundation::LUID, reserved3: u32, reserved4: *const ::std::ffi::c_void, pauthzclientcontext: *mut *mut ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcGetAuthorizationContextForClient(
            ::std::mem::transmute(clientbinding),
            impersonateonreturn.into_param().abi(),
            ::std::mem::transmute(reserved1),
            ::std::mem::transmute(pexpirationtime),
            reserved2.into_param().abi(),
            ::std::mem::transmute(reserved3),
            ::std::mem::transmute(reserved4),
            ::std::mem::transmute(pauthzclientcontext),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcIfIdVectorFree(ifidvector: *mut *mut RPC_IF_ID_VECTOR) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcIfIdVectorFree(ifidvector: *mut *mut RPC_IF_ID_VECTOR) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcIfIdVectorFree(::std::mem::transmute(ifidvector)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcIfInqId(rpcifhandle: *const ::std::ffi::c_void, rpcifid: *mut RPC_IF_ID) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcIfInqId(rpcifhandle: *const ::std::ffi::c_void, rpcifid: *mut RPC_IF_ID) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcIfInqId(::std::mem::transmute(rpcifhandle), ::std::mem::transmute(rpcifid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcImpersonateClient(bindinghandle: *const ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcImpersonateClient(bindinghandle: *const ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcImpersonateClient(::std::mem::transmute(bindinghandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcImpersonateClient2(bindinghandle: *const ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcImpersonateClient2(bindinghandle: *const ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcImpersonateClient2(::std::mem::transmute(bindinghandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcImpersonateClientContainer(bindinghandle: *const ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcImpersonateClientContainer(bindinghandle: *const ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcImpersonateClientContainer(::std::mem::transmute(bindinghandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RpcLocalAddressFormat(pub i32);
pub const rlafInvalid: RpcLocalAddressFormat = RpcLocalAddressFormat(0i32);
pub const rlafIPv4: RpcLocalAddressFormat = RpcLocalAddressFormat(1i32);
pub const rlafIPv6: RpcLocalAddressFormat = RpcLocalAddressFormat(2i32);
impl ::std::convert::From<i32> for RpcLocalAddressFormat {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RpcLocalAddressFormat {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcMgmtEnableIdleCleanup() -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcMgmtEnableIdleCleanup() -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcMgmtEnableIdleCleanup())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcMgmtEpEltInqBegin(epbinding: *const ::std::ffi::c_void, inquirytype: u32, ifid: *const RPC_IF_ID, versoption: u32, objectuuid: *const ::windows::runtime::GUID, inquirycontext: *mut *mut *mut ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcMgmtEpEltInqBegin(epbinding: *const ::std::ffi::c_void, inquirytype: u32, ifid: *const RPC_IF_ID, versoption: u32, objectuuid: *const ::windows::runtime::GUID, inquirycontext: *mut *mut *mut ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcMgmtEpEltInqBegin(::std::mem::transmute(epbinding), ::std::mem::transmute(inquirytype), ::std::mem::transmute(ifid), ::std::mem::transmute(versoption), ::std::mem::transmute(objectuuid), ::std::mem::transmute(inquirycontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcMgmtEpEltInqDone(inquirycontext: *mut *mut *mut ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcMgmtEpEltInqDone(inquirycontext: *mut *mut *mut ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcMgmtEpEltInqDone(::std::mem::transmute(inquirycontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcMgmtEpEltInqNextA(inquirycontext: *const *const ::std::ffi::c_void, ifid: *mut RPC_IF_ID, binding: *mut *mut ::std::ffi::c_void, objectuuid: *mut ::windows::runtime::GUID, annotation: *mut *mut u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcMgmtEpEltInqNextA(inquirycontext: *const *const ::std::ffi::c_void, ifid: *mut RPC_IF_ID, binding: *mut *mut ::std::ffi::c_void, objectuuid: *mut ::windows::runtime::GUID, annotation: *mut *mut u8) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcMgmtEpEltInqNextA(::std::mem::transmute(inquirycontext), ::std::mem::transmute(ifid), ::std::mem::transmute(binding), ::std::mem::transmute(objectuuid), ::std::mem::transmute(annotation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcMgmtEpEltInqNextW(inquirycontext: *const *const ::std::ffi::c_void, ifid: *mut RPC_IF_ID, binding: *mut *mut ::std::ffi::c_void, objectuuid: *mut ::windows::runtime::GUID, annotation: *mut *mut u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcMgmtEpEltInqNextW(inquirycontext: *const *const ::std::ffi::c_void, ifid: *mut RPC_IF_ID, binding: *mut *mut ::std::ffi::c_void, objectuuid: *mut ::windows::runtime::GUID, annotation: *mut *mut u16) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcMgmtEpEltInqNextW(::std::mem::transmute(inquirycontext), ::std::mem::transmute(ifid), ::std::mem::transmute(binding), ::std::mem::transmute(objectuuid), ::std::mem::transmute(annotation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcMgmtEpUnregister(epbinding: *const ::std::ffi::c_void, ifid: *const RPC_IF_ID, binding: *const ::std::ffi::c_void, objectuuid: *const ::windows::runtime::GUID) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcMgmtEpUnregister(epbinding: *const ::std::ffi::c_void, ifid: *const RPC_IF_ID, binding: *const ::std::ffi::c_void, objectuuid: *const ::windows::runtime::GUID) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcMgmtEpUnregister(::std::mem::transmute(epbinding), ::std::mem::transmute(ifid), ::std::mem::transmute(binding), ::std::mem::transmute(objectuuid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcMgmtInqComTimeout(binding: *const ::std::ffi::c_void, timeout: *mut u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcMgmtInqComTimeout(binding: *const ::std::ffi::c_void, timeout: *mut u32) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcMgmtInqComTimeout(::std::mem::transmute(binding), ::std::mem::transmute(timeout)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcMgmtInqDefaultProtectLevel(authnsvc: u32, authnlevel: *mut u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcMgmtInqDefaultProtectLevel(authnsvc: u32, authnlevel: *mut u32) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcMgmtInqDefaultProtectLevel(::std::mem::transmute(authnsvc), ::std::mem::transmute(authnlevel)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcMgmtInqIfIds(binding: *const ::std::ffi::c_void, ifidvector: *mut *mut RPC_IF_ID_VECTOR) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcMgmtInqIfIds(binding: *const ::std::ffi::c_void, ifidvector: *mut *mut RPC_IF_ID_VECTOR) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcMgmtInqIfIds(::std::mem::transmute(binding), ::std::mem::transmute(ifidvector)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcMgmtInqServerPrincNameA(binding: *const ::std::ffi::c_void, authnsvc: u32, serverprincname: *mut *mut u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcMgmtInqServerPrincNameA(binding: *const ::std::ffi::c_void, authnsvc: u32, serverprincname: *mut *mut u8) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcMgmtInqServerPrincNameA(::std::mem::transmute(binding), ::std::mem::transmute(authnsvc), ::std::mem::transmute(serverprincname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcMgmtInqServerPrincNameW(binding: *const ::std::ffi::c_void, authnsvc: u32, serverprincname: *mut *mut u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcMgmtInqServerPrincNameW(binding: *const ::std::ffi::c_void, authnsvc: u32, serverprincname: *mut *mut u16) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcMgmtInqServerPrincNameW(::std::mem::transmute(binding), ::std::mem::transmute(authnsvc), ::std::mem::transmute(serverprincname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcMgmtInqStats(binding: *const ::std::ffi::c_void, statistics: *mut *mut RPC_STATS_VECTOR) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcMgmtInqStats(binding: *const ::std::ffi::c_void, statistics: *mut *mut RPC_STATS_VECTOR) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcMgmtInqStats(::std::mem::transmute(binding), ::std::mem::transmute(statistics)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcMgmtIsServerListening(binding: *const ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcMgmtIsServerListening(binding: *const ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcMgmtIsServerListening(::std::mem::transmute(binding)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcMgmtSetAuthorizationFn(authorizationfn: ::std::option::Option<RPC_MGMT_AUTHORIZATION_FN>) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcMgmtSetAuthorizationFn(authorizationfn: ::windows::runtime::RawPtr) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcMgmtSetAuthorizationFn(::std::mem::transmute(authorizationfn)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcMgmtSetCancelTimeout(timeout: i32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcMgmtSetCancelTimeout(timeout: i32) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcMgmtSetCancelTimeout(::std::mem::transmute(timeout)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcMgmtSetComTimeout(binding: *const ::std::ffi::c_void, timeout: u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcMgmtSetComTimeout(binding: *const ::std::ffi::c_void, timeout: u32) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcMgmtSetComTimeout(::std::mem::transmute(binding), ::std::mem::transmute(timeout)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcMgmtSetServerStackSize(threadstacksize: u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcMgmtSetServerStackSize(threadstacksize: u32) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcMgmtSetServerStackSize(::std::mem::transmute(threadstacksize)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcMgmtStatsVectorFree(statsvector: *mut *mut RPC_STATS_VECTOR) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcMgmtStatsVectorFree(statsvector: *mut *mut RPC_STATS_VECTOR) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcMgmtStatsVectorFree(::std::mem::transmute(statsvector)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcMgmtStopServerListening(binding: *const ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcMgmtStopServerListening(binding: *const ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcMgmtStopServerListening(::std::mem::transmute(binding)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcMgmtWaitServerListen() -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcMgmtWaitServerListen() -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcMgmtWaitServerListen())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNetworkInqProtseqsA(protseqvector: *mut *mut RPC_PROTSEQ_VECTORA) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNetworkInqProtseqsA(protseqvector: *mut *mut RPC_PROTSEQ_VECTORA) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNetworkInqProtseqsA(::std::mem::transmute(protseqvector)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNetworkInqProtseqsW(protseqvector: *mut *mut RPC_PROTSEQ_VECTORW) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNetworkInqProtseqsW(protseqvector: *mut *mut RPC_PROTSEQ_VECTORW) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNetworkInqProtseqsW(::std::mem::transmute(protseqvector)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNetworkIsProtseqValidA(protseq: *const u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNetworkIsProtseqValidA(protseq: *const u8) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNetworkIsProtseqValidA(::std::mem::transmute(protseq)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNetworkIsProtseqValidW(protseq: *const u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNetworkIsProtseqValidW(protseq: *const u16) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNetworkIsProtseqValidW(::std::mem::transmute(protseq)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsBindingExportA(entrynamesyntax: u32, entryname: *const u8, ifspec: *const ::std::ffi::c_void, bindingvec: *const RPC_BINDING_VECTOR, objectuuidvec: *const UUID_VECTOR) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsBindingExportA(entrynamesyntax: u32, entryname: *const u8, ifspec: *const ::std::ffi::c_void, bindingvec: *const RPC_BINDING_VECTOR, objectuuidvec: *const UUID_VECTOR) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsBindingExportA(::std::mem::transmute(entrynamesyntax), ::std::mem::transmute(entryname), ::std::mem::transmute(ifspec), ::std::mem::transmute(bindingvec), ::std::mem::transmute(objectuuidvec)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsBindingExportPnPA(entrynamesyntax: u32, entryname: *const u8, ifspec: *const ::std::ffi::c_void, objectvector: *const UUID_VECTOR) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsBindingExportPnPA(entrynamesyntax: u32, entryname: *const u8, ifspec: *const ::std::ffi::c_void, objectvector: *const UUID_VECTOR) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsBindingExportPnPA(::std::mem::transmute(entrynamesyntax), ::std::mem::transmute(entryname), ::std::mem::transmute(ifspec), ::std::mem::transmute(objectvector)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsBindingExportPnPW(entrynamesyntax: u32, entryname: *const u16, ifspec: *const ::std::ffi::c_void, objectvector: *const UUID_VECTOR) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsBindingExportPnPW(entrynamesyntax: u32, entryname: *const u16, ifspec: *const ::std::ffi::c_void, objectvector: *const UUID_VECTOR) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsBindingExportPnPW(::std::mem::transmute(entrynamesyntax), ::std::mem::transmute(entryname), ::std::mem::transmute(ifspec), ::std::mem::transmute(objectvector)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsBindingExportW(entrynamesyntax: u32, entryname: *const u16, ifspec: *const ::std::ffi::c_void, bindingvec: *const RPC_BINDING_VECTOR, objectuuidvec: *const UUID_VECTOR) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsBindingExportW(entrynamesyntax: u32, entryname: *const u16, ifspec: *const ::std::ffi::c_void, bindingvec: *const RPC_BINDING_VECTOR, objectuuidvec: *const UUID_VECTOR) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsBindingExportW(::std::mem::transmute(entrynamesyntax), ::std::mem::transmute(entryname), ::std::mem::transmute(ifspec), ::std::mem::transmute(bindingvec), ::std::mem::transmute(objectuuidvec)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsBindingImportBeginA(entrynamesyntax: u32, entryname: *const u8, ifspec: *const ::std::ffi::c_void, objuuid: *const ::windows::runtime::GUID, importcontext: *mut *mut ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsBindingImportBeginA(entrynamesyntax: u32, entryname: *const u8, ifspec: *const ::std::ffi::c_void, objuuid: *const ::windows::runtime::GUID, importcontext: *mut *mut ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsBindingImportBeginA(::std::mem::transmute(entrynamesyntax), ::std::mem::transmute(entryname), ::std::mem::transmute(ifspec), ::std::mem::transmute(objuuid), ::std::mem::transmute(importcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsBindingImportBeginW(entrynamesyntax: u32, entryname: *const u16, ifspec: *const ::std::ffi::c_void, objuuid: *const ::windows::runtime::GUID, importcontext: *mut *mut ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsBindingImportBeginW(entrynamesyntax: u32, entryname: *const u16, ifspec: *const ::std::ffi::c_void, objuuid: *const ::windows::runtime::GUID, importcontext: *mut *mut ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsBindingImportBeginW(::std::mem::transmute(entrynamesyntax), ::std::mem::transmute(entryname), ::std::mem::transmute(ifspec), ::std::mem::transmute(objuuid), ::std::mem::transmute(importcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsBindingImportDone(importcontext: *mut *mut ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsBindingImportDone(importcontext: *mut *mut ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsBindingImportDone(::std::mem::transmute(importcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsBindingImportNext(importcontext: *mut ::std::ffi::c_void, binding: *mut *mut ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsBindingImportNext(importcontext: *mut ::std::ffi::c_void, binding: *mut *mut ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsBindingImportNext(::std::mem::transmute(importcontext), ::std::mem::transmute(binding)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsBindingInqEntryNameA(binding: *const ::std::ffi::c_void, entrynamesyntax: u32, entryname: *mut *mut u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsBindingInqEntryNameA(binding: *const ::std::ffi::c_void, entrynamesyntax: u32, entryname: *mut *mut u8) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsBindingInqEntryNameA(::std::mem::transmute(binding), ::std::mem::transmute(entrynamesyntax), ::std::mem::transmute(entryname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsBindingInqEntryNameW(binding: *const ::std::ffi::c_void, entrynamesyntax: u32, entryname: *mut *mut u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsBindingInqEntryNameW(binding: *const ::std::ffi::c_void, entrynamesyntax: u32, entryname: *mut *mut u16) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsBindingInqEntryNameW(::std::mem::transmute(binding), ::std::mem::transmute(entrynamesyntax), ::std::mem::transmute(entryname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsBindingLookupBeginA(entrynamesyntax: u32, entryname: *const u8, ifspec: *const ::std::ffi::c_void, objuuid: *const ::windows::runtime::GUID, bindingmaxcount: u32, lookupcontext: *mut *mut ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsBindingLookupBeginA(entrynamesyntax: u32, entryname: *const u8, ifspec: *const ::std::ffi::c_void, objuuid: *const ::windows::runtime::GUID, bindingmaxcount: u32, lookupcontext: *mut *mut ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsBindingLookupBeginA(::std::mem::transmute(entrynamesyntax), ::std::mem::transmute(entryname), ::std::mem::transmute(ifspec), ::std::mem::transmute(objuuid), ::std::mem::transmute(bindingmaxcount), ::std::mem::transmute(lookupcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsBindingLookupBeginW(entrynamesyntax: u32, entryname: *const u16, ifspec: *const ::std::ffi::c_void, objuuid: *const ::windows::runtime::GUID, bindingmaxcount: u32, lookupcontext: *mut *mut ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsBindingLookupBeginW(entrynamesyntax: u32, entryname: *const u16, ifspec: *const ::std::ffi::c_void, objuuid: *const ::windows::runtime::GUID, bindingmaxcount: u32, lookupcontext: *mut *mut ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsBindingLookupBeginW(::std::mem::transmute(entrynamesyntax), ::std::mem::transmute(entryname), ::std::mem::transmute(ifspec), ::std::mem::transmute(objuuid), ::std::mem::transmute(bindingmaxcount), ::std::mem::transmute(lookupcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsBindingLookupDone(lookupcontext: *mut *mut ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsBindingLookupDone(lookupcontext: *mut *mut ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsBindingLookupDone(::std::mem::transmute(lookupcontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsBindingLookupNext(lookupcontext: *mut ::std::ffi::c_void, bindingvec: *mut *mut RPC_BINDING_VECTOR) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsBindingLookupNext(lookupcontext: *mut ::std::ffi::c_void, bindingvec: *mut *mut RPC_BINDING_VECTOR) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsBindingLookupNext(::std::mem::transmute(lookupcontext), ::std::mem::transmute(bindingvec)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsBindingSelect(bindingvec: *mut RPC_BINDING_VECTOR, binding: *mut *mut ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsBindingSelect(bindingvec: *mut RPC_BINDING_VECTOR, binding: *mut *mut ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsBindingSelect(::std::mem::transmute(bindingvec), ::std::mem::transmute(binding)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsBindingUnexportA(entrynamesyntax: u32, entryname: *const u8, ifspec: *const ::std::ffi::c_void, objectuuidvec: *const UUID_VECTOR) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsBindingUnexportA(entrynamesyntax: u32, entryname: *const u8, ifspec: *const ::std::ffi::c_void, objectuuidvec: *const UUID_VECTOR) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsBindingUnexportA(::std::mem::transmute(entrynamesyntax), ::std::mem::transmute(entryname), ::std::mem::transmute(ifspec), ::std::mem::transmute(objectuuidvec)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsBindingUnexportPnPA(entrynamesyntax: u32, entryname: *const u8, ifspec: *const ::std::ffi::c_void, objectvector: *const UUID_VECTOR) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsBindingUnexportPnPA(entrynamesyntax: u32, entryname: *const u8, ifspec: *const ::std::ffi::c_void, objectvector: *const UUID_VECTOR) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsBindingUnexportPnPA(::std::mem::transmute(entrynamesyntax), ::std::mem::transmute(entryname), ::std::mem::transmute(ifspec), ::std::mem::transmute(objectvector)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsBindingUnexportPnPW(entrynamesyntax: u32, entryname: *const u16, ifspec: *const ::std::ffi::c_void, objectvector: *const UUID_VECTOR) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsBindingUnexportPnPW(entrynamesyntax: u32, entryname: *const u16, ifspec: *const ::std::ffi::c_void, objectvector: *const UUID_VECTOR) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsBindingUnexportPnPW(::std::mem::transmute(entrynamesyntax), ::std::mem::transmute(entryname), ::std::mem::transmute(ifspec), ::std::mem::transmute(objectvector)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsBindingUnexportW(entrynamesyntax: u32, entryname: *const u16, ifspec: *const ::std::ffi::c_void, objectuuidvec: *const UUID_VECTOR) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsBindingUnexportW(entrynamesyntax: u32, entryname: *const u16, ifspec: *const ::std::ffi::c_void, objectuuidvec: *const UUID_VECTOR) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsBindingUnexportW(::std::mem::transmute(entrynamesyntax), ::std::mem::transmute(entryname), ::std::mem::transmute(ifspec), ::std::mem::transmute(objectuuidvec)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsEntryExpandNameA(entrynamesyntax: u32, entryname: *const u8, expandedname: *mut *mut u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsEntryExpandNameA(entrynamesyntax: u32, entryname: *const u8, expandedname: *mut *mut u8) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsEntryExpandNameA(::std::mem::transmute(entrynamesyntax), ::std::mem::transmute(entryname), ::std::mem::transmute(expandedname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsEntryExpandNameW(entrynamesyntax: u32, entryname: *const u16, expandedname: *mut *mut u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsEntryExpandNameW(entrynamesyntax: u32, entryname: *const u16, expandedname: *mut *mut u16) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsEntryExpandNameW(::std::mem::transmute(entrynamesyntax), ::std::mem::transmute(entryname), ::std::mem::transmute(expandedname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsEntryObjectInqBeginA(entrynamesyntax: u32, entryname: *const u8, inquirycontext: *mut *mut ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsEntryObjectInqBeginA(entrynamesyntax: u32, entryname: *const u8, inquirycontext: *mut *mut ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsEntryObjectInqBeginA(::std::mem::transmute(entrynamesyntax), ::std::mem::transmute(entryname), ::std::mem::transmute(inquirycontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsEntryObjectInqBeginW(entrynamesyntax: u32, entryname: *const u16, inquirycontext: *mut *mut ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsEntryObjectInqBeginW(entrynamesyntax: u32, entryname: *const u16, inquirycontext: *mut *mut ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsEntryObjectInqBeginW(::std::mem::transmute(entrynamesyntax), ::std::mem::transmute(entryname), ::std::mem::transmute(inquirycontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsEntryObjectInqDone(inquirycontext: *mut *mut ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsEntryObjectInqDone(inquirycontext: *mut *mut ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsEntryObjectInqDone(::std::mem::transmute(inquirycontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsEntryObjectInqNext(inquirycontext: *mut ::std::ffi::c_void, objuuid: *mut ::windows::runtime::GUID) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsEntryObjectInqNext(inquirycontext: *mut ::std::ffi::c_void, objuuid: *mut ::windows::runtime::GUID) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsEntryObjectInqNext(::std::mem::transmute(inquirycontext), ::std::mem::transmute(objuuid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsGroupDeleteA(groupnamesyntax: GROUP_NAME_SYNTAX, groupname: *const u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsGroupDeleteA(groupnamesyntax: GROUP_NAME_SYNTAX, groupname: *const u8) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsGroupDeleteA(::std::mem::transmute(groupnamesyntax), ::std::mem::transmute(groupname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsGroupDeleteW(groupnamesyntax: GROUP_NAME_SYNTAX, groupname: *const u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsGroupDeleteW(groupnamesyntax: GROUP_NAME_SYNTAX, groupname: *const u16) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsGroupDeleteW(::std::mem::transmute(groupnamesyntax), ::std::mem::transmute(groupname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsGroupMbrAddA(groupnamesyntax: u32, groupname: *const u8, membernamesyntax: u32, membername: *const u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsGroupMbrAddA(groupnamesyntax: u32, groupname: *const u8, membernamesyntax: u32, membername: *const u8) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsGroupMbrAddA(::std::mem::transmute(groupnamesyntax), ::std::mem::transmute(groupname), ::std::mem::transmute(membernamesyntax), ::std::mem::transmute(membername)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsGroupMbrAddW(groupnamesyntax: u32, groupname: *const u16, membernamesyntax: u32, membername: *const u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsGroupMbrAddW(groupnamesyntax: u32, groupname: *const u16, membernamesyntax: u32, membername: *const u16) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsGroupMbrAddW(::std::mem::transmute(groupnamesyntax), ::std::mem::transmute(groupname), ::std::mem::transmute(membernamesyntax), ::std::mem::transmute(membername)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsGroupMbrInqBeginA(groupnamesyntax: u32, groupname: *const u8, membernamesyntax: u32, inquirycontext: *mut *mut ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsGroupMbrInqBeginA(groupnamesyntax: u32, groupname: *const u8, membernamesyntax: u32, inquirycontext: *mut *mut ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsGroupMbrInqBeginA(::std::mem::transmute(groupnamesyntax), ::std::mem::transmute(groupname), ::std::mem::transmute(membernamesyntax), ::std::mem::transmute(inquirycontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsGroupMbrInqBeginW(groupnamesyntax: u32, groupname: *const u16, membernamesyntax: u32, inquirycontext: *mut *mut ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsGroupMbrInqBeginW(groupnamesyntax: u32, groupname: *const u16, membernamesyntax: u32, inquirycontext: *mut *mut ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsGroupMbrInqBeginW(::std::mem::transmute(groupnamesyntax), ::std::mem::transmute(groupname), ::std::mem::transmute(membernamesyntax), ::std::mem::transmute(inquirycontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsGroupMbrInqDone(inquirycontext: *mut *mut ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsGroupMbrInqDone(inquirycontext: *mut *mut ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsGroupMbrInqDone(::std::mem::transmute(inquirycontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsGroupMbrInqNextA(inquirycontext: *mut ::std::ffi::c_void, membername: *mut *mut u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsGroupMbrInqNextA(inquirycontext: *mut ::std::ffi::c_void, membername: *mut *mut u8) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsGroupMbrInqNextA(::std::mem::transmute(inquirycontext), ::std::mem::transmute(membername)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsGroupMbrInqNextW(inquirycontext: *mut ::std::ffi::c_void, membername: *mut *mut u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsGroupMbrInqNextW(inquirycontext: *mut ::std::ffi::c_void, membername: *mut *mut u16) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsGroupMbrInqNextW(::std::mem::transmute(inquirycontext), ::std::mem::transmute(membername)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsGroupMbrRemoveA(groupnamesyntax: u32, groupname: *const u8, membernamesyntax: u32, membername: *const u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsGroupMbrRemoveA(groupnamesyntax: u32, groupname: *const u8, membernamesyntax: u32, membername: *const u8) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsGroupMbrRemoveA(::std::mem::transmute(groupnamesyntax), ::std::mem::transmute(groupname), ::std::mem::transmute(membernamesyntax), ::std::mem::transmute(membername)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsGroupMbrRemoveW(groupnamesyntax: u32, groupname: *const u16, membernamesyntax: u32, membername: *const u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsGroupMbrRemoveW(groupnamesyntax: u32, groupname: *const u16, membernamesyntax: u32, membername: *const u16) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsGroupMbrRemoveW(::std::mem::transmute(groupnamesyntax), ::std::mem::transmute(groupname), ::std::mem::transmute(membernamesyntax), ::std::mem::transmute(membername)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsMgmtBindingUnexportA(entrynamesyntax: u32, entryname: *const u8, ifid: *const RPC_IF_ID, versoption: u32, objectuuidvec: *const UUID_VECTOR) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsMgmtBindingUnexportA(entrynamesyntax: u32, entryname: *const u8, ifid: *const RPC_IF_ID, versoption: u32, objectuuidvec: *const UUID_VECTOR) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsMgmtBindingUnexportA(::std::mem::transmute(entrynamesyntax), ::std::mem::transmute(entryname), ::std::mem::transmute(ifid), ::std::mem::transmute(versoption), ::std::mem::transmute(objectuuidvec)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsMgmtBindingUnexportW(entrynamesyntax: u32, entryname: *const u16, ifid: *const RPC_IF_ID, versoption: u32, objectuuidvec: *const UUID_VECTOR) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsMgmtBindingUnexportW(entrynamesyntax: u32, entryname: *const u16, ifid: *const RPC_IF_ID, versoption: u32, objectuuidvec: *const UUID_VECTOR) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsMgmtBindingUnexportW(::std::mem::transmute(entrynamesyntax), ::std::mem::transmute(entryname), ::std::mem::transmute(ifid), ::std::mem::transmute(versoption), ::std::mem::transmute(objectuuidvec)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsMgmtEntryCreateA(entrynamesyntax: u32, entryname: *const u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsMgmtEntryCreateA(entrynamesyntax: u32, entryname: *const u8) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsMgmtEntryCreateA(::std::mem::transmute(entrynamesyntax), ::std::mem::transmute(entryname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsMgmtEntryCreateW(entrynamesyntax: u32, entryname: *const u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsMgmtEntryCreateW(entrynamesyntax: u32, entryname: *const u16) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsMgmtEntryCreateW(::std::mem::transmute(entrynamesyntax), ::std::mem::transmute(entryname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsMgmtEntryDeleteA(entrynamesyntax: u32, entryname: *const u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsMgmtEntryDeleteA(entrynamesyntax: u32, entryname: *const u8) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsMgmtEntryDeleteA(::std::mem::transmute(entrynamesyntax), ::std::mem::transmute(entryname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsMgmtEntryDeleteW(entrynamesyntax: u32, entryname: *const u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsMgmtEntryDeleteW(entrynamesyntax: u32, entryname: *const u16) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsMgmtEntryDeleteW(::std::mem::transmute(entrynamesyntax), ::std::mem::transmute(entryname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsMgmtEntryInqIfIdsA(entrynamesyntax: u32, entryname: *const u8, ifidvec: *mut *mut RPC_IF_ID_VECTOR) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsMgmtEntryInqIfIdsA(entrynamesyntax: u32, entryname: *const u8, ifidvec: *mut *mut RPC_IF_ID_VECTOR) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsMgmtEntryInqIfIdsA(::std::mem::transmute(entrynamesyntax), ::std::mem::transmute(entryname), ::std::mem::transmute(ifidvec)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsMgmtEntryInqIfIdsW(entrynamesyntax: u32, entryname: *const u16, ifidvec: *mut *mut RPC_IF_ID_VECTOR) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsMgmtEntryInqIfIdsW(entrynamesyntax: u32, entryname: *const u16, ifidvec: *mut *mut RPC_IF_ID_VECTOR) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsMgmtEntryInqIfIdsW(::std::mem::transmute(entrynamesyntax), ::std::mem::transmute(entryname), ::std::mem::transmute(ifidvec)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsMgmtHandleSetExpAge(nshandle: *mut ::std::ffi::c_void, expirationage: u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsMgmtHandleSetExpAge(nshandle: *mut ::std::ffi::c_void, expirationage: u32) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsMgmtHandleSetExpAge(::std::mem::transmute(nshandle), ::std::mem::transmute(expirationage)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsMgmtInqExpAge(expirationage: *mut u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsMgmtInqExpAge(expirationage: *mut u32) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsMgmtInqExpAge(::std::mem::transmute(expirationage)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsMgmtSetExpAge(expirationage: u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsMgmtSetExpAge(expirationage: u32) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsMgmtSetExpAge(::std::mem::transmute(expirationage)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsProfileDeleteA(profilenamesyntax: u32, profilename: *const u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsProfileDeleteA(profilenamesyntax: u32, profilename: *const u8) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsProfileDeleteA(::std::mem::transmute(profilenamesyntax), ::std::mem::transmute(profilename)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsProfileDeleteW(profilenamesyntax: u32, profilename: *const u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsProfileDeleteW(profilenamesyntax: u32, profilename: *const u16) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsProfileDeleteW(::std::mem::transmute(profilenamesyntax), ::std::mem::transmute(profilename)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsProfileEltAddA(profilenamesyntax: u32, profilename: *const u8, ifid: *const RPC_IF_ID, membernamesyntax: u32, membername: *const u8, priority: u32, annotation: *const u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsProfileEltAddA(profilenamesyntax: u32, profilename: *const u8, ifid: *const RPC_IF_ID, membernamesyntax: u32, membername: *const u8, priority: u32, annotation: *const u8) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsProfileEltAddA(::std::mem::transmute(profilenamesyntax), ::std::mem::transmute(profilename), ::std::mem::transmute(ifid), ::std::mem::transmute(membernamesyntax), ::std::mem::transmute(membername), ::std::mem::transmute(priority), ::std::mem::transmute(annotation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsProfileEltAddW(profilenamesyntax: u32, profilename: *const u16, ifid: *const RPC_IF_ID, membernamesyntax: u32, membername: *const u16, priority: u32, annotation: *const u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsProfileEltAddW(profilenamesyntax: u32, profilename: *const u16, ifid: *const RPC_IF_ID, membernamesyntax: u32, membername: *const u16, priority: u32, annotation: *const u16) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsProfileEltAddW(::std::mem::transmute(profilenamesyntax), ::std::mem::transmute(profilename), ::std::mem::transmute(ifid), ::std::mem::transmute(membernamesyntax), ::std::mem::transmute(membername), ::std::mem::transmute(priority), ::std::mem::transmute(annotation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsProfileEltInqBeginA(profilenamesyntax: u32, profilename: *const u8, inquirytype: u32, ifid: *const RPC_IF_ID, versoption: u32, membernamesyntax: u32, membername: *const u8, inquirycontext: *mut *mut ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsProfileEltInqBeginA(profilenamesyntax: u32, profilename: *const u8, inquirytype: u32, ifid: *const RPC_IF_ID, versoption: u32, membernamesyntax: u32, membername: *const u8, inquirycontext: *mut *mut ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsProfileEltInqBeginA(
            ::std::mem::transmute(profilenamesyntax),
            ::std::mem::transmute(profilename),
            ::std::mem::transmute(inquirytype),
            ::std::mem::transmute(ifid),
            ::std::mem::transmute(versoption),
            ::std::mem::transmute(membernamesyntax),
            ::std::mem::transmute(membername),
            ::std::mem::transmute(inquirycontext),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsProfileEltInqBeginW(profilenamesyntax: u32, profilename: *const u16, inquirytype: u32, ifid: *const RPC_IF_ID, versoption: u32, membernamesyntax: u32, membername: *const u16, inquirycontext: *mut *mut ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsProfileEltInqBeginW(profilenamesyntax: u32, profilename: *const u16, inquirytype: u32, ifid: *const RPC_IF_ID, versoption: u32, membernamesyntax: u32, membername: *const u16, inquirycontext: *mut *mut ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsProfileEltInqBeginW(
            ::std::mem::transmute(profilenamesyntax),
            ::std::mem::transmute(profilename),
            ::std::mem::transmute(inquirytype),
            ::std::mem::transmute(ifid),
            ::std::mem::transmute(versoption),
            ::std::mem::transmute(membernamesyntax),
            ::std::mem::transmute(membername),
            ::std::mem::transmute(inquirycontext),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsProfileEltInqDone(inquirycontext: *mut *mut ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsProfileEltInqDone(inquirycontext: *mut *mut ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsProfileEltInqDone(::std::mem::transmute(inquirycontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsProfileEltInqNextA(inquirycontext: *const ::std::ffi::c_void, ifid: *mut RPC_IF_ID, membername: *mut *mut u8, priority: *mut u32, annotation: *mut *mut u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsProfileEltInqNextA(inquirycontext: *const ::std::ffi::c_void, ifid: *mut RPC_IF_ID, membername: *mut *mut u8, priority: *mut u32, annotation: *mut *mut u8) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsProfileEltInqNextA(::std::mem::transmute(inquirycontext), ::std::mem::transmute(ifid), ::std::mem::transmute(membername), ::std::mem::transmute(priority), ::std::mem::transmute(annotation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsProfileEltInqNextW(inquirycontext: *const ::std::ffi::c_void, ifid: *mut RPC_IF_ID, membername: *mut *mut u16, priority: *mut u32, annotation: *mut *mut u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsProfileEltInqNextW(inquirycontext: *const ::std::ffi::c_void, ifid: *mut RPC_IF_ID, membername: *mut *mut u16, priority: *mut u32, annotation: *mut *mut u16) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsProfileEltInqNextW(::std::mem::transmute(inquirycontext), ::std::mem::transmute(ifid), ::std::mem::transmute(membername), ::std::mem::transmute(priority), ::std::mem::transmute(annotation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsProfileEltRemoveA(profilenamesyntax: u32, profilename: *const u8, ifid: *const RPC_IF_ID, membernamesyntax: u32, membername: *const u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsProfileEltRemoveA(profilenamesyntax: u32, profilename: *const u8, ifid: *const RPC_IF_ID, membernamesyntax: u32, membername: *const u8) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsProfileEltRemoveA(::std::mem::transmute(profilenamesyntax), ::std::mem::transmute(profilename), ::std::mem::transmute(ifid), ::std::mem::transmute(membernamesyntax), ::std::mem::transmute(membername)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcNsProfileEltRemoveW(profilenamesyntax: u32, profilename: *const u16, ifid: *const RPC_IF_ID, membernamesyntax: u32, membername: *const u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcNsProfileEltRemoveW(profilenamesyntax: u32, profilename: *const u16, ifid: *const RPC_IF_ID, membernamesyntax: u32, membername: *const u16) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcNsProfileEltRemoveW(::std::mem::transmute(profilenamesyntax), ::std::mem::transmute(profilename), ::std::mem::transmute(ifid), ::std::mem::transmute(membernamesyntax), ::std::mem::transmute(membername)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcObjectInqType(objuuid: *const ::windows::runtime::GUID, typeuuid: *mut ::windows::runtime::GUID) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcObjectInqType(objuuid: *const ::windows::runtime::GUID, typeuuid: *mut ::windows::runtime::GUID) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcObjectInqType(::std::mem::transmute(objuuid), ::std::mem::transmute(typeuuid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcObjectSetInqFn(inquiryfn: ::std::option::Option<RPC_OBJECT_INQ_FN>) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcObjectSetInqFn(inquiryfn: ::windows::runtime::RawPtr) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcObjectSetInqFn(::std::mem::transmute(inquiryfn)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcObjectSetType(objuuid: *const ::windows::runtime::GUID, typeuuid: *const ::windows::runtime::GUID) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcObjectSetType(objuuid: *const ::windows::runtime::GUID, typeuuid: *const ::windows::runtime::GUID) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcObjectSetType(::std::mem::transmute(objuuid), ::std::mem::transmute(typeuuid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcProtseqVectorFreeA(protseqvector: *mut *mut RPC_PROTSEQ_VECTORA) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcProtseqVectorFreeA(protseqvector: *mut *mut RPC_PROTSEQ_VECTORA) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcProtseqVectorFreeA(::std::mem::transmute(protseqvector)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcProtseqVectorFreeW(protseqvector: *mut *mut RPC_PROTSEQ_VECTORW) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcProtseqVectorFreeW(protseqvector: *mut *mut RPC_PROTSEQ_VECTORW) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcProtseqVectorFreeW(::std::mem::transmute(protseqvector)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct RpcProxyPerfCounters(pub i32);
pub const RpcCurrentUniqueUser: RpcProxyPerfCounters = RpcProxyPerfCounters(1i32);
pub const RpcBackEndConnectionAttempts: RpcProxyPerfCounters = RpcProxyPerfCounters(2i32);
pub const RpcBackEndConnectionFailed: RpcProxyPerfCounters = RpcProxyPerfCounters(3i32);
pub const RpcRequestsPerSecond: RpcProxyPerfCounters = RpcProxyPerfCounters(4i32);
pub const RpcIncomingConnections: RpcProxyPerfCounters = RpcProxyPerfCounters(5i32);
pub const RpcIncomingBandwidth: RpcProxyPerfCounters = RpcProxyPerfCounters(6i32);
pub const RpcOutgoingBandwidth: RpcProxyPerfCounters = RpcProxyPerfCounters(7i32);
pub const RpcAttemptedLbsDecisions: RpcProxyPerfCounters = RpcProxyPerfCounters(8i32);
pub const RpcFailedLbsDecisions: RpcProxyPerfCounters = RpcProxyPerfCounters(9i32);
pub const RpcAttemptedLbsMessages: RpcProxyPerfCounters = RpcProxyPerfCounters(10i32);
pub const RpcFailedLbsMessages: RpcProxyPerfCounters = RpcProxyPerfCounters(11i32);
pub const RpcLastCounter: RpcProxyPerfCounters = RpcProxyPerfCounters(12i32);
impl ::std::convert::From<i32> for RpcProxyPerfCounters {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for RpcProxyPerfCounters {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcRaiseException(exception: RPC_STATUS) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcRaiseException(exception: RPC_STATUS);
        }
        ::std::mem::transmute(RpcRaiseException(::std::mem::transmute(exception)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcRevertContainerImpersonation() -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcRevertContainerImpersonation() -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcRevertContainerImpersonation())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcRevertToSelf() -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcRevertToSelf() -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcRevertToSelf())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcRevertToSelfEx(bindinghandle: *const ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcRevertToSelfEx(bindinghandle: *const ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcRevertToSelfEx(::std::mem::transmute(bindinghandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcServerCompleteSecurityCallback(bindinghandle: *const ::std::ffi::c_void, status: RPC_STATUS) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerCompleteSecurityCallback(bindinghandle: *const ::std::ffi::c_void, status: RPC_STATUS) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcServerCompleteSecurityCallback(::std::mem::transmute(bindinghandle), ::std::mem::transmute(status)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcServerInqBindingHandle(binding: *mut *mut ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerInqBindingHandle(binding: *mut *mut ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcServerInqBindingHandle(::std::mem::transmute(binding)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcServerInqBindings(bindingvector: *mut *mut RPC_BINDING_VECTOR) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerInqBindings(bindingvector: *mut *mut RPC_BINDING_VECTOR) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcServerInqBindings(::std::mem::transmute(bindingvector)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcServerInqBindingsEx(securitydescriptor: *const ::std::ffi::c_void, bindingvector: *mut *mut RPC_BINDING_VECTOR) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerInqBindingsEx(securitydescriptor: *const ::std::ffi::c_void, bindingvector: *mut *mut RPC_BINDING_VECTOR) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcServerInqBindingsEx(::std::mem::transmute(securitydescriptor), ::std::mem::transmute(bindingvector)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcServerInqCallAttributesA(clientbinding: *const ::std::ffi::c_void, rpccallattributes: *mut ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerInqCallAttributesA(clientbinding: *const ::std::ffi::c_void, rpccallattributes: *mut ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcServerInqCallAttributesA(::std::mem::transmute(clientbinding), ::std::mem::transmute(rpccallattributes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcServerInqCallAttributesW(clientbinding: *const ::std::ffi::c_void, rpccallattributes: *mut ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerInqCallAttributesW(clientbinding: *const ::std::ffi::c_void, rpccallattributes: *mut ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcServerInqCallAttributesW(::std::mem::transmute(clientbinding), ::std::mem::transmute(rpccallattributes)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcServerInqDefaultPrincNameA(authnsvc: u32, princname: *mut *mut u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerInqDefaultPrincNameA(authnsvc: u32, princname: *mut *mut u8) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcServerInqDefaultPrincNameA(::std::mem::transmute(authnsvc), ::std::mem::transmute(princname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcServerInqDefaultPrincNameW(authnsvc: u32, princname: *mut *mut u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerInqDefaultPrincNameW(authnsvc: u32, princname: *mut *mut u16) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcServerInqDefaultPrincNameW(::std::mem::transmute(authnsvc), ::std::mem::transmute(princname)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcServerInqIf(ifspec: *const ::std::ffi::c_void, mgrtypeuuid: *const ::windows::runtime::GUID, mgrepv: *mut *mut ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerInqIf(ifspec: *const ::std::ffi::c_void, mgrtypeuuid: *const ::windows::runtime::GUID, mgrepv: *mut *mut ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcServerInqIf(::std::mem::transmute(ifspec), ::std::mem::transmute(mgrtypeuuid), ::std::mem::transmute(mgrepv)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcServerInterfaceGroupActivate(ifgroup: *const ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerInterfaceGroupActivate(ifgroup: *const ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcServerInterfaceGroupActivate(::std::mem::transmute(ifgroup)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcServerInterfaceGroupClose(ifgroup: *const ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerInterfaceGroupClose(ifgroup: *const ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcServerInterfaceGroupClose(::std::mem::transmute(ifgroup)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcServerInterfaceGroupCreateA(interfaces: *const RPC_INTERFACE_TEMPLATEA, numifs: u32, endpoints: *const RPC_ENDPOINT_TEMPLATEA, numendpoints: u32, idleperiod: u32, idlecallbackfn: ::std::option::Option<RPC_INTERFACE_GROUP_IDLE_CALLBACK_FN>, idlecallbackcontext: *const ::std::ffi::c_void, ifgroup: *mut *mut ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerInterfaceGroupCreateA(interfaces: *const ::std::mem::ManuallyDrop<RPC_INTERFACE_TEMPLATEA>, numifs: u32, endpoints: *const RPC_ENDPOINT_TEMPLATEA, numendpoints: u32, idleperiod: u32, idlecallbackfn: ::windows::runtime::RawPtr, idlecallbackcontext: *const ::std::ffi::c_void, ifgroup: *mut *mut ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcServerInterfaceGroupCreateA(
            ::std::mem::transmute(interfaces),
            ::std::mem::transmute(numifs),
            ::std::mem::transmute(endpoints),
            ::std::mem::transmute(numendpoints),
            ::std::mem::transmute(idleperiod),
            ::std::mem::transmute(idlecallbackfn),
            ::std::mem::transmute(idlecallbackcontext),
            ::std::mem::transmute(ifgroup),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcServerInterfaceGroupCreateW(interfaces: *const RPC_INTERFACE_TEMPLATEW, numifs: u32, endpoints: *const RPC_ENDPOINT_TEMPLATEW, numendpoints: u32, idleperiod: u32, idlecallbackfn: ::std::option::Option<RPC_INTERFACE_GROUP_IDLE_CALLBACK_FN>, idlecallbackcontext: *const ::std::ffi::c_void, ifgroup: *mut *mut ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerInterfaceGroupCreateW(interfaces: *const ::std::mem::ManuallyDrop<RPC_INTERFACE_TEMPLATEW>, numifs: u32, endpoints: *const RPC_ENDPOINT_TEMPLATEW, numendpoints: u32, idleperiod: u32, idlecallbackfn: ::windows::runtime::RawPtr, idlecallbackcontext: *const ::std::ffi::c_void, ifgroup: *mut *mut ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcServerInterfaceGroupCreateW(
            ::std::mem::transmute(interfaces),
            ::std::mem::transmute(numifs),
            ::std::mem::transmute(endpoints),
            ::std::mem::transmute(numendpoints),
            ::std::mem::transmute(idleperiod),
            ::std::mem::transmute(idlecallbackfn),
            ::std::mem::transmute(idlecallbackcontext),
            ::std::mem::transmute(ifgroup),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcServerInterfaceGroupDeactivate(ifgroup: *const ::std::ffi::c_void, forcedeactivation: u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerInterfaceGroupDeactivate(ifgroup: *const ::std::ffi::c_void, forcedeactivation: u32) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcServerInterfaceGroupDeactivate(::std::mem::transmute(ifgroup), ::std::mem::transmute(forcedeactivation)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcServerInterfaceGroupInqBindings(ifgroup: *const ::std::ffi::c_void, bindingvector: *mut *mut RPC_BINDING_VECTOR) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerInterfaceGroupInqBindings(ifgroup: *const ::std::ffi::c_void, bindingvector: *mut *mut RPC_BINDING_VECTOR) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcServerInterfaceGroupInqBindings(::std::mem::transmute(ifgroup), ::std::mem::transmute(bindingvector)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcServerListen(minimumcallthreads: u32, maxcalls: u32, dontwait: u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerListen(minimumcallthreads: u32, maxcalls: u32, dontwait: u32) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcServerListen(::std::mem::transmute(minimumcallthreads), ::std::mem::transmute(maxcalls), ::std::mem::transmute(dontwait)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcServerRegisterAuthInfoA(serverprincname: *const u8, authnsvc: u32, getkeyfn: ::std::option::Option<RPC_AUTH_KEY_RETRIEVAL_FN>, arg: *const ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerRegisterAuthInfoA(serverprincname: *const u8, authnsvc: u32, getkeyfn: ::windows::runtime::RawPtr, arg: *const ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcServerRegisterAuthInfoA(::std::mem::transmute(serverprincname), ::std::mem::transmute(authnsvc), ::std::mem::transmute(getkeyfn), ::std::mem::transmute(arg)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcServerRegisterAuthInfoW(serverprincname: *const u16, authnsvc: u32, getkeyfn: ::std::option::Option<RPC_AUTH_KEY_RETRIEVAL_FN>, arg: *const ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerRegisterAuthInfoW(serverprincname: *const u16, authnsvc: u32, getkeyfn: ::windows::runtime::RawPtr, arg: *const ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcServerRegisterAuthInfoW(::std::mem::transmute(serverprincname), ::std::mem::transmute(authnsvc), ::std::mem::transmute(getkeyfn), ::std::mem::transmute(arg)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcServerRegisterIf(ifspec: *const ::std::ffi::c_void, mgrtypeuuid: *const ::windows::runtime::GUID, mgrepv: *const ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerRegisterIf(ifspec: *const ::std::ffi::c_void, mgrtypeuuid: *const ::windows::runtime::GUID, mgrepv: *const ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcServerRegisterIf(::std::mem::transmute(ifspec), ::std::mem::transmute(mgrtypeuuid), ::std::mem::transmute(mgrepv)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcServerRegisterIf2(ifspec: *const ::std::ffi::c_void, mgrtypeuuid: *const ::windows::runtime::GUID, mgrepv: *const ::std::ffi::c_void, flags: u32, maxcalls: u32, maxrpcsize: u32, ifcallbackfn: ::std::option::Option<RPC_IF_CALLBACK_FN>) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerRegisterIf2(ifspec: *const ::std::ffi::c_void, mgrtypeuuid: *const ::windows::runtime::GUID, mgrepv: *const ::std::ffi::c_void, flags: u32, maxcalls: u32, maxrpcsize: u32, ifcallbackfn: ::windows::runtime::RawPtr) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcServerRegisterIf2(::std::mem::transmute(ifspec), ::std::mem::transmute(mgrtypeuuid), ::std::mem::transmute(mgrepv), ::std::mem::transmute(flags), ::std::mem::transmute(maxcalls), ::std::mem::transmute(maxrpcsize), ::std::mem::transmute(ifcallbackfn)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcServerRegisterIf3(ifspec: *const ::std::ffi::c_void, mgrtypeuuid: *const ::windows::runtime::GUID, mgrepv: *const ::std::ffi::c_void, flags: u32, maxcalls: u32, maxrpcsize: u32, ifcallback: ::std::option::Option<RPC_IF_CALLBACK_FN>, securitydescriptor: *const ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerRegisterIf3(ifspec: *const ::std::ffi::c_void, mgrtypeuuid: *const ::windows::runtime::GUID, mgrepv: *const ::std::ffi::c_void, flags: u32, maxcalls: u32, maxrpcsize: u32, ifcallback: ::windows::runtime::RawPtr, securitydescriptor: *const ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcServerRegisterIf3(::std::mem::transmute(ifspec), ::std::mem::transmute(mgrtypeuuid), ::std::mem::transmute(mgrepv), ::std::mem::transmute(flags), ::std::mem::transmute(maxcalls), ::std::mem::transmute(maxrpcsize), ::std::mem::transmute(ifcallback), ::std::mem::transmute(securitydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcServerRegisterIfEx(ifspec: *const ::std::ffi::c_void, mgrtypeuuid: *const ::windows::runtime::GUID, mgrepv: *const ::std::ffi::c_void, flags: u32, maxcalls: u32, ifcallback: ::std::option::Option<RPC_IF_CALLBACK_FN>) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerRegisterIfEx(ifspec: *const ::std::ffi::c_void, mgrtypeuuid: *const ::windows::runtime::GUID, mgrepv: *const ::std::ffi::c_void, flags: u32, maxcalls: u32, ifcallback: ::windows::runtime::RawPtr) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcServerRegisterIfEx(::std::mem::transmute(ifspec), ::std::mem::transmute(mgrtypeuuid), ::std::mem::transmute(mgrepv), ::std::mem::transmute(flags), ::std::mem::transmute(maxcalls), ::std::mem::transmute(ifcallback)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_Foundation`, `Win32_System_IO`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_IO"))]
#[inline]
pub unsafe fn RpcServerSubscribeForNotification(binding: *const ::std::ffi::c_void, notification: RPC_NOTIFICATIONS, notificationtype: RPC_NOTIFICATION_TYPES, notificationinfo: *const RPC_ASYNC_NOTIFICATION_INFO) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerSubscribeForNotification(binding: *const ::std::ffi::c_void, notification: RPC_NOTIFICATIONS, notificationtype: RPC_NOTIFICATION_TYPES, notificationinfo: *const ::std::mem::ManuallyDrop<RPC_ASYNC_NOTIFICATION_INFO>) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcServerSubscribeForNotification(::std::mem::transmute(binding), ::std::mem::transmute(notification), ::std::mem::transmute(notificationtype), ::std::mem::transmute(notificationinfo)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcServerTestCancel(bindinghandle: *const ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerTestCancel(bindinghandle: *const ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcServerTestCancel(::std::mem::transmute(bindinghandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcServerUnregisterIf(ifspec: *const ::std::ffi::c_void, mgrtypeuuid: *const ::windows::runtime::GUID, waitforcallstocomplete: u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerUnregisterIf(ifspec: *const ::std::ffi::c_void, mgrtypeuuid: *const ::windows::runtime::GUID, waitforcallstocomplete: u32) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcServerUnregisterIf(::std::mem::transmute(ifspec), ::std::mem::transmute(mgrtypeuuid), ::std::mem::transmute(waitforcallstocomplete)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcServerUnregisterIfEx(ifspec: *const ::std::ffi::c_void, mgrtypeuuid: *const ::windows::runtime::GUID, rundowncontexthandles: i32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerUnregisterIfEx(ifspec: *const ::std::ffi::c_void, mgrtypeuuid: *const ::windows::runtime::GUID, rundowncontexthandles: i32) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcServerUnregisterIfEx(::std::mem::transmute(ifspec), ::std::mem::transmute(mgrtypeuuid), ::std::mem::transmute(rundowncontexthandles)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcServerUnsubscribeForNotification(binding: *const ::std::ffi::c_void, notification: RPC_NOTIFICATIONS, notificationsqueued: *mut u32) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerUnsubscribeForNotification(binding: *const ::std::ffi::c_void, notification: RPC_NOTIFICATIONS, notificationsqueued: *mut u32) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcServerUnsubscribeForNotification(::std::mem::transmute(binding), ::std::mem::transmute(notification), ::std::mem::transmute(notificationsqueued)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcServerUseAllProtseqs(maxcalls: u32, securitydescriptor: *const ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerUseAllProtseqs(maxcalls: u32, securitydescriptor: *const ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcServerUseAllProtseqs(::std::mem::transmute(maxcalls), ::std::mem::transmute(securitydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcServerUseAllProtseqsEx(maxcalls: u32, securitydescriptor: *const ::std::ffi::c_void, policy: *const RPC_POLICY) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerUseAllProtseqsEx(maxcalls: u32, securitydescriptor: *const ::std::ffi::c_void, policy: *const RPC_POLICY) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcServerUseAllProtseqsEx(::std::mem::transmute(maxcalls), ::std::mem::transmute(securitydescriptor), ::std::mem::transmute(policy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcServerUseAllProtseqsIf(maxcalls: u32, ifspec: *const ::std::ffi::c_void, securitydescriptor: *const ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerUseAllProtseqsIf(maxcalls: u32, ifspec: *const ::std::ffi::c_void, securitydescriptor: *const ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcServerUseAllProtseqsIf(::std::mem::transmute(maxcalls), ::std::mem::transmute(ifspec), ::std::mem::transmute(securitydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcServerUseAllProtseqsIfEx(maxcalls: u32, ifspec: *const ::std::ffi::c_void, securitydescriptor: *const ::std::ffi::c_void, policy: *const RPC_POLICY) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerUseAllProtseqsIfEx(maxcalls: u32, ifspec: *const ::std::ffi::c_void, securitydescriptor: *const ::std::ffi::c_void, policy: *const RPC_POLICY) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcServerUseAllProtseqsIfEx(::std::mem::transmute(maxcalls), ::std::mem::transmute(ifspec), ::std::mem::transmute(securitydescriptor), ::std::mem::transmute(policy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcServerUseProtseqA(protseq: *const u8, maxcalls: u32, securitydescriptor: *const ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerUseProtseqA(protseq: *const u8, maxcalls: u32, securitydescriptor: *const ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcServerUseProtseqA(::std::mem::transmute(protseq), ::std::mem::transmute(maxcalls), ::std::mem::transmute(securitydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcServerUseProtseqEpA(protseq: *const u8, maxcalls: u32, endpoint: *const u8, securitydescriptor: *const ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerUseProtseqEpA(protseq: *const u8, maxcalls: u32, endpoint: *const u8, securitydescriptor: *const ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcServerUseProtseqEpA(::std::mem::transmute(protseq), ::std::mem::transmute(maxcalls), ::std::mem::transmute(endpoint), ::std::mem::transmute(securitydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcServerUseProtseqEpExA(protseq: *const u8, maxcalls: u32, endpoint: *const u8, securitydescriptor: *const ::std::ffi::c_void, policy: *const RPC_POLICY) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerUseProtseqEpExA(protseq: *const u8, maxcalls: u32, endpoint: *const u8, securitydescriptor: *const ::std::ffi::c_void, policy: *const RPC_POLICY) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcServerUseProtseqEpExA(::std::mem::transmute(protseq), ::std::mem::transmute(maxcalls), ::std::mem::transmute(endpoint), ::std::mem::transmute(securitydescriptor), ::std::mem::transmute(policy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcServerUseProtseqEpExW(protseq: *const u16, maxcalls: u32, endpoint: *const u16, securitydescriptor: *const ::std::ffi::c_void, policy: *const RPC_POLICY) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerUseProtseqEpExW(protseq: *const u16, maxcalls: u32, endpoint: *const u16, securitydescriptor: *const ::std::ffi::c_void, policy: *const RPC_POLICY) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcServerUseProtseqEpExW(::std::mem::transmute(protseq), ::std::mem::transmute(maxcalls), ::std::mem::transmute(endpoint), ::std::mem::transmute(securitydescriptor), ::std::mem::transmute(policy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcServerUseProtseqEpW(protseq: *const u16, maxcalls: u32, endpoint: *const u16, securitydescriptor: *const ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerUseProtseqEpW(protseq: *const u16, maxcalls: u32, endpoint: *const u16, securitydescriptor: *const ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcServerUseProtseqEpW(::std::mem::transmute(protseq), ::std::mem::transmute(maxcalls), ::std::mem::transmute(endpoint), ::std::mem::transmute(securitydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcServerUseProtseqExA(protseq: *const u8, maxcalls: u32, securitydescriptor: *const ::std::ffi::c_void, policy: *const RPC_POLICY) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerUseProtseqExA(protseq: *const u8, maxcalls: u32, securitydescriptor: *const ::std::ffi::c_void, policy: *const RPC_POLICY) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcServerUseProtseqExA(::std::mem::transmute(protseq), ::std::mem::transmute(maxcalls), ::std::mem::transmute(securitydescriptor), ::std::mem::transmute(policy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcServerUseProtseqExW(protseq: *const u16, maxcalls: u32, securitydescriptor: *const ::std::ffi::c_void, policy: *const RPC_POLICY) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerUseProtseqExW(protseq: *const u16, maxcalls: u32, securitydescriptor: *const ::std::ffi::c_void, policy: *const RPC_POLICY) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcServerUseProtseqExW(::std::mem::transmute(protseq), ::std::mem::transmute(maxcalls), ::std::mem::transmute(securitydescriptor), ::std::mem::transmute(policy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcServerUseProtseqIfA(protseq: *const u8, maxcalls: u32, ifspec: *const ::std::ffi::c_void, securitydescriptor: *const ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerUseProtseqIfA(protseq: *const u8, maxcalls: u32, ifspec: *const ::std::ffi::c_void, securitydescriptor: *const ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcServerUseProtseqIfA(::std::mem::transmute(protseq), ::std::mem::transmute(maxcalls), ::std::mem::transmute(ifspec), ::std::mem::transmute(securitydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcServerUseProtseqIfExA(protseq: *const u8, maxcalls: u32, ifspec: *const ::std::ffi::c_void, securitydescriptor: *const ::std::ffi::c_void, policy: *const RPC_POLICY) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerUseProtseqIfExA(protseq: *const u8, maxcalls: u32, ifspec: *const ::std::ffi::c_void, securitydescriptor: *const ::std::ffi::c_void, policy: *const RPC_POLICY) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcServerUseProtseqIfExA(::std::mem::transmute(protseq), ::std::mem::transmute(maxcalls), ::std::mem::transmute(ifspec), ::std::mem::transmute(securitydescriptor), ::std::mem::transmute(policy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcServerUseProtseqIfExW(protseq: *const u16, maxcalls: u32, ifspec: *const ::std::ffi::c_void, securitydescriptor: *const ::std::ffi::c_void, policy: *const RPC_POLICY) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerUseProtseqIfExW(protseq: *const u16, maxcalls: u32, ifspec: *const ::std::ffi::c_void, securitydescriptor: *const ::std::ffi::c_void, policy: *const RPC_POLICY) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcServerUseProtseqIfExW(::std::mem::transmute(protseq), ::std::mem::transmute(maxcalls), ::std::mem::transmute(ifspec), ::std::mem::transmute(securitydescriptor), ::std::mem::transmute(policy)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcServerUseProtseqIfW(protseq: *const u16, maxcalls: u32, ifspec: *const ::std::ffi::c_void, securitydescriptor: *const ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerUseProtseqIfW(protseq: *const u16, maxcalls: u32, ifspec: *const ::std::ffi::c_void, securitydescriptor: *const ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcServerUseProtseqIfW(::std::mem::transmute(protseq), ::std::mem::transmute(maxcalls), ::std::mem::transmute(ifspec), ::std::mem::transmute(securitydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcServerUseProtseqW(protseq: *const u16, maxcalls: u32, securitydescriptor: *const ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerUseProtseqW(protseq: *const u16, maxcalls: u32, securitydescriptor: *const ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcServerUseProtseqW(::std::mem::transmute(protseq), ::std::mem::transmute(maxcalls), ::std::mem::transmute(securitydescriptor)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcServerYield() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcServerYield();
        }
        ::std::mem::transmute(RpcServerYield())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcSmAllocate(size: usize, pstatus: *mut RPC_STATUS) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcSmAllocate(size: usize, pstatus: *mut RPC_STATUS) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(RpcSmAllocate(::std::mem::transmute(size), ::std::mem::transmute(pstatus)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcSmClientFree(pnodetofree: *const ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcSmClientFree(pnodetofree: *const ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcSmClientFree(::std::mem::transmute(pnodetofree)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcSmDestroyClientContext(contexthandle: *const *const ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcSmDestroyClientContext(contexthandle: *const *const ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcSmDestroyClientContext(::std::mem::transmute(contexthandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcSmDisableAllocate() -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcSmDisableAllocate() -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcSmDisableAllocate())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcSmEnableAllocate() -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcSmEnableAllocate() -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcSmEnableAllocate())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcSmFree(nodetofree: *const ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcSmFree(nodetofree: *const ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcSmFree(::std::mem::transmute(nodetofree)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcSmGetThreadHandle(pstatus: *mut RPC_STATUS) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcSmGetThreadHandle(pstatus: *mut RPC_STATUS) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(RpcSmGetThreadHandle(::std::mem::transmute(pstatus)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcSmSetClientAllocFree(clientalloc: ::std::option::Option<RPC_CLIENT_ALLOC>, clientfree: ::std::option::Option<RPC_CLIENT_FREE>) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcSmSetClientAllocFree(clientalloc: ::windows::runtime::RawPtr, clientfree: ::windows::runtime::RawPtr) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcSmSetClientAllocFree(::std::mem::transmute(clientalloc), ::std::mem::transmute(clientfree)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcSmSetThreadHandle(id: *const ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcSmSetThreadHandle(id: *const ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcSmSetThreadHandle(::std::mem::transmute(id)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcSmSwapClientAllocFree(clientalloc: ::std::option::Option<RPC_CLIENT_ALLOC>, clientfree: ::std::option::Option<RPC_CLIENT_FREE>, oldclientalloc: *mut ::std::option::Option<RPC_CLIENT_ALLOC>, oldclientfree: *mut ::std::option::Option<RPC_CLIENT_FREE>) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcSmSwapClientAllocFree(clientalloc: ::windows::runtime::RawPtr, clientfree: ::windows::runtime::RawPtr, oldclientalloc: *mut ::windows::runtime::RawPtr, oldclientfree: *mut ::windows::runtime::RawPtr) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcSmSwapClientAllocFree(::std::mem::transmute(clientalloc), ::std::mem::transmute(clientfree), ::std::mem::transmute(oldclientalloc), ::std::mem::transmute(oldclientfree)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcSsAllocate(size: usize) -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcSsAllocate(size: usize) -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(RpcSsAllocate(::std::mem::transmute(size)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcSsContextLockExclusive(serverbindinghandle: *const ::std::ffi::c_void, usercontext: *const ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcSsContextLockExclusive(serverbindinghandle: *const ::std::ffi::c_void, usercontext: *const ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcSsContextLockExclusive(::std::mem::transmute(serverbindinghandle), ::std::mem::transmute(usercontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcSsContextLockShared(serverbindinghandle: *const ::std::ffi::c_void, usercontext: *const ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcSsContextLockShared(serverbindinghandle: *const ::std::ffi::c_void, usercontext: *const ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcSsContextLockShared(::std::mem::transmute(serverbindinghandle), ::std::mem::transmute(usercontext)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcSsDestroyClientContext(contexthandle: *const *const ::std::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcSsDestroyClientContext(contexthandle: *const *const ::std::ffi::c_void);
        }
        ::std::mem::transmute(RpcSsDestroyClientContext(::std::mem::transmute(contexthandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcSsDisableAllocate() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcSsDisableAllocate();
        }
        ::std::mem::transmute(RpcSsDisableAllocate())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcSsDontSerializeContext() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcSsDontSerializeContext();
        }
        ::std::mem::transmute(RpcSsDontSerializeContext())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcSsEnableAllocate() {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcSsEnableAllocate();
        }
        ::std::mem::transmute(RpcSsEnableAllocate())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcSsFree(nodetofree: *const ::std::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcSsFree(nodetofree: *const ::std::ffi::c_void);
        }
        ::std::mem::transmute(RpcSsFree(::std::mem::transmute(nodetofree)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcSsGetContextBinding(contexthandle: *const ::std::ffi::c_void, binding: *mut *mut ::std::ffi::c_void) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcSsGetContextBinding(contexthandle: *const ::std::ffi::c_void, binding: *mut *mut ::std::ffi::c_void) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcSsGetContextBinding(::std::mem::transmute(contexthandle), ::std::mem::transmute(binding)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcSsGetThreadHandle() -> *mut ::std::ffi::c_void {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcSsGetThreadHandle() -> *mut ::std::ffi::c_void;
        }
        ::std::mem::transmute(RpcSsGetThreadHandle())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcSsSetClientAllocFree(clientalloc: ::std::option::Option<RPC_CLIENT_ALLOC>, clientfree: ::std::option::Option<RPC_CLIENT_FREE>) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcSsSetClientAllocFree(clientalloc: ::windows::runtime::RawPtr, clientfree: ::windows::runtime::RawPtr);
        }
        ::std::mem::transmute(RpcSsSetClientAllocFree(::std::mem::transmute(clientalloc), ::std::mem::transmute(clientfree)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcSsSetThreadHandle(id: *const ::std::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcSsSetThreadHandle(id: *const ::std::ffi::c_void);
        }
        ::std::mem::transmute(RpcSsSetThreadHandle(::std::mem::transmute(id)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcSsSwapClientAllocFree(clientalloc: ::std::option::Option<RPC_CLIENT_ALLOC>, clientfree: ::std::option::Option<RPC_CLIENT_FREE>, oldclientalloc: *mut ::std::option::Option<RPC_CLIENT_ALLOC>, oldclientfree: *mut ::std::option::Option<RPC_CLIENT_FREE>) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcSsSwapClientAllocFree(clientalloc: ::windows::runtime::RawPtr, clientfree: ::windows::runtime::RawPtr, oldclientalloc: *mut ::windows::runtime::RawPtr, oldclientfree: *mut ::windows::runtime::RawPtr);
        }
        ::std::mem::transmute(RpcSsSwapClientAllocFree(::std::mem::transmute(clientalloc), ::std::mem::transmute(clientfree), ::std::mem::transmute(oldclientalloc), ::std::mem::transmute(oldclientfree)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcStringBindingComposeA(objuuid: *const u8, protseq: *const u8, networkaddr: *const u8, endpoint: *const u8, options: *const u8, stringbinding: *mut *mut u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcStringBindingComposeA(objuuid: *const u8, protseq: *const u8, networkaddr: *const u8, endpoint: *const u8, options: *const u8, stringbinding: *mut *mut u8) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcStringBindingComposeA(::std::mem::transmute(objuuid), ::std::mem::transmute(protseq), ::std::mem::transmute(networkaddr), ::std::mem::transmute(endpoint), ::std::mem::transmute(options), ::std::mem::transmute(stringbinding)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcStringBindingComposeW(objuuid: *const u16, protseq: *const u16, networkaddr: *const u16, endpoint: *const u16, options: *const u16, stringbinding: *mut *mut u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcStringBindingComposeW(objuuid: *const u16, protseq: *const u16, networkaddr: *const u16, endpoint: *const u16, options: *const u16, stringbinding: *mut *mut u16) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcStringBindingComposeW(::std::mem::transmute(objuuid), ::std::mem::transmute(protseq), ::std::mem::transmute(networkaddr), ::std::mem::transmute(endpoint), ::std::mem::transmute(options), ::std::mem::transmute(stringbinding)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcStringBindingParseA(stringbinding: *const u8, objuuid: *mut *mut u8, protseq: *mut *mut u8, networkaddr: *mut *mut u8, endpoint: *mut *mut u8, networkoptions: *mut *mut u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcStringBindingParseA(stringbinding: *const u8, objuuid: *mut *mut u8, protseq: *mut *mut u8, networkaddr: *mut *mut u8, endpoint: *mut *mut u8, networkoptions: *mut *mut u8) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcStringBindingParseA(::std::mem::transmute(stringbinding), ::std::mem::transmute(objuuid), ::std::mem::transmute(protseq), ::std::mem::transmute(networkaddr), ::std::mem::transmute(endpoint), ::std::mem::transmute(networkoptions)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcStringBindingParseW(stringbinding: *const u16, objuuid: *mut *mut u16, protseq: *mut *mut u16, networkaddr: *mut *mut u16, endpoint: *mut *mut u16, networkoptions: *mut *mut u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcStringBindingParseW(stringbinding: *const u16, objuuid: *mut *mut u16, protseq: *mut *mut u16, networkaddr: *mut *mut u16, endpoint: *mut *mut u16, networkoptions: *mut *mut u16) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcStringBindingParseW(::std::mem::transmute(stringbinding), ::std::mem::transmute(objuuid), ::std::mem::transmute(protseq), ::std::mem::transmute(networkaddr), ::std::mem::transmute(endpoint), ::std::mem::transmute(networkoptions)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcStringFreeA(string: *mut *mut u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcStringFreeA(string: *mut *mut u8) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcStringFreeA(::std::mem::transmute(string)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcStringFreeW(string: *mut *mut u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcStringFreeW(string: *mut *mut u16) -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcStringFreeW(::std::mem::transmute(string)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcTestCancel() -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcTestCancel() -> RPC_STATUS;
        }
        ::std::mem::transmute(RpcTestCancel())
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn RpcUserFree(asynchandle: *mut ::std::ffi::c_void, pbuffer: *mut ::std::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RpcUserFree(asynchandle: *mut ::std::ffi::c_void, pbuffer: *mut ::std::ffi::c_void);
        }
        ::std::mem::transmute(RpcUserFree(::std::mem::transmute(asynchandle), ::std::mem::transmute(pbuffer)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct SCONTEXT_QUEUE {
    pub NumberOfObjects: u32,
    pub ArrayOfObjects: *mut *mut NDR_SCONTEXT_1,
}
impl SCONTEXT_QUEUE {}
impl ::std::default::Default for SCONTEXT_QUEUE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SCONTEXT_QUEUE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SCONTEXT_QUEUE").field("NumberOfObjects", &self.NumberOfObjects).field("ArrayOfObjects", &self.ArrayOfObjects).finish()
    }
}
impl ::std::cmp::PartialEq for SCONTEXT_QUEUE {
    fn eq(&self, other: &Self) -> bool {
        self.NumberOfObjects == other.NumberOfObjects && self.ArrayOfObjects == other.ArrayOfObjects
    }
}
impl ::std::cmp::Eq for SCONTEXT_QUEUE {}
unsafe impl ::windows::runtime::Abi for SCONTEXT_QUEUE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SEC_WINNT_AUTH_IDENTITY(pub u32);
pub const SEC_WINNT_AUTH_IDENTITY_ANSI: SEC_WINNT_AUTH_IDENTITY = SEC_WINNT_AUTH_IDENTITY(1u32);
pub const SEC_WINNT_AUTH_IDENTITY_UNICODE: SEC_WINNT_AUTH_IDENTITY = SEC_WINNT_AUTH_IDENTITY(2u32);
impl ::std::convert::From<u32> for SEC_WINNT_AUTH_IDENTITY {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SEC_WINNT_AUTH_IDENTITY {
    type Abi = Self;
}
impl ::std::ops::BitOr for SEC_WINNT_AUTH_IDENTITY {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for SEC_WINNT_AUTH_IDENTITY {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for SEC_WINNT_AUTH_IDENTITY {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for SEC_WINNT_AUTH_IDENTITY {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for SEC_WINNT_AUTH_IDENTITY {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct SEC_WINNT_AUTH_IDENTITY_A {
    pub User: *mut u8,
    pub UserLength: u32,
    pub Domain: *mut u8,
    pub DomainLength: u32,
    pub Password: *mut u8,
    pub PasswordLength: u32,
    pub Flags: SEC_WINNT_AUTH_IDENTITY,
}
impl SEC_WINNT_AUTH_IDENTITY_A {}
impl ::std::default::Default for SEC_WINNT_AUTH_IDENTITY_A {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SEC_WINNT_AUTH_IDENTITY_A {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SEC_WINNT_AUTH_IDENTITY_A").field("User", &self.User).field("UserLength", &self.UserLength).field("Domain", &self.Domain).field("DomainLength", &self.DomainLength).field("Password", &self.Password).field("PasswordLength", &self.PasswordLength).field("Flags", &self.Flags).finish()
    }
}
impl ::std::cmp::PartialEq for SEC_WINNT_AUTH_IDENTITY_A {
    fn eq(&self, other: &Self) -> bool {
        self.User == other.User && self.UserLength == other.UserLength && self.Domain == other.Domain && self.DomainLength == other.DomainLength && self.Password == other.Password && self.PasswordLength == other.PasswordLength && self.Flags == other.Flags
    }
}
impl ::std::cmp::Eq for SEC_WINNT_AUTH_IDENTITY_A {}
unsafe impl ::windows::runtime::Abi for SEC_WINNT_AUTH_IDENTITY_A {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct SEC_WINNT_AUTH_IDENTITY_W {
    pub User: *mut u16,
    pub UserLength: u32,
    pub Domain: *mut u16,
    pub DomainLength: u32,
    pub Password: *mut u16,
    pub PasswordLength: u32,
    pub Flags: SEC_WINNT_AUTH_IDENTITY,
}
impl SEC_WINNT_AUTH_IDENTITY_W {}
impl ::std::default::Default for SEC_WINNT_AUTH_IDENTITY_W {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SEC_WINNT_AUTH_IDENTITY_W {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SEC_WINNT_AUTH_IDENTITY_W").field("User", &self.User).field("UserLength", &self.UserLength).field("Domain", &self.Domain).field("DomainLength", &self.DomainLength).field("Password", &self.Password).field("PasswordLength", &self.PasswordLength).field("Flags", &self.Flags).finish()
    }
}
impl ::std::cmp::PartialEq for SEC_WINNT_AUTH_IDENTITY_W {
    fn eq(&self, other: &Self) -> bool {
        self.User == other.User && self.UserLength == other.UserLength && self.Domain == other.Domain && self.DomainLength == other.DomainLength && self.Password == other.Password && self.PasswordLength == other.PasswordLength && self.Flags == other.Flags
    }
}
impl ::std::cmp::Eq for SEC_WINNT_AUTH_IDENTITY_W {}
unsafe impl ::windows::runtime::Abi for SEC_WINNT_AUTH_IDENTITY_W {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub type SERVER_ROUTINE = unsafe extern "system" fn() -> i32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct STUB_PHASE(pub i32);
pub const STUB_UNMARSHAL: STUB_PHASE = STUB_PHASE(0i32);
pub const STUB_CALL_SERVER: STUB_PHASE = STUB_PHASE(1i32);
pub const STUB_MARSHAL: STUB_PHASE = STUB_PHASE(2i32);
pub const STUB_CALL_SERVER_NO_HRESULT: STUB_PHASE = STUB_PHASE(3i32);
impl ::std::convert::From<i32> for STUB_PHASE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for STUB_PHASE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
pub type STUB_THUNK = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>);
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const TARGET_IS_NT100_OR_LATER: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const TARGET_IS_NT351_OR_WIN95_OR_LATER: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const TARGET_IS_NT40_OR_LATER: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const TARGET_IS_NT50_OR_LATER: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const TARGET_IS_NT51_OR_LATER: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const TARGET_IS_NT60_OR_LATER: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const TARGET_IS_NT61_OR_LATER: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const TARGET_IS_NT62_OR_LATER: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const TARGET_IS_NT63_OR_LATER: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const TRANSPORT_TYPE_CN: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const TRANSPORT_TYPE_DG: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const TRANSPORT_TYPE_LPC: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const TRANSPORT_TYPE_WMSG: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const USER_CALL_IS_ASYNC: u32 = 256u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const USER_CALL_NEW_CORRELATION_DESC: u32 = 512u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
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
impl USER_MARSHAL_CB {}
#[cfg(feature = "Win32_System_Com")]
impl ::std::default::Default for USER_MARSHAL_CB {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::fmt::Debug for USER_MARSHAL_CB {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("USER_MARSHAL_CB").field("Flags", &self.Flags).field("pStubMsg", &self.pStubMsg).field("pReserve", &self.pReserve).field("Signature", &self.Signature).field("CBType", &self.CBType).field("pFormat", &self.pFormat).field("pTypeFormat", &self.pTypeFormat).finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::PartialEq for USER_MARSHAL_CB {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.pStubMsg == other.pStubMsg && self.pReserve == other.pReserve && self.Signature == other.Signature && self.CBType == other.CBType && self.pFormat == other.pFormat && self.pTypeFormat == other.pTypeFormat
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::Eq for USER_MARSHAL_CB {}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::runtime::Abi for USER_MARSHAL_CB {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct USER_MARSHAL_CB_TYPE(pub i32);
pub const USER_MARSHAL_CB_BUFFER_SIZE: USER_MARSHAL_CB_TYPE = USER_MARSHAL_CB_TYPE(0i32);
pub const USER_MARSHAL_CB_MARSHALL: USER_MARSHAL_CB_TYPE = USER_MARSHAL_CB_TYPE(1i32);
pub const USER_MARSHAL_CB_UNMARSHALL: USER_MARSHAL_CB_TYPE = USER_MARSHAL_CB_TYPE(2i32);
pub const USER_MARSHAL_CB_FREE: USER_MARSHAL_CB_TYPE = USER_MARSHAL_CB_TYPE(3i32);
impl ::std::convert::From<i32> for USER_MARSHAL_CB_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for USER_MARSHAL_CB_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const USER_MARSHAL_FC_BYTE: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const USER_MARSHAL_FC_CHAR: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const USER_MARSHAL_FC_DOUBLE: u32 = 12u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const USER_MARSHAL_FC_FLOAT: u32 = 10u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const USER_MARSHAL_FC_HYPER: u32 = 11u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const USER_MARSHAL_FC_LONG: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const USER_MARSHAL_FC_SHORT: u32 = 6u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const USER_MARSHAL_FC_SMALL: u32 = 3u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const USER_MARSHAL_FC_ULONG: u32 = 9u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const USER_MARSHAL_FC_USHORT: u32 = 7u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const USER_MARSHAL_FC_USMALL: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const USER_MARSHAL_FC_WCHAR: u32 = 5u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub type USER_MARSHAL_FREEING_ROUTINE = unsafe extern "system" fn(param0: *mut u32, param1: *mut ::std::ffi::c_void);
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub type USER_MARSHAL_MARSHALLING_ROUTINE = unsafe extern "system" fn(param0: *mut u32, param1: *mut u8, param2: *mut ::std::ffi::c_void) -> *mut u8;
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct USER_MARSHAL_ROUTINE_QUADRUPLE {
    pub pfnBufferSize: ::std::option::Option<USER_MARSHAL_SIZING_ROUTINE>,
    pub pfnMarshall: ::std::option::Option<USER_MARSHAL_MARSHALLING_ROUTINE>,
    pub pfnUnmarshall: ::std::option::Option<USER_MARSHAL_UNMARSHALLING_ROUTINE>,
    pub pfnFree: ::std::option::Option<USER_MARSHAL_FREEING_ROUTINE>,
}
impl USER_MARSHAL_ROUTINE_QUADRUPLE {}
impl ::std::default::Default for USER_MARSHAL_ROUTINE_QUADRUPLE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for USER_MARSHAL_ROUTINE_QUADRUPLE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("USER_MARSHAL_ROUTINE_QUADRUPLE").finish()
    }
}
impl ::std::cmp::PartialEq for USER_MARSHAL_ROUTINE_QUADRUPLE {
    fn eq(&self, other: &Self) -> bool {
        self.pfnBufferSize.map(|f| f as usize) == other.pfnBufferSize.map(|f| f as usize) && self.pfnMarshall.map(|f| f as usize) == other.pfnMarshall.map(|f| f as usize) && self.pfnUnmarshall.map(|f| f as usize) == other.pfnUnmarshall.map(|f| f as usize) && self.pfnFree.map(|f| f as usize) == other.pfnFree.map(|f| f as usize)
    }
}
impl ::std::cmp::Eq for USER_MARSHAL_ROUTINE_QUADRUPLE {}
unsafe impl ::windows::runtime::Abi for USER_MARSHAL_ROUTINE_QUADRUPLE {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub type USER_MARSHAL_SIZING_ROUTINE = unsafe extern "system" fn(param0: *mut u32, param1: u32, param2: *mut ::std::ffi::c_void) -> u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub type USER_MARSHAL_UNMARSHALLING_ROUTINE = unsafe extern "system" fn(param0: *mut u32, param1: *mut u8, param2: *mut ::std::ffi::c_void) -> *mut u8;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct UUID_VECTOR {
    pub Count: u32,
    pub Uuid: [*mut ::windows::runtime::GUID; 1],
}
impl UUID_VECTOR {}
impl ::std::default::Default for UUID_VECTOR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for UUID_VECTOR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("UUID_VECTOR").field("Count", &self.Count).field("Uuid", &self.Uuid).finish()
    }
}
impl ::std::cmp::PartialEq for UUID_VECTOR {
    fn eq(&self, other: &Self) -> bool {
        self.Count == other.Count && self.Uuid == other.Uuid
    }
}
impl ::std::cmp::Eq for UUID_VECTOR {}
unsafe impl ::windows::runtime::Abi for UUID_VECTOR {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn UuidCompare(uuid1: *const ::windows::runtime::GUID, uuid2: *const ::windows::runtime::GUID, status: *mut RPC_STATUS) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UuidCompare(uuid1: *const ::windows::runtime::GUID, uuid2: *const ::windows::runtime::GUID, status: *mut RPC_STATUS) -> i32;
        }
        ::std::mem::transmute(UuidCompare(::std::mem::transmute(uuid1), ::std::mem::transmute(uuid2), ::std::mem::transmute(status)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn UuidCreate(uuid: *mut ::windows::runtime::GUID) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UuidCreate(uuid: *mut ::windows::runtime::GUID) -> RPC_STATUS;
        }
        ::std::mem::transmute(UuidCreate(::std::mem::transmute(uuid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn UuidCreateNil(niluuid: *mut ::windows::runtime::GUID) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UuidCreateNil(niluuid: *mut ::windows::runtime::GUID) -> RPC_STATUS;
        }
        ::std::mem::transmute(UuidCreateNil(::std::mem::transmute(niluuid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn UuidCreateSequential(uuid: *mut ::windows::runtime::GUID) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UuidCreateSequential(uuid: *mut ::windows::runtime::GUID) -> RPC_STATUS;
        }
        ::std::mem::transmute(UuidCreateSequential(::std::mem::transmute(uuid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn UuidEqual(uuid1: *const ::windows::runtime::GUID, uuid2: *const ::windows::runtime::GUID, status: *mut RPC_STATUS) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UuidEqual(uuid1: *const ::windows::runtime::GUID, uuid2: *const ::windows::runtime::GUID, status: *mut RPC_STATUS) -> i32;
        }
        ::std::mem::transmute(UuidEqual(::std::mem::transmute(uuid1), ::std::mem::transmute(uuid2), ::std::mem::transmute(status)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn UuidFromStringA(stringuuid: *const u8, uuid: *mut ::windows::runtime::GUID) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UuidFromStringA(stringuuid: *const u8, uuid: *mut ::windows::runtime::GUID) -> RPC_STATUS;
        }
        ::std::mem::transmute(UuidFromStringA(::std::mem::transmute(stringuuid), ::std::mem::transmute(uuid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn UuidFromStringW(stringuuid: *const u16, uuid: *mut ::windows::runtime::GUID) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UuidFromStringW(stringuuid: *const u16, uuid: *mut ::windows::runtime::GUID) -> RPC_STATUS;
        }
        ::std::mem::transmute(UuidFromStringW(::std::mem::transmute(stringuuid), ::std::mem::transmute(uuid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn UuidHash(uuid: *const ::windows::runtime::GUID, status: *mut RPC_STATUS) -> u16 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UuidHash(uuid: *const ::windows::runtime::GUID, status: *mut RPC_STATUS) -> u16;
        }
        ::std::mem::transmute(UuidHash(::std::mem::transmute(uuid), ::std::mem::transmute(status)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn UuidIsNil(uuid: *const ::windows::runtime::GUID, status: *mut RPC_STATUS) -> i32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UuidIsNil(uuid: *const ::windows::runtime::GUID, status: *mut RPC_STATUS) -> i32;
        }
        ::std::mem::transmute(UuidIsNil(::std::mem::transmute(uuid), ::std::mem::transmute(status)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn UuidToStringA(uuid: *const ::windows::runtime::GUID, stringuuid: *mut *mut u8) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UuidToStringA(uuid: *const ::windows::runtime::GUID, stringuuid: *mut *mut u8) -> RPC_STATUS;
        }
        ::std::mem::transmute(UuidToStringA(::std::mem::transmute(uuid), ::std::mem::transmute(stringuuid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[inline]
pub unsafe fn UuidToStringW(uuid: *const ::windows::runtime::GUID, stringuuid: *mut *mut u16) -> RPC_STATUS {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UuidToStringW(uuid: *const ::windows::runtime::GUID, stringuuid: *mut *mut u16) -> RPC_STATUS;
        }
        ::std::mem::transmute(UuidToStringW(::std::mem::transmute(uuid), ::std::mem::transmute(stringuuid)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct XLAT_SIDE(pub i32);
pub const XLAT_SERVER: XLAT_SIDE = XLAT_SIDE(1i32);
pub const XLAT_CLIENT: XLAT_SIDE = XLAT_SIDE(2i32);
impl ::std::convert::From<i32> for XLAT_SIDE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for XLAT_SIDE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
#[cfg(feature = "Win32_System_Com")]
pub type XMIT_HELPER_ROUTINE = unsafe extern "system" fn(param0: *mut ::std::mem::ManuallyDrop<MIDL_STUB_MESSAGE>);
#[derive(:: std :: clone :: Clone)]
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[doc = "*Required features: `Win32_System_Rpc`, `Win32_System_Com`*"]
pub struct XMIT_ROUTINE_QUINTUPLE {
    pub pfnTranslateToXmit: ::std::option::Option<XMIT_HELPER_ROUTINE>,
    pub pfnTranslateFromXmit: ::std::option::Option<XMIT_HELPER_ROUTINE>,
    pub pfnFreeXmit: ::std::option::Option<XMIT_HELPER_ROUTINE>,
    pub pfnFreeInst: ::std::option::Option<XMIT_HELPER_ROUTINE>,
}
#[cfg(feature = "Win32_System_Com")]
impl XMIT_ROUTINE_QUINTUPLE {}
#[cfg(feature = "Win32_System_Com")]
impl ::std::default::Default for XMIT_ROUTINE_QUINTUPLE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::fmt::Debug for XMIT_ROUTINE_QUINTUPLE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("XMIT_ROUTINE_QUINTUPLE").finish()
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::PartialEq for XMIT_ROUTINE_QUINTUPLE {
    fn eq(&self, other: &Self) -> bool {
        self.pfnTranslateToXmit.map(|f| f as usize) == other.pfnTranslateToXmit.map(|f| f as usize) && self.pfnTranslateFromXmit.map(|f| f as usize) == other.pfnTranslateFromXmit.map(|f| f as usize) && self.pfnFreeXmit.map(|f| f as usize) == other.pfnFreeXmit.map(|f| f as usize) && self.pfnFreeInst.map(|f| f as usize) == other.pfnFreeInst.map(|f| f as usize)
    }
}
#[cfg(feature = "Win32_System_Com")]
impl ::std::cmp::Eq for XMIT_ROUTINE_QUINTUPLE {}
#[cfg(feature = "Win32_System_Com")]
unsafe impl ::windows::runtime::Abi for XMIT_ROUTINE_QUINTUPLE {
    type Abi = ::std::mem::ManuallyDrop<Self>;
}
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct _NDR_ASYNC_MESSAGE(pub u8);
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct _NDR_CORRELATION_INFO(pub u8);
#[repr(C)]
#[derive(:: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug, :: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy)]
pub struct _NDR_PROC_CONTEXT(pub u8);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub struct _NDR_SCONTEXT {
    pub pad: [*mut ::std::ffi::c_void; 2],
    pub userContext: *mut ::std::ffi::c_void,
}
impl _NDR_SCONTEXT {}
impl ::std::default::Default for _NDR_SCONTEXT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for _NDR_SCONTEXT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("_NDR_SCONTEXT").field("pad", &self.pad).field("userContext", &self.userContext).finish()
    }
}
impl ::std::cmp::PartialEq for _NDR_SCONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.pad == other.pad && self.userContext == other.userContext
    }
}
impl ::std::cmp::Eq for _NDR_SCONTEXT {}
unsafe impl ::windows::runtime::Abi for _NDR_SCONTEXT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Rpc`*"]
pub const __RPCPROXY_H_VERSION__: u32 = 475u32;
#[doc = "*Required features: `Win32_System_Rpc`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct system_handle_t(pub i32);
pub const SYSTEM_HANDLE_FILE: system_handle_t = system_handle_t(0i32);
pub const SYSTEM_HANDLE_SEMAPHORE: system_handle_t = system_handle_t(1i32);
pub const SYSTEM_HANDLE_EVENT: system_handle_t = system_handle_t(2i32);
pub const SYSTEM_HANDLE_MUTEX: system_handle_t = system_handle_t(3i32);
pub const SYSTEM_HANDLE_PROCESS: system_handle_t = system_handle_t(4i32);
pub const SYSTEM_HANDLE_TOKEN: system_handle_t = system_handle_t(5i32);
pub const SYSTEM_HANDLE_SECTION: system_handle_t = system_handle_t(6i32);
pub const SYSTEM_HANDLE_REG_KEY: system_handle_t = system_handle_t(7i32);
pub const SYSTEM_HANDLE_THREAD: system_handle_t = system_handle_t(8i32);
pub const SYSTEM_HANDLE_COMPOSITION_OBJECT: system_handle_t = system_handle_t(9i32);
pub const SYSTEM_HANDLE_SOCKET: system_handle_t = system_handle_t(10i32);
pub const SYSTEM_HANDLE_JOB: system_handle_t = system_handle_t(11i32);
pub const SYSTEM_HANDLE_PIPE: system_handle_t = system_handle_t(12i32);
pub const SYSTEM_HANDLE_MAX: system_handle_t = system_handle_t(12i32);
pub const SYSTEM_HANDLE_INVALID: system_handle_t = system_handle_t(255i32);
impl ::std::convert::From<i32> for system_handle_t {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for system_handle_t {
    type Abi = Self;
}
