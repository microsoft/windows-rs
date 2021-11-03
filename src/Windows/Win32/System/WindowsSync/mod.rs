#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CONFLICT_RESOLUTION_POLICY(pub i32);
pub const CRP_NONE: CONFLICT_RESOLUTION_POLICY = CONFLICT_RESOLUTION_POLICY(0i32);
pub const CRP_DESTINATION_PROVIDER_WINS: CONFLICT_RESOLUTION_POLICY = CONFLICT_RESOLUTION_POLICY(1i32);
pub const CRP_SOURCE_PROVIDER_WINS: CONFLICT_RESOLUTION_POLICY = CONFLICT_RESOLUTION_POLICY(2i32);
pub const CRP_LAST: CONFLICT_RESOLUTION_POLICY = CONFLICT_RESOLUTION_POLICY(3i32);
impl ::std::convert::From<i32> for CONFLICT_RESOLUTION_POLICY {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CONFLICT_RESOLUTION_POLICY {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct CONSTRAINT_CONFLICT_REASON(pub i32);
pub const CCR_OTHER: CONSTRAINT_CONFLICT_REASON = CONSTRAINT_CONFLICT_REASON(0i32);
pub const CCR_COLLISION: CONSTRAINT_CONFLICT_REASON = CONSTRAINT_CONFLICT_REASON(1i32);
pub const CCR_NOPARENT: CONSTRAINT_CONFLICT_REASON = CONSTRAINT_CONFLICT_REASON(2i32);
pub const CCR_IDENTITY: CONSTRAINT_CONFLICT_REASON = CONSTRAINT_CONFLICT_REASON(3i32);
impl ::std::convert::From<i32> for CONSTRAINT_CONFLICT_REASON {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CONSTRAINT_CONFLICT_REASON {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FILTERING_TYPE(pub i32);
pub const FT_CURRENT_ITEMS_ONLY: FILTERING_TYPE = FILTERING_TYPE(0i32);
pub const FT_CURRENT_ITEMS_AND_VERSIONS_FOR_MOVED_OUT_ITEMS: FILTERING_TYPE = FILTERING_TYPE(1i32);
impl ::std::convert::From<i32> for FILTERING_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FILTERING_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct FILTER_COMBINATION_TYPE(pub i32);
pub const FCT_INTERSECTION: FILTER_COMBINATION_TYPE = FILTER_COMBINATION_TYPE(0i32);
impl ::std::convert::From<i32> for FILTER_COMBINATION_TYPE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for FILTER_COMBINATION_TYPE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IAsynchronousDataRetriever(pub ::windows::runtime::IUnknown);
impl IAsynchronousDataRetriever {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_Foundation`*"]
    pub unsafe fn GetIdParameters(&self, pidparameters: *mut ID_PARAMETERS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pidparameters)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn RegisterCallback<'a, Param0: ::windows::runtime::IntoParam<'a, IDataRetrieverCallback>>(&self, pdataretrievercallback: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pdataretrievercallback.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn RevokeCallback<'a, Param0: ::windows::runtime::IntoParam<'a, IDataRetrieverCallback>>(&self, pdataretrievercallback: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pdataretrievercallback.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn LoadChangeData<'a, Param0: ::windows::runtime::IntoParam<'a, ILoadChangeContext>>(&self, ploadchangecontext: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ploadchangecontext.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IAsynchronousDataRetriever {
    type Vtable = IAsynchronousDataRetriever_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2680677488, 25066, 19080, [155, 228, 223, 86, 162, 124, 254, 242]);
}
impl ::std::convert::From<IAsynchronousDataRetriever> for ::windows::runtime::IUnknown {
    fn from(value: IAsynchronousDataRetriever) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IAsynchronousDataRetriever> for ::windows::runtime::IUnknown {
    fn from(value: &IAsynchronousDataRetriever) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IAsynchronousDataRetriever {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IAsynchronousDataRetriever {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsynchronousDataRetriever_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pidparameters: *mut ID_PARAMETERS) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdataretrievercallback: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdataretrievercallback: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ploadchangecontext: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IChangeConflict(pub ::windows::runtime::IUnknown);
impl IChangeConflict {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetDestinationProviderConflictingChange(&self) -> ::windows::runtime::Result<ISyncChange> {
        let mut result__: <ISyncChange as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ISyncChange>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetSourceProviderConflictingChange(&self) -> ::windows::runtime::Result<ISyncChange> {
        let mut result__: <ISyncChange as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ISyncChange>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetDestinationProviderConflictingData(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetSourceProviderConflictingData(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetResolveActionForChange(&self, presolveaction: *mut SYNC_RESOLVE_ACTION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(presolveaction)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn SetResolveActionForChange(&self, resolveaction: SYNC_RESOLVE_ACTION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(resolveaction)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetResolveActionForChangeUnit<'a, Param0: ::windows::runtime::IntoParam<'a, ISyncChangeUnit>>(&self, pchangeunit: Param0, presolveaction: *mut SYNC_RESOLVE_ACTION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), pchangeunit.into_param().abi(), ::std::mem::transmute(presolveaction)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn SetResolveActionForChangeUnit<'a, Param0: ::windows::runtime::IntoParam<'a, ISyncChangeUnit>>(&self, pchangeunit: Param0, resolveaction: SYNC_RESOLVE_ACTION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), pchangeunit.into_param().abi(), ::std::mem::transmute(resolveaction)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IChangeConflict {
    type Vtable = IChangeConflict_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(21938071, 40736, 20346, [189, 212, 37, 151, 156, 119, 192, 2]);
}
impl ::std::convert::From<IChangeConflict> for ::windows::runtime::IUnknown {
    fn from(value: IChangeConflict) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IChangeConflict> for ::windows::runtime::IUnknown {
    fn from(value: &IChangeConflict) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IChangeConflict {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IChangeConflict {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IChangeConflict_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppconflictingchange: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppconflictingchange: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppconflictingdata: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppconflictingdata: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, presolveaction: *mut SYNC_RESOLVE_ACTION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resolveaction: SYNC_RESOLVE_ACTION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pchangeunit: ::windows::runtime::RawPtr, presolveaction: *mut SYNC_RESOLVE_ACTION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pchangeunit: ::windows::runtime::RawPtr, resolveaction: SYNC_RESOLVE_ACTION) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IChangeUnitException(pub ::windows::runtime::IUnknown);
impl IChangeUnitException {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetItemId(&self, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbitemid), ::std::mem::transmute(pcbidsize)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetChangeUnitId(&self, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbchangeunitid), ::std::mem::transmute(pcbidsize)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetClockVector(&self, riid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(ppunk)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IChangeUnitException {
    type Vtable = IChangeUnitException_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(215477884, 65216, 16417, [153, 238, 240, 229, 52, 143, 42, 95]);
}
impl ::std::convert::From<IChangeUnitException> for ::windows::runtime::IUnknown {
    fn from(value: IChangeUnitException) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IChangeUnitException> for ::windows::runtime::IUnknown {
    fn from(value: &IChangeUnitException) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IChangeUnitException {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IChangeUnitException {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IChangeUnitException_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IChangeUnitListFilterInfo(pub ::windows::runtime::IUnknown);
impl IChangeUnitListFilterInfo {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Serialize(&self, pbbuffer: *mut u8, pcbbuffer: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbbuffer), ::std::mem::transmute(pcbbuffer)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Initialize(&self, ppbchangeunitids: *const *const u8, dwchangeunitcount: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppbchangeunitids), ::std::mem::transmute(dwchangeunitcount)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetChangeUnitIdCount(&self, pdwchangeunitidcount: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwchangeunitidcount)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetChangeUnitId(&self, dwchangeunitidindex: u32, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwchangeunitidindex), ::std::mem::transmute(pbchangeunitid), ::std::mem::transmute(pcbidsize)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IChangeUnitListFilterInfo {
    type Vtable = IChangeUnitListFilterInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4068701809, 3039, 17402, [181, 2, 35, 35, 117, 251, 80, 194]);
}
impl ::std::convert::From<IChangeUnitListFilterInfo> for ::windows::runtime::IUnknown {
    fn from(value: IChangeUnitListFilterInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IChangeUnitListFilterInfo> for ::windows::runtime::IUnknown {
    fn from(value: &IChangeUnitListFilterInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IChangeUnitListFilterInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IChangeUnitListFilterInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IChangeUnitListFilterInfo> for ISyncFilterInfo {
    fn from(value: IChangeUnitListFilterInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IChangeUnitListFilterInfo> for ISyncFilterInfo {
    fn from(value: &IChangeUnitListFilterInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyncFilterInfo> for IChangeUnitListFilterInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyncFilterInfo> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISyncFilterInfo>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyncFilterInfo> for &IChangeUnitListFilterInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyncFilterInfo> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISyncFilterInfo>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IChangeUnitListFilterInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbbuffer: *mut u8, pcbbuffer: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppbchangeunitids: *const *const u8, dwchangeunitcount: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwchangeunitidcount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwchangeunitidindex: u32, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IClockVector(pub ::windows::runtime::IUnknown);
impl IClockVector {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetClockVectorElements(&self, riid: *const ::windows::runtime::GUID, ppienumclockvector: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(ppienumclockvector)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetClockVectorElementCount(&self, pdwcount: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwcount)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IClockVector {
    type Vtable = IClockVector_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(347219786, 34456, 19654, [147, 51, 248, 155, 209, 212, 123, 196]);
}
impl ::std::convert::From<IClockVector> for ::windows::runtime::IUnknown {
    fn from(value: IClockVector) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IClockVector> for ::windows::runtime::IUnknown {
    fn from(value: &IClockVector) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IClockVector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IClockVector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IClockVector_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppienumclockvector: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwcount: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IClockVectorElement(pub ::windows::runtime::IUnknown);
impl IClockVectorElement {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetReplicaKey(&self, pdwreplicakey: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwreplicakey)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetTickCount(&self, pulltickcount: *mut u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pulltickcount)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IClockVectorElement {
    type Vtable = IClockVectorElement_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3877388880, 44536, 18951, [143, 174, 86, 105, 89, 105, 9, 193]);
}
impl ::std::convert::From<IClockVectorElement> for ::windows::runtime::IUnknown {
    fn from(value: IClockVectorElement) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IClockVectorElement> for ::windows::runtime::IUnknown {
    fn from(value: &IClockVectorElement) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IClockVectorElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IClockVectorElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IClockVectorElement_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwreplicakey: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pulltickcount: *mut u64) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ICombinedFilterInfo(pub ::windows::runtime::IUnknown);
impl ICombinedFilterInfo {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Serialize(&self, pbbuffer: *mut u8, pcbbuffer: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbbuffer), ::std::mem::transmute(pcbbuffer)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetFilterCount(&self, pdwfiltercount: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwfiltercount)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetFilterInfo(&self, dwfilterindex: u32) -> ::windows::runtime::Result<ISyncFilterInfo> {
        let mut result__: <ISyncFilterInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwfilterindex), &mut result__).from_abi::<ISyncFilterInfo>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetFilterCombinationType(&self, pfiltercombinationtype: *mut FILTER_COMBINATION_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pfiltercombinationtype)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICombinedFilterInfo {
    type Vtable = ICombinedFilterInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(301588081, 10264, 18297, [178, 172, 66, 212, 80, 86, 95, 69]);
}
impl ::std::convert::From<ICombinedFilterInfo> for ::windows::runtime::IUnknown {
    fn from(value: ICombinedFilterInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ICombinedFilterInfo> for ::windows::runtime::IUnknown {
    fn from(value: &ICombinedFilterInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICombinedFilterInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ICombinedFilterInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<ICombinedFilterInfo> for ISyncFilterInfo {
    fn from(value: ICombinedFilterInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ICombinedFilterInfo> for ISyncFilterInfo {
    fn from(value: &ICombinedFilterInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyncFilterInfo> for ICombinedFilterInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyncFilterInfo> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISyncFilterInfo>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyncFilterInfo> for &ICombinedFilterInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyncFilterInfo> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISyncFilterInfo>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICombinedFilterInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbbuffer: *mut u8, pcbbuffer: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwfiltercount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwfilterindex: u32, ppifilterinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfiltercombinationtype: *mut FILTER_COMBINATION_TYPE) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IConstraintConflict(pub ::windows::runtime::IUnknown);
impl IConstraintConflict {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetDestinationProviderConflictingChange(&self) -> ::windows::runtime::Result<ISyncChange> {
        let mut result__: <ISyncChange as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ISyncChange>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetSourceProviderConflictingChange(&self) -> ::windows::runtime::Result<ISyncChange> {
        let mut result__: <ISyncChange as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ISyncChange>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetDestinationProviderOriginalChange(&self) -> ::windows::runtime::Result<ISyncChange> {
        let mut result__: <ISyncChange as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ISyncChange>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetDestinationProviderConflictingData(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetSourceProviderConflictingData(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetDestinationProviderOriginalData(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetConstraintResolveActionForChange(&self, pconstraintresolveaction: *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(pconstraintresolveaction)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn SetConstraintResolveActionForChange(&self, constraintresolveaction: SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(constraintresolveaction)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetConstraintResolveActionForChangeUnit<'a, Param0: ::windows::runtime::IntoParam<'a, ISyncChangeUnit>>(&self, pchangeunit: Param0, pconstraintresolveaction: *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), pchangeunit.into_param().abi(), ::std::mem::transmute(pconstraintresolveaction)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn SetConstraintResolveActionForChangeUnit<'a, Param0: ::windows::runtime::IntoParam<'a, ISyncChangeUnit>>(&self, pchangeunit: Param0, constraintresolveaction: SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), pchangeunit.into_param().abi(), ::std::mem::transmute(constraintresolveaction)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetConstraintConflictReason(&self, pconstraintconflictreason: *mut CONSTRAINT_CONFLICT_REASON) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(pconstraintconflictreason)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn IsTemporary(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IConstraintConflict {
    type Vtable = IConstraintConflict_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(13774894, 7416, 18485, [184, 95, 183, 202, 79, 121, 158, 10]);
}
impl ::std::convert::From<IConstraintConflict> for ::windows::runtime::IUnknown {
    fn from(value: IConstraintConflict) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IConstraintConflict> for ::windows::runtime::IUnknown {
    fn from(value: &IConstraintConflict) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IConstraintConflict {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IConstraintConflict {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IConstraintConflict_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppconflictingchange: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppconflictingchange: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pporiginalchange: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppconflictingdata: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppconflictingdata: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pporiginaldata: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pconstraintresolveaction: *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, constraintresolveaction: SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pchangeunit: ::windows::runtime::RawPtr, pconstraintresolveaction: *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pchangeunit: ::windows::runtime::RawPtr, constraintresolveaction: SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pconstraintconflictreason: *mut CONSTRAINT_CONFLICT_REASON) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IConstructReplicaKeyMap(pub ::windows::runtime::IUnknown);
impl IConstructReplicaKeyMap {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn FindOrAddReplica(&self, pbreplicaid: *const u8, pdwreplicakey: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbreplicaid), ::std::mem::transmute(pdwreplicakey)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IConstructReplicaKeyMap {
    type Vtable = IConstructReplicaKeyMap_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3738241392, 60549, 16661, [181, 44, 68, 5, 132, 86, 66, 165]);
}
impl ::std::convert::From<IConstructReplicaKeyMap> for ::windows::runtime::IUnknown {
    fn from(value: IConstructReplicaKeyMap) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IConstructReplicaKeyMap> for ::windows::runtime::IUnknown {
    fn from(value: &IConstructReplicaKeyMap) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IConstructReplicaKeyMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IConstructReplicaKeyMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IConstructReplicaKeyMap_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbreplicaid: *const u8, pdwreplicakey: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ICoreFragment(pub ::windows::runtime::IUnknown);
impl ICoreFragment {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn NextColumn(&self, pchangeunitid: *mut u8, pchangeunitidsize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pchangeunitid), ::std::mem::transmute(pchangeunitidsize)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn NextRange(&self, pitemid: *mut u8, pitemidsize: *mut u32, piclockvector: *mut ::std::option::Option<IClockVector>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pitemid), ::std::mem::transmute(pitemidsize), ::std::mem::transmute(piclockvector)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetColumnCount(&self, pcolumncount: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pcolumncount)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetRangeCount(&self, prangecount: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(prangecount)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICoreFragment {
    type Vtable = ICoreFragment_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1631267509, 45828, 18393, [156, 49, 206, 108, 84, 64, 26, 21]);
}
impl ::std::convert::From<ICoreFragment> for ::windows::runtime::IUnknown {
    fn from(value: ICoreFragment) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ICoreFragment> for ::windows::runtime::IUnknown {
    fn from(value: &ICoreFragment) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICoreFragment {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ICoreFragment {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreFragment_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pchangeunitid: *mut u8, pchangeunitidsize: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pitemid: *mut u8, pitemidsize: *mut u32, piclockvector: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcolumncount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prangecount: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ICoreFragmentInspector(pub ::windows::runtime::IUnknown);
impl ICoreFragmentInspector {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn NextCoreFragments(&self, requestedcount: u32, ppicorefragments: *mut ::std::option::Option<ICoreFragment>, pfetchedcount: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(requestedcount), ::std::mem::transmute(ppicorefragments), ::std::mem::transmute(pfetchedcount)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ICoreFragmentInspector {
    type Vtable = ICoreFragmentInspector_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4160538109, 44582, 18041, [186, 22, 150, 170, 197, 131, 193, 52]);
}
impl ::std::convert::From<ICoreFragmentInspector> for ::windows::runtime::IUnknown {
    fn from(value: ICoreFragmentInspector) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ICoreFragmentInspector> for ::windows::runtime::IUnknown {
    fn from(value: &ICoreFragmentInspector) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICoreFragmentInspector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ICoreFragmentInspector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreFragmentInspector_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, requestedcount: u32, ppicorefragments: *mut ::windows::runtime::RawPtr, pfetchedcount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ICustomFilterInfo(pub ::windows::runtime::IUnknown);
impl ICustomFilterInfo {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Serialize(&self, pbbuffer: *mut u8, pcbbuffer: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbbuffer), ::std::mem::transmute(pcbbuffer)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetSyncFilter(&self) -> ::windows::runtime::Result<ISyncFilter> {
        let mut result__: <ISyncFilter as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ISyncFilter>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ICustomFilterInfo {
    type Vtable = ICustomFilterInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(489905663, 28552, 20045, [145, 168, 163, 243, 81, 207, 212, 115]);
}
impl ::std::convert::From<ICustomFilterInfo> for ::windows::runtime::IUnknown {
    fn from(value: ICustomFilterInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ICustomFilterInfo> for ::windows::runtime::IUnknown {
    fn from(value: &ICustomFilterInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ICustomFilterInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ICustomFilterInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<ICustomFilterInfo> for ISyncFilterInfo {
    fn from(value: ICustomFilterInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ICustomFilterInfo> for ISyncFilterInfo {
    fn from(value: &ICustomFilterInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyncFilterInfo> for ICustomFilterInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyncFilterInfo> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISyncFilterInfo>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyncFilterInfo> for &ICustomFilterInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyncFilterInfo> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISyncFilterInfo>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomFilterInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbbuffer: *mut u8, pcbbuffer: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pisyncfilter: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_Foundation`*"]
pub struct ID_PARAMETERS {
    pub dwSize: u32,
    pub replicaId: ID_PARAMETER_PAIR,
    pub itemId: ID_PARAMETER_PAIR,
    pub changeUnitId: ID_PARAMETER_PAIR,
}
#[cfg(feature = "Win32_Foundation")]
impl ID_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for ID_PARAMETERS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for ID_PARAMETERS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ID_PARAMETERS").field("dwSize", &self.dwSize).field("replicaId", &self.replicaId).field("itemId", &self.itemId).field("changeUnitId", &self.changeUnitId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for ID_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.replicaId == other.replicaId && self.itemId == other.itemId && self.changeUnitId == other.changeUnitId
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for ID_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ID_PARAMETERS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_Foundation`*"]
pub struct ID_PARAMETER_PAIR {
    pub fIsVariable: super::super::Foundation::BOOL,
    pub cbIdSize: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ID_PARAMETER_PAIR {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for ID_PARAMETER_PAIR {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for ID_PARAMETER_PAIR {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("ID_PARAMETER_PAIR").field("fIsVariable", &self.fIsVariable).field("cbIdSize", &self.cbIdSize).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for ID_PARAMETER_PAIR {
    fn eq(&self, other: &Self) -> bool {
        self.fIsVariable == other.fIsVariable && self.cbIdSize == other.cbIdSize
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for ID_PARAMETER_PAIR {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for ID_PARAMETER_PAIR {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IDataRetrieverCallback(pub ::windows::runtime::IUnknown);
impl IDataRetrieverCallback {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn LoadChangeDataComplete<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, punkdata: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), punkdata.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn LoadChangeDataError(&self, hrerror: ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(hrerror)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IDataRetrieverCallback {
    type Vtable = IDataRetrieverCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1907656251, 63849, 18038, [187, 195, 61, 159, 220, 63, 178, 199]);
}
impl ::std::convert::From<IDataRetrieverCallback> for ::windows::runtime::IUnknown {
    fn from(value: IDataRetrieverCallback) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IDataRetrieverCallback> for ::windows::runtime::IUnknown {
    fn from(value: &IDataRetrieverCallback) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IDataRetrieverCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IDataRetrieverCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataRetrieverCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punkdata: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hrerror: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IEnumChangeUnitExceptions(pub ::windows::runtime::IUnknown);
impl IEnumChangeUnitExceptions {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Next(&self, cexceptions: u32, ppchangeunitexception: *mut ::std::option::Option<IChangeUnitException>, pcfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(cexceptions), ::std::mem::transmute(ppchangeunitexception), ::std::mem::transmute(pcfetched)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Skip(&self, cexceptions: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(cexceptions)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumChangeUnitExceptions> {
        let mut result__: <IEnumChangeUnitExceptions as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumChangeUnitExceptions>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumChangeUnitExceptions {
    type Vtable = IEnumChangeUnitExceptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(812967938, 37657, 17440, [190, 33, 16, 34, 226, 226, 29, 168]);
}
impl ::std::convert::From<IEnumChangeUnitExceptions> for ::windows::runtime::IUnknown {
    fn from(value: IEnumChangeUnitExceptions) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IEnumChangeUnitExceptions> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumChangeUnitExceptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumChangeUnitExceptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumChangeUnitExceptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumChangeUnitExceptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cexceptions: u32, ppchangeunitexception: *mut ::windows::runtime::RawPtr, pcfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cexceptions: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IEnumClockVector(pub ::windows::runtime::IUnknown);
impl IEnumClockVector {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Next(&self, cclockvectorelements: u32, ppiclockvectorelements: *mut ::std::option::Option<IClockVectorElement>, pcfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(cclockvectorelements), ::std::mem::transmute(ppiclockvectorelements), ::std::mem::transmute(pcfetched)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Skip(&self, csyncversions: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(csyncversions)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumClockVector> {
        let mut result__: <IEnumClockVector as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumClockVector>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumClockVector {
    type Vtable = IEnumClockVector_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1381516507, 10295, 18329, [158, 128, 129, 166, 110, 2, 34, 12]);
}
impl ::std::convert::From<IEnumClockVector> for ::windows::runtime::IUnknown {
    fn from(value: IEnumClockVector) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IEnumClockVector> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumClockVector) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumClockVector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumClockVector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumClockVector_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cclockvectorelements: u32, ppiclockvectorelements: *mut ::windows::runtime::RawPtr, pcfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, csyncversions: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppienum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IEnumFeedClockVector(pub ::windows::runtime::IUnknown);
impl IEnumFeedClockVector {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Next(&self, cclockvectorelements: u32, ppiclockvectorelements: *mut ::std::option::Option<IFeedClockVectorElement>, pcfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(cclockvectorelements), ::std::mem::transmute(ppiclockvectorelements), ::std::mem::transmute(pcfetched)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Skip(&self, csyncversions: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(csyncversions)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumFeedClockVector> {
        let mut result__: <IEnumFeedClockVector as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumFeedClockVector>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumFeedClockVector {
    type Vtable = IEnumFeedClockVector_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1427076669, 5226, 18678, [171, 235, 108, 136, 199, 247, 5, 20]);
}
impl ::std::convert::From<IEnumFeedClockVector> for ::windows::runtime::IUnknown {
    fn from(value: IEnumFeedClockVector) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IEnumFeedClockVector> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumFeedClockVector) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumFeedClockVector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumFeedClockVector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumFeedClockVector_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cclockvectorelements: u32, ppiclockvectorelements: *mut ::windows::runtime::RawPtr, pcfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, csyncversions: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppienum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IEnumItemIds(pub ::windows::runtime::IUnknown);
impl IEnumItemIds {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Next(&self, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbitemid), ::std::mem::transmute(pcbitemidsize)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IEnumItemIds {
    type Vtable = IEnumItemIds_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1135230817, 19246, 19296, [131, 223, 177, 16, 211, 225, 72, 241]);
}
impl ::std::convert::From<IEnumItemIds> for ::windows::runtime::IUnknown {
    fn from(value: IEnumItemIds) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IEnumItemIds> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumItemIds) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumItemIds {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumItemIds {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumItemIds_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IEnumRangeExceptions(pub ::windows::runtime::IUnknown);
impl IEnumRangeExceptions {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Next(&self, cexceptions: u32, pprangeexception: *mut ::std::option::Option<IRangeException>, pcfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(cexceptions), ::std::mem::transmute(pprangeexception), ::std::mem::transmute(pcfetched)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Skip(&self, cexceptions: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(cexceptions)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumRangeExceptions> {
        let mut result__: <IEnumRangeExceptions as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumRangeExceptions>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumRangeExceptions {
    type Vtable = IEnumRangeExceptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(155468703, 56753, 16758, [183, 3, 4, 111, 242, 42, 35, 134]);
}
impl ::std::convert::From<IEnumRangeExceptions> for ::windows::runtime::IUnknown {
    fn from(value: IEnumRangeExceptions) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IEnumRangeExceptions> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumRangeExceptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumRangeExceptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumRangeExceptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumRangeExceptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cexceptions: u32, pprangeexception: *mut ::windows::runtime::RawPtr, pcfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cexceptions: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IEnumSingleItemExceptions(pub ::windows::runtime::IUnknown);
impl IEnumSingleItemExceptions {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Next(&self, cexceptions: u32, ppsingleitemexception: *mut ::std::option::Option<ISingleItemException>, pcfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(cexceptions), ::std::mem::transmute(ppsingleitemexception), ::std::mem::transmute(pcfetched)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Skip(&self, cexceptions: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(cexceptions)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumSingleItemExceptions> {
        let mut result__: <IEnumSingleItemExceptions as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumSingleItemExceptions>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumSingleItemExceptions {
    type Vtable = IEnumSingleItemExceptions_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3848484892, 6989, 19558, [151, 150, 200, 111, 172, 205, 205, 64]);
}
impl ::std::convert::From<IEnumSingleItemExceptions> for ::windows::runtime::IUnknown {
    fn from(value: IEnumSingleItemExceptions) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IEnumSingleItemExceptions> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumSingleItemExceptions) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumSingleItemExceptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumSingleItemExceptions {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSingleItemExceptions_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cexceptions: u32, ppsingleitemexception: *mut ::windows::runtime::RawPtr, pcfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cexceptions: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IEnumSyncChangeUnits(pub ::windows::runtime::IUnknown);
impl IEnumSyncChangeUnits {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Next(&self, cchanges: u32, ppchangeunit: *mut ::std::option::Option<ISyncChangeUnit>, pcfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(cchanges), ::std::mem::transmute(ppchangeunit), ::std::mem::transmute(pcfetched)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Skip(&self, cchanges: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(cchanges)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumSyncChangeUnits> {
        let mut result__: <IEnumSyncChangeUnits as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumSyncChangeUnits>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumSyncChangeUnits {
    type Vtable = IEnumSyncChangeUnits_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(879441393, 34563, 19565, [171, 26, 77, 188, 162, 207, 249, 127]);
}
impl ::std::convert::From<IEnumSyncChangeUnits> for ::windows::runtime::IUnknown {
    fn from(value: IEnumSyncChangeUnits) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IEnumSyncChangeUnits> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumSyncChangeUnits) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumSyncChangeUnits {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumSyncChangeUnits {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSyncChangeUnits_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cchanges: u32, ppchangeunit: *mut ::windows::runtime::RawPtr, pcfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cchanges: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IEnumSyncChanges(pub ::windows::runtime::IUnknown);
impl IEnumSyncChanges {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Next(&self, cchanges: u32, ppchange: *mut ::std::option::Option<ISyncChange>, pcfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(cchanges), ::std::mem::transmute(ppchange), ::std::mem::transmute(pcfetched)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Skip(&self, cchanges: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(cchanges)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumSyncChanges> {
        let mut result__: <IEnumSyncChanges as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumSyncChanges>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumSyncChanges {
    type Vtable = IEnumSyncChanges_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1602666058, 24184, 20018, [172, 28, 194, 79, 210, 35, 239, 133]);
}
impl ::std::convert::From<IEnumSyncChanges> for ::windows::runtime::IUnknown {
    fn from(value: IEnumSyncChanges) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IEnumSyncChanges> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumSyncChanges) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumSyncChanges {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumSyncChanges {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSyncChanges_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cchanges: u32, ppchange: *mut ::windows::runtime::RawPtr, pcfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cchanges: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IEnumSyncProviderConfigUIInfos(pub ::windows::runtime::IUnknown);
impl IEnumSyncProviderConfigUIInfos {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Next(&self, cfactories: u32, ppsyncproviderconfiguiinfo: *mut ::std::option::Option<ISyncProviderConfigUIInfo>, pcfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(cfactories), ::std::mem::transmute(ppsyncproviderconfiguiinfo), ::std::mem::transmute(pcfetched)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Skip(&self, cfactories: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(cfactories)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumSyncProviderConfigUIInfos> {
        let mut result__: <IEnumSyncProviderConfigUIInfos as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumSyncProviderConfigUIInfos>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumSyncProviderConfigUIInfos {
    type Vtable = IEnumSyncProviderConfigUIInfos_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4139656706, 6086, 18008, [162, 215, 104, 237, 51, 48, 246, 65]);
}
impl ::std::convert::From<IEnumSyncProviderConfigUIInfos> for ::windows::runtime::IUnknown {
    fn from(value: IEnumSyncProviderConfigUIInfos) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IEnumSyncProviderConfigUIInfos> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumSyncProviderConfigUIInfos) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumSyncProviderConfigUIInfos {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumSyncProviderConfigUIInfos {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSyncProviderConfigUIInfos_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cfactories: u32, ppsyncproviderconfiguiinfo: *mut ::windows::runtime::RawPtr, pcfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cfactories: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IEnumSyncProviderInfos(pub ::windows::runtime::IUnknown);
impl IEnumSyncProviderInfos {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Next(&self, cinstances: u32, ppsyncproviderinfo: *mut ::std::option::Option<ISyncProviderInfo>, pcfetched: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(cinstances), ::std::mem::transmute(ppsyncproviderinfo), ::std::mem::transmute(pcfetched)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Skip(&self, cinstances: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(cinstances)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<IEnumSyncProviderInfos> {
        let mut result__: <IEnumSyncProviderInfos as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumSyncProviderInfos>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IEnumSyncProviderInfos {
    type Vtable = IEnumSyncProviderInfos_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2689312848, 24241, 17933, [169, 115, 57, 63, 203, 96, 138, 17]);
}
impl ::std::convert::From<IEnumSyncProviderInfos> for ::windows::runtime::IUnknown {
    fn from(value: IEnumSyncProviderInfos) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IEnumSyncProviderInfos> for ::windows::runtime::IUnknown {
    fn from(value: &IEnumSyncProviderInfos) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IEnumSyncProviderInfos {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IEnumSyncProviderInfos {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSyncProviderInfos_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cinstances: u32, ppsyncproviderinfo: *mut ::windows::runtime::RawPtr, pcfetched: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cinstances: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFeedClockVector(pub ::windows::runtime::IUnknown);
impl IFeedClockVector {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetClockVectorElements(&self, riid: *const ::windows::runtime::GUID, ppienumclockvector: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(ppienumclockvector)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetClockVectorElementCount(&self, pdwcount: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwcount)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetUpdateCount(&self, pdwupdatecount: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwupdatecount)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_Foundation`*"]
    pub unsafe fn IsNoConflictsSpecified(&self, pfisnoconflictsspecified: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pfisnoconflictsspecified)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFeedClockVector {
    type Vtable = IFeedClockVector_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2367527121, 40888, 20169, [165, 83, 84, 221, 146, 78, 15, 103]);
}
impl ::std::convert::From<IFeedClockVector> for ::windows::runtime::IUnknown {
    fn from(value: IFeedClockVector) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IFeedClockVector> for ::windows::runtime::IUnknown {
    fn from(value: &IFeedClockVector) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFeedClockVector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IFeedClockVector {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IFeedClockVector> for IClockVector {
    fn from(value: IFeedClockVector) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFeedClockVector> for IClockVector {
    fn from(value: &IFeedClockVector) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IClockVector> for IFeedClockVector {
    fn into_param(self) -> ::windows::runtime::Param<'a, IClockVector> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IClockVector>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IClockVector> for &IFeedClockVector {
    fn into_param(self) -> ::windows::runtime::Param<'a, IClockVector> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IClockVector>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFeedClockVector_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppienumclockvector: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwcount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwupdatecount: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfisnoconflictsspecified: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFeedClockVectorElement(pub ::windows::runtime::IUnknown);
impl IFeedClockVectorElement {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetReplicaKey(&self, pdwreplicakey: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwreplicakey)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetTickCount(&self, pulltickcount: *mut u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pulltickcount)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetSyncTime(&self, psynctime: *mut SYNC_TIME) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(psynctime)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetFlags(&self, pbflags: *mut u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbflags)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFeedClockVectorElement {
    type Vtable = IFeedClockVectorElement_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2752202450, 59771, 16726, [182, 218, 153, 31, 80, 27, 15, 5]);
}
impl ::std::convert::From<IFeedClockVectorElement> for ::windows::runtime::IUnknown {
    fn from(value: IFeedClockVectorElement) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IFeedClockVectorElement> for ::windows::runtime::IUnknown {
    fn from(value: &IFeedClockVectorElement) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFeedClockVectorElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IFeedClockVectorElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IFeedClockVectorElement> for IClockVectorElement {
    fn from(value: IFeedClockVectorElement) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IFeedClockVectorElement> for IClockVectorElement {
    fn from(value: &IFeedClockVectorElement) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IClockVectorElement> for IFeedClockVectorElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, IClockVectorElement> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IClockVectorElement>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, IClockVectorElement> for &IFeedClockVectorElement {
    fn into_param(self) -> ::windows::runtime::Param<'a, IClockVectorElement> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<IClockVectorElement>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFeedClockVectorElement_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwreplicakey: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pulltickcount: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psynctime: *mut SYNC_TIME) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbflags: *mut u8) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFilterKeyMap(pub ::windows::runtime::IUnknown);
impl IFilterKeyMap {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetCount(&self, pdwcount: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwcount)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn AddFilter<'a, Param0: ::windows::runtime::IntoParam<'a, ISyncFilter>>(&self, pisyncfilter: Param0, pdwfilterkey: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pisyncfilter.into_param().abi(), ::std::mem::transmute(pdwfilterkey)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetFilter(&self, dwfilterkey: u32) -> ::windows::runtime::Result<ISyncFilter> {
        let mut result__: <ISyncFilter as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwfilterkey), &mut result__).from_abi::<ISyncFilter>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Serialize(&self, pbfilterkeymap: *mut u8, pcbfilterkeymap: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbfilterkeymap), ::std::mem::transmute(pcbfilterkeymap)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFilterKeyMap {
    type Vtable = IFilterKeyMap_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3390477906, 1990, 18184, [163, 218, 110, 78, 186, 141, 34, 151]);
}
impl ::std::convert::From<IFilterKeyMap> for ::windows::runtime::IUnknown {
    fn from(value: IFilterKeyMap) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IFilterKeyMap> for ::windows::runtime::IUnknown {
    fn from(value: &IFilterKeyMap) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFilterKeyMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IFilterKeyMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFilterKeyMap_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwcount: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pisyncfilter: ::windows::runtime::RawPtr, pdwfilterkey: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwfilterkey: u32, ppisyncfilter: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbfilterkeymap: *mut u8, pcbfilterkeymap: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFilterRequestCallback(pub ::windows::runtime::IUnknown);
impl IFilterRequestCallback {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn RequestFilter<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, pfilter: Param0, filteringtype: FILTERING_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pfilter.into_param().abi(), ::std::mem::transmute(filteringtype)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFilterRequestCallback {
    type Vtable = IFilterRequestCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2195687539, 25440, 17978, [168, 161, 237, 229, 225, 161, 89, 77]);
}
impl ::std::convert::From<IFilterRequestCallback> for ::windows::runtime::IUnknown {
    fn from(value: IFilterRequestCallback) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IFilterRequestCallback> for ::windows::runtime::IUnknown {
    fn from(value: &IFilterRequestCallback) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFilterRequestCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IFilterRequestCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFilterRequestCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfilter: ::windows::runtime::RawPtr, filteringtype: FILTERING_TYPE) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFilterTrackingProvider(pub ::windows::runtime::IUnknown);
impl IFilterTrackingProvider {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn SpecifyTrackedFilters<'a, Param0: ::windows::runtime::IntoParam<'a, IFilterTrackingRequestCallback>>(&self, pcallback: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pcallback.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn AddTrackedFilter<'a, Param0: ::windows::runtime::IntoParam<'a, ISyncFilter>>(&self, pfilter: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pfilter.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFilterTrackingProvider {
    type Vtable = IFilterTrackingProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1949533120, 64590, 17850, [173, 129, 217, 216, 76, 122, 36, 248]);
}
impl ::std::convert::From<IFilterTrackingProvider> for ::windows::runtime::IUnknown {
    fn from(value: IFilterTrackingProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IFilterTrackingProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IFilterTrackingProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFilterTrackingProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IFilterTrackingProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFilterTrackingProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcallback: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfilter: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFilterTrackingRequestCallback(pub ::windows::runtime::IUnknown);
impl IFilterTrackingRequestCallback {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn RequestTrackedFilter<'a, Param0: ::windows::runtime::IntoParam<'a, ISyncFilter>>(&self, pfilter: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pfilter.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFilterTrackingRequestCallback {
    type Vtable = IFilterTrackingRequestCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1899800507, 51288, 18036, [180, 182, 17, 34, 67, 101, 135, 169]);
}
impl ::std::convert::From<IFilterTrackingRequestCallback> for ::windows::runtime::IUnknown {
    fn from(value: IFilterTrackingRequestCallback) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IFilterTrackingRequestCallback> for ::windows::runtime::IUnknown {
    fn from(value: &IFilterTrackingRequestCallback) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFilterTrackingRequestCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IFilterTrackingRequestCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFilterTrackingRequestCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfilter: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IFilterTrackingSyncChangeBuilder(pub ::windows::runtime::IUnknown);
impl IFilterTrackingSyncChangeBuilder {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_Foundation`*"]
    pub unsafe fn AddFilterChange(&self, dwfilterkey: u32, pfilterchange: *const SYNC_FILTER_CHANGE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwfilterkey), ::std::mem::transmute(pfilterchange)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn SetAllChangeUnitsPresentFlag(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IFilterTrackingSyncChangeBuilder {
    type Vtable = IFilterTrackingSyncChangeBuilder_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(693118112, 28890, 19544, [136, 60, 206, 42, 251, 48, 141, 11]);
}
impl ::std::convert::From<IFilterTrackingSyncChangeBuilder> for ::windows::runtime::IUnknown {
    fn from(value: IFilterTrackingSyncChangeBuilder) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IFilterTrackingSyncChangeBuilder> for ::windows::runtime::IUnknown {
    fn from(value: &IFilterTrackingSyncChangeBuilder) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IFilterTrackingSyncChangeBuilder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IFilterTrackingSyncChangeBuilder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IFilterTrackingSyncChangeBuilder_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwfilterkey: u32, pfilterchange: *const SYNC_FILTER_CHANGE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IForgottenKnowledge(pub ::windows::runtime::IUnknown);
impl IForgottenKnowledge {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetOwnerReplicaId(&self, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbreplicaid), ::std::mem::transmute(pcbidsize)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_Foundation`*"]
    pub unsafe fn Serialize<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fserializereplicakeymap: Param0, pbknowledge: *mut u8, pcbknowledge: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), fserializereplicakeymap.into_param().abi(), ::std::mem::transmute(pbknowledge), ::std::mem::transmute(pcbknowledge)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn SetLocalTickCount(&self, ulltickcount: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(ulltickcount)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn ContainsChange(&self, pbversionownerreplicaid: *const u8, pgiditemid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbversionownerreplicaid), ::std::mem::transmute(pgiditemid), ::std::mem::transmute(psyncversion)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn ContainsChangeUnit(&self, pbversionownerreplicaid: *const u8, pbitemid: *const u8, pbchangeunitid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbversionownerreplicaid), ::std::mem::transmute(pbitemid), ::std::mem::transmute(pbchangeunitid), ::std::mem::transmute(psyncversion)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetScopeVector(&self, riid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(ppunk)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetReplicaKeyMap(&self) -> ::windows::runtime::Result<IReplicaKeyMap> {
        let mut result__: <IReplicaKeyMap as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IReplicaKeyMap>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<ISyncKnowledge> {
        let mut result__: <ISyncKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ISyncKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn ConvertVersion<'a, Param0: ::windows::runtime::IntoParam<'a, ISyncKnowledge>>(&self, pknowledgein: Param0, pbcurrentownerid: *const u8, pversionin: *const SYNC_VERSION, pbnewownerid: *mut u8, pcbidsize: *mut u32, pversionout: *mut SYNC_VERSION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), pknowledgein.into_param().abi(), ::std::mem::transmute(pbcurrentownerid), ::std::mem::transmute(pversionin), ::std::mem::transmute(pbnewownerid), ::std::mem::transmute(pcbidsize), ::std::mem::transmute(pversionout)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn MapRemoteToLocal<'a, Param0: ::windows::runtime::IntoParam<'a, ISyncKnowledge>>(&self, premoteknowledge: Param0) -> ::windows::runtime::Result<ISyncKnowledge> {
        let mut result__: <ISyncKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), premoteknowledge.into_param().abi(), &mut result__).from_abi::<ISyncKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Union<'a, Param0: ::windows::runtime::IntoParam<'a, ISyncKnowledge>>(&self, pknowledge: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), pknowledge.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn ProjectOntoItem(&self, pbitemid: *const u8) -> ::windows::runtime::Result<ISyncKnowledge> {
        let mut result__: <ISyncKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbitemid), &mut result__).from_abi::<ISyncKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn ProjectOntoChangeUnit(&self, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows::runtime::Result<ISyncKnowledge> {
        let mut result__: <ISyncKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbitemid), ::std::mem::transmute(pbchangeunitid), &mut result__).from_abi::<ISyncKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn ProjectOntoRange(&self, psrngsyncrange: *const SYNC_RANGE) -> ::windows::runtime::Result<ISyncKnowledge> {
        let mut result__: <ISyncKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(psrngsyncrange), &mut result__).from_abi::<ISyncKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn ExcludeItem(&self, pbitemid: *const u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbitemid)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn ExcludeChangeUnit(&self, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbitemid), ::std::mem::transmute(pbchangeunitid)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn ContainsKnowledge<'a, Param0: ::windows::runtime::IntoParam<'a, ISyncKnowledge>>(&self, pknowledge: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), pknowledge.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn FindMinTickCountForReplica(&self, pbreplicaid: *const u8, pullreplicatickcount: *mut u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbreplicaid), ::std::mem::transmute(pullreplicatickcount)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetRangeExceptions(&self, riid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(ppunk)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetSingleItemExceptions(&self, riid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(ppunk)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetChangeUnitExceptions(&self, riid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(ppunk)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn FindClockVectorForItem(&self, pbitemid: *const u8, riid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbitemid), ::std::mem::transmute(riid), ::std::mem::transmute(ppunk)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn FindClockVectorForChangeUnit(&self, pbitemid: *const u8, pbchangeunitid: *const u8, riid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbitemid), ::std::mem::transmute(pbchangeunitid), ::std::mem::transmute(riid), ::std::mem::transmute(ppunk)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetVersion(&self, pdwversion: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwversion)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn ForgetToVersion<'a, Param0: ::windows::runtime::IntoParam<'a, ISyncKnowledge>>(&self, pknowledge: Param0, pversion: *const SYNC_VERSION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), pknowledge.into_param().abi(), ::std::mem::transmute(pversion)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IForgottenKnowledge {
    type Vtable = IForgottenKnowledge_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1164840854, 24630, 17707, [159, 157, 188, 196, 180, 168, 93, 178]);
}
impl ::std::convert::From<IForgottenKnowledge> for ::windows::runtime::IUnknown {
    fn from(value: IForgottenKnowledge) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IForgottenKnowledge> for ::windows::runtime::IUnknown {
    fn from(value: &IForgottenKnowledge) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IForgottenKnowledge {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IForgottenKnowledge {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IForgottenKnowledge> for ISyncKnowledge {
    fn from(value: IForgottenKnowledge) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IForgottenKnowledge> for ISyncKnowledge {
    fn from(value: &IForgottenKnowledge) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyncKnowledge> for IForgottenKnowledge {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyncKnowledge> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISyncKnowledge>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyncKnowledge> for &IForgottenKnowledge {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyncKnowledge> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISyncKnowledge>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IForgottenKnowledge_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fserializereplicakeymap: super::super::Foundation::BOOL, pbknowledge: *mut u8, pcbknowledge: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulltickcount: u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbversionownerreplicaid: *const u8, pgiditemid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbversionownerreplicaid: *const u8, pbitemid: *const u8, pbchangeunitid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppreplicakeymap: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppclonedknowledge: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pknowledgein: ::windows::runtime::RawPtr, pbcurrentownerid: *const u8, pversionin: *const SYNC_VERSION, pbnewownerid: *mut u8, pcbidsize: *mut u32, pversionout: *mut SYNC_VERSION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, premoteknowledge: ::windows::runtime::RawPtr, ppmappedknowledge: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pknowledge: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbitemid: *const u8, ppknowledgeout: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbitemid: *const u8, pbchangeunitid: *const u8, ppknowledgeout: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psrngsyncrange: *const SYNC_RANGE, ppknowledgeout: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbitemid: *const u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pknowledge: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbreplicaid: *const u8, pullreplicatickcount: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbitemid: *const u8, riid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbitemid: *const u8, pbchangeunitid: *const u8, riid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwversion: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pknowledge: ::windows::runtime::RawPtr, pversion: *const SYNC_VERSION) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IKnowledgeSyncProvider(pub ::windows::runtime::IUnknown);
impl IKnowledgeSyncProvider {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_Foundation`*"]
    pub unsafe fn GetIdParameters(&self, pidparameters: *mut ID_PARAMETERS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pidparameters)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn BeginSession<'a, Param1: ::windows::runtime::IntoParam<'a, ISyncSessionState>>(&self, role: SYNC_PROVIDER_ROLE, psessionstate: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(role), psessionstate.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetSyncBatchParameters(&self, ppsyncknowledge: *mut ::std::option::Option<ISyncKnowledge>, pdwrequestedbatchsize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppsyncknowledge), ::std::mem::transmute(pdwrequestedbatchsize)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetChangeBatch<'a, Param1: ::windows::runtime::IntoParam<'a, ISyncKnowledge>>(&self, dwbatchsize: u32, psyncknowledge: Param1, ppsyncchangebatch: *mut ::std::option::Option<ISyncChangeBatch>, ppunkdataretriever: *mut ::std::option::Option<::windows::runtime::IUnknown>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwbatchsize), psyncknowledge.into_param().abi(), ::std::mem::transmute(ppsyncchangebatch), ::std::mem::transmute(ppunkdataretriever)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetFullEnumerationChangeBatch<'a, Param2: ::windows::runtime::IntoParam<'a, ISyncKnowledge>>(&self, dwbatchsize: u32, pblowerenumerationbound: *const u8, psyncknowledge: Param2, ppsyncchangebatch: *mut ::std::option::Option<ISyncFullEnumerationChangeBatch>, ppunkdataretriever: *mut ::std::option::Option<::windows::runtime::IUnknown>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwbatchsize), ::std::mem::transmute(pblowerenumerationbound), psyncknowledge.into_param().abi(), ::std::mem::transmute(ppsyncchangebatch), ::std::mem::transmute(ppunkdataretriever)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn ProcessChangeBatch<'a, Param1: ::windows::runtime::IntoParam<'a, ISyncChangeBatch>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param3: ::windows::runtime::IntoParam<'a, ISyncCallback>>(&self, resolutionpolicy: CONFLICT_RESOLUTION_POLICY, psourcechangebatch: Param1, punkdataretriever: Param2, pcallback: Param3, psyncsessionstatistics: *mut SYNC_SESSION_STATISTICS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(resolutionpolicy), psourcechangebatch.into_param().abi(), punkdataretriever.into_param().abi(), pcallback.into_param().abi(), ::std::mem::transmute(psyncsessionstatistics)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn ProcessFullEnumerationChangeBatch<'a, Param1: ::windows::runtime::IntoParam<'a, ISyncFullEnumerationChangeBatch>, Param2: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param3: ::windows::runtime::IntoParam<'a, ISyncCallback>>(&self, resolutionpolicy: CONFLICT_RESOLUTION_POLICY, psourcechangebatch: Param1, punkdataretriever: Param2, pcallback: Param3, psyncsessionstatistics: *mut SYNC_SESSION_STATISTICS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(resolutionpolicy), psourcechangebatch.into_param().abi(), punkdataretriever.into_param().abi(), pcallback.into_param().abi(), ::std::mem::transmute(psyncsessionstatistics)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn EndSession<'a, Param0: ::windows::runtime::IntoParam<'a, ISyncSessionState>>(&self, psessionstate: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), psessionstate.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IKnowledgeSyncProvider {
    type Vtable = IKnowledgeSyncProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1128483401, 36260, 18418, [129, 114, 173, 123, 139, 2, 73, 120]);
}
impl ::std::convert::From<IKnowledgeSyncProvider> for ::windows::runtime::IUnknown {
    fn from(value: IKnowledgeSyncProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IKnowledgeSyncProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IKnowledgeSyncProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IKnowledgeSyncProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IKnowledgeSyncProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<IKnowledgeSyncProvider> for ISyncProvider {
    fn from(value: IKnowledgeSyncProvider) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&IKnowledgeSyncProvider> for ISyncProvider {
    fn from(value: &IKnowledgeSyncProvider) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyncProvider> for IKnowledgeSyncProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyncProvider> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISyncProvider>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyncProvider> for &IKnowledgeSyncProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyncProvider> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISyncProvider>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnowledgeSyncProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pidparameters: *mut ID_PARAMETERS) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, role: SYNC_PROVIDER_ROLE, psessionstate: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppsyncknowledge: *mut ::windows::runtime::RawPtr, pdwrequestedbatchsize: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwbatchsize: u32, psyncknowledge: ::windows::runtime::RawPtr, ppsyncchangebatch: *mut ::windows::runtime::RawPtr, ppunkdataretriever: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwbatchsize: u32, pblowerenumerationbound: *const u8, psyncknowledge: ::windows::runtime::RawPtr, ppsyncchangebatch: *mut ::windows::runtime::RawPtr, ppunkdataretriever: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resolutionpolicy: CONFLICT_RESOLUTION_POLICY, psourcechangebatch: ::windows::runtime::RawPtr, punkdataretriever: ::windows::runtime::RawPtr, pcallback: ::windows::runtime::RawPtr, psyncsessionstatistics: *mut SYNC_SESSION_STATISTICS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, resolutionpolicy: CONFLICT_RESOLUTION_POLICY, psourcechangebatch: ::windows::runtime::RawPtr, punkdataretriever: ::windows::runtime::RawPtr, pcallback: ::windows::runtime::RawPtr, psyncsessionstatistics: *mut SYNC_SESSION_STATISTICS) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psessionstate: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ILoadChangeContext(pub ::windows::runtime::IUnknown);
impl ILoadChangeContext {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetSyncChange(&self) -> ::windows::runtime::Result<ISyncChange> {
        let mut result__: <ISyncChange as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ISyncChange>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn SetRecoverableErrorOnChange<'a, Param1: ::windows::runtime::IntoParam<'a, IRecoverableErrorData>>(&self, hrerror: ::windows::runtime::HRESULT, perrordata: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(hrerror), perrordata.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn SetRecoverableErrorOnChangeUnit<'a, Param1: ::windows::runtime::IntoParam<'a, ISyncChangeUnit>, Param2: ::windows::runtime::IntoParam<'a, IRecoverableErrorData>>(&self, hrerror: ::windows::runtime::HRESULT, pchangeunit: Param1, perrordata: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(hrerror), pchangeunit.into_param().abi(), perrordata.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ILoadChangeContext {
    type Vtable = ILoadChangeContext_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1151642314, 60473, 18133, [181, 201, 214, 51, 192, 238, 103, 226]);
}
impl ::std::convert::From<ILoadChangeContext> for ::windows::runtime::IUnknown {
    fn from(value: ILoadChangeContext) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ILoadChangeContext> for ::windows::runtime::IUnknown {
    fn from(value: &ILoadChangeContext) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ILoadChangeContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ILoadChangeContext {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoadChangeContext_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppsyncchange: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hrerror: ::windows::runtime::HRESULT, perrordata: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hrerror: ::windows::runtime::HRESULT, pchangeunit: ::windows::runtime::RawPtr, perrordata: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IProviderConverter(pub ::windows::runtime::IUnknown);
impl IProviderConverter {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, ISyncProvider>>(&self, pisyncprovider: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pisyncprovider.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IProviderConverter {
    type Vtable = IProviderConverter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2157671030, 39119, 18775, [147, 165, 14, 189, 211, 221, 223, 253]);
}
impl ::std::convert::From<IProviderConverter> for ::windows::runtime::IUnknown {
    fn from(value: IProviderConverter) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IProviderConverter> for ::windows::runtime::IUnknown {
    fn from(value: &IProviderConverter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IProviderConverter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IProviderConverter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProviderConverter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pisyncprovider: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IRangeException(pub ::windows::runtime::IUnknown);
impl IRangeException {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetClosedRangeStart(&self, pbclosedrangestart: *mut u8, pcbidsize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbclosedrangestart), ::std::mem::transmute(pcbidsize)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetClosedRangeEnd(&self, pbclosedrangeend: *mut u8, pcbidsize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbclosedrangeend), ::std::mem::transmute(pcbidsize)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetClockVector(&self, riid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(ppunk)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IRangeException {
    type Vtable = IRangeException_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1974372215, 26696, 18935, [149, 108, 163, 169, 47, 80, 150, 232]);
}
impl ::std::convert::From<IRangeException> for ::windows::runtime::IUnknown {
    fn from(value: IRangeException) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IRangeException> for ::windows::runtime::IUnknown {
    fn from(value: &IRangeException) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRangeException {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IRangeException {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRangeException_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbclosedrangestart: *mut u8, pcbidsize: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbclosedrangeend: *mut u8, pcbidsize: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IRecoverableError(pub ::windows::runtime::IUnknown);
impl IRecoverableError {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetStage(&self, pstage: *mut SYNC_PROGRESS_STAGE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pstage)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetProvider(&self, pproviderrole: *mut SYNC_PROVIDER_ROLE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pproviderrole)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetChangeWithRecoverableError(&self) -> ::windows::runtime::Result<ISyncChange> {
        let mut result__: <ISyncChange as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ISyncChange>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetRecoverableErrorDataForChange(&self, phrerror: *mut ::windows::runtime::HRESULT, pperrordata: *mut ::std::option::Option<IRecoverableErrorData>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(phrerror), ::std::mem::transmute(pperrordata)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetRecoverableErrorDataForChangeUnit<'a, Param0: ::windows::runtime::IntoParam<'a, ISyncChangeUnit>>(&self, pchangeunit: Param0, phrerror: *mut ::windows::runtime::HRESULT, pperrordata: *mut ::std::option::Option<IRecoverableErrorData>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), pchangeunit.into_param().abi(), ::std::mem::transmute(phrerror), ::std::mem::transmute(pperrordata)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IRecoverableError {
    type Vtable = IRecoverableError_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(257304040, 2683, 17902, [150, 55, 28, 225, 54, 69, 144, 158]);
}
impl ::std::convert::From<IRecoverableError> for ::windows::runtime::IUnknown {
    fn from(value: IRecoverableError) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IRecoverableError> for ::windows::runtime::IUnknown {
    fn from(value: &IRecoverableError) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRecoverableError {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IRecoverableError {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRecoverableError_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pstage: *mut SYNC_PROGRESS_STAGE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pproviderrole: *mut SYNC_PROVIDER_ROLE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppchangewithrecoverableerror: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phrerror: *mut ::windows::runtime::HRESULT, pperrordata: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pchangeunit: ::windows::runtime::RawPtr, phrerror: *mut ::windows::runtime::HRESULT, pperrordata: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IRecoverableErrorData(pub ::windows::runtime::IUnknown);
impl IRecoverableErrorData {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_Foundation`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pcszitemdisplayname: Param0, pcszerrordescription: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pcszitemdisplayname.into_param().abi(), pcszerrordescription.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_Foundation`*"]
    pub unsafe fn GetItemDisplayName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszitemdisplayname: Param0, pcchitemdisplayname: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pszitemdisplayname.into_param().abi(), ::std::mem::transmute(pcchitemdisplayname)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_Foundation`*"]
    pub unsafe fn GetErrorDescription<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszerrordescription: Param0, pccherrordescription: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pszerrordescription.into_param().abi(), ::std::mem::transmute(pccherrordescription)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IRecoverableErrorData {
    type Vtable = IRecoverableErrorData_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3011267082, 19325, 19501, [151, 17, 59, 0, 209, 25, 177, 200]);
}
impl ::std::convert::From<IRecoverableErrorData> for ::windows::runtime::IUnknown {
    fn from(value: IRecoverableErrorData) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IRecoverableErrorData> for ::windows::runtime::IUnknown {
    fn from(value: &IRecoverableErrorData) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRecoverableErrorData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IRecoverableErrorData {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRecoverableErrorData_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcszitemdisplayname: super::super::Foundation::PWSTR, pcszerrordescription: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszitemdisplayname: super::super::Foundation::PWSTR, pcchitemdisplayname: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszerrordescription: super::super::Foundation::PWSTR, pccherrordescription: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IRegisteredSyncProvider(pub ::windows::runtime::IUnknown);
impl IRegisteredSyncProvider {
    #[cfg(feature = "Win32_System_PropertiesSystem")]
    #[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn Init<'a, Param2: ::windows::runtime::IntoParam<'a, super::PropertiesSystem::IPropertyStore>>(&self, pguidinstanceid: *const ::windows::runtime::GUID, pguidcontenttype: *const ::windows::runtime::GUID, pcontextpropertystore: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pguidinstanceid), ::std::mem::transmute(pguidcontenttype), pcontextpropertystore.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetInstanceId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IRegisteredSyncProvider {
    type Vtable = IRegisteredSyncProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2436616054, 18369, 16565, [168, 150, 94, 138, 156, 65, 76, 20]);
}
impl ::std::convert::From<IRegisteredSyncProvider> for ::windows::runtime::IUnknown {
    fn from(value: IRegisteredSyncProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IRegisteredSyncProvider> for ::windows::runtime::IUnknown {
    fn from(value: &IRegisteredSyncProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRegisteredSyncProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IRegisteredSyncProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRegisteredSyncProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidinstanceid: *const ::windows::runtime::GUID, pguidcontenttype: *const ::windows::runtime::GUID, pcontextpropertystore: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_PropertiesSystem"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidinstanceid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IReplicaKeyMap(pub ::windows::runtime::IUnknown);
impl IReplicaKeyMap {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn LookupReplicaKey(&self, pbreplicaid: *const u8, pdwreplicakey: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbreplicaid), ::std::mem::transmute(pdwreplicakey)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn LookupReplicaId(&self, dwreplicakey: u32, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwreplicakey), ::std::mem::transmute(pbreplicaid), ::std::mem::transmute(pcbidsize)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Serialize(&self, pbreplicakeymap: *mut u8, pcbreplicakeymap: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbreplicakeymap), ::std::mem::transmute(pcbreplicakeymap)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IReplicaKeyMap {
    type Vtable = IReplicaKeyMap_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(571077884, 64784, 20464, [132, 168, 240, 161, 152, 46, 68, 14]);
}
impl ::std::convert::From<IReplicaKeyMap> for ::windows::runtime::IUnknown {
    fn from(value: IReplicaKeyMap) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IReplicaKeyMap> for ::windows::runtime::IUnknown {
    fn from(value: &IReplicaKeyMap) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IReplicaKeyMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IReplicaKeyMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IReplicaKeyMap_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbreplicaid: *const u8, pdwreplicakey: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwreplicakey: u32, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbreplicakeymap: *mut u8, pcbreplicakeymap: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct IRequestFilteredSync(pub ::windows::runtime::IUnknown);
impl IRequestFilteredSync {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn SpecifyFilter<'a, Param0: ::windows::runtime::IntoParam<'a, IFilterRequestCallback>>(&self, pcallback: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pcallback.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IRequestFilteredSync {
    type Vtable = IRequestFilteredSync_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(771883396, 27928, 18087, [163, 42, 218, 74, 235, 6, 105, 108]);
}
impl ::std::convert::From<IRequestFilteredSync> for ::windows::runtime::IUnknown {
    fn from(value: IRequestFilteredSync) -> Self {
        value.0
    }
}
impl ::std::convert::From<&IRequestFilteredSync> for ::windows::runtime::IUnknown {
    fn from(value: &IRequestFilteredSync) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IRequestFilteredSync {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IRequestFilteredSync {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRequestFilteredSync_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcallback: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISingleItemException(pub ::windows::runtime::IUnknown);
impl ISingleItemException {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetItemId(&self, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbitemid), ::std::mem::transmute(pcbidsize)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetClockVector(&self, riid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(ppunk)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISingleItemException {
    type Vtable = ISingleItemException_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2301606320, 31829, 18968, [147, 22, 253, 244, 73, 86, 155, 100]);
}
impl ::std::convert::From<ISingleItemException> for ::windows::runtime::IUnknown {
    fn from(value: ISingleItemException) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISingleItemException> for ::windows::runtime::IUnknown {
    fn from(value: &ISingleItemException) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISingleItemException {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISingleItemException {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISingleItemException_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISupportFilteredSync(pub ::windows::runtime::IUnknown);
impl ISupportFilteredSync {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn AddFilter<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, pfilter: Param0, filteringtype: FILTERING_TYPE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pfilter.into_param().abi(), ::std::mem::transmute(filteringtype)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISupportFilteredSync {
    type Vtable = ISupportFilteredSync_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1024626157, 54613, 19981, [191, 75, 251, 33, 58, 138, 147, 2]);
}
impl ::std::convert::From<ISupportFilteredSync> for ::windows::runtime::IUnknown {
    fn from(value: ISupportFilteredSync) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISupportFilteredSync> for ::windows::runtime::IUnknown {
    fn from(value: &ISupportFilteredSync) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISupportFilteredSync {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISupportFilteredSync {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISupportFilteredSync_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfilter: ::windows::runtime::RawPtr, filteringtype: FILTERING_TYPE) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISupportLastWriteTime(pub ::windows::runtime::IUnknown);
impl ISupportLastWriteTime {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetItemChangeTime(&self, pbitemid: *const u8, pulltimestamp: *mut u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbitemid), ::std::mem::transmute(pulltimestamp)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetChangeUnitChangeTime(&self, pbitemid: *const u8, pbchangeunitid: *const u8, pulltimestamp: *mut u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbitemid), ::std::mem::transmute(pbchangeunitid), ::std::mem::transmute(pulltimestamp)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISupportLastWriteTime {
    type Vtable = ISupportLastWriteTime_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3940516207, 53437, 17354, [143, 64, 90, 205, 198, 192, 111, 122]);
}
impl ::std::convert::From<ISupportLastWriteTime> for ::windows::runtime::IUnknown {
    fn from(value: ISupportLastWriteTime) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISupportLastWriteTime> for ::windows::runtime::IUnknown {
    fn from(value: &ISupportLastWriteTime) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISupportLastWriteTime {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISupportLastWriteTime {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISupportLastWriteTime_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbitemid: *const u8, pulltimestamp: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbitemid: *const u8, pbchangeunitid: *const u8, pulltimestamp: *mut u64) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISyncCallback(pub ::windows::runtime::IUnknown);
impl ISyncCallback {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn OnProgress(&self, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(provider), ::std::mem::transmute(syncstage), ::std::mem::transmute(dwcompletedwork), ::std::mem::transmute(dwtotalwork)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn OnChange<'a, Param0: ::windows::runtime::IntoParam<'a, ISyncChange>>(&self, psyncchange: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), psyncchange.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn OnConflict<'a, Param0: ::windows::runtime::IntoParam<'a, IChangeConflict>>(&self, pconflict: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pconflict.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn OnFullEnumerationNeeded(&self, pfullenumerationaction: *mut SYNC_FULL_ENUMERATION_ACTION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pfullenumerationaction)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn OnRecoverableError<'a, Param0: ::windows::runtime::IntoParam<'a, IRecoverableError>>(&self, precoverableerror: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), precoverableerror.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISyncCallback {
    type Vtable = ISyncCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(93944191, 24281, 18524, [174, 54, 12, 93, 27, 242, 231, 165]);
}
impl ::std::convert::From<ISyncCallback> for ::windows::runtime::IUnknown {
    fn from(value: ISyncCallback) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISyncCallback> for ::windows::runtime::IUnknown {
    fn from(value: &ISyncCallback) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISyncCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISyncCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psyncchange: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pconflict: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfullenumerationaction: *mut SYNC_FULL_ENUMERATION_ACTION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, precoverableerror: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISyncCallback2(pub ::windows::runtime::IUnknown);
impl ISyncCallback2 {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn OnProgress(&self, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(provider), ::std::mem::transmute(syncstage), ::std::mem::transmute(dwcompletedwork), ::std::mem::transmute(dwtotalwork)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn OnChange<'a, Param0: ::windows::runtime::IntoParam<'a, ISyncChange>>(&self, psyncchange: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), psyncchange.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn OnConflict<'a, Param0: ::windows::runtime::IntoParam<'a, IChangeConflict>>(&self, pconflict: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pconflict.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn OnFullEnumerationNeeded(&self, pfullenumerationaction: *mut SYNC_FULL_ENUMERATION_ACTION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pfullenumerationaction)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn OnRecoverableError<'a, Param0: ::windows::runtime::IntoParam<'a, IRecoverableError>>(&self, precoverableerror: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), precoverableerror.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn OnChangeApplied(&self, dwchangesapplied: u32, dwchangesfailed: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwchangesapplied), ::std::mem::transmute(dwchangesfailed)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn OnChangeFailed(&self, dwchangesapplied: u32, dwchangesfailed: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwchangesapplied), ::std::mem::transmute(dwchangesfailed)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISyncCallback2 {
    type Vtable = ISyncCallback2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1204716719, 29762, 20141, [134, 48, 18, 1, 94, 3, 10, 215]);
}
impl ::std::convert::From<ISyncCallback2> for ::windows::runtime::IUnknown {
    fn from(value: ISyncCallback2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISyncCallback2> for ::windows::runtime::IUnknown {
    fn from(value: &ISyncCallback2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISyncCallback2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISyncCallback2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<ISyncCallback2> for ISyncCallback {
    fn from(value: ISyncCallback2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISyncCallback2> for ISyncCallback {
    fn from(value: &ISyncCallback2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyncCallback> for ISyncCallback2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyncCallback> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISyncCallback>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyncCallback> for &ISyncCallback2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyncCallback> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISyncCallback>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncCallback2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psyncchange: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pconflict: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfullenumerationaction: *mut SYNC_FULL_ENUMERATION_ACTION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, precoverableerror: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwchangesapplied: u32, dwchangesfailed: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwchangesapplied: u32, dwchangesfailed: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISyncChange(pub ::windows::runtime::IUnknown);
impl ISyncChange {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetOwnerReplicaId(&self, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbreplicaid), ::std::mem::transmute(pcbidsize)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetRootItemId(&self, pbrootitemid: *mut u8, pcbidsize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbrootitemid), ::std::mem::transmute(pcbidsize)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetChangeVersion(&self, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbcurrentreplicaid), ::std::mem::transmute(pversion)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetCreationVersion(&self, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbcurrentreplicaid), ::std::mem::transmute(pversion)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetFlags(&self, pdwflags: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwflags)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetWorkEstimate(&self, pdwwork: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwwork)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetChangeUnits(&self) -> ::windows::runtime::Result<IEnumSyncChangeUnits> {
        let mut result__: <IEnumSyncChangeUnits as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumSyncChangeUnits>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetMadeWithKnowledge(&self) -> ::windows::runtime::Result<ISyncKnowledge> {
        let mut result__: <ISyncKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ISyncKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetLearnedKnowledge(&self) -> ::windows::runtime::Result<ISyncKnowledge> {
        let mut result__: <ISyncKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ISyncKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn SetWorkEstimate(&self, dwwork: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwwork)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISyncChange {
    type Vtable = ISyncChange_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2710907883, 3947, 18193, [177, 54, 1, 218, 133, 185, 104, 166]);
}
impl ::std::convert::From<ISyncChange> for ::windows::runtime::IUnknown {
    fn from(value: ISyncChange) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISyncChange> for ::windows::runtime::IUnknown {
    fn from(value: &ISyncChange) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISyncChange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISyncChange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncChange_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbrootitemid: *mut u8, pcbidsize: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwflags: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwwork: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppmadewithknowledge: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pplearnedknowledge: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwwork: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISyncChangeBatch(pub ::windows::runtime::IUnknown);
impl ISyncChangeBatch {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetChangeEnumerator(&self) -> ::windows::runtime::Result<IEnumSyncChanges> {
        let mut result__: <IEnumSyncChanges as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumSyncChanges>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_Foundation`*"]
    pub unsafe fn GetIsLastBatch(&self, pflastbatch: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pflastbatch)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetWorkEstimateForBatch(&self, pdwworkforbatch: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwworkforbatch)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetRemainingWorkEstimateForSession(&self, pdwremainingworkforsession: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwremainingworkforsession)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn BeginOrderedGroup(&self, pblowerbound: *const u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(pblowerbound)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn EndOrderedGroup<'a, Param1: ::windows::runtime::IntoParam<'a, ISyncKnowledge>>(&self, pbupperbound: *const u8, pmadewithknowledge: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbupperbound), pmadewithknowledge.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn AddItemMetadataToGroup(&self, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32) -> ::windows::runtime::Result<ISyncChangeBuilder> {
        let mut result__: <ISyncChangeBuilder as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbownerreplicaid), ::std::mem::transmute(pbitemid), ::std::mem::transmute(pchangeversion), ::std::mem::transmute(pcreationversion), ::std::mem::transmute(dwflags), ::std::mem::transmute(dwworkforchange), &mut result__).from_abi::<ISyncChangeBuilder>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetLearnedKnowledge(&self) -> ::windows::runtime::Result<ISyncKnowledge> {
        let mut result__: <ISyncKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ISyncKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetPrerequisiteKnowledge(&self) -> ::windows::runtime::Result<ISyncKnowledge> {
        let mut result__: <ISyncKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ISyncKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetSourceForgottenKnowledge(&self) -> ::windows::runtime::Result<IForgottenKnowledge> {
        let mut result__: <IForgottenKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IForgottenKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn SetLastBatch(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn SetWorkEstimateForBatch(&self, dwworkforbatch: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwworkforbatch)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn SetRemainingWorkEstimateForSession(&self, dwremainingworkforsession: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwremainingworkforsession)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Serialize(&self, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbchangebatch), ::std::mem::transmute(pcbchangebatch)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn BeginUnorderedGroup(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_Foundation`*"]
    pub unsafe fn EndUnorderedGroup<'a, Param0: ::windows::runtime::IntoParam<'a, ISyncKnowledge>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pmadewithknowledge: Param0, fallchangesforknowledge: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), pmadewithknowledge.into_param().abi(), fallchangesforknowledge.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn AddLoggedConflict<'a, Param6: ::windows::runtime::IntoParam<'a, ISyncKnowledge>>(&self, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32, pconflictknowledge: Param6) -> ::windows::runtime::Result<ISyncChangeBuilder> {
        let mut result__: <ISyncChangeBuilder as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbownerreplicaid),
            ::std::mem::transmute(pbitemid),
            ::std::mem::transmute(pchangeversion),
            ::std::mem::transmute(pcreationversion),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(dwworkforchange),
            pconflictknowledge.into_param().abi(),
            &mut result__,
        )
        .from_abi::<ISyncChangeBuilder>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISyncChangeBatch {
    type Vtable = ISyncChangeBatch_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1892044270, 14351, 19502, [143, 112, 49, 197, 91, 213, 249, 179]);
}
impl ::std::convert::From<ISyncChangeBatch> for ::windows::runtime::IUnknown {
    fn from(value: ISyncChangeBatch) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISyncChangeBatch> for ::windows::runtime::IUnknown {
    fn from(value: &ISyncChangeBatch) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISyncChangeBatch {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISyncChangeBatch {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<ISyncChangeBatch> for ISyncChangeBatchBase {
    fn from(value: ISyncChangeBatch) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISyncChangeBatch> for ISyncChangeBatchBase {
    fn from(value: &ISyncChangeBatch) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyncChangeBatchBase> for ISyncChangeBatch {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyncChangeBatchBase> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISyncChangeBatchBase>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyncChangeBatchBase> for &ISyncChangeBatch {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyncChangeBatchBase> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISyncChangeBatchBase>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncChangeBatch_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pflastbatch: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwworkforbatch: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwremainingworkforsession: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pblowerbound: *const u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbupperbound: *const u8, pmadewithknowledge: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32, ppchangebuilder: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pplearnedknowledge: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppprerequisteknowledge: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppsourceforgottenknowledge: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwworkforbatch: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwremainingworkforsession: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmadewithknowledge: ::windows::runtime::RawPtr, fallchangesforknowledge: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32, pconflictknowledge: ::windows::runtime::RawPtr, ppchangebuilder: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISyncChangeBatch2(pub ::windows::runtime::IUnknown);
impl ISyncChangeBatch2 {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetChangeEnumerator(&self) -> ::windows::runtime::Result<IEnumSyncChanges> {
        let mut result__: <IEnumSyncChanges as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumSyncChanges>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_Foundation`*"]
    pub unsafe fn GetIsLastBatch(&self, pflastbatch: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pflastbatch)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetWorkEstimateForBatch(&self, pdwworkforbatch: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwworkforbatch)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetRemainingWorkEstimateForSession(&self, pdwremainingworkforsession: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwremainingworkforsession)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn BeginOrderedGroup(&self, pblowerbound: *const u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(pblowerbound)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn EndOrderedGroup<'a, Param1: ::windows::runtime::IntoParam<'a, ISyncKnowledge>>(&self, pbupperbound: *const u8, pmadewithknowledge: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbupperbound), pmadewithknowledge.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn AddItemMetadataToGroup(&self, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32) -> ::windows::runtime::Result<ISyncChangeBuilder> {
        let mut result__: <ISyncChangeBuilder as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbownerreplicaid), ::std::mem::transmute(pbitemid), ::std::mem::transmute(pchangeversion), ::std::mem::transmute(pcreationversion), ::std::mem::transmute(dwflags), ::std::mem::transmute(dwworkforchange), &mut result__).from_abi::<ISyncChangeBuilder>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetLearnedKnowledge(&self) -> ::windows::runtime::Result<ISyncKnowledge> {
        let mut result__: <ISyncKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ISyncKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetPrerequisiteKnowledge(&self) -> ::windows::runtime::Result<ISyncKnowledge> {
        let mut result__: <ISyncKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ISyncKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetSourceForgottenKnowledge(&self) -> ::windows::runtime::Result<IForgottenKnowledge> {
        let mut result__: <IForgottenKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IForgottenKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn SetLastBatch(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn SetWorkEstimateForBatch(&self, dwworkforbatch: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwworkforbatch)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn SetRemainingWorkEstimateForSession(&self, dwremainingworkforsession: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwremainingworkforsession)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Serialize(&self, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbchangebatch), ::std::mem::transmute(pcbchangebatch)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn BeginUnorderedGroup(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_Foundation`*"]
    pub unsafe fn EndUnorderedGroup<'a, Param0: ::windows::runtime::IntoParam<'a, ISyncKnowledge>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pmadewithknowledge: Param0, fallchangesforknowledge: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), pmadewithknowledge.into_param().abi(), fallchangesforknowledge.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn AddLoggedConflict<'a, Param6: ::windows::runtime::IntoParam<'a, ISyncKnowledge>>(&self, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32, pconflictknowledge: Param6) -> ::windows::runtime::Result<ISyncChangeBuilder> {
        let mut result__: <ISyncChangeBuilder as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbownerreplicaid),
            ::std::mem::transmute(pbitemid),
            ::std::mem::transmute(pchangeversion),
            ::std::mem::transmute(pcreationversion),
            ::std::mem::transmute(dwflags),
            ::std::mem::transmute(dwworkforchange),
            pconflictknowledge.into_param().abi(),
            &mut result__,
        )
        .from_abi::<ISyncChangeBuilder>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn AddMergeTombstoneMetadataToGroup(&self, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32) -> ::windows::runtime::Result<ISyncChangeBuilder> {
        let mut result__: <ISyncChangeBuilder as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbownerreplicaid), ::std::mem::transmute(pbwinneritemid), ::std::mem::transmute(pbitemid), ::std::mem::transmute(pchangeversion), ::std::mem::transmute(pcreationversion), ::std::mem::transmute(dwworkforchange), &mut result__).from_abi::<ISyncChangeBuilder>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn AddMergeTombstoneLoggedConflict<'a, Param6: ::windows::runtime::IntoParam<'a, ISyncKnowledge>>(&self, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, pconflictknowledge: Param6) -> ::windows::runtime::Result<ISyncChangeBuilder> {
        let mut result__: <ISyncChangeBuilder as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).21)(
            ::std::mem::transmute_copy(self),
            ::std::mem::transmute(pbownerreplicaid),
            ::std::mem::transmute(pbwinneritemid),
            ::std::mem::transmute(pbitemid),
            ::std::mem::transmute(pchangeversion),
            ::std::mem::transmute(pcreationversion),
            ::std::mem::transmute(dwworkforchange),
            pconflictknowledge.into_param().abi(),
            &mut result__,
        )
        .from_abi::<ISyncChangeBuilder>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISyncChangeBatch2 {
    type Vtable = ISyncChangeBatch2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(576670259, 62958, 19655, [176, 57, 103, 162, 98, 180, 178, 172]);
}
impl ::std::convert::From<ISyncChangeBatch2> for ::windows::runtime::IUnknown {
    fn from(value: ISyncChangeBatch2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISyncChangeBatch2> for ::windows::runtime::IUnknown {
    fn from(value: &ISyncChangeBatch2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISyncChangeBatch2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISyncChangeBatch2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<ISyncChangeBatch2> for ISyncChangeBatch {
    fn from(value: ISyncChangeBatch2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISyncChangeBatch2> for ISyncChangeBatch {
    fn from(value: &ISyncChangeBatch2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyncChangeBatch> for ISyncChangeBatch2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyncChangeBatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISyncChangeBatch>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyncChangeBatch> for &ISyncChangeBatch2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyncChangeBatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISyncChangeBatch>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ISyncChangeBatch2> for ISyncChangeBatchBase {
    fn from(value: ISyncChangeBatch2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISyncChangeBatch2> for ISyncChangeBatchBase {
    fn from(value: &ISyncChangeBatch2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyncChangeBatchBase> for ISyncChangeBatch2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyncChangeBatchBase> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISyncChangeBatchBase>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyncChangeBatchBase> for &ISyncChangeBatch2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyncChangeBatchBase> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISyncChangeBatchBase>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncChangeBatch2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pflastbatch: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwworkforbatch: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwremainingworkforsession: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pblowerbound: *const u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbupperbound: *const u8, pmadewithknowledge: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32, ppchangebuilder: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pplearnedknowledge: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppprerequisteknowledge: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppsourceforgottenknowledge: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwworkforbatch: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwremainingworkforsession: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmadewithknowledge: ::windows::runtime::RawPtr, fallchangesforknowledge: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32, pconflictknowledge: ::windows::runtime::RawPtr, ppchangebuilder: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, ppchangebuilder: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, pconflictknowledge: ::windows::runtime::RawPtr, ppchangebuilder: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISyncChangeBatchAdvanced(pub ::windows::runtime::IUnknown);
impl ISyncChangeBatchAdvanced {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetFilterInfo(&self) -> ::windows::runtime::Result<ISyncFilterInfo> {
        let mut result__: <ISyncFilterInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ISyncFilterInfo>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn ConvertFullEnumerationChangeBatchToRegularChangeBatch(&self) -> ::windows::runtime::Result<ISyncChangeBatch> {
        let mut result__: <ISyncChangeBatch as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ISyncChangeBatch>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetUpperBoundItemId(&self, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbitemid), ::std::mem::transmute(pcbidsize)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_Foundation`*"]
    pub unsafe fn GetBatchLevelKnowledgeShouldBeApplied(&self, pfbatchknowledgeshouldbeapplied: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pfbatchknowledgeshouldbeapplied)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISyncChangeBatchAdvanced {
    type Vtable = ISyncChangeBatchAdvanced_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(253381013, 52168, 16925, [181, 80, 93, 11, 235, 243, 233, 165]);
}
impl ::std::convert::From<ISyncChangeBatchAdvanced> for ::windows::runtime::IUnknown {
    fn from(value: ISyncChangeBatchAdvanced) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISyncChangeBatchAdvanced> for ::windows::runtime::IUnknown {
    fn from(value: &ISyncChangeBatchAdvanced) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISyncChangeBatchAdvanced {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISyncChangeBatchAdvanced {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncChangeBatchAdvanced_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppfilterinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppchangebatch: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfbatchknowledgeshouldbeapplied: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISyncChangeBatchBase(pub ::windows::runtime::IUnknown);
impl ISyncChangeBatchBase {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetChangeEnumerator(&self) -> ::windows::runtime::Result<IEnumSyncChanges> {
        let mut result__: <IEnumSyncChanges as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumSyncChanges>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_Foundation`*"]
    pub unsafe fn GetIsLastBatch(&self, pflastbatch: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pflastbatch)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetWorkEstimateForBatch(&self, pdwworkforbatch: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwworkforbatch)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetRemainingWorkEstimateForSession(&self, pdwremainingworkforsession: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwremainingworkforsession)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn BeginOrderedGroup(&self, pblowerbound: *const u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(pblowerbound)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn EndOrderedGroup<'a, Param1: ::windows::runtime::IntoParam<'a, ISyncKnowledge>>(&self, pbupperbound: *const u8, pmadewithknowledge: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbupperbound), pmadewithknowledge.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn AddItemMetadataToGroup(&self, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32) -> ::windows::runtime::Result<ISyncChangeBuilder> {
        let mut result__: <ISyncChangeBuilder as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbownerreplicaid), ::std::mem::transmute(pbitemid), ::std::mem::transmute(pchangeversion), ::std::mem::transmute(pcreationversion), ::std::mem::transmute(dwflags), ::std::mem::transmute(dwworkforchange), &mut result__).from_abi::<ISyncChangeBuilder>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetLearnedKnowledge(&self) -> ::windows::runtime::Result<ISyncKnowledge> {
        let mut result__: <ISyncKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ISyncKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetPrerequisiteKnowledge(&self) -> ::windows::runtime::Result<ISyncKnowledge> {
        let mut result__: <ISyncKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ISyncKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetSourceForgottenKnowledge(&self) -> ::windows::runtime::Result<IForgottenKnowledge> {
        let mut result__: <IForgottenKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IForgottenKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn SetLastBatch(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn SetWorkEstimateForBatch(&self, dwworkforbatch: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwworkforbatch)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn SetRemainingWorkEstimateForSession(&self, dwremainingworkforsession: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwremainingworkforsession)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Serialize(&self, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbchangebatch), ::std::mem::transmute(pcbchangebatch)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISyncChangeBatchBase {
    type Vtable = ISyncChangeBatchBase_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1391912596, 27249, 17556, [161, 132, 168, 49, 27, 245, 210, 39]);
}
impl ::std::convert::From<ISyncChangeBatchBase> for ::windows::runtime::IUnknown {
    fn from(value: ISyncChangeBatchBase) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISyncChangeBatchBase> for ::windows::runtime::IUnknown {
    fn from(value: &ISyncChangeBatchBase) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISyncChangeBatchBase {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISyncChangeBatchBase {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncChangeBatchBase_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pflastbatch: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwworkforbatch: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwremainingworkforsession: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pblowerbound: *const u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbupperbound: *const u8, pmadewithknowledge: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32, ppchangebuilder: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pplearnedknowledge: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppprerequisteknowledge: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppsourceforgottenknowledge: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwworkforbatch: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwremainingworkforsession: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISyncChangeBatchBase2(pub ::windows::runtime::IUnknown);
impl ISyncChangeBatchBase2 {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetChangeEnumerator(&self) -> ::windows::runtime::Result<IEnumSyncChanges> {
        let mut result__: <IEnumSyncChanges as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumSyncChanges>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_Foundation`*"]
    pub unsafe fn GetIsLastBatch(&self, pflastbatch: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pflastbatch)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetWorkEstimateForBatch(&self, pdwworkforbatch: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwworkforbatch)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetRemainingWorkEstimateForSession(&self, pdwremainingworkforsession: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwremainingworkforsession)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn BeginOrderedGroup(&self, pblowerbound: *const u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(pblowerbound)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn EndOrderedGroup<'a, Param1: ::windows::runtime::IntoParam<'a, ISyncKnowledge>>(&self, pbupperbound: *const u8, pmadewithknowledge: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbupperbound), pmadewithknowledge.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn AddItemMetadataToGroup(&self, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32) -> ::windows::runtime::Result<ISyncChangeBuilder> {
        let mut result__: <ISyncChangeBuilder as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbownerreplicaid), ::std::mem::transmute(pbitemid), ::std::mem::transmute(pchangeversion), ::std::mem::transmute(pcreationversion), ::std::mem::transmute(dwflags), ::std::mem::transmute(dwworkforchange), &mut result__).from_abi::<ISyncChangeBuilder>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetLearnedKnowledge(&self) -> ::windows::runtime::Result<ISyncKnowledge> {
        let mut result__: <ISyncKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ISyncKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetPrerequisiteKnowledge(&self) -> ::windows::runtime::Result<ISyncKnowledge> {
        let mut result__: <ISyncKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ISyncKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetSourceForgottenKnowledge(&self) -> ::windows::runtime::Result<IForgottenKnowledge> {
        let mut result__: <IForgottenKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IForgottenKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn SetLastBatch(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn SetWorkEstimateForBatch(&self, dwworkforbatch: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwworkforbatch)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn SetRemainingWorkEstimateForSession(&self, dwremainingworkforsession: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwremainingworkforsession)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Serialize(&self, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbchangebatch), ::std::mem::transmute(pcbchangebatch)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn SerializeWithOptions(&self, targetformatversion: SYNC_SERIALIZATION_VERSION, dwflags: u32, pbbuffer: *mut u8, pdwserializedsize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(targetformatversion), ::std::mem::transmute(dwflags), ::std::mem::transmute(pbbuffer), ::std::mem::transmute(pdwserializedsize)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISyncChangeBatchBase2 {
    type Vtable = ISyncChangeBatchBase2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1876646250, 55125, 17796, [189, 12, 192, 194, 58, 84, 143, 191]);
}
impl ::std::convert::From<ISyncChangeBatchBase2> for ::windows::runtime::IUnknown {
    fn from(value: ISyncChangeBatchBase2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISyncChangeBatchBase2> for ::windows::runtime::IUnknown {
    fn from(value: &ISyncChangeBatchBase2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISyncChangeBatchBase2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISyncChangeBatchBase2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<ISyncChangeBatchBase2> for ISyncChangeBatchBase {
    fn from(value: ISyncChangeBatchBase2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISyncChangeBatchBase2> for ISyncChangeBatchBase {
    fn from(value: &ISyncChangeBatchBase2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyncChangeBatchBase> for ISyncChangeBatchBase2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyncChangeBatchBase> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISyncChangeBatchBase>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyncChangeBatchBase> for &ISyncChangeBatchBase2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyncChangeBatchBase> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISyncChangeBatchBase>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncChangeBatchBase2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pflastbatch: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwworkforbatch: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwremainingworkforsession: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pblowerbound: *const u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbupperbound: *const u8, pmadewithknowledge: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32, ppchangebuilder: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pplearnedknowledge: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppprerequisteknowledge: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppsourceforgottenknowledge: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwworkforbatch: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwremainingworkforsession: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, targetformatversion: SYNC_SERIALIZATION_VERSION, dwflags: u32, pbbuffer: *mut u8, pdwserializedsize: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISyncChangeBatchWithFilterKeyMap(pub ::windows::runtime::IUnknown);
impl ISyncChangeBatchWithFilterKeyMap {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetFilterKeyMap(&self) -> ::windows::runtime::Result<IFilterKeyMap> {
        let mut result__: <IFilterKeyMap as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IFilterKeyMap>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn SetFilterKeyMap<'a, Param0: ::windows::runtime::IntoParam<'a, IFilterKeyMap>>(&self, pifilterkeymap: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pifilterkeymap.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn SetFilterForgottenKnowledge<'a, Param1: ::windows::runtime::IntoParam<'a, ISyncKnowledge>>(&self, dwfilterkey: u32, pfilterforgottenknowledge: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwfilterkey), pfilterforgottenknowledge.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetFilteredReplicaLearnedKnowledge<'a, Param0: ::windows::runtime::IntoParam<'a, ISyncKnowledge>, Param1: ::windows::runtime::IntoParam<'a, IEnumItemIds>>(&self, pdestinationknowledge: Param0, pnewmoveins: Param1) -> ::windows::runtime::Result<ISyncKnowledge> {
        let mut result__: <ISyncKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pdestinationknowledge.into_param().abi(), pnewmoveins.into_param().abi(), &mut result__).from_abi::<ISyncKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetLearnedFilterForgottenKnowledge<'a, Param0: ::windows::runtime::IntoParam<'a, ISyncKnowledge>, Param1: ::windows::runtime::IntoParam<'a, IEnumItemIds>>(&self, pdestinationknowledge: Param0, pnewmoveins: Param1, dwfilterkey: u32) -> ::windows::runtime::Result<ISyncKnowledge> {
        let mut result__: <ISyncKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), pdestinationknowledge.into_param().abi(), pnewmoveins.into_param().abi(), ::std::mem::transmute(dwfilterkey), &mut result__).from_abi::<ISyncKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetFilteredReplicaLearnedForgottenKnowledge<'a, Param0: ::windows::runtime::IntoParam<'a, ISyncKnowledge>, Param1: ::windows::runtime::IntoParam<'a, IEnumItemIds>>(&self, pdestinationknowledge: Param0, pnewmoveins: Param1) -> ::windows::runtime::Result<ISyncKnowledge> {
        let mut result__: <ISyncKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), pdestinationknowledge.into_param().abi(), pnewmoveins.into_param().abi(), &mut result__).from_abi::<ISyncKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete<'a, Param0: ::windows::runtime::IntoParam<'a, ISyncKnowledge>, Param1: ::windows::runtime::IntoParam<'a, IEnumItemIds>>(&self, pdestinationknowledge: Param0, pnewmoveins: Param1) -> ::windows::runtime::Result<ISyncKnowledge> {
        let mut result__: <ISyncKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), pdestinationknowledge.into_param().abi(), pnewmoveins.into_param().abi(), &mut result__).from_abi::<ISyncKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete<'a, Param0: ::windows::runtime::IntoParam<'a, ISyncKnowledge>, Param1: ::windows::runtime::IntoParam<'a, IEnumItemIds>>(&self, pdestinationknowledge: Param0, pnewmoveins: Param1, dwfilterkey: u32) -> ::windows::runtime::Result<ISyncKnowledge> {
        let mut result__: <ISyncKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), pdestinationknowledge.into_param().abi(), pnewmoveins.into_param().abi(), ::std::mem::transmute(dwfilterkey), &mut result__).from_abi::<ISyncKnowledge>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISyncChangeBatchWithFilterKeyMap {
    type Vtable = ISyncChangeBatchWithFilterKeyMap_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3726929922, 22125, 17818, [166, 237, 165, 170, 179, 69, 159, 183]);
}
impl ::std::convert::From<ISyncChangeBatchWithFilterKeyMap> for ::windows::runtime::IUnknown {
    fn from(value: ISyncChangeBatchWithFilterKeyMap) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISyncChangeBatchWithFilterKeyMap> for ::windows::runtime::IUnknown {
    fn from(value: &ISyncChangeBatchWithFilterKeyMap) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISyncChangeBatchWithFilterKeyMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISyncChangeBatchWithFilterKeyMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncChangeBatchWithFilterKeyMap_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppifilterkeymap: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pifilterkeymap: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwfilterkey: u32, pfilterforgottenknowledge: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdestinationknowledge: ::windows::runtime::RawPtr, pnewmoveins: ::windows::runtime::RawPtr, pplearnedforgottenknowledge: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdestinationknowledge: ::windows::runtime::RawPtr, pnewmoveins: ::windows::runtime::RawPtr, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdestinationknowledge: ::windows::runtime::RawPtr, pnewmoveins: ::windows::runtime::RawPtr, pplearnedforgottenknowledge: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdestinationknowledge: ::windows::runtime::RawPtr, pnewmoveins: ::windows::runtime::RawPtr, pplearnedforgottenknowledge: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdestinationknowledge: ::windows::runtime::RawPtr, pnewmoveins: ::windows::runtime::RawPtr, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISyncChangeBatchWithPrerequisite(pub ::windows::runtime::IUnknown);
impl ISyncChangeBatchWithPrerequisite {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetChangeEnumerator(&self) -> ::windows::runtime::Result<IEnumSyncChanges> {
        let mut result__: <IEnumSyncChanges as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumSyncChanges>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_Foundation`*"]
    pub unsafe fn GetIsLastBatch(&self, pflastbatch: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pflastbatch)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetWorkEstimateForBatch(&self, pdwworkforbatch: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwworkforbatch)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetRemainingWorkEstimateForSession(&self, pdwremainingworkforsession: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwremainingworkforsession)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn BeginOrderedGroup(&self, pblowerbound: *const u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(pblowerbound)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn EndOrderedGroup<'a, Param1: ::windows::runtime::IntoParam<'a, ISyncKnowledge>>(&self, pbupperbound: *const u8, pmadewithknowledge: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbupperbound), pmadewithknowledge.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn AddItemMetadataToGroup(&self, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32) -> ::windows::runtime::Result<ISyncChangeBuilder> {
        let mut result__: <ISyncChangeBuilder as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbownerreplicaid), ::std::mem::transmute(pbitemid), ::std::mem::transmute(pchangeversion), ::std::mem::transmute(pcreationversion), ::std::mem::transmute(dwflags), ::std::mem::transmute(dwworkforchange), &mut result__).from_abi::<ISyncChangeBuilder>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetLearnedKnowledge(&self) -> ::windows::runtime::Result<ISyncKnowledge> {
        let mut result__: <ISyncKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ISyncKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetPrerequisiteKnowledge(&self) -> ::windows::runtime::Result<ISyncKnowledge> {
        let mut result__: <ISyncKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ISyncKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetSourceForgottenKnowledge(&self) -> ::windows::runtime::Result<IForgottenKnowledge> {
        let mut result__: <IForgottenKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IForgottenKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn SetLastBatch(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn SetWorkEstimateForBatch(&self, dwworkforbatch: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwworkforbatch)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn SetRemainingWorkEstimateForSession(&self, dwremainingworkforsession: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwremainingworkforsession)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Serialize(&self, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbchangebatch), ::std::mem::transmute(pcbchangebatch)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn SetPrerequisiteKnowledge<'a, Param0: ::windows::runtime::IntoParam<'a, ISyncKnowledge>>(&self, pprerequisiteknowledge: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), pprerequisiteknowledge.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetLearnedKnowledgeWithPrerequisite<'a, Param0: ::windows::runtime::IntoParam<'a, ISyncKnowledge>>(&self, pdestinationknowledge: Param0) -> ::windows::runtime::Result<ISyncKnowledge> {
        let mut result__: <ISyncKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), pdestinationknowledge.into_param().abi(), &mut result__).from_abi::<ISyncKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetLearnedForgottenKnowledge(&self) -> ::windows::runtime::Result<IForgottenKnowledge> {
        let mut result__: <IForgottenKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IForgottenKnowledge>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISyncChangeBatchWithPrerequisite {
    type Vtable = ISyncChangeBatchWithPrerequisite_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(159323070, 23442, 16456, [179, 242, 123, 66, 162, 81, 94, 7]);
}
impl ::std::convert::From<ISyncChangeBatchWithPrerequisite> for ::windows::runtime::IUnknown {
    fn from(value: ISyncChangeBatchWithPrerequisite) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISyncChangeBatchWithPrerequisite> for ::windows::runtime::IUnknown {
    fn from(value: &ISyncChangeBatchWithPrerequisite) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISyncChangeBatchWithPrerequisite {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISyncChangeBatchWithPrerequisite {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<ISyncChangeBatchWithPrerequisite> for ISyncChangeBatchBase {
    fn from(value: ISyncChangeBatchWithPrerequisite) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISyncChangeBatchWithPrerequisite> for ISyncChangeBatchBase {
    fn from(value: &ISyncChangeBatchWithPrerequisite) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyncChangeBatchBase> for ISyncChangeBatchWithPrerequisite {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyncChangeBatchBase> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISyncChangeBatchBase>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyncChangeBatchBase> for &ISyncChangeBatchWithPrerequisite {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyncChangeBatchBase> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISyncChangeBatchBase>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncChangeBatchWithPrerequisite_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pflastbatch: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwworkforbatch: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwremainingworkforsession: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pblowerbound: *const u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbupperbound: *const u8, pmadewithknowledge: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32, ppchangebuilder: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pplearnedknowledge: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppprerequisteknowledge: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppsourceforgottenknowledge: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwworkforbatch: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwremainingworkforsession: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprerequisiteknowledge: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdestinationknowledge: ::windows::runtime::RawPtr, pplearnedwithprerequisiteknowledge: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pplearnedforgottenknowledge: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISyncChangeBuilder(pub ::windows::runtime::IUnknown);
impl ISyncChangeBuilder {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn AddChangeUnitMetadata(&self, pbchangeunitid: *const u8, pchangeunitversion: *const SYNC_VERSION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbchangeunitid), ::std::mem::transmute(pchangeunitversion)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISyncChangeBuilder {
    type Vtable = ISyncChangeBuilder_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1458653041, 34423, 18511, [161, 112, 227, 134, 228, 24, 166, 118]);
}
impl ::std::convert::From<ISyncChangeBuilder> for ::windows::runtime::IUnknown {
    fn from(value: ISyncChangeBuilder) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISyncChangeBuilder> for ::windows::runtime::IUnknown {
    fn from(value: &ISyncChangeBuilder) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISyncChangeBuilder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISyncChangeBuilder {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncChangeBuilder_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbchangeunitid: *const u8, pchangeunitversion: *const SYNC_VERSION) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISyncChangeUnit(pub ::windows::runtime::IUnknown);
impl ISyncChangeUnit {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetItemChange(&self) -> ::windows::runtime::Result<ISyncChange> {
        let mut result__: <ISyncChange as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ISyncChange>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetChangeUnitId(&self, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbchangeunitid), ::std::mem::transmute(pcbidsize)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetChangeUnitVersion(&self, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbcurrentreplicaid), ::std::mem::transmute(pversion)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISyncChangeUnit {
    type Vtable = ISyncChangeUnit_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1626200266, 29505, 19383, [149, 206, 250, 182, 57, 75, 81, 203]);
}
impl ::std::convert::From<ISyncChangeUnit> for ::windows::runtime::IUnknown {
    fn from(value: ISyncChangeUnit) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISyncChangeUnit> for ::windows::runtime::IUnknown {
    fn from(value: &ISyncChangeUnit) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISyncChangeUnit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISyncChangeUnit {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncChangeUnit_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppsyncchange: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISyncChangeWithFilterKeyMap(pub ::windows::runtime::IUnknown);
impl ISyncChangeWithFilterKeyMap {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetFilterCount(&self, pdwfiltercount: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwfiltercount)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_Foundation`*"]
    pub unsafe fn GetFilterChange(&self, dwfilterkey: u32, pfilterchange: *mut SYNC_FILTER_CHANGE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwfilterkey), ::std::mem::transmute(pfilterchange)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_Foundation`*"]
    pub unsafe fn GetAllChangeUnitsPresentFlag(&self, pfallchangeunitspresent: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pfallchangeunitspresent)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetFilterForgottenKnowledge(&self, dwfilterkey: u32) -> ::windows::runtime::Result<ISyncKnowledge> {
        let mut result__: <ISyncKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwfilterkey), &mut result__).from_abi::<ISyncKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetFilteredReplicaLearnedKnowledge<'a, Param0: ::windows::runtime::IntoParam<'a, ISyncKnowledge>, Param1: ::windows::runtime::IntoParam<'a, IEnumItemIds>>(&self, pdestinationknowledge: Param0, pnewmoveins: Param1) -> ::windows::runtime::Result<ISyncKnowledge> {
        let mut result__: <ISyncKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), pdestinationknowledge.into_param().abi(), pnewmoveins.into_param().abi(), &mut result__).from_abi::<ISyncKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetLearnedFilterForgottenKnowledge<'a, Param0: ::windows::runtime::IntoParam<'a, ISyncKnowledge>, Param1: ::windows::runtime::IntoParam<'a, IEnumItemIds>>(&self, pdestinationknowledge: Param0, pnewmoveins: Param1, dwfilterkey: u32) -> ::windows::runtime::Result<ISyncKnowledge> {
        let mut result__: <ISyncKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), pdestinationknowledge.into_param().abi(), pnewmoveins.into_param().abi(), ::std::mem::transmute(dwfilterkey), &mut result__).from_abi::<ISyncKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetFilteredReplicaLearnedForgottenKnowledge<'a, Param0: ::windows::runtime::IntoParam<'a, ISyncKnowledge>, Param1: ::windows::runtime::IntoParam<'a, IEnumItemIds>>(&self, pdestinationknowledge: Param0, pnewmoveins: Param1) -> ::windows::runtime::Result<ISyncKnowledge> {
        let mut result__: <ISyncKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), pdestinationknowledge.into_param().abi(), pnewmoveins.into_param().abi(), &mut result__).from_abi::<ISyncKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete<'a, Param0: ::windows::runtime::IntoParam<'a, ISyncKnowledge>, Param1: ::windows::runtime::IntoParam<'a, IEnumItemIds>>(&self, pdestinationknowledge: Param0, pnewmoveins: Param1) -> ::windows::runtime::Result<ISyncKnowledge> {
        let mut result__: <ISyncKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), pdestinationknowledge.into_param().abi(), pnewmoveins.into_param().abi(), &mut result__).from_abi::<ISyncKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete<'a, Param0: ::windows::runtime::IntoParam<'a, ISyncKnowledge>, Param1: ::windows::runtime::IntoParam<'a, IEnumItemIds>>(&self, pdestinationknowledge: Param0, pnewmoveins: Param1, dwfilterkey: u32) -> ::windows::runtime::Result<ISyncKnowledge> {
        let mut result__: <ISyncKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), pdestinationknowledge.into_param().abi(), pnewmoveins.into_param().abi(), ::std::mem::transmute(dwfilterkey), &mut result__).from_abi::<ISyncKnowledge>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISyncChangeWithFilterKeyMap {
    type Vtable = ISyncChangeWithFilterKeyMap_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3219255040, 59517, 17149, [164, 233, 36, 45, 112, 65, 74, 239]);
}
impl ::std::convert::From<ISyncChangeWithFilterKeyMap> for ::windows::runtime::IUnknown {
    fn from(value: ISyncChangeWithFilterKeyMap) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISyncChangeWithFilterKeyMap> for ::windows::runtime::IUnknown {
    fn from(value: &ISyncChangeWithFilterKeyMap) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISyncChangeWithFilterKeyMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISyncChangeWithFilterKeyMap {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncChangeWithFilterKeyMap_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwfiltercount: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwfilterkey: u32, pfilterchange: *mut SYNC_FILTER_CHANGE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfallchangeunitspresent: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwfilterkey: u32, ppifilterforgottenknowledge: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdestinationknowledge: ::windows::runtime::RawPtr, pnewmoveins: ::windows::runtime::RawPtr, pplearnedknowledge: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdestinationknowledge: ::windows::runtime::RawPtr, pnewmoveins: ::windows::runtime::RawPtr, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdestinationknowledge: ::windows::runtime::RawPtr, pnewmoveins: ::windows::runtime::RawPtr, pplearnedforgottenknowledge: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdestinationknowledge: ::windows::runtime::RawPtr, pnewmoveins: ::windows::runtime::RawPtr, pplearnedforgottenknowledge: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdestinationknowledge: ::windows::runtime::RawPtr, pnewmoveins: ::windows::runtime::RawPtr, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISyncChangeWithPrerequisite(pub ::windows::runtime::IUnknown);
impl ISyncChangeWithPrerequisite {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetPrerequisiteKnowledge(&self) -> ::windows::runtime::Result<ISyncKnowledge> {
        let mut result__: <ISyncKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ISyncKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetLearnedKnowledgeWithPrerequisite<'a, Param0: ::windows::runtime::IntoParam<'a, ISyncKnowledge>>(&self, pdestinationknowledge: Param0) -> ::windows::runtime::Result<ISyncKnowledge> {
        let mut result__: <ISyncKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), pdestinationknowledge.into_param().abi(), &mut result__).from_abi::<ISyncKnowledge>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISyncChangeWithPrerequisite {
    type Vtable = ISyncChangeWithPrerequisite_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2654484527, 5513, 18627, [146, 228, 5, 236, 220, 180, 243, 247]);
}
impl ::std::convert::From<ISyncChangeWithPrerequisite> for ::windows::runtime::IUnknown {
    fn from(value: ISyncChangeWithPrerequisite) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISyncChangeWithPrerequisite> for ::windows::runtime::IUnknown {
    fn from(value: &ISyncChangeWithPrerequisite) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISyncChangeWithPrerequisite {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISyncChangeWithPrerequisite {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncChangeWithPrerequisite_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppprerequisiteknowledge: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdestinationknowledge: ::windows::runtime::RawPtr, pplearnedknowledgewithprerequisite: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISyncConstraintCallback(pub ::windows::runtime::IUnknown);
impl ISyncConstraintCallback {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn OnConstraintConflict<'a, Param0: ::windows::runtime::IntoParam<'a, IConstraintConflict>>(&self, pconflict: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), pconflict.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISyncConstraintCallback {
    type Vtable = ISyncConstraintCallback_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2331214910, 30131, 17292, [187, 81, 111, 2, 13, 112, 211, 203]);
}
impl ::std::convert::From<ISyncConstraintCallback> for ::windows::runtime::IUnknown {
    fn from(value: ISyncConstraintCallback) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISyncConstraintCallback> for ::windows::runtime::IUnknown {
    fn from(value: &ISyncConstraintCallback) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISyncConstraintCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISyncConstraintCallback {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncConstraintCallback_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pconflict: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISyncDataConverter(pub ::windows::runtime::IUnknown);
impl ISyncDataConverter {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn ConvertDataRetrieverFromProviderFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param1: ::windows::runtime::IntoParam<'a, IEnumSyncChanges>>(&self, punkdataretrieverin: Param0, penumsyncchanges: Param1) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), punkdataretrieverin.into_param().abi(), penumsyncchanges.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn ConvertDataRetrieverToProviderFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param1: ::windows::runtime::IntoParam<'a, IEnumSyncChanges>>(&self, punkdataretrieverin: Param0, penumsyncchanges: Param1) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), punkdataretrieverin.into_param().abi(), penumsyncchanges.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn ConvertDataFromProviderFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ILoadChangeContext>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, pdatacontext: Param0, punkdatain: Param1) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), pdatacontext.into_param().abi(), punkdatain.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn ConvertDataToProviderFormat<'a, Param0: ::windows::runtime::IntoParam<'a, ILoadChangeContext>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, pdatacontext: Param0, punkdataout: Param1) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), pdatacontext.into_param().abi(), punkdataout.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISyncDataConverter {
    type Vtable = ISyncDataConverter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1130186849, 26837, 17578, [160, 249, 114, 160, 176, 14, 249, 207]);
}
impl ::std::convert::From<ISyncDataConverter> for ::windows::runtime::IUnknown {
    fn from(value: ISyncDataConverter) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISyncDataConverter> for ::windows::runtime::IUnknown {
    fn from(value: &ISyncDataConverter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISyncDataConverter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISyncDataConverter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncDataConverter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punkdataretrieverin: ::windows::runtime::RawPtr, penumsyncchanges: ::windows::runtime::RawPtr, ppunkdataout: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, punkdataretrieverin: ::windows::runtime::RawPtr, penumsyncchanges: ::windows::runtime::RawPtr, ppunkdataout: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdatacontext: ::windows::runtime::RawPtr, punkdatain: ::windows::runtime::RawPtr, ppunkdataout: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdatacontext: ::windows::runtime::RawPtr, punkdataout: ::windows::runtime::RawPtr, ppunkdataout: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISyncFilter(pub ::windows::runtime::IUnknown);
impl ISyncFilter {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn IsIdentical<'a, Param0: ::windows::runtime::IntoParam<'a, ISyncFilter>>(&self, psyncfilter: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), psyncfilter.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Serialize(&self, pbsyncfilter: *mut u8, pcbsyncfilter: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbsyncfilter), ::std::mem::transmute(pcbsyncfilter)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISyncFilter {
    type Vtable = ISyncFilter_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(142229269, 4043, 17601, [150, 57, 83, 193, 78, 43, 85, 6]);
}
impl ::std::convert::From<ISyncFilter> for ::windows::runtime::IUnknown {
    fn from(value: ISyncFilter) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISyncFilter> for ::windows::runtime::IUnknown {
    fn from(value: &ISyncFilter) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISyncFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISyncFilter {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncFilter_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psyncfilter: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbsyncfilter: *mut u8, pcbsyncfilter: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISyncFilterDeserializer(pub ::windows::runtime::IUnknown);
impl ISyncFilterDeserializer {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn DeserializeSyncFilter(&self, pbsyncfilter: *const u8, dwcbsyncfilter: u32) -> ::windows::runtime::Result<ISyncFilter> {
        let mut result__: <ISyncFilter as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbsyncfilter), ::std::mem::transmute(dwcbsyncfilter), &mut result__).from_abi::<ISyncFilter>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISyncFilterDeserializer {
    type Vtable = ISyncFilterDeserializer_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3025894002, 58823, 18110, [156, 130, 119, 184, 177, 93, 171, 138]);
}
impl ::std::convert::From<ISyncFilterDeserializer> for ::windows::runtime::IUnknown {
    fn from(value: ISyncFilterDeserializer) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISyncFilterDeserializer> for ::windows::runtime::IUnknown {
    fn from(value: &ISyncFilterDeserializer) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISyncFilterDeserializer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISyncFilterDeserializer {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncFilterDeserializer_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbsyncfilter: *const u8, dwcbsyncfilter: u32, ppisyncfilter: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISyncFilterInfo(pub ::windows::runtime::IUnknown);
impl ISyncFilterInfo {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Serialize(&self, pbbuffer: *mut u8, pcbbuffer: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbbuffer), ::std::mem::transmute(pcbbuffer)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISyncFilterInfo {
    type Vtable = ISyncFilterInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2035198712, 16174, 18406, [151, 40, 23, 230, 252, 249, 76, 183]);
}
impl ::std::convert::From<ISyncFilterInfo> for ::windows::runtime::IUnknown {
    fn from(value: ISyncFilterInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISyncFilterInfo> for ::windows::runtime::IUnknown {
    fn from(value: &ISyncFilterInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISyncFilterInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISyncFilterInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncFilterInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbbuffer: *mut u8, pcbbuffer: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISyncFilterInfo2(pub ::windows::runtime::IUnknown);
impl ISyncFilterInfo2 {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Serialize(&self, pbbuffer: *mut u8, pcbbuffer: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbbuffer), ::std::mem::transmute(pcbbuffer)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetFlags(&self, pdwflags: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwflags)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISyncFilterInfo2 {
    type Vtable = ISyncFilterInfo2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(431199418, 58320, 18060, [147, 77, 50, 25, 104, 178, 171, 52]);
}
impl ::std::convert::From<ISyncFilterInfo2> for ::windows::runtime::IUnknown {
    fn from(value: ISyncFilterInfo2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISyncFilterInfo2> for ::windows::runtime::IUnknown {
    fn from(value: &ISyncFilterInfo2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISyncFilterInfo2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISyncFilterInfo2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<ISyncFilterInfo2> for ISyncFilterInfo {
    fn from(value: ISyncFilterInfo2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISyncFilterInfo2> for ISyncFilterInfo {
    fn from(value: &ISyncFilterInfo2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyncFilterInfo> for ISyncFilterInfo2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyncFilterInfo> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISyncFilterInfo>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyncFilterInfo> for &ISyncFilterInfo2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyncFilterInfo> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISyncFilterInfo>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncFilterInfo2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbbuffer: *mut u8, pcbbuffer: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwflags: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISyncFullEnumerationChange(pub ::windows::runtime::IUnknown);
impl ISyncFullEnumerationChange {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetLearnedKnowledgeAfterRecoveryComplete(&self) -> ::windows::runtime::Result<ISyncKnowledge> {
        let mut result__: <ISyncKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ISyncKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetLearnedForgottenKnowledge(&self) -> ::windows::runtime::Result<IForgottenKnowledge> {
        let mut result__: <IForgottenKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IForgottenKnowledge>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISyncFullEnumerationChange {
    type Vtable = ISyncFullEnumerationChange_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2542133437, 48639, 16580, [152, 197, 179, 75, 47, 25, 145, 179]);
}
impl ::std::convert::From<ISyncFullEnumerationChange> for ::windows::runtime::IUnknown {
    fn from(value: ISyncFullEnumerationChange) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISyncFullEnumerationChange> for ::windows::runtime::IUnknown {
    fn from(value: &ISyncFullEnumerationChange) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISyncFullEnumerationChange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISyncFullEnumerationChange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncFullEnumerationChange_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pplearnedknowledge: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pplearnedforgottenknowledge: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISyncFullEnumerationChangeBatch(pub ::windows::runtime::IUnknown);
impl ISyncFullEnumerationChangeBatch {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetChangeEnumerator(&self) -> ::windows::runtime::Result<IEnumSyncChanges> {
        let mut result__: <IEnumSyncChanges as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumSyncChanges>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_Foundation`*"]
    pub unsafe fn GetIsLastBatch(&self, pflastbatch: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pflastbatch)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetWorkEstimateForBatch(&self, pdwworkforbatch: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwworkforbatch)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetRemainingWorkEstimateForSession(&self, pdwremainingworkforsession: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwremainingworkforsession)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn BeginOrderedGroup(&self, pblowerbound: *const u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(pblowerbound)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn EndOrderedGroup<'a, Param1: ::windows::runtime::IntoParam<'a, ISyncKnowledge>>(&self, pbupperbound: *const u8, pmadewithknowledge: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbupperbound), pmadewithknowledge.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn AddItemMetadataToGroup(&self, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32) -> ::windows::runtime::Result<ISyncChangeBuilder> {
        let mut result__: <ISyncChangeBuilder as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbownerreplicaid), ::std::mem::transmute(pbitemid), ::std::mem::transmute(pchangeversion), ::std::mem::transmute(pcreationversion), ::std::mem::transmute(dwflags), ::std::mem::transmute(dwworkforchange), &mut result__).from_abi::<ISyncChangeBuilder>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetLearnedKnowledge(&self) -> ::windows::runtime::Result<ISyncKnowledge> {
        let mut result__: <ISyncKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ISyncKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetPrerequisiteKnowledge(&self) -> ::windows::runtime::Result<ISyncKnowledge> {
        let mut result__: <ISyncKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ISyncKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetSourceForgottenKnowledge(&self) -> ::windows::runtime::Result<IForgottenKnowledge> {
        let mut result__: <IForgottenKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IForgottenKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn SetLastBatch(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn SetWorkEstimateForBatch(&self, dwworkforbatch: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwworkforbatch)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn SetRemainingWorkEstimateForSession(&self, dwremainingworkforsession: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwremainingworkforsession)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Serialize(&self, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbchangebatch), ::std::mem::transmute(pcbchangebatch)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetLearnedKnowledgeAfterRecoveryComplete(&self) -> ::windows::runtime::Result<ISyncKnowledge> {
        let mut result__: <ISyncKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ISyncKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetClosedLowerBoundItemId(&self, pbclosedlowerbounditemid: *mut u8, pcbidsize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbclosedlowerbounditemid), ::std::mem::transmute(pcbidsize)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetClosedUpperBoundItemId(&self, pbclosedupperbounditemid: *mut u8, pcbidsize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbclosedupperbounditemid), ::std::mem::transmute(pcbidsize)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISyncFullEnumerationChangeBatch {
    type Vtable = ISyncFullEnumerationChangeBatch_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4016314749, 20292, 20130, [179, 85, 69, 36, 113, 62, 59, 237]);
}
impl ::std::convert::From<ISyncFullEnumerationChangeBatch> for ::windows::runtime::IUnknown {
    fn from(value: ISyncFullEnumerationChangeBatch) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISyncFullEnumerationChangeBatch> for ::windows::runtime::IUnknown {
    fn from(value: &ISyncFullEnumerationChangeBatch) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISyncFullEnumerationChangeBatch {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISyncFullEnumerationChangeBatch {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<ISyncFullEnumerationChangeBatch> for ISyncChangeBatchBase {
    fn from(value: ISyncFullEnumerationChangeBatch) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISyncFullEnumerationChangeBatch> for ISyncChangeBatchBase {
    fn from(value: &ISyncFullEnumerationChangeBatch) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyncChangeBatchBase> for ISyncFullEnumerationChangeBatch {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyncChangeBatchBase> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISyncChangeBatchBase>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyncChangeBatchBase> for &ISyncFullEnumerationChangeBatch {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyncChangeBatchBase> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISyncChangeBatchBase>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncFullEnumerationChangeBatch_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pflastbatch: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwworkforbatch: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwremainingworkforsession: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pblowerbound: *const u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbupperbound: *const u8, pmadewithknowledge: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32, ppchangebuilder: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pplearnedknowledge: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppprerequisteknowledge: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppsourceforgottenknowledge: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwworkforbatch: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwremainingworkforsession: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pplearnedknowledgeafterrecoverycomplete: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbclosedlowerbounditemid: *mut u8, pcbidsize: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbclosedupperbounditemid: *mut u8, pcbidsize: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISyncFullEnumerationChangeBatch2(pub ::windows::runtime::IUnknown);
impl ISyncFullEnumerationChangeBatch2 {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetChangeEnumerator(&self) -> ::windows::runtime::Result<IEnumSyncChanges> {
        let mut result__: <IEnumSyncChanges as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IEnumSyncChanges>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_Foundation`*"]
    pub unsafe fn GetIsLastBatch(&self, pflastbatch: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pflastbatch)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetWorkEstimateForBatch(&self, pdwworkforbatch: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwworkforbatch)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetRemainingWorkEstimateForSession(&self, pdwremainingworkforsession: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwremainingworkforsession)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn BeginOrderedGroup(&self, pblowerbound: *const u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(pblowerbound)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn EndOrderedGroup<'a, Param1: ::windows::runtime::IntoParam<'a, ISyncKnowledge>>(&self, pbupperbound: *const u8, pmadewithknowledge: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbupperbound), pmadewithknowledge.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn AddItemMetadataToGroup(&self, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32) -> ::windows::runtime::Result<ISyncChangeBuilder> {
        let mut result__: <ISyncChangeBuilder as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbownerreplicaid), ::std::mem::transmute(pbitemid), ::std::mem::transmute(pchangeversion), ::std::mem::transmute(pcreationversion), ::std::mem::transmute(dwflags), ::std::mem::transmute(dwworkforchange), &mut result__).from_abi::<ISyncChangeBuilder>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetLearnedKnowledge(&self) -> ::windows::runtime::Result<ISyncKnowledge> {
        let mut result__: <ISyncKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ISyncKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetPrerequisiteKnowledge(&self) -> ::windows::runtime::Result<ISyncKnowledge> {
        let mut result__: <ISyncKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ISyncKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetSourceForgottenKnowledge(&self) -> ::windows::runtime::Result<IForgottenKnowledge> {
        let mut result__: <IForgottenKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IForgottenKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn SetLastBatch(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn SetWorkEstimateForBatch(&self, dwworkforbatch: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwworkforbatch)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn SetRemainingWorkEstimateForSession(&self, dwremainingworkforsession: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwremainingworkforsession)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Serialize(&self, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbchangebatch), ::std::mem::transmute(pcbchangebatch)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetLearnedKnowledgeAfterRecoveryComplete(&self) -> ::windows::runtime::Result<ISyncKnowledge> {
        let mut result__: <ISyncKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ISyncKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetClosedLowerBoundItemId(&self, pbclosedlowerbounditemid: *mut u8, pcbidsize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbclosedlowerbounditemid), ::std::mem::transmute(pcbidsize)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetClosedUpperBoundItemId(&self, pbclosedupperbounditemid: *mut u8, pcbidsize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbclosedupperbounditemid), ::std::mem::transmute(pcbidsize)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn AddMergeTombstoneMetadataToGroup(&self, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32) -> ::windows::runtime::Result<ISyncChangeBuilder> {
        let mut result__: <ISyncChangeBuilder as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbownerreplicaid), ::std::mem::transmute(pbwinneritemid), ::std::mem::transmute(pbitemid), ::std::mem::transmute(pchangeversion), ::std::mem::transmute(pcreationversion), ::std::mem::transmute(dwworkforchange), &mut result__).from_abi::<ISyncChangeBuilder>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISyncFullEnumerationChangeBatch2 {
    type Vtable = ISyncFullEnumerationChangeBatch2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3764668916, 41477, 19301, [151, 36, 1, 178, 33, 1, 238, 193]);
}
impl ::std::convert::From<ISyncFullEnumerationChangeBatch2> for ::windows::runtime::IUnknown {
    fn from(value: ISyncFullEnumerationChangeBatch2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISyncFullEnumerationChangeBatch2> for ::windows::runtime::IUnknown {
    fn from(value: &ISyncFullEnumerationChangeBatch2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISyncFullEnumerationChangeBatch2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISyncFullEnumerationChangeBatch2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<ISyncFullEnumerationChangeBatch2> for ISyncFullEnumerationChangeBatch {
    fn from(value: ISyncFullEnumerationChangeBatch2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISyncFullEnumerationChangeBatch2> for ISyncFullEnumerationChangeBatch {
    fn from(value: &ISyncFullEnumerationChangeBatch2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyncFullEnumerationChangeBatch> for ISyncFullEnumerationChangeBatch2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyncFullEnumerationChangeBatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISyncFullEnumerationChangeBatch>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyncFullEnumerationChangeBatch> for &ISyncFullEnumerationChangeBatch2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyncFullEnumerationChangeBatch> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISyncFullEnumerationChangeBatch>::into(::std::clone::Clone::clone(self)))
    }
}
impl ::std::convert::From<ISyncFullEnumerationChangeBatch2> for ISyncChangeBatchBase {
    fn from(value: ISyncFullEnumerationChangeBatch2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISyncFullEnumerationChangeBatch2> for ISyncChangeBatchBase {
    fn from(value: &ISyncFullEnumerationChangeBatch2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyncChangeBatchBase> for ISyncFullEnumerationChangeBatch2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyncChangeBatchBase> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISyncChangeBatchBase>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyncChangeBatchBase> for &ISyncFullEnumerationChangeBatch2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyncChangeBatchBase> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISyncChangeBatchBase>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncFullEnumerationChangeBatch2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppenum: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pflastbatch: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwworkforbatch: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwremainingworkforsession: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pblowerbound: *const u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbupperbound: *const u8, pmadewithknowledge: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32, ppchangebuilder: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pplearnedknowledge: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppprerequisteknowledge: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppsourceforgottenknowledge: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwworkforbatch: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwremainingworkforsession: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pplearnedknowledgeafterrecoverycomplete: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbclosedlowerbounditemid: *mut u8, pcbidsize: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbclosedupperbounditemid: *mut u8, pcbidsize: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, ppchangebuilder: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISyncKnowledge(pub ::windows::runtime::IUnknown);
impl ISyncKnowledge {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetOwnerReplicaId(&self, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbreplicaid), ::std::mem::transmute(pcbidsize)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_Foundation`*"]
    pub unsafe fn Serialize<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fserializereplicakeymap: Param0, pbknowledge: *mut u8, pcbknowledge: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), fserializereplicakeymap.into_param().abi(), ::std::mem::transmute(pbknowledge), ::std::mem::transmute(pcbknowledge)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn SetLocalTickCount(&self, ulltickcount: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(ulltickcount)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn ContainsChange(&self, pbversionownerreplicaid: *const u8, pgiditemid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbversionownerreplicaid), ::std::mem::transmute(pgiditemid), ::std::mem::transmute(psyncversion)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn ContainsChangeUnit(&self, pbversionownerreplicaid: *const u8, pbitemid: *const u8, pbchangeunitid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbversionownerreplicaid), ::std::mem::transmute(pbitemid), ::std::mem::transmute(pbchangeunitid), ::std::mem::transmute(psyncversion)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetScopeVector(&self, riid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(ppunk)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetReplicaKeyMap(&self) -> ::windows::runtime::Result<IReplicaKeyMap> {
        let mut result__: <IReplicaKeyMap as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IReplicaKeyMap>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<ISyncKnowledge> {
        let mut result__: <ISyncKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ISyncKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn ConvertVersion<'a, Param0: ::windows::runtime::IntoParam<'a, ISyncKnowledge>>(&self, pknowledgein: Param0, pbcurrentownerid: *const u8, pversionin: *const SYNC_VERSION, pbnewownerid: *mut u8, pcbidsize: *mut u32, pversionout: *mut SYNC_VERSION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), pknowledgein.into_param().abi(), ::std::mem::transmute(pbcurrentownerid), ::std::mem::transmute(pversionin), ::std::mem::transmute(pbnewownerid), ::std::mem::transmute(pcbidsize), ::std::mem::transmute(pversionout)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn MapRemoteToLocal<'a, Param0: ::windows::runtime::IntoParam<'a, ISyncKnowledge>>(&self, premoteknowledge: Param0) -> ::windows::runtime::Result<ISyncKnowledge> {
        let mut result__: <ISyncKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), premoteknowledge.into_param().abi(), &mut result__).from_abi::<ISyncKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Union<'a, Param0: ::windows::runtime::IntoParam<'a, ISyncKnowledge>>(&self, pknowledge: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), pknowledge.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn ProjectOntoItem(&self, pbitemid: *const u8) -> ::windows::runtime::Result<ISyncKnowledge> {
        let mut result__: <ISyncKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbitemid), &mut result__).from_abi::<ISyncKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn ProjectOntoChangeUnit(&self, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows::runtime::Result<ISyncKnowledge> {
        let mut result__: <ISyncKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbitemid), ::std::mem::transmute(pbchangeunitid), &mut result__).from_abi::<ISyncKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn ProjectOntoRange(&self, psrngsyncrange: *const SYNC_RANGE) -> ::windows::runtime::Result<ISyncKnowledge> {
        let mut result__: <ISyncKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(psrngsyncrange), &mut result__).from_abi::<ISyncKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn ExcludeItem(&self, pbitemid: *const u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbitemid)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn ExcludeChangeUnit(&self, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbitemid), ::std::mem::transmute(pbchangeunitid)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn ContainsKnowledge<'a, Param0: ::windows::runtime::IntoParam<'a, ISyncKnowledge>>(&self, pknowledge: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), pknowledge.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn FindMinTickCountForReplica(&self, pbreplicaid: *const u8, pullreplicatickcount: *mut u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbreplicaid), ::std::mem::transmute(pullreplicatickcount)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetRangeExceptions(&self, riid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(ppunk)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetSingleItemExceptions(&self, riid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(ppunk)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetChangeUnitExceptions(&self, riid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(ppunk)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn FindClockVectorForItem(&self, pbitemid: *const u8, riid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbitemid), ::std::mem::transmute(riid), ::std::mem::transmute(ppunk)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn FindClockVectorForChangeUnit(&self, pbitemid: *const u8, pbchangeunitid: *const u8, riid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbitemid), ::std::mem::transmute(pbchangeunitid), ::std::mem::transmute(riid), ::std::mem::transmute(ppunk)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetVersion(&self, pdwversion: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwversion)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISyncKnowledge {
    type Vtable = ISyncKnowledge_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1633401683, 51525, 16899, [191, 75, 44, 182, 89, 25, 160, 170]);
}
impl ::std::convert::From<ISyncKnowledge> for ::windows::runtime::IUnknown {
    fn from(value: ISyncKnowledge) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISyncKnowledge> for ::windows::runtime::IUnknown {
    fn from(value: &ISyncKnowledge) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISyncKnowledge {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISyncKnowledge {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncKnowledge_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fserializereplicakeymap: super::super::Foundation::BOOL, pbknowledge: *mut u8, pcbknowledge: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulltickcount: u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbversionownerreplicaid: *const u8, pgiditemid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbversionownerreplicaid: *const u8, pbitemid: *const u8, pbchangeunitid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppreplicakeymap: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppclonedknowledge: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pknowledgein: ::windows::runtime::RawPtr, pbcurrentownerid: *const u8, pversionin: *const SYNC_VERSION, pbnewownerid: *mut u8, pcbidsize: *mut u32, pversionout: *mut SYNC_VERSION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, premoteknowledge: ::windows::runtime::RawPtr, ppmappedknowledge: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pknowledge: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbitemid: *const u8, ppknowledgeout: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbitemid: *const u8, pbchangeunitid: *const u8, ppknowledgeout: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psrngsyncrange: *const SYNC_RANGE, ppknowledgeout: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbitemid: *const u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pknowledge: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbreplicaid: *const u8, pullreplicatickcount: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbitemid: *const u8, riid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbitemid: *const u8, pbchangeunitid: *const u8, riid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwversion: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISyncKnowledge2(pub ::windows::runtime::IUnknown);
impl ISyncKnowledge2 {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetOwnerReplicaId(&self, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbreplicaid), ::std::mem::transmute(pcbidsize)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_Foundation`*"]
    pub unsafe fn Serialize<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fserializereplicakeymap: Param0, pbknowledge: *mut u8, pcbknowledge: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), fserializereplicakeymap.into_param().abi(), ::std::mem::transmute(pbknowledge), ::std::mem::transmute(pcbknowledge)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn SetLocalTickCount(&self, ulltickcount: u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(ulltickcount)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn ContainsChange(&self, pbversionownerreplicaid: *const u8, pgiditemid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbversionownerreplicaid), ::std::mem::transmute(pgiditemid), ::std::mem::transmute(psyncversion)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn ContainsChangeUnit(&self, pbversionownerreplicaid: *const u8, pbitemid: *const u8, pbchangeunitid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbversionownerreplicaid), ::std::mem::transmute(pbitemid), ::std::mem::transmute(pbchangeunitid), ::std::mem::transmute(psyncversion)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetScopeVector(&self, riid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(ppunk)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetReplicaKeyMap(&self) -> ::windows::runtime::Result<IReplicaKeyMap> {
        let mut result__: <IReplicaKeyMap as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), &mut result__).from_abi::<IReplicaKeyMap>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Clone(&self) -> ::windows::runtime::Result<ISyncKnowledge> {
        let mut result__: <ISyncKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ISyncKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn ConvertVersion<'a, Param0: ::windows::runtime::IntoParam<'a, ISyncKnowledge>>(&self, pknowledgein: Param0, pbcurrentownerid: *const u8, pversionin: *const SYNC_VERSION, pbnewownerid: *mut u8, pcbidsize: *mut u32, pversionout: *mut SYNC_VERSION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), pknowledgein.into_param().abi(), ::std::mem::transmute(pbcurrentownerid), ::std::mem::transmute(pversionin), ::std::mem::transmute(pbnewownerid), ::std::mem::transmute(pcbidsize), ::std::mem::transmute(pversionout)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn MapRemoteToLocal<'a, Param0: ::windows::runtime::IntoParam<'a, ISyncKnowledge>>(&self, premoteknowledge: Param0) -> ::windows::runtime::Result<ISyncKnowledge> {
        let mut result__: <ISyncKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), premoteknowledge.into_param().abi(), &mut result__).from_abi::<ISyncKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Union<'a, Param0: ::windows::runtime::IntoParam<'a, ISyncKnowledge>>(&self, pknowledge: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), pknowledge.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn ProjectOntoItem(&self, pbitemid: *const u8) -> ::windows::runtime::Result<ISyncKnowledge> {
        let mut result__: <ISyncKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbitemid), &mut result__).from_abi::<ISyncKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn ProjectOntoChangeUnit(&self, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows::runtime::Result<ISyncKnowledge> {
        let mut result__: <ISyncKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbitemid), ::std::mem::transmute(pbchangeunitid), &mut result__).from_abi::<ISyncKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn ProjectOntoRange(&self, psrngsyncrange: *const SYNC_RANGE) -> ::windows::runtime::Result<ISyncKnowledge> {
        let mut result__: <ISyncKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(psrngsyncrange), &mut result__).from_abi::<ISyncKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn ExcludeItem(&self, pbitemid: *const u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbitemid)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn ExcludeChangeUnit(&self, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbitemid), ::std::mem::transmute(pbchangeunitid)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn ContainsKnowledge<'a, Param0: ::windows::runtime::IntoParam<'a, ISyncKnowledge>>(&self, pknowledge: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::std::mem::transmute_copy(self), pknowledge.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn FindMinTickCountForReplica(&self, pbreplicaid: *const u8, pullreplicatickcount: *mut u64) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbreplicaid), ::std::mem::transmute(pullreplicatickcount)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetRangeExceptions(&self, riid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(ppunk)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetSingleItemExceptions(&self, riid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).22)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(ppunk)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetChangeUnitExceptions(&self, riid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(ppunk)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn FindClockVectorForItem(&self, pbitemid: *const u8, riid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).24)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbitemid), ::std::mem::transmute(riid), ::std::mem::transmute(ppunk)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn FindClockVectorForChangeUnit(&self, pbitemid: *const u8, pbchangeunitid: *const u8, riid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbitemid), ::std::mem::transmute(pbchangeunitid), ::std::mem::transmute(riid), ::std::mem::transmute(ppunk)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetVersion(&self, pdwversion: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).26)(::std::mem::transmute_copy(self), ::std::mem::transmute(pdwversion)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_Foundation`*"]
    pub unsafe fn GetIdParameters(&self, pidparameters: *mut ID_PARAMETERS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).27)(::std::mem::transmute_copy(self), ::std::mem::transmute(pidparameters)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn ProjectOntoColumnSet(&self, ppcolumns: *const *const u8, count: u32) -> ::windows::runtime::Result<ISyncKnowledge2> {
        let mut result__: <ISyncKnowledge2 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).28)(::std::mem::transmute_copy(self), ::std::mem::transmute(ppcolumns), ::std::mem::transmute(count), &mut result__).from_abi::<ISyncKnowledge2>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn SerializeWithOptions(&self, targetformatversion: SYNC_SERIALIZATION_VERSION, dwflags: u32, pbbuffer: *mut u8, pdwserializedsize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).29)(::std::mem::transmute_copy(self), ::std::mem::transmute(targetformatversion), ::std::mem::transmute(dwflags), ::std::mem::transmute(pbbuffer), ::std::mem::transmute(pdwserializedsize)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetLowestUncontainedId<'a, Param0: ::windows::runtime::IntoParam<'a, ISyncKnowledge2>>(&self, pisyncknowledge: Param0, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).30)(::std::mem::transmute_copy(self), pisyncknowledge.into_param().abi(), ::std::mem::transmute(pbitemid), ::std::mem::transmute(pcbitemidsize)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetInspector(&self, riid: *const ::windows::runtime::GUID, ppiinspector: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).31)(::std::mem::transmute_copy(self), ::std::mem::transmute(riid), ::std::mem::transmute(ppiinspector)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetMinimumSupportedVersion(&self, pversion: *mut SYNC_SERIALIZATION_VERSION) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).32)(::std::mem::transmute_copy(self), ::std::mem::transmute(pversion)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetStatistics(&self, which: SYNC_STATISTICS, pvalue: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).33)(::std::mem::transmute_copy(self), ::std::mem::transmute(which), ::std::mem::transmute(pvalue)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn ContainsKnowledgeForItem<'a, Param0: ::windows::runtime::IntoParam<'a, ISyncKnowledge>>(&self, pknowledge: Param0, pbitemid: *const u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).34)(::std::mem::transmute_copy(self), pknowledge.into_param().abi(), ::std::mem::transmute(pbitemid)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn ContainsKnowledgeForChangeUnit<'a, Param0: ::windows::runtime::IntoParam<'a, ISyncKnowledge>>(&self, pknowledge: Param0, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).35)(::std::mem::transmute_copy(self), pknowledge.into_param().abi(), ::std::mem::transmute(pbitemid), ::std::mem::transmute(pbchangeunitid)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn ProjectOntoKnowledgeWithPrerequisite<'a, Param0: ::windows::runtime::IntoParam<'a, ISyncKnowledge>, Param1: ::windows::runtime::IntoParam<'a, ISyncKnowledge>>(&self, pprerequisiteknowledge: Param0, ptemplateknowledge: Param1) -> ::windows::runtime::Result<ISyncKnowledge> {
        let mut result__: <ISyncKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).36)(::std::mem::transmute_copy(self), pprerequisiteknowledge.into_param().abi(), ptemplateknowledge.into_param().abi(), &mut result__).from_abi::<ISyncKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Complement<'a, Param0: ::windows::runtime::IntoParam<'a, ISyncKnowledge>>(&self, psyncknowledge: Param0) -> ::windows::runtime::Result<ISyncKnowledge> {
        let mut result__: <ISyncKnowledge as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).37)(::std::mem::transmute_copy(self), psyncknowledge.into_param().abi(), &mut result__).from_abi::<ISyncKnowledge>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn IntersectsWithKnowledge<'a, Param0: ::windows::runtime::IntoParam<'a, ISyncKnowledge>>(&self, psyncknowledge: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).38)(::std::mem::transmute_copy(self), psyncknowledge.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetKnowledgeCookie(&self) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).39)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn CompareToKnowledgeCookie<'a, Param0: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, pknowledgecookie: Param0, presult: *mut KNOWLEDGE_COOKIE_COMPARISON_RESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).40)(::std::mem::transmute_copy(self), pknowledgecookie.into_param().abi(), ::std::mem::transmute(presult)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISyncKnowledge2 {
    type Vtable = ISyncKnowledge2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3976912320, 15179, 18081, [154, 69, 69, 102, 29, 33, 20, 200]);
}
impl ::std::convert::From<ISyncKnowledge2> for ::windows::runtime::IUnknown {
    fn from(value: ISyncKnowledge2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISyncKnowledge2> for ::windows::runtime::IUnknown {
    fn from(value: &ISyncKnowledge2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISyncKnowledge2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISyncKnowledge2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<ISyncKnowledge2> for ISyncKnowledge {
    fn from(value: ISyncKnowledge2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISyncKnowledge2> for ISyncKnowledge {
    fn from(value: &ISyncKnowledge2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyncKnowledge> for ISyncKnowledge2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyncKnowledge> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISyncKnowledge>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyncKnowledge> for &ISyncKnowledge2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyncKnowledge> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISyncKnowledge>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncKnowledge2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fserializereplicakeymap: super::super::Foundation::BOOL, pbknowledge: *mut u8, pcbknowledge: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ulltickcount: u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbversionownerreplicaid: *const u8, pgiditemid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbversionownerreplicaid: *const u8, pbitemid: *const u8, pbchangeunitid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppreplicakeymap: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppclonedknowledge: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pknowledgein: ::windows::runtime::RawPtr, pbcurrentownerid: *const u8, pversionin: *const SYNC_VERSION, pbnewownerid: *mut u8, pcbidsize: *mut u32, pversionout: *mut SYNC_VERSION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, premoteknowledge: ::windows::runtime::RawPtr, ppmappedknowledge: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pknowledge: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbitemid: *const u8, ppknowledgeout: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbitemid: *const u8, pbchangeunitid: *const u8, ppknowledgeout: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psrngsyncrange: *const SYNC_RANGE, ppknowledgeout: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbitemid: *const u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pknowledge: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbreplicaid: *const u8, pullreplicatickcount: *mut u64) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbitemid: *const u8, riid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbitemid: *const u8, pbchangeunitid: *const u8, riid: *const ::windows::runtime::GUID, ppunk: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwversion: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pidparameters: *mut ID_PARAMETERS) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppcolumns: *const *const u8, count: u32, ppiknowledgeout: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, targetformatversion: SYNC_SERIALIZATION_VERSION, dwflags: u32, pbbuffer: *mut u8, pdwserializedsize: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pisyncknowledge: ::windows::runtime::RawPtr, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, riid: *const ::windows::runtime::GUID, ppiinspector: *mut *mut ::std::ffi::c_void) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pversion: *mut SYNC_SERIALIZATION_VERSION) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, which: SYNC_STATISTICS, pvalue: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pknowledge: ::windows::runtime::RawPtr, pbitemid: *const u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pknowledge: ::windows::runtime::RawPtr, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pprerequisiteknowledge: ::windows::runtime::RawPtr, ptemplateknowledge: ::windows::runtime::RawPtr, ppprojectedknowledge: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psyncknowledge: ::windows::runtime::RawPtr, ppcomplementedknowledge: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psyncknowledge: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppknowledgecookie: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pknowledgecookie: ::windows::runtime::RawPtr, presult: *mut KNOWLEDGE_COOKIE_COMPARISON_RESULT) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISyncMergeTombstoneChange(pub ::windows::runtime::IUnknown);
impl ISyncMergeTombstoneChange {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetWinnerItemId(&self, pbwinneritemid: *mut u8, pcbidsize: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbwinneritemid), ::std::mem::transmute(pcbidsize)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISyncMergeTombstoneChange {
    type Vtable = ISyncMergeTombstoneChange_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(1858479511, 2307, 18508, [173, 97, 54, 214, 233, 56, 244, 123]);
}
impl ::std::convert::From<ISyncMergeTombstoneChange> for ::windows::runtime::IUnknown {
    fn from(value: ISyncMergeTombstoneChange) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISyncMergeTombstoneChange> for ::windows::runtime::IUnknown {
    fn from(value: &ISyncMergeTombstoneChange) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISyncMergeTombstoneChange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISyncMergeTombstoneChange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncMergeTombstoneChange_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbwinneritemid: *mut u8, pcbidsize: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISyncProvider(pub ::windows::runtime::IUnknown);
impl ISyncProvider {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_Foundation`*"]
    pub unsafe fn GetIdParameters(&self, pidparameters: *mut ID_PARAMETERS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pidparameters)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISyncProvider {
    type Vtable = ISyncProvider_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2405789782, 11214, 18967, [140, 104, 199, 187, 120, 152, 181, 111]);
}
impl ::std::convert::From<ISyncProvider> for ::windows::runtime::IUnknown {
    fn from(value: ISyncProvider) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISyncProvider> for ::windows::runtime::IUnknown {
    fn from(value: &ISyncProvider) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISyncProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISyncProvider {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncProvider_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pidparameters: *mut ID_PARAMETERS) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISyncProviderConfigUI(pub ::windows::runtime::IUnknown);
impl ISyncProviderConfigUI {
    #[cfg(feature = "Win32_System_PropertiesSystem")]
    #[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn Init<'a, Param2: ::windows::runtime::IntoParam<'a, super::PropertiesSystem::IPropertyStore>>(&self, pguidinstanceid: *const ::windows::runtime::GUID, pguidcontenttype: *const ::windows::runtime::GUID, pconfigurationproperties: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pguidinstanceid), ::std::mem::transmute(pguidcontenttype), pconfigurationproperties.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_System_PropertiesSystem")]
    #[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetRegisteredProperties(&self) -> ::windows::runtime::Result<super::PropertiesSystem::IPropertyStore> {
        let mut result__: <super::PropertiesSystem::IPropertyStore as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<super::PropertiesSystem::IPropertyStore>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_Foundation`*"]
    pub unsafe fn CreateAndRegisterNewSyncProvider<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>>(&self, hwndparent: Param0, punkcontext: Param1) -> ::windows::runtime::Result<ISyncProviderInfo> {
        let mut result__: <ISyncProviderInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), hwndparent.into_param().abi(), punkcontext.into_param().abi(), &mut result__).from_abi::<ISyncProviderInfo>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_Foundation`*"]
    pub unsafe fn ModifySyncProvider<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HWND>, Param1: ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown>, Param2: ::windows::runtime::IntoParam<'a, ISyncProviderInfo>>(&self, hwndparent: Param0, punkcontext: Param1, pproviderinfo: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), hwndparent.into_param().abi(), punkcontext.into_param().abi(), pproviderinfo.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISyncProviderConfigUI {
    type Vtable = ISyncProviderConfigUI_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2064057846, 52173, 16497, [171, 5, 59, 220, 54, 77, 74, 12]);
}
impl ::std::convert::From<ISyncProviderConfigUI> for ::windows::runtime::IUnknown {
    fn from(value: ISyncProviderConfigUI) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISyncProviderConfigUI> for ::windows::runtime::IUnknown {
    fn from(value: &ISyncProviderConfigUI) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISyncProviderConfigUI {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISyncProviderConfigUI {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncProviderConfigUI_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_System_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidinstanceid: *const ::windows::runtime::GUID, pguidcontenttype: *const ::windows::runtime::GUID, pconfigurationproperties: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_System_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppconfiguiproperties: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_PropertiesSystem"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwndparent: super::super::Foundation::HWND, punkcontext: ::windows::runtime::RawPtr, ppproviderinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hwndparent: super::super::Foundation::HWND, punkcontext: ::windows::runtime::RawPtr, pproviderinfo: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISyncProviderConfigUIInfo(pub ::windows::runtime::IUnknown);
impl ISyncProviderConfigUIInfo {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_PropertiesSystem")]
    #[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetAt(&self, iprop: u32) -> ::windows::runtime::Result<super::PropertiesSystem::PROPERTYKEY> {
        let mut result__: <super::PropertiesSystem::PROPERTYKEY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(iprop), &mut result__).from_abi::<super::PropertiesSystem::PROPERTYKEY>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
    #[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetValue(&self, key: *const super::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::Result<super::Com::StructuredStorage::PROPVARIANT> {
        let mut result__: <super::Com::StructuredStorage::PROPVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(key), &mut result__).from_abi::<super::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
    #[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn SetValue(&self, key: *const super::PropertiesSystem::PROPERTYKEY, propvar: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(key), ::std::mem::transmute(propvar)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Commit(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetSyncProviderConfigUI(&self, dwclscontext: u32) -> ::windows::runtime::Result<ISyncProviderConfigUI> {
        let mut result__: <ISyncProviderConfigUI as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwclscontext), &mut result__).from_abi::<ISyncProviderConfigUI>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISyncProviderConfigUIInfo {
    type Vtable = ISyncProviderConfigUIInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(557924782, 13271, 19853, [142, 55, 242, 39, 232, 128, 206, 80]);
}
impl ::std::convert::From<ISyncProviderConfigUIInfo> for ::windows::runtime::IUnknown {
    fn from(value: ISyncProviderConfigUIInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISyncProviderConfigUIInfo> for ::windows::runtime::IUnknown {
    fn from(value: &ISyncProviderConfigUIInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISyncProviderConfigUIInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISyncProviderConfigUIInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_PropertiesSystem")]
impl ::std::convert::From<ISyncProviderConfigUIInfo> for super::PropertiesSystem::IPropertyStore {
    fn from(value: ISyncProviderConfigUIInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_PropertiesSystem")]
impl ::std::convert::From<&ISyncProviderConfigUIInfo> for super::PropertiesSystem::IPropertyStore {
    fn from(value: &ISyncProviderConfigUIInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_PropertiesSystem")]
impl<'a> ::windows::runtime::IntoParam<'a, super::PropertiesSystem::IPropertyStore> for ISyncProviderConfigUIInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::PropertiesSystem::IPropertyStore> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::PropertiesSystem::IPropertyStore>::into(self))
    }
}
#[cfg(feature = "Win32_System_PropertiesSystem")]
impl<'a> ::windows::runtime::IntoParam<'a, super::PropertiesSystem::IPropertyStore> for &ISyncProviderConfigUIInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::PropertiesSystem::IPropertyStore> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::PropertiesSystem::IPropertyStore>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncProviderConfigUIInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cprops: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iprop: u32, pkey: *mut super::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_PropertiesSystem"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const super::PropertiesSystem::PROPERTYKEY, pv: *mut ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const super::PropertiesSystem::PROPERTYKEY, propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwclscontext: u32, ppsyncproviderconfigui: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISyncProviderInfo(pub ::windows::runtime::IUnknown);
impl ISyncProviderInfo {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetCount(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[cfg(feature = "Win32_System_PropertiesSystem")]
    #[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetAt(&self, iprop: u32) -> ::windows::runtime::Result<super::PropertiesSystem::PROPERTYKEY> {
        let mut result__: <super::PropertiesSystem::PROPERTYKEY as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(iprop), &mut result__).from_abi::<super::PropertiesSystem::PROPERTYKEY>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
    #[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn GetValue(&self, key: *const super::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::Result<super::Com::StructuredStorage::PROPVARIANT> {
        let mut result__: <super::Com::StructuredStorage::PROPVARIANT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(key), &mut result__).from_abi::<super::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))]
    #[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_System_Ole_Automation`, `Win32_System_PropertiesSystem`*"]
    pub unsafe fn SetValue(&self, key: *const super::PropertiesSystem::PROPERTYKEY, propvar: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(key), ::std::mem::transmute(propvar)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn Commit(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetSyncProvider(&self, dwclscontext: u32) -> ::windows::runtime::Result<IRegisteredSyncProvider> {
        let mut result__: <IRegisteredSyncProvider as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(dwclscontext), &mut result__).from_abi::<IRegisteredSyncProvider>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISyncProviderInfo {
    type Vtable = ISyncProviderInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(518075870, 34980, 17668, [176, 208, 247, 146, 13, 126, 91, 166]);
}
impl ::std::convert::From<ISyncProviderInfo> for ::windows::runtime::IUnknown {
    fn from(value: ISyncProviderInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISyncProviderInfo> for ::windows::runtime::IUnknown {
    fn from(value: &ISyncProviderInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISyncProviderInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISyncProviderInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[cfg(feature = "Win32_System_PropertiesSystem")]
impl ::std::convert::From<ISyncProviderInfo> for super::PropertiesSystem::IPropertyStore {
    fn from(value: ISyncProviderInfo) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_System_PropertiesSystem")]
impl ::std::convert::From<&ISyncProviderInfo> for super::PropertiesSystem::IPropertyStore {
    fn from(value: &ISyncProviderInfo) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_System_PropertiesSystem")]
impl<'a> ::windows::runtime::IntoParam<'a, super::PropertiesSystem::IPropertyStore> for ISyncProviderInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::PropertiesSystem::IPropertyStore> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::PropertiesSystem::IPropertyStore>::into(self))
    }
}
#[cfg(feature = "Win32_System_PropertiesSystem")]
impl<'a> ::windows::runtime::IntoParam<'a, super::PropertiesSystem::IPropertyStore> for &ISyncProviderInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, super::PropertiesSystem::IPropertyStore> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<super::PropertiesSystem::IPropertyStore>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncProviderInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, cprops: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_System_PropertiesSystem")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iprop: u32, pkey: *mut super::PropertiesSystem::PROPERTYKEY) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_System_PropertiesSystem"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const super::PropertiesSystem::PROPERTYKEY, pv: *mut ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem")))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, key: *const super::PropertiesSystem::PROPERTYKEY, propvar: *const ::std::mem::ManuallyDrop<super::Com::StructuredStorage::PROPVARIANT>) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Ole_Automation", feature = "Win32_System_PropertiesSystem")))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwclscontext: u32, ppsyncprovider: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISyncProviderRegistration(pub ::windows::runtime::IUnknown);
impl ISyncProviderRegistration {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_Foundation`*"]
    pub unsafe fn CreateSyncProviderConfigUIRegistrationInstance(&self, pconfiguiconfig: *const SyncProviderConfigUIConfiguration) -> ::windows::runtime::Result<ISyncProviderConfigUIInfo> {
        let mut result__: <ISyncProviderConfigUIInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pconfiguiconfig), &mut result__).from_abi::<ISyncProviderConfigUIInfo>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn UnregisterSyncProviderConfigUI(&self, pguidinstanceid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pguidinstanceid)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn EnumerateSyncProviderConfigUIs(&self, pguidcontenttype: *const ::windows::runtime::GUID, dwsupportedarchitecture: u32) -> ::windows::runtime::Result<IEnumSyncProviderConfigUIInfos> {
        let mut result__: <IEnumSyncProviderConfigUIInfos as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pguidcontenttype), ::std::mem::transmute(dwsupportedarchitecture), &mut result__).from_abi::<IEnumSyncProviderConfigUIInfos>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn CreateSyncProviderRegistrationInstance(&self, pproviderconfiguration: *const SyncProviderConfiguration) -> ::windows::runtime::Result<ISyncProviderInfo> {
        let mut result__: <ISyncProviderInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pproviderconfiguration), &mut result__).from_abi::<ISyncProviderInfo>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn UnregisterSyncProvider(&self, pguidinstanceid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(pguidinstanceid)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetSyncProviderConfigUIInfoforProvider(&self, pguidproviderinstanceid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<ISyncProviderConfigUIInfo> {
        let mut result__: <ISyncProviderConfigUIInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(pguidproviderinstanceid), &mut result__).from_abi::<ISyncProviderConfigUIInfo>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn EnumerateSyncProviders(&self, pguidcontenttype: *const ::windows::runtime::GUID, dwstateflagstofiltermask: u32, dwstateflagstofilter: u32, refproviderclsid: *const ::windows::runtime::GUID, dwsupportedarchitecture: u32) -> ::windows::runtime::Result<IEnumSyncProviderInfos> {
        let mut result__: <IEnumSyncProviderInfos as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(pguidcontenttype), ::std::mem::transmute(dwstateflagstofiltermask), ::std::mem::transmute(dwstateflagstofilter), ::std::mem::transmute(refproviderclsid), ::std::mem::transmute(dwsupportedarchitecture), &mut result__).from_abi::<IEnumSyncProviderInfos>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetSyncProviderInfo(&self, pguidinstanceid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<ISyncProviderInfo> {
        let mut result__: <ISyncProviderInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), ::std::mem::transmute(pguidinstanceid), &mut result__).from_abi::<ISyncProviderInfo>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetSyncProviderFromInstanceId(&self, pguidinstanceid: *const ::windows::runtime::GUID, dwclscontext: u32) -> ::windows::runtime::Result<IRegisteredSyncProvider> {
        let mut result__: <IRegisteredSyncProvider as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(pguidinstanceid), ::std::mem::transmute(dwclscontext), &mut result__).from_abi::<IRegisteredSyncProvider>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetSyncProviderConfigUIInfo(&self, pguidinstanceid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<ISyncProviderConfigUIInfo> {
        let mut result__: <ISyncProviderConfigUIInfo as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::std::mem::transmute_copy(self), ::std::mem::transmute(pguidinstanceid), &mut result__).from_abi::<ISyncProviderConfigUIInfo>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetSyncProviderConfigUIFromInstanceId(&self, pguidinstanceid: *const ::windows::runtime::GUID, dwclscontext: u32) -> ::windows::runtime::Result<ISyncProviderConfigUI> {
        let mut result__: <ISyncProviderConfigUI as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::std::mem::transmute_copy(self), ::std::mem::transmute(pguidinstanceid), ::std::mem::transmute(dwclscontext), &mut result__).from_abi::<ISyncProviderConfigUI>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetSyncProviderState(&self, pguidinstanceid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::std::mem::transmute_copy(self), ::std::mem::transmute(pguidinstanceid), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn SetSyncProviderState(&self, pguidinstanceid: *const ::windows::runtime::GUID, dwstateflagsmask: u32, dwstateflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::std::mem::transmute_copy(self), ::std::mem::transmute(pguidinstanceid), ::std::mem::transmute(dwstateflagsmask), ::std::mem::transmute(dwstateflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_Foundation`*"]
    pub unsafe fn RegisterForEvent(&self, phevent: *mut super::super::Foundation::HANDLE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::std::mem::transmute_copy(self), ::std::mem::transmute(phevent)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_Foundation`*"]
    pub unsafe fn RevokeEvent<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, hevent: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::std::mem::transmute_copy(self), hevent.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_Foundation`*"]
    pub unsafe fn GetChange<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::HANDLE>>(&self, hevent: Param0) -> ::windows::runtime::Result<ISyncRegistrationChange> {
        let mut result__: <ISyncRegistrationChange as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::std::mem::transmute_copy(self), hevent.into_param().abi(), &mut result__).from_abi::<ISyncRegistrationChange>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISyncProviderRegistration {
    type Vtable = ISyncProviderRegistration_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3410335035, 30244, 18364, [164, 114, 235, 140, 172, 107, 34, 46]);
}
impl ::std::convert::From<ISyncProviderRegistration> for ::windows::runtime::IUnknown {
    fn from(value: ISyncProviderRegistration) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISyncProviderRegistration> for ::windows::runtime::IUnknown {
    fn from(value: &ISyncProviderRegistration) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISyncProviderRegistration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISyncProviderRegistration {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncProviderRegistration_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pconfiguiconfig: *const SyncProviderConfigUIConfiguration, ppconfiguiinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidinstanceid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidcontenttype: *const ::windows::runtime::GUID, dwsupportedarchitecture: u32, ppenumsyncproviderconfiguiinfos: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pproviderconfiguration: *const SyncProviderConfiguration, ppproviderinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidinstanceid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidproviderinstanceid: *const ::windows::runtime::GUID, ppproviderconfiguiinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidcontenttype: *const ::windows::runtime::GUID, dwstateflagstofiltermask: u32, dwstateflagstofilter: u32, refproviderclsid: *const ::windows::runtime::GUID, dwsupportedarchitecture: u32, ppenumsyncproviderinfos: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidinstanceid: *const ::windows::runtime::GUID, ppproviderinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidinstanceid: *const ::windows::runtime::GUID, dwclscontext: u32, ppsyncprovider: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidinstanceid: *const ::windows::runtime::GUID, ppconfiguiinfo: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidinstanceid: *const ::windows::runtime::GUID, dwclscontext: u32, ppconfigui: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidinstanceid: *const ::windows::runtime::GUID, pdwstateflags: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidinstanceid: *const ::windows::runtime::GUID, dwstateflagsmask: u32, dwstateflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phevent: *mut super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hevent: super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, hevent: super::super::Foundation::HANDLE, ppchange: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISyncRegistrationChange(pub ::windows::runtime::IUnknown);
impl ISyncRegistrationChange {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetEvent(&self) -> ::windows::runtime::Result<SYNC_REGISTRATION_EVENT> {
        let mut result__: <SYNC_REGISTRATION_EVENT as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<SYNC_REGISTRATION_EVENT>(result__)
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetInstanceId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISyncRegistrationChange {
    type Vtable = ISyncRegistrationChange_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4003518894, 27433, 17332, [158, 112, 227, 174, 51, 187, 44, 59]);
}
impl ::std::convert::From<ISyncRegistrationChange> for ::windows::runtime::IUnknown {
    fn from(value: ISyncRegistrationChange) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISyncRegistrationChange> for ::windows::runtime::IUnknown {
    fn from(value: &ISyncRegistrationChange) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISyncRegistrationChange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISyncRegistrationChange {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncRegistrationChange_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psreevent: *mut SYNC_REGISTRATION_EVENT) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pguidinstanceid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISyncSessionExtendedErrorInfo(pub ::windows::runtime::IUnknown);
impl ISyncSessionExtendedErrorInfo {
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetSyncProviderWithError(&self) -> ::windows::runtime::Result<ISyncProvider> {
        let mut result__: <ISyncProvider as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), &mut result__).from_abi::<ISyncProvider>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISyncSessionExtendedErrorInfo {
    type Vtable = ISyncSessionExtendedErrorInfo_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(845965328, 30986, 16539, [183, 65, 105, 153, 56, 135, 97, 235]);
}
impl ::std::convert::From<ISyncSessionExtendedErrorInfo> for ::windows::runtime::IUnknown {
    fn from(value: ISyncSessionExtendedErrorInfo) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISyncSessionExtendedErrorInfo> for ::windows::runtime::IUnknown {
    fn from(value: &ISyncSessionExtendedErrorInfo) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISyncSessionExtendedErrorInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISyncSessionExtendedErrorInfo {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncSessionExtendedErrorInfo_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppproviderwitherror: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISyncSessionState(pub ::windows::runtime::IUnknown);
impl ISyncSessionState {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_Foundation`*"]
    pub unsafe fn IsCanceled(&self, pfiscanceled: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pfiscanceled)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetInfoForChangeApplication(&self, pbchangeapplierinfo: *mut u8, pcbchangeapplierinfo: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbchangeapplierinfo), ::std::mem::transmute(pcbchangeapplierinfo)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn LoadInfoFromChangeApplication(&self, pbchangeapplierinfo: *const u8, cbchangeapplierinfo: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbchangeapplierinfo), ::std::mem::transmute(cbchangeapplierinfo)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetForgottenKnowledgeRecoveryRangeStart(&self, pbrangestart: *mut u8, pcbrangestart: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbrangestart), ::std::mem::transmute(pcbrangestart)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetForgottenKnowledgeRecoveryRangeEnd(&self, pbrangeend: *mut u8, pcbrangeend: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbrangeend), ::std::mem::transmute(pcbrangeend)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn SetForgottenKnowledgeRecoveryRange(&self, prange: *const SYNC_RANGE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(prange)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn OnProgress(&self, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(provider), ::std::mem::transmute(syncstage), ::std::mem::transmute(dwcompletedwork), ::std::mem::transmute(dwtotalwork)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISyncSessionState {
    type Vtable = ISyncSessionState_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(3098099966, 40705, 18491, [148, 52, 195, 125, 54, 18, 37, 217]);
}
impl ::std::convert::From<ISyncSessionState> for ::windows::runtime::IUnknown {
    fn from(value: ISyncSessionState) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISyncSessionState> for ::windows::runtime::IUnknown {
    fn from(value: &ISyncSessionState) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISyncSessionState {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISyncSessionState {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncSessionState_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfiscanceled: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbchangeapplierinfo: *mut u8, pcbchangeapplierinfo: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbchangeapplierinfo: *const u8, cbchangeapplierinfo: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbrangestart: *mut u8, pcbrangestart: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbrangeend: *mut u8, pcbrangeend: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prange: *const SYNC_RANGE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISyncSessionState2(pub ::windows::runtime::IUnknown);
impl ISyncSessionState2 {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_Foundation`*"]
    pub unsafe fn IsCanceled(&self, pfiscanceled: *mut super::super::Foundation::BOOL) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pfiscanceled)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetInfoForChangeApplication(&self, pbchangeapplierinfo: *mut u8, pcbchangeapplierinfo: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbchangeapplierinfo), ::std::mem::transmute(pcbchangeapplierinfo)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn LoadInfoFromChangeApplication(&self, pbchangeapplierinfo: *const u8, cbchangeapplierinfo: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbchangeapplierinfo), ::std::mem::transmute(cbchangeapplierinfo)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetForgottenKnowledgeRecoveryRangeStart(&self, pbrangestart: *mut u8, pcbrangestart: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbrangestart), ::std::mem::transmute(pcbrangestart)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetForgottenKnowledgeRecoveryRangeEnd(&self, pbrangeend: *mut u8, pcbrangeend: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::std::mem::transmute_copy(self), ::std::mem::transmute(pbrangeend), ::std::mem::transmute(pcbrangeend)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn SetForgottenKnowledgeRecoveryRange(&self, prange: *const SYNC_RANGE) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::std::mem::transmute_copy(self), ::std::mem::transmute(prange)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn OnProgress(&self, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::std::mem::transmute_copy(self), ::std::mem::transmute(provider), ::std::mem::transmute(syncstage), ::std::mem::transmute(dwcompletedwork), ::std::mem::transmute(dwtotalwork)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_Foundation`*"]
    pub unsafe fn SetProviderWithError<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, fself: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::std::mem::transmute_copy(self), fself.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn GetSessionErrorStatus(&self, phrsessionerror: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::std::mem::transmute_copy(self), ::std::mem::transmute(phrsessionerror)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for ISyncSessionState2 {
    type Vtable = ISyncSessionState2_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2654457763, 40504, 19553, [156, 163, 255, 232, 16, 180, 92, 162]);
}
impl ::std::convert::From<ISyncSessionState2> for ::windows::runtime::IUnknown {
    fn from(value: ISyncSessionState2) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISyncSessionState2> for ::windows::runtime::IUnknown {
    fn from(value: &ISyncSessionState2) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISyncSessionState2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISyncSessionState2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
impl ::std::convert::From<ISyncSessionState2> for ISyncSessionState {
    fn from(value: ISyncSessionState2) -> Self {
        unsafe { ::std::mem::transmute(value) }
    }
}
impl ::std::convert::From<&ISyncSessionState2> for ISyncSessionState {
    fn from(value: &ISyncSessionState2) -> Self {
        ::std::convert::From::from(::std::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyncSessionState> for ISyncSessionState2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyncSessionState> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISyncSessionState>::into(self))
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ISyncSessionState> for &ISyncSessionState2 {
    fn into_param(self) -> ::windows::runtime::Param<'a, ISyncSessionState> {
        ::windows::runtime::Param::Owned(::std::convert::Into::<ISyncSessionState>::into(::std::clone::Clone::clone(self)))
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncSessionState2_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfiscanceled: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbchangeapplierinfo: *mut u8, pcbchangeapplierinfo: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbchangeapplierinfo: *const u8, cbchangeapplierinfo: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbrangestart: *mut u8, pcbrangestart: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pbrangeend: *mut u8, pcbrangeend: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, prange: *const SYNC_RANGE) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, fself: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, phrsessionerror: *mut ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[repr(transparent)]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: clone :: Clone, :: std :: fmt :: Debug)]
pub struct ISynchronousDataRetriever(pub ::windows::runtime::IUnknown);
impl ISynchronousDataRetriever {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_Foundation`*"]
    pub unsafe fn GetIdParameters(&self, pidparameters: *mut ID_PARAMETERS) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::std::mem::transmute_copy(self), ::std::mem::transmute(pidparameters)).ok()
    }
    #[doc = "*Required features: `Win32_System_WindowsSync`*"]
    pub unsafe fn LoadChangeData<'a, Param0: ::windows::runtime::IntoParam<'a, ILoadChangeContext>>(&self, ploadchangecontext: Param0) -> ::windows::runtime::Result<::windows::runtime::IUnknown> {
        let mut result__: <::windows::runtime::IUnknown as ::windows::runtime::Abi>::Abi = ::std::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::std::mem::transmute_copy(self), ploadchangecontext.into_param().abi(), &mut result__).from_abi::<::windows::runtime::IUnknown>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for ISynchronousDataRetriever {
    type Vtable = ISynchronousDataRetriever_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(2602758825, 42189, 17992, [157, 142, 58, 81, 13, 77, 160, 75]);
}
impl ::std::convert::From<ISynchronousDataRetriever> for ::windows::runtime::IUnknown {
    fn from(value: ISynchronousDataRetriever) -> Self {
        value.0
    }
}
impl ::std::convert::From<&ISynchronousDataRetriever> for ::windows::runtime::IUnknown {
    fn from(value: &ISynchronousDataRetriever) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for ISynchronousDataRetriever {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a ISynchronousDataRetriever {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronousDataRetriever_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pidparameters: *mut ID_PARAMETERS) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ploadchangecontext: ::windows::runtime::RawPtr, ppunkdata: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct KNOWLEDGE_COOKIE_COMPARISON_RESULT(pub i32);
pub const KCCR_COOKIE_KNOWLEDGE_EQUAL: KNOWLEDGE_COOKIE_COMPARISON_RESULT = KNOWLEDGE_COOKIE_COMPARISON_RESULT(0i32);
pub const KCCR_COOKIE_KNOWLEDGE_CONTAINED: KNOWLEDGE_COOKIE_COMPARISON_RESULT = KNOWLEDGE_COOKIE_COMPARISON_RESULT(1i32);
pub const KCCR_COOKIE_KNOWLEDGE_CONTAINS: KNOWLEDGE_COOKIE_COMPARISON_RESULT = KNOWLEDGE_COOKIE_COMPARISON_RESULT(2i32);
pub const KCCR_COOKIE_KNOWLEDGE_NOT_COMPARABLE: KNOWLEDGE_COOKIE_COMPARISON_RESULT = KNOWLEDGE_COOKIE_COMPARISON_RESULT(3i32);
impl ::std::convert::From<i32> for KNOWLEDGE_COOKIE_COMPARISON_RESULT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for KNOWLEDGE_COOKIE_COMPARISON_RESULT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
pub const SYNC_CHANGE_FLAG_DELETED: u32 = 1u32;
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
pub const SYNC_CHANGE_FLAG_DOES_NOT_EXIST: u32 = 2u32;
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
pub const SYNC_CHANGE_FLAG_GHOST: u32 = 4u32;
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SYNC_CONSTRAINT_RESOLVE_ACTION(pub i32);
pub const SCRA_DEFER: SYNC_CONSTRAINT_RESOLVE_ACTION = SYNC_CONSTRAINT_RESOLVE_ACTION(0i32);
pub const SCRA_ACCEPT_DESTINATION_PROVIDER: SYNC_CONSTRAINT_RESOLVE_ACTION = SYNC_CONSTRAINT_RESOLVE_ACTION(1i32);
pub const SCRA_ACCEPT_SOURCE_PROVIDER: SYNC_CONSTRAINT_RESOLVE_ACTION = SYNC_CONSTRAINT_RESOLVE_ACTION(2i32);
pub const SCRA_TRANSFER_AND_DEFER: SYNC_CONSTRAINT_RESOLVE_ACTION = SYNC_CONSTRAINT_RESOLVE_ACTION(3i32);
pub const SCRA_MERGE: SYNC_CONSTRAINT_RESOLVE_ACTION = SYNC_CONSTRAINT_RESOLVE_ACTION(4i32);
pub const SCRA_RENAME_SOURCE: SYNC_CONSTRAINT_RESOLVE_ACTION = SYNC_CONSTRAINT_RESOLVE_ACTION(5i32);
pub const SCRA_RENAME_DESTINATION: SYNC_CONSTRAINT_RESOLVE_ACTION = SYNC_CONSTRAINT_RESOLVE_ACTION(6i32);
impl ::std::convert::From<i32> for SYNC_CONSTRAINT_RESOLVE_ACTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SYNC_CONSTRAINT_RESOLVE_ACTION {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_Foundation`*"]
pub struct SYNC_FILTER_CHANGE {
    pub fMoveIn: super::super::Foundation::BOOL,
    pub moveVersion: SYNC_VERSION,
}
#[cfg(feature = "Win32_Foundation")]
impl SYNC_FILTER_CHANGE {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SYNC_FILTER_CHANGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SYNC_FILTER_CHANGE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SYNC_FILTER_CHANGE").field("fMoveIn", &self.fMoveIn).field("moveVersion", &self.moveVersion).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SYNC_FILTER_CHANGE {
    fn eq(&self, other: &Self) -> bool {
        self.fMoveIn == other.fMoveIn && self.moveVersion == other.moveVersion
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SYNC_FILTER_CHANGE {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SYNC_FILTER_CHANGE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
pub const SYNC_FILTER_INFO_COMBINED: u32 = 8u32;
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
pub const SYNC_FILTER_INFO_FLAG_CHANGE_UNIT_LIST: u32 = 2u32;
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
pub const SYNC_FILTER_INFO_FLAG_CUSTOM: u32 = 4u32;
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
pub const SYNC_FILTER_INFO_FLAG_ITEM_LIST: u32 = 1u32;
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SYNC_FULL_ENUMERATION_ACTION(pub i32);
pub const SFEA_FULL_ENUMERATION: SYNC_FULL_ENUMERATION_ACTION = SYNC_FULL_ENUMERATION_ACTION(0i32);
pub const SFEA_PARTIAL_SYNC: SYNC_FULL_ENUMERATION_ACTION = SYNC_FULL_ENUMERATION_ACTION(1i32);
pub const SFEA_ABORT: SYNC_FULL_ENUMERATION_ACTION = SYNC_FULL_ENUMERATION_ACTION(2i32);
impl ::std::convert::From<i32> for SYNC_FULL_ENUMERATION_ACTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SYNC_FULL_ENUMERATION_ACTION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SYNC_PROGRESS_STAGE(pub i32);
pub const SPS_CHANGE_DETECTION: SYNC_PROGRESS_STAGE = SYNC_PROGRESS_STAGE(0i32);
pub const SPS_CHANGE_ENUMERATION: SYNC_PROGRESS_STAGE = SYNC_PROGRESS_STAGE(1i32);
pub const SPS_CHANGE_APPLICATION: SYNC_PROGRESS_STAGE = SYNC_PROGRESS_STAGE(2i32);
impl ::std::convert::From<i32> for SYNC_PROGRESS_STAGE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SYNC_PROGRESS_STAGE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SYNC_PROVIDER_ROLE(pub i32);
pub const SPR_SOURCE: SYNC_PROVIDER_ROLE = SYNC_PROVIDER_ROLE(0i32);
pub const SPR_DESTINATION: SYNC_PROVIDER_ROLE = SYNC_PROVIDER_ROLE(1i32);
impl ::std::convert::From<i32> for SYNC_PROVIDER_ROLE {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SYNC_PROVIDER_ROLE {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
pub struct SYNC_RANGE {
    pub pbClosedLowerBound: *mut u8,
    pub pbClosedUpperBound: *mut u8,
}
impl SYNC_RANGE {}
impl ::std::default::Default for SYNC_RANGE {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SYNC_RANGE {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SYNC_RANGE").field("pbClosedLowerBound", &self.pbClosedLowerBound).field("pbClosedUpperBound", &self.pbClosedUpperBound).finish()
    }
}
impl ::std::cmp::PartialEq for SYNC_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.pbClosedLowerBound == other.pbClosedLowerBound && self.pbClosedUpperBound == other.pbClosedUpperBound
    }
}
impl ::std::cmp::Eq for SYNC_RANGE {}
unsafe impl ::windows::runtime::Abi for SYNC_RANGE {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SYNC_REGISTRATION_EVENT(pub i32);
pub const SRE_PROVIDER_ADDED: SYNC_REGISTRATION_EVENT = SYNC_REGISTRATION_EVENT(0i32);
pub const SRE_PROVIDER_REMOVED: SYNC_REGISTRATION_EVENT = SYNC_REGISTRATION_EVENT(1i32);
pub const SRE_PROVIDER_UPDATED: SYNC_REGISTRATION_EVENT = SYNC_REGISTRATION_EVENT(2i32);
pub const SRE_PROVIDER_STATE_CHANGED: SYNC_REGISTRATION_EVENT = SYNC_REGISTRATION_EVENT(3i32);
pub const SRE_CONFIGUI_ADDED: SYNC_REGISTRATION_EVENT = SYNC_REGISTRATION_EVENT(4i32);
pub const SRE_CONFIGUI_REMOVED: SYNC_REGISTRATION_EVENT = SYNC_REGISTRATION_EVENT(5i32);
pub const SRE_CONFIGUI_UPDATED: SYNC_REGISTRATION_EVENT = SYNC_REGISTRATION_EVENT(6i32);
impl ::std::convert::From<i32> for SYNC_REGISTRATION_EVENT {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SYNC_REGISTRATION_EVENT {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SYNC_RESOLVE_ACTION(pub i32);
pub const SRA_DEFER: SYNC_RESOLVE_ACTION = SYNC_RESOLVE_ACTION(0i32);
pub const SRA_ACCEPT_DESTINATION_PROVIDER: SYNC_RESOLVE_ACTION = SYNC_RESOLVE_ACTION(1i32);
pub const SRA_ACCEPT_SOURCE_PROVIDER: SYNC_RESOLVE_ACTION = SYNC_RESOLVE_ACTION(2i32);
pub const SRA_MERGE: SYNC_RESOLVE_ACTION = SYNC_RESOLVE_ACTION(3i32);
pub const SRA_TRANSFER_AND_DEFER: SYNC_RESOLVE_ACTION = SYNC_RESOLVE_ACTION(4i32);
pub const SRA_LAST: SYNC_RESOLVE_ACTION = SYNC_RESOLVE_ACTION(5i32);
impl ::std::convert::From<i32> for SYNC_RESOLVE_ACTION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SYNC_RESOLVE_ACTION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SYNC_SERIALIZATION_VERSION(pub i32);
pub const SYNC_SERIALIZATION_VERSION_V1: SYNC_SERIALIZATION_VERSION = SYNC_SERIALIZATION_VERSION(1i32);
pub const SYNC_SERIALIZATION_VERSION_V2: SYNC_SERIALIZATION_VERSION = SYNC_SERIALIZATION_VERSION(4i32);
pub const SYNC_SERIALIZATION_VERSION_V3: SYNC_SERIALIZATION_VERSION = SYNC_SERIALIZATION_VERSION(5i32);
impl ::std::convert::From<i32> for SYNC_SERIALIZATION_VERSION {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SYNC_SERIALIZATION_VERSION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
pub const SYNC_SERIALIZE_REPLICA_KEY_MAP: u32 = 1u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
pub struct SYNC_SESSION_STATISTICS {
    pub dwChangesApplied: u32,
    pub dwChangesFailed: u32,
}
impl SYNC_SESSION_STATISTICS {}
impl ::std::default::Default for SYNC_SESSION_STATISTICS {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SYNC_SESSION_STATISTICS {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SYNC_SESSION_STATISTICS").field("dwChangesApplied", &self.dwChangesApplied).field("dwChangesFailed", &self.dwChangesFailed).finish()
    }
}
impl ::std::cmp::PartialEq for SYNC_SESSION_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.dwChangesApplied == other.dwChangesApplied && self.dwChangesFailed == other.dwChangesFailed
    }
}
impl ::std::cmp::Eq for SYNC_SESSION_STATISTICS {}
unsafe impl ::windows::runtime::Abi for SYNC_SESSION_STATISTICS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
#[derive(:: std :: cmp :: PartialEq, :: std :: cmp :: Eq, :: std :: marker :: Copy, :: std :: clone :: Clone, :: std :: default :: Default, :: std :: fmt :: Debug)]
#[repr(transparent)]
pub struct SYNC_STATISTICS(pub i32);
pub const SYNC_STATISTICS_RANGE_COUNT: SYNC_STATISTICS = SYNC_STATISTICS(0i32);
impl ::std::convert::From<i32> for SYNC_STATISTICS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for SYNC_STATISTICS {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
pub struct SYNC_TIME {
    pub dwDate: u32,
    pub dwTime: u32,
}
impl SYNC_TIME {}
impl ::std::default::Default for SYNC_TIME {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SYNC_TIME {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SYNC_TIME").field("dwDate", &self.dwDate).field("dwTime", &self.dwTime).finish()
    }
}
impl ::std::cmp::PartialEq for SYNC_TIME {
    fn eq(&self, other: &Self) -> bool {
        self.dwDate == other.dwDate && self.dwTime == other.dwTime
    }
}
impl ::std::cmp::Eq for SYNC_TIME {}
unsafe impl ::windows::runtime::Abi for SYNC_TIME {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
pub struct SYNC_VERSION {
    pub dwLastUpdatingReplicaKey: u32,
    pub ullTickCount: u64,
}
impl SYNC_VERSION {}
impl ::std::default::Default for SYNC_VERSION {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SYNC_VERSION {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SYNC_VERSION").field("dwLastUpdatingReplicaKey", &self.dwLastUpdatingReplicaKey).field("ullTickCount", &self.ullTickCount).finish()
    }
}
impl ::std::cmp::PartialEq for SYNC_VERSION {
    fn eq(&self, other: &Self) -> bool {
        self.dwLastUpdatingReplicaKey == other.dwLastUpdatingReplicaKey && self.ullTickCount == other.ullTickCount
    }
}
impl ::std::cmp::Eq for SYNC_VERSION {}
unsafe impl ::windows::runtime::Abi for SYNC_VERSION {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
pub const SYNC_VERSION_FLAG_FROM_FEED: u32 = 1u32;
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
pub const SYNC_VERSION_FLAG_HAS_BY: u32 = 2u32;
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
#[doc = "*Required features: `Win32_System_WindowsSync`, `Win32_Foundation`*"]
pub struct SyncProviderConfigUIConfiguration {
    pub dwVersion: u32,
    pub guidInstanceId: ::windows::runtime::GUID,
    pub clsidConfigUI: ::windows::runtime::GUID,
    pub guidContentType: ::windows::runtime::GUID,
    pub dwCapabilities: u32,
    pub dwSupportedArchitecture: u32,
    pub fIsGlobal: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl SyncProviderConfigUIConfiguration {}
#[cfg(feature = "Win32_Foundation")]
impl ::std::default::Default for SyncProviderConfigUIConfiguration {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::fmt::Debug for SyncProviderConfigUIConfiguration {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SyncProviderConfigUIConfiguration")
            .field("dwVersion", &self.dwVersion)
            .field("guidInstanceId", &self.guidInstanceId)
            .field("clsidConfigUI", &self.clsidConfigUI)
            .field("guidContentType", &self.guidContentType)
            .field("dwCapabilities", &self.dwCapabilities)
            .field("dwSupportedArchitecture", &self.dwSupportedArchitecture)
            .field("fIsGlobal", &self.fIsGlobal)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::PartialEq for SyncProviderConfigUIConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.guidInstanceId == other.guidInstanceId && self.clsidConfigUI == other.clsidConfigUI && self.guidContentType == other.guidContentType && self.dwCapabilities == other.dwCapabilities && self.dwSupportedArchitecture == other.dwSupportedArchitecture && self.fIsGlobal == other.fIsGlobal
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::std::cmp::Eq for SyncProviderConfigUIConfiguration {}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::runtime::Abi for SyncProviderConfigUIConfiguration {
    type Abi = Self;
}
#[derive(:: std :: clone :: Clone, :: std :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_WindowsSync`*"]
pub struct SyncProviderConfiguration {
    pub dwVersion: u32,
    pub guidInstanceId: ::windows::runtime::GUID,
    pub clsidProvider: ::windows::runtime::GUID,
    pub guidConfigUIInstanceId: ::windows::runtime::GUID,
    pub guidContentType: ::windows::runtime::GUID,
    pub dwCapabilities: u32,
    pub dwSupportedArchitecture: u32,
}
impl SyncProviderConfiguration {}
impl ::std::default::Default for SyncProviderConfiguration {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
impl ::std::fmt::Debug for SyncProviderConfiguration {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.debug_struct("SyncProviderConfiguration")
            .field("dwVersion", &self.dwVersion)
            .field("guidInstanceId", &self.guidInstanceId)
            .field("clsidProvider", &self.clsidProvider)
            .field("guidConfigUIInstanceId", &self.guidConfigUIInstanceId)
            .field("guidContentType", &self.guidContentType)
            .field("dwCapabilities", &self.dwCapabilities)
            .field("dwSupportedArchitecture", &self.dwSupportedArchitecture)
            .finish()
    }
}
impl ::std::cmp::PartialEq for SyncProviderConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.dwVersion == other.dwVersion && self.guidInstanceId == other.guidInstanceId && self.clsidProvider == other.clsidProvider && self.guidConfigUIInstanceId == other.guidConfigUIInstanceId && self.guidContentType == other.guidContentType && self.dwCapabilities == other.dwCapabilities && self.dwSupportedArchitecture == other.dwSupportedArchitecture
    }
}
impl ::std::cmp::Eq for SyncProviderConfiguration {}
unsafe impl ::windows::runtime::Abi for SyncProviderConfiguration {
    type Abi = Self;
}
pub const SyncProviderRegistration: ::windows::runtime::GUID = ::windows::runtime::GUID::from_values(4163587825, 37801, 19934, [128, 21, 247, 149, 10, 26, 110, 49]);
