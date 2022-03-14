#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
pub const CEventClass: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcdbec9c0_7a68_11d1_88f9_0080c7d771bf);
pub const CEventPublisher: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xab944620_79c6_11d1_88f9_0080c7d771bf);
pub const CEventSubscription: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7542e960_79c7_11d1_88f9_0080c7d771bf);
pub const CEventSystem: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e14fba2_2e22_11d1_9964_00c04fbbb345);
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct COMEVENTSYSCHANGEINFO {
    pub cbSize: u32,
    pub changeType: EOC_ChangeType,
    pub objectId: super::super::super::Foundation::BSTR,
    pub partitionId: super::super::super::Foundation::BSTR,
    pub applicationId: super::super::super::Foundation::BSTR,
    pub reserved: [::windows::core::GUID; 10],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for COMEVENTSYSCHANGEINFO {
    fn clone(&self) -> Self {
        Self {
            cbSize: self.cbSize,
            changeType: self.changeType,
            objectId: self.objectId.clone(),
            partitionId: self.partitionId.clone(),
            applicationId: self.applicationId.clone(),
            reserved: self.reserved,
        }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for COMEVENTSYSCHANGEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("COMEVENTSYSCHANGEINFO").field("cbSize", &self.cbSize).field("changeType", &self.changeType).field("objectId", &self.objectId).field("partitionId", &self.partitionId).field("applicationId", &self.applicationId).field("reserved", &self.reserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for COMEVENTSYSCHANGEINFO {
    type Abi = ::core::mem::ManuallyDrop<Self>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for COMEVENTSYSCHANGEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.cbSize == other.cbSize && self.changeType == other.changeType && self.objectId == other.objectId && self.partitionId == other.partitionId && self.applicationId == other.applicationId && self.reserved == other.reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for COMEVENTSYSCHANGEINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for COMEVENTSYSCHANGEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct EOC_ChangeType(pub i32);
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
pub const EOC_NewObject: EOC_ChangeType = EOC_ChangeType(0i32);
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
pub const EOC_ModifiedObject: EOC_ChangeType = EOC_ChangeType(1i32);
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
pub const EOC_DeletedObject: EOC_ChangeType = EOC_ChangeType(2i32);
impl ::core::marker::Copy for EOC_ChangeType {}
impl ::core::clone::Clone for EOC_ChangeType {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for EOC_ChangeType {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for EOC_ChangeType {
    type Abi = Self;
}
impl ::core::fmt::Debug for EOC_ChangeType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("EOC_ChangeType").field(&self.0).finish()
    }
}
pub const EventObjectChange: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xd0565000_9df4_11d1_a281_00c04fca0aa7);
pub const EventObjectChange2: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xbb07bacd_cd56_4e63_a8ff_cbf0355fb9f4);
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IDontSupportEventSubscription(::windows::core::IUnknown);
impl IDontSupportEventSubscription {}
impl ::core::convert::From<IDontSupportEventSubscription> for ::windows::core::IUnknown {
    fn from(value: IDontSupportEventSubscription) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IDontSupportEventSubscription> for ::windows::core::IUnknown {
    fn from(value: &IDontSupportEventSubscription) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IDontSupportEventSubscription {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IDontSupportEventSubscription {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IDontSupportEventSubscription {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IDontSupportEventSubscription {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDontSupportEventSubscription {}
impl ::core::fmt::Debug for IDontSupportEventSubscription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDontSupportEventSubscription").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDontSupportEventSubscription {
    type Vtable = IDontSupportEventSubscription_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x784121f1_62a6_4b89_855f_d65f296de83a);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDontSupportEventSubscription_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IEnumEventObject(::windows::core::IUnknown);
impl IEnumEventObject {
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumEventObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Clone)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumEventObject>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
    pub unsafe fn Next(&self, ppinterface: &mut [::core::option::Option<::windows::core::IUnknown>], cretelem: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::core::mem::transmute_copy(self), ppinterface.len() as _, ::core::mem::transmute(::windows::core::as_mut_ptr_or_null(ppinterface)), ::core::mem::transmute(cretelem)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
    pub unsafe fn Skip(&self, cskipelem: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::core::mem::transmute_copy(self), ::core::mem::transmute(cskipelem)).ok()
    }
}
impl ::core::convert::From<IEnumEventObject> for ::windows::core::IUnknown {
    fn from(value: IEnumEventObject) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEnumEventObject> for ::windows::core::IUnknown {
    fn from(value: &IEnumEventObject) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEnumEventObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEnumEventObject {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEnumEventObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEnumEventObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumEventObject {}
impl ::core::fmt::Debug for IEnumEventObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumEventObject").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumEventObject {
    type Vtable = IEnumEventObject_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf4a07d63_2e25_11d1_9964_00c04fbbb345);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumEventObject_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppinterface: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, creqelem: u32, ppinterface: *mut *mut ::core::ffi::c_void, cretelem: *mut u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, cskipelem: u32) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IEventClass(::windows::core::IUnknown);
impl IEventClass {
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EventClassID(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EventClassID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventClassID<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstreventclassid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEventClassID)(::core::mem::transmute_copy(self), bstreventclassid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EventClassName(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EventClassName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventClassName<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstreventclassname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEventClassName)(::core::mem::transmute_copy(self), bstreventclassname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OwnerSID(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).OwnerSID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetOwnerSID<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrownersid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOwnerSID)(::core::mem::transmute_copy(self), bstrownersid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FiringInterfaceID(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).FiringInterfaceID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFiringInterfaceID<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrfiringinterfaceid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFiringInterfaceID)(::core::mem::transmute_copy(self), bstrfiringinterfaceid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Description)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrdescription: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDescription)(::core::mem::transmute_copy(self), bstrdescription.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CustomConfigCLSID(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).CustomConfigCLSID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCustomConfigCLSID<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrcustomconfigclsid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetCustomConfigCLSID)(::core::mem::transmute_copy(self), bstrcustomconfigclsid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TypeLib(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).TypeLib)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTypeLib<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrtypelib: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetTypeLib)(::core::mem::transmute_copy(self), bstrtypelib.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IEventClass> for ::windows::core::IUnknown {
    fn from(value: IEventClass) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventClass> for ::windows::core::IUnknown {
    fn from(value: &IEventClass) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEventClass {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEventClass {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IEventClass> for super::IDispatch {
    fn from(value: IEventClass) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventClass> for super::IDispatch {
    fn from(value: &IEventClass) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IDispatch> for IEventClass {
    fn into_param(self) -> ::windows::core::Param<'a, super::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IDispatch> for &'a IEventClass {
    fn into_param(self) -> ::windows::core::Param<'a, super::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEventClass {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEventClass {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEventClass {}
impl ::core::fmt::Debug for IEventClass {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEventClass").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEventClass {
    type Vtable = IEventClass_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb2b72a0_7a68_11d1_88f9_0080c7d771bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventClass_Vtbl {
    pub base: super::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub EventClassID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstreventclassid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EventClassID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEventClassID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstreventclassid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEventClassID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EventClassName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstreventclassname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EventClassName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEventClassName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstreventclassname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEventClassName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OwnerSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrownersid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OwnerSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetOwnerSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrownersid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetOwnerSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FiringInterfaceID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrfiringinterfaceid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FiringInterfaceID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFiringInterfaceID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrfiringinterfaceid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFiringInterfaceID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDescription: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub CustomConfigCLSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrcustomconfigclsid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CustomConfigCLSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetCustomConfigCLSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrcustomconfigclsid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetCustomConfigCLSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub TypeLib: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrtypelib: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    TypeLib: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetTypeLib: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrtypelib: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetTypeLib: usize,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IEventClass2(::windows::core::IUnknown);
impl IEventClass2 {
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EventClassID(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.EventClassID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventClassID<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstreventclassid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetEventClassID)(::core::mem::transmute_copy(self), bstreventclassid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EventClassName(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.EventClassName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventClassName<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstreventclassname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetEventClassName)(::core::mem::transmute_copy(self), bstreventclassname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OwnerSID(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.OwnerSID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetOwnerSID<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrownersid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetOwnerSID)(::core::mem::transmute_copy(self), bstrownersid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FiringInterfaceID(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.FiringInterfaceID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFiringInterfaceID<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrfiringinterfaceid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetFiringInterfaceID)(::core::mem::transmute_copy(self), bstrfiringinterfaceid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.Description)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrdescription: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetDescription)(::core::mem::transmute_copy(self), bstrdescription.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CustomConfigCLSID(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.CustomConfigCLSID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetCustomConfigCLSID<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrcustomconfigclsid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetCustomConfigCLSID)(::core::mem::transmute_copy(self), bstrcustomconfigclsid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TypeLib(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).base.TypeLib)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetTypeLib<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrtypelib: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).base.SetTypeLib)(::core::mem::transmute_copy(self), bstrtypelib.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PublisherID(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PublisherID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPublisherID<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrpublisherid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPublisherID)(::core::mem::transmute_copy(self), bstrpublisherid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MultiInterfacePublisherFilterCLSID(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MultiInterfacePublisherFilterCLSID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMultiInterfacePublisherFilterCLSID<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrpubfilclsid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMultiInterfacePublisherFilterCLSID)(::core::mem::transmute_copy(self), bstrpubfilclsid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AllowInprocActivation(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: super::super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AllowInprocActivation)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllowInprocActivation<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fallowinprocactivation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAllowInprocActivation)(::core::mem::transmute_copy(self), fallowinprocactivation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FireInParallel(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: super::super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).FireInParallel)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFireInParallel<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, ffireinparallel: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFireInParallel)(::core::mem::transmute_copy(self), ffireinparallel.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IEventClass2> for ::windows::core::IUnknown {
    fn from(value: IEventClass2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventClass2> for ::windows::core::IUnknown {
    fn from(value: &IEventClass2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEventClass2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEventClass2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IEventClass2> for super::IDispatch {
    fn from(value: IEventClass2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventClass2> for super::IDispatch {
    fn from(value: &IEventClass2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IDispatch> for IEventClass2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IDispatch> for &'a IEventClass2 {
    fn into_param(self) -> ::windows::core::Param<'a, super::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IEventClass2> for IEventClass {
    fn from(value: IEventClass2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventClass2> for IEventClass {
    fn from(value: &IEventClass2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, IEventClass> for IEventClass2 {
    fn into_param(self) -> ::windows::core::Param<'a, IEventClass> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, IEventClass> for &'a IEventClass2 {
    fn into_param(self) -> ::windows::core::Param<'a, IEventClass> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEventClass2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEventClass2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEventClass2 {}
impl ::core::fmt::Debug for IEventClass2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEventClass2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEventClass2 {
    type Vtable = IEventClass2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xfb2b72a1_7a68_11d1_88f9_0080c7d771bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventClass2_Vtbl {
    pub base: IEventClass_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub PublisherID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpublisherid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PublisherID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPublisherID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpublisherid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPublisherID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MultiInterfacePublisherFilterCLSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpubfilclsid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MultiInterfacePublisherFilterCLSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMultiInterfacePublisherFilterCLSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpubfilclsid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMultiInterfacePublisherFilterCLSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AllowInprocActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfallowinprocactivation: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AllowInprocActivation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAllowInprocActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fallowinprocactivation: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAllowInprocActivation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FireInParallel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pffireinparallel: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FireInParallel: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFireInParallel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ffireinparallel: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFireInParallel: usize,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IEventControl(::windows::core::IUnknown);
impl IEventControl {
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPublisherFilter<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, IPublisherFilter>>(&self, methodname: Param0, ppublisherfilter: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPublisherFilter)(::core::mem::transmute_copy(self), methodname.into_param().abi(), ppublisherfilter.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AllowInprocActivation(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: super::super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AllowInprocActivation)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllowInprocActivation<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fallowinprocactivation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAllowInprocActivation)(::core::mem::transmute_copy(self), fallowinprocactivation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSubscriptions<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, methodname: Param0, optionalcriteria: Param1, optionalerrorindex: *const i32) -> ::windows::core::Result<IEventObjectCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSubscriptions)(::core::mem::transmute_copy(self), methodname.into_param().abi(), optionalcriteria.into_param().abi(), ::core::mem::transmute(optionalerrorindex), ::core::mem::transmute(&mut result__)).from_abi::<IEventObjectCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDefaultQuery<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, methodname: Param0, criteria: Param1) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SetDefaultQuery)(::core::mem::transmute_copy(self), methodname.into_param().abi(), criteria.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
}
impl ::core::convert::From<IEventControl> for ::windows::core::IUnknown {
    fn from(value: IEventControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventControl> for ::windows::core::IUnknown {
    fn from(value: &IEventControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEventControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEventControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IEventControl> for super::IDispatch {
    fn from(value: IEventControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventControl> for super::IDispatch {
    fn from(value: &IEventControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IDispatch> for IEventControl {
    fn into_param(self) -> ::windows::core::Param<'a, super::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IDispatch> for &'a IEventControl {
    fn into_param(self) -> ::windows::core::Param<'a, super::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEventControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEventControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEventControl {}
impl ::core::fmt::Debug for IEventControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEventControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEventControl {
    type Vtable = IEventControl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0343e2f4_86f6_11d1_b760_00c04fb926af);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventControl_Vtbl {
    pub base: super::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPublisherFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, methodname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppublisherfilter: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPublisherFilter: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AllowInprocActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfallowinprocactivation: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AllowInprocActivation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAllowInprocActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fallowinprocactivation: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAllowInprocActivation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSubscriptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, methodname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, optionalcriteria: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, optionalerrorindex: *const i32, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSubscriptions: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDefaultQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, methodname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, criteria: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, errorindex: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDefaultQuery: usize,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IEventObjectChange(::windows::core::IUnknown);
impl IEventObjectChange {
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ChangedSubscription<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, changetype: EOC_ChangeType, bstrsubscriptionid: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ChangedSubscription)(::core::mem::transmute_copy(self), ::core::mem::transmute(changetype), bstrsubscriptionid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ChangedEventClass<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, changetype: EOC_ChangeType, bstreventclassid: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ChangedEventClass)(::core::mem::transmute_copy(self), ::core::mem::transmute(changetype), bstreventclassid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ChangedPublisher<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, changetype: EOC_ChangeType, bstrpublisherid: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ChangedPublisher)(::core::mem::transmute_copy(self), ::core::mem::transmute(changetype), bstrpublisherid.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IEventObjectChange> for ::windows::core::IUnknown {
    fn from(value: IEventObjectChange) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventObjectChange> for ::windows::core::IUnknown {
    fn from(value: &IEventObjectChange) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEventObjectChange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEventObjectChange {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEventObjectChange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEventObjectChange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEventObjectChange {}
impl ::core::fmt::Debug for IEventObjectChange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEventObjectChange").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEventObjectChange {
    type Vtable = IEventObjectChange_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf4a07d70_2e25_11d1_9964_00c04fbbb345);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventObjectChange_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ChangedSubscription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changetype: EOC_ChangeType, bstrsubscriptionid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ChangedSubscription: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ChangedEventClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changetype: EOC_ChangeType, bstreventclassid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ChangedEventClass: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ChangedPublisher: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, changetype: EOC_ChangeType, bstrpublisherid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ChangedPublisher: usize,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IEventObjectChange2(::windows::core::IUnknown);
impl IEventObjectChange2 {
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ChangedSubscription(&self, pinfo: *const COMEVENTSYSCHANGEINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ChangedSubscription)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ChangedEventClass(&self, pinfo: *const COMEVENTSYSCHANGEINFO) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).ChangedEventClass)(::core::mem::transmute_copy(self), ::core::mem::transmute(pinfo)).ok()
    }
}
impl ::core::convert::From<IEventObjectChange2> for ::windows::core::IUnknown {
    fn from(value: IEventObjectChange2) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventObjectChange2> for ::windows::core::IUnknown {
    fn from(value: &IEventObjectChange2) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEventObjectChange2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEventObjectChange2 {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEventObjectChange2 {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEventObjectChange2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEventObjectChange2 {}
impl ::core::fmt::Debug for IEventObjectChange2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEventObjectChange2").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEventObjectChange2 {
    type Vtable = IEventObjectChange2_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x7701a9c3_bd68_438f_83e0_67bf4f53a422);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventObjectChange2_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub ChangedSubscription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMEVENTSYSCHANGEINFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ChangedSubscription: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub ChangedEventClass: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pinfo: *const COMEVENTSYSCHANGEINFO) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    ChangedEventClass: usize,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IEventObjectCollection(::windows::core::IUnknown);
impl IEventObjectCollection {
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
    pub unsafe fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self)._NewEnum)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn Item<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, objectid: Param0) -> ::windows::core::Result<super::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Item)(::core::mem::transmute_copy(self), objectid.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
    pub unsafe fn NewEnum(&self) -> ::windows::core::Result<IEnumEventObject> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).NewEnum)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEnumEventObject>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
    pub unsafe fn Count(&self) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Count)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn Add<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, item: *const super::VARIANT, objectid: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Add)(::core::mem::transmute_copy(self), ::core::mem::transmute(item), objectid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Remove<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, objectid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Remove)(::core::mem::transmute_copy(self), objectid.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IEventObjectCollection> for ::windows::core::IUnknown {
    fn from(value: IEventObjectCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventObjectCollection> for ::windows::core::IUnknown {
    fn from(value: &IEventObjectCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEventObjectCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEventObjectCollection {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IEventObjectCollection> for super::IDispatch {
    fn from(value: IEventObjectCollection) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventObjectCollection> for super::IDispatch {
    fn from(value: &IEventObjectCollection) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IDispatch> for IEventObjectCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IDispatch> for &'a IEventObjectCollection {
    fn into_param(self) -> ::windows::core::Param<'a, super::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEventObjectCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEventObjectCollection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEventObjectCollection {}
impl ::core::fmt::Debug for IEventObjectCollection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEventObjectCollection").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEventObjectCollection {
    type Vtable = IEventObjectCollection_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xf89ac270_d4eb_11d1_b682_00805fc79216);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventObjectCollection_Vtbl {
    pub base: super::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppunkenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub Item: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objectid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pitem: *mut super::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    Item: usize,
    pub NewEnum: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    pub Count: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub Add: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, item: *const super::VARIANT, objectid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    Add: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, objectid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Remove: usize,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IEventProperty(::windows::core::IUnknown);
impl IEventProperty {
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Name(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Name)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetName<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, propertyname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetName)(::core::mem::transmute_copy(self), propertyname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn Value(&self) -> ::windows::core::Result<super::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Value)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn SetValue(&self, propertyvalue: *const super::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetValue)(::core::mem::transmute_copy(self), ::core::mem::transmute(propertyvalue)).ok()
    }
}
impl ::core::convert::From<IEventProperty> for ::windows::core::IUnknown {
    fn from(value: IEventProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventProperty> for ::windows::core::IUnknown {
    fn from(value: &IEventProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEventProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEventProperty {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IEventProperty> for super::IDispatch {
    fn from(value: IEventProperty) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventProperty> for super::IDispatch {
    fn from(value: &IEventProperty) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IDispatch> for IEventProperty {
    fn into_param(self) -> ::windows::core::Param<'a, super::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IDispatch> for &'a IEventProperty {
    fn into_param(self) -> ::windows::core::Param<'a, super::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEventProperty {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEventProperty {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEventProperty {}
impl ::core::fmt::Debug for IEventProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEventProperty").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEventProperty {
    type Vtable = IEventProperty_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xda538ee2_f4de_11d1_b6bb_00805fc79216);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventProperty_Vtbl {
    pub base: super::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Name: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Name: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub Value: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyvalue: *mut super::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    Value: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub SetValue: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, propertyvalue: *const super::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    SetValue: usize,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IEventPublisher(::windows::core::IUnknown);
impl IEventPublisher {
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PublisherID(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PublisherID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPublisherID<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrpublisherid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPublisherID)(::core::mem::transmute_copy(self), bstrpublisherid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PublisherName(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PublisherName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPublisherName<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrpublishername: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPublisherName)(::core::mem::transmute_copy(self), bstrpublishername.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PublisherType(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PublisherType)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPublisherType<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrpublishertype: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPublisherType)(::core::mem::transmute_copy(self), bstrpublishertype.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OwnerSID(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).OwnerSID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetOwnerSID<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrownersid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOwnerSID)(::core::mem::transmute_copy(self), bstrownersid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Description)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrdescription: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDescription)(::core::mem::transmute_copy(self), bstrdescription.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetDefaultProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrpropertyname: Param0) -> ::windows::core::Result<super::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDefaultProperty)(::core::mem::transmute_copy(self), bstrpropertyname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn PutDefaultProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrpropertyname: Param0, propertyvalue: *const super::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PutDefaultProperty)(::core::mem::transmute_copy(self), bstrpropertyname.into_param().abi(), ::core::mem::transmute(propertyvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoveDefaultProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrpropertyname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveDefaultProperty)(::core::mem::transmute_copy(self), bstrpropertyname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
    pub unsafe fn GetDefaultPropertyCollection(&self) -> ::windows::core::Result<IEventObjectCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetDefaultPropertyCollection)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEventObjectCollection>(result__)
    }
}
impl ::core::convert::From<IEventPublisher> for ::windows::core::IUnknown {
    fn from(value: IEventPublisher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventPublisher> for ::windows::core::IUnknown {
    fn from(value: &IEventPublisher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEventPublisher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEventPublisher {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IEventPublisher> for super::IDispatch {
    fn from(value: IEventPublisher) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventPublisher> for super::IDispatch {
    fn from(value: &IEventPublisher) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IDispatch> for IEventPublisher {
    fn into_param(self) -> ::windows::core::Param<'a, super::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IDispatch> for &'a IEventPublisher {
    fn into_param(self) -> ::windows::core::Param<'a, super::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEventPublisher {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEventPublisher {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEventPublisher {}
impl ::core::fmt::Debug for IEventPublisher {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEventPublisher").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEventPublisher {
    type Vtable = IEventPublisher_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe341516b_2e32_11d1_9964_00c04fbbb345);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventPublisher_Vtbl {
    pub base: super::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub PublisherID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpublisherid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PublisherID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPublisherID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpublisherid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPublisherID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PublisherName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpublishername: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PublisherName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPublisherName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpublishername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPublisherName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PublisherType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpublishertype: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PublisherType: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPublisherType: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpublishertype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPublisherType: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OwnerSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrownersid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OwnerSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetOwnerSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrownersid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetOwnerSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDescription: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub GetDefaultProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertyvalue: *mut super::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    GetDefaultProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub PutDefaultProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertyvalue: *const super::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    PutDefaultProperty: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RemoveDefaultProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemoveDefaultProperty: usize,
    pub GetDefaultPropertyCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IEventSubscription(::windows::core::IUnknown);
impl IEventSubscription {
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SubscriptionID(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SubscriptionID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSubscriptionID<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrsubscriptionid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSubscriptionID)(::core::mem::transmute_copy(self), bstrsubscriptionid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SubscriptionName(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SubscriptionName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSubscriptionName<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrsubscriptionname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSubscriptionName)(::core::mem::transmute_copy(self), bstrsubscriptionname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PublisherID(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PublisherID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPublisherID<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrpublisherid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPublisherID)(::core::mem::transmute_copy(self), bstrpublisherid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EventClassID(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EventClassID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventClassID<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstreventclassid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEventClassID)(::core::mem::transmute_copy(self), bstreventclassid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MethodName(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MethodName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMethodName<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrmethodname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMethodName)(::core::mem::transmute_copy(self), bstrmethodname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SubscriberCLSID(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SubscriberCLSID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetSubscriberCLSID<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrsubscriberclsid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSubscriberCLSID)(::core::mem::transmute_copy(self), bstrsubscriberclsid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
    pub unsafe fn SubscriberInterface(&self) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SubscriberInterface)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
    pub unsafe fn SetSubscriberInterface<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, psubscriberinterface: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetSubscriberInterface)(::core::mem::transmute_copy(self), psubscriberinterface.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PerUser(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: super::super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).PerUser)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPerUser<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fperuser: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetPerUser)(::core::mem::transmute_copy(self), fperuser.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OwnerSID(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).OwnerSID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetOwnerSID<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrownersid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetOwnerSID)(::core::mem::transmute_copy(self), bstrownersid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Enabled(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: super::super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Enabled)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnabled<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fenabled: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetEnabled)(::core::mem::transmute_copy(self), fenabled.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Description(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Description)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDescription<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrdescription: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetDescription)(::core::mem::transmute_copy(self), bstrdescription.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MachineName(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).MachineName)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMachineName<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrmachinename: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMachineName)(::core::mem::transmute_copy(self), bstrmachinename.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetPublisherProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrpropertyname: Param0) -> ::windows::core::Result<super::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPublisherProperty)(::core::mem::transmute_copy(self), bstrpropertyname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn PutPublisherProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrpropertyname: Param0, propertyvalue: *const super::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PutPublisherProperty)(::core::mem::transmute_copy(self), bstrpropertyname.into_param().abi(), ::core::mem::transmute(propertyvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemovePublisherProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrpropertyname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemovePublisherProperty)(::core::mem::transmute_copy(self), bstrpropertyname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
    pub unsafe fn GetPublisherPropertyCollection(&self) -> ::windows::core::Result<IEventObjectCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetPublisherPropertyCollection)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEventObjectCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn GetSubscriberProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrpropertyname: Param0) -> ::windows::core::Result<super::VARIANT> {
        let mut result__: ::core::mem::ManuallyDrop<super::VARIANT> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSubscriberProperty)(::core::mem::transmute_copy(self), bstrpropertyname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<super::VARIANT>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`, `\"Win32_System_Ole\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub unsafe fn PutSubscriberProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrpropertyname: Param0, propertyvalue: *const super::VARIANT) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PutSubscriberProperty)(::core::mem::transmute_copy(self), bstrpropertyname.into_param().abi(), ::core::mem::transmute(propertyvalue)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoveSubscriberProperty<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrpropertyname: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveSubscriberProperty)(::core::mem::transmute_copy(self), bstrpropertyname.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
    pub unsafe fn GetSubscriberPropertyCollection(&self) -> ::windows::core::Result<IEventObjectCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSubscriberPropertyCollection)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<IEventObjectCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn InterfaceID(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).InterfaceID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetInterfaceID<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, bstrinterfaceid: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetInterfaceID)(::core::mem::transmute_copy(self), bstrinterfaceid.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IEventSubscription> for ::windows::core::IUnknown {
    fn from(value: IEventSubscription) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventSubscription> for ::windows::core::IUnknown {
    fn from(value: &IEventSubscription) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEventSubscription {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEventSubscription {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IEventSubscription> for super::IDispatch {
    fn from(value: IEventSubscription) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventSubscription> for super::IDispatch {
    fn from(value: &IEventSubscription) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IDispatch> for IEventSubscription {
    fn into_param(self) -> ::windows::core::Param<'a, super::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IDispatch> for &'a IEventSubscription {
    fn into_param(self) -> ::windows::core::Param<'a, super::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEventSubscription {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEventSubscription {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEventSubscription {}
impl ::core::fmt::Debug for IEventSubscription {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEventSubscription").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEventSubscription {
    type Vtable = IEventSubscription_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4a6b0e15_2e38_11d1_9965_00c04fbbb345);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventSubscription_Vtbl {
    pub base: super::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub SubscriptionID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsubscriptionid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SubscriptionID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSubscriptionID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsubscriptionid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSubscriptionID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SubscriptionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsubscriptionname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SubscriptionName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSubscriptionName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsubscriptionname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSubscriptionName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PublisherID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrpublisherid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PublisherID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPublisherID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpublisherid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPublisherID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EventClassID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstreventclassid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EventClassID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEventClassID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstreventclassid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEventClassID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MethodName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrmethodname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MethodName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMethodName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmethodname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMethodName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SubscriberCLSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrsubscriberclsid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SubscriberCLSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetSubscriberCLSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrsubscriberclsid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetSubscriberCLSID: usize,
    pub SubscriberInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppsubscriberinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub SetSubscriberInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psubscriberinterface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub PerUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfperuser: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PerUser: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetPerUser: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fperuser: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetPerUser: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub OwnerSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrownersid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    OwnerSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetOwnerSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrownersid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetOwnerSID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Enabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfenabled: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Enabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetEnabled: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fenabled: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetEnabled: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Description: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Description: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDescription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDescription: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub MachineName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrmachinename: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    MachineName: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetMachineName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrmachinename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetMachineName: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub GetPublisherProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertyvalue: *mut super::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    GetPublisherProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub PutPublisherProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertyvalue: *const super::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    PutPublisherProperty: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RemovePublisherProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemovePublisherProperty: usize,
    pub GetPublisherPropertyCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub GetSubscriberProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertyvalue: *mut super::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    GetSubscriberProperty: usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Ole"))]
    pub PutSubscriberProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertyvalue: *const super::VARIANT) -> ::windows::core::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Ole")))]
    PutSubscriberProperty: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RemoveSubscriberProperty: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemoveSubscriberProperty: usize,
    pub GetSubscriberPropertyCollection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, collection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub InterfaceID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstrinterfaceid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    InterfaceID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetInterfaceID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, bstrinterfaceid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetInterfaceID: usize,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IEventSystem(::windows::core::IUnknown);
impl IEventSystem {
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Query<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, progid: Param0, querycriteria: Param1, errorindex: *mut i32, ppinterface: *mut ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Query)(::core::mem::transmute_copy(self), progid.into_param().abi(), querycriteria.into_param().abi(), ::core::mem::transmute(errorindex), ::core::mem::transmute(ppinterface)).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Store<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::IUnknown>>(&self, progid: Param0, pinterface: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Store)(::core::mem::transmute_copy(self), progid.into_param().abi(), pinterface.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Remove<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, progid: Param0, querycriteria: Param1) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).Remove)(::core::mem::transmute_copy(self), progid.into_param().abi(), querycriteria.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EventObjectChangeEventClassID(&self) -> ::windows::core::Result<super::super::super::Foundation::BSTR> {
        let mut result__: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR> = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).EventObjectChangeEventClassID)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BSTR>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn QueryS<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, progid: Param0, querycriteria: Param1) -> ::windows::core::Result<::windows::core::IUnknown> {
        let mut result__: *mut ::core::ffi::c_void = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).QueryS)(::core::mem::transmute_copy(self), progid.into_param().abi(), querycriteria.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<::windows::core::IUnknown>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RemoveS<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, progid: Param0, querycriteria: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).RemoveS)(::core::mem::transmute_copy(self), progid.into_param().abi(), querycriteria.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IEventSystem> for ::windows::core::IUnknown {
    fn from(value: IEventSystem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventSystem> for ::windows::core::IUnknown {
    fn from(value: &IEventSystem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IEventSystem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IEventSystem {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IEventSystem> for super::IDispatch {
    fn from(value: IEventSystem) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IEventSystem> for super::IDispatch {
    fn from(value: &IEventSystem) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IDispatch> for IEventSystem {
    fn into_param(self) -> ::windows::core::Param<'a, super::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IDispatch> for &'a IEventSystem {
    fn into_param(self) -> ::windows::core::Param<'a, super::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IEventSystem {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IEventSystem {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEventSystem {}
impl ::core::fmt::Debug for IEventSystem {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEventSystem").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEventSystem {
    type Vtable = IEventSystem_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x4e14fb9f_2e22_11d1_9964_00c04fbbb345);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEventSystem_Vtbl {
    pub base: super::IDispatch_Vtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Query: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, progid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, querycriteria: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, errorindex: *mut i32, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Query: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Store: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, progid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pinterface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Store: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub Remove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, progid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, querycriteria: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, errorindex: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Remove: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub EventObjectChangeEventClassID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pbstreventclassid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    EventObjectChangeEventClassID: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub QueryS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, progid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, querycriteria: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, ppinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    QueryS: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub RemoveS: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, progid: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, querycriteria: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    RemoveS: usize,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IFiringControl(::windows::core::IUnknown);
impl IFiringControl {
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
    pub unsafe fn FireSubscription<'a, Param0: ::windows::core::IntoParam<'a, IEventSubscription>>(&self, subscription: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).FireSubscription)(::core::mem::transmute_copy(self), subscription.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IFiringControl> for ::windows::core::IUnknown {
    fn from(value: IFiringControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFiringControl> for ::windows::core::IUnknown {
    fn from(value: &IFiringControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IFiringControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IFiringControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::convert::From<IFiringControl> for super::IDispatch {
    fn from(value: IFiringControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IFiringControl> for super::IDispatch {
    fn from(value: &IFiringControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IDispatch> for IFiringControl {
    fn into_param(self) -> ::windows::core::Param<'a, super::IDispatch> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, super::IDispatch> for &'a IFiringControl {
    fn into_param(self) -> ::windows::core::Param<'a, super::IDispatch> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IFiringControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IFiringControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IFiringControl {}
impl ::core::fmt::Debug for IFiringControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IFiringControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IFiringControl {
    type Vtable = IFiringControl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xe0498c93_4efe_11d1_9971_00c04fbbb345);
}
#[repr(C)]
#[doc(hidden)]
pub struct IFiringControl_Vtbl {
    pub base: super::IDispatch_Vtbl,
    pub FireSubscription: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, subscription: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IMultiInterfaceEventControl(::windows::core::IUnknown);
impl IMultiInterfaceEventControl {
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
    pub unsafe fn SetMultiInterfacePublisherFilter<'a, Param0: ::windows::core::IntoParam<'a, IMultiInterfacePublisherFilter>>(&self, classfilter: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetMultiInterfacePublisherFilter)(::core::mem::transmute_copy(self), classfilter.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSubscriptions<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, eventiid: *const ::windows::core::GUID, bstrmethodname: Param1, optionalcriteria: Param2, optionalerrorindex: *const i32) -> ::windows::core::Result<IEventObjectCollection> {
        let mut result__: ::windows::core::RawPtr = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).GetSubscriptions)(::core::mem::transmute_copy(self), ::core::mem::transmute(eventiid), bstrmethodname.into_param().abi(), optionalcriteria.into_param().abi(), ::core::mem::transmute(optionalerrorindex), ::core::mem::transmute(&mut result__)).from_abi::<IEventObjectCollection>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetDefaultQuery<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>>(&self, eventiid: *const ::windows::core::GUID, bstrmethodname: Param1, bstrcriteria: Param2) -> ::windows::core::Result<i32> {
        let mut result__: i32 = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).SetDefaultQuery)(::core::mem::transmute_copy(self), ::core::mem::transmute(eventiid), bstrmethodname.into_param().abi(), bstrcriteria.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<i32>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AllowInprocActivation(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: super::super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).AllowInprocActivation)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetAllowInprocActivation<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, fallowinprocactivation: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetAllowInprocActivation)(::core::mem::transmute_copy(self), fallowinprocactivation.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn FireInParallel(&self) -> ::windows::core::Result<super::super::super::Foundation::BOOL> {
        let mut result__: super::super::super::Foundation::BOOL = ::core::mem::zeroed();
        (::windows::core::Interface::vtable(self).FireInParallel)(::core::mem::transmute_copy(self), ::core::mem::transmute(&mut result__)).from_abi::<super::super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFireInParallel<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BOOL>>(&self, ffireinparallel: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).SetFireInParallel)(::core::mem::transmute_copy(self), ffireinparallel.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IMultiInterfaceEventControl> for ::windows::core::IUnknown {
    fn from(value: IMultiInterfaceEventControl) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMultiInterfaceEventControl> for ::windows::core::IUnknown {
    fn from(value: &IMultiInterfaceEventControl) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMultiInterfaceEventControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMultiInterfaceEventControl {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMultiInterfaceEventControl {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMultiInterfaceEventControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMultiInterfaceEventControl {}
impl ::core::fmt::Debug for IMultiInterfaceEventControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMultiInterfaceEventControl").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMultiInterfaceEventControl {
    type Vtable = IMultiInterfaceEventControl_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x0343e2f5_86f6_11d1_b760_00c04fb926af);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultiInterfaceEventControl_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub SetMultiInterfacePublisherFilter: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, classfilter: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub GetSubscriptions: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventiid: *const ::windows::core::GUID, bstrmethodname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, optionalcriteria: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, optionalerrorindex: *const i32, ppcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    GetSubscriptions: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetDefaultQuery: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, eventiid: *const ::windows::core::GUID, bstrmethodname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrcriteria: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, errorindex: *mut i32) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetDefaultQuery: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub AllowInprocActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfallowinprocactivation: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    AllowInprocActivation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetAllowInprocActivation: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, fallowinprocactivation: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetAllowInprocActivation: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub FireInParallel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pffireinparallel: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    FireInParallel: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub SetFireInParallel: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ffireinparallel: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    SetFireInParallel: usize,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IMultiInterfacePublisherFilter(::windows::core::IUnknown);
impl IMultiInterfacePublisherFilter {
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, IMultiInterfaceEventControl>>(&self, peic: Param0) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Initialize)(::core::mem::transmute_copy(self), peic.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PrepareToFire<'a, Param1: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param2: ::windows::core::IntoParam<'a, IFiringControl>>(&self, iid: *const ::windows::core::GUID, methodname: Param1, firingcontrol: Param2) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PrepareToFire)(::core::mem::transmute_copy(self), ::core::mem::transmute(iid), methodname.into_param().abi(), firingcontrol.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IMultiInterfacePublisherFilter> for ::windows::core::IUnknown {
    fn from(value: IMultiInterfacePublisherFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IMultiInterfacePublisherFilter> for ::windows::core::IUnknown {
    fn from(value: &IMultiInterfacePublisherFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IMultiInterfacePublisherFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IMultiInterfacePublisherFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IMultiInterfacePublisherFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IMultiInterfacePublisherFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IMultiInterfacePublisherFilter {}
impl ::core::fmt::Debug for IMultiInterfacePublisherFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IMultiInterfacePublisherFilter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IMultiInterfacePublisherFilter {
    type Vtable = IMultiInterfacePublisherFilter_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x465e5cc1_7b26_11d1_88fb_0080c7d771bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultiInterfacePublisherFilter_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, peic: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub PrepareToFire: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, iid: *const ::windows::core::GUID, methodname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, firingcontrol: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PrepareToFire: usize,
}
#[doc = "*Required features: `\"Win32_System_Com_Events\"`*"]
#[repr(transparent)]
pub struct IPublisherFilter(::windows::core::IUnknown);
impl IPublisherFilter {
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Initialize<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, super::IDispatch>>(&self, methodname: Param0, dispuserdefined: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Initialize)(::core::mem::transmute_copy(self), methodname.into_param().abi(), dispuserdefined.into_param().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_System_Com_Events\"`, `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn PrepareToFire<'a, Param0: ::windows::core::IntoParam<'a, super::super::super::Foundation::BSTR>, Param1: ::windows::core::IntoParam<'a, IFiringControl>>(&self, methodname: Param0, firingcontrol: Param1) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).PrepareToFire)(::core::mem::transmute_copy(self), methodname.into_param().abi(), firingcontrol.into_param().abi()).ok()
    }
}
impl ::core::convert::From<IPublisherFilter> for ::windows::core::IUnknown {
    fn from(value: IPublisherFilter) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
impl ::core::convert::From<&IPublisherFilter> for ::windows::core::IUnknown {
    fn from(value: &IPublisherFilter) -> Self {
        ::core::convert::From::from(::core::clone::Clone::clone(value))
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for IPublisherFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Owned(unsafe { ::core::mem::transmute(self) })
    }
}
impl<'a> ::windows::core::IntoParam<'a, ::windows::core::IUnknown> for &'a IPublisherFilter {
    fn into_param(self) -> ::windows::core::Param<'a, ::windows::core::IUnknown> {
        ::windows::core::Param::Borrowed(unsafe { ::core::mem::transmute(self) })
    }
}
impl ::core::clone::Clone for IPublisherFilter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl ::core::cmp::PartialEq for IPublisherFilter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IPublisherFilter {}
impl ::core::fmt::Debug for IPublisherFilter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPublisherFilter").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IPublisherFilter {
    type Vtable = IPublisherFilter_Vtbl;
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x465e5cc0_7b26_11d1_88fb_0080c7d771bf);
}
#[repr(C)]
#[doc(hidden)]
pub struct IPublisherFilter_Vtbl {
    pub base: ::windows::core::IUnknownVtbl,
    #[cfg(feature = "Win32_Foundation")]
    pub Initialize: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, methodname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, dispuserdefined: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Initialize: usize,
    #[cfg(feature = "Win32_Foundation")]
    pub PrepareToFire: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, methodname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, firingcontrol: ::windows::core::RawPtr) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    PrepareToFire: usize,
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
