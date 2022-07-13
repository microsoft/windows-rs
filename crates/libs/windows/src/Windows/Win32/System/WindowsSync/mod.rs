#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CONFLICT_RESOLUTION_POLICY(pub i32);
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const CRP_NONE: CONFLICT_RESOLUTION_POLICY = CONFLICT_RESOLUTION_POLICY(0i32);
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const CRP_DESTINATION_PROVIDER_WINS: CONFLICT_RESOLUTION_POLICY = CONFLICT_RESOLUTION_POLICY(1i32);
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const CRP_SOURCE_PROVIDER_WINS: CONFLICT_RESOLUTION_POLICY = CONFLICT_RESOLUTION_POLICY(2i32);
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const CRP_LAST: CONFLICT_RESOLUTION_POLICY = CONFLICT_RESOLUTION_POLICY(3i32);
impl ::core::marker::Copy for CONFLICT_RESOLUTION_POLICY {}
impl ::core::clone::Clone for CONFLICT_RESOLUTION_POLICY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CONFLICT_RESOLUTION_POLICY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CONFLICT_RESOLUTION_POLICY {
    type Abi = Self;
}
impl ::core::fmt::Debug for CONFLICT_RESOLUTION_POLICY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CONFLICT_RESOLUTION_POLICY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CONSTRAINT_CONFLICT_REASON(pub i32);
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const CCR_OTHER: CONSTRAINT_CONFLICT_REASON = CONSTRAINT_CONFLICT_REASON(0i32);
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const CCR_COLLISION: CONSTRAINT_CONFLICT_REASON = CONSTRAINT_CONFLICT_REASON(1i32);
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const CCR_NOPARENT: CONSTRAINT_CONFLICT_REASON = CONSTRAINT_CONFLICT_REASON(2i32);
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const CCR_IDENTITY: CONSTRAINT_CONFLICT_REASON = CONSTRAINT_CONFLICT_REASON(3i32);
impl ::core::marker::Copy for CONSTRAINT_CONFLICT_REASON {}
impl ::core::clone::Clone for CONSTRAINT_CONFLICT_REASON {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CONSTRAINT_CONFLICT_REASON {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for CONSTRAINT_CONFLICT_REASON {
    type Abi = Self;
}
impl ::core::fmt::Debug for CONSTRAINT_CONFLICT_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CONSTRAINT_CONFLICT_REASON").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FILTERING_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const FT_CURRENT_ITEMS_ONLY: FILTERING_TYPE = FILTERING_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const FT_CURRENT_ITEMS_AND_VERSIONS_FOR_MOVED_OUT_ITEMS: FILTERING_TYPE = FILTERING_TYPE(1i32);
impl ::core::marker::Copy for FILTERING_TYPE {}
impl ::core::clone::Clone for FILTERING_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FILTERING_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FILTERING_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for FILTERING_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILTERING_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct FILTER_COMBINATION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const FCT_INTERSECTION: FILTER_COMBINATION_TYPE = FILTER_COMBINATION_TYPE(0i32);
impl ::core::marker::Copy for FILTER_COMBINATION_TYPE {}
impl ::core::clone::Clone for FILTER_COMBINATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for FILTER_COMBINATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for FILTER_COMBINATION_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for FILTER_COMBINATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FILTER_COMBINATION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct IAsynchronousDataRetriever(::windows::core::IUnknown);
impl IAsynchronousDataRetriever {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIdParameters(&self, pidparameters: *mut ID_PARAMETERS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetIdParameters)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pidparameters)).ok()
    }
    pub unsafe fn RegisterCallback<'a, P0>(&self, pdataretrievercallback: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDataRetrieverCallback>>,
    {
        (::windows::core::Interface::vtable(self).RegisterCallback)(::windows::core::Interface::as_raw(self), pdataretrievercallback.into().abi()).ok()
    }
    pub unsafe fn RevokeCallback<'a, P0>(&self, pdataretrievercallback: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IDataRetrieverCallback>>,
    {
        (::windows::core::Interface::vtable(self).RevokeCallback)(::windows::core::Interface::as_raw(self), pdataretrievercallback.into().abi()).ok()
    }
    pub unsafe fn LoadChangeData<'a, P0>(&self, ploadchangecontext: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ILoadChangeContext>>,
    {
        (::windows::core::Interface::vtable(self).LoadChangeData)(::windows::core::Interface::as_raw(self), ploadchangecontext.into().abi()).ok()
    }
}
impl ::core::convert::From<IAsynchronousDataRetriever> for ::windows::core::IUnknown {
    fn from(value: IAsynchronousDataRetriever) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IAsynchronousDataRetriever> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IAsynchronousDataRetriever) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IAsynchronousDataRetriever> for ::windows::core::IUnknown {
    fn from(value: &IAsynchronousDataRetriever) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IAsynchronousDataRetriever {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IAsynchronousDataRetriever {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IAsynchronousDataRetriever {}
impl ::core::fmt::Debug for IAsynchronousDataRetriever {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IAsynchronousDataRetriever").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IAsynchronousDataRetriever {
    type Vtable = IAsynchronousDataRetriever_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9fc7e470_61ea_4a88_9be4_df56a27cfef2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAsynchronousDataRetriever_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIdParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidparameters: *mut ID_PARAMETERS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIdParameters: usize,
    pub RegisterCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdataretrievercallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub RevokeCallback: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdataretrievercallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LoadChangeData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ploadchangecontext: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct IChangeConflict(::windows::core::IUnknown);
impl IChangeConflict {
    pub unsafe fn GetDestinationProviderConflictingChange(&self) -> ::windows::core::Result<ISyncChange> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetDestinationProviderConflictingChange)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncChange>(result__)
    }
    pub unsafe fn GetSourceProviderConflictingChange(&self) -> ::windows::core::Result<ISyncChange> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetSourceProviderConflictingChange)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncChange>(result__)
    }
    pub unsafe fn GetDestinationProviderConflictingData(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetDestinationProviderConflictingData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn GetSourceProviderConflictingData(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetSourceProviderConflictingData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn GetResolveActionForChange(&self, presolveaction: *mut SYNC_RESOLVE_ACTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetResolveActionForChange)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(presolveaction)).ok()
    }
    pub unsafe fn SetResolveActionForChange(&self, resolveaction: SYNC_RESOLVE_ACTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetResolveActionForChange)(::windows::core::Interface::as_raw(self), resolveaction).ok()
    }
    pub unsafe fn GetResolveActionForChangeUnit<'a, P0>(&self, pchangeunit: P0, presolveaction: *mut SYNC_RESOLVE_ACTION) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncChangeUnit>>,
    {
        (::windows::core::Interface::vtable(self).GetResolveActionForChangeUnit)(::windows::core::Interface::as_raw(self), pchangeunit.into().abi(), ::core::mem::transmute(presolveaction)).ok()
    }
    pub unsafe fn SetResolveActionForChangeUnit<'a, P0>(&self, pchangeunit: P0, resolveaction: SYNC_RESOLVE_ACTION) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncChangeUnit>>,
    {
        (::windows::core::Interface::vtable(self).SetResolveActionForChangeUnit)(::windows::core::Interface::as_raw(self), pchangeunit.into().abi(), resolveaction).ok()
    }
}
impl ::core::convert::From<IChangeConflict> for ::windows::core::IUnknown {
    fn from(value: IChangeConflict) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IChangeConflict> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IChangeConflict) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IChangeConflict> for ::windows::core::IUnknown {
    fn from(value: &IChangeConflict) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IChangeConflict {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IChangeConflict {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IChangeConflict {}
impl ::core::fmt::Debug for IChangeConflict {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IChangeConflict").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IChangeConflict {
    type Vtable = IChangeConflict_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x014ebf97_9f20_4f7a_bdd4_25979c77c002);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChangeConflict_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetDestinationProviderConflictingChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppconflictingchange: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSourceProviderConflictingChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppconflictingchange: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDestinationProviderConflictingData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppconflictingdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSourceProviderConflictingData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppconflictingdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetResolveActionForChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, presolveaction: *mut SYNC_RESOLVE_ACTION) -> ::windows::core::HRESULT,
    pub SetResolveActionForChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resolveaction: SYNC_RESOLVE_ACTION) -> ::windows::core::HRESULT,
    pub GetResolveActionForChangeUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchangeunit: *mut ::core::ffi::c_void, presolveaction: *mut SYNC_RESOLVE_ACTION) -> ::windows::core::HRESULT,
    pub SetResolveActionForChangeUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchangeunit: *mut ::core::ffi::c_void, resolveaction: SYNC_RESOLVE_ACTION) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct IChangeUnitException(::windows::core::IUnknown);
