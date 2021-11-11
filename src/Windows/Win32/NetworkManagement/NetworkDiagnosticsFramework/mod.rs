#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct ATTRIBUTE_TYPE(pub i32);
pub const AT_INVALID: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(0i32);
pub const AT_BOOLEAN: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(1i32);
pub const AT_INT8: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(2i32);
pub const AT_UINT8: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(3i32);
pub const AT_INT16: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(4i32);
pub const AT_UINT16: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(5i32);
pub const AT_INT32: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(6i32);
pub const AT_UINT32: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(7i32);
pub const AT_INT64: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(8i32);
pub const AT_UINT64: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(9i32);
pub const AT_STRING: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(10i32);
pub const AT_GUID: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(11i32);
pub const AT_LIFE_TIME: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(12i32);
pub const AT_SOCKADDR: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(13i32);
pub const AT_OCTET_STRING: ATTRIBUTE_TYPE = ATTRIBUTE_TYPE(14i32);
impl ::core::convert::From<i32> for ATTRIBUTE_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for ATTRIBUTE_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const DF_IMPERSONATION: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const DF_TRACELESS: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct DIAGNOSIS_STATUS(pub i32);
pub const DS_NOT_IMPLEMENTED: DIAGNOSIS_STATUS = DIAGNOSIS_STATUS(0i32);
pub const DS_CONFIRMED: DIAGNOSIS_STATUS = DIAGNOSIS_STATUS(1i32);
pub const DS_REJECTED: DIAGNOSIS_STATUS = DIAGNOSIS_STATUS(2i32);
pub const DS_INDETERMINATE: DIAGNOSIS_STATUS = DIAGNOSIS_STATUS(3i32);
pub const DS_DEFERRED: DIAGNOSIS_STATUS = DIAGNOSIS_STATUS(4i32);
pub const DS_PASSTHROUGH: DIAGNOSIS_STATUS = DIAGNOSIS_STATUS(5i32);
impl ::core::convert::From<i32> for DIAGNOSIS_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DIAGNOSIS_STATUS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
pub struct DIAG_SOCKADDR {
    pub family: u16,
    pub data: [super::super::Foundation::CHAR; 126],
}
#[cfg(feature = "Win32_Foundation")]
impl DIAG_SOCKADDR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DIAG_SOCKADDR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DIAG_SOCKADDR {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DIAG_SOCKADDR").field("family", &self.family).field("data", &self.data).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DIAG_SOCKADDR {
    fn eq(&self, other: &Self) -> bool {
        self.family == other.family && self.data == other.data
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DIAG_SOCKADDR {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for DIAG_SOCKADDR {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub struct DiagnosticsInfo {
    pub cost: i32,
    pub flags: u32,
}
impl DiagnosticsInfo {}
impl ::core::default::Default for DiagnosticsInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for DiagnosticsInfo {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("DiagnosticsInfo").field("cost", &self.cost).field("flags", &self.flags).finish()
    }
}
impl ::core::cmp::PartialEq for DiagnosticsInfo {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for DiagnosticsInfo {}
unsafe impl ::windows::core::Abi for DiagnosticsInfo {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
pub struct HELPER_ATTRIBUTE {
    pub pwszName: super::super::Foundation::PWSTR,
    pub r#type: ATTRIBUTE_TYPE,
    pub Anonymous: HELPER_ATTRIBUTE_0,
}
#[cfg(feature = "Win32_Foundation")]
impl HELPER_ATTRIBUTE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HELPER_ATTRIBUTE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HELPER_ATTRIBUTE {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HELPER_ATTRIBUTE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HELPER_ATTRIBUTE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union HELPER_ATTRIBUTE_0 {
    pub Boolean: super::super::Foundation::BOOL,
    pub Char: u8,
    pub Byte: u8,
    pub Short: i16,
    pub Word: u16,
    pub Int: i32,
    pub DWord: u32,
    pub Int64: i64,
    pub UInt64: u64,
    pub PWStr: super::super::Foundation::PWSTR,
    pub Guid: ::windows::core::GUID,
    pub LifeTime: LIFE_TIME,
    pub Address: DIAG_SOCKADDR,
    pub OctetString: OCTET_STRING,
}
#[cfg(feature = "Win32_Foundation")]
impl HELPER_ATTRIBUTE_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HELPER_ATTRIBUTE_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HELPER_ATTRIBUTE_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HELPER_ATTRIBUTE_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HELPER_ATTRIBUTE_0 {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
pub struct HYPOTHESIS {
    pub pwszClassName: super::super::Foundation::PWSTR,
    pub pwszDescription: super::super::Foundation::PWSTR,
    pub celt: u32,
    pub rgAttributes: *mut HELPER_ATTRIBUTE,
}
#[cfg(feature = "Win32_Foundation")]
impl HYPOTHESIS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HYPOTHESIS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HYPOTHESIS {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("HYPOTHESIS").field("pwszClassName", &self.pwszClassName).field("pwszDescription", &self.pwszDescription).field("celt", &self.celt).field("rgAttributes", &self.rgAttributes).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HYPOTHESIS {
    fn eq(&self, other: &Self) -> bool {
        self.pwszClassName == other.pwszClassName && self.pwszDescription == other.pwszDescription && self.celt == other.celt && self.rgAttributes == other.rgAttributes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HYPOTHESIS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HYPOTHESIS {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
pub struct HelperAttributeInfo {
    pub pwszName: super::super::Foundation::PWSTR,
    pub r#type: ATTRIBUTE_TYPE,
}
#[cfg(feature = "Win32_Foundation")]
impl HelperAttributeInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HelperAttributeInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HelperAttributeInfo {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("HelperAttributeInfo").field("pwszName", &self.pwszName).field("r#type", &self.r#type).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HelperAttributeInfo {
    fn eq(&self, other: &Self) -> bool {
        self.pwszName == other.pwszName && self.r#type == other.r#type
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HelperAttributeInfo {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HelperAttributeInfo {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
pub struct HypothesisResult {
    pub hypothesis: HYPOTHESIS,
    pub pathStatus: DIAGNOSIS_STATUS,
}
#[cfg(feature = "Win32_Foundation")]
impl HypothesisResult {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HypothesisResult {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HypothesisResult {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("HypothesisResult").field("hypothesis", &self.hypothesis).field("pathStatus", &self.pathStatus).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HypothesisResult {
    fn eq(&self, other: &Self) -> bool {
        self.hypothesis == other.hypothesis && self.pathStatus == other.pathStatus
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HypothesisResult {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for HypothesisResult {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INetDiagExtensibleHelper(pub ::windows::core::IUnknown);
impl INetDiagExtensibleHelper {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
    pub unsafe fn ResolveAttributes(&self, celt: u32, rgkeyattributes: *const HELPER_ATTRIBUTE, pcelt: *mut u32, prgmatchvalues: *mut *mut HELPER_ATTRIBUTE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgkeyattributes), ::core::mem::transmute(pcelt), ::core::mem::transmute(prgmatchvalues)).ok()
    }
}
unsafe impl ::windows::core::Interface for INetDiagExtensibleHelper {
    type Vtable = INetDiagExtensibleHelper_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0b35748_ebf5_11d8_bbe9_505054503030);
}
impl ::core::convert::From<INetDiagExtensibleHelper> for ::windows::core::IUnknown {
    fn from(value: INetDiagExtensibleHelper) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INetDiagExtensibleHelper> for ::windows::core::IUnknown {
    fn from(value: &INetDiagExtensibleHelper) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetDiagExtensibleHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INetDiagExtensibleHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetDiagExtensibleHelper_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32, rgkeyattributes: *const HELPER_ATTRIBUTE, pcelt: *mut u32, prgmatchvalues: *mut *mut HELPER_ATTRIBUTE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INetDiagHelper(pub ::windows::core::IUnknown);
impl INetDiagHelper {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
    pub unsafe fn Initialize(&self, celt: u32, rgattributes: *const HELPER_ATTRIBUTE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(rgattributes)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
    pub unsafe fn GetDiagnosticsInfo(&self) -> ::windows::core::Result<*mut DiagnosticsInfo> {
        let mut result__: <*mut DiagnosticsInfo as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut DiagnosticsInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
    pub unsafe fn GetKeyAttributes(&self, pcelt: *mut u32, pprgattributes: *mut *mut HELPER_ATTRIBUTE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcelt), ::core::mem::transmute(pprgattributes)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
    pub unsafe fn LowHealth<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszinstancedescription: Param0, ppwszdescription: *mut super::super::Foundation::PWSTR, pdeferredtime: *mut i32, pstatus: *mut DIAGNOSIS_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pwszinstancedescription.into_param().abi(), ::core::mem::transmute(ppwszdescription), ::core::mem::transmute(pdeferredtime), ::core::mem::transmute(pstatus)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
    pub unsafe fn HighUtilization<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pwszinstancedescription: Param0, ppwszdescription: *mut super::super::Foundation::PWSTR, pdeferredtime: *mut i32, pstatus: *mut DIAGNOSIS_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pwszinstancedescription.into_param().abi(), ::core::mem::transmute(ppwszdescription), ::core::mem::transmute(pdeferredtime), ::core::mem::transmute(pstatus)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
    pub unsafe fn GetLowerHypotheses(&self, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcelt), ::core::mem::transmute(pprghypotheses)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
    pub unsafe fn GetDownStreamHypotheses(&self, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcelt), ::core::mem::transmute(pprghypotheses)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
    pub unsafe fn GetHigherHypotheses(&self, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcelt), ::core::mem::transmute(pprghypotheses)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
    pub unsafe fn GetUpStreamHypotheses(&self, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).11)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcelt), ::core::mem::transmute(pprghypotheses)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
    pub unsafe fn Repair(&self, pinfo: *const RepairInfo, pdeferredtime: *mut i32, pstatus: *mut REPAIR_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo), ::core::mem::transmute(pdeferredtime), ::core::mem::transmute(pstatus)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
    pub unsafe fn Validate(&self, problem: PROBLEM_TYPE, pdeferredtime: *mut i32, pstatus: *mut REPAIR_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(problem), ::core::mem::transmute(pdeferredtime), ::core::mem::transmute(pstatus)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
    pub unsafe fn GetRepairInfo(&self, problem: PROBLEM_TYPE, pcelt: *mut u32, ppinfo: *mut *mut RepairInfo) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(problem), ::core::mem::transmute(pcelt), ::core::mem::transmute(ppinfo)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
    pub unsafe fn GetLifeTime(&self) -> ::windows::core::Result<LIFE_TIME> {
        let mut result__: <LIFE_TIME as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<LIFE_TIME>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
    pub unsafe fn SetLifeTime<'a, Param0: ::windows::core::IntoParam<'a, LIFE_TIME>>(&self, lifetime: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).16)(::core::mem::transmute_copy(self), lifetime.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
    pub unsafe fn GetCacheTime(&self) -> ::windows::core::Result<super::super::Foundation::FILETIME> {
        let mut result__: <super::super::Foundation::FILETIME as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::FILETIME>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
    pub unsafe fn GetAttributes(&self, pcelt: *mut u32, pprgattributes: *mut *mut HELPER_ATTRIBUTE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcelt), ::core::mem::transmute(pprgattributes)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
    pub unsafe fn Cancel(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).19)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
    pub unsafe fn Cleanup(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).20)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for INetDiagHelper {
    type Vtable = INetDiagHelper_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0b35746_ebf5_11d8_bbe9_505054503030);
}
impl ::core::convert::From<INetDiagHelper> for ::windows::core::IUnknown {
    fn from(value: INetDiagHelper) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INetDiagHelper> for ::windows::core::IUnknown {
    fn from(value: &INetDiagHelper) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetDiagHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INetDiagHelper {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetDiagHelper_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32, rgattributes: *const HELPER_ATTRIBUTE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, ppinfo: *mut *mut DiagnosticsInfo) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcelt: *mut u32, pprgattributes: *mut *mut HELPER_ATTRIBUTE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszinstancedescription: super::super::Foundation::PWSTR, ppwszdescription: *mut super::super::Foundation::PWSTR, pdeferredtime: *mut i32, pstatus: *mut DIAGNOSIS_STATUS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pwszinstancedescription: super::super::Foundation::PWSTR, ppwszdescription: *mut super::super::Foundation::PWSTR, pdeferredtime: *mut i32, pstatus: *mut DIAGNOSIS_STATUS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcelt: *mut u32, pprghypotheses: *mut *mut HYPOTHESIS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pinfo: *const RepairInfo, pdeferredtime: *mut i32, pstatus: *mut REPAIR_STATUS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, problem: PROBLEM_TYPE, pdeferredtime: *mut i32, pstatus: *mut REPAIR_STATUS) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, problem: PROBLEM_TYPE, pcelt: *mut u32, ppinfo: *mut *mut RepairInfo) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, plifetime: *mut LIFE_TIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, lifetime: LIFE_TIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcachetime: *mut super::super::Foundation::FILETIME) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcelt: *mut u32, pprgattributes: *mut *mut HELPER_ATTRIBUTE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INetDiagHelperEx(pub ::windows::core::IUnknown);
impl INetDiagHelperEx {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
    pub unsafe fn ReconfirmLowHealth(&self, celt: u32, presults: *const HypothesisResult, ppwszupdateddescription: *mut super::super::Foundation::PWSTR, pupdatedstatus: *mut DIAGNOSIS_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(celt), ::core::mem::transmute(presults), ::core::mem::transmute(ppwszupdateddescription), ::core::mem::transmute(pupdatedstatus)).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
    pub unsafe fn SetUtilities<'a, Param0: ::windows::core::IntoParam<'a, INetDiagHelperUtilFactory>>(&self, putilities: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).4)(::core::mem::transmute_copy(self), putilities.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
    pub unsafe fn ReproduceFailure(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).5)(::core::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::core::Interface for INetDiagHelperEx {
    type Vtable = INetDiagHelperEx_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x972dab4d_e4e3_4fc6_ae54_5f65ccde4a15);
}
impl ::core::convert::From<INetDiagHelperEx> for ::windows::core::IUnknown {
    fn from(value: INetDiagHelperEx) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INetDiagHelperEx> for ::windows::core::IUnknown {
    fn from(value: &INetDiagHelperEx) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetDiagHelperEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INetDiagHelperEx {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetDiagHelperEx_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, celt: u32, presults: *const HypothesisResult, ppwszupdateddescription: *mut super::super::Foundation::PWSTR, pupdatedstatus: *mut DIAGNOSIS_STATUS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, putilities: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
);
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INetDiagHelperInfo(pub ::windows::core::IUnknown);
impl INetDiagHelperInfo {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
    pub unsafe fn GetAttributeInfo(&self, pcelt: *mut u32, pprgattributeinfos: *mut *mut HelperAttributeInfo) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(pcelt), ::core::mem::transmute(pprgattributeinfos)).ok()
    }
}
unsafe impl ::windows::core::Interface for INetDiagHelperInfo {
    type Vtable = INetDiagHelperInfo_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xc0b35747_ebf5_11d8_bbe9_505054503030);
}
impl ::core::convert::From<INetDiagHelperInfo> for ::windows::core::IUnknown {
    fn from(value: INetDiagHelperInfo) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INetDiagHelperInfo> for ::windows::core::IUnknown {
    fn from(value: &INetDiagHelperInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetDiagHelperInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INetDiagHelperInfo {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetDiagHelperInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::core::RawPtr, pcelt: *mut u32, pprgattributeinfos: *mut *mut HelperAttributeInfo) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct INetDiagHelperUtilFactory(pub ::windows::core::IUnknown);
impl INetDiagHelperUtilFactory {
    #[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
    pub unsafe fn CreateUtilityInstance<T: ::windows::core::Interface>(&self) -> ::windows::core::Result<T> {
        let mut result__ = ::core::option::Option::None;
        (::windows::core::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &<T as ::windows::core::Interface>::IID, &mut result__ as *mut _ as *mut _).and_some(result__)
    }
}
unsafe impl ::windows::core::Interface for INetDiagHelperUtilFactory {
    type Vtable = INetDiagHelperUtilFactory_abi;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x104613fb_bc57_4178_95ba_88809698354a);
}
impl ::core::convert::From<INetDiagHelperUtilFactory> for ::windows::core::IUnknown {
    fn from(value: INetDiagHelperUtilFactory) -> Self {
        value.0
    }
}
impl ::core::convert::From<&INetDiagHelperUtilFactory> for ::windows::core::IUnknown {
    fn from(value: &INetDiagHelperUtilFactory) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for INetDiagHelperUtilFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(self.0)
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a INetDiagHelperUtilFactory {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct INetDiagHelperUtilFactory_abi(
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, iid: &::windows::core::GUID, interface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::core::RawPtr, riid: *const ::windows::core::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
pub struct LIFE_TIME {
    pub startTime: super::super::Foundation::FILETIME,
    pub endTime: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl LIFE_TIME {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for LIFE_TIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for LIFE_TIME {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("LIFE_TIME").field("startTime", &self.startTime).field("endTime", &self.endTime).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for LIFE_TIME {
    fn eq(&self, other: &Self) -> bool {
        self.startTime == other.startTime && self.endTime == other.endTime
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for LIFE_TIME {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for LIFE_TIME {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const NDF_ADD_CAPTURE_TRACE: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const NDF_APPLY_INCLUSION_LIST_FILTER: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const NDF_ERROR_START: u32 = 63744u32;
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const NDF_E_BAD_PARAM: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146895611i32 as _);
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const NDF_E_CANCELLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146895614i32 as _);
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const NDF_E_DISABLED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146895612i32 as _);
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const NDF_E_LENGTH_EXCEEDED: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146895616i32 as _);
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const NDF_E_NOHELPERCLASS: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146895615i32 as _);
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const NDF_E_PROBLEM_PRESENT: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146895608i32 as _);
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const NDF_E_UNKNOWN: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146895609i32 as _);
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const NDF_E_VALIDATION: ::windows::core::HRESULT = ::windows::core::HRESULT(-2146895610i32 as _);
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const NDF_INBOUND_FLAG_EDGETRAVERSAL: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const NDF_INBOUND_FLAG_HEALTHCHECK: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
#[inline]
pub unsafe fn NdfCancelIncident(handle: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdfCancelIncident(handle: *const ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        NdfCancelIncident(::core::mem::transmute(handle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
#[inline]
pub unsafe fn NdfCloseIncident(handle: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdfCloseIncident(handle: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        NdfCloseIncident(::core::mem::transmute(handle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
#[inline]
pub unsafe fn NdfCreateConnectivityIncident(handle: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdfCreateConnectivityIncident(handle: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        NdfCreateConnectivityIncident(::core::mem::transmute(handle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NdfCreateDNSIncident<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(hostname: Param0, querytype: u16, handle: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdfCreateDNSIncident(hostname: super::super::Foundation::PWSTR, querytype: u16, handle: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        NdfCreateDNSIncident(hostname.into_param().abi(), ::core::mem::transmute(querytype), ::core::mem::transmute(handle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`, `Win32_Networking_WinSock`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock"))]
#[inline]
pub unsafe fn NdfCreateGroupingIncident<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param5: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(
    cloudname: Param0,
    groupname: Param1,
    identity: Param2,
    invitation: Param3,
    addresses: *const super::super::Networking::WinSock::SOCKET_ADDRESS_LIST,
    appid: Param5,
    handle: *mut *mut ::core::ffi::c_void,
) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdfCreateGroupingIncident(cloudname: super::super::Foundation::PWSTR, groupname: super::super::Foundation::PWSTR, identity: super::super::Foundation::PWSTR, invitation: super::super::Foundation::PWSTR, addresses: *const super::super::Networking::WinSock::SOCKET_ADDRESS_LIST, appid: super::super::Foundation::PWSTR, handle: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        NdfCreateGroupingIncident(cloudname.into_param().abi(), groupname.into_param().abi(), identity.into_param().abi(), invitation.into_param().abi(), ::core::mem::transmute(addresses), appid.into_param().abi(), ::core::mem::transmute(handle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NdfCreateIncident<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(helperclassname: Param0, celt: u32, attributes: *const HELPER_ATTRIBUTE, handle: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdfCreateIncident(helperclassname: super::super::Foundation::PWSTR, celt: u32, attributes: *const HELPER_ATTRIBUTE, handle: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        NdfCreateIncident(helperclassname.into_param().abi(), ::core::mem::transmute(celt), ::core::mem::transmute(attributes), ::core::mem::transmute(handle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
#[inline]
pub unsafe fn NdfCreateNetConnectionIncident<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::GUID>>(handle: *mut *mut ::core::ffi::c_void, id: Param1) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdfCreateNetConnectionIncident(handle: *mut *mut ::core::ffi::c_void, id: ::windows::core::GUID) -> ::windows::core::HRESULT;
        }
        NdfCreateNetConnectionIncident(::core::mem::transmute(handle), id.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NdfCreatePnrpIncident<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(cloudname: Param0, peername: Param1, diagnosepublish: Param2, appid: Param3, handle: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdfCreatePnrpIncident(cloudname: super::super::Foundation::PWSTR, peername: super::super::Foundation::PWSTR, diagnosepublish: super::super::Foundation::BOOL, appid: super::super::Foundation::PWSTR, handle: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        NdfCreatePnrpIncident(cloudname.into_param().abi(), peername.into_param().abi(), diagnosepublish.into_param().abi(), appid.into_param().abi(), ::core::mem::transmute(handle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NdfCreateSharingIncident<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(uncpath: Param0, handle: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdfCreateSharingIncident(uncpath: super::super::Foundation::PWSTR, handle: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        NdfCreateSharingIncident(uncpath.into_param().abi(), ::core::mem::transmute(handle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NdfCreateWebIncident<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(url: Param0, handle: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdfCreateWebIncident(url: super::super::Foundation::PWSTR, handle: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        NdfCreateWebIncident(url.into_param().abi(), ::core::mem::transmute(handle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NdfCreateWebIncidentEx<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(url: Param0, usewinhttp: Param1, modulename: Param2, handle: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdfCreateWebIncidentEx(url: super::super::Foundation::PWSTR, usewinhttp: super::super::Foundation::BOOL, modulename: super::super::Foundation::PWSTR, handle: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        NdfCreateWebIncidentEx(url.into_param().abi(), usewinhttp.into_param().abi(), modulename.into_param().abi(), ::core::mem::transmute(handle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`, `Win32_Networking_WinSock`, `Win32_Security`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Networking_WinSock", feature = "Win32_Security"))]
#[inline]
pub unsafe fn NdfCreateWinSockIncident<'a, Param0: ::windows::core::IntoParam<'a, super::super::Networking::WinSock::SOCKET>, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::core::IntoParam<'a, super::super::Foundation::PWSTR>>(sock: Param0, host: Param1, port: u16, appid: Param3, userid: *const super::super::Security::SID, handle: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdfCreateWinSockIncident(sock: super::super::Networking::WinSock::SOCKET, host: super::super::Foundation::PWSTR, port: u16, appid: super::super::Foundation::PWSTR, userid: *const super::super::Security::SID, handle: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT;
        }
        NdfCreateWinSockIncident(sock.into_param().abi(), host.into_param().abi(), ::core::mem::transmute(port), appid.into_param().abi(), ::core::mem::transmute(userid), ::core::mem::transmute(handle)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NdfDiagnoseIncident(handle: *const ::core::ffi::c_void, rootcausecount: *mut u32, rootcauses: *mut *mut RootCauseInfo, dwwait: u32, dwflags: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdfDiagnoseIncident(handle: *const ::core::ffi::c_void, rootcausecount: *mut u32, rootcauses: *mut *mut RootCauseInfo, dwwait: u32, dwflags: u32) -> ::windows::core::HRESULT;
        }
        NdfDiagnoseIncident(::core::mem::transmute(handle), ::core::mem::transmute(rootcausecount), ::core::mem::transmute(rootcauses), ::core::mem::transmute(dwwait), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NdfExecuteDiagnosis<'a, Param1: ::windows::core::IntoParam<'a, super::super::Foundation::HWND>>(handle: *const ::core::ffi::c_void, hwnd: Param1) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdfExecuteDiagnosis(handle: *const ::core::ffi::c_void, hwnd: super::super::Foundation::HWND) -> ::windows::core::HRESULT;
        }
        NdfExecuteDiagnosis(::core::mem::transmute(handle), hwnd.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NdfGetTraceFile(handle: *const ::core::ffi::c_void) -> ::windows::core::Result<super::super::Foundation::PWSTR> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdfGetTraceFile(handle: *const ::core::ffi::c_void, tracefilelocation: *mut super::super::Foundation::PWSTR) -> ::windows::core::HRESULT;
        }
        let mut result__: <super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi = ::core::mem::zeroed();
        NdfGetTraceFile(::core::mem::transmute(handle), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn NdfRepairIncident(handle: *const ::core::ffi::c_void, repairex: *const RepairInfoEx, dwwait: u32) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn NdfRepairIncident(handle: *const ::core::ffi::c_void, repairex: *const RepairInfoEx, dwwait: u32) -> ::windows::core::HRESULT;
        }
        NdfRepairIncident(::core::mem::transmute(handle), ::core::mem::transmute(repairex), ::core::mem::transmute(dwwait)).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub struct OCTET_STRING {
    pub dwLength: u32,
    pub lpValue: *mut u8,
}
impl OCTET_STRING {}
impl ::core::default::Default for OCTET_STRING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for OCTET_STRING {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("OCTET_STRING").field("dwLength", &self.dwLength).field("lpValue", &self.lpValue).finish()
    }
}
impl ::core::cmp::PartialEq for OCTET_STRING {
    fn eq(&self, other: &Self) -> bool {
        self.dwLength == other.dwLength && self.lpValue == other.lpValue
    }
}
impl ::core::cmp::Eq for OCTET_STRING {}
unsafe impl ::windows::core::Abi for OCTET_STRING {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct PROBLEM_TYPE(pub i32);
pub const PT_INVALID: PROBLEM_TYPE = PROBLEM_TYPE(0i32);
pub const PT_LOW_HEALTH: PROBLEM_TYPE = PROBLEM_TYPE(1i32);
pub const PT_LOWER_HEALTH: PROBLEM_TYPE = PROBLEM_TYPE(2i32);
pub const PT_DOWN_STREAM_HEALTH: PROBLEM_TYPE = PROBLEM_TYPE(4i32);
pub const PT_HIGH_UTILIZATION: PROBLEM_TYPE = PROBLEM_TYPE(8i32);
pub const PT_HIGHER_UTILIZATION: PROBLEM_TYPE = PROBLEM_TYPE(16i32);
pub const PT_UP_STREAM_UTILIZATION: PROBLEM_TYPE = PROBLEM_TYPE(32i32);
impl ::core::convert::From<i32> for PROBLEM_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for PROBLEM_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const RCF_ISCONFIRMED: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const RCF_ISLEAF: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const RCF_ISTHIRDPARTY: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct REPAIR_RISK(pub i32);
pub const RR_NOROLLBACK: REPAIR_RISK = REPAIR_RISK(0i32);
pub const RR_ROLLBACK: REPAIR_RISK = REPAIR_RISK(1i32);
pub const RR_NORISK: REPAIR_RISK = REPAIR_RISK(2i32);
impl ::core::convert::From<i32> for REPAIR_RISK {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for REPAIR_RISK {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct REPAIR_SCOPE(pub i32);
pub const RS_SYSTEM: REPAIR_SCOPE = REPAIR_SCOPE(0i32);
pub const RS_USER: REPAIR_SCOPE = REPAIR_SCOPE(1i32);
pub const RS_APPLICATION: REPAIR_SCOPE = REPAIR_SCOPE(2i32);
pub const RS_PROCESS: REPAIR_SCOPE = REPAIR_SCOPE(3i32);
impl ::core::convert::From<i32> for REPAIR_SCOPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for REPAIR_SCOPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct REPAIR_STATUS(pub i32);
pub const RS_NOT_IMPLEMENTED: REPAIR_STATUS = REPAIR_STATUS(0i32);
pub const RS_REPAIRED: REPAIR_STATUS = REPAIR_STATUS(1i32);
pub const RS_UNREPAIRED: REPAIR_STATUS = REPAIR_STATUS(2i32);
pub const RS_DEFERRED: REPAIR_STATUS = REPAIR_STATUS(3i32);
pub const RS_USER_ACTION: REPAIR_STATUS = REPAIR_STATUS(4i32);
impl ::core::convert::From<i32> for REPAIR_STATUS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for REPAIR_STATUS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const RF_CONTACT_ADMIN: u32 = 131072u32;
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const RF_INFORMATION_ONLY: u32 = 33554432u32;
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const RF_REPRO: u32 = 2097152u32;
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const RF_RESERVED: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const RF_RESERVED_CA: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const RF_RESERVED_LNI: u32 = 65536u32;
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const RF_SHOW_EVENTS: u32 = 8388608u32;
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const RF_UI_ONLY: u32 = 16777216u32;
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const RF_USER_ACTION: u32 = 268435456u32;
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const RF_USER_CONFIRMATION: u32 = 134217728u32;
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const RF_VALIDATE_HELPTOPIC: u32 = 4194304u32;
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
pub const RF_WORKAROUND: u32 = 536870912u32;
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
pub struct RepairInfo {
    pub guid: ::windows::core::GUID,
    pub pwszClassName: super::super::Foundation::PWSTR,
    pub pwszDescription: super::super::Foundation::PWSTR,
    pub sidType: u32,
    pub cost: i32,
    pub flags: u32,
    pub scope: REPAIR_SCOPE,
    pub risk: REPAIR_RISK,
    pub UiInfo: UiInfo,
    pub rootCauseIndex: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl RepairInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RepairInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RepairInfo {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RepairInfo {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RepairInfo {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
pub struct RepairInfoEx {
    pub repair: RepairInfo,
    pub repairRank: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl RepairInfoEx {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RepairInfoEx {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RepairInfoEx {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RepairInfoEx {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RepairInfoEx {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
pub struct RootCauseInfo {
    pub pwszDescription: super::super::Foundation::PWSTR,
    pub rootCauseID: ::windows::core::GUID,
    pub rootCauseFlags: u32,
    pub networkInterfaceID: ::windows::core::GUID,
    pub pRepairs: *mut RepairInfoEx,
    pub repairCount: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl RootCauseInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for RootCauseInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for RootCauseInfo {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("RootCauseInfo")
            .field("pwszDescription", &self.pwszDescription)
            .field("rootCauseID", &self.rootCauseID)
            .field("rootCauseFlags", &self.rootCauseFlags)
            .field("networkInterfaceID", &self.networkInterfaceID)
            .field("pRepairs", &self.pRepairs)
            .field("repairCount", &self.repairCount)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for RootCauseInfo {
    fn eq(&self, other: &Self) -> bool {
        self.pwszDescription == other.pwszDescription && self.rootCauseID == other.rootCauseID && self.rootCauseFlags == other.rootCauseFlags && self.networkInterfaceID == other.networkInterfaceID && self.pRepairs == other.pRepairs && self.repairCount == other.repairCount
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for RootCauseInfo {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for RootCauseInfo {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
pub struct ShellCommandInfo {
    pub pwszOperation: super::super::Foundation::PWSTR,
    pub pwszFile: super::super::Foundation::PWSTR,
    pub pwszParameters: super::super::Foundation::PWSTR,
    pub pwszDirectory: super::super::Foundation::PWSTR,
    pub nShowCmd: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ShellCommandInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ShellCommandInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ShellCommandInfo {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("ShellCommandInfo").field("pwszOperation", &self.pwszOperation).field("pwszFile", &self.pwszFile).field("pwszParameters", &self.pwszParameters).field("pwszDirectory", &self.pwszDirectory).field("nShowCmd", &self.nShowCmd).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ShellCommandInfo {
    fn eq(&self, other: &Self) -> bool {
        self.pwszOperation == other.pwszOperation && self.pwszFile == other.pwszFile && self.pwszParameters == other.pwszParameters && self.pwszDirectory == other.pwszDirectory && self.nShowCmd == other.nShowCmd
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ShellCommandInfo {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ShellCommandInfo {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct UI_INFO_TYPE(pub i32);
pub const UIT_INVALID: UI_INFO_TYPE = UI_INFO_TYPE(0i32);
pub const UIT_NONE: UI_INFO_TYPE = UI_INFO_TYPE(1i32);
pub const UIT_SHELL_COMMAND: UI_INFO_TYPE = UI_INFO_TYPE(2i32);
pub const UIT_HELP_PANE: UI_INFO_TYPE = UI_INFO_TYPE(3i32);
pub const UIT_DUI: UI_INFO_TYPE = UI_INFO_TYPE(4i32);
impl ::core::convert::From<i32> for UI_INFO_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for UI_INFO_TYPE {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_NetworkManagement_NetworkDiagnosticsFramework`, `Win32_Foundation`*"]
pub struct UiInfo {
    pub r#type: UI_INFO_TYPE,
    pub Anonymous: UiInfo_0,
}
#[cfg(feature = "Win32_Foundation")]
impl UiInfo {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for UiInfo {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for UiInfo {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for UiInfo {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for UiInfo {
    type Abi = Self;
}
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub union UiInfo_0 {
    pub pwzNull: super::super::Foundation::PWSTR,
    pub ShellInfo: ShellCommandInfo,
    pub pwzHelpUrl: super::super::Foundation::PWSTR,
    pub pwzDui: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl UiInfo_0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for UiInfo_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for UiInfo_0 {
    fn eq(&self, _other: &Self) -> bool {
        unimplemented!()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for UiInfo_0 {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for UiInfo_0 {
    type Abi = Self;
}
