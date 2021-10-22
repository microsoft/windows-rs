#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub struct DEVPROP_FILTER_EXPRESSION {
    pub Operator: DEVPROP_OPERATOR,
    pub Property: super::super::System::SystemServices::DEVPROPERTY,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl DEVPROP_FILTER_EXPRESSION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::default::Default for DEVPROP_FILTER_EXPRESSION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::fmt::Debug for DEVPROP_FILTER_EXPRESSION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DEVPROP_FILTER_EXPRESSION")
            .field("Operator", &self.Operator)
            .field("Property", &self.Property)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::PartialEq for DEVPROP_FILTER_EXPRESSION {
    fn eq(&self, other: &Self) -> bool {
        self.Operator == other.Operator && self.Property == other.Property
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::Eq for DEVPROP_FILTER_EXPRESSION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for DEVPROP_FILTER_EXPRESSION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DEVPROP_OPERATOR(pub u32);
pub const DEVPROP_OPERATOR_MODIFIER_NOT: DEVPROP_OPERATOR = DEVPROP_OPERATOR(65536u32);
pub const DEVPROP_OPERATOR_MODIFIER_IGNORE_CASE: DEVPROP_OPERATOR = DEVPROP_OPERATOR(131072u32);
pub const DEVPROP_OPERATOR_NONE: DEVPROP_OPERATOR = DEVPROP_OPERATOR(0u32);
pub const DEVPROP_OPERATOR_EXISTS: DEVPROP_OPERATOR = DEVPROP_OPERATOR(1u32);
pub const DEVPROP_OPERATOR_NOT_EXISTS: DEVPROP_OPERATOR = DEVPROP_OPERATOR(65537u32);
pub const DEVPROP_OPERATOR_EQUALS: DEVPROP_OPERATOR = DEVPROP_OPERATOR(2u32);
pub const DEVPROP_OPERATOR_NOT_EQUALS: DEVPROP_OPERATOR = DEVPROP_OPERATOR(65538u32);
pub const DEVPROP_OPERATOR_GREATER_THAN: DEVPROP_OPERATOR = DEVPROP_OPERATOR(3u32);
pub const DEVPROP_OPERATOR_LESS_THAN: DEVPROP_OPERATOR = DEVPROP_OPERATOR(4u32);
pub const DEVPROP_OPERATOR_GREATER_THAN_EQUALS: DEVPROP_OPERATOR = DEVPROP_OPERATOR(5u32);
pub const DEVPROP_OPERATOR_LESS_THAN_EQUALS: DEVPROP_OPERATOR = DEVPROP_OPERATOR(6u32);
pub const DEVPROP_OPERATOR_EQUALS_IGNORE_CASE: DEVPROP_OPERATOR = DEVPROP_OPERATOR(131074u32);
pub const DEVPROP_OPERATOR_NOT_EQUALS_IGNORE_CASE: DEVPROP_OPERATOR = DEVPROP_OPERATOR(196610u32);
pub const DEVPROP_OPERATOR_BITWISE_AND: DEVPROP_OPERATOR = DEVPROP_OPERATOR(7u32);
pub const DEVPROP_OPERATOR_BITWISE_OR: DEVPROP_OPERATOR = DEVPROP_OPERATOR(8u32);
pub const DEVPROP_OPERATOR_BEGINS_WITH: DEVPROP_OPERATOR = DEVPROP_OPERATOR(9u32);
pub const DEVPROP_OPERATOR_ENDS_WITH: DEVPROP_OPERATOR = DEVPROP_OPERATOR(10u32);
pub const DEVPROP_OPERATOR_CONTAINS: DEVPROP_OPERATOR = DEVPROP_OPERATOR(11u32);
pub const DEVPROP_OPERATOR_BEGINS_WITH_IGNORE_CASE: DEVPROP_OPERATOR = DEVPROP_OPERATOR(131081u32);
pub const DEVPROP_OPERATOR_ENDS_WITH_IGNORE_CASE: DEVPROP_OPERATOR = DEVPROP_OPERATOR(131082u32);
pub const DEVPROP_OPERATOR_CONTAINS_IGNORE_CASE: DEVPROP_OPERATOR = DEVPROP_OPERATOR(131083u32);
pub const DEVPROP_OPERATOR_LIST_CONTAINS: DEVPROP_OPERATOR = DEVPROP_OPERATOR(4096u32);
pub const DEVPROP_OPERATOR_LIST_ELEMENT_BEGINS_WITH: DEVPROP_OPERATOR = DEVPROP_OPERATOR(8192u32);
pub const DEVPROP_OPERATOR_LIST_ELEMENT_ENDS_WITH: DEVPROP_OPERATOR = DEVPROP_OPERATOR(12288u32);
pub const DEVPROP_OPERATOR_LIST_ELEMENT_CONTAINS: DEVPROP_OPERATOR = DEVPROP_OPERATOR(16384u32);
pub const DEVPROP_OPERATOR_LIST_CONTAINS_IGNORE_CASE: DEVPROP_OPERATOR =
    DEVPROP_OPERATOR(135168u32);
pub const DEVPROP_OPERATOR_LIST_ELEMENT_BEGINS_WITH_IGNORE_CASE: DEVPROP_OPERATOR =
    DEVPROP_OPERATOR(139264u32);
pub const DEVPROP_OPERATOR_LIST_ELEMENT_ENDS_WITH_IGNORE_CASE: DEVPROP_OPERATOR =
    DEVPROP_OPERATOR(143360u32);
pub const DEVPROP_OPERATOR_LIST_ELEMENT_CONTAINS_IGNORE_CASE: DEVPROP_OPERATOR =
    DEVPROP_OPERATOR(147456u32);
pub const DEVPROP_OPERATOR_AND_OPEN: DEVPROP_OPERATOR = DEVPROP_OPERATOR(1048576u32);
pub const DEVPROP_OPERATOR_AND_CLOSE: DEVPROP_OPERATOR = DEVPROP_OPERATOR(2097152u32);
pub const DEVPROP_OPERATOR_OR_OPEN: DEVPROP_OPERATOR = DEVPROP_OPERATOR(3145728u32);
pub const DEVPROP_OPERATOR_OR_CLOSE: DEVPROP_OPERATOR = DEVPROP_OPERATOR(4194304u32);
pub const DEVPROP_OPERATOR_NOT_OPEN: DEVPROP_OPERATOR = DEVPROP_OPERATOR(5242880u32);
pub const DEVPROP_OPERATOR_NOT_CLOSE: DEVPROP_OPERATOR = DEVPROP_OPERATOR(6291456u32);
pub const DEVPROP_OPERATOR_ARRAY_CONTAINS: DEVPROP_OPERATOR = DEVPROP_OPERATOR(268435456u32);
pub const DEVPROP_OPERATOR_MASK_EVAL: DEVPROP_OPERATOR = DEVPROP_OPERATOR(4095u32);
pub const DEVPROP_OPERATOR_MASK_LIST: DEVPROP_OPERATOR = DEVPROP_OPERATOR(61440u32);
pub const DEVPROP_OPERATOR_MASK_MODIFIER: DEVPROP_OPERATOR = DEVPROP_OPERATOR(983040u32);
pub const DEVPROP_OPERATOR_MASK_NOT_LOGICAL: DEVPROP_OPERATOR = DEVPROP_OPERATOR(4027580415u32);
pub const DEVPROP_OPERATOR_MASK_LOGICAL: DEVPROP_OPERATOR = DEVPROP_OPERATOR(267386880u32);
pub const DEVPROP_OPERATOR_MASK_ARRAY: DEVPROP_OPERATOR = DEVPROP_OPERATOR(4026531840u32);
impl ::std::convert::From<u32> for DEVPROP_OPERATOR {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DEVPROP_OPERATOR {
    type Abi = Self;
    type DefaultType = Self;
}
impl ::std::ops::BitOr for DEVPROP_OPERATOR {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for DEVPROP_OPERATOR {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for DEVPROP_OPERATOR {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for DEVPROP_OPERATOR {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for DEVPROP_OPERATOR {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub struct DEV_OBJECT {
    pub ObjectType: DEV_OBJECT_TYPE,
    pub pszObjectId: super::super::Foundation::PWSTR,
    pub cPropertyCount: u32,
    pub pProperties: *mut super::super::System::SystemServices::DEVPROPERTY,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl DEV_OBJECT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::default::Default for DEV_OBJECT {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::fmt::Debug for DEV_OBJECT {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DEV_OBJECT")
            .field("ObjectType", &self.ObjectType)
            .field("pszObjectId", &self.pszObjectId)
            .field("cPropertyCount", &self.cPropertyCount)
            .field("pProperties", &self.pProperties)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::PartialEq for DEV_OBJECT {
    fn eq(&self, other: &Self) -> bool {
        self.ObjectType == other.ObjectType
            && self.pszObjectId == other.pszObjectId
            && self.cPropertyCount == other.cPropertyCount
            && self.pProperties == other.pProperties
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::Eq for DEV_OBJECT {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for DEV_OBJECT {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DEV_OBJECT_TYPE(pub i32);
pub const DevObjectTypeUnknown: DEV_OBJECT_TYPE = DEV_OBJECT_TYPE(0i32);
pub const DevObjectTypeDeviceInterface: DEV_OBJECT_TYPE = DEV_OBJECT_TYPE(1i32);
pub const DevObjectTypeDeviceContainer: DEV_OBJECT_TYPE = DEV_OBJECT_TYPE(2i32);
pub const DevObjectTypeDevice: DEV_OBJECT_TYPE = DEV_OBJECT_TYPE(3i32);
pub const DevObjectTypeDeviceInterfaceClass: DEV_OBJECT_TYPE = DEV_OBJECT_TYPE(4i32);
pub const DevObjectTypeAEP: DEV_OBJECT_TYPE = DEV_OBJECT_TYPE(5i32);
pub const DevObjectTypeAEPContainer: DEV_OBJECT_TYPE = DEV_OBJECT_TYPE(6i32);
pub const DevObjectTypeDeviceInstallerClass: DEV_OBJECT_TYPE = DEV_OBJECT_TYPE(7i32);
pub const DevObjectTypeDeviceInterfaceDisplay: DEV_OBJECT_TYPE = DEV_OBJECT_TYPE(8i32);
pub const DevObjectTypeDeviceContainerDisplay: DEV_OBJECT_TYPE = DEV_OBJECT_TYPE(9i32);
pub const DevObjectTypeAEPService: DEV_OBJECT_TYPE = DEV_OBJECT_TYPE(10i32);
pub const DevObjectTypeDevicePanel: DEV_OBJECT_TYPE = DEV_OBJECT_TYPE(11i32);
impl ::std::convert::From<i32> for DEV_OBJECT_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DEV_OBJECT_TYPE {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DEV_QUERY_FLAGS(pub i32);
pub const DevQueryFlagNone: DEV_QUERY_FLAGS = DEV_QUERY_FLAGS(0i32);
pub const DevQueryFlagUpdateResults: DEV_QUERY_FLAGS = DEV_QUERY_FLAGS(1i32);
pub const DevQueryFlagAllProperties: DEV_QUERY_FLAGS = DEV_QUERY_FLAGS(2i32);
pub const DevQueryFlagLocalize: DEV_QUERY_FLAGS = DEV_QUERY_FLAGS(4i32);
pub const DevQueryFlagAsyncClose: DEV_QUERY_FLAGS = DEV_QUERY_FLAGS(8i32);
impl ::std::convert::From<i32> for DEV_QUERY_FLAGS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DEV_QUERY_FLAGS {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_System_SystemServices")]
pub struct DEV_QUERY_PARAMETER {
    pub Key: super::super::System::SystemServices::DEVPROPKEY,
    pub Type: u32,
    pub BufferSize: u32,
    pub Buffer: *mut ::std::ffi::c_void,
}
#[cfg(feature = "Win32_System_SystemServices")]
impl DEV_QUERY_PARAMETER {}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::default::Default for DEV_QUERY_PARAMETER {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::fmt::Debug for DEV_QUERY_PARAMETER {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("DEV_QUERY_PARAMETER")
            .field("Key", &self.Key)
            .field("Type", &self.Type)
            .field("BufferSize", &self.BufferSize)
            .field("Buffer", &self.Buffer)
            .finish()
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::PartialEq for DEV_QUERY_PARAMETER {
    fn eq(&self, other: &Self) -> bool {
        self.Key == other.Key
            && self.Type == other.Type
            && self.BufferSize == other.BufferSize
            && self.Buffer == other.Buffer
    }
}
#[cfg(feature = "Win32_System_SystemServices")]
impl ::std::cmp::Eq for DEV_QUERY_PARAMETER {}
#[cfg(feature = "Win32_System_SystemServices")]
unsafe impl ::windows::runtime::Abi for DEV_QUERY_PARAMETER {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DEV_QUERY_RESULT_ACTION(pub i32);
pub const DevQueryResultStateChange: DEV_QUERY_RESULT_ACTION = DEV_QUERY_RESULT_ACTION(0i32);
pub const DevQueryResultAdd: DEV_QUERY_RESULT_ACTION = DEV_QUERY_RESULT_ACTION(1i32);
pub const DevQueryResultUpdate: DEV_QUERY_RESULT_ACTION = DEV_QUERY_RESULT_ACTION(2i32);
pub const DevQueryResultRemove: DEV_QUERY_RESULT_ACTION = DEV_QUERY_RESULT_ACTION(3i32);
impl ::std::convert::From<i32> for DEV_QUERY_RESULT_ACTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DEV_QUERY_RESULT_ACTION {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub struct DEV_QUERY_RESULT_ACTION_DATA {
    pub Action: DEV_QUERY_RESULT_ACTION,
    pub Data: DEV_QUERY_RESULT_ACTION_DATA_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl DEV_QUERY_RESULT_ACTION_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::default::Default for DEV_QUERY_RESULT_ACTION_DATA {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::PartialEq for DEV_QUERY_RESULT_ACTION_DATA {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::Eq for DEV_QUERY_RESULT_ACTION_DATA {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for DEV_QUERY_RESULT_ACTION_DATA {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub union DEV_QUERY_RESULT_ACTION_DATA_0 {
    pub State: DEV_QUERY_STATE,
    pub DeviceObject: DEV_OBJECT,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl DEV_QUERY_RESULT_ACTION_DATA_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::default::Default for DEV_QUERY_RESULT_ACTION_DATA_0 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::PartialEq for DEV_QUERY_RESULT_ACTION_DATA_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
impl ::std::cmp::Eq for DEV_QUERY_RESULT_ACTION_DATA_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
unsafe impl ::windows::runtime::Abi for DEV_QUERY_RESULT_ACTION_DATA_0 {
    type Abi = Self;
    type DefaultType = Self;
}
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DEV_QUERY_STATE(pub i32);
pub const DevQueryStateInitialized: DEV_QUERY_STATE = DEV_QUERY_STATE(0i32);
pub const DevQueryStateEnumCompleted: DEV_QUERY_STATE = DEV_QUERY_STATE(1i32);
pub const DevQueryStateAborted: DEV_QUERY_STATE = DEV_QUERY_STATE(2i32);
pub const DevQueryStateClosed: DEV_QUERY_STATE = DEV_QUERY_STATE(3i32);
impl ::std::convert::From<i32> for DEV_QUERY_STATE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DEV_QUERY_STATE {
    type Abi = Self;
    type DefaultType = Self;
}
pub unsafe fn DevCloseObjectQuery(hdevquery: *const HDEVQUERY__) {
    #[cfg(windows)]
    {
        #[link(name = "api-ms-win-devices-query-l1-1-0")]
        extern "system" {
            fn DevCloseObjectQuery(hdevquery: *const HDEVQUERY__);
        }
        ::std::mem::transmute(DevCloseObjectQuery(::std::mem::transmute(hdevquery)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn DevCreateObjectQuery(
    objecttype: DEV_OBJECT_TYPE,
    queryflags: u32,
    crequestedproperties: u32,
    prequestedproperties: *const super::super::System::SystemServices::DEVPROPCOMPKEY,
    cfilterexpressioncount: u32,
    pfilter: *const DEVPROP_FILTER_EXPRESSION,
    pcallback: ::std::option::Option<PDEV_QUERY_RESULT_CALLBACK>,
    pcontext: *const ::std::ffi::c_void,
) -> ::windows::runtime::Result<*mut HDEVQUERY__> {
    #[cfg(windows)]
    {
        #[link(name = "api-ms-win-devices-query-l1-1-0")]
        extern "system" {
            fn DevCreateObjectQuery(
                objecttype: DEV_OBJECT_TYPE,
                queryflags: u32,
                crequestedproperties: u32,
                prequestedproperties: *const super::super::System::SystemServices::DEVPROPCOMPKEY,
                cfilterexpressioncount: u32,
                pfilter: *const DEVPROP_FILTER_EXPRESSION,
                pcallback: ::windows::runtime::RawPtr,
                pcontext: *const ::std::ffi::c_void,
                phdevquery: *mut *mut HDEVQUERY__,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <*mut HDEVQUERY__ as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        DevCreateObjectQuery(
            ::std::mem::transmute(objecttype),
            ::std::mem::transmute(queryflags),
            ::std::mem::transmute(crequestedproperties),
            ::std::mem::transmute(prequestedproperties),
            ::std::mem::transmute(cfilterexpressioncount),
            ::std::mem::transmute(pfilter),
            ::std::mem::transmute(pcallback),
            ::std::mem::transmute(pcontext),
            &mut result__,
        )
        .from_abi::<*mut HDEVQUERY__>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn DevCreateObjectQueryEx(
    objecttype: DEV_OBJECT_TYPE,
    queryflags: u32,
    crequestedproperties: u32,
    prequestedproperties: *const super::super::System::SystemServices::DEVPROPCOMPKEY,
    cfilterexpressioncount: u32,
    pfilter: *const DEVPROP_FILTER_EXPRESSION,
    cextendedparametercount: u32,
    pextendedparameters: *const DEV_QUERY_PARAMETER,
    pcallback: ::std::option::Option<PDEV_QUERY_RESULT_CALLBACK>,
    pcontext: *const ::std::ffi::c_void,
) -> ::windows::runtime::Result<*mut HDEVQUERY__> {
    #[cfg(windows)]
    {
        #[link(name = "api-ms-win-devices-query-l1-1-1")]
        extern "system" {
            fn DevCreateObjectQueryEx(
                objecttype: DEV_OBJECT_TYPE,
                queryflags: u32,
                crequestedproperties: u32,
                prequestedproperties: *const super::super::System::SystemServices::DEVPROPCOMPKEY,
                cfilterexpressioncount: u32,
                pfilter: *const DEVPROP_FILTER_EXPRESSION,
                cextendedparametercount: u32,
                pextendedparameters: *const DEV_QUERY_PARAMETER,
                pcallback: ::windows::runtime::RawPtr,
                pcontext: *const ::std::ffi::c_void,
                phdevquery: *mut *mut HDEVQUERY__,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <*mut HDEVQUERY__ as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        DevCreateObjectQueryEx(
            ::std::mem::transmute(objecttype),
            ::std::mem::transmute(queryflags),
            ::std::mem::transmute(crequestedproperties),
            ::std::mem::transmute(prequestedproperties),
            ::std::mem::transmute(cfilterexpressioncount),
            ::std::mem::transmute(pfilter),
            ::std::mem::transmute(cextendedparametercount),
            ::std::mem::transmute(pextendedparameters),
            ::std::mem::transmute(pcallback),
            ::std::mem::transmute(pcontext),
            &mut result__,
        )
        .from_abi::<*mut HDEVQUERY__>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn DevCreateObjectQueryFromId<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    objecttype: DEV_OBJECT_TYPE,
    pszobjectid: Param1,
    queryflags: u32,
    crequestedproperties: u32,
    prequestedproperties: *const super::super::System::SystemServices::DEVPROPCOMPKEY,
    cfilterexpressioncount: u32,
    pfilter: *const DEVPROP_FILTER_EXPRESSION,
    pcallback: ::std::option::Option<PDEV_QUERY_RESULT_CALLBACK>,
    pcontext: *const ::std::ffi::c_void,
) -> ::windows::runtime::Result<*mut HDEVQUERY__> {
    #[cfg(windows)]
    {
        #[link(name = "api-ms-win-devices-query-l1-1-0")]
        extern "system" {
            fn DevCreateObjectQueryFromId(
                objecttype: DEV_OBJECT_TYPE,
                pszobjectid: super::super::Foundation::PWSTR,
                queryflags: u32,
                crequestedproperties: u32,
                prequestedproperties: *const super::super::System::SystemServices::DEVPROPCOMPKEY,
                cfilterexpressioncount: u32,
                pfilter: *const DEVPROP_FILTER_EXPRESSION,
                pcallback: ::windows::runtime::RawPtr,
                pcontext: *const ::std::ffi::c_void,
                phdevquery: *mut *mut HDEVQUERY__,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <*mut HDEVQUERY__ as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        DevCreateObjectQueryFromId(
            ::std::mem::transmute(objecttype),
            pszobjectid.into_param().abi(),
            ::std::mem::transmute(queryflags),
            ::std::mem::transmute(crequestedproperties),
            ::std::mem::transmute(prequestedproperties),
            ::std::mem::transmute(cfilterexpressioncount),
            ::std::mem::transmute(pfilter),
            ::std::mem::transmute(pcallback),
            ::std::mem::transmute(pcontext),
            &mut result__,
        )
        .from_abi::<*mut HDEVQUERY__>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn DevCreateObjectQueryFromIdEx<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    objecttype: DEV_OBJECT_TYPE,
    pszobjectid: Param1,
    queryflags: u32,
    crequestedproperties: u32,
    prequestedproperties: *const super::super::System::SystemServices::DEVPROPCOMPKEY,
    cfilterexpressioncount: u32,
    pfilter: *const DEVPROP_FILTER_EXPRESSION,
    cextendedparametercount: u32,
    pextendedparameters: *const DEV_QUERY_PARAMETER,
    pcallback: ::std::option::Option<PDEV_QUERY_RESULT_CALLBACK>,
    pcontext: *const ::std::ffi::c_void,
) -> ::windows::runtime::Result<*mut HDEVQUERY__> {
    #[cfg(windows)]
    {
        #[link(name = "api-ms-win-devices-query-l1-1-1")]
        extern "system" {
            fn DevCreateObjectQueryFromIdEx(
                objecttype: DEV_OBJECT_TYPE,
                pszobjectid: super::super::Foundation::PWSTR,
                queryflags: u32,
                crequestedproperties: u32,
                prequestedproperties: *const super::super::System::SystemServices::DEVPROPCOMPKEY,
                cfilterexpressioncount: u32,
                pfilter: *const DEVPROP_FILTER_EXPRESSION,
                cextendedparametercount: u32,
                pextendedparameters: *const DEV_QUERY_PARAMETER,
                pcallback: ::windows::runtime::RawPtr,
                pcontext: *const ::std::ffi::c_void,
                phdevquery: *mut *mut HDEVQUERY__,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <*mut HDEVQUERY__ as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        DevCreateObjectQueryFromIdEx(
            ::std::mem::transmute(objecttype),
            pszobjectid.into_param().abi(),
            ::std::mem::transmute(queryflags),
            ::std::mem::transmute(crequestedproperties),
            ::std::mem::transmute(prequestedproperties),
            ::std::mem::transmute(cfilterexpressioncount),
            ::std::mem::transmute(pfilter),
            ::std::mem::transmute(cextendedparametercount),
            ::std::mem::transmute(pextendedparameters),
            ::std::mem::transmute(pcallback),
            ::std::mem::transmute(pcontext),
            &mut result__,
        )
        .from_abi::<*mut HDEVQUERY__>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn DevCreateObjectQueryFromIds<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    objecttype: DEV_OBJECT_TYPE,
    pszzobjectids: Param1,
    queryflags: u32,
    crequestedproperties: u32,
    prequestedproperties: *const super::super::System::SystemServices::DEVPROPCOMPKEY,
    cfilterexpressioncount: u32,
    pfilter: *const DEVPROP_FILTER_EXPRESSION,
    pcallback: ::std::option::Option<PDEV_QUERY_RESULT_CALLBACK>,
    pcontext: *const ::std::ffi::c_void,
) -> ::windows::runtime::Result<*mut HDEVQUERY__> {
    #[cfg(windows)]
    {
        #[link(name = "api-ms-win-devices-query-l1-1-0")]
        extern "system" {
            fn DevCreateObjectQueryFromIds(
                objecttype: DEV_OBJECT_TYPE,
                pszzobjectids: super::super::Foundation::PWSTR,
                queryflags: u32,
                crequestedproperties: u32,
                prequestedproperties: *const super::super::System::SystemServices::DEVPROPCOMPKEY,
                cfilterexpressioncount: u32,
                pfilter: *const DEVPROP_FILTER_EXPRESSION,
                pcallback: ::windows::runtime::RawPtr,
                pcontext: *const ::std::ffi::c_void,
                phdevquery: *mut *mut HDEVQUERY__,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <*mut HDEVQUERY__ as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        DevCreateObjectQueryFromIds(
            ::std::mem::transmute(objecttype),
            pszzobjectids.into_param().abi(),
            ::std::mem::transmute(queryflags),
            ::std::mem::transmute(crequestedproperties),
            ::std::mem::transmute(prequestedproperties),
            ::std::mem::transmute(cfilterexpressioncount),
            ::std::mem::transmute(pfilter),
            ::std::mem::transmute(pcallback),
            ::std::mem::transmute(pcontext),
            &mut result__,
        )
        .from_abi::<*mut HDEVQUERY__>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn DevCreateObjectQueryFromIdsEx<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    objecttype: DEV_OBJECT_TYPE,
    pszzobjectids: Param1,
    queryflags: u32,
    crequestedproperties: u32,
    prequestedproperties: *const super::super::System::SystemServices::DEVPROPCOMPKEY,
    cfilterexpressioncount: u32,
    pfilter: *const DEVPROP_FILTER_EXPRESSION,
    cextendedparametercount: u32,
    pextendedparameters: *const DEV_QUERY_PARAMETER,
    pcallback: ::std::option::Option<PDEV_QUERY_RESULT_CALLBACK>,
    pcontext: *const ::std::ffi::c_void,
) -> ::windows::runtime::Result<*mut HDEVQUERY__> {
    #[cfg(windows)]
    {
        #[link(name = "api-ms-win-devices-query-l1-1-1")]
        extern "system" {
            fn DevCreateObjectQueryFromIdsEx(
                objecttype: DEV_OBJECT_TYPE,
                pszzobjectids: super::super::Foundation::PWSTR,
                queryflags: u32,
                crequestedproperties: u32,
                prequestedproperties: *const super::super::System::SystemServices::DEVPROPCOMPKEY,
                cfilterexpressioncount: u32,
                pfilter: *const DEVPROP_FILTER_EXPRESSION,
                cextendedparametercount: u32,
                pextendedparameters: *const DEV_QUERY_PARAMETER,
                pcallback: ::windows::runtime::RawPtr,
                pcontext: *const ::std::ffi::c_void,
                phdevquery: *mut *mut HDEVQUERY__,
            ) -> ::windows::runtime::HRESULT;
        }
        let mut result__: <*mut HDEVQUERY__ as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        DevCreateObjectQueryFromIdsEx(
            ::std::mem::transmute(objecttype),
            pszzobjectids.into_param().abi(),
            ::std::mem::transmute(queryflags),
            ::std::mem::transmute(crequestedproperties),
            ::std::mem::transmute(prequestedproperties),
            ::std::mem::transmute(cfilterexpressioncount),
            ::std::mem::transmute(pfilter),
            ::std::mem::transmute(cextendedparametercount),
            ::std::mem::transmute(pextendedparameters),
            ::std::mem::transmute(pcallback),
            ::std::mem::transmute(pcontext),
            &mut result__,
        )
        .from_abi::<*mut HDEVQUERY__>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn DevFindProperty<
    'a,
    Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    pkey: *const super::super::System::SystemServices::DEVPROPKEY,
    store: super::super::System::SystemServices::DEVPROPSTORE,
    pszlocalename: Param2,
    cproperties: u32,
    pproperties: *const super::super::System::SystemServices::DEVPROPERTY,
) -> *mut super::super::System::SystemServices::DEVPROPERTY {
    #[cfg(windows)]
    {
        #[link(name = "api-ms-win-devices-query-l1-1-0")]
        extern "system" {
            fn DevFindProperty(
                pkey: *const super::super::System::SystemServices::DEVPROPKEY,
                store: super::super::System::SystemServices::DEVPROPSTORE,
                pszlocalename: super::super::Foundation::PWSTR,
                cproperties: u32,
                pproperties: *const super::super::System::SystemServices::DEVPROPERTY,
            ) -> *mut super::super::System::SystemServices::DEVPROPERTY;
        }
        ::std::mem::transmute(DevFindProperty(
            ::std::mem::transmute(pkey),
            ::std::mem::transmute(store),
            pszlocalename.into_param().abi(),
            ::std::mem::transmute(cproperties),
            ::std::mem::transmute(pproperties),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn DevFreeObjectProperties(
    cpropertycount: u32,
    pproperties: *const super::super::System::SystemServices::DEVPROPERTY,
) {
    #[cfg(windows)]
    {
        #[link(name = "api-ms-win-devices-query-l1-1-0")]
        extern "system" {
            fn DevFreeObjectProperties(
                cpropertycount: u32,
                pproperties: *const super::super::System::SystemServices::DEVPROPERTY,
            );
        }
        ::std::mem::transmute(DevFreeObjectProperties(
            ::std::mem::transmute(cpropertycount),
            ::std::mem::transmute(pproperties),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn DevFreeObjects(cobjectcount: u32, pobjects: *const DEV_OBJECT) {
    #[cfg(windows)]
    {
        #[link(name = "api-ms-win-devices-query-l1-1-0")]
        extern "system" {
            fn DevFreeObjects(cobjectcount: u32, pobjects: *const DEV_OBJECT);
        }
        ::std::mem::transmute(DevFreeObjects(
            ::std::mem::transmute(cobjectcount),
            ::std::mem::transmute(pobjects),
        ))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn DevGetObjectProperties<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    objecttype: DEV_OBJECT_TYPE,
    pszobjectid: Param1,
    queryflags: u32,
    crequestedproperties: u32,
    prequestedproperties: *const super::super::System::SystemServices::DEVPROPCOMPKEY,
    pcpropertycount: *mut u32,
    ppproperties: *mut *mut super::super::System::SystemServices::DEVPROPERTY,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "api-ms-win-devices-query-l1-1-0")]
        extern "system" {
            fn DevGetObjectProperties(
                objecttype: DEV_OBJECT_TYPE,
                pszobjectid: super::super::Foundation::PWSTR,
                queryflags: u32,
                crequestedproperties: u32,
                prequestedproperties: *const super::super::System::SystemServices::DEVPROPCOMPKEY,
                pcpropertycount: *mut u32,
                ppproperties: *mut *mut super::super::System::SystemServices::DEVPROPERTY,
            ) -> ::windows::runtime::HRESULT;
        }
        DevGetObjectProperties(
            ::std::mem::transmute(objecttype),
            pszobjectid.into_param().abi(),
            ::std::mem::transmute(queryflags),
            ::std::mem::transmute(crequestedproperties),
            ::std::mem::transmute(prequestedproperties),
            ::std::mem::transmute(pcpropertycount),
            ::std::mem::transmute(ppproperties),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn DevGetObjectPropertiesEx<
    'a,
    Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>,
>(
    objecttype: DEV_OBJECT_TYPE,
    pszobjectid: Param1,
    queryflags: u32,
    crequestedproperties: u32,
    prequestedproperties: *const super::super::System::SystemServices::DEVPROPCOMPKEY,
    cextendedparametercount: u32,
    pextendedparameters: *const DEV_QUERY_PARAMETER,
    pcpropertycount: *mut u32,
    ppproperties: *mut *mut super::super::System::SystemServices::DEVPROPERTY,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "api-ms-win-devices-query-l1-1-1")]
        extern "system" {
            fn DevGetObjectPropertiesEx(
                objecttype: DEV_OBJECT_TYPE,
                pszobjectid: super::super::Foundation::PWSTR,
                queryflags: u32,
                crequestedproperties: u32,
                prequestedproperties: *const super::super::System::SystemServices::DEVPROPCOMPKEY,
                cextendedparametercount: u32,
                pextendedparameters: *const DEV_QUERY_PARAMETER,
                pcpropertycount: *mut u32,
                ppproperties: *mut *mut super::super::System::SystemServices::DEVPROPERTY,
            ) -> ::windows::runtime::HRESULT;
        }
        DevGetObjectPropertiesEx(
            ::std::mem::transmute(objecttype),
            pszobjectid.into_param().abi(),
            ::std::mem::transmute(queryflags),
            ::std::mem::transmute(crequestedproperties),
            ::std::mem::transmute(prequestedproperties),
            ::std::mem::transmute(cextendedparametercount),
            ::std::mem::transmute(pextendedparameters),
            ::std::mem::transmute(pcpropertycount),
            ::std::mem::transmute(ppproperties),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn DevGetObjects(
    objecttype: DEV_OBJECT_TYPE,
    queryflags: u32,
    crequestedproperties: u32,
    prequestedproperties: *const super::super::System::SystemServices::DEVPROPCOMPKEY,
    cfilterexpressioncount: u32,
    pfilter: *const DEVPROP_FILTER_EXPRESSION,
    pcobjectcount: *mut u32,
    ppobjects: *mut *mut DEV_OBJECT,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "api-ms-win-devices-query-l1-1-0")]
        extern "system" {
            fn DevGetObjects(
                objecttype: DEV_OBJECT_TYPE,
                queryflags: u32,
                crequestedproperties: u32,
                prequestedproperties: *const super::super::System::SystemServices::DEVPROPCOMPKEY,
                cfilterexpressioncount: u32,
                pfilter: *const DEVPROP_FILTER_EXPRESSION,
                pcobjectcount: *mut u32,
                ppobjects: *mut *mut DEV_OBJECT,
            ) -> ::windows::runtime::HRESULT;
        }
        DevGetObjects(
            ::std::mem::transmute(objecttype),
            ::std::mem::transmute(queryflags),
            ::std::mem::transmute(crequestedproperties),
            ::std::mem::transmute(prequestedproperties),
            ::std::mem::transmute(cfilterexpressioncount),
            ::std::mem::transmute(pfilter),
            ::std::mem::transmute(pcobjectcount),
            ::std::mem::transmute(ppobjects),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub unsafe fn DevGetObjectsEx(
    objecttype: DEV_OBJECT_TYPE,
    queryflags: u32,
    crequestedproperties: u32,
    prequestedproperties: *const super::super::System::SystemServices::DEVPROPCOMPKEY,
    cfilterexpressioncount: u32,
    pfilter: *const DEVPROP_FILTER_EXPRESSION,
    cextendedparametercount: u32,
    pextendedparameters: *const DEV_QUERY_PARAMETER,
    pcobjectcount: *mut u32,
    ppobjects: *mut *mut DEV_OBJECT,
) -> ::windows::runtime::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "api-ms-win-devices-query-l1-1-1")]
        extern "system" {
            fn DevGetObjectsEx(
                objecttype: DEV_OBJECT_TYPE,
                queryflags: u32,
                crequestedproperties: u32,
                prequestedproperties: *const super::super::System::SystemServices::DEVPROPCOMPKEY,
                cfilterexpressioncount: u32,
                pfilter: *const DEVPROP_FILTER_EXPRESSION,
                cextendedparametercount: u32,
                pextendedparameters: *const DEV_QUERY_PARAMETER,
                pcobjectcount: *mut u32,
                ppobjects: *mut *mut DEV_OBJECT,
            ) -> ::windows::runtime::HRESULT;
        }
        DevGetObjectsEx(
            ::std::mem::transmute(objecttype),
            ::std::mem::transmute(queryflags),
            ::std::mem::transmute(crequestedproperties),
            ::std::mem::transmute(prequestedproperties),
            ::std::mem::transmute(cfilterexpressioncount),
            ::std::mem::transmute(pfilter),
            ::std::mem::transmute(cextendedparametercount),
            ::std::mem::transmute(pextendedparameters),
            ::std::mem::transmute(pcobjectcount),
            ::std::mem::transmute(ppobjects),
        )
        .ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
pub struct HDEVQUERY__ {
    pub unused: i32,
}
impl HDEVQUERY__ {}
impl ::std::default::Default for HDEVQUERY__ {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for HDEVQUERY__ {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("HDEVQUERY__")
            .field("unused", &self.unused)
            .finish()
    }
}
impl ::std::cmp::PartialEq for HDEVQUERY__ {
    fn eq(&self, other: &Self) -> bool {
        self.unused == other.unused
    }
}
impl ::std::cmp::Eq for HDEVQUERY__ {}
unsafe impl ::windows::runtime::Abi for HDEVQUERY__ {
    type Abi = Self;
    type DefaultType = Self;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_SystemServices"))]
pub type PDEV_QUERY_RESULT_CALLBACK = unsafe extern "system" fn(
    hdevquery: *const HDEVQUERY__,
    pcontext: *const ::std::ffi::c_void,
    pactiondata: *const DEV_QUERY_RESULT_ACTION_DATA,
);