impl IChangeUnitException {
    pub unsafe fn GetItemId(&self, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetItemId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pcbidsize)).ok()
    }
    pub unsafe fn GetChangeUnitId(&self, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetChangeUnitId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbchangeunitid), ::core::mem::transmute(pcbidsize)).ok()
    }
    pub unsafe fn GetClockVector(&self, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetClockVector)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppunk)).ok()
    }
}
impl ::core::convert::From<IChangeUnitException> for ::windows::core::IUnknown {
    fn from(value: IChangeUnitException) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IChangeUnitException> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IChangeUnitException) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IChangeUnitException> for ::windows::core::IUnknown {
    fn from(value: &IChangeUnitException) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IChangeUnitException {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IChangeUnitException {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IChangeUnitException {}
impl ::core::fmt::Debug for IChangeUnitException {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IChangeUnitException").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IChangeUnitException {
    type Vtable = IChangeUnitException_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0cd7ee7c_fec0_4021_99ee_f0e5348f2a5f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChangeUnitException_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetItemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT,
    pub GetChangeUnitId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT,
    pub GetClockVector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct IChangeUnitListFilterInfo(::windows::core::IUnknown);
impl IChangeUnitListFilterInfo {
    pub unsafe fn Serialize(&self, pbbuffer: *mut u8, pcbbuffer: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Serialize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbbuffer), ::core::mem::transmute(pcbbuffer)).ok()
    }
    pub unsafe fn Initialize(&self, ppbchangeunitids: *const *const u8, dwchangeunitcount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppbchangeunitids), dwchangeunitcount).ok()
    }
    pub unsafe fn GetChangeUnitIdCount(&self, pdwchangeunitidcount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetChangeUnitIdCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwchangeunitidcount)).ok()
    }
    pub unsafe fn GetChangeUnitId(&self, dwchangeunitidindex: u32, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetChangeUnitId)(::windows::core::Interface::as_raw(self), dwchangeunitidindex, ::core::mem::transmute(pbchangeunitid), ::core::mem::transmute(pcbidsize)).ok()
    }
}
impl ::core::convert::From<IChangeUnitListFilterInfo> for ::windows::core::IUnknown {
    fn from(value: IChangeUnitListFilterInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IChangeUnitListFilterInfo> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IChangeUnitListFilterInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IChangeUnitListFilterInfo> for ::windows::core::IUnknown {
    fn from(value: &IChangeUnitListFilterInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IChangeUnitListFilterInfo> for ISyncFilterInfo {
    fn from(value: IChangeUnitListFilterInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IChangeUnitListFilterInfo> for &'a ISyncFilterInfo {
    fn from(value: &'a IChangeUnitListFilterInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IChangeUnitListFilterInfo> for ISyncFilterInfo {
    fn from(value: &IChangeUnitListFilterInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IChangeUnitListFilterInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IChangeUnitListFilterInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IChangeUnitListFilterInfo {}
impl ::core::fmt::Debug for IChangeUnitListFilterInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IChangeUnitListFilterInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IChangeUnitListFilterInfo {
    type Vtable = IChangeUnitListFilterInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf2837671_0bdf_43fa_b502_232375fb50c2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IChangeUnitListFilterInfo_Vtbl {
    pub base__: ISyncFilterInfo_Vtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppbchangeunitids: *const *const u8, dwchangeunitcount: u32) -> ::windows::core::HRESULT,
    pub GetChangeUnitIdCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwchangeunitidcount: *mut u32) -> ::windows::core::HRESULT,
    pub GetChangeUnitId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwchangeunitidindex: u32, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct IClockVector(::windows::core::IUnknown);
impl IClockVector {
    pub unsafe fn GetClockVectorElements(&self, riid: *const ::windows::core::GUID, ppienumclockvector: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetClockVectorElements)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppienumclockvector)).ok()
    }
    pub unsafe fn GetClockVectorElementCount(&self, pdwcount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetClockVectorElementCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwcount)).ok()
    }
}
impl ::core::convert::From<IClockVector> for ::windows::core::IUnknown {
    fn from(value: IClockVector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IClockVector> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IClockVector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IClockVector> for ::windows::core::IUnknown {
    fn from(value: &IClockVector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IClockVector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IClockVector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IClockVector {}
impl ::core::fmt::Debug for IClockVector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IClockVector").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IClockVector {
    type Vtable = IClockVector_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x14b2274a_8698_4cc6_9333_f89bd1d47bc4);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClockVector_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetClockVectorElements: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppienumclockvector: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetClockVectorElementCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct IClockVectorElement(::windows::core::IUnknown);
impl IClockVectorElement {
    pub unsafe fn GetReplicaKey(&self, pdwreplicakey: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetReplicaKey)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwreplicakey)).ok()
    }
    pub unsafe fn GetTickCount(&self, pulltickcount: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetTickCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pulltickcount)).ok()
    }
}
impl ::core::convert::From<IClockVectorElement> for ::windows::core::IUnknown {
    fn from(value: IClockVectorElement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IClockVectorElement> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IClockVectorElement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IClockVectorElement> for ::windows::core::IUnknown {
    fn from(value: &IClockVectorElement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IClockVectorElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IClockVectorElement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IClockVectorElement {}
impl ::core::fmt::Debug for IClockVectorElement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IClockVectorElement").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IClockVectorElement {
    type Vtable = IClockVectorElement_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe71c4250_adf8_4a07_8fae_5669596909c1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IClockVectorElement_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetReplicaKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwreplicakey: *mut u32) -> ::windows::core::HRESULT,
    pub GetTickCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pulltickcount: *mut u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct ICombinedFilterInfo(::windows::core::IUnknown);
impl ICombinedFilterInfo {
    pub unsafe fn Serialize(&self, pbbuffer: *mut u8, pcbbuffer: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Serialize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbbuffer), ::core::mem::transmute(pcbbuffer)).ok()
    }
    pub unsafe fn GetFilterCount(&self, pdwfiltercount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFilterCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwfiltercount)).ok()
    }
    pub unsafe fn GetFilterInfo(&self, dwfilterindex: u32) -> ::windows::core::Result<ISyncFilterInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetFilterInfo)(::windows::core::Interface::as_raw(self), dwfilterindex, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncFilterInfo>(result__)
    }
    pub unsafe fn GetFilterCombinationType(&self, pfiltercombinationtype: *mut FILTER_COMBINATION_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFilterCombinationType)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pfiltercombinationtype)).ok()
    }
}
impl ::core::convert::From<ICombinedFilterInfo> for ::windows::core::IUnknown {
    fn from(value: ICombinedFilterInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ICombinedFilterInfo> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ICombinedFilterInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICombinedFilterInfo> for ::windows::core::IUnknown {
    fn from(value: &ICombinedFilterInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ICombinedFilterInfo> for ISyncFilterInfo {
    fn from(value: ICombinedFilterInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ICombinedFilterInfo> for &'a ISyncFilterInfo {
    fn from(value: &'a ICombinedFilterInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICombinedFilterInfo> for ISyncFilterInfo {
    fn from(value: &ICombinedFilterInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ICombinedFilterInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICombinedFilterInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICombinedFilterInfo {}
impl ::core::fmt::Debug for ICombinedFilterInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICombinedFilterInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICombinedFilterInfo {
    type Vtable = ICombinedFilterInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x11f9de71_2818_4779_b2ac_42d450565f45);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICombinedFilterInfo_Vtbl {
    pub base__: ISyncFilterInfo_Vtbl,
    pub GetFilterCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwfiltercount: *mut u32) -> ::windows::core::HRESULT,
    pub GetFilterInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwfilterindex: u32, ppifilterinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetFilterCombinationType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfiltercombinationtype: *mut FILTER_COMBINATION_TYPE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct IConstraintConflict(::windows::core::IUnknown);
impl IConstraintConflict {
    pub unsafe fn GetDestinationProviderConflictingChange(&self) -> ::windows::core::Result<ISyncChange> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetDestinationProviderConflictingChange)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncChange>(result__)
    }
    pub unsafe fn GetSourceProviderConflictingChange(&self) -> ::windows::core::Result<ISyncChange> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetSourceProviderConflictingChange)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncChange>(result__)
    }
    pub unsafe fn GetDestinationProviderOriginalChange(&self) -> ::windows::core::Result<ISyncChange> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetDestinationProviderOriginalChange)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncChange>(result__)
    }
    pub unsafe fn GetDestinationProviderConflictingData(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetDestinationProviderConflictingData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn GetSourceProviderConflictingData(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetSourceProviderConflictingData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn GetDestinationProviderOriginalData(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetDestinationProviderOriginalData)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn GetConstraintResolveActionForChange(&self, pconstraintresolveaction: *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetConstraintResolveActionForChange)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pconstraintresolveaction)).ok()
    }
    pub unsafe fn SetConstraintResolveActionForChange(&self, constraintresolveaction: SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetConstraintResolveActionForChange)(::windows::core::Interface::as_raw(self), constraintresolveaction).ok()
    }
    pub unsafe fn GetConstraintResolveActionForChangeUnit<'a, P0>(&self, pchangeunit: P0, pconstraintresolveaction: *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncChangeUnit>>,
    {
        (::windows::core::Interface::vtable(self).GetConstraintResolveActionForChangeUnit)(::windows::core::Interface::as_raw(self), pchangeunit.into().abi(), ::core::mem::transmute(pconstraintresolveaction)).ok()
    }
    pub unsafe fn SetConstraintResolveActionForChangeUnit<'a, P0>(&self, pchangeunit: P0, constraintresolveaction: SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncChangeUnit>>,
    {
        (::windows::core::Interface::vtable(self).SetConstraintResolveActionForChangeUnit)(::windows::core::Interface::as_raw(self), pchangeunit.into().abi(), constraintresolveaction).ok()
    }
    pub unsafe fn GetConstraintConflictReason(&self, pconstraintconflictreason: *mut CONSTRAINT_CONFLICT_REASON) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetConstraintConflictReason)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pconstraintconflictreason)).ok()
    }
    pub unsafe fn IsTemporary(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsTemporary)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IConstraintConflict> for ::windows::core::IUnknown {
    fn from(value: IConstraintConflict) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IConstraintConflict> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IConstraintConflict) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IConstraintConflict> for ::windows::core::IUnknown {
    fn from(value: &IConstraintConflict) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IConstraintConflict {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IConstraintConflict {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IConstraintConflict {}
impl ::core::fmt::Debug for IConstraintConflict {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IConstraintConflict").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IConstraintConflict {
    type Vtable = IConstraintConflict_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x00d2302e_1cf8_4835_b85f_b7ca4f799e0a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConstraintConflict_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetDestinationProviderConflictingChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppconflictingchange: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSourceProviderConflictingChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppconflictingchange: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDestinationProviderOriginalChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pporiginalchange: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDestinationProviderConflictingData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppconflictingdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSourceProviderConflictingData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppconflictingdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetDestinationProviderOriginalData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pporiginaldata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetConstraintResolveActionForChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconstraintresolveaction: *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows::core::HRESULT,
    pub SetConstraintResolveActionForChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, constraintresolveaction: SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows::core::HRESULT,
    pub GetConstraintResolveActionForChangeUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchangeunit: *mut ::core::ffi::c_void, pconstraintresolveaction: *mut SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows::core::HRESULT,
    pub SetConstraintResolveActionForChangeUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchangeunit: *mut ::core::ffi::c_void, constraintresolveaction: SYNC_CONSTRAINT_RESOLVE_ACTION) -> ::windows::core::HRESULT,
    pub GetConstraintConflictReason: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconstraintconflictreason: *mut CONSTRAINT_CONFLICT_REASON) -> ::windows::core::HRESULT,
    pub IsTemporary: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct IConstructReplicaKeyMap(::windows::core::IUnknown);
impl IConstructReplicaKeyMap {
    pub unsafe fn FindOrAddReplica(&self, pbreplicaid: *const u8, pdwreplicakey: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FindOrAddReplica)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbreplicaid), ::core::mem::transmute(pdwreplicakey)).ok()
    }
}
impl ::core::convert::From<IConstructReplicaKeyMap> for ::windows::core::IUnknown {
    fn from(value: IConstructReplicaKeyMap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IConstructReplicaKeyMap> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IConstructReplicaKeyMap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IConstructReplicaKeyMap> for ::windows::core::IUnknown {
    fn from(value: &IConstructReplicaKeyMap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IConstructReplicaKeyMap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IConstructReplicaKeyMap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IConstructReplicaKeyMap {}
impl ::core::fmt::Debug for IConstructReplicaKeyMap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IConstructReplicaKeyMap").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IConstructReplicaKeyMap {
    type Vtable = IConstructReplicaKeyMap_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xded10970_ec85_4115_b52c_4405845642a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct IConstructReplicaKeyMap_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub FindOrAddReplica: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbreplicaid: *const u8, pdwreplicakey: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct ICoreFragment(::windows::core::IUnknown);
impl ICoreFragment {
    pub unsafe fn NextColumn(&self, pchangeunitid: *mut u8, pchangeunitidsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NextColumn)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pchangeunitid), ::core::mem::transmute(pchangeunitidsize)).ok()
    }
    pub unsafe fn NextRange(&self, pitemid: *mut u8, pitemidsize: *mut u32, piclockvector: *mut ::core::option::Option<IClockVector>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NextRange)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pitemid), ::core::mem::transmute(pitemidsize), ::core::mem::transmute(piclockvector)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetColumnCount(&self, pcolumncount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetColumnCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pcolumncount)).ok()
    }
    pub unsafe fn GetRangeCount(&self, prangecount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRangeCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(prangecount)).ok()
    }
}
impl ::core::convert::From<ICoreFragment> for ::windows::core::IUnknown {
    fn from(value: ICoreFragment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ICoreFragment> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ICoreFragment) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICoreFragment> for ::windows::core::IUnknown {
    fn from(value: &ICoreFragment) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ICoreFragment {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICoreFragment {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICoreFragment {}
impl ::core::fmt::Debug for ICoreFragment {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICoreFragment").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICoreFragment {
    type Vtable = ICoreFragment_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x613b2ab5_b304_47d9_9c31_ce6c54401a15);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreFragment_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub NextColumn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchangeunitid: *mut u8, pchangeunitidsize: *mut u32) -> ::windows::core::HRESULT,
    pub NextRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pitemid: *mut u8, pitemidsize: *mut u32, piclockvector: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetColumnCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcolumncount: *mut u32) -> ::windows::core::HRESULT,
    pub GetRangeCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prangecount: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct ICoreFragmentInspector(::windows::core::IUnknown);
impl ICoreFragmentInspector {
    pub unsafe fn NextCoreFragments(&self, requestedcount: u32, ppicorefragments: *mut ::core::option::Option<ICoreFragment>, pfetchedcount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).NextCoreFragments)(::windows::core::Interface::as_raw(self), requestedcount, ::core::mem::transmute(ppicorefragments), ::core::mem::transmute(pfetchedcount)).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<ICoreFragmentInspector> for ::windows::core::IUnknown {
    fn from(value: ICoreFragmentInspector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ICoreFragmentInspector> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ICoreFragmentInspector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICoreFragmentInspector> for ::windows::core::IUnknown {
    fn from(value: &ICoreFragmentInspector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ICoreFragmentInspector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICoreFragmentInspector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICoreFragmentInspector {}
impl ::core::fmt::Debug for ICoreFragmentInspector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICoreFragmentInspector").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICoreFragmentInspector {
    type Vtable = ICoreFragmentInspector_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf7fcc5fd_ae26_4679_ba16_96aac583c134);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreFragmentInspector_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub NextCoreFragments: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, requestedcount: u32, ppicorefragments: *mut *mut ::core::ffi::c_void, pfetchedcount: *mut u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct ICustomFilterInfo(::windows::core::IUnknown);
impl ICustomFilterInfo {
    pub unsafe fn Serialize(&self, pbbuffer: *mut u8, pcbbuffer: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Serialize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbbuffer), ::core::mem::transmute(pcbbuffer)).ok()
    }
    pub unsafe fn GetSyncFilter(&self) -> ::windows::core::Result<ISyncFilter> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetSyncFilter)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncFilter>(result__)
    }
}
impl ::core::convert::From<ICustomFilterInfo> for ::windows::core::IUnknown {
    fn from(value: ICustomFilterInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ICustomFilterInfo> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ICustomFilterInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICustomFilterInfo> for ::windows::core::IUnknown {
    fn from(value: &ICustomFilterInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ICustomFilterInfo> for ISyncFilterInfo {
    fn from(value: ICustomFilterInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ICustomFilterInfo> for &'a ISyncFilterInfo {
    fn from(value: &'a ICustomFilterInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ICustomFilterInfo> for ISyncFilterInfo {
    fn from(value: &ICustomFilterInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ICustomFilterInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ICustomFilterInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ICustomFilterInfo {}
impl ::core::fmt::Debug for ICustomFilterInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ICustomFilterInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ICustomFilterInfo {
    type Vtable = ICustomFilterInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1d335dff_6f88_4e4d_91a8_a3f351cfd473);
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomFilterInfo_Vtbl {
    pub base__: ISyncFilterInfo_Vtbl,
    pub GetSyncFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisyncfilter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ID_PARAMETERS {
    pub dwSize: u32,
    pub replicaId: ID_PARAMETER_PAIR,
    pub itemId: ID_PARAMETER_PAIR,
    pub changeUnitId: ID_PARAMETER_PAIR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ID_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ID_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ID_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ID_PARAMETERS").field("dwSize", &self.dwSize).field("replicaId", &self.replicaId).field("itemId", &self.itemId).field("changeUnitId", &self.changeUnitId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ID_PARAMETERS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ID_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ID_PARAMETERS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ID_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ID_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ID_PARAMETER_PAIR {
    pub fIsVariable: super::super::Foundation::BOOL,
    pub cbIdSize: u16,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ID_PARAMETER_PAIR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ID_PARAMETER_PAIR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ID_PARAMETER_PAIR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ID_PARAMETER_PAIR").field("fIsVariable", &self.fIsVariable).field("cbIdSize", &self.cbIdSize).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for ID_PARAMETER_PAIR {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ID_PARAMETER_PAIR {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<ID_PARAMETER_PAIR>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ID_PARAMETER_PAIR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ID_PARAMETER_PAIR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct IDataRetrieverCallback(::windows::core::IUnknown);
impl IDataRetrieverCallback {
    pub unsafe fn LoadChangeDataComplete<'a, P0>(&self, punkdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).LoadChangeDataComplete)(::windows::core::Interface::as_raw(self), punkdata.into().abi()).ok()
    }
    pub unsafe fn LoadChangeDataError(&self, hrerror: ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LoadChangeDataError)(::windows::core::Interface::as_raw(self), hrerror).ok()
    }
}
impl ::core::convert::From<IDataRetrieverCallback> for ::windows::core::IUnknown {
    fn from(value: IDataRetrieverCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IDataRetrieverCallback> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IDataRetrieverCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDataRetrieverCallback> for ::windows::core::IUnknown {
    fn from(value: &IDataRetrieverCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IDataRetrieverCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDataRetrieverCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDataRetrieverCallback {}
impl ::core::fmt::Debug for IDataRetrieverCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDataRetrieverCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDataRetrieverCallback {
    type Vtable = IDataRetrieverCallback_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x71b4863b_f969_4676_bbc3_3d9fdc3fb2c7);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDataRetrieverCallback_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub LoadChangeDataComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkdata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub LoadChangeDataError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrerror: ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct IEnumChangeUnitExceptions(::windows::core::IUnknown);
impl IEnumChangeUnitExceptions {
    pub unsafe fn Next(&self, cexceptions: u32, ppchangeunitexception: *mut ::core::option::Option<IChangeUnitException>, pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), cexceptions, ::core::mem::transmute(ppchangeunitexception), ::core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Skip(&self, cexceptions: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), cexceptions).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumChangeUnitExceptions> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumChangeUnitExceptions>(result__)
    }
}
impl ::core::convert::From<IEnumChangeUnitExceptions> for ::windows::core::IUnknown {
    fn from(value: IEnumChangeUnitExceptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEnumChangeUnitExceptions> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEnumChangeUnitExceptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumChangeUnitExceptions> for ::windows::core::IUnknown {
    fn from(value: &IEnumChangeUnitExceptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IEnumChangeUnitExceptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumChangeUnitExceptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumChangeUnitExceptions {}
impl ::core::fmt::Debug for IEnumChangeUnitExceptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumChangeUnitExceptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumChangeUnitExceptions {
    type Vtable = IEnumChangeUnitExceptions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3074e802_9319_4420_be21_1022e2e21da8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumChangeUnitExceptions_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cexceptions: u32, ppchangeunitexception: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cexceptions: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct IEnumClockVector(::windows::core::IUnknown);
impl IEnumClockVector {
    pub unsafe fn Next(&self, cclockvectorelements: u32, ppiclockvectorelements: *mut ::core::option::Option<IClockVectorElement>, pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), cclockvectorelements, ::core::mem::transmute(ppiclockvectorelements), ::core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Skip(&self, csyncversions: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), csyncversions).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumClockVector> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumClockVector>(result__)
    }
}
impl ::core::convert::From<IEnumClockVector> for ::windows::core::IUnknown {
    fn from(value: IEnumClockVector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEnumClockVector> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEnumClockVector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumClockVector> for ::windows::core::IUnknown {
    fn from(value: &IEnumClockVector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IEnumClockVector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumClockVector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumClockVector {}
impl ::core::fmt::Debug for IEnumClockVector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumClockVector").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumClockVector {
    type Vtable = IEnumClockVector_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x525844db_2837_4799_9e80_81a66e02220c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumClockVector_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cclockvectorelements: u32, ppiclockvectorelements: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, csyncversions: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppienum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct IEnumFeedClockVector(::windows::core::IUnknown);
impl IEnumFeedClockVector {
    pub unsafe fn Next(&self, cclockvectorelements: u32, ppiclockvectorelements: *mut ::core::option::Option<IFeedClockVectorElement>, pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), cclockvectorelements, ::core::mem::transmute(ppiclockvectorelements), ::core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Skip(&self, csyncversions: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), csyncversions).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumFeedClockVector> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumFeedClockVector>(result__)
    }
}
impl ::core::convert::From<IEnumFeedClockVector> for ::windows::core::IUnknown {
    fn from(value: IEnumFeedClockVector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEnumFeedClockVector> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEnumFeedClockVector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumFeedClockVector> for ::windows::core::IUnknown {
    fn from(value: &IEnumFeedClockVector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IEnumFeedClockVector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumFeedClockVector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumFeedClockVector {}
impl ::core::fmt::Debug for IEnumFeedClockVector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumFeedClockVector").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumFeedClockVector {
    type Vtable = IEnumFeedClockVector_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x550f763d_146a_48f6_abeb_6c88c7f70514);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumFeedClockVector_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cclockvectorelements: u32, ppiclockvectorelements: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, csyncversions: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppienum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct IEnumItemIds(::windows::core::IUnknown);
impl IEnumItemIds {
    pub unsafe fn Next(&self, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pcbitemidsize)).ok()
    }
}
impl ::core::convert::From<IEnumItemIds> for ::windows::core::IUnknown {
    fn from(value: IEnumItemIds) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEnumItemIds> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEnumItemIds) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumItemIds> for ::windows::core::IUnknown {
    fn from(value: &IEnumItemIds) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IEnumItemIds {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumItemIds {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumItemIds {}
impl ::core::fmt::Debug for IEnumItemIds {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumItemIds").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumItemIds {
    type Vtable = IEnumItemIds_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43aa3f61_4b2e_4b60_83df_b110d3e148f1);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumItemIds_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct IEnumRangeExceptions(::windows::core::IUnknown);
impl IEnumRangeExceptions {
    pub unsafe fn Next(&self, cexceptions: u32, pprangeexception: *mut ::core::option::Option<IRangeException>, pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), cexceptions, ::core::mem::transmute(pprangeexception), ::core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Skip(&self, cexceptions: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), cexceptions).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumRangeExceptions> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumRangeExceptions>(result__)
    }
}
impl ::core::convert::From<IEnumRangeExceptions> for ::windows::core::IUnknown {
    fn from(value: IEnumRangeExceptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEnumRangeExceptions> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEnumRangeExceptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumRangeExceptions> for ::windows::core::IUnknown {
    fn from(value: &IEnumRangeExceptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IEnumRangeExceptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumRangeExceptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumRangeExceptions {}
impl ::core::fmt::Debug for IEnumRangeExceptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumRangeExceptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumRangeExceptions {
    type Vtable = IEnumRangeExceptions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0944439f_ddb1_4176_b703_046ff22a2386);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumRangeExceptions_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cexceptions: u32, pprangeexception: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cexceptions: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct IEnumSingleItemExceptions(::windows::core::IUnknown);
impl IEnumSingleItemExceptions {
    pub unsafe fn Next(&self, cexceptions: u32, ppsingleitemexception: *mut ::core::option::Option<ISingleItemException>, pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), cexceptions, ::core::mem::transmute(ppsingleitemexception), ::core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Skip(&self, cexceptions: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), cexceptions).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumSingleItemExceptions> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumSingleItemExceptions>(result__)
    }
}
impl ::core::convert::From<IEnumSingleItemExceptions> for ::windows::core::IUnknown {
    fn from(value: IEnumSingleItemExceptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEnumSingleItemExceptions> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEnumSingleItemExceptions) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumSingleItemExceptions> for ::windows::core::IUnknown {
    fn from(value: &IEnumSingleItemExceptions) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IEnumSingleItemExceptions {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumSingleItemExceptions {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumSingleItemExceptions {}
impl ::core::fmt::Debug for IEnumSingleItemExceptions {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumSingleItemExceptions").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumSingleItemExceptions {
    type Vtable = IEnumSingleItemExceptions_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe563381c_1b4d_4c66_9796_c86faccdcd40);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSingleItemExceptions_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cexceptions: u32, ppsingleitemexception: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cexceptions: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct IEnumSyncChangeUnits(::windows::core::IUnknown);
impl IEnumSyncChangeUnits {
    pub unsafe fn Next(&self, cchanges: u32, ppchangeunit: *mut ::core::option::Option<ISyncChangeUnit>, pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), cchanges, ::core::mem::transmute(ppchangeunit), ::core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Skip(&self, cchanges: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), cchanges).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumSyncChangeUnits> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumSyncChangeUnits>(result__)
    }
}
impl ::core::convert::From<IEnumSyncChangeUnits> for ::windows::core::IUnknown {
    fn from(value: IEnumSyncChangeUnits) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEnumSyncChangeUnits> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEnumSyncChangeUnits) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumSyncChangeUnits> for ::windows::core::IUnknown {
    fn from(value: &IEnumSyncChangeUnits) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IEnumSyncChangeUnits {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumSyncChangeUnits {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumSyncChangeUnits {}
impl ::core::fmt::Debug for IEnumSyncChangeUnits {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumSyncChangeUnits").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumSyncChangeUnits {
    type Vtable = IEnumSyncChangeUnits_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x346b35f1_8703_4c6d_ab1a_4dbca2cff97f);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSyncChangeUnits_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchanges: u32, ppchangeunit: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchanges: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct IEnumSyncChanges(::windows::core::IUnknown);
impl IEnumSyncChanges {
    pub unsafe fn Next(&self, cchanges: u32, ppchange: *mut ::core::option::Option<ISyncChange>, pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), cchanges, ::core::mem::transmute(ppchange), ::core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Skip(&self, cchanges: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), cchanges).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumSyncChanges> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumSyncChanges>(result__)
    }
}
impl ::core::convert::From<IEnumSyncChanges> for ::windows::core::IUnknown {
    fn from(value: IEnumSyncChanges) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEnumSyncChanges> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEnumSyncChanges) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumSyncChanges> for ::windows::core::IUnknown {
    fn from(value: &IEnumSyncChanges) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IEnumSyncChanges {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumSyncChanges {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumSyncChanges {}
impl ::core::fmt::Debug for IEnumSyncChanges {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumSyncChanges").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumSyncChanges {
    type Vtable = IEnumSyncChanges_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x5f86be4a_5e78_4e32_ac1c_c24fd223ef85);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSyncChanges_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchanges: u32, ppchange: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cchanges: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct IEnumSyncProviderConfigUIInfos(::windows::core::IUnknown);
impl IEnumSyncProviderConfigUIInfos {
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Next(&self, cfactories: u32, ppsyncproviderconfiguiinfo: *mut ::core::option::Option<ISyncProviderConfigUIInfo>, pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), cfactories, ::core::mem::transmute(ppsyncproviderconfiguiinfo), ::core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Skip(&self, cfactories: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), cfactories).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumSyncProviderConfigUIInfos> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumSyncProviderConfigUIInfos>(result__)
    }
}
impl ::core::convert::From<IEnumSyncProviderConfigUIInfos> for ::windows::core::IUnknown {
    fn from(value: IEnumSyncProviderConfigUIInfos) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEnumSyncProviderConfigUIInfos> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEnumSyncProviderConfigUIInfos) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumSyncProviderConfigUIInfos> for ::windows::core::IUnknown {
    fn from(value: &IEnumSyncProviderConfigUIInfos) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IEnumSyncProviderConfigUIInfos {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumSyncProviderConfigUIInfos {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumSyncProviderConfigUIInfos {}
impl ::core::fmt::Debug for IEnumSyncProviderConfigUIInfos {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumSyncProviderConfigUIInfos").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumSyncProviderConfigUIInfos {
    type Vtable = IEnumSyncProviderConfigUIInfos_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf6be2602_17c6_4658_a2d7_68ed3330f641);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSyncProviderConfigUIInfos_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cfactories: u32, ppsyncproviderconfiguiinfo: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cfactories: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct IEnumSyncProviderInfos(::windows::core::IUnknown);
impl IEnumSyncProviderInfos {
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Next(&self, cinstances: u32, ppsyncproviderinfo: *mut ::core::option::Option<ISyncProviderInfo>, pcfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), cinstances, ::core::mem::transmute(ppsyncproviderinfo), ::core::mem::transmute(pcfetched)).ok()
    }
    pub unsafe fn Skip(&self, cinstances: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), cinstances).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumSyncProviderInfos> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumSyncProviderInfos>(result__)
    }
}
impl ::core::convert::From<IEnumSyncProviderInfos> for ::windows::core::IUnknown {
    fn from(value: IEnumSyncProviderInfos) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IEnumSyncProviderInfos> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IEnumSyncProviderInfos) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumSyncProviderInfos> for ::windows::core::IUnknown {
    fn from(value: &IEnumSyncProviderInfos) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IEnumSyncProviderInfos {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumSyncProviderInfos {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumSyncProviderInfos {}
impl ::core::fmt::Debug for IEnumSyncProviderInfos {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumSyncProviderInfos").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumSyncProviderInfos {
    type Vtable = IEnumSyncProviderInfos_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa04ba850_5eb1_460d_a973_393fcb608a11);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumSyncProviderInfos_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cinstances: u32, ppsyncproviderinfo: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Next: usize,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cinstances: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct IFeedClockVector(::windows::core::IUnknown);
impl IFeedClockVector {
    pub unsafe fn GetClockVectorElements(&self, riid: *const ::windows::core::GUID, ppienumclockvector: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetClockVectorElements)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppienumclockvector)).ok()
    }
    pub unsafe fn GetClockVectorElementCount(&self, pdwcount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetClockVectorElementCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwcount)).ok()
    }
    pub unsafe fn GetUpdateCount(&self, pdwupdatecount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetUpdateCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwupdatecount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsNoConflictsSpecified(&self, pfisnoconflictsspecified: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsNoConflictsSpecified)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pfisnoconflictsspecified)).ok()
    }
}
impl ::core::convert::From<IFeedClockVector> for ::windows::core::IUnknown {
    fn from(value: IFeedClockVector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IFeedClockVector> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IFeedClockVector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFeedClockVector> for ::windows::core::IUnknown {
    fn from(value: &IFeedClockVector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IFeedClockVector> for IClockVector {
    fn from(value: IFeedClockVector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IFeedClockVector> for &'a IClockVector {
    fn from(value: &'a IFeedClockVector) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFeedClockVector> for IClockVector {
    fn from(value: &IFeedClockVector) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IFeedClockVector {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFeedClockVector {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFeedClockVector {}
impl ::core::fmt::Debug for IFeedClockVector {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFeedClockVector").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IFeedClockVector {
    type Vtable = IFeedClockVector_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8d1d98d1_9fb8_4ec9_a553_54dd924e0f67);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFeedClockVector_Vtbl {
    pub base__: IClockVector_Vtbl,
    pub GetUpdateCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwupdatecount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub IsNoConflictsSpecified: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfisnoconflictsspecified: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsNoConflictsSpecified: usize,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct IFeedClockVectorElement(::windows::core::IUnknown);
impl IFeedClockVectorElement {
    pub unsafe fn GetReplicaKey(&self, pdwreplicakey: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetReplicaKey)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwreplicakey)).ok()
    }
    pub unsafe fn GetTickCount(&self, pulltickcount: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetTickCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pulltickcount)).ok()
    }
    pub unsafe fn GetSyncTime(&self, psynctime: *mut SYNC_TIME) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSyncTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(psynctime)).ok()
    }
    pub unsafe fn GetFlags(&self, pbflags: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFlags)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbflags)).ok()
    }
}
impl ::core::convert::From<IFeedClockVectorElement> for ::windows::core::IUnknown {
    fn from(value: IFeedClockVectorElement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IFeedClockVectorElement> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IFeedClockVectorElement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFeedClockVectorElement> for ::windows::core::IUnknown {
    fn from(value: &IFeedClockVectorElement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IFeedClockVectorElement> for IClockVectorElement {
    fn from(value: IFeedClockVectorElement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IFeedClockVectorElement> for &'a IClockVectorElement {
    fn from(value: &'a IFeedClockVectorElement) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFeedClockVectorElement> for IClockVectorElement {
    fn from(value: &IFeedClockVectorElement) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IFeedClockVectorElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFeedClockVectorElement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFeedClockVectorElement {}
impl ::core::fmt::Debug for IFeedClockVectorElement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFeedClockVectorElement").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IFeedClockVectorElement {
    type Vtable = IFeedClockVectorElement_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa40b46d2_e97b_4156_b6da_991f501b0f05);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFeedClockVectorElement_Vtbl {
    pub base__: IClockVectorElement_Vtbl,
    pub GetSyncTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psynctime: *mut SYNC_TIME) -> ::windows::core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbflags: *mut u8) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct IFilterKeyMap(::windows::core::IUnknown);
impl IFilterKeyMap {
    pub unsafe fn GetCount(&self, pdwcount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwcount)).ok()
    }
    pub unsafe fn AddFilter<'a, P0>(&self, pisyncfilter: P0, pdwfilterkey: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncFilter>>,
    {
        (::windows::core::Interface::vtable(self).AddFilter)(::windows::core::Interface::as_raw(self), pisyncfilter.into().abi(), ::core::mem::transmute(pdwfilterkey)).ok()
    }
    pub unsafe fn GetFilter(&self, dwfilterkey: u32) -> ::windows::core::Result<ISyncFilter> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetFilter)(::windows::core::Interface::as_raw(self), dwfilterkey, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncFilter>(result__)
    }
    pub unsafe fn Serialize(&self, pbfilterkeymap: *mut u8, pcbfilterkeymap: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Serialize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbfilterkeymap), ::core::mem::transmute(pcbfilterkeymap)).ok()
    }
}
impl ::core::convert::From<IFilterKeyMap> for ::windows::core::IUnknown {
    fn from(value: IFilterKeyMap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IFilterKeyMap> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IFilterKeyMap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFilterKeyMap> for ::windows::core::IUnknown {
    fn from(value: &IFilterKeyMap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IFilterKeyMap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFilterKeyMap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFilterKeyMap {}
impl ::core::fmt::Debug for IFilterKeyMap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFilterKeyMap").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IFilterKeyMap {
    type Vtable = IFilterKeyMap_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xca169652_07c6_4708_a3da_6e4eba8d2297);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFilterKeyMap_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwcount: *mut u32) -> ::windows::core::HRESULT,
    pub AddFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisyncfilter: *mut ::core::ffi::c_void, pdwfilterkey: *mut u32) -> ::windows::core::HRESULT,
    pub GetFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwfilterkey: u32, ppisyncfilter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Serialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbfilterkeymap: *mut u8, pcbfilterkeymap: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct IFilterRequestCallback(::windows::core::IUnknown);
impl IFilterRequestCallback {
    pub unsafe fn RequestFilter<'a, P0>(&self, pfilter: P0, filteringtype: FILTERING_TYPE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).RequestFilter)(::windows::core::Interface::as_raw(self), pfilter.into().abi(), filteringtype).ok()
    }
}
impl ::core::convert::From<IFilterRequestCallback> for ::windows::core::IUnknown {
    fn from(value: IFilterRequestCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IFilterRequestCallback> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IFilterRequestCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFilterRequestCallback> for ::windows::core::IUnknown {
    fn from(value: &IFilterRequestCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IFilterRequestCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFilterRequestCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFilterRequestCallback {}
impl ::core::fmt::Debug for IFilterRequestCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFilterRequestCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IFilterRequestCallback {
    type Vtable = IFilterRequestCallback_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x82df8873_6360_463a_a8a1_ede5e1a1594d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFilterRequestCallback_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub RequestFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilter: *mut ::core::ffi::c_void, filteringtype: FILTERING_TYPE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct IFilterTrackingProvider(::windows::core::IUnknown);
impl IFilterTrackingProvider {
    pub unsafe fn SpecifyTrackedFilters<'a, P0>(&self, pcallback: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFilterTrackingRequestCallback>>,
    {
        (::windows::core::Interface::vtable(self).SpecifyTrackedFilters)(::windows::core::Interface::as_raw(self), pcallback.into().abi()).ok()
    }
    pub unsafe fn AddTrackedFilter<'a, P0>(&self, pfilter: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncFilter>>,
    {
        (::windows::core::Interface::vtable(self).AddTrackedFilter)(::windows::core::Interface::as_raw(self), pfilter.into().abi()).ok()
    }
}
impl ::core::convert::From<IFilterTrackingProvider> for ::windows::core::IUnknown {
    fn from(value: IFilterTrackingProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IFilterTrackingProvider> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IFilterTrackingProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFilterTrackingProvider> for ::windows::core::IUnknown {
    fn from(value: &IFilterTrackingProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IFilterTrackingProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFilterTrackingProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFilterTrackingProvider {}
impl ::core::fmt::Debug for IFilterTrackingProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFilterTrackingProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IFilterTrackingProvider {
    type Vtable = IFilterTrackingProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x743383c0_fc4e_45ba_ad81_d9d84c7a24f8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFilterTrackingProvider_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SpecifyTrackedFilters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddTrackedFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct IFilterTrackingRequestCallback(::windows::core::IUnknown);
impl IFilterTrackingRequestCallback {
    pub unsafe fn RequestTrackedFilter<'a, P0>(&self, pfilter: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncFilter>>,
    {
        (::windows::core::Interface::vtable(self).RequestTrackedFilter)(::windows::core::Interface::as_raw(self), pfilter.into().abi()).ok()
    }
}
impl ::core::convert::From<IFilterTrackingRequestCallback> for ::windows::core::IUnknown {
    fn from(value: IFilterTrackingRequestCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IFilterTrackingRequestCallback> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IFilterTrackingRequestCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFilterTrackingRequestCallback> for ::windows::core::IUnknown {
    fn from(value: &IFilterTrackingRequestCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IFilterTrackingRequestCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFilterTrackingRequestCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFilterTrackingRequestCallback {}
impl ::core::fmt::Debug for IFilterTrackingRequestCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFilterTrackingRequestCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IFilterTrackingRequestCallback {
    type Vtable = IFilterTrackingRequestCallback_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x713ca7bb_c858_4674_b4b6_1122436587a9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFilterTrackingRequestCallback_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub RequestTrackedFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct IFilterTrackingSyncChangeBuilder(::windows::core::IUnknown);
impl IFilterTrackingSyncChangeBuilder {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AddFilterChange(&self, dwfilterkey: u32, pfilterchange: *const SYNC_FILTER_CHANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddFilterChange)(::windows::core::Interface::as_raw(self), dwfilterkey, ::core::mem::transmute(pfilterchange)).ok()
    }
    pub unsafe fn SetAllChangeUnitsPresentFlag(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAllChangeUnitsPresentFlag)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IFilterTrackingSyncChangeBuilder> for ::windows::core::IUnknown {
    fn from(value: IFilterTrackingSyncChangeBuilder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IFilterTrackingSyncChangeBuilder> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IFilterTrackingSyncChangeBuilder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFilterTrackingSyncChangeBuilder> for ::windows::core::IUnknown {
    fn from(value: &IFilterTrackingSyncChangeBuilder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IFilterTrackingSyncChangeBuilder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFilterTrackingSyncChangeBuilder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFilterTrackingSyncChangeBuilder {}
impl ::core::fmt::Debug for IFilterTrackingSyncChangeBuilder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFilterTrackingSyncChangeBuilder").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IFilterTrackingSyncChangeBuilder {
    type Vtable = IFilterTrackingSyncChangeBuilder_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x295024a0_70da_4c58_883c_ce2afb308d0b);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFilterTrackingSyncChangeBuilder_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub AddFilterChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwfilterkey: u32, pfilterchange: *const SYNC_FILTER_CHANGE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AddFilterChange: usize,
    pub SetAllChangeUnitsPresentFlag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct IForgottenKnowledge(::windows::core::IUnknown);
impl IForgottenKnowledge {
    pub unsafe fn GetOwnerReplicaId(&self, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetOwnerReplicaId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbreplicaid), ::core::mem::transmute(pcbidsize)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Serialize<'a, P0>(&self, fserializereplicakeymap: P0, pbknowledge: *mut u8, pcbknowledge: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.Serialize)(::windows::core::Interface::as_raw(self), fserializereplicakeymap.into(), ::core::mem::transmute(pbknowledge), ::core::mem::transmute(pcbknowledge)).ok()
    }
    pub unsafe fn SetLocalTickCount(&self, ulltickcount: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetLocalTickCount)(::windows::core::Interface::as_raw(self), ulltickcount).ok()
    }
    pub unsafe fn ContainsChange(&self, pbversionownerreplicaid: *const u8, pgiditemid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ContainsChange)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbversionownerreplicaid), ::core::mem::transmute(pgiditemid), ::core::mem::transmute(psyncversion)).ok()
    }
    pub unsafe fn ContainsChangeUnit(&self, pbversionownerreplicaid: *const u8, pbitemid: *const u8, pbchangeunitid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ContainsChangeUnit)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbversionownerreplicaid), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pbchangeunitid), ::core::mem::transmute(psyncversion)).ok()
    }
    pub unsafe fn GetScopeVector(&self, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetScopeVector)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppunk)).ok()
    }
    pub unsafe fn GetReplicaKeyMap(&self) -> ::windows::core::Result<IReplicaKeyMap> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetReplicaKeyMap)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IReplicaKeyMap>(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.Clone)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn ConvertVersion<'a, P0>(&self, pknowledgein: P0, pbcurrentownerid: *const u8, pversionin: *const SYNC_VERSION, pbnewownerid: *mut u8, pcbidsize: *mut u32, pversionout: *mut SYNC_VERSION) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncKnowledge>>,
    {
        (::windows::core::Interface::vtable(self).base__.ConvertVersion)(::windows::core::Interface::as_raw(self), pknowledgein.into().abi(), ::core::mem::transmute(pbcurrentownerid), ::core::mem::transmute(pversionin), ::core::mem::transmute(pbnewownerid), ::core::mem::transmute(pcbidsize), ::core::mem::transmute(pversionout)).ok()
    }
    pub unsafe fn MapRemoteToLocal<'a, P0>(&self, premoteknowledge: P0) -> ::windows::core::Result<ISyncKnowledge>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncKnowledge>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.MapRemoteToLocal)(::windows::core::Interface::as_raw(self), premoteknowledge.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn Union<'a, P0>(&self, pknowledge: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncKnowledge>>,
    {
        (::windows::core::Interface::vtable(self).base__.Union)(::windows::core::Interface::as_raw(self), pknowledge.into().abi()).ok()
    }
    pub unsafe fn ProjectOntoItem(&self, pbitemid: *const u8) -> ::windows::core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.ProjectOntoItem)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbitemid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn ProjectOntoChangeUnit(&self, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows::core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.ProjectOntoChangeUnit)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pbchangeunitid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn ProjectOntoRange(&self, psrngsyncrange: *const SYNC_RANGE) -> ::windows::core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.ProjectOntoRange)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(psrngsyncrange), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn ExcludeItem(&self, pbitemid: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ExcludeItem)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbitemid)).ok()
    }
    pub unsafe fn ExcludeChangeUnit(&self, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ExcludeChangeUnit)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pbchangeunitid)).ok()
    }
    pub unsafe fn ContainsKnowledge<'a, P0>(&self, pknowledge: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncKnowledge>>,
    {
        (::windows::core::Interface::vtable(self).base__.ContainsKnowledge)(::windows::core::Interface::as_raw(self), pknowledge.into().abi()).ok()
    }
    pub unsafe fn FindMinTickCountForReplica(&self, pbreplicaid: *const u8, pullreplicatickcount: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.FindMinTickCountForReplica)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbreplicaid), ::core::mem::transmute(pullreplicatickcount)).ok()
    }
    pub unsafe fn GetRangeExceptions(&self, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetRangeExceptions)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppunk)).ok()
    }
    pub unsafe fn GetSingleItemExceptions(&self, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetSingleItemExceptions)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppunk)).ok()
    }
    pub unsafe fn GetChangeUnitExceptions(&self, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetChangeUnitExceptions)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppunk)).ok()
    }
    pub unsafe fn FindClockVectorForItem(&self, pbitemid: *const u8, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.FindClockVectorForItem)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbitemid), ::core::mem::transmute(riid), ::core::mem::transmute(ppunk)).ok()
    }
    pub unsafe fn FindClockVectorForChangeUnit(&self, pbitemid: *const u8, pbchangeunitid: *const u8, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.FindClockVectorForChangeUnit)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pbchangeunitid), ::core::mem::transmute(riid), ::core::mem::transmute(ppunk)).ok()
    }
    pub unsafe fn GetVersion(&self, pdwversion: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetVersion)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwversion)).ok()
    }
    pub unsafe fn ForgetToVersion<'a, P0>(&self, pknowledge: P0, pversion: *const SYNC_VERSION) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncKnowledge>>,
    {
        (::windows::core::Interface::vtable(self).ForgetToVersion)(::windows::core::Interface::as_raw(self), pknowledge.into().abi(), ::core::mem::transmute(pversion)).ok()
    }
}
impl ::core::convert::From<IForgottenKnowledge> for ::windows::core::IUnknown {
    fn from(value: IForgottenKnowledge) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IForgottenKnowledge> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IForgottenKnowledge) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IForgottenKnowledge> for ::windows::core::IUnknown {
    fn from(value: &IForgottenKnowledge) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IForgottenKnowledge> for ISyncKnowledge {
    fn from(value: IForgottenKnowledge) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IForgottenKnowledge> for &'a ISyncKnowledge {
    fn from(value: &'a IForgottenKnowledge) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IForgottenKnowledge> for ISyncKnowledge {
    fn from(value: &IForgottenKnowledge) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IForgottenKnowledge {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IForgottenKnowledge {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IForgottenKnowledge {}
impl ::core::fmt::Debug for IForgottenKnowledge {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IForgottenKnowledge").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IForgottenKnowledge {
    type Vtable = IForgottenKnowledge_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x456e0f96_6036_452b_9f9d_bcc4b4a85db2);
}
#[repr(C)]
#[doc(hidden)]
pub struct IForgottenKnowledge_Vtbl {
    pub base__: ISyncKnowledge_Vtbl,
    pub ForgetToVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pknowledge: *mut ::core::ffi::c_void, pversion: *const SYNC_VERSION) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct IKnowledgeSyncProvider(::windows::core::IUnknown);
impl IKnowledgeSyncProvider {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIdParameters(&self, pidparameters: *mut ID_PARAMETERS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetIdParameters)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pidparameters)).ok()
    }
    pub unsafe fn BeginSession<'a, P0>(&self, role: SYNC_PROVIDER_ROLE, psessionstate: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncSessionState>>,
    {
        (::windows::core::Interface::vtable(self).BeginSession)(::windows::core::Interface::as_raw(self), role, psessionstate.into().abi()).ok()
    }
    pub unsafe fn GetSyncBatchParameters(&self, ppsyncknowledge: *mut ::core::option::Option<ISyncKnowledge>, pdwrequestedbatchsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSyncBatchParameters)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppsyncknowledge), ::core::mem::transmute(pdwrequestedbatchsize)).ok()
    }
    pub unsafe fn GetChangeBatch<'a, P0>(&self, dwbatchsize: u32, psyncknowledge: P0, ppsyncchangebatch: *mut ::core::option::Option<ISyncChangeBatch>, ppunkdataretriever: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncKnowledge>>,
    {
        (::windows::core::Interface::vtable(self).GetChangeBatch)(::windows::core::Interface::as_raw(self), dwbatchsize, psyncknowledge.into().abi(), ::core::mem::transmute(ppsyncchangebatch), ::core::mem::transmute(ppunkdataretriever)).ok()
    }
    pub unsafe fn GetFullEnumerationChangeBatch<'a, P0>(&self, dwbatchsize: u32, pblowerenumerationbound: *const u8, psyncknowledge: P0, ppsyncchangebatch: *mut ::core::option::Option<ISyncFullEnumerationChangeBatch>, ppunkdataretriever: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncKnowledge>>,
    {
        (::windows::core::Interface::vtable(self).GetFullEnumerationChangeBatch)(::windows::core::Interface::as_raw(self), dwbatchsize, ::core::mem::transmute(pblowerenumerationbound), psyncknowledge.into().abi(), ::core::mem::transmute(ppsyncchangebatch), ::core::mem::transmute(ppunkdataretriever)).ok()
    }
    pub unsafe fn ProcessChangeBatch<'a, P0, P1, P2>(&self, resolutionpolicy: CONFLICT_RESOLUTION_POLICY, psourcechangebatch: P0, punkdataretriever: P1, pcallback: P2, psyncsessionstatistics: *mut SYNC_SESSION_STATISTICS) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncChangeBatch>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
        P2: ::std::convert::Into<::windows::core::InParam<'a, ISyncCallback>>,
    {
        (::windows::core::Interface::vtable(self).ProcessChangeBatch)(::windows::core::Interface::as_raw(self), resolutionpolicy, psourcechangebatch.into().abi(), punkdataretriever.into().abi(), pcallback.into().abi(), ::core::mem::transmute(psyncsessionstatistics)).ok()
    }
    pub unsafe fn ProcessFullEnumerationChangeBatch<'a, P0, P1, P2>(&self, resolutionpolicy: CONFLICT_RESOLUTION_POLICY, psourcechangebatch: P0, punkdataretriever: P1, pcallback: P2, psyncsessionstatistics: *mut SYNC_SESSION_STATISTICS) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncFullEnumerationChangeBatch>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
        P2: ::std::convert::Into<::windows::core::InParam<'a, ISyncCallback>>,
    {
        (::windows::core::Interface::vtable(self).ProcessFullEnumerationChangeBatch)(::windows::core::Interface::as_raw(self), resolutionpolicy, psourcechangebatch.into().abi(), punkdataretriever.into().abi(), pcallback.into().abi(), ::core::mem::transmute(psyncsessionstatistics)).ok()
    }
    pub unsafe fn EndSession<'a, P0>(&self, psessionstate: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncSessionState>>,
    {
        (::windows::core::Interface::vtable(self).EndSession)(::windows::core::Interface::as_raw(self), psessionstate.into().abi()).ok()
    }
}
impl ::core::convert::From<IKnowledgeSyncProvider> for ::windows::core::IUnknown {
    fn from(value: IKnowledgeSyncProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IKnowledgeSyncProvider> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IKnowledgeSyncProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IKnowledgeSyncProvider> for ::windows::core::IUnknown {
    fn from(value: &IKnowledgeSyncProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<IKnowledgeSyncProvider> for ISyncProvider {
    fn from(value: IKnowledgeSyncProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IKnowledgeSyncProvider> for &'a ISyncProvider {
    fn from(value: &'a IKnowledgeSyncProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IKnowledgeSyncProvider> for ISyncProvider {
    fn from(value: &IKnowledgeSyncProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IKnowledgeSyncProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IKnowledgeSyncProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IKnowledgeSyncProvider {}
impl ::core::fmt::Debug for IKnowledgeSyncProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKnowledgeSyncProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IKnowledgeSyncProvider {
    type Vtable = IKnowledgeSyncProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x43434a49_8da4_47f2_8172_ad7b8b024978);
}
#[repr(C)]
#[doc(hidden)]
pub struct IKnowledgeSyncProvider_Vtbl {
    pub base__: ISyncProvider_Vtbl,
    pub BeginSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, role: SYNC_PROVIDER_ROLE, psessionstate: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSyncBatchParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsyncknowledge: *mut *mut ::core::ffi::c_void, pdwrequestedbatchsize: *mut u32) -> ::windows::core::HRESULT,
    pub GetChangeBatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwbatchsize: u32, psyncknowledge: *mut ::core::ffi::c_void, ppsyncchangebatch: *mut *mut ::core::ffi::c_void, ppunkdataretriever: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetFullEnumerationChangeBatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwbatchsize: u32, pblowerenumerationbound: *const u8, psyncknowledge: *mut ::core::ffi::c_void, ppsyncchangebatch: *mut *mut ::core::ffi::c_void, ppunkdataretriever: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ProcessChangeBatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resolutionpolicy: CONFLICT_RESOLUTION_POLICY, psourcechangebatch: *mut ::core::ffi::c_void, punkdataretriever: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, psyncsessionstatistics: *mut SYNC_SESSION_STATISTICS) -> ::windows::core::HRESULT,
    pub ProcessFullEnumerationChangeBatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, resolutionpolicy: CONFLICT_RESOLUTION_POLICY, psourcechangebatch: *mut ::core::ffi::c_void, punkdataretriever: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void, psyncsessionstatistics: *mut SYNC_SESSION_STATISTICS) -> ::windows::core::HRESULT,
    pub EndSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psessionstate: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct ILoadChangeContext(::windows::core::IUnknown);
impl ILoadChangeContext {
    pub unsafe fn GetSyncChange(&self) -> ::windows::core::Result<ISyncChange> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetSyncChange)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncChange>(result__)
    }
    pub unsafe fn SetRecoverableErrorOnChange<'a, P0>(&self, hrerror: ::windows::core::HRESULT, perrordata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IRecoverableErrorData>>,
    {
        (::windows::core::Interface::vtable(self).SetRecoverableErrorOnChange)(::windows::core::Interface::as_raw(self), hrerror, perrordata.into().abi()).ok()
    }
    pub unsafe fn SetRecoverableErrorOnChangeUnit<'a, P0, P1>(&self, hrerror: ::windows::core::HRESULT, pchangeunit: P0, perrordata: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncChangeUnit>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IRecoverableErrorData>>,
    {
        (::windows::core::Interface::vtable(self).SetRecoverableErrorOnChangeUnit)(::windows::core::Interface::as_raw(self), hrerror, pchangeunit.into().abi(), perrordata.into().abi()).ok()
    }
}
impl ::core::convert::From<ILoadChangeContext> for ::windows::core::IUnknown {
    fn from(value: ILoadChangeContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ILoadChangeContext> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ILoadChangeContext) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ILoadChangeContext> for ::windows::core::IUnknown {
    fn from(value: &ILoadChangeContext) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ILoadChangeContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ILoadChangeContext {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ILoadChangeContext {}
impl ::core::fmt::Debug for ILoadChangeContext {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ILoadChangeContext").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ILoadChangeContext {
    type Vtable = ILoadChangeContext_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x44a4aaca_ec39_46d5_b5c9_d633c0ee67e2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ILoadChangeContext_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetSyncChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsyncchange: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetRecoverableErrorOnChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrerror: ::windows::core::HRESULT, perrordata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetRecoverableErrorOnChangeUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hrerror: ::windows::core::HRESULT, pchangeunit: *mut ::core::ffi::c_void, perrordata: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct IProviderConverter(::windows::core::IUnknown);
impl IProviderConverter {
    pub unsafe fn Initialize<'a, P0>(&self, pisyncprovider: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncProvider>>,
    {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), pisyncprovider.into().abi()).ok()
    }
}
impl ::core::convert::From<IProviderConverter> for ::windows::core::IUnknown {
    fn from(value: IProviderConverter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IProviderConverter> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IProviderConverter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IProviderConverter> for ::windows::core::IUnknown {
    fn from(value: &IProviderConverter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IProviderConverter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IProviderConverter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IProviderConverter {}
impl ::core::fmt::Debug for IProviderConverter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IProviderConverter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IProviderConverter {
    type Vtable = IProviderConverter_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x809b7276_98cf_4957_93a5_0ebdd3dddffd);
}
#[repr(C)]
#[doc(hidden)]
pub struct IProviderConverter_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisyncprovider: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct IRangeException(::windows::core::IUnknown);
impl IRangeException {
    pub unsafe fn GetClosedRangeStart(&self, pbclosedrangestart: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetClosedRangeStart)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbclosedrangestart), ::core::mem::transmute(pcbidsize)).ok()
    }
    pub unsafe fn GetClosedRangeEnd(&self, pbclosedrangeend: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetClosedRangeEnd)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbclosedrangeend), ::core::mem::transmute(pcbidsize)).ok()
    }
    pub unsafe fn GetClockVector(&self, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetClockVector)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppunk)).ok()
    }
}
impl ::core::convert::From<IRangeException> for ::windows::core::IUnknown {
    fn from(value: IRangeException) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IRangeException> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRangeException) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRangeException> for ::windows::core::IUnknown {
    fn from(value: &IRangeException) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IRangeException {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRangeException {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRangeException {}
impl ::core::fmt::Debug for IRangeException {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRangeException").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRangeException {
    type Vtable = IRangeException_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x75ae8777_6848_49f7_956c_a3a92f5096e8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRangeException_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetClosedRangeStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbclosedrangestart: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT,
    pub GetClosedRangeEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbclosedrangeend: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT,
    pub GetClockVector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct IRecoverableError(::windows::core::IUnknown);
impl IRecoverableError {
    pub unsafe fn GetStage(&self, pstage: *mut SYNC_PROGRESS_STAGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStage)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pstage)).ok()
    }
    pub unsafe fn GetProvider(&self, pproviderrole: *mut SYNC_PROVIDER_ROLE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetProvider)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pproviderrole)).ok()
    }
    pub unsafe fn GetChangeWithRecoverableError(&self) -> ::windows::core::Result<ISyncChange> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetChangeWithRecoverableError)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncChange>(result__)
    }
    pub unsafe fn GetRecoverableErrorDataForChange(&self, phrerror: *mut ::windows::core::HRESULT, pperrordata: *mut ::core::option::Option<IRecoverableErrorData>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRecoverableErrorDataForChange)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(phrerror), ::core::mem::transmute(pperrordata)).ok()
    }
    pub unsafe fn GetRecoverableErrorDataForChangeUnit<'a, P0>(&self, pchangeunit: P0, phrerror: *mut ::windows::core::HRESULT, pperrordata: *mut ::core::option::Option<IRecoverableErrorData>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncChangeUnit>>,
    {
        (::windows::core::Interface::vtable(self).GetRecoverableErrorDataForChangeUnit)(::windows::core::Interface::as_raw(self), pchangeunit.into().abi(), ::core::mem::transmute(phrerror), ::core::mem::transmute(pperrordata)).ok()
    }
}
impl ::core::convert::From<IRecoverableError> for ::windows::core::IUnknown {
    fn from(value: IRecoverableError) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IRecoverableError> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRecoverableError) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRecoverableError> for ::windows::core::IUnknown {
    fn from(value: &IRecoverableError) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IRecoverableError {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRecoverableError {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRecoverableError {}
impl ::core::fmt::Debug for IRecoverableError {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRecoverableError").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRecoverableError {
    type Vtable = IRecoverableError_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f5625e8_0a7b_45ee_9637_1ce13645909e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRecoverableError_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetStage: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstage: *mut SYNC_PROGRESS_STAGE) -> ::windows::core::HRESULT,
    pub GetProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproviderrole: *mut SYNC_PROVIDER_ROLE) -> ::windows::core::HRESULT,
    pub GetChangeWithRecoverableError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppchangewithrecoverableerror: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetRecoverableErrorDataForChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phrerror: *mut ::windows::core::HRESULT, pperrordata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetRecoverableErrorDataForChangeUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pchangeunit: *mut ::core::ffi::c_void, phrerror: *mut ::windows::core::HRESULT, pperrordata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct IRecoverableErrorData(::windows::core::IUnknown);
impl IRecoverableErrorData {
    pub unsafe fn Initialize<'a, P0, P1>(&self, pcszitemdisplayname: P0, pcszerrordescription: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
        P1: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).Initialize)(::windows::core::Interface::as_raw(self), pcszitemdisplayname.into(), pcszerrordescription.into()).ok()
    }
    pub unsafe fn GetItemDisplayName<'a, P0>(&self, pszitemdisplayname: P0, pcchitemdisplayname: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetItemDisplayName)(::windows::core::Interface::as_raw(self), pszitemdisplayname.into(), ::core::mem::transmute(pcchitemdisplayname)).ok()
    }
    pub unsafe fn GetErrorDescription<'a, P0>(&self, pszerrordescription: P0, pccherrordescription: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::PCWSTR>,
    {
        (::windows::core::Interface::vtable(self).GetErrorDescription)(::windows::core::Interface::as_raw(self), pszerrordescription.into(), ::core::mem::transmute(pccherrordescription)).ok()
    }
}
impl ::core::convert::From<IRecoverableErrorData> for ::windows::core::IUnknown {
    fn from(value: IRecoverableErrorData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IRecoverableErrorData> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRecoverableErrorData) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRecoverableErrorData> for ::windows::core::IUnknown {
    fn from(value: &IRecoverableErrorData) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IRecoverableErrorData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRecoverableErrorData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRecoverableErrorData {}
impl ::core::fmt::Debug for IRecoverableErrorData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRecoverableErrorData").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRecoverableErrorData {
    type Vtable = IRecoverableErrorData_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb37c4a0a_4b7d_4c2d_9711_3b00d119b1c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRecoverableErrorData_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcszitemdisplayname: ::windows::core::PCWSTR, pcszerrordescription: ::windows::core::PCWSTR) -> ::windows::core::HRESULT,
    pub GetItemDisplayName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszitemdisplayname: ::windows::core::PCWSTR, pcchitemdisplayname: *mut u32) -> ::windows::core::HRESULT,
    pub GetErrorDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pszerrordescription: ::windows::core::PCWSTR, pccherrordescription: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct IRegisteredSyncProvider(::windows::core::IUnknown);
