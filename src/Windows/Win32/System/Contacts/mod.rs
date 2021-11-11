#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_System_Contacts`*"]
pub const CGD_ARRAY_NODE: u32 = 8u32;
#[doc = "*Required features: `Win32_System_Contacts`*"]
pub const CGD_BINARY_PROPERTY: u32 = 4u32;
#[doc = "*Required features: `Win32_System_Contacts`*"]
pub const CGD_DATE_PROPERTY: u32 = 2u32;
#[doc = "*Required features: `Win32_System_Contacts`*"]
pub const CGD_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_System_Contacts`*"]
pub const CGD_STRING_PROPERTY: u32 = 1u32;
#[doc = "*Required features: `Win32_System_Contacts`*"]
pub const CGD_UNKNOWN_PROPERTY: u32 = 0u32;
pub const CLSID_ContactAggregationManager: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x96c8ad95_c199_44de_b34e_ac33c442df39);
#[derive(:: core :: clone :: Clone, :: core :: marker :: Copy)]
#[repr(C)]
#[doc = "*Required features: `Win32_System_Contacts`*"]
pub struct CONTACT_AGGREGATION_BLOB {
    pub dwCount: u32,
    pub lpb: *mut u8,
}
impl CONTACT_AGGREGATION_BLOB {}
impl ::core::default::Default for CONTACT_AGGREGATION_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::fmt::Debug for CONTACT_AGGREGATION_BLOB {
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt.debug_struct("CONTACT_AGGREGATION_BLOB").field("dwCount", &self.dwCount).field("lpb", &self.lpb).finish()
    }
}
impl ::core::cmp::PartialEq for CONTACT_AGGREGATION_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.dwCount == other.dwCount && self.lpb == other.lpb
    }
}
impl ::core::cmp::Eq for CONTACT_AGGREGATION_BLOB {}
unsafe impl ::windows::runtime::Abi for CONTACT_AGGREGATION_BLOB {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Contacts`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CONTACT_AGGREGATION_COLLECTION_OPTIONS(pub i32);
pub const CACO_DEFAULT: CONTACT_AGGREGATION_COLLECTION_OPTIONS = CONTACT_AGGREGATION_COLLECTION_OPTIONS(0i32);
pub const CACO_INCLUDE_EXTERNAL: CONTACT_AGGREGATION_COLLECTION_OPTIONS = CONTACT_AGGREGATION_COLLECTION_OPTIONS(1i32);
pub const CACO_EXTERNAL_ONLY: CONTACT_AGGREGATION_COLLECTION_OPTIONS = CONTACT_AGGREGATION_COLLECTION_OPTIONS(2i32);
impl ::core::convert::From<i32> for CONTACT_AGGREGATION_COLLECTION_OPTIONS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CONTACT_AGGREGATION_COLLECTION_OPTIONS {
    type Abi = Self;
}
#[doc = "*Required features: `Win32_System_Contacts`*"]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
#[repr(transparent)]
pub struct CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS(pub i32);
pub const CA_CREATE_LOCAL: CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS = CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS(0i32);
pub const CA_CREATE_EXTERNAL: CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS = CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS(1i32);
impl ::core::convert::From<i32> for CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS {
    type Abi = Self;
}
pub const Contact: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x61b68808_8eee_4fd1_acb8_3d804c8db056);
pub const ContactManager: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7165c8ab_af88_42bd_86fd_5310b4285a02);
#[doc = "*Required features: `Win32_System_Contacts`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IContact(pub ::windows::runtime::IUnknown);
impl IContact {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn GetContactID<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszcontactid: Param0, cchcontactid: u32, pdwcchcontactidrequired: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pszcontactid.into_param().abi(), ::core::mem::transmute(cchcontactid), ::core::mem::transmute(pdwcchcontactidrequired)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn GetPath<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpath: Param0, cchpath: u32, pdwcchpathrequired: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pszpath.into_param().abi(), ::core::mem::transmute(cchpath), ::core::mem::transmute(pdwcchpathrequired)).ok()
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn CommitChanges(&self, dwcommitflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), ::core::mem::transmute(dwcommitflags)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IContact {
    type Vtable = IContact_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf941b671_bda7_4f77_884a_f46462f226a7);
}
impl ::core::convert::From<IContact> for ::windows::runtime::IUnknown {
    fn from(value: IContact) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IContact> for ::windows::runtime::IUnknown {
    fn from(value: &IContact) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IContact {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IContact {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContact_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszcontactid: super::super::Foundation::PWSTR, cchcontactid: u32, pdwcchcontactidrequired: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpath: super::super::Foundation::PWSTR, cchpath: u32, pdwcchpathrequired: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, dwcommitflags: u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Contacts`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IContactAggregationAggregate(pub ::windows::runtime::IUnknown);
impl IContactAggregationAggregate {
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn Save(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn GetComponentItems(&self) -> ::windows::runtime::Result<IContactAggregationContactCollection> {
        let mut result__: <IContactAggregationContactCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IContactAggregationContactCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn Link<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, paggregateid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), paggregateid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn Groups(&self, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS) -> ::windows::runtime::Result<IContactAggregationGroupCollection> {
        let mut result__: <IContactAggregationGroupCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(options), &mut result__).from_abi::<IContactAggregationGroupCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn AntiLink(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn SetAntiLink<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pantilink: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pantilink.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn FavoriteOrder(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn SetFavoriteOrder(&self, favoriteorder: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), ::core::mem::transmute(favoriteorder)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IContactAggregationAggregate {
    type Vtable = IContactAggregationAggregate_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7ed1c814_cd30_43c8_9b8d_2e489e53d54b);
}
impl ::core::convert::From<IContactAggregationAggregate> for ::windows::runtime::IUnknown {
    fn from(value: IContactAggregationAggregate) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IContactAggregationAggregate> for ::windows::runtime::IUnknown {
    fn from(value: &IContactAggregationAggregate) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IContactAggregationAggregate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IContactAggregationAggregate {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAggregationAggregate_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcomponentitems: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, paggregateid: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS, ppgroups: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppantilink: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pantilink: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfavoriteorder: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, favoriteorder: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppitemid: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Contacts`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IContactAggregationAggregateCollection(pub ::windows::runtime::IUnknown);
impl IContactAggregationAggregateCollection {
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn FindFirst(&self) -> ::windows::runtime::Result<IContactAggregationAggregate> {
        let mut result__: <IContactAggregationAggregate as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IContactAggregationAggregate>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn FindFirstByAntiLinkId<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pantilinkid: Param0) -> ::windows::runtime::Result<IContactAggregationAggregate> {
        let mut result__: <IContactAggregationAggregate as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pantilinkid.into_param().abi(), &mut result__).from_abi::<IContactAggregationAggregate>(result__)
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn FindNext(&self) -> ::windows::runtime::Result<IContactAggregationAggregate> {
        let mut result__: <IContactAggregationAggregate as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IContactAggregationAggregate>(result__)
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IContactAggregationAggregateCollection {
    type Vtable = IContactAggregationAggregateCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x2359f3a6_3a68_40af_98db_0f9eb143c3bb);
}
impl ::core::convert::From<IContactAggregationAggregateCollection> for ::windows::runtime::IUnknown {
    fn from(value: IContactAggregationAggregateCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IContactAggregationAggregateCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IContactAggregationAggregateCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IContactAggregationAggregateCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IContactAggregationAggregateCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAggregationAggregateCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppaggregate: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pantilinkid: super::super::Foundation::PWSTR, ppaggregate: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppaggregate: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcount: *mut i32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Contacts`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IContactAggregationContact(pub ::windows::runtime::IUnknown);
impl IContactAggregationContact {
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn Save(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn MoveToAggregate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, paggregateid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), paggregateid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn Unlink(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn AccountId(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn SetAccountId<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, paccountid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), paccountid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn AggregateId(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn IsMe(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn IsExternal(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn NetworkSourceId(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn SetNetworkSourceId(&self, networksourceid: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(networksourceid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn NetworkSourceIdString(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn SetNetworkSourceIdString<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pnetworksourceid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), pnetworksourceid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn RemoteObjectId(&self) -> ::windows::runtime::Result<*mut CONTACT_AGGREGATION_BLOB> {
        let mut result__: <*mut CONTACT_AGGREGATION_BLOB as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut CONTACT_AGGREGATION_BLOB>(result__)
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn SetRemoteObjectId(&self, premoteobjectid: *const CONTACT_AGGREGATION_BLOB) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(premoteobjectid)).ok()
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn SyncIdentityHash(&self) -> ::windows::runtime::Result<*mut CONTACT_AGGREGATION_BLOB> {
        let mut result__: <*mut CONTACT_AGGREGATION_BLOB as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut CONTACT_AGGREGATION_BLOB>(result__)
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn SetSyncIdentityHash(&self, psyncidentityhash: *const CONTACT_AGGREGATION_BLOB) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), ::core::mem::transmute(psyncidentityhash)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IContactAggregationContact {
    type Vtable = IContactAggregationContact_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1eb22e86_4c86_41f0_9f9f_c251e9fda6c3);
}
impl ::core::convert::From<IContactAggregationContact> for ::windows::runtime::IUnknown {
    fn from(value: IContactAggregationContact) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IContactAggregationContact> for ::windows::runtime::IUnknown {
    fn from(value: &IContactAggregationContact) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IContactAggregationContact {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IContactAggregationContact {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAggregationContact_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, paggregateid: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppaccountid: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, paccountid: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppaggregateid: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppitemid: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pisme: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pisexternal: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnetworksourceid: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, networksourceid: u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppnetworksourceid: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnetworksourceid: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppremoteobjectid: *mut *mut CONTACT_AGGREGATION_BLOB) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, premoteobjectid: *const CONTACT_AGGREGATION_BLOB) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppsyncidentityhash: *mut *mut CONTACT_AGGREGATION_BLOB) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psyncidentityhash: *const CONTACT_AGGREGATION_BLOB) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Contacts`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IContactAggregationContactCollection(pub ::windows::runtime::IUnknown);
impl IContactAggregationContactCollection {
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn FindFirst(&self) -> ::windows::runtime::Result<IContactAggregationContact> {
        let mut result__: <IContactAggregationContact as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IContactAggregationContact>(result__)
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn FindNext(&self) -> ::windows::runtime::Result<IContactAggregationContact> {
        let mut result__: <IContactAggregationContact as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IContactAggregationContact>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn FindFirstByIdentityHash<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, psourcetype: Param0, paccountid: Param1, pidentityhash: *const CONTACT_AGGREGATION_BLOB) -> ::windows::runtime::Result<IContactAggregationContact> {
        let mut result__: <IContactAggregationContact as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), psourcetype.into_param().abi(), paccountid.into_param().abi(), ::core::mem::transmute(pidentityhash), &mut result__).from_abi::<IContactAggregationContact>(result__)
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<i32> {
        let mut result__: <i32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<i32>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn FindFirstByRemoteId<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, psourcetype: Param0, paccountid: Param1, premoteobjectid: *const CONTACT_AGGREGATION_BLOB) -> ::windows::runtime::Result<IContactAggregationContact> {
        let mut result__: <IContactAggregationContact as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), psourcetype.into_param().abi(), paccountid.into_param().abi(), ::core::mem::transmute(premoteobjectid), &mut result__).from_abi::<IContactAggregationContact>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IContactAggregationContactCollection {
    type Vtable = IContactAggregationContactCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x826e66fa_81de_43ca_a6fb_8c785cd996c6);
}
impl ::core::convert::From<IContactAggregationContactCollection> for ::windows::runtime::IUnknown {
    fn from(value: IContactAggregationContactCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IContactAggregationContactCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IContactAggregationContactCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IContactAggregationContactCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IContactAggregationContactCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAggregationContactCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppitem: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppitem: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psourcetype: super::super::Foundation::PWSTR, paccountid: super::super::Foundation::PWSTR, pidentityhash: *const CONTACT_AGGREGATION_BLOB, ppitem: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcount: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psourcetype: super::super::Foundation::PWSTR, paccountid: super::super::Foundation::PWSTR, premoteobjectid: *const CONTACT_AGGREGATION_BLOB, ppitem: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Contacts`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IContactAggregationGroup(pub ::windows::runtime::IUnknown);
impl IContactAggregationGroup {
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn Save(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn Add<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, paggregateid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), paggregateid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn Remove<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, paggregateid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), paggregateid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn Members(&self) -> ::windows::runtime::Result<IContactAggregationAggregateCollection> {
        let mut result__: <IContactAggregationAggregateCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IContactAggregationAggregateCollection>(result__)
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn GlobalObjectId(&self) -> ::windows::runtime::Result<::windows::runtime::GUID> {
        let mut result__: <::windows::runtime::GUID as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<::windows::runtime::GUID>(result__)
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn SetGlobalObjectId(&self, pglobalobjectid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), ::core::mem::transmute(pglobalobjectid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn Name(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn SetName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pname: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), pname.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IContactAggregationGroup {
    type Vtable = IContactAggregationGroup_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xc93c545f_1284_499b_96af_07372af473e0);
}
impl ::core::convert::From<IContactAggregationGroup> for ::windows::runtime::IUnknown {
    fn from(value: IContactAggregationGroup) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IContactAggregationGroup> for ::windows::runtime::IUnknown {
    fn from(value: &IContactAggregationGroup) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IContactAggregationGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IContactAggregationGroup {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAggregationGroup_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, paggregateid: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, paggregateid: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppaggregatecontactcollection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pglobalobjectid: *mut ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pglobalobjectid: *const ::windows::runtime::GUID) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppitemid: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppname: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pname: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Contacts`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IContactAggregationGroupCollection(pub ::windows::runtime::IUnknown);
impl IContactAggregationGroupCollection {
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn FindFirst(&self) -> ::windows::runtime::Result<IContactAggregationGroup> {
        let mut result__: <IContactAggregationGroup as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IContactAggregationGroup>(result__)
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn FindFirstByGlobalObjectId(&self, pglobalobjectid: *const ::windows::runtime::GUID) -> ::windows::runtime::Result<IContactAggregationGroup> {
        let mut result__: <IContactAggregationGroup as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), ::core::mem::transmute(pglobalobjectid), &mut result__).from_abi::<IContactAggregationGroup>(result__)
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn FindNext(&self) -> ::windows::runtime::Result<IContactAggregationGroup> {
        let mut result__: <IContactAggregationGroup as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IContactAggregationGroup>(result__)
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IContactAggregationGroupCollection {
    type Vtable = IContactAggregationGroupCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x20a19a9c_d2f3_4b83_9143_beffd2cc226d);
}
impl ::core::convert::From<IContactAggregationGroupCollection> for ::windows::runtime::IUnknown {
    fn from(value: IContactAggregationGroupCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IContactAggregationGroupCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IContactAggregationGroupCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IContactAggregationGroupCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IContactAggregationGroupCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAggregationGroupCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppgroup: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pglobalobjectid: *const ::windows::runtime::GUID, ppgroup: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppgroup: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcount: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Contacts`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IContactAggregationLink(pub ::windows::runtime::IUnknown);
impl IContactAggregationLink {
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn Save(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn AccountId(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn SetAccountId<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, paccountid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), paccountid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn IsLinkResolved(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn SetIsLinkResolved<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, islinkresolved: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), islinkresolved.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn NetworkSourceIdString(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn SetNetworkSourceIdString<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pnetworksourceid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), pnetworksourceid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn RemoteObjectId(&self) -> ::windows::runtime::Result<*mut CONTACT_AGGREGATION_BLOB> {
        let mut result__: <*mut CONTACT_AGGREGATION_BLOB as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut CONTACT_AGGREGATION_BLOB>(result__)
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn SetRemoteObjectId(&self, premoteobjectid: *const CONTACT_AGGREGATION_BLOB) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(premoteobjectid)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn ServerPerson(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn SetServerPerson<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pserverpersonid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), pserverpersonid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn ServerPersonBaseline(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn SetServerPersonBaseline<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pserverpersonid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), pserverpersonid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn SyncIdentityHash(&self) -> ::windows::runtime::Result<*mut CONTACT_AGGREGATION_BLOB> {
        let mut result__: <*mut CONTACT_AGGREGATION_BLOB as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut CONTACT_AGGREGATION_BLOB>(result__)
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn SetSyncIdentityHash(&self, psyncidentityhash: *const CONTACT_AGGREGATION_BLOB) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), ::core::mem::transmute(psyncidentityhash)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IContactAggregationLink {
    type Vtable = IContactAggregationLink_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb6813323_a183_4654_8627_79b30de3a0ec);
}
impl ::core::convert::From<IContactAggregationLink> for ::windows::runtime::IUnknown {
    fn from(value: IContactAggregationLink) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IContactAggregationLink> for ::windows::runtime::IUnknown {
    fn from(value: &IContactAggregationLink) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IContactAggregationLink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IContactAggregationLink {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAggregationLink_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppaccountid: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, paccountid: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppitemid: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pislinkresolved: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, islinkresolved: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppnetworksourceid: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pnetworksourceid: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppremoteobjectid: *mut *mut CONTACT_AGGREGATION_BLOB) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, premoteobjectid: *const CONTACT_AGGREGATION_BLOB) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppserverpersonid: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pserverpersonid: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppserverpersonid: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pserverpersonid: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppsyncidentityhash: *mut *mut CONTACT_AGGREGATION_BLOB) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psyncidentityhash: *const CONTACT_AGGREGATION_BLOB) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Contacts`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IContactAggregationLinkCollection(pub ::windows::runtime::IUnknown);
impl IContactAggregationLinkCollection {
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn FindFirst(&self) -> ::windows::runtime::Result<IContactAggregationLink> {
        let mut result__: <IContactAggregationLink as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IContactAggregationLink>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn FindFirstByRemoteId<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, psourcetype: Param0, paccountid: Param1, premoteid: *const CONTACT_AGGREGATION_BLOB) -> ::windows::runtime::Result<IContactAggregationLink> {
        let mut result__: <IContactAggregationLink as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), psourcetype.into_param().abi(), paccountid.into_param().abi(), ::core::mem::transmute(premoteid), &mut result__).from_abi::<IContactAggregationLink>(result__)
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn FindNext(&self) -> ::windows::runtime::Result<IContactAggregationLink> {
        let mut result__: <IContactAggregationLink as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IContactAggregationLink>(result__)
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IContactAggregationLinkCollection {
    type Vtable = IContactAggregationLinkCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xf8bc0e93_fb55_4f28_b9fa_b1c274153292);
}
impl ::core::convert::From<IContactAggregationLinkCollection> for ::windows::runtime::IUnknown {
    fn from(value: IContactAggregationLinkCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IContactAggregationLinkCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IContactAggregationLinkCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IContactAggregationLinkCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IContactAggregationLinkCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAggregationLinkCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppservercontactlink: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psourcetype: super::super::Foundation::PWSTR, paccountid: super::super::Foundation::PWSTR, premoteid: *const CONTACT_AGGREGATION_BLOB, ppservercontactlink: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppservercontactlink: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcount: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Contacts`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IContactAggregationManager(pub ::windows::runtime::IUnknown);
impl IContactAggregationManager {
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn GetVersionInfo(&self, plmajorversion: *mut i32, plminorversion: *mut i32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), ::core::mem::transmute(plmajorversion), ::core::mem::transmute(plminorversion)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn CreateOrOpenGroup<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pgroupname: Param0, options: CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS, pcreatedgroup: *mut super::super::Foundation::BOOL, ppgroup: *mut ::core::option::Option<IContactAggregationGroup>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pgroupname.into_param().abi(), ::core::mem::transmute(options), ::core::mem::transmute(pcreatedgroup), ::core::mem::transmute(ppgroup)).ok()
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn CreateExternalContact(&self) -> ::windows::runtime::Result<IContactAggregationContact> {
        let mut result__: <IContactAggregationContact as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IContactAggregationContact>(result__)
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn CreateServerPerson(&self) -> ::windows::runtime::Result<IContactAggregationServerPerson> {
        let mut result__: <IContactAggregationServerPerson as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IContactAggregationServerPerson>(result__)
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn CreateServerContactLink(&self) -> ::windows::runtime::Result<IContactAggregationLink> {
        let mut result__: <IContactAggregationLink as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IContactAggregationLink>(result__)
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn Flush(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn OpenAggregateContact<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pitemid: Param0) -> ::windows::runtime::Result<IContactAggregationAggregate> {
        let mut result__: <IContactAggregationAggregate as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pitemid.into_param().abi(), &mut result__).from_abi::<IContactAggregationAggregate>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn OpenContact<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pitemid: Param0) -> ::windows::runtime::Result<IContactAggregationContact> {
        let mut result__: <IContactAggregationContact as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pitemid.into_param().abi(), &mut result__).from_abi::<IContactAggregationContact>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn OpenServerContactLink<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pitemid: Param0) -> ::windows::runtime::Result<IContactAggregationLink> {
        let mut result__: <IContactAggregationLink as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), pitemid.into_param().abi(), &mut result__).from_abi::<IContactAggregationLink>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn OpenServerPerson<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pitemid: Param0) -> ::windows::runtime::Result<IContactAggregationServerPerson> {
        let mut result__: <IContactAggregationServerPerson as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), pitemid.into_param().abi(), &mut result__).from_abi::<IContactAggregationServerPerson>(result__)
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn Contacts(&self, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS) -> ::windows::runtime::Result<IContactAggregationContactCollection> {
        let mut result__: <IContactAggregationContactCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), ::core::mem::transmute(options), &mut result__).from_abi::<IContactAggregationContactCollection>(result__)
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn AggregateContacts(&self, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS) -> ::windows::runtime::Result<IContactAggregationAggregateCollection> {
        let mut result__: <IContactAggregationAggregateCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(options), &mut result__).from_abi::<IContactAggregationAggregateCollection>(result__)
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn Groups(&self, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS) -> ::windows::runtime::Result<IContactAggregationGroupCollection> {
        let mut result__: <IContactAggregationGroupCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(options), &mut result__).from_abi::<IContactAggregationGroupCollection>(result__)
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn ServerPersons(&self) -> ::windows::runtime::Result<IContactAggregationServerPersonCollection> {
        let mut result__: <IContactAggregationServerPersonCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IContactAggregationServerPersonCollection>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn ServerContactLinks<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, ppersonitemid: Param0) -> ::windows::runtime::Result<IContactAggregationLinkCollection> {
        let mut result__: <IContactAggregationLinkCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), ppersonitemid.into_param().abi(), &mut result__).from_abi::<IContactAggregationLinkCollection>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IContactAggregationManager {
    type Vtable = IContactAggregationManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x1d865989_4b1f_4b60_8f34_c2ad468b2b50);
}
impl ::core::convert::From<IContactAggregationManager> for ::windows::runtime::IUnknown {
    fn from(value: IContactAggregationManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IContactAggregationManager> for ::windows::runtime::IUnknown {
    fn from(value: &IContactAggregationManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IContactAggregationManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IContactAggregationManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAggregationManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plmajorversion: *mut i32, plminorversion: *mut i32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pgroupname: super::super::Foundation::PWSTR, options: CONTACT_AGGREGATION_CREATE_OR_OPEN_OPTIONS, pcreatedgroup: *mut super::super::Foundation::BOOL, ppgroup: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppitem: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppserverperson: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppservercontactlink: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pitemid: super::super::Foundation::PWSTR, ppitem: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pitemid: super::super::Foundation::PWSTR, ppitem: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pitemid: super::super::Foundation::PWSTR, ppitem: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pitemid: super::super::Foundation::PWSTR, ppitem: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS, ppitems: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS, ppaggregates: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, options: CONTACT_AGGREGATION_COLLECTION_OPTIONS, ppgroups: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppserverpersoncollection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppersonitemid: super::super::Foundation::PWSTR, ppservercontactlinkcollection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Contacts`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IContactAggregationServerPerson(pub ::windows::runtime::IUnknown);
impl IContactAggregationServerPerson {
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn Delete(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn Save(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn AggregateId(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn SetAggregateId<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, paggregateid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), paggregateid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn AntiLink(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn SetAntiLink<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pantilink: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pantilink.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn AntiLinkBaseline(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn SetAntiLinkBaseline<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pantilink: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pantilink.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn FavoriteOrder(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn SetFavoriteOrder(&self, favoriteorder: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), ::core::mem::transmute(favoriteorder)).ok()
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn FavoriteOrderBaseline(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn SetFavoriteOrderBaseline(&self, favoriteorder: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), ::core::mem::transmute(favoriteorder)).ok()
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn Groups(&self) -> ::windows::runtime::Result<*mut CONTACT_AGGREGATION_BLOB> {
        let mut result__: <*mut CONTACT_AGGREGATION_BLOB as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut CONTACT_AGGREGATION_BLOB>(result__)
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn SetGroups(&self, pgroups: *const CONTACT_AGGREGATION_BLOB) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).16)(::core::mem::transmute_copy(self), ::core::mem::transmute(pgroups)).ok()
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn GroupsBaseline(&self) -> ::windows::runtime::Result<*mut CONTACT_AGGREGATION_BLOB> {
        let mut result__: <*mut CONTACT_AGGREGATION_BLOB as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).17)(::core::mem::transmute_copy(self), &mut result__).from_abi::<*mut CONTACT_AGGREGATION_BLOB>(result__)
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn SetGroupsBaseline(&self, pgroups: *const CONTACT_AGGREGATION_BLOB) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).18)(::core::mem::transmute_copy(self), ::core::mem::transmute(pgroups)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn Id(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).19)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn IsTombstone(&self) -> ::windows::runtime::Result<super::super::Foundation::BOOL> {
        let mut result__: <super::super::Foundation::BOOL as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).20)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::BOOL>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn SetIsTombstone<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, istombstone: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).21)(::core::mem::transmute_copy(self), istombstone.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn LinkedAggregateId(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).22)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn SetLinkedAggregateId<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, plinkedaggregateid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).23)(::core::mem::transmute_copy(self), plinkedaggregateid.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn ObjectId(&self) -> ::windows::runtime::Result<super::super::Foundation::PWSTR> {
        let mut result__: <super::super::Foundation::PWSTR as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).24)(::core::mem::transmute_copy(self), &mut result__).from_abi::<super::super::Foundation::PWSTR>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn SetObjectId<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pobjectid: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).25)(::core::mem::transmute_copy(self), pobjectid.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IContactAggregationServerPerson {
    type Vtable = IContactAggregationServerPerson_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x7fdc3d4b_1b82_4334_85c5_25184ee5a5f2);
}
impl ::core::convert::From<IContactAggregationServerPerson> for ::windows::runtime::IUnknown {
    fn from(value: IContactAggregationServerPerson) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IContactAggregationServerPerson> for ::windows::runtime::IUnknown {
    fn from(value: &IContactAggregationServerPerson) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IContactAggregationServerPerson {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IContactAggregationServerPerson {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAggregationServerPerson_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppaggregateid: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, paggregateid: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppantilink: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pantilink: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppantilink: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pantilink: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfavoriteorder: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, favoriteorder: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pfavoriteorder: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, favoriteorder: u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pgroups: *mut *mut CONTACT_AGGREGATION_BLOB) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pgroups: *const CONTACT_AGGREGATION_BLOB) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppgroups: *mut *mut CONTACT_AGGREGATION_BLOB) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pgroups: *const CONTACT_AGGREGATION_BLOB) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppid: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pistombstone: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, istombstone: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pplinkedaggregateid: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, plinkedaggregateid: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppobjectid: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pobjectid: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Contacts`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IContactAggregationServerPersonCollection(pub ::windows::runtime::IUnknown);
impl IContactAggregationServerPersonCollection {
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn FindFirst(&self) -> ::windows::runtime::Result<IContactAggregationServerPerson> {
        let mut result__: <IContactAggregationServerPerson as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IContactAggregationServerPerson>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn FindFirstByServerId<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pserverid: Param0) -> ::windows::runtime::Result<IContactAggregationServerPerson> {
        let mut result__: <IContactAggregationServerPerson as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pserverid.into_param().abi(), &mut result__).from_abi::<IContactAggregationServerPerson>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn FindFirstByAggregateId<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, paggregateid: Param0) -> ::windows::runtime::Result<IContactAggregationServerPerson> {
        let mut result__: <IContactAggregationServerPerson as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), paggregateid.into_param().abi(), &mut result__).from_abi::<IContactAggregationServerPerson>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn FindFirstByLinkedAggregateId<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, paggregateid: Param0) -> ::windows::runtime::Result<IContactAggregationServerPerson> {
        let mut result__: <IContactAggregationServerPerson as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), paggregateid.into_param().abi(), &mut result__).from_abi::<IContactAggregationServerPerson>(result__)
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn FindNext(&self) -> ::windows::runtime::Result<IContactAggregationServerPerson> {
        let mut result__: <IContactAggregationServerPerson as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IContactAggregationServerPerson>(result__)
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn Count(&self) -> ::windows::runtime::Result<u32> {
        let mut result__: <u32 as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<u32>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IContactAggregationServerPersonCollection {
    type Vtable = IContactAggregationServerPersonCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x4f730a4a_6604_47b6_a987_669ecf1e5751);
}
impl ::core::convert::From<IContactAggregationServerPersonCollection> for ::windows::runtime::IUnknown {
    fn from(value: IContactAggregationServerPersonCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IContactAggregationServerPersonCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IContactAggregationServerPersonCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IContactAggregationServerPersonCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IContactAggregationServerPersonCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactAggregationServerPersonCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppserverperson: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pserverid: super::super::Foundation::PWSTR, ppserverperson: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, paggregateid: super::super::Foundation::PWSTR, ppserverperson: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, paggregateid: super::super::Foundation::PWSTR, ppserverperson: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppserverperson: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pcount: *mut u32) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Contacts`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IContactCollection(pub ::windows::runtime::IUnknown);
impl IContactCollection {
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn Next(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn GetCurrent(&self) -> ::windows::runtime::Result<IContact> {
        let mut result__: <IContact as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IContact>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IContactCollection {
    type Vtable = IContactCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xb6afa338_d779_11d9_8bde_f66bad1e3f3a);
}
impl ::core::convert::From<IContactCollection> for ::windows::runtime::IUnknown {
    fn from(value: IContactCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IContactCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IContactCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IContactCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IContactCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppcontact: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Contacts`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IContactManager(pub ::windows::runtime::IUnknown);
impl IContactManager {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn Initialize<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszappname: Param0, pszappversion: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pszappname.into_param().abi(), pszappversion.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn Load<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszcontactid: Param0) -> ::windows::runtime::Result<IContact> {
        let mut result__: <IContact as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pszcontactid.into_param().abi(), &mut result__).from_abi::<IContact>(result__)
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn MergeContactIDs<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param1: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, psznewcontactid: Param0, pszoldcontactid: Param1) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), psznewcontactid.into_param().abi(), pszoldcontactid.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn GetMeContact(&self) -> ::windows::runtime::Result<IContact> {
        let mut result__: <IContact as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IContact>(result__)
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn SetMeContact<'a, Param0: ::windows::runtime::IntoParam<'a, IContact>>(&self, pmecontact: Param0) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pmecontact.into_param().abi()).ok()
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn GetContactCollection(&self) -> ::windows::runtime::Result<IContactCollection> {
        let mut result__: <IContactCollection as ::windows::runtime::Abi>::Abi = ::core::mem::zeroed();
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), &mut result__).from_abi::<IContactCollection>(result__)
    }
}
unsafe impl ::windows::runtime::Interface for IContactManager {
    type Vtable = IContactManager_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xad553d98_deb1_474a_8e17_fc0c2075b738);
}
impl ::core::convert::From<IContactManager> for ::windows::runtime::IUnknown {
    fn from(value: IContactManager) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IContactManager> for ::windows::runtime::IUnknown {
    fn from(value: &IContactManager) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IContactManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IContactManager {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactManager_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszappname: super::super::Foundation::PWSTR, pszappversion: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszcontactid: super::super::Foundation::PWSTR, ppcontact: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, psznewcontactid: super::super::Foundation::PWSTR, pszoldcontactid: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppmecontact: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pmecontact: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, ppcontactcollection: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
);
#[doc = "*Required features: `Win32_System_Contacts`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IContactProperties(pub ::windows::runtime::IUnknown);
impl IContactProperties {
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn GetString<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpropertyname: Param0, dwflags: u32, pszvalue: Param2, cchvalue: u32, pdwcchpropertyvaluerequired: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self), pszpropertyname.into_param().abi(), ::core::mem::transmute(dwflags), pszvalue.into_param().abi(), ::core::mem::transmute(cchvalue), ::core::mem::transmute(pdwcchpropertyvaluerequired)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn GetDate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpropertyname: Param0, dwflags: u32, pftdatetime: *mut super::super::Foundation::FILETIME) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self), pszpropertyname.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(pftdatetime)).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn GetBinary<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpropertyname: Param0, dwflags: u32, pszcontenttype: Param2, cchcontenttype: u32, pdwcchcontenttyperequired: *mut u32, ppstream: *mut ::core::option::Option<super::Com::IStream>) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pszpropertyname.into_param().abi(), ::core::mem::transmute(dwflags), pszcontenttype.into_param().abi(), ::core::mem::transmute(cchcontenttype), ::core::mem::transmute(pdwcchcontenttyperequired), ::core::mem::transmute(ppstream)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn GetLabels<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszarrayelementname: Param0, dwflags: u32, pszlabels: Param2, cchlabels: u32, pdwcchlabelsrequired: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), pszarrayelementname.into_param().abi(), ::core::mem::transmute(dwflags), pszlabels.into_param().abi(), ::core::mem::transmute(cchlabels), ::core::mem::transmute(pdwcchlabelsrequired)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn SetString<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpropertyname: Param0, dwflags: u32, pszvalue: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), pszpropertyname.into_param().abi(), ::core::mem::transmute(dwflags), pszvalue.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn SetDate<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::FILETIME>>(&self, pszpropertyname: Param0, dwflags: u32, ftdatetime: Param2) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), pszpropertyname.into_param().abi(), ::core::mem::transmute(dwflags), ftdatetime.into_param().abi()).ok()
    }
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`, `Win32_System_Com`*"]
    pub unsafe fn SetBinary<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param3: ::windows::runtime::IntoParam<'a, super::Com::IStream>>(&self, pszpropertyname: Param0, dwflags: u32, pszcontenttype: Param2, pstream: Param3) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pszpropertyname.into_param().abi(), ::core::mem::transmute(dwflags), pszcontenttype.into_param().abi(), pstream.into_param().abi()).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn SetLabels<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszarrayelementname: Param0, dwflags: u32, dwlabelcount: u32, ppszlabels: *const super::super::Foundation::PWSTR) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).10)(::core::mem::transmute_copy(self), pszarrayelementname.into_param().abi(), ::core::mem::transmute(dwflags), ::core::mem::transmute(dwlabelcount), ::core::mem::transmute(ppszlabels)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn CreateArrayNode<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>, Param3: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszarrayname: Param0, dwflags: u32, fappend: Param2, psznewarrayelementname: Param3, cchnewarrayelementname: u32, pdwcchnewarrayelementnamerequired: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).11)(::core::mem::transmute_copy(self), pszarrayname.into_param().abi(), ::core::mem::transmute(dwflags), fappend.into_param().abi(), psznewarrayelementname.into_param().abi(), ::core::mem::transmute(cchnewarrayelementname), ::core::mem::transmute(pdwcchnewarrayelementnamerequired)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn DeleteProperty<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpropertyname: Param0, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).12)(::core::mem::transmute_copy(self), pszpropertyname.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn DeleteArrayNode<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszarrayelementname: Param0, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).13)(::core::mem::transmute_copy(self), pszarrayelementname.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn DeleteLabels<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszarrayelementname: Param0, dwflags: u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).14)(::core::mem::transmute_copy(self), pszarrayelementname.into_param().abi(), ::core::mem::transmute(dwflags)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn GetPropertyCollection<'a, Param2: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>, Param5: ::windows::runtime::IntoParam<'a, super::super::Foundation::BOOL>>(&self, pppropertycollection: *mut ::core::option::Option<IContactPropertyCollection>, dwflags: u32, pszmultivaluename: Param2, dwlabelcount: u32, ppszlabels: *const super::super::Foundation::PWSTR, fanylabelmatches: Param5) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).15)(::core::mem::transmute_copy(self), ::core::mem::transmute(pppropertycollection), ::core::mem::transmute(dwflags), pszmultivaluename.into_param().abi(), ::core::mem::transmute(dwlabelcount), ::core::mem::transmute(ppszlabels), fanylabelmatches.into_param().abi()).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IContactProperties {
    type Vtable = IContactProperties_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0x70dd27dd_5cbd_46e8_bef0_23b6b346288f);
}
impl ::core::convert::From<IContactProperties> for ::windows::runtime::IUnknown {
    fn from(value: IContactProperties) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IContactProperties> for ::windows::runtime::IUnknown {
    fn from(value: &IContactProperties) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IContactProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IContactProperties {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactProperties_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpropertyname: super::super::Foundation::PWSTR, dwflags: u32, pszvalue: super::super::Foundation::PWSTR, cchvalue: u32, pdwcchpropertyvaluerequired: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpropertyname: super::super::Foundation::PWSTR, dwflags: u32, pftdatetime: *mut super::super::Foundation::FILETIME) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpropertyname: super::super::Foundation::PWSTR, dwflags: u32, pszcontenttype: super::super::Foundation::PWSTR, cchcontenttype: u32, pdwcchcontenttyperequired: *mut u32, ppstream: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszarrayelementname: super::super::Foundation::PWSTR, dwflags: u32, pszlabels: super::super::Foundation::PWSTR, cchlabels: u32, pdwcchlabelsrequired: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpropertyname: super::super::Foundation::PWSTR, dwflags: u32, pszvalue: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpropertyname: super::super::Foundation::PWSTR, dwflags: u32, ftdatetime: super::super::Foundation::FILETIME) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpropertyname: super::super::Foundation::PWSTR, dwflags: u32, pszcontenttype: super::super::Foundation::PWSTR, pstream: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(not(all(feature = "Win32_Foundation", feature = "Win32_System_Com")))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszarrayelementname: super::super::Foundation::PWSTR, dwflags: u32, dwlabelcount: u32, ppszlabels: *const super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszarrayname: super::super::Foundation::PWSTR, dwflags: u32, fappend: super::super::Foundation::BOOL, psznewarrayelementname: super::super::Foundation::PWSTR, cchnewarrayelementname: u32, pdwcchnewarrayelementnamerequired: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpropertyname: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszarrayelementname: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszarrayelementname: super::super::Foundation::PWSTR, dwflags: u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pppropertycollection: *mut ::windows::runtime::RawPtr, dwflags: u32, pszmultivaluename: super::super::Foundation::PWSTR, dwlabelcount: u32, ppszlabels: *const super::super::Foundation::PWSTR, fanylabelmatches: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
#[doc = "*Required features: `Win32_System_Contacts`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: clone :: Clone, :: core :: fmt :: Debug)]
pub struct IContactPropertyCollection(pub ::windows::runtime::IUnknown);
impl IContactPropertyCollection {
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn Reset(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).3)(::core::mem::transmute_copy(self)).ok()
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn Next(&self) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).4)(::core::mem::transmute_copy(self)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn GetPropertyName<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszpropertyname: Param0, cchpropertyname: u32, pdwcchpropertynamerequired: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).5)(::core::mem::transmute_copy(self), pszpropertyname.into_param().abi(), ::core::mem::transmute(cchpropertyname), ::core::mem::transmute(pdwcchpropertynamerequired)).ok()
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn GetPropertyType(&self, pdwtype: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).6)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwtype)).ok()
    }
    #[doc = "*Required features: `Win32_System_Contacts`*"]
    pub unsafe fn GetPropertyVersion(&self, pdwversion: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).7)(::core::mem::transmute_copy(self), ::core::mem::transmute(pdwversion)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn GetPropertyModificationDate(&self, pftmodificationdate: *mut super::super::Foundation::FILETIME) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).8)(::core::mem::transmute_copy(self), ::core::mem::transmute(pftmodificationdate)).ok()
    }
    #[cfg(feature = "Win32_Foundation")]
    #[doc = "*Required features: `Win32_System_Contacts`, `Win32_Foundation`*"]
    pub unsafe fn GetPropertyArrayElementID<'a, Param0: ::windows::runtime::IntoParam<'a, super::super::Foundation::PWSTR>>(&self, pszarrayelementid: Param0, ccharrayelementid: u32, pdwccharrayelementidrequired: *mut u32) -> ::windows::runtime::Result<()> {
        (::windows::runtime::Interface::vtable(self).9)(::core::mem::transmute_copy(self), pszarrayelementid.into_param().abi(), ::core::mem::transmute(ccharrayelementid), ::core::mem::transmute(pdwccharrayelementidrequired)).ok()
    }
}
unsafe impl ::windows::runtime::Interface for IContactPropertyCollection {
    type Vtable = IContactPropertyCollection_abi;
    const IID: ::windows::runtime::GUID = ::windows::runtime::GUID::from_u128(0xffd3adf8_fa64_4328_b1b6_2e0db509cb3c);
}
impl ::core::convert::From<IContactPropertyCollection> for ::windows::runtime::IUnknown {
    fn from(value: IContactPropertyCollection) -> Self {
        value.0
    }
}
impl ::core::convert::From<&IContactPropertyCollection> for ::windows::runtime::IUnknown {
    fn from(value: &IContactPropertyCollection) -> Self {
        value.0.clone()
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for IContactPropertyCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Owned(self.0)
    }
}
impl<'a> ::windows::runtime::IntoParam<'a, ::windows::runtime::IUnknown> for &'a IContactPropertyCollection {
    fn into_param(self) -> ::windows::runtime::Param<'a, ::windows::runtime::IUnknown> {
        ::windows::runtime::Param::Borrowed(&self.0)
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IContactPropertyCollection_abi(
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, iid: &::windows::runtime::GUID, interface: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> u32,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszpropertyname: super::super::Foundation::PWSTR, cchpropertyname: u32, pdwcchpropertynamerequired: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwtype: *mut u32) -> ::windows::runtime::HRESULT,
    pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pdwversion: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pftmodificationdate: *mut super::super::Foundation::FILETIME) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
    #[cfg(feature = "Win32_Foundation")] pub unsafe extern "system" fn(this: ::windows::runtime::RawPtr, pszarrayelementid: super::super::Foundation::PWSTR, ccharrayelementid: u32, pdwccharrayelementidrequired: *mut u32) -> ::windows::runtime::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))] usize,
);
