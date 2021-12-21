#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_DeviceQuery', 'Win32_Devices_Properties', 'Win32_Foundation'*"]
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
pub struct DEVPROP_FILTER_EXPRESSION {
    pub Operator: DEVPROP_OPERATOR,
    pub Property: super::Properties::DEVPROPERTY,
}
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
impl ::core::marker::Copy for DEVPROP_FILTER_EXPRESSION {}
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
impl ::core::clone::Clone for DEVPROP_FILTER_EXPRESSION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
impl ::core::fmt::Debug for DEVPROP_FILTER_EXPRESSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEVPROP_FILTER_EXPRESSION").field("Operator", &self.Operator).field("Property", &self.Property).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
unsafe impl ::windows::core::Abi for DEVPROP_FILTER_EXPRESSION {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
impl ::core::cmp::PartialEq for DEVPROP_FILTER_EXPRESSION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEVPROP_FILTER_EXPRESSION>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
impl ::core::cmp::Eq for DEVPROP_FILTER_EXPRESSION {}
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
impl ::core::default::Default for DEVPROP_FILTER_EXPRESSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub type DEVPROP_OPERATOR = u32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DEVPROP_OPERATOR_MODIFIER_NOT: DEVPROP_OPERATOR = 65536u32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DEVPROP_OPERATOR_MODIFIER_IGNORE_CASE: DEVPROP_OPERATOR = 131072u32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DEVPROP_OPERATOR_NONE: DEVPROP_OPERATOR = 0u32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DEVPROP_OPERATOR_EXISTS: DEVPROP_OPERATOR = 1u32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DEVPROP_OPERATOR_NOT_EXISTS: DEVPROP_OPERATOR = 65537u32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DEVPROP_OPERATOR_EQUALS: DEVPROP_OPERATOR = 2u32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DEVPROP_OPERATOR_NOT_EQUALS: DEVPROP_OPERATOR = 65538u32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DEVPROP_OPERATOR_GREATER_THAN: DEVPROP_OPERATOR = 3u32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DEVPROP_OPERATOR_LESS_THAN: DEVPROP_OPERATOR = 4u32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DEVPROP_OPERATOR_GREATER_THAN_EQUALS: DEVPROP_OPERATOR = 5u32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DEVPROP_OPERATOR_LESS_THAN_EQUALS: DEVPROP_OPERATOR = 6u32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DEVPROP_OPERATOR_EQUALS_IGNORE_CASE: DEVPROP_OPERATOR = 131074u32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DEVPROP_OPERATOR_NOT_EQUALS_IGNORE_CASE: DEVPROP_OPERATOR = 196610u32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DEVPROP_OPERATOR_BITWISE_AND: DEVPROP_OPERATOR = 7u32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DEVPROP_OPERATOR_BITWISE_OR: DEVPROP_OPERATOR = 8u32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DEVPROP_OPERATOR_BEGINS_WITH: DEVPROP_OPERATOR = 9u32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DEVPROP_OPERATOR_ENDS_WITH: DEVPROP_OPERATOR = 10u32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DEVPROP_OPERATOR_CONTAINS: DEVPROP_OPERATOR = 11u32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DEVPROP_OPERATOR_BEGINS_WITH_IGNORE_CASE: DEVPROP_OPERATOR = 131081u32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DEVPROP_OPERATOR_ENDS_WITH_IGNORE_CASE: DEVPROP_OPERATOR = 131082u32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DEVPROP_OPERATOR_CONTAINS_IGNORE_CASE: DEVPROP_OPERATOR = 131083u32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DEVPROP_OPERATOR_LIST_CONTAINS: DEVPROP_OPERATOR = 4096u32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DEVPROP_OPERATOR_LIST_ELEMENT_BEGINS_WITH: DEVPROP_OPERATOR = 8192u32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DEVPROP_OPERATOR_LIST_ELEMENT_ENDS_WITH: DEVPROP_OPERATOR = 12288u32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DEVPROP_OPERATOR_LIST_ELEMENT_CONTAINS: DEVPROP_OPERATOR = 16384u32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DEVPROP_OPERATOR_LIST_CONTAINS_IGNORE_CASE: DEVPROP_OPERATOR = 135168u32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DEVPROP_OPERATOR_LIST_ELEMENT_BEGINS_WITH_IGNORE_CASE: DEVPROP_OPERATOR = 139264u32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DEVPROP_OPERATOR_LIST_ELEMENT_ENDS_WITH_IGNORE_CASE: DEVPROP_OPERATOR = 143360u32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DEVPROP_OPERATOR_LIST_ELEMENT_CONTAINS_IGNORE_CASE: DEVPROP_OPERATOR = 147456u32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DEVPROP_OPERATOR_AND_OPEN: DEVPROP_OPERATOR = 1048576u32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DEVPROP_OPERATOR_AND_CLOSE: DEVPROP_OPERATOR = 2097152u32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DEVPROP_OPERATOR_OR_OPEN: DEVPROP_OPERATOR = 3145728u32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DEVPROP_OPERATOR_OR_CLOSE: DEVPROP_OPERATOR = 4194304u32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DEVPROP_OPERATOR_NOT_OPEN: DEVPROP_OPERATOR = 5242880u32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DEVPROP_OPERATOR_NOT_CLOSE: DEVPROP_OPERATOR = 6291456u32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DEVPROP_OPERATOR_ARRAY_CONTAINS: DEVPROP_OPERATOR = 268435456u32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DEVPROP_OPERATOR_MASK_EVAL: DEVPROP_OPERATOR = 4095u32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DEVPROP_OPERATOR_MASK_LIST: DEVPROP_OPERATOR = 61440u32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DEVPROP_OPERATOR_MASK_MODIFIER: DEVPROP_OPERATOR = 983040u32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DEVPROP_OPERATOR_MASK_NOT_LOGICAL: DEVPROP_OPERATOR = 4027580415u32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DEVPROP_OPERATOR_MASK_LOGICAL: DEVPROP_OPERATOR = 267386880u32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DEVPROP_OPERATOR_MASK_ARRAY: DEVPROP_OPERATOR = 4026531840u32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_DeviceQuery', 'Win32_Devices_Properties', 'Win32_Foundation'*"]
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
pub struct DEV_OBJECT {
    pub ObjectType: DEV_OBJECT_TYPE,
    pub pszObjectId: super::super::Foundation::PWSTR,
    pub cPropertyCount: u32,
    pub pProperties: *mut super::Properties::DEVPROPERTY,
}
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
impl ::core::marker::Copy for DEV_OBJECT {}
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
impl ::core::clone::Clone for DEV_OBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
impl ::core::fmt::Debug for DEV_OBJECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEV_OBJECT").field("ObjectType", &self.ObjectType).field("pszObjectId", &self.pszObjectId).field("cPropertyCount", &self.cPropertyCount).field("pProperties", &self.pProperties).finish()
    }
}
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
unsafe impl ::windows::core::Abi for DEV_OBJECT {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
impl ::core::cmp::PartialEq for DEV_OBJECT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEV_OBJECT>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
impl ::core::cmp::Eq for DEV_OBJECT {}
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
impl ::core::default::Default for DEV_OBJECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub type DEV_OBJECT_TYPE = i32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DevObjectTypeUnknown: DEV_OBJECT_TYPE = 0i32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DevObjectTypeDeviceInterface: DEV_OBJECT_TYPE = 1i32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DevObjectTypeDeviceContainer: DEV_OBJECT_TYPE = 2i32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DevObjectTypeDevice: DEV_OBJECT_TYPE = 3i32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DevObjectTypeDeviceInterfaceClass: DEV_OBJECT_TYPE = 4i32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DevObjectTypeAEP: DEV_OBJECT_TYPE = 5i32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DevObjectTypeAEPContainer: DEV_OBJECT_TYPE = 6i32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DevObjectTypeDeviceInstallerClass: DEV_OBJECT_TYPE = 7i32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DevObjectTypeDeviceInterfaceDisplay: DEV_OBJECT_TYPE = 8i32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DevObjectTypeDeviceContainerDisplay: DEV_OBJECT_TYPE = 9i32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DevObjectTypeAEPService: DEV_OBJECT_TYPE = 10i32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DevObjectTypeDevicePanel: DEV_OBJECT_TYPE = 11i32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub type DEV_QUERY_FLAGS = i32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DevQueryFlagNone: DEV_QUERY_FLAGS = 0i32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DevQueryFlagUpdateResults: DEV_QUERY_FLAGS = 1i32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DevQueryFlagAllProperties: DEV_QUERY_FLAGS = 2i32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DevQueryFlagLocalize: DEV_QUERY_FLAGS = 4i32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DevQueryFlagAsyncClose: DEV_QUERY_FLAGS = 8i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_DeviceQuery', 'Win32_Devices_Properties'*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub struct DEV_QUERY_PARAMETER {
    pub Key: super::Properties::DEVPROPKEY,
    pub Type: u32,
    pub BufferSize: u32,
    pub Buffer: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_Devices_Properties")]
impl ::core::marker::Copy for DEV_QUERY_PARAMETER {}
#[cfg(feature = "Win32_Devices_Properties")]
impl ::core::clone::Clone for DEV_QUERY_PARAMETER {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Devices_Properties")]
impl ::core::fmt::Debug for DEV_QUERY_PARAMETER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DEV_QUERY_PARAMETER").field("Key", &self.Key).field("Type", &self.Type).field("BufferSize", &self.BufferSize).field("Buffer", &self.Buffer).finish()
    }
}
#[cfg(feature = "Win32_Devices_Properties")]
unsafe impl ::windows::core::Abi for DEV_QUERY_PARAMETER {
    type Abi = Self;
}
#[cfg(feature = "Win32_Devices_Properties")]
impl ::core::cmp::PartialEq for DEV_QUERY_PARAMETER {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEV_QUERY_PARAMETER>()) == 0 }
    }
}
#[cfg(feature = "Win32_Devices_Properties")]
impl ::core::cmp::Eq for DEV_QUERY_PARAMETER {}
#[cfg(feature = "Win32_Devices_Properties")]
impl ::core::default::Default for DEV_QUERY_PARAMETER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub type DEV_QUERY_RESULT_ACTION = i32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DevQueryResultStateChange: DEV_QUERY_RESULT_ACTION = 0i32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DevQueryResultAdd: DEV_QUERY_RESULT_ACTION = 1i32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DevQueryResultUpdate: DEV_QUERY_RESULT_ACTION = 2i32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DevQueryResultRemove: DEV_QUERY_RESULT_ACTION = 3i32;
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_DeviceQuery', 'Win32_Devices_Properties', 'Win32_Foundation'*"]
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
pub struct DEV_QUERY_RESULT_ACTION_DATA {
    pub Action: DEV_QUERY_RESULT_ACTION,
    pub Data: DEV_QUERY_RESULT_ACTION_DATA_0,
}
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
impl ::core::marker::Copy for DEV_QUERY_RESULT_ACTION_DATA {}
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
impl ::core::clone::Clone for DEV_QUERY_RESULT_ACTION_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
unsafe impl ::windows::core::Abi for DEV_QUERY_RESULT_ACTION_DATA {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
impl ::core::cmp::PartialEq for DEV_QUERY_RESULT_ACTION_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEV_QUERY_RESULT_ACTION_DATA>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
impl ::core::cmp::Eq for DEV_QUERY_RESULT_ACTION_DATA {}
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
impl ::core::default::Default for DEV_QUERY_RESULT_ACTION_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_DeviceQuery', 'Win32_Devices_Properties', 'Win32_Foundation'*"]
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
pub union DEV_QUERY_RESULT_ACTION_DATA_0 {
    pub State: DEV_QUERY_STATE,
    pub DeviceObject: DEV_OBJECT,
}
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
impl ::core::marker::Copy for DEV_QUERY_RESULT_ACTION_DATA_0 {}
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
impl ::core::clone::Clone for DEV_QUERY_RESULT_ACTION_DATA_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
unsafe impl ::windows::core::Abi for DEV_QUERY_RESULT_ACTION_DATA_0 {
    type Abi = Self;
}
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
impl ::core::cmp::PartialEq for DEV_QUERY_RESULT_ACTION_DATA_0 {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<DEV_QUERY_RESULT_ACTION_DATA_0>()) == 0 }
    }
}
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
impl ::core::cmp::Eq for DEV_QUERY_RESULT_ACTION_DATA_0 {}
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
impl ::core::default::Default for DEV_QUERY_RESULT_ACTION_DATA_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub type DEV_QUERY_STATE = i32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DevQueryStateInitialized: DEV_QUERY_STATE = 0i32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DevQueryStateEnumCompleted: DEV_QUERY_STATE = 1i32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DevQueryStateAborted: DEV_QUERY_STATE = 2i32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub const DevQueryStateClosed: DEV_QUERY_STATE = 3i32;
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
#[inline]
pub unsafe fn DevCloseObjectQuery(hdevquery: *const HDEVQUERY__) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DevCloseObjectQuery(hdevquery: *const HDEVQUERY__);
        }
        DevCloseObjectQuery(::core::mem::transmute(hdevquery))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_DeviceQuery', 'Win32_Devices_Properties', 'Win32_Foundation'*"]
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
#[inline]
pub unsafe fn DevCreateObjectQuery(objecttype: DEV_OBJECT_TYPE, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount: u32, pfilter: *const DEVPROP_FILTER_EXPRESSION, pcallback: PDEV_QUERY_RESULT_CALLBACK, pcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<*mut HDEVQUERY__> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DevCreateObjectQuery(objecttype: DEV_OBJECT_TYPE, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount: u32, pfilter: *const DEVPROP_FILTER_EXPRESSION, pcallback: ::windows::core::RawPtr, pcontext: *const ::core::ffi::c_void, phdevquery: *mut *mut HDEVQUERY__) -> ::windows::core::HRESULT;
        }
        let mut result__: *mut HDEVQUERY__ = ::core::mem::zeroed();
        DevCreateObjectQuery(::core::mem::transmute(objecttype), ::core::mem::transmute(queryflags), ::core::mem::transmute(crequestedproperties), ::core::mem::transmute(prequestedproperties), ::core::mem::transmute(cfilterexpressioncount), ::core::mem::transmute(pfilter), ::core::mem::transmute(pcallback), ::core::mem::transmute(pcontext), ::core::mem::transmute(&mut result__)).from_abi::<*mut HDEVQUERY__>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_DeviceQuery', 'Win32_Devices_Properties', 'Win32_Foundation'*"]
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
#[inline]
pub unsafe fn DevCreateObjectQueryEx(objecttype: DEV_OBJECT_TYPE, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount: u32, pfilter: *const DEVPROP_FILTER_EXPRESSION, cextendedparametercount: u32, pextendedparameters: *const DEV_QUERY_PARAMETER, pcallback: PDEV_QUERY_RESULT_CALLBACK, pcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<*mut HDEVQUERY__> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DevCreateObjectQueryEx(objecttype: DEV_OBJECT_TYPE, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount: u32, pfilter: *const DEVPROP_FILTER_EXPRESSION, cextendedparametercount: u32, pextendedparameters: *const DEV_QUERY_PARAMETER, pcallback: ::windows::core::RawPtr, pcontext: *const ::core::ffi::c_void, phdevquery: *mut *mut HDEVQUERY__) -> ::windows::core::HRESULT;
        }
        let mut result__: *mut HDEVQUERY__ = ::core::mem::zeroed();
        DevCreateObjectQueryEx(::core::mem::transmute(objecttype), ::core::mem::transmute(queryflags), ::core::mem::transmute(crequestedproperties), ::core::mem::transmute(prequestedproperties), ::core::mem::transmute(cfilterexpressioncount), ::core::mem::transmute(pfilter), ::core::mem::transmute(cextendedparametercount), ::core::mem::transmute(pextendedparameters), ::core::mem::transmute(pcallback), ::core::mem::transmute(pcontext), ::core::mem::transmute(&mut result__)).from_abi::<*mut HDEVQUERY__>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_DeviceQuery', 'Win32_Devices_Properties', 'Win32_Foundation'*"]
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
#[inline]
pub unsafe fn DevCreateObjectQueryFromId<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(objecttype: DEV_OBJECT_TYPE, pszobjectid: Param1, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount: u32, pfilter: *const DEVPROP_FILTER_EXPRESSION, pcallback: PDEV_QUERY_RESULT_CALLBACK, pcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<*mut HDEVQUERY__> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DevCreateObjectQueryFromId(objecttype: DEV_OBJECT_TYPE, pszobjectid: super::super::Foundation::PWSTR, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount: u32, pfilter: *const DEVPROP_FILTER_EXPRESSION, pcallback: ::windows::core::RawPtr, pcontext: *const ::core::ffi::c_void, phdevquery: *mut *mut HDEVQUERY__) -> ::windows::core::HRESULT;
        }
        let mut result__: *mut HDEVQUERY__ = ::core::mem::zeroed();
        DevCreateObjectQueryFromId(::core::mem::transmute(objecttype), pszobjectid.into_param().abi(), ::core::mem::transmute(queryflags), ::core::mem::transmute(crequestedproperties), ::core::mem::transmute(prequestedproperties), ::core::mem::transmute(cfilterexpressioncount), ::core::mem::transmute(pfilter), ::core::mem::transmute(pcallback), ::core::mem::transmute(pcontext), ::core::mem::transmute(&mut result__)).from_abi::<*mut HDEVQUERY__>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_DeviceQuery', 'Win32_Devices_Properties', 'Win32_Foundation'*"]
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
#[inline]
pub unsafe fn DevCreateObjectQueryFromIdEx<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(objecttype: DEV_OBJECT_TYPE, pszobjectid: Param1, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount: u32, pfilter: *const DEVPROP_FILTER_EXPRESSION, cextendedparametercount: u32, pextendedparameters: *const DEV_QUERY_PARAMETER, pcallback: PDEV_QUERY_RESULT_CALLBACK, pcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<*mut HDEVQUERY__> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DevCreateObjectQueryFromIdEx(objecttype: DEV_OBJECT_TYPE, pszobjectid: super::super::Foundation::PWSTR, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount: u32, pfilter: *const DEVPROP_FILTER_EXPRESSION, cextendedparametercount: u32, pextendedparameters: *const DEV_QUERY_PARAMETER, pcallback: ::windows::core::RawPtr, pcontext: *const ::core::ffi::c_void, phdevquery: *mut *mut HDEVQUERY__) -> ::windows::core::HRESULT;
        }
        let mut result__: *mut HDEVQUERY__ = ::core::mem::zeroed();
        DevCreateObjectQueryFromIdEx(::core::mem::transmute(objecttype), pszobjectid.into_param().abi(), ::core::mem::transmute(queryflags), ::core::mem::transmute(crequestedproperties), ::core::mem::transmute(prequestedproperties), ::core::mem::transmute(cfilterexpressioncount), ::core::mem::transmute(pfilter), ::core::mem::transmute(cextendedparametercount), ::core::mem::transmute(pextendedparameters), ::core::mem::transmute(pcallback), ::core::mem::transmute(pcontext), ::core::mem::transmute(&mut result__)).from_abi::<*mut HDEVQUERY__>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_DeviceQuery', 'Win32_Devices_Properties', 'Win32_Foundation'*"]
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
#[inline]
pub unsafe fn DevCreateObjectQueryFromIds<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(objecttype: DEV_OBJECT_TYPE, pszzobjectids: Param1, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount: u32, pfilter: *const DEVPROP_FILTER_EXPRESSION, pcallback: PDEV_QUERY_RESULT_CALLBACK, pcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<*mut HDEVQUERY__> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DevCreateObjectQueryFromIds(objecttype: DEV_OBJECT_TYPE, pszzobjectids: super::super::Foundation::PWSTR, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount: u32, pfilter: *const DEVPROP_FILTER_EXPRESSION, pcallback: ::windows::core::RawPtr, pcontext: *const ::core::ffi::c_void, phdevquery: *mut *mut HDEVQUERY__) -> ::windows::core::HRESULT;
        }
        let mut result__: *mut HDEVQUERY__ = ::core::mem::zeroed();
        DevCreateObjectQueryFromIds(::core::mem::transmute(objecttype), pszzobjectids.into_param().abi(), ::core::mem::transmute(queryflags), ::core::mem::transmute(crequestedproperties), ::core::mem::transmute(prequestedproperties), ::core::mem::transmute(cfilterexpressioncount), ::core::mem::transmute(pfilter), ::core::mem::transmute(pcallback), ::core::mem::transmute(pcontext), ::core::mem::transmute(&mut result__)).from_abi::<*mut HDEVQUERY__>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_DeviceQuery', 'Win32_Devices_Properties', 'Win32_Foundation'*"]
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
#[inline]
pub unsafe fn DevCreateObjectQueryFromIdsEx<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(objecttype: DEV_OBJECT_TYPE, pszzobjectids: Param1, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount: u32, pfilter: *const DEVPROP_FILTER_EXPRESSION, cextendedparametercount: u32, pextendedparameters: *const DEV_QUERY_PARAMETER, pcallback: PDEV_QUERY_RESULT_CALLBACK, pcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<*mut HDEVQUERY__> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DevCreateObjectQueryFromIdsEx(objecttype: DEV_OBJECT_TYPE, pszzobjectids: super::super::Foundation::PWSTR, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount: u32, pfilter: *const DEVPROP_FILTER_EXPRESSION, cextendedparametercount: u32, pextendedparameters: *const DEV_QUERY_PARAMETER, pcallback: ::windows::core::RawPtr, pcontext: *const ::core::ffi::c_void, phdevquery: *mut *mut HDEVQUERY__) -> ::windows::core::HRESULT;
        }
        let mut result__: *mut HDEVQUERY__ = ::core::mem::zeroed();
        DevCreateObjectQueryFromIdsEx(::core::mem::transmute(objecttype), pszzobjectids.into_param().abi(), ::core::mem::transmute(queryflags), ::core::mem::transmute(crequestedproperties), ::core::mem::transmute(prequestedproperties), ::core::mem::transmute(cfilterexpressioncount), ::core::mem::transmute(pfilter), ::core::mem::transmute(cextendedparametercount), ::core::mem::transmute(pextendedparameters), ::core::mem::transmute(pcallback), ::core::mem::transmute(pcontext), ::core::mem::transmute(&mut result__)).from_abi::<*mut HDEVQUERY__>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_DeviceQuery', 'Win32_Devices_Properties', 'Win32_Foundation'*"]
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
#[inline]
pub unsafe fn DevFindProperty<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(pkey: *const super::Properties::DEVPROPKEY, store: super::Properties::DEVPROPSTORE, pszlocalename: Param2, cproperties: u32, pproperties: *const super::Properties::DEVPROPERTY) -> *mut super::Properties::DEVPROPERTY {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DevFindProperty(pkey: *const super::Properties::DEVPROPKEY, store: super::Properties::DEVPROPSTORE, pszlocalename: super::super::Foundation::PWSTR, cproperties: u32, pproperties: *const super::Properties::DEVPROPERTY) -> *mut super::Properties::DEVPROPERTY;
        }
        ::core::mem::transmute(DevFindProperty(::core::mem::transmute(pkey), ::core::mem::transmute(store), pszlocalename.into_param().abi(), ::core::mem::transmute(cproperties), ::core::mem::transmute(pproperties)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_DeviceQuery', 'Win32_Devices_Properties', 'Win32_Foundation'*"]
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
#[inline]
pub unsafe fn DevFreeObjectProperties(cpropertycount: u32, pproperties: *const super::Properties::DEVPROPERTY) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DevFreeObjectProperties(cpropertycount: u32, pproperties: *const super::Properties::DEVPROPERTY);
        }
        DevFreeObjectProperties(::core::mem::transmute(cpropertycount), ::core::mem::transmute(pproperties))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_DeviceQuery', 'Win32_Devices_Properties', 'Win32_Foundation'*"]
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
#[inline]
pub unsafe fn DevFreeObjects(cobjectcount: u32, pobjects: *const DEV_OBJECT) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DevFreeObjects(cobjectcount: u32, pobjects: *const DEV_OBJECT);
        }
        DevFreeObjects(::core::mem::transmute(cobjectcount), ::core::mem::transmute(pobjects))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_DeviceQuery', 'Win32_Devices_Properties', 'Win32_Foundation'*"]
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
#[inline]
pub unsafe fn DevGetObjectProperties<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(objecttype: DEV_OBJECT_TYPE, pszobjectid: Param1, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, pcpropertycount: *mut u32, ppproperties: *mut *mut super::Properties::DEVPROPERTY) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DevGetObjectProperties(objecttype: DEV_OBJECT_TYPE, pszobjectid: super::super::Foundation::PWSTR, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, pcpropertycount: *mut u32, ppproperties: *mut *mut super::Properties::DEVPROPERTY) -> ::windows::core::HRESULT;
        }
        DevGetObjectProperties(::core::mem::transmute(objecttype), pszobjectid.into_param().abi(), ::core::mem::transmute(queryflags), ::core::mem::transmute(crequestedproperties), ::core::mem::transmute(prequestedproperties), ::core::mem::transmute(pcpropertycount), ::core::mem::transmute(ppproperties)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_DeviceQuery', 'Win32_Devices_Properties', 'Win32_Foundation'*"]
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
#[inline]
pub unsafe fn DevGetObjectPropertiesEx<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(objecttype: DEV_OBJECT_TYPE, pszobjectid: Param1, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, cextendedparametercount: u32, pextendedparameters: *const DEV_QUERY_PARAMETER, pcpropertycount: *mut u32, ppproperties: *mut *mut super::Properties::DEVPROPERTY) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DevGetObjectPropertiesEx(objecttype: DEV_OBJECT_TYPE, pszobjectid: super::super::Foundation::PWSTR, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, cextendedparametercount: u32, pextendedparameters: *const DEV_QUERY_PARAMETER, pcpropertycount: *mut u32, ppproperties: *mut *mut super::Properties::DEVPROPERTY) -> ::windows::core::HRESULT;
        }
        DevGetObjectPropertiesEx(::core::mem::transmute(objecttype), pszobjectid.into_param().abi(), ::core::mem::transmute(queryflags), ::core::mem::transmute(crequestedproperties), ::core::mem::transmute(prequestedproperties), ::core::mem::transmute(cextendedparametercount), ::core::mem::transmute(pextendedparameters), ::core::mem::transmute(pcpropertycount), ::core::mem::transmute(ppproperties)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_DeviceQuery', 'Win32_Devices_Properties', 'Win32_Foundation'*"]
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
#[inline]
pub unsafe fn DevGetObjects(objecttype: DEV_OBJECT_TYPE, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount: u32, pfilter: *const DEVPROP_FILTER_EXPRESSION, pcobjectcount: *mut u32, ppobjects: *mut *mut DEV_OBJECT) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DevGetObjects(objecttype: DEV_OBJECT_TYPE, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount: u32, pfilter: *const DEVPROP_FILTER_EXPRESSION, pcobjectcount: *mut u32, ppobjects: *mut *mut DEV_OBJECT) -> ::windows::core::HRESULT;
        }
        DevGetObjects(::core::mem::transmute(objecttype), ::core::mem::transmute(queryflags), ::core::mem::transmute(crequestedproperties), ::core::mem::transmute(prequestedproperties), ::core::mem::transmute(cfilterexpressioncount), ::core::mem::transmute(pfilter), ::core::mem::transmute(pcobjectcount), ::core::mem::transmute(ppobjects)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: 'Win32_Devices_DeviceQuery', 'Win32_Devices_Properties', 'Win32_Foundation'*"]
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
#[inline]
pub unsafe fn DevGetObjectsEx(objecttype: DEV_OBJECT_TYPE, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount: u32, pfilter: *const DEVPROP_FILTER_EXPRESSION, cextendedparametercount: u32, pextendedparameters: *const DEV_QUERY_PARAMETER, pcobjectcount: *mut u32, ppobjects: *mut *mut DEV_OBJECT) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn DevGetObjectsEx(objecttype: DEV_OBJECT_TYPE, queryflags: u32, crequestedproperties: u32, prequestedproperties: *const super::Properties::DEVPROPCOMPKEY, cfilterexpressioncount: u32, pfilter: *const DEVPROP_FILTER_EXPRESSION, cextendedparametercount: u32, pextendedparameters: *const DEV_QUERY_PARAMETER, pcobjectcount: *mut u32, ppobjects: *mut *mut DEV_OBJECT) -> ::windows::core::HRESULT;
        }
        DevGetObjectsEx(::core::mem::transmute(objecttype), ::core::mem::transmute(queryflags), ::core::mem::transmute(crequestedproperties), ::core::mem::transmute(prequestedproperties), ::core::mem::transmute(cfilterexpressioncount), ::core::mem::transmute(pfilter), ::core::mem::transmute(cextendedparametercount), ::core::mem::transmute(pextendedparameters), ::core::mem::transmute(pcobjectcount), ::core::mem::transmute(ppobjects)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: 'Win32_Devices_DeviceQuery'*"]
pub struct HDEVQUERY__ {
    pub unused: i32,
}
impl ::core::marker::Copy for HDEVQUERY__ {}
impl ::core::clone::Clone for HDEVQUERY__ {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HDEVQUERY__ {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HDEVQUERY__").field("unused", &self.unused).finish()
    }
}
unsafe impl ::windows::core::Abi for HDEVQUERY__ {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for HDEVQUERY__ {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<HDEVQUERY__>()) == 0 }
    }
}
impl ::core::cmp::Eq for HDEVQUERY__ {}
impl ::core::default::Default for HDEVQUERY__ {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: 'Win32_Devices_DeviceQuery', 'Win32_Devices_Properties', 'Win32_Foundation'*"]
#[cfg(all(feature = "Win32_Devices_Properties", feature = "Win32_Foundation"))]
pub type PDEV_QUERY_RESULT_CALLBACK = ::core::option::Option<unsafe extern "system" fn(hdevquery: *const HDEVQUERY__, pcontext: *const ::core::ffi::c_void, pactiondata: *const DEV_QUERY_RESULT_ACTION_DATA)>;