impl IRegisteredSyncProvider {
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Init<'a, P0>(&self, pguidinstanceid: *const ::windows::core::GUID, pguidcontenttype: *const ::windows::core::GUID, pcontextpropertystore: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::UI::Shell::PropertiesSystem::IPropertyStore>>,
    {
        (::windows::core::Interface::vtable(self).Init)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pguidinstanceid), ::core::mem::transmute(pguidcontenttype), pcontextpropertystore.into().abi()).ok()
    }
    pub unsafe fn GetInstanceId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetInstanceId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
}
impl ::core::convert::From<IRegisteredSyncProvider> for ::windows::core::IUnknown {
    fn from(value: IRegisteredSyncProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IRegisteredSyncProvider> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRegisteredSyncProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRegisteredSyncProvider> for ::windows::core::IUnknown {
    fn from(value: &IRegisteredSyncProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IRegisteredSyncProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRegisteredSyncProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRegisteredSyncProvider {}
impl ::core::fmt::Debug for IRegisteredSyncProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRegisteredSyncProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRegisteredSyncProvider {
    type Vtable = IRegisteredSyncProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x913bcf76_47c1_40b5_a896_5e8a9c414c14);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRegisteredSyncProvider_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Init: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID, pguidcontenttype: *const ::windows::core::GUID, pcontextpropertystore: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Init: usize,
    pub GetInstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidinstanceid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct IReplicaKeyMap(::windows::core::IUnknown);
impl IReplicaKeyMap {
    pub unsafe fn LookupReplicaKey(&self, pbreplicaid: *const u8, pdwreplicakey: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LookupReplicaKey)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbreplicaid), ::core::mem::transmute(pdwreplicakey)).ok()
    }
    pub unsafe fn LookupReplicaId(&self, dwreplicakey: u32, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LookupReplicaId)(::windows::core::Interface::as_raw(self), dwreplicakey, ::core::mem::transmute(pbreplicaid), ::core::mem::transmute(pcbidsize)).ok()
    }
    pub unsafe fn Serialize(&self, pbreplicakeymap: *mut u8, pcbreplicakeymap: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Serialize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbreplicakeymap), ::core::mem::transmute(pcbreplicakeymap)).ok()
    }
}
impl ::core::convert::From<IReplicaKeyMap> for ::windows::core::IUnknown {
    fn from(value: IReplicaKeyMap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IReplicaKeyMap> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IReplicaKeyMap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IReplicaKeyMap> for ::windows::core::IUnknown {
    fn from(value: &IReplicaKeyMap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IReplicaKeyMap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IReplicaKeyMap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IReplicaKeyMap {}
impl ::core::fmt::Debug for IReplicaKeyMap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IReplicaKeyMap").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IReplicaKeyMap {
    type Vtable = IReplicaKeyMap_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2209f4fc_fd10_4ff0_84a8_f0a1982e440e);
}
#[repr(C)]
#[doc(hidden)]
pub struct IReplicaKeyMap_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub LookupReplicaKey: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbreplicaid: *const u8, pdwreplicakey: *mut u32) -> ::windows::core::HRESULT,
    pub LookupReplicaId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwreplicakey: u32, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT,
    pub Serialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbreplicakeymap: *mut u8, pcbreplicakeymap: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct IRequestFilteredSync(::windows::core::IUnknown);
impl IRequestFilteredSync {
    pub unsafe fn SpecifyFilter<'a, P0>(&self, pcallback: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFilterRequestCallback>>,
    {
        (::windows::core::Interface::vtable(self).SpecifyFilter)(::windows::core::Interface::as_raw(self), pcallback.into().abi()).ok()
    }
}
impl ::core::convert::From<IRequestFilteredSync> for ::windows::core::IUnknown {
    fn from(value: IRequestFilteredSync) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a IRequestFilteredSync> for &'a ::windows::core::IUnknown {
    fn from(value: &'a IRequestFilteredSync) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IRequestFilteredSync> for ::windows::core::IUnknown {
    fn from(value: &IRequestFilteredSync) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for IRequestFilteredSync {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IRequestFilteredSync {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IRequestFilteredSync {}
impl ::core::fmt::Debug for IRequestFilteredSync {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IRequestFilteredSync").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IRequestFilteredSync {
    type Vtable = IRequestFilteredSync_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x2e020184_6d18_46a7_a32a_da4aeb06696c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IRequestFilteredSync_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub SpecifyFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcallback: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct ISingleItemException(::windows::core::IUnknown);
impl ISingleItemException {
    pub unsafe fn GetItemId(&self, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetItemId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pcbidsize)).ok()
    }
    pub unsafe fn GetClockVector(&self, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetClockVector)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppunk)).ok()
    }
}
impl ::core::convert::From<ISingleItemException> for ::windows::core::IUnknown {
    fn from(value: ISingleItemException) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISingleItemException> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISingleItemException) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISingleItemException> for ::windows::core::IUnknown {
    fn from(value: &ISingleItemException) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISingleItemException {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISingleItemException {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISingleItemException {}
impl ::core::fmt::Debug for ISingleItemException {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISingleItemException").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISingleItemException {
    type Vtable = ISingleItemException_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x892fb9b0_7c55_4a18_9316_fdf449569b64);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISingleItemException_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetItemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT,
    pub GetClockVector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct ISupportFilteredSync(::windows::core::IUnknown);
impl ISupportFilteredSync {
    pub unsafe fn AddFilter<'a, P0>(&self, pfilter: P0, filteringtype: FILTERING_TYPE) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).AddFilter)(::windows::core::Interface::as_raw(self), pfilter.into().abi(), filteringtype).ok()
    }
}
impl ::core::convert::From<ISupportFilteredSync> for ::windows::core::IUnknown {
    fn from(value: ISupportFilteredSync) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISupportFilteredSync> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISupportFilteredSync) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISupportFilteredSync> for ::windows::core::IUnknown {
    fn from(value: &ISupportFilteredSync) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISupportFilteredSync {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISupportFilteredSync {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISupportFilteredSync {}
impl ::core::fmt::Debug for ISupportFilteredSync {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISupportFilteredSync").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISupportFilteredSync {
    type Vtable = ISupportFilteredSync_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x3d128ded_d555_4e0d_bf4b_fb213a8a9302);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISupportFilteredSync_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub AddFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilter: *mut ::core::ffi::c_void, filteringtype: FILTERING_TYPE) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct ISupportLastWriteTime(::windows::core::IUnknown);
impl ISupportLastWriteTime {
    pub unsafe fn GetItemChangeTime(&self, pbitemid: *const u8, pulltimestamp: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetItemChangeTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pulltimestamp)).ok()
    }
    pub unsafe fn GetChangeUnitChangeTime(&self, pbitemid: *const u8, pbchangeunitid: *const u8, pulltimestamp: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetChangeUnitChangeTime)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pbchangeunitid), ::core::mem::transmute(pulltimestamp)).ok()
    }
}
impl ::core::convert::From<ISupportLastWriteTime> for ::windows::core::IUnknown {
    fn from(value: ISupportLastWriteTime) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISupportLastWriteTime> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISupportLastWriteTime) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISupportLastWriteTime> for ::windows::core::IUnknown {
    fn from(value: &ISupportLastWriteTime) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISupportLastWriteTime {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISupportLastWriteTime {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISupportLastWriteTime {}
impl ::core::fmt::Debug for ISupportLastWriteTime {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISupportLastWriteTime").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISupportLastWriteTime {
    type Vtable = ISupportLastWriteTime_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeadf816f_d0bd_43ca_8f40_5acdc6c06f7a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISupportLastWriteTime_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetItemChangeTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbitemid: *const u8, pulltimestamp: *mut u64) -> ::windows::core::HRESULT,
    pub GetChangeUnitChangeTime: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8, pulltimestamp: *mut u64) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct ISyncCallback(::windows::core::IUnknown);
impl ISyncCallback {
    pub unsafe fn OnProgress(&self, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnProgress)(::windows::core::Interface::as_raw(self), provider, syncstage, dwcompletedwork, dwtotalwork).ok()
    }
    pub unsafe fn OnChange<'a, P0>(&self, psyncchange: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncChange>>,
    {
        (::windows::core::Interface::vtable(self).OnChange)(::windows::core::Interface::as_raw(self), psyncchange.into().abi()).ok()
    }
    pub unsafe fn OnConflict<'a, P0>(&self, pconflict: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IChangeConflict>>,
    {
        (::windows::core::Interface::vtable(self).OnConflict)(::windows::core::Interface::as_raw(self), pconflict.into().abi()).ok()
    }
    pub unsafe fn OnFullEnumerationNeeded(&self, pfullenumerationaction: *mut SYNC_FULL_ENUMERATION_ACTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnFullEnumerationNeeded)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pfullenumerationaction)).ok()
    }
    pub unsafe fn OnRecoverableError<'a, P0>(&self, precoverableerror: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IRecoverableError>>,
    {
        (::windows::core::Interface::vtable(self).OnRecoverableError)(::windows::core::Interface::as_raw(self), precoverableerror.into().abi()).ok()
    }
}
impl ::core::convert::From<ISyncCallback> for ::windows::core::IUnknown {
    fn from(value: ISyncCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISyncCallback> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISyncCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncCallback> for ::windows::core::IUnknown {
    fn from(value: &ISyncCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISyncCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncCallback {}
impl ::core::fmt::Debug for ISyncCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISyncCallback {
    type Vtable = ISyncCallback_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0599797f_5ed9_485c_ae36_0c5d1bf2e7a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncCallback_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub OnProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> ::windows::core::HRESULT,
    pub OnChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psyncchange: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnConflict: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconflict: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnFullEnumerationNeeded: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfullenumerationaction: *mut SYNC_FULL_ENUMERATION_ACTION) -> ::windows::core::HRESULT,
    pub OnRecoverableError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, precoverableerror: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct ISyncCallback2(::windows::core::IUnknown);
impl ISyncCallback2 {
    pub unsafe fn OnProgress(&self, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.OnProgress)(::windows::core::Interface::as_raw(self), provider, syncstage, dwcompletedwork, dwtotalwork).ok()
    }
    pub unsafe fn OnChange<'a, P0>(&self, psyncchange: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncChange>>,
    {
        (::windows::core::Interface::vtable(self).base__.OnChange)(::windows::core::Interface::as_raw(self), psyncchange.into().abi()).ok()
    }
    pub unsafe fn OnConflict<'a, P0>(&self, pconflict: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IChangeConflict>>,
    {
        (::windows::core::Interface::vtable(self).base__.OnConflict)(::windows::core::Interface::as_raw(self), pconflict.into().abi()).ok()
    }
    pub unsafe fn OnFullEnumerationNeeded(&self, pfullenumerationaction: *mut SYNC_FULL_ENUMERATION_ACTION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.OnFullEnumerationNeeded)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pfullenumerationaction)).ok()
    }
    pub unsafe fn OnRecoverableError<'a, P0>(&self, precoverableerror: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IRecoverableError>>,
    {
        (::windows::core::Interface::vtable(self).base__.OnRecoverableError)(::windows::core::Interface::as_raw(self), precoverableerror.into().abi()).ok()
    }
    pub unsafe fn OnChangeApplied(&self, dwchangesapplied: u32, dwchangesfailed: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnChangeApplied)(::windows::core::Interface::as_raw(self), dwchangesapplied, dwchangesfailed).ok()
    }
    pub unsafe fn OnChangeFailed(&self, dwchangesapplied: u32, dwchangesfailed: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnChangeFailed)(::windows::core::Interface::as_raw(self), dwchangesapplied, dwchangesfailed).ok()
    }
}
impl ::core::convert::From<ISyncCallback2> for ::windows::core::IUnknown {
    fn from(value: ISyncCallback2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISyncCallback2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISyncCallback2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncCallback2> for ::windows::core::IUnknown {
    fn from(value: &ISyncCallback2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ISyncCallback2> for ISyncCallback {
    fn from(value: ISyncCallback2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISyncCallback2> for &'a ISyncCallback {
    fn from(value: &'a ISyncCallback2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncCallback2> for ISyncCallback {
    fn from(value: &ISyncCallback2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISyncCallback2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncCallback2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncCallback2 {}
impl ::core::fmt::Debug for ISyncCallback2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncCallback2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISyncCallback2 {
    type Vtable = ISyncCallback2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x47ce84af_7442_4ead_8630_12015e030ad7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncCallback2_Vtbl {
    pub base__: ISyncCallback_Vtbl,
    pub OnChangeApplied: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwchangesapplied: u32, dwchangesfailed: u32) -> ::windows::core::HRESULT,
    pub OnChangeFailed: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwchangesapplied: u32, dwchangesfailed: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct ISyncChange(::windows::core::IUnknown);
impl ISyncChange {
    pub unsafe fn GetOwnerReplicaId(&self, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetOwnerReplicaId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbreplicaid), ::core::mem::transmute(pcbidsize)).ok()
    }
    pub unsafe fn GetRootItemId(&self, pbrootitemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRootItemId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbrootitemid), ::core::mem::transmute(pcbidsize)).ok()
    }
    pub unsafe fn GetChangeVersion(&self, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetChangeVersion)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbcurrentreplicaid), ::core::mem::transmute(pversion)).ok()
    }
    pub unsafe fn GetCreationVersion(&self, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetCreationVersion)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbcurrentreplicaid), ::core::mem::transmute(pversion)).ok()
    }
    pub unsafe fn GetFlags(&self, pdwflags: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFlags)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwflags)).ok()
    }
    pub unsafe fn GetWorkEstimate(&self, pdwwork: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetWorkEstimate)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwwork)).ok()
    }
    pub unsafe fn GetChangeUnits(&self) -> ::windows::core::Result<IEnumSyncChangeUnits> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetChangeUnits)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumSyncChangeUnits>(result__)
    }
    pub unsafe fn GetMadeWithKnowledge(&self) -> ::windows::core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetMadeWithKnowledge)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetLearnedKnowledge(&self) -> ::windows::core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetLearnedKnowledge)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn SetWorkEstimate(&self, dwwork: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetWorkEstimate)(::windows::core::Interface::as_raw(self), dwwork).ok()
    }
}
impl ::core::convert::From<ISyncChange> for ::windows::core::IUnknown {
    fn from(value: ISyncChange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISyncChange> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISyncChange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncChange> for ::windows::core::IUnknown {
    fn from(value: &ISyncChange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISyncChange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncChange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncChange {}
impl ::core::fmt::Debug for ISyncChange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncChange").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISyncChange {
    type Vtable = ISyncChange_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xa1952beb_0f6b_4711_b136_01da85b968a6);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncChange_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetOwnerReplicaId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT,
    pub GetRootItemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbrootitemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT,
    pub GetChangeVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows::core::HRESULT,
    pub GetCreationVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows::core::HRESULT,
    pub GetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT,
    pub GetWorkEstimate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwwork: *mut u32) -> ::windows::core::HRESULT,
    pub GetChangeUnits: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetMadeWithKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppmadewithknowledge: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetLearnedKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplearnedknowledge: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetWorkEstimate: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwwork: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct ISyncChangeBatch(::windows::core::IUnknown);
impl ISyncChangeBatch {
    pub unsafe fn GetChangeEnumerator(&self) -> ::windows::core::Result<IEnumSyncChanges> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetChangeEnumerator)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumSyncChanges>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsLastBatch(&self, pflastbatch: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetIsLastBatch)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pflastbatch)).ok()
    }
    pub unsafe fn GetWorkEstimateForBatch(&self, pdwworkforbatch: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetWorkEstimateForBatch)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwworkforbatch)).ok()
    }
    pub unsafe fn GetRemainingWorkEstimateForSession(&self, pdwremainingworkforsession: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetRemainingWorkEstimateForSession)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwremainingworkforsession)).ok()
    }
    pub unsafe fn BeginOrderedGroup(&self, pblowerbound: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.BeginOrderedGroup)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pblowerbound)).ok()
    }
    pub unsafe fn EndOrderedGroup<'a, P0>(&self, pbupperbound: *const u8, pmadewithknowledge: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncKnowledge>>,
    {
        (::windows::core::Interface::vtable(self).base__.EndOrderedGroup)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbupperbound), pmadewithknowledge.into().abi()).ok()
    }
    pub unsafe fn AddItemMetadataToGroup(&self, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32) -> ::windows::core::Result<ISyncChangeBuilder> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.AddItemMetadataToGroup)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbownerreplicaid), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pchangeversion), ::core::mem::transmute(pcreationversion), dwflags, dwworkforchange, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncChangeBuilder>(result__)
    }
    pub unsafe fn GetLearnedKnowledge(&self) -> ::windows::core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetLearnedKnowledge)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetPrerequisiteKnowledge(&self) -> ::windows::core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetPrerequisiteKnowledge)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetSourceForgottenKnowledge(&self) -> ::windows::core::Result<IForgottenKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetSourceForgottenKnowledge)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IForgottenKnowledge>(result__)
    }
    pub unsafe fn SetLastBatch(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetLastBatch)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetWorkEstimateForBatch(&self, dwworkforbatch: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetWorkEstimateForBatch)(::windows::core::Interface::as_raw(self), dwworkforbatch).ok()
    }
    pub unsafe fn SetRemainingWorkEstimateForSession(&self, dwremainingworkforsession: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetRemainingWorkEstimateForSession)(::windows::core::Interface::as_raw(self), dwremainingworkforsession).ok()
    }
    pub unsafe fn Serialize(&self, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Serialize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbchangebatch), ::core::mem::transmute(pcbchangebatch)).ok()
    }
    pub unsafe fn BeginUnorderedGroup(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeginUnorderedGroup)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EndUnorderedGroup<'a, P0, P1>(&self, pmadewithknowledge: P0, fallchangesforknowledge: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncKnowledge>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).EndUnorderedGroup)(::windows::core::Interface::as_raw(self), pmadewithknowledge.into().abi(), fallchangesforknowledge.into()).ok()
    }
    pub unsafe fn AddLoggedConflict<'a, P0>(&self, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32, pconflictknowledge: P0) -> ::windows::core::Result<ISyncChangeBuilder>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncKnowledge>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AddLoggedConflict)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbownerreplicaid), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pchangeversion), ::core::mem::transmute(pcreationversion), dwflags, dwworkforchange, pconflictknowledge.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncChangeBuilder>(result__)
    }
}
impl ::core::convert::From<ISyncChangeBatch> for ::windows::core::IUnknown {
    fn from(value: ISyncChangeBatch) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISyncChangeBatch> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISyncChangeBatch) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncChangeBatch> for ::windows::core::IUnknown {
    fn from(value: &ISyncChangeBatch) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ISyncChangeBatch> for ISyncChangeBatchBase {
    fn from(value: ISyncChangeBatch) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISyncChangeBatch> for &'a ISyncChangeBatchBase {
    fn from(value: &'a ISyncChangeBatch) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncChangeBatch> for ISyncChangeBatchBase {
    fn from(value: &ISyncChangeBatch) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISyncChangeBatch {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncChangeBatch {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncChangeBatch {}
impl ::core::fmt::Debug for ISyncChangeBatch {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncChangeBatch").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISyncChangeBatch {
    type Vtable = ISyncChangeBatch_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x70c64dee_380f_4c2e_8f70_31c55bd5f9b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncChangeBatch_Vtbl {
    pub base__: ISyncChangeBatchBase_Vtbl,
    pub BeginUnorderedGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub EndUnorderedGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pmadewithknowledge: *mut ::core::ffi::c_void, fallchangesforknowledge: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EndUnorderedGroup: usize,
    pub AddLoggedConflict: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32, pconflictknowledge: *mut ::core::ffi::c_void, ppchangebuilder: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct ISyncChangeBatch2(::windows::core::IUnknown);
impl ISyncChangeBatch2 {
    pub unsafe fn GetChangeEnumerator(&self) -> ::windows::core::Result<IEnumSyncChanges> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetChangeEnumerator)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumSyncChanges>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsLastBatch(&self, pflastbatch: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetIsLastBatch)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pflastbatch)).ok()
    }
    pub unsafe fn GetWorkEstimateForBatch(&self, pdwworkforbatch: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetWorkEstimateForBatch)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwworkforbatch)).ok()
    }
    pub unsafe fn GetRemainingWorkEstimateForSession(&self, pdwremainingworkforsession: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetRemainingWorkEstimateForSession)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwremainingworkforsession)).ok()
    }
    pub unsafe fn BeginOrderedGroup(&self, pblowerbound: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.BeginOrderedGroup)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pblowerbound)).ok()
    }
    pub unsafe fn EndOrderedGroup<'a, P0>(&self, pbupperbound: *const u8, pmadewithknowledge: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncKnowledge>>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.EndOrderedGroup)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbupperbound), pmadewithknowledge.into().abi()).ok()
    }
    pub unsafe fn AddItemMetadataToGroup(&self, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32) -> ::windows::core::Result<ISyncChangeBuilder> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.AddItemMetadataToGroup)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbownerreplicaid), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pchangeversion), ::core::mem::transmute(pcreationversion), dwflags, dwworkforchange, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncChangeBuilder>(result__)
    }
    pub unsafe fn GetLearnedKnowledge(&self) -> ::windows::core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetLearnedKnowledge)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetPrerequisiteKnowledge(&self) -> ::windows::core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetPrerequisiteKnowledge)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetSourceForgottenKnowledge(&self) -> ::windows::core::Result<IForgottenKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetSourceForgottenKnowledge)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IForgottenKnowledge>(result__)
    }
    pub unsafe fn SetLastBatch(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetLastBatch)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetWorkEstimateForBatch(&self, dwworkforbatch: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetWorkEstimateForBatch)(::windows::core::Interface::as_raw(self), dwworkforbatch).ok()
    }
    pub unsafe fn SetRemainingWorkEstimateForSession(&self, dwremainingworkforsession: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetRemainingWorkEstimateForSession)(::windows::core::Interface::as_raw(self), dwremainingworkforsession).ok()
    }
    pub unsafe fn Serialize(&self, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Serialize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbchangebatch), ::core::mem::transmute(pcbchangebatch)).ok()
    }
    pub unsafe fn BeginUnorderedGroup(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.BeginUnorderedGroup)(::windows::core::Interface::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EndUnorderedGroup<'a, P0, P1>(&self, pmadewithknowledge: P0, fallchangesforknowledge: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncKnowledge>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.EndUnorderedGroup)(::windows::core::Interface::as_raw(self), pmadewithknowledge.into().abi(), fallchangesforknowledge.into()).ok()
    }
    pub unsafe fn AddLoggedConflict<'a, P0>(&self, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32, pconflictknowledge: P0) -> ::windows::core::Result<ISyncChangeBuilder>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncKnowledge>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.AddLoggedConflict)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbownerreplicaid), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pchangeversion), ::core::mem::transmute(pcreationversion), dwflags, dwworkforchange, pconflictknowledge.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncChangeBuilder>(result__)
    }
    pub unsafe fn AddMergeTombstoneMetadataToGroup(&self, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32) -> ::windows::core::Result<ISyncChangeBuilder> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AddMergeTombstoneMetadataToGroup)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbownerreplicaid), ::core::mem::transmute(pbwinneritemid), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pchangeversion), ::core::mem::transmute(pcreationversion), dwworkforchange, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncChangeBuilder>(result__)
    }
    pub unsafe fn AddMergeTombstoneLoggedConflict<'a, P0>(&self, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, pconflictknowledge: P0) -> ::windows::core::Result<ISyncChangeBuilder>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncKnowledge>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AddMergeTombstoneLoggedConflict)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbownerreplicaid), ::core::mem::transmute(pbwinneritemid), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pchangeversion), ::core::mem::transmute(pcreationversion), dwworkforchange, pconflictknowledge.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncChangeBuilder>(result__)
    }
}
impl ::core::convert::From<ISyncChangeBatch2> for ::windows::core::IUnknown {
    fn from(value: ISyncChangeBatch2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISyncChangeBatch2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISyncChangeBatch2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncChangeBatch2> for ::windows::core::IUnknown {
    fn from(value: &ISyncChangeBatch2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ISyncChangeBatch2> for ISyncChangeBatchBase {
    fn from(value: ISyncChangeBatch2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISyncChangeBatch2> for &'a ISyncChangeBatchBase {
    fn from(value: &'a ISyncChangeBatch2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncChangeBatch2> for ISyncChangeBatchBase {
    fn from(value: &ISyncChangeBatch2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ISyncChangeBatch2> for ISyncChangeBatch {
    fn from(value: ISyncChangeBatch2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISyncChangeBatch2> for &'a ISyncChangeBatch {
    fn from(value: &'a ISyncChangeBatch2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncChangeBatch2> for ISyncChangeBatch {
    fn from(value: &ISyncChangeBatch2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISyncChangeBatch2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncChangeBatch2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncChangeBatch2 {}
impl ::core::fmt::Debug for ISyncChangeBatch2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncChangeBatch2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISyncChangeBatch2 {
    type Vtable = ISyncChangeBatch2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x225f4a33_f5ee_4cc7_b039_67a262b4b2ac);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncChangeBatch2_Vtbl {
    pub base__: ISyncChangeBatch_Vtbl,
    pub AddMergeTombstoneMetadataToGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, ppchangebuilder: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddMergeTombstoneLoggedConflict: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, pconflictknowledge: *mut ::core::ffi::c_void, ppchangebuilder: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct ISyncChangeBatchAdvanced(::windows::core::IUnknown);
impl ISyncChangeBatchAdvanced {
    pub unsafe fn GetFilterInfo(&self) -> ::windows::core::Result<ISyncFilterInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetFilterInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncFilterInfo>(result__)
    }
    pub unsafe fn ConvertFullEnumerationChangeBatchToRegularChangeBatch(&self) -> ::windows::core::Result<ISyncChangeBatch> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ConvertFullEnumerationChangeBatchToRegularChangeBatch)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncChangeBatch>(result__)
    }
    pub unsafe fn GetUpperBoundItemId(&self, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetUpperBoundItemId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pcbidsize)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBatchLevelKnowledgeShouldBeApplied(&self, pfbatchknowledgeshouldbeapplied: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetBatchLevelKnowledgeShouldBeApplied)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pfbatchknowledgeshouldbeapplied)).ok()
    }
}
impl ::core::convert::From<ISyncChangeBatchAdvanced> for ::windows::core::IUnknown {
    fn from(value: ISyncChangeBatchAdvanced) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISyncChangeBatchAdvanced> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISyncChangeBatchAdvanced) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncChangeBatchAdvanced> for ::windows::core::IUnknown {
    fn from(value: &ISyncChangeBatchAdvanced) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISyncChangeBatchAdvanced {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncChangeBatchAdvanced {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncChangeBatchAdvanced {}
impl ::core::fmt::Debug for ISyncChangeBatchAdvanced {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncChangeBatchAdvanced").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISyncChangeBatchAdvanced {
    type Vtable = ISyncChangeBatchAdvanced_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0f1a4995_cbc8_421d_b550_5d0bebf3e9a5);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncChangeBatchAdvanced_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetFilterInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppfilterinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ConvertFullEnumerationChangeBatchToRegularChangeBatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppchangebatch: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetUpperBoundItemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbitemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetBatchLevelKnowledgeShouldBeApplied: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfbatchknowledgeshouldbeapplied: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetBatchLevelKnowledgeShouldBeApplied: usize,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct ISyncChangeBatchBase(::windows::core::IUnknown);
impl ISyncChangeBatchBase {
    pub unsafe fn GetChangeEnumerator(&self) -> ::windows::core::Result<IEnumSyncChanges> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetChangeEnumerator)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumSyncChanges>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsLastBatch(&self, pflastbatch: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetIsLastBatch)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pflastbatch)).ok()
    }
    pub unsafe fn GetWorkEstimateForBatch(&self, pdwworkforbatch: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetWorkEstimateForBatch)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwworkforbatch)).ok()
    }
    pub unsafe fn GetRemainingWorkEstimateForSession(&self, pdwremainingworkforsession: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRemainingWorkEstimateForSession)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwremainingworkforsession)).ok()
    }
    pub unsafe fn BeginOrderedGroup(&self, pblowerbound: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).BeginOrderedGroup)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pblowerbound)).ok()
    }
    pub unsafe fn EndOrderedGroup<'a, P0>(&self, pbupperbound: *const u8, pmadewithknowledge: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncKnowledge>>,
    {
        (::windows::core::Interface::vtable(self).EndOrderedGroup)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbupperbound), pmadewithknowledge.into().abi()).ok()
    }
    pub unsafe fn AddItemMetadataToGroup(&self, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32) -> ::windows::core::Result<ISyncChangeBuilder> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AddItemMetadataToGroup)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbownerreplicaid), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pchangeversion), ::core::mem::transmute(pcreationversion), dwflags, dwworkforchange, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncChangeBuilder>(result__)
    }
    pub unsafe fn GetLearnedKnowledge(&self) -> ::windows::core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetLearnedKnowledge)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetPrerequisiteKnowledge(&self) -> ::windows::core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetPrerequisiteKnowledge)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetSourceForgottenKnowledge(&self) -> ::windows::core::Result<IForgottenKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetSourceForgottenKnowledge)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IForgottenKnowledge>(result__)
    }
    pub unsafe fn SetLastBatch(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLastBatch)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetWorkEstimateForBatch(&self, dwworkforbatch: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetWorkEstimateForBatch)(::windows::core::Interface::as_raw(self), dwworkforbatch).ok()
    }
    pub unsafe fn SetRemainingWorkEstimateForSession(&self, dwremainingworkforsession: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetRemainingWorkEstimateForSession)(::windows::core::Interface::as_raw(self), dwremainingworkforsession).ok()
    }
    pub unsafe fn Serialize(&self, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Serialize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbchangebatch), ::core::mem::transmute(pcbchangebatch)).ok()
    }
}
impl ::core::convert::From<ISyncChangeBatchBase> for ::windows::core::IUnknown {
    fn from(value: ISyncChangeBatchBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISyncChangeBatchBase> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISyncChangeBatchBase) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncChangeBatchBase> for ::windows::core::IUnknown {
    fn from(value: &ISyncChangeBatchBase) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISyncChangeBatchBase {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncChangeBatchBase {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncChangeBatchBase {}
impl ::core::fmt::Debug for ISyncChangeBatchBase {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncChangeBatchBase").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISyncChangeBatchBase {
    type Vtable = ISyncChangeBatchBase_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x52f6e694_6a71_4494_a184_a8311bf5d227);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncChangeBatchBase_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetChangeEnumerator: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIsLastBatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pflastbatch: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIsLastBatch: usize,
    pub GetWorkEstimateForBatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwworkforbatch: *mut u32) -> ::windows::core::HRESULT,
    pub GetRemainingWorkEstimateForSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwremainingworkforsession: *mut u32) -> ::windows::core::HRESULT,
    pub BeginOrderedGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pblowerbound: *const u8) -> ::windows::core::HRESULT,
    pub EndOrderedGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbupperbound: *const u8, pmadewithknowledge: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub AddItemMetadataToGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32, ppchangebuilder: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetLearnedKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplearnedknowledge: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetPrerequisiteKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppprerequisteknowledge: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSourceForgottenKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsourceforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetLastBatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetWorkEstimateForBatch: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwworkforbatch: u32) -> ::windows::core::HRESULT,
    pub SetRemainingWorkEstimateForSession: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwremainingworkforsession: u32) -> ::windows::core::HRESULT,
    pub Serialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct ISyncChangeBatchBase2(::windows::core::IUnknown);
impl ISyncChangeBatchBase2 {
    pub unsafe fn GetChangeEnumerator(&self) -> ::windows::core::Result<IEnumSyncChanges> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetChangeEnumerator)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumSyncChanges>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsLastBatch(&self, pflastbatch: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetIsLastBatch)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pflastbatch)).ok()
    }
    pub unsafe fn GetWorkEstimateForBatch(&self, pdwworkforbatch: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetWorkEstimateForBatch)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwworkforbatch)).ok()
    }
    pub unsafe fn GetRemainingWorkEstimateForSession(&self, pdwremainingworkforsession: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetRemainingWorkEstimateForSession)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwremainingworkforsession)).ok()
    }
    pub unsafe fn BeginOrderedGroup(&self, pblowerbound: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.BeginOrderedGroup)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pblowerbound)).ok()
    }
    pub unsafe fn EndOrderedGroup<'a, P0>(&self, pbupperbound: *const u8, pmadewithknowledge: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncKnowledge>>,
    {
        (::windows::core::Interface::vtable(self).base__.EndOrderedGroup)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbupperbound), pmadewithknowledge.into().abi()).ok()
    }
    pub unsafe fn AddItemMetadataToGroup(&self, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32) -> ::windows::core::Result<ISyncChangeBuilder> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.AddItemMetadataToGroup)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbownerreplicaid), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pchangeversion), ::core::mem::transmute(pcreationversion), dwflags, dwworkforchange, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncChangeBuilder>(result__)
    }
    pub unsafe fn GetLearnedKnowledge(&self) -> ::windows::core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetLearnedKnowledge)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetPrerequisiteKnowledge(&self) -> ::windows::core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetPrerequisiteKnowledge)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetSourceForgottenKnowledge(&self) -> ::windows::core::Result<IForgottenKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetSourceForgottenKnowledge)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IForgottenKnowledge>(result__)
    }
    pub unsafe fn SetLastBatch(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetLastBatch)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetWorkEstimateForBatch(&self, dwworkforbatch: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetWorkEstimateForBatch)(::windows::core::Interface::as_raw(self), dwworkforbatch).ok()
    }
    pub unsafe fn SetRemainingWorkEstimateForSession(&self, dwremainingworkforsession: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetRemainingWorkEstimateForSession)(::windows::core::Interface::as_raw(self), dwremainingworkforsession).ok()
    }
    pub unsafe fn Serialize(&self, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Serialize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbchangebatch), ::core::mem::transmute(pcbchangebatch)).ok()
    }
    pub unsafe fn SerializeWithOptions(&self, targetformatversion: SYNC_SERIALIZATION_VERSION, dwflags: u32, pbbuffer: *mut u8, pdwserializedsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SerializeWithOptions)(::windows::core::Interface::as_raw(self), targetformatversion, dwflags, ::core::mem::transmute(pbbuffer), ::core::mem::transmute(pdwserializedsize)).ok()
    }
}
impl ::core::convert::From<ISyncChangeBatchBase2> for ::windows::core::IUnknown {
    fn from(value: ISyncChangeBatchBase2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISyncChangeBatchBase2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISyncChangeBatchBase2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncChangeBatchBase2> for ::windows::core::IUnknown {
    fn from(value: &ISyncChangeBatchBase2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ISyncChangeBatchBase2> for ISyncChangeBatchBase {
    fn from(value: ISyncChangeBatchBase2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISyncChangeBatchBase2> for &'a ISyncChangeBatchBase {
    fn from(value: &'a ISyncChangeBatchBase2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncChangeBatchBase2> for ISyncChangeBatchBase {
    fn from(value: &ISyncChangeBatchBase2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISyncChangeBatchBase2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncChangeBatchBase2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncChangeBatchBase2 {}
impl ::core::fmt::Debug for ISyncChangeBatchBase2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncChangeBatchBase2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISyncChangeBatchBase2 {
    type Vtable = ISyncChangeBatchBase2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6fdb596a_d755_4584_bd0c_c0c23a548fbf);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncChangeBatchBase2_Vtbl {
    pub base__: ISyncChangeBatchBase_Vtbl,
    pub SerializeWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetformatversion: SYNC_SERIALIZATION_VERSION, dwflags: u32, pbbuffer: *mut u8, pdwserializedsize: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct ISyncChangeBatchWithFilterKeyMap(::windows::core::IUnknown);
impl ISyncChangeBatchWithFilterKeyMap {
    pub unsafe fn GetFilterKeyMap(&self) -> ::windows::core::Result<IFilterKeyMap> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetFilterKeyMap)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IFilterKeyMap>(result__)
    }
    pub unsafe fn SetFilterKeyMap<'a, P0>(&self, pifilterkeymap: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IFilterKeyMap>>,
    {
        (::windows::core::Interface::vtable(self).SetFilterKeyMap)(::windows::core::Interface::as_raw(self), pifilterkeymap.into().abi()).ok()
    }
    pub unsafe fn SetFilterForgottenKnowledge<'a, P0>(&self, dwfilterkey: u32, pfilterforgottenknowledge: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncKnowledge>>,
    {
        (::windows::core::Interface::vtable(self).SetFilterForgottenKnowledge)(::windows::core::Interface::as_raw(self), dwfilterkey, pfilterforgottenknowledge.into().abi()).ok()
    }
    pub unsafe fn GetFilteredReplicaLearnedKnowledge<'a, P0, P1>(&self, pdestinationknowledge: P0, pnewmoveins: P1) -> ::windows::core::Result<ISyncKnowledge>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncKnowledge>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IEnumItemIds>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetFilteredReplicaLearnedKnowledge)(::windows::core::Interface::as_raw(self), pdestinationknowledge.into().abi(), pnewmoveins.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetLearnedFilterForgottenKnowledge<'a, P0, P1>(&self, pdestinationknowledge: P0, pnewmoveins: P1, dwfilterkey: u32) -> ::windows::core::Result<ISyncKnowledge>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncKnowledge>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IEnumItemIds>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetLearnedFilterForgottenKnowledge)(::windows::core::Interface::as_raw(self), pdestinationknowledge.into().abi(), pnewmoveins.into().abi(), dwfilterkey, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetFilteredReplicaLearnedForgottenKnowledge<'a, P0, P1>(&self, pdestinationknowledge: P0, pnewmoveins: P1) -> ::windows::core::Result<ISyncKnowledge>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncKnowledge>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IEnumItemIds>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetFilteredReplicaLearnedForgottenKnowledge)(::windows::core::Interface::as_raw(self), pdestinationknowledge.into().abi(), pnewmoveins.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete<'a, P0, P1>(&self, pdestinationknowledge: P0, pnewmoveins: P1) -> ::windows::core::Result<ISyncKnowledge>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncKnowledge>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IEnumItemIds>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete)(::windows::core::Interface::as_raw(self), pdestinationknowledge.into().abi(), pnewmoveins.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete<'a, P0, P1>(&self, pdestinationknowledge: P0, pnewmoveins: P1, dwfilterkey: u32) -> ::windows::core::Result<ISyncKnowledge>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncKnowledge>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IEnumItemIds>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete)(::windows::core::Interface::as_raw(self), pdestinationknowledge.into().abi(), pnewmoveins.into().abi(), dwfilterkey, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
}
impl ::core::convert::From<ISyncChangeBatchWithFilterKeyMap> for ::windows::core::IUnknown {
    fn from(value: ISyncChangeBatchWithFilterKeyMap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISyncChangeBatchWithFilterKeyMap> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISyncChangeBatchWithFilterKeyMap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncChangeBatchWithFilterKeyMap> for ::windows::core::IUnknown {
    fn from(value: &ISyncChangeBatchWithFilterKeyMap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISyncChangeBatchWithFilterKeyMap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncChangeBatchWithFilterKeyMap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncChangeBatchWithFilterKeyMap {}
impl ::core::fmt::Debug for ISyncChangeBatchWithFilterKeyMap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncChangeBatchWithFilterKeyMap").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISyncChangeBatchWithFilterKeyMap {
    type Vtable = ISyncChangeBatchWithFilterKeyMap_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xde247002_566d_459a_a6ed_a5aab3459fb7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncChangeBatchWithFilterKeyMap_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetFilterKeyMap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppifilterkeymap: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetFilterKeyMap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pifilterkeymap: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetFilterForgottenKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwfilterkey: u32, pfilterforgottenknowledge: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetFilteredReplicaLearnedKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestinationknowledge: *mut ::core::ffi::c_void, pnewmoveins: *mut ::core::ffi::c_void, pplearnedforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetLearnedFilterForgottenKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestinationknowledge: *mut ::core::ffi::c_void, pnewmoveins: *mut ::core::ffi::c_void, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetFilteredReplicaLearnedForgottenKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestinationknowledge: *mut ::core::ffi::c_void, pnewmoveins: *mut ::core::ffi::c_void, pplearnedforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestinationknowledge: *mut ::core::ffi::c_void, pnewmoveins: *mut ::core::ffi::c_void, pplearnedforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestinationknowledge: *mut ::core::ffi::c_void, pnewmoveins: *mut ::core::ffi::c_void, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct ISyncChangeBatchWithPrerequisite(::windows::core::IUnknown);
impl ISyncChangeBatchWithPrerequisite {
    pub unsafe fn GetChangeEnumerator(&self) -> ::windows::core::Result<IEnumSyncChanges> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetChangeEnumerator)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumSyncChanges>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsLastBatch(&self, pflastbatch: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetIsLastBatch)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pflastbatch)).ok()
    }
    pub unsafe fn GetWorkEstimateForBatch(&self, pdwworkforbatch: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetWorkEstimateForBatch)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwworkforbatch)).ok()
    }
    pub unsafe fn GetRemainingWorkEstimateForSession(&self, pdwremainingworkforsession: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetRemainingWorkEstimateForSession)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwremainingworkforsession)).ok()
    }
    pub unsafe fn BeginOrderedGroup(&self, pblowerbound: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.BeginOrderedGroup)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pblowerbound)).ok()
    }
    pub unsafe fn EndOrderedGroup<'a, P0>(&self, pbupperbound: *const u8, pmadewithknowledge: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncKnowledge>>,
    {
        (::windows::core::Interface::vtable(self).base__.EndOrderedGroup)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbupperbound), pmadewithknowledge.into().abi()).ok()
    }
    pub unsafe fn AddItemMetadataToGroup(&self, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32) -> ::windows::core::Result<ISyncChangeBuilder> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.AddItemMetadataToGroup)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbownerreplicaid), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pchangeversion), ::core::mem::transmute(pcreationversion), dwflags, dwworkforchange, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncChangeBuilder>(result__)
    }
    pub unsafe fn GetLearnedKnowledge(&self) -> ::windows::core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetLearnedKnowledge)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetPrerequisiteKnowledge(&self) -> ::windows::core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetPrerequisiteKnowledge)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetSourceForgottenKnowledge(&self) -> ::windows::core::Result<IForgottenKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetSourceForgottenKnowledge)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IForgottenKnowledge>(result__)
    }
    pub unsafe fn SetLastBatch(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetLastBatch)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetWorkEstimateForBatch(&self, dwworkforbatch: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetWorkEstimateForBatch)(::windows::core::Interface::as_raw(self), dwworkforbatch).ok()
    }
    pub unsafe fn SetRemainingWorkEstimateForSession(&self, dwremainingworkforsession: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetRemainingWorkEstimateForSession)(::windows::core::Interface::as_raw(self), dwremainingworkforsession).ok()
    }
    pub unsafe fn Serialize(&self, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Serialize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbchangebatch), ::core::mem::transmute(pcbchangebatch)).ok()
    }
    pub unsafe fn SetPrerequisiteKnowledge<'a, P0>(&self, pprerequisiteknowledge: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncKnowledge>>,
    {
        (::windows::core::Interface::vtable(self).SetPrerequisiteKnowledge)(::windows::core::Interface::as_raw(self), pprerequisiteknowledge.into().abi()).ok()
    }
    pub unsafe fn GetLearnedKnowledgeWithPrerequisite<'a, P0>(&self, pdestinationknowledge: P0) -> ::windows::core::Result<ISyncKnowledge>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncKnowledge>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetLearnedKnowledgeWithPrerequisite)(::windows::core::Interface::as_raw(self), pdestinationknowledge.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetLearnedForgottenKnowledge(&self) -> ::windows::core::Result<IForgottenKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetLearnedForgottenKnowledge)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IForgottenKnowledge>(result__)
    }
}
impl ::core::convert::From<ISyncChangeBatchWithPrerequisite> for ::windows::core::IUnknown {
    fn from(value: ISyncChangeBatchWithPrerequisite) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISyncChangeBatchWithPrerequisite> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISyncChangeBatchWithPrerequisite) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncChangeBatchWithPrerequisite> for ::windows::core::IUnknown {
    fn from(value: &ISyncChangeBatchWithPrerequisite) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ISyncChangeBatchWithPrerequisite> for ISyncChangeBatchBase {
    fn from(value: ISyncChangeBatchWithPrerequisite) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISyncChangeBatchWithPrerequisite> for &'a ISyncChangeBatchBase {
    fn from(value: &'a ISyncChangeBatchWithPrerequisite) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncChangeBatchWithPrerequisite> for ISyncChangeBatchBase {
    fn from(value: &ISyncChangeBatchWithPrerequisite) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISyncChangeBatchWithPrerequisite {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncChangeBatchWithPrerequisite {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncChangeBatchWithPrerequisite {}
impl ::core::fmt::Debug for ISyncChangeBatchWithPrerequisite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncChangeBatchWithPrerequisite").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISyncChangeBatchWithPrerequisite {
    type Vtable = ISyncChangeBatchWithPrerequisite_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x097f13be_5b92_4048_b3f2_7b42a2515e07);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncChangeBatchWithPrerequisite_Vtbl {
    pub base__: ISyncChangeBatchBase_Vtbl,
    pub SetPrerequisiteKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprerequisiteknowledge: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetLearnedKnowledgeWithPrerequisite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestinationknowledge: *mut ::core::ffi::c_void, pplearnedwithprerequisiteknowledge: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetLearnedForgottenKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplearnedforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct ISyncChangeBuilder(::windows::core::IUnknown);
impl ISyncChangeBuilder {
    pub unsafe fn AddChangeUnitMetadata(&self, pbchangeunitid: *const u8, pchangeunitversion: *const SYNC_VERSION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).AddChangeUnitMetadata)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbchangeunitid), ::core::mem::transmute(pchangeunitversion)).ok()
    }
}
impl ::core::convert::From<ISyncChangeBuilder> for ::windows::core::IUnknown {
    fn from(value: ISyncChangeBuilder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISyncChangeBuilder> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISyncChangeBuilder) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncChangeBuilder> for ::windows::core::IUnknown {
    fn from(value: &ISyncChangeBuilder) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISyncChangeBuilder {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncChangeBuilder {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncChangeBuilder {}
impl ::core::fmt::Debug for ISyncChangeBuilder {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncChangeBuilder").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISyncChangeBuilder {
    type Vtable = ISyncChangeBuilder_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x56f14771_8677_484f_a170_e386e418a676);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncChangeBuilder_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub AddChangeUnitMetadata: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbchangeunitid: *const u8, pchangeunitversion: *const SYNC_VERSION) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct ISyncChangeUnit(::windows::core::IUnknown);
impl ISyncChangeUnit {
    pub unsafe fn GetItemChange(&self) -> ::windows::core::Result<ISyncChange> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetItemChange)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncChange>(result__)
    }
    pub unsafe fn GetChangeUnitId(&self, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetChangeUnitId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbchangeunitid), ::core::mem::transmute(pcbidsize)).ok()
    }
    pub unsafe fn GetChangeUnitVersion(&self, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetChangeUnitVersion)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbcurrentreplicaid), ::core::mem::transmute(pversion)).ok()
    }
}
impl ::core::convert::From<ISyncChangeUnit> for ::windows::core::IUnknown {
    fn from(value: ISyncChangeUnit) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISyncChangeUnit> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISyncChangeUnit) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncChangeUnit> for ::windows::core::IUnknown {
    fn from(value: &ISyncChangeUnit) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISyncChangeUnit {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncChangeUnit {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncChangeUnit {}
impl ::core::fmt::Debug for ISyncChangeUnit {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncChangeUnit").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISyncChangeUnit {
    type Vtable = ISyncChangeUnit_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x60edd8ca_7341_4bb7_95ce_fab6394b51cb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncChangeUnit_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetItemChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsyncchange: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetChangeUnitId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbchangeunitid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT,
    pub GetChangeUnitVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbcurrentreplicaid: *const u8, pversion: *mut SYNC_VERSION) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct ISyncChangeWithFilterKeyMap(::windows::core::IUnknown);
impl ISyncChangeWithFilterKeyMap {
    pub unsafe fn GetFilterCount(&self, pdwfiltercount: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFilterCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwfiltercount)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFilterChange(&self, dwfilterkey: u32, pfilterchange: *mut SYNC_FILTER_CHANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFilterChange)(::windows::core::Interface::as_raw(self), dwfilterkey, ::core::mem::transmute(pfilterchange)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAllChangeUnitsPresentFlag(&self, pfallchangeunitspresent: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetAllChangeUnitsPresentFlag)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pfallchangeunitspresent)).ok()
    }
    pub unsafe fn GetFilterForgottenKnowledge(&self, dwfilterkey: u32) -> ::windows::core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetFilterForgottenKnowledge)(::windows::core::Interface::as_raw(self), dwfilterkey, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetFilteredReplicaLearnedKnowledge<'a, P0, P1>(&self, pdestinationknowledge: P0, pnewmoveins: P1) -> ::windows::core::Result<ISyncKnowledge>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncKnowledge>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IEnumItemIds>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetFilteredReplicaLearnedKnowledge)(::windows::core::Interface::as_raw(self), pdestinationknowledge.into().abi(), pnewmoveins.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetLearnedFilterForgottenKnowledge<'a, P0, P1>(&self, pdestinationknowledge: P0, pnewmoveins: P1, dwfilterkey: u32) -> ::windows::core::Result<ISyncKnowledge>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncKnowledge>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IEnumItemIds>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetLearnedFilterForgottenKnowledge)(::windows::core::Interface::as_raw(self), pdestinationknowledge.into().abi(), pnewmoveins.into().abi(), dwfilterkey, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetFilteredReplicaLearnedForgottenKnowledge<'a, P0, P1>(&self, pdestinationknowledge: P0, pnewmoveins: P1) -> ::windows::core::Result<ISyncKnowledge>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncKnowledge>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IEnumItemIds>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetFilteredReplicaLearnedForgottenKnowledge)(::windows::core::Interface::as_raw(self), pdestinationknowledge.into().abi(), pnewmoveins.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete<'a, P0, P1>(&self, pdestinationknowledge: P0, pnewmoveins: P1) -> ::windows::core::Result<ISyncKnowledge>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncKnowledge>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IEnumItemIds>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete)(::windows::core::Interface::as_raw(self), pdestinationknowledge.into().abi(), pnewmoveins.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete<'a, P0, P1>(&self, pdestinationknowledge: P0, pnewmoveins: P1, dwfilterkey: u32) -> ::windows::core::Result<ISyncKnowledge>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncKnowledge>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IEnumItemIds>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete)(::windows::core::Interface::as_raw(self), pdestinationknowledge.into().abi(), pnewmoveins.into().abi(), dwfilterkey, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
}
impl ::core::convert::From<ISyncChangeWithFilterKeyMap> for ::windows::core::IUnknown {
    fn from(value: ISyncChangeWithFilterKeyMap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISyncChangeWithFilterKeyMap> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISyncChangeWithFilterKeyMap) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncChangeWithFilterKeyMap> for ::windows::core::IUnknown {
    fn from(value: &ISyncChangeWithFilterKeyMap) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISyncChangeWithFilterKeyMap {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncChangeWithFilterKeyMap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncChangeWithFilterKeyMap {}
impl ::core::fmt::Debug for ISyncChangeWithFilterKeyMap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncChangeWithFilterKeyMap").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISyncChangeWithFilterKeyMap {
    type Vtable = ISyncChangeWithFilterKeyMap_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbfe1ef00_e87d_42fd_a4e9_242d70414aef);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncChangeWithFilterKeyMap_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetFilterCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwfiltercount: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetFilterChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwfilterkey: u32, pfilterchange: *mut SYNC_FILTER_CHANGE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetFilterChange: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetAllChangeUnitsPresentFlag: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfallchangeunitspresent: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetAllChangeUnitsPresentFlag: usize,
    pub GetFilterForgottenKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwfilterkey: u32, ppifilterforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetFilteredReplicaLearnedKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestinationknowledge: *mut ::core::ffi::c_void, pnewmoveins: *mut ::core::ffi::c_void, pplearnedknowledge: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetLearnedFilterForgottenKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestinationknowledge: *mut ::core::ffi::c_void, pnewmoveins: *mut ::core::ffi::c_void, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetFilteredReplicaLearnedForgottenKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestinationknowledge: *mut ::core::ffi::c_void, pnewmoveins: *mut ::core::ffi::c_void, pplearnedforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetFilteredReplicaLearnedForgottenKnowledgeAfterRecoveryComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestinationknowledge: *mut ::core::ffi::c_void, pnewmoveins: *mut ::core::ffi::c_void, pplearnedforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetLearnedFilterForgottenKnowledgeAfterRecoveryComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestinationknowledge: *mut ::core::ffi::c_void, pnewmoveins: *mut ::core::ffi::c_void, dwfilterkey: u32, pplearnedfilterforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct ISyncChangeWithPrerequisite(::windows::core::IUnknown);
impl ISyncChangeWithPrerequisite {
    pub unsafe fn GetPrerequisiteKnowledge(&self) -> ::windows::core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetPrerequisiteKnowledge)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetLearnedKnowledgeWithPrerequisite<'a, P0>(&self, pdestinationknowledge: P0) -> ::windows::core::Result<ISyncKnowledge>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncKnowledge>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetLearnedKnowledgeWithPrerequisite)(::windows::core::Interface::as_raw(self), pdestinationknowledge.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
}
impl ::core::convert::From<ISyncChangeWithPrerequisite> for ::windows::core::IUnknown {
    fn from(value: ISyncChangeWithPrerequisite) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISyncChangeWithPrerequisite> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISyncChangeWithPrerequisite) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncChangeWithPrerequisite> for ::windows::core::IUnknown {
    fn from(value: &ISyncChangeWithPrerequisite) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISyncChangeWithPrerequisite {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncChangeWithPrerequisite {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncChangeWithPrerequisite {}
impl ::core::fmt::Debug for ISyncChangeWithPrerequisite {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncChangeWithPrerequisite").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISyncChangeWithPrerequisite {
    type Vtable = ISyncChangeWithPrerequisite_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e38382f_1589_48c3_92e4_05ecdcb4f3f7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncChangeWithPrerequisite_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetPrerequisiteKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppprerequisiteknowledge: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetLearnedKnowledgeWithPrerequisite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdestinationknowledge: *mut ::core::ffi::c_void, pplearnedknowledgewithprerequisite: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct ISyncConstraintCallback(::windows::core::IUnknown);
impl ISyncConstraintCallback {
    pub unsafe fn OnConstraintConflict<'a, P0>(&self, pconflict: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, IConstraintConflict>>,
    {
        (::windows::core::Interface::vtable(self).OnConstraintConflict)(::windows::core::Interface::as_raw(self), pconflict.into().abi()).ok()
    }
}
impl ::core::convert::From<ISyncConstraintCallback> for ::windows::core::IUnknown {
    fn from(value: ISyncConstraintCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISyncConstraintCallback> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISyncConstraintCallback) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncConstraintCallback> for ::windows::core::IUnknown {
    fn from(value: &ISyncConstraintCallback) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISyncConstraintCallback {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncConstraintCallback {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncConstraintCallback {}
impl ::core::fmt::Debug for ISyncConstraintCallback {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncConstraintCallback").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISyncConstraintCallback {
    type Vtable = ISyncConstraintCallback_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8af3843e_75b3_438c_bb51_6f020d70d3cb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncConstraintCallback_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub OnConstraintConflict: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconflict: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct ISyncDataConverter(::windows::core::IUnknown);
impl ISyncDataConverter {
    pub unsafe fn ConvertDataRetrieverFromProviderFormat<'a, P0, P1>(&self, punkdataretrieverin: P0, penumsyncchanges: P1) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IEnumSyncChanges>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ConvertDataRetrieverFromProviderFormat)(::windows::core::Interface::as_raw(self), punkdataretrieverin.into().abi(), penumsyncchanges.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn ConvertDataRetrieverToProviderFormat<'a, P0, P1>(&self, punkdataretrieverin: P0, penumsyncchanges: P1) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, IEnumSyncChanges>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ConvertDataRetrieverToProviderFormat)(::windows::core::Interface::as_raw(self), punkdataretrieverin.into().abi(), penumsyncchanges.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn ConvertDataFromProviderFormat<'a, P0, P1>(&self, pdatacontext: P0, punkdatain: P1) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ILoadChangeContext>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ConvertDataFromProviderFormat)(::windows::core::Interface::as_raw(self), pdatacontext.into().abi(), punkdatain.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn ConvertDataToProviderFormat<'a, P0, P1>(&self, pdatacontext: P0, punkdataout: P1) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ILoadChangeContext>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ConvertDataToProviderFormat)(::windows::core::Interface::as_raw(self), pdatacontext.into().abi(), punkdataout.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
}
impl ::core::convert::From<ISyncDataConverter> for ::windows::core::IUnknown {
    fn from(value: ISyncDataConverter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISyncDataConverter> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISyncDataConverter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncDataConverter> for ::windows::core::IUnknown {
    fn from(value: &ISyncDataConverter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISyncDataConverter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncDataConverter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncDataConverter {}
impl ::core::fmt::Debug for ISyncDataConverter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncDataConverter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISyncDataConverter {
    type Vtable = ISyncDataConverter_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x435d4861_68d5_44aa_a0f9_72a0b00ef9cf);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncDataConverter_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub ConvertDataRetrieverFromProviderFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkdataretrieverin: *mut ::core::ffi::c_void, penumsyncchanges: *mut ::core::ffi::c_void, ppunkdataout: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ConvertDataRetrieverToProviderFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, punkdataretrieverin: *mut ::core::ffi::c_void, penumsyncchanges: *mut ::core::ffi::c_void, ppunkdataout: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ConvertDataFromProviderFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatacontext: *mut ::core::ffi::c_void, punkdatain: *mut ::core::ffi::c_void, ppunkdataout: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ConvertDataToProviderFormat: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdatacontext: *mut ::core::ffi::c_void, punkdataout: *mut ::core::ffi::c_void, ppunkdataout: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct ISyncFilter(::windows::core::IUnknown);
impl ISyncFilter {
    pub unsafe fn IsIdentical<'a, P0>(&self, psyncfilter: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncFilter>>,
    {
        (::windows::core::Interface::vtable(self).IsIdentical)(::windows::core::Interface::as_raw(self), psyncfilter.into().abi()).ok()
    }
    pub unsafe fn Serialize(&self, pbsyncfilter: *mut u8, pcbsyncfilter: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Serialize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbsyncfilter), ::core::mem::transmute(pcbsyncfilter)).ok()
    }
}
impl ::core::convert::From<ISyncFilter> for ::windows::core::IUnknown {
    fn from(value: ISyncFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISyncFilter> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISyncFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncFilter> for ::windows::core::IUnknown {
    fn from(value: &ISyncFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISyncFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncFilter {}
impl ::core::fmt::Debug for ISyncFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncFilter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISyncFilter {
    type Vtable = ISyncFilter_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x087a3f15_0fcb_44c1_9639_53c14e2b5506);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncFilter_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub IsIdentical: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psyncfilter: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Serialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbsyncfilter: *mut u8, pcbsyncfilter: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct ISyncFilterDeserializer(::windows::core::IUnknown);
impl ISyncFilterDeserializer {
    pub unsafe fn DeserializeSyncFilter(&self, pbsyncfilter: *const u8, dwcbsyncfilter: u32) -> ::windows::core::Result<ISyncFilter> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).DeserializeSyncFilter)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbsyncfilter), dwcbsyncfilter, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncFilter>(result__)
    }
}
impl ::core::convert::From<ISyncFilterDeserializer> for ::windows::core::IUnknown {
    fn from(value: ISyncFilterDeserializer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISyncFilterDeserializer> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISyncFilterDeserializer) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncFilterDeserializer> for ::windows::core::IUnknown {
    fn from(value: &ISyncFilterDeserializer) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISyncFilterDeserializer {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncFilterDeserializer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncFilterDeserializer {}
impl ::core::fmt::Debug for ISyncFilterDeserializer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncFilterDeserializer").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISyncFilterDeserializer {
    type Vtable = ISyncFilterDeserializer_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb45b7a72_e5c7_46be_9c82_77b8b15dab8a);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncFilterDeserializer_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub DeserializeSyncFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbsyncfilter: *const u8, dwcbsyncfilter: u32, ppisyncfilter: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct ISyncFilterInfo(::windows::core::IUnknown);
impl ISyncFilterInfo {
    pub unsafe fn Serialize(&self, pbbuffer: *mut u8, pcbbuffer: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Serialize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbbuffer), ::core::mem::transmute(pcbbuffer)).ok()
    }
}
impl ::core::convert::From<ISyncFilterInfo> for ::windows::core::IUnknown {
    fn from(value: ISyncFilterInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISyncFilterInfo> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISyncFilterInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncFilterInfo> for ::windows::core::IUnknown {
    fn from(value: &ISyncFilterInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISyncFilterInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncFilterInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncFilterInfo {}
impl ::core::fmt::Debug for ISyncFilterInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncFilterInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISyncFilterInfo {
    type Vtable = ISyncFilterInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x794eaaf8_3f2e_47e6_9728_17e6fcf94cb7);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncFilterInfo_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub Serialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbbuffer: *mut u8, pcbbuffer: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct ISyncFilterInfo2(::windows::core::IUnknown);
impl ISyncFilterInfo2 {
    pub unsafe fn Serialize(&self, pbbuffer: *mut u8, pcbbuffer: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Serialize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbbuffer), ::core::mem::transmute(pcbbuffer)).ok()
    }
    pub unsafe fn GetFlags(&self, pdwflags: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetFlags)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwflags)).ok()
    }
}
impl ::core::convert::From<ISyncFilterInfo2> for ::windows::core::IUnknown {
    fn from(value: ISyncFilterInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISyncFilterInfo2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISyncFilterInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncFilterInfo2> for ::windows::core::IUnknown {
    fn from(value: &ISyncFilterInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ISyncFilterInfo2> for ISyncFilterInfo {
    fn from(value: ISyncFilterInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISyncFilterInfo2> for &'a ISyncFilterInfo {
    fn from(value: &'a ISyncFilterInfo2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncFilterInfo2> for ISyncFilterInfo {
    fn from(value: &ISyncFilterInfo2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISyncFilterInfo2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncFilterInfo2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncFilterInfo2 {}
impl ::core::fmt::Debug for ISyncFilterInfo2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncFilterInfo2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISyncFilterInfo2 {
    type Vtable = ISyncFilterInfo2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x19b394ba_e3d0_468c_934d_321968b2ab34);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncFilterInfo2_Vtbl {
    pub base__: ISyncFilterInfo_Vtbl,
    pub GetFlags: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwflags: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct ISyncFullEnumerationChange(::windows::core::IUnknown);
impl ISyncFullEnumerationChange {
    pub unsafe fn GetLearnedKnowledgeAfterRecoveryComplete(&self) -> ::windows::core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetLearnedKnowledgeAfterRecoveryComplete)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetLearnedForgottenKnowledge(&self) -> ::windows::core::Result<IForgottenKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetLearnedForgottenKnowledge)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IForgottenKnowledge>(result__)
    }
}
impl ::core::convert::From<ISyncFullEnumerationChange> for ::windows::core::IUnknown {
    fn from(value: ISyncFullEnumerationChange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISyncFullEnumerationChange> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISyncFullEnumerationChange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncFullEnumerationChange> for ::windows::core::IUnknown {
    fn from(value: &ISyncFullEnumerationChange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISyncFullEnumerationChange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncFullEnumerationChange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncFullEnumerationChange {}
impl ::core::fmt::Debug for ISyncFullEnumerationChange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncFullEnumerationChange").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISyncFullEnumerationChange {
    type Vtable = ISyncFullEnumerationChange_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9785e0bd_bdff_40c4_98c5_b34b2f1991b3);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncFullEnumerationChange_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetLearnedKnowledgeAfterRecoveryComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplearnedknowledge: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetLearnedForgottenKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplearnedforgottenknowledge: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct ISyncFullEnumerationChangeBatch(::windows::core::IUnknown);
impl ISyncFullEnumerationChangeBatch {
    pub unsafe fn GetChangeEnumerator(&self) -> ::windows::core::Result<IEnumSyncChanges> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetChangeEnumerator)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumSyncChanges>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsLastBatch(&self, pflastbatch: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetIsLastBatch)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pflastbatch)).ok()
    }
    pub unsafe fn GetWorkEstimateForBatch(&self, pdwworkforbatch: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetWorkEstimateForBatch)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwworkforbatch)).ok()
    }
    pub unsafe fn GetRemainingWorkEstimateForSession(&self, pdwremainingworkforsession: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetRemainingWorkEstimateForSession)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwremainingworkforsession)).ok()
    }
    pub unsafe fn BeginOrderedGroup(&self, pblowerbound: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.BeginOrderedGroup)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pblowerbound)).ok()
    }
    pub unsafe fn EndOrderedGroup<'a, P0>(&self, pbupperbound: *const u8, pmadewithknowledge: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncKnowledge>>,
    {
        (::windows::core::Interface::vtable(self).base__.EndOrderedGroup)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbupperbound), pmadewithknowledge.into().abi()).ok()
    }
    pub unsafe fn AddItemMetadataToGroup(&self, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32) -> ::windows::core::Result<ISyncChangeBuilder> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.AddItemMetadataToGroup)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbownerreplicaid), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pchangeversion), ::core::mem::transmute(pcreationversion), dwflags, dwworkforchange, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncChangeBuilder>(result__)
    }
    pub unsafe fn GetLearnedKnowledge(&self) -> ::windows::core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetLearnedKnowledge)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetPrerequisiteKnowledge(&self) -> ::windows::core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetPrerequisiteKnowledge)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetSourceForgottenKnowledge(&self) -> ::windows::core::Result<IForgottenKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetSourceForgottenKnowledge)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IForgottenKnowledge>(result__)
    }
    pub unsafe fn SetLastBatch(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetLastBatch)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetWorkEstimateForBatch(&self, dwworkforbatch: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetWorkEstimateForBatch)(::windows::core::Interface::as_raw(self), dwworkforbatch).ok()
    }
    pub unsafe fn SetRemainingWorkEstimateForSession(&self, dwremainingworkforsession: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetRemainingWorkEstimateForSession)(::windows::core::Interface::as_raw(self), dwremainingworkforsession).ok()
    }
    pub unsafe fn Serialize(&self, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Serialize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbchangebatch), ::core::mem::transmute(pcbchangebatch)).ok()
    }
    pub unsafe fn GetLearnedKnowledgeAfterRecoveryComplete(&self) -> ::windows::core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetLearnedKnowledgeAfterRecoveryComplete)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetClosedLowerBoundItemId(&self, pbclosedlowerbounditemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetClosedLowerBoundItemId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbclosedlowerbounditemid), ::core::mem::transmute(pcbidsize)).ok()
    }
    pub unsafe fn GetClosedUpperBoundItemId(&self, pbclosedupperbounditemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetClosedUpperBoundItemId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbclosedupperbounditemid), ::core::mem::transmute(pcbidsize)).ok()
    }
}
impl ::core::convert::From<ISyncFullEnumerationChangeBatch> for ::windows::core::IUnknown {
    fn from(value: ISyncFullEnumerationChangeBatch) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISyncFullEnumerationChangeBatch> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISyncFullEnumerationChangeBatch) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncFullEnumerationChangeBatch> for ::windows::core::IUnknown {
    fn from(value: &ISyncFullEnumerationChangeBatch) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ISyncFullEnumerationChangeBatch> for ISyncChangeBatchBase {
    fn from(value: ISyncFullEnumerationChangeBatch) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISyncFullEnumerationChangeBatch> for &'a ISyncChangeBatchBase {
    fn from(value: &'a ISyncFullEnumerationChangeBatch) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncFullEnumerationChangeBatch> for ISyncChangeBatchBase {
    fn from(value: &ISyncFullEnumerationChangeBatch) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISyncFullEnumerationChangeBatch {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncFullEnumerationChangeBatch {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncFullEnumerationChangeBatch {}
impl ::core::fmt::Debug for ISyncFullEnumerationChangeBatch {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncFullEnumerationChangeBatch").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISyncFullEnumerationChangeBatch {
    type Vtable = ISyncFullEnumerationChangeBatch_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xef64197d_4f44_4ea2_b355_4524713e3bed);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncFullEnumerationChangeBatch_Vtbl {
    pub base__: ISyncChangeBatchBase_Vtbl,
    pub GetLearnedKnowledgeAfterRecoveryComplete: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pplearnedknowledgeafterrecoverycomplete: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetClosedLowerBoundItemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbclosedlowerbounditemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT,
    pub GetClosedUpperBoundItemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbclosedupperbounditemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct ISyncFullEnumerationChangeBatch2(::windows::core::IUnknown);
impl ISyncFullEnumerationChangeBatch2 {
    pub unsafe fn GetChangeEnumerator(&self) -> ::windows::core::Result<IEnumSyncChanges> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetChangeEnumerator)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumSyncChanges>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIsLastBatch(&self, pflastbatch: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetIsLastBatch)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pflastbatch)).ok()
    }
    pub unsafe fn GetWorkEstimateForBatch(&self, pdwworkforbatch: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetWorkEstimateForBatch)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwworkforbatch)).ok()
    }
    pub unsafe fn GetRemainingWorkEstimateForSession(&self, pdwremainingworkforsession: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.GetRemainingWorkEstimateForSession)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwremainingworkforsession)).ok()
    }
    pub unsafe fn BeginOrderedGroup(&self, pblowerbound: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.BeginOrderedGroup)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pblowerbound)).ok()
    }
    pub unsafe fn EndOrderedGroup<'a, P0>(&self, pbupperbound: *const u8, pmadewithknowledge: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncKnowledge>>,
    {
        (::windows::core::Interface::vtable(self).base__.base__.EndOrderedGroup)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbupperbound), pmadewithknowledge.into().abi()).ok()
    }
    pub unsafe fn AddItemMetadataToGroup(&self, pbownerreplicaid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwflags: u32, dwworkforchange: u32) -> ::windows::core::Result<ISyncChangeBuilder> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.AddItemMetadataToGroup)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbownerreplicaid), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pchangeversion), ::core::mem::transmute(pcreationversion), dwflags, dwworkforchange, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncChangeBuilder>(result__)
    }
    pub unsafe fn GetLearnedKnowledge(&self) -> ::windows::core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetLearnedKnowledge)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetPrerequisiteKnowledge(&self) -> ::windows::core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetPrerequisiteKnowledge)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetSourceForgottenKnowledge(&self) -> ::windows::core::Result<IForgottenKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.base__.GetSourceForgottenKnowledge)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IForgottenKnowledge>(result__)
    }
    pub unsafe fn SetLastBatch(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetLastBatch)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn SetWorkEstimateForBatch(&self, dwworkforbatch: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetWorkEstimateForBatch)(::windows::core::Interface::as_raw(self), dwworkforbatch).ok()
    }
    pub unsafe fn SetRemainingWorkEstimateForSession(&self, dwremainingworkforsession: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.SetRemainingWorkEstimateForSession)(::windows::core::Interface::as_raw(self), dwremainingworkforsession).ok()
    }
    pub unsafe fn Serialize(&self, pbchangebatch: *mut u8, pcbchangebatch: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.base__.Serialize)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbchangebatch), ::core::mem::transmute(pcbchangebatch)).ok()
    }
    pub unsafe fn GetLearnedKnowledgeAfterRecoveryComplete(&self) -> ::windows::core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetLearnedKnowledgeAfterRecoveryComplete)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn GetClosedLowerBoundItemId(&self, pbclosedlowerbounditemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetClosedLowerBoundItemId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbclosedlowerbounditemid), ::core::mem::transmute(pcbidsize)).ok()
    }
    pub unsafe fn GetClosedUpperBoundItemId(&self, pbclosedupperbounditemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetClosedUpperBoundItemId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbclosedupperbounditemid), ::core::mem::transmute(pcbidsize)).ok()
    }
    pub unsafe fn AddMergeTombstoneMetadataToGroup(&self, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32) -> ::windows::core::Result<ISyncChangeBuilder> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).AddMergeTombstoneMetadataToGroup)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbownerreplicaid), ::core::mem::transmute(pbwinneritemid), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pchangeversion), ::core::mem::transmute(pcreationversion), dwworkforchange, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncChangeBuilder>(result__)
    }
}
impl ::core::convert::From<ISyncFullEnumerationChangeBatch2> for ::windows::core::IUnknown {
    fn from(value: ISyncFullEnumerationChangeBatch2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISyncFullEnumerationChangeBatch2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISyncFullEnumerationChangeBatch2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncFullEnumerationChangeBatch2> for ::windows::core::IUnknown {
    fn from(value: &ISyncFullEnumerationChangeBatch2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ISyncFullEnumerationChangeBatch2> for ISyncChangeBatchBase {
    fn from(value: ISyncFullEnumerationChangeBatch2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISyncFullEnumerationChangeBatch2> for &'a ISyncChangeBatchBase {
    fn from(value: &'a ISyncFullEnumerationChangeBatch2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncFullEnumerationChangeBatch2> for ISyncChangeBatchBase {
    fn from(value: &ISyncFullEnumerationChangeBatch2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ISyncFullEnumerationChangeBatch2> for ISyncFullEnumerationChangeBatch {
    fn from(value: ISyncFullEnumerationChangeBatch2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISyncFullEnumerationChangeBatch2> for &'a ISyncFullEnumerationChangeBatch {
    fn from(value: &'a ISyncFullEnumerationChangeBatch2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncFullEnumerationChangeBatch2> for ISyncFullEnumerationChangeBatch {
    fn from(value: &ISyncFullEnumerationChangeBatch2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISyncFullEnumerationChangeBatch2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncFullEnumerationChangeBatch2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncFullEnumerationChangeBatch2 {}
impl ::core::fmt::Debug for ISyncFullEnumerationChangeBatch2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncFullEnumerationChangeBatch2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISyncFullEnumerationChangeBatch2 {
    type Vtable = ISyncFullEnumerationChangeBatch2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe06449f4_a205_4b65_9724_01b22101eec1);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncFullEnumerationChangeBatch2_Vtbl {
    pub base__: ISyncFullEnumerationChangeBatch_Vtbl,
    pub AddMergeTombstoneMetadataToGroup: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbownerreplicaid: *const u8, pbwinneritemid: *const u8, pbitemid: *const u8, pchangeversion: *const SYNC_VERSION, pcreationversion: *const SYNC_VERSION, dwworkforchange: u32, ppchangebuilder: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct ISyncKnowledge(::windows::core::IUnknown);
impl ISyncKnowledge {
    pub unsafe fn GetOwnerReplicaId(&self, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetOwnerReplicaId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbreplicaid), ::core::mem::transmute(pcbidsize)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Serialize<'a, P0>(&self, fserializereplicakeymap: P0, pbknowledge: *mut u8, pcbknowledge: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).Serialize)(::windows::core::Interface::as_raw(self), fserializereplicakeymap.into(), ::core::mem::transmute(pbknowledge), ::core::mem::transmute(pcbknowledge)).ok()
    }
    pub unsafe fn SetLocalTickCount(&self, ulltickcount: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetLocalTickCount)(::windows::core::Interface::as_raw(self), ulltickcount).ok()
    }
    pub unsafe fn ContainsChange(&self, pbversionownerreplicaid: *const u8, pgiditemid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ContainsChange)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbversionownerreplicaid), ::core::mem::transmute(pgiditemid), ::core::mem::transmute(psyncversion)).ok()
    }
    pub unsafe fn ContainsChangeUnit(&self, pbversionownerreplicaid: *const u8, pbitemid: *const u8, pbchangeunitid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ContainsChangeUnit)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbversionownerreplicaid), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pbchangeunitid), ::core::mem::transmute(psyncversion)).ok()
    }
    pub unsafe fn GetScopeVector(&self, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetScopeVector)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppunk)).ok()
    }
    pub unsafe fn GetReplicaKeyMap(&self) -> ::windows::core::Result<IReplicaKeyMap> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetReplicaKeyMap)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IReplicaKeyMap>(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn ConvertVersion<'a, P0>(&self, pknowledgein: P0, pbcurrentownerid: *const u8, pversionin: *const SYNC_VERSION, pbnewownerid: *mut u8, pcbidsize: *mut u32, pversionout: *mut SYNC_VERSION) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncKnowledge>>,
    {
        (::windows::core::Interface::vtable(self).ConvertVersion)(::windows::core::Interface::as_raw(self), pknowledgein.into().abi(), ::core::mem::transmute(pbcurrentownerid), ::core::mem::transmute(pversionin), ::core::mem::transmute(pbnewownerid), ::core::mem::transmute(pcbidsize), ::core::mem::transmute(pversionout)).ok()
    }
    pub unsafe fn MapRemoteToLocal<'a, P0>(&self, premoteknowledge: P0) -> ::windows::core::Result<ISyncKnowledge>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncKnowledge>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).MapRemoteToLocal)(::windows::core::Interface::as_raw(self), premoteknowledge.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn Union<'a, P0>(&self, pknowledge: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncKnowledge>>,
    {
        (::windows::core::Interface::vtable(self).Union)(::windows::core::Interface::as_raw(self), pknowledge.into().abi()).ok()
    }
    pub unsafe fn ProjectOntoItem(&self, pbitemid: *const u8) -> ::windows::core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ProjectOntoItem)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbitemid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn ProjectOntoChangeUnit(&self, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows::core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ProjectOntoChangeUnit)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pbchangeunitid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn ProjectOntoRange(&self, psrngsyncrange: *const SYNC_RANGE) -> ::windows::core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ProjectOntoRange)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(psrngsyncrange), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn ExcludeItem(&self, pbitemid: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ExcludeItem)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbitemid)).ok()
    }
    pub unsafe fn ExcludeChangeUnit(&self, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ExcludeChangeUnit)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pbchangeunitid)).ok()
    }
    pub unsafe fn ContainsKnowledge<'a, P0>(&self, pknowledge: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncKnowledge>>,
    {
        (::windows::core::Interface::vtable(self).ContainsKnowledge)(::windows::core::Interface::as_raw(self), pknowledge.into().abi()).ok()
    }
    pub unsafe fn FindMinTickCountForReplica(&self, pbreplicaid: *const u8, pullreplicatickcount: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FindMinTickCountForReplica)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbreplicaid), ::core::mem::transmute(pullreplicatickcount)).ok()
    }
    pub unsafe fn GetRangeExceptions(&self, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetRangeExceptions)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppunk)).ok()
    }
    pub unsafe fn GetSingleItemExceptions(&self, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSingleItemExceptions)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppunk)).ok()
    }
    pub unsafe fn GetChangeUnitExceptions(&self, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetChangeUnitExceptions)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppunk)).ok()
    }
    pub unsafe fn FindClockVectorForItem(&self, pbitemid: *const u8, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FindClockVectorForItem)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbitemid), ::core::mem::transmute(riid), ::core::mem::transmute(ppunk)).ok()
    }
    pub unsafe fn FindClockVectorForChangeUnit(&self, pbitemid: *const u8, pbchangeunitid: *const u8, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FindClockVectorForChangeUnit)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pbchangeunitid), ::core::mem::transmute(riid), ::core::mem::transmute(ppunk)).ok()
    }
    pub unsafe fn GetVersion(&self, pdwversion: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetVersion)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwversion)).ok()
    }
}
impl ::core::convert::From<ISyncKnowledge> for ::windows::core::IUnknown {
    fn from(value: ISyncKnowledge) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISyncKnowledge> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISyncKnowledge) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncKnowledge> for ::windows::core::IUnknown {
    fn from(value: &ISyncKnowledge) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISyncKnowledge {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncKnowledge {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncKnowledge {}
impl ::core::fmt::Debug for ISyncKnowledge {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncKnowledge").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISyncKnowledge {
    type Vtable = ISyncKnowledge_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x615bbb53_c945_4203_bf4b_2cb65919a0aa);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncKnowledge_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetOwnerReplicaId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Serialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fserializereplicakeymap: super::super::Foundation::BOOL, pbknowledge: *mut u8, pcbknowledge: *mut u32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Serialize: usize,
    pub SetLocalTickCount: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ulltickcount: u64) -> ::windows::core::HRESULT,
    pub ContainsChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbversionownerreplicaid: *const u8, pgiditemid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows::core::HRESULT,
    pub ContainsChangeUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbversionownerreplicaid: *const u8, pbitemid: *const u8, pbchangeunitid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows::core::HRESULT,
    pub GetScopeVector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetReplicaKeyMap: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppreplicakeymap: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppclonedknowledge: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ConvertVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pknowledgein: *mut ::core::ffi::c_void, pbcurrentownerid: *const u8, pversionin: *const SYNC_VERSION, pbnewownerid: *mut u8, pcbidsize: *mut u32, pversionout: *mut SYNC_VERSION) -> ::windows::core::HRESULT,
    pub MapRemoteToLocal: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, premoteknowledge: *mut ::core::ffi::c_void, ppmappedknowledge: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Union: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pknowledge: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ProjectOntoItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbitemid: *const u8, ppknowledgeout: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ProjectOntoChangeUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8, ppknowledgeout: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ProjectOntoRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psrngsyncrange: *const SYNC_RANGE, ppknowledgeout: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub ExcludeItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbitemid: *const u8) -> ::windows::core::HRESULT,
    pub ExcludeChangeUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows::core::HRESULT,
    pub ContainsKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pknowledge: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FindMinTickCountForReplica: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbreplicaid: *const u8, pullreplicatickcount: *mut u64) -> ::windows::core::HRESULT,
    pub GetRangeExceptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSingleItemExceptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetChangeUnitExceptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FindClockVectorForItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbitemid: *const u8, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub FindClockVectorForChangeUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pdwversion: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct ISyncKnowledge2(::windows::core::IUnknown);
impl ISyncKnowledge2 {
    pub unsafe fn GetOwnerReplicaId(&self, pbreplicaid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetOwnerReplicaId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbreplicaid), ::core::mem::transmute(pcbidsize)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Serialize<'a, P0>(&self, fserializereplicakeymap: P0, pbknowledge: *mut u8, pcbknowledge: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).base__.Serialize)(::windows::core::Interface::as_raw(self), fserializereplicakeymap.into(), ::core::mem::transmute(pbknowledge), ::core::mem::transmute(pcbknowledge)).ok()
    }
    pub unsafe fn SetLocalTickCount(&self, ulltickcount: u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetLocalTickCount)(::windows::core::Interface::as_raw(self), ulltickcount).ok()
    }
    pub unsafe fn ContainsChange(&self, pbversionownerreplicaid: *const u8, pgiditemid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ContainsChange)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbversionownerreplicaid), ::core::mem::transmute(pgiditemid), ::core::mem::transmute(psyncversion)).ok()
    }
    pub unsafe fn ContainsChangeUnit(&self, pbversionownerreplicaid: *const u8, pbitemid: *const u8, pbchangeunitid: *const u8, psyncversion: *const SYNC_VERSION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ContainsChangeUnit)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbversionownerreplicaid), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pbchangeunitid), ::core::mem::transmute(psyncversion)).ok()
    }
    pub unsafe fn GetScopeVector(&self, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetScopeVector)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppunk)).ok()
    }
    pub unsafe fn GetReplicaKeyMap(&self) -> ::windows::core::Result<IReplicaKeyMap> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetReplicaKeyMap)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IReplicaKeyMap>(result__)
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.Clone)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn ConvertVersion<'a, P0>(&self, pknowledgein: P0, pbcurrentownerid: *const u8, pversionin: *const SYNC_VERSION, pbnewownerid: *mut u8, pcbidsize: *mut u32, pversionout: *mut SYNC_VERSION) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncKnowledge>>,
    {
        (::windows::core::Interface::vtable(self).base__.ConvertVersion)(::windows::core::Interface::as_raw(self), pknowledgein.into().abi(), ::core::mem::transmute(pbcurrentownerid), ::core::mem::transmute(pversionin), ::core::mem::transmute(pbnewownerid), ::core::mem::transmute(pcbidsize), ::core::mem::transmute(pversionout)).ok()
    }
    pub unsafe fn MapRemoteToLocal<'a, P0>(&self, premoteknowledge: P0) -> ::windows::core::Result<ISyncKnowledge>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncKnowledge>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.MapRemoteToLocal)(::windows::core::Interface::as_raw(self), premoteknowledge.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn Union<'a, P0>(&self, pknowledge: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncKnowledge>>,
    {
        (::windows::core::Interface::vtable(self).base__.Union)(::windows::core::Interface::as_raw(self), pknowledge.into().abi()).ok()
    }
    pub unsafe fn ProjectOntoItem(&self, pbitemid: *const u8) -> ::windows::core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.ProjectOntoItem)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbitemid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn ProjectOntoChangeUnit(&self, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows::core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.ProjectOntoChangeUnit)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pbchangeunitid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn ProjectOntoRange(&self, psrngsyncrange: *const SYNC_RANGE) -> ::windows::core::Result<ISyncKnowledge> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.ProjectOntoRange)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(psrngsyncrange), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn ExcludeItem(&self, pbitemid: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ExcludeItem)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbitemid)).ok()
    }
    pub unsafe fn ExcludeChangeUnit(&self, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.ExcludeChangeUnit)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pbchangeunitid)).ok()
    }
    pub unsafe fn ContainsKnowledge<'a, P0>(&self, pknowledge: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncKnowledge>>,
    {
        (::windows::core::Interface::vtable(self).base__.ContainsKnowledge)(::windows::core::Interface::as_raw(self), pknowledge.into().abi()).ok()
    }
    pub unsafe fn FindMinTickCountForReplica(&self, pbreplicaid: *const u8, pullreplicatickcount: *mut u64) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.FindMinTickCountForReplica)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbreplicaid), ::core::mem::transmute(pullreplicatickcount)).ok()
    }
    pub unsafe fn GetRangeExceptions(&self, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetRangeExceptions)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppunk)).ok()
    }
    pub unsafe fn GetSingleItemExceptions(&self, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetSingleItemExceptions)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppunk)).ok()
    }
    pub unsafe fn GetChangeUnitExceptions(&self, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetChangeUnitExceptions)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppunk)).ok()
    }
    pub unsafe fn FindClockVectorForItem(&self, pbitemid: *const u8, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.FindClockVectorForItem)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbitemid), ::core::mem::transmute(riid), ::core::mem::transmute(ppunk)).ok()
    }
    pub unsafe fn FindClockVectorForChangeUnit(&self, pbitemid: *const u8, pbchangeunitid: *const u8, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.FindClockVectorForChangeUnit)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pbchangeunitid), ::core::mem::transmute(riid), ::core::mem::transmute(ppunk)).ok()
    }
    pub unsafe fn GetVersion(&self, pdwversion: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetVersion)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pdwversion)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIdParameters(&self, pidparameters: *mut ID_PARAMETERS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetIdParameters)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pidparameters)).ok()
    }
    pub unsafe fn ProjectOntoColumnSet(&self, ppcolumns: *const *const u8, count: u32) -> ::windows::core::Result<ISyncKnowledge2> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ProjectOntoColumnSet)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(ppcolumns), count, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge2>(result__)
    }
    pub unsafe fn SerializeWithOptions(&self, targetformatversion: SYNC_SERIALIZATION_VERSION, dwflags: u32, pbbuffer: *mut u8, pdwserializedsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SerializeWithOptions)(::windows::core::Interface::as_raw(self), targetformatversion, dwflags, ::core::mem::transmute(pbbuffer), ::core::mem::transmute(pdwserializedsize)).ok()
    }
    pub unsafe fn GetLowestUncontainedId<'a, P0>(&self, pisyncknowledge: P0, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncKnowledge2>>,
    {
        (::windows::core::Interface::vtable(self).GetLowestUncontainedId)(::windows::core::Interface::as_raw(self), pisyncknowledge.into().abi(), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pcbitemidsize)).ok()
    }
    pub unsafe fn GetInspector(&self, riid: *const ::windows::core::GUID, ppiinspector: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetInspector)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(riid), ::core::mem::transmute(ppiinspector)).ok()
    }
    pub unsafe fn GetMinimumSupportedVersion(&self, pversion: *mut SYNC_SERIALIZATION_VERSION) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetMinimumSupportedVersion)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pversion)).ok()
    }
    pub unsafe fn GetStatistics(&self, which: SYNC_STATISTICS, pvalue: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStatistics)(::windows::core::Interface::as_raw(self), which, ::core::mem::transmute(pvalue)).ok()
    }
    pub unsafe fn ContainsKnowledgeForItem<'a, P0>(&self, pknowledge: P0, pbitemid: *const u8) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncKnowledge>>,
    {
        (::windows::core::Interface::vtable(self).ContainsKnowledgeForItem)(::windows::core::Interface::as_raw(self), pknowledge.into().abi(), ::core::mem::transmute(pbitemid)).ok()
    }
    pub unsafe fn ContainsKnowledgeForChangeUnit<'a, P0>(&self, pknowledge: P0, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncKnowledge>>,
    {
        (::windows::core::Interface::vtable(self).ContainsKnowledgeForChangeUnit)(::windows::core::Interface::as_raw(self), pknowledge.into().abi(), ::core::mem::transmute(pbitemid), ::core::mem::transmute(pbchangeunitid)).ok()
    }
    pub unsafe fn ProjectOntoKnowledgeWithPrerequisite<'a, P0, P1>(&self, pprerequisiteknowledge: P0, ptemplateknowledge: P1) -> ::windows::core::Result<ISyncKnowledge>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncKnowledge>>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, ISyncKnowledge>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).ProjectOntoKnowledgeWithPrerequisite)(::windows::core::Interface::as_raw(self), pprerequisiteknowledge.into().abi(), ptemplateknowledge.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn Complement<'a, P0>(&self, psyncknowledge: P0) -> ::windows::core::Result<ISyncKnowledge>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncKnowledge>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).Complement)(::windows::core::Interface::as_raw(self), psyncknowledge.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncKnowledge>(result__)
    }
    pub unsafe fn IntersectsWithKnowledge<'a, P0>(&self, psyncknowledge: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ISyncKnowledge>>,
    {
        (::windows::core::Interface::vtable(self).IntersectsWithKnowledge)(::windows::core::Interface::as_raw(self), psyncknowledge.into().abi()).ok()
    }
    pub unsafe fn GetKnowledgeCookie(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetKnowledgeCookie)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
    pub unsafe fn CompareToKnowledgeCookie<'a, P0>(&self, pknowledgecookie: P0, presult: *mut KNOWLEDGE_COOKIE_COMPARISON_RESULT) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        (::windows::core::Interface::vtable(self).CompareToKnowledgeCookie)(::windows::core::Interface::as_raw(self), pknowledgecookie.into().abi(), ::core::mem::transmute(presult)).ok()
    }
}
impl ::core::convert::From<ISyncKnowledge2> for ::windows::core::IUnknown {
    fn from(value: ISyncKnowledge2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISyncKnowledge2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISyncKnowledge2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncKnowledge2> for ::windows::core::IUnknown {
    fn from(value: &ISyncKnowledge2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ISyncKnowledge2> for ISyncKnowledge {
    fn from(value: ISyncKnowledge2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISyncKnowledge2> for &'a ISyncKnowledge {
    fn from(value: &'a ISyncKnowledge2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncKnowledge2> for ISyncKnowledge {
    fn from(value: &ISyncKnowledge2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISyncKnowledge2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncKnowledge2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncKnowledge2 {}
impl ::core::fmt::Debug for ISyncKnowledge2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncKnowledge2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISyncKnowledge2 {
    type Vtable = ISyncKnowledge2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xed0addc0_3b4b_46a1_9a45_45661d2114c8);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncKnowledge2_Vtbl {
    pub base__: ISyncKnowledge_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIdParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidparameters: *mut ID_PARAMETERS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIdParameters: usize,
    pub ProjectOntoColumnSet: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppcolumns: *const *const u8, count: u32, ppiknowledgeout: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SerializeWithOptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, targetformatversion: SYNC_SERIALIZATION_VERSION, dwflags: u32, pbbuffer: *mut u8, pdwserializedsize: *mut u32) -> ::windows::core::HRESULT,
    pub GetLowestUncontainedId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pisyncknowledge: *mut ::core::ffi::c_void, pbitemid: *mut u8, pcbitemidsize: *mut u32) -> ::windows::core::HRESULT,
    pub GetInspector: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppiinspector: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetMinimumSupportedVersion: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pversion: *mut SYNC_SERIALIZATION_VERSION) -> ::windows::core::HRESULT,
    pub GetStatistics: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, which: SYNC_STATISTICS, pvalue: *mut u32) -> ::windows::core::HRESULT,
    pub ContainsKnowledgeForItem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pknowledge: *mut ::core::ffi::c_void, pbitemid: *const u8) -> ::windows::core::HRESULT,
    pub ContainsKnowledgeForChangeUnit: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pknowledge: *mut ::core::ffi::c_void, pbitemid: *const u8, pbchangeunitid: *const u8) -> ::windows::core::HRESULT,
    pub ProjectOntoKnowledgeWithPrerequisite: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pprerequisiteknowledge: *mut ::core::ffi::c_void, ptemplateknowledge: *mut ::core::ffi::c_void, ppprojectedknowledge: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Complement: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psyncknowledge: *mut ::core::ffi::c_void, ppcomplementedknowledge: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub IntersectsWithKnowledge: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psyncknowledge: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetKnowledgeCookie: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppknowledgecookie: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub CompareToKnowledgeCookie: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pknowledgecookie: *mut ::core::ffi::c_void, presult: *mut KNOWLEDGE_COOKIE_COMPARISON_RESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct ISyncMergeTombstoneChange(::windows::core::IUnknown);
impl ISyncMergeTombstoneChange {
    pub unsafe fn GetWinnerItemId(&self, pbwinneritemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetWinnerItemId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbwinneritemid), ::core::mem::transmute(pcbidsize)).ok()
    }
}
impl ::core::convert::From<ISyncMergeTombstoneChange> for ::windows::core::IUnknown {
    fn from(value: ISyncMergeTombstoneChange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISyncMergeTombstoneChange> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISyncMergeTombstoneChange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncMergeTombstoneChange> for ::windows::core::IUnknown {
    fn from(value: &ISyncMergeTombstoneChange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISyncMergeTombstoneChange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncMergeTombstoneChange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncMergeTombstoneChange {}
impl ::core::fmt::Debug for ISyncMergeTombstoneChange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncMergeTombstoneChange").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISyncMergeTombstoneChange {
    type Vtable = ISyncMergeTombstoneChange_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x6ec62597_0903_484c_ad61_36d6e938f47b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncMergeTombstoneChange_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetWinnerItemId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbwinneritemid: *mut u8, pcbidsize: *mut u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct ISyncProvider(::windows::core::IUnknown);
impl ISyncProvider {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIdParameters(&self, pidparameters: *mut ID_PARAMETERS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetIdParameters)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pidparameters)).ok()
    }
}
impl ::core::convert::From<ISyncProvider> for ::windows::core::IUnknown {
    fn from(value: ISyncProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISyncProvider> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISyncProvider) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncProvider> for ::windows::core::IUnknown {
    fn from(value: &ISyncProvider) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISyncProvider {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncProvider {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncProvider {}
impl ::core::fmt::Debug for ISyncProvider {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncProvider").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISyncProvider {
    type Vtable = ISyncProvider_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f657056_2bce_4a17_8c68_c7bb7898b56f);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncProvider_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIdParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidparameters: *mut ID_PARAMETERS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIdParameters: usize,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct ISyncProviderConfigUI(::windows::core::IUnknown);
impl ISyncProviderConfigUI {
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Init<'a, P0>(&self, pguidinstanceid: *const ::windows::core::GUID, pguidcontenttype: *const ::windows::core::GUID, pconfigurationproperties: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, super::super::UI::Shell::PropertiesSystem::IPropertyStore>>,
    {
        (::windows::core::Interface::vtable(self).Init)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pguidinstanceid), ::core::mem::transmute(pguidcontenttype), pconfigurationproperties.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetRegisteredProperties(&self) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::IPropertyStore> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetRegisteredProperties)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::UI::Shell::PropertiesSystem::IPropertyStore>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn CreateAndRegisterNewSyncProvider<'a, P0, P1>(&self, hwndparent: P0, punkcontext: P1) -> ::windows::core::Result<ISyncProviderInfo>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateAndRegisterNewSyncProvider)(::windows::core::Interface::as_raw(self), hwndparent.into(), punkcontext.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncProviderInfo>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn ModifySyncProvider<'a, P0, P1, P2>(&self, hwndparent: P0, punkcontext: P1, pproviderinfo: P2) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
        P1: ::std::convert::Into<::windows::core::InParam<'a, ::windows::core::IUnknown>>,
        P2: ::std::convert::Into<::windows::core::InParam<'a, ISyncProviderInfo>>,
    {
        (::windows::core::Interface::vtable(self).ModifySyncProvider)(::windows::core::Interface::as_raw(self), hwndparent.into(), punkcontext.into().abi(), pproviderinfo.into().abi()).ok()
    }
}
impl ::core::convert::From<ISyncProviderConfigUI> for ::windows::core::IUnknown {
    fn from(value: ISyncProviderConfigUI) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISyncProviderConfigUI> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISyncProviderConfigUI) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncProviderConfigUI> for ::windows::core::IUnknown {
    fn from(value: &ISyncProviderConfigUI) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISyncProviderConfigUI {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncProviderConfigUI {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncProviderConfigUI {}
impl ::core::fmt::Debug for ISyncProviderConfigUI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncProviderConfigUI").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISyncProviderConfigUI {
    type Vtable = ISyncProviderConfigUI_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7b0705f6_cbcd_4071_ab05_3bdc364d4a0c);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncProviderConfigUI_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub Init: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID, pguidcontenttype: *const ::windows::core::GUID, pconfigurationproperties: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    Init: usize,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetRegisteredProperties: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppconfiguiproperties: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetRegisteredProperties: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub CreateAndRegisterNewSyncProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, punkcontext: *mut ::core::ffi::c_void, ppproviderinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem")))]
    CreateAndRegisterNewSyncProvider: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub ModifySyncProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hwndparent: super::super::Foundation::HWND, punkcontext: *mut ::core::ffi::c_void, pproviderinfo: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem")))]
    ModifySyncProvider: usize,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[repr(transparent)]
pub struct ISyncProviderConfigUIInfo(::windows::core::IUnknown);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ISyncProviderConfigUIInfo {
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetAt(&self, iprop: u32) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::PROPERTYKEY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetAt)(::windows::core::Interface::as_raw(self), iprop, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::UI::Shell::PropertiesSystem::PROPERTYKEY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn GetValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<super::Com::StructuredStorage::PROPVARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn SetValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, propvar: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(propvar)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Commit)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetSyncProviderConfigUI(&self, dwclscontext: u32) -> ::windows::core::Result<ISyncProviderConfigUI> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetSyncProviderConfigUI)(::windows::core::Interface::as_raw(self), dwclscontext, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncProviderConfigUI>(result__)
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::convert::From<ISyncProviderConfigUIInfo> for ::windows::core::IUnknown {
    fn from(value: ISyncProviderConfigUIInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl<'a> ::core::convert::From<&'a ISyncProviderConfigUIInfo> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISyncProviderConfigUIInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::convert::From<&ISyncProviderConfigUIInfo> for ::windows::core::IUnknown {
    fn from(value: &ISyncProviderConfigUIInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::convert::From<ISyncProviderConfigUIInfo> for super::super::UI::Shell::PropertiesSystem::IPropertyStore {
    fn from(value: ISyncProviderConfigUIInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl<'a> ::core::convert::From<&'a ISyncProviderConfigUIInfo> for &'a super::super::UI::Shell::PropertiesSystem::IPropertyStore {
    fn from(value: &'a ISyncProviderConfigUIInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::convert::From<&ISyncProviderConfigUIInfo> for super::super::UI::Shell::PropertiesSystem::IPropertyStore {
    fn from(value: &ISyncProviderConfigUIInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::clone::Clone for ISyncProviderConfigUIInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::PartialEq for ISyncProviderConfigUIInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::Eq for ISyncProviderConfigUIInfo {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::fmt::Debug for ISyncProviderConfigUIInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncProviderConfigUIInfo").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
unsafe impl ::windows::core::Interface for ISyncProviderConfigUIInfo {
    type Vtable = ISyncProviderConfigUIInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x214141ae_33d7_4d8d_8e37_f227e880ce50);
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[repr(C)]
#[doc(hidden)]
pub struct ISyncProviderConfigUIInfo_Vtbl {
    pub base__: super::super::UI::Shell::PropertiesSystem::IPropertyStore_Vtbl,
    pub GetSyncProviderConfigUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwclscontext: u32, ppsyncproviderconfigui: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[repr(transparent)]
pub struct ISyncProviderInfo(::windows::core::IUnknown);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ISyncProviderInfo {
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetCount)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetAt(&self, iprop: u32) -> ::windows::core::Result<super::super::UI::Shell::PropertiesSystem::PROPERTYKEY> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetAt)(::windows::core::Interface::as_raw(self), iprop, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::super::UI::Shell::PropertiesSystem::PROPERTYKEY>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn GetValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY) -> ::windows::core::Result<super::Com::StructuredStorage::PROPVARIANT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).base__.GetValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<super::Com::StructuredStorage::PROPVARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn SetValue(&self, key: *const super::super::UI::Shell::PropertiesSystem::PROPERTYKEY, propvar: *const super::Com::StructuredStorage::PROPVARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetValue)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(key), ::core::mem::transmute(propvar)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn Commit(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.Commit)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetSyncProvider(&self, dwclscontext: u32) -> ::windows::core::Result<IRegisteredSyncProvider> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetSyncProvider)(::windows::core::Interface::as_raw(self), dwclscontext, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRegisteredSyncProvider>(result__)
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::convert::From<ISyncProviderInfo> for ::windows::core::IUnknown {
    fn from(value: ISyncProviderInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl<'a> ::core::convert::From<&'a ISyncProviderInfo> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISyncProviderInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::convert::From<&ISyncProviderInfo> for ::windows::core::IUnknown {
    fn from(value: &ISyncProviderInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::convert::From<ISyncProviderInfo> for super::super::UI::Shell::PropertiesSystem::IPropertyStore {
    fn from(value: ISyncProviderInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl<'a> ::core::convert::From<&'a ISyncProviderInfo> for &'a super::super::UI::Shell::PropertiesSystem::IPropertyStore {
    fn from(value: &'a ISyncProviderInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::convert::From<&ISyncProviderInfo> for super::super::UI::Shell::PropertiesSystem::IPropertyStore {
    fn from(value: &ISyncProviderInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::clone::Clone for ISyncProviderInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::PartialEq for ISyncProviderInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::cmp::Eq for ISyncProviderInfo {}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
impl ::core::fmt::Debug for ISyncProviderInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncProviderInfo").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
unsafe impl ::windows::core::Interface for ISyncProviderInfo {
    type Vtable = ISyncProviderInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x1ee135de_88a4_4504_b0d0_f7920d7e5ba6);
}
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[repr(C)]
#[doc(hidden)]
pub struct ISyncProviderInfo_Vtbl {
    pub base__: super::super::UI::Shell::PropertiesSystem::IPropertyStore_Vtbl,
    pub GetSyncProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, dwclscontext: u32, ppsyncprovider: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct ISyncProviderRegistration(::windows::core::IUnknown);
impl ISyncProviderRegistration {
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub unsafe fn CreateSyncProviderConfigUIRegistrationInstance(&self, pconfiguiconfig: *const SyncProviderConfigUIConfiguration) -> ::windows::core::Result<ISyncProviderConfigUIInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateSyncProviderConfigUIRegistrationInstance)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pconfiguiconfig), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncProviderConfigUIInfo>(result__)
    }
    pub unsafe fn UnregisterSyncProviderConfigUI(&self, pguidinstanceid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnregisterSyncProviderConfigUI)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pguidinstanceid)).ok()
    }
    pub unsafe fn EnumerateSyncProviderConfigUIs(&self, pguidcontenttype: *const ::windows::core::GUID, dwsupportedarchitecture: u32) -> ::windows::core::Result<IEnumSyncProviderConfigUIInfos> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).EnumerateSyncProviderConfigUIs)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pguidcontenttype), dwsupportedarchitecture, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumSyncProviderConfigUIInfos>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn CreateSyncProviderRegistrationInstance(&self, pproviderconfiguration: *const SyncProviderConfiguration) -> ::windows::core::Result<ISyncProviderInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).CreateSyncProviderRegistrationInstance)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pproviderconfiguration), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncProviderInfo>(result__)
    }
    pub unsafe fn UnregisterSyncProvider(&self, pguidinstanceid: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).UnregisterSyncProvider)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pguidinstanceid)).ok()
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetSyncProviderConfigUIInfoforProvider(&self, pguidproviderinstanceid: *const ::windows::core::GUID) -> ::windows::core::Result<ISyncProviderConfigUIInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetSyncProviderConfigUIInfoforProvider)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pguidproviderinstanceid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncProviderConfigUIInfo>(result__)
    }
    pub unsafe fn EnumerateSyncProviders(&self, pguidcontenttype: *const ::windows::core::GUID, dwstateflagstofiltermask: u32, dwstateflagstofilter: u32, refproviderclsid: *const ::windows::core::GUID, dwsupportedarchitecture: u32) -> ::windows::core::Result<IEnumSyncProviderInfos> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).EnumerateSyncProviders)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pguidcontenttype), dwstateflagstofiltermask, dwstateflagstofilter, ::core::mem::transmute(refproviderclsid), dwsupportedarchitecture, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IEnumSyncProviderInfos>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetSyncProviderInfo(&self, pguidinstanceid: *const ::windows::core::GUID) -> ::windows::core::Result<ISyncProviderInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetSyncProviderInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pguidinstanceid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncProviderInfo>(result__)
    }
    pub unsafe fn GetSyncProviderFromInstanceId(&self, pguidinstanceid: *const ::windows::core::GUID, dwclscontext: u32) -> ::windows::core::Result<IRegisteredSyncProvider> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetSyncProviderFromInstanceId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pguidinstanceid), dwclscontext, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<IRegisteredSyncProvider>(result__)
    }
    #[doc = "*Required features: `\"Win32_UI_Shell_PropertiesSystem\"`*"]
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub unsafe fn GetSyncProviderConfigUIInfo(&self, pguidinstanceid: *const ::windows::core::GUID) -> ::windows::core::Result<ISyncProviderConfigUIInfo> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetSyncProviderConfigUIInfo)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pguidinstanceid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncProviderConfigUIInfo>(result__)
    }
    pub unsafe fn GetSyncProviderConfigUIFromInstanceId(&self, pguidinstanceid: *const ::windows::core::GUID, dwclscontext: u32) -> ::windows::core::Result<ISyncProviderConfigUI> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetSyncProviderConfigUIFromInstanceId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pguidinstanceid), dwclscontext, ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncProviderConfigUI>(result__)
    }
    pub unsafe fn GetSyncProviderState(&self, pguidinstanceid: *const ::windows::core::GUID) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetSyncProviderState)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pguidinstanceid), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<u32>(result__)
    }
    pub unsafe fn SetSyncProviderState(&self, pguidinstanceid: *const ::windows::core::GUID, dwstateflagsmask: u32, dwstateflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSyncProviderState)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pguidinstanceid), dwstateflagsmask, dwstateflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterForEvent(&self, phevent: *mut super::super::Foundation::HANDLE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RegisterForEvent)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(phevent)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RevokeEvent<'a, P0>(&self, hevent: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Interface::vtable(self).RevokeEvent)(::windows::core::Interface::as_raw(self), hevent.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetChange<'a, P0>(&self, hevent: P0) -> ::windows::core::Result<ISyncRegistrationChange>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetChange)(::windows::core::Interface::as_raw(self), hevent.into(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncRegistrationChange>(result__)
    }
}
impl ::core::convert::From<ISyncProviderRegistration> for ::windows::core::IUnknown {
    fn from(value: ISyncProviderRegistration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISyncProviderRegistration> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISyncProviderRegistration) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncProviderRegistration> for ::windows::core::IUnknown {
    fn from(value: &ISyncProviderRegistration) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISyncProviderRegistration {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncProviderRegistration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncProviderRegistration {}
impl ::core::fmt::Debug for ISyncProviderRegistration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncProviderRegistration").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISyncProviderRegistration {
    type Vtable = ISyncProviderRegistration_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcb45953b_7624_47bc_a472_eb8cac6b222e);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncProviderRegistration_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub CreateSyncProviderConfigUIRegistrationInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pconfiguiconfig: *const SyncProviderConfigUIConfiguration, ppconfiguiinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_PropertiesSystem")))]
    CreateSyncProviderConfigUIRegistrationInstance: usize,
    pub UnregisterSyncProviderConfigUI: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub EnumerateSyncProviderConfigUIs: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidcontenttype: *const ::windows::core::GUID, dwsupportedarchitecture: u32, ppenumsyncproviderconfiguiinfos: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub CreateSyncProviderRegistrationInstance: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pproviderconfiguration: *const SyncProviderConfiguration, ppproviderinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    CreateSyncProviderRegistrationInstance: usize,
    pub UnregisterSyncProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetSyncProviderConfigUIInfoforProvider: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidproviderinstanceid: *const ::windows::core::GUID, ppproviderconfiguiinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetSyncProviderConfigUIInfoforProvider: usize,
    pub EnumerateSyncProviders: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidcontenttype: *const ::windows::core::GUID, dwstateflagstofiltermask: u32, dwstateflagstofilter: u32, refproviderclsid: *const ::windows::core::GUID, dwsupportedarchitecture: u32, ppenumsyncproviderinfos: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetSyncProviderInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID, ppproviderinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetSyncProviderInfo: usize,
    pub GetSyncProviderFromInstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID, dwclscontext: u32, ppsyncprovider: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
    pub GetSyncProviderConfigUIInfo: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID, ppconfiguiinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_UI_Shell_PropertiesSystem"))]
    GetSyncProviderConfigUIInfo: usize,
    pub GetSyncProviderConfigUIFromInstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID, dwclscontext: u32, ppconfigui: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSyncProviderState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID, pdwstateflags: *mut u32) -> ::windows::core::HRESULT,
    pub SetSyncProviderState: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidinstanceid: *const ::windows::core::GUID, dwstateflagsmask: u32, dwstateflags: u32) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub RegisterForEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phevent: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RegisterForEvent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RevokeEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RevokeEvent: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, hevent: super::super::Foundation::HANDLE, ppchange: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetChange: usize,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct ISyncRegistrationChange(::windows::core::IUnknown);
impl ISyncRegistrationChange {
    pub unsafe fn GetEvent(&self) -> ::windows::core::Result<SYNC_REGISTRATION_EVENT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetEvent)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<SYNC_REGISTRATION_EVENT>(result__)
    }
    pub unsafe fn GetInstanceId(&self) -> ::windows::core::Result<::windows::core::GUID> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetInstanceId)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::GUID>(result__)
    }
}
impl ::core::convert::From<ISyncRegistrationChange> for ::windows::core::IUnknown {
    fn from(value: ISyncRegistrationChange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISyncRegistrationChange> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISyncRegistrationChange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncRegistrationChange> for ::windows::core::IUnknown {
    fn from(value: &ISyncRegistrationChange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISyncRegistrationChange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncRegistrationChange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncRegistrationChange {}
impl ::core::fmt::Debug for ISyncRegistrationChange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncRegistrationChange").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISyncRegistrationChange {
    type Vtable = ISyncRegistrationChange_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xeea0d9ae_6b29_43b4_9e70_e3ae33bb2c3b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncRegistrationChange_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetEvent: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psreevent: *mut SYNC_REGISTRATION_EVENT) -> ::windows::core::HRESULT,
    pub GetInstanceId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pguidinstanceid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct ISyncSessionExtendedErrorInfo(::windows::core::IUnknown);
impl ISyncSessionExtendedErrorInfo {
    pub unsafe fn GetSyncProviderWithError(&self) -> ::windows::core::Result<ISyncProvider> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).GetSyncProviderWithError)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<ISyncProvider>(result__)
    }
}
impl ::core::convert::From<ISyncSessionExtendedErrorInfo> for ::windows::core::IUnknown {
    fn from(value: ISyncSessionExtendedErrorInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISyncSessionExtendedErrorInfo> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISyncSessionExtendedErrorInfo) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncSessionExtendedErrorInfo> for ::windows::core::IUnknown {
    fn from(value: &ISyncSessionExtendedErrorInfo) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISyncSessionExtendedErrorInfo {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncSessionExtendedErrorInfo {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncSessionExtendedErrorInfo {}
impl ::core::fmt::Debug for ISyncSessionExtendedErrorInfo {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncSessionExtendedErrorInfo").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISyncSessionExtendedErrorInfo {
    type Vtable = ISyncSessionExtendedErrorInfo_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x326c6810_790a_409b_b741_6999388761eb);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncSessionExtendedErrorInfo_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    pub GetSyncProviderWithError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppproviderwitherror: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct ISyncSessionState(::windows::core::IUnknown);
impl ISyncSessionState {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCanceled(&self, pfiscanceled: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsCanceled)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pfiscanceled)).ok()
    }
    pub unsafe fn GetInfoForChangeApplication(&self, pbchangeapplierinfo: *mut u8, pcbchangeapplierinfo: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetInfoForChangeApplication)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbchangeapplierinfo), ::core::mem::transmute(pcbchangeapplierinfo)).ok()
    }
    pub unsafe fn LoadInfoFromChangeApplication(&self, pbchangeapplierinfo: *const u8, cbchangeapplierinfo: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).LoadInfoFromChangeApplication)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbchangeapplierinfo), cbchangeapplierinfo).ok()
    }
    pub unsafe fn GetForgottenKnowledgeRecoveryRangeStart(&self, pbrangestart: *mut u8, pcbrangestart: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetForgottenKnowledgeRecoveryRangeStart)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbrangestart), ::core::mem::transmute(pcbrangestart)).ok()
    }
    pub unsafe fn GetForgottenKnowledgeRecoveryRangeEnd(&self, pbrangeend: *mut u8, pcbrangeend: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetForgottenKnowledgeRecoveryRangeEnd)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbrangeend), ::core::mem::transmute(pcbrangeend)).ok()
    }
    pub unsafe fn SetForgottenKnowledgeRecoveryRange(&self, prange: *const SYNC_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetForgottenKnowledgeRecoveryRange)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(prange)).ok()
    }
    pub unsafe fn OnProgress(&self, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnProgress)(::windows::core::Interface::as_raw(self), provider, syncstage, dwcompletedwork, dwtotalwork).ok()
    }
}
impl ::core::convert::From<ISyncSessionState> for ::windows::core::IUnknown {
    fn from(value: ISyncSessionState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISyncSessionState> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISyncSessionState) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncSessionState> for ::windows::core::IUnknown {
    fn from(value: &ISyncSessionState) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISyncSessionState {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncSessionState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncSessionState {}
impl ::core::fmt::Debug for ISyncSessionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncSessionState").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISyncSessionState {
    type Vtable = ISyncSessionState_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xb8a940fe_9f01_483b_9434_c37d361225d9);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncSessionState_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub IsCanceled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfiscanceled: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    IsCanceled: usize,
    pub GetInfoForChangeApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbchangeapplierinfo: *mut u8, pcbchangeapplierinfo: *mut u32) -> ::windows::core::HRESULT,
    pub LoadInfoFromChangeApplication: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbchangeapplierinfo: *const u8, cbchangeapplierinfo: u32) -> ::windows::core::HRESULT,
    pub GetForgottenKnowledgeRecoveryRangeStart: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbrangestart: *mut u8, pcbrangestart: *mut u32) -> ::windows::core::HRESULT,
    pub GetForgottenKnowledgeRecoveryRangeEnd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbrangeend: *mut u8, pcbrangeend: *mut u32) -> ::windows::core::HRESULT,
    pub SetForgottenKnowledgeRecoveryRange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, prange: *const SYNC_RANGE) -> ::windows::core::HRESULT,
    pub OnProgress: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct ISyncSessionState2(::windows::core::IUnknown);
impl ISyncSessionState2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCanceled(&self, pfiscanceled: *mut super::super::Foundation::BOOL) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.IsCanceled)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pfiscanceled)).ok()
    }
    pub unsafe fn GetInfoForChangeApplication(&self, pbchangeapplierinfo: *mut u8, pcbchangeapplierinfo: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetInfoForChangeApplication)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbchangeapplierinfo), ::core::mem::transmute(pcbchangeapplierinfo)).ok()
    }
    pub unsafe fn LoadInfoFromChangeApplication(&self, pbchangeapplierinfo: *const u8, cbchangeapplierinfo: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.LoadInfoFromChangeApplication)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbchangeapplierinfo), cbchangeapplierinfo).ok()
    }
    pub unsafe fn GetForgottenKnowledgeRecoveryRangeStart(&self, pbrangestart: *mut u8, pcbrangestart: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetForgottenKnowledgeRecoveryRangeStart)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbrangestart), ::core::mem::transmute(pcbrangestart)).ok()
    }
    pub unsafe fn GetForgottenKnowledgeRecoveryRangeEnd(&self, pbrangeend: *mut u8, pcbrangeend: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.GetForgottenKnowledgeRecoveryRangeEnd)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pbrangeend), ::core::mem::transmute(pcbrangeend)).ok()
    }
    pub unsafe fn SetForgottenKnowledgeRecoveryRange(&self, prange: *const SYNC_RANGE) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.SetForgottenKnowledgeRecoveryRange)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(prange)).ok()
    }
    pub unsafe fn OnProgress(&self, provider: SYNC_PROVIDER_ROLE, syncstage: SYNC_PROGRESS_STAGE, dwcompletedwork: u32, dwtotalwork: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base__.OnProgress)(::windows::core::Interface::as_raw(self), provider, syncstage, dwcompletedwork, dwtotalwork).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetProviderWithError<'a, P0>(&self, fself: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Interface::vtable(self).SetProviderWithError)(::windows::core::Interface::as_raw(self), fself.into()).ok()
    }
    pub unsafe fn GetSessionErrorStatus(&self, phrsessionerror: *mut ::windows::core::HRESULT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSessionErrorStatus)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(phrsessionerror)).ok()
    }
}
impl ::core::convert::From<ISyncSessionState2> for ::windows::core::IUnknown {
    fn from(value: ISyncSessionState2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISyncSessionState2> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISyncSessionState2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncSessionState2> for ::windows::core::IUnknown {
    fn from(value: &ISyncSessionState2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::convert::From<ISyncSessionState2> for ISyncSessionState {
    fn from(value: ISyncSessionState2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISyncSessionState2> for &'a ISyncSessionState {
    fn from(value: &'a ISyncSessionState2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISyncSessionState2> for ISyncSessionState {
    fn from(value: &ISyncSessionState2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISyncSessionState2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISyncSessionState2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISyncSessionState2 {}
impl ::core::fmt::Debug for ISyncSessionState2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISyncSessionState2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISyncSessionState2 {
    type Vtable = ISyncSessionState2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9e37cfa3_9e38_4c61_9ca3_ffe810b45ca2);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISyncSessionState2_Vtbl {
    pub base__: ISyncSessionState_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetProviderWithError: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fself: super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetProviderWithError: usize,
    pub GetSessionErrorStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, phrsessionerror: *mut ::windows::core::HRESULT) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
pub struct ISynchronousDataRetriever(::windows::core::IUnknown);
impl ISynchronousDataRetriever {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetIdParameters(&self, pidparameters: *mut ID_PARAMETERS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetIdParameters)(::windows::core::Interface::as_raw(self), ::core::mem::transmute(pidparameters)).ok()
    }
    pub unsafe fn LoadChangeData<'a, P0>(&self, ploadchangecontext: P0) -> ::windows::core::Result<::windows::core::IUnknown>
    where
        P0: ::std::convert::Into<::windows::core::InParam<'a, ILoadChangeContext>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Interface::vtable(self).LoadChangeData)(::windows::core::Interface::as_raw(self), ploadchangecontext.into().abi(), ::core::mem::transmute(result__.as_mut_ptr())).from_abi::<::windows::core::IUnknown>(result__)
    }
}
impl ::core::convert::From<ISynchronousDataRetriever> for ::windows::core::IUnknown {
    fn from(value: ISynchronousDataRetriever) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl<'a> ::core::convert::From<&'a ISynchronousDataRetriever> for &'a ::windows::core::IUnknown {
    fn from(value: &'a ISynchronousDataRetriever) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&ISynchronousDataRetriever> for ::windows::core::IUnknown {
    fn from(value: &ISynchronousDataRetriever) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl ::core::clone::Clone for ISynchronousDataRetriever {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for ISynchronousDataRetriever {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ISynchronousDataRetriever {}
impl ::core::fmt::Debug for ISynchronousDataRetriever {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ISynchronousDataRetriever").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for ISynchronousDataRetriever {
    type Vtable = ISynchronousDataRetriever_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x9b22f2a9_a4cd_4648_9d8e_3a510d4da04b);
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronousDataRetriever_Vtbl {
    pub base__: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub GetIdParameters: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pidparameters: *mut ID_PARAMETERS) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetIdParameters: usize,
    pub LoadChangeData: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ploadchangecontext: *mut ::core::ffi::c_void, ppunkdata: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct KNOWLEDGE_COOKIE_COMPARISON_RESULT(pub i32);
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const KCCR_COOKIE_KNOWLEDGE_EQUAL: KNOWLEDGE_COOKIE_COMPARISON_RESULT = KNOWLEDGE_COOKIE_COMPARISON_RESULT(0i32);
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const KCCR_COOKIE_KNOWLEDGE_CONTAINED: KNOWLEDGE_COOKIE_COMPARISON_RESULT = KNOWLEDGE_COOKIE_COMPARISON_RESULT(1i32);
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const KCCR_COOKIE_KNOWLEDGE_CONTAINS: KNOWLEDGE_COOKIE_COMPARISON_RESULT = KNOWLEDGE_COOKIE_COMPARISON_RESULT(2i32);
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const KCCR_COOKIE_KNOWLEDGE_NOT_COMPARABLE: KNOWLEDGE_COOKIE_COMPARISON_RESULT = KNOWLEDGE_COOKIE_COMPARISON_RESULT(3i32);
impl ::core::marker::Copy for KNOWLEDGE_COOKIE_COMPARISON_RESULT {}
impl ::core::clone::Clone for KNOWLEDGE_COOKIE_COMPARISON_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for KNOWLEDGE_COOKIE_COMPARISON_RESULT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for KNOWLEDGE_COOKIE_COMPARISON_RESULT {
    type Abi = Self;
}
impl ::core::fmt::Debug for KNOWLEDGE_COOKIE_COMPARISON_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("KNOWLEDGE_COOKIE_COMPARISON_RESULT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_CAPABILITIES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x554b24ea_e8e3_45ba_9352_dfb561e171e4), pid: 5u32 };
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_CLSID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x554b24ea_e8e3_45ba_9352_dfb561e171e4), pid: 3u32 };
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_CONTENTTYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x554b24ea_e8e3_45ba_9352_dfb561e171e4), pid: 4u32 };
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_DESCRIPTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x554b24ea_e8e3_45ba_9352_dfb561e171e4), pid: 9u32 };
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_ICON: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x554b24ea_e8e3_45ba_9352_dfb561e171e4), pid: 11u32 };
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_INSTANCEID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x554b24ea_e8e3_45ba_9352_dfb561e171e4), pid: 2u32 };
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_IS_GLOBAL: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x554b24ea_e8e3_45ba_9352_dfb561e171e4), pid: 7u32 };
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_MENUITEM: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x554b24ea_e8e3_45ba_9352_dfb561e171e4), pid: 13u32 };
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_MENUITEM_NOUI: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x554b24ea_e8e3_45ba_9352_dfb561e171e4), pid: 12u32 };
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x554b24ea_e8e3_45ba_9352_dfb561e171e4), pid: 8u32 };
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_SUPPORTED_ARCHITECTURE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x554b24ea_e8e3_45ba_9352_dfb561e171e4), pid: 6u32 };
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_CONFIGUI_TOOLTIPS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x554b24ea_e8e3_45ba_9352_dfb561e171e4), pid: 10u32 };
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_CAPABILITIES: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x84179e61_60f6_4c1c_88ed_f1c531b32bda), pid: 6u32 };
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_CLSID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x84179e61_60f6_4c1c_88ed_f1c531b32bda), pid: 3u32 };
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_CONFIGUI: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x84179e61_60f6_4c1c_88ed_f1c531b32bda), pid: 4u32 };
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_CONTENTTYPE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x84179e61_60f6_4c1c_88ed_f1c531b32bda), pid: 5u32 };
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_DESCRIPTION: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x84179e61_60f6_4c1c_88ed_f1c531b32bda), pid: 9u32 };
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_ICON: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x84179e61_60f6_4c1c_88ed_f1c531b32bda), pid: 11u32 };
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_INSTANCEID: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x84179e61_60f6_4c1c_88ed_f1c531b32bda), pid: 2u32 };
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_NAME: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x84179e61_60f6_4c1c_88ed_f1c531b32bda), pid: 8u32 };
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_SUPPORTED_ARCHITECTURE: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x84179e61_60f6_4c1c_88ed_f1c531b32bda), pid: 7u32 };
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_UI_Shell_PropertiesSystem\"`*"]
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const PKEY_PROVIDER_TOOLTIPS: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY { fmtid: ::windows::core::GUID::from_u128(0x84179e61_60f6_4c1c_88ed_f1c531b32bda), pid: 10u32 };
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SYNC_CHANGE_FLAG_DELETED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SYNC_CHANGE_FLAG_DOES_NOT_EXIST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SYNC_CHANGE_FLAG_GHOST: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SYNC_CONSTRAINT_RESOLVE_ACTION(pub i32);
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SCRA_DEFER: SYNC_CONSTRAINT_RESOLVE_ACTION = SYNC_CONSTRAINT_RESOLVE_ACTION(0i32);
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SCRA_ACCEPT_DESTINATION_PROVIDER: SYNC_CONSTRAINT_RESOLVE_ACTION = SYNC_CONSTRAINT_RESOLVE_ACTION(1i32);
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SCRA_ACCEPT_SOURCE_PROVIDER: SYNC_CONSTRAINT_RESOLVE_ACTION = SYNC_CONSTRAINT_RESOLVE_ACTION(2i32);
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SCRA_TRANSFER_AND_DEFER: SYNC_CONSTRAINT_RESOLVE_ACTION = SYNC_CONSTRAINT_RESOLVE_ACTION(3i32);
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SCRA_MERGE: SYNC_CONSTRAINT_RESOLVE_ACTION = SYNC_CONSTRAINT_RESOLVE_ACTION(4i32);
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SCRA_RENAME_SOURCE: SYNC_CONSTRAINT_RESOLVE_ACTION = SYNC_CONSTRAINT_RESOLVE_ACTION(5i32);
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SCRA_RENAME_DESTINATION: SYNC_CONSTRAINT_RESOLVE_ACTION = SYNC_CONSTRAINT_RESOLVE_ACTION(6i32);
impl ::core::marker::Copy for SYNC_CONSTRAINT_RESOLVE_ACTION {}
impl ::core::clone::Clone for SYNC_CONSTRAINT_RESOLVE_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYNC_CONSTRAINT_RESOLVE_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SYNC_CONSTRAINT_RESOLVE_ACTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for SYNC_CONSTRAINT_RESOLVE_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNC_CONSTRAINT_RESOLVE_ACTION").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SYNC_FILTER_CHANGE {
    pub fMoveIn: super::super::Foundation::BOOL,
    pub moveVersion: SYNC_VERSION,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SYNC_FILTER_CHANGE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SYNC_FILTER_CHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SYNC_FILTER_CHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYNC_FILTER_CHANGE").field("fMoveIn", &self.fMoveIn).field("moveVersion", &self.moveVersion).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SYNC_FILTER_CHANGE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SYNC_FILTER_CHANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SYNC_FILTER_CHANGE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SYNC_FILTER_CHANGE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SYNC_FILTER_CHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SYNC_FILTER_INFO_COMBINED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SYNC_FILTER_INFO_FLAG_CHANGE_UNIT_LIST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SYNC_FILTER_INFO_FLAG_CUSTOM: u32 = 4u32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SYNC_FILTER_INFO_FLAG_ITEM_LIST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SYNC_FULL_ENUMERATION_ACTION(pub i32);
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SFEA_FULL_ENUMERATION: SYNC_FULL_ENUMERATION_ACTION = SYNC_FULL_ENUMERATION_ACTION(0i32);
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SFEA_PARTIAL_SYNC: SYNC_FULL_ENUMERATION_ACTION = SYNC_FULL_ENUMERATION_ACTION(1i32);
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SFEA_ABORT: SYNC_FULL_ENUMERATION_ACTION = SYNC_FULL_ENUMERATION_ACTION(2i32);
impl ::core::marker::Copy for SYNC_FULL_ENUMERATION_ACTION {}
impl ::core::clone::Clone for SYNC_FULL_ENUMERATION_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYNC_FULL_ENUMERATION_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SYNC_FULL_ENUMERATION_ACTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for SYNC_FULL_ENUMERATION_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNC_FULL_ENUMERATION_ACTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SYNC_PROGRESS_STAGE(pub i32);
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SPS_CHANGE_DETECTION: SYNC_PROGRESS_STAGE = SYNC_PROGRESS_STAGE(0i32);
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SPS_CHANGE_ENUMERATION: SYNC_PROGRESS_STAGE = SYNC_PROGRESS_STAGE(1i32);
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SPS_CHANGE_APPLICATION: SYNC_PROGRESS_STAGE = SYNC_PROGRESS_STAGE(2i32);
impl ::core::marker::Copy for SYNC_PROGRESS_STAGE {}
impl ::core::clone::Clone for SYNC_PROGRESS_STAGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYNC_PROGRESS_STAGE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SYNC_PROGRESS_STAGE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SYNC_PROGRESS_STAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNC_PROGRESS_STAGE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SYNC_PROVIDER_ROLE(pub i32);
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SPR_SOURCE: SYNC_PROVIDER_ROLE = SYNC_PROVIDER_ROLE(0i32);
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SPR_DESTINATION: SYNC_PROVIDER_ROLE = SYNC_PROVIDER_ROLE(1i32);
impl ::core::marker::Copy for SYNC_PROVIDER_ROLE {}
impl ::core::clone::Clone for SYNC_PROVIDER_ROLE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYNC_PROVIDER_ROLE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SYNC_PROVIDER_ROLE {
    type Abi = Self;
}
impl ::core::fmt::Debug for SYNC_PROVIDER_ROLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNC_PROVIDER_ROLE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub struct SYNC_RANGE {
    pub pbClosedLowerBound: *mut u8,
    pub pbClosedUpperBound: *mut u8,
}
impl ::core::marker::Copy for SYNC_RANGE {}
impl ::core::clone::Clone for SYNC_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYNC_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYNC_RANGE").field("pbClosedLowerBound", &self.pbClosedLowerBound).field("pbClosedUpperBound", &self.pbClosedUpperBound).finish()
    }
}
unsafe impl ::windows::core::Abi for SYNC_RANGE {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SYNC_RANGE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SYNC_RANGE>()) == 0 }
    }
}
impl ::core::cmp::Eq for SYNC_RANGE {}
impl ::core::default::Default for SYNC_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SYNC_REGISTRATION_EVENT(pub i32);
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SRE_PROVIDER_ADDED: SYNC_REGISTRATION_EVENT = SYNC_REGISTRATION_EVENT(0i32);
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SRE_PROVIDER_REMOVED: SYNC_REGISTRATION_EVENT = SYNC_REGISTRATION_EVENT(1i32);
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SRE_PROVIDER_UPDATED: SYNC_REGISTRATION_EVENT = SYNC_REGISTRATION_EVENT(2i32);
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SRE_PROVIDER_STATE_CHANGED: SYNC_REGISTRATION_EVENT = SYNC_REGISTRATION_EVENT(3i32);
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SRE_CONFIGUI_ADDED: SYNC_REGISTRATION_EVENT = SYNC_REGISTRATION_EVENT(4i32);
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SRE_CONFIGUI_REMOVED: SYNC_REGISTRATION_EVENT = SYNC_REGISTRATION_EVENT(5i32);
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SRE_CONFIGUI_UPDATED: SYNC_REGISTRATION_EVENT = SYNC_REGISTRATION_EVENT(6i32);
impl ::core::marker::Copy for SYNC_REGISTRATION_EVENT {}
impl ::core::clone::Clone for SYNC_REGISTRATION_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYNC_REGISTRATION_EVENT {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SYNC_REGISTRATION_EVENT {
    type Abi = Self;
}
impl ::core::fmt::Debug for SYNC_REGISTRATION_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNC_REGISTRATION_EVENT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SYNC_RESOLVE_ACTION(pub i32);
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SRA_DEFER: SYNC_RESOLVE_ACTION = SYNC_RESOLVE_ACTION(0i32);
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SRA_ACCEPT_DESTINATION_PROVIDER: SYNC_RESOLVE_ACTION = SYNC_RESOLVE_ACTION(1i32);
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SRA_ACCEPT_SOURCE_PROVIDER: SYNC_RESOLVE_ACTION = SYNC_RESOLVE_ACTION(2i32);
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SRA_MERGE: SYNC_RESOLVE_ACTION = SYNC_RESOLVE_ACTION(3i32);
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SRA_TRANSFER_AND_DEFER: SYNC_RESOLVE_ACTION = SYNC_RESOLVE_ACTION(4i32);
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SRA_LAST: SYNC_RESOLVE_ACTION = SYNC_RESOLVE_ACTION(5i32);
impl ::core::marker::Copy for SYNC_RESOLVE_ACTION {}
impl ::core::clone::Clone for SYNC_RESOLVE_ACTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYNC_RESOLVE_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SYNC_RESOLVE_ACTION {
    type Abi = Self;
}
impl ::core::fmt::Debug for SYNC_RESOLVE_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNC_RESOLVE_ACTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SYNC_SERIALIZATION_VERSION(pub i32);
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SYNC_SERIALIZATION_VERSION_V1: SYNC_SERIALIZATION_VERSION = SYNC_SERIALIZATION_VERSION(1i32);
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SYNC_SERIALIZATION_VERSION_V2: SYNC_SERIALIZATION_VERSION = SYNC_SERIALIZATION_VERSION(4i32);
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SYNC_SERIALIZATION_VERSION_V3: SYNC_SERIALIZATION_VERSION = SYNC_SERIALIZATION_VERSION(5i32);
impl ::core::marker::Copy for SYNC_SERIALIZATION_VERSION {}
impl ::core::clone::Clone for SYNC_SERIALIZATION_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYNC_SERIALIZATION_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SYNC_SERIALIZATION_VERSION {
    type Abi = Self;
}
impl ::core::fmt::Debug for SYNC_SERIALIZATION_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNC_SERIALIZATION_VERSION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SYNC_SERIALIZE_REPLICA_KEY_MAP: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub struct SYNC_SESSION_STATISTICS {
    pub dwChangesApplied: u32,
    pub dwChangesFailed: u32,
}
impl ::core::marker::Copy for SYNC_SESSION_STATISTICS {}
impl ::core::clone::Clone for SYNC_SESSION_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYNC_SESSION_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYNC_SESSION_STATISTICS").field("dwChangesApplied", &self.dwChangesApplied).field("dwChangesFailed", &self.dwChangesFailed).finish()
    }
}
unsafe impl ::windows::core::Abi for SYNC_SESSION_STATISTICS {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SYNC_SESSION_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SYNC_SESSION_STATISTICS>()) == 0 }
    }
}
impl ::core::cmp::Eq for SYNC_SESSION_STATISTICS {}
impl ::core::default::Default for SYNC_SESSION_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct SYNC_STATISTICS(pub i32);
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SYNC_STATISTICS_RANGE_COUNT: SYNC_STATISTICS = SYNC_STATISTICS(0i32);
impl ::core::marker::Copy for SYNC_STATISTICS {}
impl ::core::clone::Clone for SYNC_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for SYNC_STATISTICS {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for SYNC_STATISTICS {
    type Abi = Self;
}
impl ::core::fmt::Debug for SYNC_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("SYNC_STATISTICS").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub struct SYNC_TIME {
    pub dwDate: u32,
    pub dwTime: u32,
}
impl ::core::marker::Copy for SYNC_TIME {}
impl ::core::clone::Clone for SYNC_TIME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYNC_TIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYNC_TIME").field("dwDate", &self.dwDate).field("dwTime", &self.dwTime).finish()
    }
}
unsafe impl ::windows::core::Abi for SYNC_TIME {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SYNC_TIME {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SYNC_TIME>()) == 0 }
    }
}
impl ::core::cmp::Eq for SYNC_TIME {}
impl ::core::default::Default for SYNC_TIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub struct SYNC_VERSION {
    pub dwLastUpdatingReplicaKey: u32,
    pub ullTickCount: u64,
}
impl ::core::marker::Copy for SYNC_VERSION {}
impl ::core::clone::Clone for SYNC_VERSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SYNC_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SYNC_VERSION").field("dwLastUpdatingReplicaKey", &self.dwLastUpdatingReplicaKey).field("ullTickCount", &self.ullTickCount).finish()
    }
}
unsafe impl ::windows::core::Abi for SYNC_VERSION {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SYNC_VERSION {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SYNC_VERSION>()) == 0 }
    }
}
impl ::core::cmp::Eq for SYNC_VERSION {}
impl ::core::default::Default for SYNC_VERSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SYNC_VERSION_FLAG_FROM_FEED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub const SYNC_VERSION_FLAG_HAS_BY: u32 = 2u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct SyncProviderConfigUIConfiguration {
    pub dwVersion: u32,
    pub guidInstanceId: ::windows::core::GUID,
    pub clsidConfigUI: ::windows::core::GUID,
    pub guidContentType: ::windows::core::GUID,
    pub dwCapabilities: u32,
    pub dwSupportedArchitecture: u32,
    pub fIsGlobal: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for SyncProviderConfigUIConfiguration {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for SyncProviderConfigUIConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for SyncProviderConfigUIConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SyncProviderConfigUIConfiguration").field("dwVersion", &self.dwVersion).field("guidInstanceId", &self.guidInstanceId).field("clsidConfigUI", &self.clsidConfigUI).field("guidContentType", &self.guidContentType).field("dwCapabilities", &self.dwCapabilities).field("dwSupportedArchitecture", &self.dwSupportedArchitecture).field("fIsGlobal", &self.fIsGlobal).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for SyncProviderConfigUIConfiguration {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for SyncProviderConfigUIConfiguration {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SyncProviderConfigUIConfiguration>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for SyncProviderConfigUIConfiguration {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for SyncProviderConfigUIConfiguration {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_WindowsSync\"`*"]
pub struct SyncProviderConfiguration {
    pub dwVersion: u32,
    pub guidInstanceId: ::windows::core::GUID,
    pub clsidProvider: ::windows::core::GUID,
    pub guidConfigUIInstanceId: ::windows::core::GUID,
    pub guidContentType: ::windows::core::GUID,
    pub dwCapabilities: u32,
    pub dwSupportedArchitecture: u32,
}
impl ::core::marker::Copy for SyncProviderConfiguration {}
impl ::core::clone::Clone for SyncProviderConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for SyncProviderConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("SyncProviderConfiguration").field("dwVersion", &self.dwVersion).field("guidInstanceId", &self.guidInstanceId).field("clsidProvider", &self.clsidProvider).field("guidConfigUIInstanceId", &self.guidConfigUIInstanceId).field("guidContentType", &self.guidContentType).field("dwCapabilities", &self.dwCapabilities).field("dwSupportedArchitecture", &self.dwSupportedArchitecture).finish()
    }
}
unsafe impl ::windows::core::Abi for SyncProviderConfiguration {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for SyncProviderConfiguration {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<SyncProviderConfiguration>()) == 0 }
    }
}
impl ::core::cmp::Eq for SyncProviderConfiguration {}
impl ::core::default::Default for SyncProviderConfiguration {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
pub const SyncProviderRegistration: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf82b4ef1_93a9_4dde_8015_f7950a1a6e31);
#[cfg(feature = "implement")]
::core::include!("impl.rs");
